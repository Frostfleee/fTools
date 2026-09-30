use std::borrow::Cow;

use crate::ebml;

#[derive(Clone, Debug, PartialEq)]
pub enum Codec {
    H264 { avcc: Vec<u8> },
    Hevc { hvcc: Vec<u8> },
    Vp8,
    Vp9,
    Av1 { config: Vec<u8> },
    Aac { asc: Vec<u8> },
    Mp3,
    Opus { head: Vec<u8> },
    Vorbis { headers: Vec<u8> },
    Unsupported(String),
}

impl Codec {
    pub fn name(&self) -> String {
        match self {
            Codec::H264 { .. } => "H.264".to_string(),
            Codec::Hevc { .. } => "HEVC (H.265)".to_string(),
            Codec::Vp8 => "VP8".to_string(),
            Codec::Vp9 => "VP9".to_string(),
            Codec::Av1 { .. } => "AV1".to_string(),
            Codec::Aac { .. } => "AAC".to_string(),
            Codec::Mp3 => "MP3".to_string(),
            Codec::Opus { .. } => "Opus".to_string(),
            Codec::Vorbis { .. } => "Vorbis".to_string(),
            Codec::Unsupported(name) => name.clone(),
        }
    }
}

pub struct Sample<'a> {
    pub pts_ns: i64,
    pub keyframe: bool,
    pub data: Cow<'a, [u8]>,
}

pub struct Track<'a> {
    pub video: bool,
    pub codec: Codec,
    pub width: u32,
    pub height: u32,
    pub sample_rate: u32,
    pub channels: u16,
    pub codec_delay_ns: i64,
    pub samples: Vec<Sample<'a>>,
}

impl Track<'_> {
    pub fn start_ns(&self) -> i64 {
        self.samples.iter().map(|s| s.pts_ns).min().unwrap_or(0) - self.codec_delay_ns
    }

    pub fn end_ns(&self) -> i64 {
        let mut times: Vec<i64> = self.samples.iter().map(|s| s.pts_ns).collect();
        times.sort_unstable();
        let last = times.last().copied().unwrap_or(0);
        let step = if times.len() >= 2 { last - times[times.len() - 2] } else { 0 };
        last + step - self.codec_delay_ns
    }

    pub fn bitrate_bps(&self) -> u64 {
        let bytes: u64 = self.samples.iter().map(|s| s.data.len() as u64).sum();
        let span = (self.end_ns() - self.start_ns()).max(1_000_000);
        (bytes as u128 * 8 * 1_000_000_000 / span as u128) as u64
    }
}

const ID_BLOCK_GROUP: u32 = 0xA0;
const ID_BLOCK: u32 = 0xA1;
const ID_REFERENCE_BLOCK: u32 = 0xFB;
const UNKNOWN_SIZE: u64 = u64::MAX;

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
    let bad = || "This file has a malformed block, so it can't be converted.".to_string();
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

