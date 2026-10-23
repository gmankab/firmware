// SPDX-License-Identifier: GPL-3.0-only
//! Begin allocation of a device DMA/event record with reference-aware linking.
//! Lookup and failure handling use checked typed A32 operations.
use arm32::{
    data_immediate, mov_register, mov_shifted_register, push_registers,
    transfer_multiple, transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x65b8 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | r10.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 28)?)?;
    c.emit(sub(R::SP, R::SP, 44)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov_register(C::Al, r9, R::R1))?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(load(r6, R::R0, 12)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R11, -32)?)?;
    c.branch(C::Eq, 0x65f8)?;
    c.emit(load(R::R0, r6, 108)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x6700)?;
    c.emit(add(r10, r6, 104)?)?;
    c.branch(C::Al, 0x65fc)?;
    c.check(0x65f8)?;
    c.emit(add(r10, r6, 92)?)?;
    c.check(0x65fc)?;
    c.emit(load(R::R0, r6, 120)?)?;
    c.call(0x5958)?;
    c.emit(mov_register(C::Al, r5, r6))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, R::R0, r5, 116)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R11, -32)?)?;
    c.branch(C::Eq, 0x663c)?;
    c.check(0x6618)?;
    c.emit(mov_register(C::Al, r8, R::R0))?;
    c.emit(load(R::R0, r8, 16)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x66dc)?;
    c.emit(load(R::R0, r8, 28)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R11, -32)?)?;
    c.branch(C::Ne, 0x6618)?;
    c.branch(C::Al, 0x6640)?;
    c.check(0x663c)?;
    c.emit(mov(r8, 0)?)?;
    c.check(0x6640)?;
    c.emit(sub(R::R1, R::R11, 32)?)?;
    c.emit(mov(R::R0, 32)?)?;
    c.call(0x57d4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x66f8)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 32)?)?;
    c.call(0x31ec)?;
    c.emit(load(R::R1, r10, 4)?)?;
    c.emit(load(R::R0, r10, 8)?)?;
    c.emit(add(R::R2, R::SP, 36)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    c.call(0x696c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x66d0)?;
    c.emit(load(R::R0, r10, 0)?)?;
    c.emit(load(R::R1, R::SP, 36)?)?;
    c.emit(mov(R::R2, 16)?)?;
    c.emit(store(R::R1, R::SP, 12)?)?;
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0,
        R::R1, Shift::Lsl, 4, false)?)?;
    c.emit(load(R::R1, R::R11, -32)?)?;
    c.emit(add(r7, R::R1, 8)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R1,
        R::R0.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(store(R::R2, R::R1, 16)?)?;
    c.emit(store(r10, R::R1, 24)?)?;
    c.emit(mov(R::R0, 4)?)?;
    c.emit(mov_register(C::Al, R::R1, r7))?;
    c.call(0x57d4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x676c)?;
    c.emit(load(R::R1, r10, 4)?)?;
    c.emit(load(R::R0, r10, 8)?)?;
    c.emit(load(R::R2, R::SP, 12)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    c.call(0x69fc)?;
    c.check(0x66d0)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.call(0x5838)?;
    c.branch(C::Al, 0x66f8)?;
    c.check(0x66dc)?;
    c.emit(mov_register(C::Al, R::R0, r8))?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(add(R::R2, R::SP, 36)?)?;
    c.emit(mov(R::R1, 16)?)?;
    c.call(0x696c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x6710)?;
    c.check(0x66f8)?;
    c.emit(load(R::R0, r6, 120)?)?;
    c.call(0x59e8)?;
    c.check(0x6700)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r7, R::R0, 0)?)?;
    c.check(0x6704)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(sub(R::SP, R::R11, 28)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x6710)?;
    Ok(())
}
