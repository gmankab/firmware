// SPDX-License-Identifier: GPL-3.0-only
//! Integer formatting and `%n` stores in the Venus diagnostic printer.
//! The number-conversion paths use public ARM32 signed arithmetic and typed
//! byte-store operations. No reference opcode bytes are included.
use arm32::{
    data_immediate, data_immediate_with_flags, data_register, mov_register,
    mov_shifted_register, store_halfword, transfer_byte_preindexed_writeback,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x39d4 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r6 = R::new(6)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;

    // `%n` stores: 16-, 8-, 32- and 64-bit integer counts.
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(store_halfword(C::Al, r4, R::R0, 0)?)?;
    c.branch(C::Al, 0x35c0)?;
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r4, R::R0, 0)?)?;
    c.branch(C::Al, 0x35c0)?;
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(store(r4, R::R0, 0)?)?;
    c.branch(C::Al, 0x35c0)?;
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, r4, Shift::Asr, 31)?)?;
    c.emit(store(r4, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, 4)?)?;
    c.branch(C::Al, 0x35c0)?;

    // Format a wide integer as hexadecimal nibbles. Convert both halves of
    // a 64-bit value through a common backwards byte output buffer.
    c.check(0x3a0c)?;
    c.emit(load(R::R0, R::SP, 40)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R11, R::R11, 8)?)?;
    c.emit(mov(r6, 120)?)?;
    c.emit(cmp(R::R0, 8)?)?;
    c.emit(data_immediate(C::Ls, Op::Mov, R::R0, R::R0, 8)?)?;
    c.emit(store(R::R0, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, r10, R::R1))?;
    c.emit(load(r3, R::SP, 104)?)?;
    c.emit(load(R::R2, R::SP, 108)?)?;
    c.literal(R::R1, 0x3fd0)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, r6, 32)?)?;
    c.check(0x3a38)?;
    c.emit(data_register(C::Al, Op::Orr, R::R12, r3, R::R2, Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x3a64)?;
    c.emit(mov_shifted_register(C::Al, R::R12, r3, Shift::Lsr, 4)?)?;
    c.emit(data_immediate(C::Al, Op::And, r3, r3, 15)?)?;
    c.emit(arm32::transfer_byte_register(C::Al, true, r3, R::R1, r3, Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R12, R::R12, R::R2, Shift::Lsl, 28, false)?)?;
    c.emit(mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsr, 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, r3, R::R0, r3, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r3, r10, -1)?)?;
    c.emit(mov_register(C::Al, r3, R::R12))?;
    c.branch(C::Al, 0x3a38)?;
    c.check(0x3a64)?;
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(load(R::R2, R::SP, 108)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R2, Shift::Lsl, 0, true)?)?;
    c.branch(C::Ne, 0x3b98)?;
    c.literal(r9, 0x3fc4)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 12)?)?;
    c.branch(C::Al, 0x3bac)?;

    // Octal encoder uses three bits per digit and preserves the caller's
    // zero-padding and explicit-sign flags.
    c.check(0x3a84)?;
    c.emit(mov_register(C::Al, r10, R::R1))?;
    c.emit(load(R::R1, R::SP, 104)?)?;
    c.emit(load(R::R0, R::SP, 108)?)?;
    c.check(0x3a90)?;
    c.emit(data_register(C::Al, Op::Orr, R::R2, R::R1, R::R0, Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x3ab8)?;
    c.emit(mov_shifted_register(C::Al, R::R2, R::R1, Shift::Lsr, 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 7)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R2, R::R2, R::R0, Shift::Lsl, 29, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 48)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 3)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, R::R1, r10, -1)?)?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.branch(C::Al, 0x3a90)?;
    c.check(0x3ab8)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R11, 8)?)?;
    c.branch(C::Eq, 0x3a74)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(load(R::R1, R::SP, 40)?)?;
    c.literal(r9, 0x3fc4)?;
    c.emit(store(R::R0, R::SP, 12)?)?;
    c.emit(load(R::R0, R::SP, 20)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R0, r10, Shift::Lsl, 0, false)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, R::R1, R::R0))?;
    c.emit(store(R::R1, R::SP, 40)?)?;
    c.branch(C::Al, 0x3bac)?;

    // Signed 64-bit decimal: convert a negative number to its absolute value
    // using RSB/RSBS and RSC, preserving borrow across both 32-bit halves.
    c.check(0x3aec)?;
    c.emit(load(R::R0, R::SP, 108)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ge, 0x3b20)?;
    c.emit(load(R::R1, R::SP, 104)?)?;
    c.literal(r9, 0x3fc4)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsb, R::R1, R::R1, 0, true)?)?;
    c.emit(store(R::R1, R::SP, 104)?)?;
    c.emit(load(R::R1, R::SP, 20)?)?;
    c.emit(data_immediate(C::Al, Op::Rsc, R::R0, R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 108)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(store(R::R0, R::SP, 12)?)?;
    c.branch(C::Al, 0x3b50)?;
    c.check(0x3b20)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, R::R0, R::R11, 2048, true)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R11, 1)?)?;
    c.emit(mov(R::R2, 1)?)?;
    c.emit(mov_register(C::Eq, R::R2, R::R1))?;
    c.emit(cmp(R::R1, 0)?)?;
    c.literal(R::R1, 0x3fc4)?;
    c.emit(store(R::R2, R::SP, 12)?)?;
    c.emit(mov_register(C::Al, r9, R::R1))?;
    c.emit(data_immediate(C::Ne, Op::Add, r9, r9, 2)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, r9, R::R1, 1)?)?;
    c.emit(load(R::R1, R::SP, 20)?)?;
    c.check(0x3b50)?;
    c.emit(mov_register(C::Al, r10, R::R1))?;
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(load(R::R1, R::SP, 108)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x3b7c)?;
    c.check(0x3b64)?;
    c.emit(mov(R::R2, 10)?)?;
    c.emit(mov(r3, 0)?)?;
    c.call(0x8187c)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 48)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, R::R2, r10, -1)?)?;
    c.branch(C::Al, 0x3b5c)?;
    c.check(0x3b7c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x3bac)?;
    c.emit(mov(R::R1, 10)?)?;
    c.call(0x81850)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 48)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, R::R1, r10, -1)?)?;
    c.branch(C::Al, 0x3b7c)?;
    c.check(0x3b98)?;
    c.literal(r9, 0x3fc4)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, R::R0, R::R11, 8, true)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(store(R::R0, R::SP, 12)?)?;
    c.emit(data_register(C::Ne, Op::Add, r9, r9, r6, Shift::Lsr, 4, false)?)?;
    c.check(0x3bac)?;
    c.emit(load(R::R2, R::SP, 40)?)?;
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(load(R::R1, R::SP, 108)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.emit(data_immediate(C::Ge, Op::Bic, R::R11, R::R11, 65536)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Ne, 0x3be0)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 40)?)?;
    c.emit(load(R::R0, R::SP, 20)?)?;
    c.emit(mov_register(C::Al, r10, R::R0))?;
    c.branch(C::Al, 0x3c84)?;
    c.check(0x3be0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_preceding_output_is_rejected() { assert!(extend(&mut Vec::new()).is_err()); }
}
