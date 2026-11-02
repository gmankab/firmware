// SPDX-License-Identifier: GPL-3.0-only
//! Walk DMA allocation descriptors, release completed regions, and unlink
//! them from the device list. All control flow is explicit ARM A32 encoding.
use arm32::{
    data_register, mov_register, mov_shifted_register,
    push_registers, transfer_multiple, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6810 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r3 = R::new(3)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(load(r6, R::R0, 12)?)?;
    c.emit(load(R::R0, r6, 120)?)?;
    c.call(0x5958)?;
    c.emit(load(R::R0, r6, 116)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x6908)?;
    c.emit(mov(r7, 0)?)?;
    c.check(0x683c)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(load(R::R0, r4, 20)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0,
        r5, Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, 0x685c)?;
    c.emit(load(R::R1, r4, 4)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R0,
        R::R1, Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1,
        r5, Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, 0x6870)?;
    c.check(0x685c)?;
    c.emit(load(R::R0, r4, 28)?)?;
    c.emit(mov_register(C::Al, r7, r4))?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x683c)?;
    c.branch(C::Al, 0x6908)?;
    c.check(0x6870)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r5, R::R0,
        Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R1, 16)?)?;
    c.emit(mov_shifted_register(C::Al, R::R2, R::R0, Shift::Lsr, 8)?)?;
    c.emit(load(R::R0, r4, 8)?)?;
    c.call(0x69fc)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6908)?;
    c.emit(load(R::R0, r4, 16)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(store(R::R0, r4, 16)?)?;
    c.emit(load(R::R1, r4, 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0,
        R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x6908)?;
    c.emit(load(R::R0, r4, 24)?)?;
    c.emit(load(R::R1, r4, 0)?)?;
    c.emit(load(r3, R::R0, 4)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(mov_shifted_register(C::Al, R::R2, R::R1, Shift::Lsr, 4)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, r3, Shift::Lsr, 4)?)?;
    c.call(0x69fc)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6908)?;
    c.emit(load(R::R0, r4, 12)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x68d8)?;
    c.call(0x4f1c)?;
    c.check(0x68d8)?;
    c.emit(cmp(r7, 0)?)?;
    c.branch(C::Eq, 0x68ec)?;
    c.emit(load(R::R0, r4, 28)?)?;
    c.emit(store(R::R0, r7, 28)?)?;
    c.branch(C::Al, 0x68f8)?;
    c.check(0x68ec)?;
    c.emit(load(R::R0, r6, 116)?)?;
    c.emit(load(R::R0, R::R0, 28)?)?;
    c.emit(store(R::R0, r6, 116)?)?;
    c.check(0x68f8)?;
    c.emit(load(R::R0, r4, 8)?)?;
    c.call(0x5838)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x5838)?;
    c.check(0x6908)?;
    c.emit(load(R::R0, r6, 120)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved,
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x59e8)?;
    c.check(0x6914)?;
    Ok(())
}
