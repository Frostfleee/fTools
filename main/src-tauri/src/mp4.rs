use std::io::Write;

use crate::ebml::{self, TrackMeta};

const ID_BLOCK_GROUP: u32 = 0xA0;
const ID_BLOCK: u32 = 0xA1;
const ID_REFERENCE_BLOCK: u32 = 0xFB;
const UNKNOWN_SIZE: u64 = u64::MAX;

const MOVIE_TIMESCALE: u32 = 1000;
const VIDEO_TIMESCALE: u32 = 90_000;
const CHUNK_SECONDS_DIVISOR: u32 = 2;

struct Frame<'a> {
    track: u64,
    ticks: i64,
    keyframe: bool,
    data: &'a [u8],
}

fn read_id(d: &[u8], p: &mut usize) -> Option<u32> {
    let b0 = *d.get(*p)?;
    let len = b0.leading_zeros() as usize + 1;
    if len > 4 || *p + len > d.len() {
        return None;
    }
    let id = d[*p..*p + len].iter().fold(0u32, |acc, &b| (acc << 8) | b as u32);
    *p += len;
    Some(id)
}

fn read_vint(d: &[u8], p: &mut usize) -> Option<(u64, usize)> {
    let b0 = *d.get(*p)?;
    let len = b0.leading_zeros() as usize + 1;
    if len > 8 || *p + len > d.len() {
        return None;
    }
    let mut v = (b0 as u64) & (0xFFu64 >> len);
    for &b in &d[*p + 1..*p + len] {
        v = (v << 8) | b as u64;
    }
    *p += len;
    Some((v, len))
}

fn read_size(d: &[u8], p: &mut usize) -> Option<u64> {
    let (v, len) = read_vint(d, p)?;
    Some(if v == (1u64 << (7 * len)) - 1 { UNKNOWN_SIZE } else { v })
}

fn parse_block<'a>(
    body: &'a [u8],
    keyframe_override: Option<bool>,
    cluster_ts: i64,
    wanted: &[u64],
    frames: &mut Vec<Frame<'a>>,
) -> Result<(), String> {
    let bad = || "This MKV file has a malformed block, so it can't be repackaged.".to_string();
    let mut q = 0usize;
    let (track, _) = read_vint(body, &mut q).ok_or_else(bad)?;
    if !wanted.contains(&track) {
        return Ok(());
    }
    if q + 3 > body.len() {
        return Err(bad());
    }
    let ticks = cluster_ts + i16::from_be_bytes([body[q], body[q + 1]]) as i64;
    let flags = body[q + 2];
    q += 3;
    let keyframe = keyframe_override.unwrap_or(flags & 0x80 != 0);

    let lacing = (flags >> 1) & 3;
    if lacing == 0 {
        frames.push(Frame { track, ticks, keyframe, data: &body[q..] });
        return Ok(());
    }

    let count = *body.get(q).ok_or_else(bad)? as usize + 1;
    q += 1;
    let mut sizes: Vec<usize> = Vec::with_capacity(count);
    match lacing {
        1 => {
            for _ in 0..count - 1 {
                let mut s = 0usize;
                loop {
                    let b = *body.get(q).ok_or_else(bad)?;
                    q += 1;
                    s += b as usize;
                    if b != 255 {
                        break;
                    }
                }
                sizes.push(s);
            }
        }
        3 => {
            if count > 1 {
                let (first, _) = read_vint(body, &mut q).ok_or_else(bad)?;
                sizes.push(first as usize);
                let mut prev = first as i64;
                for _ in 1..count - 1 {
                    let (raw, len) = read_vint(body, &mut q).ok_or_else(bad)?;
                    let v = prev + raw as i64 - ((1i64 << (7 * len - 1)) - 1);
                    if v < 0 {
                        return Err(bad());
                    }
                    sizes.push(v as usize);
                    prev = v;
                }
            }
        }
        _ => {
            let rest = body.len() - q;
            if rest % count != 0 {
                return Err(bad());
            }
            sizes.resize(count - 1, rest / count);
        }
    }
    let used: usize = sizes.iter().sum();
    if q + used > body.len() {
        return Err(bad());
    }
    sizes.push(body.len() - q - used);
    for s in sizes {
        frames.push(Frame { track, ticks, keyframe, data: &body[q..q + s] });
        q += s;
    }
    Ok(())
}

