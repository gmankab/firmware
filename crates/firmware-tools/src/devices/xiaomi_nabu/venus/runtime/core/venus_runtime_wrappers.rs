// SPDX-License-Identifier: GPL-3.0-only
//! Venus bank-disable routine, object initialization and service wrappers.
//! All instruction bytes are synthesized from typed ARM32 operations.
use arm32::{
    data_immediate, data_register_shifted_by_register, mov_register, push_registers,
    transfer_multiple, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2808 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;

    // Clear a bit in the selected interrupt bank after bounds checks.
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.literal(r8, 0x287c)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(r7, 0)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov(R::R1, 183)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(cmp(r5, 16)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r7, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.call(0x17b0)?;
    c.emit(cmp(r4, 6)?)?;
    c.emit(mov(R::R1, 184)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.emit(data_immediate(C::Cc, Op::Mov, r6, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x17b0)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov(R::R1, 328)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.call(0x17b0)?;
    c.literal(R::R0, 0x2880)?;
    c.emit(mov(R::R2, 1)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, r4, Shift::Lsl, 2)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Bic,
        R::R1, R::R1, R::R2, Shift::Lsl, r5, false)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R1, R::R0, r4, Shift::Lsl, 2)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() |
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_27f9)?; // Error string address
    c.emit(0x0010_3484)?; // Interrupt bank table
    c.check(0x2884)?;

    // Zero a shared mask array and query its first resource pointer.
    c.literal(R::R0, 0x2894)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 24)?)?;
    c.branch(C::Al, 0x31ec)?;
    c.emit(0x0010_3484)?; // Shared mask array
    c.emit(load(R::R0, R::R0, 72)?)?;
    c.branch(C::Al, 0x2b58)?;
    c.check(0x28a0)?;

    // Create and initialize an object, publishing its pointer only if the
    // common runtime allocator confirms success.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.literal(R::R2, 0x28e8)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov(R::R1, 15)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(r6, 0)?)?;
    c.call(0x2c44)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r6, R::R0, 1)?)?;
    c.emit(arm32::transfer_word(C::Eq, false, r4, r5, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_28dc)?; // Error message id
    c.check(0x28ec)?;

    // Free a resource object and clear its state pointer.
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.literal(R::R2, 0x2920)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(mov(R::R1, 29)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x2ce0)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 4)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_28dc)?;
    c.check(0x2924)?;

    // Memory-status service forwarding wrappers.
    c.emit(transfer_multiple(C::Al, false, R::R0,
        R::R1.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(arm32::bx(C::Al, R::LR))?;
    for (start, target) in [
        (0x292c, 0x2d20), (0x293c, 0x2e40), (0x294c, 0x2dbc),
    ] {
        c.check(start)?;
        c.emit(load(R::R2, R::R0, 0)?)?;
        c.emit(load(R::R1, R::R0, 4)?)?;
        c.emit(mov_register(C::Al, R::R0, R::R2))?;
        c.branch(C::Al, target)?;
    }
    c.check(0x295c)?;

    // Format a 2-word descriptor for the shared event queue.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2984)?;
    c.emit(store(R::R0, R::SP, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, R::SP))?;
    c.branch(C::Al, 0x2988)?;
    c.emit(mov(R::R0, 0)?)?;
    c.call(0x4e34)?;
    c.literal(R::R1, 0x29ac)?;
    c.emit(mov(R::R0, 128)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x58d8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::SP, R::R11))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x0010_349c)?; // Runtime event-queue pointer
    c.branch(C::Al, 0x4f08)?;
    c.check(0x29b4)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_boot_block_is_rejected() { assert!(extend(&mut Vec::new()).is_err()); }
}
