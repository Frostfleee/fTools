use std::borrow::Cow;
use std::io::Write;

use crate::demux::{self, Codec, Sample, Track};
use crate::h264;

const SEGMENT_LIMIT: u64 = 1 << 30;
const AVIIF_KEYFRAME: u32 = 0x10;
const NOT_KEYFRAME: u32 = 0x8000_0000;
const SUPER_INDEX_HEADER: usize = 24;
const STD_INDEX_HEADER: u64 = 24;

fn le16(d: &[u8], at: usize) -> u16 {
    d.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]])).unwrap_or(0)
}

fn le32(d: &[u8], at: usize) -> u32 {
    d.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).unwrap_or(0)
}

fn chunks(d: &[u8]) -> Vec<([u8; 4], &[u8])> {
    let mut out = Vec::new();
    let mut p = 0usize;
    while p + 8 <= d.len() {
        let id: [u8; 4] = d[p..p + 4].try_into().unwrap();
        let size = le32(d, p + 4) as usize;
        let start = p + 8;
        let end = start.saturating_add(size).min(d.len());
        out.push((id, &d[start..end]));
        p = start.saturating_add(size).saturating_add(size & 1);
    }
    out
}

fn list<'a>(id: [u8; 4], body: &'a [u8], kind: &[u8; 4]) -> Option<&'a [u8]> {
    (&id == b"LIST" && body.get(0..4) == Some(kind.as_slice())).then(|| &body[4..])
}

const V1_L1: [u32; 15] = [0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448];
const V1_L2: [u32; 15] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384];
const V1_L3: [u32; 15] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320];
const V2_L1: [u32; 15] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256];
const V2_L23: [u32; 15] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160];

pub struct MpaFrame {
    pub len: usize,
    pub samples: u32,
    pub rate: u32,
    pub channels: u16,
    pub layer: u8,
    pub bitrate: u32,
    signature: u32,
}

pub fn mpa_frame(d: &[u8]) -> Option<MpaFrame> {
    let h = u32::from_be_bytes(d.get(0..4)?.try_into().ok()?);
    if h >> 21 != 0x7FF {
        return None;
    }
    let version = (h >> 19) & 3;
    let layer = 4 - ((h >> 17) & 3) as u8;
    let bitrate_index = ((h >> 12) & 15) as usize;
    let rate_index = ((h >> 10) & 3) as usize;
    if version == 1 || layer == 4 || bitrate_index == 0 || bitrate_index == 15 || rate_index == 3 {
        return None;
    }
    let base = [44100, 48000, 32000][rate_index];
    let rate = match version {
        3 => base,
        2 => base / 2,
        _ => base / 4,
    };
    let v1 = version == 3;
    let table = match (v1, layer) {
        (true, 1) => &V1_L1,
        (true, 2) => &V1_L2,
        (true, _) => &V1_L3,
        (false, 1) => &V2_L1,
        (false, _) => &V2_L23,
    };
    let bitrate = table[bitrate_index] * 1000;
    let padding = (h >> 9) & 1;
    let (len, samples) = match layer {
        1 => ((12 * bitrate / rate + padding) * 4, 384),
        2 => (144 * bitrate / rate + padding, 1152),
        _ if v1 => (144 * bitrate / rate + padding, 1152),
        _ => (72 * bitrate / rate + padding, 576),
    };
    let channels = if (h >> 6) & 3 == 3 { 1 } else { 2 };
    Some(MpaFrame { len: len as usize, samples, rate, channels, layer, bitrate, signature: h & 0xFFFE_0C00 })
}

pub fn split_mpa(data: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut locked: Option<u32> = None;
    let mut p = 0usize;
    while p + 4 <= data.len() {
        if let Some(f) = mpa_frame(&data[p..]) {
            let end = p + f.len;
            let fits = f.len > 4 && end <= data.len();
            let consistent = match locked {
                Some(sig) => sig == f.signature,
                None => end == data.len() || mpa_frame(&data[end.min(data.len())..]).is_some_and(|n| n.signature == f.signature),
            };
            if fits && consistent {
                locked = Some(f.signature);
                out.push((p, end));
                p = end;
                continue;
            }
        }
        p += 1;
    }
    out
}

fn adts_frames(data: &[u8]) -> (Vec<(usize, usize)>, Option<Vec<u8>>) {
    let mut out = Vec::new();
    let mut asc = None;
    let mut p = 0usize;
    while p + 7 <= data.len() {
        let d = &data[p..];
        if d[0] != 0xFF || d[1] & 0xF6 != 0xF0 {
            p += 1;
            continue;
        }
        let header = if d[1] & 1 == 1 { 7 } else { 9 };
        let len = (((d[3] & 3) as usize) << 11) | ((d[4] as usize) << 3) | (d[5] as usize >> 5);
        if len <= header || p + len > data.len() {
            p += 1;
            continue;
        }
        if asc.is_none() {
            let profile = (d[2] >> 6) + 1;
            let rate_index = (d[2] >> 2) & 0x0F;
            let channels = ((d[2] & 1) << 2) | (d[3] >> 6);
            asc = Some(vec![(profile << 3) | (rate_index >> 1), ((rate_index & 1) << 7) | (channels << 3)]);
        }
        out.push((p + header, p + len));
        p += len;
    }
    (out, asc)
}

struct StreamHeader<'a> {
    kind: [u8; 4],
    scale: u32,
    rate: u32,
    start: u32,
    length: u32,
    sample_size: u32,
    format: &'a [u8],
}

impl StreamHeader<'_> {
    fn units_ns(&self, units: u64) -> i64 {
        let (scale, rate) = if self.rate == 0 || self.scale == 0 { (1, 25) } else { (self.scale, self.rate) };
        (units as u128 * scale as u128 * 1_000_000_000 / rate as u128) as i64
    }
}

fn parse_strl(body: &[u8]) -> Option<StreamHeader<'_>> {
    let mut header = None;
    let mut format: &[u8] = &[];
    for (id, data) in chunks(body) {
        match &id {
            b"strh" if data.len() >= 48 => header = Some(data),
            b"strf" => format = data,
            _ => {}
        }
    }
    let h = header?;
    Some(StreamHeader {
        kind: h[0..4].try_into().ok()?,
        scale: le32(h, 20),
        rate: le32(h, 24),
        start: le32(h, 28),
        length: le32(h, 32),
        sample_size: le32(h, 44),
        format,
    })
}

