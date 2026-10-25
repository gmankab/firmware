// SPDX-License-Identifier: GPL-3.0-only
//! Checked NDS32 encodings used by the Novatek upper-ILM reconstruction.
//!
//! Each variant names an instruction and its semantic operands. Instruction
//! bytes are always big-endian, including on a little-endian data target.
//! Displacements passed to memory operations are byte displacements; scaling
//! is checked here. Branch operands are absolute firmware-file offsets. Their
//! differences equal the runtime-address differences within the ILM mapping.
//!
//! Encoding references: GNU binutils 2.30 `opcodes/nds32-asm.c` operand and
//! opcode tables, and `include/opcode/nds32.h`, contributed by Andes Technology.
//! In particular, a compressed four-bit register field maps to r0..r11 and
//! r16..r19, rather than r0..r15. EX9.IT executes an instruction-table entry;
//! it is not assigned a software-interrupt meaning by this encoder.

/// Architectural register identity, never an already-encoded field value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpr {
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14, R15,
    R16, R17, R18, R19, R20, R21, R22, R23,
    R24, R25, R26, R27, R28, R29, R30, R31,
}

impl Gpr {
    pub const fn number(self) -> u32 { self as u32 }

    pub fn new(number: u8) -> Result<Self, String> {
        const REGISTERS: [Gpr; 32] = [
            Gpr::R0, Gpr::R1, Gpr::R2, Gpr::R3, Gpr::R4, Gpr::R5, Gpr::R6, Gpr::R7,
            Gpr::R8, Gpr::R9, Gpr::R10, Gpr::R11, Gpr::R12, Gpr::R13, Gpr::R14, Gpr::R15,
            Gpr::R16, Gpr::R17, Gpr::R18, Gpr::R19, Gpr::R20, Gpr::R21, Gpr::R22, Gpr::R23,
            Gpr::R24, Gpr::R25, Gpr::R26, Gpr::R27, Gpr::R28, Gpr::R29, Gpr::R30, Gpr::R31,
        ];
        REGISTERS.get(number as usize).copied()
            .ok_or_else(|| format!("NDS32 register r{number} is outside r0..r31"))
    }
}

/// Width-preserving instructions; no pseudo-instruction relaxation is done.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Highest saved register (r6/r8/r10/r14), extra stack reservation in bytes.
    Push25(Gpr, u16),
    Pop25(Gpr, u16),
    Mov55(Gpr, Gpr),
    Movi55(Gpr, i32),
    Movi(Gpr, i32),
    LwiGp(Gpr, i32),
    SbiGp(Gpr, i32),
    LbiGp(Gpr, i32),
    Swi37Sp(Gpr, u16),
    Ex9It(u16),
    Beq(Gpr, Gpr, u32),
    Bne(Gpr, Gpr, u32),
    Beqz38(Gpr, u32),
    Bnez38(Gpr, u32),
    Bnec(Gpr, i32, u32),
    Beqc(Gpr, i32, u32),
    Sbi333(Gpr, Gpr, u8),
    Lbi333(Gpr, Gpr, u8),
    Sbi(Gpr, Gpr, i32),
    J8(u32),
    Jal(u32),
    Addri36Sp(Gpr, u16),
    Lb(Gpr, Gpr, Gpr, u8),
    Sb(Gpr, Gpr, Gpr, u8),
    Addi45(Gpr, u8),
    Addi(Gpr, Gpr, i32),
    Lwi(Gpr, Gpr, i32),
    Lwi450(Gpr, Gpr),
    Srli45(Gpr, u8),
    Zeb33(Gpr, Gpr),
    /// Last included bit, 0..7; extracts the low (last_bit + 1) bits.
    Fexti33(Gpr, u8),
    /// Signed comparison with implicit result register r15 (TA).
    Slts45(Gpr, Gpr),
    Beqzs8(u32),
    Bnezs8(u32),
    Add333(Gpr, Gpr, Gpr),
    /// Unsigned comparison with immediate; implicit result register r15 (TA).
    Slti45(Gpr, u8),
    Slt45(Gpr, Gpr),
    Ret5(Gpr),
}

fn small_register(reg: Gpr) -> Result<u32, String> {
    if reg.number() <= 7 { Ok(reg.number()) }
    else { Err(format!("compressed three-bit field cannot encode {reg:?}")) }
}