pub fn read_matroska(raw: &[u8]) -> Result<Vec<Track<'_>>, String> {
    let metas = ebml::parse_tracks(raw).map_err(|e| {
        format!("This doesn't look like a valid Matroska/WebM file (couldn't read its Tracks element): {e}")
    })?;
    let metas: Vec<&ebml::TrackMeta> = metas.iter().filter(|t| t.track_type == 1 || t.track_type == 2).collect();
    let wanted: Vec<u64> = metas.iter().map(|t| t.number).collect();
    let (frames, timestamp_scale) = read_frames(raw, &wanted)?;

    let mut tracks = Vec::new();
    for t in metas {
        let cp = t.codec_private.clone();
        let codec = match t.codec_id.as_str() {
            "V_MPEG4/ISO/AVC" => Codec::H264 { avcc: cp },
            "V_MPEGH/ISO/HEVC" => Codec::Hevc { hvcc: cp },
            "V_VP8" => Codec::Vp8,
            "V_VP9" => Codec::Vp9,
            "V_AV1" => Codec::Av1 { config: cp },
            "A_AAC" => Codec::Aac { asc: cp },
            "A_MPEG/L3" => Codec::Mp3,
            "A_OPUS" => Codec::Opus { head: cp },
            "A_VORBIS" => Codec::Vorbis { headers: cp },
            other => Codec::Unsupported(other.to_string()),
        };
        let samples: Vec<Sample> = frames
            .iter()
            .filter(|f| f.track == t.number)
            .map(|f| Sample {
                pts_ns: (f.ticks as i128 * timestamp_scale as i128) as i64,
                keyframe: f.keyframe,
                data: Cow::Borrowed(f.data),
            })
            .collect();
        if samples.is_empty() {
            continue;
        }
        tracks.push(Track {
            video: t.track_type == 1,
            codec,
            width: t.width.unwrap_or(0) as u32,
            height: t.height.unwrap_or(0) as u32,
            sample_rate: t.sample_rate.unwrap_or(0.0).round() as u32,
            channels: t.channels.unwrap_or(0) as u16,
            codec_delay_ns: t.codec_delay_ns.unwrap_or(0) as i64,
            samples,
        });
    }
    Ok(tracks)
}

fn be16(d: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes(d.get(at..at + 2)?.try_into().ok()?))
}

fn be32(d: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(d.get(at..at + 4)?.try_into().ok()?))
}

fn be64(d: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_be_bytes(d.get(at..at + 8)?.try_into().ok()?))
}

fn boxes(d: &[u8]) -> Vec<([u8; 4], &[u8])> {
    let mut out = Vec::new();
    let mut p = 0usize;
    while p + 8 <= d.len() {
        let size = be32(d, p).unwrap_or(0) as u64;
        let kind: [u8; 4] = d[p + 4..p + 8].try_into().unwrap();
        let (header, total) = match size {
            0 => (8u64, (d.len() - p) as u64),
            1 => match be64(d, p + 8) {
                Some(s) => (16, s),
                None => break,
            },
            s => (8, s),
        };
        if total < header || p as u64 + total > d.len() as u64 {
            break;
        }
        out.push((kind, &d[p + header as usize..p + total as usize]));
        p += total as usize;
    }
    out
}

fn child<'a>(d: &'a [u8], kind: &[u8; 4]) -> Option<&'a [u8]> {
    boxes(d).into_iter().find(|(k, _)| k == kind).map(|(_, b)| b)
}

fn find_nested<'a>(d: &'a [u8], kind: &[u8; 4], depth: u32) -> Option<&'a [u8]> {
    for (k, body) in boxes(d) {
        if &k == kind {
            return Some(body);
        }
        if depth > 0 {
            if let Some(found) = find_nested(body, kind, depth - 1) {
                return Some(found);
            }
        }
    }
    None
}

fn read_descriptor(d: &[u8], p: &mut usize) -> Option<(u8, usize, usize)> {
    let tag = *d.get(*p)?;
    *p += 1;
    let mut len = 0usize;
    for _ in 0..4 {
        let b = *d.get(*p)?;
        *p += 1;
        len = (len << 7) | (b & 0x7F) as usize;
        if b & 0x80 == 0 {
            break;
        }
    }
    let start = *p;
    if start + len > d.len() {
        return None;
    }
    Some((tag, start, start + len))
}

