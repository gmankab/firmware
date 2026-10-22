// SPDX-License-Identifier: GPL-3.0-only
//! Venus callback registration, lightweight trampoline and process startup.
//! The branch targets are device-supplied service addresses. Executable A32
//! words are produced by a checked encoder, never copied from the reference.
use arm32::{
    bx, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3084 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let preserved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, preserved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.call(0x5ed8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(store(r6, R::R0, 4)?)?;
    c.emit(store(r5, R::R0, 8)?)?;
    c.emit(store(r4, R::R0, 12)?)?;
    c.emit(mov(R::R1, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        preserved, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x205c)?;
    c.check(0x30bc)?;

    // Short entrypoints for the common service runner and queue commands.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x9140)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(transfer_multiple(C::Al, false, R::R0,
        R::R1.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(load(R::R1, R::R1, 4)?)?;
    c.emit(bx(C::Al, R::R1))?;
    c.check(0x30f0)?;

    // Start the device runtime with a named bootstrap descriptor.
    c.emit(mov_register(C::Al, R::R0, R::R0))?;
    c.call(0xe22cc)?;
    c.emit(mov_register(C::Al, R::R0, R::R0))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov_register(C::Al, R::R11, R::R1))?;
    c.emit(mov_register(C::Al, R::R2, R::R1))?;
    c.literal(R::R0, 0x3114)?;
    c.call(0x3118)?;
    c.branch(C::Al, 0x3110)?;
    c.emit(0x0000_1090)?; // Runtime bootstrap descriptor
    c.check(0x3118)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_parent_block_fails() { assert!(extend(&mut Vec::new()).is_err()); }
}
