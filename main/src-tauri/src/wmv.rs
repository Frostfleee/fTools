use std::path::Path;

use windows::core::{Interface, HSTRING};
use windows::Win32::Media::MediaFoundation::{
    IMFAttributes, IMFMediaType, IMFSample, IMFSinkWriter, IMFSourceReader, MFAudioFormat_Float, MFAudioFormat_PCM,
    MFAudioFormat_WMAudioV8, MFCreateAttributes, MFCreateMediaType, MFCreateMemoryBuffer, MFCreateSample,
    MFCreateSinkWriterFromURL, MFCreateSourceReaderFromURL, MFMediaType_Audio, MFMediaType_Video, MFShutdown, MFStartup,
    MFTranscodeGetAudioOutputAvailableTypes, MFVideoFormat_I420, MFVideoFormat_WMV3, MFVideoInterlace_Progressive,
    MFSTARTUP_FULL, MFT_ENUM_FLAG_ALL, MFT_ENUM_FLAG_FIELDOFUSE, MF_MT_ALL_SAMPLES_INDEPENDENT,
    MF_MT_AUDIO_AVG_BYTES_PER_SECOND, MF_MT_AUDIO_BITS_PER_SAMPLE, MF_MT_AUDIO_BLOCK_ALIGNMENT, MF_MT_AUDIO_NUM_CHANNELS,
    MF_MT_AUDIO_SAMPLES_PER_SECOND, MF_MT_AVG_BITRATE, MF_MT_DEFAULT_STRIDE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE,
    MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE, MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SUBTYPE, MF_SINK_WRITER_DISABLE_THROTTLING,
    MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED, MF_SOURCE_READERF_ENDOFSTREAM, MF_SOURCE_READERF_ERROR,
    MF_SOURCE_READER_ALL_STREAMS, MF_SOURCE_READER_ANY_STREAM, MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING,
    MF_SOURCE_READER_FIRST_AUDIO_STREAM, MF_SOURCE_READER_FIRST_VIDEO_STREAM, MF_VERSION,
};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

pub struct Session {
    com: bool,
}

impl Session {
    pub fn start() -> Result<Session, String> {
        unsafe {
            let com = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();
            if let Err(e) = MFStartup(MF_VERSION, MFSTARTUP_FULL) {
                if com {
                    CoUninitialize();
                }
                return Err(format!("Couldn't start Media Foundation: {e}"));
            }
            Ok(Session { com })
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            let _ = MFShutdown();
            if self.com {
                CoUninitialize();
            }
        }
    }
}

const ASF_HEADER: [u8; 16] = [0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C];
const ASF_FILE_PROPERTIES: [u8; 16] = [0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11, 0x8E, 0xE4, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65];

pub fn duration_ms(head: &[u8]) -> Option<u64> {
    if head.len() < 30 || head[0..16] != ASF_HEADER {
        return None;
    }
    let mut p = 30usize;
    while p + 24 <= head.len() {
        let size = u64::from_le_bytes(head[p + 16..p + 24].try_into().ok()?) as usize;
        if head[p..p + 16] == ASF_FILE_PROPERTIES {
            let field = |at: usize| head.get(p + at..p + at + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap()));
            let play = field(64)?;
            let preroll = field(80)?;
            let ms = (play / 10_000).saturating_sub(preroll);
            return (ms > 0).then_some(ms);
        }
        if size < 24 {
            return None;
        }
        p += size;
    }
    None
}

pub enum Media<'a> {
    Video { width: usize, height: usize, fps: f32, i420: &'a [u8], pts_ns: i64 },
    Audio { pcm: &'a [f32], rate: u32, channels: u16, pts_ns: i64 },
}

fn attributes(count: u32) -> Result<IMFAttributes, String> {
    let mut attrs: Option<IMFAttributes> = None;
    unsafe { MFCreateAttributes(&mut attrs, count) }.map_err(|e| e.to_string())?;
    attrs.ok_or_else(|| "Media Foundation didn't return an attributes object.".to_string())
}

fn media_type(major: &windows::core::GUID, subtype: &windows::core::GUID) -> Result<IMFMediaType, String> {
    unsafe {
        let t = MFCreateMediaType().map_err(|e| e.to_string())?;
        t.SetGUID(&MF_MT_MAJOR_TYPE, major).map_err(|e| e.to_string())?;
        t.SetGUID(&MF_MT_SUBTYPE, subtype).map_err(|e| e.to_string())?;
        Ok(t)
    }
}

fn pair(value: u64) -> (u32, u32) {
    ((value >> 32) as u32, value as u32)
}

fn pack(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) | low as u64
}

struct VideoLayout {
    width: usize,
    height: usize,
    stride: usize,
    fps: f32,
}