fn read_frames<'a>(raw: &'a [u8], wanted: &[u64]) -> Result<(Vec<Frame<'a>>, u64), String> {
    let mut frames = Vec::new();
    let mut timestamp_scale = 1_000_000u64;
    let mut cluster_ts: i64 = 0;
    let mut p = 0usize;
    while p < raw.len() {
        let Some(id) = read_id(raw, &mut p) else { break };
        let Some(size) = read_size(raw, &mut p) else { break };
        if id == ebml::ID_SEGMENT || id == ebml::ID_CLUSTER {
            if id == ebml::ID_CLUSTER {
                cluster_ts = 0;
            }
            continue;
        }
        if size == UNKNOWN_SIZE {
            break;
        }
        let Some(body_end) = p.checked_add(size as usize).filter(|&e| e <= raw.len()) else { break };
        let body = &raw[p..body_end];
        match id {
            ebml::ID_INFO => {
                if let Ok(children) = ebml::read_elements(body) {
                    if let Some(e) = ebml::find(&children, ebml::ID_TIMESTAMP_SCALE) {
                        timestamp_scale = ebml::as_uint(e).max(1);
                    }
                }
            }
            ebml::ID_TIMESTAMP => {
                cluster_ts = body.iter().fold(0i64, |acc, &b| (acc << 8) | b as i64);
            }
            ebml::ID_SIMPLE_BLOCK => parse_block(body, None, cluster_ts, wanted, &mut frames)?,
            ID_BLOCK_GROUP => {
                let mut block: Option<&[u8]> = None;
                let mut has_reference = false;
                let mut g = 0usize;
                while g < body.len() {
                    let Some(cid) = read_id(body, &mut g) else { break };
                    let Some(csize) = read_size(body, &mut g) else { break };
                    let Some(cend) = g.checked_add(csize as usize).filter(|&e| e <= body.len()) else { break };
                    match cid {
                        ID_BLOCK => block = Some(&body[g..cend]),
                        ID_REFERENCE_BLOCK => has_reference = true,
                        _ => {}
                    }
                    g = cend;
                }
                if let Some(b) = block {
                    parse_block(b, Some(!has_reference), cluster_ts, wanted, &mut frames)?;
                }
            }
            _ => {}
        }
        p = body_end;
    }
    Ok((frames, timestamp_scale))
}

fn ticks_to_units(ticks: i64, timestamp_scale: u64, timescale: u32) -> i64 {
    let num = ticks as i128 * timestamp_scale as i128 * timescale as i128;
    (num + 500_000_000).div_euclid(1_000_000_000) as i64
}

fn units_to_movie(units: i64, timescale: u32) -> i64 {
    let num = units as i128 * MOVIE_TIMESCALE as i128;
    (num + timescale as i128 / 2).div_euclid(timescale as i128) as i64
}

fn h264_sample_is_idr(data: &[u8], length_size: usize) -> bool {
    let mut i = 0usize;
    while i + length_size <= data.len() {
        let len = data[i..i + length_size].iter().fold(0usize, |acc, &b| (acc << 8) | b as usize);
        i += length_size;
        if len == 0 || i + len > data.len() {
            break;
        }
        if data[i] & 0x1F == 5 {
            return true;
        }
        i += len;
    }
    false
}

enum Kind<'a> {
    Avc { avcc: &'a [u8], width: u16, height: u16 },
    Aac { asc: &'a [u8], channels: u16, sample_rate: u32 },
    Mp3 { channels: u16, sample_rate: u32 },
}

struct OutTrack<'a> {
    kind: Kind<'a>,
    timescale: u32,
    samples: Vec<&'a [u8]>,
    decode_times: Vec<i64>,
    durations: Vec<u32>,
    composition_offsets: Vec<u32>,
    sync_samples: Option<Vec<u32>>,
    start_ns: i128,
    media_time: i64,
    trim: i64,
    empty_edit: i64,
    chunk_sizes: Vec<u32>,
    chunk_offsets: Vec<u64>,
}