fn headers(hdrl: &[u8]) -> Vec<StreamHeader<'_>> {
    chunks(hdrl).into_iter().filter_map(|(id, body)| list(id, body, b"strl").and_then(parse_strl)).collect()
}

fn scan_movi<'a>(d: &'a [u8], streams: &mut [Vec<&'a [u8]>]) {
    for (id, body) in chunks(d) {
        if let Some(rec) = list(id, body, b"rec ") {
            scan_movi(rec, streams);
        } else if id[0].is_ascii_digit() && id[1].is_ascii_digit() && &id[2..4] != b"pc" {
            let n = ((id[0] - b'0') * 10 + (id[1] - b'0')) as usize;
            if let Some(list) = streams.get_mut(n) {
                list.push(body);
            }
        }
    }
}

fn video_codec_name(fourcc: &[u8; 4]) -> String {
    match fourcc {
        b"DIV3" | b"DIV4" | b"MP43" | b"MPG3" | b"AP41" | b"COL1" => "MS MPEG-4 v3 (DivX 3)".to_string(),
        b"MP42" | b"DIV2" => "MS MPEG-4 v2".to_string(),
        b"MPG4" | b"MP41" | b"DIV1" => "MS MPEG-4 v1".to_string(),
        b"WMV1" | b"WMV2" | b"WMV3" | b"WVC1" => "Windows Media Video".to_string(),
        b"HEVC" | b"H265" | b"X265" | b"HVC1" | b"HEV1" => "HEVC (H.265)".to_string(),
        b"HFYU" => "HuffYUV".to_string(),
        b"FFV1" => "FFV1".to_string(),
        b"CVID" => "Cinepak".to_string(),
        b"IV31" | b"IV32" | b"IV41" | b"IV50" => "Indeo".to_string(),
        b"MPG1" | b"MPG2" | b"MPEG" => "MPEG-1/2 video".to_string(),
        b"DVSD" | b"DV25" | b"DV50" | b"CDVC" => "DV".to_string(),
        [0, 0, 0, 0] | b"DIB " | b"RGB " | b"RAW " => "uncompressed video".to_string(),
        _ if fourcc.iter().all(|b| b.is_ascii_graphic() || *b == b' ') => {
            format!("the {} video codec", String::from_utf8_lossy(fourcc).trim())
        }
        _ => "an unknown AVI video codec".to_string(),
    }
}

fn audio_codec_name(tag: u16) -> String {
    match tag {
        0x0002 => "MS ADPCM".to_string(),
        0x0011 => "IMA ADPCM".to_string(),
        0x0006 | 0x0007 => "G.711".to_string(),
        0x0031 => "GSM 6.10".to_string(),
        0x0160..=0x0163 => "Windows Media Audio".to_string(),
        0x2000 => "AC-3".to_string(),
        0x2001 => "DTS".to_string(),
        0x674F..=0x6751 | 0x676F..=0x6771 => "Vorbis".to_string(),
        _ => format!("audio codec 0x{tag:04X}"),
    }
}

fn vop_types(d: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 4 < d.len() {
        if d[i] == 0 && d[i + 1] == 0 && d[i + 2] == 1 && d[i + 3] == 0xB6 {
            out.push(d[i + 4] >> 6);
            i += 4;
        } else {
            i += 1;
        }
    }
    out
}

fn mpeg4_config(first: &[u8]) -> Vec<u8> {
    if !first.starts_with(&[0, 0, 1]) {
        return Vec::new();
    }
    let vop = first.windows(4).position(|w| w == [0, 0, 1, 0xB6]).unwrap_or(0);
    first[..vop].to_vec()
}

fn display_times(times: &[i64], order: &[usize]) -> Vec<i64> {
    let mut out = vec![0; times.len()];
    for (k, &i) in order.iter().enumerate() {
        out[i] = times[k];
    }
    out
}

fn h264_display_order(samples: &[Sample], length_size: usize, avcc: &[u8]) -> Option<Vec<usize>> {
    let sets = h264::parameter_sets(avcc);
    let sps: Vec<h264::Sps> = sets.iter().filter(|n| n.first().map(|b| b & 0x1F) == Some(7)).filter_map(|n| h264::parse_sps(n)).collect();
    let pps: Vec<(u32, u32)> = sets.iter().filter(|n| n.first().map(|b| b & 0x1F) == Some(8)).filter_map(|n| h264::pps_ids(n)).collect();
    let mut tracker = h264::PocTracker::new(&sps, pps);
    if !tracker.reorders() {
        return None;
    }
    let mut order = Vec::with_capacity(samples.len());
    let mut group: Vec<(i64, usize)> = Vec::new();
    for (i, s) in samples.iter().enumerate() {
        let (idr, poc) = tracker.picture(&h264::length_prefixed(&s.data, length_size))?;
        if idr && !group.is_empty() {
            group.sort_by_key(|&(poc, i)| (poc, i));
            order.extend(group.drain(..).map(|g| g.1));
        }
        group.push((poc, i));
    }
    group.sort_by_key(|&(poc, i)| (poc, i));
    order.extend(group.into_iter().map(|g| g.1));
    Some(order)
}

