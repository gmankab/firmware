// SPDX-License-Identifier: GPL-3.0-only
//! Bounded formatting-buffer entry point. The stream descriptor and stack
//! scratch array are built from named fields and typed A32 instructions.
use arm32::{
    data_immediate, data_register, mov_register, push_registers,
    transfer_byte, transfer_multiple, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4550 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | R::LR.bit();
    let ret = saved & !R::LR.bit() | R::PC.bit();

    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(sub(R::SP, R::SP, 152)?)?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.literal(R::R1, 0x460c)?;
    c.emit(mov_register(C::Al, r6, R::R2))?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.emit(mov_register(C::Al, R::R0, R::SP))?;
    c.emit(mov(R::R2, 144)?)?;
    c.emit(mov_register(C::Al, r4, r3))?;
    c.call(0x465c)?;

    // Check the requested output bound. Invalid nonzero bounds use the
    // diagnostics path while a zero bound addresses the last scratch byte.
    c.emit(sub(R::R0, r5, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmn, R::R0, R::R0, 0x8000_0001)?)?;
    c.branch(C::Cc, 0x45ac)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x45a4)?;
    c.call(0x4a6c)?;
    c.emit(mov(R::R1, 75)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(add(R::SP, R::SP, 152)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x45a4)?;
    c.emit(add(r7, R::SP, 151)?)?;
    c.emit(mov(r5, 1)?)?;
    c.check(0x45ac)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R0, r7, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r5, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r5, R::R0))?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7, r5, Shift::Lsl, 0, false)?)?;
    c.emit(store(r5, R::SP, 48)?)?;
    for (register, offset) in [(r7, 20), (r7, 44), (R::R0, 16), (R::R0, 28)] {
        c.emit(store(register, R::SP, offset)?)?;
    }
    c.emit(mov_register(C::Al, R::R0, R::SP))?;
    c.call(0x3410)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x4604)?;
    c.emit(load(R::R1, R::SP, 16)?)?;
    c.emit(load(R::R2, R::SP, 20)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Sub, R::R2, R::R2, 1)?)?;
    c.emit(transfer_byte(C::Al, false, R::R1, R::R2, 0)?)?;
    c.check(0x4604)?;
    c.emit(add(R::SP, R::SP, 152)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_2b40)?; // Runtime formatter descriptor address
    c.check(0x4610)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_preceding_code() { assert!(extend(&mut Vec::new()).is_err()); }
}