impl OutTrack<'_> {
    fn media_duration(&self) -> i64 {
        self.durations.iter().map(|&d| d as i64).sum()
    }

    fn presented_duration(&self) -> i64 {
        (self.media_duration() - self.trim).max(0)
    }

    fn movie_duration(&self) -> i64 {
        self.empty_edit + units_to_movie(self.presented_duration(), self.timescale)
    }

    fn is_video(&self) -> bool {
        matches!(self.kind, Kind::Avc { .. })
    }
}

fn build_video_track<'a>(
    t: &'a TrackMeta,
    frames: &[&Frame<'a>],
    timestamp_scale: u64,
) -> Result<OutTrack<'a>, String> {
    if t.codec_private.len() < 7 {
        return Err(format!(
            "This file's H.264 track (track {}) has no usable decoder configuration (avcC), so it can't be repackaged.",
            t.number
        ));
    }
    let length_size = ((t.codec_private[4] & 0x03) + 1) as usize;
    let n = frames.len();

    let pts: Vec<i64> = frames.iter().map(|f| ticks_to_units(f.ticks, timestamp_scale, VIDEO_TIMESCALE)).collect();
    let mut sorted = pts.clone();
    sorted.sort_unstable();
    let mut dts: Vec<i64> = Vec::with_capacity(n);
    for (i, &s) in sorted.iter().enumerate() {
        dts.push(if i > 0 && s <= dts[i - 1] { dts[i - 1] + 1 } else { s });
    }
    let shift = dts.iter().zip(&pts).map(|(d, p)| d - p).max().unwrap_or(0).max(0);
    let composition_offsets: Vec<u32> = dts.iter().zip(&pts).map(|(d, p)| (p - d + shift) as u32).collect();

    let mut durations: Vec<u32> = dts.windows(2).map(|w| (w[1] - w[0]) as u32).collect();
    let last = durations.last().copied().unwrap_or(VIDEO_TIMESCALE / 30);
    durations.push(last);

    let mut sync: Vec<u32> = (0..n)
        .filter(|&i| h264_sample_is_idr(frames[i].data, length_size))
        .map(|i| i as u32 + 1)
        .collect();
    if sync.is_empty() {
        sync = (0..n).filter(|&i| frames[i].keyframe).map(|i| i as u32 + 1).collect();
    }
    if sync.is_empty() {
        sync.push(1);
    }
    let sync_samples = if sync.len() == n { None } else { Some(sync) };

    let min_ticks = frames.iter().map(|f| f.ticks).min().unwrap_or(0);
    Ok(OutTrack {
        kind: Kind::Avc {
            avcc: &t.codec_private,
            width: t.width.unwrap_or(0).min(u16::MAX as u64) as u16,
            height: t.height.unwrap_or(0).min(u16::MAX as u64) as u16,
        },
        timescale: VIDEO_TIMESCALE,
        samples: frames.iter().map(|f| f.data).collect(),
        decode_times: dts.iter().map(|d| d - dts[0]).collect(),
        durations,
        composition_offsets,
        sync_samples,
        start_ns: min_ticks as i128 * timestamp_scale as i128,
        media_time: shift,
        trim: 0,
        empty_edit: 0,
        chunk_sizes: Vec::new(),
        chunk_offsets: Vec::new(),
    })
}

