// SPDX-License-Identifier: GPL-3.0-only
//! Controller service queue, IRQ checks, descriptor priority list operations.
//! Global pointers are modeled as named non-executable literal data, rather
//! than ARM instruction words; all real instructions use typed operations.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_runtime_services::Code;

pub const START:usize=0x9fc8;
pub const END:usize=0xa44c;
pub const FILE_OFFSET:usize=START+0x4000;
const HEAD_POINTER:u32=0x0010d818;
const QUEUE_ALTERNATIVES:[u32;2]=[0x0010d014, 0x0010d00c];
const STATUS_POINTER:u32=0x0010d000;
const QUEUE_HANDLE:u32=0x0010d818;
const OVERRIDE_POINTERS:[u32;2]=[0x00103a78, 0x0010d008];
const CLEAR_OVERRIDE_POINTERS:[u32;2]=[0x00103a78, 0x0010d008];
const ACTIVE_CONTROLLER:u32=0x001039f0;
const SERVICE_LOOP_POINTERS:[u32;2]=[0x0010d008, 0x00103a78];
const SERVICE_FLAG_POINTER:u32=0x00103a74;
const NODE_DATA:[u32;2]=[0x0010d008, 0xa5a5a5a5];
const IRQ_VALIDATION_GLOBALS:[u32;3]=[0x0010d008, 0x0010d000, 0x001039f4];
const CHAIN_HEADS:[u32;2]=[0x00103a7c, 0x00103a90];
const PRIORITY_HEADS:[u32;2]=[0x00103a7c, 0x00103a90];

/// assign queue head `[0x9fc8,0x9fd8)`.
fn assign_queue_head(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9fc8)?;
    c.literal(R::R1,0x9fd8)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x9fd8)?;
    Ok(())
}

/// advance resource queue `[0x9fdc,0xa01c)`.
fn advance_resource_queue(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9fdc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.literal(R::R2,0xa01c)?;
    c.literal(R::new(3)?,0xa020)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Eq, R::new(3)?, R::R2))?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa01c)?;
    Ok(())
}

/// set status pointer `[0xa024,0xa034)`.
fn set_status_pointer(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa024)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::PC,4)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa034)?;
    Ok(())
}

/// read queue handle `[0xa038,0xa040)`.
fn read_queue_handle(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa038)?;
    c.literal(R::R0,0xa040)?;
    c.branch(C::Al,0x8cc4)?;
    c.check(0xa040)?;
    Ok(())
}

/// install controller override `[0xa044,0xa098)`.
fn install_controller_override(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa044)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 147)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0xa098)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0xa09c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,116)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,112)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,116)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa098)?;
    Ok(())
}

/// clear controller override `[0xa0a0,0xa0e0)`.
fn clear_controller_override(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa0a0)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0xa0e0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0xa0e4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,116)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,116)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa0e0)?;
    Ok(())
}

/// read active controller `[0xa0e8,0xa0f4)`.
fn read_active_controller(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa0e8)?;
    c.literal(R::R0,0xa0f4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa0f4)?;
    Ok(())
}

/// run controller service loop `[0xa0f8,0xa1ac)`.
fn run_controller_service_loop(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa0f8)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.literal(R::R0,0xa1ac)?;
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::SP))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    c.call(0xa3a0)?;
    c.literal(R::new(7)?,0xa1b0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(9)?))?;
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::new(6)?, 1)?)?;
    c.branch(C::Ne,0xa18c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 1)?)?;
    c.call(0xa5bc)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    c.call(0xa4cc)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.call(0xe2248)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.branch(C::Ne,0xa130)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 1)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(8)?))?;
    c.branch(C::Al,0xa12c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::new(9)?, 255)?)?;
    c.branch(C::Eq,0xa1a4)?;
    c.literal(R::R0,0xa1ac)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.call(0x9538)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa1ac)?;
    Ok(())
}

/// read service flag `[0xa1b4,0xa1c0)`.
fn read_service_flag(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa1b4)?;
    c.literal(R::R0,0xa1c0)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa1c0)?;
    Ok(())
}

/// verify service node `[0xa1c4,0xa228)`.
fn verify_service_node(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa1c4)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.literal(R::R0,0xa228)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0xa224)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R1,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::LR, R::R0, 255)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,65280)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R12,R::R1,Shift::Lsl,2)?)?;
    c.literal(R::R2,0xa22c)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 3)?)?;
    c.branch(C::Ls,0xa1fc)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa228)?;
    Ok(())
}

/// platform irq state valid `[0xa230,0xa27c)`.
fn platform_irq_state_valid(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa230)?;
    c.literal(R::R1,0xa27c)?;
    c.literal(R::R2,0xa280)?;
    c.emit(arm32::mrs_status_register(C::Al,false,R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 19)?)?;
    c.literal(R::R0,0xa284)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(3)?, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa27c)?;
    Ok(())
}

/// repair resource neighbor order `[0xa288,0xa32c)`.
fn repair_resource_neighbor_order(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa288)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R12, 0)?)?;
    c.branch(C::Eq,0xa2b4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,20)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R12))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0xa2b8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Ne,0xa29c)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,28)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::bx(C::Eq,R::LR))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,28)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.literal(R::R0,0xa32c)?;
    c.literal(R::R2,0xa330)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(3)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa32c)?;
    Ok(())
}

