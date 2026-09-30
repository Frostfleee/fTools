use std::io::Write;

use crate::demux::{Codec, Track};
use crate::ebml::{self, build_elem, uint_body};

const ID_SEEK_HEAD: u32 = 0x114D_9B74;
const ID_SEEK: u32 = 0x4DBB;
const ID_SEEK_ID: u32 = 0x53AB;
const ID_SEEK_POSITION: u32 = 0x53AC;
const ID_CUES: u32 = 0x1C53_BB6B;
const ID_CUE_POINT: u32 = 0xBB;
const ID_CUE_TIME: u32 = 0xB3;
const ID_CUE_TRACK_POSITIONS: u32 = 0xB7;
const ID_CUE_TRACK: u32 = 0xF7;
const ID_CUE_CLUSTER_POSITION: u32 = 0xF1;
const ID_FLAG_LACING: u32 = 0x9C;

const CLUSTER_MAX_MS: i64 = 5000;
const OPUS_SEEK_PREROLL_NS: u64 = 80_000_000;

fn codec_id(codec: &Codec, webm: bool) -> Result<(&'static str, Vec<u8>), String> {
    let (id, private, webm_ok) = match codec {
        Codec::Vp8 => ("V_VP8", Vec::new(), true),
        Codec::Vp9 => ("V_VP9", Vec::new(), true),
        Codec::Av1 { config } => ("V_AV1", config.clone(), true),
        Codec::Opus { head } => ("A_OPUS", head.clone(), true),
        Codec::Vorbis { headers } => ("A_VORBIS", headers.clone(), true),
        Codec::H264 { avcc } => ("V_MPEG4/ISO/AVC", avcc.clone(), false),
        Codec::Hevc { hvcc } => ("V_MPEGH/ISO/HEVC", hvcc.clone(), false),
        Codec::Aac { asc } => ("A_AAC", asc.clone(), false),
        Codec::Mp3 => ("A_MPEG/L3", Vec::new(), false),
        Codec::Unsupported(name) => return Err(format!("{name} can't be written to this container.")),
    };
    if webm && !webm_ok {
        return Err(format!("WEBM can't hold {} without re-encoding.", codec.name()));
    }
    Ok((id, private))
}

fn fixed_uint(v: u64) -> Vec<u8> {
    v.to_be_bytes().to_vec()
}

fn id_bytes(id: u32) -> Vec<u8> {
    let bytes = id.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(3);
    bytes[start..].to_vec()
}

