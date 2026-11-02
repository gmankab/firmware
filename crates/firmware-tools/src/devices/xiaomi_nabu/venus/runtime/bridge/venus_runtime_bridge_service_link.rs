// SPDX-License-Identifier: GPL-3.0-only
//! Final Venus boot-to-service bridge: queue balancing, slot registration,
//! session initialization, forwarders to the separately built service routines,
//! and controller validation. Literal words are deliberately represented as
//! non-executable device data; executable words are encoded by checked A32.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_runtime_services::Code;

pub const START:usize=0xa44c;
pub const END:usize=0xaa0c;
pub const FILE_OFFSET:usize=START+0x4000;
const RESOURCE_POOL_HEADS:[u32;2]=[0x00103a7c, 0x00103a90];
const PRIORITY_RESOURCE_HEAD:u32=0x00103a7c;
const QUEUE_BALANCE_HEADS:[u32;2]=[0x00103a7c, 0x00103a90];
const NOTIFY_RESOURCE_HEADS:[u32;2]=[0x00103a7c, 0x00103a90];
const RESOURCE_LINK_HEADS:[u32;2]=[0x00103a7c, 0x00103a90];
const VIDEO_SESSION_GLOBALS:[u32;6]=[0x0010d894, 0x0000a800, 0x00103aa8, 0x00103ba2, 0x00103b64, 0x00103ae8];
const VIDEO_CONTROLLER_GLOBALS:[u32;2]=[0x00103aa4, 0x0010dad0];
const ALLOCATOR_CONTEXT:u32=0x0010d894;
const CODEC_BUFFER_CONTEXT:u32=0x0010d894;
const FRAME_ALLOCATION_CONTEXT:u32=0x0010d894;
const FRAME_VALIDATION_CONTEXT:u32=0x0010d894;
const SESSION_RESET_CONTEXT:u32=0x0010d894;
const SESSION_CONTEXT_DATA:u32=0x0010d894;
const DEVICE_CALLBACKS:[u32;3]=[0x0000a99c, 0x0000a9d4, 0x000e37d7];
const CONTROLLER_VALIDATION_DIAGNOSTIC:u32=0x000e37d7;
const FRAME_VALIDATION_DIAGNOSTIC:u32=0x000e37d7;

/// adjust priority resources `[0xa454,0xa520)`.
fn adjust_priority_resources(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa454)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.call(0xa3bc)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0)?)?;
    c.branch(C::Eq,0xa4a4)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    c.call(0xa334)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0xa4b8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    c.call(0x94f4)?;
    c.branch(C::Al,0xa4b8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0xa4b8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,12)?)?;
    c.call(0x9538)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.branch(C::Ne,0xa4c4)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe21b0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::PC,48)?)?;
    c.emit(arm32::data_register(C::Ne, Op::Cmp, R::R0, R::new(5)?, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0xa50c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,8)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(5)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R2,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.branch(C::Ne,0xa4f8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne,0xa518)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe21b0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa520)?;
    Ok(())
}

/// recalculate queue balance `[0xa524,0xa5b4)`.
fn recalculate_queue_balance(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa524)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.call(0xa3bc)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    c.branch(C::Al,0xa454)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.call(0x8e98)?;
    c.literal(R::R0,0xa5b4)?;
    c.literal(R::new(6)?,0xa5b8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(5)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0xa5ac)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi,0xa5a4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.call(0xa3bc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0xa594)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,12)?)?;
    c.call(0x9538)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(7)?))?;
    c.branch(C::Ne,0xa568)?;
    c.branch(C::Al,0xa5ac)?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0xa5b4)?;
    Ok(())
}

/// notify resource change `[0xa5bc,0xa638)`.
fn notify_resource_change(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa5bc)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R2))?;
    c.call(0xa454)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x95f0)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.literal(R::R0,0xa638)?;
    c.literal(R::new(6)?,0xa63c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0xa628)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0xa61c)?;
    c.call(0xa3bc)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    c.branch(C::Ne,0xa608)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x1029c4)?;
    c.check(0xa638)?;
    Ok(())
}

/// walk resource link `[0xa640,0xa69c)`.
fn walk_resource_link(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa640)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0xa69c)?;
    c.literal(R::R2,0xa6a0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0xa684)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R1,R::new(4)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 125)?)?;
    c.branch(C::Gt,0xa684)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0xa668)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x1029c4)?;
    c.check(0xa69c)?;
    Ok(())
}

