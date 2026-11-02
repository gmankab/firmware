// SPDX-License-Identifier: GPL-3.0-only
//! Venus record attribute validation, format-type dispatch, and output
//! queue selection. The dispatch jump table and diagnostics are structured
//! runtime address/data values, not imported instruction encodings.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
use super::venus_runtime_services::Code;

pub const START: usize = 0x883c;
pub const END: usize = 0x8c00;
pub const FILE_OFFSET: usize = START + 0x4000;
// Runtime branches selected by eight attribute-format classes.
const TYPE_DISPATCH: [u32; 8] = [
    0x0000_89bc, 0x0000_8a74, 0x0000_8a68, 0x0000_8ac8,
    0x0000_8a68, 0x0000_8a68, 0x0000_8a68, 0x0000_8b18,
];
// Table version/selector field, followed by error/debug string locations.
const DIAGNOSTIC_WORDS: [u32; 6] = [
    0x0116_0004, 0x000e_36c2, 0x000e_367f,
    0x000e_36e4, 0x000e_3622, 0x000e_35fc,
];
const QUEUE_HEAD_POINTER: u32 = 0x0010_39e0;

/// validate record header: `[0x883c,0x899c)`.
fn validate_record_header(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x883c)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0x888c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::new(6)?, 0)?)?;
    c.branch(C::Ne,0x889c)?;
    c.literal(R::R0,0x8ba4)?;
    c.branch(C::Al,0x8a14)?;
    c.literal(R::R0,0x8ba8)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.call(0x8bac)?;
    c.branch(C::Al,0x8a24)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.call(0x83e0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,24)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    c.call(0x8738)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmn, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(9)?, R::R0, 4278190080)?)?;
    c.emit(arm32::mov_register(C::Eq, R::new(4)?, R::new(6)?))?;
    c.branch(C::Al,0x8924)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x897c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 24)?)?;
    c.branch(C::Hi,0x8a04)?;
    c.literal(R::R0,0x8b94)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,R::new(10)?,false)?)?;
    c.branch(C::Eq,0x8900)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    c.branch(C::Al,0x8924)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 8)?)?;
    c.branch(C::Ne,0x8a04)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,255)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,3,true)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,4)?)?;
    c.emit(arm32::data_register(C::Ne,Op::Add,R::new(4)?,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::new(10)?,R::R0,Shift::Lsr,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R0,8388608)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R1,R::R1,R::new(9)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    c.branch(C::Eq,0x88d4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(7)?,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.call(0x841e4)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne,0x88e0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::new(10)?,17)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 7)?)?;
    c.branch(C::Hi,0x8a34)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R1,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R2,R::R1,Shift::Lsl,0)?)?;
    c.check(0x899c)?;
    Ok(())
}

/// extract record attributes: `[0x89bc,0x8b94)`.
fn extract_record_attributes(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x89bc)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    c.call(0x3388)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    c.branch(C::Al,0x8a28)?;
    c.literal(R::R0,0x8b98)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    c.call(0x8bac)?;
    c.literal(R::R0,0x8b9c)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(6)?))?;
    c.call(0x8bac)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 2)?)?;
    c.branch(C::Eq,0x8b68)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 8)?)?;
    c.branch(C::Ne,0x8a68)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 17)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,5)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    c.branch(C::Al,0x8a28)?;
    c.literal(R::R0,0x8ba0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    c.branch(C::Al,0x8894)?;
    c.call(0x8c88)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    c.branch(C::Eq,0x8a28)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,3)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 18)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    c.branch(C::Al,0x8a28)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 18)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    c.branch(C::Al,0x8a28)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,32)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,32)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,8)?)?;
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 17)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    c.branch(C::Al,0x8a28)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,8)?)?;
    c.call(0x8c90)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 2)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R0.bit()|R::new(4)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.branch(C::Al,0x8a28)?;
    c.check(0x8b94)?;
    Ok(())
}

/// resolve queue head: `[0x8bac,0x8bd8)`.
fn resolve_queue_head(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8bac)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::bx(C::Eq,R::LR))?;
    c.literal(R::R1,0x8bd8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Ne,0x8bd4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8bd8)?;
    Ok(())
}

/// choose queue head: `[0x8bdc,0x8c00)`.
fn choose_queue_head(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8bdc)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::bx(C::Ne,R::LR))?;
    c.literal(R::R1,0x8c1c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8c00)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8c00)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let skip = START - 0x1000;
    let mut bytes = vec![0u8; skip];
    let mut c = Code { bytes: &mut bytes, at: START };
    validate_record_header(&mut c)?;
    c.check(0x899c)?;
    for data in TYPE_DISPATCH { c.emit(data)?; }
    c.check(0x89bc)?;
    extract_record_attributes(&mut c)?;
    c.check(0x8b94)?;
    for data in DIAGNOSTIC_WORDS { c.emit(data)?; }
    c.check(0x8bac)?;
    resolve_queue_head(&mut c)?;
    c.check(0x8bd8)?;
    c.emit(QUEUE_HEAD_POINTER)?;
    c.check(0x8bdc)?;
    choose_queue_head(&mut c)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attributes_and_queues_are_contiguous() {
        let image = fragment().unwrap();
        assert_eq!(image.len(), END - START);
        assert_eq!(&image[0x899c-START..0x89bc-START],
            &TYPE_DISPATCH.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>());
        assert_eq!(&image[0x8b94-START..0x8bac-START],
            &DIAGNOSTIC_WORDS.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>());
        assert_eq!(&image[0x8bd8-START..0x8bdc-START], &QUEUE_HEAD_POINTER.to_le_bytes());
    }
}
