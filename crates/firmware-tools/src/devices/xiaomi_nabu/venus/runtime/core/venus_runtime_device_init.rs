// SPDX-License-Identifier: GPL-3.0-only
//! Device runtime initialization: acquire buffers, resolve service endpoints,
//! initialize dispatch callbacks and free partially initialized resources.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6050 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 48)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(r6, R::R0, 12)?)?;
    c.emit(load(R::R0, r6, 4)?)?;
    c.emit(add(r5, r6, 16)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x83e0)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6174)?;
    c.literal(R::R1, 0x6180)?;
    c.emit(add(r3, R::SP, 28)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6174)?;
    c.emit(load(R::R0, R::SP, 36)?)?;
    c.literal(R::R1, 0x6184)?;
    c.emit(add(r3, R::SP, 28)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r7, 0)?)?;
    c.emit(store(R::R0, r6, 88)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6174)?;
    c.emit(load(R::R2, R::SP, 36)?)?;
    c.emit(add(R::R0, r6, 92)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(mov(r3, 4096)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, false, R::SP,
        R::R0.bit() | r7.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(mov(R::R0, 3200)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 524288)?)?;
    c.call(0x573c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6174)?;
    c.emit(load(R::R0, r6, 92)?)?;
    c.emit(add(R::R1, R::SP, 8)?)?;
    c.call(0x5888)?;
    c.emit(load(R::R0, R::SP, 8)?)?;
    c.call(0x6a54)?;
    c.emit(load(R::R0, r6, 88)?)?;
    c.call(0x631c)?;
    c.call(0x6b18)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6164)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.call(0x700c)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.call(0x6ea0)?;
    c.emit(sub(r3, R::R11, 24)?)?;
    for reg in [R::R0, R::R1, R::R2] { c.emit(mov(reg, 0)?)?; }
    c.call(0x6ee4)?;
    c.call(0x6bd4)?;
    c.emit(mov(R::R0, 16)?)?;
    c.call(0x6b30)?;
    c.call(0x7070)?;
    c.emit(mov(R::R1, 255)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 3840)?)?;
    c.call(0x6da8)?;
    c.call(0x6a98)?;
    c.check(0x6164)?;
    c.emit(load(R::R0, r6, 88)?)?;
    c.call(0x6e60)?;
    c.call(0x6e08)?;
    c.emit(mov(r4, 0)?)?;
    c.check(0x6174)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x6180)?;
    // Property/service IDs are data, not executable instructions.
    c.emit(0x000e_3424)?;
    c.emit(0x000e_3432)?;
    c.check(0x6188)?;
    Ok(())
}
