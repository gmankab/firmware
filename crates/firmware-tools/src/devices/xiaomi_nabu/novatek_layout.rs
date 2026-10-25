// SPDX-License-Identifier: GPL-3.0-only
//! Strict, reversible *metadata* representation of the Nabu NT36xxx-style
//! touchscreen firmware. It does not infer the unconfirmed MCU instruction set
//! and never copies an executable payload into generated source.
//!
//! Nabu's 0x200-byte prefix contains two 0x100-byte descriptor banks. They
//! share the ILM/DLM descriptors, while their variant-specific info entries
//! and trailer/checksum records differ. The upstream NT36xxx Linux driver
//! documents ILM/DLM and the 16-byte `sram:size:file:crc` info entries, but
//! its generic "parse every entry up to header_length" rule would mistake
//! the second vendor-specific bank for a single flat list. We deliberately
//! expose only the structure actually verified in both Nabu revisions.

pub const IMAGE_LENGTH: usize = 0x30000;
pub const HEADER_BYTES: usize = 0x200;
pub const BANK_BYTES: usize = 0x100;
/// Nabu's observed all-zero reserved tail; this is specific to the pinned
/// 196,608-byte firmware layout, not a general Novatek-family assumption.
pub const RESERVED_TAIL_START: usize = 0x2f900;
pub const END_MARKER_OFFSET: usize = IMAGE_LENGTH - 3;
pub const TRAILER_MARKER: &[u8; 3] = b"NVT";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorePartition {
    pub file_offset: u32,
    pub sram_address: u32,
    /// Format stores `length - 1`; this is NOT a byte count.
    pub size_minus_one: u32,
    /// Stored controller hardware CRC. The CRC algorithm is still unknown.
    pub stored_crc: u32,
}

