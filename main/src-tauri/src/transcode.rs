use std::borrow::Cow;

use openh264::decoder::{Decoder as H264Decoder, DecoderConfig, Flush};
use openh264::encoder::{
    BitRate, Complexity, Encoder as H264Encoder, EncoderConfig, FrameRate, FrameType, IntraFramePeriod,
    RateControlMode, UsageType,
};
use openh264::formats::{YUVBuffer, YUVSource};
use openh264::{OpenH264API, Timestamp};
use symphonia::core::audio::{Channels, SampleBuffer};
use symphonia::core::codecs::{CodecParameters, DecoderOptions, CODEC_TYPE_AAC, CODEC_TYPE_MP3, CODEC_TYPE_VORBIS};
use symphonia::core::formats::Packet;

use crate::demux::{self, Codec, Sample, Track};
use crate::vpx;

pub type AacEncoder<'e> = &'e dyn Fn(&[f32], u32, u16) -> Result<Vec<u8>, String>;

const OPUS_RATE: u32 = 48_000;
const VORBIS_BLOCK: usize = 1024;

#[derive(Clone, Copy, PartialEq)]
pub enum Target {
    Mp4,
    WebM,
}

fn fits(codec: &Codec, target: Target) -> bool {
    match target {
        Target::Mp4 => matches!(codec, Codec::H264 { .. } | Codec::Aac { .. } | Codec::Mp3),
        Target::WebM => matches!(codec, Codec::Vp8 | Codec::Vp9 | Codec::Av1 { .. } | Codec::Opus { .. } | Codec::Vorbis { .. }),
    }
}

pub fn needs_transcode(tracks: &[Track], target: Target) -> bool {
    tracks.iter().any(|t| !fits(&t.codec, target))
}

pub fn prepare<'a>(
    tracks: Vec<Track<'a>>,
    target: Target,
    aac_encoder: AacEncoder,
    report: &dyn Fn(u8),
) -> Result<Vec<Track<'a>>, String> {
    let video_work: usize = tracks.iter().filter(|t| t.video && !fits(&t.codec, target)).map(|t| t.samples.len()).sum();
    let mut done = 0usize;
    let mut out = Vec::with_capacity(tracks.len());
    for t in tracks {
        if fits(&t.codec, target) {
            out.push(t);
            continue;
        }
        if t.video {
            let base = done;
            let progress = |i: usize| {
                if video_work > 0 {
                    report((10 + (base + i) * 75 / video_work).min(85) as u8);
                }
            };
            done += t.samples.len();
            out.push(transcode_video(&t, target, &progress)?);
        } else {
            out.push(transcode_audio(&t, target, aac_encoder)?);
        }
    }
    Ok(out)
}

fn thread_count() -> u32 {
    std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(4).min(16)
}

struct Picture {
    width: usize,
    height: usize,
    i420: Vec<u8>,
}

fn pack_even(width: usize, height: usize, planes: [(&[u8], usize); 3]) -> Picture {
    let w = width & !1;
    let h = height & !1;
    let mut i420 = Vec::with_capacity(w * h * 3 / 2);
    for (index, (plane, stride)) in planes.iter().enumerate() {
        let (pw, ph) = if index == 0 { (w, h) } else { (w / 2, h / 2) };
        for row in 0..ph {
            i420.extend_from_slice(&plane[row * stride..row * stride + pw]);
        }
    }
    Picture { width: w, height: h, i420 }
}

fn picture_from_openh264(yuv: &impl YUVSource) -> Picture {
    let (w, h) = yuv.dimensions();
    let (sy, su, sv) = yuv.strides();
    pack_even(w, h, [(yuv.y(), sy), (yuv.u(), su), (yuv.v(), sv)])
}

fn picture_from_vpx(frame: vpx::DecodedFrame) -> Picture {
    let (w, h) = (frame.width as usize, frame.height as usize);
    if w % 2 == 0 && h % 2 == 0 {
        return Picture { width: w, height: h, i420: frame.i420 };
    }
    let (cw, ch) = ((w + 1) / 2, (h + 1) / 2);
    let y = &frame.i420[..w * h];
    let u = &frame.i420[w * h..w * h + cw * ch];
    let v = &frame.i420[w * h + cw * ch..];
    pack_even(w, h, [(y, w), (u, cw), (v, cw)])
}

