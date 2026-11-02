// SPDX-License-Identifier: GPL-3.0-only
//! Venus interrupt dispatch initialization and codec callback shutdown.
//! Separate executable ARM32 operations from MMIO/firmware data and pointers.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;

const IRQ_INITIALIZER_CONFIG:[u32;13]=[
    0x0010_e078,0x0010_e07c,0x0010_40b4,0x0010_cf04,
    0x0002_5fc4,0x0002_5fe8,0x0002_6014,0x0002_61d0,
    0x0010_40a4,0xe00d_2004,0xe00c_2020,0x0010_40d0,
    0x000e_832c,
];
const IRQ_PENDING_INDICATOR:[u32;1]=[0xe00e_000c];
const IRQ_CLEAR_RESOURCES:[u32;2]=[0x0010_40d0,0xe00c_100c];

/// Initialize controller IRQ callbacks for several event IDs and manage
/// the shared IRQ resource initialization/rollback on failure.
fn initialize_irq_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0x25d14)?;
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,917504)?)?;
    c.literal_cond(C::Al,R::R2,0x25f90)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,3758096384)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65280)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4278190080)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.literal_cond(C::Al,R::R1,0x25f94)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,184)?)?;
    c.bl(0x2efc)?;
    c.literal_cond(C::Al,R::new(5)?,0x25f98)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    c.b(C::Eq,0x25f44)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,21)?)?;
    c.b(C::Ne,0x25d84)?;
    c.literal_cond(C::Al,R::new(6)?,0x25f9c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.bl(0xd708)?;
    c.literal_cond(C::Al,R::new(4)?,0x25fa0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,19)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.bl(0xd654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    c.bl(0xd708)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    c.bl(0xd654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.bl(0xd708)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.literal_cond(C::Al,R::R2,0x25fa4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,22)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    c.bl(0xd654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    c.bl(0xd708)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    c.bl(0xd654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    c.bl(0xd708)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.literal_cond(C::Al,R::R2,0x25fa8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    c.bl(0xd654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    c.bl(0xd708)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.literal_cond(C::Al,R::R2,0x25fac)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    c.bl(0xd654)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,168)?)?;
    c.bl(0x2b8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x25f58)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,172)?)?;
    c.bl(0x2b8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x25f58)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,176)?)?;
    c.bl(0x2b8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x25f58)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.literal_cond(C::Al,R::new(4)?,0x25fb0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,180)?)?;
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,12)?)?;
    c.bl(0x2b8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x25f58)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    c.bl(0x2b8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x25f58)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,8)?)?;
    c.bl(0x2b8c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    c.b(C::Ne,0x25f58)?;
    c.literal_cond(C::Al,R::R0,0x25fb4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,130)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,268)?)?;
    c.literal_cond(C::Al,R::R1,0x25fb8)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,8)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.literal_cond(C::Al,R::R0,0x25fbc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.bl(0x312fc)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.b(C::Al,0x25f88)?;
    c.literal_cond(C::Al,R::R2,0x25fc0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,229)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    c.bl(0x17b0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,176)?)?;
    c.bl(0x2bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,172)?)?;
    c.bl(0x2bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,168)?)?;
    c.bl(0x2bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.bl(0x2f1c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x25f90)?;
    for addr in IRQ_INITIALIZER_CONFIG {c.emit(addr)?;}
    c.check(0x25fc4)
}

/// Submit the callback for a pending handler and return an explicit status.
fn update_irq_notifier(c:&mut Code)->Result<(),String>{
    c.check(0x25fc4)?;
    c.emit(arm32::push_registers(C::Al,R::R11.bit() | R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    c.literal_cond(C::Al,R::R0,0x25fe4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    c.bl(0x15d0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;    c.check(0x25fe4)?;
    for addr in IRQ_PENDING_INDICATOR {c.emit(addr)?;}
    c.check(0x25fe8)
}

/// Advance the IRQ status counter and acknowledge a cleared pending flag.
fn release_irq_notifier(c:&mut Code)->Result<(),String>{
    c.check(0x25fe8)?;
    c.literal_cond(C::Al,R::R0,0x2600c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    c.literal_cond(C::Al,R::R0,0x26010)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;    c.check(0x2600c)?;
    for addr in IRQ_CLEAR_RESOURCES {c.emit(addr)?;}
    c.check(0x26014)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    initialize_irq_dispatch(c)?;
    update_irq_notifier(c)?;
    release_irq_notifier(c)?;
    Ok(())
}
