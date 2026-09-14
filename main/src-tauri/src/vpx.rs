#![allow(dead_code)]
//! Direct FFI wrapper around libvpx via the `vpx_sys` crate (env-libvpx-sys,
//! renamed in Cargo.toml). We do NOT use the `vpx-rs` high-level crate:
//! its bindgen-generated `vpx_codec_enc_cfg`/`vpx_codec_dec_cfg` come out
//! opaque in this build (a libvpx/bindgen quirk - several libvpx headers,
//! e.g. warnings.h, forward-declare `struct vpx_codec_enc_cfg;` before
//! vpx_encoder.h defines it, and bindgen keeps the incomplete version).
//! Everything else (vpx_image_t, vpx_codec_ctx_t, the encode/decode
//! functions) generates correctly, so we only hand-write these two structs,
//! transcribed field-for-field from the ACTUAL installed
//! vcpkg\installed\x64-windows-static\include\vpx\{vpx_encoder.h,vpx_decoder.h}
//! (not from memory, not from a guessed version), and call the real
//! generated functions for everything else.
//!
//! Safety note: RealEncCfg/RealDecCfg below must stay byte-for-byte
//! identical to the C structs. If you ever update the vendored libvpx
//! version, re-diff these against the new header before assuming they still
//! match - a silent mismatch here is memory corruption, not a compile error.

use std::ffi::c_void;
use std::os::raw::{c_int, c_long, c_uint, c_ulong};
use std::ptr;

use vpx_sys as sys;

const VPX_SS_MAX_LAYERS: usize = 5;
const VPX_TS_MAX_LAYERS: usize = 5;
const VPX_TS_MAX_PERIODICITY: usize = 16;
const VPX_MAX_LAYERS: usize = 12;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VpxRational {
    pub num: i32,
    pub den: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VpxFixedBuf {
    pub buf: *mut c_void,
    pub sz: usize,
}
impl Default for VpxFixedBuf {
    fn default() -> Self {
        Self { buf: ptr::null_mut(), sz: 0 }
    }
}

/// Bit-for-bit layout match of the real `vpx_codec_enc_cfg`. See module docs.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RealEncCfg {
    pub g_usage: u32,
    pub g_threads: u32,
    pub g_profile: u32,
    pub g_w: u32,
    pub g_h: u32,
    pub g_bit_depth: i32,
    pub g_input_bit_depth: u32,
    pub g_timebase: VpxRational,
    pub g_error_resilient: u32,
    pub g_pass: i32,
    pub g_lag_in_frames: u32,
    pub rc_dropframe_thresh: u32,
    pub rc_resize_allowed: u32,
    pub rc_scaled_width: u32,
    pub rc_scaled_height: u32,
    pub rc_resize_up_thresh: u32,
    pub rc_resize_down_thresh: u32,
    pub rc_end_usage: i32,
    pub rc_twopass_stats_in: VpxFixedBuf,
    pub rc_firstpass_mb_stats_in: VpxFixedBuf,
    pub rc_target_bitrate: u32,
    pub rc_min_quantizer: u32,
    pub rc_max_quantizer: u32,
    pub rc_undershoot_pct: u32,
    pub rc_overshoot_pct: u32,
    pub rc_buf_sz: u32,
    pub rc_buf_initial_sz: u32,
    pub rc_buf_optimal_sz: u32,
    pub rc_2pass_vbr_bias_pct: u32,
    pub rc_2pass_vbr_minsection_pct: u32,
    pub rc_2pass_vbr_maxsection_pct: u32,
    pub rc_2pass_vbr_corpus_complexity: u32,
    pub kf_mode: i32,
    pub kf_min_dist: u32,
    pub kf_max_dist: u32,
    pub ss_number_layers: u32,
    pub ss_enable_auto_alt_ref: [i32; VPX_SS_MAX_LAYERS],
    pub ss_target_bitrate: [u32; VPX_SS_MAX_LAYERS],
    pub ts_number_layers: u32,
    pub ts_target_bitrate: [u32; VPX_TS_MAX_LAYERS],
    pub ts_rate_decimator: [u32; VPX_TS_MAX_LAYERS],
    pub ts_periodicity: u32,
    pub ts_layer_id: [u32; VPX_TS_MAX_PERIODICITY],
    pub layer_target_bitrate: [u32; VPX_MAX_LAYERS],
    pub temporal_layering_mode: i32,
    pub use_vizier_rc_params: i32,
    pub active_wq_factor: VpxRational,
    pub err_per_mb_factor: VpxRational,
    pub sr_default_decay_limit: VpxRational,
    pub sr_diff_factor: VpxRational,
    pub kf_err_per_mb_factor: VpxRational,
    pub kf_frame_min_boost_factor: VpxRational,
    pub kf_frame_max_boost_first_factor: VpxRational,
    pub kf_frame_max_boost_subs_factor: VpxRational,
    pub kf_max_total_boost_factor: VpxRational,
    pub gf_max_total_boost_factor: VpxRational,
    pub gf_frame_max_boost_factor: VpxRational,
    pub zm_factor: VpxRational,
    pub rd_mult_inter_qp_fac: VpxRational,
    pub rd_mult_arf_qp_fac: VpxRational,
    pub rd_mult_key_qp_fac: VpxRational,
}

