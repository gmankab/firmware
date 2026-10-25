// SPDX-License-Identifier: GPL-3.0-only
//! Strict, sparse Novatek firmware reconstruction from independently supplied
//! ILM/DLM/configuration bytes and known structural constants. Unknown ranges
//! are never copied from a reference image or implicitly zero-filled.
//!
//! This module intentionally **does not** calculate or validate the Novatek
//! controller's stored CRC fields. Its complete output, when source payloads
//! are supplied, is a *structural image*, NOT a proven bootable firmware.

use super::novatek_layout::{CoverageRegion, NabuNovatekLayout, RegionKind};

/// Caller-authored executable/data bytes. `None` means source is missing and
/// MUST prevent complete generation (even if that region is zeros in an OEM
/// reference). Lengths are independently checked against the layout.
#[derive(Debug, Clone, Copy, Default)]
pub struct AuthoredPayloads<'a> {
    pub ilm: Option<&'a [u8]>,
    pub dlm: Option<&'a [u8]>,
    pub auxiliary: Option<&'a [u8]>,
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

/// Return the full gapless region plan. This never needs the original
/// firmware bytes (only typed header scalars and the Nabu layout rules).
pub fn projected_regions(layout: &NabuNovatekLayout) -> Result<Vec<ProjectedRegion>, String> {
    let regions = layout.coverage()?;
    Ok(regions.into_iter().map(|range| {
        let bytes = match range.kind {
            RegionKind::StructuredMetadataWithOpaqueCrc => Some(layout.write_header().to_vec()),
            RegionKind::IndependentlyGeneratedZero => Some(vec![0; range.length()]),
            RegionKind::IndependentlyGeneratedMarker => Some(layout.write_marker().to_vec()),
            RegionKind::UnimplementedIlm | RegionKind::UnimplementedDlm |
            RegionKind::UnimplementedAuxiliary => None,
        };
        ProjectedRegion {range,bytes}
    }).collect())
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
/// cannot substitute any source portion from it. Stored CRCs remain opaque.
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
        let part = match region.range.kind {
            RegionKind::UnimplementedIlm => check_supplied(&region.range,authored.ilm)?,
            RegionKind::UnimplementedDlm => check_supplied(&region.range,authored.dlm)?,
            RegionKind::UnimplementedAuxiliary => check_supplied(&region.range,authored.auxiliary)?,
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
        let ilm=CorePartition{file_offset:0x200,sram_address:0,size_minus_one:15,stored_crc:0x12345678};
        let dlm=CorePartition{file_offset:0x1000,sram_address:0x20000,size_minus_one:7,stored_crc:0xabcd4321};
        let info=InfoDescriptor{sram_address:0x20000,size_minus_one:0,file_offset:0x1000,stored_crc:0};
        NabuNovatekLayout{ilm,dlm,image_len:IMAGE_LENGTH,
            banks:[DescriptorBank{mode:3,options:0,info:[info;3],auxiliary:info},
                   DescriptorBank{mode:1,options:0,info:[info;3],auxiliary:info}]}
    }
    #[test]
    fn initial_projection_exposes_all_holes_without_filling_them() {
        let layout=sample_layout();
        let projection=projected_regions(&layout).unwrap();
        assert_eq!(projection.len(),7);
        assert_eq!(projection.iter().filter(|x|x.bytes.is_none()).count(),3);
        assert_eq!(projection[0].bytes.as_ref().unwrap().len(),HEADER_BYTES);
        assert_eq!(projection[2].bytes.as_ref().unwrap().len(),0xdf0);
        assert_eq!(projection[5].bytes.as_ref().unwrap().len(),1789);
        assert_eq!(projection[6].bytes.as_ref().unwrap(),b"NVT");
        let e=assemble_structural_unverified(&layout,AuthoredPayloads::default()).unwrap_err();
        assert!(e.contains("missing independently authored ILM"),"{e}");
    }
    #[test]
    fn refuses_missing_dlm_and_aux_even_when_other_payloads_exist() {
        let layout=sample_layout();
        let ilm=vec![3;16]; let dlm=vec![2;8];
        let e=assemble_structural_unverified(&layout,AuthoredPayloads{ilm:Some(&ilm),..Default::default()}).unwrap_err();
        assert!(e.contains("DLM"));
        let e=assemble_structural_unverified(&layout,AuthoredPayloads{ilm:Some(&ilm),dlm:Some(&dlm),auxiliary:None}).unwrap_err();
        assert!(e.contains("auxiliary"));
    }
    #[test]
    fn refuses_payloads_of_wrong_lengths_before_finalizing() {
        let layout=sample_layout();
        let incorrect=vec![0;17];
        let e=assemble_structural_unverified(&layout,AuthoredPayloads{ilm:Some(&incorrect),..Default::default()}).unwrap_err();
        assert!(e.contains("length 17 != required 16"));
    }
    #[test]
    fn assembles_synthetic_complete_image_but_crc_remains_unverified() {
        let layout=sample_layout();
        let ilm=vec![0xa5;16]; let dlm=vec![0x55;8];
        let auxiliary=vec![0x35; 0x2f900-0x1008];
        let img=assemble_structural_unverified(&layout,AuthoredPayloads{
            ilm:Some(&ilm),dlm:Some(&dlm),auxiliary:Some(&auxiliary)}).unwrap();
        assert_eq!(img.bytes.len(),IMAGE_LENGTH);
        assert_eq!(&img.bytes[0..HEADER_BYTES],&layout.write_header());
        assert_eq!(&img.bytes[0x200..0x210],&ilm);
        assert!(img.bytes[0x210..0x1000].iter().all(|&x|x==0));
        assert_eq!(&img.bytes[0x1000..0x1008],&dlm);
        assert_eq!(&img.bytes[0x1008..0x2f900],&auxiliary);
        assert!(img.bytes[0x2f900..0x2fffd].iter().all(|&x|x==0));
        assert_eq!(&img.bytes[0x2fffd..],b"NVT");
        assert!(!img.is_hardware_crc_verified());
    }
}