fn compact_register(reg: Gpr) -> Result<u32, String> {
    match reg.number() {
        n @ 0..=11 => Ok(n),
        n @ 16..=19 => Ok(n - 4),
        _ => Err(format!("compressed four-bit field cannot encode {reg:?}")),
    }
}

fn signed_field(value: i64, bits: u32, name: &str) -> Result<u32, String> {
    let limit = 1_i64 << (bits - 1);
    if !(-limit..limit).contains(&value) {
        return Err(format!("{name}: {value} does not fit a signed {bits}-bit field"));
    }
    Ok((value as u32) & ((1_u32 << bits) - 1))
}

fn unsigned_field(value: u32, bits: u32, name: &str) -> Result<u32, String> {
    if value >= 1_u32 << bits {
        return Err(format!("{name}: {value} does not fit an unsigned {bits}-bit field"));
    }
    Ok(value)
}

fn signed_scaled(value: i32, bits: u32, shift: u32, name: &str) -> Result<u32, String> {
    let scale = 1_i64 << shift;
    if i64::from(value) % scale != 0 {
        return Err(format!("{name}: displacement {value} must be a multiple of {scale}"));
    }
    signed_field(i64::from(value) / scale, bits, name)
}

fn unsigned_scaled(value: u16, bits: u32, shift: u32, name: &str) -> Result<u32, String> {
    let scale = 1_u32 << shift;
    if u32::from(value) % scale != 0 {
        return Err(format!("{name}: displacement {value} must be a multiple of {scale}"));
    }
    unsigned_field(u32::from(value) / scale, bits, name)
}

fn branch_field(at: u32, target: u32, bits: u32) -> Result<u32, String> {
    if (at | target) & 1 != 0 {
        return Err("NDS32 branch source and target must be halfword aligned".into());
    }
    // Widen before subtracting: a backward branch must never wrap at zero.
    signed_field((i64::from(target) - i64::from(at)) / 2, bits, "branch displacement")
}

fn push_register(last: Gpr) -> Result<u32, String> {
    match last {
        Gpr::R6 => Ok(0), Gpr::R8 => Ok(1), Gpr::R10 => Ok(2), Gpr::R14 => Ok(3),
        _ => Err("PUSH25/POP25 highest register must be r6, r8, r10 or r14".into()),
    }
}

impl Instruction {
    pub const fn width(self) -> usize {
        use Instruction::*;
        match self {
            Movi(..) | LwiGp(..) | SbiGp(..) | LbiGp(..) | Beq(..) | Bne(..)
            | Bnec(..) | Beqc(..) | Sbi(..) | Jal(..) | Lb(..) | Sb(..)
            | Addi(..) | Lwi(..) => 4,
            _ => 2,
        }
    }

