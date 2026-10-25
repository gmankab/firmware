// SPDX-License-Identifier: GPL-3.0-only
//! Strict, sparse Novatek firmware reconstruction from independently supplied
//! ILM/DLM/configuration bytes and known structural constants. Unknown ranges
//! are never copied from a reference image or implicitly zero-filled.
//!
//! The metadata-only assembly path **does not automatically recompute**
//! CRC-32/MPEG-2. For a separately checksum-verified structural output,
//! use `novatek_crc::recompute_all_crcs` on caller-authored payloads. Neither
//! path proves executable provenance beyond the NDS32 ISR or yields
//! a proven bootable firmware.

use super::novatek_layout::{CoverageRegion, NabuNovatekLayout, RegionKind};
use super::novatek_vector::CandidateVectorTable;
use super::novatek_crc::{recompute_all_crcs,verify_all_crcs,CrcAudit};

/// Caller-authored executable/data bytes. `None` means source is missing and
/// MUST prevent complete generation (even if that region is zeros in an OEM
/// reference). Lengths are independently checked against the layout.
#[derive(Debug, Clone, Copy, Default)]
pub struct AuthoredPayloads<'a> {
    pub vector_table: Option<&'a CandidateVectorTable>,
    /// Independently authored ILM *after* the already generated NDS32 ISR
    /// paths: `[0x5de, ILM_END)`. The first 398 executable bytes are written
    /// by us. Older full-ILM callers must remove that prefix and cannot
    /// override it with reference bytes.
    pub ilm: Option<&'a [u8]>,
    pub dlm: Option<&'a [u8]>,
}

/// Generated values for *only* source-known regions. An absent fragment
/// is represented as `None` and is not materialized as a misleading zero gap.
#[derive(Debug, Clone)]
pub struct ProjectedRegion {
    pub range: CoverageRegion,
    pub bytes: Option<Vec<u8>>,
}

/// A byte-complete structurally assembled image. The hardware CRC fields
/// still require independent verification before this can be used on a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnverifiedStructuralImage {
    pub bytes: Vec<u8>,
}

impl UnverifiedStructuralImage {
    pub fn is_hardware_crc_verified(&self) -> bool { false }
}

/// Assembled from independently supplied vector/ILM/DLM payloads and
/// *recomputed* CRC-32/MPEG-2 fields. This asserts only checksum consistency;
/// it does not establish any controller ISA, code provenance or signing.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct CrcValidatedStructuralImage {
    pub bytes: Vec<u8>,
    pub checksum_audit: CrcAudit,
}
impl CrcValidatedStructuralImage {
    pub fn crc_fields_generated(&self) -> usize { self.checksum_audit.stored_fields() }
    pub fn verified_mcu_program(&self) -> bool { false }
    pub fn flashable(&self) -> bool { false }
}

/// For a complete caller-authored payload set, generate the header CRCs,
/// six info-entry CRCs and two bank-footer CRCs without vendor checksum
/// inputs. The DLM-mirror region is derived from `authored.dlm` as before.
/// Unknown executable bytes are never materialized implicitly; failing to
/// supply them produces the same strict error as metadata-only assembly.
pub fn assemble_crc_validated(
    layout:&NabuNovatekLayout,
    authored:AuthoredPayloads<'_>,
)->Result<CrcValidatedStructuralImage,String>{
    let mut result=assemble_structural_unverified(layout,authored)?.bytes;
    let computed=recompute_all_crcs(&mut result)?;
    let updated=NabuNovatekLayout::parse(&result)?;
    updated.verify_envelope(&result)?;
    let checked=verify_all_crcs(&updated,&result)?;
    if checked != computed {return Err("CRC recomputation audit mismatch".into());}
    Ok(CrcValidatedStructuralImage{bytes:result,checksum_audit:checked})
}

