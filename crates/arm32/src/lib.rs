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

/// Encode an immediate ARM-to-Thumb BLX call. Unlike B/BL this encoding has
/// no condition field: the top nibble is 0xF, and bit 24 carries the halfword
/// alignment of the destination. The PC-relative displacement is signed 25
/// bits (counting halfwords); neither a destination nor an address is accepted
/// if it cannot be represented without truncation.
pub fn branch_exchange_immediate(at: u32, target: u32) -> Result<u32, String> {
    let delta = i64::from(target) - i64::from(at) - 8;
    if at % 4 != 0 || target % 2 != 0 || delta % 2 != 0 ||
       !(-0x0200_0000..=0x01ff_fffe).contains(&delta) {
        return Err("ARM32 BLX immediate requires aligned, encodable target".into());
    }
    let halfword = ((delta as u32 >> 1) & 1) << 24;
    let words = ((delta >> 2) as i32 as u32) & 0x00ff_ffff;
    Ok(0xfa00_0000 | halfword | words)
}

/// Compiler-generated ARM-mode trap used by LLVM/Clang `__builtin_trap`.
/// The fixed encoding is independently verified against LLVM 17's ARM
/// backend. This is an undefined/trap instruction, not an OEM opcode blob.
pub fn compiler_trap() -> u32 { 0xe7ff_defe }

/// A32 immediate-mode word LDR/STR, preindexed, no writeback.
pub fn transfer_word(condition: Condition, load: bool, rd: Register, rn: Register,
                     signed_offset: i32) -> Result<u32, String> {
    if signed_offset.unsigned_abs() > 4095 { return Err("ARM32 word offset exceeds 12 bits".into()); }
    Ok(cond(condition) | 0x0500_0000
       | if signed_offset >= 0 { 1 << 23 } else { 0 }
       | if load { 1 << 20 } else { 0 }
       | (rn.number() << 16) | (rd.number() << 12) | signed_offset.unsigned_abs())
}

