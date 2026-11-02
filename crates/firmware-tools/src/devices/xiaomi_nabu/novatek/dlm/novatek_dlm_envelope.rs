// SPDX-License-Identifier: GPL-3.0-only
//! Strict Agent 20-19 envelope verification for the postmarketOS Nabu profile.
//!
//! This module accepts one image and derives all parser state from that image.
//! It does not accept a caller-supplied layout or label externally supplied
//! metadata, vectors, DLM contents, or their mirror as independently authored.
//! Exact reference SHA-256 checking belongs to the outer comparison command;
//! this verifier also accepts synthetic/source-generated payloads that obey
//! the measured profile, existing authored boot prefix, and checksum rules.
//!
//! The non-CRC metadata recipe below was measured from the separately held
//! 196,608-byte postmarketOS reference with SHA-256
//! 8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5.
//! It records external scalar metadata, not a recovered firmware algorithm.
//! No original vector values or proprietary payload bytes are stored here.

use super::novatek_crc::{verify_all_crcs, CrcAudit};
use super::novatek_layout::{
    CorePartition, DescriptorBank, InfoDescriptor, NabuNovatekLayout, BANK_BYTES,
    CANDIDATE_VECTORS_END, DLM_FILE_END, DLM_FILE_START, DLM_MIRROR_BYTES,
    END_MARKER_OFFSET, HEADER_BYTES, ILM_CODE_START, ILM_SOURCE_PREFIX_END,
    ILM_SPACER_START, IMAGE_LENGTH, POST_DLM_PADDING_END, RESERVED_TAIL_START,
};
use super::novatek_vector::{CandidateVectorTable, VECTOR_COUNT};

const DLM_SRAM_START: u32 = 0x20000;
const INFO_DLM_OFFSETS: [[u32; 3]; 2] = [[0, 0xa00, 0x3200], [0, 0x4a00, 0x7200]];

/// Independent checks performed on one internally parsed image.
/// These counts describe verified categories, not full source reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeAudit {
    pub crc: CrcAudit,
    pub source_executable_bytes: usize,
    pub envelope_bytes: usize,
    /// Header plus vector scalars remain externally supplied metadata.
    pub metadata_bytes: usize,
    /// This is a dependent copy of DLM data, not independent source coverage.
    pub mirror_bytes: usize,
    pub vector_slots: usize,
}

/// Fixed non-CRC metadata geometry. Checksum fields are placeholders only;
/// callers must compute them from their complete supplied payload.
fn metadata_profile() -> NabuNovatekLayout {
    let ilm = CorePartition {
        file_offset: HEADER_BYTES as u32,
        sram_address: 0,
        size_minus_one: (ILM_SPACER_START - HEADER_BYTES - 1) as u32,
        stored_crc: 0,
    };
    let dlm = CorePartition {
        file_offset: DLM_FILE_START as u32,
        sram_address: DLM_SRAM_START,
        size_minus_one: (DLM_FILE_END - DLM_FILE_START - 1) as u32,
        stored_crc: 0,
    };
    let bank = |index: usize| DescriptorBank {
        mode: if index == 0 { 3 } else { 1 },
        options: if index == 0 { 0x5381_10e0 } else { 0 },
        info: INFO_DLM_OFFSETS[index].map(|offset| InfoDescriptor {
            sram_address: DLM_SRAM_START + offset,
            // Each of these profile descriptors checks exactly one byte.
            size_minus_one: 0,
            file_offset: DLM_FILE_START as u32 + offset,
            stored_crc: 0,
        }),
        auxiliary: InfoDescriptor {
            sram_address: DLM_SRAM_START
                + (DLM_FILE_END - DLM_FILE_START + index * BANK_BYTES) as u32,
            size_minus_one: (BANK_BYTES - 1) as u32,
            file_offset: (index * BANK_BYTES) as u32,
            // This footer covers the preceding 252 metadata bytes, rather
            // than the byte range encoded by its auxiliary descriptor.
            stored_crc: 0,
        },
    };
    NabuNovatekLayout {
        ilm,
        dlm,
        banks: [bank(0), bank(1)],
        image_len: IMAGE_LENGTH,
    }
}

