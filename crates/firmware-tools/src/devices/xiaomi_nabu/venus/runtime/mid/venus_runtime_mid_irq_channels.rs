// SPDX-License-Identifier: GPL-3.0-only
//! Venus IRQ source handlers and controller mask registers, including MMIO
//! bit configuration, status snapshots, and repeat-read barriers. All A32
//! instruction fields are constructed from ISA operations; mapped peripheral
//! pointers and logging constants are separately classified as data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const IRQ_CLASS_ONE:[u32;2]=[0x0010_4098,0x0010_ceac];
const IRQ_STATUS_20:[u32;1]=[0x0010_408c];
const IRQ_CLASSES_2_AND_3:[u32;2]=[0x0010_4090,0x0010_4094];
const IRQ_STATUS_28:[u32;1]=[0x0010_4098];
const IRQ_DMA_CLOCK:[u32;1]=[0xe00c_2024];
const IRQ_DISPATCH_CONTROL:[u32;6]=[
    0x0010_40e0,0xe00e_0110,0x0010_40c0,0x000e_8364,
    0x000e_2788,0x0010_40bc,
];
const IRQ_HW_RESET:[u32;6]=[
    0xe00e_0100,0xe000_8004,0xffff_fef9,
    0xe000_8468,0xe000_98dc,0xe000_c508,
];

/// Set channel IRQ mask bit 4 and publish the associated platform pointer.
fn write_irq_class_one(c:&mut Code)->Result<(),String>{
    c.check(0x265e4)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    c.literal_cond(C::Al,R::new(5)?,0x26654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1040)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x26650)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    c.bl(0x2d20)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,44)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x292c)?;    c.check(0x26650)?;
    for literal in IRQ_CLASS_ONE{c.emit(literal)?;}
    c.check(0x26658)
}

/// Clear and ack the per-source status for bit 20.
fn write_irq_status_20(c:&mut Code)->Result<(),String>{
    c.check(0x26658)?;
    c.literal_cond(C::Al,R::R2,0x26678)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x26678)?;
    for literal in IRQ_STATUS_20{c.emit(literal)?;}
    c.check(0x2667c)
}

/// Read/clear channel status and write synchronized class two/three flags.
fn write_irq_classes_two_and_three(c:&mut Code)->Result<(),String>{
    c.check(0x2667c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,36)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.literal_cond(C::Al,R::R2,0x266d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,2)?)?;
    c.literal_cond(C::Al,R::R2,0x266d4)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x266d0)?;
    for literal in IRQ_CLASSES_2_AND_3{c.emit(literal)?;}
    c.check(0x266d8)
}

/// Reset a channel status bit using an MMIO interface.
fn clear_irq_status_24(c:&mut Code)->Result<(),String>{
    c.check(0x266d8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,40)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.literal_cond(C::Al,R::R2,0x26724)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,28)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x26724)?;
    for literal in IRQ_STATUS_28{c.emit(literal)?;}
    c.check(0x26728)
}

/// Read channel clock/mask register; helper also controls IRQ 28 reset.
fn read_irq_clock(c:&mut Code)->Result<(),String>{
    c.check(0x26728)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.literal_cond(C::Al,R::R0,0x26760)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x26760)?;
    for literal in IRQ_DMA_CLOCK{c.emit(literal)?;}
    c.check(0x26764)
}

/// Await platform IRQ pending flags, update service counters and return the final state.
fn synchronize_irq_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0x26764)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.literal_cond(C::Al,R::R0,0x2682c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Eq,0x267fc)?;
    c.literal_cond(C::Al,R::new(5)?,0x26830)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    c.b(C::Eq,0x2677c)?;
    c.literal_cond(C::Al,R::new(6)?,0x26834)?;
    c.literal_cond(C::Al,R::R1,0x26838)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(4)?,R::R0,Shift::Lsr,2,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,0)?)?;
    c.bl(0x2ec94)?;
    c.literal_cond(C::Al,R::R1,0x2683c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.bl(0x2ec94)?;
    c.bl(0x3070)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x267ec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,6)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    c.b(C::Ne,0x267d4)?;
    c.bl(0x26844)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    c.bl(0x3074)?;
    c.b(C::Al,0x26824)?;
    c.bl(0x3070)?;
    c.literal_cond(C::Al,R::R0,0x26840)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    c.b(C::Ne,0x2681c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.bl(0x26844)?;
    c.bl(0x3074)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x2682c)?;
    for literal in IRQ_DISPATCH_CONTROL{c.emit(literal)?;}
    c.check(0x26844)
}

/// Hardware init/reset writes MMIO controls and repeated read barriers; returns readiness.
fn reset_irq_hardware(c:&mut Code)->Result<(),String>{
    c.check(0x26844)?;
    c.literal_cond(C::Al,R::R0,0x26924)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    c.b(C::Ne,0x26858)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    c.b(C::Ne,0x26870)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    c.b(C::Ne,0x2688c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,20)?)?;
    c.b(C::Ne,0x268a4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,8)?)?;
    c.literal_cond(C::Al,R::R2,0x26928)?;
    c.literal_cond(C::Al,R::new(3)?,0x2692c)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,3)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,52)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    c.literal_cond(C::Al,R::R2,0x26930)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.literal_cond(C::Al,R::R2,0x26934)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,-256)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,50)?)?;
    c.b(C::Ne,0x268f4)?;
    c.literal_cond(C::Al,R::R0,0x26938)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x26924)?;
    for literal in IRQ_HW_RESET{c.emit(literal)?;}
    c.check(0x2693c)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    write_irq_class_one(c)?;
    write_irq_status_20(c)?;
    write_irq_classes_two_and_three(c)?;
    clear_irq_status_24(c)?;
    read_irq_clock(c)?;
    synchronize_irq_dispatch(c)?;
    reset_irq_hardware(c)?;
    Ok(())
}
