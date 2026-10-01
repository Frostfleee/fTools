#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod animation;
mod avi;
mod demux;
mod ebml;
mod flv;
mod h264;
mod matroska;
mod opus_decode;
mod mp4;
mod mpeg4;
mod transcode;
mod update;
mod vpx;
mod wmv;

use std::fs;
use std::io::Cursor;
use std::io::Write;
use std::os::windows::fs::FileTimesExt;
use std::path::{Path, PathBuf};

use image::ImageFormat;
use img_parts::ImageEXIF;
use rand::Rng;
use libheif_rs::{
    Channel, ColorSpace, CompressionFormat, EncoderQuality, HeifContext, Image as HeifImage,
    LibHeif, RgbChroma,
};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_prevent_default::{Flags, PlatformOptions};
use window_vibrancy::apply_acrylic;
use windows::Win32::Graphics::Dwm::{
    DwmEnableBlurBehindWindow, DWM_BB_BLURREGION, DWM_BB_ENABLE,
    DWM_BB_TRANSITIONONMAXIMIZED, DWM_BLURBEHIND,
};
use windows::Win32::Graphics::Gdi::{CreateRectRgn, DeleteObject};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT,
    KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
    MAPVK_VK_TO_VSC_EX, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
    MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT,
    MOUSE_EVENT_FLAGS, VIRTUAL_KEY, VK_BACK, VK_CAPITAL, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END,
    VK_HOME, VK_INSERT, VK_LEFT, VK_MENU, VK_NEXT, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4,
    VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS,
    VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
};


fn image_conversion_supported(ext: &str) -> bool {
    matches!(
        ext,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "ico" | "avif" | "heic" | "heif"
    )
}

fn image_source_supported(ext: &str) -> bool {
    matches!(
        ext,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "ico" | "avif" | "heic" | "heif"
    )
}

fn is_heif_container(ext: &str) -> bool {
    matches!(ext, "heic" | "heif" | "avif")
}

fn decode_heif(path: &Path) -> Result<image::DynamicImage, String> {
    let path_str = path
        .to_str()
        .ok_or_else(|| "The file path contains characters libheif can't handle.".to_string())?;

    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(path_str).map_err(|e| e.to_string())?;
    let handle = ctx.primary_image_handle().map_err(|e| e.to_string())?;

    let width = handle.width();
    let height = handle.height();

    let decoded = lib_heif
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
        .map_err(|e| e.to_string())?;

    let planes = decoded.planes();
    let plane = planes
        .interleaved
        .ok_or_else(|| "Decoded HEIF image has no interleaved RGB plane.".to_string())?;

    let stride = plane.stride;
    let data = plane.data;
    let row_bytes = width as usize * 3;

    let mut buf = vec![0u8; row_bytes * height as usize];
    for y in 0..height as usize {
        let src_start = y * stride;
        let dst_start = y * row_bytes;
        buf[dst_start..dst_start + row_bytes]
            .copy_from_slice(&data[src_start..src_start + row_bytes]);
    }

    image::RgbImage::from_raw(width, height, buf)
        .map(image::DynamicImage::ImageRgb8)
        .ok_or_else(|| "Couldn't reassemble the decoded HEIF pixel data.".to_string())
}

fn encode_heif(img: &image::DynamicImage, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    let output_str = output_path
        .to_str()
        .ok_or_else(|| "The output path contains characters libheif can't handle.".to_string())?;

    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let row_bytes = width as usize * 3;

    let mut heif_image = HeifImage::new(width, height, ColorSpace::Rgb(RgbChroma::Rgb))
        .map_err(|e| e.to_string())?;
    heif_image
        .create_plane(Channel::Interleaved, width, height, 8)
        .map_err(|e| e.to_string())?;

    {
        let planes = heif_image.planes_mut();
        let plane = planes
            .interleaved
            .ok_or_else(|| "Couldn't allocate an interleaved RGB plane.".to_string())?;
        let stride = plane.stride;
        let data = plane.data;
        let src = rgb.as_raw();
        for y in 0..height as usize {
            let dst_start = y * stride;
            let src_start = y * row_bytes;
            data[dst_start..dst_start + row_bytes]
                .copy_from_slice(&src[src_start..src_start + row_bytes]);
        }
    }

    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new().map_err(|e| e.to_string())?;
    let mut encoder = lib_heif
        .encoder_for_format(CompressionFormat::Hevc)
        .map_err(|e| e.to_string())?;
    encoder
        .set_quality(EncoderQuality::Lossy(quality.pick(50, 70, 90, 97)))
        .map_err(|e| e.to_string())?;
    context
        .encode_image(&heif_image, &mut encoder, None)
        .map_err(|e| e.to_string())?;
    context.write_to_file(output_str).map_err(|e| e.to_string())
}

const ICO_SIZES: [u32; 7] = [16, 32, 48, 64, 128, 256, 512];

fn encode_multi_size_ico(img: &image::DynamicImage, output_path: &Path) -> Result<(), String> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in ICO_SIZES.iter() {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let (width, height) = rgba.dimensions();

        let icon_image = ico::IconImage::from_rgba_data(width, height, rgba.into_raw());
        let entry = ico::IconDirEntry::encode(&icon_image).map_err(|e| e.to_string())?;
        icon_dir.add_entry(entry);
    }

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    icon_dir.write(file).map_err(|e| e.to_string())
}

fn extract_exif(bytes: &[u8], ext: &str) -> Option<img_parts::Bytes> {
    match ext {
        "jpg" | "jpeg" => img_parts::jpeg::Jpeg::from_bytes(img_parts::Bytes::copy_from_slice(bytes))
            .ok()?
            .exif(),
        "png" => img_parts::png::Png::from_bytes(img_parts::Bytes::copy_from_slice(bytes))
            .ok()?
            .exif(),
        _ => None,
    }
}

fn inject_exif(bytes: Vec<u8>, ext: &str, exif: img_parts::Bytes) -> Vec<u8> {
    let input = img_parts::Bytes::from(bytes.clone());
    let rewritten: Option<Vec<u8>> = match ext {
        "jpg" | "jpeg" => img_parts::jpeg::Jpeg::from_bytes(input).ok().and_then(|mut jpeg| {
            jpeg.set_exif(Some(exif));
            let mut out = Vec::new();
            jpeg.encoder().write_to(&mut out).ok().map(|_| out)
        }),
        "png" => img_parts::png::Png::from_bytes(input).ok().and_then(|mut png| {
            png.set_exif(Some(exif));
            let mut out = Vec::new();
            png.encoder().write_to(&mut out).ok().map(|_| out)
        }),
        _ => None,
    };
    rewritten.unwrap_or(bytes)
}

const CANCELLED: &str = "Conversion cancelled.";
static CONVERSION_GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

thread_local! {
    static CONVERSION_JOB: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}

fn start_conversion_job(job: Option<u64>) {
    let job = job.unwrap_or_else(|| CONVERSION_GENERATION.load(Ordering::SeqCst));
    CONVERSION_JOB.with(|current| current.set(Some(job)));
}

fn check_cancelled() -> Result<(), String> {
    let cancelled = CONVERSION_JOB
        .with(|current| current.get())
        .is_some_and(|job| job != CONVERSION_GENERATION.load(Ordering::SeqCst));
    if cancelled { Err(CANCELLED.to_string()) } else { Ok(()) }
}

fn discard_if_cancelled(result: &Result<String, String>, output_path: &Path, started: std::time::SystemTime) {
    CONVERSION_JOB.with(|current| current.set(None));
    if !matches!(result, Err(e) if e == CANCELLED) {
        return;
    }
    let written_now = fs::metadata(output_path).and_then(|m| m.modified()).is_ok_and(|modified| modified >= started);
    if written_now {
        let _ = fs::remove_file(output_path);
    }
}

#[tauri::command]
fn begin_conversion() -> u64 {
    CONVERSION_GENERATION.load(Ordering::SeqCst)
}

#[tauri::command]
fn cancel_conversion() {
    CONVERSION_GENERATION.fetch_add(1, Ordering::SeqCst);
}

fn output_folder(source_path: &Path, output_dir: Option<&str>) -> PathBuf {
    match output_dir.filter(|dir| !dir.trim().is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => source_path.parent().map(Path::to_path_buf).unwrap_or_default(),
    }
}

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct MediaInfo {
    size: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
    duration_ms: Option<u64>,
}

fn read_head(path: &Path, limit: u64) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut head = Vec::new();
    fs::File::open(path).ok()?.take(limit).read_to_end(&mut head).ok()?;
    Some(head)
}

fn asf_duration_ms(path: &Path) -> Option<u64> {
    let start = read_head(path, 24)?;
    let size = u64::from_le_bytes(start.get(16..24)?.try_into().ok()?);
    wmv::duration_ms(&read_head(path, size.clamp(30, 8 << 20))?)
}

fn media_duration_ms(path: &Path) -> Option<u64> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    match ext.as_str() {
        "flv" => return flv::duration_ms(&read_head(path, 64 * 1024)?),
        "avi" => return avi::duration_ms(&read_head(path, 1 << 20)?),
        "wmv" | "wma" | "asf" => return asf_duration_ms(path),
        _ => {}
    }
    let file = fs::File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .ok()?;
    probed
        .format
        .tracks()
        .iter()
        .filter_map(|track| {
            let params = &track.codec_params;
            let frames = params.n_frames?;
            if let Some(time_base) = params.time_base {
                let time = time_base.calc_time(frames);
                Some(time.seconds * 1000 + (time.frac * 1000.0) as u64)
            } else {
                params.sample_rate.filter(|&rate| rate > 0).map(|rate| frames * 1000 / rate as u64)
            }
        })
        .max()
}

#[tauri::command]
async fn media_info(paths: Vec<String>) -> Vec<MediaInfo> {
    tauri::async_runtime::spawn_blocking(move || {
        paths
            .iter()
            .map(|path| {
                let path = Path::new(path);
                let size = fs::metadata(path).ok().map(|m| m.len());
                match image::image_dimensions(path) {
                    Ok((width, height)) => MediaInfo { size, width: Some(width), height: Some(height), duration_ms: None },
                    Err(_) => MediaInfo { size, duration_ms: media_duration_ms(path), ..Default::default() },
                }
            })
            .collect()
    })
    .await
    .unwrap_or_default()
}

#[tauri::command]
fn existing_output_folder(source_path: String, output_name: String, target_ext: String, output_dir: Option<String>) -> Option<String> {
    let source_path = PathBuf::from(source_path);
    let output_dir = output_folder(&source_path, output_dir.as_deref());
    let output_path = output_dir.join(format!("{output_name}.{}", target_ext.to_lowercase()));
    if output_path == source_path || !output_path.exists() {
        return None;
    }
    let folder = output_dir.file_name().unwrap_or(output_dir.as_os_str());
    Some(folder.to_string_lossy().to_string())
}

#[tauri::command]
async fn convert_image(
    app: AppHandle,
    source_path: String,
    output_name: String,
    target_ext: String,
    keep_metadata: bool,
    preserve_date: bool,
    overwrite: bool,
    output_dir: Option<String>,
    job: Option<u64>,
    quality: Option<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
    start_conversion_job(job);
    let quality = transcode::Quality::from_name(quality.as_deref());
    let started = std::time::SystemTime::now();
    let output_path = output_folder(Path::new(&source_path), output_dir.as_deref())
        .join(format!("{output_name}.{}", target_ext.to_lowercase()));
    let result = (|| -> Result<String, String> {
    let report = |percent: u8| {
        let _ = app.emit("conversion-progress", percent);
    };

    let source_path = PathBuf::from(source_path);
    let target_ext = target_ext.to_lowercase();

    if !image_conversion_supported(&target_ext) {
        return Err(format!(
            "\"{}\" isn't wired up for image conversion yet.",
            target_ext
        ));
    }

    let source_ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !image_source_supported(&source_ext) {
        return Err(format!("Can't read \"{}\" files: this source format isn't supported yet.", source_ext));
    }

    report(10);

    check_cancelled()?;

    let source_bytes =
        fs::read(&source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;

    report(30);

    check_cancelled()?;

    let decoded = if is_heif_container(&source_ext) {
        decode_heif(&source_path).map_err(|e| format!("Couldn't decode this {} file: {e}", source_ext))?
    } else {
        image::load_from_memory(&source_bytes)
            .map_err(|e| format!("This doesn't look like a valid image: {e}"))?
    };

    report(55);

    check_cancelled()?;


    if is_heif_container(&target_ext) && target_ext != "avif" {
        encode_heif(&decoded, &output_path, quality)
            .map_err(|e| format!("Couldn't encode the image as {}: {e}", target_ext))?;
    } else if target_ext == "ico" {
        encode_multi_size_ico(&decoded, &output_path)
            .map_err(|e| format!("Couldn't encode the image as ico: {e}"))?;
    } else {
        let format = ImageFormat::from_extension(&target_ext)
            .ok_or_else(|| format!("Unknown target format \"{}\".", target_ext))?;

        let mut encoded: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut encoded);
        let written = match format {
            ImageFormat::Jpeg => decoded.write_with_encoder(image::codecs::jpeg::JpegEncoder::new_with_quality(
                &mut cursor,
                quality.pick(50, 65, 75, 92),
            )),
            ImageFormat::Avif => decoded.write_with_encoder(image::codecs::avif::AvifEncoder::new_with_speed_quality(
                &mut cursor,
                4,
                quality.pick(50, 65, 80, 92),
            )),
            _ => decoded.write_to(&mut cursor, format),
        };
        written.map_err(|e| format!("Couldn't encode the image as {}: {e}", target_ext))?;

        if keep_metadata {
            if let Some(exif) = extract_exif(&source_bytes, &source_ext) {
                encoded = inject_exif(encoded, &target_ext, exif);
            }
        }

        fs::write(&output_path, &encoded)
            .map_err(|e| format!("Couldn't write the converted file: {e}"))?;
    }

    report(85);

    check_cancelled()?;

    if preserve_date {
        preserve_file_date(&source_path, &output_path);
    }

    if overwrite && output_path != source_path {
        let _ = fs::remove_file(&source_path);
    }

    report(100);

    check_cancelled()?;

    Ok(output_path.to_string_lossy().to_string())
    })();
    discard_if_cancelled(&result, &output_path, started);
    result
    })
    .await
    .map_err(|e| format!("Conversion task panicked: {e}"))?
}

