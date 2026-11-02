// SPDX-License-Identifier: GPL-3.0-only
//! Minimal Venus resource accessors and interface-registration stubs.
//! The registration block uses a named magic value, pointers to runtime
//! objects and checked ARM32 instructions instead of opaque instruction bytes.
use arm32::{
    bx, mov_register, push_registers, transfer_multiple,
    transfer_word_preindexed_writeback, BlockMode, Condition as C, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5fa8 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    // Query and translate an object runtime pointer to its backing data.
    c.emit(load(R::R0, R::R0, 24)?)?;
    c.branch(C::Al, 0x9738)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x5fc4)?;
    // Register a platform service with its initial structure, diagnostic
    // identifier and double-linked list sentinels.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.literal(R::R2, 0x6028)?;
    c.literal(r5, 0x602c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x846f8)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Ne, 0x6020)?;
    c.literal(R::R0, 0x6030)?;
    c.emit(mov_register(C::Al, r7, r5))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false,
        R::R0, r7, 16)?)?;
    c.literal(R::R0, 0x6034)?;
    c.emit(store(R::R0, r5, 20)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(store(r5, r5, 24)?)?;
    c.call(0x84804)?;
    c.emit(store(r7, r4, 0)?)?;
    c.check(0x6020)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    for literal in [0x0010_3708, 0x0010_3780, 0xda1f_da1f, 0x000e_33fc] {
        c.emit(literal)?;
    }
    c.check(0x6038)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.branch(C::Al, 0x84838)?;
    c.check(0x6050)?;
    Ok(())
}
