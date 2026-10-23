// SPDX-License-Identifier: GPL-3.0-only
//! Event-list ownership, removal callbacks, and descriptor linkage.
//! These functions manipulate known record layouts using checked ARM A32
//! encodings; the remaining firmware is not reconstructed by this module.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_byte,
    transfer_multiple, transfer_word_preindexed_writeback, BlockMode,
    Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5cbc };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    let ret = (saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5d50)?;
    c.emit(load(r5, R::R0, 20)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, r4, r5, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, r4, 0)?)?;
    c.branch(C::Eq, 0x5d50)?;
    c.emit(load(R::R0, r5, 20)?)?;
    c.emit(cmp(R::R0, 1)?)?;
    c.branch(C::Eq, 0x5d2c)?;
    c.emit(load(R::R0, r4, 16)?)?;
    c.call(0x5958)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, R::R1, R::R0, 28)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, R::R0, r4, 32)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, R::R0, R::R0, 24)?)?;
    c.emit(store(r5, R::R0, 0)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(store(r5, r4, 32)?)?;
    c.emit(store(R::R0, r5, 20)?)?;
    c.emit(store(R::R1, r5, 24)?)?;
    c.emit(load(R::R0, r4, 16)?)?;
    c.call(0x59e8)?;
    c.check(0x5d2c)?;
    c.emit(load(R::R0, r4, 12)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(store(R::R1, R::SP, 0)?)?;
    c.emit(mov(R::R1, 256)?)?;
    c.call(0x553c)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5d50)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5d5c)?;
    // Allocate an event record, retaining references to its owning
    // resource, event dispatch parameters, and associated callback object.
    let wide_saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, wide_saved)?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, r5, r3))?;
    c.emit(mov_register(C::Al, r8, R::R2))?;
    c.emit(mov_register(C::Al, r7, R::R1))?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(cmp(r6, 0)?)?;
    c.emit(store(R::R0, R::SP, 4)?)?;
    c.branch(C::Eq, 0x5ddc)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(arm32::transfer_byte(C::Ne, true, R::R0, r5, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Tst, R::R0, R::R0, 16)?)?;
    c.branch(C::Eq, 0x5ddc)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 28)?)?;
    c.call(0x57d4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x5ddc)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(load(R::R0, R::R11, 8)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(store(r6, R::R1, 0)?)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R1,
        r7.bit() | r8.bit(), BlockMode::IncrementBefore, false)?)?;
    c.emit(store(r5, R::R1, 12)?)?;
    c.emit(store(R::R0, R::R1, 16)?)?;
    c.emit(store(r4, R::R1, 20)?)?;
    c.emit(store(r4, R::R1, 24)?)?;
    c.emit(store(R::R1, r5, 20)?)?;
    c.check(0x5ddc)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (wide_saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5de8)?;
    // Release the sidecar only when its callback flag and pointer are valid.
    let short_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let short_ret = (short_saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, short_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, short_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 16)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, R::R1, r4, 20)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne, 0x5e18)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, short_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5e18)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.call(0x5838)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 20)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, short_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5e2c)?;
    Ok(())
}