fn preserve_file_date(source_path: &Path, output_path: &Path) {
    if let Ok(metadata) = fs::metadata(source_path) {
        let created = metadata.created();
        let modified = metadata.modified();
        if created.is_ok() || modified.is_ok() {
            if let Ok(file) = fs::OpenOptions::new().write(true).open(output_path) {
                let mut times = fs::FileTimes::new();
                if let Ok(created) = created {
                    times = times.set_created(created);
                }
                if let Ok(modified) = modified {
                    times = times.set_modified(modified);
                }
                let _ = file.set_times(times);
            }
        }
    }
}


fn patch_flac_streaminfo_for_symphonia_bug(bytes: &mut [u8]) {
    if bytes.len() < 42 || &bytes[0..4] != b"fLaC" {
        return;
    }
    let block_type = bytes[4] & 0x7f;
    let block_len = ((bytes[5] as usize) << 16) | ((bytes[6] as usize) << 8) | bytes[7] as usize;
    if block_type != 0 || block_len != 34 {
        return;
    }
    bytes[8] = bytes[10];
    bytes[9] = bytes[11];
}

fn decode_to_pcm(source_path: &Path) -> Result<(Vec<f32>, u32, u16), String> {
    let source_ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if matches!(source_ext.as_str(), "wma" | "wmv" | "asf") {
        return decode_with_media_foundation(source_path);
    }

    if source_ext == "opus" {
        return decode_opus(source_path);
    }

    if matches!(source_ext.as_str(), "flv" | "avi") {
        let raw = fs::read(source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;
        let tracks = read_container(&raw, &source_ext)?;
        let track = tracks.iter().find(|t| !t.video).ok_or("This file has no audio track.")?;
        return transcode::decode_audio(track);
    }

    if source_ext == "webm" {
        if let Some(result) = decode_webm_opus(source_path)? {
            return Ok(result);
        }
    }

    let mss = if source_ext == "flac" {
        let mut bytes = fs::read(source_path).map_err(|e| format!("Couldn't open the source file: {e}"))?;
        patch_flac_streaminfo_for_symphonia_bug(&mut bytes);
        MediaSourceStream::new(Box::new(std::io::Cursor::new(bytes)), Default::default())
    } else {
        let file = fs::File::open(source_path).map_err(|e| format!("Couldn't open the source file: {e}"))?;
        MediaSourceStream::new(Box::new(file), Default::default())
    };

    let mut hint = Hint::new();
    if !source_ext.is_empty() {
        hint.with_extension(&source_ext);
    }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| format!("This doesn't look like a supported audio file: {e}"))?;

    let mut format = probed.format;

    let dec_opts: DecoderOptions = Default::default();
    let (track_id, mut decoder) = format
        .tracks()
        .iter()
        .filter(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .find_map(|t| {
            symphonia::default::get_codecs()
                .make(&t.codec_params, &dec_opts)
                .ok()
                .map(|decoder| (t.id, decoder))
        })
        .ok_or_else(|| "Couldn't find a supported audio track in this file.".to_string())?;

    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    let mut all_samples: Vec<f32> = Vec::new();
    let mut sample_rate: u32 = 0;
    let mut channels: u16 = 0;

    loop {
        check_cancelled()?;
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(_)) => break,
            Err(SymphoniaError::ResetRequired) => break,
            Err(e) => return Err(format!("Error while reading the audio stream: {e}")),
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(SymphoniaError::IoError(_)) => continue,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(format!("Error while decoding audio: {e}")),
        };

        if sample_buf.is_none() {
            let spec = *decoded.spec();
            sample_rate = spec.rate;
            channels = spec.channels.count() as u16;
            sample_buf = Some(SampleBuffer::<f32>::new(decoded.capacity() as u64, spec));
        }

        if let Some(buf) = sample_buf.as_mut() {
            buf.copy_interleaved_ref(decoded);
            all_samples.extend_from_slice(buf.samples());
        }
    }

    if all_samples.is_empty() {
        return Err("No audio data could be decoded from this file.".to_string());
    }

    Ok((all_samples, sample_rate, channels))
}

fn to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * i16::MAX as f32).round() as i16
}

fn interleaved_to_planar(samples: &[f32], channels: usize) -> Vec<Vec<f32>> {
    let frames = samples.len() / channels;
    let mut planar = vec![Vec::with_capacity(frames); channels];
    for frame in samples.chunks(channels) {
        for (ch, &s) in frame.iter().enumerate() {
            planar[ch].push(s);
        }
    }
    planar
}

fn to_stereo_i16(samples: &[f32], channels: u16) -> (Vec<i16>, Vec<i16>) {
    if channels <= 1 {
        let mono: Vec<i16> = samples.iter().map(|&s| to_i16(s)).collect();
        (mono.clone(), mono)
    } else {
        let frames = samples.len() / channels as usize;
        let mut left = Vec::with_capacity(frames);
        let mut right = Vec::with_capacity(frames);
        for frame in samples.chunks(channels as usize) {
            left.push(to_i16(frame[0]));
            right.push(to_i16(frame.get(1).copied().unwrap_or(frame[0])));
        }
        (left, right)
    }
}

fn encode_wav(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(output_path, spec).map_err(|e| e.to_string())?;
    for &s in samples {
        writer.write_sample(s).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())
}

fn f64_to_ieee80(value: f64) -> [u8; 10] {
    if value == 0.0 {
        return [0u8; 10];
    }
    let bits = value.to_bits();
    let sign = (bits >> 63) & 1;
    let exponent = ((bits >> 52) & 0x7FF) as i64 - 1023;
    let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;

    let ext_exponent = (exponent + 16383) as u16;
    let ext_mantissa: u64 = (1u64 << 63) | (mantissa << 11);

    let mut result = [0u8; 10];
    result[0] = ((sign as u16) << 7 | (ext_exponent >> 8)) as u8;
    result[1] = (ext_exponent & 0xFF) as u8;
    result[2..10].copy_from_slice(&ext_mantissa.to_be_bytes());
    result
}

fn encode_aiff(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    let frames = samples.len() / channels.max(1) as usize;
    let data: Vec<i16> = samples.iter().map(|&s| to_i16(s)).collect();
    let data_bytes: Vec<u8> = data.iter().flat_map(|s| s.to_be_bytes()).collect();

    let comm_size: u32 = 18;
    let ssnd_size: u32 = 8 + data_bytes.len() as u32;
    let form_size: u32 = 4 + (8 + comm_size) + (8 + ssnd_size);

    let mut out = Vec::with_capacity(8 + form_size as usize);
    out.extend_from_slice(b"FORM");
    out.extend_from_slice(&form_size.to_be_bytes());
    out.extend_from_slice(b"AIFF");

    out.extend_from_slice(b"COMM");
    out.extend_from_slice(&comm_size.to_be_bytes());
    out.extend_from_slice(&(channels as i16).to_be_bytes());
    out.extend_from_slice(&(frames as u32).to_be_bytes());
    out.extend_from_slice(&16i16.to_be_bytes());
    out.extend_from_slice(&f64_to_ieee80(sample_rate as f64));

    out.extend_from_slice(b"SSND");
    out.extend_from_slice(&ssnd_size.to_be_bytes());
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(&data_bytes);

    fs::write(output_path, &out).map_err(|e| e.to_string())
}

fn encode_flac(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    use flacenc::component::BitRepr;
    use flacenc::error::Verify;

    let bits_per_sample: usize = 16;
    let int_samples: Vec<i32> = samples.iter().map(|&s| to_i16(s) as i32).collect();

    let config = flacenc::config::Encoder::default()
        .into_verified()
        .map_err(|e| format!("FLAC config error: {:?}", e))?;
    let source = flacenc::source::MemSource::from_samples(
        &int_samples,
        channels as usize,
        bits_per_sample,
        sample_rate as usize,
    );
    let flac_stream = flacenc::encode_with_fixed_block_size(&config, source, config.block_size)
        .map_err(|e| format!("FLAC encode error: {:?}", e))?;

    let mut sink = flacenc::bitsink::ByteSink::new();
    flac_stream
        .write(&mut sink)
        .map_err(|e| format!("FLAC write error: {:?}", e))?;

    fs::write(output_path, sink.as_slice()).map_err(|e| e.to_string())
}

fn encode_mp3_bytes(samples: &[f32], sample_rate: u32, channels: u16, quality: transcode::Quality) -> Result<Vec<u8>, String> {
    use mp3lame_encoder::{Builder, DualPcm, FlushNoGap};

    let mut builder = Builder::new().ok_or_else(|| "Couldn't create the MP3 encoder.".to_string())?;
    builder.set_num_channels(2).map_err(|e| format!("{:?}", e))?;
    builder.set_sample_rate(sample_rate).map_err(|e| format!("{:?}", e))?;
    builder
        .set_brate(quality.pick(
            mp3lame_encoder::Bitrate::Kbps128,
            mp3lame_encoder::Bitrate::Kbps160,
            mp3lame_encoder::Bitrate::Kbps192,
            mp3lame_encoder::Bitrate::Kbps320,
        ))
        .map_err(|e| format!("{:?}", e))?;
    builder
        .set_quality(mp3lame_encoder::Quality::Best)
        .map_err(|e| format!("{:?}", e))?;
    let mut encoder = builder.build().map_err(|e| format!("{:?}", e))?;

    let (left, right) = to_stereo_i16(samples, channels);
    let input = DualPcm { left: &left, right: &right };

    let mut out_buffer = Vec::new();
    out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(left.len()));
    let encoded_size = encoder
        .encode(input, out_buffer.spare_capacity_mut())
        .map_err(|e| format!("MP3 encode error: {:?}", e))?;
    unsafe {
        out_buffer.set_len(out_buffer.len() + encoded_size);
    }

    let flushed_size = encoder
        .flush::<FlushNoGap>(out_buffer.spare_capacity_mut())
        .map_err(|e| format!("MP3 flush error: {:?}", e))?;
    unsafe {
        out_buffer.set_len(out_buffer.len() + flushed_size);
    }

    Ok(out_buffer)
}

fn encode_mp3(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    let bytes = encode_mp3_bytes(samples, sample_rate, channels, quality)?;
    fs::write(output_path, &bytes).map_err(|e| e.to_string())
}

fn encode_ogg_vorbis(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    use std::num::{NonZeroU32, NonZeroU8};
    use vorbis_rs::VorbisEncoderBuilder;

    let sr = NonZeroU32::new(sample_rate).ok_or_else(|| "Invalid sample rate.".to_string())?;
    let ch = NonZeroU8::new(channels as u8).ok_or_else(|| "Invalid channel count.".to_string())?;

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut builder = VorbisEncoderBuilder::new(sr, ch, file)
        .map_err(|e| format!("Vorbis setup error: {e}"))?;
    builder.bitrate_management_strategy(vorbis_rs::VorbisBitrateManagementStrategy::QualityVbr {
        target_quality: quality.pick(0.2, 0.35, 0.5, 0.8),
    });
    let mut encoder = builder.build().map_err(|e| format!("Vorbis build error: {e}"))?;

    let planar = interleaved_to_planar(samples, channels as usize);
    const BLOCK_SIZE: usize = 1024;
    let total_frames = if planar.is_empty() { 0 } else { planar[0].len() };
    let mut pos = 0;
    while pos < total_frames {
        let end = (pos + BLOCK_SIZE).min(total_frames);
        let block: Vec<&[f32]> = planar.iter().map(|c| &c[pos..end]).collect();
        encoder
            .encode_audio_block(&block)
            .map_err(|e| format!("Vorbis encode error: {e}"))?;
        pos = end;
    }

    encoder.finish().map_err(|e| format!("Vorbis finish error: {e}"))?;
    Ok(())
}

fn resample_planar(planar_in: &[Vec<f32>], from_rate: u32, to_rate: u32) -> Result<Vec<Vec<f32>>, String> {
    use rubato::{FftFixedInOut, Resampler};

    let channels = planar_in.len();
    let total_in = planar_in.first().map(|c| c.len()).unwrap_or(0);

    let mut resampler = FftFixedInOut::<f32>::new(from_rate as usize, to_rate as usize, 1024, channels)
        .map_err(|e| format!("Couldn't set up the resampler: {e}"))?;

    let chunk_in = resampler.input_frames_next();
    let mut out: Vec<Vec<f32>> = vec![Vec::new(); channels];
    let mut pos = 0usize;

    while pos < total_in {
        let end = (pos + chunk_in).min(total_in);
        let in_buf: Vec<Vec<f32>> = (0..channels)
            .map(|ch| {
                let mut v = planar_in[ch][pos..end].to_vec();
                v.resize(chunk_in, 0.0);
                v
            })
            .collect();
        let chunk_out_max = resampler.output_frames_next();
        let mut out_buf: Vec<Vec<f32>> = vec![vec![0.0f32; chunk_out_max]; channels];

        let (_used, produced) = resampler
            .process_into_buffer(&in_buf, &mut out_buf, None)
            .map_err(|e| format!("Resampling failed: {e}"))?;

        for (ch, channel_out) in out.iter_mut().enumerate() {
            channel_out.extend_from_slice(&out_buf[ch][..produced]);
        }
        pos = end;
    }

    Ok(out)
}

