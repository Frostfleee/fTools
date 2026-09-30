
use std::io::{self, Read, Write};

pub const ID_EBML: u32 = 0x1A45DFA3;
pub const ID_SEGMENT: u32 = 0x1853_8067;
pub const ID_INFO: u32 = 0x1549_A966;
pub const ID_TIMESTAMP_SCALE: u32 = 0x2A_D7B1;
pub const ID_DURATION: u32 = 0x4489;
pub const ID_MUXING_APP: u32 = 0x4D80;
pub const ID_WRITING_APP: u32 = 0x5741;
pub const ID_TRACKS: u32 = 0x1654_AE6B;
pub const ID_TRACK_ENTRY: u32 = 0xAE;
pub const ID_TRACK_NUMBER: u32 = 0xD7;
pub const ID_TRACK_UID: u32 = 0x73C5;
pub const ID_TRACK_TYPE: u32 = 0x83;
pub const ID_CODEC_ID: u32 = 0x86;
pub const ID_CODEC_PRIVATE: u32 = 0x63A2;
pub const ID_CODEC_DELAY: u32 = 0x56AA;
pub const ID_SEEK_PREROLL: u32 = 0x56BB;
pub const ID_VIDEO: u32 = 0xE0;
pub const ID_PIXEL_WIDTH: u32 = 0xB0;
pub const ID_PIXEL_HEIGHT: u32 = 0xBA;
pub const ID_AUDIO: u32 = 0xE1;
pub const ID_SAMPLING_FREQUENCY: u32 = 0xB5;
pub const ID_CHANNELS: u32 = 0x9F;
pub const ID_CLUSTER: u32 = 0x1F43_B675;
pub const ID_TIMESTAMP: u32 = 0xE7;
pub const ID_SIMPLE_BLOCK: u32 = 0xA3;
pub const ID_DOCTYPE: u32 = 0x4282;
pub const ID_EBML_VERSION: u32 = 0x4286;
pub const ID_EBML_READ_VERSION: u32 = 0x42F7;
pub const ID_EBML_MAX_ID_LENGTH: u32 = 0x42F2;
pub const ID_EBML_MAX_SIZE_LENGTH: u32 = 0x42F3;
pub const ID_DOCTYPE_VERSION: u32 = 0x4287;
pub const ID_DOCTYPE_READ_VERSION: u32 = 0x4285;

const UNKNOWN_SIZE: u64 = u64::MAX;


fn read_vint_id<R: Read>(r: &mut R) -> io::Result<Option<u32>> {
    let mut first = [0u8; 1];
    if r.read(&mut first)? == 0 {
        return Ok(None);
    }
    let b0 = first[0];
    let len = b0.leading_zeros() as usize + 1;
    if len > 4 || b0 == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "bad EBML ID length"));
    }
    let mut buf = [0u8; 4];
    buf[4 - len] = b0;
    if len > 1 {
        r.read_exact(&mut buf[4 - len + 1..4])?;
    }
    Ok(Some(u32::from_be_bytes(buf)))
}

fn read_vint_size<R: Read>(r: &mut R) -> io::Result<u64> {
    let mut first = [0u8; 1];
    r.read_exact(&mut first)?;
    let b0 = first[0];
    let len = b0.leading_zeros() as usize + 1;
    if len > 8 || b0 == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "bad EBML size length"));
    }
    let mask: u8 = if len >= 8 { 0 } else { 0xFFu8 >> len };
    let mut value = (b0 & mask) as u64;
    let mut all_ones = value == (mask as u64);
    let mut rest = vec![0u8; len - 1];
    if !rest.is_empty() {
        r.read_exact(&mut rest)?;
    }
    for b in &rest {
        value = (value << 8) | (*b as u64);
        if *b != 0xFF {
            all_ones = false;
        }
    }
    if all_ones {
        return Ok(UNKNOWN_SIZE);
    }
    Ok(value)
}

pub struct Elem {
    pub id: u32,
    pub data: Vec<u8>,
}

pub fn read_elements(mut data: &[u8]) -> io::Result<Vec<Elem>> {
    let mut out = Vec::new();
    loop {
        if data.is_empty() {
            break;
        }
        let mut cursor = data;
        let id = match read_vint_id(&mut cursor)? {
            Some(id) => id,
            None => break,
        };
        let consumed_id = data.len() - cursor.len();
        let size = read_vint_size(&mut cursor)?;
        let consumed_size = (data.len() - consumed_id) - cursor.len();
        let header_len = consumed_id + consumed_size;
        let size = if size == UNKNOWN_SIZE {
            (data.len() - header_len) as u64
        } else {
            size
        };
        let body_start = header_len;
        let body_end = body_start + size as usize;
        if body_end > data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "element runs past end of buffer"));
        }
        out.push(Elem { id, data: data[body_start..body_end].to_vec() });
        data = &data[body_end..];
    }
    Ok(out)
}

