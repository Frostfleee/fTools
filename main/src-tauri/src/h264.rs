pub struct BitReader {
    data: Vec<u8>,
    pos: usize,
}

impl BitReader {
    pub fn new(data: Vec<u8>) -> Self {
        BitReader { data, pos: 0 }
    }

    pub fn bit(&mut self) -> Option<u32> {
        let byte = *self.data.get(self.pos / 8)?;
        let v = (byte >> (7 - self.pos % 8)) & 1;
        self.pos += 1;
        Some(v as u32)
    }

    pub fn bits(&mut self, n: u32) -> Option<u32> {
        let mut v = 0;
        for _ in 0..n {
            v = (v << 1) | self.bit()?;
        }
        Some(v)
    }

    pub fn ue(&mut self) -> Option<u32> {
        let mut zeros = 0;
        while self.bit()? == 0 {
            zeros += 1;
            if zeros > 31 {
                return None;
            }
        }
        Some(((1u64 << zeros) - 1 + self.bits(zeros)? as u64) as u32)
    }

    pub fn se(&mut self) -> Option<i32> {
        let v = self.ue()?;
        Some(if v % 2 == 1 { v.div_ceil(2) as i32 } else { -((v / 2) as i32) })
    }
}

fn rbsp(nal: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(nal.len());
    let mut zeros = 0;
    for &b in nal.get(1..).unwrap_or_default() {
        if zeros >= 2 && b == 3 {
            zeros = 0;
            continue;
        }
        zeros = if b == 0 { zeros + 1 } else { 0 };
        out.push(b);
    }
    out
}

pub struct Sps {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub frame_num_bits: u32,
    pub poc_type: u32,
    pub poc_lsb_bits: u32,
    pub frame_mbs_only: bool,
    pub separate_planes: bool,
}

pub fn parse_sps(nal: &[u8]) -> Option<Sps> {
    let mut r = BitReader::new(rbsp(nal));
    let profile = r.bits(8)?;
    r.bits(16)?;
    let id = r.ue()?;
    let mut chroma_format = 1;
    let mut separate_planes = false;
    if matches!(profile, 100 | 110 | 122 | 244 | 44 | 83 | 86 | 118 | 128 | 138 | 139 | 134 | 135) {
        chroma_format = r.ue()?;
        if chroma_format == 3 {
            separate_planes = r.bit()? == 1;
        }
        r.ue()?;
        r.ue()?;
        r.bit()?;
        if r.bit()? == 1 {
            for i in 0..if chroma_format == 3 { 12 } else { 8 } {
                if r.bit()? == 1 {
                    let size = if i < 6 { 16 } else { 64 };
                    let (mut last, mut next) = (8i32, 8i32);
                    for _ in 0..size {
                        if next != 0 {
                            next = (last + r.se()? + 256) % 256;
                        }
                        if next != 0 {
                            last = next;
                        }
                    }
                }
            }
        }
    }
    let frame_num_bits = r.ue()? + 4;
    let poc_type = r.ue()?;
    let mut poc_lsb_bits = 0;
    match poc_type {
        0 => poc_lsb_bits = r.ue()? + 4,
        1 => {
            r.bit()?;
            r.se()?;
            r.se()?;
            for _ in 0..r.ue()? {
                r.se()?;
            }
        }
        _ => {}
    }
    r.ue()?;
    r.bit()?;
    let width_mbs = r.ue()? + 1;
    let height_units = r.ue()? + 1;
    let frame_mbs_only = r.bit()?;
    if frame_mbs_only == 0 {
        r.bit()?;
    }
    r.bit()?;
    let (mut crop_x, mut crop_y) = (0, 0);
    if r.bit()? == 1 {
        let (left, right, top, bottom) = (r.ue()?, r.ue()?, r.ue()?, r.ue()?);
        let (unit_x, unit_y) = if chroma_format == 0 || separate_planes {
            (1, 2 - frame_mbs_only)
        } else {
            let sub_w = if chroma_format == 3 { 1 } else { 2 };
            let sub_h = if chroma_format == 1 { 2 } else { 1 };
            (sub_w, sub_h * (2 - frame_mbs_only))
        };
        crop_x = (left + right) * unit_x;
        crop_y = (top + bottom) * unit_y;
    }
    let width = (width_mbs * 16).checked_sub(crop_x)?;
    let height = ((2 - frame_mbs_only) * height_units * 16).checked_sub(crop_y)?;
    Some(Sps {
        id,
        width,
        height,
        frame_num_bits,
        poc_type,
        poc_lsb_bits,
        frame_mbs_only: frame_mbs_only == 1,
        separate_planes,
    })
}

