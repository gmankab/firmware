// SPDX-License-Identifier: GPL-3.0-only
//! Interrupt pending-bit event loop and controller resource synchronization.
//! The two original routines handle a 4-way priority sweep, callback handoff
//! and status-dependent flags; all instructions are A32-typed, not opaque
//! machine words. Five global pointers are classified as data separately.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

const EVENT_CONTROL_LITERALS:[u32;5]=[
    0x0010_ceac,0xe000_8464,0x0010_409c,0x0010_40a0,0x000e_832c,
];

/// Service pending event bits, visit registered per-source callbacks, and
/// acknowledge the controller until pending bits are fully drained.
fn service_pending_events(c:&mut Code)->Result<(),String>{
    c.check(0x26014)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,180)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(8)?,R::R0,R::new(5)?,Shift::Lsl,15)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    c.b(C::Eq,0x261c4)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,15)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,1024)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,132)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R0,4096)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,52)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,1536)?)?;
    c.b(C::Eq,0x260c0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(10)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    c.b(C::Eq,0x260c0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(10)?,R::new(9)?,Shift::Lsl,R::new(7)?,false)?)?;
    c.b(C::Eq,0x260b4)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(7)?,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x260a8)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::new(7)?,Shift::Lsl,3)?)?;
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    c.b(C::Al,0x260b4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    c.bl(0x15d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    c.b(C::Cc,0x26084)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7798784)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1996488704)?)?;
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    c.b(C::Eq,0x2615c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,458752)?)?;
    c.b(C::Ne,0x26104)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,7340032)?)?;
    c.b(C::Ne,0x26104)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,117440512)?)?;
    c.b(C::Ne,0x26104)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,1879048192)?)?;
    c.b(C::Eq,0x2610c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(10)?,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    c.b(C::Eq,0x2615c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(7)?,R::new(9)?,Shift::Lsl,R::new(6)?,false)?)?;
    c.b(C::Eq,0x26150)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x26144)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    c.b(C::Al,0x26150)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    c.bl(0x15d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,6)?)?;
    c.b(C::Cc,0x26120)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,256)?)?;
    c.b(C::Eq,0x261b4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(7)?,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    c.b(C::Eq,0x261b4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(7)?,R::new(9)?,Shift::Lsl,R::new(6)?,false)?)?;
    c.b(C::Eq,0x261a8)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,3,false)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,76)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    c.b(C::Eq,0x2619c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,72)?)?;
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    c.b(C::Al,0x261a8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    c.bl(0x15d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,4)?)?;
    c.b(C::Cc,0x26178)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(8)?,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    c.b(C::Ne,0x26064)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x261d0)
}

/// Preserve IRQ-controller state while dispatching the handler results.
fn synchronize_controller_event_state(c:&mut Code)->Result<(),String>{
    c.check(0x261d0)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    c.literal_cond(C::Al,R::R0,0x26338)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32768)?)?;
    c.literal_cond(C::Al,R::new(10)?,0x26340)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,63)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,3758096384)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,184)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,156)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,164)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,184)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    c.b(C::Al,0x2630c)?;
    c.literal_cond(C::Al,R::R0,0x2633c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,2)?)?;
    c.b(C::Ne,0x2625c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,4)?)?;
    c.b(C::Ne,0x26284)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,256)?)?;
    c.b(C::Ne,0x262b4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.bl(0x15d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,122)?)?;
    c.b(C::Al,0x262e8)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,196)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,196)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,2208)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,2208)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,60)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(10)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,20)?)?;
    c.bl(0x292c)?;
    c.b(C::Al,0x26304)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(6)?,228)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,228)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,60)?)?;
    c.literal_cond(C::Al,R::R0,0x26344)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(6)?,24)?)?;
    c.bl(0x292c)?;
    c.b(C::Al,0x26304)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,52)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    c.b(C::Ne,0x262f8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    c.b(C::Ne,0x26300)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    c.bl(0x15d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,115)?)?;
    c.literal_cond(C::Al,R::R2,0x26348)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    c.bl(0x17b0)?;
    c.b(C::Al,0x26304)?;
    c.bl(0x25a74)?;
    c.b(C::Al,0x26304)?;
    c.bl(0x25acc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Eq,Op::Tst,R::R0,R::R0,4)?)?;
    c.b(C::Ne,0x2621c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,256)?)?;
    c.b(C::Ne,0x2621c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x26338)?;
    for addr in EVENT_CONTROL_LITERALS {c.emit(addr)?;}
    c.check(0x2634c)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    service_pending_events(c)?;
    synchronize_controller_event_state(c)?;
    Ok(())
}
