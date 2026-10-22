// SPDX-License-Identifier: GPL-3.0-only
//! Venus two-resource setup and rollback paths.
//! Firmware service addresses and literal pointers are structured data;
//! instructions are emitted via the checked ARM A32 encoders.
use arm32::{
    mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x29b4 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    // Create resource-bearing context using the public ARM32 ABI, and
    // initialize the requested event-service descriptors one at a time.
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.emit(mov(R::R0, 32)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.call(0x57d4)?;
    c.emit(load(r6, r4, 0)?)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Eq, 0x2ab8)?;
    c.emit(load(R::R0, r7, 8)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(add(r5, r6, 8)?)?;
    c.emit(store(R::R0, r6, 0)?)?;
    c.emit(load(R::R0, r7, 12)?)?;
    c.emit(store(R::R0, r6, 4)?)?;
    c.emit(transfer_multiple(C::Al, true, r7,
        R::R0.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(load(R::R1, r7, 16)?)?;
    c.emit(store(r5, R::SP, 0)?)?;
    c.emit(store(r3, R::SP, 4)?)?;
    c.emit(mov(r3, 1)?)?;
    c.emit(mov(r5, 1)?)?;
    c.call(0x5b60)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x2abc)?;
    c.emit(add(r8, r6, 12)?)?;
    c.emit(mov(R::R0, 16)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r9, 0)?)?;
    c.emit(mov_register(C::Al, R::R1, r8))?;
    c.call(0x5454)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x2a98)?;
    c.emit(load(R::R0, r6, 8)?)?;
    c.emit(load(r3, r6, 12)?)?;
    c.literal(R::R1, 0x2af8)?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.emit(store(r9, R::SP, 0)?)?;
    c.call(0x5d5c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x2a98)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(R::R0, r6, 24)?)?;
    c.emit(store(R::R1, r6, 28)?)?;
    c.emit(add(R::R1, r6, 20)?)?;
    c.emit(mov(R::R0, 128)?)?;
    c.call(0x58d8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x2a98)?;
    c.emit(add(R::R1, r6, 16)?)?;
    c.emit(mov(R::R0, 256)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x5454)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2ac8)?;
    c.check(0x2a98)?;

    // Release partial resources in reverse allocation order. Preserve the
    // context object ownership invariant on error paths.
    c.emit(load(R::R0, r6, 8)?)?;
    c.call(0x4f1c)?;
    c.emit(load(R::R0, r6, 20)?)?;
    c.call(0x5838)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x5838)?;
    c.emit(store(r9, r4, 0)?)?;
    c.branch(C::Al, 0x2abc)?;
    c.check(0x2ab8)?;
    c.emit(mov(r5, 1)?)?;
    c.check(0x2abc)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() |
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x2ac8)?;
    c.emit(load(R::R0, r7, 20)?)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2abc)?;
    c.emit(load(R::R0, r8, 0)?)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(mov(R::R1, 256)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(store(r5, R::SP, 0)?)?;
    c.call(0x553c)?;
    c.branch(C::Al, 0x2abc)?;
    c.emit(0x0000_2afc)?; // Message descriptor link
    c.check(0x2afc)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_contiguous_input_is_rejected() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