fn read_video<'a>(h: &StreamHeader<'a>, data: &[&'a [u8]]) -> Track<'a> {
    let f = h.format;
    let mut width = (le32(f, 4) as i32).unsigned_abs();
    let mut height = (le32(f, 8) as i32).unsigned_abs();
    let fourcc: [u8; 4] = f.get(16..20).and_then(|b| b.try_into().ok()).unwrap_or([0; 4]);
    let upper = fourcc.map(|b| b.to_ascii_uppercase());
    let extradata = f.get(40..).unwrap_or_default();

    let mut times = Vec::with_capacity(data.len());
    let mut present: Vec<&'a [u8]> = Vec::with_capacity(data.len());
    for (n, chunk) in data.iter().enumerate() {
        if !chunk.is_empty() {
            times.push(h.units_ns(h.start as u64 + n as u64));
            present.push(chunk);
        }
    }

    let (codec, samples) = match &upper {
        b"H264" | b"X264" | b"AVC1" | b"DAVC" | b"VSSH" | b"AVC " => {
            let (avcc, length_size, mut samples) = if extradata.first() == Some(&1) && extradata.len() >= 7 {
                let length_size = ((extradata[4] & 3) + 1) as usize;
                let samples: Vec<Sample> = present
                    .iter()
                    .map(|d| Sample { pts_ns: 0, keyframe: false, data: Cow::Borrowed(*d) })
                    .collect();
                (Some(extradata.to_vec()), length_size, samples)
            } else {
                let mut sps: Vec<Vec<u8>> = Vec::new();
                let mut pps: Vec<Vec<u8>> = Vec::new();
                let keep = |nal: &[u8], sps: &mut Vec<Vec<u8>>, pps: &mut Vec<Vec<u8>>| {
                    let list = match nal.first().map(|b| b & 0x1F) {
                        Some(7) => sps,
                        Some(8) => pps,
                        _ => return,
                    };
                    if !list.iter().any(|known| known.as_slice() == nal) {
                        list.push(nal.to_vec());
                    }
                };
                for nal in h264::split_annexb(extradata) {
                    keep(nal, &mut sps, &mut pps);
                }
                let mut samples = Vec::with_capacity(present.len());
                for d in &present {
                    let nals = if d.starts_with(&[0, 0, 1]) || d.starts_with(&[0, 0, 0, 1]) {
                        h264::split_annexb(d)
                    } else {
                        h264::length_prefixed(d, 4)
                    };
                    let mut out = Vec::with_capacity(d.len() + 16);
                    for nal in nals {
                        match nal.first().map(|b| b & 0x1F) {
                            Some(7 | 8) => keep(nal, &mut sps, &mut pps),
                            Some(9) | None => {}
                            _ => {
                                out.extend_from_slice(&(nal.len() as u32).to_be_bytes());
                                out.extend_from_slice(nal);
                            }
                        }
                    }
                    samples.push(Sample { pts_ns: 0, keyframe: false, data: Cow::Owned(out) });
                }
                (h264::build_avcc(&sps, &pps), 4, samples)
            };
            for s in samples.iter_mut() {
                s.keyframe = crate::mp4::h264_sample_is_idr(&s.data, length_size);
            }
            if !samples.iter().any(|s| s.keyframe) {
                if let Some(first) = samples.first_mut() {
                    first.keyframe = true;
                }
            }
            match avcc {
                Some(avcc) => {
                    if let Some((w, hgt)) = h264::sps_dimensions(&avcc) {
                        (width, height) = (w, hgt);
                    }
                    let pts = match h264_display_order(&samples, length_size, &avcc) {
                        Some(order) => display_times(&times, &order),
                        None => times.clone(),
                    };
                    for (s, t) in samples.iter_mut().zip(pts) {
                        s.pts_ns = t;
                    }
                    (Codec::H264 { avcc }, samples)
                }
                None => (Codec::Unsupported("H.264 without a decoder configuration".to_string()), samples),
            }
        }
        b"XVID" | b"DIVX" | b"DX50" | b"FMP4" | b"MP4V" | b"3IV2" | b"M4S2" | b"BLZ0" | b"DXGM" | b"RMP4" | b"SEDG"
        | b"WV1F" | b"UMP4" | b"XVIX" | b"DIV5" | b"DIV6" | b"LMP4" | b"SMP4" | b"NDIG" | b"PVMM" | b"M4CC" | b"MP4S" => {
            let config = if extradata.is_empty() { present.first().map(|d| mpeg4_config(d)).unwrap_or_default() } else { extradata.to_vec() };
            let types: Vec<Vec<u8>> = present.iter().map(|d| vop_types(d)).collect();
            let packed = types.iter().any(|t| t.len() > 1);
            let mut samples: Vec<Sample> = present
                .iter()
                .zip(&types)
                .map(|(d, t)| Sample { pts_ns: 0, keyframe: t.first() == Some(&0), data: Cow::Borrowed(*d) })
                .collect();
            let reorder = !packed && types.iter().any(|t| t.first() == Some(&2));
            let pts = if reorder {
                let mut order = Vec::with_capacity(types.len());
                let mut pending: Option<usize> = None;
                for (i, t) in types.iter().enumerate() {
                    if t.first() == Some(&2) {
                        order.push(i);
                    } else if let Some(p) = pending.replace(i) {
                        order.push(p);
                    }
                }
                order.extend(pending);
                display_times(&times, &order)
            } else {
                times.clone()
            };
            for (s, t) in samples.iter_mut().zip(pts) {
                s.pts_ns = t;
            }
            if let Some(first) = samples.first_mut() {
                first.keyframe = true;
            }
            (Codec::Mpeg4 { config }, samples)
        }
        b"MJPG" | b"AVRN" | b"LJPG" | b"JPGL" | b"DMB1" | b"MJPA" => {
            let samples = present
                .iter()
                .zip(&times)
                .map(|(d, &t)| Sample { pts_ns: t, keyframe: true, data: Cow::Borrowed(*d) })
                .collect();
            (Codec::Mjpeg, samples)
        }
        _ => {
            let samples = present
                .iter()
                .zip(&times)
                .enumerate()
                .map(|(i, (d, &t))| Sample { pts_ns: t, keyframe: i == 0, data: Cow::Borrowed(*d) })
                .collect();
            (Codec::Unsupported(video_codec_name(&upper)), samples)
        }
    };
    Track { video: true, codec, width, height, sample_rate: 0, channels: 0, codec_delay_ns: 0, samples }
}

fn read_audio<'a>(h: &StreamHeader<'a>, data: &[&'a [u8]]) -> Track<'a> {
    let f = h.format;
    let mut tag = le16(f, 0);
    let mut channels = le16(f, 2).max(1);
    let mut rate = le32(f, 4);
    let mut block_align = le16(f, 12) as usize;
    let mut bits = le16(f, 14);
    let extra_len = le16(f, 16) as usize;
    let extra = f.get(18..18 + extra_len).or_else(|| f.get(18..)).unwrap_or_default();
    if tag == 0xFFFE && extra.len() >= 22 {
        tag = le16(extra, 6);
        let valid = le16(extra, 0);
        if valid > 0 && valid < bits && tag == 1 {
            bits = bits.max(valid);
        }
    }
    let start_ns = h.units_ns(h.start as u64);
    let ns = |samples: u64, rate: u32| start_ns + (samples as u128 * 1_000_000_000 / rate.max(1) as u128) as i64;

    let (codec, samples): (Codec, Vec<Sample<'a>>) = match tag {
        0x0050 | 0x0055 => {
            let joined: Vec<u8> = data.concat();
            let frames = split_mpa(&joined);
            let first = frames.first().and_then(|&(a, _)| mpa_frame(&joined[a..]));
            let codec = match first.as_ref().map(|f| f.layer) {
                Some(3) => Codec::Mp3,
                Some(2) => Codec::Mp2,
                Some(_) => Codec::Unsupported("MPEG audio layer I".to_string()),
                None => Codec::Unsupported("damaged MPEG audio".to_string()),
            };
            if let Some(first) = &first {
                (rate, channels) = (first.rate, first.channels);
            }
            let per_frame = first.map(|f| f.samples as u64).unwrap_or(1152);
            let samples = frames
                .iter()
                .enumerate()
                .map(|(k, &(a, b))| Sample { pts_ns: ns(k as u64 * per_frame, rate), keyframe: true, data: Cow::Owned(joined[a..b].to_vec()) })
                .collect();
            (codec, samples)
        }
        0x0001 | 0x0003 => {
            let float = tag == 3;
            let valid = if float { matches!(bits, 32 | 64) } else { matches!(bits, 8 | 16 | 24 | 32) };
            if block_align == 0 {
                block_align = channels as usize * bits as usize / 8;
            }
            let mut offset = 0u64;
            let samples = data
                .iter()
                .filter(|d| !d.is_empty())
                .map(|d| {
                    let s = Sample { pts_ns: ns(offset / block_align.max(1) as u64, rate), keyframe: true, data: Cow::Borrowed(*d) };
                    offset += d.len() as u64;
                    s
                })
                .collect();
            let codec = if valid { Codec::Pcm { bits, float } } else { Codec::Unsupported(format!("{bits}-bit PCM")) };
            (codec, samples)
        }
        0x00FF if extra.len() >= 2 => {
            let asc = extra.to_vec();
            if let Some((_, index, ch)) = demux::adts_params(&asc) {
                rate = demux::AAC_SAMPLE_RATES.get(index as usize).copied().unwrap_or(rate);
                if ch > 0 {
                    channels = ch as u16;
                }
            }
            let samples = data
                .iter()
                .filter(|d| !d.is_empty())
                .enumerate()
                .map(|(k, d)| Sample { pts_ns: ns(k as u64 * 1024, rate), keyframe: true, data: Cow::Borrowed(*d) })
                .collect();
            (Codec::Aac { asc }, samples)
        }
        0x1600 | 0x706D => {
            let joined: Vec<u8> = data.concat();
            let (frames, asc) = adts_frames(&joined);
            match asc {
                Some(asc) => {
                    if let Some((_, index, ch)) = demux::adts_params(&asc) {
                        rate = demux::AAC_SAMPLE_RATES.get(index as usize).copied().unwrap_or(rate);
                        if ch > 0 {
                            channels = ch as u16;
                        }
                    }
                    let samples = frames
                        .iter()
                        .enumerate()
                        .map(|(k, &(a, b))| Sample { pts_ns: ns(k as u64 * 1024, rate), keyframe: true, data: Cow::Owned(joined[a..b].to_vec()) })
                        .collect();
                    (Codec::Aac { asc }, samples)
                }
                None => (Codec::Unsupported("damaged AAC".to_string()), Vec::new()),
            }
        }
        _ => {
            let samples = data
                .iter()
                .filter(|d| !d.is_empty())
                .enumerate()
                .map(|(k, d)| Sample {
                    pts_ns: if h.sample_size == 0 { h.units_ns(h.start as u64 + k as u64) } else { start_ns },
                    keyframe: true,
                    data: Cow::Borrowed(*d),
                })
                .collect();
            (Codec::Unsupported(audio_codec_name(tag)), samples)
        }
    };
    Track { video: false, codec, width: 0, height: 0, sample_rate: rate, channels, codec_delay_ns: 0, samples }
}

pub fn read(raw: &[u8]) -> Result<Vec<Track<'_>>, String> {
    if raw.len() < 12 || &raw[0..4] != b"RIFF" || &raw[8..12] != b"AVI " {
        return Err("This doesn't look like a valid AVI file.".to_string());
    }
    let mut streams: Vec<StreamHeader> = Vec::new();
    let mut data: Vec<Vec<&[u8]>> = Vec::new();
    for (id, body) in chunks(raw) {
        if &id != b"RIFF" || body.len() < 4 || !matches!(&body[0..4], b"AVI " | b"AVIX") {
            continue;
        }
        for (sub, content) in chunks(&body[4..]) {
            if let Some(hdrl) = list(sub, content, b"hdrl") {
                if streams.is_empty() {
                    streams = headers(hdrl);
                    data = streams.iter().map(|_| Vec::new()).collect();
                }
            } else if let Some(movi) = list(sub, content, b"movi") {
                scan_movi(movi, &mut data);
            }
        }
    }
    if streams.is_empty() {
        return Err("This AVI file has no stream headers.".to_string());
    }

    let mut tracks = Vec::new();
    if let Some(i) = streams.iter().position(|s| &s.kind == b"vids") {
        if data[i].iter().any(|d| !d.is_empty()) {
            tracks.push(read_video(&streams[i], &data[i]));
        }
    }
    if let Some(i) = streams.iter().position(|s| &s.kind == b"auds") {
        if data[i].iter().any(|d| !d.is_empty()) {
            let track = read_audio(&streams[i], &data[i]);
            if !track.samples.is_empty() {
                tracks.push(track);
            }
        }
    }
    if tracks.is_empty() {
        return Err("No audio or video frames were found in this AVI file.".to_string());
    }
    Ok(tracks)
}

pub fn duration_ms(head: &[u8]) -> Option<u64> {
    if head.len() < 12 || &head[0..4] != b"RIFF" || &head[8..12] != b"AVI " {
        return None;
    }
    let hdrl = chunks(&head[12..]).into_iter().find_map(|(id, body)| list(id, body, b"hdrl"))?;
    let streams = headers(hdrl);
    let stream = streams.iter().find(|s| &s.kind == b"vids").or_else(|| streams.iter().find(|s| &s.kind == b"auds"))?;
    let ms = stream.units_ns(stream.start as u64 + stream.length as u64) / 1_000_000;
    (ms > 0).then_some(ms as u64)
}

struct OutChunk<'a> {
    time: i64,
    data: Cow<'a, [u8]>,
    key: bool,
}

