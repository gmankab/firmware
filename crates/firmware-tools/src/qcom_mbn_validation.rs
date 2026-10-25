// SPDX-License-Identifier: GPL-3.0-only
//! Read-only, device-independent validation of MBNv6 ELF32 segment hashes.
//!
//! A passing digest check confirms only that the embedded table agrees with
//! file-backed ELF segments. It does NOT validate a Qualcomm/OEM signature,
//! establish trust-chain provenance, or authorize deployment on hardware.

use crate::{parse_elf32, ElfHeader, image_plan::ImagePlan,
    qcom_mbn::{sha384_table, V6Header, V6Layout, SHA384_SIZE}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashAudit {
    pub segment_count: usize,
    pub hashed_count: usize,
    pub zero_placeholder_count: usize,
    pub hash_segment_file_offset: usize,
    pub hash_segment_length: usize,
    pub signed_layout: V6Layout,
    pub has_signatures: bool,
}

/// Refuse malformed ELF hash geometry before reading reference bytes or
/// accepting generated segment bytes. BSS (p_filesz == 0) contributes no file
/// bytes even when its p_offset is beyond EOF, as seen in original Nabu MBNs.
/// The signed MBNv6 hash/authentication region must never overlap a segment
/// that is independently hashed: such overlap would mix signing bytes with
/// source program bytes and make provenance ambiguous.
fn check_hash_input_geometry(elf: &ElfHeader, hash_segment_index: usize) -> Result<(), String> {
    let hash = elf.program_headers.get(hash_segment_index)
        .ok_or("invalid MBNv6 hash-segment program-header index")?;
    if hash.flags & 0x0700_0000 != 0x0200_0000 {
        return Err("selected ELF program header is not a Qualcomm hash segment".into());
    }
    let hash_start = hash.offset as usize;
    let hash_end = hash_start.checked_add(hash.file_size as usize)
        .ok_or("MBNv6 hash segment offset overflow")?;
    for (index, segment) in elf.program_headers.iter().enumerate() {
        if segment.kind == 1 && segment.file_size > segment.mem_size {
            return Err(format!("ELF PT_LOAD index {index} has p_filesz greater than p_memsz"));
        }
        if index == hash_segment_index || segment.file_size == 0 { continue; }
        let start = segment.offset as usize;
        let end = start.checked_add(segment.file_size as usize)
            .ok_or("ELF segment offset overflow")?;
        if start < hash_end && hash_start < end {
            return Err(format!("ELF segment {index} overlaps signed MBNv6 hash/authentication segment"));
        }
    }
    Ok(())
}

/// Build SHA-384 hashes **only** from independently generated, file-backed
/// ELF segment bytes. The hash segment and explicit BSS (p_filesz == 0)
/// produce zero-digest placeholders. Unknown bytes or imported signatures
/// inside any hashed segment make the build fail with a first-hole error.
/// This does not authenticate the resulting image or its signer.
pub fn sha384_table_from_generated_plan(
    elf: &ElfHeader, hash_segment_index: usize, plan: &ImagePlan,
) -> Result<Vec<u8>, String> {
    if elf.program_headers.is_empty() || hash_segment_index >= elf.program_headers.len() {
        return Err("invalid MBNv6 hash-segment program-header index".into());
    }
    check_hash_input_geometry(elf, hash_segment_index)?;
    let mut entries = Vec::with_capacity(elf.program_headers.len());
    for (index, segment) in elf.program_headers.iter().enumerate() {
        if index == hash_segment_index || segment.file_size == 0 {
            entries.push(None);
            continue;
        }
        if segment.kind == 1 && segment.file_size > segment.mem_size {
            return Err(format!("ELF PT_LOAD index {index} has p_filesz greater than p_memsz"));
        }
        let start = segment.offset as usize;
        let end = start.checked_add(segment.file_size as usize)
            .ok_or("ELF segment offset overflow")?;
        entries.push(Some(plan.generated_slice(start..end)
            .map_err(|reason| format!("ELF segment {index}: {reason}"))?));
    }
    sha384_table(&entries.iter().map(|entry| entry.as_deref()).collect::<Vec<_>>())
}

pub fn verify_elf32_mbnv6_hashes(image: &[u8]) -> Result<HashAudit, String> {
    let elf = parse_elf32(image)?;
    if elf.program_headers.is_empty() {
        return Err("ELF contains no program headers".into());
    }
    let mut hash_segments = crate::qcom_mdt_hash_segments(&elf);
    if hash_segments.len() != 1 {
        return Err(format!("expected one MBNv6 hash segment, found {}", hash_segments.len()));
    }
    let (hash_index, hash_segment) = hash_segments.remove(0);
    check_hash_input_geometry(&elf, hash_index)?;
    let hash_start = hash_segment.offset as usize;
    let hash_end = hash_start.checked_add(hash_segment.file_size as usize)
        .ok_or("MBNv6 signed region offset overflow")?;
    let signed_bytes = image.get(hash_start..hash_end)
        .ok_or("MBNv6 signed region extends beyond ELF")?;
    let header = V6Header::parse(signed_bytes)?;
    let signed_layout = header.layout(signed_bytes.len())?;
    let actual_hashes = &signed_bytes[signed_layout.hashes.clone()];
    if actual_hashes.len() != elf.program_headers.len() * SHA384_SIZE {
        return Err("MBNv6 hash table entry count disagrees with ELF program headers".into());
    }

    let mut entries: Vec<Option<&[u8]>> = Vec::with_capacity(elf.program_headers.len());
    for (idx, segment) in elf.program_headers.iter().enumerate() {
        if idx == hash_index || segment.file_size == 0 {
            entries.push(None);
            continue;
        }
        let start = segment.offset as usize;
        let end = start.checked_add(segment.file_size as usize)
            .ok_or("ELF segment offset overflow")?;
        let data = image.get(start..end).ok_or("ELF segment extends beyond image")?;
        entries.push(Some(data));
    }
    let expected = sha384_table(&entries)?;
    for (idx, (actual, generated)) in actual_hashes.chunks_exact(SHA384_SIZE)
        .zip(expected.chunks_exact(SHA384_SIZE)).enumerate() {
        if actual != generated {
            return Err(format!("MBNv6 SHA-384 mismatch at ELF program-header index {idx}"));
        }
    }
    Ok(HashAudit {
        segment_count: entries.len(),
        hashed_count: entries.iter().filter(|entry| entry.is_some()).count(),
        zero_placeholder_count: entries.iter().filter(|entry| entry.is_none()).count(),
        hash_segment_file_offset: hash_start,
        hash_segment_length: signed_bytes.len(),
        has_signatures: signed_layout.oem_signature.len() != 0
            || signed_layout.qti_signature.len() != 0,
        signed_layout,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn put16(b: &mut [u8], offset: usize, value: u16) {
        b[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }
    fn put32(b: &mut [u8], offset: usize, value: u32) {
        b[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    fn fixture() -> Vec<u8> {
        // Illustrative format-only test firmware. No vendor bytes or keys.
        let mut b = vec![0; 0x220];
        b[..4].copy_from_slice(b"\x7fELF");
        b[4] = 1; b[5] = 1; b[6] = 1;
        put16(&mut b, 16, 2);
        put16(&mut b, 18, 164);
        put32(&mut b, 20, 1);
        put32(&mut b, 28, 52);
        put16(&mut b, 40, 52);
        put16(&mut b, 42, 32);
        put16(&mut b, 44, 3);
        let p0 = 52;
        put32(&mut b, p0 + 16, 148);
        put32(&mut b, p0 + 24, 0x0700_0000);
        let p1 = 84;
        put32(&mut b, p1 + 4, 0x100);
        put32(&mut b, p1 + 16, 48 + 3 * 48);
        put32(&mut b, p1 + 24, 0x0200_0000);
        let p2 = 116;
        put32(&mut b, p2, 1);
        put32(&mut b, p2 + 4, 0x200);
        put32(&mut b, p2 + 16, 32);
        put32(&mut b, p2 + 20, 32);
        put32(&mut b, p2 + 24, 5);
        for i in 0..32 { b[0x200 + i] = (i as u8).wrapping_mul(7).wrapping_add(1); }
        let header = V6Header {
            image_id: 0, version: 6, qti_signature_size: 0,
            qti_certificate_size: 0, declared_payload_size: 3 * 48,
            hash_table_size: 3 * 48, oem_signature_pointer: u32::MAX,
            oem_signature_size: 0, oem_certificate_pointer: u32::MAX,
            oem_certificate_size: 0, qti_metadata_size: 0, oem_metadata_size: 0,
        };
        b[0x100..0x130].copy_from_slice(&header.encode());
        let hashes = sha384_table(&[Some(&b[..148]), None, Some(&b[0x200..0x220])]).unwrap();
        b[0x130..0x1c0].copy_from_slice(&hashes);
        b
    }
    #[test]
    fn reconstructs_hashes_only_from_source_generated_file_bytes() {
        let original = fixture();
        let elf = parse_elf32(&original).unwrap();
        let mut plan = ImagePlan::new(original.len()).unwrap();
        plan.place("header", 0, &original[..148]).unwrap();
        plan.place("generated program", 0x200, &original[0x200..0x220]).unwrap();
        let rebuilt = sha384_table_from_generated_plan(&elf, 1, &plan).unwrap();
        assert_eq!(rebuilt, original[0x130..0x1c0]);
        assert!(sha384_table_from_generated_plan(&elf, 0, &plan).unwrap_err().contains("hash segment"));
        let mut incomplete = ImagePlan::new(original.len()).unwrap();
        incomplete.place("partial header", 0, &original[..30]).unwrap();
        incomplete.place("program", 0x200, &original[0x200..0x220]).unwrap();
        assert!(sha384_table_from_generated_plan(&elf, 1, &incomplete)
            .unwrap_err().contains("unimplemented file byte at 0x1e"));
        let mut external = ImagePlan::new(original.len()).unwrap();
        external.place_external_authentication("header copied", 0, &original[..148]).unwrap();
        external.place("program", 0x200, &original[0x200..0x220]).unwrap();
        assert!(sha384_table_from_generated_plan(&elf, 1, &external)
            .unwrap_err().contains("externally supplied"));
        let mut bogus = elf;
        bogus.program_headers[2].mem_size = 1;
        assert!(sha384_table_from_generated_plan(&bogus, 1, &plan)
            .unwrap_err().contains("greater than"));
    }

    #[test]
    fn validates_generic_elf32_mbnv6_and_rejects_changed_payload() {
        let mut firmware = fixture();
        let report = verify_elf32_mbnv6_hashes(&firmware).unwrap();
        assert_eq!(report.segment_count, 3);
        assert_eq!(report.hashed_count, 2);
        assert_eq!(report.zero_placeholder_count, 1);
        assert!(!report.has_signatures);
        firmware[0x20a] ^= 0x40;
        assert!(verify_elf32_mbnv6_hashes(&firmware).unwrap_err().contains("index 2"));
        firmware = fixture();
        firmware[0x130] ^= 1;
        assert!(verify_elf32_mbnv6_hashes(&firmware).unwrap_err().contains("index 0"));
    }
    #[test]
    fn rejects_pt_load_p_filesz_greater_than_p_memsz() {
        let mut firmware = fixture();
        put32(&mut firmware, 116 + 20, 31); // existing 32-byte PT_LOAD program
        assert!(verify_elf32_mbnv6_hashes(&firmware).unwrap_err()
            .contains("p_filesz greater than p_memsz"));
        let elf = parse_elf32(&firmware).unwrap();
        let plan = ImagePlan::new(firmware.len()).unwrap();
        assert!(sha384_table_from_generated_plan(&elf, 1, &plan).unwrap_err()
            .contains("p_filesz greater than p_memsz"));
    }

    #[test]
    fn rejects_segments_overlapping_signed_hash_or_authentication() {
        let mut firmware = fixture();
        put32(&mut firmware, 116 + 4, 0x180); // malicious program inside hash region
        let reason = verify_elf32_mbnv6_hashes(&firmware).unwrap_err();
        assert!(reason.contains("overlaps signed MBNv6 hash"), "{reason}");
        let elf = parse_elf32(&firmware).unwrap();
        let plan = ImagePlan::new(firmware.len()).unwrap();
        assert!(sha384_table_from_generated_plan(&elf, 1, &plan).unwrap_err()
            .contains("overlaps signed MBNv6 hash"));
        // A zero-file-size BSS segment with offset beyond EOF is *not*
        // an overlapping file-backed program; it must remain accepted.
        put32(&mut firmware, 116 + 16, 0);
        put32(&mut firmware, 116 + 4, 0xff00);
        let elf = parse_elf32(&firmware).unwrap();
        let mut plan = ImagePlan::new(firmware.len()).unwrap();
        plan.place("original ELF header", 0, &firmware[..148]).unwrap();
        assert!(sha384_table_from_generated_plan(&elf, 1, &plan).is_ok());
    }

    #[test]
    fn rejects_bad_format_and_truncated_authentication() {
        let mut firmware = fixture();
        firmware[0x104] = 7;
        assert!(verify_elf32_mbnv6_hashes(&firmware).is_err());
        firmware = fixture();
        firmware.truncate(0x150);
        assert!(verify_elf32_mbnv6_hashes(&firmware).is_err());
        firmware = fixture();
        firmware[84 + 24..84 + 28].copy_from_slice(&0u32.to_le_bytes());
        assert!(verify_elf32_mbnv6_hashes(&firmware).is_err());
    }
}