fn parse_esds(esds: &[u8]) -> Option<(u8, Vec<u8>)> {
    let d = esds.get(4..)?;
    let mut p = 0usize;
    let (tag, start, end) = read_descriptor(d, &mut p)?;
    if tag != 0x03 {
        return None;
    }
    let es = &d[start..end];
    let flags = *es.get(2)?;
    let mut q = 3usize;
    if flags & 0x80 != 0 {
        q += 2;
    }
    if flags & 0x40 != 0 {
        q += 1 + *es.get(q)? as usize;
    }
    if flags & 0x20 != 0 {
        q += 2;
    }
    let (tag, start, end) = read_descriptor(es, &mut q)?;
    if tag != 0x04 {
        return None;
    }
    let dcd = &es[start..end];
    let object_type = *dcd.first()?;
    let mut r = 13usize;
    let mut asc = Vec::new();
    while r < dcd.len() {
        let Some((tag, start, end)) = read_descriptor(dcd, &mut r) else { break };
        if tag == 0x05 {
            asc = dcd[start..end].to_vec();
        }
        r = end;
    }
    Some((object_type, asc))
}

fn opus_head_from_dops(dops: &[u8]) -> Option<Vec<u8>> {
    let channels = *dops.get(1)?;
    let pre_skip = be16(dops, 2)?;
    let rate = be32(dops, 4)?;
    let gain = be16(dops, 8)?;
    let family = *dops.get(10)?;
    let mut head = b"OpusHead".to_vec();
    head.push(1);
    head.push(channels);
    head.extend_from_slice(&pre_skip.to_le_bytes());
    head.extend_from_slice(&rate.to_le_bytes());
    head.extend_from_slice(&gain.to_le_bytes());
    head.push(family);
    if family != 0 {
        head.extend_from_slice(dops.get(11..)?);
    }
    Some(head)
}

struct Mp4Entry {
    codec: Codec,
    width: u32,
    height: u32,
    sample_rate: u32,
    channels: u16,
}

fn parse_sample_entry(kind: [u8; 4], body: &[u8], video: bool) -> Mp4Entry {
    let mut entry = Mp4Entry { codec: Codec::Unsupported(String::from_utf8_lossy(&kind).trim().to_string()), width: 0, height: 0, sample_rate: 0, channels: 0 };
    if video {
        entry.width = be16(body, 24).unwrap_or(0) as u32;
        entry.height = be16(body, 26).unwrap_or(0) as u32;
        let children = body.get(78..).unwrap_or(&[]);
        entry.codec = match &kind {
            b"avc1" | b"avc3" => match child(children, b"avcC") {
                Some(avcc) => Codec::H264 { avcc: avcc.to_vec() },
                None => entry.codec,
            },
            b"hvc1" | b"hev1" => Codec::Hevc { hvcc: child(children, b"hvcC").unwrap_or(&[]).to_vec() },
            b"vp08" => Codec::Vp8,
            b"vp09" => Codec::Vp9,
            b"av01" => Codec::Av1 { config: child(children, b"av1C").unwrap_or(&[]).to_vec() },
            _ => entry.codec,
        };
        return entry;
    }

    let version = be16(body, 8).unwrap_or(0);
    entry.channels = be16(body, 16).unwrap_or(0);
    entry.sample_rate = be16(body, 24).unwrap_or(0) as u32;
    let children_at = match version {
        1 => 44,
        2 => {
            entry.sample_rate = f64::from_bits(be64(body, 32).unwrap_or(0)).round() as u32;
            entry.channels = be32(body, 40).unwrap_or(0) as u16;
            64
        }
        _ => 28,
    };
    let children = body.get(children_at..).unwrap_or(&[]);
    entry.codec = match &kind {
        b"mp4a" => match find_nested(children, b"esds", 2).and_then(parse_esds) {
            Some((0x40 | 0x66 | 0x67 | 0x68, asc)) if !asc.is_empty() => Codec::Aac { asc },
            Some((0x69 | 0x6B, _)) => Codec::Mp3,
            _ => entry.codec,
        },
        b".mp3" => Codec::Mp3,
        b"Opus" => match child(children, b"dOps").and_then(opus_head_from_dops) {
            Some(head) => Codec::Opus { head },
            None => entry.codec,
        },
        _ => entry.codec,
    };
    entry
}

