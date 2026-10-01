use std::borrow::Cow;
use std::io::Write;

use crate::demux::{self, Codec, Sample, Track};

const TAG_AUDIO: u8 = 8;
const TAG_VIDEO: u8 = 9;
const TAG_SCRIPT: u8 = 18;

fn be24(d: &[u8], at: usize) -> Option<u32> {
    let b = d.get(at..at + 3)?;
    Some(u32::from_be_bytes([0, b[0], b[1], b[2]]))
}

fn be32(d: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(d.get(at..at + 4)?.try_into().ok()?))
}

fn mp3_header(frame: &[u8]) -> Option<(u32, u16)> {
    let h = be32(frame, 0)?;
    if h >> 21 != 0x7FF {
        return None;
    }
    let version = (h >> 19) & 3;
    let rate_index = ((h >> 10) & 3) as usize;
    let base = [44100, 48000, 32000].get(rate_index)?;
    let rate = match version {
        3 => *base,
        2 => base / 2,
        0 => base / 4,
        _ => return None,
    };
    let channels = if (h >> 6) & 3 == 3 { 1 } else { 2 };
    Some((rate, channels))
}

fn video_codec_name(id: u8) -> String {
    match id {
        2 => "Sorenson Spark (H.263)",
        3 => "Screen Video",
        4 | 5 => "On2 VP6",
        6 => "Screen Video 2",
        12 => "HEVC (H.265)",
        _ => "an unknown FLV video codec",
    }
    .to_string()
}

fn audio_codec_name(format: u8) -> String {
    match format {
        0 | 3 => "uncompressed FLV PCM",
        1 => "ADPCM",
        4..=6 => "Nellymoser",
        7 | 8 => "G.711",
        11 => "Speex",
        _ => "an unknown FLV audio codec",
    }
    .to_string()
}

pub fn read(raw: &[u8]) -> Result<Vec<Track<'_>>, String> {
    if raw.len() < 13 || &raw[0..3] != b"FLV" {
        return Err("This doesn't look like a valid FLV file.".to_string());
    }
    let mut p = be32(raw, 5).unwrap_or(9) as usize + 4;

    let mut avcc: Option<Vec<u8>> = None;
    let mut video_samples: Vec<Sample> = Vec::new();
    let mut video_other: Option<String> = None;
    let mut asc: Option<Vec<u8>> = None;
    let mut audio_samples: Vec<Sample> = Vec::new();
    let mut audio_mp3 = false;
    let mut audio_other: Option<String> = None;
    let mut audio_stereo = true;

    while p + 11 <= raw.len() {
        let kind = raw[p] & 0x1F;
        let size = be24(raw, p + 1).unwrap_or(0) as usize;
        let ts = be24(raw, p + 4).unwrap_or(0) | ((raw[p + 7] as u32) << 24);
        let Some(data) = raw.get(p + 11..p + 11 + size) else { break };
        p += 11 + size + 4;
        let ts_ns = ts as i64 * 1_000_000;

        match kind {
            TAG_VIDEO if !data.is_empty() => {
                let frame_type = data[0] >> 4;
                let codec = data[0] & 0x0F;
                if frame_type == 5 {
                    continue;
                }
                if codec == 7 {
                    if data.len() < 5 {
                        continue;
                    }
                    let cts = ((be24(data, 2).unwrap_or(0) << 8) as i32 >> 8) as i64;
                    match data[1] {
                        0 => avcc = Some(data[5..].to_vec()),
                        1 => video_samples.push(Sample {
                            pts_ns: ts_ns + cts * 1_000_000,
                            keyframe: frame_type == 1,
                            data: Cow::Borrowed(&data[5..]),
                        }),
                        _ => {}
                    }
                } else {
                    video_other.get_or_insert_with(|| video_codec_name(codec));
                    video_samples.push(Sample { pts_ns: ts_ns, keyframe: frame_type == 1, data: Cow::Borrowed(&data[1..]) });
                }
            }
            TAG_AUDIO if !data.is_empty() => {
                let format = data[0] >> 4;
                audio_stereo = data[0] & 1 == 1;
                match format {
                    10 if data.len() >= 2 => match data[1] {
                        0 => asc = Some(data[2..].to_vec()),
                        _ => audio_samples.push(Sample { pts_ns: ts_ns, keyframe: true, data: Cow::Borrowed(&data[2..]) }),
                    },
                    2 | 14 => {
                        audio_mp3 = true;
                        audio_samples.push(Sample { pts_ns: ts_ns, keyframe: true, data: Cow::Borrowed(&data[1..]) });
                    }
                    _ => {
                        audio_other.get_or_insert_with(|| audio_codec_name(format));
                        audio_samples.push(Sample { pts_ns: ts_ns, keyframe: true, data: Cow::Borrowed(&data[1..]) });
                    }
                }
            }
            _ => {}
        }
    }

    let mut tracks = Vec::new();
    if !video_samples.is_empty() {
        let (codec, (width, height)) = match (video_other, avcc) {
            (None, Some(avcc)) => {
                let size = crate::h264::sps_dimensions(&avcc).unwrap_or((0, 0));
                (Codec::H264 { avcc }, size)
            }
            (None, None) => (Codec::Unsupported("H.264 without a decoder configuration".to_string()), (0, 0)),
            (Some(name), _) => (Codec::Unsupported(name), (0, 0)),
        };
        tracks.push(Track { video: true, codec, width, height, sample_rate: 0, channels: 0, codec_delay_ns: 0, samples: video_samples });
    }
    if !audio_samples.is_empty() {
        let (codec, sample_rate, channels) = if let Some(name) = audio_other {
            (Codec::Unsupported(name), 0, 0)
        } else if audio_mp3 {
            let (rate, channels) = audio_samples.iter().find_map(|s| mp3_header(&s.data)).unwrap_or((44100, 2));
            (Codec::Mp3, rate, channels)
        } else if let Some(asc) = asc {
            let (_, rate_index, channels) = demux::adts_params(&asc).unwrap_or((1, 4, 2));
            let rate = demux::AAC_SAMPLE_RATES.get(rate_index as usize).copied().unwrap_or(44100);
            let channels = if channels == 0 { if audio_stereo { 2 } else { 1 } } else { channels as u16 };
            (Codec::Aac { asc }, rate, channels)
        } else {
            (Codec::Unsupported("AAC without a decoder configuration".to_string()), 0, 0)
        };
        tracks.push(Track { video: false, codec, width: 0, height: 0, sample_rate, channels, codec_delay_ns: 0, samples: audio_samples });
    }
    if tracks.is_empty() {
        return Err("No audio or video frames were found in this FLV file.".to_string());
    }
    Ok(tracks)
}

