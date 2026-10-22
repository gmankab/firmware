// SPDX-License-Identifier: GPL-3.0-only
//! A label-resolving, deterministic QRISC-A6xx instruction generator.
//!
//! Input is programmer-supplied typed instructions and symbolic labels, not
//! disassembled OEM instruction bytes. This library does NOT reconstruct any
//! vendor program and does not emulate the GPU's device-specific behavior.

use crate::qrisc_codec::QriscWord;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<'a> {
    Label(&'a str),
    Instr(QriscWord),
    /// `opcode` is 0x30..=0x33; delta is relative to the branch word itself.
    BranchTo { opcode: u8, src: u8, condition: u8, label: &'a str },
    /// Absolute word index of a `call` target (not a byte address).
    CallTo { label: &'a str },
    /// A deliberately generated shader slot, two little-endian words.
    /// The generator does not interpret arbitrary IR3 opcodes.
    ShaderSlot { low: u32, high: u32 },
}

impl Item<'_> {
    fn width_words(&self) -> usize {
        match self { Self::Label(_) => 0, Self::ShaderSlot { .. } => 2, _ => 1 }
    }
}

/// Deterministically assemble QRISC instructions with labels and synthetic IR3
/// slots, checking duplicates, unresolved symbols, and relative branch ranges.
/// All caller-supplied payload bytes must have an independent lawful origin.
pub fn assemble(program: &[Item<'_>]) -> Result<Vec<u8>, String> {
    let mut labels = HashMap::new();
    let mut words = 0usize;
    for item in program {
        match *item {
            Item::Label(name) => {
                if name.is_empty() { return Err("empty label".into()); }
                if labels.insert(name, words).is_some() {
                    return Err(format!("duplicate QRISC label: {name}"));
                }
            }
            _ => words = words.checked_add(item.width_words()).ok_or("program length overflow")?,
        }
    }
    let mut out = Vec::with_capacity(words.checked_mul(4).ok_or("output size overflow")?);
    let mut index = 0usize;
    for item in program {
        let w = match *item {
            Item::Label(_) => continue,
            Item::Instr(ref ins) => ins.encode().map_err(str::to_string)?,
            Item::BranchTo { opcode, src, condition, label } => {
                let target = *labels.get(label).ok_or_else(|| format!("unknown QRISC label: {label}"))?;
                let delta = target as i64 - index as i64;
                let delta = i16::try_from(delta)
                    .map_err(|_| format!("QRISC branch range exceeds signed 16 bits: {label}"))?;
                QriscWord::Branch { opcode, src, condition, delta }.encode().map_err(str::to_string)?
            }
            Item::CallTo { label } => {
                let target = *labels.get(label).ok_or_else(|| format!("unknown QRISC label: {label}"))?;
                let target = u32::try_from(target).map_err(|_| "call target exceeds 32 bits")?;
                QriscWord::Call { target }.encode().map_err(str::to_string)?
            }
            Item::ShaderSlot { low, high } => {
                out.extend_from_slice(&low.to_le_bytes());
                out.extend_from_slice(&high.to_le_bytes());
                index += 2;
                continue;
            }
        };
        out.extend_from_slice(&w.to_le_bytes());
        index += 1;
    }
    debug_assert_eq!(out.len(), words * 4);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qrisc_codec::{firmware_header, QriscWord};
    #[test]
    fn forward_and_backward_branches_are_instruction_relative() {
        let p = [
            Item::Label("start"),
            Item::Instr(firmware_header(0x6ee, 1).unwrap()),
            Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "end" },
            Item::Instr(QriscWord::MovImmediate { repeat: false, dst: 2, shift: 0, imm: 42 }),
            Item::Label("end"),
            Item::BranchTo { opcode: 0x30, src: 2, condition: 0, label: "start" },
            Item::Instr(QriscWord::Return { ignored_bits: 1 << 24 }),
        ];
        let words: Vec<_> = assemble(&p).unwrap().chunks_exact(4)
            .map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(words.len(), 5);
        assert_eq!(words[0], 0x016e_e001); // public compatible synthetic FW ID
        assert_eq!(words[1], 0xc800_0002); // jump from word 1 -> word 3
        assert_eq!(words[3], 0xc040_fffd); // brne from word 3 -> word 0
        assert_eq!(words[4], 0xd100_0000); // ret with an uninterpreted bit
    }
    #[test]
    fn label_positions_skip_empty_and_include_shader_slots() {
        let p = [Item::Label("a"), Item::ShaderSlot { low: 10, high: 20 },
            Item::Label("b"), Item::CallTo { label: "b" },
            Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "a" }];
        let words: Vec<_> = assemble(&p).unwrap().chunks_exact(4)
            .map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        assert_eq!(words, [10, 20, 0xd400_0002, 0xc800_fffd]);
    }
    #[test]
    fn duplicates_unresolved_and_out_of_range_rejected() {
        assert!(assemble(&[Item::Label("x"), Item::Label("x")]).unwrap_err().contains("duplicate"));
        assert!(assemble(&[Item::CallTo { label: "missing" }]).unwrap_err().contains("unknown"));
        let mut p = vec![Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "distant" }];
        for _ in 0..32770 { p.push(Item::Instr(QriscWord::Nop { metadata: 0 })); }
        p.push(Item::Label("distant"));
        assert!(assemble(&p).unwrap_err().contains("range"));
    }
    #[test]
    fn normal_vs_interrupt_return_and_uninterpreted_ret_bit() {
        let w = assemble(&[
            Item::Instr(QriscWord::Return { ignored_bits: 0 }),
            Item::Instr(QriscWord::Return { ignored_bits: 1 << 24 }),
            Item::Instr(QriscWord::InterruptReturn { ignored_bits: 0 }),
        ]).unwrap();
        assert_eq!(w, [0,0,0,0xd0, 0,0,0,0xd1, 0,0,0,0xd2]);
    }
}