pub fn write<W: Write>(tracks: &[Track], webm: bool, out: &mut W) -> Result<(), String> {
    let tracks: Vec<&Track> = tracks.iter().filter(|t| !t.samples.is_empty()).collect();
    if tracks.is_empty() {
        return Err("No audio or video frames were found in this file.".to_string());
    }

    let mut tracks_body = Vec::new();
    for (i, t) in tracks.iter().enumerate() {
        let number = i as u64 + 1;
        let (id, private) = codec_id(&t.codec, webm)?;
        let mut te = Vec::new();
        te.extend(build_elem(ebml::ID_TRACK_NUMBER, &uint_body(number)));
        te.extend(build_elem(ebml::ID_TRACK_UID, &uint_body(number)));
        te.extend(build_elem(ebml::ID_TRACK_TYPE, &uint_body(if t.video { 1 } else { 2 })));
        te.extend(build_elem(ID_FLAG_LACING, &uint_body(0)));
        te.extend(build_elem(ebml::ID_CODEC_ID, id.as_bytes()));
        if !private.is_empty() {
            te.extend(build_elem(ebml::ID_CODEC_PRIVATE, &private));
        }
        if t.codec_delay_ns > 0 {
            te.extend(build_elem(ebml::ID_CODEC_DELAY, &uint_body(t.codec_delay_ns as u64)));
        }
        if matches!(t.codec, Codec::Opus { .. }) {
            te.extend(build_elem(ebml::ID_SEEK_PREROLL, &uint_body(OPUS_SEEK_PREROLL_NS)));
        }
        if t.video {
            let mut video = build_elem(ebml::ID_PIXEL_WIDTH, &uint_body(t.width as u64));
            video.extend(build_elem(ebml::ID_PIXEL_HEIGHT, &uint_body(t.height as u64)));
            te.extend(build_elem(ebml::ID_VIDEO, &video));
        } else {
            let mut audio = build_elem(ebml::ID_SAMPLING_FREQUENCY, &ebml::float_body_f64(t.sample_rate as f64));
            audio.extend(build_elem(ebml::ID_CHANNELS, &uint_body(t.channels.max(1) as u64)));
            te.extend(build_elem(ebml::ID_AUDIO, &audio));
        }
        tracks_body.extend(build_elem(ebml::ID_TRACK_ENTRY, &te));
    }

    let base_ns = tracks.iter().flat_map(|t| t.samples.iter().map(|s| s.pts_ns)).min().unwrap_or(0).min(0);
    let to_ms = |ns: i64| (ns - base_ns + 500_000).div_euclid(1_000_000);

    struct Entry {
        key: i64,
        ts: i64,
        track: usize,
        index: usize,
    }
    let mut entries: Vec<Entry> = Vec::new();
    for (ti, t) in tracks.iter().enumerate() {
        let mut running = i64::MIN;
        for (si, s) in t.samples.iter().enumerate() {
            let ts = to_ms(s.pts_ns);
            running = running.max(ts);
            entries.push(Entry { key: running, ts, track: ti, index: si });
        }
    }
    entries.sort_by(|a, b| a.key.cmp(&b.key).then(a.track.cmp(&b.track)).then(a.index.cmp(&b.index)));

    let cue_track = tracks.iter().position(|t| t.video);
    let mut clusters: Vec<u8> = Vec::new();
    let mut cues: Vec<(i64, usize, u64)> = Vec::new();
    let mut i = 0usize;
    while i < entries.len() {
        let cluster_ts = entries[i].key.max(0);
        let first = &entries[i];
        let starts_on_key = match cue_track {
            Some(v) => first.track == v && tracks[v].samples[first.index].keyframe,
            None => true,
        };
        let mut body = build_elem(ebml::ID_TIMESTAMP, &uint_body(cluster_ts as u64));
        let mut j = i;
        while j < entries.len() {
            let e = &entries[j];
            let rel = e.ts - cluster_ts;
            let sample = &tracks[e.track].samples[e.index];
            let is_video_key = cue_track == Some(e.track) && sample.keyframe;
            if j > i && (e.key - cluster_ts >= CLUSTER_MAX_MS || is_video_key) {
                break;
            }
            if !(i16::MIN as i64..=i16::MAX as i64).contains(&rel) {
                if j == i {
                    return Err("A frame's timestamp is too far from its neighbours to be stored.".to_string());
                }
                break;
            }
            let keyframe = sample.keyframe || !tracks[e.track].video;
            let block = ebml::simple_block_body(e.track as u64 + 1, rel as i16, keyframe, &sample.data);
            body.extend(build_elem(ebml::ID_SIMPLE_BLOCK, &block));
            j += 1;
        }
        if starts_on_key {
            cues.push((cluster_ts, cue_track.unwrap_or(0) + 1, clusters.len() as u64));
        }
        clusters.extend(build_elem(ebml::ID_CLUSTER, &body));
        i = j;
    }

    let end_ms = tracks.iter().map(|t| to_ms(t.end_ns() + t.codec_delay_ns)).max().unwrap_or(0);
    let mut info = build_elem(ebml::ID_TIMESTAMP_SCALE, &uint_body(1_000_000));
    info.extend(build_elem(ebml::ID_DURATION, &ebml::float_body_f64(end_ms.max(1) as f64)));
    info.extend(build_elem(ebml::ID_MUXING_APP, b"fTools"));
    info.extend(build_elem(ebml::ID_WRITING_APP, b"fTools"));
    let info = build_elem(ebml::ID_INFO, &info);
    let tracks_elem = build_elem(ebml::ID_TRACKS, &tracks_body);

    let seek = |id: u32, pos: u64| {
        let mut s = build_elem(ID_SEEK_ID, &id_bytes(id));
        s.extend(build_elem(ID_SEEK_POSITION, &fixed_uint(pos)));
        build_elem(ID_SEEK, &s)
    };
    let seek_head_len = build_elem(ID_SEEK_HEAD, &[seek(ebml::ID_INFO, 0), seek(ebml::ID_TRACKS, 0), seek(ID_CUES, 0)].concat()).len() as u64;
    let info_pos = seek_head_len;
    let tracks_pos = info_pos + info.len() as u64;
    let clusters_pos = tracks_pos + tracks_elem.len() as u64;
    let cues_pos = clusters_pos + clusters.len() as u64;
    let seek_head = build_elem(
        ID_SEEK_HEAD,
        &[seek(ebml::ID_INFO, info_pos), seek(ebml::ID_TRACKS, tracks_pos), seek(ID_CUES, cues_pos)].concat(),
    );

    let mut cues_body = Vec::new();
    for (time, track, offset) in cues {
        let mut positions = build_elem(ID_CUE_TRACK, &uint_body(track as u64));
        positions.extend(build_elem(ID_CUE_CLUSTER_POSITION, &uint_body(clusters_pos + offset)));
        let mut point = build_elem(ID_CUE_TIME, &uint_body(time as u64));
        point.extend(build_elem(ID_CUE_TRACK_POSITIONS, &positions));
        cues_body.extend(build_elem(ID_CUE_POINT, &point));
    }

    let mut segment = seek_head;
    segment.extend(info);
    segment.extend(tracks_elem);
    segment.extend(clusters);
    segment.extend(build_elem(ID_CUES, &cues_body));

    let doctype: &[u8] = if webm { b"webm" } else { b"matroska" };
    let mut header = build_elem(ebml::ID_EBML_VERSION, &uint_body(1));
    header.extend(build_elem(ebml::ID_EBML_READ_VERSION, &uint_body(1)));
    header.extend(build_elem(ebml::ID_EBML_MAX_ID_LENGTH, &uint_body(4)));
    header.extend(build_elem(ebml::ID_EBML_MAX_SIZE_LENGTH, &uint_body(8)));
    header.extend(build_elem(ebml::ID_DOCTYPE, doctype));
    header.extend(build_elem(ebml::ID_DOCTYPE_VERSION, &uint_body(4)));
    header.extend(build_elem(ebml::ID_DOCTYPE_READ_VERSION, &uint_body(2)));

    let write_err = |e: std::io::Error| format!("Couldn't write the output file: {e}");
    out.write_all(&build_elem(ebml::ID_EBML, &header)).map_err(write_err)?;
    out.write_all(&build_elem(ebml::ID_SEGMENT, &segment)).map_err(write_err)?;
    Ok(())
}