fn read_mp4_track<'a>(raw: &'a [u8], trak: &[u8], movie_timescale: u32) -> Option<Track<'a>> {
    let mdia = child(trak, b"mdia")?;
    let hdlr = child(mdia, b"hdlr")?;
    let handler = hdlr.get(8..12)?;
    let video = match handler {
        b"vide" => true,
        b"soun" => false,
        _ => return None,
    };
    let mdhd = child(mdia, b"mdhd")?;
    let timescale = if mdhd[0] == 1 { be32(mdhd, 20)? } else { be32(mdhd, 12)? }.max(1);
    let stbl = child(child(mdia, b"minf")?, b"stbl")?;

    let stsd = child(stbl, b"stsd")?;
    let (kind, entry_body) = *boxes(stsd.get(8..)?).first()?;
    let entry = parse_sample_entry(kind, entry_body, video);

    let mut sizes: Vec<u32> = Vec::new();
    if let Some(stsz) = child(stbl, b"stsz") {
        let uniform = be32(stsz, 4)?;
        let count = be32(stsz, 8)? as usize;
        for i in 0..count {
            sizes.push(if uniform != 0 { uniform } else { be32(stsz, 12 + 4 * i)? });
        }
    } else if let Some(stz2) = child(stbl, b"stz2") {
        let field = *stz2.get(7)?;
        let count = be32(stz2, 8)? as usize;
        for i in 0..count {
            sizes.push(match field {
                4 => ((stz2.get(12 + i / 2)? >> if i % 2 == 0 { 4 } else { 0 }) & 0x0F) as u32,
                8 => *stz2.get(12 + i)? as u32,
                _ => be16(stz2, 12 + 2 * i)? as u32,
            });
        }
    }
    let n = sizes.len();
    if n == 0 {
        return None;
    }

    let mut chunk_offsets: Vec<u64> = Vec::new();
    if let Some(stco) = child(stbl, b"stco") {
        for i in 0..be32(stco, 4)? as usize {
            chunk_offsets.push(be32(stco, 8 + 4 * i)? as u64);
        }
    } else if let Some(co64) = child(stbl, b"co64") {
        for i in 0..be32(co64, 4)? as usize {
            chunk_offsets.push(be64(co64, 8 + 8 * i)?);
        }
    }
    let stsc = child(stbl, b"stsc")?;
    let stsc_entries: Vec<(u32, u32)> = (0..be32(stsc, 4)? as usize)
        .map(|i| Some((be32(stsc, 8 + 12 * i)?, be32(stsc, 12 + 12 * i)?)))
        .collect::<Option<_>>()?;

    let mut offsets: Vec<u64> = Vec::with_capacity(n);
    for (ci, &chunk_offset) in chunk_offsets.iter().enumerate() {
        let chunk_number = ci as u32 + 1;
        let per_chunk = stsc_entries.iter().rev().find(|(first, _)| *first <= chunk_number).map(|e| e.1).unwrap_or(0);
        let mut pos = chunk_offset;
        for _ in 0..per_chunk {
            if offsets.len() == n {
                break;
            }
            offsets.push(pos);
            pos += sizes[offsets.len() - 1] as u64;
        }
    }
    if offsets.len() < n {
        return None;
    }

    let stts = child(stbl, b"stts")?;
    let mut dts: Vec<i64> = Vec::with_capacity(n);
    let mut t: i64 = 0;
    for i in 0..be32(stts, 4)? as usize {
        let count = be32(stts, 8 + 8 * i)?;
        let delta = be32(stts, 12 + 8 * i)? as i64;
        for _ in 0..count {
            if dts.len() < n {
                dts.push(t);
            }
            t += delta;
        }
    }
    while dts.len() < n {
        dts.push(t);
    }

    let mut composition = vec![0i64; n];
    if let Some(ctts) = child(stbl, b"ctts") {
        let mut i = 0usize;
        for e in 0..be32(ctts, 4)? as usize {
            let count = be32(ctts, 8 + 8 * e)?;
            let offset = be32(ctts, 12 + 8 * e)? as i32 as i64;
            for _ in 0..count {
                if i < n {
                    composition[i] = offset;
                }
                i += 1;
            }
        }
    }

    let sync: Option<Vec<u32>> = child(stbl, b"stss")
        .map(|stss| (0..be32(stss, 4).unwrap_or(0) as usize).filter_map(|i| be32(stss, 8 + 4 * i)).collect());

    let mut empty_ns: i64 = 0;
    let mut media_time: i64 = 0;
    if let Some(elst) = child(trak, b"edts").and_then(|e| child(e, b"elst")) {
        let v1 = elst[0] == 1;
        let entry_len = if v1 { 20 } else { 12 };
        for i in 0..be32(elst, 4)? as usize {
            let at = 8 + entry_len * i;
            let (duration, time) = if v1 {
                (be64(elst, at)? as i64, be64(elst, at + 8)? as i64)
            } else {
                (be32(elst, at)? as i64, be32(elst, at + 4)? as i32 as i64)
            };
            if time == -1 {
                empty_ns += (duration as i128 * 1_000_000_000 / movie_timescale.max(1) as i128) as i64;
            } else {
                media_time = time;
                break;
            }
        }
    }

    let to_ns = |units: i64| (units as i128 * 1_000_000_000 / timescale as i128) as i64;
    let mut samples = Vec::with_capacity(n);
    for i in 0..n {
        let start = offsets[i] as usize;
        let end = start.checked_add(sizes[i] as usize)?;
        let data = raw.get(start..end)?;
        let pts = dts[i] + composition[i];
        let pts_ns = if video { to_ns(pts - media_time) + empty_ns } else { to_ns(pts) + empty_ns };
        let keyframe = sync.as_ref().map(|s| s.binary_search(&(i as u32 + 1)).is_ok()).unwrap_or(true);
        samples.push(Sample { pts_ns, keyframe, data: Cow::Borrowed(data) });
    }

    Some(Track {
        video,
        codec: entry.codec,
        width: entry.width,
        height: entry.height,
        sample_rate: entry.sample_rate,
        channels: entry.channels,
        codec_delay_ns: if video { 0 } else { to_ns(media_time) },
        samples,
    })
}