/// insert resource by priority `[0xa334,0xa398)`.
fn insert_resource_by_priority(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa334)?;
    c.literal(R::R2,0xa398)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R12))?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,8)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs,0xa364)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R12,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R12))?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,8)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi,0xa34c)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,4)?)?;
    c.literal(R::R0,0xa39c)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R12, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(3)?,0)?)?;
    c.emit(arm32::data_register(C::Ne,Op::Sub,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(3)?,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa398)?;
    Ok(())
}

/// initialize service node `[0xa3a0,0xa3bc)`.
fn initialize_service_node(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa3a0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,16)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa3bc)?;
    Ok(())
}

/// remove service node `[0xa3bc,0xa44c)`.
fn remove_service_node(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa3bc)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0xa434)?;
    c.literal(R::R2,0xa44c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(5)?, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0xa3fc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,8)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(5)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.branch(C::Ne,0xa3e8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,4)?)?;
    c.literal(R::R2,0xa450)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(5)?,8)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::R1,8)?)?;
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::R1,8)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,8)?)?;
    c.branch(C::Al,0xa438)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne,0xa444)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe21b0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa44c)?;
    Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000;
    let mut bytes=vec![0;skip];
    let mut c=Code{bytes:&mut bytes,at:START};
    c.check(0x9fc8)?;
    assign_queue_head(&mut c)?;
    c.check(0x9fd8)?;
    c.check(0x9fd8)?;
    c.emit(HEAD_POINTER)?;
    c.check(0x9fdc)?;
    c.check(0x9fdc)?;
    advance_resource_queue(&mut c)?;
    c.check(0xa01c)?;
    c.check(0xa01c)?;
    for word in QUEUE_ALTERNATIVES{c.emit(word)?;}
    c.check(0xa024)?;
    c.check(0xa024)?;
    set_status_pointer(&mut c)?;
    c.check(0xa034)?;
    c.check(0xa034)?;
    c.emit(STATUS_POINTER)?;
    c.check(0xa038)?;
    c.check(0xa038)?;
    read_queue_handle(&mut c)?;
    c.check(0xa040)?;
    c.check(0xa040)?;
    c.emit(QUEUE_HANDLE)?;
    c.check(0xa044)?;
    c.check(0xa044)?;
    install_controller_override(&mut c)?;
    c.check(0xa098)?;
    c.check(0xa098)?;
    for word in OVERRIDE_POINTERS{c.emit(word)?;}
    c.check(0xa0a0)?;
    c.check(0xa0a0)?;
    clear_controller_override(&mut c)?;
    c.check(0xa0e0)?;
    c.check(0xa0e0)?;
    for word in CLEAR_OVERRIDE_POINTERS{c.emit(word)?;}
    c.check(0xa0e8)?;
    c.check(0xa0e8)?;
    read_active_controller(&mut c)?;
    c.check(0xa0f4)?;
    c.check(0xa0f4)?;
    c.emit(ACTIVE_CONTROLLER)?;
    c.check(0xa0f8)?;
    c.check(0xa0f8)?;
    run_controller_service_loop(&mut c)?;
    c.check(0xa1ac)?;
    c.check(0xa1ac)?;
    for word in SERVICE_LOOP_POINTERS{c.emit(word)?;}
    c.check(0xa1b4)?;
    c.check(0xa1b4)?;
    read_service_flag(&mut c)?;
    c.check(0xa1c0)?;
    c.check(0xa1c0)?;
    c.emit(SERVICE_FLAG_POINTER)?;
    c.check(0xa1c4)?;
    c.check(0xa1c4)?;
    verify_service_node(&mut c)?;
    c.check(0xa228)?;
    c.check(0xa228)?;
    for word in NODE_DATA{c.emit(word)?;}
    c.check(0xa230)?;
    c.check(0xa230)?;
    platform_irq_state_valid(&mut c)?;
    c.check(0xa27c)?;
    c.check(0xa27c)?;
    for word in IRQ_VALIDATION_GLOBALS{c.emit(word)?;}
    c.check(0xa288)?;
    c.check(0xa288)?;
    repair_resource_neighbor_order(&mut c)?;
    c.check(0xa32c)?;
    c.check(0xa32c)?;
    for word in CHAIN_HEADS{c.emit(word)?;}
    c.check(0xa334)?;
    c.check(0xa334)?;
    insert_resource_by_priority(&mut c)?;
    c.check(0xa398)?;
    c.check(0xa398)?;
    for word in PRIORITY_HEADS{c.emit(word)?;}
    c.check(0xa3a0)?;
    c.check(0xa3a0)?;
    initialize_service_node(&mut c)?;
    c.check(0xa3bc)?;
    c.check(0xa3bc)?;
    remove_service_node(&mut c)?;
    c.check(0xa44c)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_queues_and_priority_lists_are_contiguous(){
        let bytes=fragment().unwrap();
        assert_eq!(bytes.len(),END-START);
        assert_eq!(&bytes[0xa32c-START..0xa330-START],&CHAIN_HEADS[0].to_le_bytes());
    }
}
