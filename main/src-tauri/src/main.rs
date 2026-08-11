#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Cursor;
use std::os::windows::fs::FileTimesExt;
use std::path::{Path, PathBuf};

use image::ImageFormat;
use img_parts::ImageEXIF;
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
use tauri::{AppHandle, Emitter};

/*
   ╭─────────────────────────────╮
   │       IMAGE CONVERSION      │
   ╰─────────────────────────────╯
   Codec scope, and why: everything except heic/heif/avif goes through
   the pure-Rust `image` crate, no system dependencies. avif *encoding*
   also goes through `image` (via ravif/rav1e, still pure Rust). Reading
   heic/heif/avif, and encoding *to* heic/heif, go through libheif-rs,
   which needs the native libheif library (built via vcpkg) to be
   present and discoverable at build time.
*/

fn image_conversion_supported(ext: &str) -> bool {
    matches!(
        ext,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "ico" | "avif" | "heic" | "heif"
    )
}

// Formats we can actually decode as a *source* image.
fn image_source_supported(ext: &str) -> bool {
    matches!(
        ext,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "ico" | "avif" | "heic" | "heif"
    )
}

fn is_heif_container(ext: &str) -> bool {
    matches!(ext, "heic" | "heif" | "avif")
}

// Decodes a HEIC/HEIF/AVIF file via libheif. The same libheif API
// handles all three container types transparently, so no branching on
// ext is needed here beyond routing into this function in the first
// place.
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

// Encodes to HEIC via libheif's HEVC (x265) encoder and writes straight
// to output_path (unlike the `image` crate path, there's no in-memory
// buffer step here).
fn encode_heif(img: &image::DynamicImage, output_path: &Path) -> Result<(), String> {
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
        .set_quality(EncoderQuality::Lossy(90))
        .map_err(|e| e.to_string())?;
    context
        .encode_image(&heif_image, &mut encoder, None)
        .map_err(|e| e.to_string())?;
    context.write_to_file(output_str).map_err(|e| e.to_string())
}

// Windows icons are a directory of several sizes bundled into one file,
// not a single raster, so the OS can pick whichever fits the context
// (taskbar, Explorer tile, shortcut, etc). Each size is a fresh resize
// down from the original decode rather than progressively downscaling
// the previous size, so quality doesn't compound across entries. The ico
// crate's IconDirEntry::encode stores each frame as PNG internally,
// which is lossless raster compression, not a lossy codec like JPEG.
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

// Best-effort EXIF read. Only JPEG and PNG are handled (the formats
// img-parts supports); anything else just returns None and conversion
// proceeds without metadata rather than failing.
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

// Best-effort EXIF re-injection into the freshly encoded bytes. On any
// failure this just falls back to the un-annotated bytes instead of
// failing the whole conversion.
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