fn resample_interleaved(samples: &[f32], channels: u16, from_rate: u32, to_rate: u32) -> Result<Vec<f32>, String> {
    let planar = interleaved_to_planar(samples, channels as usize);
    let resampled = resample_planar(&planar, from_rate, to_rate)?;
    let frames = resampled.first().map(|c| c.len()).unwrap_or(0);
    let mut out = Vec::with_capacity(frames * channels as usize);
    for frame in 0..frames {
        for channel in resampled.iter() {
            out.push(channel[frame]);
        }
    }
    Ok(out)
}

const OPUS_VALID_RATES: [u32; 5] = [8000, 12000, 16000, 24000, 48000];

fn build_opus_head(channels: u8, input_sample_rate: u32) -> Vec<u8> {
    let mut head = Vec::with_capacity(19);
    head.extend_from_slice(b"OpusHead");
    head.push(1);
    head.push(channels);
    head.extend_from_slice(&0u16.to_le_bytes());
    head.extend_from_slice(&input_sample_rate.to_le_bytes());
    head.extend_from_slice(&0i16.to_le_bytes());
    head.push(0);
    head
}

fn build_opus_tags() -> Vec<u8> {
    let mut tags = Vec::new();
    tags.extend_from_slice(b"OpusTags");
    let vendor = b"fTools";
    tags.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    tags.extend_from_slice(vendor);
    tags.extend_from_slice(&0u32.to_le_bytes());
    tags
}

fn encode_opus(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    use ogg::writing::{PacketWriteEndInfo, PacketWriter};
    use opus_rs::{Application, OpusEncoder};

    if channels == 0 || channels > 2 {
        return Err("Opus encoding here only supports mono or stereo sources.".to_string());
    }

    let (opus_rate, interleaved) = if OPUS_VALID_RATES.contains(&sample_rate) {
        (sample_rate, samples.to_vec())
    } else {
        (48000, resample_interleaved(samples, channels, sample_rate, 48000)?)
    };

    let mut encoder = OpusEncoder::new(opus_rate as i32, channels as usize, Application::Audio)
        .map_err(|e| format!("Couldn't create the Opus encoder: {e}"))?;
    encoder.bitrate_bps = quality.pick(64_000, 96_000, 128_000, 192_000);

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut writer = PacketWriter::new(file);
    let serial: u32 = 0x66_54_6F_6C;

    writer
        .write_packet(build_opus_head(channels as u8, sample_rate), serial, PacketWriteEndInfo::EndPage, 0)
        .map_err(|e| format!("Couldn't write the Opus header: {e}"))?;
    writer
        .write_packet(build_opus_tags(), serial, PacketWriteEndInfo::EndPage, 0)
        .map_err(|e| format!("Couldn't write the Opus comment header: {e}"))?;

    let frame_size = (opus_rate / 50) as usize;
    let frame_len = frame_size * channels as usize;
    let mut encode_buf = vec![0u8; 4000];
    let mut granule: u64 = 0;
    let mut pos = 0usize;

    loop {
        let end = (pos + frame_len).min(interleaved.len());
        let mut chunk = interleaved[pos..end].to_vec();
        chunk.resize(frame_len, 0.0);

        let size = encoder
            .encode(&chunk, frame_size, &mut encode_buf)
            .map_err(|e| format!("Opus encode error: {e}"))?;

        granule += frame_size as u64;
        pos = end;
        let is_last = pos >= interleaved.len();
        let end_info = if is_last { PacketWriteEndInfo::EndStream } else { PacketWriteEndInfo::NormalPacket };

        writer
            .write_packet(encode_buf[..size].to_vec(), serial, end_info, granule)
            .map_err(|e| format!("Couldn't write an Opus packet: {e}"))?;

        if is_last {
            break;
        }
    }

    Ok(())
}

fn decode_opus(source_path: &Path) -> Result<(Vec<f32>, u32, u16), String> {
    use ogg::reading::PacketReader;

    let file = fs::File::open(source_path).map_err(|e| format!("Couldn't open the source file: {e}"))?;
    let mut reader = PacketReader::new(file);

    let head = reader
        .read_packet()
        .map_err(|e| format!("Couldn't read this Ogg file: {e}"))?
        .ok_or_else(|| "This Opus file is empty.".to_string())?;
    if head.data.len() < 19 || &head.data[0..8] != b"OpusHead" {
        return Err("This doesn't look like a valid Opus file (missing OpusHead).".to_string());
    }

    reader
        .read_packet()
        .map_err(|e| format!("Couldn't read this Ogg file: {e}"))?
        .ok_or_else(|| "This Opus file is missing its comment header.".to_string())?;

    let mut decoder = opus_decode::OpusStreamDecoder::from_head(&head.data)?;
    let channels = decoder.output_channels();
    let mut all_samples: Vec<f32> = Vec::new();

    while let Some(packet) = reader
        .read_packet()
        .map_err(|e| format!("Error while reading Opus audio data: {e}"))?
    {
        check_cancelled()?;
        decoder.decode(&packet.data, &mut all_samples)?;
    }

    let skip_samples = (decoder.pre_skip() as usize * channels as usize).min(all_samples.len());
    all_samples.drain(0..skip_samples);

    if all_samples.is_empty() {
        return Err("No audio data could be decoded from this Opus file.".to_string());
    }

    Ok((all_samples, 48000, channels))
}

fn decode_webm_opus(source_path: &Path) -> Result<Option<(Vec<f32>, u32, u16)>, String> {
    use symphonia::core::codecs::CODEC_TYPE_OPUS;

    let file = fs::File::open(source_path).map_err(|e| format!("Couldn't open the source file: {e}"))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("webm");

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| format!("This doesn't look like a supported WebM file: {e}"))?;

    let mut format = probed.format;

    let (track_id, mut decoder) = {
        let Some(t) = format.tracks().iter().find(|t| t.codec_params.codec == CODEC_TYPE_OPUS) else {
            return Ok(None);
        };
        let decoder = match t.codec_params.extra_data.as_deref() {
            Some(head) if head.starts_with(b"OpusHead") => opus_decode::OpusStreamDecoder::from_head(head)?,
            _ => opus_decode::OpusStreamDecoder::from_channels(
                t.codec_params.channels.map(|c| c.count() as u16).filter(|&c| c > 0).unwrap_or(2),
            )?,
        };
        (t.id, decoder)
    };
    let channels = decoder.output_channels();
    let pre_skip = decoder.pre_skip();
    let mut all_samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(_)) => break,
            Err(SymphoniaError::ResetRequired) => break,
            Err(e) => return Err(format!("Error while reading the WebM stream: {e}")),
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id || packet.data.is_empty() {
            continue;
        }

        check_cancelled()?;
        decoder.decode(&packet.data, &mut all_samples)?;
    }

    let skip_samples = (pre_skip as usize) * channels as usize;
    if skip_samples < all_samples.len() {
        all_samples.drain(0..skip_samples);
    }

    if all_samples.is_empty() {
        return Err("No audio data could be decoded from this WebM file's Opus track.".to_string());
    }

    Ok(Some((all_samples, 48000, channels)))
}

const AAC_SUPPORTED_RATES: [u32; 9] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000];

fn nearest_supported_rate(rate: u32, table: &[u32]) -> u32 {
    *table
        .iter()
        .min_by_key(|&&candidate| (candidate as i64 - rate as i64).abs())
        .expect("rate table is never empty")
}

struct MfEncoderProfile {
    subtype: windows::core::GUID,
    valid_rates: &'static [u32],
    bitrate_stereo: u32,
    bitrate_mono: u32,
    adts: bool,
    negotiate_bitrate: bool,
}

fn encode_via_media_foundation(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    output_path: &Path,
    profile: MfEncoderProfile,
) -> Result<(), String> {
    use windows::core::HSTRING;
    use windows::Win32::Media::MediaFoundation::{
        IMFAttributes, MFAudioFormat_PCM, MFCreateAttributes, MFCreateMediaType, MFCreateMemoryBuffer,
        MFCreateSample, MFCreateSinkWriterFromURL, MFMediaType_Audio, MFShutdown, MFStartup,
        MFTranscodeContainerType_ADTS, MF_MT_AAC_PAYLOAD_TYPE, MF_MT_ALL_SAMPLES_INDEPENDENT,
        MF_MT_AUDIO_AVG_BYTES_PER_SECOND, MF_MT_AUDIO_BITS_PER_SAMPLE, MF_MT_AUDIO_BLOCK_ALIGNMENT,
        MF_MT_AUDIO_NUM_CHANNELS, MF_MT_AUDIO_SAMPLES_PER_SECOND, MF_MT_AVG_BITRATE, MF_MT_MAJOR_TYPE,
        MF_MT_SUBTYPE, MF_SINK_WRITER_DISABLE_THROTTLING, MF_TRANSCODE_CONTAINERTYPE, MF_VERSION, MFSTARTUP_FULL,
    };
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

    if channels == 0 || channels > 2 {
        return Err("This encoder only supports mono or stereo sources.".to_string());
    }

    let (target_rate, interleaved) = if profile.valid_rates.contains(&sample_rate) {
        (sample_rate, samples.to_vec())
    } else {
        let nearest = nearest_supported_rate(sample_rate, profile.valid_rates);
        (nearest, resample_interleaved(samples, channels, sample_rate, nearest)?)
    };

    let block_align: u32 = 2 * channels as u32;
    let pcm_bytes_per_sec = target_rate * block_align;
    let avg_bitrate = if channels >= 2 { profile.bitrate_stereo } else { profile.bitrate_mono };
    let pcm_i16: Vec<u8> = interleaved.iter().flat_map(|&s| to_i16(s).to_le_bytes()).collect();
    let url = HSTRING::from(output_path.to_string_lossy().as_ref());

    let handle = std::thread::spawn(move || -> Result<(), String> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .map_err(|e| format!("Couldn't initialize COM on the encoder thread: {e}"))?;

            let result: Result<(), String> = (|| {
            MFStartup(MF_VERSION, MFSTARTUP_FULL).map_err(|e| format!("Couldn't start Media Foundation: {e}"))?;

            let inner: Result<(), String> = (|| {
                let output_type = MFCreateMediaType().map_err(|e| e.to_string())?;
                output_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Audio).map_err(|e| e.to_string())?;
                output_type.SetGUID(&MF_MT_SUBTYPE, &profile.subtype).map_err(|e| e.to_string())?;
                output_type.SetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE, 16).map_err(|e| e.to_string())?;
                output_type.SetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND, target_rate).map_err(|e| e.to_string())?;
                output_type.SetUINT32(&MF_MT_AUDIO_NUM_CHANNELS, channels as u32).map_err(|e| e.to_string())?;
                if !profile.negotiate_bitrate {
                    output_type.SetUINT32(&MF_MT_AVG_BITRATE, avg_bitrate).map_err(|e| e.to_string())?;
                }
                if profile.adts {
                    output_type.SetUINT32(&MF_MT_AAC_PAYLOAD_TYPE, 1).map_err(|e| e.to_string())?;
                }

                let input_type = MFCreateMediaType().map_err(|e| e.to_string())?;
                input_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Audio).map_err(|e| e.to_string())?;
                input_type.SetGUID(&MF_MT_SUBTYPE, &MFAudioFormat_PCM).map_err(|e| e.to_string())?;
                input_type.SetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE, 16).map_err(|e| e.to_string())?;
                input_type.SetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND, target_rate).map_err(|e| e.to_string())?;
                input_type.SetUINT32(&MF_MT_AUDIO_NUM_CHANNELS, channels as u32).map_err(|e| e.to_string())?;
                input_type.SetUINT32(&MF_MT_AUDIO_BLOCK_ALIGNMENT, block_align).map_err(|e| e.to_string())?;
                input_type
                    .SetUINT32(&MF_MT_AUDIO_AVG_BYTES_PER_SECOND, pcm_bytes_per_sec)
                    .map_err(|e| e.to_string())?;
                input_type.SetUINT32(&MF_MT_ALL_SAMPLES_INDEPENDENT, 1).map_err(|e| e.to_string())?;

                let mut writer_attrs: Option<IMFAttributes> = None;
                MFCreateAttributes(&mut writer_attrs, 2).map_err(|e| e.to_string())?;
                let writer_attrs = writer_attrs
                    .ok_or_else(|| "Media Foundation didn't return an attributes object.".to_string())?;
                writer_attrs
                    .SetUINT32(&MF_SINK_WRITER_DISABLE_THROTTLING, 1)
                    .map_err(|e| e.to_string())?;
                if profile.adts {
                    writer_attrs
                        .SetGUID(&MF_TRANSCODE_CONTAINERTYPE, &MFTranscodeContainerType_ADTS)
                        .map_err(|e| e.to_string())?;
                }
                let writer = MFCreateSinkWriterFromURL(&url, None, &writer_attrs)
                    .map_err(|e| format!("Couldn't create the output writer: {e}"))?;

                let stream_index = writer
                    .AddStream(&output_type)
                    .map_err(|e| format!("Couldn't configure the output stream: {e}"))?;

                let encoding_params: Option<IMFAttributes> = if profile.negotiate_bitrate {
                    let mut ep: Option<IMFAttributes> = None;
                    MFCreateAttributes(&mut ep, 1).map_err(|e| e.to_string())?;
                    let ep = ep.ok_or_else(|| "Media Foundation didn't return an attributes object.".to_string())?;
                    ep.SetUINT32(&MF_MT_AVG_BITRATE, avg_bitrate).map_err(|e| e.to_string())?;
                    Some(ep)
                } else {
                    None
                };
                writer
                    .SetInputMediaType(stream_index, &input_type, encoding_params.as_ref())
                    .map_err(|e| format!("Couldn't configure the PCM input stream: {e}"))?;
                writer
                    .BeginWriting()
                    .map_err(|e| format!("Couldn't begin writing the output file: {e}"))?;

                let chunk_frames = (target_rate / 50).max(1) as usize;
                let chunk_bytes = chunk_frames * block_align as usize;
                let mut offset = 0usize;
                let mut frames_written: i64 = 0;
                let mut chunk_index: u32 = 0;
                let total_chunks = pcm_i16.len().div_ceil(chunk_bytes.max(1));

                while offset < pcm_i16.len() {
                    let end = (offset + chunk_bytes).min(pcm_i16.len());
                    let slice = &pcm_i16[offset..end];
                    let frames_in_chunk = (slice.len() / block_align as usize) as i64;

                    let buffer = MFCreateMemoryBuffer(slice.len() as u32).map_err(|e| e.to_string())?;
                    let mut ptr: *mut u8 = std::ptr::null_mut();
                    buffer.Lock(&mut ptr, None, None).map_err(|e| e.to_string())?;
                    std::ptr::copy_nonoverlapping(slice.as_ptr(), ptr, slice.len());
                    buffer.Unlock().map_err(|e| e.to_string())?;
                    buffer.SetCurrentLength(slice.len() as u32).map_err(|e| e.to_string())?;

                    let sample = MFCreateSample().map_err(|e| e.to_string())?;
                    sample.AddBuffer(&buffer).map_err(|e| e.to_string())?;

                    let sample_time: i64 = frames_written * 10_000_000 / target_rate as i64;
                    let next_sample_time: i64 = (frames_written + frames_in_chunk) * 10_000_000 / target_rate as i64;
                    let duration = next_sample_time - sample_time;
                    sample.SetSampleTime(sample_time).map_err(|e| e.to_string())?;
                    sample.SetSampleDuration(duration).map_err(|e| e.to_string())?;

                    writer.WriteSample(stream_index, &sample).map_err(|e| {
                        format!("Couldn't write audio data (chunk {chunk_index} of {total_chunks}): {e}")
                    })?;

                    frames_written += frames_in_chunk;
                    chunk_index += 1;
                    offset = end;
                }

                writer.Finalize().map_err(|e| format!("Couldn't finalize the output file: {e}"))?;
                Ok(())
            })();

            let _ = MFShutdown();
            inner
        })();

            CoUninitialize();
            result
        }
    });

    handle
        .join()
        .unwrap_or_else(|_| Err("The Media Foundation encoder thread panicked.".to_string()))
}