fn same_scalar(name: &str, actual: u32, expected: u32) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "postmarketOS profile {name}: 0x{actual:x} != 0x{expected:x}"
        ));
    }
    Ok(())
}

fn same_descriptor(
    name: &str,
    actual: &InfoDescriptor,
    expected: &InfoDescriptor,
) -> Result<(), String> {
    same_scalar(&format!("{name} SRAM address"), actual.sram_address, expected.sram_address)?;
    same_scalar(&format!("{name} size_minus_one"), actual.size_minus_one, expected.size_minus_one)?;
    same_scalar(&format!("{name} file offset"), actual.file_offset, expected.file_offset)
}

fn validate_profile(layout: &NabuNovatekLayout) -> Result<(), String> {
    let expected = metadata_profile();
    if layout.image_len != IMAGE_LENGTH {
        return Err("postmarketOS profile image length mismatch".into());
    }
    for (name, actual, wanted) in [
        ("ILM", &layout.ilm, &expected.ilm),
        ("DLM", &layout.dlm, &expected.dlm),
    ] {
        same_scalar(&format!("{name} file offset"), actual.file_offset, wanted.file_offset)?;
        same_scalar(&format!("{name} SRAM address"), actual.sram_address, wanted.sram_address)?;
        same_scalar(&format!("{name} size_minus_one"), actual.size_minus_one, wanted.size_minus_one)?;
    }
    for (index, (actual, wanted)) in layout.banks.iter().zip(&expected.banks).enumerate() {
        same_scalar(&format!("bank {index} mode"), actual.mode, wanted.mode)?;
        same_scalar(&format!("bank {index} options"), actual.options, wanted.options)?;
        for (entry, (a, e)) in actual.info.iter().zip(&wanted.info).enumerate() {
            same_descriptor(&format!("bank {index} info {entry}"), a, e)?;
        }
        same_descriptor(&format!("bank {index} footer"), &actual.auxiliary, &wanted.auxiliary)?;
    }
    Ok(())
}

