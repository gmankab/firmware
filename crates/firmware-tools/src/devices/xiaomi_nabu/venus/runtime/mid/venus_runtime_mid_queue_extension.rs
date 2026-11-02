// SPDX-License-Identifier: GPL-3.0-only
//! Queue MMIO field writers, channel selection and indexed codec event
//! callbacks. All A32 instructions are encoded from checked register and
//! immediate operations; hardware addresses and callback pointers are data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const CODEC_QUEUE_WINDOW:[u32;1]=[0xe00e_00b4];
const CHANNEL_QUEUE_SELECTORS:[u32;2]=[0x0fff_8090,0xe00e_00bc];
const PENDING_IRQ_BITFIELD:[u32;1]=[0xe00e_00b8];
const CODEC_EVENT_REGISTRY:[u32;6]=[
    0x0010_40b4,0xe00e_00b4,0x0010_de9c,
    0x000e_832c,0x0002_9494,0x0002_64d8,
];
const CODEC_RESYNC_HANDLERS:[u32;4]=[
    0x0002_9494,0x000e_832c,0x0002_64d8,0x0010_40b4,
];

/// Store selected per-channel queued IRQ size under queue lock.
fn set_codec_queue_window(c:&mut Code)->Result<(),String>{
    c.check(0x27684)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2080)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    c.bl(0x273a4)?;
    c.literal_cond(C::Al,R::R0,0x276a8)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x276a8)?;
    for word in CODEC_QUEUE_WINDOW{c.emit(word)?;}
    c.check(0x276ac)
}

/// Select the alternate queue or buffer descriptor and publish its value.
fn set_codec_queue_selectors(c:&mut Code)->Result<(),String>{
    c.check(0x276ac)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    c.literal_cond(C::Al,R::R1,0x276e8)?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    c.bl(0x273a4)?;
    c.literal_cond(C::Al,R::R0,0x276ec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    c.emit(arm32::mov_register(C::Eq,R::R2,R::R0))?;
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x276e8)?;
    for word in CHANNEL_QUEUE_SELECTORS{c.emit(word)?;}
    c.check(0x276f0)
}

/// Configure the codec pending bitfield in its MMIO register.
fn set_codec_queue_bitfield(c:&mut Code)->Result<(),String>{
    c.check(0x276f0)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,16384)?)?;
    c.bl(0x273a4)?;
    c.literal_cond(C::Al,R::R0,0x27714)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x27714)?;
    for word in PENDING_IRQ_BITFIELD{c.emit(word)?;}
    c.check(0x27718)
}

/// Validate controller input channel and register control-message callbacks.
fn register_codec_event_and_channel(c:&mut Code)->Result<(),String>{
    c.check(0x27718)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.literal_cond(C::Al,R::new(7)?,0x27828)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26af8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2080)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    c.bl(0x273a4)?;
    c.literal_cond(C::Al,R::R0,0x2782c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(6)?,8192)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.b(C::Eq,0x27774)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,16)?)?;
    c.b(C::Ne,0x2781c)?;
    c.literal_cond(C::Al,R::new(6)?,0x27830)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(4)?,Shift::Lsl,2,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,124)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2048)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2176)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,272)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x277d4)?;
    c.literal_cond(C::Al,R::R2,0x27834)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,214)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x277e4)?;
    c.literal_cond(C::Al,R::R1,0x27838)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,76)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x2780c)?;
    c.literal_cond(C::Al,R::R2,0x27834)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,222)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x2781c)?;
    c.literal_cond(C::Al,R::R1,0x2783c)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,76)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,72)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x3192c)?;
    c.check(0x27828)?;
    for word in CODEC_EVENT_REGISTRY{c.emit(word)?;}
    c.check(0x27840)
}

/// Attempt bounded channel re-synchronization and restore active control state.
fn dispatch_codec_resync(c:&mut Code)->Result<(),String>{
    c.check(0x27840)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,8192)?)?;
    c.b(C::Eq,0x27860)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    c.b(C::Ne,0x278cc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2176)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,2348810240)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x278e8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x278a0)?;
    c.literal_cond(C::Al,R::R2,0x278ec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,238)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.literal_cond(C::Al,R::R1,0x278f0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0x25b5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x278cc)?;
    c.literal_cond(C::Al,R::R2,0x278ec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,245)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.bl(0x26de0)?;
    c.literal_cond(C::Al,R::R0,0x278f4)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(4)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.check(0x278e8)?;
    for word in CODEC_RESYNC_HANDLERS{c.emit(word)?;}
    c.check(0x278f8)
}

/// Single-instruction barrier before the next controller processing loop.
fn finish_codec_resync(c:&mut Code)->Result<(),String>{
    c.check(0x278f8)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x278fc)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    set_codec_queue_window(c)?;
    set_codec_queue_selectors(c)?;
    set_codec_queue_bitfield(c)?;
    register_codec_event_and_channel(c)?;
    dispatch_codec_resync(c)?;
    finish_codec_resync(c)?;
    Ok(())
}