struct OutStream<'a> {
    video: bool,
    handler: [u8; 4],
    scale: u32,
    rate: u32,
    start: u32,
    length: u32,
    sample_size: u32,
    format: Vec<u8>,
    chunks: Vec<OutChunk<'a>>,
}

impl OutStream<'_> {
    fn id(&self, index: usize) -> [u8; 4] {
        let kind = if self.video { *b"dc" } else { *b"wb" };
        [b'0' + (index / 10) as u8, b'0' + (index % 10) as u8, kind[0], kind[1]]
    }

    fn duration_units(&self, chunks: &[usize]) -> u32 {
        if self.sample_size > 0 {
            (chunks.iter().map(|&c| self.chunks[c].data.len() as u64).sum::<u64>() / self.sample_size as u64) as u32
        } else {
            chunks.len() as u32
        }
    }

    fn max_chunk(&self) -> u32 {
        self.chunks.iter().map(|c| c.data.len() as u32).max().unwrap_or(0)
    }
}

const FRAME_RATES: [(u32, u32); 17] = [
    (1001, 24000),
    (1, 24),
    (1, 25),
    (1001, 30000),
    (1, 30),
    (1, 50),
    (1001, 60000),
    (1, 60),
    (1, 15),
    (1, 10),
    (1, 12),
    (1, 20),
    (1, 48),
    (1, 90),
    (1, 100),
    (1, 120),
    (1, 144),
];

