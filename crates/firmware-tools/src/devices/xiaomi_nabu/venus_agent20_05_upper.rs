// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-05: independent function-aligned Venus ARM32 upper-runtime sources.
//!
//! Exclusive reservation: runtime [0x68000, 0xa8000), ELF file offsets
//! [0x6c000, 0xac000). The reservation begins inside a prior function;
//! the first *complete* function available here starts at 0x69b80.
//!
//! These are independently typed ARM ISA encodings, not embedded OEM bytes.
//! External firmware bytes must be provided to verify; no originals are
//! redistributed or silently used to populate missing executable ranges.
use arm32::{
    branch, data_immediate, data_register, mov_register, mov_shifted_register,
    push_registers, transfer_byte, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const OWNED_START: usize = 0x68000;
pub const OWNED_END: usize = 0xa8000;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const VERIFIED_START: usize = 0x69b80;
pub const VERIFIED_END: usize = 0x69e44;
pub const INITIAL_UNCOVERED: usize = VERIFIED_START - OWNED_START;

struct A32 {
    pc: usize,
    bytes: Vec<u8>,
}
impl A32 {
    fn new(start: usize) -> Result<Self, String> {
        if start < OWNED_START || start >= OWNED_END || start & 3 != 0 {
            return Err(format!("outside agent 20-05 ARM range: 0x{start:x}"));
        }
        Ok(Self { pc: start, bytes: Vec::new() })
    }
    fn check(&self, wanted: usize) -> Result<(), String> {
        if self.pc != wanted {
            return Err(format!("ARM32 layout 0x{:x} != expected 0x{wanted:x}", self.pc));
        }
        Ok(())
    }
    fn emit(&mut self, encoded: u32) -> Result<(), String> {
        if self.pc < OWNED_START || self.pc + 4 > OWNED_END || self.pc & 3 != 0 {
            return Err(format!("agent 20-05 out-of-bounds emission 0x{:x}", self.pc));
        }
        self.bytes.extend_from_slice(&encoded.to_le_bytes());
        self.pc += 4;
        Ok(())
    }
    fn b(&mut self, cond: C, target: usize) -> Result<(), String> {
        self.emit(branch(cond, self.pc as u32, target as u32, false)?)
    }
    fn bl(&mut self, target: usize) -> Result<(), String> {
        self.emit(branch(C::Al, self.pc as u32, target as u32, true)?)
    }
    fn lit(&mut self, dst: R, location: usize) -> Result<(), String> {
        let displacement = i32::try_from(location as i64 - self.pc as i64 - 8)
            .map_err(|_| "literal address displacement overflow")?;
        self.emit(transfer_word(C::Al, true, dst, R::PC, displacement)?)
    }
}
fn r(n: u8) -> Result<R, String> { R::new(n) }
fn imm(c: &mut A32, cond: C, op: Op, dst: R, src: R, value: u32) -> Result<(), String> {
    c.emit(data_immediate(cond, op, dst, src, value)?)
}
fn reg(c: &mut A32, cond: C, op: Op, dst: R, lhs: R, rhs: R,
       shift: Shift, count: u8) -> Result<(), String> {
    c.emit(data_register(cond, op, dst, lhs, rhs, shift, count, false)?)
}
fn load(c: &mut A32, cond: C, dst: R, base: R, displacement: i32) -> Result<(), String> {
    c.emit(transfer_word(cond, true, dst, base, displacement)?)
}
fn store(c: &mut A32, cond: C, src: R, base: R, displacement: i32) -> Result<(), String> {
    c.emit(transfer_word(cond, false, src, base, displacement)?)
}
fn mov(c: &mut A32, cond: C, dst: R, src: R) -> Result<(), String> {
    c.emit(mov_register(cond, dst, src))
}
fn movshift(c: &mut A32, cond: C, dst: R, src: R, shift: Shift, count: u8) -> Result<(), String> {
    c.emit(mov_shifted_register(cond, dst, src, shift, count)?)
}
/// Public ARM A32 UMULL: low/high halves of unsigned 32x32 multiplication.
/// Encoded from the ISA format, with reserved registers and aliases rejected.
fn umull(c: &mut A32, low: R, high: R, lhs: R, rhs: R) -> Result<(), String> {
    if low == high || [low, high, lhs, rhs].iter().any(|v| v.number() > 12) {
        return Err("invalid UMULL registers".into());
    }
    c.emit(0xe080_0090 | (high.number() << 16) | (low.number() << 12)
           | (rhs.number() << 8) | lhs.number())
}

/// Populate the descriptor's quarter-size counts and 16.16 fixed-point
/// scaled buffer dimensions. When the hardware layout flag is clear, the
/// scale factor is read from controller state; otherwise it is unity.
/// The field and memory-offset interpretation needs device-level validation.
fn emit_scaled_descriptor_init(c: &mut A32) -> Result<(), String> {
    let r3=r(3)?; let r4=r(4)?; let r5=r(5)?; let r6=r(6)?; let r7=r(7)?;
    let frame=r4.bit()|r5.bit()|r6.bit()|r7.bit()|R::R11.bit()|R::LR.bit();
    c.check(VERIFIED_START)?;
    c.emit(push_registers(C::Al, frame)?)?;                        // 0x69b80
    imm(c,C::Al,Op::Add,R::R11,R::SP,16)?;
    mov(c,C::Al,r7,R::R2)?;
    mov(c,C::Al,r4,R::R0)?;
    reg(c,C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,1)?;
    movshift(c,C::Al,R::R2,R::R0,Shift::Lsr,2)?;
    movshift(c,C::Al,R::R0,R::R1,Shift::Lsr,2)?;
    imm(c,C::Al,Op::Cmp,R::R0,r7,0)?;
    store(c,C::Al,R::R2,r4,0)?;
    c.emit(transfer_multiple(C::Al,false,r4,R::R0.bit()|R::R1.bit(),
                             BlockMode::IncrementBefore,false)?)?;
    c.b(C::Eq,0x69bb4)?;
    imm(c,C::Al,Op::Mov,r3,R::R0,0x10000)?;
    c.b(C::Al,0x69bc8)?;
    c.check(0x69bb4)?;
    load(c,C::Al,r3,r3,0x838)?;
    c.emit(transfer_byte(C::Al,true,R::R12,r3,0xe0)?)?;
    c.lit(r3,0x69c38)?;
    imm(c,C::Al,Op::Cmp,R::R0,R::R12,0)?;
    imm(c,C::Ne,Op::Mov,r3,R::R0,0x10000)?;
    c.check(0x69bc8)?;
    umull(c,r5,r6,R::R2,r3)?;
    store(c,C::Al,r3,r4,0x20)?;
    movshift(c,C::Al,R::R2,r5,Shift::Lsr,16)?;
    reg(c,C::Al,Op::Orr,R::R2,R::R2,r6,Shift::Lsl,16)?;
    store(c,C::Al,R::R2,r4,0x10)?;
    umull(c,R::R2,r6,r3,R::R0)?;
    movshift(c,C::Al,R::R0,R::R2,Shift::Lsr,16)?;
    reg(c,C::Al,Op::Orr,R::R0,R::R0,r6,Shift::Lsl,16)?;
    store(c,C::Al,R::R0,r4,0x0c)?;
    umull(c,R::R0,R::R2,r3,R::R1)?;
    imm(c,C::Al,Op::Mov,R::R1,R::R0,0)?;
    movshift(c,C::Al,R::R0,R::R0,Shift::Lsr,16)?;
    reg(c,C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,16)?;
    imm(c,C::Al,Op::Mov,R::R2,R::R0,21)?;
    store(c,C::Al,R::R0,r4,0x14)?;
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    for off in [0x18,0x1c,0xd4,0xd8] { store(c,C::Al,R::R0,r4,off)?; }
    imm(c,C::Al,Op::Add,R::R0,r4,0xbc)?;
    c.bl(0x31ec)?;                // preexisting memset-style runtime helper
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0x10000)?;
    imm(c,C::Al,Op::Cmp,R::R0,r7,0)?;
    imm(c,C::Eq,Op::Mov,R::R0,R::R0,0x20000)?;
    store(c,C::Al,R::R0,r4,0xdc)?;
    store(c,C::Al,R::R0,r4,0xe0)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,
        (frame&!R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x69c38)?;
    const SCALING_FACTOR_Q16: u32 = 0x0001_3333;
    c.emit(SCALING_FACTOR_Q16)?;
    c.check(0x69c3c)
}

