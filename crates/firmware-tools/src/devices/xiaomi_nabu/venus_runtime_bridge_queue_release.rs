// SPDX-License-Identifier: GPL-3.0-only
//! Venus resource unlink/release, reference-count changes and queue setup.
//! Device global pointers and error-address literal words are modeled as
//! named non-instruction data. No vendor instruction words are copied.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_runtime_services::Code;

pub const START:usize=0x9b50;
pub const END:usize=0x9fc8;
pub const FILE_OFFSET:usize=START+0x4000;
const DISABLED_STATE_WORDS:[u32;2]=[0x00103a78, 0x0010d000];
const RELEASE_COUNTERS:[u32;6]=[0x00103a78, 0x001039f0, 0x001039f8, 0x0010d008, 0x001039f4, 0x0010d818];
const UNLOCK_POINTERS:[u32;3]=[0x00103a78, 0x001039f4, 0x001039f8];
const REFCOUNT_POINTERS:[u32;3]=[0x00103a78, 0x001039f8, 0x001039f4];
const QUEUE_HEAD:u32=0x0010d008;
const PRIORITY_SLOT:u32=0x0010d008;
const QUEUE_INITIALIZER_WORDS:[u32;11]=[0x00103a78, 0x0010d010, 0x0010cff4, 0x0010d818, 0x001039fc, 0x0010d008, 0x0010d000, 0x000e378f, 0x000096d0, 0x0010d018, 0x00103a74];

/// clear disabled state `[0x9b50,0x9bc8)`.
fn clear_disabled_state(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9b50)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9bc8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.branch(C::Ne,0x9bc0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,76)?)?;
    c.literal(R::R0,0x9bcc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls,0x9bc0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x9bc0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x9bc0)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.call(0x9264)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x9bc8)?;
    Ok(())
}

/// unlink resource neighbors `[0x9bd0,0x9c5c)`.
fn unlink_resource_neighbors(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9bd0)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R12, 0)?)?;
    c.branch(C::Eq,0x9c04)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0)?)?;
    c.branch(C::Eq,0x9c34)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x9c10)?;
    c.branch(C::Al,0x9c34)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::LR,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x9c34)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::LR))?;
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,80)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x9c20)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R12, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::LR, 0)?)?;
    c.branch(C::Eq,0x9c58)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::LR,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9c5c)?;
    Ok(())
}

/// release resource descriptor `[0x9c5c,0x9d9c)`.
fn release_resource_descriptor(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9c5c)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9d9c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.call(0x9db4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne,0x9ca0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x9d90)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.call(0x9bd0)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.call(0xa5e0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,116)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x9ccc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,112)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Lt,0x9ccc)?;
    c.call(0xa2f0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(4)?,40)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,72)?)?;
    c.branch(C::Al,0x9d14)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,36)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 1)?)?;
    c.branch(C::Eq,0x9cec)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,32)?)?;
    c.branch(C::Al,0x9cf4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,36)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,36)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,72)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,72)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc,0x9d3c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x9cd8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.call(0x9358)?;
    c.branch(C::Al,0x9cd0)?;
    c.literal(R::R0,0x9da0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.branch(C::Eq,0x9d90)?;
    c.literal(R::R0,0x9da8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x9d84)?;
    c.literal(R::R0,0x9da4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::PC,44)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    c.literal(R::R0,0x9db0)?;
    c.call(0x8cc4)?;
    c.call(0x9264)?;
    c.call(0x9e2c)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x9d9c)?;
    Ok(())
}

/// unlock resource handler `[0x9db4,0x9e20)`.
fn unlock_resource_handler(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9db4)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al, Op::Teq, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9e20)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0x9e24)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.literal(R::R0,0x9e28)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x9e20)?;
    Ok(())
}