/// A32 immediate-offset LDR/STR with pre-indexing and base writeback.
/// The base is updated before access; loading into that same base is unsafe.
pub fn transfer_word_preindexed_writeback(condition: Condition, load: bool,
                                         rd: Register, rn: Register,
                                         signed_offset: i32) -> Result<u32, String> {
    if rn == Register::PC || signed_offset.unsigned_abs() > 4095 || (load && rd == rn) {
        return Err("invalid A32 pre-indexed transfer with writeback".into());
    }
    Ok(transfer_word(condition, load, rd, rn, signed_offset)? | (1 << 21))
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

/// ARM A32 byte LDRB/STRB with immediate pre-indexed addressing.
pub fn transfer_byte(condition: Condition, load: bool, rd: Register,
                     rn: Register, signed_offset: i32) -> Result<u32, String> {
    Ok(transfer_word(condition, load, rd, rn, signed_offset)? | (1 << 22))
}

/// ARM A32 byte LDRB/STRB with post-indexed immediate addressing.
pub fn transfer_byte_postindexed(condition: Condition, load: bool, rd: Register,
                                 rn: Register, signed_offset: i32) -> Result<u32, String> {
    Ok(transfer_word_postindexed(condition, load, rd, rn, signed_offset)? | (1 << 22))
}

/// ARM A32 LDRSB immediate offset: sign-extend one byte into a register.
/// Supports a signed, pre-indexed 8-bit displacement without base writeback.
/// These extra load/store instruction encodings split imm8 across two nibbles.
pub fn load_signed_byte(condition: Condition, rd: Register, rn: Register,
                        signed_offset: i32) -> Result<u32, String> {
    if rd == Register::PC || rn == Register::PC || signed_offset.unsigned_abs() > 255 {
        return Err("invalid ARM32 signed-byte load operands".into());
    }
    let magnitude = signed_offset.unsigned_abs();
    Ok(cond(condition) | 0x0150_00d0
        | if signed_offset >= 0 { 1 << 23 } else { 0 }
        | (rn.number() << 16) | (rd.number() << 12)
        | ((magnitude & 0xf0) << 4) | (magnitude & 0x0f))
}

/// A32 signed-byte load with post-indexed 8-bit immediate displacement.
/// The base register is updated *after* the signed byte is read.
/// Reject PC and load-into-base aliases, both architecturally unsafe.
pub fn load_signed_byte_postindexed(condition: Condition, rd: Register,
                                    rn: Register, signed_offset: i32) -> Result<u32,String> {
    if rd == Register::PC || rn == Register::PC || rd == rn || signed_offset.unsigned_abs() > 255 {
        return Err("invalid ARM32 signed-byte postindex operands".into());
    }
    let magnitude = signed_offset.unsigned_abs();
    Ok(cond(condition) | 0x0050_00d0
        | if signed_offset >= 0 {1 << 23} else {0}
        | (rn.number() << 16) | (rd.number() << 12)
        | ((magnitude & 0xf0) << 4) | (magnitude & 0x0f))
}

/// A32 LDRSB with unshifted positive register offset, preserving signed
/// extension to a 32-bit destination; PC operands are not supported.
pub fn load_signed_byte_register(condition: Condition, rd: Register,
                                 rn: Register, rm: Register) -> Result<u32,String> {
    if [rd,rn,rm].contains(&Register::PC) {
        return Err("A32 LDRSB register offset must not use PC".into());
    }
    Ok(cond(condition) | 0x0190_00d0 | (rn.number()<<16)
       | (rd.number()<<12) | rm.number())
}

/// A32 immediate byte transfer with pre-indexed writeback (`[Rn, #imm]!`).
/// Rejects ambiguous load-into-base and use of PC as the modified base.
pub fn transfer_byte_preindexed_writeback(condition: Condition, load: bool,
                                          rd: Register, rn: Register,
                                          signed_offset: i32) -> Result<u32, String> {
    if rn == Register::PC || (load && rd == rn) || signed_offset.unsigned_abs() > 4095 {
        return Err("invalid ARM32 byte transfer with pre-index writeback".into());
    }
    Ok(transfer_byte(condition, load, rd, rn, signed_offset)? | (1 << 21))
}

/// ARM A32 STRH with post-indexed signed immediate offset (8-bit field).
pub fn store_halfword_postindexed(condition: Condition, rd: Register, rn: Register,
                                  signed_offset: i32) -> Result<u32, String> {
    if rd == Register::PC || rn == Register::PC || signed_offset.unsigned_abs() > 255 {
        return Err("invalid ARM32 halfword post-index operands".into());
    }
    let magnitude = signed_offset.unsigned_abs();
    Ok(cond(condition) | 0x0040_00b0
       | if signed_offset >= 0 { 1 << 23 } else { 0 }
       | (rn.number() << 16) | (rd.number() << 12)
       | ((magnitude & 0xf0) << 4) | (magnitude & 0x0f))
}

/// A32 immediate-offset halfword store (`STRH Rt, [Rn, #imm]`).
/// The split 8-bit displacement is checked and assembled from its nibbles.
pub fn store_halfword(condition: Condition, rd: Register, rn: Register,
                      signed_offset: i32) -> Result<u32, String> {
    if rd == Register::PC || rn == Register::PC || signed_offset.unsigned_abs() > 255 {
        return Err("invalid ARM32 halfword store operands".into());
    }
    let magnitude = signed_offset.unsigned_abs();
    Ok(cond(condition) | 0x0140_00b0
        | if signed_offset >= 0 { 1 << 23 } else { 0 }
        | (rn.number() << 16) | (rd.number() << 12)
        | ((magnitude & 0xf0) << 4) | (magnitude & 0x0f))
}

/// A32 immediate-offset halfword load (`LDRH Rt, [Rn, #imm]`).
/// Uses the same split immediate as STRH with the load bit enabled.
pub fn load_halfword(condition: Condition, rd: Register, rn: Register,
                     signed_offset: i32) -> Result<u32, String> {
    Ok(store_halfword(condition, rd, rn, signed_offset)? | (1 << 20))
}

/// A32 register-offset halfword transfer, pre-indexed without writeback.
/// A32 uses a different addressing-field layout from word loads/stores:
/// the register offset is only Rm (no barrel shift), with bits 7 and 4 set.
pub fn transfer_halfword_register(condition: Condition, load: bool,
                                  rd: Register, rn: Register, rm: Register)
                                  -> Result<u32, String> {
    if [rd, rn, rm].contains(&Register::PC) {
        return Err("PC is not valid in A32 register-indexed halfword transfers".into());
    }
    Ok(cond(condition) | 0x0180_00b0 | (if load { 1 << 20 } else { 0 })
       | (rn.number() << 16) | (rd.number() << 12) | rm.number())
}

/// A32 signed halfword load with a checked signed imm8 displacement.
/// This uses the halfword transfer's split eight-bit immediate field.
pub fn load_signed_halfword(condition: Condition, rd: Register, rn: Register,
                            signed_offset: i32) -> Result<u32, String> {
    if rd == Register::PC || rn == Register::PC || signed_offset.unsigned_abs() > 255 {
        return Err("invalid ARM32 signed-halfword load operands".into());
    }
    let offset = signed_offset.unsigned_abs();
    Ok(cond(condition) | 0x0150_00f0
        | if signed_offset >= 0 { 1 << 23 } else { 0 }
        | (rn.number() << 16) | (rd.number() << 12)
        | ((offset & 0xf0) << 4) | (offset & 0x0f))
}

/// A32 register-indexed halfword LDRH/STRH without writeback.
/// Registers are unshifted; the instruction requires dedicated extra-load/
/// store fields, unlike normal word-register addressing.
pub fn transfer_halfword_register(condition: Condition, load: bool,
    rd: Register, rn: Register, rm: Register) -> Result<u32,String> {
    if [rd,rn,rm].contains(&Register::PC) {
        return Err("PC is not a supported A32 halfword register transfer operand".into());
    }
    Ok(cond(condition) | 0x0180_00b0 | if load { 1 << 20 } else { 0 }
       | (rn.number() << 16) | (rd.number() << 12) | rm.number())
}

/// A32 LDRSH signed halfword with unshifted register offset.
/// The architecture uses the extra-load/store group with S/H opcode bits.
pub fn load_signed_halfword_register(condition: Condition,rd:Register,
                                     rn:Register,rm:Register) -> Result<u32,String> {
    if [rd,rn,rm].contains(&Register::PC) {
        return Err("A32 LDRSH register offset must not use PC".into());
    }
    Ok(cond(condition) | 0x0190_00f0 | (rn.number()<<16)
       | (rd.number()<<12) | rm.number())
}

/// A32 aligned LDRD or STRD: the pair is always Rt and Rt+1.
/// Invalid odd or high register pairs, PC bases and large displacements are
/// rejected instead of silently encoding unpredictable instructions.
pub fn transfer_doubleword(condition: Condition, load: bool,
                           rd: Register, rn: Register,
                           signed_offset: i32) -> Result<u32, String> {
    if rd.number() > 12 || rd.number() % 2 != 0 || rn == Register::PC
        || signed_offset.unsigned_abs() > 255 {
        return Err("invalid ARM32 doubleword transfer operands".into());
    }
    let offset = signed_offset.unsigned_abs();
    Ok(cond(condition) | if load { 0x0140_00d0 } else { 0x0140_00f0 }
        | if signed_offset >= 0 { 1 << 23 } else { 0 }
        | (rn.number() << 16) | (rd.number() << 12)
        | ((offset & 0xf0) << 4) | (offset & 0x0f))
}

/// ARM A32 register-offset post-indexed word transfer (`[Rn], Rm`).
/// Only positive, unshifted register offsets are currently supported.
/// The base register is updated after the access, including when the
/// transfer's source register is also used as the index.
pub fn transfer_word_postindexed_register(condition: Condition, load: bool,
                                         rd: Register, rn: Register, rm: Register) -> Result<u32, String> {
    if rn == Register::PC || rm == Register::PC || (load && rd == rn) {
        return Err("unsafe or undefined A32 register post-index addressing".into());
    }
    Ok(cond(condition) | 0x0680_0000
       | if load { 1 << 20 } else { 0 }
       | (rn.number() << 16) | (rd.number() << 12) | rm.number())
}

/// A32 post-indexed LDR/STR word with a scaled register offset.
/// Address `[Rn], Rm, LSL #amount` updates Rn after the transfer. Only the
/// architecture-defined shifted-register immediate form is supported.
pub fn transfer_word_postindexed_scaled_register(condition: Condition, load: bool,
      rd: Register, rn: Register, rm: Register, shift: Shift, amount: u8) -> Result<u32,String> {
    if rn == Register::PC || rm == Register::PC || (load && rd == rn) {
        return Err("unsafe A32 postindexed scaled register addressing".into());
    }
    Ok(transfer_word_register(condition,load,rd,rn,rm,shift,amount)? & !(1 << 24))
}

/// A32 register-offset post-indexed byte transfer (`[Rn], Rm`). The
/// word variant validates unsafe register combinations before the byte bit
/// is set, so unsupported modes cannot silently pass through.
pub fn transfer_byte_postindexed_register(condition: Condition, load: bool,
                                          rd: Register, rn: Register,
                                          rm: Register) -> Result<u32,String> {
    Ok(transfer_word_postindexed_register(condition, load, rd, rn, rm)? | (1<<22))
}

/// A32 pre-indexed register-offset word transfer with writeback.
/// The offset is `Rm << amount`. Register fields are validated independently,
/// and PC cannot be modified as the address base.
pub fn transfer_word_register_preindexed_writeback(
    condition: Condition, load: bool, rd: Register, rn: Register,
    rm: Register, shift: Shift, amount: u8,
) -> Result<u32, String> {
    if rn == Register::PC || rm == Register::PC || (load && rd == rn) {
        return Err("invalid A32 pre-indexed register transfer with writeback".into());
    }
    Ok(transfer_word_register(condition, load, rd, rn, rm, shift, amount)? | (1 << 21))
}

/// A32 LDR/STR word with a shifted positive or negative register offset.
/// This is the pre-indexed no-writeback form; the U bit chooses addition
/// or subtraction of `Rm << shift`. Reject unsupported PC address fields.
pub fn transfer_word_register_offset(condition: Condition, load: bool,
                                     rd: Register, rn: Register, rm: Register,
                                     shift: Shift, amount: u8,
                                     subtract: bool) -> Result<u32,String> {
    if rn == Register::PC || rm == Register::PC || rd == Register::PC {
        return Err("invalid A32 word transfer register offset operands".into());
    }
    let base = transfer_word_register(condition,load,rd,rn,rm,shift,amount)?;
    Ok(if subtract {base & !(1 << 23)} else {base})
}

/// ARM A32 register-indexed pre-indexed byte load/store without writeback.
pub fn transfer_byte_register(condition: Condition, load: bool, rd: Register,
                              rn: Register, rm: Register,
                              shift: Shift, amount: u8) -> Result<u32, String> {
    Ok(transfer_word_register(condition, load, rd, rn, rm, shift, amount)? | (1 << 22))
}

/// A32 byte transfer with register-offset preindexing and base writeback.
/// Supports positive Rm << amount shifts; rejects undefined PC/base aliases.
pub fn transfer_byte_register_preindexed_writeback(condition: Condition,
    load: bool, rd: Register, rn: Register, rm: Register,
    shift: Shift, amount: u8) -> Result<u32,String> {
    Ok(transfer_word_register_preindexed_writeback(condition,load,rd,rn,rm,shift,amount)?
       | (1 << 22))
}

/// A32 DSP SMLABB: signed 16-bit bottom-half multiply and accumulate.
/// Computes `Rd = (i16)Rm[15:0] * (i16)Rs[15:0] + Rn`.
/// Reject PC in any operand; no vendor instruction bytes are embedded.
pub fn signed_multiply_accumulate_bottom_halves(condition: Condition,
    rd: Register, rm: Register, rs: Register, rn: Register) -> Result<u32,String> {
    if [rd,rm,rs,rn].contains(&Register::PC) {
        return Err("SMLABB cannot use PC as an operand".into());
    }
    Ok(cond(condition) | 0x0100_0080 | (rd.number()<<16)
       | (rn.number()<<12) | (rs.number()<<8) | rm.number())
}

/// A32 MLA: multiply two registers and add a third, without flag updates.
/// The destination may alias an input, but PC is forbidden in each operand.
/// A32 register-offset byte load/store supporting addition and subtraction
/// of a shifted offset register. Restricts unsafe base/PC combinations.
pub fn transfer_byte_register_offset(condition: Condition, load: bool,
                                     rd: Register, rn: Register, rm: Register,
                                     shift: Shift, amount: u8,
                                     subtract: bool) -> Result<u32, String> {
    if rn == Register::PC || rm == Register::PC {
        return Err("invalid ARM32 indexed byte transfer registers".into());
    }
    let encoded = transfer_byte_register(condition, load, rd, rn, rm, shift, amount)?;
    Ok(if subtract { encoded & !(1 << 23) } else { encoded })
}

pub fn multiply_accumulate(condition: Condition, rd: Register, rm: Register,
                           rs: Register, addend: Register) -> Result<u32, String> {
    if [rd, rm, rs, addend].contains(&Register::PC) {
        return Err("PC is not permitted in A32 MLA operands".into());
    }
    Ok(cond(condition) | 0x0020_0090 | (rd.number() << 16)
       | (addend.number() << 12) | (rs.number() << 8) | rm.number())
}

/// A32 SMULBB: signed 16-bit low halves multiplied into a 32-bit result.
/// This is the public ARM DSP extension encoding, not an extracted opcode.
pub fn signed_multiply_bottom_halves(condition: Condition, rd: Register,
                                     rm: Register, rs: Register) -> Result<u32, String> {
    if [rd, rm, rs].contains(&Register::PC) {
        return Err("SMULBB cannot take PC as a register operand".into());
    }
    Ok(cond(condition) | 0x0160_0080 | (rd.number() << 16)
       | (rs.number() << 8) | rm.number())
}

/// A32 SMULBB: multiply the signed bottom 16 bits of two ARM registers.
/// This is an architectural DSP instruction, not an OEM opcode constant.
pub fn signed_multiply_bottom_halves(condition: Condition, rd: Register,
                                   rm: Register, rs: Register) -> Result<u32,String> {
    if [rd,rm,rs].contains(&Register::PC) {
        return Err("ARM32 SMULBB must not use PC".into());
    }
    Ok(cond(condition) | 0x0160_0080 | (rd.number()<<16)
      | (rs.number()<<8) | rm.number())
}

/// A32 MUL: multiply two general-purpose registers, without accumulate.
/// The register fields are checked, and older ARM restrictions on using the
/// destination as the first multiplicand are enforced.
pub fn multiply_register(condition: Condition, rd: Register,
                         rm: Register, rs: Register) -> Result<u32, String> {
    if [rd, rm, rs].contains(&Register::PC) || rd == rm {
        return Err("invalid ARM32 MUL register operands".into());
    }
    Ok(cond(condition) | 0x0000_0090
        | (rd.number() << 16) | (rs.number() << 8) | rm.number())
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
pub enum DataOp { And = 0, Eor = 1, Sub = 2, Rsb = 3, Add = 4, Sbc = 6, Rsc = 7, Tst = 8, Teq = 9, Cmp = 10, Cmn = 11, Orr = 12, Mov = 13, Bic = 14, Mvn = 15 }

pub fn data_immediate(condition: Condition, op: DataOp, rd: Register, rn: Register,
                      value: u32) -> Result<u32, String> {
    let test = matches!(op, DataOp::Cmp | DataOp::Tst | DataOp::Teq | DataOp::Cmn);
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
    if matches!(op, DataOp::Cmp | DataOp::Tst | DataOp::Teq | DataOp::Cmn) {
        return Err("CMP, TST and TEQ always update flags; use data_immediate".into());
    }
    Ok(data_immediate(condition, op, rd, rn, value)? | if flags { 1 << 20 } else { 0 })
}

pub fn data_register(condition: Condition, op: DataOp, rd: Register, rn: Register,
                     rm: Register, shift: Shift, amount: u8,
                     flags: bool) -> Result<u32, String> {
    let test = matches!(op, DataOp::Cmp | DataOp::Tst | DataOp::Teq | DataOp::Cmn);
    if flags && test { return Err("CMP, TST and TEQ set flags automatically".into()); }
    Ok(cond(condition) | ((op as u32) << 21) | ((rn.number()) << 16)
       | if flags || test { 1 << 20 } else { 0 }
       | (if test { 0 } else { rd.number() } << 12)
       | shifted_register(rm, shift, amount)?)
}

/// A32 data processing using a shift amount held in another register.
/// Separate from the immediate-shift encoding: operand2 bit 4 is set and
/// shift amount comes from the low byte of `rs` at execution time.
pub fn data_register_shifted_by_register(condition: Condition, op: DataOp,
                                         rd: Register, rn: Register, rm: Register,
                                         shift: Shift, rs: Register,
                                         flags: bool) -> Result<u32, String> {
    if rs == Register::PC || rm == Register::PC {
        return Err("PC is not a valid ARM32 shift operand".into());
    }
    let test = matches!(op, DataOp::Cmp | DataOp::Tst | DataOp::Teq | DataOp::Cmn);
    if test && flags { return Err("test opcodes always set flags".into()); }
    Ok(cond(condition) | ((op as u32) << 21) | (rn.number() << 16)
       | (if flags || test { 1 << 20 } else { 0 })
       | ((if test { 0 } else { rd.number() }) << 12)
       | (rs.number() << 8) | ((shift as u32) << 5) | 0x10 | rm.number())
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

/// ARM32 BLX Rm: branch with link and exchange instruction state.
/// The source may be any non-PC general-purpose register.
pub fn branch_exchange_with_link(condition: Condition, target: Register) -> Result<u32, String> {
    if target == Register::PC {
        return Err("ARM32 BLX register must not be PC".into());
    }
    Ok(cond(condition) | 0x012f_ff30 | target.number())
}

/// ARM A32 SVC/SWI instruction with checked 24-bit immediate.
pub fn supervisor_call(condition: Condition, immediate: u32) -> Result<u32, String> {
    if immediate > 0x00ff_ffff {
        return Err("ARM32 supervisor call immediate exceeds 24 bits".into());
    }
    Ok(cond(condition) | 0x0f00_0000 | immediate)
}

pub fn push_registers(condition: Condition, registers: u16) -> Result<u32, String> {
    if registers == 0 { return Err("empty ARM push register list".into()); }
    // STMDB SP!, {registers}.
    Ok(cond(condition) | 0x092d_0000 | u32::from(registers))
}

#[cfg(test)]
mod tests {
    #[test]
    fn smulbb_encoding_and_register_validation() {
        let r6 = Register::new(6).unwrap();
        let r7 = Register::new(7).unwrap();
        assert_eq!(signed_multiply_bottom_halves(Condition::Al,
            Register::R1, Register::LR, r6).unwrap(), 0xe161_068e);
        assert_eq!(signed_multiply_bottom_halves(Condition::Eq,
            r7, Register::R0, Register::R1).unwrap(), 0x0167_0180);
        assert!(signed_multiply_bottom_halves(Condition::Al,
            Register::PC, Register::R1, r6).is_err());
    }

    use super::*;
    #[test]
    fn register_offset_halfword_encoding_and_rejections() {
        let r4=Register::new(4).unwrap();
        let r9=Register::new(9).unwrap();
        assert_eq!(transfer_halfword_register(Condition::Al,false,
            Register::R0,r4,Register::R1).unwrap(),0xe184_00b1);
        assert_eq!(transfer_halfword_register(Condition::Al,false,
            Register::R0,r4,r9).unwrap(),0xe184_00b9);
        assert_eq!(transfer_halfword_register(Condition::Al,true,
            Register::R0,r4,r9).unwrap(),0xe194_00b9);
        assert_eq!(transfer_halfword_register(Condition::Al,false,
            Register::R1,r4,Register::R2).unwrap(),0xe184_10b2);
        assert!(transfer_halfword_register(Condition::Al,true,
            Register::R0,Register::PC,r9).is_err());
        assert!(transfer_halfword_register(Condition::Al,false,
            Register::PC,r4,r9).is_err());
    }

    #[test]
    fn teq_always_updates_flags_and_has_no_destination() {
        // Independently disassembled A32 `teq r1, r2` = e1310002.
        assert_eq!(data_register(Condition::Al, DataOp::Teq, Register::R0,
            Register::R1, Register::R2, Shift::Lsl, 0, false).unwrap(), 0xe131_0002);
        assert!(data_register(Condition::Al, DataOp::Teq, Register::R0,
            Register::R1, Register::R2, Shift::Lsl, 0, true).is_err());
        assert_eq!(data_immediate(Condition::Al, DataOp::Teq, Register::R0,
            Register::R1, 1).unwrap(), 0xe331_0001);
        assert!(data_immediate_with_flags(Condition::Al, DataOp::Teq,
            Register::R0, Register::R1, 1, true).is_err());
    }
    #[test]
    fn thumb_interworking_blx_encoding_and_range_checks() {
        // ARM BLX immediate follows a signed PC+8 displacement. Bit H
        // represents the extra 2-byte target adjustment for Thumb code.
        assert_eq!(branch_exchange_immediate(0x1000, 0x1008).unwrap(), 0xfa00_0000);
        assert_eq!(branch_exchange_immediate(0x1000, 0x100a).unwrap(), 0xfb00_0000);
        assert_eq!(branch_exchange_immediate(0x5978, 0x0e_2198).unwrap(), 0xfa03_7206);
        assert!(branch_exchange_immediate(0x1002, 0x1008).is_err());
        assert!(branch_exchange_immediate(0x1000, 0x1009).is_err());
        assert!(branch_exchange_immediate(0x1000, 0x4000_0000).is_err());
    }
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
    fn halfword_load_encoding() {
        assert_eq!(load_halfword(Condition::Al, Register::new(3).unwrap(),
            Register::R0, 0).unwrap(), 0xe1d0_30b0);
        assert!(load_halfword(Condition::Al, Register::R0,
            Register::R1, -256).is_err());
    }
    #[test]
    fn halfword_and_carry_aware_arithmetic() {
        assert_eq!(store_halfword(Condition::Al, Register::new(4).unwrap(),
            Register::R0, 0).unwrap(), 0xe1c0_40b0);
        assert_eq!(data_immediate_with_flags(Condition::Al, DataOp::Rsb,
            Register::R1, Register::R1, 0, true).unwrap(), 0xe271_1000);
        assert_eq!(data_immediate(Condition::Al, DataOp::Rsc,
            Register::R0, Register::R0, 0).unwrap(), 0xe2e0_0000);
        assert!(store_halfword(Condition::Al, Register::PC,
            Register::R1, 0).is_err());
        assert!(store_halfword(Condition::Al, Register::R1,
            Register::R0, -256).is_err());
    }
    #[test]
    fn signed_halfword_register_offset_encoding() {
        let r9=Register::new(9).unwrap();
        assert_eq!(load_signed_halfword_register(Condition::Al,
            Register::R0,r9,Register::R0).unwrap(),0xe199_00f0);
        assert_eq!(load_signed_halfword_register(Condition::Al,
            Register::R1,r9,Register::R1).unwrap(),0xe199_10f1);
        assert!(load_signed_halfword_register(Condition::Al,
            Register::PC,r9,Register::R0).is_err());
    }

    #[test]
    fn register_halfword_and_byte_preindexed_writeback_encoders() {
        let r3=Register::new(3).unwrap();
        let r4=Register::new(4).unwrap();
        let r6=Register::new(6).unwrap();
        assert_eq!(transfer_halfword_register(Condition::Al,true,
            Register::R1,r4,Register::R1).unwrap(),0xe194_10b1);
        assert_eq!(transfer_byte_register_preindexed_writeback(Condition::Al,false,
            r6,r3,Register::LR,Shift::Lsl,0).unwrap(),0xe7e3_600e);
        assert!(transfer_halfword_register(Condition::Al,true,
            Register::PC,r3,r4).is_err());
        assert!(transfer_byte_register_preindexed_writeback(Condition::Al,true,
            r3,r3,Register::R1,Shift::Lsl,0).is_err());
    }

    #[test]
    fn halfword_writeback_and_postindex_encoders() {
        let r9=Register::new(9).unwrap();
        let r10=Register::new(10).unwrap();
        let r3=Register::new(3).unwrap();
        assert_eq!(load_halfword_register_preindexed_writeback(Condition::Al,
            Register::R2, r10, Register::R0).unwrap(),0xe1ba_20b0);
        assert_eq!(load_halfword_register_preindexed_writeback(Condition::Al,
            Register::R0, r9, Register::R0).unwrap(),0xe1b9_00b0);
        assert_eq!(load_halfword_preindexed_writeback(Condition::Al,
            r3, Register::R0, -2).unwrap(),0xe170_30b2);
        assert_eq!(load_halfword_postindexed(Condition::Al,Register::R2,
            Register::R0,2).unwrap(),0xe0d0_20b2);
        assert!(load_halfword_register_preindexed_writeback(Condition::Al,
            Register::R2,Register::R2,Register::R0).is_err());
        assert!(load_halfword_postindexed(Condition::Al,
            Register::R0,Register::R0,2).is_err());
        assert!(load_halfword_preindexed_writeback(Condition::Al,
            Register::R0,Register::R0,-2).is_err());
        assert!(load_halfword_postindexed(Condition::Al,
            Register::R2,Register::PC,2).is_err());
    }

    #[test]
    fn smlabb_signed_halfword_multiply_accumulate_encoding() {
        let r3=Register::new(3).unwrap();
        let r7=Register::new(7).unwrap();
        assert_eq!(signed_multiply_accumulate_bottom_halves(Condition::Al,
            r7,r3,Register::R1,r7).unwrap(),0xe107_7183);
        assert_eq!(signed_multiply_accumulate_bottom_halves(Condition::Al,
            Register::R2,r3,Register::R1,Register::R2).unwrap(),0xe102_2183);
        assert!(signed_multiply_accumulate_bottom_halves(Condition::Al,
            Register::PC,r3,Register::R1,r7).is_err());
    }

    #[test]
    fn byte_postindexed_register_uses_checked_addressing() {
        let r3 = Register::new(3).unwrap();
        assert_eq!(transfer_byte_postindexed_register(Condition::Al,false,
            r3,Register::R0,Register::R1).unwrap(),0xe6c0_3001);
        assert_eq!(transfer_byte_postindexed_register(Condition::Eq,false,
            r3,Register::R0,Register::R1).unwrap(),0x06c0_3001);
        assert!(transfer_byte_postindexed_register(Condition::Al,true,
            Register::R0,Register::R0,Register::R1).is_err());
        assert!(transfer_byte_postindexed_register(Condition::Al,false,
            Register::R1,Register::PC,Register::R0).is_err());
    }

    #[test]
    fn indexed_transfers_and_multiply_accumulate() {
        let r3 = Register::new(3).unwrap();
        let r4 = Register::new(4).unwrap();
        let r5 = Register::new(5).unwrap();
        let r9 = Register::new(9).unwrap();
        assert_eq!(multiply_accumulate(Condition::Al, Register::R1, r5, r3, r9).unwrap(), 0xe021_9395);
        assert_eq!(transfer_byte_register(Condition::Al, true, Register::R1,
            Register::R1, Register::R0, Shift::Lsl, 0).unwrap(), 0xe7d1_1000);
        assert_eq!(transfer_word_register_preindexed_writeback(Condition::Al, true,
            Register::R1, Register::R0, r4, Shift::Lsl, 3).unwrap(), 0xe7b0_1184);
        assert!(multiply_accumulate(Condition::Al, Register::PC, r5, r3, r9).is_err());
        assert!(transfer_word_register_preindexed_writeback(Condition::Al, true,
            Register::R0, Register::R0, r4, Shift::Lsl, 0).is_err());
    }
    #[test]
    fn byte_preindexed_writeback_encoding() {
        assert_eq!(transfer_byte_preindexed_writeback(Condition::Al, true,
            Register::R1, Register::R0, 1).unwrap(), 0xe5f0_1001);
        assert!(transfer_byte_preindexed_writeback(Condition::Al, true,
            Register::R0, Register::R0, 1).is_err());
        assert!(transfer_byte_preindexed_writeback(Condition::Al, false,
            Register::R1, Register::PC, 1).is_err());
    }
    #[test]
    fn clang_builtin_trap_uses_arm_mode_trap_encoding() {
        // Cross-checked by compiling `__builtin_trap()` for armv7-none-eabi.
        assert_eq!(compiler_trap(), 0xe7ff_defe);
    }
    #[test]
    fn independent_arm_multiply_encoding() {
        let r3 = Register::new(3).unwrap();
        let r4 = Register::new(4).unwrap();
        let r5 = Register::new(5).unwrap();
        assert_eq!(multiply_register(Condition::Al, r3, Register::R1, Register::R12).unwrap(), 0xe003_0c91);
        assert_eq!(multiply_register(Condition::Al, r5, Register::R2, r4).unwrap(), 0xe005_0492);
        assert!(multiply_register(Condition::Al, r3, r3, Register::R1).is_err());
    }
    #[test]
    fn compare_negative_immediate() {
        assert_eq!(data_immediate(Condition::Al, DataOp::Cmn,
            Register::R0, Register::R0, 0x8000_0001).unwrap(), 0xe370_0106);
    }
    #[test]
    fn signed_register_index_byte_transfer() {
        let r3 = Register::new(3).unwrap();
        let r7 = Register::new(7).unwrap();
        assert_eq!(transfer_byte_register_offset(Condition::Al, true,
            r3, Register::R1, r7, Shift::Lsl, 0, true).unwrap(), 0xe751_3007);
        assert!(transfer_byte_register_offset(Condition::Al, true,
            r3, Register::PC, r7, Shift::Lsl, 0, true).is_err());
        assert!(transfer_byte_register_offset(Condition::Al, true,
            r3, Register::R1, Register::PC, Shift::Lsl, 0, true).is_err());
    }
    #[test]
    fn signed_halfword_and_doubleword_memory_transfers() {
        let r2 = Register::new(2).unwrap();
        let r3 = Register::new(3).unwrap();
        // Encodings cross-checked against the GNU-compatible ARM assembler.
        assert_eq!(load_signed_halfword(Condition::Al, r2, Register::R1, 0).unwrap(), 0xe1d1_20f0);
        assert_eq!(transfer_doubleword(Condition::Al, true, r2, Register::R1, 0).unwrap(), 0xe1c1_20d0);
        assert_eq!(transfer_doubleword(Condition::Al, false, r2, Register::R0, 0).unwrap(), 0xe1c0_20f0);
        assert!(load_signed_halfword(Condition::Al, Register::PC, Register::R1, 0).is_err());
        assert!(load_signed_halfword(Condition::Al, r2, Register::R1, -256).is_err());
        assert!(transfer_doubleword(Condition::Al, true, r3, Register::R1, 0).is_err());
        assert!(transfer_doubleword(Condition::Al, false, r2, Register::PC, 0).is_err());
        assert!(transfer_doubleword(Condition::Al, true, r2, Register::R0, 256).is_err());
    }
    #[test]
    fn scaled_postindexed_register_uses_architecture_fields() {
        assert_eq!(transfer_word_postindexed_scaled_register(Condition::Al,true,
            Register::R0,Register::R1,Register::R2,Shift::Lsl,2).unwrap(),0xe691_0102);
        let r4=Register::new(4).unwrap();
        assert_eq!(transfer_word_postindexed_scaled_register(Condition::Al,true,
            Register::R1,r4,Register::R1,Shift::Lsl,2).unwrap(),0xe694_1101);
        assert!(transfer_word_postindexed_scaled_register(Condition::Al,true,
            Register::R1,Register::R1,Register::R2,Shift::Lsl,2).is_err());
        assert!(transfer_word_postindexed_scaled_register(Condition::Al,true,
            Register::R0,Register::PC,Register::R2,Shift::Lsl,2).is_err());
        assert!(transfer_word_postindexed_scaled_register(Condition::Al,true,
            Register::R0,Register::R1,Register::PC,Shift::Lsl,2).is_err());
        assert!(transfer_word_postindexed_scaled_register(Condition::Al,true,
            Register::R0,Register::R1,Register::R2,Shift::Lsl,32).is_err());
    }

    #[test]
    fn signed_byte_postindexed_immediate_is_architecture_encoded() {
        let r3=Register::new(3).unwrap();
        assert_eq!(load_signed_byte_postindexed(Condition::Al,r3,Register::R1,1).unwrap(),0xe0d1_30d1);
        assert_eq!(load_signed_byte_postindexed(Condition::Eq,Register::R2,Register::R1,-2).unwrap(),0x0051_20d2);
        assert!(load_signed_byte_postindexed(Condition::Al,Register::R1,Register::R1,1).is_err());
        assert!(load_signed_byte_postindexed(Condition::Al,Register::PC,Register::R1,1).is_err());
        assert!(load_signed_byte_postindexed(Condition::Al,r3,Register::PC,0).is_err());
        assert!(load_signed_byte_postindexed(Condition::Al,r3,Register::R1,256).is_err());
    }

    #[test]
    fn signed_byte_load_encoding_and_rejections() {
        let r4 = Register::new(4).unwrap();
        assert_eq!(load_signed_byte(Condition::Al, Register::R1, r4, 74).unwrap(), 0xe1d4_14da);
        assert_eq!(load_signed_byte(Condition::Eq, Register::R2, Register::R0, -15).unwrap(), 0x0150_20df);
        assert!(load_signed_byte(Condition::Al, Register::PC, r4, 2).is_err());
        assert!(load_signed_byte(Condition::Al, Register::R1, Register::PC, 2).is_err());
        assert!(load_signed_byte(Condition::Al, Register::R1, r4, 256).is_err());
    }
    #[test]
    fn immediate_and_transfer_encoding_and_rejections() {
        assert_eq!(data_immediate(Condition::Al, DataOp::And, Register::R1, Register::R1, 1).unwrap(), 0xe201_1001);
        assert_eq!(data_immediate(Condition::Al, DataOp::Cmp, Register::R0, Register::R1, 0).unwrap(), 0xe351_0000);
        assert_eq!(transfer_word(Condition::Al, true, Register::R0, Register::PC, -12).unwrap(), 0xe51f_000c);
        assert_eq!(transfer_word(Condition::Al, false, Register::R1, Register::R0, 0).unwrap(), 0xe580_1000);
        assert_eq!(msr_cpsr_control(Condition::Al, 0xd1).unwrap(), 0xe321_f0d1);
        assert_eq!(branch_exchange_with_link(Condition::Al, Register::R2).unwrap(), 0xe12f_ff32);
        assert_eq!(supervisor_call(Condition::Al, 0).unwrap(), 0xef00_0000);
        assert!(supervisor_call(Condition::Al, 0x0100_0000).is_err());
        assert_eq!(transfer_word_preindexed_writeback(Condition::Al, true, Register::R0, Register::new(4).unwrap(), -4).unwrap(), 0xe534_0004);
        assert_eq!(transfer_byte_postindexed(Condition::Mi, false, Register::R1, Register::R0, 1).unwrap(), 0x44c0_1001);
        assert_eq!(data_register(Condition::Al, DataOp::Eor, Register::R2, Register::R1, Register::R0, Shift::Lsl, 0, false).unwrap(), 0xe021_2000);
        assert_eq!(store_halfword_postindexed(Condition::Cs, Register::R1, Register::R0, 2).unwrap(), 0x20c0_10b2);
        assert_eq!(transfer_byte(Condition::Cs, false, Register::R1, Register::R0, 0).unwrap(), 0x25c0_1000);
        assert!(store_halfword_postindexed(Condition::Al, Register::R1, Register::R0, 256).is_err());
        assert!(transfer_word_preindexed_writeback(Condition::Al, true, Register::R1, Register::R1, 4).is_err());
        assert!(branch_exchange_with_link(Condition::Al, Register::PC).is_err());
        assert_eq!(mcr(Condition::Al, 15, 0, Register::R0, 2, 0, 0).unwrap(), 0xee02_0f10);
        assert_eq!(mov_shifted_register(Condition::Al, Register::new(4).unwrap(), Register::new(4).unwrap(), Shift::Lsr, 20).unwrap(), 0xe1a0_4a24);
        assert_eq!(data_immediate_with_flags(Condition::Al, DataOp::Sub, Register::R1, Register::R1, 1, true).unwrap(), 0xe2511001);
        assert_eq!(data_register(Condition::Al, DataOp::Cmp, Register::R0, Register::R1, Register::new(4).unwrap(), Shift::Lsl, 0, false).unwrap(), 0xe1510004);
        assert_eq!(transfer_word_register(Condition::Al, false, Register::new(3).unwrap(), Register::R0, Register::R1, Shift::Lsl, 2).unwrap(), 0xe7803101);
        assert_eq!(data_immediate(Condition::Al, DataOp::Rsb, Register::R0, Register::R0, 31).unwrap(), 0xe260001f);
        assert_eq!(data_register_shifted_by_register(Condition::Al, DataOp::Tst, Register::R0, Register::new(7).unwrap(), Register::new(5).unwrap(), Shift::Lsl, Register::R1, false).unwrap(), 0xe1170115);
        assert_eq!(data_register_shifted_by_register(Condition::Al, DataOp::Bic, Register::new(7).unwrap(), Register::new(7).unwrap(), Register::new(5).unwrap(), Shift::Lsl, Register::R0, false).unwrap(), 0xe1c77015);
        assert!(data_register_shifted_by_register(Condition::Al, DataOp::Mov, Register::R1, Register::R0, Register::R1, Shift::Lsl, Register::PC, false).is_err());
        assert_eq!(mrc(Condition::Al, 15, 0, Register::R0, 1, 0, 0).unwrap(), 0xee11_0f10);
        assert_eq!(bx(Condition::Al, Register::R12), 0xe12f_ff1c);
        assert_eq!(transfer_word_postindexed(Condition::Al, false, Register::R2, Register::R1, 4).unwrap(), 0xe481_2004);
        assert_eq!(transfer_word_postindexed(Condition::Al, true, Register::R2, Register::R1, -4).unwrap(), 0xe411_2004);
        assert_eq!(transfer_word_postindexed_register(Condition::Al, true, Register::R1, Register::R2, Register::R1).unwrap(), 0xe692_1001);
        assert!(transfer_word_postindexed_register(Condition::Al, true, Register::PC, Register::PC, Register::R0).is_err());
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
