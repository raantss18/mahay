//! A minimal ICC v4 display profile writer (matrix/TRC: colorants, white point, chromatic
//! adaptation and parametric sRGB curves), built from the ICC.1 specification. Used to tag
//! exported images with the document's colour profile.

use crate::profile::{Mat3, apply, inverse, mul, xy_to_xyz};

/// The ICC profile connection space illuminant (D50).
const PCS_D50: [f64; 3] = [0.9642, 1.0, 0.8249];

/// The Bradford cone response matrix (for chromatic adaptation).
const BRADFORD: Mat3 = [[0.8951, 0.2664, -0.1614], [-0.7502, 1.7135, 0.0367], [0.0389, -0.0685, 1.0296]];

/// sRGB transfer curve as ICC parametric curve type 3: g, a, b, c, d.
const SRGB_CURVE: [f64; 5] = [2.4, 1.0 / 1.055, 0.055 / 1.055, 1.0 / 12.92, 0.04045];

/// A big-endian byte writer.
#[derive(Default)]
struct Be(Vec<u8>);

impl Be {
    fn u16(&mut self, v: u16) -> &mut Self {
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }
    fn u32(&mut self, v: u32) -> &mut Self {
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }
    fn sig(&mut self, s: &[u8; 4]) -> &mut Self {
        self.0.extend_from_slice(s);
        self
    }
    /// An `s15Fixed16Number`.
    fn fixed(&mut self, v: f64) -> &mut Self {
        let v = if v.is_finite() { (v * 65536.0).round().clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32 } else { 0 };
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }
    fn zeros(&mut self, n: usize) -> &mut Self {
        self.0.resize(self.0.len() + n, 0);
        self
    }
}

/// `multiLocalizedUnicodeType` with one en-US record.
fn mluc(text: &str) -> Vec<u8> {
    let utf16: Vec<u8> = text.encode_utf16().flat_map(u16::to_be_bytes).collect();
    let mut b = Be::default();
    b.sig(b"mluc").zeros(4).u32(1).u32(12).sig(b"enUS").u32(utf16.len() as u32).u32(28);
    b.0.extend_from_slice(&utf16);
    b.0
}

fn xyz(v: [f64; 3]) -> Vec<u8> {
    let mut b = Be::default();
    b.sig(b"XYZ ").zeros(4).fixed(v[0]).fixed(v[1]).fixed(v[2]);
    b.0
}

/// The Bradford matrix adapting XYZ under `src` white to XYZ under `dst` white.
pub(crate) fn bradford(src: [f64; 3], dst: [f64; 3]) -> Mat3 {
    let (s, d) = (apply(&BRADFORD, src), apply(&BRADFORD, dst));
    let ratio = |i: usize| if s[i].abs() < 1e-12 { 1.0 } else { d[i] / s[i] };
    let scale = [[ratio(0), 0.0, 0.0], [0.0, ratio(1), 0.0], [0.0, 0.0, ratio(2)]];
    let inv = inverse(&BRADFORD).unwrap_or(scale);
    mul(&inv, &mul(&scale, &BRADFORD))
}

/// A matrix/TRC RGB display profile: `to_xyz` maps linear RGB to XYZ under `white`; every channel
/// uses the sRGB curve.
pub(crate) fn matrix_trc_profile(description: &str, to_xyz: &Mat3, white: (f64, f64)) -> Vec<u8> {
    let chad = bradford(xy_to_xyz(white), PCS_D50);
    let colorants = mul(&chad, to_xyz);
    let column = |j: usize| [colorants[0][j], colorants[1][j], colorants[2][j]];
    let mut sf32 = Be::default();
    sf32.sig(b"sf32").zeros(4);
    for row in chad {
        for v in row {
            sf32.fixed(v);
        }
    }
    let mut para = Be::default();
    para.sig(b"para").zeros(4).u16(3).u16(0);
    for v in SRGB_CURVE {
        para.fixed(v);
    }
    let tags: [(&[u8; 4], Vec<u8>); 10] = [
        (b"desc", mluc(description)),
        (b"cprt", mluc("No copyright, use freely")),
        (b"wtpt", xyz(PCS_D50)),
        (b"chad", sf32.0),
        (b"rXYZ", xyz(column(0))),
        (b"gXYZ", xyz(column(1))),
        (b"bXYZ", xyz(column(2))),
        (b"rTRC", para.0.clone()),
        (b"gTRC", para.0.clone()),
        (b"bTRC", para.0),
    ];
    // Tag data follows the header and the tag table, each element 4-byte aligned.
    let mut offset = 128 + 4 + 12 * tags.len();
    let mut table = Be::default();
    table.u32(tags.len() as u32);
    let mut data = Vec::new();
    for (sig, bytes) in &tags {
        table.sig(sig).u32(offset as u32).u32(bytes.len() as u32);
        let padded = bytes.len().div_ceil(4) * 4;
        data.extend_from_slice(bytes);
        data.resize(data.len() + padded - bytes.len(), 0);
        offset += padded;
    }
    let mut h = Be::default();
    h.u32(offset as u32) // profile size
        .zeros(4) // preferred CMM
        .u32(0x0430_0000) // version 4.3
        .sig(b"mntr")
        .sig(b"RGB ")
        .sig(b"XYZ ")
        .u16(2026)
        .u16(1)
        .u16(1)
        .zeros(6) // creation date and time
        .sig(b"acsp")
        .zeros(4 + 4 + 4 + 4 + 8) // platform, flags, manufacturer, model, attributes
        .u32(0) // perceptual rendering intent
        .fixed(PCS_D50[0])
        .fixed(PCS_D50[1])
        .fixed(PCS_D50[2])
        .zeros(4 + 16 + 28); // creator, profile id (not computed), reserved
    let mut out = h.0;
    out.extend_from_slice(&table.0);
    out.extend_from_slice(&data);
    out
}