/// decrement resource refcount `[0x9e2c,0x9ea8)`.
fn decrement_resource_refcount(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9e2c)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al, Op::Teq, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9ea8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0x9eac)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0x9e90)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R1,R::R1,1,true)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.branch(C::Ne,0x9ea0)?;
    c.literal(R::R0,0x9eb0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.call(0x9264)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x9ea8)?;
    Ok(())
}

/// resolve queue head `[0x9eb4,0x9ec4)`.
fn resolve_queue_head(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9eb4)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0,0x9ec4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.branch(C::Al,0x9170)?;
    c.check(0x9ec4)?;
    Ok(())
}

/// resolve priority `[0x9ec8,0x9ed8)`.
fn resolve_priority(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9ec8)?;
    c.literal(R::R0,0x9ed8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x9ed8)?;
    Ok(())
}

/// initialize controller queues `[0x9edc,0x9f9c)`.
fn initialize_controller_queues(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9edc)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 211)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2210)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9f9c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0x9fa0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.literal(R::R0,0x9fa4)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::store_halfword(C::Al,R::new(5)?,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.call(0xa2fc)?;
    c.literal(R::new(4)?,0x9fac)?;
    c.literal(R::R0,0x9fa8)?;
    c.literal(R::new(6)?,0x9fb4)?;
    c.literal(R::R1,0x9fc0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 2048)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    c.literal(R::R0,0x9fb0)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.literal(R::R0,0x9fb8)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    c.literal(R::R0,0x9fbc)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.call(0x98bc)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,20)?)?;
    c.call(0xe2248)?;
    c.literal(R::R0,0x9fc4)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2190)?;
    c.check(0x9f9c)?;
    Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000;
    let mut bytes=vec![0;skip];
    let mut c=Code{bytes:&mut bytes,at:START};
    c.check(0x9b50)?;
    clear_disabled_state(&mut c)?;
    c.check(0x9bc8)?;
    c.check(0x9bc8)?;
    for word in DISABLED_STATE_WORDS{c.emit(word)?;}
    c.check(0x9bd0)?;
    c.check(0x9bd0)?;
    unlink_resource_neighbors(&mut c)?;
    c.check(0x9c5c)?;
    c.check(0x9c5c)?;
    release_resource_descriptor(&mut c)?;
    c.check(0x9d9c)?;
    c.check(0x9d9c)?;
    for word in RELEASE_COUNTERS{c.emit(word)?;}
    c.check(0x9db4)?;
    c.check(0x9db4)?;
    unlock_resource_handler(&mut c)?;
    c.check(0x9e20)?;
    c.check(0x9e20)?;
    for word in UNLOCK_POINTERS{c.emit(word)?;}
    c.check(0x9e2c)?;
    c.check(0x9e2c)?;
    decrement_resource_refcount(&mut c)?;
    c.check(0x9ea8)?;
    c.check(0x9ea8)?;
    for word in REFCOUNT_POINTERS{c.emit(word)?;}
    c.check(0x9eb4)?;
    c.check(0x9eb4)?;
    resolve_queue_head(&mut c)?;
    c.check(0x9ec4)?;
    c.check(0x9ec4)?;
    c.emit(QUEUE_HEAD)?;
    c.check(0x9ec8)?;
    c.check(0x9ec8)?;
    resolve_priority(&mut c)?;
    c.check(0x9ed8)?;
    c.check(0x9ed8)?;
    c.emit(PRIORITY_SLOT)?;
    c.check(0x9edc)?;
    c.check(0x9edc)?;
    initialize_controller_queues(&mut c)?;
    c.check(0x9f9c)?;
    c.check(0x9f9c)?;
    for word in QUEUE_INITIALIZER_WORDS{c.emit(word)?;}
    c.check(0x9fc8)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn resource_release_and_queue_init_are_contiguous(){
        let bytes=fragment().unwrap();
        assert_eq!(bytes.len(),END-START);
        assert_eq!(u32::from_le_bytes(bytes[0x9f9c-START..0x9fa0-START].try_into().unwrap()),
                   QUEUE_INITIALIZER_WORDS[0]);
    }
}