fn video_layout(reader: &IMFSourceReader) -> Result<VideoLayout, String> {
    unsafe {
        let t = reader
            .GetCurrentMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32)
            .map_err(|e| format!("Couldn't read the video format: {e}"))?;
        let (width, height) = pair(t.GetUINT64(&MF_MT_FRAME_SIZE).map_err(|e| e.to_string())?);
        let stride = t.GetUINT32(&MF_MT_DEFAULT_STRIDE).map(|s| (s as i32).unsigned_abs() as usize).unwrap_or(width as usize);
        let fps = t
            .GetUINT64(&MF_MT_FRAME_RATE)
            .ok()
            .map(pair)
            .filter(|&(n, d)| n > 0 && d > 0)
            .map(|(n, d)| n as f32 / d as f32)
            .unwrap_or(30.0);
        Ok(VideoLayout { width: width as usize, height: height as usize, stride: stride.max(width as usize), fps })
    }
}

fn audio_layout(reader: &IMFSourceReader) -> Result<(u32, u16), String> {
    unsafe {
        let t = reader
            .GetCurrentMediaType(MF_SOURCE_READER_FIRST_AUDIO_STREAM.0 as u32)
            .map_err(|e| format!("Couldn't read the audio format: {e}"))?;
        let rate = t.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND).map_err(|e| e.to_string())?;
        let channels = t.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS).map_err(|e| e.to_string())? as u16;
        Ok((rate, channels.max(1)))
    }
}

fn with_sample_bytes<T>(sample: &IMFSample, f: impl FnOnce(&[u8]) -> T) -> Result<T, String> {
    unsafe {
        let buffer = sample.ConvertToContiguousBuffer().map_err(|e| e.to_string())?;
        let mut ptr: *mut u8 = std::ptr::null_mut();
        let mut len = 0u32;
        buffer.Lock(&mut ptr, None, Some(&mut len)).map_err(|e| e.to_string())?;
        let result = f(if ptr.is_null() { &[] } else { std::slice::from_raw_parts(ptr, len as usize) });
        let _ = buffer.Unlock();
        Ok(result)
    }
}

fn pack_i420(data: &[u8], layout: &VideoLayout, out: &mut Vec<u8>) -> bool {
    let (w, h, stride) = (layout.width & !1, layout.height & !1, layout.stride);
    if stride == 0 || data.len() < stride * layout.height * 3 / 2 {
        return false;
    }
    let exact = stride * layout.height + 2 * (stride / 2) * layout.height.div_ceil(2);
    let rows = if data.len() <= exact { layout.height } else { (data.len() * 2 / (3 * stride)).max(layout.height) };
    let chroma_stride = stride / 2;
    let u_start = stride * rows;
    let v_start = u_start + chroma_stride * rows.div_ceil(2);
    if v_start + chroma_stride * (h / 2) > data.len() {
        return false;
    }
    out.clear();
    for row in 0..h {
        out.extend_from_slice(&data[row * stride..row * stride + w]);
    }
    for start in [u_start, v_start] {
        for row in 0..h / 2 {
            let o = start + row * chroma_stride;
            out.extend_from_slice(&data[o..o + w / 2]);
        }
    }
    true
}