/// Return the full gapless region plan. This never needs the original
/// firmware bytes (only typed header scalars and the Nabu layout rules).
pub fn projected_regions(layout: &NabuNovatekLayout) -> Result<Vec<ProjectedRegion>, String> {
    let regions = layout.coverage()?;
    regions.into_iter().map(|range| {
        let bytes = match range.kind {
            RegionKind::StructuredMetadataRecomputableCrc => Some(layout.write_header().to_vec()),
            RegionKind::IndependentlyGeneratedNds32 =>
                Some(super::novatek_nds32::authored_startup_prefix(layout.ilm.actual_length()?)?),
            RegionKind::StructuredVectorsWithOpaqueTargets => None,
            RegionKind::IndependentlyGeneratedZero => Some(vec![0; range.length()]),
            RegionKind::IndependentlyGeneratedMarker => Some(layout.write_marker().to_vec()),
            RegionKind::UnimplementedIlm | RegionKind::UnimplementedDlm |
            RegionKind::DerivedFromAuthoredDlm => None,
        };
        Ok(ProjectedRegion {range,bytes})
    }).collect()
}

fn check_supplied<'a>(region: &CoverageRegion, data: Option<&'a [u8]>) -> Result<&'a [u8], String> {
    let data = data.ok_or_else(|| format!(
        "cannot assemble firmware: missing independently authored {} [0x{:x},0x{:x}), {} bytes",
        region.name,region.start,region.end,region.length()
    ))?;
    if data.len()!=region.length() {
        return Err(format!("authored {} length {} != required {} (0x{:x}..0x{:x})",
            region.name,data.len(),region.length(),region.start,region.end));
    }
    Ok(data)
}