fn avcc_to_annexb_header(avcc: &[u8]) -> Result<(usize, Vec<u8>), String> {
    let bad = || "This H.264 track's decoder configuration record is damaged.".to_string();
    if avcc.len() < 7 {
        return Err(bad());
    }
    let length_size = ((avcc[4] & 0x03) + 1) as usize;
    let mut out = Vec::new();
    let mut pos = 5usize;
    for count_mask in [0x1Fu8, 0xFF] {
        let count = (*avcc.get(pos).ok_or_else(bad)? & count_mask) as usize;
        pos += 1;
        for _ in 0..count {
            let len = u16::from_be_bytes(avcc.get(pos..pos + 2).ok_or_else(bad)?.try_into().unwrap()) as usize;
            pos += 2;
            out.extend_from_slice(&[0, 0, 0, 1]);
            out.extend_from_slice(avcc.get(pos..pos + len).ok_or_else(bad)?);
            pos += len;
        }
    }
    Ok((length_size, out))
}

fn length_prefixed_to_annexb(data: &[u8], length_size: usize, out: &mut Vec<u8>) {
    let mut i = 0usize;
    while i + length_size <= data.len() {
        let len = data[i..i + length_size].iter().fold(0usize, |acc, &b| (acc << 8) | b as usize);
        i += length_size;
        if len == 0 || i + len > data.len() {
            break;
        }
        out.extend_from_slice(&[0, 0, 0, 1]);
        out.extend_from_slice(&data[i..i + len]);
        i += len;
    }
}

fn split_annexb(data: &[u8]) -> Vec<&[u8]> {
    let mut starts = Vec::new();
    let mut i = 0usize;
    while i + 3 <= data.len() {
        if data[i] == 0 && data[i + 1] == 0 && data[i + 2] == 1 {
            starts.push(i + 3);
            i += 3;
        } else {
            i += 1;
        }
    }
    let mut out = Vec::with_capacity(starts.len());
    for (k, &s) in starts.iter().enumerate() {
        let mut end = if k + 1 < starts.len() { starts[k + 1] - 3 } else { data.len() };
        while end > s && data[end - 1] == 0 {
            end -= 1;
        }
        if end > s {
            out.push(&data[s..end]);
        }
    }
    out
}

enum VideoDecoder {
    H264 { decoder: H264Decoder, length_size: usize, header: Vec<u8> },
    Vpx(vpx::VpxDecoder),
}

impl VideoDecoder {
    fn new(codec: &Codec) -> Result<Self, String> {
        match codec {
            Codec::H264 { avcc } => {
                let (length_size, header) = avcc_to_annexb_header(avcc)?;
                let config = DecoderConfig::new().flush_after_decode(Flush::NoFlush);
                let decoder = H264Decoder::with_api_config(OpenH264API::from_source(), config)
                    .map_err(|e| format!("Couldn't start the H.264 decoder: {e}"))?;
                Ok(VideoDecoder::H264 { decoder, length_size, header })
            }
            Codec::Vp8 => Ok(VideoDecoder::Vpx(vpx::VpxDecoder::new(true, thread_count())?)),
            Codec::Vp9 => Ok(VideoDecoder::Vpx(vpx::VpxDecoder::new(false, thread_count())?)),
            other => Err(format!("{} video can't be decoded yet, so this file can't be converted.", other.name())),
        }
    }

    fn decode(&mut self, sample: &Sample, first: bool, out: &mut Vec<Picture>) -> Result<(), String> {
        match self {
            VideoDecoder::H264 { decoder, length_size, header } => {
                let mut annexb = Vec::with_capacity(sample.data.len() + header.len() + 16);
                if first || sample.keyframe {
                    annexb.extend_from_slice(header);
                }
                length_prefixed_to_annexb(&sample.data, *length_size, &mut annexb);
                if let Some(yuv) = decoder.decode(&annexb).map_err(|e| format!("H.264 decode error: {e}"))? {
                    out.push(picture_from_openh264(&yuv));
                }
            }
            VideoDecoder::Vpx(decoder) => {
                out.extend(decoder.decode_packet(&sample.data)?.into_iter().map(picture_from_vpx));
            }
        }
        Ok(())
    }