pub fn read(
    path: &Path,
    want_video: bool,
    want_audio: bool,
    mut sink: impl FnMut(Media) -> Result<(), String>,
) -> Result<(), String> {
    let url = HSTRING::from(path.to_string_lossy().as_ref());
    unsafe {
        let attrs = attributes(1)?;
        attrs.SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1).map_err(|e| e.to_string())?;
        let reader = MFCreateSourceReaderFromURL(&url, &attrs).map_err(|e| format!("Couldn't open the source file: {e}"))?;
        reader.SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS.0 as u32, false).map_err(|e| e.to_string())?;

        let video_stream = MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32;
        let audio_stream = MF_SOURCE_READER_FIRST_AUDIO_STREAM.0 as u32;
        let mut has_video = false;
        let mut has_audio = false;
        if want_video && reader.SetStreamSelection(video_stream, true).is_ok() {
            let t = media_type(&MFMediaType_Video, &MFVideoFormat_I420)?;
            reader
                .SetCurrentMediaType(video_stream, None, &t)
                .map_err(|e| format!("Windows couldn't decode this file's video: {e}"))?;
            has_video = true;
        }
        if want_audio && reader.SetStreamSelection(audio_stream, true).is_ok() {
            let t = media_type(&MFMediaType_Audio, &MFAudioFormat_Float)?;
            if reader.SetCurrentMediaType(audio_stream, None, &t).is_ok() {
                has_audio = true;
            } else {
                let _ = reader.SetStreamSelection(audio_stream, false);
            }
        }
        if !has_video && !has_audio {
            return Err("No audio or video that Windows can decode was found in this file.".to_string());
        }

        let mut video = if has_video { Some(video_layout(&reader)?) } else { None };
        let mut audio = if has_audio { Some(audio_layout(&reader)?) } else { None };
        let mut open = has_video as u32 + has_audio as u32;
        let mut frame = Vec::new();
        let mut pcm: Vec<f32> = Vec::new();

        while open > 0 {
            let mut index = 0u32;
            let mut flags = 0u32;
            let mut timestamp = 0i64;
            let mut sample: Option<IMFSample> = None;
            reader
                .ReadSample(MF_SOURCE_READER_ANY_STREAM.0 as u32, 0, Some(&mut index), Some(&mut flags), Some(&mut timestamp), Some(&mut sample))
                .map_err(|e| format!("Error while reading media data: {e}"))?;
            if flags & MF_SOURCE_READERF_ERROR.0 as u32 != 0 {
                return Err("Windows reported an error while decoding this file.".to_string());
            }
            let current = reader.GetCurrentMediaType(index).map_err(|e| e.to_string())?;
            let is_video = current.GetGUID(&MF_MT_MAJOR_TYPE).map_err(|e| e.to_string())? == MFMediaType_Video;
            if flags & MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED.0 as u32 != 0 {
                if is_video {
                    video = Some(video_layout(&reader)?);
                } else {
                    audio = Some(audio_layout(&reader)?);
                }
            }
            if let Some(sample) = sample {
                let pts_ns = timestamp * 100;
                if is_video {
                    if let Some(layout) = &video {
                        let ok = with_sample_bytes(&sample, |data| pack_i420(data, layout, &mut frame))?;
                        if ok {
                            sink(Media::Video {
                                width: layout.width & !1,
                                height: layout.height & !1,
                                fps: layout.fps,
                                i420: &frame,
                                pts_ns,
                            })?;
                        }
                    }
                } else if let Some((rate, channels)) = audio {
                    with_sample_bytes(&sample, |data| {
                        pcm.clear();
                        pcm.extend(data.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])));
                    })?;
                    sink(Media::Audio { pcm: &pcm, rate, channels, pts_ns })?;
                }
            }
            if flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 {
                open = open.saturating_sub(1);
            }
        }
    }
    Ok(())
}

pub struct VideoSpec {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub bitrate: u32,
}

pub struct AudioSpec {
    pub rate: u32,
    pub channels: u16,
    pub bitrate: u32,
}

pub struct Writer {
    writer: IMFSinkWriter,
    video: Option<(u32, usize)>,
    audio: Option<u32>,
    pub audio_rate: u32,
}

fn wma_type(spec: &AudioSpec) -> Result<IMFMediaType, String> {
    unsafe {
        let flags = (MFT_ENUM_FLAG_ALL.0 & !MFT_ENUM_FLAG_FIELDOFUSE.0) as u32;
        let types = MFTranscodeGetAudioOutputAvailableTypes(&MFAudioFormat_WMAudioV8, flags, None)
            .map_err(|e| format!("Windows has no WMA encoder available: {e}"))?;
        let count = types.GetElementCount().map_err(|e| e.to_string())?;
        let mut best: Option<(u64, IMFMediaType)> = None;
        for i in 0..count {
            let Ok(t) = types.GetElement(i).and_then(|u| u.cast::<IMFMediaType>()) else { continue };
            let rate = t.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND).unwrap_or(0);
            let channels = t.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS).unwrap_or(0);
            let bits = t.GetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE).unwrap_or(16);
            if channels != spec.channels as u32 || bits != 16 || rate == 0 {
                continue;
            }
            let bytes = t.GetUINT32(&MF_MT_AUDIO_AVG_BYTES_PER_SECOND).unwrap_or(0) as i64;
            let score = (rate as i64 - spec.rate as i64).unsigned_abs() * 1_000_000 + (bytes * 8 - spec.bitrate as i64).unsigned_abs();
            if best.as_ref().is_none_or(|(s, _)| score < *s) {
                best = Some((score, t));
            }
        }
        best.map(|(_, t)| t).ok_or_else(|| "Windows' WMA encoder doesn't support this channel layout.".to_string())
    }
}

