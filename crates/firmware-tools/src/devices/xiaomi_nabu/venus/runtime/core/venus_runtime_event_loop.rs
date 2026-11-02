// SPDX-License-Identifier: GPL-3.0-only
//! Lifecycle for a device-owned event queue and its registered callbacks.
//! Object creation, start, draining, and release are encoded as structured
//! ARM32 operations; this routine does not contain reference opcode arrays.
use arm32::{
    branch_exchange_with_link, data_immediate, mov_register, push_registers,
    transfer_multiple, transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5a5c };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.call(0x5fa8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x5ab8)?;
    c.emit(mov_register(C::Al, r9, r4))?;
    c.emit(mov(R::R0, 256)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r8, r9, 12)?)?;
    c.emit(mov_register(C::Al, R::R1, r9))?;
    c.call(0x5454)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x5ab8)?;
    c.emit(add(r6, r4, 16)?)?;
    c.emit(mov(R::R0, 64)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r7, 0)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.call(0x58d8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5ac4)?;
    c.check(0x5ab8)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5ac4)?;
    c.emit(mov_register(C::Al, R::R0, r8))?;
    c.emit(mov(R::R1, 256)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(R::new(3)?, 0)?)?;
    c.emit(store(r7, r4, 20)?)?;
    c.emit(store(r7, R::SP, 0)?)?;
    c.call(0x553c)?;
    c.check(0x5ae0)?;
    c.emit(load(R::R0, r9, 0)?)?;
    c.call(0x5618)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x5ae0)?;
    c.branch(C::Al, 0x5b40)?;
    c.check(0x5af4)?;
    // Repeated event dispatch: preserve the linked-list successor, release
    // its transient resources and call the user-supplied service function.
    c.emit(store(r7, r5, 20)?)?;
    c.emit(load(R::R0, r5, 24)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 28)?)?;
    c.emit(arm32::transfer_word(C::Eq, false, r7, r4, 32)?)?;
    c.emit(load(R::R0, r6, 0)?)?;
    c.call(0x59e8)?;
    c.emit(load(R::R0, r5, 16)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5b20)?;
    c.call(0x5958)?;
    c.check(0x5b20)?;
    c.emit(load(R::R2, r5, 4)?)?;
    c.emit(load(R::R1, r5, 8)?)?;
    c.emit(load(R::R0, r5, 12)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R2)?)?;
    c.emit(load(R::R0, r5, 16)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5b40)?;
    c.call(0x59e8)?;
    c.check(0x5b40)?;
    c.emit(load(R::R0, r4, 16)?)?;
    c.call(0x5958)?;
    c.emit(load(r5, r4, 28)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Ne, 0x5af4)?;
    c.emit(load(R::R0, r6, 0)?)?;
    c.call(0x59e8)?;
    c.branch(C::Al, 0x5ae0)?;
    c.check(0x5b60)?;
    Ok(())
}