fn snap_rate(duration_ns: f64) -> Option<(u32, u32)> {
    let error = |&(scale, rate): &(u32, u32)| {
        let d = scale as f64 * 1e9 / rate as f64;
        ((d - duration_ns) / d).abs()
    };
    FRAME_RATES.into_iter().filter(|r| error(r) < 0.005).min_by(|a, b| error(a).total_cmp(&error(b)))
}

fn frame_rate(sorted: &[i64]) -> (u32, u32) {
    let mut diffs: Vec<i64> = sorted.windows(2).map(|w| w[1] - w[0]).filter(|&d| d > 0).collect();
    if diffs.is_empty() {
        return (1, 30);
    }
    diffs.sort_unstable();
    let median = diffs[diffs.len() / 2] as f64;
    let average = (sorted[sorted.len() - 1] - sorted[0]) as f64 / (sorted.len() - 1) as f64;
    snap_rate(median).or_else(|| snap_rate(average)).unwrap_or_else(|| {
        let d = if (average - median).abs() / median < 0.1 { average } else { median };
        (1000, (1e12 / d).round().clamp(1.0, u32::MAX as f64) as u32)
    })
}

fn bitmap_info(width: u32, height: u32, fourcc: &[u8; 4], extradata: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(40 + extradata.len());
    f.extend_from_slice(&(40 + extradata.len() as u32).to_le_bytes());
    f.extend_from_slice(&width.to_le_bytes());
    f.extend_from_slice(&height.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&24u16.to_le_bytes());
    f.extend_from_slice(fourcc);
    f.extend_from_slice(&(width * height * 3).to_le_bytes());
    f.extend_from_slice(&[0; 16]);
    f.extend_from_slice(extradata);
    f
}

fn wave_format(tag: u16, channels: u16, rate: u32, avg_bytes: u32, block_align: u16, bits: u16, extra: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(18 + extra.len());
    f.extend_from_slice(&tag.to_le_bytes());
    f.extend_from_slice(&channels.to_le_bytes());
    f.extend_from_slice(&rate.to_le_bytes());
    f.extend_from_slice(&avg_bytes.to_le_bytes());
    f.extend_from_slice(&block_align.to_le_bytes());
    f.extend_from_slice(&bits.to_le_bytes());
    f.extend_from_slice(&(extra.len() as u16).to_le_bytes());
    f.extend_from_slice(extra);
    f
}

fn video_stream<'a>(t: &'a Track, base: i64) -> Result<OutStream<'a>, String> {
    let pts: Vec<i64> = t.samples.iter().map(|s| s.pts_ns).collect();
    let mut sorted = pts.clone();
    sorted.sort_unstable();
    let mut dts: Vec<i64> = Vec::with_capacity(sorted.len());
    for (i, &s) in sorted.iter().enumerate() {
        dts.push(if i > 0 && s <= dts[i - 1] { dts[i - 1] + 1 } else { s });
    }
    let (scale, rate) = frame_rate(&sorted);
    let unit = scale as i128 * 1_000_000_000;
    let slot = |ns: i64| (((ns - base).max(0) as i128 * rate as i128 + unit / 2) / unit) as i64;
    let slot_time = |s: i64| base + (s as i128 * unit / rate as i128) as i64;

    let (handler, extradata, header) = match &t.codec {
        Codec::H264 { avcc } => {
            let sets = h264::parameter_sets(avcc);
            if sets.is_empty() {
                return Err("This H.264 track has no usable decoder configuration, so it can't be converted.".to_string());
            }
            let mut annexb = Vec::new();
            for nal in sets {
                annexb.extend_from_slice(&[0, 0, 0, 1]);
                annexb.extend_from_slice(nal);
            }
            (*b"H264", annexb.clone(), annexb)
        }
        Codec::Mpeg4 { config } => (*b"XVID", config.clone(), config.clone()),
        Codec::Mjpeg => (*b"MJPG", Vec::new(), Vec::new()),
        other => return Err(format!("AVI output can't hold {} video without re-encoding.", other.name())),
    };
    let length_size = match &t.codec {
        Codec::H264 { avcc } if avcc.len() > 4 => ((avcc[4] & 3) + 1) as usize,
        _ => 4,
    };
    let idr: Vec<bool> = match &t.codec {
        Codec::H264 { .. } => t.samples.iter().map(|s| crate::mp4::h264_sample_is_idr(&s.data, length_size)).collect(),
        _ => t.samples.iter().map(|s| s.keyframe).collect(),
    };
    let use_idr = idr.iter().any(|&k| k);

    let start = dts.first().map(|&d| slot(d)).unwrap_or(0);
    let mut last = start - 1;
    let mut chunks = Vec::with_capacity(t.samples.len());
    for (i, sample) in t.samples.iter().enumerate() {
        let s = slot(dts[i]).max(last + 1);
        for gap in last + 1..s {
            chunks.push(OutChunk { time: slot_time(gap), data: Cow::Borrowed(&[][..]), key: false });
        }
        let key = i == 0 || if use_idr { idr[i] } else { sample.keyframe };
        let data: Cow<[u8]> = match &t.codec {
            Codec::H264 { .. } => {
                let nals = h264::length_prefixed(&sample.data, length_size);
                let mut out = Vec::with_capacity(sample.data.len() + if key { header.len() } else { 0 } + 16);
                if key {
                    out.extend_from_slice(&header);
                }
                for nal in nals {
                    out.extend_from_slice(&[0, 0, 0, 1]);
                    out.extend_from_slice(nal);
                }
                Cow::Owned(out)
            }
            Codec::Mpeg4 { .. } if i == 0 && !header.is_empty() && !sample.data.starts_with(&header[..header.len().min(4)]) => {
                let mut out = header.clone();
                out.extend_from_slice(&sample.data);
                Cow::Owned(out)
            }
            _ => Cow::Borrowed(&sample.data[..]),
        };
        chunks.push(OutChunk { time: slot_time(s), data, key });
        last = s;
    }
    Ok(OutStream {
        video: true,
        handler,
        scale,
        rate,
        start: start.max(0) as u32,
        length: (last - start + 1).max(0) as u32,
        sample_size: 0,
        format: bitmap_info(t.width, t.height, &handler, &extradata),
        chunks,
    })
}