impl Writer {
    pub fn create(path: &Path, video: Option<VideoSpec>, audio: Option<AudioSpec>) -> Result<Writer, String> {
        let url = HSTRING::from(path.to_string_lossy().as_ref());
        unsafe {
            let attrs = attributes(1)?;
            attrs.SetUINT32(&MF_SINK_WRITER_DISABLE_THROTTLING, 1).map_err(|e| e.to_string())?;
            let writer = MFCreateSinkWriterFromURL(&url, None, &attrs).map_err(|e| format!("Couldn't create the output file: {e}"))?;

            let mut video_index = None;
            if let Some(v) = &video {
                let fps_num = (v.fps * 1000.0).round().max(1.0) as u32;
                let configure = |t: &IMFMediaType| -> Result<(), String> {
                    t.SetUINT64(&MF_MT_FRAME_SIZE, pack(v.width, v.height)).map_err(|e| e.to_string())?;
                    t.SetUINT64(&MF_MT_FRAME_RATE, pack(fps_num, 1000)).map_err(|e| e.to_string())?;
                    t.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pack(1, 1)).map_err(|e| e.to_string())?;
                    t.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32).map_err(|e| e.to_string())?;
                    Ok(())
                };
                let output = media_type(&MFMediaType_Video, &MFVideoFormat_WMV3)?;
                configure(&output)?;
                output.SetUINT32(&MF_MT_AVG_BITRATE, v.bitrate).map_err(|e| e.to_string())?;
                let index = writer.AddStream(&output).map_err(|e| format!("Windows couldn't set up the WMV encoder: {e}"))?;
                let input = media_type(&MFMediaType_Video, &MFVideoFormat_I420)?;
                configure(&input)?;
                input.SetUINT32(&MF_MT_DEFAULT_STRIDE, v.width).map_err(|e| e.to_string())?;
                writer
                    .SetInputMediaType(index, &input, None)
                    .map_err(|e| format!("Windows' WMV encoder rejected this video size or frame rate: {e}"))?;
                video_index = Some((index, v.width as usize * v.height as usize * 3 / 2));
            }

            let mut audio_index = None;
            let mut audio_rate = 0;
            if let Some(a) = &audio {
                let output = wma_type(a)?;
                audio_rate = output.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND).map_err(|e| e.to_string())?;
                let index = writer.AddStream(&output).map_err(|e| format!("Windows couldn't set up the WMA encoder: {e}"))?;
                let input = media_type(&MFMediaType_Audio, &MFAudioFormat_PCM)?;
                let block = 2 * a.channels as u32;
                input.SetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE, 16).map_err(|e| e.to_string())?;
                input.SetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND, audio_rate).map_err(|e| e.to_string())?;
                input.SetUINT32(&MF_MT_AUDIO_NUM_CHANNELS, a.channels as u32).map_err(|e| e.to_string())?;
                input.SetUINT32(&MF_MT_AUDIO_BLOCK_ALIGNMENT, block).map_err(|e| e.to_string())?;
                input.SetUINT32(&MF_MT_AUDIO_AVG_BYTES_PER_SECOND, block * audio_rate).map_err(|e| e.to_string())?;
                input.SetUINT32(&MF_MT_ALL_SAMPLES_INDEPENDENT, 1).map_err(|e| e.to_string())?;
                writer
                    .SetInputMediaType(index, &input, None)
                    .map_err(|e| format!("Windows' WMA encoder rejected this audio format: {e}"))?;
                audio_index = Some(index);
            }
            writer.BeginWriting().map_err(|e| format!("Couldn't begin writing the output file: {e}"))?;
            Ok(Writer { writer, video: video_index, audio: audio_index, audio_rate })
        }
    }

    fn write(&self, index: u32, data: &[u8], pts_ns: i64, duration_ns: i64) -> Result<(), String> {
        unsafe {
            let buffer = MFCreateMemoryBuffer(data.len() as u32).map_err(|e| e.to_string())?;
            let mut ptr: *mut u8 = std::ptr::null_mut();
            buffer.Lock(&mut ptr, None, None).map_err(|e| e.to_string())?;
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
            buffer.Unlock().map_err(|e| e.to_string())?;
            buffer.SetCurrentLength(data.len() as u32).map_err(|e| e.to_string())?;
            let sample = MFCreateSample().map_err(|e| e.to_string())?;
            sample.AddBuffer(&buffer).map_err(|e| e.to_string())?;
            sample.SetSampleTime(pts_ns / 100).map_err(|e| e.to_string())?;
            sample.SetSampleDuration((duration_ns / 100).max(1)).map_err(|e| e.to_string())?;
            self.writer.WriteSample(index, &sample).map_err(|e| format!("Couldn't write media data: {e}"))
        }
    }

    pub fn video(&self, i420: &[u8], pts_ns: i64, duration_ns: i64) -> Result<(), String> {
        match self.video {
            Some((index, size)) if i420.len() >= size => self.write(index, &i420[..size], pts_ns, duration_ns),
            Some(_) => Err("A video frame didn't match the output size.".to_string()),
            None => Ok(()),
        }
    }

    pub fn audio(&self, pcm16: &[u8], pts_ns: i64, duration_ns: i64) -> Result<(), String> {
        match self.audio {
            Some(index) => self.write(index, pcm16, pts_ns, duration_ns),
            None => Ok(()),
        }
    }

    pub fn finish(self) -> Result<(), String> {
        unsafe { self.writer.Finalize() }.map_err(|e| format!("Couldn't finalize the output file: {e}"))
    }
}