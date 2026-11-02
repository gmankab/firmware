// SPDX-License-Identifier: GPL-3.0-only
//! Venus interrupt handler slots, controller-mask initialization, and shared
//! MMIO register initialization. This source encodes typed ARM A32 instructions;
//! data pools below are runtime address/diagnostic constants, not opcodes.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;

const IRQ_SLOT_BASE:[u32;1]=[0x0010_40b4];
const IRQ_CLEAR_MESSAGES:[u32;6]=[
    0x0010_40b4,0x000e_8303,0x000e_82e8,
    0x000e_2788,0x000e_832c,0x000e_82b8];
const IRQ_MASK_MMIO:[u32;1]=[0xe00c_1008];
const REGISTERED_IRQ:[u32;1]=[0x0010_cf04];
const DRIVER_CONTROL_REGISTERS:[u32;6]=[
    0xe00d_2004,0xe00c_2020,0x0010_40d0,
    0x0010_40b8,0x0010_40bc,0x02bd_e780];

/// Install or update one controller callback when the selected slot is empty.
fn assign_controller_slot(c:&mut Code)->Result<(),String>{
    c.check(0x25b1c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal_cond(C::Al,R::LR,0x25b58)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(4)?,R::LR,R::new(3)?,Shift::Lsl,2)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(4)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(5)?,R::new(4)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.b(C::Ne,0x25b50)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,1)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::LR,R::new(3)?,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,3)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x25b58)?;
    for word in IRQ_SLOT_BASE { c.emit(word)?; }
    c.check(0x25b5c)
}

/// Remove the matching callback from an indexed controller slot. Diagnose
/// unexpected or unallocated records with the platform message writer.
fn release_controller_slot(c:&mut Code)->Result<(),String>{
    c.check(0x25b5c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal_cond(C::Al,R::R12,0x25c0c)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R12,R::R2,Shift::Lsl,2)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(3)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(3)?,R::new(4)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    c.b(C::Eq,0x25bc4)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    c.b(C::Eq,0x25bf0)?;
    c.literal_cond(C::Al,R::R1,0x25c10)?;
    c.literal_cond(C::Al,R::R2,0x25c14)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,21)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x25c18)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R2,0x25c1c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x25c04)?;
    c.literal_cond(C::Al,R::R1,0x25c20)?;
    c.literal_cond(C::Al,R::R2,0x25c14)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x25c18)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.bl(0x2ec94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.b(C::Al,0x25c04)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R12,R::R2,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,3)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x25c0c)?;
    for word in IRQ_CLEAR_MESSAGES { c.emit(word)?; }
    c.check(0x25c24)
}

/// Reset the hardware interrupt acknowledge/ready window and initialize
/// software counter storage; preserve the original ARM register layout.
fn control_interrupt_window(c:&mut Code)->Result<(),String>{
    c.check(0x25c24)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,790528)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3758096384)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.literal_cond(C::Al,R::R0,0x25c5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x25c5c)?;
    for word in IRQ_MASK_MMIO {c.emit(word)?;}
    c.check(0x25c60)
}

/// Install the callback link for interrupt source 22.
fn register_irq_callback(c:&mut Code)->Result<(),String>{
    c.check(0x25c60)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal_cond(C::Al,R::new(4)?,0x25c88)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0xd6c0)?;    c.check(0x25c88)?;
    for word in REGISTERED_IRQ {c.emit(word)?;}
    c.check(0x25c8c)
}

/// Program controller startup registers, clear pending per-channel masks and
/// prepare the shared platform completion flags.
fn startup_irq_state(c:&mut Code)->Result<(),String>{
    c.check(0x25c8c)?;
    c.literal_cond(C::Al,R::R0,0x25cfc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,130)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,268)?)?;
    c.literal_cond(C::Al,R::R1,0x25d00)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x25d04)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x25d08)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x25d0c)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,790528)?)?;
    c.literal_cond(C::Al,R::R2,0x25d10)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x25cfc)?;
    for word in DRIVER_CONTROL_REGISTERS {c.emit(word)?;}
    c.check(0x25d14)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    assign_controller_slot(c)?;
    release_controller_slot(c)?;
    control_interrupt_window(c)?;
    register_irq_callback(c)?;
    startup_irq_state(c)?;
    Ok(())
}