fn audio_first_time(t: &Track) -> i64 {
    t.samples.first().map(|s| s.pts_ns - t.codec_delay_ns).unwrap_or(0)
}

fn audio_stream<'a>(t: &'a Track, joined: &'a [u8], base: i64) -> Result<OutStream<'a>, String> {
    let first_time = audio_first_time(t);
    match &t.codec {
        Codec::Mp3 | Codec::Mp2 => {
            let frames = split_mpa(joined);
            let first = frames
                .first()
                .and_then(|&(a, _)| mpa_frame(&joined[a..]))
                .ok_or("The MP3 audio couldn't be read, so it can't be written to AVI.")?;
            let (per_frame, rate) = (first.samples, first.rate);
            let unit = per_frame as i128 * 1_000_000_000;
            let start = (((first_time - base).max(0) as i128 * rate as i128 + unit / 2) / unit) as u32;
            let chunks: Vec<OutChunk> = frames
                .iter()
                .enumerate()
                .map(|(k, &(a, b))| OutChunk {
                    time: first_time + (k as i128 * unit / rate as i128) as i64,
                    data: Cow::Borrowed(&joined[a..b]),
                    key: true,
                })
                .collect();
            let total: u64 = frames.iter().map(|&(a, b)| (b - a) as u64).sum();
            let seconds = (frames.len() as f64 * per_frame as f64 / rate as f64).max(1e-3);
            let avg = (total as f64 / seconds).round() as u32;
            let mut extra = Vec::new();
            let tag = if first.layer == 3 {
                extra.extend_from_slice(&1u16.to_le_bytes());
                extra.extend_from_slice(&2u32.to_le_bytes());
                extra.extend_from_slice(&(first.len as u16).to_le_bytes());
                extra.extend_from_slice(&1u16.to_le_bytes());
                extra.extend_from_slice(&1393u16.to_le_bytes());
                0x55
            } else {
                let mode: u16 = if first.channels == 1 { 8 } else { 1 };
                extra.extend_from_slice(&2u16.to_le_bytes());
                extra.extend_from_slice(&first.bitrate.to_le_bytes());
                extra.extend_from_slice(&mode.to_le_bytes());
                extra.extend_from_slice(&0u16.to_le_bytes());
                extra.extend_from_slice(&1u16.to_le_bytes());
                extra.extend_from_slice(&if rate >= 32000 { 0x18u16 } else { 0x08 }.to_le_bytes());
                extra.extend_from_slice(&[0; 8]);
                0x50
            };
            Ok(OutStream {
                video: false,
                handler: [0; 4],
                scale: per_frame,
                rate,
                start,
                length: chunks.len() as u32,
                sample_size: 0,
                format: wave_format(tag, first.channels, rate, avg, per_frame as u16, 0, &extra),
                chunks,
            })
        }
        Codec::Pcm { bits, float } => {
            let channels = t.channels.max(1);
            let block_align = (channels as u32 * *bits as u32 / 8).max(1);
            let rate = t.sample_rate.max(1);
            let start = (((first_time - base).max(0) as i128 * rate as i128 + 500_000_000) / 1_000_000_000) as u32;
            let mut offset = 0u64;
            let chunks: Vec<OutChunk> = t
                .samples
                .iter()
                .map(|s| {
                    let c = OutChunk {
                        time: first_time + ((offset / block_align as u64) as i128 * 1_000_000_000 / rate as i128) as i64,
                        data: Cow::Borrowed(&s.data[..]),
                        key: true,
                    };
                    offset += s.data.len() as u64;
                    c
                })
                .collect();
            let tag = if *float { 3u16 } else { 1 };
            let format = if channels > 2 || (*bits > 16 && !*float) {
                let mask: u32 = match channels {
                    1 => 0x4,
                    2 => 0x3,
                    3 => 0x7,
                    4 => 0x33,
                    5 => 0x37,
                    6 => 0x3F,
                    7 => 0x13F,
                    8 => 0x63F,
                    _ => 0,
                };
                let mut extra = Vec::with_capacity(22);
                extra.extend_from_slice(&bits.to_le_bytes());
                extra.extend_from_slice(&mask.to_le_bytes());
                extra.extend_from_slice(&tag.to_le_bytes());
                extra.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71]);
                wave_format(0xFFFE, channels, rate, rate * block_align, block_align as u16, *bits, &extra)
            } else {
                wave_format(tag, channels, rate, rate * block_align, block_align as u16, *bits, &[])
            };
            Ok(OutStream {
                video: false,
                handler: [0; 4],
                scale: block_align,
                rate: rate * block_align,
                start,
                length: (offset / block_align as u64) as u32,
                sample_size: block_align,
                format,
                chunks,
            })
        }
        other => Err(format!("AVI output can't hold {} audio without re-encoding.", other.name())),
    }
}

