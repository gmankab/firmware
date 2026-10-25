// SPDX-License-Identifier: GPL-3.0-only
//! Venus queue IRQ barriers, soft queue teardown, and codec-aware resource
//! release callbacks. Executable code is reconstructed as typed ARM A32;
//! all MMIO sources and diagnostic addresses are separately classified data.
use arm32::{Condition as C,DataOp as Op,Register as R,BlockMode};
use super::Code;
const IRQ_QUEUE_PENDING:[u32;1]=[0xe00e_0098];
const IRQ_QUEUE_CLEAR:[u32;1]=[0xe00e_0110];
const QUEUE_READY_REGISTERS:[u32;4]=[0x0010_40c0,0x0010_40e0,0xe000_98dc,0x3f81_c793];
const QUEUE_SHUTDOWN_CALLBACKS:[u32;5]=[
    0x0010_40c0,0x0010_40e0,0x0010_409c,0x0010_40a0,0x0010_40a4,
];
const STREAM_SHUTDOWN_REGISTERS:[u32;3]=[0x0010_40c4,0x0010_40e0,0x0010_40b4];

/// Synchronously poll posted IRQ writes until the host controller acknowledges.
fn wait_on_queued_irq(c:&mut Code)->Result<(),String>{
    c.check(0x273a4)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    c.bl(0x3070)?;
    c.literal_cond(C::Al,R::R1,0x27418)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1808)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(4)?,R::R0,-8)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    c.b(C::Eq,0x273ec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R2,1)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(3)?,R::new(3)?,4,true)?)?;
    c.b(C::Eq,0x273d4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1808)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::And,R::R2,R::R2,4,true)?)?;
    c.b(C::Ne,0x273fc)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x3074)?;    c.check(0x27418)?;
    for word in IRQ_QUEUE_PENDING{c.emit(word)?;}
    c.check(0x2741c)
}

/// Reset queue-pending status then wait on its readback before releasing resource.
fn drain_irq_queue(c:&mut Code)->Result<(),String>{
    c.check(0x2741c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.bl(0x3070)?;
    c.literal_cond(C::Al,R::R0,0x274a8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-272)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    c.b(C::Ne,0x27438)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1808)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x27468)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R2,4,true)?)?;
    c.b(C::Eq,0x27450)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1808)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::And,R::R2,R::R2,4,true)?)?;
    c.b(C::Ne,0x27478)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-272)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    c.b(C::Ne,0x27490)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x3074)?;    c.check(0x274a8)?;
    for word in IRQ_QUEUE_CLEAR{c.emit(word)?;}
    c.check(0x274ac)
}

/// Switch the controller to a ready or sleep state after checking status flags.
fn request_queue_ready(c:&mut Code)->Result<(),String>{
    c.check(0x274ac)?;
    c.literal_cond(C::Al,R::R1,0x2750c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x274cc)?;
    c.literal_cond(C::Al,R::R1,0x27508)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::bx(C::Eq,R::LR))?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x274e8)?;
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    c.bl(0x2693c)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x26764)?;
    c.literal_cond(C::Al,R::R0,0x27510)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x27514)?;
    c.b(C::Al,0x2741c)?;    c.check(0x27508)?;
    for word in QUEUE_READY_REGISTERS{c.emit(word)?;}
    c.check(0x27518)
}

/// Disable registered codec callbacks and clear the mapped input/output resources.
fn shutdown_controller_queues(c:&mut Code)->Result<(),String>{
    c.check(0x27518)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal_cond(C::Al,R::R0,0x275e4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x27540)?;
    c.literal_cond(C::Al,R::R0,0x275e0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.literal_cond(C::Al,R::new(4)?,0x275e8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    c.b(C::Ne,0x27564)?;
    c.literal_cond(C::Al,R::new(5)?,0x275f0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    c.b(C::Al,0x27584)?;
    c.literal_cond(C::Al,R::R0,0x275ec)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    c.b(C::Ne,0x27588)?;
    c.literal_cond(C::Al,R::new(5)?,0x275f0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,4)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.bl(0x274ac)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    c.b(C::Ne,0x275b4)?;
    c.literal_cond(C::Al,R::new(4)?,0x275f0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.bl(0x2c34)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.literal_cond(C::Al,R::R0,0x275ec)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    c.b(C::Ne,0x275dc)?;
    c.literal_cond(C::Al,R::new(4)?,0x275f0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.bl(0x2c34)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x275e0)?;
    for word in QUEUE_SHUTDOWN_CALLBACKS{c.emit(word)?;}
    c.check(0x275f4)
}

/// Close mapped codec channels and restore per-resource counter state.
fn close_controller_channel(c:&mut Code)->Result<(),String>{
    c.check(0x275f4)?;
    c.literal_cond(C::Al,R::R0,0x2767c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x27614)?;
    c.literal_cond(C::Al,R::R0,0x27678)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Eq,R::LR))?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal_cond(C::Al,R::new(4)?,0x27680)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.bl(0x2bf4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    c.bl(0x272e4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.bl(0x2c34)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2c34)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2c34)?;    c.check(0x27678)?;
    for word in STREAM_SHUTDOWN_REGISTERS{c.emit(word)?;}
    c.check(0x27684)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    wait_on_queued_irq(c)?;
    drain_irq_queue(c)?;
    request_queue_ready(c)?;
    shutdown_controller_queues(c)?;
    close_controller_channel(c)?;
    Ok(())
}