pub fn duration_ms(head: &[u8]) -> Option<u64> {
    let at = head.windows(10).position(|w| w == b"\x00\x08duration")?;
    let marker = *head.get(at + 10)?;
    if marker != 0 {
        return None;
    }
    let seconds = f64::from_be_bytes(head.get(at + 11..at + 19)?.try_into().ok()?);
    (seconds.is_finite() && seconds > 0.0).then(|| (seconds * 1000.0).round() as u64)
}

fn amf_string(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u16).to_be_bytes());
    out.extend_from_slice(s.as_bytes());
}

fn amf_number(out: &mut Vec<u8>, key: &str, value: f64) {
    amf_string(out, key);
    out.push(0);
    out.extend_from_slice(&value.to_be_bytes());
}

fn amf_bool(out: &mut Vec<u8>, key: &str, value: bool) {
    amf_string(out, key);
    out.push(1);
    out.push(value as u8);
}

fn write_tag<W: Write>(out: &mut W, kind: u8, ts_ms: i64, parts: &[&[u8]]) -> Result<(), String> {
    let size: usize = parts.iter().map(|part| part.len()).sum();
    if size >= 1 << 24 {
        return Err("A frame is too large to fit in an FLV file.".to_string());
    }
    let ts = ts_ms.clamp(0, u32::MAX as i64) as u32;
    let mut header = [0u8; 11];
    header[0] = kind;
    header[1..4].copy_from_slice(&(size as u32).to_be_bytes()[1..]);
    header[4..7].copy_from_slice(&ts.to_be_bytes()[1..]);
    header[7] = (ts >> 24) as u8;
    let write_err = |e: std::io::Error| format!("Couldn't write the output file: {e}");
    out.write_all(&header).map_err(write_err)?;
    for part in parts {
        out.write_all(part).map_err(write_err)?;
    }
    out.write_all(&(11 + size as u32).to_be_bytes()).map_err(write_err)
}

fn mp3_flags(sample_rate: u32, channels: u16) -> u8 {
    let rate = match sample_rate {
        0..=8000 => 0,
        8001..=12000 => 1,
        12001..=24000 => 2,
        _ => 3,
    };
    (2 << 4) | (rate << 2) | (1 << 1) | (channels >= 2) as u8
}