fn encode_m4a(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    use windows::Win32::Media::MediaFoundation::MFAudioFormat_AAC;
    encode_via_media_foundation(
        samples,
        sample_rate,
        channels,
        output_path,
        MfEncoderProfile {
            subtype: MFAudioFormat_AAC,
            valid_rates: &AAC_SUPPORTED_RATES,
            bitrate_stereo: quality.pick(96_000, 128_000, 192_000, 192_000),
            bitrate_mono: 96_000,
            adts: false,
            negotiate_bitrate: false,
        },
    )
}

fn encode_aac(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    use windows::Win32::Media::MediaFoundation::MFAudioFormat_AAC;
    encode_via_media_foundation(
        samples,
        sample_rate,
        channels,
        output_path,
        MfEncoderProfile {
            subtype: MFAudioFormat_AAC,
            valid_rates: &AAC_SUPPORTED_RATES,
            bitrate_stereo: quality.pick(96_000, 128_000, 192_000, 192_000),
            bitrate_mono: 96_000,
            adts: true,
            negotiate_bitrate: false,
        },
    )
}

fn encode_wma(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path, quality: transcode::Quality) -> Result<(), String> {
    on_media_foundation_thread(|| {
        let spec = wmv::AudioSpec { rate: sample_rate, channels, bitrate: quality.pick(96_000, 128_000, 160_000, 192_000) };
        let writer = wmv::Writer::create(output_path, None, Some(spec))?;
        let resampled = if writer.audio_rate == sample_rate {
            samples.to_vec()
        } else {
            resample_interleaved(samples, channels, sample_rate, writer.audio_rate)?
        };
        let pcm16: Vec<u8> = resampled.iter().flat_map(|&s| to_i16(s).to_le_bytes()).collect();
        let mut written = 0usize;
        write_wma_until(&writer, &pcm16, &mut written, 2 * channels as usize, 0, None)?;
        writer.finish()
    })
}

#[tauri::command]
async fn convert_audio(
    app: AppHandle,
    source_path: String,
    output_name: String,
    target_ext: String,
    preserve_date: bool,
    overwrite: bool,
    output_dir: Option<String>,
    job: Option<u64>,
    quality: Option<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
    start_conversion_job(job);
    let quality = transcode::Quality::from_name(quality.as_deref());
    let started = std::time::SystemTime::now();
    let output_path = output_folder(Path::new(&source_path), output_dir.as_deref())
        .join(format!("{output_name}.{}", target_ext.to_lowercase()));
    let result = (|| -> Result<String, String> {
    let report = |percent: u8| {
        let _ = app.emit("conversion-progress", percent);
    };

    let source_path = PathBuf::from(source_path);
    let target_ext = target_ext.to_lowercase();

    let supported = matches!(
        target_ext.as_str(),
        "wav" | "aiff" | "flac" | "mp3" | "ogg" | "opus" | "m4a" | "aac" | "wma"
    );
    if !supported {
        return Err(format!(
            "\"{}\" isn't wired up for audio conversion yet.",
            target_ext
        ));
    }

    report(5);

    check_cancelled()?;

    let source_ext = source_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    let extracted = if target_ext == "aac" { extract_aac(&source_path, &source_ext) } else { None };
    if let Some(adts) = extracted {
        fs::write(&output_path, adts).map_err(|e| format!("Couldn't write the output file: {e}"))?;
    } else {
        let (samples, sample_rate, channels) = decode_to_pcm(&source_path)?;
        let (samples, channels) = if matches!(target_ext.as_str(), "mp3" | "opus" | "m4a" | "aac" | "wma") {
            transcode::to_stereo_or_mono(samples, channels)
        } else {
            (samples, channels)
        };

        report(50);

        check_cancelled()?;

        match target_ext.as_str() {
            "wav" => encode_wav(&samples, sample_rate, channels, &output_path)?,
            "aiff" => encode_aiff(&samples, sample_rate, channels, &output_path)?,
            "flac" => encode_flac(&samples, sample_rate, channels, &output_path)?,
            "mp3" => encode_mp3(&samples, sample_rate, channels, &output_path, quality)?,
            "ogg" => encode_ogg_vorbis(&samples, sample_rate, channels, &output_path, quality)?,
            "opus" => encode_opus(&samples, sample_rate, channels, &output_path, quality)?,
            "m4a" => encode_m4a(&samples, sample_rate, channels, &output_path, quality)?,
            "aac" => encode_aac(&samples, sample_rate, channels, &output_path, quality)?,
            "wma" => encode_wma(&samples, sample_rate, channels, &output_path, quality)?,
            _ => unreachable!(),
        }
    }

    report(85);

    check_cancelled()?;

    if preserve_date {
        preserve_file_date(&source_path, &output_path);
    }

    if overwrite && output_path != source_path {
        let _ = fs::remove_file(&source_path);
    }

    report(100);

    check_cancelled()?;

    Ok(output_path.to_string_lossy().to_string())
    })();
    discard_if_cancelled(&result, &output_path, started);
    result
    })
    .await
    .map_err(|e| format!("Conversion task panicked: {e}"))?
}


fn mkv_codec_from_mf(
    major_type: &windows::core::GUID,
    subtype: &windows::core::GUID,
    native_type: &windows::Win32::Media::MediaFoundation::IMFMediaType,
) -> Result<(String, Vec<u8>), String> {
    use windows::Win32::Media::MediaFoundation::{
        MFAudioFormat_AAC, MFAudioFormat_MP3, MFMediaType_Video, MFVideoFormat_H264,
        MFVideoFormat_HEVC, MF_MT_MPEG_SEQUENCE_HEADER, MF_MT_USER_DATA,
    };

    let get_blob = |guid: &windows::core::GUID| -> Vec<u8> {
        let size = match unsafe { native_type.GetBlobSize(guid) } {
            Ok(s) if s > 0 => s,
            _ => return Vec::new(),
        };
        let mut buf = vec![0u8; size as usize];
        if unsafe { native_type.GetBlob(guid, &mut buf, None) }.is_err() {
            return Vec::new();
        }
        buf
    };

    if *major_type == MFMediaType_Video {
        if *subtype == MFVideoFormat_H264 {
            let extradata = get_blob(&MF_MT_MPEG_SEQUENCE_HEADER);
            if extradata.is_empty() {
                return Err("This H.264 track doesn't expose a sequence header, so it can't be repackaged into MKV without re-encoding.".to_string());
            }
            let avcc = build_avcc_from_annexb(&extradata)?;
            return Ok(("V_MPEG4/ISO/AVC".to_string(), avcc));
        }
        if *subtype == MFVideoFormat_HEVC {
            return Err("HEVC (H.265) video can't be converted to MKV yet. Only H.264 video is supported.".to_string());
        }
        return Err("This file's video codec isn't H.264 or HEVC, so it can't be repackaged into MKV without re-encoding.".to_string());
    }

    if *subtype == MFAudioFormat_AAC {
        let raw = get_blob(&MF_MT_USER_DATA);
        let asc = if raw.len() > 12 { raw[12..].to_vec() } else { raw };
        if asc.is_empty() {
            return Err("This AAC track's decoder configuration couldn't be read, so it can't be repackaged into MKV without re-encoding.".to_string());
        }
        return Ok(("A_AAC".to_string(), asc));
    }
    if *subtype == MFAudioFormat_MP3 {
        return Ok(("A_MPEG/L3".to_string(), Vec::new()));
    }

    Err("This file's audio codec isn't AAC or MP3, so it can't be repackaged into MKV without re-encoding.".to_string())
}

fn split_annexb_nalus(data: &[u8]) -> Vec<&[u8]> {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i + 2 < data.len() {
        if data[i] == 0 && data[i + 1] == 0 {
            if data[i + 2] == 1 {
                starts.push((i, 3));
                i += 3;
                continue;
            } else if i + 3 < data.len() && data[i + 2] == 0 && data[i + 3] == 1 {
                starts.push((i, 4));
                i += 4;
                continue;
            }
        }
        i += 1;
    }
    if starts.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(starts.len());
    for idx in 0..starts.len() {
        let (start, code_len) = starts[idx];
        let nalu_start = start + code_len;
        let nalu_end = if idx + 1 < starts.len() { starts[idx + 1].0 } else { data.len() };
        if nalu_end > nalu_start {
            out.push(&data[nalu_start..nalu_end]);
        }
    }
    out
}

fn annexb_to_length_prefixed(data: &[u8]) -> Vec<u8> {
    let nalus = split_annexb_nalus(data);
    if nalus.is_empty() {
        return data.to_vec();
    }
    let mut out = Vec::with_capacity(data.len() + nalus.len() * 4);
    for nalu in nalus {
        out.extend_from_slice(&(nalu.len() as u32).to_be_bytes());
        out.extend_from_slice(nalu);
    }
    out
}

fn build_avcc_from_annexb(extradata: &[u8]) -> Result<Vec<u8>, String> {
    let nalus = split_annexb_nalus(extradata);
    if nalus.is_empty() {
        return Err("The H.264 sequence header wasn't in the expected Annex-B start-code format.".to_string());
    }
    let mut sps_list: Vec<&[u8]> = Vec::new();
    let mut pps_list: Vec<&[u8]> = Vec::new();
    for nalu in &nalus {
        if nalu.is_empty() {
            continue;
        }
        match nalu[0] & 0x1F {
            7 => sps_list.push(nalu),
            8 => pps_list.push(nalu),
            _ => {}
        }
    }
    let sps = sps_list
        .first()
        .ok_or_else(|| "No SPS NAL unit found in this H.264 track's sequence header.".to_string())?;
    if sps.len() < 4 {
        return Err("This H.264 track's SPS NAL unit is too short to read profile/level from.".to_string());
    }
    if pps_list.is_empty() {
        return Err("No PPS NAL unit found in this H.264 track's sequence header.".to_string());
    }

    let mut out = Vec::new();
    out.push(1u8);
    out.push(sps[1]);
    out.push(sps[2]);
    out.push(sps[3]);
    out.push(0xFF);
    out.push(0xE0 | (sps_list.len() as u8 & 0x1F));
    for s in &sps_list {
        out.extend_from_slice(&(s.len() as u16).to_be_bytes());
        out.extend_from_slice(s);
    }
    out.push(pps_list.len() as u8);
    for p in &pps_list {
        out.extend_from_slice(&(p.len() as u16).to_be_bytes());
        out.extend_from_slice(p);
    }
    Ok(out)
}

