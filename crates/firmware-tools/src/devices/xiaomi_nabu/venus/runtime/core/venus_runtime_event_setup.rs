// SPDX-License-Identifier: GPL-3.0-only
//! Construct the Venus event descriptor and link its backing resources.
//! Cleanup is explicit on failure; this is a partial ARM32 implementation,
//! not a complete independently authenticated Venus firmware image.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5b60 };
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
    c.emit(sub(R::SP, R::SP, 80)?)?;
    c.emit(load(r4, R::R11, 12)?)?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov_register(C::Al, r8, R::R1))?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(add(R::R2, R::SP, 8)?)?;
    c.emit(mov(R::R0, 256)?)?;
    c.emit(store(r4, R::R11, -28)?)?;
    c.call(0x5454)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r7, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5ba8)?;
    c.check(0x5b9c)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x5ba8)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x5c34)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 40)?)?;
    c.call(0x31ec)?;
    c.emit(load(R::R1, r4, 0)?)?;
    c.emit(load(R::R0, R::R11, 8)?)?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.emit(mov_register(C::Al, r3, r8))?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 4)?)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(store(R::R1, r4, 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(store(R::R1, r4, 20)?)?;
    c.emit(store(r4, R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x5eec)?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.emit(cmp(r7, 0)?)?;
    c.branch(C::Ne, 0x5c14)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.call(0x5618)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5c7c)?;
    c.check(0x5c14)?;
    c.emit(load(R::R0, R::R11, -28)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Lt, 0x5c54)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 40)?)?;
    c.call(0x31ec)?;
    c.branch(C::Al, 0x5b9c)?;
    c.check(0x5c34)?;
    c.emit(sub(R::R1, R::R11, 28)?)?;
    c.emit(mov(R::R0, 40)?)?;
    c.call(0x57d4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5c5c)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.call(0x4f1c)?;
    c.branch(C::Al, 0x5b9c)?;
    c.check(0x5c54)?;
    c.call(0x5838)?;
    c.branch(C::Al, 0x5b9c)?;
    c.check(0x5c5c)?;
    c.emit(load(r4, R::R11, -28)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 36)?)?;
    c.emit(add(R::R0, r4, 4)?)?;
    c.call(0x31ec)?;
    c.emit(mov(R::R0, 0x8000_0000)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.branch(C::Al, 0x5bc0)?;
    c.check(0x5c7c)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.call(0x4f1c)?;
    c.emit(mov(r7, 0)?)?;
    c.branch(C::Al, 0x5b9c)?;
    c.check(0x5c8c)?;
    // Legacy thunk forwards two additional stack arguments.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, R::R12, R::R0))?;
    c.literal(R::R0, 0x5cb8)?;
    c.emit(transfer_multiple(C::Al, false, R::SP,
        R::R2.bit() | r3.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, R::R12))?;
    c.call(0x5b60)?;
    c.emit(mov_register(C::Al, R::SP, R::R11))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_33e6)?; // Named legacy service identifier
    c.check(0x5cbc)?;
    Ok(())
}
