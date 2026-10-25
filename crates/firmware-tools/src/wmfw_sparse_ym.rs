// SPDX-License-Identifier: GPL-3.0-only
//! Reference-verified explicit zeros in the CS35L41 HALO packed Y memory.
//!
//! This is a *sparse external-payload packaging* optimization. The named
//! zero-run is encoded independently of any input bytes, but its location was
//! determined by inspecting the exact reference. It is not a recovery of
//! HALO source code or proof that the Y-memory segment is semantically BSS.
//! 3,984 bytes, not 3,984 + 5,445, remains the conservative count of
//! genuinely independently authored firmware metadata.
//!
//! In this firmware's packed Y payload, DSP words 223..2038 are all zero.
//! The 669-byte prefix and 18-byte suffix are *opaque external inputs*,
//! cryptographically pinned alongside the other six opaque DSP payloads.
use crate::{wmfw_external_payload, wmfw_plan, wmfw_sha256};
use std::{fs, path::Path};

/// The Y-memory region starts at file offset 0xf90, after the WMFW descriptor.
pub const YM_FILE_START: usize = 0x0f90;
pub const YM_WORD_COUNT: usize = 2044;
pub const YM_BYTES_PER_WORD: usize = 3;
pub const ZERO_FIRST_WORD: usize = 223;
pub const ZERO_END_WORD: usize = 2038;
pub const ZERO_BYTES: usize = (ZERO_END_WORD-ZERO_FIRST_WORD)*YM_BYTES_PER_WORD;
pub const PREFIX_BYTES: usize = ZERO_FIRST_WORD*YM_BYTES_PER_WORD;
pub const SUFFIX_BYTES: usize = (YM_WORD_COUNT-ZERO_END_WORD)*YM_BYTES_PER_WORD;
pub const SPARSE_EXTERNAL_SOURCE_BYTES: usize = wmfw_external_payload::EXTERNAL_SOURCE_BYTES - ZERO_BYTES;

const PREFIX_NAME: &str = "region-0f90-prefix.bin";
const SUFFIX_NAME: &str = "region-0f90-suffix.bin";
const PREFIX_SHA256: &str = "606ab38dc030f2d48a750f2b85c33b994ef4a3cc6f56fd87b207a170b314b798";
const SUFFIX_SHA256: &str = "9a2123ce83ba0651e0672c7ce6fe5d992ced8ef9d71e101a9b580faf354c20d7";

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ExternalYPart { Prefix, Suffix }
impl ExternalYPart {
    pub fn filename(self) -> &'static str {
        match self { Self::Prefix => PREFIX_NAME, Self::Suffix => SUFFIX_NAME }
    }
    pub fn expected_sha256(self) -> &'static str {
        match self { Self::Prefix => PREFIX_SHA256, Self::Suffix => SUFFIX_SHA256 }
    }
    pub fn length(self) -> usize {
        match self { Self::Prefix => PREFIX_BYTES, Self::Suffix => SUFFIX_BYTES }
    }
}

fn validate_part(part: ExternalYPart, bytes: &[u8]) -> Result<(),String> {
    if bytes.len()!=part.length() {
        return Err(format!("sparse external {} length {} != {}",part.filename(),bytes.len(),part.length()));
    }
    if wmfw_sha256::hex_digest(bytes)!=part.expected_sha256() {
        return Err(format!("sparse external {} SHA-256 mismatch",part.filename()));
    }
    Ok(())
}

/// Verify that the seven opaque WMFW regions match the expected schema.
fn validate_manifest() -> Result<(), String> {
    let regions=wmfw_plan::regions()?;
    let packed:Vec<_>=regions.iter().filter(|r|matches!(r.kind,0x10..=0x12)).collect();
    if packed.len()!=wmfw_external_payload::OPAQUE_REGIONS.len() {
        return Err("unexpected packed WMFW region count".into());
    }
    for (wanted,actual) in wmfw_external_payload::OPAQUE_REGIONS.iter().zip(packed) {
        if wanted.file_offset!=actual.payload_offset || wanted.size!=actual.payload_length ||
           wanted.kind!=actual.kind {
            return Err(format!("sparse region manifest mismatch at 0x{:x}",wanted.file_offset));
        }
    }
    let ym=wmfw_external_payload::OPAQUE_REGIONS.iter().find(|x|x.kind==0x12)
        .ok_or("missing Y-memory region")?;
    if ym.file_offset!=YM_FILE_START || ym.size!=YM_WORD_COUNT*YM_BYTES_PER_WORD ||
       PREFIX_BYTES+ZERO_BYTES+SUFFIX_BYTES!=ym.size || ZERO_END_WORD>YM_WORD_COUNT {
        return Err("packed Y-memory zero-run is out of bounds".into());
    }
    Ok(())
}

