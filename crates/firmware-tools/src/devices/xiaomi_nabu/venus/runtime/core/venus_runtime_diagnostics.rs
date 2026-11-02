// SPDX-License-Identifier: GPL-3.0-only
//! Runtime diagnostic allocation, error reporting, and register snapshots.
//! The failure paths retain their return codes and conditional cleanup.
use arm32::{
    bx, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x573c };
    let r3 = R::new(3)?;
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
    c.emit(load(r6, R::R11, 12)?)?;
    c.emit(load(r8, R::R11, 8)?)?;
    c.emit(mov_register(C::Al, r7, R::R2))?;
    c.emit(mov_register(C::Al, r9, R::R0))?;
    c.emit(mov(r5, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmn, R::R0, r7, 1)?)?;
    c.emit(store(r6, R::SP, 4)?)?;
    c.emit(store(r5, r8, 0)?)?;
    c.branch(C::Eq, 0x57c8)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Ne, 0x578c)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 36)?)?;
    c.call(0x57d4)?;
    c.emit(load(r6, R::SP, 4)?)?;
    c.emit(mov(r5, 0x8000_0000)?)?;
    c.check(0x578c)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.call(0x56d0)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x57b4)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x57c8)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0xa8a0)?;
    c.branch(C::Al, 0x57c8)?;
    c.check(0x57b4)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, r5, r9, Shift::Lsl, 0, false)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 3)?)?;
    c.emit(store(R::R0, r6, 0)?)?;
    c.emit(store(r6, r8, 0)?)?;
    c.check(0x57c8)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x57d4)?;
    let small_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let small_ret = (small_saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, small_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5814)?;
    c.call(0xa864)?;
    c.literal(R::R2, 0x582c)?;
    c.emit(mov(R::R1, 160)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.call(0x17b0)?;
    c.emit(load(R::R1, r4, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(transfer_multiple(C::Ne, true, R::SP, small_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.literal(R::R2, 0x5830)?;
    c.branch(C::Al, 0x5818)?;
    c.check(0x5814)?;
    c.literal(R::R2, 0x5834)?;
    c.check(0x5818)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.call(0x4fb8)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, small_ret,
        BlockMode::IncrementAfter, true)?)?;
    for diagnostic_label in [0x000e_3390, 0x000e_33c9, 0x000e_336d] { c.emit(diagnostic_label)?; }
    c.check(0x5838)?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0xa8a0)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x584c)?;
    // Normalize one common diagnostic completion status.
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
    c.call(0xa8a0)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5888)?;
    // Snapshot four register-sized fields and derive a masked status word.
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq, 0x58d0)?;
    c.emit(load(R::R2, R::R0, 4)?)?;
    c.emit(load(r3, R::R2, 0)?)?;
    c.emit(store(r3, R::R1, 0)?)?;
    c.emit(load(r3, R::R2, 4)?)?;
    c.emit(store(r3, R::R1, 4)?)?;
    c.emit(load(R::R2, R::R2, 8)?)?;
    c.emit(store(R::R2, R::R1, 8)?)?;
    c.emit(store(R::R2, R::R1, 12)?)?;
    c.emit(mov(R::R2, 4080)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 1044480)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(R::R0, R::R1, 16)?)?;
    c.check(0x58d0)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x58d8)?;
    Ok(())
}
