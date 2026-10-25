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
    qcom_mbn::{V6Header, V6Metadata120},
    qcom_mbn_validation::sha384_table_from_generated_plan,
    parse_elf32,
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
    if elf_header_len != spec.headers[0].filesz as usize {
        return Err("A640 ELF header hash span changed".into());
    }
    let oem_metadata = metadata().encode();
    if oem_metadata.len() != signed_layout.oem_metadata.len() {
        return Err("MBNv6 OEM metadata size mismatch".into());
    }
    let mut plan = ImagePlan::new(total)?;
    plan.place("ELF32 metadata and program headers", 0, &prefix)?;
    plan.place("typed MBNv6 unsigned header", AUTHENTICATION_OFFSET,
        &signed_header.encode())?;
    plan.place("typed MBNv6 OEM image-policy metadata",
        AUTHENTICATION_OFFSET + signed_layout.oem_metadata.start, &oem_metadata)?;
    plan.place("deterministic zero ELF alignment padding", authentication_end,
        &vec![0; EXECUTABLE_OFFSET - authentication_end])?;
    plan.place("fully encoded QRISC and IR3 executable", EXECUTABLE_OFFSET, &program)?;

    // The source-generated spans are the only permissible inputs to the
    // hash builder. This avoids accidentally hashing imported reference or
    // signed-authentication bytes and makes future missing ELF text fatal.
    let elf = parse_elf32(&prefix)?;
    let segment_hashes = sha384_table_from_generated_plan(&elf, 1, &plan)?;
    if segment_hashes.len() != signed_layout.hashes.len() {
        return Err("MBNv6 digest count does not match ELF segments".into());
    }
    plan.place("SHA-384 digest table of generated ELF and executable",
        AUTHENTICATION_OFFSET + signed_layout.hashes.start, &segment_hashes)?;
    Ok(plan)
}


/// Bytes supplied separately by the device owner, never stored in the source
/// repository. The input must contain *exactly* the OEM signature followed by
/// the fixed-width certificate-chain field; it is not a private signing key.
pub const EXTERNAL_AUTH_SIZE: usize = 0x1900;

/// Reassemble the original signed A640 container using independently generated
/// data plus externally supplied OEM authentication bytes.
///
/// This function checks geometry and segment hashes, but **does not** prove
/// that the imported certificate root is trusted by the device or that the
/// signature is valid. A changed payload requires an independently authorized
/// signature. Do not call this output newly signed firmware.
pub fn assemble_with_external_authentication(external_auth: &[u8]) -> Result<Vec<u8>, String> {
    let mut plan = partial_container()?;
    let expected = 0x1138..0x2a38;
    if plan.holes() != vec![expected.clone()] {
        return Err("unexpected missing-byte layout in A640 signed container".into());
    }
    if external_auth.len() != EXTERNAL_AUTH_SIZE {
        return Err(format!(
            "A640 OEM authentication input must be exactly {} bytes; received {}",
            EXTERNAL_AUTH_SIZE, external_auth.len()
        ));
    }
    plan.place_external_authentication("externally supplied OEM signature and certificate field",
        expected.start, external_auth)?;
    let image = plan.build_complete()?;
    crate::qcom_mbn_validation::verify_elf32_mbnv6_hashes(&image)?;
    Ok(image)
}

/// External-auth bytes are *borrowed from the original reference* solely
/// to prove that every other byte can be regenerated. Passing this check is
/// **not** equivalent to deriving the signature, validating device secure
/// boot policy, or producing a new signed firmware release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct A640ReferenceAudit {
    pub original_file_bytes: usize,
    pub independently_generated_file_bytes: usize,
    pub original_oem_authentication_bytes: usize,
    pub authentication_file_range: std::ops::Range<usize>,
    pub matching_file_bytes: usize,
}