fn build_audio_track<'a>(
    t: &'a TrackMeta,
    frames: &[&Frame<'a>],
    timestamp_scale: u64,
) -> Result<OutTrack<'a>, String> {
    let sample_rate = t.sample_rate.unwrap_or(0.0).round() as u32;
    if sample_rate == 0 {
        return Err(format!("This file's audio track (track {}) doesn't declare a sample rate.", t.number));
    }
    let channels = t.channels.unwrap_or(2).clamp(1, u16::MAX as u64) as u16;
    let (kind, candidates): (Kind, &[u32]) = if t.codec_id == "A_AAC" {
        if t.codec_private.len() < 2 {
            return Err(format!(
                "This file's AAC track (track {}) has no decoder configuration, so it can't be repackaged.",
                t.number
            ));
        }
        (Kind::Aac { asc: &t.codec_private, channels, sample_rate }, &[1024, 960, 2048, 1920])
    } else {
        (Kind::Mp3 { channels, sample_rate }, &[1152, 576])
    };

    let n = frames.len();
    let frame_duration = if n >= 2 {
        let span = ticks_to_units(frames[n - 1].ticks - frames[0].ticks, timestamp_scale, sample_rate);
        let average = span as f64 / (n - 1) as f64;
        *candidates
            .iter()
            .min_by(|a, b| (**a as f64 - average).abs().total_cmp(&(**b as f64 - average).abs()))
            .unwrap()
    } else {
        candidates[0]
    };

    Ok(OutTrack {
        kind,
        timescale: sample_rate,
        samples: frames.iter().map(|f| f.data).collect(),
        decode_times: (0..n as i64).map(|i| i * frame_duration as i64).collect(),
        durations: vec![frame_duration; n],
        composition_offsets: Vec::new(),
        sync_samples: None,
        start_ns: frames[0].ticks as i128 * timestamp_scale as i128 - t.codec_delay_ns.unwrap_or(0) as i128,
        media_time: 0,
        trim: 0,
        empty_edit: 0,
        chunk_sizes: Vec::new(),
        chunk_offsets: Vec::new(),
    })
}

fn bx(kind: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len() + 8);
    out.extend_from_slice(&(body.len() as u32 + 8).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
    out
}

fn full_box(kind: &[u8; 4], version: u8, flags: u32, body: &[u8]) -> Vec<u8> {
    let mut inner = Vec::with_capacity(body.len() + 4);
    inner.extend_from_slice(&((version as u32) << 24 | (flags & 0x00FF_FFFF)).to_be_bytes());
    inner.extend_from_slice(body);
    bx(kind, &inner)
}

fn descriptor(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    let len = body.len();
    let mut groups = vec![(len & 0x7F) as u8];
    let mut rest = len >> 7;
    while rest > 0 {
        groups.push((rest & 0x7F) as u8 | 0x80);
        rest >>= 7;
    }
    out.extend(groups.iter().rev());
    out.extend_from_slice(body);
    out
}

const IDENTITY_MATRIX: [u32; 9] = [0x0001_0000, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000];

fn push_matrix(b: &mut Vec<u8>) {
    for v in IDENTITY_MATRIX {
        b.extend_from_slice(&v.to_be_bytes());
    }
}

fn esds(track_id: u32, track: &OutTrack) -> Vec<u8> {
    let (object_type, asc): (u8, &[u8]) = match track.kind {
        Kind::Aac { asc, .. } => (0x40, asc),
        _ => (0x6B, &[]),
    };
    let total_bytes: u64 = track.samples.iter().map(|s| s.len() as u64).sum();
    let seconds = (track.media_duration() as f64 / track.timescale as f64).max(0.001);
    let avg_bitrate = (total_bytes as f64 * 8.0 / seconds) as u32;
    let mut per_second: std::collections::HashMap<i64, u64> = std::collections::HashMap::new();
    for (s, &t) in track.samples.iter().zip(&track.decode_times) {
        *per_second.entry(t / track.timescale as i64).or_insert(0) += s.len() as u64;
    }
    let max_bitrate = per_second.values().max().map(|&b| (b * 8).min(u32::MAX as u64) as u32).unwrap_or(avg_bitrate);
    let buffer_size = track.samples.iter().map(|s| s.len()).max().unwrap_or(0).min(0xFF_FFFF) as u32;

    let mut dcd = vec![object_type, 0x15];
    dcd.extend_from_slice(&buffer_size.to_be_bytes()[1..]);
    dcd.extend_from_slice(&max_bitrate.max(avg_bitrate).to_be_bytes());
    dcd.extend_from_slice(&avg_bitrate.to_be_bytes());
    if !asc.is_empty() {
        dcd.extend(descriptor(0x05, asc));
    }

    let mut es = Vec::new();
    es.extend_from_slice(&(track_id.min(u16::MAX as u32) as u16).to_be_bytes());
    es.push(0);
    es.extend(descriptor(0x04, &dcd));
    es.extend(descriptor(0x06, &[0x02]));
    full_box(b"esds", 0, 0, &descriptor(0x03, &es))
}