/// Build a WMFW from exact-version external original bytes for five X regions,
/// one PM region, and two small Y fragments, plus the typed zero-init range.
/// The two independent inputs are tested before the zero run is generated.
/// Opaque memory data remains opaque even when a subrange is all zero.
pub fn build_with_sparse_y<X,Y>(mut read_opaque:X,mut read_y:Y)->Result<Vec<u8>,String>
where X:FnMut(wmfw_external_payload::OpaqueRegion)->Result<Vec<u8>,String>,
      Y:FnMut(ExternalYPart)->Result<Vec<u8>,String>
{
    validate_manifest()?;
    let mut plan=wmfw_plan::partial_container()?;
    let mut imported_bytes=0usize;
    for region in wmfw_external_payload::OPAQUE_REGIONS {
        if region.kind==0x12 { continue; }
        let source=read_opaque(region)?;
        if source.len()!=region.size {
            return Err(format!("external {} length {} != {}",region.filename(),source.len(),region.size));
        }
        if wmfw_sha256::hex_digest(&source)!=region.expected_sha256 {
            return Err(format!("external {} SHA-256 mismatch",region.filename()));
        }
        plan.place(&format!("EXTERNAL original DSP payload {}",region.filename()),region.file_offset,&source)?;
        imported_bytes=imported_bytes.checked_add(source.len()).ok_or("external byte count overflow")?;
    }
    for part in [ExternalYPart::Prefix,ExternalYPart::Suffix] {
        let source=read_y(part)?;
        validate_part(part,&source)?;
        let offset=match part {
            ExternalYPart::Prefix=>YM_FILE_START,
            ExternalYPart::Suffix=>YM_FILE_START+PREFIX_BYTES+ZERO_BYTES,
        };
        plan.place(&format!("EXTERNAL original Y-memory {}",part.filename()),offset,&source)?;
        imported_bytes+=source.len();
    }
    if imported_bytes!=SPARSE_EXTERNAL_SOURCE_BYTES {
        return Err("sparse external byte count mismatch".into());
    }
    plan.place("reference-verified zero-initialized Y-memory words 223..2038",
               YM_FILE_START+PREFIX_BYTES,&vec![0u8;ZERO_BYTES])?;
    let assembled=plan.build_complete()?;
    if wmfw_sha256::hex_digest(&assembled)!=wmfw_external_payload::REFERENCE_SHA256 {
        return Err("sparse WMFW assembled SHA-256 mismatch".into());
    }
    Ok(assembled)
}

pub fn assemble_directory(dir:&Path)->Result<Vec<u8>,String> {
    build_with_sparse_y(
        |region|fs::read(dir.join(region.filename()))
            .map_err(|e|format!("reading {}: {e}",region.filename())),
        |part|fs::read(dir.join(part.filename()))
            .map_err(|e|format!("reading {}: {e}",part.filename())),
    )
}

/// Extract exact-original external inputs into a private, atomic directory.
/// Unlike the original seven-whole-region mode, Y-memory's known zero run
/// is not copied into a standalone file.
pub fn extract_private_sparse_inputs(reference:&[u8],dir:&Path)->Result<(),String> {
    validate_manifest()?;
    if wmfw_sha256::hex_digest(reference)!=wmfw_external_payload::REFERENCE_SHA256 {
        return Err("WMFW reference SHA-256 mismatch".into());
    }
    wmfw_plan::verify_container(reference)?;
    let mut entries=Vec::with_capacity(8);
    for region in wmfw_external_payload::OPAQUE_REGIONS {
        if region.kind==0x12 { continue; }
        let source=reference.get(region.file_offset..region.file_offset+region.size)
            .ok_or("reference opaque memory exceeds file")?;
        if wmfw_sha256::hex_digest(source)!=region.expected_sha256 {
            return Err(format!("reference {} SHA-256 mismatch",region.filename()));
        }
        entries.push((region.filename(),source.to_vec()));
    }
    for part in [ExternalYPart::Prefix,ExternalYPart::Suffix] {
        let offset=match part {
            ExternalYPart::Prefix=>YM_FILE_START,
            ExternalYPart::Suffix=>YM_FILE_START+PREFIX_BYTES+ZERO_BYTES,
        };
        let source=reference.get(offset..offset+part.length())
            .ok_or("reference Y subregion exceeds file")?;
        validate_part(part,source)?;
        entries.push((part.filename().to_string(),source.to_vec()));
    }
    let zero=reference.get(YM_FILE_START+PREFIX_BYTES..YM_FILE_START+PREFIX_BYTES+ZERO_BYTES)
        .ok_or("reference zero range outside file")?;
    if zero.iter().any(|x|*x!=0) {
        return Err("reference packed Y-memory zero words are not all zero".into());
    }
    wmfw_external_payload::write_private_region_directory(dir,&entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_zero_run_is_bounded_and_well_formed() {
        validate_manifest().unwrap();
        assert_eq!(YM_WORD_COUNT,2044);
        assert_eq!(ZERO_BYTES,5445);
        assert_eq!(PREFIX_BYTES,669);
        assert_eq!(SUFFIX_BYTES,18);
        assert_eq!(SPARSE_EXTERNAL_SOURCE_BYTES,24_807);
        assert_eq!(YM_FILE_START+PREFIX_BYTES,0x122d);
        assert_eq!(YM_FILE_START+PREFIX_BYTES+ZERO_BYTES,0x2772);
        assert_eq!(PREFIX_BYTES+ZERO_BYTES+SUFFIX_BYTES,6132);
    }
    #[test]
    fn sparse_builder_rejects_missing_inputs_and_fabricated_reference_payload() {
        let error=build_with_sparse_y(|_|Err("missing source".into()),|_|Err("missing Y".into())).unwrap_err();
        assert!(error.contains("missing source"));
        let error=build_with_sparse_y(|r|Ok(vec![0;r.size]), |_|Ok(vec![0;PREFIX_BYTES])).unwrap_err();
        assert!(error.contains("SHA-256"));
        assert!(validate_part(ExternalYPart::Prefix,&[0;3]).unwrap_err().contains("length"));
        assert!(validate_part(ExternalYPart::Suffix,&vec![0;SUFFIX_BYTES]).unwrap_err().contains("SHA-256"));
    }
}
