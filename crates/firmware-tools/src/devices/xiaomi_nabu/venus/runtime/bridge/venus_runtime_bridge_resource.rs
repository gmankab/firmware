// SPDX-License-Identifier: GPL-3.0-only
//! Venus resource initialization, aligned record copies and disabling links.
//! The sentinel pattern and global pointer words are intentionally separated
//! from ARM32 instructions as non-executable data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_runtime_services::Code;

pub const START:usize=0x970c;
pub const END:usize=0x9b50;
pub const FILE_OFFSET:usize=START+0x4000;
const PLATFORM_HANDLER_STATE:u32=0x0010d008;
const RESOURCE_DESCRIPTOR_WORDS:[u32;7]=[0xa5a5a5a5, 0x0000970c, 0x00103a78, 0x0010d818, 0x001039fc, 0x0010d000, 0x001039f0];
const ALIGNED_COPY_DIAGNOSTIC:u32=0x000e375f;
const EXTENDED_COPY_DIAGNOSTIC:u32=0x000e375f;
const STRIDED_COPY_DIAGNOSTIC:u32=0x000e375f;
const RESOURCE_STATE_WORDS:[u32;3]=[0x00103a78, 0x0010d008, 0x0010d818];

/// invoke platform handler `[0x970c,0x9734)`.
fn invoke_platform_handler(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x970c)?;
    c.emit(arm32::push_registers(C::Al, R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    c.literal(R::R0,0x9734)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x9c5c)?;
    c.check(0x9734)?;
    Ok(())
}

/// forward resource registration `[0x9738,0x9740)`.
fn forward_resource_registration(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9738)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.branch(C::Al,0x9c5c)?;
    c.check(0x9740)?;
    Ok(())
}

/// initialize resource descriptor `[0x9740,0x98a0)`.
fn initialize_resource_descriptor(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9740)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.literal(R::R0,0x98a0)?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(3)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 19)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(3)?,R::R0,-68)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    c.literal(R::new(3)?,0x98a4)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,44)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,56)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,60)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(4)?,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,104)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,32)?)?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R0,R::R1,36)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,72)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,20)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,76)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(4)?,100)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,95)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,16)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq,0x97f4)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,84)?)?;
    c.emit(arm32::transfer_byte_postindexed(C::Al,true,R::R1,R::R2,1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 11)?)?;
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::R1,R::new(3)?,1)?)?;
    c.branch(C::Ne,0x97e0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,108)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,112)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,116)?)?;
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x98a8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0x98ac)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.literal(R::R1,0x98b0)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::new(3)?,R::R0,20)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R0,24)?)?;
    c.emit(arm32::data_register(C::Ne, Op::Cmp, R::R0, R::new(3)?, R::new(5)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi,0x9830)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,28)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,28)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,28)?)?;
    c.literal(R::R0,0x98b4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(5)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs,0x9888)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x9888)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.call(0x9264)?;
    c.literal(R::R0,0x98b8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x98a0)?;
    Ok(())
}

/// copy aligned record `[0x98bc,0x9968)`.
fn copy_aligned_record(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x98bc)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(3)?))?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.literal(R::R2,0x9968)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 324)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(4)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::new(5)?,3,true)?)?;
    c.emit(arm32::data_register(C::Ne,Op::Sub,R::new(5)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.call(0x9740)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9968)?;
    Ok(())
}

/// copy aligned record extended `[0x996c,0x9a18)`.
fn copy_aligned_record_extended(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x996c)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(3)?))?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.literal(R::R2,0x9a18)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 117)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(4)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,16)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::new(5)?,3,true)?)?;
    c.emit(arm32::data_register(C::Ne,Op::Sub,R::new(5)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::new(5)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    c.call(0x9740)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9a18)?;
    Ok(())
}

/// copy strided records `[0x9a1c,0x9ac8)`.
fn copy_strided_records(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9a1c)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(3)?))?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.literal(R::R2,0x9ac8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 161)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,8)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R1,R::new(7)?,3,true)?)?;
    c.branch(C::Eq,0x9a94)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(7)?,4)?)?;
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(7)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Sub,R::new(5)?,R::new(5)?,1)?)?;
    c.branch(C::Al,0x9a98)?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,24)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(4)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(5)?,Shift::Lsl,2)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(8)?))?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    c.call(0x9740)?;
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9ac8)?;
    Ok(())
}

/// set resource disabled `[0x9acc,0x9b44)`.
fn set_resource_disabled(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9acc)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,76)?)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9b44)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.literal(R::R0,0x9b48)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.branch(C::Ne,0x9b3c)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al, Op::Teq, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x9b3c)?;
    c.literal(R::R0,0x9b4c)?;
    c.call(0x8cc4)?;
    c.call(0x9264)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0xe2248)?;
    c.check(0x9b44)?;
    Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000;
    let mut bytes=vec![0;skip];
    let mut c=Code{bytes:&mut bytes,at:START};
    c.check(0x970c)?;
    invoke_platform_handler(&mut c)?;
    c.check(0x9734)?;
    c.check(0x9734)?;
    c.emit(PLATFORM_HANDLER_STATE)?;
    c.check(0x9738)?;
    c.check(0x9738)?;
    forward_resource_registration(&mut c)?;
    c.check(0x9740)?;
    c.check(0x9740)?;
    initialize_resource_descriptor(&mut c)?;
    c.check(0x98a0)?;
    c.check(0x98a0)?;
    for word in RESOURCE_DESCRIPTOR_WORDS{c.emit(word)?;}
    c.check(0x98bc)?;
    c.check(0x98bc)?;
    copy_aligned_record(&mut c)?;
    c.check(0x9968)?;
    c.check(0x9968)?;
    c.emit(ALIGNED_COPY_DIAGNOSTIC)?;
    c.check(0x996c)?;
    c.check(0x996c)?;
    copy_aligned_record_extended(&mut c)?;
    c.check(0x9a18)?;
    c.check(0x9a18)?;
    c.emit(EXTENDED_COPY_DIAGNOSTIC)?;
    c.check(0x9a1c)?;
    c.check(0x9a1c)?;
    copy_strided_records(&mut c)?;
    c.check(0x9ac8)?;
    c.check(0x9ac8)?;
    c.emit(STRIDED_COPY_DIAGNOSTIC)?;
    c.check(0x9acc)?;
    c.check(0x9acc)?;
    set_resource_disabled(&mut c)?;
    c.check(0x9b44)?;
    c.check(0x9b44)?;
    for word in RESOURCE_STATE_WORDS{c.emit(word)?;}
    c.check(0x9b50)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn resource_descriptor_and_record_copies_are_contiguous(){
        let bytes=fragment().unwrap();
        assert_eq!(bytes.len(),END-START);
        assert_eq!(u32::from_le_bytes(bytes[0x98a0-START..0x98a4-START].try_into().unwrap()),
                   RESOURCE_DESCRIPTOR_WORDS[0]);
    }
}
