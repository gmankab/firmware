// SPDX-License-Identifier: GPL-3.0-only
//! Codec buffer segment sequencer and packed signed bit-plane transfer.
//! Implements bounded, two-pass descriptor writes to controller memory.
//! All 119 A32 instructions are emitted from named, checked operations.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

/// Split stream data into controller words, coordinate transfer state, and
/// restore the saved registers after validating packet dimensions.
fn sequence_codec_transfer(c:&mut Code)->Result<(),String>{
    c.check(0x28568)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,52)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Lsr,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,536870912)?)?;
    c.b(C::Cc,0x28604)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,132)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R0,R::R2,Shift::Lsl,15)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(7)?,R::R2,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,458752)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::R1,7340032,true)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Eor,R::new(3)?,R::R1,1)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5504)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2147483649)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,388)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5120)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Lsl,15)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(7)?,4128768)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2621440)?)?;
    c.b(C::Ne,0x285e8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R2,132)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.b(C::Eq,0x2870c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,204)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,5120)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(6)?,255)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    c.bl(0x3ede0)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,3)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,2)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(7)?,Shift::Lsl,2,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::new(6)?,255)?)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R0,16)?)?;
    c.b(C::Hi,0x28624)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,364)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,360)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,14336)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,14336)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,4096)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,255)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::new(5)?,Shift::Lsl,2,false)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Lsl,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R1,4096)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::new(10)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R12,R::new(9)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::new(7)?,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R12,R::new(3)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::LR,R::R1,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R1,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R1,R::new(8)?,Shift::Lsl,0,false)?)?;
    c.b(C::Ne,0x286a0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R1,255)?)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    c.b(C::Hi,0x28684)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,364)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3120)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x28744)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    sequence_codec_transfer(c)?;
    Ok(())
}