    /// Encode at the start of an instruction, without implicit PC advance.
    pub fn encode(self, at_file: u32) -> Result<Vec<u8>, String> {
        use Instruction::*;
        if at_file & 1 != 0 {
            return Err(format!("instruction offset 0x{at_file:x} is not halfword aligned"));
        }
        at_file.checked_add(self.width() as u32)
            .ok_or("instruction extends past the u32 address space")?;

        let word: u32 = match self {
            Push25(last, extra) | Pop25(last, extra) => {
                let base = if matches!(self, Pop25(..)) { 0xfc80 } else { 0xfc00 };
                base | push_register(last)? << 5
                    | unsigned_scaled(extra, 5, 3, "PUSH25/POP25 stack reservation")?
            }
            Mov55(rt, ra) => 0x8000 | rt.number() << 5 | ra.number(),
            Movi55(rt, immediate) => 0x8400 | rt.number() << 5
                | signed_field(i64::from(immediate), 5, "MOVI55 immediate")?,
            Movi(rt, immediate) => (0x22 << 25) | rt.number() << 20
                | signed_field(i64::from(immediate), 20, "MOVI immediate")?,
            LwiGp(rt, displacement) => (0x1e << 25) | (6 << 17) | rt.number() << 20
                | signed_scaled(displacement, 17, 2, "LWI.GP")?,
            SbiGp(rt, displacement) | LbiGp(rt, displacement) => {
                let opcode = if matches!(self, SbiGp(..)) { 0x1f } else { 0x17 };
                (opcode << 25) | rt.number() << 20
                    | signed_field(i64::from(displacement), 19, "byte GP displacement")?
            }
            Swi37Sp(rt, displacement) => 0xf080 | small_register(rt)? << 8
                | unsigned_scaled(displacement, 7, 2, "SWI37.SP")?,
            Ex9It(index) => {
                let index = unsigned_field(u32::from(index), 9, "EX9.IT index")?;
                if index < 32 { 0xdd40 | index } else { 0xea00 | index }
            }
            Beq(rt, ra, target) | Bne(rt, ra, target) => {
                let condition = if matches!(self, Bne(..)) { 1 << 14 } else { 0 };
                (0x26 << 25) | rt.number() << 20 | ra.number() << 15 | condition
                    | branch_field(at_file, target, 14)?
            }
            Beqz38(rt, target) | Bnez38(rt, target) => {
                let base = if matches!(self, Bnez38(..)) { 0xc800 } else { 0xc000 };
                base | small_register(rt)? << 8 | branch_field(at_file, target, 8)?
            }
            Bnec(rt, immediate, target) | Beqc(rt, immediate, target) => {
                let condition = if matches!(self, Bnec(..)) { 1 << 19 } else { 0 };
                (0x2d << 25) | rt.number() << 20 | condition
                    | signed_field(i64::from(immediate), 11, "BR3 comparison constant")? << 8
                    | branch_field(at_file, target, 8)?
            }
            Sbi333(rt, ra, displacement) | Lbi333(rt, ra, displacement) => {
                let base = if matches!(self, Sbi333(..)) { 0xae00 } else { 0xa600 };
                base | small_register(rt)? << 6 | small_register(ra)? << 3
                    | unsigned_field(u32::from(displacement), 3, "byte 333 displacement")?
            }
            Sbi(rt, ra, displacement) => (0x08 << 25) | rt.number() << 20 | ra.number() << 15
                | signed_field(i64::from(displacement), 15, "SBI displacement")?,
            J8(target) => 0xd500 | branch_field(at_file, target, 8)?,
            Jal(target) => (0x24 << 25) | (1 << 24) | branch_field(at_file, target, 24)?,
            Addri36Sp(rt, displacement) => 0xb000 | small_register(rt)? << 6
                | unsigned_scaled(displacement, 6, 2, "ADDRI36.SP")?,
            Lb(rt, ra, rb, shift) | Sb(rt, ra, rb, shift) => {
                let sub = if matches!(self, Sb(..)) { 8 } else { 0 };
                (0x1c << 25) | rt.number() << 20 | ra.number() << 15 | rb.number() << 10
                    | unsigned_field(u32::from(shift), 2, "MEM index shift")? << 8 | sub
            }
            Addi45(rt, immediate) | Srli45(rt, immediate) | Slti45(rt, immediate) => {
                let base = match self { Addi45(..) => 0x8c00, Srli45(..) => 0x9200, _ => 0xe600 };
                base | compact_register(rt)? << 5
                    | unsigned_field(u32::from(immediate), 5, "compact immediate")?
            }
            Addi(rt, ra, immediate) => (0x28 << 25) | rt.number() << 20 | ra.number() << 15
                | signed_field(i64::from(immediate), 15, "ADDI immediate")?,
            Lwi(rt, ra, displacement) => (0x02 << 25) | rt.number() << 20 | ra.number() << 15
                | signed_scaled(displacement, 15, 2, "LWI")?,
            Lwi450(rt, ra) => 0xb400 | compact_register(rt)? << 5 | ra.number(),
            Zeb33(rt, ra) => 0x9600 | small_register(rt)? << 6 | small_register(ra)? << 3,
            Fexti33(rt, last_bit) => 0x9607 | small_register(rt)? << 6
                | unsigned_field(u32::from(last_bit), 3, "FEXTI33 last bit")? << 3,
            Slts45(rt, ra) | Slt45(rt, ra) => {
                let base = if matches!(self, Slt45(..)) { 0xe200 } else { 0xe000 };
                base | compact_register(rt)? << 5 | ra.number()
            }
            Beqzs8(target) => 0xe800 | branch_field(at_file, target, 8)?,
            Bnezs8(target) => 0xe900 | branch_field(at_file, target, 8)?,
            Add333(rt, ra, rb) => 0x9800 | small_register(rt)? << 6
                | small_register(ra)? << 3 | small_register(rb)?,
            Ret5(ra) => 0xdd80 | ra.number(),
        };
        Ok(if self.width() == 2 { (word as u16).to_be_bytes().to_vec() }
            else { word.to_be_bytes().to_vec() })
    }
}

