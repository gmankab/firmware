// SPDX-License-Identifier: GPL-3.0-only
//! Event-resource teardown, diagnostics state access, and runtime descriptor setup.
//! Code is assembled from typed ARM A32 instructions; addresses and region
//! constants are properties of this device's runtime image.
use arm32::{
    data_immediate, data_register, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5e2c };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r3 = R::new(3)?;
    let saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    let ret = (saved & !R::LR.bit()) | R::PC.bit();
    // Clear a linked event and release its dependent resources.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, r5, R::R0, 0)?)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 15)?)?;
    c.emit(cmp(R::R0, 4)?)?;
    c.branch(C::Ne, 0x5ea4)?;
    for (at, offset, skip) in [(0x5e4c, 12, 0x5e5c), (0x5e5c, 16, 0x5e6c)] {
        c.check(at)?;
        c.emit(load(R::R0, r4, offset)?)?;
        c.emit(cmp(R::R0, 0)?)?;
        c.branch(C::Eq, skip)?;
        c.call(0x4f1c)?;
    }
    c.check(0x5e6c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x5fb0)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Lt, 0x5e98)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 40)?)?;
    c.emit(mov(r5, 0)?)?;
    c.call(0x31ec)?;
    c.branch(C::Al, 0x5ea4)?;
    c.check(0x5e98)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x5838)?;
    c.emit(mov(r5, 0)?)?;
    c.check(0x5ea4)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    // Three accessors for a runtime context structure.
    c.check(0x5eac)?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x8e88)?;
    c.emit(add(R::R0, R::R0, 84)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x5ec0)?;
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.call(0x8e88)?;
    c.emit(store(r4, R::R0, 96)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5ed8)?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x8e88)?;
    c.emit(load(R::R0, R::R0, 96)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // Initialize a bounded resource descriptor and its callback/command bank.
    c.check(0x5eec)?;
    let wide_saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, wide_saved)?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(sub(R::SP, R::SP, 24)?)?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.emit(mov_register(C::Al, r4, r3))?;
    c.emit(mov_register(C::Al, r8, R::R2))?;
    c.emit(sub(R::R0, r5, 1)?)?;
    c.emit(cmp(R::R0, 249)?)?;
    c.emit(load(R::R0, r7, 0)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r5, R::R0, 100)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, r5,
        Shift::Lsl, 4, false)?)?;
    c.emit(store(R::R0, r7, 0)?)?;
    c.call(0x8e98)?;
    c.literal(R::R0, 0x5f9c)?;
    c.literal(R::R2, 0x5fa0)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(mov_register(C::Ne, R::R2, R::R1))?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r4, R::R0, 16384)?)?;
    c.emit(store(R::R2, r7, 24)?)?;
    c.emit(add(r6, r4, 120)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R2, r6,
        Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.call(0xe2248)?;
    c.emit(load(R::R0, r7, 24)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.emit(mov(r9, 0)?)?;
    c.call(0x31ec)?;
    c.emit(load(R::R0, r7, 24)?)?;
    c.literal(R::R1, 0x5fa4)?;
    c.emit(store(r9, R::SP, 16)?)?;
    c.emit(transfer_multiple(C::Al, false, R::SP,
        r7.bit() | r8.bit() | r9.bit(), BlockMode::IncrementBefore, false)?)?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.emit(mov_register(C::Al, r3, r5))?;
    c.emit(store(R::R1, R::SP, 0)?)?;
    c.emit(add(R::R1, R::R0, 120)?)?;
    c.call(0x996c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (wide_saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5f9c)?;
    for datum in [0x0010_36c0, 0x0000_0048, 0x0000_5a5c] {
        c.emit(datum)?;
    }
    c.check(0x5fa8)?;
    Ok(())
}