fn remux_video_container(source_path: &Path, output_path: &Path, target_ext: &str) -> Result<(), String> {
    use std::collections::HashMap;
    use windows::core::HSTRING;
    use windows::Win32::Media::MediaFoundation::{
        IMFAttributes, IMFMediaType, IMFSample, IMFSourceReader, MFCreateAttributes, MFCreateSinkWriterFromURL,
        MFCreateSourceReaderFromURL, MFMediaType_Audio, MFMediaType_Video, MFShutdown, MFStartup, MF_MT_MAJOR_TYPE,
        MF_READWRITE_DISABLE_CONVERTERS, MF_SINK_WRITER_DISABLE_THROTTLING, MF_SOURCE_READERF_ENDOFSTREAM,
        MF_SOURCE_READER_ALL_STREAMS, MF_SOURCE_READER_ANY_STREAM, MF_VERSION, MFSTARTUP_FULL,
    };
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

    let (write_path, needs_rename) = if target_ext == "mov" {
        (output_path.with_extension("__ftools_mov_tmp__.mp4"), true)
    } else {
        (output_path.to_path_buf(), false)
    };

    let source_url = HSTRING::from(source_path.to_string_lossy().as_ref());
    let write_url = HSTRING::from(write_path.to_string_lossy().as_ref());

    let job = CONVERSION_JOB.with(|current| current.get());
    let handle = std::thread::spawn(move || -> Result<(), String> {
        CONVERSION_JOB.with(|current| current.set(job));
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .map_err(|e| format!("Couldn't initialize COM on the remux thread: {e}"))?;

            let result: Result<(), String> = (|| {
                MFStartup(MF_VERSION, MFSTARTUP_FULL).map_err(|e| format!("Couldn't start Media Foundation: {e}"))?;

                let inner: Result<(), String> = (|| {
                    let mut reader_attrs: Option<IMFAttributes> = None;
                    MFCreateAttributes(&mut reader_attrs, 1).map_err(|e| e.to_string())?;
                    let reader_attrs = reader_attrs
                        .ok_or_else(|| "Media Foundation didn't return an attributes object.".to_string())?;
                    reader_attrs
                        .SetUINT32(&MF_READWRITE_DISABLE_CONVERTERS, 1)
                        .map_err(|e| e.to_string())?;

                    let reader: IMFSourceReader = MFCreateSourceReaderFromURL(&source_url, &reader_attrs)
                        .map_err(|e| format!("Couldn't open the source file: {e}"))?;

                    reader
                        .SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS.0 as u32, false)
                        .map_err(|e| e.to_string())?;

                    let mut writer_attrs: Option<IMFAttributes> = None;
                    MFCreateAttributes(&mut writer_attrs, 1).map_err(|e| e.to_string())?;
                    let writer_attrs = writer_attrs
                        .ok_or_else(|| "Media Foundation didn't return an attributes object.".to_string())?;
                    writer_attrs
                        .SetUINT32(&MF_SINK_WRITER_DISABLE_THROTTLING, 1)
                        .map_err(|e| e.to_string())?;

                    let writer = MFCreateSinkWriterFromURL(&write_url, None, &writer_attrs)
                        .map_err(|e| format!("Couldn't create the output writer: {e}"))?;

                    let mut stream_map: HashMap<u32, u32> = HashMap::new();
                    let mut stream_index: u32 = 0;
                    loop {
                        let native_type: IMFMediaType = match reader.GetNativeMediaType(stream_index, 0) {
                            Ok(t) => t,
                            Err(_) => break,
                        };

                        let major_type = native_type.GetGUID(&MF_MT_MAJOR_TYPE).map_err(|e| e.to_string())?;
                        if major_type == MFMediaType_Video || major_type == MFMediaType_Audio {
                            reader.SetStreamSelection(stream_index, true).map_err(|e| e.to_string())?;
                            reader.SetCurrentMediaType(stream_index, None, &native_type).map_err(|e| {
                                format!("This file's stream {stream_index} uses a format that can't be copied through as-is: {e}")
                            })?;

                            let output_index = writer
                                .AddStream(&native_type)
                                .map_err(|e| format!("Couldn't configure output stream {stream_index}: {e}"))?;
                            writer.SetInputMediaType(output_index, &native_type, None).map_err(|e| {
                                format!("Couldn't match output stream {stream_index} to the source format: {e}")
                            })?;

                            stream_map.insert(stream_index, output_index);
                        }

                        stream_index += 1;
                    }

                    if stream_map.is_empty() {
                        return Err("Couldn't find a video or audio stream to copy in this file.".to_string());
                    }

                    writer.BeginWriting().map_err(|e| format!("Couldn't begin writing the output file: {e}"))?;

                    loop {
                        check_cancelled()?;
                        let mut actual_stream_index: u32 = 0;
                        let mut stream_flags: u32 = 0;
                        let mut timestamp: i64 = 0;
                        let mut sample: Option<IMFSample> = None;

                        reader
                            .ReadSample(
                                MF_SOURCE_READER_ANY_STREAM.0 as u32,
                                0,
                                Some(&mut actual_stream_index),
                                Some(&mut stream_flags),
                                Some(&mut timestamp),
                                Some(&mut sample),
                            )
                            .map_err(|e| format!("Error while reading media data: {e}"))?;

                        if stream_flags & (MF_SOURCE_READERF_ENDOFSTREAM.0 as u32) != 0 {
                            break;
                        }

                        if let Some(sample) = sample {
                            if let Some(&output_index) = stream_map.get(&actual_stream_index) {
                                writer
                                    .WriteSample(output_index, &sample)
                                    .map_err(|e| format!("Couldn't write media data: {e}"))?;
                            }
                        }
                    }

                    writer.Finalize().map_err(|e| format!("Couldn't finalize the output file: {e}"))?;
                    Ok(())
                })();

                let _ = MFShutdown();
                inner
            })();

            CoUninitialize();
            result
        }
    });

    let joined = handle
        .join()
        .unwrap_or_else(|_| Err("The Media Foundation remux thread panicked.".to_string()));
    if joined.is_err() && needs_rename {
        let _ = fs::remove_file(&write_path);
    }
    joined?;

    if needs_rename {
        fs::rename(&write_path, output_path).map_err(|e| {
            let _ = fs::remove_file(&write_path);
            format!("Couldn't finish writing the .mov file: {e}")
        })?;
    }

    Ok(())
}

fn remux_container_to_mkv(source_path: &Path, output_path: &Path) -> Result<(), String> {
    use windows::core::HSTRING;
    use windows::Win32::Media::MediaFoundation::{
        IMFAttributes, IMFMediaType, IMFSample, IMFSourceReader, MFCreateAttributes,
        MFCreateSourceReaderFromURL, MFMediaType_Audio, MFMediaType_Video,
        MFSampleExtension_CleanPoint, MFShutdown, MFStartup, MF_MT_AUDIO_NUM_CHANNELS,
        MF_MT_AUDIO_SAMPLES_PER_SECOND, MF_MT_FRAME_SIZE, MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE,
        MF_READWRITE_DISABLE_CONVERTERS, MF_SOURCE_READERF_ENDOFSTREAM,
        MF_SOURCE_READER_ALL_STREAMS, MF_SOURCE_READER_ANY_STREAM, MF_VERSION, MFSTARTUP_FULL,
    };
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

    struct TrackOut {
        number: u64,
        codec_id: String,
        codec_private: Vec<u8>,
        track_type: u64,
        width: Option<u64>,
        height: Option<u64>,
        sample_rate: Option<f64>,
        channels: Option<u64>,
    }
    struct Pkt { track_number: u64, ts_100ns: i64, keyframe: bool, data: Vec<u8> }

    let source_url = HSTRING::from(source_path.to_string_lossy().as_ref());

    let job = CONVERSION_JOB.with(|current| current.get());
    let handle = std::thread::spawn(move || -> Result<(Vec<TrackOut>, Vec<Pkt>), String> {
        CONVERSION_JOB.with(|current| current.set(job));
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .map_err(|e| format!("Couldn't initialize COM on the remux thread: {e}"))?;

            let result: Result<(Vec<TrackOut>, Vec<Pkt>), String> = (|| {
                MFStartup(MF_VERSION, MFSTARTUP_FULL).map_err(|e| format!("Couldn't start Media Foundation: {e}"))?;

                let inner: Result<(Vec<TrackOut>, Vec<Pkt>), String> = (|| {
                    let mut reader_attrs: Option<IMFAttributes> = None;
                    MFCreateAttributes(&mut reader_attrs, 1).map_err(|e| e.to_string())?;
                    let reader_attrs = reader_attrs
                        .ok_or_else(|| "Media Foundation didn't return an attributes object.".to_string())?;
                    reader_attrs
                        .SetUINT32(&MF_READWRITE_DISABLE_CONVERTERS, 1)
                        .map_err(|e| e.to_string())?;

                    let reader: IMFSourceReader = MFCreateSourceReaderFromURL(&source_url, &reader_attrs)
                        .map_err(|e| format!("Couldn't open the source file: {e}"))?;

                    reader
                        .SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS.0 as u32, false)
                        .map_err(|e| e.to_string())?;

                    let mut tracks: Vec<TrackOut> = Vec::new();
                    let mut track_number_by_stream: std::collections::HashMap<u32, u64> = std::collections::HashMap::new();
                    let mut stream_index: u32 = 0;
                    let mut next_track_number: u64 = 1;
                    loop {
                        let native_type: IMFMediaType = match reader.GetNativeMediaType(stream_index, 0) {
                            Ok(t) => t,
                            Err(_) => break,
                        };

                        let major_type = native_type.GetGUID(&MF_MT_MAJOR_TYPE).map_err(|e| e.to_string())?;
                        if major_type == MFMediaType_Video || major_type == MFMediaType_Audio {
                            let subtype = native_type.GetGUID(&MF_MT_SUBTYPE).map_err(|e| e.to_string())?;
                            let (codec_id, codec_private) = mkv_codec_from_mf(&major_type, &subtype, &native_type)?;

                            reader.SetStreamSelection(stream_index, true).map_err(|e| e.to_string())?;
                            reader.SetCurrentMediaType(stream_index, None, &native_type).map_err(|e| {
                                format!("This file's stream {stream_index} uses a format that can't be copied through as-is: {e}")
                            })?;

                            let track_number = next_track_number;
                            next_track_number += 1;
                            track_number_by_stream.insert(stream_index, track_number);

                            if major_type == MFMediaType_Video {
                                let packed = native_type.GetUINT64(&MF_MT_FRAME_SIZE).unwrap_or(0);
                                tracks.push(TrackOut {
                                    number: track_number, codec_id, codec_private, track_type: 1,
                                    width: Some(packed >> 32), height: Some(packed & 0xFFFF_FFFF),
                                    sample_rate: None, channels: None,
                                });
                            } else {
                                let rate = native_type.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND).unwrap_or(0) as f64;
                                let channels = native_type.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS).unwrap_or(0) as u64;
                                tracks.push(TrackOut {
                                    number: track_number, codec_id, codec_private, track_type: 2,
                                    width: None, height: None,
                                    sample_rate: Some(rate), channels: Some(channels),
                                });
                            }
                        }
                        stream_index += 1;
                    }

                    if tracks.is_empty() {
                        return Err("Couldn't find a video or audio stream to copy in this file.".to_string());
                    }

                    let video_track_numbers: std::collections::HashSet<u64> =
                        tracks.iter().filter(|t| t.track_type == 1).map(|t| t.number).collect();

                    let mut packets: Vec<Pkt> = Vec::new();
                    loop {
                        check_cancelled()?;
                        let mut actual_stream_index: u32 = 0;
                        let mut stream_flags: u32 = 0;
                        let mut timestamp: i64 = 0;
                        let mut sample: Option<IMFSample> = None;

                        reader
                            .ReadSample(
                                MF_SOURCE_READER_ANY_STREAM.0 as u32,
                                0,
                                Some(&mut actual_stream_index),
                                Some(&mut stream_flags),
                                Some(&mut timestamp),
                                Some(&mut sample),
                            )
                            .map_err(|e| format!("Error while reading media data: {e}"))?;

                        if stream_flags & (MF_SOURCE_READERF_ENDOFSTREAM.0 as u32) != 0 {
                            break;
                        }

                        if let Some(sample) = sample {
                            if let Some(&track_number) = track_number_by_stream.get(&actual_stream_index) {
                                let keyframe = sample
                                    .GetUINT32(&MFSampleExtension_CleanPoint)
                                    .map(|v| v != 0)
                                    .unwrap_or(true);

                                let buffer = sample.ConvertToContiguousBuffer().map_err(|e| e.to_string())?;
                                let mut ptr: *mut u8 = std::ptr::null_mut();
                                let mut len: u32 = 0;
                                buffer.Lock(&mut ptr, None, Some(&mut len)).map_err(|e| e.to_string())?;
                                let mut data = std::slice::from_raw_parts(ptr, len as usize).to_vec();
                                buffer.Unlock().map_err(|e| e.to_string())?;
                                if video_track_numbers.contains(&track_number) {
                                    data = annexb_to_length_prefixed(&data);
                                }

                                packets.push(Pkt { track_number, ts_100ns: timestamp, keyframe, data });
                            }
                        }
                    }

                    Ok((tracks, packets))
                })();

                let _ = MFShutdown();
                inner
            })();

            CoUninitialize();
            result
        }
    });

    let (tracks, mut packets) = handle
        .join()
        .unwrap_or_else(|_| Err("The Media Foundation remux thread panicked.".to_string()))?;

    let mut tracks_body = Vec::new();
    for t in &tracks {
        let mut te = Vec::new();
        te.extend(ebml::build_elem(ebml::ID_TRACK_NUMBER, &ebml::uint_body(t.number)));
        te.extend(ebml::build_elem(ebml::ID_TRACK_UID, &ebml::uint_body(t.number)));
        te.extend(ebml::build_elem(ebml::ID_TRACK_TYPE, &ebml::uint_body(t.track_type)));
        te.extend(ebml::build_elem(ebml::ID_CODEC_ID, t.codec_id.as_bytes()));
        if !t.codec_private.is_empty() {
            te.extend(ebml::build_elem(ebml::ID_CODEC_PRIVATE, &t.codec_private));
        }
        if let (Some(w), Some(h)) = (t.width, t.height) {
            let mut video_body = Vec::new();
            video_body.extend(ebml::build_elem(ebml::ID_PIXEL_WIDTH, &ebml::uint_body(w)));
            video_body.extend(ebml::build_elem(ebml::ID_PIXEL_HEIGHT, &ebml::uint_body(h)));
            te.extend(ebml::build_elem(ebml::ID_VIDEO, &video_body));
        }
        if let (Some(rate), Some(ch)) = (t.sample_rate, t.channels) {
            let mut audio_body = Vec::new();
            audio_body.extend(ebml::build_elem(ebml::ID_SAMPLING_FREQUENCY, &ebml::float_body_f64(rate)));
            audio_body.extend(ebml::build_elem(ebml::ID_CHANNELS, &ebml::uint_body(ch)));
            te.extend(ebml::build_elem(ebml::ID_AUDIO, &audio_body));
        }
        tracks_body.extend(ebml::build_elem(ebml::ID_TRACK_ENTRY, &te));
    }

    packets.sort_by_key(|p| p.ts_100ns);
    let total_duration_ms = packets.iter().map(|p| p.ts_100ns / 10_000).max().unwrap_or(0) as f64 + 20.0;
    let mut info_body = Vec::new();
    info_body.extend(ebml::build_elem(ebml::ID_TIMESTAMP_SCALE, &ebml::uint_body(1_000_000)));
    info_body.extend(ebml::build_elem(ebml::ID_DURATION, &ebml::float_body_f64(total_duration_ms)));
    info_body.extend(ebml::build_elem(ebml::ID_MUXING_APP, b"fTools"));
    info_body.extend(ebml::build_elem(ebml::ID_WRITING_APP, b"fTools"));

    let mut clusters_body: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < packets.len() {
        let cluster_start_ms = packets[i].ts_100ns / 10_000;
        let mut cluster_body = Vec::new();
        cluster_body.extend(ebml::build_elem(ebml::ID_TIMESTAMP, &ebml::uint_body(cluster_start_ms.max(0) as u64)));
        while i < packets.len() && (packets[i].ts_100ns / 10_000) - cluster_start_ms < 1000 {
            let ts_ms = packets[i].ts_100ns / 10_000;
            let rel = (ts_ms - cluster_start_ms) as i16;
            let block = ebml::simple_block_body(packets[i].track_number, rel, packets[i].keyframe, &packets[i].data);
            cluster_body.extend(ebml::build_elem(ebml::ID_SIMPLE_BLOCK, &block));
            i += 1;
        }
        clusters_body.extend(ebml::build_elem(ebml::ID_CLUSTER, &cluster_body));
    }

    let mut segment_body = Vec::new();
    segment_body.extend(ebml::build_elem(ebml::ID_INFO, &info_body));
    segment_body.extend(ebml::build_elem(ebml::ID_TRACKS, &tracks_body));
    segment_body.extend(clusters_body);

    let mut ebml_header_body = Vec::new();
    ebml_header_body.extend(ebml::build_elem(ebml::ID_EBML_VERSION, &ebml::uint_body(1)));
    ebml_header_body.extend(ebml::build_elem(ebml::ID_EBML_READ_VERSION, &ebml::uint_body(1)));
    ebml_header_body.extend(ebml::build_elem(ebml::ID_EBML_MAX_ID_LENGTH, &ebml::uint_body(4)));
    ebml_header_body.extend(ebml::build_elem(ebml::ID_EBML_MAX_SIZE_LENGTH, &ebml::uint_body(8)));
    ebml_header_body.extend(ebml::build_elem(ebml::ID_DOCTYPE, b"matroska"));
    ebml_header_body.extend(ebml::build_elem(ebml::ID_DOCTYPE_VERSION, &ebml::uint_body(2)));
    ebml_header_body.extend(ebml::build_elem(ebml::ID_DOCTYPE_READ_VERSION, &ebml::uint_body(2)));

    let mut out = fs::File::create(output_path).map_err(|e| format!("Couldn't create the output file: {e}"))?;
    out.write_all(&ebml::build_elem(ebml::ID_EBML, &ebml_header_body))
        .map_err(|e| format!("Couldn't write the output file: {e}"))?;
    out.write_all(&ebml::build_elem(ebml::ID_SEGMENT, &segment_body))
        .map_err(|e| format!("Couldn't write the output file: {e}"))?;

    Ok(())
}