    fn finish(&mut self, out: &mut Vec<Picture>) -> Result<(), String> {
        if let VideoDecoder::H264 { decoder, .. } = self {
            for yuv in decoder.flush_remaining().map_err(|e| format!("H.264 decode error: {e}"))? {
                out.push(picture_from_openh264(&yuv));
            }
        }
        Ok(())
    }
}

struct EncodedPacket {
    pts_ms: i64,
    keyframe: bool,
    data: Vec<u8>,
}

enum VideoEncoder {
    Vp9(vpx::Vp9Encoder),
    H264 { encoder: H264Encoder, sps: Option<Vec<u8>>, pps: Option<Vec<u8>> },
}

impl VideoEncoder {
    fn new(target: Target, width: usize, height: usize, bitrate_bps: u64, fps: f32) -> Result<Self, String> {
        match target {
            Target::WebM => {
                let kbps = (bitrate_bps / 1000).clamp(200, 20_000) as u32;
                Ok(VideoEncoder::Vp9(vpx::Vp9Encoder::new(width as u32, height as u32, kbps, thread_count())?))
            }
            Target::Mp4 => {
                let config = EncoderConfig::new()
                    .bitrate(BitRate::from_bps(bitrate_bps.clamp(300_000, 40_000_000) as u32))
                    .max_frame_rate(FrameRate::from_hz(fps))
                    .rate_control_mode(RateControlMode::Bitrate)
                    .skip_frames(false)
                    .usage_type(UsageType::CameraVideoRealTime)
                    .complexity(Complexity::High)
                    .intra_frame_period(IntraFramePeriod::from_num_frames((fps * 2.0).round().max(1.0) as u32))
                    .num_threads(thread_count() as u16);
                let encoder = H264Encoder::with_api_config(OpenH264API::from_source(), config)
                    .map_err(|e| format!("Couldn't start the H.264 encoder: {e}"))?;
                Ok(VideoEncoder::H264 { encoder, sps: None, pps: None })
            }
        }
    }

    fn encode(&mut self, picture: Picture, pts_ms: i64, duration_ms: u64) -> Result<Vec<EncodedPacket>, String> {
        match self {
            VideoEncoder::Vp9(encoder) => Ok(encoder
                .encode_frame(pts_ms, duration_ms, &picture.i420)?
                .into_iter()
                .map(|f| EncodedPacket { pts_ms: f.pts, keyframe: f.keyframe, data: f.data })
                .collect()),
            VideoEncoder::H264 { encoder, sps, pps } => {
                let yuv = YUVBuffer::from_vec(picture.i420, picture.width, picture.height);
                let stream = encoder
                    .encode_at(&yuv, Timestamp::from_millis(pts_ms.max(0) as u64))
                    .map_err(|e| format!("H.264 encode error: {e}"))?;
                if matches!(stream.frame_type(), FrameType::Skip | FrameType::Invalid) {
                    return Ok(Vec::new());
                }
                let annexb = stream.to_vec();
                let mut data = Vec::with_capacity(annexb.len() + 16);
                let mut keyframe = false;
                for nal in split_annexb(&annexb) {
                    match nal[0] & 0x1F {
                        7 => {
                            sps.get_or_insert_with(|| nal.to_vec());
                        }
                        8 => {
                            pps.get_or_insert_with(|| nal.to_vec());
                        }
                        kind => {
                            keyframe |= kind == 5;
                            data.extend_from_slice(&(nal.len() as u32).to_be_bytes());
                            data.extend_from_slice(nal);
                        }
                    }
                }
                if data.is_empty() {
                    return Ok(Vec::new());
                }
                Ok(vec![EncodedPacket { pts_ms, keyframe, data }])
            }
        }
    }