pub fn write<W: Write>(tracks: &[Track], out: &mut W) -> Result<(), String> {
    let video = tracks.iter().find(|t| t.video && !t.samples.is_empty());
    let audio = tracks.iter().find(|t| !t.video && !t.samples.is_empty());
    if video.is_none() && audio.is_none() {
        return Err("No audio or video frames were found in this file.".to_string());
    }
    let avcc = match video.map(|t| &t.codec) {
        Some(Codec::H264 { avcc }) if avcc.len() >= 7 => Some(avcc.as_slice()),
        Some(Codec::H264 { .. }) => return Err("This H.264 track has no usable decoder configuration, so it can't be converted.".to_string()),
        Some(other) => return Err(format!("FLV output can't hold {} video without re-encoding.", other.name())),
        None => None,
    };
    let asc = match audio.map(|t| &t.codec) {
        Some(Codec::Aac { asc }) if asc.len() >= 2 => Some(asc.as_slice()),
        Some(Codec::Aac { .. }) => return Err("This AAC track has no decoder configuration, so it can't be converted.".to_string()),
        Some(Codec::Mp3) | None => None,
        Some(other) => return Err(format!("FLV output can't hold {} audio without re-encoding.", other.name())),
    };

    let to_ms = |ns: i64| (ns as f64 / 1_000_000.0).round() as i64;
    let mut video_times: Vec<(i64, i64)> = Vec::new();
    if let Some(t) = video {
        let pts: Vec<i64> = t.samples.iter().map(|s| to_ms(s.pts_ns)).collect();
        let mut sorted = pts.clone();
        sorted.sort_unstable();
        let mut dts: Vec<i64> = Vec::with_capacity(pts.len());
        for (i, &s) in sorted.iter().enumerate() {
            dts.push(if i > 0 && s <= dts[i - 1] { dts[i - 1] + 1 } else { s });
        }
        let shift = dts.iter().zip(&pts).map(|(d, p)| d - p).max().unwrap_or(0).max(0);
        video_times = dts.iter().zip(&pts).map(|(d, p)| (d - shift, p - (d - shift))).collect();
    }
    let audio_times: Vec<i64> = audio
        .map(|t| t.samples.iter().map(|s| to_ms(s.pts_ns - t.codec_delay_ns)).collect())
        .unwrap_or_default();

    let base = video_times.iter().map(|t| t.0).chain(audio_times.iter().copied()).min().unwrap_or(0);
    let end = video_times
        .iter()
        .map(|t| t.0 + t.1)
        .chain(audio_times.iter().copied())
        .max()
        .unwrap_or(0);

    let mut order: Vec<(i64, u8, usize)> = Vec::with_capacity(video_times.len() + audio_times.len());
    order.extend(audio_times.iter().enumerate().map(|(i, &ts)| (ts - base, TAG_AUDIO, i)));
    order.extend(video_times.iter().enumerate().map(|(i, t)| (t.0 - base, TAG_VIDEO, i)));
    order.sort_by_key(|&(ts, kind, i)| (ts, kind, i));

    let flags = (video.is_some() as u8) | ((audio.is_some() as u8) << 2);
    let write_err = |e: std::io::Error| format!("Couldn't write the output file: {e}");
    out.write_all(&[b'F', b'L', b'V', 1, flags, 0, 0, 0, 9, 0, 0, 0, 0]).map_err(write_err)?;

    let mut meta = vec![2];
    amf_string(&mut meta, "onMetaData");
    meta.push(8);
    let mut props = Vec::new();
    let mut count = 0u32;
    amf_number(&mut props, "duration", (end - base).max(0) as f64 / 1000.0);
    count += 1;
    if let Some(t) = video {
        amf_number(&mut props, "width", t.width as f64);
        amf_number(&mut props, "height", t.height as f64);
        amf_number(&mut props, "videocodecid", 7.0);
        count += 3;
    }
    if let Some(t) = audio {
        amf_number(&mut props, "audiocodecid", if asc.is_some() { 10.0 } else { 2.0 });
        amf_number(&mut props, "audiosamplerate", t.sample_rate as f64);
        amf_number(&mut props, "audiosamplesize", 16.0);
        amf_bool(&mut props, "stereo", t.channels >= 2);
        count += 4;
    }
    meta.extend_from_slice(&count.to_be_bytes());
    meta.extend_from_slice(&props);
    meta.extend_from_slice(&[0, 0, 9]);
    write_tag(out, TAG_SCRIPT, 0, &[&meta])?;

    if let Some(avcc) = avcc {
        write_tag(out, TAG_VIDEO, 0, &[&[0x17, 0, 0, 0, 0], avcc])?;
    }
    if let Some(asc) = asc {
        write_tag(out, TAG_AUDIO, 0, &[&[0xAF, 0], asc])?;
    }
    let audio_flags = audio.map(|t| if asc.is_some() { 0xAF } else { mp3_flags(t.sample_rate, t.channels) });
    let length_size = avcc.map(|a| ((a[4] & 0x03) + 1) as usize).unwrap_or(4);
    let mut keyframes: Vec<bool> = video
        .map(|t| t.samples.iter().map(|s| crate::mp4::h264_sample_is_idr(&s.data, length_size)).collect())
        .unwrap_or_default();
    if !keyframes.iter().any(|&k| k) {
        keyframes = video.map(|t| t.samples.iter().map(|s| s.keyframe).collect()).unwrap_or_default();
    }

    for (ts, kind, i) in order {
        if kind == TAG_VIDEO {
            let t = video.unwrap();
            let sample = &t.samples[i];
            let key = keyframes[i] || i == 0;
            let cts = video_times[i].1.clamp(-(1 << 23), (1 << 23) - 1) as i32;
            let cts = cts.to_be_bytes();
            let header = [if key { 0x17 } else { 0x27 }, 1, cts[1], cts[2], cts[3]];
            write_tag(out, TAG_VIDEO, ts, &[&header, &sample.data])?;
        } else {
            let sample = &audio.unwrap().samples[i];
            let flags = audio_flags.unwrap();
            if asc.is_some() {
                write_tag(out, TAG_AUDIO, ts, &[&[flags, 1], &sample.data])?;
            } else {
                write_tag(out, TAG_AUDIO, ts, &[&[flags], &sample.data])?;
            }
        }
    }
    Ok(())
}