pub fn read_mp4(raw: &[u8]) -> Result<Vec<Track<'_>>, String> {
    let top = boxes(raw);
    let moov = top
        .iter()
        .find(|(k, _)| k == b"moov")
        .map(|(_, b)| *b)
        .ok_or_else(|| "This doesn't look like a valid MP4/MOV file (no moov box).".to_string())?;
    if top.iter().any(|(k, _)| k == b"moof") {
        return Err("This is a fragmented MP4/MOV file, which isn't supported yet.".to_string());
    }
    let mvhd = child(moov, b"mvhd").unwrap_or(&[]);
    let movie_timescale = if mvhd.first() == Some(&1) { be32(mvhd, 20) } else { be32(mvhd, 12) }.unwrap_or(1000);
    let tracks: Vec<Track> = boxes(moov)
        .into_iter()
        .filter(|(k, _)| k == b"trak")
        .filter_map(|(_, trak)| read_mp4_track(raw, trak, movie_timescale))
        .collect();
    if tracks.is_empty() {
        return Err("No readable audio or video tracks were found in this file.".to_string());
    }
    Ok(tracks)
}

const AAC_SAMPLE_RATES: [u32; 13] = [96000, 88200, 64000, 48000, 44100, 32000, 24000, 22050, 16000, 12000, 11025, 8000, 7350];

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl BitReader<'_> {
    fn read(&mut self, bits: usize) -> Option<u32> {
        let mut v = 0u32;
        for _ in 0..bits {
            let byte = *self.data.get(self.pos / 8)?;
            v = (v << 1) | ((byte >> (7 - self.pos % 8)) & 1) as u32;
            self.pos += 1;
        }
        Some(v)
    }
}

