// SPDX-License-Identifier: GPL-3.0-only
//! Venus object pool allocation and guarded deallocation routines.
//! Hardware calls use symbolic branch addresses; machine words are generated
//! by public ARM32 instruction encoders.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2c44 };
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;
    let keep = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let ret = r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit();

    // Allocate an object, optionally accepting externally initialized backing
    // memory, then initialize its work queue; failures release owned resources.
    c.emit(push_registers(C::Al, keep)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x2c74)?;
    c.emit(add(R::R0, R::R1, 16)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.check(0x2c68)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2c74)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 16)?)?;
    c.call(0x57d4)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2cbc)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(add(R::R1, R::R0, 4)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(r4, R::R0, 8)?)?;
    c.emit(store(r4, R::R0, 12)?)?;
    c.emit(mov(R::R0, 128)?)?;
    c.call(0x58d8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2cc4)?;
    c.emit(mov(r4, 1)?)?;
    c.branch(C::Al, 0x2c68)?;
    c.check(0x2cbc)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.branch(C::Al, 0x2c68)?;
    c.check(0x2cc4)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 256)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x5454)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r4, R::R0, 1)?)?;
    c.branch(C::Al, 0x2c68)?;
    c.check(0x2ce0)?;

    // Dispose of allocated state only after its reference reaches zero.
    c.emit(push_registers(C::Al, keep)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(load(r4, R::R0, 0)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R0, r4, 12)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2d04)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R0, r4, 4)?)?;
    c.call(0x4f1c)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x4f1c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, keep,
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x5838)?;
    c.check(0x2d20)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_runtime_prefix() { assert!(extend(&mut Vec::new()).is_err()); }
}
