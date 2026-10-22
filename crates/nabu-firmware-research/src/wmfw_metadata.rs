// SPDX-License-Identifier: GPL-3.0-only
//! Rebuilds Cirrus WMFW **container metadata only**, not HALO DSP code or data.
//! The schema follows Linux's public CS DSP loader and contains no region payloads.

#[derive(Debug, Clone, Copy)]
pub struct WmfwRegionDescriptor {
    pub file_offset: usize,
    pub kind: u8,
    pub dsp_address: u32,
    pub payload_size: u32,
}

pub const WMFW_REGIONS: &[WmfwRegionDescriptor] = &[
    WmfwRegionDescriptor { file_offset: 0x28, kind: 0xff, dsp_address: 0, payload_size: 48 },
    WmfwRegionDescriptor { file_offset: 0x60, kind: 0x11, dsp_address: 0, payload_size: 72 },
    WmfwRegionDescriptor { file_offset: 0xb0, kind: 0x11, dsp_address: 0x34, payload_size: 528 },
    WmfwRegionDescriptor { file_offset: 0x2c8, kind: 0x11, dsp_address: 0x140, payload_size: 1584 },
    WmfwRegionDescriptor { file_offset: 0x900, kind: 0x11, dsp_address: 0x354, payload_size: 1284 },
    WmfwRegionDescriptor { file_offset: 0xe0c, kind: 0x11, dsp_address: 0x530, payload_size: 372 },
    WmfwRegionDescriptor { file_offset: 0xf88, kind: 0x12, dsp_address: 0, payload_size: 6132 },
    WmfwRegionDescriptor { file_offset: 0x2784, kind: 0x10, dsp_address: 0, payload_size: 20280 },
    WmfwRegionDescriptor { file_offset: 0x76c4, kind: 0xf2, dsp_address: 0, payload_size: 948 },
    WmfwRegionDescriptor { file_offset: 0x7a80, kind: 0xf2, dsp_address: 0, payload_size: 2100 },
    WmfwRegionDescriptor { file_offset: 0x82bc, kind: 0xf2, dsp_address: 0, payload_size: 760 },
];

/// The 40-byte WMFW header: magic, format, four DSP memory sizes,
/// 64-bit build timestamp, 32-bit reserved field.
pub fn build_wmfw_header() -> [u8;40] {
    let mut b = [0u8;40];
    b[..4].copy_from_slice(b"WMFW");
    b[4..8].copy_from_slice(&40u32.to_le_bytes());
    b[8..10].copy_from_slice(&0x300u16.to_le_bytes());
    b[10]=4; // HALO
    b[11]=3; // WMFW v3
    for (i, size) in [0x5acu32, 0x7fc, 0xfd8, 0].iter().enumerate() {
        b[12+i*4..16+i*4].copy_from_slice(&size.to_le_bytes());
    }
    b[28..36].copy_from_slice(&0x60676a96u64.to_le_bytes());
    // Last 4 bytes are reserved, zero-initialized.
    b
}

impl WmfwRegionDescriptor {
    pub fn encode(self) -> [u8; 8] {
        let mut b = [0u8;8];
        let descriptor = ((self.kind as u32) << 24) | (self.dsp_address & 0x00ff_ffff);
        b[..4].copy_from_slice(&descriptor.to_le_bytes());
        b[4..].copy_from_slice(&self.payload_size.to_le_bytes());
        b
    }
}

/// Verifies *only* the metadata-byte fields against a locally supplied
/// proprietary input. Region payloads are deliberately not compared here.
pub fn verify_wmfw_structure(original: &[u8]) -> Result<usize, String> {
    if original.get(..40) != Some(&build_wmfw_header()[..]) {
        return Err("WMFW 40-byte header mismatch".into());
    }
    let mut matched = 40;
    for descriptor in WMFW_REGIONS {
        let expected = descriptor.encode();
        let offset = descriptor.file_offset;
        if original.get(offset..offset+8) != Some(&expected[..]) {
            return Err(format!("WMFW region descriptor at 0x{offset:x} mismatch"));
        }
        matched += 8;
    }
    Ok(matched)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metadata_layout_is_self_consistent() {
        assert_eq!(&build_wmfw_header()[..4], b"WMFW");
        assert_eq!(WMFW_REGIONS.len(), 11);
        let mut previous_end=40;
        for r in WMFW_REGIONS {
            assert_eq!(r.file_offset, previous_end);
            previous_end = r.file_offset + 8 + r.payload_size as usize;
        }
        assert_eq!(previous_end, 34236);
    }
}