/// initialize video session `[0xa6a4,0xa7e8)`.
fn initialize_video_session(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa6a4)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,9)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(5)?,9)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R2, 15)?)?;
    c.branch(C::Ne,0xa6b0)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.literal(R::new(4)?,0xa7e8)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(5)?,10)?)?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R0,Shift::Asr,31)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsr,28,false)?)?;
    c.literal(R::R1,0xa7ec)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,8)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,15)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,12)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    c.literal(R::R1,0xa7f0)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,44)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,560)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 17)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,-7)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,-6)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(5)?,-8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,-2)?)?;
    c.call(0x257b0)?;
    c.literal(R::new(6)?,0xa7f4)?;
    c.literal(R::new(8)?,0xa7f8)?;
    c.literal(R::new(7)?,0xa7fc)?;
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(6)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::R1,Shift::Lsl,1,false)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R1,R::R1,0)?)?;
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,2)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x257b0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 54)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x257b0)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 564)?)?;
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R0,R::new(4)?,R::new(9)?)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::R0,Shift::Lsl,1,false)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R1,R::R1,0)?)?;
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(7)?,R::R1,Shift::Lsl,2)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 568)?)?;
    c.emit(arm32::transfer_halfword_register(C::Al,false,R::R1,R::new(4)?,R::R2)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    c.call(0xaa0c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.call(0xaba0)?;
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(5)?,-8)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,-6)?)?;
    c.emit(arm32::transfer_halfword_register(C::Al,true,R::R0,R::new(4)?,R::new(9)?)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,-4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,-2)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::load_halfword(C::Al,R::R2,R::new(5)?,0)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(5)?,-10)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa7e8)?;
    Ok(())
}

/// register video controller `[0xa800,0xa85c)`.
fn register_video_controller(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa800)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.literal(R::new(5)?,0xa85c)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.branch(C::Hi,0xa850)?;
    c.literal(R::new(6)?,0xa860)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 304)?)?;
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::R0,R::R2)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(4)?,R::new(6)?,R::new(3)?,Shift::Lsl,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,8)?)?;
    c.call(0xd040)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,48)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 64)?)?;
    c.call(0xd3e0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0xa85c)?;
    Ok(())
}

/// forward session allocator `[0xa864,0xa870)`.
fn forward_session_allocator(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa864)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0,0xa870)?;
    c.branch(C::Al,0xb28c)?;
    c.check(0xa870)?;
    Ok(())
}

/// forward codec buffers `[0xa874,0xa888)`.
fn forward_codec_buffers(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa874)?;
    c.branch(C::Al,0xa864)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0,0xa888)?;
    c.branch(C::Al,0xc4cc)?;
    c.check(0xa888)?;
    Ok(())
}

/// forward frame allocation `[0xa88c,0xa89c)`.
fn forward_frame_allocation(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa88c)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0,0xa89c)?;
    c.branch(C::Al,0xb1b0)?;
    c.check(0xa89c)?;
    Ok(())
}

/// forward frame validation `[0xa8a0,0xa8ac)`.
fn forward_frame_validation(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa8a0)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0,0xa8ac)?;
    c.branch(C::Al,0xbe70)?;
    c.check(0xa8ac)?;
    Ok(())
}

/// reset session controller `[0xa8b0,0xa904)`.
fn reset_session_controller(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa8b0)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.literal(R::new(4)?,0xa904)?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 572)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.call(0x31ec)?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0xa8e8)?;
    c.branch(C::Al,0xa8e4)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.call(0xa6a4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    c.call(0x8634c)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xa954)?;
    c.check(0xa904)?;
    Ok(())
}

/// query session context `[0xa908,0xa914)`.
fn query_session_context(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa908)?;
    c.literal(R::R0,0xa914)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0xa914)?;
    Ok(())
}

/// handle device session callbacks `[0xa918,0xa990)`.
fn handle_device_session_callbacks(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa918)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 2)?)?;
    c.branch(C::Eq,0xa944)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 4)?)?;
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.call(0x257b0)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x257b0)?;
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::PC,48)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,36)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::PC,44)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,40)?)?;
    c.emit(arm32::bx(C::Ne,R::LR))?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x86644)?;
    c.literal(R::R2,0xa998)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 218)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x17b0)?;
    c.check(0xa990)?;
    Ok(())
}

/// validate session controller `[0xa99c,0xa9d0)`.
fn validate_session_controller(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa99c)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    c.call(0x86374)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.call(0x86644)?;
    c.literal(R::R2,0xa9d0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 91)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x17b0)?;
    c.check(0xa9d0)?;
    Ok(())
}

