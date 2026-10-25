// SPDX-License-Identifier: GPL-3.0-only
//! Firmware construction and inspection tools: formats, metadata builders and the
//! reconstructed text configuration. This is not a complete executable
//! firmware replacement.
pub mod a640_microcode;
pub use ir3;
pub use qrisc::{codec as qrisc_codec, program as qrisc_program, a640 as a640_routines};
pub mod image_plan;
pub mod sha384;
pub mod qcom_mbn;
pub mod qcom_mbn_validation;
pub mod elf32_prefix;
pub mod devices;
pub use devices::xiaomi_nabu::modemuw::build_modemuw;
pub mod wmfw_metadata;
pub mod wmfw_plan;
pub mod wmfw_algorithms;
pub mod wmfw_halo_memory;
pub mod wmfw_sha256;
pub mod wmfw_external_payload;
pub mod wmfw_memory_map;
pub mod wmfw_control_ranges;

/// Parse only the bounded ELF32 little-endian header and segment table.
/// Neither this metadata inspector nor its output is an executable decompiler.
#[derive(Debug, PartialEq, Eq)]
pub struct ElfHeader {
    pub machine: u16,
    pub entry: u32,
    pub program_headers: Vec<ProgramHeader>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ProgramHeader {
    pub kind: u32,
    pub offset: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub file_size: u32,
    pub mem_size: u32,
    pub flags: u32,
}
fn read_u16(b: &[u8], i: usize) -> Result<u16, String> {
    let x = b.get(i..i.checked_add(2).ok_or("offset overflow")?).ok_or("truncated ELF")?;
    Ok(u16::from_le_bytes(x.try_into().unwrap()))
}
fn read_u32(b: &[u8], i: usize) -> Result<u32, String> {
    let x = b.get(i..i.checked_add(4).ok_or("offset overflow")?).ok_or("truncated ELF")?;
    Ok(u32::from_le_bytes(x.try_into().unwrap()))
}
pub fn parse_elf32(b: &[u8]) -> Result<ElfHeader, String> {
    if b.get(..4) != Some(&b"\x7fELF"[..]) { return Err("not ELF".into()); }
    if b.get(4) != Some(&1) || b.get(5) != Some(&1) {
        return Err("only little-endian ELF32 supported".into());
    }
    if b.len() < 52 { return Err("truncated ELF header".into()); }
    let machine = read_u16(b, 18)?;
    let entry = read_u32(b, 24)?;
    let phoff = read_u32(b, 28)? as usize;
    let phentsize = read_u16(b, 42)? as usize;
    let phnum = read_u16(b, 44)? as usize;
    if phnum > 0 && phentsize < 32 { return Err("program-header record too short".into()); }
    let mut program_headers = Vec::with_capacity(phnum);
    for n in 0..phnum {
        let at = phoff.checked_add(n.checked_mul(phentsize).ok_or("index overflow")?)
            .ok_or("offset overflow")?;
        let _ = b.get(at..at.checked_add(phentsize).ok_or("record overflow")?)
            .ok_or("truncated program-header table")?;
        program_headers.push(ProgramHeader {
            kind: read_u32(b, at)?, offset: read_u32(b, at+4)?,
            vaddr: read_u32(b, at+8)?, paddr: read_u32(b, at+12)?,
            file_size: read_u32(b, at+16)?, mem_size: read_u32(b, at+20)?,
            flags: read_u32(b, at+24)?,
        });
    }
    Ok(ElfHeader { machine, entry, program_headers })
}

pub fn machine_name(machine: u16) -> &'static str {
    match machine { 40 => "ARM", 164 => "Qualcomm Hexagon DSP", _ => "other" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bytes_and_layout() {
        let data = build_modemuw();
        assert_eq!(data.len(), 713);
        assert!(data.starts_with("{\n    \"sr_version\":"));
        assert!(data.ends_with("    ]\n}"));
        assert!(!data.ends_with('\n'));
    }
    #[test]
    fn rejects_truncated_elf() {
        assert!(parse_elf32(b"\x7fELF").is_err());
        assert!(parse_elf32(b"not an ELF").is_err());
    }
}

/// Fully parses WMFW container boundaries and metadata, but does not interpret
/// Cirrus Halo DSP machine instructions or redistribute proprietary payloads.
#[derive(Debug, PartialEq, Eq)]
pub struct WmfwHeader {
    pub revision: u16,
    pub core: u8,
    pub version: u8,
    pub region_headers: Vec<WmfwRegion>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WmfwRegion {
    pub file_offset: usize,
    pub dsp_offset: u32,
    pub region_type: u8,
    pub payload_length: usize,
}

pub fn wmfw_type_name(kind: u8) -> &'static str {
    // From Linux include/linux/firmware/cirrus/wmfw.h, core WMFW_HALO == 4.
    match kind {
        0x10 => "HALO_PM_PACKED",
        0x11 => "HALO_XM_PACKED",
        0x12 => "HALO_YM_PACKED",
        0xf0 => "ABSOLUTE",
        0xf2 => "ALGORITHM_DATA",
        0xfc => "METADATA",
        0xfe => "NAME_TEXT",
        0xff => "INFO_TEXT",
        _ => "UNRECOGNIZED_REGION",
    }
}

pub fn parse_wmfw(b: &[u8]) -> Result<WmfwHeader, String> {
    if b.get(..4) != Some(&b"WMFW"[..]) { return Err("not WMFW".into()); }
    if b.len() < 12 { return Err("truncated WMFW header".into()); }
    let header_len = read_u32(b, 4)? as usize;
    let revision = read_u16(b, 8)?;
    let core = b[10];
    let version = b[11];
    // v3 and HALO use a 12-byte header + four 4-byte memory sizes + 12-byte footer.
    if version != 3 || core != 4 {
        return Err(format!("unhandled WMFW layout: core={core}, version={version}"));
    }
    if header_len != 40 || header_len > b.len() {
        return Err("unexpected HALO v3 header length".into());
    }
    let mut offset = header_len;
    let mut region_headers = Vec::new();
    while offset < b.len() {
        let word = read_u32(b, offset)?;
        let length = read_u32(b, offset + 4)? as usize;
        let next = offset.checked_add(8).and_then(|v| v.checked_add(length))
            .ok_or("WMFW region length overflow")?;
        if next > b.len() { return Err("WMFW region exceeds file".into()); }
        region_headers.push(WmfwRegion {
            file_offset: offset,
            dsp_offset: word & 0x00ff_ffff,
            region_type: (word >> 24) as u8,
            payload_length: length,
        });
        offset = next;
    }
    Ok(WmfwHeader { revision, core, version, region_headers })
}

/// Find Qualcomm MDT hash-segment descriptors by their kernel-defined p_flags.
/// This reveals a *candidate authentication boundary*, not verified signatures.
pub fn qcom_mdt_hash_segments(header: &ElfHeader) -> Vec<(usize, &ProgramHeader)> {
    header.program_headers.iter().enumerate()
        .filter(|(_, s)| s.flags & 0x0700_0000 == 0x0200_0000)
        .collect()
}

#[cfg(test)]
mod firmware_tests {
    use super::*;

