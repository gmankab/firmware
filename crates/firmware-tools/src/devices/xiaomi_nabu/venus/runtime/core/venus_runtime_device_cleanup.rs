// SPDX-License-Identifier: GPL-3.0-only
//! Conditional resource release and small device-state service operations.
use arm32::{
    bx, data_immediate, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x653c };
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(r4, R::R0, 12)?)?;
    for (property, release, next) in [
        (120, 0x4f1c, 0x655c),
        (112, 0x5838, 0x656c),
        (100, 0x5838, 0x657c),
    ] {
        c.emit(load(R::R0, r4, property)?)?;
        c.emit(cmp(R::R0, 0)?)?;
        c.branch(C::Eq, next)?;
        c.call(release)?;
    }
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
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
    c.check(0x65b8)?;
    Ok(())
}