/// validate frame controller `[0xa9d4,0xaa08)`.
fn validate_frame_controller(c:&mut Code<'_>)->Result<(),String>{
    c.check(0xa9d4)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,44)?)?;
    c.call(0x86394)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.call(0x86644)?;
    c.literal(R::R2,0xaa08)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 101)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x17b0)?;
    c.check(0xaa08)?;
    Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000;
    let mut bytes=vec![0;skip];
    let mut c=Code{bytes:&mut bytes,at:START};
    c.check(0xa44c)?;
    for word in RESOURCE_POOL_HEADS{c.emit(word)?;}
    c.check(0xa454)?;
    c.check(0xa454)?;
    adjust_priority_resources(&mut c)?;
    c.check(0xa520)?;
    c.check(0xa520)?;
    c.emit(PRIORITY_RESOURCE_HEAD)?;
    c.check(0xa524)?;
    c.check(0xa524)?;
    recalculate_queue_balance(&mut c)?;
    c.check(0xa5b4)?;
    c.check(0xa5b4)?;
    for word in QUEUE_BALANCE_HEADS{c.emit(word)?;}
    c.check(0xa5bc)?;
    c.check(0xa5bc)?;
    notify_resource_change(&mut c)?;
    c.check(0xa638)?;
    c.check(0xa638)?;
    for word in NOTIFY_RESOURCE_HEADS{c.emit(word)?;}
    c.check(0xa640)?;
    c.check(0xa640)?;
    walk_resource_link(&mut c)?;
    c.check(0xa69c)?;
    c.check(0xa69c)?;
    for word in RESOURCE_LINK_HEADS{c.emit(word)?;}
    c.check(0xa6a4)?;
    c.check(0xa6a4)?;
    initialize_video_session(&mut c)?;
    c.check(0xa7e8)?;
    c.check(0xa7e8)?;
    for word in VIDEO_SESSION_GLOBALS{c.emit(word)?;}
    c.check(0xa800)?;
    c.check(0xa800)?;
    register_video_controller(&mut c)?;
    c.check(0xa85c)?;
    c.check(0xa85c)?;
    for word in VIDEO_CONTROLLER_GLOBALS{c.emit(word)?;}
    c.check(0xa864)?;
    c.check(0xa864)?;
    forward_session_allocator(&mut c)?;
    c.check(0xa870)?;
    c.check(0xa870)?;
    c.emit(ALLOCATOR_CONTEXT)?;
    c.check(0xa874)?;
    c.check(0xa874)?;
    forward_codec_buffers(&mut c)?;
    c.check(0xa888)?;
    c.check(0xa888)?;
    c.emit(CODEC_BUFFER_CONTEXT)?;
    c.check(0xa88c)?;
    c.check(0xa88c)?;
    forward_frame_allocation(&mut c)?;
    c.check(0xa89c)?;
    c.check(0xa89c)?;
    c.emit(FRAME_ALLOCATION_CONTEXT)?;
    c.check(0xa8a0)?;
    c.check(0xa8a0)?;
    forward_frame_validation(&mut c)?;
    c.check(0xa8ac)?;
    c.check(0xa8ac)?;
    c.emit(FRAME_VALIDATION_CONTEXT)?;
    c.check(0xa8b0)?;
    c.check(0xa8b0)?;
    reset_session_controller(&mut c)?;
    c.check(0xa904)?;
    c.check(0xa904)?;
    c.emit(SESSION_RESET_CONTEXT)?;
    c.check(0xa908)?;
    c.check(0xa908)?;
    query_session_context(&mut c)?;
    c.check(0xa914)?;
    c.check(0xa914)?;
    c.emit(SESSION_CONTEXT_DATA)?;
    c.check(0xa918)?;
    c.check(0xa918)?;
    handle_device_session_callbacks(&mut c)?;
    c.check(0xa990)?;
    c.check(0xa990)?;
    for word in DEVICE_CALLBACKS{c.emit(word)?;}
    c.check(0xa99c)?;
    c.check(0xa99c)?;
    validate_session_controller(&mut c)?;
    c.check(0xa9d0)?;
    c.check(0xa9d0)?;
    c.emit(CONTROLLER_VALIDATION_DIAGNOSTIC)?;
    c.check(0xa9d4)?;
    c.check(0xa9d4)?;
    validate_frame_controller(&mut c)?;
    c.check(0xaa08)?;
    c.check(0xaa08)?;
    c.emit(FRAME_VALIDATION_DIAGNOSTIC)?;
    c.check(0xaa0c)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn final_bridge_reaches_service_region_exactly(){
        let bytes=fragment().unwrap();
        assert_eq!(bytes.len(),END-START);
        assert_eq!(END,0xaa0c);
        assert_eq!(&bytes[0xaa08-START..0xaa0c-START],
                   &FRAME_VALIDATION_DIAGNOSTIC.to_le_bytes());
    }
}