    #[test]
    fn wmfw_minimal_valid_halo_header() {
        let mut b = vec![0u8; 40];
        b[0..4].copy_from_slice(b"WMFW");
        b[4..8].copy_from_slice(&40u32.to_le_bytes());
        b[9] = 3; b[10] = 4; b[11] = 3;
        // One INFO_TEXT region with zero-length payload
        b.extend_from_slice(&0xff000000u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        let parsed = parse_wmfw(&b).unwrap();
        assert_eq!(parsed.region_headers.len(), 1);
        assert_eq!(parsed.region_headers[0].region_type, 0xff);
        assert_eq!(parsed.revision, 768);
    }

    #[test]
    fn reject_truncated_wmfw_region() {
        let mut b = vec![0u8; 40];
        b[0..4].copy_from_slice(b"WMFW");
        b[4..8].copy_from_slice(&40u32.to_le_bytes());
        b[10] = 4; b[11] = 3;
        b.extend_from_slice(&0x11000000u32.to_le_bytes());
        b.extend_from_slice(&64u32.to_le_bytes());
        assert!(parse_wmfw(&b).is_err());
    }

    #[test]
    fn hash_segment_classified_by_flags_not_content() {
        let header = ElfHeader { machine: 164, entry: 0,
            program_headers: vec![
                ProgramHeader {kind: 0, offset: 0, vaddr: 0, paddr: 0, file_size: 10,
                    mem_size: 0, flags: 0x0220_0000},
                ProgramHeader {kind: 1, offset: 20, vaddr: 0, paddr: 0, file_size: 10,
                    mem_size: 10, flags: 0x0800_0005},
            ],
        };
        assert_eq!(qcom_mdt_hash_segments(&header).len(), 1);
    }
}

/// Tentative Novatek NT36xxx-style container map, NOT an MCU/ISA identification.
/// Inspired by the public Linux/Android NT36xxx firmware-update parser, which
/// comes from a *related* chipset. Nabu NT36523 format compatibility is unverified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovatekPartition {
    pub kind: String,
    pub file_offset: u32,
    pub sram_address: u32,
    pub size_minus_one: u32,
    pub checksum_word: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovatekCandidateHeader {
    pub header_length: usize,
    pub has_nvt_marker: bool,
    pub partitions: Vec<NovatekPartition>,
}

pub fn parse_novatek_candidate(b: &[u8]) -> Result<NovatekCandidateHeader, String> {
    if b.len() < 0x40 { return Err("NVT candidate too small".into()); }
    let header_length = read_u32(b, 0)? as usize;
    if !(0x30..=0x2000).contains(&header_length)
        || header_length > b.len()
        || (header_length - 0x30) % 16 != 0
    {
        return Err("NVT candidate header length is implausible".into());
    }
    let mut partitions = Vec::new();
    // First two 12-byte entries represent ILM and DLM in related NT36xxx
    // drivers; size_minus_one follows that driver's inclusive-length encoding.
    for i in 0..2 {
        let at = i*12;
        let file_offset = read_u32(b, at)?;
        let sram_address = read_u32(b, at+4)?;
        let size_minus_one = read_u32(b, at+8)?;
        let checksum_word = read_u32(b, 0x18 + i*4)?;
        let end = (file_offset as usize).checked_add(size_minus_one as usize)
            .and_then(|v| v.checked_add(1)).ok_or("partition size overflow")?;
        if end > b.len() { return Err("ILM/DLM extends past file".into()); }
        partitions.push(NovatekPartition {
            kind: if i == 0 { "ILM" } else { "DLM" }.into(),
            file_offset, sram_address, size_minus_one, checksum_word,
        });
    }
    // The rest are tentative 16-byte entries. Some can be metadata or
    // replicated headers in this vendor-specific NT36523 image.
    for i in 0..(header_length-0x30)/16 {
        let at=0x30+i*16;
        let (sram_address, size_minus_one, file_offset, checksum_word) = (
            read_u32(b,at)?, read_u32(b,at+4)?, read_u32(b,at+8)?, read_u32(b,at+12)?
        );
        // Do not insist that these are actual partitions. Their semantics
        // depend on the exact device-side vendor driver.
        partitions.push(NovatekPartition {
            kind: format!("candidate-{i}"), file_offset, sram_address,
            size_minus_one, checksum_word,
        });
    }
    Ok(NovatekCandidateHeader {
        header_length, has_nvt_marker: b.ends_with(b"NVT"), partitions,
    })
}

#[cfg(test)]
mod novatek_tests {
    use super::*;
    #[test]
    fn parses_bounded_candidate() {
        let mut b = vec![0u8;0x300];
        b[0..4].copy_from_slice(&0x30u32.to_le_bytes());
        b[8..12].copy_from_slice(&0x80u32.to_le_bytes());
        b[12..16].copy_from_slice(&0xb1u32.to_le_bytes());
        b[20..24].copy_from_slice(&0x20u32.to_le_bytes());
        b[0x2fd..].copy_from_slice(b"NVT");
        let parsed = parse_novatek_candidate(&b).unwrap();
        assert_eq!(parsed.partitions.len(),2);
        assert!(parsed.has_nvt_marker);
    }
    #[test]
    fn rejects_bad_candidate_header() {
        let mut b = vec![0u8;0x300];
        b[..4].copy_from_slice(&0x234u32.to_le_bytes());
        assert!(parse_novatek_candidate(&b).is_err());
    }
}

/// Prepare a *temporary analysis-only* ELF view that lets Ghidra inspect one
/// boot code segment of a Qualcomm image whose e_entry is a physical address.
///
/// SECURITY / PROVENANCE: this modifies a local copy of proprietary vendor
/// firmware. The result is INVALID for device boot, not GPL-licensed firmware,
/// and MUST NOT be flashed, shipped as a firmware image, or confused with an
/// actual rebuilt executable. Only our transformation algorithm is GPLv3.
pub fn make_ghidra_bootstrap_view(b: &[u8]) -> Result<Vec<u8>, String> {
    let elf = parse_elf32(b)?;
    if elf.machine != 164 {
        return Err("only Qualcomm Hexagon ELF32 supported".into());
    }
    let mut result = b.to_vec();
    for (i, seg) in elf.program_headers.iter().enumerate() {
        if seg.kind != 1 || (seg.flags & 1) == 0 || seg.file_size == 0 { continue; }
        let from = seg.paddr as u64;
        let end = from + seg.file_size as u64;
        let entry = elf.entry as u64;
        if entry >= from && entry < end {
            let translated = (seg.vaddr as u64) + (entry - from);
            if translated > u32::MAX as u64 || i >= u16::MAX as usize {
                return Err("ELF virtual entry address overflow".into());
            }
            result[24..28].copy_from_slice(&(translated as u32).to_le_bytes());
            result[44..46].copy_from_slice(&((i+1) as u16).to_le_bytes());
            return Ok(result);
        }
    }
    Err("entry does not belong to a file-backed executable physical segment".into())
}

#[cfg(test)]
mod analysis_elf_tests {
    use super::*;
    #[test]
    fn translates_physical_entry_and_restricts_segment_table() {
        let mut b = vec![0u8;0x200];
        b[..4].copy_from_slice(b"\x7fELF"); b[4]=1; b[5]=1;
        b[18..20].copy_from_slice(&164u16.to_le_bytes());
        b[24..28].copy_from_slice(&0x8004u32.to_le_bytes());
        b[28..32].copy_from_slice(&52u32.to_le_bytes());
        b[42..44].copy_from_slice(&32u16.to_le_bytes());
        b[44..46].copy_from_slice(&3u16.to_le_bytes());
        let ph=52+2*32;
        b[ph..ph+4].copy_from_slice(&1u32.to_le_bytes());
        b[ph+8..ph+12].copy_from_slice(&0xf000u32.to_le_bytes());
        b[ph+12..ph+16].copy_from_slice(&0x8000u32.to_le_bytes());
        b[ph+16..ph+20].copy_from_slice(&256u32.to_le_bytes());
        b[ph+20..ph+24].copy_from_slice(&256u32.to_le_bytes());
        b[ph+24..ph+28].copy_from_slice(&5u32.to_le_bytes());
        let out=make_ghidra_bootstrap_view(&b).unwrap();
        assert_eq!(read_u32(&out,24).unwrap(),0xf004);
        assert_eq!(read_u16(&out,44).unwrap(),3);
        assert_eq!(read_u32(&b,24).unwrap(),0x8004); // never mutates input
        assert_eq!(&out[52..],&b[52..]);
    }
    #[test]
    fn rejects_non_hexagon() {
        let b=b"not an ELF";
        assert!(make_ghidra_bootstrap_view(b).is_err());
    }
}