    fn finish(&mut self) -> Result<(Vec<EncodedPacket>, Codec), String> {
        match self {
            VideoEncoder::Vp9(encoder) => Ok((
                encoder
                    .flush()?
                    .into_iter()
                    .map(|f| EncodedPacket { pts_ms: f.pts, keyframe: f.keyframe, data: f.data })
                    .collect(),
                Codec::Vp9,
            )),
            VideoEncoder::H264 { sps, pps, .. } => {
                let (Some(sps), Some(pps)) = (sps.as_ref(), pps.as_ref()) else {
                    return Err("The H.264 encoder didn't produce a sequence header.".to_string());
                };
                let mut avcc = vec![1, sps[1], sps[2], sps[3], 0xFF, 0xE1];
                avcc.extend_from_slice(&(sps.len() as u16).to_be_bytes());
                avcc.extend_from_slice(sps);
                avcc.push(1);
                avcc.extend_from_slice(&(pps.len() as u16).to_be_bytes());
                avcc.extend_from_slice(pps);
                Ok((Vec::new(), Codec::H264 { avcc }))
            }
        }
    }
}

fn transcode_video(t: &Track, target: Target, progress: &dyn Fn(usize)) -> Result<Track<'static>, String> {
    let mut decoder = VideoDecoder::new(&t.codec)?;
    let base_ns = t.samples.iter().map(|s| s.pts_ns).min().unwrap_or(0);
    let mut times: Vec<i64> = t.samples.iter().map(|s| (s.pts_ns - base_ns + 500_000) / 1_000_000).collect();
    times.sort_unstable();
    for i in 1..times.len() {
        if times[i] <= times[i - 1] {
            times[i] = times[i - 1] + 1;
        }
    }
    let n = times.len();
    let frame_ms = if n >= 2 { ((times[n - 1] - times[0]) as f64 / (n - 1) as f64).max(1.0) } else { 33.0 };
    let fps = (1000.0 / frame_ms) as f32;
    let bitrate = match target {
        Target::WebM => t.bitrate_bps() * 4 / 5,
        Target::Mp4 => t.bitrate_bps() * 3 / 2,
    };

    let mut encoder: Option<VideoEncoder> = None;
    let mut size: Option<(usize, usize)> = None;
    let mut packets: Vec<EncodedPacket> = Vec::new();
    let mut shown = 0usize;
    let mut pictures: Vec<Picture> = Vec::new();

    let mut encode_pictures = |pictures: &mut Vec<Picture>,
                               encoder: &mut Option<VideoEncoder>,
                               packets: &mut Vec<EncodedPacket>,
                               shown: &mut usize|
     -> Result<(), String> {
        for picture in pictures.drain(..) {
            if picture.width == 0 || picture.height == 0 {
                continue;
            }
            let (w, h) = *size.get_or_insert((picture.width, picture.height));
            if (picture.width, picture.height) != (w, h) {
                return Err("This video changes resolution midway, which isn't supported yet.".to_string());
            }
            if encoder.is_none() {
                *encoder = Some(VideoEncoder::new(target, w, h, bitrate, fps)?);
            }
            let pts = times.get(*shown).copied().unwrap_or_else(|| times.last().copied().unwrap_or(0) + (*shown as f64 * frame_ms) as i64);
            let next = times.get(*shown + 1).copied().unwrap_or(pts + frame_ms.round() as i64);
            *shown += 1;
            packets.extend(encoder.as_mut().unwrap().encode(picture, pts, (next - pts).max(1) as u64)?);
        }
        Ok(())
    };

    for (i, sample) in t.samples.iter().enumerate() {
        decoder.decode(sample, i == 0, &mut pictures)?;
        encode_pictures(&mut pictures, &mut encoder, &mut packets, &mut shown)?;
        if i % 8 == 0 {
            progress(i);
        }
    }
    decoder.finish(&mut pictures)?;
    encode_pictures(&mut pictures, &mut encoder, &mut packets, &mut shown)?;

    let mut encoder = encoder.ok_or_else(|| "No video frames could be decoded from this file.".to_string())?;
    let (tail, codec) = encoder.finish()?;
    packets.extend(tail);
    let (width, height) = size.unwrap_or((0, 0));
    progress(t.samples.len());

    Ok(Track {
        video: true,
        codec,
        width: width as u32,
        height: height as u32,
        sample_rate: 0,
        channels: 0,
        codec_delay_ns: 0,
        samples: packets
            .into_iter()
            .map(|p| Sample { pts_ns: base_ns + p.pts_ms * 1_000_000, keyframe: p.keyframe, data: Cow::Owned(p.data) })
            .collect(),
    })
}

