// SPDX-License-Identifier: GPL-3.0-only
//! Tear down registered Venus callback channels, expose accessor tail calls,
//! and configure three controller IRQ classes. Opcode bytes are generated
//! entirely via checked A32 operations with identified literal constants.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const IRQ_DISCONNECT_STATE:[u32;2]=[0x0010_cf04,0x0010_40b4];
const IRQ_SLOT_TABLE_ADDR:u32=0x0010_40b4;
const IRQ_CHANNEL_ONE_DATA:[u32;2]=[0x0010_408c,0x0010_ceac];
const IRQ_CHANNEL_TWO_DATA:[u32;2]=[0x0010_4090,0x0010_ceac];

/// Disable named callback IDs and return resources to the controller pool.
fn release_interrupt_resource(c:&mut Code)->Result<(),String>{
    c.check(0x2634c)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal_cond(C::Al,R::new(4)?,0x2641c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    c.bl(0xd6c0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    c.bl(0xd6c0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.bl(0xd6c0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    c.bl(0xd6c0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    c.bl(0xd6c0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    c.bl(0xd74c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    c.bl(0xd6c0)?;
    c.literal_cond(C::Al,R::new(4)?,0x26420)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.bl(0x2bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    c.bl(0x2bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x2f1c)?;    c.check(0x2641c)?;
    for literal in IRQ_DISCONNECT_STATE { c.emit(literal)?; }
    c.check(0x26424)
}

/// Tail-call a single shared A32 accessor against the selected indexed slot.
/// Nine wrappers provide three data fields to three low-level register helpers.
fn emit_slot_accessor(c:&mut Code,entry:usize,field:i32,helper:usize)->Result<(),String>{
    c.check(entry)?;
    c.literal(R::R1,entry+0x10)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,field)?)?;
    c.b(C::Al,helper)?;
    c.check(entry+0x10)?;
    c.emit(IRQ_SLOT_TABLE_ADDR)?;
    c.check(entry+0x14)
}
fn resolve_indexed_slots(c:&mut Code)->Result<(),String>{
    // Explicit table dimensions: field a8/ac/b0 x operations read/write/release.
    for (base,field) in [(0x26424,168),(0x26460,172),(0x2649c,176)] {
        // Offsets are 0x14, including the intervening literal pool word.
        for (selector,operation) in [0x2bf4,0x2c04,0x2c34].iter().enumerate(){
            emit_slot_accessor(c,base+selector*0x14,field,*operation)?;
        }
    }
    c.check(0x264d8)
}

/// Enable the controller IRQ for one allocated hardware queue.
fn enable_controller_line_one(c:&mut Code)->Result<(),String>{
    c.check(0x264d8)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.literal_cond(C::Al,R::new(5)?,0x26530)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x2652c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    c.bl(0x2d20)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x292c)?;    c.check(0x2652c)?;
    for addr in IRQ_CHANNEL_ONE_DATA{c.emit(addr)?;}
    c.check(0x26534)
}

/// Enable the second controller IRQ with conditional per-queue fanout.
fn enable_controller_line_two(c:&mut Code)->Result<(),String>{
    c.check(0x26534)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,60)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    c.b(C::Ne,0x26574)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1040)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,15,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    c.literal_cond(C::Al,R::new(5)?,0x265e0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,15,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,63)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.literal_cond(C::Al,R::R1,0x265dc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    c.bl(0x2d20)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,8)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,16)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R0,52)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R0,R::R0,60)?)?;
    c.bl(0x292c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,3,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,68)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    c.b(C::Al,0x292c)?;    c.check(0x265dc)?;
    for addr in IRQ_CHANNEL_TWO_DATA{c.emit(addr)?;}
    c.check(0x265e4)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    release_interrupt_resource(c)?;
    resolve_indexed_slots(c)?;
    enable_controller_line_one(c)?;
    enable_controller_line_two(c)?;
    Ok(())
}
