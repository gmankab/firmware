// SPDX-License-Identifier: GPL-3.0-only
//! Checked ARM A32 encoders for a small, explicitly supported instruction set.
//! These are architecture-level encodings, not a decompiler or assembler for
//! arbitrary ARM programs. Unsupported operations must be implemented openly.

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition { Eq = 0, Ne = 1, Cs = 2, Cc = 3, Mi = 4, Pl = 5, Hi = 8, Ls = 9, Ge = 10, Lt = 11, Gt = 12, Le = 13, Al = 14 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register(u8);
impl Register {
    pub const R0: Self = Self(0);
    pub const R1: Self = Self(1);
    pub const R2: Self = Self(2);
    pub const R11: Self = Self(11);
    pub const R12: Self = Self(12);
    pub const SP: Self = Self(13);
    pub const LR: Self = Self(14);
    pub const PC: Self = Self(15);
    pub fn new(n: u8) -> Result<Self, String> {
        if n > 15 { return Err("ARM32 register index exceeds 15".into()); }
        Ok(Self(n))
    }
    pub fn number(self) -> u32 { self.0 as u32 }
    pub fn bit(self) -> u16 { 1u16 << self.0 }
}

fn cond(cond: Condition) -> u32 { (cond as u32) << 28 }

/// Encode a PC-relative A32 B/BL, rejecting targets that cannot fit in imm24.
pub fn branch(condition: Condition, at: u32, target: u32, link: bool) -> Result<u32, String> {
    let delta = i64::from(target) - i64::from(at) - 8;
    if delta % 4 != 0 || !(-0x0200_0000..=0x01ff_fffc).contains(&delta) {
        return Err("ARM32 branch displacement outside signed 24-bit range".into());
    }
    let imm24 = ((delta / 4) as i32 as u32) & 0x00ff_ffff;
    Ok(cond(condition) | if link { 0x0b00_0000 } else { 0x0a00_0000 } | imm24)
}

/// A32 immediate-mode word LDR/STR, preindexed, no writeback.
pub fn transfer_word(condition: Condition, load: bool, rd: Register, rn: Register,
                     signed_offset: i32) -> Result<u32, String> {
    if signed_offset.unsigned_abs() > 4095 { return Err("ARM32 word offset exceeds 12 bits".into()); }
    Ok(cond(condition) | 0x0500_0000
       | if signed_offset >= 0 { 1 << 23 } else { 0 }
       | if load { 1 << 20 } else { 0 }
       | (rn.number() << 16) | (rd.number() << 12) | signed_offset.unsigned_abs())
}

/// A32 single-word LDR/STR with post-indexed immediate addressing.
/// The memory access uses the *unmodified* base and updates that base after
/// the transfer; unlike pre-indexed addressing, the P bit is clear.
pub fn transfer_word_postindexed(condition: Condition, load: bool, rd: Register,
                                 rn: Register, signed_offset: i32) -> Result<u32, String> {
    if signed_offset.unsigned_abs() > 4095 {
        return Err("ARM32 post-index offset exceeds 12 bits".into());
    }
    if rn == Register::PC || rd == rn && load {
        return Err("ARM32 post-index cannot use PC base or load into base".into());
    }
    Ok(cond(condition) | 0x0400_0000
       | if signed_offset >= 0 { 1 << 23 } else { 0 }
       | if load { 1 << 20 } else { 0 }
       | (rn.number() << 16) | (rd.number() << 12) | signed_offset.unsigned_abs())
}

/// Select a legal ARM modified-immediate rotation; many 32-bit constants are
/// intentionally not encodable with one A32 data-processing instruction.
fn modified_immediate(value: u32) -> Result<u32, String> {
    for rotation in 0..16u32 {
        let candidate = value.rotate_left(2 * rotation);
        if candidate <= 0xff {
            return Ok((rotation << 8) | candidate);
        }
    }
    Err(format!("0x{value:08x} is not representable as an A32 modified immediate"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataOp { And = 0, Sub = 2, Add = 4, Tst = 8, Cmp = 10, Orr = 12, Mov = 13, Bic = 14, Mvn = 15 }

pub fn data_immediate(condition: Condition, op: DataOp, rd: Register, rn: Register,
                      value: u32) -> Result<u32, String> {
    let test = matches!(op, DataOp::Cmp | DataOp::Tst);
    let rd_field = if test { 0 } else { rd.number() };
    Ok(cond(condition) | (1 << 25) | ((op as u32) << 21)
       | if test { 1 << 20 } else { 0 }
       | (rn.number() << 16) | (rd_field << 12) | modified_immediate(value)?)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shift { Lsl = 0, Lsr = 1, Asr = 2, Ror = 3 }

fn shifted_register(rm: Register, shift: Shift, amount: u8) -> Result<u32, String> {
    if amount > 31 || (shift != Shift::Lsl && amount == 0) {
        return Err("unsupported ARM32 register shift amount".into());
    }
    Ok((u32::from(amount) << 7) | ((shift as u32) << 5) | rm.number())
}

pub fn mov_shifted_register(condition: Condition, rd: Register, rm: Register,
                            shift: Shift, amount: u8) -> Result<u32, String> {
    Ok(cond(condition) | (13 << 21) | (rd.number() << 12)
       | shifted_register(rm, shift, amount)?)
}

pub fn data_immediate_with_flags(condition: Condition, op: DataOp, rd: Register,
                                 rn: Register, value: u32, flags: bool) -> Result<u32, String> {
    if matches!(op, DataOp::Cmp | DataOp::Tst) {
        return Err("CMP and TST always update flags; use data_immediate".into());
    }
    Ok(data_immediate(condition, op, rd, rn, value)? | if flags { 1 << 20 } else { 0 })
}

pub fn data_register(condition: Condition, op: DataOp, rd: Register, rn: Register,
                     rm: Register, shift: Shift, amount: u8,
                     flags: bool) -> Result<u32, String> {
    let test = matches!(op, DataOp::Cmp | DataOp::Tst);
    if flags && test { return Err("CMP and TST set flags automatically".into()); }
    Ok(cond(condition) | ((op as u32) << 21) | ((rn.number()) << 16)
       | if flags || test { 1 << 20 } else { 0 }
       | (if test { 0 } else { rd.number() } << 12)
       | shifted_register(rm, shift, amount)?)
}

/// STR/LDR word with a positively indexed register shifted by an immediate.
pub fn transfer_word_register(condition: Condition, load: bool, rd: Register,
                              rn: Register, rm: Register,
                              shift: Shift, amount: u8) -> Result<u32, String> {
    Ok(cond(condition) | 0x0780_0000 | (rn.number() << 16) | (rd.number() << 12)
       | if load { 1 << 20 } else { 0 }
       | shifted_register(rm, shift, amount)?)
}

pub fn mov_register(condition: Condition, rd: Register, rm: Register) -> u32 {
    cond(condition) | (13 << 21) | (rd.number() << 12) | rm.number()
}

/// Write the ARM CPSR control byte (mode and interrupt masks).
/// Only the control-field immediate form is implemented.
pub fn msr_cpsr_control(condition: Condition, control: u32) -> Result<u32, String> {
    Ok(cond(condition) | 0x0321_f000 | modified_immediate(control)?)
}

/// ARM coprocessor transfer (MCR), typically used for CP15 system registers.
pub fn mcr(condition: Condition, coprocessor: u8, opcode1: u8,
           source: Register, crn: u8, crm: u8, opcode2: u8) -> Result<u32, String> {
    if coprocessor > 15 || opcode1 > 7 || crn > 15 || crm > 15 || opcode2 > 7 {
        return Err("ARM MCR register/opcode field out of range".into());
    }
    Ok(cond(condition) | 0x0e00_0010 | (u32::from(opcode1) << 21)
       | (u32::from(crn) << 16) | (source.number() << 12)
       | (u32::from(coprocessor) << 8) | (u32::from(opcode2) << 5)
       | u32::from(crm))
}

/// Read an ARM coprocessor register (MRC).
pub fn mrc(condition: Condition, coprocessor: u8, opcode1: u8,
           dest: Register, crn: u8, crm: u8, opcode2: u8) -> Result<u32, String> {
    Ok(mcr(condition, coprocessor, opcode1, dest, crn, crm, opcode2)? | (1 << 20))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockMode { IncrementAfter, DecrementAfter, IncrementBefore, DecrementBefore }

pub fn transfer_multiple(condition: Condition, load: bool, base: Register,
                         regs: u16, mode: BlockMode, writeback: bool) -> Result<u32, String> {
    if regs == 0 { return Err("empty ARM register transfer list".into()); }
    let up = if matches!(mode, BlockMode::IncrementAfter | BlockMode::IncrementBefore) { 1 << 23 } else { 0 };
    let pre = if matches!(mode, BlockMode::IncrementBefore | BlockMode::DecrementBefore) { 1 << 24 } else { 0 };
    Ok(cond(condition) | 0x0800_0000 | up | pre
       | (if writeback { 1 << 21 } else { 0 })
       | (if load { 1 << 20 } else { 0 })
       | (base.number() << 16) | u32::from(regs))
}

/// MSR to all four CPSR or SPSR fields, register operand.
pub fn msr_status_register(condition: Condition, saved_status: bool,
                           source: Register) -> u32 {
    cond(condition) | (if saved_status { 0x016f_f000 } else { 0x012f_f000 })
       | source.number()
}

/// MSR CPSR_c, Rm: update the CPSR control field from a general register.
pub fn msr_cpsr_control_register(condition: Condition, source: Register) -> u32 {
    cond(condition) | 0x0121_f000 | source.number()
}

pub fn mrs_status_register(condition: Condition, saved_status: bool,
                           destination: Register) -> u32 {
    cond(condition) | (if saved_status { 0x014f_0000 } else { 0x010f_0000 })
       | (destination.number() << 12)
}

pub fn bx(condition: Condition, target: Register) -> u32 {
    cond(condition) | 0x012f_ff10 | target.number()
}

pub fn push_registers(condition: Condition, registers: u16) -> Result<u32, String> {
    if registers == 0 { return Err("empty ARM push register list".into()); }
    // STMDB SP!, {registers}.
    Ok(cond(condition) | 0x092d_0000 | u32::from(registers))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_and_exception_instruction_encodings() {
        assert_eq!(transfer_word(Condition::Al, true, Register::PC, Register::PC, 24).unwrap(), 0xe59f_f018);
        assert_eq!(push_registers(Condition::Al, Register::R11.bit() | Register::R12.bit()).unwrap(), 0xe92d_1800);
        assert_eq!(data_immediate(Condition::Al, DataOp::Mov, Register::R11, Register::R0, 4).unwrap(), 0xe3a0_b004);
        assert_eq!(branch(Condition::Al, 0x5c, 0x308, false).unwrap(), 0xea00_00a9);
        assert_eq!(branch(Condition::Eq, 0xa0, 0x94, false).unwrap(), 0x0aff_fffb);
        assert_eq!(mov_register(Condition::Al, Register::R0, Register::R0), 0xe1a0_0000);
    }
    #[test]
    fn immediate_and_transfer_encoding_and_rejections() {
        assert_eq!(data_immediate(Condition::Al, DataOp::And, Register::R1, Register::R1, 1).unwrap(), 0xe201_1001);
        assert_eq!(data_immediate(Condition::Al, DataOp::Cmp, Register::R0, Register::R1, 0).unwrap(), 0xe351_0000);
        assert_eq!(transfer_word(Condition::Al, true, Register::R0, Register::PC, -12).unwrap(), 0xe51f_000c);
        assert_eq!(transfer_word(Condition::Al, false, Register::R1, Register::R0, 0).unwrap(), 0xe580_1000);
        assert_eq!(msr_cpsr_control(Condition::Al, 0xd1).unwrap(), 0xe321_f0d1);
        assert_eq!(mcr(Condition::Al, 15, 0, Register::R0, 2, 0, 0).unwrap(), 0xee02_0f10);
        assert_eq!(mov_shifted_register(Condition::Al, Register::new(4).unwrap(), Register::new(4).unwrap(), Shift::Lsr, 20).unwrap(), 0xe1a0_4a24);
        assert_eq!(data_immediate_with_flags(Condition::Al, DataOp::Sub, Register::R1, Register::R1, 1, true).unwrap(), 0xe2511001);
        assert_eq!(data_register(Condition::Al, DataOp::Cmp, Register::R0, Register::R1, Register::new(4).unwrap(), Shift::Lsl, 0, false).unwrap(), 0xe1510004);
        assert_eq!(transfer_word_register(Condition::Al, false, Register::new(3).unwrap(), Register::R0, Register::R1, Shift::Lsl, 2).unwrap(), 0xe7803101);
        assert_eq!(mrc(Condition::Al, 15, 0, Register::R0, 1, 0, 0).unwrap(), 0xee11_0f10);
        assert_eq!(bx(Condition::Al, Register::R12), 0xe12f_ff1c);
        assert_eq!(transfer_word_postindexed(Condition::Al, false, Register::R2, Register::R1, 4).unwrap(), 0xe481_2004);
        assert_eq!(transfer_word_postindexed(Condition::Al, true, Register::R2, Register::R1, -4).unwrap(), 0xe411_2004);
        assert!(transfer_word_postindexed(Condition::Al, false, Register::R1, Register::R1, 4096).is_err());
        assert!(transfer_word_postindexed(Condition::Al, true, Register::R1, Register::R1, 4).is_err());
        assert_eq!(transfer_multiple(Condition::Al, true, Register::R12, Register::R0.bit() | Register::SP.bit() | Register::LR.bit(), BlockMode::DecrementAfter, true).unwrap(), 0xe83c6001);
        assert_eq!(msr_status_register(Condition::Al, true, Register::R0), 0xe16ff000);
        assert_eq!(msr_status_register(Condition::Al, false, Register::R2), 0xe12ff002);
        assert_eq!(mrs_status_register(Condition::Al, false, Register::R0), 0xe10f0000);
        assert_eq!(msr_cpsr_control_register(Condition::Al, Register::R0), 0xe121_f000);
        assert_eq!(data_immediate(Condition::Al, DataOp::Tst, Register::R0, Register::R0, 15).unwrap(), 0xe310000f);
        assert_eq!(data_immediate(Condition::Eq, DataOp::Orr, Register::R0, Register::R0, 15).unwrap(), 0x0380000f);
        assert_eq!(branch(Condition::Hi, 0x524, 0x558, false).unwrap(), 0x8a00000b);
        assert!(data_immediate_with_flags(Condition::Al, DataOp::Tst,
            Register::R0, Register::R0, 1, true).is_err());
        assert!(Register::new(16).is_err());
        assert!(branch(Condition::Al, 0, 3, false).is_err());
        assert!(branch(Condition::Al, 0, 0x80000000, false).is_err());
        assert!(transfer_word(Condition::Al, true, Register::R0, Register::PC, 4096).is_err());
        assert!(data_immediate(Condition::Al, DataOp::Mov, Register::R0, Register::R0, 0x12345678).is_err());
    }
}
