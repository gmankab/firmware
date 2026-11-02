// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller IRQ finalization, flag masks and readiness probes.
//! Hardware data words are intentionally separated from ARM32 opcodes.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
use super::venus_runtime_services::Code;

pub const START:usize=0x9400;
pub const END:usize=0x970c;
pub const FILE_OFFSET:usize=START+0x4000;
const IRQ_REPORT:[u32;3]=[0x000e375f, 0x0010d000, 0x000e379d];
const PENDING_IRQ_STATE:u32=0x00103a78;
const IRQ_GLOBAL_STATE:[u32;2]=[0x00103a78, 0x0010d000];
const POLL_GLOBAL_STATE:[u32;5]=[0x000e375f, 0x001039f4, 0x00103a78, 0x0010d008, 0x0010d818];
const READY_CALLS:[u32;2]=[0x0010cffc, 0x0010cff8];

/// finalize device irq `[0x9400,0x94e8)`.
fn finalize_device_irq(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9400)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 1)?)?;
    c.branch(C::Eq,0x94bc)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,32)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(3)?, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::mov_register(C::Hi, R::new(6)?, R::R2))?;
    c.emit(arm32::mov_register(C::Hi, R::R1, R::new(3)?))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    c.branch(C::Ne,0x9410)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0)?)?;
    c.branch(C::Eq,0x94d8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,36)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 1)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R12,R::R1,32)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(6)?,36)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,36)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,32)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,36)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,112)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Lt,0x9484)?;
    c.call(0xa2f4)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0)?)?;
    c.branch(C::Eq,0x94d4)?;
    c.literal(R::R0,0x94ec)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls,0x94d4)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.branch(C::Al,0x9264)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(5)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.literal(R::R2,0x94f0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 62)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    c.branch(C::Al,0x939c)?;
    c.check(0x94e8)?;
    Ok(())
}

/// clear controller flags `[0x94f4,0x9534)`.
fn clear_controller_flags(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x94f4)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x9534)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::new(6)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,12)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9534)?;
    Ok(())
}

/// enable controller flags `[0x9538,0x95e0)`.
fn enable_controller_flags(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x9538)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x95e0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,12)?)?;
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    c.emit(arm32::data_register(C::Al, Op::Tst, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x95d4)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,112)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Lt,0x9590)?;
    c.call(0xa2f4)?;
    c.literal(R::R0,0x95e4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,20)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls,0x95d4)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,76)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x95d4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x95d4)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x95d4)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    c.call(0x9264)?;
    c.call(0xe2248)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x95e0)?;
    Ok(())
}

/// poll controller flags `[0x95e8,0x96bc)`.
fn poll_controller_flags(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x95e8)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2230)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,31)?)?;
    c.literal(R::new(5)?,0x96bc)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 31)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 18)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1952)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.call(0x17b0)?;
    c.literal(R::R0,0x96c0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 162)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1792)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x96c4)?;
    c.literal(R::new(6)?,0x96c8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,12)?)?;
    c.emit(arm32::data_register(C::Al, Op::Tst, R::R0, R::new(5)?, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne,0x96b0)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,16)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,116)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0x969c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,112)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Lt,0x969c)?;
    c.call(0xa2f0)?;
    c.literal(R::R0,0x96cc)?;
    c.call(0x8cc4)?;
    c.call(0x9264)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,12)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x96bc)?;
    Ok(())
}

/// wait for controller ready `[0x96d0,0x9704)`.
fn wait_for_controller_ready(c:&mut Code<'_>)->Result<(),String>{
    c.check(0x96d0)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.literal(R::R0,0x9704)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_multiple(C::Ne,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.literal(R::new(4)?,0x9708)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x96f0)?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.check(0x9704)?;
    Ok(())
}

pub fn fragment()->Result<Vec<u8>,String>{
    let skip=START-0x1000;
    let mut bytes=vec![0;skip];
    let mut c=Code{bytes:&mut bytes,at:START};
    c.check(0x9400)?;
    finalize_device_irq(&mut c)?;
    c.check(0x94e8)?;
    c.check(0x94e8)?;
    for word in IRQ_REPORT{c.emit(word)?;}
    c.check(0x94f4)?;
    c.check(0x94f4)?;
    clear_controller_flags(&mut c)?;
    c.check(0x9534)?;
    c.check(0x9534)?;
    c.emit(PENDING_IRQ_STATE)?;
    c.check(0x9538)?;
    c.check(0x9538)?;
    enable_controller_flags(&mut c)?;
    c.check(0x95e0)?;
    c.check(0x95e0)?;
    for word in IRQ_GLOBAL_STATE{c.emit(word)?;}
    c.check(0x95e8)?;
    c.check(0x95e8)?;
    poll_controller_flags(&mut c)?;
    c.check(0x96bc)?;
    c.check(0x96bc)?;
    for word in POLL_GLOBAL_STATE{c.emit(word)?;}
    c.check(0x96d0)?;
    c.check(0x96d0)?;
    wait_for_controller_ready(&mut c)?;
    c.check(0x9704)?;
    c.check(0x9704)?;
    for word in READY_CALLS{c.emit(word)?;}
    c.check(0x970c)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_irq_and_ready_probes_are_contiguous(){
        assert_eq!(fragment().unwrap().len(),END-START);
    }
}