/// A32 immediate post-indexed `LDRH Rt, [Rn], #offset`. The architecture's
/// post-index form has P=0, U derived from the offset, and L=1. Checked
/// separately because the older shared arm32 crate only exposes STRH here.
fn load_halfword_postindexed(c:&mut A32, dst:R, base:R, signed_offset:i32)->Result<(),String>{
    if dst == R::PC || base == R::PC || dst == base || signed_offset.unsigned_abs()>255 {
        return Err("invalid postindexed A32 halfword load operands".into());
    }
    let n=signed_offset.unsigned_abs();
    c.emit(0xe050_00b0 | if signed_offset>=0 {1<<23} else {0}
      | (base.number()<<16) | (dst.number()<<12)
      | ((n & 0xf0)<<4) | (n & 0x0f))
}

/// Compare two converted six-component color-coordinate records, each
/// obtained via the existing `0x71e14` conversion service. This preserves
/// the maximum signed component difference and logs whether it exceeds 160;
/// the caller's two flag byte-pointers are then assigned from the input IDs.
fn emit_compare_converted_components(c:&mut A32)->Result<(),String>{
    let r3=r(3)?; let r4=r(4)?; let r5=r(5)?; let r6=r(6)?;
    let r7=r(7)?; let r8=r(8)?; let r9=r(9)?;
    let saved=(4..=9).map(|n|1u16<<n).sum::<u16>()|R::R11.bit()|R::LR.bit();
    c.check(0x69c3c)?;
    c.emit(push_registers(C::Al,saved)?)?;
    imm(c,C::Al,Op::Add,R::R11,R::SP,24)?;
    imm(c,C::Al,Op::Sub,R::SP,R::SP,24)?;
    mov(c,C::Al,r6,R::R0)?;
    imm(c,C::Al,Op::Add,r7,R::SP,12)?;
    mov(c,C::Al,r9,R::R1)?;
    mov(c,C::Al,r8,r3)?;
    mov(c,C::Al,r5,R::R2)?;
    mov(c,C::Al,R::R0,r7)?;
    mov(c,C::Al,R::R1,r6)?;
    c.bl(0x71e14)?;
    mov(c,C::Al,r4,R::SP)?;
    mov(c,C::Al,R::R1,r9)?;
    mov(c,C::Al,R::R2,r5)?;
    mov(c,C::Al,R::R0,r4)?;
    c.bl(0x71e14)?;
    load(c,C::Al,r5,R::R11,8)?;
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    imm(c,C::Al,Op::Mvn,R::R1,R::R0,0)?;
    c.check(0x69c88)?;
    load_halfword_postindexed(c,R::R2,r7,2)?;
    load_halfword_postindexed(c,r3,r4,2)?;
    imm(c,C::Al,Op::Add,R::R0,R::R0,1)?;
    reg(c,C::Al,Op::Sub,R::R2,r3,R::R2,Shift::Lsl,0)?;
    reg(c,C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0)?;
    mov(c,C::Gt,R::R1,R::R2)?;
    imm(c,C::Al,Op::Cmp,R::R0,R::R0,6)?;
    c.b(C::Ne,0x69c88)?;
    c.lit(R::R2,0x69cd4)?;
    imm(c,C::Al,Op::Cmp,R::R0,R::R1,160)?;
    imm(c,C::Al,Op::Mov,R::R1,R::R0,78)?;
    imm(c,C::Al,Op::Mov,R::R0,R::R0,0)?;
    imm(c,C::Lt,Op::Mov,R::R0,R::R0,1)?;
    imm(c,C::Al,Op::Orr,R::R1,R::R1,512)?;
    c.bl(0x17b0)?;
    c.emit(transfer_byte(C::Al,false,r6,r8,0)?)?;
    c.emit(transfer_byte(C::Al,false,r9,r5,0)?)?;
    imm(c,C::Al,Op::Sub,R::SP,R::R11,24)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,
      (saved & !R::LR.bit()) | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x69cd4)?;
    const BAD_COLOR_DISTANCE: u32=0x000f_2e4c;
    c.emit(BAD_COLOR_DISTANCE)?;
    c.check(0x69cd8)
}

