// SPDX-License-Identifier: GPL-3.0-only
//! Runtime interrupt-counter accounting, active slot relinking and per-slot
//! platform transitions. Literal tables are named separately from A32 code.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
use super::venus_runtime_services::Code;

pub const START: usize = 0x8e98;
pub const END: usize = 0x9400;
pub const FILE_OFFSET: usize = START + 0x4000;
const IRQ_HANDLER_STATE: u32 = 0x00103a78;
const SLOT_CONTEXT: [u32; 5] = [0x001039f4, 0x000e375f, 0x00103a78, 0x0010d008, 0x0010d818];
const PENDING_IRQS: u32 = 0x001039f4;
const SYNC_A: [u32; 3] = [0x00103a78, 0x0010d818, 0x0010d000];
const SYNC_B: [u32; 4] = [0x00103a78, 0x0010d008, 0x0010d000, 0x001039f4];
const SYNC_C: [u32; 3] = [0x000e375f, 0x00103a78, 0x0010d008];

/// count pending interrupts `[0x8e98,0x8ec0)`.
fn count_pending_interrupts(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8e98)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R2,0x8ec0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x8ec0)?;
    Ok(())
}

/// relink active slots `[0x8ec4,0x9110)`.
fn relink_active_slots(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8ec4)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.literal(R::R0,0x9110)?;
    c.literal(R::new(5)?,0x9114)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 77)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 78)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9118)?;
    c.literal(R::new(8)?,0x911c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,68)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,72)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 84)?)?;
    c.emit(arm32::data_immediate(C::Cc, Op::Mov, R::new(6)?, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.call(0x17b0)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0x90b4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x90e0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,72)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,72)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,36)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,36)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls,0x9088)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,72)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x9054)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,40)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc,0x9054)?;
    c.literal(R::new(6)?,0x9114)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 168)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R2,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,1024)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 2)?)?;
    c.branch(C::Cs,0x9014)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.call(0x17b0)?;
    c.branch(C::Al,0x902c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x902c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::R2,-4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x9050)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,40)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    c.branch(C::Cs,0x8ff8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,40)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs,0x9078)?;
    c.literal(R::R2,0x9114)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 179)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,20)?)?;
    c.call(0x9170)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,116)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x90a4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,112)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Lt,0x90a4)?;
    c.call(0xa2f0)?;
    c.literal(R::R0,0x9120)?;
    c.call(0x8cc4)?;
    c.call(0x9264)?;
    c.branch(C::Al,0x9108)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,72)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,72)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    c.branch(C::Al,0x9108)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 255)?)?;
    c.literal(R::R2,0x9114)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 92)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x9110)?;
    Ok(())
}

/// is slot active `[0x9124,0x913c)`.
fn is_slot_active(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x9124)?;
    c.literal(R::R0,0x913c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x913c)?;
    Ok(())
}

/// record interrupt transition `[0x9140,0x9258)`.
fn record_interrupt_transition(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x9140)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(5)?,20)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::new(7)?,0x9258)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,20)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(4)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x9250)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,24)?)?;
    c.literal(R::R0,0x925c)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,24)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi,0x91e0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,28)?)?;
    c.literal(R::R0,0x9260)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs,0x924c)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(5)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x924c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x924c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x924c)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    c.call(0x9264)?;
    c.call(0xe2248)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9258)?;
    Ok(())
}

/// track platform state `[0x9264,0x92c8)`.
fn track_platform_state(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x9264)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x92c8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.literal(R::R1,0x92d4)?;
    c.emit(arm32::mrs_status_register(C::Al,false,R::R0))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0x92bc)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 19)?)?;
    c.branch(C::Ne,0x92bc)?;
    c.literal(R::R0,0x92cc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.literal(R::R0,0x92d0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::branch(C::Ne,c.at as u32,0xa288,true)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x1029d0)?;
    c.check(0x92c8)?;
    Ok(())
}

/// update slot list `[0x92d8,0x934c)`.
fn update_slot_list(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x92d8)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.literal(R::R2,0x934c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 154)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9350)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0x9354)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 1)?)?;
    c.call(0x9358)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x934c)?;
    Ok(())
}

/// begin device irq handler `[0x9358,0x9400)`.
fn begin_device_irq_handler(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x9358)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,72)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R2))?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,40)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc,0x9390)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x9390)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0x93a8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 253)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.literal(R::R2,0x94e8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x17b0)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 2)?)?;
    c.emit(arm32::data_immediate(C::Cs,Op::Sub,R::R0,R::R2,1)?)?;
    c.emit(arm32::transfer_byte(C::Cs,false,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Cs,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(5)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x93d4)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 15)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    c.branch(C::Al,0x9398)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,72)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x93f8)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R2))?;
    c.call(0x9170)?;
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(5)?))?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::R12,8)?)?;
    c.check(0x9400)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let skip = START - 0x1000;
    let mut bytes = vec![0u8; skip];
    let mut c = Code { bytes: &mut bytes, at: START };
    count_pending_interrupts(&mut c)?;
    c.check(0x8ec0)?;
    c.emit(IRQ_HANDLER_STATE)?;
    c.check(0x8ec4)?;
    relink_active_slots(&mut c)?;
    c.check(0x9110)?;
    for w in SLOT_CONTEXT { c.emit(w)?; }
    c.check(0x9124)?;
    is_slot_active(&mut c)?;
    c.check(0x913c)?;
    c.emit(PENDING_IRQS)?;
    c.check(0x9140)?;
    record_interrupt_transition(&mut c)?;
    c.check(0x9258)?;
    for w in SYNC_A { c.emit(w)?; }
    c.check(0x9264)?;
    track_platform_state(&mut c)?;
    c.check(0x92c8)?;
    for w in SYNC_B { c.emit(w)?; }
    c.check(0x92d8)?;
    update_slot_list(&mut c)?;
    c.check(0x934c)?;
    for w in SYNC_C { c.emit(w)?; }
    c.check(0x9358)?;
    begin_device_irq_handler(&mut c)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn platform_interrupt_state_has_exact_layout() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(),END - START);
        assert_eq!(&bytes[0x934c-START..0x9358-START],
                   &SYNC_C.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<_>>());
    }
}