fn sample_entry(track_id: u32, track: &OutTrack) -> Vec<u8> {
    let mut b = vec![0u8; 6];
    b.extend_from_slice(&1u16.to_be_bytes());
    match track.kind {
        Kind::Avc { avcc, width, height } => {
            b.extend_from_slice(&[0u8; 16]);
            b.extend_from_slice(&width.to_be_bytes());
            b.extend_from_slice(&height.to_be_bytes());
            b.extend_from_slice(&0x0048_0000u32.to_be_bytes());
            b.extend_from_slice(&0x0048_0000u32.to_be_bytes());
            b.extend_from_slice(&0u32.to_be_bytes());
            b.extend_from_slice(&1u16.to_be_bytes());
            b.extend_from_slice(&[0u8; 32]);
            b.extend_from_slice(&0x0018u16.to_be_bytes());
            b.extend_from_slice(&(-1i16).to_be_bytes());
            b.extend(bx(b"avcC", avcc));
            bx(b"avc1", &b)
        }
        Kind::Aac { channels, sample_rate, .. } | Kind::Mp3 { channels, sample_rate } => {
            b.extend_from_slice(&[0u8; 8]);
            b.extend_from_slice(&channels.to_be_bytes());
            b.extend_from_slice(&16u16.to_be_bytes());
            b.extend_from_slice(&[0u8; 4]);
            let rate_fixed = if sample_rate <= u16::MAX as u32 { sample_rate << 16 } else { 0 };
            b.extend_from_slice(&rate_fixed.to_be_bytes());
            b.extend(esds(track_id, track));
            bx(b"mp4a", &b)
        }
    }
}

fn stbl(track_id: u32, track: &OutTrack, use_co64: bool) -> Vec<u8> {
    let mut out = Vec::new();

    let mut stsd = 1u32.to_be_bytes().to_vec();
    stsd.extend(sample_entry(track_id, track));
    out.extend(full_box(b"stsd", 0, 0, &stsd));

    let mut runs: Vec<(u32, u32)> = Vec::new();
    for &d in &track.durations {
        match runs.last_mut() {
            Some((count, delta)) if *delta == d => *count += 1,
            _ => runs.push((1, d)),
        }
    }
    let mut stts = (runs.len() as u32).to_be_bytes().to_vec();
    for (count, delta) in &runs {
        stts.extend_from_slice(&count.to_be_bytes());
        stts.extend_from_slice(&delta.to_be_bytes());
    }
    out.extend(full_box(b"stts", 0, 0, &stts));

    if track.composition_offsets.iter().any(|&c| c != 0) {
        let mut runs: Vec<(u32, u32)> = Vec::new();
        for &c in &track.composition_offsets {
            match runs.last_mut() {
                Some((count, offset)) if *offset == c => *count += 1,
                _ => runs.push((1, c)),
            }
        }
        let mut ctts = (runs.len() as u32).to_be_bytes().to_vec();
        for (count, offset) in &runs {
            ctts.extend_from_slice(&count.to_be_bytes());
            ctts.extend_from_slice(&offset.to_be_bytes());
        }
        out.extend(full_box(b"ctts", 0, 0, &ctts));
    }

    if let Some(sync) = &track.sync_samples {
        let mut stss = (sync.len() as u32).to_be_bytes().to_vec();
        for s in sync {
            stss.extend_from_slice(&s.to_be_bytes());
        }
        out.extend(full_box(b"stss", 0, 0, &stss));
    }

    let mut chunk_runs: Vec<(u32, u32)> = Vec::new();
    for (i, &size) in track.chunk_sizes.iter().enumerate() {
        if chunk_runs.last().map(|&(_, s)| s) != Some(size) {
            chunk_runs.push((i as u32 + 1, size));
        }
    }
    let mut stsc = (chunk_runs.len() as u32).to_be_bytes().to_vec();
    for (first, size) in &chunk_runs {
        stsc.extend_from_slice(&first.to_be_bytes());
        stsc.extend_from_slice(&size.to_be_bytes());
        stsc.extend_from_slice(&1u32.to_be_bytes());
    }
    out.extend(full_box(b"stsc", 0, 0, &stsc));

    let mut stsz = 0u32.to_be_bytes().to_vec();
    stsz.extend_from_slice(&(track.samples.len() as u32).to_be_bytes());
    for s in &track.samples {
        stsz.extend_from_slice(&(s.len() as u32).to_be_bytes());
    }
    out.extend(full_box(b"stsz", 0, 0, &stsz));

    let mut offsets = (track.chunk_offsets.len() as u32).to_be_bytes().to_vec();
    for &o in &track.chunk_offsets {
        if use_co64 {
            offsets.extend_from_slice(&o.to_be_bytes());
        } else {
            offsets.extend_from_slice(&(o as u32).to_be_bytes());
        }
    }
    out.extend(full_box(if use_co64 { b"co64" } else { b"stco" }, 0, 0, &offsets));

    bx(b"stbl", &out)
}