fn vorbis_extra_data(private: &[u8]) -> Result<Vec<u8>, String> {
    let bad = || "This Vorbis track's headers are damaged.".to_string();
    let count = *private.first().ok_or_else(bad)? as usize + 1;
    let mut pos = 1usize;
    let mut sizes = Vec::new();
    for _ in 0..count - 1 {
        let mut size = 0usize;
        loop {
            let b = *private.get(pos).ok_or_else(bad)?;
            pos += 1;
            size += b as usize;
            if b != 255 {
                break;
            }
        }
        sizes.push(size);
    }
    let mut packets = Vec::new();
    for size in sizes {
        packets.push(private.get(pos..pos + size).ok_or_else(bad)?);
        pos += size;
    }
    packets.push(&private[pos.min(private.len())..]);
    let ident = packets.iter().find(|p| p.first() == Some(&1)).ok_or_else(bad)?;
    let setup = packets.iter().find(|p| p.first() == Some(&5)).ok_or_else(bad)?;
    Ok([*ident, *setup].concat())
}

pub fn opus_packet_samples(packet: &[u8]) -> usize {
    let Some(&toc) = packet.first() else { return 0 };
    let config = toc >> 3;
    let per_frame_48k = match config {
        0..=11 => [480, 960, 1920, 2880][(config % 4) as usize],
        12..=15 => [480, 960][(config % 2) as usize],
        _ => [120, 240, 480, 960][(config % 4) as usize],
    };
    let frames = match toc & 0x03 {
        0 => 1,
        1 | 2 => 2,
        _ => packet.get(1).map(|b| (b & 0x3F) as usize).unwrap_or(1),
    };
    (per_frame_48k * frames).min(5760)
}

fn decode_audio(t: &Track) -> Result<(Vec<f32>, u32, u16), String> {
    if let Codec::Opus { head } = &t.codec {
        let channels = *head.get(9).unwrap_or(&2) as usize;
        if channels == 0 || channels > 2 {
            return Err("Only mono or stereo Opus audio can be converted.".to_string());
        }
        let mut decoder = opus_rs::OpusDecoder::new(OPUS_RATE as i32, channels)
            .map_err(|e| format!("Couldn't create the Opus decoder: {e}"))?;
        let mut buf = vec![0.0f32; 5760 * channels];
        let mut pcm = Vec::new();
        for s in &t.samples {
            if s.data.is_empty() {
                continue;
            }
            let got = decoder
                .decode(&s.data, opus_packet_samples(&s.data), &mut buf)
                .map_err(|e| format!("Opus decode error: {e}"))?;
            pcm.extend_from_slice(&buf[..got * channels]);
        }
        return Ok((pcm, OPUS_RATE, channels as u16));
    }

    let (codec_type, extra) = match &t.codec {
        Codec::Aac { asc } => (CODEC_TYPE_AAC, asc.clone()),
        Codec::Mp3 => (CODEC_TYPE_MP3, Vec::new()),
        Codec::Vorbis { headers } => (CODEC_TYPE_VORBIS, vorbis_extra_data(headers)?),
        other => return Err(format!("{} audio can't be decoded yet, so this file can't be converted.", other.name())),
    };
    let mut params = CodecParameters::new();
    params.for_codec(codec_type).with_sample_rate(t.sample_rate.max(1));
    if t.channels > 0 {
        params.with_channels(Channels::from_bits_truncate((1u32 << t.channels.min(18)) - 1));
    }
    if !extra.is_empty() {
        params.with_extra_data(extra.into_boxed_slice());
    }
    let mut decoder = symphonia::default::get_codecs()
        .make(&params, &DecoderOptions::default())
        .map_err(|e| format!("Couldn't start the {} decoder: {e}", t.codec.name()))?;

    let mut pcm = Vec::new();
    let mut rate = t.sample_rate;
    let mut channels = t.channels;
    let mut buf: Option<SampleBuffer<f32>> = None;
    for (i, s) in t.samples.iter().enumerate() {
        let packet = Packet::new_from_slice(0, i as u64, 0, &s.data);
        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(format!("{} decode error: {e}", t.codec.name())),
        };
        let spec = *decoded.spec();
        rate = spec.rate;
        channels = spec.channels.count() as u16;
        if buf.as_ref().map(|b| b.capacity() < decoded.capacity() * channels as usize).unwrap_or(true) {
            buf = Some(SampleBuffer::<f32>::new(decoded.capacity() as u64, spec));
        }
        let b = buf.as_mut().unwrap();
        b.copy_interleaved_ref(decoded);
        pcm.extend_from_slice(b.samples());
    }
    if pcm.is_empty() {
        return Err("No audio could be decoded from this file.".to_string());
    }
    Ok((pcm, rate, channels))
}