/// Bit-for-bit layout match of the real `vpx_codec_dec_cfg`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RealDecCfg {
    pub threads: u32,
    pub w: u32,
    pub h: u32,
}

// enum vpx_rc_mode values (vpx_encoder.h)
const VPX_CBR: i32 = 1;
// enum vpx_kf_mode values
const VPX_KF_AUTO: i32 = 1;
// enum vpx_enc_pass
const VPX_RC_ONE_PASS: i32 = 0;

pub struct EncodedFrame {
    pub data: Vec<u8>,
    pub pts: i64,
    pub keyframe: bool,
}

pub struct Vp9Encoder {
    ctx: sys::vpx_codec_ctx_t,
    width: u32,
    height: u32,
}

impl Vp9Encoder {
    pub fn new(width: u32, height: u32, fps_num: u32, fps_den: u32, bitrate_kbps: u32) -> Result<Self, String> {
        if width == 0 || height == 0 || width % 2 != 0 || height % 2 != 0 {
            return Err("Video dimensions must be non-zero and even for VP9 encoding.".to_string());
        }

        unsafe {
            let iface = sys::vpx_codec_vp9_cx();
            let mut cfg = RealEncCfg::default();
            let err = sys::vpx_codec_enc_config_default(iface, &mut cfg as *mut RealEncCfg as *mut sys::vpx_codec_enc_cfg_t, 0);
            if err != sys::vpx_codec_err_t::VPX_CODEC_OK {
                return Err(format!("Couldn't get default VP9 encoder config (libvpx error {err:?}).").to_string());
            }

            cfg.g_w = width;
            cfg.g_h = height;
            cfg.g_timebase = VpxRational { num: fps_den.max(1) as i32, den: fps_num.max(1) as i32 };
            cfg.rc_target_bitrate = bitrate_kbps;
            cfg.rc_end_usage = VPX_CBR;
            cfg.g_pass = VPX_RC_ONE_PASS;
            cfg.kf_mode = VPX_KF_AUTO;
            cfg.g_lag_in_frames = 0;

            let mut ctx: sys::vpx_codec_ctx_t = std::mem::zeroed();
            let err = sys::vpx_codec_enc_init_ver(
                &mut ctx,
                iface,
                &cfg as *const RealEncCfg as *const sys::vpx_codec_enc_cfg_t,
                0,
                sys::VPX_ENCODER_ABI_VERSION as c_int,
            );
            if err != sys::vpx_codec_err_t::VPX_CODEC_OK {
                return Err(format!("Couldn't initialize the VP9 encoder (libvpx error {err:?}).").to_string());
            }

            Ok(Self { ctx, width, height })
        }
    }

    /// `i420` must be exactly `width * height * 3 / 2` bytes, tightly packed
    /// (Y plane, then U, then V, no row padding). `frame_number` is a simple
    /// 0-based running count.
    pub fn encode_frame(&mut self, frame_number: i64, i420: &[u8]) -> Result<Vec<EncodedFrame>, String> {
        let expected_len = (self.width as usize * self.height as usize * 3) / 2;
        if i420.len() != expected_len {
            return Err(format!(
                "Frame buffer is {} bytes, expected {expected_len} for a {}x{} I420 frame.",
                i420.len(), self.width, self.height
            ));
        }

        unsafe {
            let mut img: sys::vpx_image_t = std::mem::zeroed();
            let got = sys::vpx_img_wrap(
                &mut img,
                sys::vpx_img_fmt::VPX_IMG_FMT_I420,
                self.width,
                self.height,
                1,
                i420.as_ptr() as *mut u8,
            );
            if got.is_null() {
                return Err("Couldn't wrap this frame's bytes as a VP9 input image.".to_string());
            }

            let err = sys::vpx_codec_encode(&mut self.ctx, &img, frame_number, 1, 0, sys::VPX_DL_REALTIME as c_ulong);
            if err != sys::vpx_codec_err_t::VPX_CODEC_OK {
                return Err(format!("VP9 encode error on frame {frame_number} (libvpx error {err:?}).").to_string());
            }

            let mut out = Vec::new();
            let mut iter: sys::vpx_codec_iter_t = ptr::null();
            loop {
                let pkt = sys::vpx_codec_get_cx_data(&mut self.ctx, &mut iter);
                if pkt.is_null() {
                    break;
                }
                let pkt = &*pkt;
                if pkt.kind == sys::vpx_codec_cx_pkt_kind::VPX_CODEC_CX_FRAME_PKT {
                    let frame = &pkt.data.frame;
                    let bytes = std::slice::from_raw_parts(frame.buf as *const u8, frame.sz).to_vec();
                    let keyframe = (frame.flags & sys::VPX_FRAME_IS_KEY) != 0;
                    out.push(EncodedFrame { data: bytes, pts: frame.pts, keyframe });
                }
            }
            Ok(out)
        }
    }
}