/// Validate the measured profile, all candidate-vector bounds, the inherited
/// 538-byte authored boot prefix, reserved zeros, DLM mirror, suffix, and all
/// 12 stored CRC fields. Checksums establish consistency only.
pub fn verify_envelope(image: &[u8]) -> Result<EnvelopeAudit, String> {
    // Validate the physical input size before any legacy fixed-offset helper.
    if image.len() != IMAGE_LENGTH {
        return Err(format!(
            "postmarketOS envelope length {} != {IMAGE_LENGTH}", image.len()
        ));
    }
    let layout = NabuNovatekLayout::parse(image)?;
    validate_profile(&layout)?;
    CandidateVectorTable::parse(image, &layout)?;
    // Its layout was just parsed from the same image and checked against the
    // exact profile, so legacy slice helpers cannot receive fabricated bounds.
    layout.verify_envelope(image)?;
    let crc = verify_all_crcs(&layout, image)?;
    Ok(EnvelopeAudit {
        crc,
        source_executable_bytes: ILM_SOURCE_PREFIX_END - ILM_CODE_START,
        envelope_bytes: (ILM_CODE_START - CANDIDATE_VECTORS_END)
            + (DLM_FILE_START - ILM_SPACER_START)
            + (POST_DLM_PADDING_END - DLM_FILE_END)
            + (END_MARKER_OFFSET - RESERVED_TAIL_START)
            + 3,
        metadata_bytes: CANDIDATE_VECTORS_END,
        mirror_bytes: DLM_MIRROR_BYTES,
        vector_slots: VECTOR_COUNT,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::novatek_crc::{crc32_mpeg2, recompute_all_crcs};
    use super::super::novatek_nds32::authored_boot_prefix;
    use super::super::novatek_layout::DLM_MIRROR_START;

    fn set_word(image: &mut [u8], offset: usize, word: u32) {
        image[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
    }

    // Synthetic fixture: no proprietary firmware payload/vector words.
    fn synthetic_profile_image() -> Vec<u8> {
        let layout = metadata_profile();
        let mut image = vec![0; IMAGE_LENGTH];
        image[..HEADER_BYTES].copy_from_slice(&layout.write_header());
        let vectors = CandidateVectorTable {
            initial_control_word: 0x0102_0304,
            opaque_slots: [0; VECTOR_COUNT - 1],
        };
        image[HEADER_BYTES..CANDIDATE_VECTORS_END].copy_from_slice(&vectors.write());
        image[ILM_CODE_START..ILM_SOURCE_PREFIX_END].copy_from_slice(
            &authored_boot_prefix(layout.ilm.actual_length().unwrap()).unwrap(),
        );
        image[DLM_FILE_START] = 0x41;
        image[DLM_MIRROR_START] = 0x41;
        image[END_MARKER_OFFSET..].copy_from_slice(b"NVT");
        recompute_all_crcs(&mut image).unwrap();
        image
    }

    #[test]
    fn accepts_consistent_synthetic_profile_without_claiming_payload_provenance() {
        let image = synthetic_profile_image();
        let audit = verify_envelope(&image).unwrap();
        assert_eq!(audit.crc.total_checks(), 10);
        assert_eq!(audit.crc.stored_fields(), 12);
        assert_eq!(audit.source_executable_bytes, 538);
        assert_eq!(audit.envelope_bytes, 35136);
        assert_eq!(audit.metadata_bytes, 1024);
        assert_eq!(audit.mirror_bytes, 2304);
        assert_eq!(audit.vector_slots, 128);
    }

    #[test]
    fn rejects_forged_descriptor_even_when_stale_layout_crc_audit_passes() {
        let mut image = synthetic_profile_image();
        let stale = NabuNovatekLayout::parse(&image).unwrap();
        set_word(&mut image, 0x38, DLM_FILE_START as u32 + 1);
        let footer = crc32_mpeg2(&image[..0xfc]);
        set_word(&mut image, 0xfc, footer);
        // Demonstrate why caller-provided parser state must not be accepted:
        // the inherited API checks the old descriptor, not the current one.
        assert!(verify_all_crcs(&stale, &image).is_ok());
        assert!(verify_envelope(&image).unwrap_err().contains("bank 0 info 0 file offset"));
    }

    #[test]
    fn rejects_out_of_range_vector_with_recomputed_valid_crc_fields() {
        let mut image = synthetic_profile_image();
        set_word(&mut image, HEADER_BYTES + 4, 0xdead_beef);
        recompute_all_crcs(&mut image).unwrap();
        assert!(verify_envelope(&image).unwrap_err().contains("outside ILM SRAM"));
    }

    #[test]
    fn rejects_wrong_sram_and_shortened_dlm_after_checksum_repair() {
        for (field, value, message) in [
            (0x04, 4, "ILM SRAM address"),
            (0x10, DLM_SRAM_START + 0x100, "DLM SRAM address"),
            (0x14, (DLM_FILE_END - DLM_FILE_START - 2) as u32, "DLM size_minus_one"),
        ] {
            let mut image = synthetic_profile_image();
            set_word(&mut image, field, value);
            set_word(&mut image, BANK_BYTES + field, value);
            recompute_all_crcs(&mut image).unwrap();
            assert!(verify_envelope(&image).unwrap_err().contains(message));
        }
    }

    #[test]
    fn rejects_unknown_bank_scalar_even_if_its_crc_is_recomputed() {
        let mut image = synthetic_profile_image();
        set_word(&mut image, BANK_BYTES + 0x24, 1);
        recompute_all_crcs(&mut image).unwrap();
        assert!(verify_envelope(&image).unwrap_err().contains("bank 1 options"));
    }

    #[test]
    fn truncated_or_oversized_images_return_errors_without_panicking() {
        let image = synthetic_profile_image();
        for length in [0, 1, HEADER_BYTES - 1, CANDIDATE_VECTORS_END - 1, IMAGE_LENGTH - 1] {
            assert!(verify_envelope(&image[..length]).is_err());
        }
        let mut oversized = image;
        oversized.push(0);
        assert!(verify_envelope(&oversized).is_err());
    }

    #[test]
    fn rejects_boot_padding_and_mirror_mutations_independently_of_crc() {
        for offset in [ILM_CODE_START + 1, ILM_SPACER_START, DLM_MIRROR_START + 5] {
            let mut image = synthetic_profile_image();
            image[offset] ^= 1;
            recompute_all_crcs(&mut image).unwrap();
            assert!(verify_envelope(&image).is_err(), "accepted tamper at {offset:#x}");
        }
    }
}
