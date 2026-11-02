// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec queue pointer resolution, bounded copy, codec slot acquisition
//! and global queue-state access. Pointer pools are data, not A32 opcodes.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
use super::venus_runtime_services::Code;

pub const START: usize = 0x8c00;
pub const END: usize = 0x8e98;
pub const FILE_OFFSET: usize = START + 0x4000;
const QUEUE_WORDS: [u32; 2] = [0x0010_39e0, 0x0010_3048];
const CODEC_CURRENT: u32 = 0x0010_36f4;
const BAD_QUEUE_CONTEXT: [u32; 3] = [0x000e_372d, 0x0010_d000, 0x000e_3700];
const CODEC_STATE_WORDS: [u32; 3] = [0x0010_d008, 0x000e_375f, 0x0010_3a78];
const CODEC_STATE: u32 = 0x0010_d008;

/// resolve global queue `[0x8c00,0x8c1c)`.
fn resolve_global_queue(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8c00)?;
    c.literal(R::R0,0x8c20)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.emit(arm32::mov_register(C::Ne, R::R0, R::R2))?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8c1c)?;
    Ok(())
}

/// initialize queue once `[0x8c24,0x8c48)`.
fn initialize_queue_once(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8c24)?;
    c.literal(R::R0,0x8c48)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::bx(C::Ne,R::LR))?;
    c.literal(R::R1,0x8c4c)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8c48)?;
    Ok(())
}

/// current codec ptr `[0x8c50,0x8c58)`.
fn current_codec_ptr(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8c50)?;
    c.literal(R::R0,0x8c58)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8c58)?;
    Ok(())
}

/// install current queue `[0x8c5c,0x8c80)`.
fn install_current_queue(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8c5c)?;
    c.literal(R::R2,0x8c80)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::PC,20)?)?;
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R1,4)?)?;
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R2,0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8c80)?;
    Ok(())
}

/// get active descriptor `[0x8c88,0x8c90)`.
fn get_active_descriptor(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8c88)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8c90)?;
    Ok(())
}

/// compare queue limits `[0x8c90,0x8d44)`.
fn compare_queue_limits(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8c90)?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(3)?))?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(4)?, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::mov_register(C::Ls, R::new(4)?, R::R1))?;
    c.emit(arm32::mov_register(C::Al, R::R1, R::R2))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    c.call(0x471c)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.branch(C::Al,0x17b0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8d10)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8d24)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,76)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x8cfc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne,0x8cfc)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,32)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq,0x8d38)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne,0x8cd8)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 199)?)?;
    c.branch(C::Al,0x8d28)?;
    c.literal(R::R2,0x8d4c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 170)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    c.branch(C::Al,0x17b0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 182)?)?;
    c.literal(R::R2,0x8d44)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.branch(C::Al,0x17b0)?;
    c.literal(R::R1,0x8d48)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8d44)?;
    Ok(())
}

/// acquire codec slot `[0x8d50,0x8e7c)`.
fn acquire_codec_slot(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8d50)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    c.literal(R::R0,0x8e7c)?;
    c.literal(R::new(6)?,0x8e80)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 3)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,0)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.call(0x17b0)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 4)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,72)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,68)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 7)?)?;
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    c.emit(arm32::data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.call(0x17b0)?;
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0xe2198)?)?;
    c.literal(R::R1,0x8e84)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R2,1)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq,0x8e10)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(5)?, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq,0x8e3c)?;
    c.call(0xe2248)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.branch(C::Al,0x8e54)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,72)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,72)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,4)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,4)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,20)?)?;
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    c.branch(C::Al,0x8e4c)?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 255)?)?;
    c.branch(C::Eq,0x8e5c)?;
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,0)?)?;
    c.call(0xe2248)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 1)?)?;
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    c.call(0xe2248)?;
    c.literal(R::R2,0x8e80)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 19)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    c.call(0x17b0)?;
    c.branch(C::Al,0x8e54)?;
    c.check(0x8e7c)?;
    Ok(())
}

/// read codec state `[0x8e88,0x8e94)`.
fn read_codec_state(c: &mut Code<'_>) -> Result<(), String> {
    c.check(0x8e88)?;
    c.literal(R::R0,0x8e94)?;
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x8e94)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let skip = START - 0x1000;
    let mut bytes = vec![0u8; skip];
    let mut c = Code { bytes: &mut bytes, at: START };
    resolve_global_queue(&mut c)?;
    c.check(0x8c1c)?;
    for value in QUEUE_WORDS { c.emit(value)?; }
    c.check(0x8c24)?;
    initialize_queue_once(&mut c)?;
    c.check(0x8c48)?;
    for value in QUEUE_WORDS { c.emit(value)?; }
    c.check(0x8c50)?;
    current_codec_ptr(&mut c)?;
    c.check(0x8c58)?;
    c.emit(CODEC_CURRENT)?;
    c.check(0x8c5c)?;
    install_current_queue(&mut c)?;
    c.check(0x8c80)?;
    for value in QUEUE_WORDS { c.emit(value)?; }
    c.check(0x8c88)?;
    get_active_descriptor(&mut c)?;
    compare_queue_limits(&mut c)?;
    c.check(0x8d44)?;
    for value in BAD_QUEUE_CONTEXT { c.emit(value)?; }
    c.check(0x8d50)?;
    acquire_codec_slot(&mut c)?;
    c.check(0x8e7c)?;
    for value in CODEC_STATE_WORDS { c.emit(value)?; }
    c.check(0x8e88)?;
    read_codec_state(&mut c)?;
    c.check(0x8e94)?;
    c.emit(CODEC_STATE)?;
    c.check(0x8e98)?;
    c.check(END)?;
    Ok(bytes.split_off(skip))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn codec_queues_have_no_gaps() {
        let bytes=fragment().unwrap();
        assert_eq!(bytes.len(), END-START);
        assert_eq!(&bytes[0x8e94-START..], &CODEC_STATE.to_le_bytes());
        assert_eq!(&bytes[0x8d44-START..0x8d50-START],
            &BAD_QUEUE_CONTEXT.iter().flat_map(|word| word.to_le_bytes()).collect::<Vec<_>>());
    }
}
