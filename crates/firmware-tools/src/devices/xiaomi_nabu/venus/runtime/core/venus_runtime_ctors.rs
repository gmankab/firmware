// SPDX-License-Identifier: GPL-3.0-only
//! Venus runtime constructor traversal and shutdown service wrappers.
//! Includes named, zero-initialized constructor table boundaries, not copied
//! firmware instructions or opaque opcode arrays.
use arm32::{
    branch_exchange_with_link, bx, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word_postindexed, transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3118 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    // Pass runtime argument-vector contents to the entrypoint after visiting
    // the zero-initialized constructor table, if any entries are present.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() |
        r9.bit() | r10.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r5,
        r6, Shift::Lsl, 2, false)?)?;
    c.emit(add(r7, R::R0, 4)?)?;
    c.call(0x316c)?;
    c.literal(r8, 0x3164)?;
    c.literal(r9, 0x3168)?;
    c.check(0x313c)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0,
        r8, r9, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x3150)?;
    c.emit(transfer_word_postindexed(C::Al, true, R::R0, r8, 4)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R0)?)?;
    c.branch(C::Al, 0x313c)?;
    c.check(0x3150)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.emit(mov_register(C::Al, R::R2, r7))?;
    c.emit(branch_exchange_with_link(C::Al, r4)?)?;
    c.call(0x31a0)?;
    // Empty constructor list: both boundaries are 0 in this reference.
    c.emit(0)?;
    c.emit(0)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x3170)?;

    // Destructors are walked backward with the decrement-before addressing
    // mode and called through BLX. The table is currently empty.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r7.bit() | R::LR.bit())?)?;
    c.literal(r4, 0x3198)?;
    c.literal(r5, 0x319c)?;
    c.check(0x317c)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0,
        r4, r5, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x3190)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, R::R0, r4, -4)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R0)?)?;
    c.branch(C::Al, 0x317c)?;
    c.check(0x3190)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r7.bit() | R::LR.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x31bc)?;
    c.emit(0)?;
    c.emit(0)?;
    c.check(0x31a0)?;

    // Exit routine: invoke all finalizers, release runtime state, then forward
    // the final status to the platform-owned shutdown service.
    c.emit(push_registers(C::Al, r4.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.call(0x31bc)?;
    c.call(0x3170)?;
    c.call(0x4cb0)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x33f0)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x31c0)?;

    // Forward a four-argument diagnostic record to the variadic formatter.
    c.emit(sub(R::SP, R::SP, 4)?)?;
    c.emit(push_registers(C::Al, r7.bit() | R::LR.bit())?)?;
    c.emit(sub(R::SP, R::SP, 4)?)?;
    c.emit(store(R::new(3)?, R::SP, 12)?)?;
    c.emit(add(R::new(3)?, R::SP, 12)?)?;
    c.emit(store(R::new(3)?, R::SP, 0)?)?;
    c.call(0x3fd4)?;
    c.emit(add(R::SP, R::SP, 4)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r7.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(add(R::SP, R::SP, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x31ec)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn previous_region_is_required() { assert!(extend(&mut Vec::new()).is_err()); }
}