/// Reconstruct the exact byte layout **only after** a caller has authored all
/// missing payloads. The function will not accept a reference image, and
/// cannot substitute any source portion from it. CRC words remain the
/// externally provided fields until explicitly regenerated.
///
/// Do not use this result for firmware flashing: this function deliberately
/// does not assert/check any CRC fields, or establish executable authenticity.
pub fn assemble_structural_unverified(
    layout: &NabuNovatekLayout,
    authored: AuthoredPayloads<'_>,
) -> Result<UnverifiedStructuralImage, String> {
    let mut result = Vec::with_capacity(layout.image_len);
    let regions = projected_regions(layout)?;
    for region in regions {
        if result.len()!=region.range.start {
            return Err(format!("noncontiguous layout at 0x{:x}",region.range.start));
        }
        if region.range.kind == RegionKind::StructuredVectorsWithOpaqueTargets {
            let words=authored.vector_table.ok_or_else(||format!(
                "cannot assemble firmware: missing externally authored candidate vector table [0x{:x},0x{:x})",
                region.range.start,region.range.end))?;
            result.extend_from_slice(&words.write());
            continue;
        }
        if region.range.kind == RegionKind::DerivedFromAuthoredDlm {
            let dlm=authored.dlm.ok_or_else(||"missing independently authored DLM for mirror".to_string())?;
            let generated=layout.write_dlm_mirror(dlm)?;
            if generated.len() != region.range.length() {return Err("DLM mirror has wrong length".into());}
            result.extend_from_slice(&generated);
            continue;
        }
        let part = match region.range.kind {
            RegionKind::StructuredVectorsWithOpaqueTargets => unreachable!("handled above"),
            RegionKind::UnimplementedIlm => check_supplied(&region.range,authored.ilm)?,
            RegionKind::UnimplementedDlm => check_supplied(&region.range,authored.dlm)?,
            RegionKind::DerivedFromAuthoredDlm => unreachable!("DLM mirror handled above"),
            _ => region.bytes.as_deref().ok_or("missing generated fixed region")?,
        };
        if part.len()!=region.range.length() {
            return Err(format!("wrong length for {}",region.range.name));
        }
        result.extend_from_slice(part);
    }
    if result.len()!=layout.image_len {
        return Err(format!("incomplete structural image: {} != {}",result.len(),layout.image_len));
    }
    Ok(UnverifiedStructuralImage {bytes: result})
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::novatek_layout::{CorePartition,InfoDescriptor,DescriptorBank,IMAGE_LENGTH,HEADER_BYTES};

    fn sample_layout() -> NabuNovatekLayout {
        let ilm=CorePartition{file_offset:0x200,sram_address:0,size_minus_one:0x1aa7f,stored_crc:0x12345678};
        let dlm=CorePartition{file_offset:0x22000,sram_address:0x20000,size_minus_one:0xc18f,stored_crc:0xabcd4321};
        let info=InfoDescriptor{sram_address:0x20000,size_minus_one:0,file_offset:0x22000,stored_crc:0};
        NabuNovatekLayout{ilm,dlm,image_len:IMAGE_LENGTH,
            banks:[DescriptorBank{mode:3,options:0,info:[info;3],auxiliary:info},
                   DescriptorBank{mode:1,options:0,info:[info;3],auxiliary:info}]}
    }
    #[test]
    fn initial_projection_exposes_all_holes_without_filling_them() {
        let layout=sample_layout();
        let projection=projected_regions(&layout).unwrap();
        assert_eq!(projection.len(),11);
        assert_eq!(projection.iter().filter(|x|x.bytes.is_none()).count(),4); // vectors, remaining ILM, DLM, derived mirror
        assert_eq!(projection[0].bytes.as_ref().unwrap().len(),HEADER_BYTES);
        assert_eq!(projection[2].bytes.as_ref().unwrap().len(),0x50);
        assert_eq!(projection[3].bytes.as_ref().unwrap().len(),0x18e);
        assert_eq!(projection[5].bytes.as_ref().unwrap().len(),0x7380);
        assert_eq!(projection[7].bytes.as_ref().unwrap().len(),0xe70);
        assert_eq!(projection[9].bytes.as_ref().unwrap().len(),1789);
        assert_eq!(projection[10].bytes.as_ref().unwrap(),b"NVT");
        let e=assemble_structural_unverified(&layout,AuthoredPayloads::default()).unwrap_err();
        assert!(e.contains("missing externally authored candidate vector table"),"{e}");
    }
    #[test]
    fn cannot_override_source_derived_isr_with_stale_full_ilm_input() {
        let layout=sample_layout();
        let vectors=CandidateVectorTable{initial_control_word:0,opaque_slots:[0;127]};
        // The old API accepted the full ILM starting at 0x450; this should
        // now fail, because 0x450..0x4aa is authored by the encoder.
        let legacy_whole_ilm=vec![0x55;0x1ac80-0x450];
        let err=assemble_structural_unverified(&layout,AuthoredPayloads{
            vector_table:Some(&vectors),ilm:Some(&legacy_whole_ilm),..Default::default()
        }).unwrap_err();
        assert!(err.contains("108592 != required 108194"),"{err}");
        assert_eq!(projected_regions(&layout).unwrap()[3].bytes.as_ref().unwrap(),
            &super::super::novatek_nds32::authored_startup_prefix(layout.ilm.actual_length().unwrap()).unwrap());
    }
    #[test]
    fn refuses_missing_dlm_and_uses_derived_config_from_authored_dlm() {
        let layout=sample_layout();
        let vectors=CandidateVectorTable{initial_control_word:0,
            opaque_slots:[0;127]};
        let ilm=vec![3;0x1ac80-0x5de]; let dlm=vec![2;0xc190];
        let e=assemble_structural_unverified(&layout,AuthoredPayloads{
            vector_table:Some(&vectors),ilm:Some(&ilm),..Default::default()}).unwrap_err();
        assert!(e.contains("DLM"));
        let img=assemble_structural_unverified(&layout,AuthoredPayloads{
            vector_table:Some(&vectors),ilm:Some(&ilm),dlm:Some(&dlm)}).unwrap();
        assert_eq!(&img.bytes[0x2f000..0x2f900],&dlm[..0x900]);
    }
    #[test]
    fn refuses_payloads_of_wrong_lengths_before_finalizing() {
        let layout=sample_layout();
        let vectors=CandidateVectorTable{initial_control_word:0,opaque_slots:[0;127]};
        let incorrect=vec![0;17];
        let e=assemble_structural_unverified(&layout,AuthoredPayloads{
            vector_table:Some(&vectors),ilm:Some(&incorrect),..Default::default()}).unwrap_err();
        assert!(e.contains("length 17 != required 108194"),"{e}");
    }
    #[test]
    fn crc_aware_assembler_generates_all_twelve_fields_and_dlm_mirror() {
        let layout=sample_layout();
        let vectors=CandidateVectorTable{initial_control_word:0,opaque_slots:[0;127]};
        let mut ilm=vec![0xa5;0x1ac80-0x5de];
        let mut dlm=vec![0x5a;0xc190];
        ilm[0x203]=0x80;dlm[0x24]=0xc6;
        let image=assemble_crc_validated(&layout,AuthoredPayloads{
            vector_table:Some(&vectors),ilm:Some(&ilm),dlm:Some(&dlm)
        }).unwrap();
        assert_eq!(image.crc_fields_generated(),12);
        assert_eq!(image.bytes.len(),IMAGE_LENGTH);
        assert_eq!(&image.bytes[0x450..0x5de],
            &super::super::novatek_nds32::authored_startup_prefix(layout.ilm.actual_length().unwrap()).unwrap());
        assert_eq!(&image.bytes[0x2f000..0x2f900],&dlm[..0x900]);
        assert!(!image.verified_mcu_program());
        assert!(!image.flashable());
        let checked=NabuNovatekLayout::parse(&image.bytes).unwrap();
        assert_eq!(verify_all_crcs(&checked,&image.bytes).unwrap(),image.checksum_audit);
        assert_ne!(&image.bytes[..HEADER_BYTES],&layout.write_header());
    }
    #[test]
    fn crc_aware_assembly_rejects_unknown_input_code() {
        let layout=sample_layout();
        let error=assemble_crc_validated(&layout,AuthoredPayloads::default()).unwrap_err();
        assert!(error.contains("missing externally authored candidate vector table"),"{error}");
    }
    #[test]
    fn assembles_synthetic_complete_image_but_crc_remains_unverified() {
        let layout=sample_layout();
        let ilm=vec![0xa5;0x1ac80-0x5de]; let dlm=vec![0x55;0xc190];
        let vectors=CandidateVectorTable{initial_control_word:0x41020304,
            opaque_slots:[0x0480;127]};
        let img=assemble_structural_unverified(&layout,AuthoredPayloads{
            vector_table:Some(&vectors),ilm:Some(&ilm),dlm:Some(&dlm)}).unwrap();
        assert_eq!(img.bytes.len(),IMAGE_LENGTH);
        assert_eq!(&img.bytes[0..HEADER_BYTES],&layout.write_header());
        assert_eq!(&img.bytes[0x200..0x400],&vectors.write());
        assert!(img.bytes[0x400..0x450].iter().all(|&x|x==0));
        assert_eq!(&img.bytes[0x450..0x5de],
            &super::super::novatek_nds32::authored_startup_prefix(layout.ilm.actual_length().unwrap()).unwrap());
        assert_eq!(&img.bytes[0x5de..0x1ac80],&ilm);
        assert!(img.bytes[0x1ac80..0x22000].iter().all(|&x|x==0));
        assert_eq!(&img.bytes[0x22000..0x2e190],&dlm);
        assert!(img.bytes[0x2e190..0x2f000].iter().all(|&x|x==0));
        assert_eq!(&img.bytes[0x2f000..0x2f900],&dlm[..0x900]);
        assert!(img.bytes[0x2f900..0x2fffd].iter().all(|&x|x==0));
        assert_eq!(&img.bytes[0x2fffd..],b"NVT");
        assert!(!img.is_hardware_crc_verified());
    }
}
