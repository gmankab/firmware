// SPDX-License-Identifier: GPL-3.0-only
//! Format-aware *analysis* of an ELF-wrapped QRISC instruction stream.
//!
//! The header's EM_HEXAGON designation does not prove the payload ISA.
//! Mesa's publicly documented QRISC branch and no-op encodings provide a
//! conservative way to locate a candidate jump-over-IR3-program region.
//! This module does not decompile, execute, copy, or relicense firmware.

use crate::parse_elf32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedShader {
    /// Offsets in 32-bit words relative to the executable load segment.
    pub start_word: usize,
    pub end_word: usize,
    pub skip_jump_word: usize,
}

impl EmbeddedShader {
    pub fn bytes(&self) -> usize { (self.end_word - self.start_word) * 4 }
    pub fn ir3_slots(&self) -> usize { (self.end_word - self.start_word) / 2 }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicrocodeLayout {
    pub segment_file_offset: usize,
    pub segment_size: usize,
    pub segment_virtual_addr: u32,
    /// Firmware family ID extracted from the first microcode word; NOT e_flags.
    pub firmware_id: u16,
    pub total_words: usize,
    pub shader: Option<EmbeddedShader>,
    pub qrisc_branches: usize,
    pub qrisc_calls: usize,
    pub qrisc_returns: usize,
}

fn word(data: &[u8], index: usize) -> u32 {
    let start = index * 4;
    u32::from_le_bytes(data[start..start + 4].try_into().unwrap())
}

fn qrisc_nop(w: u32) -> bool {
    // On A6xx, the low 24 bits of this instruction are ignored.
    (w >> 24) == 0x01
}

/// Extract the unique nonempty executable segment from an ELF32 wrapper.
/// Limited to one segment by design: multiple-segment Hexagon MDT images
/// require other analyses and should not silently be mistaken for A640.
pub fn executable_segment<'a>(firmware: &'a [u8]) -> Result<(&'a [u8], usize, u32), String> {
    let elf = parse_elf32(firmware)?;
    if elf.machine != 164 { return Err("expected an EM_HEXAGON ELF wrapper".into()); }
    let executable: Vec<_> = elf.program_headers.iter()
        .filter(|p| p.kind == 1 && p.flags & 1 != 0 && p.file_size > 0)
        .collect();
    if executable.len() != 1 {
        return Err(format!("expected one executable segment, found {}", executable.len()));
    }
    let p = executable[0];
    let start = p.offset as usize;
    let end = start.checked_add(p.file_size as usize).ok_or("segment offset overflow")?;
    let segment = firmware.get(start..end).ok_or("executable segment exceeds image")?;
    if segment.len() < 4 || segment.len() % 4 != 0 {
        return Err("segment must contain whole 32-bit words".into());
    }
    if elf.entry != p.vaddr { return Err("entry point not at start of executable segment".into()); }
    Ok((segment, start, p.vaddr))
}

/// Locate a likely embedded shader based on a QRISC unconditional forward
/// branch over a two-NOP delay/alignment area and an even-sized data range.
/// This is a hypothesis. Confirm it with an independent IR3 decoder.
pub fn candidate_shader(stream: &[u8]) -> Option<EmbeddedShader> {
    if stream.len() % 4 != 0 { return None; }
    let size = stream.len() / 4;
    let mut best: Option<EmbeddedShader> = None;
    for i in 0..size.saturating_sub(3) {
        let ins = word(stream, i);
        // Mesa QRISC: unconditional `jump` is an alias for branch-on-zero
        // and uses opcode bits 31:26 == 0x32, register/bit fields zero.
        if (ins & 0xffff_0000) != 0xc800_0000 { continue; }
        let delta = (ins as u16 as i16) as isize;
        if delta <= 0 { continue; }
        let Some(end_word) = i.checked_add(delta as usize) else { continue; };
        let start_word = i + 3;
        if end_word > size || start_word >= end_word ||
           !start_word.is_multiple_of(2) || !(end_word - start_word).is_multiple_of(2) ||
           end_word - start_word < 32 ||
           !qrisc_nop(word(stream, i + 1)) || !qrisc_nop(word(stream, i + 2)) {
            continue;
        }
        let found = EmbeddedShader { start_word, end_word, skip_jump_word: i };
        if best.as_ref().is_none_or(|b| found.bytes() > b.bytes()) { best = Some(found); }
    }
    best
}