fn trak(track_id: u32, track: &OutTrack, enabled: bool, use_co64: bool) -> Vec<u8> {
    let video = track.is_video();

    let mut tkhd = Vec::new();
    tkhd.extend_from_slice(&[0u8; 8]);
    tkhd.extend_from_slice(&track_id.to_be_bytes());
    tkhd.extend_from_slice(&[0u8; 4]);
    tkhd.extend_from_slice(&(track.movie_duration().clamp(0, u32::MAX as i64) as u32).to_be_bytes());
    tkhd.extend_from_slice(&[0u8; 8]);
    tkhd.extend_from_slice(&0u16.to_be_bytes());
    tkhd.extend_from_slice(&(if video { 0u16 } else { 1u16 }).to_be_bytes());
    tkhd.extend_from_slice(&(if video { 0u16 } else { 0x0100u16 }).to_be_bytes());
    tkhd.extend_from_slice(&[0u8; 2]);
    push_matrix(&mut tkhd);
    let (w, h) = match track.kind {
        Kind::Avc { width, height, .. } => (width as u32, height as u32),
        _ => (0, 0),
    };
    tkhd.extend_from_slice(&(w << 16).to_be_bytes());
    tkhd.extend_from_slice(&(h << 16).to_be_bytes());

    let mut elst_entries: Vec<(u32, i32)> = Vec::new();
    if track.empty_edit > 0 {
        elst_entries.push((track.empty_edit.min(u32::MAX as i64) as u32, -1));
    }
    elst_entries.push((
        units_to_movie(track.presented_duration(), track.timescale).clamp(0, u32::MAX as i64) as u32,
        track.media_time.clamp(0, i32::MAX as i64) as i32,
    ));
    let mut elst = (elst_entries.len() as u32).to_be_bytes().to_vec();
    for (duration, media_time) in elst_entries {
        elst.extend_from_slice(&duration.to_be_bytes());
        elst.extend_from_slice(&media_time.to_be_bytes());
        elst.extend_from_slice(&1u16.to_be_bytes());
        elst.extend_from_slice(&0u16.to_be_bytes());
    }
    let edts = bx(b"edts", &full_box(b"elst", 0, 0, &elst));

    let mut mdhd = Vec::new();
    mdhd.extend_from_slice(&[0u8; 8]);
    mdhd.extend_from_slice(&track.timescale.to_be_bytes());
    mdhd.extend_from_slice(&(track.media_duration().clamp(0, u32::MAX as i64) as u32).to_be_bytes());
    mdhd.extend_from_slice(&0x55C4u16.to_be_bytes());
    mdhd.extend_from_slice(&0u16.to_be_bytes());

    let mut hdlr = vec![0u8; 4];
    hdlr.extend_from_slice(if video { b"vide" } else { b"soun" });
    hdlr.extend_from_slice(&[0u8; 12]);
    hdlr.extend_from_slice(if video { b"VideoHandler\0" } else { b"SoundHandler\0" });

    let media_header = if video {
        full_box(b"vmhd", 0, 1, &[0u8; 8])
    } else {
        full_box(b"smhd", 0, 0, &[0u8; 4])
    };
    let mut dref = 1u32.to_be_bytes().to_vec();
    dref.extend(full_box(b"url ", 0, 1, &[]));
    let dinf = bx(b"dinf", &full_box(b"dref", 0, 0, &dref));

    let mut minf = media_header;
    minf.extend(dinf);
    minf.extend(stbl(track_id, track, use_co64));

    let mut mdia = full_box(b"mdhd", 0, 0, &mdhd);
    mdia.extend(full_box(b"hdlr", 0, 0, &hdlr));
    mdia.extend(bx(b"minf", &minf));

    let mut body = full_box(b"tkhd", 0, if enabled { 3 } else { 2 }, &tkhd);
    body.extend(edts);
    body.extend(bx(b"mdia", &mdia));
    bx(b"trak", &body)
}