fn adts_params(asc: &[u8]) -> Option<(u8, u8, u8)> {
    let mut r = BitReader { data: asc, pos: 0 };
    let read_aot = |r: &mut BitReader| -> Option<u32> {
        let aot = r.read(5)?;
        if aot == 31 { Some(32 + r.read(6)?) } else { Some(aot) }
    };
    let read_rate_index = |r: &mut BitReader| -> Option<u8> {
        let index = r.read(4)?;
        if index == 15 {
            let rate = r.read(24)?;
            AAC_SAMPLE_RATES.iter().position(|&s| s == rate).map(|i| i as u8)
        } else {
            Some(index as u8)
        }
    };
    let mut aot = read_aot(&mut r)?;
    let sr_index = read_rate_index(&mut r)?;
    let channels = r.read(4)? as u8;
    if aot == 5 || aot == 29 {
        read_rate_index(&mut r)?;
        aot = read_aot(&mut r)?;
    }
    if !(1..=4).contains(&aot) || sr_index as usize >= AAC_SAMPLE_RATES.len() || channels > 7 {
        return None;
    }
    Some((aot as u8 - 1, sr_index, channels))
}

pub fn write_adts(track: &Track) -> Option<Vec<u8>> {
    let Codec::Aac { asc } = &track.codec else { return None };
    let (profile, sr_index, channels) = adts_params(asc)?;
    let mut out = Vec::with_capacity(track.samples.iter().map(|s| s.data.len() + 7).sum());
    for s in &track.samples {
        let len = s.data.len() + 7;
        if len > 0x1FFF {
            return None;
        }
        out.extend_from_slice(&[
            0xFF,
            0xF1,
            (profile << 6) | (sr_index << 2) | (channels >> 2),
            ((channels & 3) << 6) | ((len >> 11) & 3) as u8,
            ((len >> 3) & 0xFF) as u8,
            (((len & 7) << 5) | 0x1F) as u8,
            0xFC,
        ]);
        out.extend_from_slice(&s.data);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(track: u8, rel: i16, flags: u8, lace: &[u8], frames: &[&[u8]]) -> Vec<u8> {
        let mut b = vec![0x80 | track];
        b.extend_from_slice(&rel.to_be_bytes());
        b.push(flags);
        b.extend_from_slice(lace);
        for f in frames {
            b.extend_from_slice(f);
        }
        b
    }

    fn elem(id: &[u8], body: &[u8]) -> Vec<u8> {
        let mut v = id.to_vec();
        v.push(0x01);
        v.extend_from_slice(&(body.len() as u64).to_be_bytes()[1..]);
        v.extend_from_slice(body);
        v
    }

    #[test]
    fn lacing_variants() {
        let a = vec![1u8; 5];
        let b = vec![2u8; 300];
        let c = vec![3u8; 7];
        let frames: [&[u8]; 3] = [&a, &b, &c];

        let xiph = block(2, 10, 0x80 | 0x02, &[2, 5, 255, 45], &frames);
        let ebml_diff = (300i64 - 5) + ((1 << 13) - 1);
        let ebml_lace = [2u8, 0x80 | 5, 0x40 | (ebml_diff >> 8) as u8, ebml_diff as u8];
        let ebmlb = block(2, 10, 0x80 | 0x06, &ebml_lace, &frames);
        for body in [&xiph, &ebmlb] {
            let mut out = Vec::new();
            parse_block(body, None, 100, &[2], &mut out).unwrap();
            assert_eq!(out.iter().map(|f| f.data.len()).collect::<Vec<_>>(), vec![5, 300, 7]);
            assert!(out.iter().all(|f| f.ticks == 110 && f.keyframe && f.track == 2));
            assert_eq!(out[1].data[0], 2);
        }

        let four = [9u8; 4];
        let fixed = block(2, -5, 0x04, &[2], &[&four, &four, &four]);
        let mut out = Vec::new();
        parse_block(&fixed, None, 100, &[2], &mut out).unwrap();
        assert_eq!(out.len(), 3);
        assert!(out.iter().all(|f| f.data.len() == 4 && f.ticks == 95 && !f.keyframe));

        let mut out = Vec::new();
        parse_block(&fixed, None, 100, &[1], &mut out).unwrap();
        assert!(out.is_empty());
        assert!(parse_block(&block(2, 0, 0x02, &[2, 200], &[&a]), None, 0, &[2], &mut out).is_err());
    }

    #[test]
    fn cluster_walk() {
        let mut scale = vec![0x2A, 0xD7, 0xB1, 0x83];
        scale.extend_from_slice(&[0x0F, 0x42, 0x40]);
        let info = elem(&[0x15, 0x49, 0xA9, 0x66], &scale);
        let sb_neg = elem(&[0xA3], &block(1, -30, 0x80, &[], &[&[0, 0, 0, 1, 0x65]]));
        let bg_ref = elem(&[0xA0], &[elem(&[0xA1], &block(1, 5, 0, &[], &[&[7, 7]])), elem(&[0xFB], &[0xFF])].concat());
        let bg_key = elem(&[0xA0], &elem(&[0xA1], &block(1, 9, 0, &[], &[&[8]])));
        let mut cluster = vec![0x1F, 0x43, 0xB6, 0x75, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        cluster.extend(elem(&[0xE7], &[0x03, 0xE8]));
        cluster.extend(sb_neg);
        cluster.extend(elem(&[0xEC], &[0; 3]));
        cluster.extend(bg_ref);
        cluster.extend(bg_key);
        let mut cluster2 = vec![0x1F, 0x43, 0xB6, 0x75, 0xFF];
        cluster2.extend(elem(&[0xE7], &[0x07, 0xD0]));
        cluster2.extend(elem(&[0xA3], &block(1, 0, 0x80, &[], &[&[1]])));
        let mut file = elem(&[0x1A, 0x45, 0xDF, 0xA3], &[0x42, 0x82, 0x84, b'm', b'k', b'v', 0]);
        file.extend_from_slice(&[0x18, 0x53, 0x80, 0x67, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        file.extend(info);
        file.extend(cluster);
        file.extend(cluster2);

        let (frames, scale) = read_frames(&file, &[1]).unwrap();
        assert_eq!(scale, 1_000_000);
        let got: Vec<(i64, bool, usize)> = frames.iter().map(|f| (f.ticks, f.keyframe, f.data.len())).collect();
        assert_eq!(got, vec![(970, true, 5), (1005, false, 2), (1009, true, 1), (2000, true, 1)]);
    }

    #[test]
    fn adts_headers() {
        let track = Track {
            video: false,
            codec: Codec::Aac { asc: vec![0x11, 0x90, 0x56, 0xE5, 0x00] },
            width: 0,
            height: 0,
            sample_rate: 48000,
            channels: 2,
            codec_delay_ns: 0,
            samples: vec![
                Sample { pts_ns: 0, keyframe: true, data: Cow::Owned(vec![1; 300]) },
                Sample { pts_ns: 0, keyframe: true, data: Cow::Owned(vec![2; 9]) },
            ],
        };
        let adts = write_adts(&track).unwrap();
        assert_eq!(adts.len(), 300 + 9 + 14);
        assert_eq!(&adts[..7], &[0xFF, 0xF1, 0x4C, 0x80, 0x26, 0x7F, 0xFC]);
        assert_eq!(&adts[307..314], &[0xFF, 0xF1, 0x4C, 0x80, 0x02, 0x1F, 0xFC]);
        assert_eq!(adts_params(&[0x2B, 0x92, 0x08, 0x00]), Some((1, 7, 2)));
        assert_eq!(adts_params(&[0xF9, 0x48, 0x80]), None);
    }
}