fn read_container<'a>(raw: &'a [u8], ext: &str) -> Result<Vec<demux::Track<'a>>, String> {
    match ext {
        "mkv" | "webm" => demux::read_matroska(raw),
        "flv" => flv::read(raw),
        "avi" => avi::read(raw),
        "mp4" | "mov" | "m4a" => demux::read_mp4(raw),
        other => Err(format!("Reading .{other} files isn't supported here.")),
    }
}

fn encode_aac_for_mux(samples: &[f32], sample_rate: u32, channels: u16, quality: transcode::Quality) -> Result<Vec<u8>, String> {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let temp = std::env::temp_dir().join(format!("ftools_aac_{}_{nanos}.m4a", std::process::id()));
    let result = encode_m4a(samples, sample_rate, channels, &temp, quality)
        .and_then(|_| fs::read(&temp).map_err(|e| format!("Couldn't read the encoded audio back: {e}")));
    let _ = fs::remove_file(&temp);
    result
}

fn write_output(output_path: &Path, write: impl FnOnce(&mut std::io::BufWriter<fs::File>) -> Result<(), String>) -> Result<(), String> {
    let file = fs::File::create(output_path).map_err(|e| format!("Couldn't create the output file: {e}"))?;
    let mut writer = std::io::BufWriter::new(file);
    let result = write(&mut writer).and_then(|_| writer.flush().map_err(|e| format!("Couldn't write the output file: {e}")));
    drop(writer);
    if result.is_err() {
        let _ = fs::remove_file(output_path);
    }
    result
}

fn media_target(ext: &str) -> transcode::Target {
    match ext {
        "webm" => transcode::Target::WebM,
        "avi" => transcode::Target::Avi,
        _ => transcode::Target::Mp4,
    }
}

fn encode_audio_for(target: transcode::Target, samples: &[f32], rate: u32, channels: u16, quality: transcode::Quality) -> Result<Vec<u8>, String> {
    if target == transcode::Target::Avi {
        encode_mp3_bytes(samples, rate, channels, quality)
    } else {
        encode_aac_for_mux(samples, rate, channels, quality)
    }
}

fn write_tracks(output_path: &Path, tracks: &[demux::Track], target_ext: &str) -> Result<(), String> {
    write_output(output_path, |w| match target_ext {
        "mp4" | "mov" => mp4::write(tracks, w),
        "flv" => flv::write(tracks, w),
        "avi" => avi::write(tracks, w),
        "webm" => matroska::write(tracks, true, w),
        _ => matroska::write(tracks, false, w),
    })
}