impl CorePartition {
    pub fn actual_length(&self) -> Result<usize, String> {
        let size = (self.size_minus_one as u64) + 1;
        usize::try_from(size).map_err(|_| "partition length exceeds usize".to_string())
    }
    pub fn end(&self) -> Result<usize, String> {
        (self.file_offset as usize)
            .checked_add(self.actual_length()?)
            .ok_or_else(|| "partition end overflows usize".into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoDescriptor {
    pub sram_address: u32,
    pub size_minus_one: u32,
    pub file_offset: u32,
    pub stored_crc: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorBank {
    /// Vendor-specific mode field at relative offset 0x20.
    pub mode: u32,
    /// Additional vendor-specific control word at relative offset 0x24.
    pub options: u32,
    /// Three populated info table entries at relative offsets 0x30..0x5f.
    pub info: [InfoDescriptor; 3],
    /// Special auxiliary metadata descriptor at offset 0xf0, not executable.
    pub auxiliary: InfoDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NabuNovatekLayout {
    pub ilm: CorePartition,
    pub dlm: CorePartition,
    pub banks: [DescriptorBank; 2],
    pub image_len: usize,
}


/// Classify provenance, not only whether bytes happen to match a reference.
/// Metadata is serializable but depends on opaque vendor-provided CRC fields;
/// zero regions and the literal suffix are independently constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    StructuredMetadataWithOpaqueCrc,
    UnimplementedIlm,
    IndependentlyGeneratedZero,
    UnimplementedDlm,
    UnimplementedAuxiliary,
    IndependentlyGeneratedMarker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverageRegion {
    pub name: &'static str,
    pub start: usize,
    pub end: usize,
    pub kind: RegionKind,
}

impl CoverageRegion {
    pub fn length(&self) -> usize { self.end - self.start }
    pub fn source_independent(&self) -> bool {
        matches!(self.kind, RegionKind::IndependentlyGeneratedZero
                  | RegionKind::IndependentlyGeneratedMarker)
    }
    pub fn is_unimplemented(&self) -> bool {
        matches!(self.kind, RegionKind::UnimplementedIlm
                  | RegionKind::UnimplementedDlm
                  | RegionKind::UnimplementedAuxiliary)
    }
}

/// Verify complete *classification*, not complete executable reconstruction:
/// every byte of the input has exactly one declared provenance category.
fn validate_coverage(regions: &[CoverageRegion], size: usize) -> Result<(), String> {
    let mut cursor = 0;
    for region in regions {
        if region.end <= region.start || region.start != cursor || region.end > size {
            return Err(format!("invalid/overlapping firmware coverage at 0x{:x}", region.start));
        }
        cursor = region.end;
    }
    if cursor != size { return Err(format!("unclassified firmware gap at 0x{cursor:x}")); }
    Ok(())
}

fn u32_at(b: &[u8], offset: usize) -> Result<u32, String> {
    let bytes = b.get(offset..offset + 4)
        .ok_or_else(|| format!("missing 32-bit word at offset 0x{offset:x}"))?;
    Ok(u32::from_le_bytes(bytes.try_into().expect("exact slice length")))
}
fn set_u32(b: &mut [u8], offset: usize, v: u32) {
    b[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
}
fn info_at(b: &[u8], offset: usize) -> Result<InfoDescriptor, String> {
    Ok(InfoDescriptor {
        sram_address: u32_at(b, offset)?,
        size_minus_one: u32_at(b, offset + 4)?,
        file_offset: u32_at(b, offset + 8)?,
        stored_crc: u32_at(b, offset + 12)?,
    })
}
fn set_info(b: &mut [u8], offset: usize, info: &InfoDescriptor) {
    for (i, value) in [info.sram_address, info.size_minus_one,
                       info.file_offset, info.stored_crc].into_iter().enumerate() {
        set_u32(b, offset + 4 * i, value);
    }
}
fn require_zero(b: &[u8], start: usize, end: usize) -> Result<(), String> {
    if let Some(i) = (start..end).find(|&i| b[i] != 0) {
        return Err(format!("expected reserved zero at file offset 0x{i:x}"));
    }
    Ok(())
}

impl NabuNovatekLayout {
    /// Parse the exact two-bank Nabu variant, rejecting ambiguous unsupported
    /// layouts instead of guessing that all 29 nominal info entries are live.
    pub fn parse(firmware: &[u8]) -> Result<Self, String> {
        if firmware.len() != IMAGE_LENGTH {
            return Err(format!("Nabu NT36523 reference must be 0x{IMAGE_LENGTH:x} bytes"));
        }
        if !firmware.ends_with(TRAILER_MARKER) {
            return Err("missing 3-byte NVT end marker".into());
        }
        if u32_at(firmware, 0)? as usize != HEADER_BYTES {
            return Err("unsupported Novatek metadata prefix size".into());
        }
        if firmware.get(..0x20) != firmware.get(BANK_BYTES..BANK_BYTES + 0x20) {
            return Err("the two Nabu banks disagree on ILM/DLM metadata".into());
        }
        let ilm = CorePartition {
            file_offset: u32_at(firmware, 0)?,
            sram_address: u32_at(firmware, 4)?,
            size_minus_one: u32_at(firmware, 8)?,
            stored_crc: u32_at(firmware, 0x18)?,
        };
        let dlm = CorePartition {
            file_offset: u32_at(firmware, 0x0c)?,
            sram_address: u32_at(firmware, 0x10)?,
            size_minus_one: u32_at(firmware, 0x14)?,
            stored_crc: u32_at(firmware, 0x1c)?,
        };
        if ilm.file_offset as usize != HEADER_BYTES || ilm.actual_length()? == 0 {
            return Err("ILM does not start immediately after Nabu metadata".into());
        }
        if ilm.end()? > dlm.file_offset as usize || dlm.end()? > firmware.len() {
            return Err("overlapping or out-of-bounds ILM/DLM payloads".into());
        }
        let mut banks = Vec::with_capacity(2);
        for base in [0, BANK_BYTES] {
            require_zero(firmware, base + 0x28, base + 0x30)?;
            require_zero(firmware, base + 0x60, base + 0xf0)?;
            banks.push(DescriptorBank {
                mode: u32_at(firmware, base + 0x20)?,
                options: u32_at(firmware, base + 0x24)?,
                info: [info_at(firmware, base + 0x30)?,
                       info_at(firmware, base + 0x40)?,
                       info_at(firmware, base + 0x50)?],
                auxiliary: info_at(firmware, base + 0xf0)?,
            });
        }
        let result = Self { ilm, dlm,
            banks: [banks[0], banks[1]], image_len: firmware.len() };
        // This is a *layout* equality check, not CRC verification. CRC values
        // are parsed opaque words whose calculation is not yet established.
        if result.write_header() != firmware[..HEADER_BYTES] {
            return Err("Novatek header is not representable by the two-bank layout".into());
        }
        Ok(result)
    }

    /// Write exactly 512 bytes from field values, without reading any vendor
    /// image, making this routine suitable for replacing future source tables.
    /// The 10 CRC fields are opaque metadata supplied by the caller.
    pub fn write_header(&self) -> [u8; HEADER_BYTES] {
        let mut bytes = [0u8; HEADER_BYTES];
        for base in [0, BANK_BYTES] {
            set_u32(&mut bytes, base, self.ilm.file_offset);
            set_u32(&mut bytes, base + 4, self.ilm.sram_address);
            set_u32(&mut bytes, base + 8, self.ilm.size_minus_one);
            set_u32(&mut bytes, base + 12, self.dlm.file_offset);
            set_u32(&mut bytes, base + 16, self.dlm.sram_address);
            set_u32(&mut bytes, base + 20, self.dlm.size_minus_one);
            set_u32(&mut bytes, base + 24, self.ilm.stored_crc);
            set_u32(&mut bytes, base + 28, self.dlm.stored_crc);
        }
        for (bank, base) in self.banks.iter().zip([0, BANK_BYTES]) {
            set_u32(&mut bytes, base + 0x20, bank.mode);
            set_u32(&mut bytes, base + 0x24, bank.options);
            for (i, info) in bank.info.iter().enumerate() {
                set_info(&mut bytes, base + 0x30 + i*16, info);
            }
            set_info(&mut bytes, base + 0xf0, &bank.auxiliary);
        }
        bytes
    }

    /// Empty payload spacer between ILM and DLM. This method emits zeros,
    /// never copies OEM payload bytes, and refuses nonzero reference gaps.
    pub fn write_padding(&self, reference: &[u8]) -> Result<Vec<u8>, String> {
        if reference.len() != self.image_len { return Err("different image length".into()); }
        let start = self.ilm.end()?;
        let end = self.dlm.file_offset as usize;
        require_zero(reference, start, end)?;
        Ok(vec![0;end - start])
    }

    pub fn padding_range(&self) -> Result<(usize, usize), String> {
        Ok((self.ilm.end()?, self.dlm.file_offset as usize))
    }

    /// Observed reserved gap after the trailer-bearing area, independently
    /// generated as zeros. Refuse nonzero bytes rather than quietly deleting
    /// undocumented controller data from a different firmware revision.
    pub fn write_reserved_tail(&self, reference: &[u8]) -> Result<Vec<u8>, String> {
        if reference.len() != self.image_len { return Err("different image length".into()); }
        require_zero(reference, RESERVED_TAIL_START, END_MARKER_OFFSET)?;
        Ok(vec![0; END_MARKER_OFFSET - RESERVED_TAIL_START])
    }

    /// The three-byte fixed end marker can be emitted without firmware input.
    pub fn write_marker(&self) -> [u8; 3] { *TRAILER_MARKER }

    /// A complete, gapless provenance map. The unknown regions remain marked
    /// unknown even when the reference happens to have incidental zero bytes.
    pub fn coverage(&self) -> Result<Vec<CoverageRegion>, String> {
        let ilm_end = self.ilm.end()?;
        let dlm_begin = self.dlm.file_offset as usize;
        let dlm_end = self.dlm.end()?;
        let regions = vec![
            CoverageRegion { name: "dual-bank metadata (opaque CRCs)",
                start: 0, end: HEADER_BYTES,
                kind: RegionKind::StructuredMetadataWithOpaqueCrc },
            CoverageRegion { name: "ILM instruction/data", start: HEADER_BYTES,
                end: ilm_end, kind: RegionKind::UnimplementedIlm },
            CoverageRegion { name: "ILM–DLM gap", start: ilm_end,
                end: dlm_begin, kind: RegionKind::IndependentlyGeneratedZero },
            CoverageRegion { name: "DLM instruction/data", start: dlm_begin,
                end: dlm_end, kind: RegionKind::UnimplementedDlm },
            CoverageRegion { name: "auxiliary calibration/trailer", start: dlm_end,
                end: RESERVED_TAIL_START, kind: RegionKind::UnimplementedAuxiliary },
            CoverageRegion { name: "reserved end padding", start: RESERVED_TAIL_START,
                end: END_MARKER_OFFSET, kind: RegionKind::IndependentlyGeneratedZero },
            CoverageRegion { name: "NVT magic suffix", start: END_MARKER_OFFSET,
                end: IMAGE_LENGTH, kind: RegionKind::IndependentlyGeneratedMarker },
        ];
        validate_coverage(&regions, self.image_len)?;
        Ok(regions)
    }

    /// Verify *only* independently reconstructible ranges plus metadata
    /// serialization. This says nothing about unknown executable bytes.
    pub fn verify_envelope(&self, reference: &[u8]) -> Result<(), String> {
        if reference.len() != self.image_len { return Err("different image length".into()); }
        if self.write_header().as_slice() != &reference[..HEADER_BYTES] {
            return Err("metadata does not match fields".into());
        }
        let (start, end) = self.padding_range()?;
        if self.write_padding(reference)? != reference[start..end] {
            return Err("ILM–DLM padding mismatch".into());
        }
        if self.write_reserved_tail(reference)? != reference[RESERVED_TAIL_START..END_MARKER_OFFSET] {
            return Err("reserved-tail padding mismatch".into());
        }
        if self.write_marker().as_slice() != &reference[END_MARKER_OFFSET..] {
            return Err("firmware suffix mismatch".into());
        }
        self.coverage()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn synthetic() -> Vec<u8> {
        let ilm = CorePartition { file_offset: 0x200, sram_address: 0,
            size_minus_one: 15, stored_crc: 0x0123_4567 };
        let dlm = CorePartition { file_offset: 0x1000, sram_address: 0x20000,
            size_minus_one: 7, stored_crc: 0x7654_3210 };
        let info = InfoDescriptor { sram_address: 0x20200, size_minus_one: 0,
            file_offset: 0x1100, stored_crc: 0x1212_3434 };
        let image = NabuNovatekLayout {
            ilm, dlm, image_len: IMAGE_LENGTH,
            banks: [DescriptorBank { mode: 3, options: 0x5381_10e0,
                       info: [info; 3], auxiliary: info },
                    DescriptorBank { mode: 1, options: 0,
                       info: [info; 3], auxiliary: info }],
        };
        let mut bytes = vec![0; IMAGE_LENGTH];
        bytes[..HEADER_BYTES].copy_from_slice(&image.write_header());
        bytes[IMAGE_LENGTH - 3..].copy_from_slice(TRAILER_MARKER);
        bytes
    }
    #[test]
    fn parses_and_rebuilds_512_byte_metadata() {
        let raw = synthetic();
        let parsed = NabuNovatekLayout::parse(&raw).unwrap();
        assert_eq!(&parsed.write_header(), &raw[..HEADER_BYTES]);
        assert_eq!(parsed.padding_range().unwrap(), (0x210,0x1000));
        assert_eq!(parsed.write_padding(&raw).unwrap().len(), 0xdf0);
    }
    #[test]
    fn rejects_wrong_variant_and_modified_padding() {
        let mut raw = synthetic();
        raw[0x100] ^= 1;
        assert!(NabuNovatekLayout::parse(&raw).is_err());
        let mut raw = synthetic();
        raw[0x80] = 1;
        assert!(NabuNovatekLayout::parse(&raw).is_err());
        let mut raw = synthetic();
        raw[0x210] = 1;
        let parsed = NabuNovatekLayout::parse(&raw).unwrap();
        assert!(parsed.write_padding(&raw).is_err());
    }
    #[test]
    fn whole_image_coverage_is_disjoint_and_full() {
        let raw = synthetic();
        let layout = NabuNovatekLayout::parse(&raw).unwrap();
        let regions = layout.coverage().unwrap();
        assert_eq!(regions.len(), 7);
        assert_eq!(regions.first().unwrap().start, 0);
        assert_eq!(regions.last().unwrap().end, IMAGE_LENGTH);
        assert_eq!(regions.iter().map(CoverageRegion::length).sum::<usize>(), IMAGE_LENGTH);
        assert_eq!(regions.iter().filter(|r|r.is_unimplemented()).count(),3);
        let independents: usize = regions.iter().filter(|r| r.source_independent())
            .map(CoverageRegion::length).sum();
        assert_eq!(independents, 0xdf0 + (END_MARKER_OFFSET-RESERVED_TAIL_START) + 3);
    }
    #[test]
    fn suffix_and_reserved_padding_can_be_rebuilt_independently() {
        let mut raw = synthetic();
        let layout = NabuNovatekLayout::parse(&raw).unwrap();
        assert_eq!(layout.write_reserved_tail(&raw).unwrap().len(), 1789);
        assert_eq!(&layout.write_marker(), b"NVT");
        layout.verify_envelope(&raw).unwrap();
        raw[RESERVED_TAIL_START+5] = 0x4f;
        assert!(layout.write_reserved_tail(&raw).is_err());
        assert!(layout.verify_envelope(&raw).is_err());
    }
    #[test]
    fn rejects_unrepresentable_coverage_without_copying_unknowns() {
        let mut raw = synthetic();
        let mut layout = NabuNovatekLayout::parse(&raw).unwrap();
        layout.dlm.file_offset = 0x1ffff; // ILM gap no longer aligned with actual bytes
        let regions = layout.coverage().unwrap();
        assert_eq!(regions[2].end, 0x1ffff);
        layout.dlm.file_offset = 0x30001;
        assert!(layout.coverage().is_err());
        raw[END_MARKER_OFFSET] = 0;
        assert!(NabuNovatekLayout::parse(&raw).is_err());
    }
    #[test]
    fn rejects_invalid_magic_and_truncated_header() {
        let mut raw = synthetic();
        raw[IMAGE_LENGTH - 1] = 0;
        assert!(NabuNovatekLayout::parse(&raw).is_err());
        assert!(NabuNovatekLayout::parse(&[0;20]).is_err());
    }
}