#[tauri::command]
fn convert_image(
    app: AppHandle,
    source_path: String,
    output_name: String,
    target_ext: String,
    keep_metadata: bool,
    preserve_date: bool,
    overwrite: bool,
) -> Result<String, String> {
    // Coarse, stage-based progress. Neither the `image` crate nor libheif
    // expose a byte-level progress callback, so this reports real
    // checkpoints the conversion actually reaches (read done, decode done,
    // encode+write done, fully done) rather than faking a smooth ramp.
    // A failed emit just means nobody's listening; it's not fatal.
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

    let source_bytes =
        fs::read(&source_path).map_err(|e| format!("Couldn't read the source file: {e}"))?;

    report(30);

    let decoded = if is_heif_container(&source_ext) {
        decode_heif(&source_path).map_err(|e| format!("Couldn't decode this {} file: {e}", source_ext))?
    } else {
        image::load_from_memory(&source_bytes)
            .map_err(|e| format!("This doesn't look like a valid image: {e}"))?
    };

    report(55);

    let output_dir = source_path.parent().unwrap_or_else(|| Path::new(""));
    let output_path = output_dir.join(format!("{output_name}.{target_ext}"));

    if is_heif_container(&target_ext) && target_ext != "avif" {
        // HEIC/HEIF target: libheif writes straight to disk. Metadata
        // preservation isn't wired up for this path yet.
        encode_heif(&decoded, &output_path)
            .map_err(|e| format!("Couldn't encode the image as {}: {e}", target_ext))?;
    } else if target_ext == "ico" {
        // A real Windows icon isn't one raster at one size; it's a
        // directory of several sizes bundled together so the OS can pick
        // whichever fits the context (taskbar vs. Explorer tile vs.
        // shortcut, etc). The generic single-frame path below can't
        // produce that, so ICO gets its own encoder.
        encode_multi_size_ico(&decoded, &output_path)
            .map_err(|e| format!("Couldn't encode the image as ico: {e}"))?;
    } else {
        let format = ImageFormat::from_extension(&target_ext)
            .ok_or_else(|| format!("Unknown target format \"{}\".", target_ext))?;

        let mut encoded: Vec<u8> = Vec::new();
        decoded
            .write_to(&mut Cursor::new(&mut encoded), format)
            .map_err(|e| format!("Couldn't encode the image as {}: {e}", target_ext))?;

        if keep_metadata {
            if let Some(exif) = extract_exif(&source_bytes, &source_ext) {
                encoded = inject_exif(encoded, &target_ext, exif);
            }
        }

        fs::write(&output_path, &encoded)
            .map_err(|e| format!("Couldn't write the converted file: {e}"))?;
    }

    report(85);

    if preserve_date {
        preserve_file_date(&source_path, &output_path);
    }

    if overwrite && output_path != source_path {
        let _ = fs::remove_file(&source_path);
    }

    report(100);

    Ok(output_path.to_string_lossy().to_string())
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

/*
   ╭─────────────────────────────╮
   │       AUDIO CONVERSION      │
   ╰─────────────────────────────╯
   Symphonia decodes; there's no equivalent all-in-one crate for audio the
   way the `image` crate covers images, so each output format needed its
   own encoder crate, evaluated and verified separately against each
   crate's own docs: hound (WAV, pure Rust), flacenc (FLAC, pure Rust),
   mp3lame-encoder (MP3, wraps LAME, LGPL-3.0), vorbis_rs (OGG/Vorbis,
   BSD-3-Clause, bundles its own Ogg muxing). AIFF is hand-written; it's
   WAV's cousin, no crate needed. FLAC and AAC/M4A sources both already
   decode through Symphonia's ordinary probe path with zero extra code:
   the "all" feature pulls in symphonia-bundle-flac, and separately
   symphonia-codec-aac ships its own AdtsReader (registered for the "aac"
   extension) alongside symphonia-format-isomp4 for the .m4a container,
   confirmed by checking the actual resolved dependency tree rather than
   assumed from the crate name. WMA sources can't be read at all: it's a
   proprietary codec with no open-source decoder anywhere in the Rust
   ecosystem. Writing *to* WMA doesn't have that problem, see below.

   Opus: the earlier libopusenc attempt failed to link on Windows/MSVC
   with unresolved opus_* symbols, a documented, recurring issue with
   that exact library/toolchain combo. Rather than fight the linker
   again, this now goes through opus-rs, a from-scratch pure-Rust port of
   the reference libopus (RFC 6716) with zero C dependencies, so there is
   nothing to link at all. opus-rs only speaks raw Opus frames in and
   out; the surrounding .opus file is an Ogg container (RFC 7845), which
   nothing in the dependency tree produces or parses on its own, so the
   `ogg` crate (RustAudio, also pure Rust) handles page framing, and the
   OpusHead/OpusTags header packets are hand-built here the same way
   AIFF's header is: it's a handful of fixed fields, not worth a crate.
   Opus only encodes at 8/12/16/24/48 kHz, unlike every other format
   here, so sources at other rates (44.1 kHz is the common case) are
   resampled first via rubato (pure Rust, sinc-based). The pre-skip field
   in OpusHead is set to 0 rather than measuring opus-rs's actual
   algorithmic delay: this is spec-legal and just means a few
   milliseconds of encoder priming may be audible as near-silence at the
   very start of playback, not a correctness bug.

   AAC/M4A and raw AAC (.aac/ADTS): no mature pure-Rust encoder exists for
   either, and wrapping fdk-aac drags in a license that complicates
   redistribution given AAC's own patent licensing besides. Since this
   app is Windows-only already (see the rest of this file), the pragmatic
   option is the one Windows ships for free: Media Foundation's AAC
   encoder MFT, driven through IMFSinkWriter via the `windows` crate.
   Confirmed working on a real Windows machine, not just compiling. Raw
   AAC reuses the exact same encoder as M4A; the only difference is
   telling IMFSinkWriter to produce the ADTS container
   (MFTranscodeContainerType_ADTS) instead of letting it infer MP4 from
   the .m4a extension, and setting the AAC payload type attribute to ADTS
   instead of raw.

   WMA output was attempted the same way (encode_via_media_foundation,
   still present, deliberately unused, see the comment on encode_wma) but
   is not wired into convert_audio. Every sample writes successfully; the
   ASF muxer's own Finalize() step fails with a bare E_UNEXPECTED that
   several real, verified fixes (disabling the sink writer's default
   throttling, removing compounding timestamp rounding error, negotiating
   bitrate instead of pre-declaring it) didn't resolve, confirmed on a
   real Windows machine each time, not guessed. The remaining lead
   (discovering the encoder's actual supported output types via
   MFTEnumEx + IMFTransform::GetOutputAvailableType, rather than
   declaring one and hoping) requires manually managing a Windows-
   allocated array of raw COM interface pointers, which is a documented
   source of memory bugs even for experienced windows-rs users. Given
   everything else in this file fails cleanly (a compile error or a clean
   HRESULT) rather than a crash, that tradeoff wasn't worth taking on
   blind, so WMA output is parked rather than shipped half-working.

   Decoding always produces one big interleaved f32 buffer rather than
   streaming straight into an encoder, matching how convert_image already
   loads the whole source into memory: it keeps every encoder's code
   simple and self-contained instead of juggling streaming decode+encode
   across multiple different library paradigms at once. Opus is the one
   exception on the decode side too: Symphonia has no native Opus decoder
   (confirmed against the published crate registry, not just assumed), so
   reading a .opus source bypasses Symphonia entirely and reuses the same
   `ogg` + opus-rs pipeline as the encoder, just running backwards.
*/

// Works around a real bug in symphonia-bundle-flac 0.5.5's frame
// validator (strict_frame_header_check in its parser.rs): it treats
// "STREAMINFO's min and max block length are equal" as synonymous with
// "the stream uses fixed (frame-numbered) blocking," and rejects every
// single frame as non-monotonic the instant those two lengths differ.
// They differ on essentially any real FLAC file, since a shorter final
// block (whenever the sample count isn't an exact multiple of the block
// size) is completely normal and spec-legal. Confirmed by hand against
// the FLAC spec: the frame data itself (including its CRC-8) is valid:
// this is Symphonia misreading STREAMINFO, not a malformed file.
// STREAMINFO's min/max block length fields are purely informational
// (actual per-frame block size comes from each frame's own header, which
// this doesn't touch), so overwriting min to match max here is a safe,
// narrowly-targeted fix that doesn't affect the real audio data at all,
// applied to whatever bytes get handed to Symphonia's probe regardless
// of whether the file came from encode_flac or somewhere else entirely.
fn patch_flac_streaminfo_for_symphonia_bug(bytes: &mut [u8]) {
    if bytes.len() < 42 || &bytes[0..4] != b"fLaC" {
        return; // not a native FLAC stream; let Symphonia report its own error
    }
    let block_type = bytes[4] & 0x7f;
    let block_len = ((bytes[5] as usize) << 16) | ((bytes[6] as usize) << 8) | bytes[7] as usize;
    if block_type != 0 || block_len != 34 {
        return; // STREAMINFO isn't the first metadata block or isn't its usual size; leave it alone
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

    if source_ext == "wma" {
        return Err(
            "WMA is a proprietary codec with no open-source decoder available, so it can't be read yet."
                .to_string(),
        );
    }

    if source_ext == "opus" {
        return decode_opus(source_path);
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

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| "Couldn't find a supported audio track in this file.".to_string())?;

    let dec_opts: DecoderOptions = Default::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .map_err(|e| format!("Couldn't create a decoder for this file: {e}"))?;
    let track_id = track.id;

    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    let mut all_samples: Vec<f32> = Vec::new();
    let mut sample_rate: u32 = 0;
    let mut channels: u16 = 0;

    loop {
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

// mp3lame-encoder wants separate left/right i16 buffers rather than
// interleaved samples. Mono sources are upmixed to dual-mono rather than
// needing a separate mono code path.
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

// AIFF is WAV's IFF-based, big-endian cousin; simple enough to write by
// hand rather than pull in a dedicated crate. The one tricky part is the
// sample rate field, stored as an 80-bit SANE/IEEE-extended float rather
// than a plain integer; standard sample rates (44100, 48000, ...) are
// small positive integers that convert exactly, no precision loss.
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

fn encode_mp3(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    use mp3lame_encoder::{Builder, DualPcm, FlushNoGap};

    let mut builder = Builder::new().ok_or_else(|| "Couldn't create the MP3 encoder.".to_string())?;
    builder.set_num_channels(2).map_err(|e| format!("{:?}", e))?;
    builder.set_sample_rate(sample_rate).map_err(|e| format!("{:?}", e))?;
    builder
        .set_brate(mp3lame_encoder::Bitrate::Kbps192)
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

    fs::write(output_path, &out_buffer).map_err(|e| e.to_string())
}

fn encode_ogg_vorbis(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    use std::num::{NonZeroU32, NonZeroU8};
    use vorbis_rs::VorbisEncoderBuilder;

    let sr = NonZeroU32::new(sample_rate).ok_or_else(|| "Invalid sample rate.".to_string())?;
    let ch = NonZeroU8::new(channels as u8).ok_or_else(|| "Invalid channel count.".to_string())?;

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut builder = VorbisEncoderBuilder::new(sr, ch, file)
        .map_err(|e| format!("Vorbis setup error: {e}"))?;
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

// Shared by encode_opus and encode_aac_m4a: both formats only accept audio
// at specific fixed sample rates, unlike WAV/FLAC/MP3/Vorbis which take
// whatever the source already is. rubato's FftFixedInOut runs in fixed
// chunks, so the last chunk is zero-padded up to the required size; that
// only ever adds a few milliseconds of trailing silence, not a correctness
// problem.
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

// Interleaved-in, resampled-and-reinterleaved-out. A thin wrapper around
// resample_planar since both PCM producers (encode_opus, encode_aac_m4a)
// work with interleaved buffers like the rest of this file, not planar.
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

// The two Ogg Opus header packets (RFC 7845 §5.1/§5.2). Every Ogg Opus
// file starts with exactly these two, each alone on its own page, before
// any audio data. input_sample_rate is informational only (players use it
// as a hint; it doesn't have to match the rate actually encoded below) and
// pre_skip is left at 0 rather than measured from opus-rs's own
// algorithmic delay, see the comment at the top of this section.
fn build_opus_head(channels: u8, input_sample_rate: u32) -> Vec<u8> {
    let mut head = Vec::with_capacity(19);
    head.extend_from_slice(b"OpusHead");
    head.push(1); // version
    head.push(channels);
    head.extend_from_slice(&0u16.to_le_bytes()); // pre-skip
    head.extend_from_slice(&input_sample_rate.to_le_bytes());
    head.extend_from_slice(&0i16.to_le_bytes()); // output gain
    head.push(0); // channel mapping family 0: plain mono/stereo
    head
}

fn build_opus_tags() -> Vec<u8> {
    let mut tags = Vec::new();
    tags.extend_from_slice(b"OpusTags");
    let vendor = b"fTools";
    tags.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    tags.extend_from_slice(vendor);
    tags.extend_from_slice(&0u32.to_le_bytes()); // 0 user comments
    tags
}

fn encode_opus(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
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
    encoder.bitrate_bps = 128_000;

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut writer = PacketWriter::new(file);
    let serial: u32 = 0x66_54_6F_6C; // arbitrary but stable per-file Ogg stream serial

    writer
        .write_packet(build_opus_head(channels as u8, sample_rate), serial, PacketWriteEndInfo::EndPage, 0)
        .map_err(|e| format!("Couldn't write the Opus header: {e}"))?;
    writer
        .write_packet(build_opus_tags(), serial, PacketWriteEndInfo::EndPage, 0)
        .map_err(|e| format!("Couldn't write the Opus comment header: {e}"))?;

    let frame_size = (opus_rate / 50) as usize; // 20ms frames, a standard Opus choice
    let frame_len = frame_size * channels as usize;
    let mut encode_buf = vec![0u8; 4000]; // a single Opus frame maxes out at 1275 bytes
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

// Reads an Ogg Opus source by hand rather than through Symphonia (see the
// comment at the top of this section for why). Always decodes at 48 kHz:
// per the Opus spec the decoder's output rate is independent of whatever
// rate was actually encoded, and 48 kHz is the highest-fidelity, always-
// valid choice regardless of the source's internal bandwidth.
fn decode_opus(source_path: &Path) -> Result<(Vec<f32>, u32, u16), String> {
    use ogg::reading::PacketReader;
    use opus_rs::OpusDecoder;

    let file = fs::File::open(source_path).map_err(|e| format!("Couldn't open the source file: {e}"))?;
    let mut reader = PacketReader::new(file);

    let head = reader
        .read_packet()
        .map_err(|e| format!("Couldn't read this Ogg file: {e}"))?
        .ok_or_else(|| "This Opus file is empty.".to_string())?;
    if head.data.len() < 19 || &head.data[0..8] != b"OpusHead" {
        return Err("This doesn't look like a valid Opus file (missing OpusHead).".to_string());
    }
    let channels = head.data[9] as u16;
    if channels == 0 || channels > 2 {
        return Err("Only mono or stereo Opus files are supported.".to_string());
    }

    reader
        .read_packet()
        .map_err(|e| format!("Couldn't read this Ogg file: {e}"))?
        .ok_or_else(|| "This Opus file is missing its comment header.".to_string())?;

    let mut decoder = OpusDecoder::new(48000, channels as usize)
        .map_err(|e| format!("Couldn't create the Opus decoder: {e}"))?;

    const MAX_FRAME_SAMPLES: usize = 5760; // 120ms @ 48kHz, the largest possible Opus frame
    let mut pcm_buf = vec![0.0f32; MAX_FRAME_SAMPLES * channels as usize];
    let mut all_samples: Vec<f32> = Vec::new();

    while let Some(packet) = reader
        .read_packet()
        .map_err(|e| format!("Error while reading Opus audio data: {e}"))?
    {
        if packet.data.is_empty() {
            continue;
        }
        let decoded = decoder
            .decode(&packet.data, MAX_FRAME_SAMPLES, &mut pcm_buf)
            .map_err(|e| format!("Opus decode error: {e}"))?;
        all_samples.extend_from_slice(&pcm_buf[..decoded * channels as usize]);
    }

    if all_samples.is_empty() {
        return Err("No audio data could be decoded from this Opus file.".to_string());
    }

    Ok((all_samples, 48000, channels))
}

// Sample rates the Windows Media Foundation AAC encoder MFT accepts, per
// Microsoft's documented AAC Encoder input requirements. WMA's encoder MFT
// is documented against the same common set, so it's reused rather than
// duplicated below.
const AAC_SUPPORTED_RATES: [u32; 9] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000];

fn nearest_supported_rate(rate: u32, table: &[u32]) -> u32 {
    *table
        .iter()
        .min_by_key(|&&candidate| (candidate as i64 - rate as i64).abs())
        .expect("rate table is never empty")
}

// What differs between the three Media Foundation Sink Writer encoders
// below; everything else about driving IMFSinkWriter is identical.
struct MfEncoderProfile {
    subtype: windows::core::GUID,
    valid_rates: &'static [u32],
    bitrate_stereo: u32,
    bitrate_mono: u32,
    // Raw ADTS AAC (.aac) has no container of its own that Media
    // Foundation can pick from the file extension the way it picks MP4
    // for .m4a or ASF for .wma, so it needs an explicit payload-type
    // attribute and an explicit transcode container type. M4A and WMA
    // both leave this false and let MFCreateSinkWriterFromURL resolve
    // the container from the output extension on its own.
    adts: bool,
    // WMA's encoder MFT only accepts a small table of fixed internal
    // bitrate/rate/channel profiles, unlike AAC's, which accepts a
    // freely declared combination. Pre-declaring an exact AVG_BITRATE on
    // the output type that doesn't happen to match one of those profiles
    // is what produced "the data specified... is invalid, inconsistent,
    // or not supported" on SetInputMediaType. Instead, the bitrate is
    // passed as a hint via SetInputMediaType's encoding-parameters
    // argument, letting the sink writer pick a real matching profile.
    negotiate_bitrate: bool,
}

// AAC/M4A and raw AAC/ADTS go through the Media Foundation AAC encoder
// MFT via IMFSinkWriter. Confirmed working on a real Windows machine.
// WMA was also built on this same helper but isn't wired into
// convert_audio; see the comment on encode_wma below for why.
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

    let block_align: u32 = 2 * channels as u32; // 16-bit PCM, the only input format these MFTs accept
    let pcm_bytes_per_sec = target_rate * block_align;
    let avg_bitrate = if channels >= 2 { profile.bitrate_stereo } else { profile.bitrate_mono };
    let pcm_i16: Vec<u8> = interleaved.iter().flat_map(|&s| to_i16(s).to_le_bytes()).collect();
    let url = HSTRING::from(output_path.to_string_lossy().as_ref());

    // Run the whole COM/Media Foundation lifecycle on a dedicated, fresh
    // thread rather than whatever thread Tauri happens to run this command
    // on. The WMA encoder MFT in particular is old enough (it's a
    // holdover from the pre-Media-Foundation Windows Media SDK era) that
    // it can be sensitive to the calling thread's COM apartment state,
    // and there's no way to know what state Tauri's own thread pool
    // already left that thread in. A brand new thread guarantees a clean
    // slate for CoInitializeEx to actually establish STA on, rather than
    // possibly finding the thread already in MTA from something else and
    // silently failing (the previous version ignored that failure
    // entirely via `let _ =`, which is exactly the kind of thing that
    // surfaces later as an opaque "catastrophic failure" instead of a
    // clear error here).
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
                    output_type.SetUINT32(&MF_MT_AAC_PAYLOAD_TYPE, 1).map_err(|e| e.to_string())?; // 1 = ADTS
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
                // WriteSample blocks the calling thread to rate-limit
                // incoming data by default, on the assumption it's being
                // fed in real time; this is a one-shot batch write, not a
                // live capture, so that throttling is both unnecessary and
                // a documented source of odd failures in exactly this
                // usage pattern.
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

                let chunk_frames = (target_rate / 50).max(1) as usize; // 20ms chunks
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

                    // Computed fresh from the exact cumulative frame count on
                    // every chunk, rather than repeatedly adding a rounded
                    // per-chunk duration, since 10_000_000 / target_rate
                    // doesn't divide evenly for common rates (44100 in
                    // particular) and the rounding error would otherwise
                    // compound chunk over chunk across the whole file.
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

fn encode_m4a(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    use windows::Win32::Media::MediaFoundation::MFAudioFormat_AAC;
    encode_via_media_foundation(
        samples,
        sample_rate,
        channels,
        output_path,
        MfEncoderProfile {
            subtype: MFAudioFormat_AAC,
            valid_rates: &AAC_SUPPORTED_RATES,
            bitrate_stereo: 192_000,
            bitrate_mono: 96_000,
            adts: false,
            negotiate_bitrate: false,
        },
    )
}

// Same encoder MFT as encode_m4a; the only difference is that the output
// is bare ADTS-framed AAC (a .aac elementary stream) instead of wrapped in
// an MP4 container. This is also what makes it readable back through
// decode_to_pcm's normal Symphonia path: Symphonia's AAC codec crate
// ships its own AdtsReader registered for the "aac" extension.
fn encode_aac(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    use windows::Win32::Media::MediaFoundation::MFAudioFormat_AAC;
    encode_via_media_foundation(
        samples,
        sample_rate,
        channels,
        output_path,
        MfEncoderProfile {
            subtype: MFAudioFormat_AAC,
            valid_rates: &AAC_SUPPORTED_RATES,
            bitrate_stereo: 192_000,
            bitrate_mono: 96_000,
            adts: true,
            negotiate_bitrate: false,
        },
    )
}

// PARKED, not wired into convert_audio: WriteSample succeeds for every
// chunk (confirmed via per-chunk error reporting that never fired), but
// IMFSinkWriter::Finalize() fails with a bare E_UNEXPECTED ("catastrophic
// failure") while writing the ASF file's final header. Tried and ruled
// out: throttling (MF_SINK_WRITER_DISABLE_THROTTLING), timestamp rounding
// drift, letting the sink negotiate bitrate via SetInputMediaType's
// encoding-parameters instead of pre-declaring it, and our own code
// holding a conflicting file handle at finalize time (it doesn't).
//
// The remaining real lead is that the WMA encoder MFT is old and
// particular enough that it may need its actual supported output types
// discovered via MFTEnumEx + IMFTransform::GetOutputAvailableType rather
// than a type declared and hoped for, the same way M4A/AAC/Opus all
// work. That path involves manually managing a Windows-allocated array of
// raw COM interface pointers, which is a documented source of memory
// bugs even for experienced windows-rs users (see the array-cleanup
// confusion in microsoft/windows-rs#1685), and unlike everything else in
// this file, a mistake there risks a crash or leak instead of a clean
// compile error or a clean HRESULT. Deliberately not attempted blind.
// This function and its rate table are left in place, unused, for
// whoever picks this back up rather than deleted outright.
#[allow(dead_code)]
const WMA_SUPPORTED_RATES: [u32; 7] = [8000, 11025, 16000, 22050, 32000, 44100, 48000];

#[allow(dead_code)]
fn encode_wma(samples: &[f32], sample_rate: u32, channels: u16, output_path: &Path) -> Result<(), String> {
    use windows::Win32::Media::MediaFoundation::MFAudioFormat_WMAudioV8;
    encode_via_media_foundation(
        samples,
        sample_rate,
        channels,
        output_path,
        MfEncoderProfile {
            subtype: MFAudioFormat_WMAudioV8,
            valid_rates: &WMA_SUPPORTED_RATES,
            bitrate_stereo: 128_000,
            bitrate_mono: 64_000,
            adts: false,
            negotiate_bitrate: true,
        },
    )
}

#[tauri::command]
fn convert_audio(
    app: AppHandle,
    source_path: String,
    output_name: String,
    target_ext: String,
    preserve_date: bool,
    overwrite: bool,
) -> Result<String, String> {
    let report = |percent: u8| {
        let _ = app.emit("conversion-progress", percent);
    };

    let source_path = PathBuf::from(source_path);
    let target_ext = target_ext.to_lowercase();

    let supported = matches!(
        target_ext.as_str(),
        "wav" | "aiff" | "flac" | "mp3" | "ogg" | "opus" | "m4a" | "aac"
    );
    if !supported {
        return Err(format!(
            "\"{}\" isn't wired up for audio conversion yet.",
            target_ext
        ));
    }

    report(5);

    let (samples, sample_rate, channels) = decode_to_pcm(&source_path)?;

    report(50);

    let output_dir = source_path.parent().unwrap_or_else(|| Path::new(""));
    let output_path = output_dir.join(format!("{output_name}.{target_ext}"));

    match target_ext.as_str() {
        "wav" => encode_wav(&samples, sample_rate, channels, &output_path)?,
        "aiff" => encode_aiff(&samples, sample_rate, channels, &output_path)?,
        "flac" => encode_flac(&samples, sample_rate, channels, &output_path)?,
        "mp3" => encode_mp3(&samples, sample_rate, channels, &output_path)?,
        "ogg" => encode_ogg_vorbis(&samples, sample_rate, channels, &output_path)?,
        "opus" => encode_opus(&samples, sample_rate, channels, &output_path)?,
        "m4a" => encode_m4a(&samples, sample_rate, channels, &output_path)?,
        "aac" => encode_aac(&samples, sample_rate, channels, &output_path)?,
        _ => unreachable!(),
    }

    report(85);

    if preserve_date {
        preserve_file_date(&source_path, &output_path);
    }

    if overwrite && output_path != source_path {
        let _ = fs::remove_file(&source_path);
    }

    report(100);

    Ok(output_path.to_string_lossy().to_string())
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

            // Opus always resamples to a fixed 48kHz internally (see
            // encode_opus), so unlike the others its round trip can't be
            // expected to come back at the original 44.1kHz. FLAC's
            // decode round trip only works because decode_to_pcm patches
            // around a real Symphonia bug on the way in; see
            // patch_flac_streaminfo_for_symphonia_bug for why.
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
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert_image, convert_audio])
        .run(tauri::generate_context!())
        .expect("error while running fTools");
}