fn to_stereo_or_mono(pcm: Vec<f32>, channels: u16) -> (Vec<f32>, u16) {
    let c = channels as usize;
    if c <= 2 {
        return (pcm, channels.max(1));
    }
    let mut out = Vec::with_capacity(pcm.len() / c * 2);
    for frame in pcm.chunks_exact(c) {
        let (l, r) = if c == 6 {
            let k = std::f32::consts::FRAC_1_SQRT_2;
            (frame[0] + k * frame[2] + k * frame[4], frame[1] + k * frame[2] + k * frame[5])
        } else {
            let even: f32 = frame.iter().step_by(2).sum::<f32>() / ((c + 1) / 2) as f32;
            let odd: f32 = frame.iter().skip(1).step_by(2).sum::<f32>() / (c / 2) as f32;
            (even, odd)
        };
        out.push(l.clamp(-1.0, 1.0));
        out.push(r.clamp(-1.0, 1.0));
    }
    (out, 2)
}

fn transcode_audio(t: &Track, target: Target, aac_encoder: AacEncoder) -> Result<Track<'static>, String> {
    let (pcm, rate, channels) = decode_audio(t)?;
    let delay_ns = match &t.codec {
        Codec::Opus { head } if t.codec_delay_ns <= 0 && head.len() >= 12 => {
            u16::from_le_bytes([head[10], head[11]]) as i64 * 1_000_000_000 / OPUS_RATE as i64
        }
        _ => t.codec_delay_ns.max(0),
    };
    let skip = ((delay_ns as i128 * rate as i128 / 1_000_000_000) as usize * channels.max(1) as usize).min(pcm.len());
    let (pcm, channels) = to_stereo_or_mono(pcm[skip..].to_vec(), channels);
    let start_ns = t.samples.iter().map(|s| s.pts_ns).min().unwrap_or(0);

    match target {
        Target::WebM => encode_vorbis_track(&pcm, rate, channels, start_ns),
        Target::Mp4 => {
            let bytes = aac_encoder(&pcm, rate, channels)?;
            let encoded = demux::read_mp4(&bytes)?;
            let track = encoded
                .into_iter()
                .find(|e| matches!(e.codec, Codec::Aac { .. }))
                .ok_or_else(|| "The AAC encoder didn't produce an audio track.".to_string())?;
            Ok(Track {
                video: false,
                codec: track.codec,
                width: 0,
                height: 0,
                sample_rate: track.sample_rate,
                channels: track.channels,
                codec_delay_ns: track.codec_delay_ns,
                samples: track
                    .samples
                    .into_iter()
                    .map(|s| Sample { pts_ns: s.pts_ns + start_ns, keyframe: true, data: Cow::Owned(s.data.into_owned()) })
                    .collect(),
            })
        }
    }
}

fn xiph_lace_length(len: usize, out: &mut Vec<u8>) {
    out.extend(std::iter::repeat(255u8).take(len / 255));
    out.push((len % 255) as u8);
}

fn vorbis_mode_blockflags(setup: &[u8]) -> Option<Vec<bool>> {
    let bits: Vec<bool> = setup.iter().rev().flat_map(|&b| (0..8).rev().map(move |i| (b >> i) & 1 == 1)).collect();
    let read = |pos: usize, n: usize| -> Option<u32> {
        bits.get(pos..pos + n).map(|s| s.iter().fold(0u32, |acc, &b| (acc << 1) | b as u32))
    };
    let mut pos = 0usize;
    while bits.len() - pos > 97 && !bits[pos] {
        pos += 1;
    }
    pos += 1;
    let framing_end = pos;
    let mut count = 0usize;
    let mut last_valid = 0usize;
    while bits.len() >= pos + 97 {
        if read(pos, 8)? > 63 || read(pos + 8, 16)? != 0 || read(pos + 24, 16)? != 0 {
            break;
        }
        pos += 41;
        count += 1;
        if count > 64 {
            break;
        }
        if read(pos, 6)? as usize + 1 == count {
            last_valid = count;
        }
    }
    if last_valid == 0 {
        return None;
    }
    let mut flags = vec![false; last_valid];
    let mut pos = framing_end;
    for i in (0..last_valid).rev() {
        pos += 40;
        flags[i] = *bits.get(pos)?;
        pos += 1;
    }
    Some(flags)
}

