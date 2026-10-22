// SPDX-License-Identifier: GPL-3.0-only
//! Checked, address-aware assembly of QRISC GPU command processor operations.
//!
//! A script is a sequence of named operations, not a decompiler transcript or
//! an encoded-word table. Call and branch destinations are **absolute word
//! indices**, while branch fields are signed, instruction-relative offsets.
//! A script can be linked as a fragment at a specified base word address.
//! Actual register availability is determined by the target GPU generation.

use crate::{codec::QriscWord, command_stream::{CommandStream, StreamValue}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImmediateOperation {
    Add, And, Or, Xor, ShiftLeft, ShiftRightUnsigned, Rotate,
}
impl ImmediateOperation {
    const fn opcode(self) -> u8 {
        match self {
            Self::Add => 0x02, Self::And => 0x0a, Self::Or => 0x0c,
            Self::Xor => 0x0e, Self::ShiftLeft => 0x12,
            Self::ShiftRightUnsigned => 0x14, Self::Rotate => 0x18,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegisterOperation { AddWithCarry, Or, Compare }
impl RegisterOperation {
    const fn function(self) -> u16 {
        match self { Self::AddWithCarry => 2, Self::Or => 6, Self::Compare => 16 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchCondition {
    /// Branch when a QRISC register does not equal the embedded 5-bit value.
    NotEqualImmediate { register: u8, value: u8 },
    /// Branch when a QRISC register equals the embedded 5-bit value.
    EqualImmediate { register: u8, value: u8 },
    /// Branch if this 0..31 bit is clear; bit zero of register zero is an
    /// unconditional forward/backward jump (Mesa's `jump` alias).
    BitClear { register: u8, bit: u8 },
    /// Branch if this 0..31 bit is set.
    BitSet { register: u8, bit: u8 },
    Always,

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    start_word: usize,
    words: Vec<QriscWord>,
}

impl Script {
    pub fn new(start_word: usize) -> Self { Self { start_word, words: Vec::new() } }
    pub fn position(&self) -> usize { self.start_word + self.words.len() }
    pub fn words(&self) -> &[QriscWord] { &self.words }
    pub fn emit(&mut self, op: QriscWord) { self.words.push(op); }

    pub fn expect_position(&self, word: usize) -> Result<(), String> {
        if self.position() != word {
            return Err(format!("QRISC layout drift: expected word 0x{word:x}, got 0x{:x}", self.position()));
        }
        Ok(())
    }
    pub fn mov(&mut self, dst: u8, value: u16) { self.mov_shift(dst, value, 0); }
    pub fn mov_shift(&mut self, dst: u8, value: u16, shift: u8) {
        self.emit(QriscWord::MovImmediate { repeat: false, dst, shift, imm: value });
    }
    pub fn mov_repeat(&mut self, dst: u8, value: u16) {
        self.emit(QriscWord::MovImmediate { repeat: true, dst, shift: 0, imm: value });
    }
    pub fn imm(&mut self, operation: ImmediateOperation, dst: u8, src: u8, value: u16) {
        self.emit(QriscWord::ImmediateAlu { opcode: operation.opcode(), src, dst, imm: value });
    }
    pub fn register(&mut self, operation: RegisterOperation, dst: u8, src1: u8, src2: u8) {
        self.emit(QriscWord::RegisterAlu { src1, src2, dst, function: operation.function() });
    }
    pub fn control_write(&mut self, source: u8, register: u16) {
        self.emit(QriscWord::ControlWrite { base: 0, source, reg: register });
    }
    pub fn control_read(&mut self, dest: u8, register: u16) {
        self.emit(QriscWord::ControlRead { base: 0, dest, reg: register });
    }
    /// Standard command-processor `$addr` and `$data` registers.
    pub fn data_address(&mut self, address: u16) { self.mov(29, address); }
    pub fn data(&mut self, value: u16) { self.mov(31, value); }
    pub fn write_data_from(&mut self, source: u8) {
        self.register(RegisterOperation::Or, 31, 0, source);
    }
    pub fn nop(&mut self) { self.emit(QriscWord::Nop { metadata: 0 }); }
    pub fn ret(&mut self, delayed: bool) {
        // The role of bit 24 is unknown; Mesa prints both as `ret`.
        self.emit(QriscWord::Return { ignored_bits: if delayed { 1 << 24 } else { 0 } });
    }
    pub fn call(&mut self, target_word: u32) { self.emit(QriscWord::Call { target: target_word }); }
    pub fn branch(&mut self, condition: BranchCondition, target_word: usize) -> Result<(), String> {
        let delta = i64::try_from(target_word).map_err(|_| "QRISC branch destination too large")?
            - i64::try_from(self.position()).map_err(|_| "QRISC branch source too large")?;
        let delta = i16::try_from(delta).map_err(|_| "QRISC branch out of range")?;
        let (opcode, src, condition) = match condition {
            BranchCondition::NotEqualImmediate { register, value } => (0x30, register, value),
            BranchCondition::EqualImmediate { register, value } => (0x31, register, value),
            BranchCondition::BitClear { register, bit } => (0x32, register, bit),
            BranchCondition::BitSet { register, bit } => (0x33, register, bit),
            BranchCondition::Always => (0x32, 0, 0),
        };
        self.emit(QriscWord::Branch { opcode, src, condition, delta });
        Ok(())
    }
    /// Emit a group of consecutive GPU register writes via the command port.
    pub fn write_register_range(&mut self, first: u16, values: &[StreamValue]) -> Result<(), String> {
        let mut writes = CommandStream::new();
        writes.write_range(first, values).map_err(str::to_string)?;
        self.words.extend_from_slice(writes.words());
        Ok(())
    }
    /// A common event-initiator suffix masks an identifier before placing it
    /// into `$data` on the command stream. This is a bitwise data operation,
    /// not an implicit transfer from the host CPU.
    pub fn masked_data(&mut self, source: u8, mask: u16) {
        self.imm(ImmediateOperation::And, 31, source, mask);
    }
    pub fn finish(self, expected_end_word: usize) -> Result<Vec<u8>, String> {
        self.expect_position(expected_end_word)?;
        let mut bytes = Vec::with_capacity(self.words.len() * 4);
        for op in self.words {
            bytes.extend_from_slice(&op.encode().map_err(str::to_string)?.to_le_bytes());
        }
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn branches_use_absolute_word_addresses_and_check_distance() {
        let mut s = Script::new(100);
        s.nop();
        s.branch(BranchCondition::Always, 100).unwrap();
        s.branch(BranchCondition::NotEqualImmediate { register: 3, value: 0 }, 104).unwrap();
        assert_eq!(s.position(), 103);
        assert!(s.branch(BranchCondition::Always, 90000).is_err());
        let bytes = s.finish(103).unwrap();
        let words: Vec<_> = bytes.chunks_exact(4).map(|chunk| {
            u32::from_le_bytes(chunk.try_into().unwrap())
        }).collect();
        assert_eq!(QriscWord::decode(words[1]), Some(QriscWord::Branch {
            opcode: 0x32, src: 0, condition: 0, delta: -1,
        }));
        assert_eq!(QriscWord::decode(words[2]), Some(QriscWord::Branch {
            opcode: 0x30, src: 3, condition: 0, delta: 2,
        }));
    }
    #[test]
    fn stream_operations_are_typed_and_length_checked() {
        let mut s = Script::new(7);
        s.write_register_range(0xab00, &[StreamValue::Small(0x11),
            StreamValue::Wide { upper: 0x55, lower: 0xff }]).unwrap();
        assert_eq!(s.position(), 11);
        assert!(s.clone().finish(12).unwrap_err().contains("drift"));
        assert_eq!(s.finish(11).unwrap().len(), 16);
        let mut s = Script::new(2);
        assert!(s.write_register_range(0xffff, &[StreamValue::Small(1), StreamValue::Small(2)]).is_err());
        assert_eq!(s.position(), 2);
    }
}
