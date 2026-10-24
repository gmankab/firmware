// SPDX-License-Identifier: GPL-3.0-only
//! Checked, partially reconstructed Cirrus WMFW v3 HALO firmware layout.
//!
//! Only independently authored container headers and region descriptors are
//! provided here. DSP XM/YM/PM payloads and algorithm descriptions remain
//! intentionally absent and must never be filled from the reference image.

use crate::image_plan::ImagePlan;
use crate::wmfw_metadata::{build_wmfw_header, WmfwRegionDescriptor, WMFW_REGIONS};
use crate::{parse_wmfw, wmfw_type_name};

pub const FILE_SIZE: usize = 34_236;
pub const HEADER_SIZE: usize = 40;
pub const DESCRIPTOR_SIZE: usize = 8;
pub const REGION_COUNT: usize = 11;
pub const INDEPENDENT_METADATA_BYTES: usize = HEADER_SIZE + REGION_COUNT * DESCRIPTOR_SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WmfwLayoutRegion {
    pub file_header_offset: usize,
    pub payload_offset: usize,
    pub kind: u8,
    pub dsp_address: u32,
    pub payload_length: usize,
    pub word_count: Option<usize>,
}

impl WmfwLayoutRegion {
    pub fn end(&self) -> usize { self.payload_offset + self.payload_length }

    pub fn name(&self) -> &'static str { wmfw_type_name(self.kind) }
}

pub fn regions() -> Result<Vec<WmfwLayoutRegion>, String> {
    let mut cursor = HEADER_SIZE;
    let mut regions = Vec::with_capacity(WMFW_REGIONS.len());
    for WmfwRegionDescriptor { file_offset, kind, dsp_address, payload_size } in WMFW_REGIONS {
        if *file_offset != cursor {
            return Err(format!("WMFW descriptor gap/overlap: expected 0x{cursor:x}, got 0x{file_offset:x}"));
        }
        let length = usize::try_from(*payload_size).map_err(|_| "payload too large")?;
        let payload_offset = cursor.checked_add(DESCRIPTOR_SIZE).ok_or("payload offset overflow")?;
        cursor = payload_offset.checked_add(length).ok_or("payload length overflow")?;
        if cursor > FILE_SIZE { return Err(format!("WMFW region exceeds file at 0x{cursor:x}")); }
        // Cirrus HALO packed instruction words use 5 bytes. XM/YM packed
        // DSP data carries 4 bytes per address, but that is NOT a claim that
        // every 4-byte group is an independent HALO instruction.
        let bytes_per_word = match *kind {
            0x10 => Some(5usize),
            0x11 | 0x12 => Some(4usize),
            _ => None,
        };
        let word_count = match bytes_per_word {
            Some(n) if length % n == 0 => Some(length / n),
            Some(n) => return Err(format!("WMFW kind 0x{kind:02x} payload is not divisible by {n}")),
            None => None,
        };
        regions.push(WmfwLayoutRegion {
            file_header_offset: *file_offset, payload_offset, kind: *kind,
            dsp_address: *dsp_address, payload_length: length, word_count,
        });
    }
    if cursor != FILE_SIZE {
        return Err(format!("WMFW ended at 0x{cursor:x}, not 0x{FILE_SIZE:x}"));
    }
    Ok(regions)
}

/// Rebuild only the 128 independently authored metadata bytes. None of the
/// proprietary payload bytes is imported, silently supplied or zero-filled.
pub fn partial_container() -> Result<ImagePlan, String> {
    let mut plan = ImagePlan::new(FILE_SIZE)?;
    plan.place("Cirrus HALO WMFW v3 header", 0, &build_wmfw_header())?;
    for r in WMFW_REGIONS {
        plan.place(&format!("region 0x{:02x} descriptor @0x{:x}", r.kind, r.file_offset),
            r.file_offset, &r.encode())?;
    }
    assert_eq!(plan.filled_bytes(), INDEPENDENT_METADATA_BYTES);
    assert_eq!(plan.holes().len(), REGION_COUNT);
    Ok(plan)
}

/// Verify exact structure and original file length. Payload contents are
/// deliberately not tested against independently authored source here.
pub fn verify_container(reference: &[u8]) -> Result<(), String> {
    if reference.len() != FILE_SIZE {
        return Err(format!("WMFW reference file length {} != {}", reference.len(), FILE_SIZE));
    }
    let parsed = parse_wmfw(reference)?;
    if parsed.revision != 0x300 || parsed.core != 4 || parsed.version != 3 {
        return Err("incorrect HALO WMFW v3 variant".into());
    }
    let known = regions()?;
    if parsed.region_headers.len() != known.len() {
        return Err(format!("WMFW count {} differs from expected {}", parsed.region_headers.len(), known.len()));
    }
    for (actual, expected) in parsed.region_headers.iter().zip(&known) {
        if actual.file_offset != expected.file_header_offset ||
           actual.region_type != expected.kind ||
           actual.dsp_offset != expected.dsp_address ||
           actual.payload_length != expected.payload_length {
            return Err(format!("WMFW region descriptor differs at 0x{:x}", expected.file_header_offset));
        }
    }
    partial_container()?.compare_known(reference)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_reference() -> Vec<u8> {
        let mut reference = vec![0xa5; FILE_SIZE];
        reference[..HEADER_SIZE].copy_from_slice(&build_wmfw_header());
        for region in WMFW_REGIONS {
            reference[region.file_offset..region.file_offset+DESCRIPTOR_SIZE]
                .copy_from_slice(&region.encode());
        }
        reference
    }

    #[test]
    fn isolated_metadata_and_holes() {
        let rs = regions().unwrap();
        assert_eq!(rs.len(), REGION_COUNT);
        assert_eq!(rs[7].kind, 0x10);
        assert_eq!(rs[7].word_count, Some(4_056));
        assert_eq!(rs[6].word_count, Some(1_533));
        assert_eq!(rs[0].name(), "INFO_TEXT");
        let plan = partial_container().unwrap();
        assert_eq!(plan.filled_bytes(), 128);
        assert_eq!(plan.total_bytes(), FILE_SIZE);
        assert_eq!(plan.holes().len(), 11);
        assert_eq!(plan.holes().first().unwrap(), &(48..96));
        assert_eq!(plan.holes().last().unwrap(), &(0x82c4..0x85bc));
        assert!(plan.build_complete().unwrap_err().contains("incomplete"));
    }

    #[test]
    fn reference_is_never_mistaken_for_independent_payload() {
        let mut reference = synthetic_reference();
        verify_container(&reference).unwrap();
        // Unknown payloads can differ: only metadata is an independently
        // generated result at this checkpoint.
        reference[0x278c] ^= 0xff;
        verify_container(&reference).unwrap();
        reference[0x76c4] ^= 0x80;
        assert!(verify_container(&reference).is_err());
        reference[0x76c4] ^= 0x80;
        reference.push(0);
        assert!(verify_container(&reference).unwrap_err().contains("length"));
    }
}
