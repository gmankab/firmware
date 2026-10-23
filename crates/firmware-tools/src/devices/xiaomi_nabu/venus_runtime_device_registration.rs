// SPDX-License-Identifier: GPL-3.0-only
//! Reusable ARM32 service list registration and a resource-list release loop.
use arm32::{
    bx, data_register, mov_register, push_registers, transfer_multiple,
    transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x631c };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved_short = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let ret_short = (saved_short & !R::LR.bit()) | R::PC.bit();
    // Drain a list until the final callback reports the starting object.
    c.emit(push_registers(C::Al, saved_short)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.call(0x6dec)?;
    c.check(0x632c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x6e40)?;
    c.call(0x6e88)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0,
        r4, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, ret_short,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x6e60)?;
    c.branch(C::Al, 0x632c)?;
    c.check(0x634c)?;
    // Register a second service namespace with a named static object, using
    // exactly the same public linked-list primitives as the earlier block.
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.literal(R::R2, 0x63b0)?;
    c.literal(r5, 0x63b4)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x846f8)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, r6, 0)?)?;
    c.branch(C::Ne, 0x63a8)?;
    c.literal(R::R0, 0x63b8)?;
    c.emit(mov_register(C::Al, r7, r5))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false,
        R::R0, r7, 16)?)?;
    c.literal(R::R0, 0x63bc)?;
    c.emit(store(R::R0, r5, 20)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(store(r5, r5, 24)?)?;
    c.call(0x84804)?;
    c.emit(store(r7, r4, 0)?)?;
    c.check(0x63a8)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    for literal in [0x0010_37a0, 0x0010_3834, 0xda1f_da1f, 0x000e_344c] {
        c.emit(literal)?;
    }
    c.check(0x63c0)?;
    for _ in 0..2 {
        c.emit(mov(R::R0, 0)?)?;
        c.emit(bx(C::Al, R::LR))?;
    }
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.branch(C::Al, 0x84838)?;
    c.check(0x63d8)?;
    Ok(())
}
