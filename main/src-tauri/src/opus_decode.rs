use opus_rs::OpusDecoder;

const RATE: i32 = 48000;
const MAX_FRAME: usize = 5760;

pub fn packet_samples(packet: &[u8]) -> usize {
    let Some(&toc) = packet.first() else { return 0 };
    let config = toc >> 3;
    let per_frame = match config {
        0..=11 => [480, 960, 1920, 2880][(config % 4) as usize],
        12..=15 => [480, 960][(config % 2) as usize],
        _ => [120, 240, 480, 960][(config % 4) as usize],
    };
    let frames = match toc & 0x03 {
        0 => 1,
        1 | 2 => 2,
        _ => packet.get(1).map(|b| (b & 0x3F) as usize).unwrap_or(1),
    };
    (per_frame * frames).min(MAX_FRAME)
}

fn new_decoder(channels: usize) -> Result<OpusDecoder, String> {
    OpusDecoder::new(RATE, channels).map_err(|e| format!("Couldn't create the Opus decoder: {e}"))
}

struct Elementary {
    channels: usize,
    mono: Option<OpusDecoder>,
    stereo: Option<OpusDecoder>,
    buf: Vec<f32>,
}

impl Elementary {
    fn new(channels: usize) -> Self {
        Elementary { channels, mono: None, stereo: None, buf: vec![0.0; MAX_FRAME * 2] }
    }

    fn decode(&mut self, packet: &[u8], frame: usize, out: &mut Vec<f32>) -> Result<(), String> {
        let packet_channels = if packet.first().is_some_and(|toc| toc & 0x04 != 0) { 2 } else { 1 };
        let slot = if packet_channels == 2 { &mut self.stereo } else { &mut self.mono };
        if slot.is_none() {
            *slot = Some(new_decoder(packet_channels)?);
        }
        let decoder = slot.as_mut().unwrap();
        let got = decoder.decode(packet, frame, &mut self.buf).map_err(|e| format!("Opus decode error: {e}"))?;
        let decoded = &self.buf[..got * packet_channels];
        out.clear();
        match (packet_channels, self.channels) {
            (1, 2) => decoded.iter().for_each(|&s| out.extend_from_slice(&[s, s])),
            (2, 1) => out.extend(decoded.chunks_exact(2).map(|pair| (pair[0] + pair[1]) * 0.5)),
            _ => out.extend_from_slice(decoded),
        }
        Ok(())
    }
}

fn read_size(data: &[u8], p: &mut usize) -> Option<usize> {
    let first = *data.get(*p)? as usize;
    if first < 252 {
        *p += 1;
        Some(first)
    } else {
        let second = *data.get(*p + 1)? as usize;
        *p += 2;
        Some(first + 4 * second)
    }
}

fn split_self_delimited(data: &[u8]) -> Option<(Vec<u8>, usize)> {
    let toc = *data.first()?;
    let mut p = 1usize;
    let (field_start, field_end, payload) = match toc & 0x03 {
        0 => {
            let len = read_size(data, &mut p)?;
            (1, p, len)
        }
        1 => {
            let len = read_size(data, &mut p)?;
            (1, p, len * 2)
        }
        2 => {
            let first = read_size(data, &mut p)?;
            let start = p;
            let second = read_size(data, &mut p)?;
            (start, p, first + second)
        }
        _ => {
            let count = *data.get(p)?;
            p += 1;
            let frames = (count & 0x3F) as usize;
            let mut padding = 0usize;
            if count & 0x40 != 0 {
                loop {
                    let value = *data.get(p)? as usize;
                    p += 1;
                    if value == 255 {
                        padding += 254;
                    } else {
                        padding += value;
                        break;
                    }
                }
            }
            if count & 0x80 != 0 {
                let mut total = 0usize;
                for _ in 1..frames {
                    total += read_size(data, &mut p)?;
                }
                let start = p;
                total += read_size(data, &mut p)?;
                (start, p, total + padding)
            } else {
                let start = p;
                let len = read_size(data, &mut p)?;
                (start, p, len * frames + padding)
            }
        }
    };
    let end = field_end.checked_add(payload)?;
    let body = data.get(field_end..end)?;
    let mut packet = data[..field_start].to_vec();
    packet.extend_from_slice(body);
    Some((packet, end))
}

pub struct OpusStreamDecoder {
    streams: Vec<Elementary>,
    coupled: usize,
    mapping: Vec<u8>,
    family: u8,
    pre_skip: u16,
    decoded: Vec<Vec<f32>>,
}

