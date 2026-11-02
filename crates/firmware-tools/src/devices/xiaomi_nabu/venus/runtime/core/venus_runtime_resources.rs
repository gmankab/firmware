// SPDX-License-Identifier: GPL-3.0-only
//! Runtime resource descriptors, optional external backing, and event modes.
//! Flags and control cases remain named inputs to checked A32 encodings.
use arm32::{
    branch_exchange_with_link, data_immediate, mov_register, push_registers,
    transfer_byte, transfer_multiple, BlockMode, Condition as C,
    DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5454 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.emit(store(R::R2, R::SP, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r4, 192)?)?;
    c.emit(store(R::R0, r5, 0)?)?;
    c.branch(C::Ne, 0x5530)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r4, 48)?)?;
    c.emit(data_immediate(C::Eq, Op::Orr, r4, r4, 256)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r7, r4, 16)?)?;
    c.branch(C::Eq, 0x54a4)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R2, 0)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.branch(C::Al, 0x54d0)?;
    c.check(0x54a4)?;
    c.emit(mov(R::R0, 12)?)?;
    c.emit(cmp(r7, 0)?)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 32)?)?;
    c.call(0x57d4)?;
    c.emit(load(R::R2, R::SP, 4)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x5530)?;
    c.emit(mov(R::R0, 0x8000_0000)?)?;
    c.emit(store(R::R0, R::R2, 0)?)?;
    c.emit(mov(R::R0, 0x8000_0001)?)?;
    c.check(0x54d0)?;
    c.emit(store(R::R0, R::R2, 0)?)?;
    c.emit(store(R::R2, r5, 0)?)?;
    c.emit(cmp(r7, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, R::R0, R::R2, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Orr, R::R0, R::R0, 16)?)?;
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r4, 32)?)?;
    c.branch(C::Eq, 0x5508)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::R2, 4)?)?;
    c.emit(store(R::R0, R::R2, 8)?)?;
    c.emit(load(R::R0, R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 32)?)?;
    c.emit(store(R::R0, R::R2, 0)?)?;
    c.check(0x5508)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r4, 256)?)?;
    c.branch(C::Eq, 0x5530)?;
    c.emit(load(R::R0, R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 256)?)?;
    c.emit(store(R::R0, R::R2, 0)?)?;
    c.call(0x8e88)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R1, R::R0.bit() | r6.bit(),
        BlockMode::IncrementBefore, false)?)?;
    c.check(0x5530)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x553c)?;

    // Resource control dispatch: create, disable, refresh, and invoke the
    // optional client callback using the caller-supplied extra argument.
    let small_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let small_return = (small_saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, small_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x5608)?;
    c.emit(cmp(R::R1, 1024)?)?;
    c.branch(C::Eq, 0x55a4)?;
    c.emit(cmp(R::R1, 512)?)?;
    c.branch(C::Eq, 0x55bc)?;
    c.emit(cmp(R::R1, 256)?)?;
    c.branch(C::Ne, 0x5608)?;
    c.emit(load(R::R1, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 32)?)?;
    c.branch(C::Ne, 0x55e0)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 16)?)?;
    c.branch(C::Ne, 0x560c)?;
    c.emit(load(R::R1, r4, 4)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x5608)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(store(R::R0, r4, 8)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov(R::R1, 1)?)?;
    c.call(0x9538)?;
    c.branch(C::Al, 0x5604)?;
    c.check(0x55a4)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 48)?)?;
    c.emit(transfer_multiple(C::Ne, true, R::SP, small_return,
        BlockMode::IncrementAfter, true)?)?;
    c.call(0x8e88)?;
    c.emit(store(R::R0, r4, 4)?)?;
    c.branch(C::Al, 0x5604)?;
    c.check(0x55bc)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 48)?)?;
    c.branch(C::Ne, 0x5608)?;
    c.call(0x8e88)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R1, r4, 4)?)?;
    c.emit(store(R::R0, r4, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, small_return,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x55e0)?;
    c.emit(load(R::LR, r4, 4)?)?;
    c.emit(cmp(R::LR, 0)?)?;
    c.branch(C::Eq, 0x5608)?;
    c.emit(load(R::R12, R::R11, 8)?)?;
    c.emit(load(R::R0, r4, 8)?)?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.emit(mov_register(C::Al, R::R2, R::new(3)?))?;
    c.emit(mov_register(C::Al, R::new(3)?, R::R12))?;
    c.emit(branch_exchange_with_link(C::Al, R::LR)?)?;
    c.check(0x5604)?;
    c.emit(mov(R::R0, 0)?)?;
    c.check(0x5608)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, small_return,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x560c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, small_saved,
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x5cbc)?;
    c.check(0x5618)?;
    Ok(())
}
