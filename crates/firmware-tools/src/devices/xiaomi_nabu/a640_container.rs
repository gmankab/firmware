// SPDX-License-Identifier: GPL-3.0-only
//! Deterministic, deliberately incomplete signed A640 ELF/MBNv6 image.
//!
//! The ELF header, all QRISC/IR3 instructions, alignment, unsigned MBNv6
//! header, and SHA-384 segment hashes can be generated independently.
//! The OEM metadata, original signature and certificate chain remain absent.
//! The resulting ImagePlan refuses to emit a potentially misleading firmware.

use crate::{
    elf32_prefix::write_elf32_first_page,
    image_plan::ImagePlan,
    qcom_mbn::{sha384_table, V6Header, V6Metadata120},
};
use super::{a640_plan, elf_specs::KNOWN_ELFS};

pub const HEADER_BYTES: usize = 0x1000;
pub const AUTHENTICATION_OFFSET: usize = 0x1000;
pub const EXECUTABLE_OFFSET: usize = 0x3000;

/// A640-specific public MBNv6 header fields. These describe lengths and
/// placement only; they cannot authorize any firmware under secure boot.
fn mbn_header() -> V6Header {
    V6Header {
        image_id: 0,
        version: 6,
        qti_signature_size: 0,
        qti_certificate_size: 0,
        declared_payload_size: 0x1990,
        hash_table_size: 3 * 48,
        oem_signature_pointer: u32::MAX,
        oem_signature_size: 0x100,
        oem_certificate_pointer: u32::MAX,
        oem_certificate_size: 0x1800,
        qti_metadata_size: 0,
        oem_metadata_size: 0x78,
    }
}

/// Image-policy fields recovered from the signed image and expressed using
/// the public MBNv6 metadata format. Changing them would require a new valid
/// signature; these are not generated OEM signing credentials.
fn metadata() -> V6Metadata120 {
    let mut signed_policy = V6Metadata120 {
        software_id: 20,
        hardware_id: 0x0a50e1,
        oem_id: 0x72,
        policy_flags: V6Metadata120::FLAG_USE_SOC_VERSION
            | (1 << V6Metadata120::DEBUG_POLICY_SHIFT),
        anti_rollback_version: 1,
        ..Default::default()
    };
    signed_policy.supported_soc_versions[0] = 0x6003;
    signed_policy
}

/// Construct only bytes we can produce through structured code and hashing.
/// None of the missing authentication bytes are sourced from reference input.
pub fn partial_container() -> Result<ImagePlan, String> {
    let spec = KNOWN_ELFS.iter().find(|s| s.name == "a640_zap.mbn")
        .ok_or("A640 ELF description unavailable")?;
    if spec.headers.len() != 3 { return Err("A640 program-header count changed".into()); }
    let hash = &spec.headers[1];
    let code = &spec.headers[2];
    if hash.flags & 0x0700_0000 != 0x0200_0000 ||
       hash.offset as usize != AUTHENTICATION_OFFSET ||
       code.kind != 1 || code.flags & 1 == 0 ||
       code.offset as usize != EXECUTABLE_OFFSET ||
       code.filesz as usize != a640_plan::EXECUTABLE_SIZE {
        return Err("A640 metadata does not match executable/authentication segment model".into());
    }
    let authentication_end = (hash.offset as usize).checked_add(hash.filesz as usize)
        .ok_or("authentication extent overflow")?;
    let total = EXECUTABLE_OFFSET.checked_add(a640_plan::EXECUTABLE_SIZE)
        .ok_or("A640 image extent overflow")?;
    if authentication_end > EXECUTABLE_OFFSET {
        return Err("authentication segment overlaps executable".into());
    }

    let prefix = write_elf32_first_page(spec)?;
    if prefix.len() != HEADER_BYTES { return Err("unexpected ELF header page size".into()); }
    let program = a640_plan::partial_executable()?.build_complete()?;

    let signed_header = mbn_header();
    let signed_layout = signed_header.layout(hash.filesz as usize)?;
    if signed_layout.padding.len() != 0 || signed_layout.hashes.len() != 3 * 48 {
        return Err("unexpected A640 signed segment layout".into());
    }
    // For an ELF32 header without section records, the first digest covers
    // e_ehsize and all program headers, not the rest of the first zero page.
    let elf_header_len = 52usize.checked_add(spec.headers.len() * 32)
        .ok_or("ELF header digest length overflow")?;
    if elf_header_len > prefix.len() { return Err("ELF digest exceeds prefix".into()); }
    let oem_metadata = metadata().encode();
    if oem_metadata.len() != signed_layout.oem_metadata.len() {
        return Err("MBNv6 OEM metadata size mismatch".into());
    }
    let segment_hashes = sha384_table(&[Some(&prefix[..elf_header_len]), None, Some(&program)])?;
    if segment_hashes.len() != signed_layout.hashes.len() {
        return Err("MBNv6 digest count does not match ELF segments".into());
    }

    let mut plan = ImagePlan::new(total)?;
    plan.place("ELF32 metadata and program headers", 0, &prefix)?;
    plan.place("typed MBNv6 unsigned header", AUTHENTICATION_OFFSET,
        &signed_header.encode())?;
    plan.place("typed MBNv6 OEM image-policy metadata",
        AUTHENTICATION_OFFSET + signed_layout.oem_metadata.start, &oem_metadata)?;
    plan.place("SHA-384 digest table of generated ELF and executable",
        AUTHENTICATION_OFFSET + signed_layout.hashes.start, &segment_hashes)?;
    plan.place("deterministic zero ELF alignment padding", authentication_end,
        &vec![0; EXECUTABLE_OFFSET - authentication_end])?;
    plan.place("fully encoded QRISC and IR3 executable", EXECUTABLE_OFFSET, &program)?;
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_mbnv6_fields_and_hashes_do_not_make_signed_image_complete() {
        let plan = partial_container().unwrap();
        assert_eq!(plan.total_bytes(), 0x37b0);
        assert_eq!(plan.filled_bytes(), 4096 + 48 + 120 + 144 + 1480 + 1968);
        assert_eq!(plan.holes(), vec![0x1138..0x2a38]);
        assert!(plan.build_complete().unwrap_err().contains("incomplete"));
        assert_eq!(plan.regions().len(), 6);
    }

    #[test]
    fn generated_mbn_hashes_track_payload_bytes() {
        let plan = partial_container().unwrap();
        let expected = sha384_table(&[
            Some(&write_elf32_first_page(KNOWN_ELFS.iter()
                .find(|s| s.name == "a640_zap.mbn").unwrap()).unwrap()[..148]),
            None,
            Some(&a640_plan::partial_executable().unwrap().build_complete().unwrap()),
        ]).unwrap();
        let hashes = &plan.regions()[3];
        let mut bytes = vec![0u8; plan.total_bytes()];
        // compare_known validates generated bytes independently; only a
        // complete signed image can be emitted by build_complete().
        bytes[hashes.offset..hashes.offset + hashes.len].copy_from_slice(&expected);
        assert_eq!(hashes.offset, 0x10a8);
        assert_eq!(hashes.len, 144);
        assert!(plan.compare_known(&bytes).is_err());
    }
}