impl OpusStreamDecoder {
    pub fn from_head(head: &[u8]) -> Result<Self, String> {
        if head.len() < 19 || &head[0..8] != b"OpusHead" {
            return Err("This Opus audio has an invalid header.".to_string());
        }
        let channels = head[9] as usize;
        let pre_skip = u16::from_le_bytes([head[10], head[11]]);
        let family = head[18];
        if channels == 0 {
            return Err("This Opus audio has no channels.".to_string());
        }
        let (stream_count, coupled, mapping) = if family == 0 {
            if channels > 2 {
                return Err("This Opus audio has an invalid channel layout.".to_string());
            }
            (1, channels - 1, (0..channels as u8).collect::<Vec<_>>())
        } else {
            let stream_count = *head.get(19).ok_or("This Opus audio has an invalid channel layout.")? as usize;
            let coupled = *head.get(20).ok_or("This Opus audio has an invalid channel layout.")? as usize;
            let mapping = head.get(21..21 + channels).ok_or("This Opus audio has an invalid channel layout.")?.to_vec();
            if stream_count == 0 || coupled > stream_count {
                return Err("This Opus audio has an invalid channel layout.".to_string());
            }
            (stream_count, coupled, mapping)
        };
        let streams = (0..stream_count).map(|i| Elementary::new(if i < coupled { 2 } else { 1 })).collect();
        Ok(OpusStreamDecoder { streams, coupled, mapping, family, pre_skip, decoded: vec![Vec::new(); stream_count] })
    }

    pub fn from_channels(channels: u16) -> Result<Self, String> {
        let mut head = b"OpusHead".to_vec();
        head.push(1);
        head.push(channels.clamp(1, 2) as u8);
        head.extend_from_slice(&[0; 9]);
        Self::from_head(&head)
    }

    pub fn output_channels(&self) -> u16 {
        self.mapping.len().min(2) as u16
    }

    pub fn pre_skip(&self) -> u16 {
        self.pre_skip
    }

    fn channel_sample(&self, channel: usize, i: usize) -> f32 {
        let index = self.mapping[channel] as usize;
        if index == 255 {
            return 0.0;
        }
        let (stream, offset, width) = if index < 2 * self.coupled {
            (index / 2, index % 2, 2)
        } else {
            (self.coupled + index - 2 * self.coupled, 0, 1)
        };
        self.decoded.get(stream).and_then(|d| d.get(i * width + offset)).copied().unwrap_or(0.0)
    }

    pub fn decode(&mut self, packet: &[u8], out: &mut Vec<f32>) -> Result<(), String> {
        if packet.is_empty() {
            return Ok(());
        }
        let mut rest = packet;
        let mut frame = 0usize;
        let stream_count = self.streams.len();
        for s in 0..stream_count {
            let (standard, used) = if s + 1 < stream_count {
                split_self_delimited(rest).ok_or("This Opus packet is damaged.")?
            } else {
                (rest.to_vec(), rest.len())
            };
            rest = &rest[used..];
            if s == 0 {
                frame = packet_samples(&standard);
            }
            let mut decoded = std::mem::take(&mut self.decoded[s]);
            let result = self.streams[s].decode(&standard, frame, &mut decoded);
            self.decoded[s] = decoded;
            result?;
        }

        let channels = self.mapping.len();
        if channels <= 2 {
            for i in 0..frame {
                for c in 0..channels {
                    out.push(self.channel_sample(c, i));
                }
            }
            return Ok(());
        }

        const H: f32 = std::f32::consts::FRAC_1_SQRT_2;
        let weights: &[(f32, f32)] = match (self.family, channels) {
            (1, 3) => &[(1.0, 0.0), (H, H), (0.0, 1.0)],
            (1, 4) => &[(1.0, 0.0), (0.0, 1.0), (H, 0.0), (0.0, H)],
            (1, 5) => &[(1.0, 0.0), (H, H), (0.0, 1.0), (H, 0.0), (0.0, H)],
            (1, 6) => &[(1.0, 0.0), (H, H), (0.0, 1.0), (H, 0.0), (0.0, H), (0.0, 0.0)],
            (1, 7) => &[(1.0, 0.0), (H, H), (0.0, 1.0), (H, 0.0), (0.0, H), (0.5, 0.5), (0.0, 0.0)],
            (1, 8) => &[(1.0, 0.0), (H, H), (0.0, 1.0), (H, 0.0), (0.0, H), (H, 0.0), (0.0, H), (0.0, 0.0)],
            _ => &[(1.0, 0.0), (0.0, 1.0)],
        };
        let left_total: f32 = weights.iter().map(|w| w.0).sum();
        let right_total: f32 = weights.iter().map(|w| w.1).sum();
        for i in 0..frame {
            let (mut left, mut right) = (0.0f32, 0.0f32);
            for (c, &(wl, wr)) in weights.iter().enumerate() {
                let v = self.channel_sample(c, i);
                left += v * wl;
                right += v * wr;
            }
            out.push((left / left_total).clamp(-1.0, 1.0));
            out.push((right / right_total).clamp(-1.0, 1.0));
        }
        Ok(())
    }
}