fn chunk_header(id: &[u8; 4], len: usize) -> [u8; 8] {
    let mut h = [0u8; 8];
    h[0..4].copy_from_slice(id);
    h[4..8].copy_from_slice(&(len as u32).to_le_bytes());
    h
}

fn push_chunk(out: &mut Vec<u8>, id: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&chunk_header(id, body.len()));
    out.extend_from_slice(body);
    if body.len() % 2 == 1 {
        out.push(0);
    }
}

fn push_list(out: &mut Vec<u8>, kind: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&chunk_header(b"LIST", body.len() + 4));
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
}

struct Segment {
    items: std::ops::Range<usize>,
    per_stream: Vec<Vec<usize>>,
    body: u64,
}

fn padded(len: usize) -> u64 {
    8 + len as u64 + (len as u64 & 1)
}

fn std_index_len(entries: usize) -> u64 {
    if entries == 0 { 0 } else { 8 + STD_INDEX_HEADER + 8 * entries as u64 }
}

fn plan(streams: &[OutStream], items: &[(i64, usize, usize)], first_overhead: u64, limit: u64) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut i = 0usize;
    loop {
        let first = segments.is_empty();
        let overhead = if first { first_overhead } else { 24 };
        let mut counts = vec![0usize; streams.len()];
        let mut per_stream: Vec<Vec<usize>> = vec![Vec::new(); streams.len()];
        let mut body = 0u64;
        let start = i;
        while i < items.len() {
            let (_, s, c) = items[i];
            let size = padded(streams[s].chunks[c].data.len());
            counts[s] += 1;
            let total_entries: usize = counts.iter().sum();
            let trailer: u64 = counts.iter().map(|&n| std_index_len(n)).sum::<u64>() + if first { 8 + 16 * total_entries as u64 } else { 0 };
            if i > start && overhead + body + size + trailer > limit {
                counts[s] -= 1;
                break;
            }
            body += size;
            per_stream[s].push(c);
            i += 1;
        }
        segments.push(Segment { items: start..i, per_stream, body });
        if i >= items.len() {
            return segments;
        }
    }
}

struct SuperEntry {
    offset: u64,
    size: u32,
    duration: u32,
}

fn build_hdrl(streams: &[OutStream], capacity: usize, entries: &[Vec<SuperEntry>], first_frames: u32, width: u32, height: u32) -> Vec<u8> {
    let video = streams.iter().find(|s| s.video);
    let micro_per_frame = video.map(|v| (v.scale as u64 * 1_000_000 / v.rate.max(1) as u64) as u32).unwrap_or(0);
    let total_bytes: u64 = streams.iter().flat_map(|s| s.chunks.iter()).map(|c| c.data.len() as u64).sum();
    let seconds = streams
        .iter()
        .map(|s| (s.start as u64 + s.length as u64) as f64 * s.scale as f64 / s.rate.max(1) as f64)
        .fold(0.0, f64::max)
        .max(1e-3);
    let max_chunk = streams.iter().map(|s| s.max_chunk()).max().unwrap_or(0);

    let mut avih = Vec::with_capacity(56);
    for v in [
        micro_per_frame,
        (total_bytes as f64 / seconds).round() as u32,
        0,
        0x10 | 0x100 | 0x800,
        first_frames,
        0,
        streams.len() as u32,
        max_chunk + 8,
        width,
        height,
        0,
        0,
        0,
        0,
    ] {
        avih.extend_from_slice(&v.to_le_bytes());
    }

    let mut hdrl = Vec::new();
    push_chunk(&mut hdrl, b"avih", &avih);
    for (index, s) in streams.iter().enumerate() {
        let mut strh = Vec::with_capacity(56);
        strh.extend_from_slice(if s.video { b"vids" } else { b"auds" });
        strh.extend_from_slice(&s.handler);
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        for v in [s.scale, s.rate, s.start, s.length, s.max_chunk(), u32::MAX, s.sample_size] {
            strh.extend_from_slice(&v.to_le_bytes());
        }
        if s.video {
            for v in [0u16, 0, width.min(u16::MAX as u32) as u16, height.min(u16::MAX as u32) as u16] {
                strh.extend_from_slice(&v.to_le_bytes());
            }
        } else {
            strh.extend_from_slice(&[0; 8]);
        }

        let mut indx = Vec::with_capacity(SUPER_INDEX_HEADER + 16 * capacity);
        indx.extend_from_slice(&4u16.to_le_bytes());
        indx.push(0);
        indx.push(0);
        indx.extend_from_slice(&(entries[index].len() as u32).to_le_bytes());
        indx.extend_from_slice(&s.id(index));
        indx.extend_from_slice(&[0; 12]);
        for e in &entries[index] {
            indx.extend_from_slice(&e.offset.to_le_bytes());
            indx.extend_from_slice(&e.size.to_le_bytes());
            indx.extend_from_slice(&e.duration.to_le_bytes());
        }
        indx.resize(SUPER_INDEX_HEADER + 16 * capacity, 0);

        let mut strl = Vec::new();
        push_chunk(&mut strl, b"strh", &strh);
        push_chunk(&mut strl, b"strf", &s.format);
        push_chunk(&mut strl, b"indx", &indx);
        push_list(&mut hdrl, b"strl", &strl);
    }
    let mut dmlh = vec![0u8; 248];
    dmlh[0..4].copy_from_slice(&video.map(|v| v.length).unwrap_or(0).to_le_bytes());
    let mut odml = Vec::new();
    push_chunk(&mut odml, b"dmlh", &dmlh);
    push_list(&mut hdrl, b"odml", &odml);

    let mut out = Vec::with_capacity(hdrl.len() + 12);
    push_list(&mut out, b"hdrl", &hdrl);
    out
}

struct Counted<'w, W: Write> {
    inner: &'w mut W,
    pos: u64,
}

impl<W: Write> Counted<'_, W> {
    fn put(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.inner.write_all(bytes).map_err(|e| format!("Couldn't write the output file: {e}"))?;
        self.pos += bytes.len() as u64;
        Ok(())
    }
}

pub fn write<W: Write>(tracks: &[Track], out: &mut W) -> Result<(), String> {
    write_with_limit(tracks, out, SEGMENT_LIMIT)
}