impl Drop for Vp9Encoder {
    fn drop(&mut self) {
        unsafe {
            sys::vpx_codec_destroy(&mut self.ctx);
        }
    }
}

pub struct DecodedFrame {
    /// Tightly-packed I420 bytes, width*height*3/2 long.
    pub i420: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct Vp9Decoder {
    ctx: sys::vpx_codec_ctx_t,
}

impl Vp9Decoder {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let iface = sys::vpx_codec_vp9_dx();
            let cfg = RealDecCfg::default();
            let mut ctx: sys::vpx_codec_ctx_t = std::mem::zeroed();
            let err = sys::vpx_codec_dec_init_ver(
                &mut ctx,
                iface,
                &cfg as *const RealDecCfg as *const sys::vpx_codec_dec_cfg_t,
                0,
                sys::VPX_DECODER_ABI_VERSION as c_int,
            );
            if err != sys::vpx_codec_err_t::VPX_CODEC_OK {
                return Err(format!("Couldn't initialize the VP9 decoder (libvpx error {err:?}).").to_string());
            }
            Ok(Self { ctx })
        }
    }

    /// Feed one compressed VP9 packet (one WEBM SimpleBlock's payload).
    /// May return zero, one, or more decoded frames.
    pub fn decode_packet(&mut self, data: &[u8]) -> Result<Vec<DecodedFrame>, String> {
        unsafe {
            let err = sys::vpx_codec_decode(&mut self.ctx, data.as_ptr(), data.len() as c_uint, ptr::null_mut(), 0 as c_long);
            if err != sys::vpx_codec_err_t::VPX_CODEC_OK {
                return Err(format!("VP9 decode error (libvpx error {err:?}).").to_string());
            }

            let mut out = Vec::new();
            let mut iter: sys::vpx_codec_iter_t = ptr::null();
            loop {
                let img = sys::vpx_codec_get_frame(&mut self.ctx, &mut iter);
                if img.is_null() {
                    break;
                }
                let img = &*img;
                let w = img.d_w;
                let h = img.d_h;
                let i420 = pack_i420_from_image(img, w as usize, h as usize);
                out.push(DecodedFrame { i420, width: w, height: h });
            }
            Ok(out)
        }
    }
}

impl Drop for Vp9Decoder {
    fn drop(&mut self) {
        unsafe {
            sys::vpx_codec_destroy(&mut self.ctx);
        }
    }
}

/// Copies a decoded image's Y/U/V planes (which have row padding/stride
/// beyond the logical width) into a tightly-packed I420 buffer.
unsafe fn pack_i420_from_image(img: &sys::vpx_image_t, width: usize, height: usize) -> Vec<u8> {
    let cw = (width + 1) / 2;
    let ch = (height + 1) / 2;
    let mut out = Vec::with_capacity(width * height + 2 * cw * ch);

    copy_plane(&mut out, img.planes[0], img.stride[0], width, height);
    copy_plane(&mut out, img.planes[1], img.stride[1], cw, ch);
    copy_plane(&mut out, img.planes[2], img.stride[2], cw, ch);

    out
}

unsafe fn copy_plane(out: &mut Vec<u8>, plane: *mut u8, stride: c_int, row_width: usize, rows: usize) {
    for row in 0..rows {
        let row_ptr = plane.offset(row as isize * stride as isize);
        out.extend_from_slice(std::slice::from_raw_parts(row_ptr, row_width));
    }
}