fn convert_to_gif(source_path: &Path, output_path: &Path, source_ext: &str, report: &dyn Fn(u8)) -> Result<(), String> {
    let raw = fs::read(source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;
    let tracks = read_container(&raw, source_ext)?;
    let total = tracks.iter().find(|t| t.video).map(|t| t.samples.len()).unwrap_or(0).max(1);
    let progress = |i: usize| report((10 + i * 75 / total).min(85) as u8);
    write_output(output_path, |w| animation::video_to_gif(&tracks, w, &progress))
}

fn convert_from_gif(source_path: &Path, output_path: &Path, target_ext: &str, quality: transcode::Quality) -> Result<(), String> {
    let raw = fs::read(source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;
    let target = if target_ext == "webm" { transcode::Target::WebM } else { transcode::Target::Mp4 };
    let track = animation::gif_to_video(&raw, target, quality)?;
    write_tracks(output_path, &[track], target_ext)
}

fn convert_container(
    source_path: &Path,
    output_path: &Path,
    source_ext: &str,
    target_ext: &str,
    quality: transcode::Quality,
    report: &dyn Fn(u8),
) -> Result<(), String> {
    let raw = fs::read(source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;
    let tracks = read_container(&raw, source_ext)?;
    if tracks.is_empty() {
        return Err("No audio or video tracks were found in this file.".to_string());
    }
    let target = media_target(target_ext);
    let encoder = |samples: &[f32], rate: u32, channels: u16| encode_audio_for(target, samples, rate, channels, quality);
    let tracks = match target_ext {
        "mkv" => tracks,
        _ => transcode::prepare(tracks, target, &encoder, quality, report)?,
    };
    write_tracks(output_path, &tracks, target_ext)
}

fn on_media_foundation_thread<T: Send>(work: impl FnOnce() -> Result<T, String> + Send) -> Result<T, String> {
    let job = CONVERSION_JOB.with(|current| current.get());
    std::thread::scope(|scope| {
        scope
            .spawn(move || {
                CONVERSION_JOB.with(|current| current.set(job));
                let _session = wmv::Session::start()?;
                work()
            })
            .join()
            .unwrap_or_else(|_| Err("The Media Foundation thread panicked.".to_string()))
    })
}

fn decode_with_media_foundation(source_path: &Path) -> Result<(Vec<f32>, u32, u16), String> {
    on_media_foundation_thread(|| {
        let mut pcm = Vec::new();
        let mut format = None;
        wmv::read(source_path, false, true, |media| {
            check_cancelled()?;
            if let wmv::Media::Audio { pcm: chunk, rate, channels, .. } = media {
                format.get_or_insert((rate, channels));
                pcm.extend_from_slice(chunk);
            }
            Ok(())
        })?;
        let (rate, channels) = format.ok_or("This file has no audio track that Windows can decode.")?;
        if pcm.is_empty() {
            return Err("No audio data could be decoded from this file.".to_string());
        }
        Ok((pcm, rate, channels))
    })
}

type DecodedAudio = (Vec<f32>, u32, u16, i64);

fn wmv_to_tracks(
    source_path: &Path,
    target: transcode::Target,
    quality: transcode::Quality,
    report: &(dyn Fn(u8) + Sync),
) -> Result<Vec<demux::Track<'static>>, String> {
    let duration_ms = asf_duration_ms(source_path).unwrap_or(0);
    let file_bits = fs::metadata(source_path).map(|m| m.len() * 8).unwrap_or(0);
    let source_bps = if duration_ms > 0 { file_bits * 1000 / duration_ms } else { 2_000_000 };
    let factor = if target == transcode::Target::WebM { 0.8 } else { 1.5 } * quality.pick(0.5, 0.75, 1.0, 1.6);
    let bitrate = (source_bps.saturating_sub(128_000).max(300_000) as f64 * factor) as u64;

    let (video, audio) = on_media_foundation_thread(|| -> Result<(Option<demux::Track<'static>>, Option<DecodedAudio>), String> {
        let mut encoder: Option<transcode::VideoEncoder> = None;
        let mut packets = Vec::new();
        let mut size = (0usize, 0usize);
        let mut frame_ms = 33u64;
        let mut start: Option<i64> = None;
        let mut pending: Option<(transcode::Picture, i64)> = None;
        let mut pcm: Vec<f32> = Vec::new();
        let mut format: Option<(u32, u16, i64)> = None;
        wmv::read(source_path, true, true, |media| {
            check_cancelled()?;
            match media {
                wmv::Media::Video { width, height, fps, i420, pts_ns } => {
                    let base = *start.get_or_insert(pts_ns);
                    if encoder.is_none() {
                        size = (width, height);
                        frame_ms = (1000.0 / fps.max(1.0)).round().max(1.0) as u64;
                        encoder = Some(transcode::VideoEncoder::new(target, width, height, bitrate, fps)?);
                    }
                    if (width, height) != size {
                        return Err("This video changes resolution midway, which isn't supported yet.".to_string());
                    }
                    if let Some((picture, previous)) = pending.take() {
                        let duration = ((pts_ns - previous) / 1_000_000).max(1) as u64;
                        packets.extend(encoder.as_mut().unwrap().encode(picture, (previous - base) / 1_000_000, duration)?);
                    }
                    pending = Some((transcode::Picture { width, height, i420: i420.to_vec() }, pts_ns));
                    if duration_ms > 0 {
                        report((10 + (pts_ns / 1_000_000).max(0) as u64 * 75 / duration_ms).min(85) as u8);
                    }
                }
                wmv::Media::Audio { pcm: chunk, rate, channels, pts_ns } => {
                    format.get_or_insert((rate, channels, pts_ns));
                    pcm.extend_from_slice(chunk);
                }
            }
            Ok(())
        })?;
        let video = match encoder {
            Some(mut encoder) => {
                let base = start.unwrap_or(0);
                if let Some((picture, previous)) = pending.take() {
                    packets.extend(encoder.encode(picture, (previous - base) / 1_000_000, frame_ms)?);
                }
                Some(transcode::encoded_video_track(encoder, packets, size.0, size.1, base)?)
            }
            None => None,
        };
        Ok((video, format.map(|(rate, channels, start)| (pcm, rate, channels, start))))
    })?;

    let mut tracks: Vec<demux::Track<'static>> = video.into_iter().collect();
    if let Some((pcm, rate, channels, start)) = audio {
        let (pcm, channels) = transcode::to_stereo_or_mono(pcm, channels);
        let encoder = |samples: &[f32], rate: u32, channels: u16| encode_audio_for(target, samples, rate, channels, quality);
        tracks.push(transcode::encode_audio_track(&pcm, rate, channels, start, target, &encoder, quality)?);
    }
    if tracks.is_empty() {
        return Err("No audio or video could be decoded from this file.".to_string());
    }
    Ok(tracks)
}

fn write_wma_until(
    writer: &wmv::Writer,
    pcm16: &[u8],
    written: &mut usize,
    block: usize,
    offset_ns: i64,
    until_ns: Option<i64>,
) -> Result<(), String> {
    let rate = writer.audio_rate.max(1) as i64;
    let chunk = (rate as usize / 10).max(1) * block;
    while *written + block <= pcm16.len() {
        let pts = offset_ns + (*written / block) as i64 * 1_000_000_000 / rate;
        if until_ns.is_some_and(|until| pts > until) {
            break;
        }
        let end = (*written + chunk).min(pcm16.len() / block * block);
        let frames = ((end - *written) / block) as i64;
        writer.audio(&pcm16[*written..end], pts, frames * 1_000_000_000 / rate)?;
        *written = end;
    }
    Ok(())
}

fn tracks_to_wmv(
    tracks: &[demux::Track],
    output_path: &Path,
    quality: transcode::Quality,
    report: &(dyn Fn(u8) + Sync),
) -> Result<(), String> {
    let video = tracks.iter().find(|t| t.video && !t.samples.is_empty());
    let audio = match tracks.iter().find(|t| !t.video && !t.samples.is_empty()) {
        Some(t) => Some(transcode::audio_for_encoding(t)?),
        None => None,
    };
    if video.is_none() && audio.is_none() {
        return Err("No audio or video tracks were found in this file.".to_string());
    }
    let timing = video.map(|t| transcode::video_timing(t).0);
    let base = timing.as_ref().map(|t| t.base_ns).into_iter().chain(audio.as_ref().map(|a| a.3)).min().unwrap_or(0);
    let fps = timing.as_ref().map(|t| (1000.0 / t.frame_ms) as f32).unwrap_or(30.0);
    let video_bps = video
        .map(|t| if t.codec == demux::Codec::Mjpeg { (t.width as f64 * t.height as f64 * fps as f64 * 0.15) as u64 } else { t.bitrate_bps() })
        .unwrap_or(0);
    let bitrate = ((video_bps as f64 * 1.5 * quality.pick(0.5, 0.75, 1.0, 1.6)) as u64).clamp(400_000, 20_000_000) as u32;
    let audio_bitrate = quality.pick(96_000, 128_000, 160_000, 192_000);
    let audio_offset = audio.as_ref().map(|a| a.3 - base).unwrap_or(0);
    let block = 2 * audio.as_ref().map(|a| a.2 as usize).unwrap_or(1);
    let total = video.map(|t| t.samples.len()).unwrap_or(1).max(1);

    on_media_foundation_thread(|| {
        let open = |size: Option<(u32, u32)>| -> Result<(wmv::Writer, Vec<u8>), String> {
            let video_spec = size.map(|(width, height)| wmv::VideoSpec { width, height, fps, bitrate });
            let audio_spec = audio.as_ref().map(|a| wmv::AudioSpec { rate: a.1, channels: a.2, bitrate: audio_bitrate });
            let writer = wmv::Writer::create(output_path, video_spec, audio_spec)?;
            let pcm16 = match &audio {
                Some((pcm, rate, channels, _)) => {
                    let resampled =
                        if writer.audio_rate == *rate { pcm.clone() } else { resample_interleaved(pcm, *channels, *rate, writer.audio_rate)? };
                    resampled.iter().flat_map(|&s| to_i16(s).to_le_bytes()).collect()
                }
                None => Vec::new(),
            };
            Ok((writer, pcm16))
        };

        let mut writer: Option<wmv::Writer> = None;
        let mut pcm16: Vec<u8> = Vec::new();
        let mut written = 0usize;
        if let (Some(t), Some(timing)) = (video, timing.as_ref()) {
            let progress = |i: usize| report((10 + i * 75 / total).min(85) as u8);
            transcode::for_each_frame(t, &progress, |picture, pts_ms, duration_ms| {
                if writer.is_none() {
                    let (w, p) = open(Some((picture.width as u32, picture.height as u32)))?;
                    writer = Some(w);
                    pcm16 = p;
                }
                let w = writer.as_ref().unwrap();
                let pts_ns = timing.base_ns - base + pts_ms * 1_000_000;
                write_wma_until(w, &pcm16, &mut written, block, audio_offset, Some(pts_ns + 1_000_000_000))?;
                w.video(&picture.i420, pts_ns, duration_ms as i64 * 1_000_000)
            })?;
        }
        let writer = match writer {
            Some(w) => w,
            None => {
                let (w, p) = open(None)?;
                pcm16 = p;
                w
            }
        };
        write_wma_until(&writer, &pcm16, &mut written, block, audio_offset, None)?;
        writer.finish()
    })
}

fn extract_aac(source_path: &Path, source_ext: &str) -> Option<Vec<u8>> {
    if !matches!(source_ext, "mp4" | "mov" | "m4a" | "mkv" | "webm" | "flv" | "avi") {
        return None;
    }
    let raw = fs::read(source_path).ok()?;
    let tracks = read_container(&raw, source_ext).ok()?;
    tracks.iter().filter(|t| !t.video).find_map(demux::write_adts)
}

#[tauri::command]
async fn convert_video(
    app: AppHandle,
    source_path: String,
    output_name: String,
    target_ext: String,
    preserve_date: bool,
    overwrite: bool,
    output_dir: Option<String>,
    job: Option<u64>,
    quality: Option<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
    start_conversion_job(job);
    let quality = transcode::Quality::from_name(quality.as_deref());
    let started = std::time::SystemTime::now();
    let output_path = output_folder(Path::new(&source_path), output_dir.as_deref())
        .join(format!("{output_name}.{}", target_ext.to_lowercase()));
    let result = (|| -> Result<String, String> {
    let report = |percent: u8| {
        let _ = app.emit("conversion-progress", percent);
    };

    let source_path = PathBuf::from(source_path);
    let target_ext = target_ext.to_lowercase();
    let source_ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    let mf_pair = matches!(source_ext.as_str(), "mp4" | "mov") && matches!(target_ext.as_str(), "mp4" | "mov");
    let mf_to_mkv = matches!(source_ext.as_str(), "mp4" | "mov") && target_ext == "mkv";
    let is_video = |ext: &str| matches!(ext, "mp4" | "mov" | "mkv" | "webm" | "flv" | "avi");
    let is_asf = |ext: &str| matches!(ext, "wmv" | "asf");
    let native = is_video(&source_ext) && is_video(&target_ext) && source_ext != target_ext;
    let to_gif = is_video(&source_ext) && target_ext == "gif";
    let from_gif = source_ext == "gif" && is_video(&target_ext);
    let to_wmv = (is_video(&source_ext) || source_ext == "gif") && target_ext == "wmv";
    let from_wmv = is_asf(&source_ext) && (is_video(&target_ext) || target_ext == "gif");

    if !mf_pair && !mf_to_mkv && !native && !to_gif && !from_gif && !to_wmv && !from_wmv {
        return Err(format!(
            "Converting \"{}\" to \"{}\" isn't supported yet.",
            source_ext, target_ext
        ));
    }

    report(5);

    check_cancelled()?;


    if to_wmv {
        let raw = fs::read(&source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;
        let tracks = if source_ext == "gif" {
            vec![animation::gif_to_video(&raw, transcode::Target::Mp4, transcode::Quality::Best)?]
        } else {
            read_container(&raw, &source_ext)?
        };
        tracks_to_wmv(&tracks, &output_path, quality, &report)?;
    } else if from_wmv {
        if target_ext == "gif" {
            let tracks = wmv_to_tracks(&source_path, transcode::Target::Mp4, transcode::Quality::Best, &report)?;
            let total = tracks.iter().find(|t| t.video).map(|t| t.samples.len()).unwrap_or(0).max(1);
            let progress = |i: usize| report((50 + i * 35 / total).min(85) as u8);
            write_output(&output_path, |w| animation::video_to_gif(&tracks, w, &progress))?;
        } else {
            let tracks = wmv_to_tracks(&source_path, media_target(&target_ext), quality, &report)?;
            write_tracks(&output_path, &tracks, &target_ext)?;
        }
    } else if mf_pair {
        remux_video_container(&source_path, &output_path, &target_ext)?;
    } else if mf_to_mkv {
        remux_container_to_mkv(&source_path, &output_path)?;
    } else if to_gif {
        convert_to_gif(&source_path, &output_path, &source_ext, &report)?;
    } else if from_gif {
        convert_from_gif(&source_path, &output_path, &target_ext, quality)?;
    } else {
        convert_container(&source_path, &output_path, &source_ext, &target_ext, quality, &report)?;
    }

    report(85);

    check_cancelled()?;

    if preserve_date {
        preserve_file_date(&source_path, &output_path);
    }

    if overwrite && output_path != source_path {
        let _ = fs::remove_file(&source_path);
    }

    report(100);

    check_cancelled()?;

    Ok(output_path.to_string_lossy().to_string())
    })();
    discard_if_cancelled(&result, &output_path, started);
    result
    })
    .await
    .map_err(|e| format!("Conversion task panicked: {e}"))?
}

/// One configured autoclicker input: either a mouse button or a keyboard key
/// (possibly a "Ctrl + Shift + F6"-style combo), matching the shape the
/// frontend already stores for the mouse/keyboard/multiple click modes.
#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum AcAction {
    Mouse { button: String },
    Keyboard { key: String },
}

struct AcMouseFlags {
    down: MOUSE_EVENT_FLAGS,
    up: MOUSE_EVENT_FLAGS,
}

fn ac_mouse_button_flags(button: &str) -> Result<AcMouseFlags, String> {
    match button {
        "left" => Ok(AcMouseFlags { down: MOUSEEVENTF_LEFTDOWN, up: MOUSEEVENTF_LEFTUP }),
        "right" => Ok(AcMouseFlags { down: MOUSEEVENTF_RIGHTDOWN, up: MOUSEEVENTF_RIGHTUP }),
        "middle" => Ok(AcMouseFlags { down: MOUSEEVENTF_MIDDLEDOWN, up: MOUSEEVENTF_MIDDLEUP }),
        other => Err(format!("Unrecognized mouse button \"{other}\" for the autoclicker.")),
    }
}

fn ac_vk_for_modifier(name: &str) -> Option<u16> {
    match name {
        "Ctrl" => Some(VK_CONTROL.0),
        "Shift" => Some(VK_SHIFT.0),
        "Alt" => Some(VK_MENU.0),
        _ => None,
    }
}

/// Maps one of the app's key-display strings (see KEY_DISPLAY_NAMES in
/// app.js) to a Windows virtual-key code. Letters/digits map straight to
/// their ASCII value, since VK_A-VK_Z and VK_0-VK_9 are defined to equal
/// ASCII 'A'-'Z' and '0'-'9' on Windows.
fn ac_vk_for_key(name: &str) -> Option<u16> {
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if c.is_ascii_uppercase() || c.is_ascii_digit() {
            return Some(c as u16);
        }
    }
    if let Some(rest) = name.strip_prefix('F') {
        if let Ok(n) = rest.parse::<u16>() {
            if (1..=24).contains(&n) {
                return Some(0x70 + (n - 1)); // VK_F1 = 0x70, contiguous through VK_F24 = 0x87
            }
        }
    }
    match name {
        "Space" => Some(VK_SPACE.0),
        "Enter" => Some(VK_RETURN.0),
        "Tab" => Some(VK_TAB.0),
        "Backspace" => Some(VK_BACK.0),
        "Del" => Some(VK_DELETE.0),
        "Caps Lock" => Some(VK_CAPITAL.0),
        "PgUp" => Some(VK_PRIOR.0),
        "PgDn" => Some(VK_NEXT.0),
        "Home" => Some(VK_HOME.0),
        "End" => Some(VK_END.0),
        "Insert" => Some(VK_INSERT.0),
        "Up Arrow" => Some(VK_UP.0),
        "Down Arrow" => Some(VK_DOWN.0),
        "Left Arrow" => Some(VK_LEFT.0),
        "Right Arrow" => Some(VK_RIGHT.0),
        "Comma" => Some(VK_OEM_COMMA.0),
        "Period" => Some(VK_OEM_PERIOD.0),
        "Semicolon" => Some(VK_OEM_1.0),
        "Quote" => Some(VK_OEM_7.0),
        "Slash" => Some(VK_OEM_2.0),
        "Backslash" => Some(VK_OEM_5.0),
        "[" => Some(VK_OEM_4.0),
        "]" => Some(VK_OEM_6.0),
        "-" => Some(VK_OEM_MINUS.0),
        "=" => Some(VK_OEM_PLUS.0),
        "`" => Some(VK_OEM_3.0),
        _ => None,
    }
}

/// Splits a "Ctrl + Shift + F6"-style display string into its modifier VK
/// codes and its main-key VK code.
fn ac_parse_key_combo(display: &str) -> Option<(Vec<u16>, u16)> {
    let parts: Vec<&str> = display.split(" + ").collect();
    let (main, mods) = parts.split_last()?;
    let main_vk = ac_vk_for_key(main)?;
    let mod_vks = mods.iter().filter_map(|m| ac_vk_for_modifier(m)).collect();
    Some((mod_vks, main_vk))
}

