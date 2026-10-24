// SPDX-License-Identifier: GPL-3.0-only
//! Venus ARM32 boot/service bridge: named record lookup and table traversal.
//! Entry point at 0x8490; each register operation is encoded by ARM32 ISA
//! constructors. Literal words are distinguished from executable instructions.
//! This fragment alone does not close the entire boot-to-service gap.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
use super::venus_runtime_services::Code;

pub const START: usize = 0x8490;
pub const END: usize = 0x883c;
pub const FILE_OFFSET: usize = START + 0x4000;
const INVALID_LOOKUP: u32 = 0x000e_359a;
const INVALID_RESULT: u32 = 0x000e_35bd;
const RECORD_ERROR: u32 = 0x000e_35ea;
const LOOKUP_POINTER: u32 = 0x0010_39ec;

/// lookup named record across runtime `[0x8490, 0x8618)`.
fn lookup_named_record(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8490)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.branch(C::Eq,0x8540)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,12)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8540)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(5)?,8)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8540)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0x8508)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 5)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5376)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,255)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,5,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::R2,R::new(3)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Ne,0x84ec)?;
    c.branch(C::Al,0x8510)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 5)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5376)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R2,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(3)?,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(7)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x8538)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,24)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc,0x851c)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x855c)?;
    c.literal(R::R0,0x8614)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.call(0x8bac)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0)?)?;
    c.branch(C::Eq,0x85f0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 1)?)?;
    c.branch(C::Lt,0x8540)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,20)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.call(0x3388)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(8)?,4)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R1))?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(9)?,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R0,R::new(9)?,Shift::Lsl,3)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.call(0x3388)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x85dc)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    c.call(0x84210)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x85fc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,1)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(10)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Lt,0x8598)?;
    c.branch(C::Al,0x8540)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.branch(C::Al,0x8554)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(9)?,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.branch(C::Al,0x8554)?;
    c.emit(INVALID_LOOKUP)?;
    c.check(0x8618)?;
    Ok(())
}

/// resolve record pointer across runtime `[0x8618, 0x8688)`.
fn resolve_record_pointer(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8618)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    c.call(0x8490)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8668)?;
    c.literal(R::R0,0x8684)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,4)?)?;
    c.call(0x8bac)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(4)?, R::R0, 0)?)?;
    c.branch(C::Al,0x8678)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(INVALID_RESULT)?;
    c.check(0x8688)?;
    Ok(())
}

/// lookup compatible record across runtime `[0x8688, 0x8700)`.
fn lookup_compatible_record(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8688)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0)?)?;
    c.call(0x8bdc)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0x86ec)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    c.call(0x8618)?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0)?)?;
    c.branch(C::Eq,0x86f8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x8bdc)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0x86f4)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x8618)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(6)?, R::R0, 0)?)?;
    c.branch(C::Al,0x86f8)?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x8700)?;
    Ok(())
}

/// lookup by scope across runtime `[0x8700, 0x8738)`.
fn lookup_by_scope(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8700)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    c.call(0x8bdc)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x8618)?;
    c.check(0x8738)?;
    Ok(())
}

/// find record in section across runtime `[0x8738, 0x883c)`.
fn find_record_in_section(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8738)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R1))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::SP))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(6)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,4278190080)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 335544320)?)?;
    c.branch(C::Ne,0x87ac)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,8388608)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 4278190080)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    c.literal(R::R1,0x8834)?;
    c.call(0x841e4)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x87b8)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R0,1,true)?)?;
    c.branch(C::Eq,0x87ac)?;
    c.literal(R::R2,0x8838)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R2,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsr,16,false)?)?;
    c.branch(C::Eq,0x8818)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc,0x87f8)?;
    c.branch(C::Al,0x87ac)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 255)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    c.branch(C::Al,0x87ac)?;
    c.emit(RECORD_ERROR)?;
    c.emit(LOOKUP_POINTER)?;
    c.check(0x883c)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    // Code::emit checks the absolute virtual position, so the transient prefix
    // allows it to reject both missing instructions and accidental overlap.
    let skip = START - 0x1000;
    let mut bytes = vec![0u8; skip];
    let mut c = Code { bytes: &mut bytes, at: START };
    lookup_named_record(&mut c)?;
    resolve_record_pointer(&mut c)?;
    lookup_compatible_record(&mut c)?;
    lookup_by_scope(&mut c)?;
    find_record_in_section(&mut c)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bridge_fragment_has_exact_layout() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        for (a, expected) in [
            (0x8614, INVALID_LOOKUP), (0x8684, INVALID_RESULT),
            (0x8834, RECORD_ERROR), (0x8838, LOOKUP_POINTER),
        ] {
            let i = a - START;
            assert_eq!(&bytes[i..i + 4], &expected.to_le_bytes());
        }
    }
}