fn moov(tracks: &[OutTrack], use_co64: bool) -> Vec<u8> {
    let duration = tracks.iter().map(|t| t.movie_duration()).max().unwrap_or(0);
    let mut mvhd = Vec::new();
    mvhd.extend_from_slice(&[0u8; 8]);
    mvhd.extend_from_slice(&MOVIE_TIMESCALE.to_be_bytes());
    mvhd.extend_from_slice(&(duration.clamp(0, u32::MAX as i64) as u32).to_be_bytes());
    mvhd.extend_from_slice(&0x0001_0000u32.to_be_bytes());
    mvhd.extend_from_slice(&0x0100u16.to_be_bytes());
    mvhd.extend_from_slice(&[0u8; 10]);
    push_matrix(&mut mvhd);
    mvhd.extend_from_slice(&[0u8; 24]);
    mvhd.extend_from_slice(&(tracks.len() as u32 + 1).to_be_bytes());

    let mut body = full_box(b"mvhd", 0, 0, &mvhd);
    let mut seen_video = false;
    let mut seen_audio = false;
    for (i, t) in tracks.iter().enumerate() {
        let seen = if t.is_video() { &mut seen_video } else { &mut seen_audio };
        body.extend(trak(i as u32 + 1, t, !*seen, use_co64));
        *seen = true;
    }
    bx(b"moov", &body)
}

