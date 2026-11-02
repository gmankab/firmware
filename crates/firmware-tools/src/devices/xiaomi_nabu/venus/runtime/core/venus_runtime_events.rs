// SPDX-License-Identifier: GPL-3.0-only
//! Event object allocation and callback-aware reference counting.
//! ARM/Thumb interworking follows the public BLX immediate encoding.
use arm32::{
    branch_exchange_immediate, bx, data_immediate, data_register,
    mov_register, push_registers, transfer_byte, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x58d8 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(r6, 0)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.emit(store(R::R2, R::SP, 4)?)?;
    c.emit(store(r6, r4, 0)?)?;
    c.branch(C::Eq, 0x5908)?;
    c.emit(mov(R::R0, 2)?)?;
    c.branch(C::Al, 0x5924)?;
    c.check(0x5908)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 24)?)?;
    c.call(0x57d4)?;
    c.emit(load(R::R2, R::SP, 4)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x5950)?;
    c.emit(mov(R::R0, 0x8000_0002)?)?;
    c.check(0x5924)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, r5, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r5, 16)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R2, R::R0.bit() | r6.bit(),
        BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Eq, 0x5940)?;
    c.emit(add(R::R0, R::R2, 8)?)?;
    c.call(0x8d50)?;
    c.emit(load(R::R2, R::SP, 4)?)?;
    c.check(0x5940)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R2, r4, 0)?)?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.branch(C::Al, 0x5948)?;
    c.check(0x5958)?;
    // Increment reference count either via the external Thumb helper or
    // the sidecar callback depending on the descriptor's bit flags.
    let small_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let ret = (small_saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, small_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 16)?)?;
    c.branch(C::Ne, 0x5990)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x000e_2198)?)?;
    c.emit(load(R::R1, r4, 4)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, R::R1, R::R1, 1)?)?;
    c.emit(store(R::R1, r4, 4)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5990)?;
    c.emit(add(R::R0, r4, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, small_saved,
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x8ec4)?;
    c.check(0x599c)?;
    // Reference/state query, redirecting to the sidecar only when enabled.
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 16)?)?;
    c.branch(C::Ne, 0x59c0)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x59c0)?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(add(R::R0, R::R1, 8)?)?;
    c.call(0x8d70)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(data_immediate(C::Eq, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x59e8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 16)?)?;
    c.branch(C::Ne, 0x5a14)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x5a1c)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(store(R::R1, R::R0, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x5a14)?;
    c.emit(add(R::R0, R::R0, 8)?)?;
    c.branch(C::Al, 0x92d8)?;
    c.branch(C::Al, 0x1029c4)?;
    c.check(0x5a20)?;
    // Completion marker, also reached from the earlier tagged dispatcher.
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(data_immediate(C::Ge, Op::Mov, R::R1, R::R0, 0x00ad_0000)?)?;
    c.emit(data_immediate(C::Ge, Op::Orr, R::R1, R::R1, 0xde00_0000)?)?;
    c.emit(arm32::transfer_word(C::Ge, false, R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Ge, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Ge, R::LR))?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x5838)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x5a5c)?;
    Ok(())
}