/// A key ready to press: its virtual-key code plus the real hardware scan
/// code (and extended-key bit) a physical keyboard would report for it.
#[derive(Clone, Copy)]
struct AcKeyPress {
    vk: u16,
    scan: u16,
    extended: bool,
}

/// Resolves a VK code to the scan code (and extended-key bit) a real
/// keyboard driver would report for it. Computed once per key when a run
/// starts, not on every individual press, both because it's a syscall and
/// because the mapping never changes mid-run.
///
/// This matters for two separate problems: some CPS testers and games
/// (DirectInput/raw-input titles in particular) read the scan code rather
/// than, or in addition to, the virtual key, and silently ignore or flag
/// synthetic-looking events that only carry a virtual key with wScan left
/// at 0, which is what a plain SendInput call defaults to.
fn ac_resolve_key_press(vk: u16) -> AcKeyPress {
    // MAPVK_VK_TO_VSC_EX also encodes the E0/E1 extended-key prefix in the
    // high byte for keys like the arrows, Home/End/PageUp/PageDown/Insert/
    // Delete, so this doubles as the extended-key check, no separate list
    // of "which keys are extended" to keep in sync.
    let mapped = unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC_EX) };
    let scan = (mapped & 0xFF) as u16;
    let prefix = mapped & 0xFF00;
    let extended = prefix == 0xE000 || prefix == 0xE100;
    AcKeyPress { vk, scan, extended }
}

fn ac_send_key_event(press: AcKeyPress, key_up: bool) {
    // Scan code is sent alongside the virtual key (not instead of it): most
    // things read one or the other, and Windows fills in the other side
    // for anything reading via a different path (e.g. a low-level keyboard
    // hook still gets a real vkCode even though KEYEVENTF_SCANCODE was
    // used), so populating both is what a real keyboard driver effectively
    // produces and is the most broadly compatible.
    let mut dw_flags = KEYEVENTF_SCANCODE;
    if press.extended {
        dw_flags |= KEYEVENTF_EXTENDEDKEY;
    }
    if key_up {
        dw_flags |= KEYEVENTF_KEYUP;
    }
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(press.vk),
                wScan: press.scan,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

fn ac_send_mouse_event(flags: MOUSE_EVENT_FLAGS) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// Returns a duration randomized a bit around `base_ms` (roughly ±12%,
/// floor of 1ms), used for both the wait between clicks and the hold time.
/// A real person never presses at a perfectly uniform rate or holds a key
/// for the exact same number of milliseconds every time; a couple of the
/// CPS testers this was checked against flag input specifically for having
/// zero timing variance, which a fixed interval always has.
fn ac_jittered_duration(base_ms: u64) -> std::time::Duration {
    let base = base_ms.max(1) as i64;
    let amplitude = (base / 8).max(1);
    let jitter = rand::thread_rng().gen_range(-amplitude..=amplitude);
    std::time::Duration::from_millis((base + jitter).max(1) as u64)
}

fn ac_randomized_duration(base_ms: u64, percent: u64) -> std::time::Duration {
    let base = base_ms.max(1) as i64;
    let amplitude = base * percent.min(90) as i64 / 100;
    let jitter = if amplitude > 0 { rand::thread_rng().gen_range(-amplitude..=amplitude) } else { 0 };
    std::time::Duration::from_millis((base + jitter).max(1) as u64)
}

enum AcResolvedAction {
    Mouse(AcMouseFlags),
    Keyboard(Vec<AcKeyPress>, AcKeyPress),
}

/// Tracks the autoclicker's current "run generation". Bumped by
/// bump_autoclicker_generation whenever the run is started or stopped from
/// the frontend, so start_autoclicker_loop can tell a fresh run from a
/// stale one that was already in the IPC queue when Stop was pressed.
struct AcClickerState {
    generation: tokio::sync::watch::Sender<u64>,
    // Kept alive for the app's lifetime purely so `generation` always has at
    // least one live receiver. tokio::sync::watch::Sender::send() silently
    // fails to update the value when there are zero receivers, and the
    // receiver returned alongside the sender by watch::channel() would
    // otherwise be dropped immediately, which broke the very first bump
    // (i.e. the very first Start after launch) before anything else had
    // ever subscribed.
    _generation_rx: tokio::sync::watch::Receiver<u64>,
}

/// Bumps the autoclicker's run generation and returns the new value. The
/// frontend calls this both when starting a run (to get the token every
/// tick of that run should be stamped with) and when stopping one (to
/// invalidate that token immediately, the return value is unused there).
#[tauri::command]
fn bump_autoclicker_generation(state: tauri::State<'_, AcClickerState>) -> u64 {
    let next = state.generation.borrow().wrapping_add(1);
    let _ = state.generation.send(next);
    next
}

/// RAII guard that raises the Windows system timer resolution to 1ms for as
/// long as an autoclicker run is alive, then restores it on drop. Windows'
/// default timer resolution (roughly 15.6ms) is far too coarse to hit click
/// rates anywhere near the UI's 1000 CPS ceiling; without this, waits like
/// tokio's interval/sleep are quantized to that coarser resolution and the
/// achievable rate plateaus well below what's configured.
struct AcHighResTimer;

impl AcHighResTimer {
    fn acquire() -> Self {
        unsafe {
            windows::Win32::Media::timeBeginPeriod(1);
        }
        Self
    }
}

impl Drop for AcHighResTimer {
    fn drop(&mut self) {
        unsafe {
            windows::Win32::Media::timeEndPeriod(1);
        }
    }
}

/// Runs an autoclicker session end-to-end: on every tick, presses every
/// configured action down (mouse button and/or keyboard key, for the
/// mouse/keyboard/multiple click modes), holds for `hold_ms`, releases
/// everything in reverse order, waits out the rest of the click-speed
/// interval, and repeats, until `generation` is invalidated by Stop (or a
/// new run starting). Uses SendInput, a real Windows-level input event, so
/// clicks land on whatever window/app currently has the cursor or focus,
/// exactly like the toggle hotkey works regardless of which window is
/// active.
///
/// The whole loop lives on the Rust side and is driven by a single
/// `tokio::time::interval` rather than the frontend calling back in on
/// every tick: a JS `setInterval` plus one IPC round trip per click was
/// itself the bottleneck on achievable CPS (observed plateauing well under
/// a configured 1000 CPS). One `invoke` call per run removes both.
///
/// Every action is resolved to its VK codes/button flags up front, before
/// anything is pressed, so a single unrecognized key never leaves some
/// other action's button or modifier stuck held down. `generation` is the
/// token the frontend stamped this run with when it started; every loop
/// iteration checks it against the live value in `state`, so Stop (which
/// bumps that value) ends the loop on its next tick or hold-wait, whichever
/// comes first, without ever finishing a run out on its own.
#[tauri::command]
async fn start_autoclicker_loop(
    actions: Vec<AcAction>,
    hold_ms: u64,
    interval_ms: u64,
    random_percent: Option<u64>,
    generation: u64,
    state: tauri::State<'_, AcClickerState>,
) -> Result<(), String> {
    if *state.generation.borrow() != generation {
        return Ok(());
    }

    let mut resolved = Vec::with_capacity(actions.len());
    for action in &actions {
        match action {
            AcAction::Mouse { button } => {
                resolved.push(AcResolvedAction::Mouse(ac_mouse_button_flags(button)?));
            }
            AcAction::Keyboard { key } => {
                let (mods, main) = ac_parse_key_combo(key)
                    .ok_or_else(|| format!("Unrecognized autoclicker key \"{key}\"."))?;
                let mod_presses = mods.into_iter().map(ac_resolve_key_press).collect();
                let main_press = ac_resolve_key_press(main);
                resolved.push(AcResolvedAction::Keyboard(mod_presses, main_press));
            }
        }
    }

    let random_percent = random_percent.unwrap_or(0);
    let _high_res_timer = AcHighResTimer::acquire();
    let mut generation_rx = state.generation.subscribe();

    loop {
        tokio::select! {
            _ = tokio::time::sleep(ac_randomized_duration(interval_ms, random_percent)) => {},
            _ = generation_rx.changed() => { break; }
        }
        if *generation_rx.borrow() != generation {
            break;
        }

        for action in &resolved {
            match action {
                AcResolvedAction::Mouse(flags) => ac_send_mouse_event(flags.down),
                AcResolvedAction::Keyboard(mods, main) => {
                    for m in mods {
                        ac_send_key_event(*m, false);
                    }
                    ac_send_key_event(*main, false);
                }
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(ac_jittered_duration(hold_ms)) => {},
            _ = generation_rx.changed() => {},
        }

        for action in resolved.iter().rev() {
            match action {
                AcResolvedAction::Mouse(flags) => ac_send_mouse_event(flags.up),
                AcResolvedAction::Keyboard(mods, main) => {
                    ac_send_key_event(*main, true);
                    for m in mods.iter().rev() {
                        ac_send_key_event(*m, true);
                    }
                }
            }
        }

        if *generation_rx.borrow() != generation {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod audio_encoder_tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_path(extension: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ftools-audio-test-{nonce}.{extension}"))
    }

    #[test]
    fn wav_flac_mp3_and_ogg_outputs_are_decodable() {
        const SAMPLE_RATE: u32 = 44_100;
        const CHANNELS: u16 = 2;
        let samples: Vec<f32> = (0..SAMPLE_RATE as usize)
            .flat_map(|frame| {
                let sample = (frame as f32 * 440.0 * std::f32::consts::TAU / SAMPLE_RATE as f32).sin() * 0.25;
                [sample, sample]
            })
            .collect();

        let encoders: [(&str, &[u8], fn(&[f32], u32, u16, &Path) -> Result<(), String>); 5] = [
            ("wav", b"RIFF", encode_wav),
            ("flac", b"fLaC", encode_flac),
            ("mp3", b"\xFF", encode_mp3),
            ("ogg", b"OggS", encode_ogg_vorbis),
            ("opus", b"OggS", encode_opus),
        ];

        for (extension, signature, encode) in encoders {
            let path = test_path(extension);
            encode(&samples, SAMPLE_RATE, CHANNELS, &path)
                .unwrap_or_else(|error| panic!("{extension} encoding failed: {error}"));

            let bytes = fs::read(&path).expect("couldn't read temporary audio output");
            assert!(bytes.starts_with(signature), "{extension} output has an invalid signature");
            assert!(bytes.len() > signature.len(), "{extension} output contains no audio data");

            let (decoded, rate, channels) = decode_to_pcm(&path)
                .unwrap_or_else(|error| panic!("{extension} decoding failed: {error}"));
            if extension == "opus" {
                assert_eq!(rate, 48_000, "opus should always decode back at 48kHz");
            } else {
                assert_eq!(rate, SAMPLE_RATE, "{extension} sample rate changed");
            }
            assert_eq!(channels, CHANNELS, "{extension} channel count changed");
            assert!(!decoded.is_empty(), "{extension} output contains no audio samples");

            fs::remove_file(path).expect("couldn't remove temporary audio output");
        }
    }
}

fn main() {
    update::remove_previous_version();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(
            tauri_plugin_prevent_default::Builder::new()
                .with_flags(
                    Flags::DEV_TOOLS
                        | Flags::FIND
                        | Flags::RELOAD
                        | Flags::PRINT
                        | Flags::CONTEXT_MENU
                )
                .platform(PlatformOptions::new().browser_accelerator_keys(true))
                .build()
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage({
            let (generation, generation_rx) = tokio::sync::watch::channel(0u64);
            AcClickerState { generation, _generation_rx: generation_rx }
        })
        .setup(|app| {
            let window = app.get_webview_window("main").expect("no main window");
            apply_window_blur(&window);

            if let Ok(hwnd_061) = window.hwnd() {
                let hwnd = windows::Win32::Foundation::HWND(hwnd_061.0 as *mut _);
                unsafe {
                    let old = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, force_active_wndproc as *const () as isize);
                    ORIGINAL_WNDPROC.store(old, Ordering::SeqCst);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            update::check_for_update,
            update::install_update,
            begin_conversion,
            cancel_conversion,
            media_info,
            existing_output_folder,
            convert_image,
            convert_audio,
            convert_video,
            start_autoclicker_loop,
            bump_autoclicker_generation
        ])
        .run(tauri::generate_context!())
        .expect("error while running fTools");
}

fn apply_window_blur(window: &tauri::WebviewWindow) {
    let _ = apply_acrylic(window, Some((0, 0, 0, 20)));

    if let Ok(hwnd_061) = window.hwnd() {
        let hwnd = windows::Win32::Foundation::HWND(hwnd_061.0 as *mut _);
        unsafe {
            let region = CreateRectRgn(0, 0, -1, -1);
            let bb = DWM_BLURBEHIND {
                dwFlags: DWM_BB_ENABLE | DWM_BB_BLURREGION | DWM_BB_TRANSITIONONMAXIMIZED,
                fEnable: true.into(),
                hRgnBlur: region,
                fTransitionOnMaximized: true.into(),
            };

            let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
            let _ = DeleteObject(region.into());
        }
    }
}

use std::sync::atomic::{AtomicIsize, Ordering};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, SetWindowLongPtrW, GWLP_WNDPROC, WM_NCACTIVATE, WNDPROC,
};

static ORIGINAL_WNDPROC: AtomicIsize = AtomicIsize::new(0);

unsafe extern "system" fn force_active_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let orig_raw = ORIGINAL_WNDPROC.load(Ordering::SeqCst);
    let orig: WNDPROC = std::mem::transmute(orig_raw);

    if msg == WM_NCACTIVATE {
        return CallWindowProcW(orig, hwnd, msg, WPARAM(1), LPARAM(-1));
    }

    CallWindowProcW(orig, hwnd, msg, wparam, lparam)
}