pub fn write_with_limit<W: Write>(tracks: &[Track], out: &mut W, limit: u64) -> Result<(), String> {
    let video = tracks.iter().find(|t| t.video && !t.samples.is_empty());
    let audio = tracks.iter().find(|t| !t.video && !t.samples.is_empty());
    if video.is_none() && audio.is_none() {
        return Err("No audio or video frames were found in this file.".to_string());
    }
    let base = video
        .and_then(|t| t.samples.iter().map(|s| s.pts_ns).min())
        .into_iter()
        .chain(audio.map(audio_first_time))
        .min()
        .unwrap_or(0);

    let joined: Vec<u8> = match audio.map(|t| &t.codec) {
        Some(Codec::Mp3 | Codec::Mp2) => audio.unwrap().samples.iter().flat_map(|s| s.data.iter().copied()).collect(),
        _ => Vec::new(),
    };
    let mut streams = Vec::new();
    if let Some(t) = video {
        streams.push(video_stream(t, base)?);
    }
    if let Some(t) = audio {
        streams.push(audio_stream(t, &joined, base)?);
    }
    let (width, height) = video.map(|t| (t.width, t.height)).unwrap_or((0, 0));

    let mut items: Vec<(i64, usize, usize)> = streams
        .iter()
        .enumerate()
        .flat_map(|(s, stream)| stream.chunks.iter().enumerate().map(move |(c, chunk)| (chunk.time, s, c)))
        .collect();
    items.sort_by_key(|&(time, s, c)| (time, s, c));

    let empty: Vec<Vec<SuperEntry>> = streams.iter().map(|_| Vec::new()).collect();
    let mut capacity = 1usize;
    let (segments, hdrl_len) = loop {
        let hdrl_len = build_hdrl(&streams, capacity, &empty, 0, width, height).len() as u64;
        let segments = plan(&streams, &items, 12 + hdrl_len + 12, limit);
        if segments.len() <= capacity {
            break (segments, hdrl_len);
        }
        capacity = segments.len();
    };

    let mut layout = Vec::with_capacity(segments.len());
    let mut entries: Vec<Vec<SuperEntry>> = streams.iter().map(|_| Vec::new()).collect();
    let mut pos = 0u64;
    for (k, seg) in segments.iter().enumerate() {
        let riff_start = pos;
        let movi = riff_start + 12 + if k == 0 { hdrl_len } else { 0 } + 8;
        let mut ix_pos = movi + 4 + seg.body;
        let mut ix_total = 0u64;
        for (s, list) in seg.per_stream.iter().enumerate() {
            let len = std_index_len(list.len());
            if len > 0 {
                entries[s].push(SuperEntry { offset: ix_pos, size: len as u32, duration: streams[s].duration_units(list) });
            }
            ix_pos += len;
            ix_total += len;
        }
        let idx1 = if k == 0 { 8 + 16 * seg.items.len() as u64 } else { 0 };
        let end = movi + 4 + seg.body + ix_total + idx1;
        layout.push((riff_start, movi, ix_total, end));
        pos = end;
    }

    let first_frames = segments.first().and_then(|seg| streams.iter().position(|s| s.video).map(|v| seg.per_stream[v].len() as u32)).unwrap_or(0);
    let hdrl = build_hdrl(&streams, capacity, &entries, first_frames, width, height);
    debug_assert_eq!(hdrl.len() as u64, hdrl_len);

    let mut w = Counted { inner: out, pos: 0 };
    for (k, seg) in segments.iter().enumerate() {
        let (riff_start, movi, ix_total, end) = layout[k];
        w.put(&chunk_header(b"RIFF", (end - riff_start - 8) as usize))?;
        w.put(if k == 0 { b"AVI " } else { b"AVIX" })?;
        if k == 0 {
            w.put(&hdrl)?;
        }
        w.put(&chunk_header(b"LIST", (4 + seg.body + ix_total) as usize))?;
        w.put(b"movi")?;
        let mut std_entries: Vec<Vec<(u32, u32)>> = streams.iter().map(|_| Vec::new()).collect();
        let mut legacy: Vec<u8> = Vec::with_capacity(if k == 0 { 16 * seg.items.len() } else { 0 });
        for &(_, s, c) in &items[seg.items.clone()] {
            let stream = &streams[s];
            let chunk = &stream.chunks[c];
            let id = stream.id(s);
            let rel = w.pos - movi;
            w.put(&chunk_header(&id, chunk.data.len()))?;
            w.put(&chunk.data)?;
            if chunk.data.len() % 2 == 1 {
                w.put(&[0])?;
            }
            let key = chunk.key || !stream.video;
            std_entries[s].push(((rel + 8) as u32, chunk.data.len() as u32 | if key { 0 } else { NOT_KEYFRAME }));
            if k == 0 {
                legacy.extend_from_slice(&id);
                legacy.extend_from_slice(&(if key { AVIIF_KEYFRAME } else { 0 }).to_le_bytes());
                legacy.extend_from_slice(&(rel as u32).to_le_bytes());
                legacy.extend_from_slice(&(chunk.data.len() as u32).to_le_bytes());
            }
        }
        for (s, list) in std_entries.iter().enumerate() {
            if list.is_empty() {
                continue;
            }
            let mut ix = Vec::with_capacity(STD_INDEX_HEADER as usize + 8 * list.len());
            ix.extend_from_slice(&2u16.to_le_bytes());
            ix.push(0);
            ix.push(1);
            ix.extend_from_slice(&(list.len() as u32).to_le_bytes());
            ix.extend_from_slice(&streams[s].id(s));
            ix.extend_from_slice(&movi.to_le_bytes());
            ix.extend_from_slice(&0u32.to_le_bytes());
            for &(offset, size) in list {
                ix.extend_from_slice(&offset.to_le_bytes());
                ix.extend_from_slice(&size.to_le_bytes());
            }
            let id = [b'i', b'x', b'0' + (s / 10) as u8, b'0' + (s % 10) as u8];
            w.put(&chunk_header(&id, ix.len()))?;
            w.put(&ix)?;
        }
        if k == 0 {
            w.put(&chunk_header(b"idx1", legacy.len()))?;
            w.put(&legacy)?;
        }
        if w.pos != end {
            return Err("The AVI layout came out inconsistent, so the file wasn't written.".to_string());
        }
    }
    Ok(())
}