/// Reconstruct a complete bytewise comparison image *in memory* using only
/// externally borrowed OEM authentication. Refuse a mismatching revision at
/// the first offset and leave the private reference untouched.
pub fn audit_complete_reference(reference: &[u8]) -> Result<A640ReferenceAudit, String> {
    let partial = partial_container()?;
    let holes = partial.holes();
    if holes.len() != 1 || holes[0] != (0x1138..0x2a38) {
        return Err("unexpected A640 authentication boundary".into());
    }
    if reference.len() != partial.total_bytes() {
        return Err(format!("wrong A640 reference revision or length: expected {}, got {}",
            partial.total_bytes(), reference.len()));
    }
    // This compares all source-generated regions before referencing any
    // signature bytes; no unknown firmware byte is allowed to match by fiat.
    partial.compare_known(reference)?;
    let borrowed_auth = &reference[holes[0].clone()];
    let result = assemble_with_external_authentication(borrowed_auth)?;
    if let Some(offset) = result.iter().zip(reference).position(|(a,b)| a != b) {
        return Err(format!("full A640 reassembly differs at file offset 0x{offset:x}"));
    }
    let coverage = partial.coverage();
    Ok(A640ReferenceAudit {
        original_file_bytes: reference.len(),
        independently_generated_file_bytes: coverage.generated_bytes,
        original_oem_authentication_bytes: borrowed_auth.len(),
        authentication_file_range: holes[0].clone(),
        matching_file_bytes: result.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qcom_mbn::sha384_table;

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
    fn external_authentication_is_exactly_bounded_and_never_guessed() {
        assert!(assemble_with_external_authentication(&[])
            .unwrap_err().contains("exactly 6400 bytes"));
        assert!(assemble_with_external_authentication(&vec![0x55; EXTERNAL_AUTH_SIZE - 1])
            .unwrap_err().contains("exactly 6400 bytes"));
        assert!(assemble_with_external_authentication(&vec![0x55; EXTERNAL_AUTH_SIZE + 1])
            .unwrap_err().contains("exactly 6400 bytes"));
        let original = partial_container().unwrap();
        assert_eq!(original.holes(), vec![0x1138..0x2a38]);
        // Synthetic bytes exercise the layout without embedding OEM material.
        // Segment hashes still match; vendor signature validity is NOT tested.
        let completed = assemble_with_external_authentication(&vec![0x55; EXTERNAL_AUTH_SIZE]).unwrap();
        assert_eq!(completed.len(), 0x37b0);
        original.compare_known(&completed).unwrap();
        assert_eq!(&completed[0x1138..0x2a38], &vec![0x55; EXTERNAL_AUTH_SIZE]);
        let mut derived = partial_container().unwrap();
        derived.place_external_authentication("OEM test-only bytes", 0x1138,
            &vec![0x55; EXTERNAL_AUTH_SIZE]).unwrap();
        assert_eq!(derived.coverage().generated_bytes, 7856);
        assert_eq!(derived.coverage().externally_supplied_bytes, 6400);
        assert!(derived.build_independently_generated().is_err());
    }

    #[test]
    fn reference_audit_rejects_wrong_revision_and_modified_generated_bytes() {
        let mut reference=assemble_with_external_authentication(&vec![0x44;EXTERNAL_AUTH_SIZE]).unwrap();
        let stats=audit_complete_reference(&reference).unwrap();
        assert_eq!(stats.original_file_bytes,0x37b0);
        assert_eq!(stats.independently_generated_file_bytes,7856);
        assert_eq!(stats.original_oem_authentication_bytes,6400);
        assert_eq!(stats.matching_file_bytes,0x37b0);
        assert_eq!(stats.authentication_file_range,0x1138..0x2a38);
        assert!(audit_complete_reference(&reference[..reference.len()-1]).is_err());
        reference[0x3000]^=1;
        assert!(audit_complete_reference(&reference).unwrap_err().contains("byte mismatch"));
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
        let hashes = plan.regions().iter()
            .find(|r| r.name == "SHA-384 digest table of generated ELF and executable").unwrap();
        let mut bytes = vec![0u8; plan.total_bytes()];
        // compare_known validates generated bytes independently; only a
        // complete signed image can be emitted by build_complete().
        bytes[hashes.offset..hashes.offset + hashes.len].copy_from_slice(&expected);
        assert_eq!(hashes.offset, 0x10a8);
        assert_eq!(hashes.len, 144);
        assert!(plan.compare_known(&bytes).is_err());
    }
}