/// A32 register-offset LDRH with base writeback (`[Rn, Rm]!`).
/// Base must be writable and distinct from the loaded register.
fn load_halfword_register_preindexed_writeback(
    c:&mut A32, dst:R, base:R, offset:R
) -> Result<(),String> {
    if [dst,base,offset].contains(&R::PC) || dst == base {
        return Err("invalid register-offset halfword load with writeback".into());
    }
    c.emit(arm32::transfer_halfword_register(C::Al,true,dst,base,offset)? | (1<<21))
}
fn halfword(c:&mut A32, dst:R, base:R, offset:i32)->Result<(),String>{
    c.emit(arm32::load_halfword(C::Al,dst,base,offset)?)
}
fn halfword_reg(c:&mut A32, dst:R, base:R, offset:R)->Result<(),String>{
    c.emit(arm32::transfer_halfword_register(C::Al,true,dst,base,offset)?)
}
fn store_halfword(c:&mut A32, src:R, base:R, offset:i32)->Result<(),String>{
    c.emit(arm32::store_halfword(C::Al,src,base,offset)?)
}
/// Read a table's six per-index halfword components and five additional
/// halfwords at a second channel index, merging them into the fixed record
/// layout (`dst` offsets 0..10, 24..34). The high two indices select a
/// device table or one of three codec tables, with literal tables separated
/// from instruction bytes.
fn emit_copy_indexed_halfword_planes(c:&mut A32)->Result<(),String>{
    let r3=r(3)?;let r4=r(4)?;let r5=r(5)?;let r6=r(6)?;let r7=r(7)?;
    let mask=r4.bit()|r5.bit()|r6.bit()|r7.bit()|R::R11.bit()|R::LR.bit();
    c.check(0x69cd8)?;
    c.emit(push_registers(C::Al,mask)?)?;
    imm(c,C::Al,Op::Add,R::R11,R::SP,16)?;
    imm(c,C::Al,Op::Sub,R::R12,r3,1)?;
    imm(c,C::Al,Op::And,r3,R::R12,255)?;
    imm(c,C::Al,Op::Cmp,R::R0,r3,3)?;
    c.b(C::Cs,0x69d8c)?;
    c.lit(r6,0x69e40)?;
    movshift(c,C::Al,r3,R::R12,Shift::Lsl,24)?;
    movshift(c,C::Al,r3,r3,Shift::Asr,24)?;
    c.emit(arm32::transfer_word_register(C::Al,true,r4,r6,r3,Shift::Lsl,2)?)?;
    movshift(c,C::Al,r3,R::R1,Shift::Lsl,1)?;
    mov(c,C::Al,r6,r4)?;
    imm(c,C::Al,Op::Add,R::R12,r4,256)?;
    imm(c,C::Al,Op::Add,R::LR,r4,768)?;
    load_halfword_register_preindexed_writeback(c,r3,r6,r3)?;
    reg(c,C::Al,Op::Add,r5,R::R12,R::R1,Shift::Lsl,1)?;
    store_halfword(c,r3,R::R0,0)?;
    halfword(c,r5,r5,0)?;
    store_halfword(c,r5,R::R0,2)?;
    imm(c,C::Al,Op::Add,r5,r4,512)?;
    reg(c,C::Al,Op::Add,r3,r5,R::R1,Shift::Lsl,1)?;
    halfword(c,r3,r3,0)?;
    store_halfword(c,r3,R::R0,4)?;
    reg(c,C::Al,Op::Add,r3,R::LR,R::R1,Shift::Lsl,1)?;
    halfword(c,r3,r3,0)?;
    store_halfword(c,r3,R::R0,6)?;
    imm(c,C::Al,Op::Add,r3,r4,1024)?;
    reg(c,C::Al,Op::Add,R::R1,r3,R::R1,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,8)?;
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1280)?;
    halfword_reg(c,R::R1,r6,R::R1)?;
    store_halfword(c,R::R1,R::R0,10)?;
    reg(c,C::Al,Op::Add,R::R1,r4,R::R2,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,24)?;
    reg(c,C::Al,Op::Add,R::R1,R::R12,R::R2,Shift::Lsl,1)?;
    reg(c,C::Al,Op::Add,R::R12,r3,R::R2,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,26)?;
    reg(c,C::Al,Op::Add,R::R1,r5,R::R2,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,28)?;
    reg(c,C::Al,Op::Add,R::R1,R::LR,R::R2,Shift::Lsl,1)?;
    c.b(C::Al,0x69e18)?;
    c.check(0x69d8c)?;
    // Alternate static memory table for extended controller class numbers.
    c.lit(r4,0x69e3c)?;
    movshift(c,C::Al,R::R12,R::R1,Shift::Lsl,1)?;
    mov(c,C::Al,R::LR,r4)?;
    load_halfword_register_preindexed_writeback(c,r3,R::LR,R::R12)?;
    imm(c,C::Al,Op::Add,R::R12,r4,256)?;
    store_halfword(c,r3,R::R0,0)?;
    reg(c,C::Al,Op::Add,r3,R::R12,R::R1,Shift::Lsl,1)?;
    halfword(c,r3,r3,0)?;
    store_halfword(c,r3,R::R0,2)?;
    imm(c,C::Al,Op::Add,r3,r4,512)?;
    reg(c,C::Al,Op::Add,r5,r3,R::R1,Shift::Lsl,1)?;
    halfword(c,r5,r5,0)?;
    store_halfword(c,r5,R::R0,4)?;
    imm(c,C::Al,Op::Add,r5,r4,768)?;
    reg(c,C::Al,Op::Add,r6,r5,R::R1,Shift::Lsl,1)?;
    halfword(c,r6,r6,0)?;
    store_halfword(c,r6,R::R0,6)?;
    imm(c,C::Al,Op::Add,r6,r4,1024)?;
    reg(c,C::Al,Op::Add,R::R1,r6,R::R1,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,8)?;
    imm(c,C::Al,Op::Mov,R::R1,R::R0,1280)?;
    halfword_reg(c,R::R1,R::LR,R::R1)?;
    store_halfword(c,R::R1,R::R0,10)?;
    reg(c,C::Al,Op::Add,R::R1,r4,R::R2,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,24)?;
    reg(c,C::Al,Op::Add,R::R1,R::R12,R::R2,Shift::Lsl,1)?;
    reg(c,C::Al,Op::Add,R::R12,r6,R::R2,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,26)?;
    reg(c,C::Al,Op::Add,R::R1,r3,R::R2,Shift::Lsl,1)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,28)?;
    reg(c,C::Al,Op::Add,R::R1,r5,R::R2,Shift::Lsl,1)?;
    c.check(0x69e18)?;
    halfword(c,R::R1,R::R1,0)?;
    store_halfword(c,R::R1,R::R0,30)?;
    halfword(c,R::R1,R::R12,0)?;
    store_halfword(c,R::R1,R::R0,32)?;
    reg(c,C::Al,Op::Add,R::R1,r4,R::R2,Shift::Lsl,1)?;
    imm(c,C::Al,Op::Mov,R::R2,R::R0,1280)?;
    halfword_reg(c,R::R1,R::R1,R::R2)?;
    store_halfword(c,R::R1,R::R0,34)?;
    c.emit(transfer_multiple(C::Al,true,R::SP,
        (mask&!R::LR.bit())|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x69e3c)?;
    const EXTENDED_TABLE:u32=0x0010_5f60;
    const STATIC_CLASS_MAP:u32=0x000f_2e40;
    c.emit(EXTENDED_TABLE)?;
    c.emit(STATIC_CLASS_MAP)?;
    c.check(0x69e44)
}

