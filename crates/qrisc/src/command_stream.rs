// SPDX-License-Identifier: GPL-3.0-only
//! A typed builder for the A6xx QRISC command-processor register stream.
//!
//! These are GPU register *write commands* generated from explicit values.
//! The builder does not execute, schedule or interpret the commands, and
//! register availability and safe programming sequences are GPU-specific.

use crate::codec::QriscWord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamValue {
    /// A 16-bit value placed directly in the data port.
    Small(u16),
    /// A 32-bit value synthesized by shifting an upper halfword and ORing the
    /// low halfword into the special data port, with scratch QRISC register 4.
    Wide { upper: u16, lower: u16 },
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CommandStream {
    words: Vec<QriscWord>,
}

impl CommandStream {
    pub fn new() -> Self { Self::default() }

    /// Emit a contiguous sequence of GPU register values, starting from the
    /// named control register. This stream form increments its data address
    /// implicitly via the special `$data` command processor register.
    pub fn write_range(&mut self, first_register: u16, values: &[StreamValue]) -> Result<(), &'static str> {
        if values.is_empty() { return Err("QRISC register stream cannot be empty"); }
        let count = u32::try_from(values.len()).map_err(|_| "register stream too large")?;
        if u32::from(first_register) + count - 1 > u32::from(u16::MAX) {
            return Err("QRISC register stream address exceeds 16 bits");
        }
        self.words.push(QriscWord::MovImmediate {
            repeat: false, dst: 29, shift: 0, imm: first_register,
        });
        for value in values {
            match *value {
                StreamValue::Small(imm) => self.words.push(QriscWord::MovImmediate {
                    repeat: false, dst: 31, shift: 0, imm,
                }),
                StreamValue::Wide { upper, lower } => {
                    self.words.push(QriscWord::MovImmediate {
                        repeat: false, dst: 4, shift: 16, imm: upper,
                    });
                    self.words.push(QriscWord::ImmediateAlu {
                        opcode: 0x0c, src: 4, dst: 31, imm: lower,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn words(&self) -> &[QriscWord] { &self.words }

    pub fn encode(&self) -> Result<Vec<u8>, &'static str> {
        let mut result = Vec::with_capacity(self.words.len() * 4);
        for word in &self.words { result.extend_from_slice(&word.encode()?.to_le_bytes()); }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emitting_register_ranges_preserves_boundaries_and_wide_values() {
        use StreamValue::*;
        let mut writes = CommandStream::new();
        writes.write_range(0x123, &[Small(2), Wide { upper: 0x33, lower: 0x4455 }, Small(3)]).unwrap();
        let emitted = writes.encode().unwrap();
        assert_eq!(emitted.len(), 5 * 4);
        assert_eq!(writes.words()[0], QriscWord::MovImmediate {
            repeat: false, dst: 29, shift: 0, imm: 0x123,
        });
        assert_eq!(writes.words()[2], QriscWord::MovImmediate {
            repeat: false, dst: 4, shift: 16, imm: 0x33,
        });
        assert_eq!(writes.words()[3], QriscWord::ImmediateAlu {
            opcode: 0x0c, src: 4, dst: 31, imm: 0x4455,
        });
        assert_eq!(u32::from_le_bytes(emitted[16..20].try_into().unwrap()),
            QriscWord::MovImmediate { repeat:false, dst:31, shift:0, imm:3 }.encode().unwrap());
    }

    #[test]
    fn rejects_empty_and_oversized_ranges_without_mutation() {
        let mut writes = CommandStream::new();
        assert!(writes.write_range(0x42, &[]).is_err());
        assert!(writes.write_range(0xffff, &[StreamValue::Small(1), StreamValue::Small(2)]).is_err());
        assert_eq!(writes.words().len(), 0);
    }
}
