// SPDX-License-Identifier: GPL-3.0-only
//! Venus status polling, descriptor-range validation and IRQ wait-state
//! activation. The routines combine checked A32 instructions with the
//! original firmware's mapped register/data pointers (separately classified).
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const STATUS_CHECK_REGISTERS:[u32;2]=[0x0010_40e0,0x0010_40c4];
const DESCRIPTOR_RANGE_DIAGNOSTICS:[u32;9]=[
    0xe00e_102c,0x0010_ceac,0x000e_85ab,0x000e_2788,
    0x000e_85f3,0x000e_8602,0x000e_864a,0x000e_85e8,
    0x000e_865c,
];
const IRQ_MODE_STATUS:[u32;4]=[
    0x0010_40e0,0x0010_40c4,0xe000_3d5c,0xe000_4510,
];

/// Return whether two controller status words permit the selected mode.
fn controller_status_is_ready(c:&mut Code)->Result<(),String>{
    c.check(0x27180)?;
    c.literal_cond(C::Al,R::R1,0x271b4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Eq,R::LR))?;
    c.literal_cond(C::Al,R::R1,0x271b8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x271b4)?;
    for word in STATUS_CHECK_REGISTERS {c.emit(word)?;}
    c.check(0x271bc)
}

/// Validate a stream-buffer/descriptor range and send explicit diagnostics
/// for invalid limits, out-of-range lengths or conflicting controller state.
fn validate_stream_descriptor_range(c:&mut Code)->Result<(),String>{
    c.check(0x271bc)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    c.literal_cond(C::Al,R::R0,0x272c0)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    c.literal_cond(C::Al,R::R1,0x272c8)?;
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x272c4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,136)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit() | R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x272cc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.bl(0x2ec94)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.b(C::Ls,0x27224)?;
    c.literal_cond(C::Al,R::R1,0x272e0)?;
    c.literal_cond(C::Al,R::R2,0x272d8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,10)?)?;
    c.b(C::Al,0x27278)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.b(C::Ls,0x2725c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(5)?,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.b(C::Ls,0x2726c)?;
    c.literal_cond(C::Al,R::R1,0x272dc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x272cc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.bl(0x2ec94)?;
    c.b(C::Al,0x27294)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.b(C::Cs,0x272a0)?;
    c.literal_cond(C::Al,R::R1,0x272d4)?;
    c.literal_cond(C::Al,R::R2,0x272d8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1792)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x272cc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    c.bl(0x2ec94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(4)?,R::R0,2147483648)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.literal_cond(C::Al,R::R1,0x272d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x272cc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.bl(0x2ec94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.b(C::Al,0x27294)?;    c.check(0x272c0)?;
    for word in DESCRIPTOR_RANGE_DIAGNOSTICS {c.emit(word)?;}
    c.check(0x272e4)
}

/// Transition controller availability after draining pending MMIO requests,
/// optionally reinitializing IRQ masks through the previously emitted helper.
fn enable_controller_ready(c:&mut Code)->Result<(),String>{
    c.check(0x272e4)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.literal_cond(C::Al,R::R0,0x27394)?;
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x2731c)?;
    c.literal_cond(C::Al,R::R0,0x27398)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.literal_cond(C::Al,R::R0,0x2739c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,917504)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3758096384)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,50)?)?;
    c.b(C::Ne,0x27340)?;
    c.literal_cond(C::Al,R::R0,0x273a0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,3758096396)?)?;
    c.bl(0x273a4)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,3758096388)?)?;
    c.bl(0x273a4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    c.b(C::Eq,0x27390)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x26c60)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x27394)?;
    for word in IRQ_MODE_STATUS {c.emit(word)?;}
    c.check(0x273a4)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    controller_status_is_ready(c)?;
    validate_stream_descriptor_range(c)?;
    enable_controller_ready(c)?;
    Ok(())
}