#[cfg(test)]
mod tests {
    use super::{Gpr::*, Instruction::*};

    #[test]
    fn physical_registers_do_not_alias_compressed_register_fields() {
        assert_eq!(Addi45(R16, 1).encode(0).unwrap(), [0x8d, 0x81]);
        assert_eq!(Srli45(R19, 31).encode(0).unwrap(), [0x93, 0xff]);
        assert_eq!(Lwi450(R16, R31).encode(0).unwrap(), [0xb5, 0x9f]);
        for r in [R12, R13, R14, R15, R20, R31] {
            for instruction in [Addi45(r, 1), Srli45(r, 1), Slti45(r, 1),
                Slts45(r, R0), Slt45(r, R0), Lwi450(r, R0)] {
                assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
            }
            // Full-width register fields continue to accept those identities.
            assert!(Mov55(r, R31).encode(0).is_ok());
        }
        assert_eq!(super::Gpr::new(31).unwrap(), R31);
        assert!(super::Gpr::new(32).is_err());
    }

    #[test]
    fn byte_displacements_are_scaled_only_after_alignment_and_range_checks() {
        assert_eq!(LwiGp(R0, -82992).encode(0).unwrap(), [0x3c, 0x0d, 0xae, 0xf4]);
        assert_eq!(Lwi(R0, R6, 80).encode(0).unwrap(), [0x04, 0x03, 0x00, 0x14]);
        for instruction in [LwiGp(R0, -3), LwiGp(R0, 262144), LwiGp(R0, -262148),
            Lwi(R0, R0, 2), Lwi(R0, R0, 65536), Lwi(R0, R0, -65540),
            Swi37Sp(R0, 2), Swi37Sp(R0, 512), Addri36Sp(R0, 2), Addri36Sp(R0, 256)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
        for instruction in [LwiGp(R31, -262144), LwiGp(R31, 262140),
            Lwi(R31, R31, -65536), Lwi(R31, R31, 65532),
            Swi37Sp(R7, 508), Addri36Sp(R7, 252)] {
            assert!(instruction.encode(0).is_ok(), "rejected {instruction:?}");
        }
    }

    #[test]
    fn relative_branches_use_instruction_start_and_checked_signed_halfwords() {
        assert_eq!(Jal(0xfffc).encode(0x1003a).unwrap(), [0x49, 0xff, 0xff, 0xe1]);
        assert_eq!(J8(0x10116).encode(0x1015a).unwrap(), [0xd5, 0xde]);
        assert_eq!(Beq(R5, R0, 0x101aa).encode(0x10028).unwrap(), [0x4c, 0x50, 0x00, 0xc1]);
        assert_eq!(Bnec(R0, 32, 0x10056).encode(0x10060).unwrap(), [0x5a, 0x08, 0x20, 0xfb]);
        assert_eq!(J8(0xf00).encode(0x1000).unwrap(), [0xd5, 0x80]);
        assert_eq!(J8(0x10fe).encode(0x1000).unwrap(), [0xd5, 0x7f]);
        assert!(J8(0xefe).encode(0x1000).is_err());
        assert!(J8(0x1100).encode(0x1000).is_err());
        assert!(J8(0x1001).encode(0x1000).is_err());
        assert!(J8(0x1000).encode(0x1001).is_err());
        assert!(Beq(R0, R1, 0x8000).encode(0x4000).is_err());
        assert!(Jal(0x2000000).encode(0).is_err());
        // Widened subtraction must reject huge wraps rather than truncate them.
        assert!(J8(u32::MAX - 1).encode(0).is_err());
        assert!(Jal(0).encode(u32::MAX - 5).is_err());
    }

    #[test]
    fn immediates_do_not_silently_truncate_or_change_signedness() {
        assert_eq!(Movi55(R31, -16).encode(0).unwrap(), [0x87, 0xf0]);
        assert_eq!(Movi55(R16, 9).encode(0).unwrap(), [0x86, 0x09]);
        assert_eq!(Addi(R15, R1, -32).encode(0).unwrap(), [0x50, 0xf0, 0xff, 0xe0]);
        for instruction in [Movi55(R0, -17), Movi55(R0, 16), Movi(R0, -524289),
            Movi(R0, 524288), Addi(R0, R0, -16385), Addi(R0, R0, 16384),
            Bnec(R0, -1025, 0), Beqc(R0, 1024, 0), SbiGp(R0, -262145),
            LbiGp(R0, 262144), Sbi(R0, R0, 16384), Addi45(R0, 32),
            Srli45(R0, 32), Slti45(R0, 32), Fexti33(R0, 8),
            Sbi333(R0, R0, 8), Lb(R0, R0, R0, 4)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
    }

    #[test]
    fn ex9_encodings_preserve_the_split_index_domain() {
        assert_eq!(Ex9It(0).encode(0).unwrap(), [0xdd, 0x40]);
        assert_eq!(Ex9It(31).encode(0).unwrap(), [0xdd, 0x5f]);
        assert_eq!(Ex9It(32).encode(0).unwrap(), [0xea, 0x20]);
        assert_eq!(Ex9It(205).encode(0).unwrap(), [0xea, 0xcd]);
        assert_eq!(Ex9It(511).encode(0).unwrap(), [0xeb, 0xff]);
        assert!(Ex9It(512).encode(0).is_err());
    }

    #[test]
    fn stack_encodings_check_register_groups_and_reservation_units() {
        assert_eq!(Push25(R10, 8).encode(0).unwrap(), [0xfc, 0x41]);
        assert_eq!(Pop25(R10, 8).encode(0).unwrap(), [0xfc, 0xc1]);
        assert_eq!(Push25(R14, 248).encode(0).unwrap(), [0xfc, 0x7f]);
        for r in [R0, R7, R9, R12, R16, R31] {
            assert!(Push25(r, 0).encode(0).is_err());
            assert!(Pop25(r, 0).encode(0).is_err());
        }
        assert!(Push25(R6, 1).encode(0).is_err());
        assert!(Pop25(R6, 256).encode(0).is_err());
    }

    #[test]
    fn small_register_and_instruction_address_constraints_are_enforced() {
        for instruction in [Beqz38(R8, 0), Bnez38(R8, 0), Swi37Sp(R8, 0),
            Addri36Sp(R8, 0), Sbi333(R8, R0, 0), Lbi333(R0, R8, 0),
            Zeb33(R0, R8), Fexti33(R8, 0), Add333(R0, R0, R8)] {
            assert!(instruction.encode(0).is_err(), "accepted {instruction:?}");
        }
        assert!(Ret5(R30).encode(1).is_err());
        assert!(Movi(R0, 0).encode(u32::MAX - 1).is_err());
        assert_eq!(Ret5(R30).encode(0).unwrap(), [0xdd, 0x9e]);
    }

    #[test]
    fn widths_preserve_full_and_compact_forms() {
        let short = [Push25(R6, 0), Pop25(R6, 0), Mov55(R31, R0), Movi55(R0, 0),
            Swi37Sp(R0, 0), Ex9It(0), Ex9It(511), Beqz38(R0, 0), Bnez38(R0, 0),
            Sbi333(R0, R0, 0), Lbi333(R0, R0, 0), J8(0), Addri36Sp(R0, 0),
            Addi45(R0, 0), Lwi450(R0, R0), Srli45(R0, 0), Zeb33(R0, R0),
            Fexti33(R0, 0), Slts45(R0, R0), Beqzs8(0), Bnezs8(0),
            Add333(R0, R0, R0), Slti45(R0, 0), Slt45(R0, R0), Ret5(R30)];
        let wide = [Movi(R0, 0), LwiGp(R0, 0), SbiGp(R0, 0), LbiGp(R0, 0),
            Beq(R0, R0, 0), Bne(R0, R0, 0), Bnec(R0, 0, 0), Beqc(R0, 0, 0),
            Sbi(R0, R0, 0), Jal(0), Lb(R0, R0, R0, 0), Sb(R0, R0, R0, 0),
            Addi(R0, R0, 0), Lwi(R0, R0, 0)];
        for (instructions, expected) in [(&short[..], 2), (&wide[..], 4)] {
            for instruction in instructions {
                let bytes = instruction.encode(0).unwrap();
                assert_eq!(instruction.width(), expected);
                assert_eq!(bytes.len(), expected);
                assert_eq!(bytes[0] >> 7, if expected == 2 { 1 } else { 0 });
            }
        }
    }
}