pub fn pps_ids(nal: &[u8]) -> Option<(u32, u32)> {
    let mut r = BitReader::new(rbsp(nal));
    Some((r.ue()?, r.ue()?))
}

pub fn sps_dimensions(avcc: &[u8]) -> Option<(u32, u32)> {
    if avcc.len() < 8 || avcc[5] & 0x1F == 0 {
        return None;
    }
    let len = u16::from_be_bytes([avcc[6], avcc[7]]) as usize;
    let sps = parse_sps(avcc.get(8..8 + len)?)?;
    Some((sps.width, sps.height))
}

pub fn parameter_sets(avcc: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut pos = 5usize;
    for mask in [0x1Fu8, 0xFF] {
        let Some(&count) = avcc.get(pos) else { return out };
        pos += 1;
        for _ in 0..count & mask {
            let Some(len) = avcc.get(pos..pos + 2).map(|b| u16::from_be_bytes([b[0], b[1]]) as usize) else { return out };
            pos += 2;
            let Some(nal) = avcc.get(pos..pos + len) else { return out };
            out.push(nal);
            pos += len;
        }
    }
    out
}

pub fn build_avcc(sps: &[Vec<u8>], pps: &[Vec<u8>]) -> Option<Vec<u8>> {
    let first = sps.first().filter(|s| s.len() >= 4)?;
    if pps.is_empty() {
        return None;
    }
    let mut out = vec![1, first[1], first[2], first[3], 0xFF, 0xE0 | (sps.len().min(31) as u8)];
    for s in sps.iter().take(31) {
        out.extend_from_slice(&(s.len() as u16).to_be_bytes());
        out.extend_from_slice(s);
    }
    out.push(pps.len().min(255) as u8);
    for p in pps.iter().take(255) {
        out.extend_from_slice(&(p.len() as u16).to_be_bytes());
        out.extend_from_slice(p);
    }
    Some(out)
}

pub fn split_annexb(data: &[u8]) -> Vec<&[u8]> {
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

pub fn length_prefixed(data: &[u8], length_size: usize) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + length_size <= data.len() {
        let len = data[i..i + length_size].iter().fold(0usize, |acc, &b| (acc << 8) | b as usize);
        i += length_size;
        if len == 0 || i + len > data.len() {
            break;
        }
        out.push(&data[i..i + len]);
        i += len;
    }
    out
}

pub struct PocTracker<'s> {
    sps: &'s [Sps],
    pps: Vec<(u32, u32)>,
    prev_msb: i64,
    prev_lsb: i64,
}

impl<'s> PocTracker<'s> {
    pub fn new(sps: &'s [Sps], pps: Vec<(u32, u32)>) -> Self {
        PocTracker { sps, pps, prev_msb: 0, prev_lsb: 0 }
    }

    pub fn reorders(&self) -> bool {
        self.sps.iter().any(|s| s.poc_type == 0)
    }

    pub fn picture(&mut self, nals: &[&[u8]]) -> Option<(bool, i64)> {
        let nal = nals.iter().find(|n| matches!(n.first().map(|b| b & 0x1F), Some(1 | 5)))?;
        let idr = nal[0] & 0x1F == 5;
        let reference = nal[0] & 0x60 != 0;
        let mut r = BitReader::new(rbsp(nal));
        r.ue()?;
        r.ue()?;
        let pps_id = r.ue()?;
        let sps_id = self.pps.iter().find(|p| p.0 == pps_id).map(|p| p.1).unwrap_or(0);
        let sps = self.sps.iter().find(|s| s.id == sps_id).or(self.sps.first())?;
        if sps.poc_type != 0 {
            return None;
        }
        if sps.separate_planes {
            r.bits(2)?;
        }
        r.bits(sps.frame_num_bits)?;
        if !sps.frame_mbs_only && r.bit()? == 1 {
            r.bit()?;
        }
        if idr {
            r.ue()?;
        }
        let lsb = r.bits(sps.poc_lsb_bits)? as i64;
        if idr {
            self.prev_msb = 0;
            self.prev_lsb = 0;
        }
        let max = 1i64 << sps.poc_lsb_bits;
        let msb = if lsb < self.prev_lsb && self.prev_lsb - lsb >= max / 2 {
            self.prev_msb + max
        } else if lsb > self.prev_lsb && lsb - self.prev_lsb > max / 2 {
            self.prev_msb - max
        } else {
            self.prev_msb
        };
        if reference {
            self.prev_msb = msb;
            self.prev_lsb = lsb;
        }
        Some((idr, msb + lsb))
    }
}