// SPDX-License-Identifier: GPL-3.0-only
//! Venus callback and object-lifetime interfaces, represented as A32 code.
//! Service calls preserve the ARM32 calling convention and do not embed
//! original executable words.
use arm32::{
    branch_exchange_with_link, bx, data_immediate, mov_register, push_registers,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2afc };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r7 = R::new(7)?;
    // Invoke a runtime-owned callback descriptor through its function pointer.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R0, R::R1, 4)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R2)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x2b18)?;

    // Free the three tracked external resources before the owner object.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(load(r5, r4, 0)?)?;
    for offset in [8, 20, 16] {
        c.emit(load(R::R0, r5, offset)?)?;
        c.call(0x4f1c)?;
    }
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x5838)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2b58)?;

    // Ask the event queue to unregister a callback from its resource object.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 12)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(R::new(3)?, 0)?)?;
    c.emit(store(R::R1, R::SP, 0)?)?;
    c.emit(mov(R::R1, 256)?)?;
    c.call(0x553c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::SP, R::R11))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x2b8c)?;

    // Allocate a 4-byte context object and attach its allocation result.
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 4)?)?;
    c.call(0x57d4)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(mov(R::R0, 16)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x58d8)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2bcc)?;

    // Only live object pointers enter the common deallocation function.
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x4f1c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x5838)?;
    c.check(0x2bf4)?;

    // Thin fallible runtime interface shims that preserve NULL semantics.
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.branch(C::Al, 0x5958)?;
    c.check(0x2c04)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.call(0x599c)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x2c34)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.branch(C::Al, 0x59e8)?;
    c.check(0x2c44)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_incomplete_prefix_accepted() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
