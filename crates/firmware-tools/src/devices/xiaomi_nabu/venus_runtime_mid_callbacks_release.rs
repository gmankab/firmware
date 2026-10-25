// SPDX-License-Identifier: GPL-3.0-only
//! Venus control-command callback release. Two small mutually related
//! peripheral handlers and explicit MMIO/diagnostic address pools.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const CONTROLLER_CALLBACK_DATA:[u32;7]=[
  0x0010_40b4,0xe00e_00b4,0x0010_de9c,0xffff_fdfd,
  0x000e_832c,0x0002_98e0,0x0002_64d8,
];
const OUTPUT_CALLBACK_DATA:[u32;4]=[0x0002_98e0,0x000e_832c,0x0002_64d8,0x0010_40b4];
/// Release controller resource and host-side streaming event triggers.
fn release_codec_control_callbacks(c:&mut Code)->Result<(),String>{
    c.check(0x290a4)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.literal_cond(C::Al,R::new(6)?,0x291d4)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26af8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2080)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    c.bl(0x273a4)?;
    c.literal_cond(C::Al,R::R0,0x291d8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x29154)?;
    c.literal_cond(C::Al,R::new(7)?,0x291dc)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::new(7)?,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,124)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2176)?)?;
    c.literal_cond(C::Al,R::R2,0x291e0)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x29180)?;
    c.literal_cond(C::Al,R::R2,0x291e4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,246)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x29190)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,124)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2176)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.b(C::Al,0x291c8)?;
    c.literal_cond(C::Al,R::R1,0x291e8)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,76)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x291b8)?;
    c.literal_cond(C::Al,R::R2,0x291e4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x291c8)?;
    c.literal_cond(C::Al,R::R1,0x291ec)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,76)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,72)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x3192c)?;
    c.check(0x291d4)?;
    for word in CONTROLLER_CALLBACK_DATA { c.emit(word)?; }
    c.check(0x291f0)
}
/// Close codec output DMA frame and acknowledge readback of pending state.
fn close_codec_output_callback(c:&mut Code)->Result<(),String>{
    c.check(0x291f0)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    c.b(C::Eq,0x29270)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2176)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x2928c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x29244)?;
    c.literal_cond(C::Al,R::R2,0x29290)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,13)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3328)?)?;
    c.bl(0x17b0)?;
    c.literal_cond(C::Al,R::R1,0x29294)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x29270)?;
    c.literal_cond(C::Al,R::R2,0x29290)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,276)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3072)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26de0)?;
    c.literal_cond(C::Al,R::R0,0x29298)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.check(0x2928c)?;
    for word in OUTPUT_CALLBACK_DATA { c.emit(word)?; }
    c.check(0x2929c)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    release_codec_control_callbacks(c)?;
    close_codec_output_callback(c)?;
    Ok(())
}