pub fn fragment() -> Result<Vec<u8>,String> {
    let mut c=A32::new(VERIFIED_START)?;
    emit_scaled_descriptor_init(&mut c)?;
    emit_compare_converted_components(&mut c)?;
    emit_copy_indexed_halfword_planes(&mut c)?;
    Ok(c.bytes)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]fn aligned_and_address_bounded(){
        let output=fragment().unwrap();
        assert_eq!(output.len(), VERIFIED_END-VERIFIED_START);
        assert_eq!(VERIFIED_START,0x69b80);
        assert_eq!(INITIAL_UNCOVERED,0x1b80);
        assert_eq!(&output[0x69c38-VERIFIED_START..0x69c3c-VERIFIED_START], &0x0001_3333u32.to_le_bytes());
        assert_eq!(&output[0x69cd4-VERIFIED_START..0x69cd8-VERIFIED_START], &0x000f_2e4cu32.to_le_bytes());
        assert_eq!(&output[output.len()-8..output.len()-4], &0x0010_5f60u32.to_le_bytes());
        assert_eq!(&output[output.len()-4..], &0x000f_2e40u32.to_le_bytes());
    }
    #[test]fn unsigned_multiply_long_uses_valid_regs(){
        let mut c=A32::new(VERIFIED_START).unwrap();
        assert!(umull(&mut c,R::R0,R::R0,R::R1,R::R2).is_err());
        assert!(umull(&mut c,R::R0,R::R1,R::PC,R::R2).is_err());
    }
}
