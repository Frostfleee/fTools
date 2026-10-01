use std::io::{Cursor, Write};

use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::imageops::FilterType;
use image::{AnimationDecoder, Delay, Frame, ImageDecoder, RgbaImage};

use crate::demux::Track;
use crate::transcode::{self, EncodedPacket, Picture, Target, VideoEncoder};

const GIF_MAX_SIDE: u32 = 480;
const GIF_MIN_FRAME_MS: f64 = 1000.0 / 15.0;
const GIF_ENCODE_SPEED: i32 = 10;
const GIF_MIN_DELAY_MS: u64 = 20;
const GIF_FALLBACK_DELAY_MS: u64 = 100;
const STILL_GIF_DURATION_MS: u64 = 1000;

fn uses_bt709(height: usize) -> bool {
    height >= 720
}

fn picture_to_rgba(p: &Picture) -> RgbaImage {
    let (w, h) = (p.width, p.height);
    let (y_plane, chroma) = p.i420.split_at(w * h);
    let (u_plane, v_plane) = chroma.split_at((w / 2) * (h / 2));
    let (rv, gu, gv, bu) = if uses_bt709(h) { (459, 55, 136, 541) } else { (409, 100, 208, 516) };
    let mut rgba = vec![0u8; w * h * 4];
    for row in 0..h {
        for col in 0..w {
            let c = 298 * (y_plane[row * w + col] as i32 - 16);
            let ci = (row / 2) * (w / 2) + col / 2;
            let d = u_plane[ci] as i32 - 128;
            let e = v_plane[ci] as i32 - 128;
            let o = (row * w + col) * 4;
            rgba[o] = ((c + rv * e + 128) >> 8).clamp(0, 255) as u8;
            rgba[o + 1] = ((c - gu * d - gv * e + 128) >> 8).clamp(0, 255) as u8;
            rgba[o + 2] = ((c + bu * d + 128) >> 8).clamp(0, 255) as u8;
            rgba[o + 3] = 255;
        }
    }
    RgbaImage::from_raw(w as u32, h as u32, rgba).expect("buffer matches dimensions")
}

pub(crate) fn rgba_to_picture(img: &RgbaImage) -> Picture {
    let src_w = img.width() as usize;
    let (w, h) = (src_w & !1, img.height() as usize & !1);
    let raw = img.as_raw();
    let rgb = |x: usize, y: usize| -> [i32; 3] {
        let o = (y * src_w + x) * 4;
        let a = raw[o + 3] as i32;
        [raw[o] as i32 * a / 255, raw[o + 1] as i32 * a / 255, raw[o + 2] as i32 * a / 255]
    };
    let (ky, ku, kv): ([i32; 3], [i32; 3], [i32; 3]) = if uses_bt709(h) {
        ([47, 157, 16], [-26, -87, 112], [112, -102, -10])
    } else {
        ([66, 129, 25], [-38, -74, 112], [112, -94, -18])
    };
    let dot = |k: [i32; 3], c: [i32; 3]| k[0] * c[0] + k[1] * c[1] + k[2] * c[2];

    let mut i420 = Vec::with_capacity(w * h * 3 / 2);
    for y in 0..h {
        for x in 0..w {
            i420.push((((dot(ky, rgb(x, y)) + 128) >> 8) + 16).clamp(0, 255) as u8);
        }
    }
    let mut u_plane = Vec::with_capacity(w * h / 4);
    let mut v_plane = Vec::with_capacity(w * h / 4);
    for y in (0..h).step_by(2) {
        for x in (0..w).step_by(2) {
            let block = [rgb(x, y), rgb(x + 1, y), rgb(x, y + 1), rgb(x + 1, y + 1)];
            let avg = [0, 1, 2].map(|i| (block.iter().map(|p| p[i]).sum::<i32>() + 2) / 4);
            u_plane.push((((dot(ku, avg) + 128) >> 8) + 128).clamp(0, 255) as u8);
            v_plane.push((((dot(kv, avg) + 128) >> 8) + 128).clamp(0, 255) as u8);
        }
    }
    i420.extend(u_plane);
    i420.extend(v_plane);
    Picture { width: w, height: h, i420 }
}

fn gif_size(width: usize, height: usize) -> (u32, u32) {
    let longest = width.max(height) as f64;
    if longest <= GIF_MAX_SIDE as f64 {
        return (width as u32, height as u32);
    }
    let scale = GIF_MAX_SIDE as f64 / longest;
    (((width as f64 * scale).round() as u32).max(1), ((height as f64 * scale).round() as u32).max(1))
}