pub fn find<'a>(elems: &'a [Elem], id: u32) -> Option<&'a Elem> {
    elems.iter().find(|e| e.id == id)
}

pub fn find_all<'a>(elems: &'a [Elem], id: u32) -> Vec<&'a Elem> {
    elems.iter().filter(|e| e.id == id).collect()
}

pub fn as_uint(e: &Elem) -> u64 {
    e.data.iter().fold(0u64, |acc, b| (acc << 8) | (*b as u64))
}

pub fn as_string(e: &Elem) -> String {
    String::from_utf8_lossy(&e.data).trim_end_matches('\0').to_string()
}

#[derive(Debug, Clone)]
pub struct TrackMeta {
    pub number: u64,
    pub track_type: u64,
    pub codec_id: String,
    pub codec_private: Vec<u8>,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub sample_rate: Option<f64>,
    pub channels: Option<u64>,
    pub codec_delay_ns: Option<u64>,
}

pub fn parse_tracks(full_file: &[u8]) -> io::Result<Vec<TrackMeta>> {
    let top = read_elements(full_file)?;
    let segment = find(&top, ID_SEGMENT).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no Segment element"))?;
    let seg_children = read_elements(&segment.data)?;
    let tracks_elem = find(&seg_children, ID_TRACKS).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no Tracks element"))?;
    let tracks_elems = read_elements(&tracks_elem.data)?;
    let track_entries = find_all(&tracks_elems, ID_TRACK_ENTRY);

    let mut out = Vec::new();
    for te in &track_entries {
        let fields = read_elements(&te.data)?;
        let number = find(&fields, ID_TRACK_NUMBER).map(as_uint).unwrap_or(0);
        let track_type = find(&fields, ID_TRACK_TYPE).map(as_uint).unwrap_or(0);
        let codec_id = find(&fields, ID_CODEC_ID).map(as_string).unwrap_or_default();
        let codec_private = find(&fields, ID_CODEC_PRIVATE).map(|e| e.data.clone()).unwrap_or_default();
        let codec_delay_ns = find(&fields, ID_CODEC_DELAY).map(as_uint);

        let (mut width, mut height) = (None, None);
        if let Some(v) = find(&fields, ID_VIDEO) {
            let vf = read_elements(&v.data)?;
            width = find(&vf, ID_PIXEL_WIDTH).map(as_uint);
            height = find(&vf, ID_PIXEL_HEIGHT).map(as_uint);
        }
        let (mut sample_rate, mut channels) = (None, None);
        if let Some(a) = find(&fields, ID_AUDIO) {
            let af = read_elements(&a.data)?;
            sample_rate = find(&af, ID_SAMPLING_FREQUENCY).map(|e| {
                if e.data.len() == 4 {
                    f32::from_be_bytes(e.data[..4].try_into().unwrap()) as f64
                } else if e.data.len() == 8 {
                    f64::from_be_bytes(e.data[..8].try_into().unwrap())
                } else {
                    0.0
                }
            });
            channels = find(&af, ID_CHANNELS).map(as_uint);
        }

        out.push(TrackMeta {
            number, track_type, codec_id, codec_private,
            width, height, sample_rate, channels,
            codec_delay_ns,
        });
    }
    Ok(out)
}


fn write_vint_size<W: Write>(w: &mut W, value: u64) -> io::Result<()> {
    for len in 1u32..=8 {
        let data_bits = 7 * len;
        if len == 8 || value < (1u64 << data_bits) - 1 {
            let marker = 1u8 << (8 - len);
            let mut bytes = value.to_be_bytes();
            let start = 8 - len as usize;
            bytes[start] |= marker;
            w.write_all(&bytes[start..])?;
            return Ok(());
        }
    }
    unreachable!()
}

fn write_id<W: Write>(w: &mut W, id: u32) -> io::Result<()> {
    let bytes = id.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(3);
    w.write_all(&bytes[start..])
}

pub fn write_elem<W: Write>(w: &mut W, id: u32, body: &[u8]) -> io::Result<()> {
    write_id(w, id)?;
    write_vint_size(w, body.len() as u64)?;
    w.write_all(body)
}

pub fn build_elem(id: u32, body: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(body.len() + 12);
    write_elem(&mut v, id, body).unwrap();
    v
}

pub fn uint_body(v: u64) -> Vec<u8> {
    if v == 0 {
        return vec![0];
    }
    let bytes = v.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap();
    bytes[start..].to_vec()
}

pub fn float_body_f64(v: f64) -> Vec<u8> {
    v.to_be_bytes().to_vec()
}

pub fn simple_block_body(track_number: u64, rel_ts: i16, keyframe: bool, frame: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(frame.len() + 8);
    write_vint_size(&mut out, track_number).unwrap();
    out.extend_from_slice(&rel_ts.to_be_bytes());
    out.push(if keyframe { 0x80 } else { 0x00 });
    out.extend_from_slice(frame);
    out
}
