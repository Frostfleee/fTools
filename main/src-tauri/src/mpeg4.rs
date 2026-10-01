const ZIGZAG: [u8; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28, 35, 42,
    49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

const ALT_HORIZONTAL: [u8; 64] = [
    0, 1, 2, 3, 8, 9, 16, 17, 10, 11, 4, 5, 6, 7, 15, 14, 13, 12, 19, 18, 24, 25, 32, 33, 26, 27, 20, 21, 22, 23, 28, 29, 30, 31,
    34, 35, 40, 41, 48, 49, 42, 43, 36, 37, 38, 39, 44, 45, 46, 47, 50, 51, 56, 57, 58, 59, 52, 53, 54, 55, 60, 61, 62, 63,
];

const ALT_VERTICAL: [u8; 64] = [
    0, 8, 16, 24, 1, 9, 2, 10, 17, 25, 32, 40, 48, 56, 57, 49, 41, 33, 26, 18, 3, 11, 4, 12, 19, 27, 34, 42, 50, 58, 35, 43, 51,
    59, 20, 28, 5, 13, 6, 14, 21, 29, 36, 44, 52, 60, 37, 45, 53, 61, 22, 30, 7, 15, 23, 31, 38, 46, 54, 62, 39, 47, 55, 63,
];

const DEFAULT_INTRA_MATRIX: [u16; 64] = [
    8, 17, 18, 19, 21, 23, 25, 27, 17, 18, 19, 21, 23, 25, 27, 28, 20, 21, 22, 23, 24, 26, 28, 30, 21, 22, 23, 24, 26, 28, 30, 32,
    22, 23, 24, 26, 28, 30, 32, 35, 23, 24, 26, 28, 30, 32, 35, 38, 25, 26, 28, 30, 32, 35, 38, 41, 27, 28, 30, 32, 35, 38, 41, 45,
];

const DEFAULT_INTER_MATRIX: [u16; 64] = [
    16, 17, 18, 19, 20, 21, 22, 23, 17, 18, 19, 20, 21, 22, 23, 24, 18, 19, 20, 21, 22, 23, 24, 25, 19, 20, 21, 22, 23, 24, 26, 27,
    20, 21, 22, 23, 25, 26, 27, 28, 21, 22, 23, 24, 26, 27, 28, 30, 22, 23, 24, 26, 27, 28, 30, 31, 23, 24, 25, 27, 28, 30, 31, 33,
];

const INTRA_MCBPC: [(u32, u8); 9] = [(1, 1), (1, 3), (2, 3), (3, 3), (1, 4), (1, 6), (2, 6), (3, 6), (1, 9)];

const INTER_MCBPC: [(u32, u8); 21] = [
    (1, 1),
    (3, 4),
    (2, 4),
    (5, 6),
    (3, 3),
    (7, 7),
    (6, 7),
    (5, 9),
    (2, 3),
    (5, 7),
    (4, 7),
    (5, 8),
    (3, 5),
    (4, 8),
    (3, 8),
    (3, 7),
    (4, 6),
    (4, 9),
    (3, 9),
    (2, 9),
    (1, 9),
];

const CBPY: [(u32, u8); 16] = [
    (3, 4),
    (5, 5),
    (4, 5),
    (9, 4),
    (3, 5),
    (7, 4),
    (2, 6),
    (11, 4),
    (2, 5),
    (3, 6),
    (5, 4),
    (10, 4),
    (4, 4),
    (8, 4),
    (6, 4),
    (3, 2),
];

const DC_LUMA: [(u32, u8); 13] =
    [(3, 3), (3, 2), (2, 2), (2, 3), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (1, 10), (1, 11)];

const DC_CHROMA: [(u32, u8); 13] =
    [(3, 2), (2, 2), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (1, 10), (1, 11), (1, 12)];

const MVD: [(u32, u8); 33] = [
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (3, 6),
    (5, 7),
    (4, 7),
    (3, 7),
    (11, 9),
    (10, 9),
    (9, 9),
    (17, 10),
    (16, 10),
    (15, 10),
    (14, 10),
    (13, 10),
    (12, 10),
    (11, 10),
    (10, 10),
    (9, 10),
    (8, 10),
    (7, 10),
    (6, 10),
    (5, 10),
    (4, 10),
    (7, 11),
    (6, 11),
    (5, 11),
    (4, 11),
    (3, 11),
    (2, 11),
    (3, 12),
    (2, 12),
];

const B_MB_TYPE: [(u32, u8); 4] = [(1, 1), (1, 2), (1, 3), (1, 4)];

const SPRITE_LENGTH: [(u32, u8); 15] = [
    (0x00, 2),
    (0x02, 3),
    (0x03, 3),
    (0x04, 3),
    (0x05, 3),
    (0x06, 3),
    (0x0E, 4),
    (0x1E, 5),
    (0x3E, 6),
    (0x7E, 7),
    (0xFE, 8),
    (0x1FE, 9),
    (0x3FE, 10),
    (0x7FE, 11),
    (0xFFE, 12),
];

const INTER_MAX_LEVEL: [&[u8]; 2] = [
    &[12, 6, 4, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    &[3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const INTER_CODES: [(u32, u8); 102] = [
    (0x2, 2),
    (0xf, 4),
    (0x15, 6),
    (0x17, 7),
    (0x1f, 8),
    (0x25, 9),
    (0x24, 9),
    (0x21, 10),
    (0x20, 10),
    (0x7, 11),
    (0x6, 11),
    (0x20, 11),
    (0x6, 3),
    (0x14, 6),
    (0x1e, 8),
    (0xf, 10),
    (0x21, 11),
    (0x50, 12),
    (0xe, 4),
    (0x1d, 8),
    (0xe, 10),
    (0x51, 12),
    (0xd, 5),
    (0x23, 9),
    (0xd, 10),
    (0xc, 5),
    (0x22, 9),
    (0x52, 12),
    (0xb, 5),
    (0xc, 10),
    (0x53, 12),
    (0x13, 6),
    (0xb, 10),
    (0x54, 12),
    (0x12, 6),
    (0xa, 10),
    (0x11, 6),
    (0x9, 10),
    (0x10, 6),
    (0x8, 10),
    (0x16, 7),
    (0x55, 12),
    (0x15, 7),
    (0x14, 7),
    (0x1c, 8),
    (0x1b, 8),
    (0x21, 9),
    (0x20, 9),
    (0x1f, 9),
    (0x1e, 9),
    (0x1d, 9),
    (0x1c, 9),
    (0x1b, 9),
    (0x1a, 9),
    (0x22, 11),
    (0x23, 11),
    (0x56, 12),
    (0x57, 12),
    (0x7, 4),
    (0x19, 9),
    (0x5, 11),
    (0xf, 6),
    (0x4, 11),
    (0xe, 6),
    (0xd, 6),
    (0xc, 6),
    (0x13, 7),
    (0x12, 7),
    (0x11, 7),
    (0x10, 7),
    (0x1a, 8),
    (0x19, 8),
    (0x18, 8),
    (0x17, 8),
    (0x16, 8),
    (0x15, 8),
    (0x14, 8),
    (0x13, 8),
    (0x18, 9),
    (0x17, 9),
    (0x16, 9),
    (0x15, 9),
    (0x14, 9),
    (0x13, 9),
    (0x12, 9),
    (0x11, 9),
    (0x7, 10),
    (0x6, 10),
    (0x5, 10),
    (0x4, 10),
    (0x24, 11),
    (0x25, 11),
    (0x26, 11),
    (0x27, 11),
    (0x58, 12),
    (0x59, 12),
    (0x5a, 12),
    (0x5b, 12),
    (0x5c, 12),
    (0x5d, 12),
    (0x5e, 12),
    (0x5f, 12),
];

const INTRA_MAX_LEVEL: [&[u8]; 2] = [
    &[27, 10, 5, 4, 3, 3, 3, 3, 2, 2, 1, 1, 1, 1, 1],
    &[8, 3, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const INTRA_CODES: [(u32, u8); 102] = [
    (0x2, 2),
    (0x6, 3),
    (0xf, 4),
    (0xd, 5),
    (0xc, 5),
    (0x15, 6),
    (0x13, 6),
    (0x12, 6),
    (0x17, 7),
    (0x1f, 8),
    (0x1e, 8),
    (0x1d, 8),
    (0x25, 9),
    (0x24, 9),
    (0x23, 9),
    (0x21, 9),
    (0x21, 10),
    (0x20, 10),
    (0xf, 10),
    (0xe, 10),
    (0x7, 11),
    (0x6, 11),
    (0x20, 11),
    (0x21, 11),
    (0x50, 12),
    (0x51, 12),
    (0x52, 12),
    (0xe, 4),
    (0x14, 6),
    (0x16, 7),
    (0x1c, 8),
    (0x20, 9),
    (0x1f, 9),
    (0xd, 10),
    (0x22, 11),
    (0x53, 12),
    (0x55, 12),
    (0xb, 5),
    (0x15, 7),
    (0x1e, 9),
    (0xc, 10),
    (0x56, 12),
    (0x11, 6),
    (0x1b, 8),
    (0x1d, 9),
    (0xb, 10),
    (0x10, 6),
    (0x22, 9),
    (0xa, 10),
    (0xd, 6),
    (0x1c, 9),
    (0x8, 10),
    (0x12, 7),
    (0x1b, 9),
    (0x54, 12),
    (0x14, 7),
    (0x1a, 9),
    (0x57, 12),
    (0x19, 8),
    (0x9, 10),
    (0x18, 8),
    (0x23, 11),
    (0x17, 8),
    (0x19, 9),
    (0x18, 9),
    (0x7, 10),
    (0x58, 12),
    (0x7, 4),
    (0xc, 6),
    (0x16, 8),
    (0x17, 9),
    (0x6, 10),
    (0x5, 11),
    (0x4, 11),
    (0x59, 12),
    (0xf, 6),
    (0x16, 9),
    (0x5, 10),
    (0xe, 6),
    (0x4, 10),
    (0x11, 7),
    (0x24, 11),
    (0x10, 7),
    (0x25, 11),
    (0x13, 7),
    (0x5a, 12),
    (0x15, 8),
    (0x5b, 12),
    (0x14, 8),
    (0x13, 8),
    (0x1a, 8),
    (0x15, 9),
    (0x14, 9),
    (0x13, 9),
    (0x12, 9),
    (0x11, 9),
    (0x26, 11),
    (0x27, 11),
    (0x5c, 12),
    (0x5d, 12),
    (0x5e, 12),
    (0x5f, 12),
];

const ESCAPE: (u32, u8) = (0x3, 7);
const STUFFING: i16 = -2;
const START_CODE_VOL: u8 = 0x20;
const START_CODE_USER_DATA: u8 = 0xB2;
const START_CODE_GOV: u8 = 0xB3;
const START_CODE_VOP: u8 = 0xB6;

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8]) -> Self {
        Bits { data, pos: 0 }
    }

    fn peek(&self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let byte = self.pos >> 3;
        let mut v = 0u64;
        for i in 0..5 {
            v = (v << 8) | *self.data.get(byte + i).unwrap_or(&0) as u64;
        }
        ((v >> (40 - (self.pos & 7) as u32 - n)) & ((1u64 << n) - 1)) as u32
    }

    fn skip(&mut self, n: u32) {
        self.pos += n as usize;
    }

    fn get(&mut self, n: u32) -> u32 {
        let v = self.peek(n);
        self.pos += n as usize;
        v
    }

    fn bit(&mut self) -> bool {
        self.get(1) == 1
    }

    fn overrun(&self) -> bool {
        self.pos > self.data.len() * 8
    }

    fn total(&self) -> usize {
        self.data.len() * 8
    }
}

struct Vlc {
    bits: u32,
    table: Vec<(i16, u8)>,
}

impl Vlc {
    fn new(codes: impl IntoIterator<Item = (u32, u8, i16)>) -> Self {
        let codes: Vec<(u32, u8, i16)> = codes.into_iter().collect();
        let bits = codes.iter().map(|c| c.1 as u32).max().unwrap_or(1);
        let mut table = vec![(-1i16, 0u8); 1 << bits];
        for (code, len, value) in codes {
            let shift = bits - len as u32;
            let first = (code << shift) as usize;
            for entry in table.iter_mut().skip(first).take(1 << shift) {
                *entry = (value, len);
            }
        }
        Vlc { bits, table }
    }

    fn read(&self, br: &mut Bits) -> Option<i16> {
        let (value, len) = self.table[br.peek(self.bits) as usize];
        if len == 0 {
            return None;
        }
        br.skip(len as u32);
        Some(value)
    }
}

struct RunLevel {
    vlc: Vlc,
    entries: Vec<(bool, u8, u8)>,
    max_level: [[u8; 64]; 2],
    max_run: [[u8; 64]; 2],
}

const ESCAPE_INDEX: i16 = 1000;

impl RunLevel {
    fn new(codes: &[(u32, u8); 102], max_level: [&[u8]; 2]) -> Self {
        let mut entries = Vec::with_capacity(102);
        let mut tables = RunLevel { vlc: Vlc::new([]), entries: Vec::new(), max_level: [[0; 64]; 2], max_run: [[0; 64]; 2] };
        for (last, levels) in max_level.iter().enumerate() {
            for (run, &top) in levels.iter().enumerate() {
                tables.max_level[last][run] = top;
                for level in 1..=top {
                    entries.push((last == 1, run as u8, level));
                    let slot = &mut tables.max_run[last][level as usize];
                    *slot = (*slot).max(run as u8);
                }
            }
        }
        let mut list: Vec<(u32, u8, i16)> = codes.iter().enumerate().map(|(i, &(c, l))| (c, l, i as i16)).collect();
        list.push((ESCAPE.0, ESCAPE.1, ESCAPE_INDEX));
        tables.vlc = Vlc::new(list);
        tables.entries = entries;
        tables
    }

    fn read(&self, br: &mut Bits) -> Option<(bool, usize, i32)> {
        let index = self.vlc.read(br)?;
        if index != ESCAPE_INDEX {
            let (last, run, level) = self.entries[index as usize];
            let level = if br.bit() { -(level as i32) } else { level as i32 };
            return Some((last, run as usize, level));
        }
        if !br.bit() {
            let index = self.vlc.read(br)?;
            if index == ESCAPE_INDEX {
                return None;
            }
            let (last, run, level) = self.entries[index as usize];
            let level = level as i32 + self.max_level[last as usize][run as usize] as i32;
            return Some((last, run as usize, if br.bit() { -level } else { level }));
        }
        if !br.bit() {
            let index = self.vlc.read(br)?;
            if index == ESCAPE_INDEX {
                return None;
            }
            let (last, run, level) = self.entries[index as usize];
            let run = run as usize + self.max_run[last as usize][level as usize] as usize + 1;
            let level = level as i32;
            return Some((last, run, if br.bit() { -level } else { level }));
        }
        let last = br.bit();
        let run = br.get(6) as usize;
        br.skip(1);
        let raw = br.get(12) as i32;
        br.skip(1);
        let level = (raw << 20) >> 20;
        if level == 0 {
            return None;
        }
        Some((last, run, level))
    }
}

struct Tables {
    intra_mcbpc: Vlc,
    inter_mcbpc: Vlc,
    cbpy: Vlc,
    dc_luma: Vlc,
    dc_chroma: Vlc,
    mvd: Vlc,
    b_mb_type: Vlc,
    sprite_length: Vlc,
    intra: RunLevel,
    inter: RunLevel,
    idct: [[i32; 8]; 8],
}

impl Tables {
    fn new() -> Self {
        let indexed = |codes: &[(u32, u8)]| -> Vec<(u32, u8, i16)> { codes.iter().enumerate().map(|(i, &(c, l))| (c, l, i as i16)).collect() };
        let mut intra_mcbpc = indexed(&INTRA_MCBPC);
        intra_mcbpc[8].2 = STUFFING;
        let mut inter_mcbpc = indexed(&INTER_MCBPC);
        inter_mcbpc[20].2 = STUFFING;
        let mut idct = [[0i32; 8]; 8];
        for (x, row) in idct.iter_mut().enumerate() {
            for (u, k) in row.iter_mut().enumerate() {
                let c = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                let v = 0.5 * c * ((2 * x + 1) as f64 * u as f64 * std::f64::consts::PI / 16.0).cos();
                *k = (v * 8192.0).round() as i32;
            }
        }
        Tables {
            intra_mcbpc: Vlc::new(intra_mcbpc),
            inter_mcbpc: Vlc::new(inter_mcbpc),
            cbpy: Vlc::new(indexed(&CBPY)),
            dc_luma: Vlc::new(indexed(&DC_LUMA)),
            dc_chroma: Vlc::new(indexed(&DC_CHROMA)),
            mvd: Vlc::new(indexed(&MVD)),
            b_mb_type: Vlc::new(indexed(&B_MB_TYPE)),
            sprite_length: Vlc::new(indexed(&SPRITE_LENGTH)),
            intra: RunLevel::new(&INTRA_CODES, INTRA_MAX_LEVEL),
            inter: RunLevel::new(&INTER_CODES, INTER_MAX_LEVEL),
            idct,
        }
    }

    fn idct(&self, block: &mut [i32; 64]) {
        let k = &self.idct;
        let mut tmp = [0i32; 64];
        for r in 0..8 {
            let row = &block[r * 8..r * 8 + 8];
            if row.iter().all(|&v| v == 0) {
                continue;
            }
            for x in 0..8 {
                let mut s = 0i32;
                for u in 0..8 {
                    s += k[x][u] * row[u];
                }
                tmp[r * 8 + x] = (s + (1 << 10)) >> 11;
            }
        }
        for c in 0..8 {
            for y in 0..8 {
                let mut s = 0i32;
                for v in 0..8 {
                    s += k[y][v] * tmp[v * 8 + c];
                }
                block[y * 8 + c] = (s + (1 << 14)) >> 15;
            }
        }
    }
}

#[derive(Clone)]
struct Frame {
    y: Vec<u8>,
    u: Vec<u8>,
    v: Vec<u8>,
    stride: usize,
    cstride: usize,
}

impl Frame {
    fn new(mb_w: usize, mb_h: usize) -> Self {
        let stride = mb_w * 16;
        let cstride = mb_w * 8;
        Frame { y: vec![0; stride * mb_h * 16], u: vec![128; cstride * mb_h * 8], v: vec![128; cstride * mb_h * 8], stride, cstride }
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub i420: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq)]
enum MbKind {
    Intra,
    Inter,
    Inter4v,
    Skipped,
}

#[derive(Clone)]
struct Vol {
    width: usize,
    height: usize,
    mb_w: usize,
    mb_h: usize,
    time_bits: u32,
    time_res: i64,
    mpeg_quant: bool,
    intra_matrix: [u16; 64],
    inter_matrix: [u16; 64],
    qpel: bool,
    resync: bool,
    warp_points: u32,
    warp_accuracy: u32,
    gmc: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum VopType {
    I,
    P,
    B,
}

#[derive(Clone, Copy)]
struct Gmc {
    offset: [[i64; 2]; 2],
    delta: [[i64; 2]; 2],
    shift: [i64; 2],
    points: u32,
    accuracy: u32,
}

struct Vop {
    gmc: Option<Gmc>,
    kind: VopType,
    rounding: i32,
    dc_threshold: i32,
    qp: i32,
    fcode_forward: u32,
    fcode_backward: u32,
}

struct Plane<'a> {
    data: &'a [u8],
    stride: usize,
    width: usize,
    height: usize,
}

fn dc_scale(qp: i32, luma: bool) -> i32 {
    if luma {
        match qp {
            0..=4 => 8,
            5..=8 => 2 * qp,
            9..=24 => qp + 8,
            _ => 2 * qp - 16,
        }
    } else {
        match qp {
            0..=4 => 8,
            5..=24 => (qp + 13) / 2,
            _ => qp - 6,
        }
    }
}

fn rounded_div(a: i32, b: i32) -> i32 {
    if a >= 0 { (a + b / 2) / b } else { (a - b / 2) / b }
}

fn median(a: i32, b: i32, c: i32) -> i32 {
    a.max(b).min(a.min(b).max(c))
}

fn chroma_round(sum: i32) -> i32 {
    const TABLE: [i32; 16] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2];
    TABLE[(sum & 15) as usize] + ((sum >> 3) & !1)
}

fn fetch(p: &Plane, x0: isize, y0: isize, w: usize, h: usize, out: &mut [u8]) {
    let max_x = p.width as isize - 1;
    let max_y = p.height as isize - 1;
    if x0 >= 0 && y0 >= 0 && x0 + w as isize - 1 <= max_x && y0 + h as isize - 1 <= max_y {
        for j in 0..h {
            let start = (y0 as usize + j) * p.stride + x0 as usize;
            out[j * w..j * w + w].copy_from_slice(&p.data[start..start + w]);
        }
        return;
    }
    for j in 0..h {
        let sy = (y0 + j as isize).clamp(0, max_y) as usize * p.stride;
        for i in 0..w {
            let sx = (x0 + i as isize).clamp(0, max_x) as usize;
            out[j * w + i] = p.data[sy + sx];
        }
    }
}

fn halfpel(p: &Plane, x: isize, y: isize, w: usize, h: usize, mvx: i32, mvy: i32, rounding: i32, out: &mut [u8], out_stride: usize) {
    let mut buf = [0u8; 17 * 17];
    let bw = w + 1;
    fetch(p, x + (mvx >> 1) as isize, y + (mvy >> 1) as isize, bw, h + 1, &mut buf);
    let (fx, fy) = (mvx & 1, mvy & 1);
    for j in 0..h {
        for i in 0..w {
            let a = buf[j * bw + i] as i32;
            let v = match (fx, fy) {
                (0, 0) => a,
                (1, 0) => (a + buf[j * bw + i + 1] as i32 + 1 - rounding) >> 1,
                (0, 1) => (a + buf[(j + 1) * bw + i] as i32 + 1 - rounding) >> 1,
                _ => {
                    (a + buf[j * bw + i + 1] as i32 + buf[(j + 1) * bw + i] as i32 + buf[(j + 1) * bw + i + 1] as i32 + 2 - rounding)
                        >> 2
                }
            };
            out[j * out_stride + i] = v as u8;
        }
    }
}

fn mirror(n: isize, w: usize) -> usize {
    let w = w as isize;
    (if n < 0 {
        -1 - n
    } else if n > w {
        2 * w + 1 - n
    } else {
        n
    }) as usize
}

fn lowpass(s: &dyn Fn(usize) -> i32, n: usize, w: usize, rounding: i32) -> u8 {
    let at = |k: isize| s(mirror(n as isize + k, w));
    let sum = (at(0) + at(1)) * 20 - (at(-1) + at(2)) * 6 + (at(-2) + at(3)) * 3 - (at(-3) + at(4));
    ((sum + 16 - rounding) >> 5).clamp(0, 255) as u8
}

fn qpel(p: &Plane, x: isize, y: isize, w: usize, h: usize, mvx: i32, mvy: i32, rounding: i32, out: &mut [u8], out_stride: usize) {
    let mut buf = [0u8; 17 * 17];
    let bw = w + 1;
    fetch(p, x + (mvx >> 2) as isize, y + (mvy >> 2) as isize, bw, h + 1, &mut buf);
    let (fx, fy) = (mvx & 3, mvy & 3);
    let rows = if fy == 0 { h } else { h + 1 };
    let mut horizontal = [0u8; 17 * 16];
    for r in 0..rows {
        let row = &buf[r * bw..r * bw + bw];
        for i in 0..w {
            horizontal[r * w + i] = match fx {
                0 => row[i],
                _ => {
                    let half = lowpass(&|k| row[k] as i32, i, w, rounding);
                    match fx {
                        2 => half,
                        1 => ((row[i] as i32 + half as i32 + 1 - rounding) >> 1) as u8,
                        _ => ((row[i + 1] as i32 + half as i32 + 1 - rounding) >> 1) as u8,
                    }
                }
            };
        }
    }
    for j in 0..h {
        for i in 0..w {
            let v = match fy {
                0 => horizontal[j * w + i],
                _ => {
                    let half = lowpass(&|k| horizontal[k * w + i] as i32, j, h, rounding);
                    match fy {
                        2 => half,
                        1 => ((horizontal[j * w + i] as i32 + half as i32 + 1 - rounding) >> 1) as u8,
                        _ => ((horizontal[(j + 1) * w + i] as i32 + half as i32 + 1 - rounding) >> 1) as u8,
                    }
                }
            };
            out[j * out_stride + i] = v;
        }
    }
}

fn gmc1(p: &Plane, x: isize, y: isize, size: usize, offset: [i64; 2], accuracy: u32, rounding: i32, out: &mut [u8], out_stride: usize) {
    let (width, height) = (p.width as isize, p.height as isize);
    let mut motion = [offset[0] as isize, offset[1] as isize];
    let mut src_x = x + (motion[0] >> (accuracy + 1));
    let mut src_y = y + (motion[1] >> (accuracy + 1));
    motion[0] <<= 3 - accuracy;
    motion[1] <<= 3 - accuracy;
    src_x = src_x.clamp(-(size as isize), width);
    if src_x == width {
        motion[0] = 0;
    }
    src_y = src_y.clamp(-(size as isize), height);
    if src_y == height {
        motion[1] = 0;
    }
    if (motion[0] | motion[1]) & 7 == 0 {
        halfpel(p, src_x, src_y, size, size, ((motion[0] >> 3) & 1) as i32, ((motion[1] >> 3) & 1) as i32, rounding, out, out_stride);
        return;
    }
    let (fx, fy) = ((motion[0] & 15) as i32, (motion[1] & 15) as i32);
    let mut buf = [0u8; 17 * 17];
    let bw = size + 1;
    fetch(p, src_x, src_y, bw, size + 1, &mut buf);
    let (ka, kb, kc, kd) = ((16 - fx) * (16 - fy), fx * (16 - fy), (16 - fx) * fy, fx * fy);
    let round = 128 - rounding;
    for j in 0..size {
        for i in 0..size {
            let v = ka * buf[j * bw + i] as i32
                + kb * buf[j * bw + i + 1] as i32
                + kc * buf[(j + 1) * bw + i] as i32
                + kd * buf[(j + 1) * bw + i + 1] as i32
                + round;
            out[j * out_stride + i] = (v >> 8) as u8;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn gmc_warp(p: &Plane, size: usize, mut ox: i64, mut oy: i64, delta: [[i64; 2]; 2], shift: u32, rounder: i64, out: &mut [u8], out_stride: usize) {
    let s = 1i64 << shift;
    let width = p.width as i64 - 1;
    let height = p.height as i64 - 1;
    let at = |x: i64, y: i64| p.data[y as usize * p.stride + x as usize] as i64;
    for j in 0..size {
        let (mut vx, mut vy) = (ox, oy);
        for i in 0..size {
            let (mut src_x, mut src_y) = (vx >> 16, vy >> 16);
            let fx = src_x & (s - 1);
            let fy = src_y & (s - 1);
            src_x >>= shift;
            src_y >>= shift;
            let inside_x = src_x >= 0 && src_x < width;
            let inside_y = src_y >= 0 && src_y < height;
            let v = match (inside_x, inside_y) {
                (true, true) => {
                    ((at(src_x, src_y) * (s - fx) + at(src_x + 1, src_y) * fx) * (s - fy)
                        + (at(src_x, src_y + 1) * (s - fx) + at(src_x + 1, src_y + 1) * fx) * fy
                        + rounder)
                        >> (shift * 2)
                }
                (true, false) => {
                    let y = src_y.clamp(0, height);
                    ((at(src_x, y) * (s - fx) + at(src_x + 1, y) * fx) * s + rounder) >> (shift * 2)
                }
                (false, true) => {
                    let x = src_x.clamp(0, width);
                    ((at(x, src_y) * (s - fy) + at(x, src_y + 1) * fy) * s + rounder) >> (shift * 2)
                }
                (false, false) => at(src_x.clamp(0, width), src_y.clamp(0, height)),
            };
            out[j * out_stride + i] = v.clamp(0, 255) as u8;
            vx += delta[0][0];
            vy += delta[1][0];
        }
        ox += delta[0][1];
        oy += delta[1][1];
    }
}

struct Prediction {
    y: [u8; 256],
    u: [u8; 64],
    v: [u8; 64],
}

impl Prediction {
    fn new() -> Self {
        Prediction { y: [0; 256], u: [0; 64], v: [0; 64] }
    }

    fn average(&mut self, other: &Prediction) {
        for (a, b) in self.y.iter_mut().zip(&other.y) {
            *a = ((*a as u16 + *b as u16 + 1) >> 1) as u8;
        }
        for (a, b) in self.u.iter_mut().zip(&other.u) {
            *a = ((*a as u16 + *b as u16 + 1) >> 1) as u8;
        }
        for (a, b) in self.v.iter_mut().zip(&other.v) {
            *a = ((*a as u16 + *b as u16 + 1) >> 1) as u8;
        }
    }
}

pub struct Mpeg4Decoder {
    t: Tables,
    vol: Option<Vol>,
    past: Option<Frame>,
    future: Option<Frame>,
    spare: Option<Frame>,
    pending: bool,
    packed: bool,
    future_kinds: Vec<MbKind>,
    future_mvs: Vec<[i32; 2]>,
    time_base: i64,
    last_time_base: i64,
    last_non_b_time: i64,
    pp_time: i64,
    pb_time: i64,
    kinds: Vec<MbKind>,
    qps: Vec<i32>,
    mvs: Vec<[i32; 2]>,
    dc: Vec<i32>,
    ac: Vec<[i32; 14]>,
    packet_start: usize,
    last_b_mv: [[i32; 2]; 2],
}

fn unsupported(what: &str) -> String {
    format!("This MPEG-4 video uses {what}, which isn't supported yet.")
}

impl Mpeg4Decoder {
    pub fn new(config: &[u8]) -> Result<Self, String> {
        let mut decoder = Mpeg4Decoder {
            t: Tables::new(),
            vol: None,
            past: None,
            future: None,
            spare: None,
            pending: false,
            packed: false,
            future_kinds: Vec::new(),
            future_mvs: Vec::new(),
            time_base: 0,
            last_time_base: 0,
            last_non_b_time: 0,
            pp_time: 0,
            pb_time: 0,
            kinds: Vec::new(),
            qps: Vec::new(),
            mvs: Vec::new(),
            dc: Vec::new(),
            ac: Vec::new(),
            packet_start: 0,
            last_b_mv: [[0; 2]; 2],
        };
        if !config.is_empty() {
            let mut sink = Vec::new();
            decoder.decode(config, &mut sink)?;
        }
        Ok(decoder)
    }

    fn parse_vol(&mut self, br: &mut Bits) -> Result<(), String> {
        br.skip(1);
        br.skip(8);
        let mut verid = 1;
        if br.bit() {
            verid = br.get(4);
            br.skip(3);
        }
        if br.get(4) == 15 {
            br.skip(16);
        }
        if br.bit() {
            br.skip(2);
            br.skip(1);
            if br.bit() {
                br.skip(15 + 1 + 15 + 1 + 15 + 1 + 3 + 11 + 1 + 15 + 1);
            }
        }
        let shape = br.get(2);
        if shape != 0 {
            return Err(unsupported("non-rectangular shapes"));
        }
        br.skip(1);
        let time_res = br.get(16).max(1);
        br.skip(1);
        let time_bits = (32 - (time_res - 1).leading_zeros()).max(1);
        if br.bit() {
            br.skip(time_bits);
        }
        br.skip(1);
        let width = br.get(13) as usize;
        br.skip(1);
        let height = br.get(13) as usize;
        br.skip(1);
        if br.bit() {
            return Err(unsupported("interlaced coding"));
        }
        br.skip(1);
        let sprite = if verid == 1 { br.get(1) } else { br.get(2) };
        let (mut warp_points, mut warp_accuracy) = (0, 0);
        if sprite == 1 || sprite == 3 {
            return Err(unsupported("static sprites"));
        }
        if sprite == 2 {
            warp_points = br.get(6);
            warp_accuracy = br.get(2);
            if br.bit() {
                return Err(unsupported("sprite brightness changes"));
            }
            if warp_points > 3 {
                return Err(unsupported("more than three GMC warping points"));
            }
        }
        if br.bit() {
            if br.get(4) != 5 || br.get(4) != 8 {
                return Err(unsupported("a bit depth other than 8"));
            }
        }
        let mpeg_quant = br.bit();
        let mut intra_matrix = DEFAULT_INTRA_MATRIX;
        let mut inter_matrix = DEFAULT_INTER_MATRIX;
        if mpeg_quant {
            for matrix in [&mut intra_matrix, &mut inter_matrix] {
                if br.bit() {
                    let mut last = 0u16;
                    for i in 0..64 {
                        let v = br.get(8) as u16;
                        if v == 0 {
                            for j in i..64 {
                                matrix[ZIGZAG[j] as usize] = last;
                            }
                            break;
                        }
                        last = v;
                        matrix[ZIGZAG[i] as usize] = v;
                    }
                }
            }
        }
        let qpel = verid != 1 && br.bit();
        if !br.bit() {
            return Err(unsupported("complexity estimation headers"));
        }
        let resync = !br.bit();
        if br.bit() {
            return Err(unsupported("data partitioning"));
        }
        if verid != 1 {
            if br.bit() {
                return Err(unsupported("NEWPRED"));
            }
            if br.bit() {
                return Err(unsupported("reduced resolution VOPs"));
            }
        }
        if br.bit() {
            return Err(unsupported("scalability"));
        }
        if width == 0 || height == 0 || width > 8192 || height > 8192 {
            return Err("This MPEG-4 video has an invalid frame size.".to_string());
        }
        let vol = Vol {
            width,
            height,
            mb_w: width.div_ceil(16),
            mb_h: height.div_ceil(16),
            time_bits,
            time_res: time_res as i64,
            mpeg_quant,
            intra_matrix,
            inter_matrix,
            qpel,
            resync,
            warp_points,
            warp_accuracy,
            gmc: sprite == 2,
        };
        let size_changed = self.vol.as_ref().is_none_or(|old| old.mb_w != vol.mb_w || old.mb_h != vol.mb_h);
        if size_changed {
            let mbs = vol.mb_w * vol.mb_h;
            self.kinds = vec![MbKind::Skipped; mbs];
            self.qps = vec![1; mbs];
            self.mvs = vec![[0; 2]; mbs * 4];
            self.dc = vec![1024; mbs * 6];
            self.ac = vec![[0; 14]; mbs * 6];
            self.future_kinds = vec![MbKind::Skipped; mbs];
            self.future_mvs = vec![[0; 2]; mbs * 4];
            self.past = None;
            self.future = None;
            self.spare = None;
            self.pending = false;
        }
        self.vol = Some(vol);
        Ok(())
    }

    fn parse_vop_header(&mut self, br: &mut Bits, vol: &Vol) -> Option<Vop> {
        let coding_type = br.get(2);
        let kind = match coding_type {
            0 => VopType::I,
            2 => VopType::B,
            _ => VopType::P,
        };
        let sprite = coding_type == 3;
        if sprite && !vol.gmc {
            return None;
        }
        let mut modulo = 0i64;
        while br.bit() {
            modulo += 1;
            if br.overrun() {
                return None;
            }
        }
        br.skip(1);
        let increment = br.get(vol.time_bits) as i64;
        br.skip(1);
        let res = vol.time_res;
        if kind != VopType::B {
            self.last_time_base = self.time_base;
            self.time_base += modulo;
            let mut time = self.time_base * res + increment;
            if time < self.last_non_b_time {
                self.time_base += 1;
                time += res;
            }
            self.pp_time = time - self.last_non_b_time;
            self.last_non_b_time = time;
        } else {
            let time = (self.last_time_base + modulo) * res + increment;
            self.pb_time = self.pp_time - (self.last_non_b_time - time);
        }
        if !br.bit() {
            return Some(Vop { gmc: None, kind, rounding: -1, dc_threshold: 0, qp: 0, fcode_forward: 1, fcode_backward: 1 });
        }
        let rounding = if kind == VopType::P { br.get(1) as i32 } else { 0 };
        let dc_threshold = [99, 13, 15, 17, 19, 21, 23, 0][br.get(3) as usize];
        let gmc = if sprite { Some(self.sprite_trajectory(br, vol)?) } else { None };
        let qp = (br.get(5) as i32).max(1);
        let fcode_forward = if kind != VopType::I { br.get(3).max(1) } else { 1 };
        let fcode_backward = if kind == VopType::B { br.get(3).max(1) } else { 1 };
        Some(Vop { gmc, kind, rounding, dc_threshold, qp, fcode_forward, fcode_backward })
    }

    fn sprite_trajectory(&self, br: &mut Bits, vol: &Vol) -> Option<Gmc> {
        let mut d = [[0i64; 2]; 4];
        for point in d.iter_mut().take(vol.warp_points as usize) {
            for value in point.iter_mut() {
                let length = self.t.sprite_length.read(br)? as u32;
                if length > 0 {
                    let code = br.get(length) as i64;
                    *value = if code >> (length - 1) == 0 { code - (1 << length) + 1 } else { code };
                }
                br.skip(1);
            }
        }
        let accuracy = vol.warp_accuracy;
        let a = 2i64 << accuracy;
        let rho = 3 - accuracy as i64;
        let r = 16 / a;
        let (w, h) = (vol.width as i64, vol.height as i64);
        let mut alpha = 1i64;
        let mut beta = 0i64;
        while (1 << alpha) < w {
            alpha += 1;
        }
        while (1 << beta) < h {
            beta += 1;
        }
        let (w2, h2) = (1i64 << alpha, 1i64 << beta);
        let vop_ref = [[0i64, 0], [w, 0], [0, h]];
        let sprite_ref = [
            [(a >> 1) * (2 * vop_ref[0][0] + d[0][0]), (a >> 1) * (2 * vop_ref[0][1] + d[0][1])],
            [(a >> 1) * (2 * vop_ref[1][0] + d[0][0] + d[1][0]), (a >> 1) * (2 * vop_ref[1][1] + d[0][1] + d[1][1])],
            [(a >> 1) * (2 * vop_ref[2][0] + d[0][0] + d[2][0]), (a >> 1) * (2 * vop_ref[2][1] + d[0][1] + d[2][1])],
        ];
        let rdiv = |a: i64, b: i64| if a >= 0 { (a + b / 2) / b } else { (a - b / 2) / b };
        let virtual_ref = [
            [
                16 * (vop_ref[0][0] + w2)
                    + rdiv((w - w2) * (r * sprite_ref[0][0] - 16 * vop_ref[0][0]) + w2 * (r * sprite_ref[1][0] - 16 * vop_ref[1][0]), w),
                16 * vop_ref[0][1]
                    + rdiv((w - w2) * (r * sprite_ref[0][1] - 16 * vop_ref[0][1]) + w2 * (r * sprite_ref[1][1] - 16 * vop_ref[1][1]), w),
            ],
            [
                16 * vop_ref[0][0]
                    + rdiv((h - h2) * (r * sprite_ref[0][0] - 16 * vop_ref[0][0]) + h2 * (r * sprite_ref[2][0] - 16 * vop_ref[2][0]), h),
                16 * (vop_ref[0][1] + h2)
                    + rdiv((h - h2) * (r * sprite_ref[0][1] - 16 * vop_ref[0][1]) + h2 * (r * sprite_ref[2][1] - 16 * vop_ref[2][1]), h),
            ],
        ];
        let (sr, vr) = (sprite_ref, virtual_ref);
        let (mut offset, mut delta, mut shift);
        match vol.warp_points {
            0 => {
                offset = [[0i64; 2]; 2];
                delta = [[a, 0], [0, a]];
                shift = [0i64, 0];
            }
            1 => {
                offset = [
                    [sr[0][0] - a * vop_ref[0][0], sr[0][1] - a * vop_ref[0][1]],
                    [((sr[0][0] >> 1) | (sr[0][0] & 1)) - a * (vop_ref[0][0] / 2), ((sr[0][1] >> 1) | (sr[0][1] & 1)) - a * (vop_ref[0][1] / 2)],
                ];
                delta = [[a, 0], [0, a]];
                shift = [0, 0];
            }
            2 => {
                let ar = alpha + rho;
                offset = [
                    [
                        sr[0][0] * (1 << ar)
                            + (-r * sr[0][0] + vr[0][0]) * -vop_ref[0][0]
                            + (r * sr[0][1] - vr[0][1]) * -vop_ref[0][1]
                            + (1 << (ar - 1)),
                        sr[0][1] * (1 << ar)
                            + (-r * sr[0][1] + vr[0][1]) * -vop_ref[0][0]
                            + (-r * sr[0][0] + vr[0][0]) * -vop_ref[0][1]
                            + (1 << (ar - 1)),
                    ],
                    [
                        (-r * sr[0][0] + vr[0][0]) * (-2 * vop_ref[0][0] + 1)
                            + (r * sr[0][1] - vr[0][1]) * (-2 * vop_ref[0][1] + 1)
                            + 2 * w2 * r * sr[0][0]
                            - 16 * w2
                            + (1 << (ar + 1)),
                        (-r * sr[0][1] + vr[0][1]) * (-2 * vop_ref[0][0] + 1)
                            + (-r * sr[0][0] + vr[0][0]) * (-2 * vop_ref[0][1] + 1)
                            + 2 * w2 * r * sr[0][1]
                            - 16 * w2
                            + (1 << (ar + 1)),
                    ],
                ];
                delta = [
                    [-r * sr[0][0] + vr[0][0], r * sr[0][1] - vr[0][1]],
                    [-r * sr[0][1] + vr[0][1], -r * sr[0][0] + vr[0][0]],
                ];
                shift = [ar, ar + 2];
            }
            _ => {
                let min_ab = alpha.min(beta);
                let (w3, h3) = (w2 >> min_ab, h2 >> min_ab);
                let base = alpha + beta + rho - min_ab;
                offset = [
                    [
                        sr[0][0] * (1 << base)
                            + (-r * sr[0][0] + vr[0][0]) * h3 * -vop_ref[0][0]
                            + (-r * sr[0][0] + vr[1][0]) * w3 * -vop_ref[0][1]
                            + (1 << (base - 1)),
                        sr[0][1] * (1 << base)
                            + (-r * sr[0][1] + vr[0][1]) * h3 * -vop_ref[0][0]
                            + (-r * sr[0][1] + vr[1][1]) * w3 * -vop_ref[0][1]
                            + (1 << (base - 1)),
                    ],
                    [
                        (-r * sr[0][0] + vr[0][0]) * h3 * (-2 * vop_ref[0][0] + 1)
                            + (-r * sr[0][0] + vr[1][0]) * w3 * (-2 * vop_ref[0][1] + 1)
                            + 2 * w2 * h3 * r * sr[0][0]
                            - 16 * w2 * h3
                            + (1 << (base + 1)),
                        (-r * sr[0][1] + vr[0][1]) * h3 * (-2 * vop_ref[0][0] + 1)
                            + (-r * sr[0][1] + vr[1][1]) * w3 * (-2 * vop_ref[0][1] + 1)
                            + 2 * w2 * h3 * r * sr[0][1]
                            - 16 * w2 * h3
                            + (1 << (base + 1)),
                    ],
                ];
                delta = [
                    [(-r * sr[0][0] + vr[0][0]) * h3, (-r * sr[0][0] + vr[1][0]) * w3],
                    [(-r * sr[0][1] + vr[0][1]) * h3, (-r * sr[0][1] + vr[1][1]) * w3],
                ];
                shift = [base, base + 2];
            }
        }
        let points;
        if delta == [[a << shift[0], 0], [0, a << shift[0]]] {
            for i in 0..2 {
                offset[0][i] >>= shift[0];
                offset[1][i] >>= shift[1];
            }
            delta = [[a, 0], [0, a]];
            shift = [0, 0];
            points = 1;
        } else {
            let (shift_y, shift_c) = (16 - shift[0], 16 - shift[1]);
            if shift_y < 0 || shift_c < 0 {
                return None;
            }
            for i in 0..2 {
                offset[0][i] <<= shift_y;
                offset[1][i] <<= shift_c;
                delta[0][i] <<= shift_y;
                delta[1][i] <<= shift_y;
            }
            shift = [16, 16];
            points = vol.warp_points;
        }
        Some(Gmc { offset, delta, shift, points, accuracy })
    }

    pub fn decode(&mut self, data: &[u8], out: &mut Vec<Image>) -> Result<(), String> {
        let mut starts = Vec::new();
        let mut i = 0usize;
        while i + 3 < data.len() {
            if data[i] == 0 && data[i + 1] == 0 && data[i + 2] == 1 {
                starts.push(i);
                i += 4;
            } else {
                i += 1;
            }
        }
        let vops = starts.iter().filter(|&&s| data[s + 3] == START_CODE_VOP).count();
        if vops > 1 {
            self.packed = true;
        }
        for (k, &s) in starts.iter().enumerate() {
            let end = starts.get(k + 1).copied().unwrap_or(data.len());
            let code = data[s + 3];
            let payload = &data[s + 4..end];
            match code {
                START_CODE_VOL..=0x2F => self.parse_vol(&mut Bits::new(payload))?,
                START_CODE_USER_DATA => {
                    if payload.starts_with(b"DivX") && payload.last() == Some(&b'p') {
                        self.packed = true;
                    }
                }
                START_CODE_GOV => {
                    let mut br = Bits::new(payload);
                    let hours = br.get(5) as i64;
                    let minutes = br.get(6) as i64;
                    br.skip(1);
                    let seconds = br.get(6) as i64;
                    self.time_base = seconds + 60 * (minutes + 60 * hours);
                }
                START_CODE_VOP => self.decode_vop(payload, out)?,
                _ => {}
            }
        }
        Ok(())
    }

    pub fn finish(&mut self, out: &mut Vec<Image>) {
        if self.pending {
            if let (Some(frame), Some(vol)) = (&self.future, &self.vol) {
                out.push(image(frame, vol));
            }
            self.pending = false;
        }
    }

    fn take_frame(&mut self, vol: &Vol) -> Frame {
        self.spare.take().unwrap_or_else(|| Frame::new(vol.mb_w, vol.mb_h))
    }

    fn decode_vop(&mut self, payload: &[u8], out: &mut Vec<Image>) -> Result<(), String> {
        let Some(vol) = self.vol.clone() else { return Ok(()) };
        let mut br = Bits::new(payload);
        let Some(vop) = self.parse_vop_header(&mut br, &vol) else { return Ok(()) };

        if vop.rounding < 0 {
            if self.packed {
                return Ok(());
            }
            match vop.kind {
                VopType::B => {
                    if let Some(f) = self.past.as_ref().or(self.future.as_ref()) {
                        out.push(image(f, &vol));
                    }
                }
                _ => {
                    if let Some(future) = &self.future {
                        if self.pending {
                            out.push(image(future, &vol));
                        }
                        self.past = Some(future.clone());
                        self.future_kinds.fill(MbKind::Skipped);
                        self.future_mvs.fill([0; 2]);
                        self.pending = true;
                    }
                }
            }
            return Ok(());
        }

        if vop.kind != VopType::I && self.future.is_none() {
            return Ok(());
        }
        if vop.kind == VopType::B && self.past.is_none() {
            return Ok(());
        }

        let mut frame = self.take_frame(&vol);
        let result = match vop.kind {
            VopType::I => self.decode_intra_vop(&mut br, &vol, &vop, &mut frame),
            VopType::P => {
                let reference = self.future.take().unwrap();
                let r = self.decode_p_vop(&mut br, &vol, &vop, &reference, &mut frame);
                self.future = Some(reference);
                r
            }
            VopType::B => {
                let past = self.past.take().unwrap();
                let future = self.future.take().unwrap();
                let r = self.decode_b_vop(&mut br, &vol, &vop, &past, &future, &mut frame);
                self.past = Some(past);
                self.future = Some(future);
                r
            }
        };
        if let Err(failed_at) = result {
            self.conceal(&vol, &vop, failed_at, &mut frame);
        }

        if vop.kind == VopType::B {
            out.push(image(&frame, &vol));
            self.spare = Some(frame);
            return Ok(());
        }
        if self.pending {
            if let Some(future) = &self.future {
                out.push(image(future, &vol));
            }
        }
        self.spare = self.past.take();
        self.past = self.future.take();
        self.future = Some(frame);
        self.pending = true;
        std::mem::swap(&mut self.future_kinds, &mut self.kinds);
        std::mem::swap(&mut self.future_mvs, &mut self.mvs);
        if vop.kind == VopType::I {
            self.future_kinds.fill(MbKind::Intra);
            self.future_mvs.fill([0; 2]);
        }
        Ok(())
    }

    fn conceal(&mut self, vol: &Vol, vop: &Vop, from: usize, frame: &mut Frame) {
        let reference = if vop.kind == VopType::I { None } else { self.future.as_ref() };
        for index in from..vol.mb_w * vol.mb_h {
            let (x, y) = (index % vol.mb_w, index / vol.mb_w);
            if let Some(r) = reference {
                for j in 0..16 {
                    let o = (y * 16 + j) * frame.stride + x * 16;
                    frame.y[o..o + 16].copy_from_slice(&r.y[o..o + 16]);
                }
                for j in 0..8 {
                    let o = (y * 8 + j) * frame.cstride + x * 8;
                    frame.u[o..o + 8].copy_from_slice(&r.u[o..o + 8]);
                    frame.v[o..o + 8].copy_from_slice(&r.v[o..o + 8]);
                }
            }
            if vop.kind != VopType::B {
                self.kinds[index] = MbKind::Skipped;
                for b in 0..4 {
                    let g = self.grid(x, y, b);
                    self.mvs[g] = [0; 2];
                }
            }
        }
    }

    fn grid(&self, x: usize, y: usize, n: usize) -> usize {
        let w = self.vol.as_ref().map(|v| v.mb_w).unwrap_or(0) * 2;
        (2 * y + (n >> 1)) * w + 2 * x + (n & 1)
    }

    fn check_resync(&self, br: &Bits, prefix: u32, mb_total: usize) -> Option<(usize, usize)> {
        let n = 8 - (br.pos & 7) as u32;
        let stuffing = (1u32 << (n - 1)) - 1;
        if br.peek(n) != stuffing {
            return None;
        }
        let mut probe = Bits { data: br.data, pos: br.pos + n as usize };
        let mut zeros = 0;
        while zeros < 32 && !probe.bit() {
            zeros += 1;
        }
        if zeros < prefix || probe.overrun() {
            return None;
        }
        let bits = usize::BITS - (mb_total - 1).max(1).leading_zeros();
        let mb_num = probe.get(bits) as usize;
        if mb_num >= mb_total {
            return None;
        }
        Some((mb_num, probe.pos))
    }

    fn resync(&mut self, br: &mut Bits, vol: &Vol, vop: &Vop, index: usize, qp: &mut i32) -> bool {
        if !vol.resync || index == 0 {
            return true;
        }
        let prefix = match vop.kind {
            VopType::I => 16,
            VopType::P => 15 + vop.fcode_forward,
            VopType::B => 15 + vop.fcode_forward.max(vop.fcode_backward).max(2),
        };
        let mb_total = vol.mb_w * vol.mb_h;
        let Some((mb_num, pos)) = self.check_resync(br, prefix, mb_total) else { return true };
        if mb_num != index {
            return mb_num > index && vop.kind == VopType::B && self.future_kinds[index..mb_num].iter().all(|k| *k == MbKind::Skipped);
        }
        br.pos = pos;
        *qp = (br.get(5) as i32).max(1);
        if br.bit() {
            while br.bit() {
                if br.overrun() {
                    return false;
                }
            }
            br.skip(1);
            br.skip(vol.time_bits);
            br.skip(1);
            br.skip(2);
            br.skip(3);
            if vop.kind != VopType::I {
                br.skip(3);
            }
            if vop.kind == VopType::B {
                br.skip(3);
            }
        }
        self.packet_start = index;
        self.last_b_mv = [[0; 2]; 2];
        true
    }

    fn intra_available(&self, gx: isize, gy: isize, luma: bool, vol: &Vol, cur: usize) -> bool {
        let (mw, mh) = if luma { (vol.mb_w as isize * 2, vol.mb_h as isize * 2) } else { (vol.mb_w as isize, vol.mb_h as isize) };
        if gx < 0 || gy < 0 || gx >= mw || gy >= mh {
            return false;
        }
        let (mx, my) = if luma { (gx >> 1, gy >> 1) } else { (gx, gy) };
        let index = my as usize * vol.mb_w + mx as usize;
        index >= self.packet_start && index <= cur && self.kinds[index] == MbKind::Intra
    }

    fn store_index(&self, gx: usize, gy: usize, n: usize, vol: &Vol) -> usize {
        let luma = vol.mb_w * vol.mb_h * 4;
        match n {
            0..=3 => gy * vol.mb_w * 2 + gx,
            4 => luma + gy * vol.mb_w + gx,
            _ => luma + vol.mb_w * vol.mb_h + gy * vol.mb_w + gx,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn intra_block(
        &mut self,
        br: &mut Bits,
        vol: &Vol,
        n: usize,
        x: usize,
        y: usize,
        cur: usize,
        qp: i32,
        use_dc_vlc: bool,
        ac_pred: bool,
        coded: bool,
        block: &mut [i32; 64],
    ) -> Option<()> {
        let luma = n < 4;
        let (gx, gy) = if luma { (2 * x + (n & 1), 2 * y + (n >> 1)) } else { (x, y) };
        let (gxi, gyi) = (gx as isize, gy as isize);
        let left = self.intra_available(gxi - 1, gyi, luma, vol, cur).then(|| self.store_index(gx - 1, gy, n, vol));
        let top_left = self.intra_available(gxi - 1, gyi - 1, luma, vol, cur).then(|| self.store_index(gx - 1, gy - 1, n, vol));
        let top = self.intra_available(gxi, gyi - 1, luma, vol, cur).then(|| self.store_index(gx, gy - 1, n, vol));
        let a = left.map(|i| self.dc[i]).unwrap_or(1024);
        let b = top_left.map(|i| self.dc[i]).unwrap_or(1024);
        let c = top.map(|i| self.dc[i]).unwrap_or(1024);
        let from_top = (a - b).abs() < (b - c).abs();
        let pred = if from_top { c } else { a };
        let scale = dc_scale(qp, luma);

        block.fill(0);
        let mut start = 0usize;
        if use_dc_vlc {
            let size = if luma { self.t.dc_luma.read(br)? } else { self.t.dc_chroma.read(br)? } as u32;
            let mut diff = 0i32;
            if size > 0 {
                let code = br.get(size) as i32;
                diff = if code >> (size - 1) == 0 { code - (1 << size) + 1 } else { code };
                if size > 8 {
                    br.skip(1);
                }
            }
            block[0] = diff;
            start = 1;
        }
        let scan = if ac_pred {
            if from_top { &ALT_HORIZONTAL } else { &ALT_VERTICAL }
        } else {
            &ZIGZAG
        };
        if coded {
            let mut i = start;
            loop {
                let (last, run, level) = self.t.intra.read(br)?;
                i += run;
                if i >= 64 {
                    return None;
                }
                block[scan[i] as usize] = level;
                i += 1;
                if last {
                    break;
                }
            }
        }
        block[0] += (pred + scale / 2) / scale;

        if ac_pred {
            if from_top {
                if let Some(t) = top {
                    let mb = if luma { (gy - 1) / 2 * vol.mb_w + gx / 2 } else { (gy - 1) * vol.mb_w + gx };
                    let qp_t = self.qps[mb];
                    for i in 1..8 {
                        let v = self.ac[t][i - 1];
                        block[i] += if qp_t == qp { v } else { rounded_div(v * qp_t, qp) };
                    }
                }
            } else if let Some(l) = left {
                let mb = if luma { gy / 2 * vol.mb_w + (gx - 1) / 2 } else { gy * vol.mb_w + gx - 1 };
                let qp_l = self.qps[mb];
                for i in 1..8 {
                    let v = self.ac[l][7 + i - 1];
                    block[i * 8] += if qp_l == qp { v } else { rounded_div(v * qp_l, qp) };
                }
            }
        }

        let store = self.store_index(gx, gy, n, vol);
        self.dc[store] = (block[0] * scale).clamp(0, 2047);
        let mut saved = [0i32; 14];
        for i in 1..8 {
            saved[i - 1] = block[i];
            saved[7 + i - 1] = block[i * 8];
        }
        self.ac[store] = saved;

        block[0] *= scale;
        if vol.mpeg_quant {
            for i in 1..64 {
                let l = block[i];
                if l != 0 {
                    let v = (l.abs() * qp * 2 * vol.intra_matrix[i] as i32) >> 4;
                    block[i] = if l < 0 { -v } else { v }.clamp(-2048, 2047);
                }
            }
        } else {
            let add = if qp & 1 == 1 { qp } else { qp - 1 };
            for v in block.iter_mut().skip(1) {
                if *v != 0 {
                    let m = 2 * qp * v.abs() + add;
                    *v = if *v < 0 { -m } else { m }.clamp(-2048, 2047);
                }
            }
        }
        Some(())
    }

    fn inter_block(&self, br: &mut Bits, vol: &Vol, qp: i32, block: &mut [i32; 64]) -> Option<()> {
        block.fill(0);
        let mut i = 0usize;
        loop {
            let (last, run, level) = self.t.inter.read(br)?;
            i += run;
            if i >= 64 {
                return None;
            }
            block[ZIGZAG[i] as usize] = level;
            i += 1;
            if last {
                break;
            }
        }
        if vol.mpeg_quant {
            let mut sum = -1i32;
            for (i, v) in block.iter_mut().enumerate() {
                if *v != 0 {
                    let m = ((2 * v.abs() + 1) * qp * vol.inter_matrix[i] as i32) >> 4;
                    *v = if *v < 0 { -m } else { m }.clamp(-2048, 2047);
                    sum += *v;
                }
            }
            block[63] ^= sum & 1;
        } else {
            let add = if qp & 1 == 1 { qp } else { qp - 1 };
            for v in block.iter_mut() {
                if *v != 0 {
                    let m = 2 * qp * v.abs() + add;
                    *v = if *v < 0 { -m } else { m }.clamp(-2048, 2047);
                }
            }
        }
        Some(())
    }

    fn put_block(&self, frame: &mut Frame, n: usize, x: usize, y: usize, block: &mut [i32; 64], add: bool) {
        self.t.idct(block);
        let (plane, stride, ox, oy) = match n {
            0..=3 => (&mut frame.y, frame.stride, x * 16 + (n & 1) * 8, y * 16 + (n >> 1) * 8),
            4 => (&mut frame.u, frame.cstride, x * 8, y * 8),
            _ => (&mut frame.v, frame.cstride, x * 8, y * 8),
        };
        for j in 0..8 {
            let row = &mut plane[(oy + j) * stride + ox..(oy + j) * stride + ox + 8];
            for i in 0..8 {
                let v = block[j * 8 + i];
                row[i] = if add { (row[i] as i32 + v).clamp(0, 255) } else { v.clamp(0, 255) } as u8;
            }
        }
    }

    fn decode_intra_vop(&mut self, br: &mut Bits, vol: &Vol, vop: &Vop, frame: &mut Frame) -> Result<(), usize> {
        let mut qp = vop.qp;
        self.packet_start = 0;
        let mut block = [0i32; 64];
        for index in 0..vol.mb_w * vol.mb_h {
            let (x, y) = (index % vol.mb_w, index / vol.mb_w);
            if !self.resync(br, vol, vop, index, &mut qp) {
                return Err(index);
            }
            let mcbpc = loop {
                match self.t.intra_mcbpc.read(br) {
                    Some(STUFFING) => continue,
                    Some(v) => break v,
                    None => return Err(index),
                }
            };
            let ac_pred = br.bit();
            let cbpy = self.t.cbpy.read(br).ok_or(index)? as u32;
            let use_dc_vlc = qp < vop.dc_threshold;
            if mcbpc >= 4 {
                qp = (qp + [-1, -2, 1, 2][br.get(2) as usize]).clamp(1, 31);
            }
            let cbp = (cbpy << 2) | (mcbpc as u32 & 3);
            self.kinds[index] = MbKind::Intra;
            self.qps[index] = qp;
            for n in 0..6 {
                self.intra_block(br, vol, n, x, y, index, qp, use_dc_vlc, ac_pred, cbp & (32 >> n) != 0, &mut block).ok_or(index)?;
                self.put_block(frame, n, x, y, &mut block, false);
            }
            for n in 0..4 {
                let g = self.grid(x, y, n);
                self.mvs[g] = [0; 2];
            }
            if br.overrun() {
                return Err(index);
            }
        }
        Ok(())
    }

    fn mv_valid(&self, gx: isize, gy: isize, vol: &Vol, cur: usize) -> Option<[i32; 2]> {
        if gx < 0 || gy < 0 || gx >= vol.mb_w as isize * 2 || gy >= vol.mb_h as isize * 2 {
            return None;
        }
        let index = (gy as usize >> 1) * vol.mb_w + (gx as usize >> 1);
        (index >= self.packet_start && index <= cur).then(|| self.mvs[gy as usize * vol.mb_w * 2 + gx as usize])
    }

    fn mv_predict(&self, vol: &Vol, x: usize, y: usize, n: usize, cur: usize) -> [i32; 2] {
        let gx = (2 * x + (n & 1)) as isize;
        let gy = (2 * y + (n >> 1)) as isize;
        let c_off = [2, 1, 1, -1][n];
        let candidates = [self.mv_valid(gx - 1, gy, vol, cur), self.mv_valid(gx, gy - 1, vol, cur), self.mv_valid(gx + c_off, gy - 1, vol, cur)];
        let valid: Vec<[i32; 2]> = candidates.iter().flatten().copied().collect();
        match valid.len() {
            0 => [0, 0],
            1 => valid[0],
            _ => {
                let pick = |c: Option<[i32; 2]>| c.unwrap_or([0, 0]);
                let (a, b, c) = (pick(candidates[0]), pick(candidates[1]), pick(candidates[2]));
                [median(a[0], b[0], c[0]), median(a[1], b[1], c[1])]
            }
        }
    }

    fn read_mv_component(&self, br: &mut Bits, pred: i32, fcode: u32) -> Option<i32> {
        let code = self.t.mvd.read(br)? as i32;
        if code == 0 {
            return Some(pred);
        }
        let negative = br.bit();
        let shift = fcode - 1;
        let mut diff = code;
        if shift > 0 {
            diff = ((code - 1) << shift) + br.get(shift) as i32 + 1;
        }
        if negative {
            diff = -diff;
        }
        let bits = 5 + fcode;
        let v = pred + diff;
        Some((v << (32 - bits)) >> (32 - bits))
    }

    fn read_mv(&self, br: &mut Bits, pred: [i32; 2], fcode: u32) -> Option<[i32; 2]> {
        let x = self.read_mv_component(br, pred[0], fcode)?;
        let y = self.read_mv_component(br, pred[1], fcode)?;
        Some([x, y])
    }

    #[allow(clippy::too_many_arguments)]
    fn predict(&self, vol: &Vol, reference: &Frame, x: usize, y: usize, mvs: &[[i32; 2]; 4], four: bool, rounding: i32, out: &mut Prediction) {
        let luma = Plane { data: &reference.y, stride: reference.stride, width: vol.width, height: vol.height };
        let (bx, by) = ((x * 16) as isize, (y * 16) as isize);
        let luma_mc = |p: &Plane, px: isize, py: isize, size: usize, mv: [i32; 2], out: &mut [u8]| {
            if vol.qpel {
                qpel(p, px, py, size, size, mv[0], mv[1], rounding, out, 16);
            } else {
                halfpel(p, px, py, size, size, mv[0], mv[1], rounding, out, 16);
            }
        };
        let (mut sx, mut sy);
        if four {
            (sx, sy) = (0, 0);
            for (n, mv) in mvs.iter().enumerate() {
                let (ox, oy) = ((n & 1) * 8, (n >> 1) * 8);
                luma_mc(&luma, bx + ox as isize, by + oy as isize, 8, *mv, &mut out.y[oy * 16 + ox..]);
                if vol.qpel {
                    sx += mv[0] / 2;
                    sy += mv[1] / 2;
                } else {
                    sx += mv[0];
                    sy += mv[1];
                }
            }
        } else {
            luma_mc(&luma, bx, by, 16, mvs[0], &mut out.y);
            let (mx, my) = if vol.qpel { (mvs[0][0] / 2, mvs[0][1] / 2) } else { (mvs[0][0], mvs[0][1]) };
            (sx, sy) = (mx * 4, my * 4);
        }
        let (cx, cy) = (chroma_round(sx), chroma_round(sy));
        let (cw, ch) = (vol.width >> 1, vol.height >> 1);
        let u = Plane { data: &reference.u, stride: reference.cstride, width: cw, height: ch };
        let v = Plane { data: &reference.v, stride: reference.cstride, width: cw, height: ch };
        let (ux, uy) = ((x * 8) as isize, (y * 8) as isize);
        halfpel(&u, ux, uy, 8, 8, cx, cy, rounding, &mut out.u, 8);
        halfpel(&v, ux, uy, 8, 8, cx, cy, rounding, &mut out.v, 8);
    }

    fn predict_gmc(&self, vol: &Vol, reference: &Frame, x: usize, y: usize, g: &Gmc, rounding: i32, out: &mut Prediction) {
        let luma = Plane { data: &reference.y, stride: reference.stride, width: vol.width, height: vol.height };
        let (cw, ch) = if g.points == 1 { (vol.width >> 1, vol.height >> 1) } else { ((vol.width + 1) >> 1, (vol.height + 1) >> 1) };
        let u = Plane { data: &reference.u, stride: reference.cstride, width: cw, height: ch };
        let v = Plane { data: &reference.v, stride: reference.cstride, width: cw, height: ch };
        if g.points == 1 {
            gmc1(&luma, (x * 16) as isize, (y * 16) as isize, 16, g.offset[0], g.accuracy, rounding, &mut out.y, 16);
            gmc1(&u, (x * 8) as isize, (y * 8) as isize, 8, g.offset[1], g.accuracy, rounding, &mut out.u, 8);
            gmc1(&v, (x * 8) as isize, (y * 8) as isize, 8, g.offset[1], g.accuracy, rounding, &mut out.v, 8);
            return;
        }
        let a = g.accuracy;
        let shift = a + 1;
        let rounder = (1i64 << (2 * a + 1)) - rounding as i64;
        let (mx, my) = (x as i64, y as i64);
        let ox = g.offset[0][0] + g.delta[0][0] * mx * 16 + g.delta[0][1] * my * 16;
        let oy = g.offset[0][1] + g.delta[1][0] * mx * 16 + g.delta[1][1] * my * 16;
        gmc_warp(&luma, 16, ox, oy, g.delta, shift, rounder, &mut out.y, 16);
        let ox = g.offset[1][0] + g.delta[0][0] * mx * 8 + g.delta[0][1] * my * 8;
        let oy = g.offset[1][1] + g.delta[1][0] * mx * 8 + g.delta[1][1] * my * 8;
        gmc_warp(&u, 8, ox, oy, g.delta, shift, rounder, &mut out.u, 8);
        gmc_warp(&v, 8, ox, oy, g.delta, shift, rounder, &mut out.v, 8);
    }

    fn average_mv(vol: &Vol, g: &Gmc, x: usize, y: usize, fcode: u32) -> [i32; 2] {
        let len = 1i64 << (fcode + 4);
        let a = g.accuracy as i64;
        let q = vol.qpel as i64;
        let rshift = |v: i64, b: i64| if v > 0 { (v + ((1 << b) >> 1)) >> b } else { (v + ((1 << b) >> 1) - 1) >> b };
        let mut mv = [0i32; 2];
        for (n, out) in mv.iter_mut().enumerate() {
            let sum = if g.points == 1 {
                rshift(g.offset[0][n] * (1 << q), a)
            } else {
                let mut dx = g.delta[n][0];
                let mut dy = g.delta[n][1];
                let shift = g.shift[0];
                if n == 1 {
                    dy -= 1 << (shift + a + 1);
                } else {
                    dx -= 1 << (shift + a + 1);
                }
                let base = g.offset[0][n] + dx * x as i64 * 16 + dy * y as i64 * 16;
                let mut sum = 0i64;
                for j in 0..16 {
                    let mut v = base + dy * j;
                    for _ in 0..16 {
                        sum += v >> shift;
                        v += dx;
                    }
                }
                rshift(sum, a + 8 - q)
            };
            *out = sum.clamp(-len, len - 1) as i32;
        }
        mv
    }

    fn write_prediction(frame: &mut Frame, x: usize, y: usize, p: &Prediction) {
        for j in 0..16 {
            let o = (y * 16 + j) * frame.stride + x * 16;
            frame.y[o..o + 16].copy_from_slice(&p.y[j * 16..j * 16 + 16]);
        }
        for j in 0..8 {
            let o = (y * 8 + j) * frame.cstride + x * 8;
            frame.u[o..o + 8].copy_from_slice(&p.u[j * 8..j * 8 + 8]);
            frame.v[o..o + 8].copy_from_slice(&p.v[j * 8..j * 8 + 8]);
        }
    }

    fn decode_p_vop(&mut self, br: &mut Bits, vol: &Vol, vop: &Vop, reference: &Frame, frame: &mut Frame) -> Result<(), usize> {
        let mut qp = vop.qp;
        self.packet_start = 0;
        let mut block = [0i32; 64];
        let mut pred = Prediction::new();
        for index in 0..vol.mb_w * vol.mb_h {
            let (x, y) = (index % vol.mb_w, index / vol.mb_w);
            if !self.resync(br, vol, vop, index, &mut qp) {
                return Err(index);
            }
            let mcbpc = loop {
                if br.bit() {
                    break -1;
                }
                match self.t.inter_mcbpc.read(br) {
                    Some(STUFFING) => continue,
                    Some(v) => break v,
                    None => return Err(index),
                }
            };
            if mcbpc < 0 {
                self.qps[index] = qp;
                let mv = match &vop.gmc {
                    Some(g) => {
                        self.kinds[index] = MbKind::Inter;
                        self.predict_gmc(vol, reference, x, y, g, vop.rounding, &mut pred);
                        Self::average_mv(vol, g, x, y, vop.fcode_forward)
                    }
                    None => {
                        self.kinds[index] = MbKind::Skipped;
                        self.predict(vol, reference, x, y, &[[0; 2]; 4], false, vop.rounding, &mut pred);
                        [0; 2]
                    }
                };
                for n in 0..4 {
                    let g = self.grid(x, y, n);
                    self.mvs[g] = mv;
                }
                Self::write_prediction(frame, x, y, &pred);
                continue;
            }
            let mb_type = mcbpc / 4;
            let intra = mb_type >= 3;
            let mcsel = vop.gmc.is_some() && mb_type < 2 && br.bit();
            let ac_pred = intra && br.bit();
            let mut cbpy = self.t.cbpy.read(br).ok_or(index)? as u32;
            if !intra {
                cbpy ^= 15;
            }
            let use_dc_vlc = qp < vop.dc_threshold;
            if mb_type == 1 || mb_type == 4 {
                qp = (qp + [-1, -2, 1, 2][br.get(2) as usize]).clamp(1, 31);
            }
            let cbp = (cbpy << 2) | (mcbpc as u32 & 3);
            self.qps[index] = qp;
            if intra {
                self.kinds[index] = MbKind::Intra;
                for n in 0..4 {
                    let g = self.grid(x, y, n);
                    self.mvs[g] = [0; 2];
                }
                for n in 0..6 {
                    self.intra_block(br, vol, n, x, y, index, qp, use_dc_vlc, ac_pred, cbp & (32 >> n) != 0, &mut block).ok_or(index)?;
                    self.put_block(frame, n, x, y, &mut block, false);
                }
                continue;
            }
            let four = mb_type == 2;
            self.kinds[index] = if four { MbKind::Inter4v } else { MbKind::Inter };
            let mut mvs = [[0i32; 2]; 4];
            if let (true, Some(g)) = (mcsel, &vop.gmc) {
                let mv = Self::average_mv(vol, g, x, y, vop.fcode_forward);
                for n in 0..4 {
                    let gi = self.grid(x, y, n);
                    self.mvs[gi] = mv;
                }
                self.predict_gmc(vol, reference, x, y, g, vop.rounding, &mut pred);
            } else if four {
                for (n, mv) in mvs.iter_mut().enumerate() {
                    let p = self.mv_predict(vol, x, y, n, index);
                    *mv = self.read_mv(br, p, vop.fcode_forward).ok_or(index)?;
                    let g = self.grid(x, y, n);
                    self.mvs[g] = *mv;
                }
            } else {
                let p = self.mv_predict(vol, x, y, 0, index);
                let mv = self.read_mv(br, p, vop.fcode_forward).ok_or(index)?;
                mvs = [mv; 4];
                for n in 0..4 {
                    let g = self.grid(x, y, n);
                    self.mvs[g] = mv;
                }
            }
            if !mcsel {
                self.predict(vol, reference, x, y, &mvs, four, vop.rounding, &mut pred);
            }
            Self::write_prediction(frame, x, y, &pred);
            for n in 0..6 {
                if cbp & (32 >> n) != 0 {
                    self.inter_block(br, vol, qp, &mut block).ok_or(index)?;
                    self.put_block(frame, n, x, y, &mut block, true);
                }
            }
            if br.overrun() {
                return Err(index);
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn decode_b_vop(&mut self, br: &mut Bits, vol: &Vol, vop: &Vop, past: &Frame, future: &Frame, frame: &mut Frame) -> Result<(), usize> {
        let mut qp = vop.qp;
        self.packet_start = 0;
        let mut block = [0i32; 64];
        let mut forward = Prediction::new();
        let mut backward = Prediction::new();
        let (mut trb, mut trd) = (self.pb_time, self.pp_time);
        if trd <= 0 || trb <= 0 || trb >= trd {
            (trb, trd) = (1, 2);
        }
        for index in 0..vol.mb_w * vol.mb_h {
            let (x, y) = (index % vol.mb_w, index / vol.mb_w);
            if x == 0 {
                self.last_b_mv = [[0; 2]; 2];
            }
            if !self.resync(br, vol, vop, index, &mut qp) {
                return Err(index);
            }
            if self.future_kinds[index] == MbKind::Skipped {
                self.predict(vol, past, x, y, &[[0; 2]; 4], false, 0, &mut forward);
                Self::write_prediction(frame, x, y, &forward);
                continue;
            }
            let mut mb_type = 0i16;
            let mut cbp = 0u32;
            let direct_skip = br.bit();
            if !direct_skip {
                let has_cbp = !br.bit();
                mb_type = self.t.b_mb_type.read(br).ok_or(index)?;
                if has_cbp {
                    cbp = br.get(6);
                }
                if mb_type != 0 && cbp != 0 && br.bit() {
                    qp = (qp + if br.bit() { 2 } else { -2 }).clamp(1, 31);
                }
            }
            match mb_type {
                0 => {
                    let delta = if direct_skip { [0, 0] } else { self.read_mv(br, [0, 0], 1).ok_or(index)? };
                    let four = vol.qpel || self.future_kinds[index] == MbKind::Inter4v;
                    let mut fwd = [[0i32; 2]; 4];
                    let mut bwd = [[0i32; 2]; 4];
                    for n in 0..4 {
                        let co = self.future_mvs[self.grid(x, y, n)];
                        for c in 0..2 {
                            let m = co[c] as i64;
                            let f = (trb * m / trd) as i32 + delta[c];
                            fwd[n][c] = f;
                            bwd[n][c] = if delta[c] != 0 { f - co[c] } else { ((trb - trd) * m / trd) as i32 };
                        }
                    }
                    self.predict(vol, past, x, y, &fwd, four, 0, &mut forward);
                    self.predict(vol, future, x, y, &bwd, four, 0, &mut backward);
                    forward.average(&backward);
                    Self::write_prediction(frame, x, y, &forward);
                }
                _ => {
                    let use_forward = mb_type == 1 || mb_type == 3;
                    let use_backward = mb_type == 1 || mb_type == 2;
                    if use_forward {
                        let mv = self.read_mv(br, self.last_b_mv[0], vop.fcode_forward).ok_or(index)?;
                        self.last_b_mv[0] = mv;
                        self.predict(vol, past, x, y, &[mv; 4], false, 0, &mut forward);
                    }
                    if use_backward {
                        let mv = self.read_mv(br, self.last_b_mv[1], vop.fcode_backward).ok_or(index)?;
                        self.last_b_mv[1] = mv;
                        self.predict(vol, future, x, y, &[mv; 4], false, 0, &mut backward);
                    }
                    if use_forward && use_backward {
                        forward.average(&backward);
                        Self::write_prediction(frame, x, y, &forward);
                    } else if use_forward {
                        Self::write_prediction(frame, x, y, &forward);
                    } else {
                        Self::write_prediction(frame, x, y, &backward);
                    }
                }
            }
            for n in 0..6 {
                if cbp & (32 >> n) != 0 {
                    self.inter_block(br, vol, qp, &mut block).ok_or(index)?;
                    self.put_block(frame, n, x, y, &mut block, true);
                }
            }
            if br.overrun() || br.pos > br.total() {
                return Err(index);
            }
        }
        Ok(())
    }
}

fn image(frame: &Frame, vol: &Vol) -> Image {
    let w = vol.width & !1;
    let h = vol.height & !1;
    let mut i420 = Vec::with_capacity(w * h * 3 / 2);
    for row in 0..h {
        i420.extend_from_slice(&frame.y[row * frame.stride..row * frame.stride + w]);
    }
    for plane in [&frame.u, &frame.v] {
        for row in 0..h / 2 {
            i420.extend_from_slice(&plane[row * frame.cstride..row * frame.cstride + w / 2]);
        }
    }
    Image { width: w, height: h, i420 }
}