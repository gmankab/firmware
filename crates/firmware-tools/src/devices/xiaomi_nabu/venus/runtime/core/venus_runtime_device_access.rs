// SPDX-License-Identifier: GPL-3.0-only
//! Small device service accessors, optional resource release, and parameter reset.
use arm32::{
    bx, data_immediate, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6188 };
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(r4, R::R0, 12)?)?;
    c.call(0x6ac4)?;
    c.emit(load(R::R0, r4, 92)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x61ac)?;
    c.call(0x4f1c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x61b4)?;
    for _ in 0..2 {
        c.emit(mov(R::R0, 0)?)?;
        c.emit(bx(C::Al, R::LR))?;
    }
    c.emit(mov(R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 65536)?)?;
    c.emit(store(R::R0, R::R1, 4)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for _ in 0..2 {
        c.emit(mov(R::R0, 0)?)?;
        c.emit(bx(C::Al, R::LR))?;
    }
    c.check(0x61e8)?;
    Ok(())
}