pub fn video_to_gif<W: Write>(tracks: &[Track], out: W, progress: &dyn Fn(usize)) -> Result<(), String> {
    let track = tracks
        .iter()
        .find(|t| t.video)
        .ok_or_else(|| "This file has no video track to turn into a GIF.".to_string())?;
    let gif_err = |e: image::ImageError| format!("Couldn't write the GIF: {e}");

    let mut encoder = GifEncoder::new_with_speed(out, GIF_ENCODE_SPEED);
    encoder.set_repeat(Repeat::Infinite).map_err(gif_err)?;

    let write_frame = |encoder: &mut GifEncoder<W>, image: RgbaImage, start: i64, end: i64| {
        let delay_cs = ((end + 5) / 10 - (start + 5) / 10).max(2) as u32;
        encoder.encode_frame(Frame::from_parts(image, 0, 0, Delay::from_numer_denom_ms(delay_cs * 10, 1))).map_err(gif_err)
    };

    let (timing, _) = transcode::video_timing(track);
    let step = timing.frame_ms * (GIF_MIN_FRAME_MS / timing.frame_ms - 0.01).ceil().max(1.0);
    let tolerance = timing.frame_ms / 2.0;
    let mut pending: Option<(RgbaImage, i64)> = None;
    let mut next_pick: Option<f64> = None;
    let mut end = 0i64;
    transcode::for_each_frame(track, progress, |picture, pts, duration| {
        end = pts + duration as i64;
        let t = pts as f64;
        match next_pick {
            Some(slot) if t < slot - tolerance => return Ok(()),
            Some(slot) if t < slot + step => next_pick = Some(slot + step),
            _ => next_pick = Some(t + step),
        }
        let (w, h) = gif_size(picture.width, picture.height);
        let mut image = picture_to_rgba(&picture);
        if (w, h) != image.dimensions() {
            image = image::imageops::resize(&image, w, h, FilterType::Triangle);
        }
        if let Some((previous, start)) = pending.take() {
            write_frame(&mut encoder, previous, start, pts)?;
        }
        pending = Some((image, pts));
        Ok(())
    })?;
    if let Some((last, start)) = pending {
        write_frame(&mut encoder, last, start, end.max(start + 1))?;
    }
    Ok(())
}

pub fn gif_to_video(raw: &[u8], target: Target, quality: transcode::Quality) -> Result<Track<'static>, String> {
    let read_err = |e: image::ImageError| format!("Couldn't read this GIF: {e}");
    let decoder = GifDecoder::new(Cursor::new(raw)).map_err(read_err)?;
    let (w, h) = decoder.dimensions();
    let (w, h) = ((w & !1) as usize, (h & !1) as usize);
    if w == 0 || h == 0 {
        return Err("This GIF is too small to turn into a video.".to_string());
    }

    let mut encoder: Option<VideoEncoder> = None;
    let mut packets: Vec<EncodedPacket> = Vec::new();
    let mut pts = 0i64;
    let mut last: Option<(Picture, i64, u64)> = None;
    let mut count = 0usize;
    for frame in decoder.into_frames() {
        crate::check_cancelled()?;
        let frame = frame.map_err(read_err)?;
        let (numer, denom) = frame.delay().numer_denom_ms();
        let mut delay = (numer / denom.max(1)) as u64;
        if delay < GIF_MIN_DELAY_MS {
            delay = GIF_FALLBACK_DELAY_MS;
        }
        if encoder.is_none() {
            let fps = 1000.0 / delay as f32;
            let bitrate = (w * h) as f64 * fps as f64 * if target == Target::WebM { 0.08 } else { 0.12 } * quality.pick(0.5, 0.75, 1.0, 1.6);
            encoder = Some(VideoEncoder::new(target, w, h, bitrate as u64, fps)?);
        }
        let picture = rgba_to_picture(frame.buffer());
        let copy = Picture { width: picture.width, height: picture.height, i420: picture.i420.clone() };
        packets.extend(encoder.as_mut().unwrap().encode(picture, pts, delay)?);
        last = Some((copy, pts, delay));
        pts += delay as i64;
        count += 1;
    }

    let mut encoder = encoder.ok_or_else(|| "This GIF has no frames.".to_string())?;
    if let Some((picture, start, mut delay)) = last {
        if count == 1 {
            delay = delay.max(STILL_GIF_DURATION_MS);
        }
        let half = (delay / 2).max(1) as i64;
        packets.extend(encoder.encode(picture, start + half, half as u64)?);
    }
    transcode::encoded_video_track(encoder, packets, w, h, 0)
}