pub fn inspect(firmware: &[u8]) -> Result<MicrocodeLayout, String> {
    let (segment, offset, vaddr) = executable_segment(firmware)?;
    let shader = candidate_shader(segment);
    let mut branches = 0;
    let mut calls = 0;
    let mut returns = 0;
    for i in 0..(segment.len() / 4) {
        if shader.as_ref().is_some_and(|s| (s.start_word..s.end_word).contains(&i)) { continue; }
        let opcode = word(segment, i) >> 26;
        match opcode {
            0x30..=0x33 => branches += 1,
            0x35 => calls += 1,
            0x34 => returns += 1,
            _ => {}
        }
    }
    let header_word = word(segment, 0);
    Ok(MicrocodeLayout {
        segment_file_offset: offset,
        segment_size: segment.len(),
        segment_virtual_addr: vaddr,
        firmware_id: ((header_word >> 12) & 0x0fff) as u16,
        total_words: segment.len() / 4,
        shader, qrisc_branches: branches, qrisc_calls: calls, qrisc_returns: returns,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        let mut firmware = vec![0u8; 256 + 48 * 4];
        firmware[0..6].copy_from_slice(b"\x7fELF\x01\x01");
        firmware[18..20].copy_from_slice(&164u16.to_le_bytes());
        firmware[24..28].copy_from_slice(&0x5000u32.to_le_bytes());
        firmware[28..32].copy_from_slice(&52u32.to_le_bytes());
        firmware[42..44].copy_from_slice(&32u16.to_le_bytes());
        firmware[44..46].copy_from_slice(&1u16.to_le_bytes());
        let ph = 52;
        for (o, v) in [(0, 1u32), (4, 256), (8, 0x5000), (16, 48 * 4), (20, 48 * 4), (24, 5)] {
            firmware[ph+o..ph+o+4].copy_from_slice(&v.to_le_bytes());
        }
        for i in 0..48 {
            firmware[256+i*4..256+(i+1)*4].copy_from_slice(&0x0100_0000u32.to_le_bytes());
        }
        // Synthetic Mesa-compatible unconditional jump over sixteen IR3 slots.
        firmware[256..260].copy_from_slice(&0x016e_e001u32.to_le_bytes());
        firmware[256+9*4..256+10*4].copy_from_slice(&(0xc800_0000u32 | (44 - 9)).to_le_bytes());
        firmware[256+44*4..256+45*4].copy_from_slice(&0xd000_0000u32.to_le_bytes());
        firmware
    }

    #[test]
    fn discovers_synthetic_shader() {
        let layout = inspect(&fixture()).unwrap();
        assert_eq!(layout.firmware_id, 0x6ee);
        assert_eq!(layout.total_words, 48);
        assert_eq!(layout.shader.unwrap(), EmbeddedShader { start_word: 12, end_word: 44, skip_jump_word: 9 });
        assert_eq!(layout.qrisc_calls, 0);
        assert_eq!(layout.qrisc_returns, 1);
    }
    #[test]
    fn rejects_non_elf() { assert!(inspect(b"not ELF").is_err()); }
    #[test]
    fn rejects_truncated_executable() {
        let mut f = fixture();f.truncate(256+30);
        assert!(inspect(&f).is_err());
    }
    #[test]
    fn nop_is_isa_based_not_zero_byte_test() {
        let mut f = fixture();
        f[256+10*4..256+11*4].copy_from_slice(&0u32.to_le_bytes());
        assert!(inspect(&f).unwrap().shader.is_none());
    }
}
