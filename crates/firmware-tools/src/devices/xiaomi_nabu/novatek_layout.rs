// SPDX-License-Identifier: GPL-3.0-only
//! Strict, reversible metadata plus independently generated NDS32 ISR prefix
//! for Nabu NT36xxx-style
//! touchscreen firmware. The NDS32 instruction set is corroborated by public
//! ISA rules and exact instruction encodings; other executable payload bytes
//! remain unimplemented and are never copied into generated source.
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
/// The Nabu ILM starts with 128 opaque 32-bit vector/descriptors, then 80
/// reserved zeros before an independently reconstructed NDS32 handler.
pub const CANDIDATE_VECTORS_END: usize = 0x400;
pub const ILM_CODE_START: usize = 0x450;
/// First 398 bytes are independently reconstructed NDS32 exception/setup code.
pub const ILM_SOURCE_PREFIX_END: usize = super::novatek_nds32::BOOT_SETUP_END;
/// Nabu's observed all-zero reserved tail; this is specific to the pinned
/// 196,608-byte firmware layout, not a general Novatek-family assumption.
pub const ILM_SPACER_START: usize = 0x1ac80;
pub const DLM_FILE_START: usize = 0x22000;
pub const DLM_FILE_END: usize = 0x2e190;
/// Exact two-revision Nabu format: reserved zero bytes after DLM and before
/// the nonzero configuration bank. These aren't a generic Novatek rule.
pub const POST_DLM_PADDING_END: usize = 0x2f000;
pub const RESERVED_TAIL_START: usize = 0x2f900;
/// The 0x900-byte bank at 0x2f000 is a byte-for-byte mirror of the DLM
/// prefix, in both independent Nabu firmware revisions. This is a *derived*
/// region: it can only be written after the DLM payload has been authored.
pub const DLM_MIRROR_START: usize = POST_DLM_PADDING_END;
pub const DLM_MIRROR_BYTES: usize = RESERVED_TAIL_START - DLM_MIRROR_START;
pub const END_MARKER_OFFSET: usize = IMAGE_LENGTH - 3;
pub const TRAILER_MARKER: &[u8; 3] = b"NVT";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorePartition {
    pub file_offset: u32,
    pub sram_address: u32,
    /// Format stores `length - 1`; this is NOT a byte count.
    pub size_minus_one: u32,
    /// Stored CRC-32/MPEG-2. Regenerable from authored payload via
    /// `novatek_crc::recompute_all_crcs`, but not by this metadata-only parser.
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
    /// Bank CRC footer at offset 0xf0. The final `stored_crc` checks the
    /// preceding 0xfc bytes of this bank; the other three fields are metadata.
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
/// NDS32 handler bytes are source-generated; metadata is serializable and
/// its CRC-32/MPEG-2 fields are computable from
/// independently authored payloads. Zero ranges and `NVT` are independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    StructuredMetadataRecomputableCrc,
    StructuredVectorsWithOpaqueTargets,
    /// Source-generated NDS32 interrupt entry/return instructions.
    IndependentlyGeneratedNds32,
    UnimplementedIlm,
    IndependentlyGeneratedZero,
    UnimplementedDlm,
    IndependentlyGeneratedMarker,
    /// Emit from a validated, independently authored DLM prefix, not the OEM
    /// reference. No unknown data may be substituted here.
    DerivedFromAuthoredDlm,
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
                  | RegionKind::IndependentlyGeneratedMarker
                  | RegionKind::IndependentlyGeneratedNds32)
    }
    pub fn is_unimplemented(&self) -> bool {
        matches!(self.kind, RegionKind::StructuredVectorsWithOpaqueTargets |
                  RegionKind::UnimplementedIlm
                  | RegionKind::UnimplementedDlm)
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
        // This checks layout representation only. CRC verification is an
        // explicit operation in `novatek_crc::verify_all_crcs`.
        if result.write_header() != firmware[..HEADER_BYTES] {
            return Err("Novatek header is not representable by the two-bank layout".into());
        }
        Ok(result)
    }

    /// Write exactly 512 bytes from field values, without reading any vendor
    /// image, making this routine suitable for replacing future source tables.
    /// This metadata-only method copies the supplied scalar CRC values.
    /// For independent CRC generation call `novatek_crc::recompute_all_crcs`
    /// on an image assembled from authored ILM and DLM payloads.
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

    /// DLM-end..0x2f000: fully zero in both verified Nabu revisions.
    /// This method reconstructs from the layout, using the reference *only*
    /// to reject a revision with nonzero bytes in the reserved gap.
    pub fn write_post_dlm_padding(&self, reference: &[u8]) -> Result<Vec<u8>, String> {
        if reference.len() != self.image_len {return Err("different image length".into());}
        let start = self.dlm.end()?;
        if start > DLM_FILE_END || start < DLM_FILE_START {
            return Err("unexpected Nabu DLM end".into());
        }
        require_zero(reference, start, POST_DLM_PADDING_END)?;
        Ok(vec![0; POST_DLM_PADDING_END - start])
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

    /// Recreate the 2304-byte configuration mirror from the supplied DLM.
    /// The *source* DLM must still be independently authored/decompiled;
    /// simply passing the OEM DLM does not reconstruct its instructions.
    pub fn write_dlm_mirror(&self, authored_dlm: &[u8]) -> Result<Vec<u8>, String> {
        if authored_dlm.len() != self.dlm.actual_length()? {
            return Err("DLM mirror source has unexpected length".into());
        }
        let bytes = authored_dlm.get(..DLM_MIRROR_BYTES)
            .ok_or("DLM payload shorter than mirrored block")?;
        Ok(bytes.to_vec())
    }

    /// Confirm the Nabu-specific mirror invariant against a reference.
    pub fn verify_dlm_mirror(&self, reference: &[u8]) -> Result<(), String> {
        if reference.len()!=self.image_len {return Err("different image length".into());}
        let dlm = reference.get(self.dlm.file_offset as usize..self.dlm.end()?)
            .ok_or("missing DLM data")?;
        let from_dlm = self.write_dlm_mirror(dlm)?;
        if from_dlm != reference[DLM_MIRROR_START..RESERVED_TAIL_START] {
            return Err("DLM mirrored configuration is not identical to DLM prefix".into());
        }
        Ok(())
    }

    /// A complete, gapless provenance map. The unknown regions remain marked
    /// unknown even when the reference happens to have incidental zero bytes.
    pub fn coverage(&self) -> Result<Vec<CoverageRegion>, String> {
        // The GP-relative interrupt handler is profile-specific. Reject a
        // foreign ILM length before claiming its executable bytes are ours.
        super::novatek_nds32::boot_profile_for_ilm_length(self.ilm.actual_length()?)?;
        let ilm_end = self.ilm.end()?;
        let dlm_begin = self.dlm.file_offset as usize;
        let dlm_end = self.dlm.end()?;
        // Only the exact known Nabu partition geometry is supported. Without
        // this guard a synthetic/foreign revision might silently lose data
        // from the newly identified post-DLM zero-padded region.
        if ilm_end > ILM_SPACER_START || ilm_end <= ILM_SOURCE_PREFIX_END ||
           dlm_begin != DLM_FILE_START ||
           dlm_end > DLM_FILE_END || dlm_end <= DLM_FILE_START ||
           self.image_len != IMAGE_LENGTH {
            return Err("unsupported Novatek partition geometry".into());
        }
        let regions = vec![
            CoverageRegion { name: "dual-bank metadata (CRC-32/MPEG-2)",
                start: 0, end: HEADER_BYTES,
                kind: RegionKind::StructuredMetadataRecomputableCrc },
            CoverageRegion { name: "candidate interrupt/handler words (externally supplied)",
                start: HEADER_BYTES, end: CANDIDATE_VECTORS_END,
                kind: RegionKind::StructuredVectorsWithOpaqueTargets },
            CoverageRegion { name: "ILM candidate-vector reserved zeros",
                start: CANDIDATE_VECTORS_END, end: ILM_CODE_START,
                kind: RegionKind::IndependentlyGeneratedZero },
            CoverageRegion { name: "NDS32 authored exception entry path",
                start: ILM_CODE_START, end: ILM_SOURCE_PREFIX_END,
                kind: RegionKind::IndependentlyGeneratedNds32 },
            CoverageRegion { name: "ILM instruction/data after authored ISR",
                start: ILM_SOURCE_PREFIX_END, end: ilm_end,
                kind: RegionKind::UnimplementedIlm },
            CoverageRegion { name: "ILM–DLM gap", start: ilm_end,
                end: dlm_begin, kind: RegionKind::IndependentlyGeneratedZero },
            CoverageRegion { name: "DLM instruction/data", start: dlm_begin,
                end: dlm_end, kind: RegionKind::UnimplementedDlm },
            CoverageRegion { name: "post-DLM reserved zero gap", start: dlm_end,
                end: POST_DLM_PADDING_END, kind: RegionKind::IndependentlyGeneratedZero },
            CoverageRegion { name: "DLM-prefix configuration mirror (requires authored DLM)",
                start: DLM_MIRROR_START, end: RESERVED_TAIL_START,
                kind: RegionKind::DerivedFromAuthoredDlm },
            CoverageRegion { name: "reserved end padding", start: RESERVED_TAIL_START,
                end: END_MARKER_OFFSET, kind: RegionKind::IndependentlyGeneratedZero },
            CoverageRegion { name: "NVT magic suffix", start: END_MARKER_OFFSET,
                end: IMAGE_LENGTH, kind: RegionKind::IndependentlyGeneratedMarker },
        ];
        validate_coverage(&regions, self.image_len)?;
        Ok(regions)
    }

    /// Verify independently reconstructible ranges, NDS32 entry-path bytes,
    /// and metadata serialization. This says nothing about the unknown ILM
    /// remainder, DLM instructions or the physical MCU model.
    pub fn verify_envelope(&self, reference: &[u8]) -> Result<(), String> {
        if reference.len() != self.image_len { return Err("different image length".into()); }
        if self.write_header().as_slice() != &reference[..HEADER_BYTES] {
            return Err("metadata does not match fields".into());
        }
        require_zero(reference,CANDIDATE_VECTORS_END,ILM_CODE_START)?;
        super::novatek_nds32::verify_authored_startup_prefix(reference,self.ilm.actual_length()?)?;
        let (start, end) = self.padding_range()?;
        if self.write_padding(reference)? != reference[start..end] {
            return Err("ILM–DLM padding mismatch".into());
        }
        if self.write_post_dlm_padding(reference)? != reference[self.dlm.end()?..POST_DLM_PADDING_END] {
            return Err("post-DLM padding mismatch".into());
        }
        if self.write_reserved_tail(reference)? != reference[RESERVED_TAIL_START..END_MARKER_OFFSET] {
            return Err("reserved-tail padding mismatch".into());
        }
        self.verify_dlm_mirror(reference)?;
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
            size_minus_one: 0x1aa7f, stored_crc: 0x0123_4567 };
        let dlm = CorePartition { file_offset: 0x22000, sram_address: 0x20000,
            size_minus_one: 0xc18f, stored_crc: 0x7654_3210 };
        let info = InfoDescriptor { sram_address: 0x20200, size_minus_one: 0,
            file_offset: 0x22000, stored_crc: 0x1212_3434 };
        let image = NabuNovatekLayout {
            ilm, dlm, image_len: IMAGE_LENGTH,
            banks: [DescriptorBank { mode: 3, options: 0x5381_10e0,
                       info: [info; 3], auxiliary: info },
                    DescriptorBank { mode: 1, options: 0,
                       info: [info; 3], auxiliary: info }],
        };
        let mut bytes = vec![0; IMAGE_LENGTH];
        bytes[..HEADER_BYTES].copy_from_slice(&image.write_header());
        bytes[ILM_CODE_START..ILM_SOURCE_PREFIX_END]
            .copy_from_slice(&super::super::novatek_nds32::authored_startup_prefix(image.ilm.actual_length().unwrap()).unwrap());
        bytes[IMAGE_LENGTH - 3..].copy_from_slice(TRAILER_MARKER);
        bytes
    }
    #[test]
    fn parses_and_rebuilds_512_byte_metadata() {
        let raw = synthetic();
        let parsed = NabuNovatekLayout::parse(&raw).unwrap();
        assert_eq!(&parsed.write_header(), &raw[..HEADER_BYTES]);
        assert_eq!(parsed.padding_range().unwrap(), (0x1ac80,0x22000));
        assert_eq!(parsed.write_padding(&raw).unwrap().len(), 0x7380);
        assert_eq!(parsed.write_post_dlm_padding(&raw).unwrap().len(), 0xe70);
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
        raw[0x1ac80] = 1;
        let parsed = NabuNovatekLayout::parse(&raw).unwrap();
        assert!(parsed.write_padding(&raw).is_err());
    }
    #[test]
    fn whole_image_coverage_is_disjoint_and_full() {
        let raw = synthetic();
        let layout = NabuNovatekLayout::parse(&raw).unwrap();
        let regions = layout.coverage().unwrap();
        assert_eq!(regions.len(), 11);
        assert_eq!(regions.first().unwrap().start, 0);
        assert_eq!(regions.last().unwrap().end, IMAGE_LENGTH);
        assert_eq!(regions.iter().map(CoverageRegion::length).sum::<usize>(), IMAGE_LENGTH);
        assert_eq!(regions.iter().filter(|r|r.is_unimplemented()).count(),3);
        let independents: usize = regions.iter().filter(|r| r.source_independent())
            .map(CoverageRegion::length).sum();
        assert_eq!(independents, (ILM_CODE_START-CANDIDATE_VECTORS_END)
            + (ILM_SOURCE_PREFIX_END-ILM_CODE_START)
            + (DLM_FILE_START-ILM_SPACER_START) + (POST_DLM_PADDING_END-DLM_FILE_END)
            + (END_MARKER_OFFSET-RESERVED_TAIL_START) + 3);
    }
    #[test]
    fn detects_modified_post_dlm_zero_gap() {
        let mut raw = synthetic();
        let layout = NabuNovatekLayout::parse(&raw).unwrap();
        let generated = layout.write_post_dlm_padding(&raw).unwrap();
        assert_eq!(generated.len(), 3696);
        assert!(generated.iter().all(|b|*b==0));
        raw[0x2e195] = 0xa5;
        assert!(layout.write_post_dlm_padding(&raw).unwrap_err().contains("0x2e195"));
        assert!(layout.verify_envelope(&raw).is_err());
    }
    #[test]
    fn dlm_prefix_mirror_is_derived_and_tamper_detected() {
        let mut raw=synthetic();
        let layout=NabuNovatekLayout::parse(&raw).unwrap();
        let dlm=raw[0x22000..0x2e190].to_vec();
        assert_eq!(layout.write_dlm_mirror(&dlm).unwrap(), vec![0;DLM_MIRROR_BYTES]);
        layout.verify_dlm_mirror(&raw).unwrap();
        raw[DLM_MIRROR_START+0x29]=0x42;
        assert!(layout.verify_dlm_mirror(&raw).unwrap_err().contains("mirrored"));
        assert!(layout.verify_envelope(&raw).is_err());
    }
    #[test]
    fn alternate_revision_shorter_dlm_can_have_larger_padding() {
        let mut raw = synthetic();
        let mut expected = NabuNovatekLayout::parse(&raw).unwrap();
        expected.ilm.size_minus_one = 0x19e8f; // upstream ILM ends 0x1a090
        expected.dlm.size_minus_one = 0xc177; // upstream DLM ends 0x2e178
        // This source-generated path is relocatable: the old linker variant
        // places GP 0x1f0 bytes earlier than the postmarketOS image.
        raw[ILM_CODE_START..ILM_SOURCE_PREFIX_END].copy_from_slice(&
            super::super::novatek_nds32::authored_startup_prefix(expected.ilm.actual_length().unwrap()).unwrap());
        raw[..HEADER_BYTES].copy_from_slice(&expected.write_header());
        let layout = NabuNovatekLayout::parse(&raw).unwrap();
        assert_eq!(layout.write_padding(&raw).unwrap().len(),0x7f70);
        assert_eq!(layout.write_post_dlm_padding(&raw).unwrap().len(),0xe88);
        assert_eq!(layout.coverage().unwrap().len(),11);
        layout.verify_envelope(&raw).unwrap();
    }
    #[test]
    fn generated_nds32_isr_has_distinct_nonopaque_coverage() {
        let raw=synthetic();
        let layout=NabuNovatekLayout::parse(&raw).unwrap();
        let coverage=layout.coverage().unwrap();
        let code=coverage.iter().find(|r|r.kind==RegionKind::IndependentlyGeneratedNds32).unwrap();
        assert_eq!((code.start,code.end,code.length()),(0x450,0x5de,398));
        assert!(code.source_independent());
        let remaining=coverage.iter().find(|r|r.kind==RegionKind::UnimplementedIlm).unwrap();
        assert_eq!(remaining.start,0x5de);
        assert_eq!(remaining.length(),108194);
        let total_independent=coverage.iter().filter(|r|r.source_independent())
            .map(CoverageRegion::length).sum::<usize>();
        assert_eq!(total_independent,35534);
    }
    #[test]
    fn mutated_encoded_isr_is_rejected_without_mutating_other_regions() {
        let mut raw=synthetic();
        let layout=NabuNovatekLayout::parse(&raw).unwrap();
        assert!(layout.verify_envelope(&raw).is_ok());
        raw[0x46d]^=1;
        let e=layout.verify_envelope(&raw).unwrap_err();
        assert!(e.contains("0x46d"),"{e}");
        // Parsing metadata is intentionally still possible after an ISR mutation.
        assert!(NabuNovatekLayout::parse(&raw).is_ok());
    }
    #[test]
    fn third_handler_mutation_is_detected_with_original_offset() {
        let mut raw=synthetic();
        let layout=NabuNovatekLayout::parse(&raw).unwrap();
        layout.verify_envelope(&raw).unwrap();
        raw[0x565]^=0x01;
        let e=layout.verify_envelope(&raw).unwrap_err();
        assert!(e.contains("0x565"),"{e}");
        assert!(NabuNovatekLayout::parse(&raw).is_ok());
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
        assert!(layout.coverage().is_err());
        layout.dlm.file_offset = 0x30001;
        assert!(layout.coverage().is_err());
        raw[END_MARKER_OFFSET] = 0;
        assert!(NabuNovatekLayout::parse(&raw).is_err());
    }
    #[test]
    fn reserved_vector_padding_must_be_zero() {
        let mut raw = synthetic();
        let layout = NabuNovatekLayout::parse(&raw).unwrap();
        layout.verify_envelope(&raw).unwrap();
        raw[0x414] = 0xa5;
        assert!(layout.verify_envelope(&raw).is_err());
    }
    #[test]
    fn rejects_invalid_magic_and_truncated_header() {
        let mut raw = synthetic();
        raw[IMAGE_LENGTH - 1] = 0;
        assert!(NabuNovatekLayout::parse(&raw).is_err());
        assert!(NabuNovatekLayout::parse(&[0;20]).is_err());
    }
}