fn encode_vorbis_track(pcm: &[f32], rate: u32, channels: u16, start_ns: i64) -> Result<Track<'static>, String> {
    use std::num::{NonZeroU32, NonZeroU8};

    let c = channels as usize;
    let mut ogg_bytes: Vec<u8> = Vec::new();
    {
        let sr = NonZeroU32::new(rate).ok_or_else(|| "Invalid sample rate.".to_string())?;
        let ch = NonZeroU8::new(c as u8).ok_or_else(|| "Invalid channel count.".to_string())?;
        let mut builder = vorbis_rs::VorbisEncoderBuilder::new(sr, ch, &mut ogg_bytes)
            .map_err(|e| format!("Vorbis setup error: {e}"))?;
        let mut encoder = builder.build().map_err(|e| format!("Vorbis build error: {e}"))?;
        let planar: Vec<Vec<f32>> = (0..c).map(|ch| pcm.iter().skip(ch).step_by(c).copied().collect()).collect();
        let total = planar[0].len();
        let mut pos = 0usize;
        while pos < total {
            let end = (pos + VORBIS_BLOCK).min(total);
            let block: Vec<&[f32]> = planar.iter().map(|p| &p[pos..end]).collect();
            encoder.encode_audio_block(&block).map_err(|e| format!("Vorbis encode error: {e}"))?;
            pos = end;
        }
        encoder.finish().map_err(|e| format!("Vorbis finish error: {e}"))?;
    }

    let mut reader = ogg::reading::PacketReader::new(std::io::Cursor::new(ogg_bytes));
    let mut packets: Vec<Vec<u8>> = Vec::new();
    while let Some(p) = reader.read_packet().map_err(|e| format!("Couldn't read the Vorbis stream: {e}"))? {
        packets.push(p.data);
    }
    if packets.len() < 3 || packets[0].len() < 30 {
        return Err("The Vorbis encoder didn't produce valid headers.".to_string());
    }
    let audio_packets = packets.split_off(3);
    let blocksizes = [1u32 << (packets[0][28] & 0x0F), 1u32 << (packets[0][28] >> 4)];
    let modes = vorbis_mode_blockflags(&packets[2]).ok_or_else(|| "Couldn't read the Vorbis setup header.".to_string())?;
    let mode_bits = usize::BITS - (modes.len().max(2) - 1).leading_zeros();

    let mut private = vec![2u8];
    xiph_lace_length(packets[0].len(), &mut private);
    xiph_lace_length(packets[1].len(), &mut private);
    for p in &packets {
        private.extend_from_slice(p);
    }

    let mut position: u64 = 0;
    let mut previous: Option<u32> = None;
    let mut samples = Vec::with_capacity(audio_packets.len());
    for data in audio_packets {
        let Some(&first) = data.first() else { continue };
        if first & 1 == 1 {
            continue;
        }
        let mode = if modes.len() == 1 { 0 } else { ((first >> 1) as usize) & ((1usize << mode_bits) - 1) };
        let current = blocksizes[*modes.get(mode).unwrap_or(&false) as usize];
        samples.push(Sample {
            pts_ns: start_ns + (position as i128 * 1_000_000_000 / rate as i128) as i64,
            keyframe: true,
            data: Cow::Owned(data),
        });
        if let Some(prev) = previous {
            position += ((prev + current) / 4) as u64;
        }
        previous = Some(current);
    }

    Ok(Track {
        video: false,
        codec: Codec::Vorbis { headers: private },
        width: 0,
        height: 0,
        sample_rate: rate,
        channels,
        codec_delay_ns: 0,
        samples,
    })
}
