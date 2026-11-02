// SPDX-License-Identifier: GPL-3.0-only
//! Venus IRQ-mask control handshake, controller sleep/wake status, and final
//! resource acquisition acknowledgement. Encodes original A32 operands as
//! checked typed operations, with OEM platform references as data only.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const IRQ_HANDSHAKE_LITERALS:[u32;9]=[
    0x0010_40e0,0xe00e_0110,0x0010_40c0,0x000e_838e,
    0x000e_2788,0xe000_98dc,0x0010_409c,0x0010_40a0,
    0xe000_8040,
];
const IRQ_HANDSHAKE_READY:[u32;2]=[0x0010_40c0,0x0010_40e0];
const IRQ_STATUS_TRANSITION:[u32;7]=[
    0x0010_40e0,0xe00e_0090,0x0010_40c4,0x000e_832c,
    0x000e_83bf,0x000e_2788,0x0010_40b8,
];

/// Coordinate clock and IRQ release when no registered hardware work is
/// outstanding, including repeated MMIO acknowledgment and polling limits.
fn request_controller_handshake(c:&mut Code)->Result<(),String>{
    c.check(0x2693c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.literal_cond(C::Al,R::R0,0x26aa0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x26a98)?;
    c.literal_cond(C::Al,R::new(5)?,0x26aa4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    c.b(C::Eq,0x26958)?;
    c.literal_cond(C::Al,R::new(6)?,0x26aa8)?;
    c.literal_cond(C::Al,R::R1,0x26aac)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,0)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x26ab0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.bl(0x2ec94)?;
    c.bl(0x3070)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x269a4)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R0,R::R0,1,true)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    c.b(C::Eq,0x269ac)?;
    c.bl(0x3074)?;
    c.b(C::Al,0x26a98)?;
    c.literal_cond(C::Al,R::R1,0x26ab4)?;
    c.literal_cond(C::Al,R::R0,0x26ab8)?;
    c.literal_cond(C::Al,R::new(3)?,0x26abc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,true)?)?;
    c.literal_cond(C::Al,R::R0,0x26ac0)?;
    c.b(C::Ne,0x269f4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,97)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,6080)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1073741824)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,-216)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(3)?,196608)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,196608)?)?;
    c.b(C::Ne,0x269e4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,-272)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,50)?)?;
    c.b(C::Ne,0x26a04)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    c.b(C::Ne,0x26a28)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    c.b(C::Ne,0x26a28)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,4)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(6)?,R::new(6)?,3,true)?)?;
    c.b(C::Eq,0x26a44)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,4)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(4)?,R::new(4)?,768,true)?)?;
    c.b(C::Eq,0x26a50)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    c.b(C::Ne,0x26a28)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    c.b(C::Ne,0x26a6c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,6)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    c.b(C::Ne,0x26a80)?;
    c.bl(0x3074)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x26aa0)?;
    for word in IRQ_HANDSHAKE_LITERALS{c.emit(word)?;}
    c.check(0x26ac4)
}

/// Read two IRQ-control flags and return whether both match the ready state.
fn handshake_ready_state(c:&mut Code)->Result<(),String>{
    c.check(0x26ac4)?;
    c.literal_cond(C::Al,R::R0,0x26af0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x26af4)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x26af0)?;
    for word in IRQ_HANDSHAKE_READY{c.emit(word)?;}
    c.check(0x26af8)
}

/// Request an explicit transition of firmware IRQ availability, including
/// queue drain/reset and status-reporting callbacks.
fn request_irq_state_transition(c:&mut Code)->Result<(),String>{
    c.check(0x26af8)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    c.literal_cond(C::Al,R::R0,0x26c44)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x26b54)?;
    c.literal_cond(C::Al,R::new(6)?,0x26c48)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    c.b(C::Eq,0x26b88)?;
    c.literal_cond(C::Al,R::R2,0x26c50)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,312)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    c.b(C::Al,0x26b9c)?;
    c.bl(0x3070)?;
    c.literal_cond(C::Al,R::R0,0x26c5c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(8)?,Shift::Lsl,0,true)?)?;
    c.b(C::Eq,0x26c24)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,1)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Eq,Op::And,R::R2,R::R1,2,true)?)?;
    c.b(C::Ne,0x26c34)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.b(C::Al,0x26c30)?;
    c.literal_cond(C::Al,R::new(10)?,0x26c4c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(6)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.b(C::Eq,0x26b9c)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R1,Shift::Lsr,2)?)?;
    c.literal_cond(C::Al,R::R1,0x26c54)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x26c58)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.bl(0x2ec94)?;
    c.bl(0x3070)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x26c10)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R1,R::new(7)?,Shift::Lsl,0,false)?)?;
    c.b(C::Eq,0x26bec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x26bec)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    c.bl(0x26c60)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
    c.bl(0x3074)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.b(C::Al,0x26c3c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    c.bl(0x26c60)?;
    c.bl(0x3074)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x26c44)?;
    for word in IRQ_STATUS_TRANSITION{c.emit(word)?;}
    c.check(0x26c60)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    request_controller_handshake(c)?;
    handshake_ready_state(c)?;
    request_irq_state_transition(c)?;
    Ok(())
}