pub fn write_from_mkv<W: Write>(raw: &[u8], tracks: &[TrackMeta], out: &mut W) -> Result<(), String> {
    let wanted: Vec<u64> = tracks.iter().map(|t| t.number).collect();
    let (frames, timestamp_scale) = read_frames(raw, &wanted)?;

    let mut out_tracks: Vec<OutTrack> = Vec::new();
    for t in tracks {
        let track_frames: Vec<&Frame> = frames.iter().filter(|f| f.track == t.number).collect();
        if track_frames.is_empty() {
            continue;
        }
        out_tracks.push(if t.track_type == 1 {
            build_video_track(t, &track_frames, timestamp_scale)?
        } else {
            build_audio_track(t, &track_frames, timestamp_scale)?
        });
    }
    if out_tracks.is_empty() {
        return Err("No audio or video frames were found in this file.".to_string());
    }

    let zero_ns = out_tracks.iter().map(|t| t.start_ns).min().unwrap_or(0).max(0);
    for t in &mut out_tracks {
        let offset_ns = t.start_ns - zero_ns;
        if offset_ns > 0 {
            t.empty_edit = ((offset_ns * MOVIE_TIMESCALE as i128 + 500_000_000) / 1_000_000_000) as i64;
        } else if offset_ns < 0 {
            let trim = ((-offset_ns * t.timescale as i128 + 500_000_000) / 1_000_000_000) as i64;
            t.trim = trim.min(t.media_duration() - 1).max(0);
            t.media_time += t.trim;
        }
    }

    struct Chunk {
        track: usize,
        first: usize,
        count: usize,
        start_seconds: f64,
    }
    let mut chunks: Vec<Chunk> = Vec::new();
    for (ti, t) in out_tracks.iter().enumerate() {
        let limit = (t.timescale / CHUNK_SECONDS_DIVISOR) as i64;
        let offset_seconds = t.empty_edit as f64 / MOVIE_TIMESCALE as f64 - t.media_time as f64 / t.timescale as f64;
        let mut first = 0usize;
        while first < t.samples.len() {
            let mut last = first + 1;
            while last < t.samples.len() && t.decode_times[last] - t.decode_times[first] < limit {
                last += 1;
            }
            chunks.push(Chunk {
                track: ti,
                first,
                count: last - first,
                start_seconds: t.decode_times[first] as f64 / t.timescale as f64 + offset_seconds,
            });
            first = last;
        }
    }
    chunks.sort_by(|a, b| a.start_seconds.total_cmp(&b.start_seconds).then(a.track.cmp(&b.track)));

    let mdat_payload: u64 = out_tracks.iter().flat_map(|t| &t.samples).map(|s| s.len() as u64).sum();
    let mdat_header_len: u64 = if mdat_payload + 8 > u32::MAX as u64 { 16 } else { 8 };

    let has_video = out_tracks.iter().any(|t| t.is_video());
    let mut ftyp = b"isom".to_vec();
    ftyp.extend_from_slice(&0x200u32.to_be_bytes());
    ftyp.extend_from_slice(b"isomiso2");
    if has_video {
        ftyp.extend_from_slice(b"avc1");
    }
    ftyp.extend_from_slice(b"mp41");
    let ftyp = bx(b"ftyp", &ftyp);

    let assign_offsets = |out_tracks: &mut Vec<OutTrack>, base: u64| {
        for t in out_tracks.iter_mut() {
            t.chunk_sizes.clear();
            t.chunk_offsets.clear();
        }
        let mut pos = base;
        for c in &chunks {
            let t = &mut out_tracks[c.track];
            t.chunk_sizes.push(c.count as u32);
            t.chunk_offsets.push(pos);
            pos += t.samples[c.first..c.first + c.count].iter().map(|s| s.len() as u64).sum::<u64>();
        }
    };

    assign_offsets(&mut out_tracks, 0);
    let mut use_co64 = false;
    let moov_len = moov(&out_tracks, false).len() as u64;
    if ftyp.len() as u64 + moov_len + mdat_header_len + mdat_payload > u32::MAX as u64 {
        use_co64 = true;
    }
    let moov_len = moov(&out_tracks, use_co64).len() as u64;
    assign_offsets(&mut out_tracks, ftyp.len() as u64 + moov_len + mdat_header_len);
    let moov_box = moov(&out_tracks, use_co64);

    let write_err = |e: std::io::Error| format!("Couldn't write the output file: {e}");
    out.write_all(&ftyp).map_err(write_err)?;
    out.write_all(&moov_box).map_err(write_err)?;
    if mdat_header_len == 16 {
        out.write_all(&1u32.to_be_bytes()).map_err(write_err)?;
        out.write_all(b"mdat").map_err(write_err)?;
        out.write_all(&(mdat_payload + 16).to_be_bytes()).map_err(write_err)?;
    } else {
        out.write_all(&(mdat_payload as u32 + 8).to_be_bytes()).map_err(write_err)?;
        out.write_all(b"mdat").map_err(write_err)?;
    }
    for c in &chunks {
        for s in &out_tracks[c.track].samples[c.first..c.first + c.count] {
            out.write_all(s).map_err(write_err)?;
        }
    }
    Ok(())
}
