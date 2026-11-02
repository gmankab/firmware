// SPDX-License-Identifier: GPL-3.0-only
//! Small bounded diagnostic string formatter with integer conversions.
//! This routine handles percent-escape scanning, width and precision fields,
//! string and numeric arguments, and copies into a bounded destination.
//! All machine instructions come from the generic typed ARM A32 encoders.
use arm32::{
    data_immediate, data_register,
    mov_register, mov_shifted_register, push_registers,
    transfer_byte, transfer_byte_postindexed, transfer_byte_register,
    transfer_byte_preindexed_writeback, transfer_multiple,
    transfer_word_postindexed, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3fd4 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | r10.bit() | R::R11.bit() | R::LR.bit();

    // Set up a bounded formatting buffer and find the first usable specifier.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(sub(R::SP, R::SP, 44)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(add(R::R0, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, r5, r3))?;
    c.emit(add(R::R0, R::R0, 40)?)?;
    c.emit(store(R::R0, R::SP, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.check(0x3ff4)?;
    c.emit(transfer_byte(C::Al, true, r3, R::R0, 0)?)?;
    c.emit(cmp(r3, 37)?)?;
    c.branch(C::Eq, 0x4030)?;
    c.emit(cmp(r3, 0)?)?;
    c.branch(C::Ne, 0x4090)?;
    c.emit(sub(r8, R::R1, 1)?)?;
    c.emit(load(R::R1, R::SP, 0)?)?;
    c.emit(mov(r9, 0)?)?;
    c.emit(mov(r7, 0)?)?;
    c.check(0x4018)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R2, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x4098)?;
    c.check(0x4024)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_byte_register(C::Al, false, R::R0, r4, r7, Shift::Lsl, 0)?)?;
    c.branch(C::Al, 0x4084)?;

    // Skip validated decimal-width fields and decide which public formatting
    // conversion families are supported by this bounded wrapper.
    c.check(0x4030)?;
    c.emit(mov_register(C::Al, r3, R::R0))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R12, R::R0, 1)?)?;
    c.emit(sub(R::LR, R::R12, 48)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::LR, R::LR, 255)?)?;
    c.emit(cmp(R::LR, 10)?)?;
    c.branch(C::Cc, 0x4030)?;
    c.emit(cmp(R::R12, 108)?)?;
    c.emit(transfer_byte(C::Eq, true, R::R12, r3, 2)?)?;
    c.emit(cmp(R::R12, 100)?)?;
    c.branch(C::Eq, 0x4090)?;
    c.emit(cmp(R::R12, 120)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R12, 117)?)?;
    c.branch(C::Eq, 0x4090)?;
    c.emit(cmp(R::R12, 115)?)?;
    c.emit(transfer_byte(C::Eq, true, r3, r3, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, R::R0, r3, 37)?)?;
    c.branch(C::Eq, 0x4090)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, r3, r5))?;
    c.call(0x4550)?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.check(0x4084)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(add(R::SP, R::SP, 44)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        saved & !R::LR.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x4090)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.branch(C::Al, 0x3ff4)?;

    // Copy plain text until a percent conversion or end of input.
    c.check(0x4098)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R2, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x4024)?;
    c.emit(cmp(R::R0, 37)?)?;
    c.branch(C::Eq, 0x40cc)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, r8, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x40c4)?;
    c.emit(transfer_byte_register(C::Al, false, R::R0, r4, r7, Shift::Lsl, 0)?)?;
    c.emit(add(r7, r7, 1)?)?;
    c.emit(add(R::R2, R::R2, 1)?)?;
    c.branch(C::Al, 0x4098)?;
    c.check(0x40c4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x4024)?;

    // Width may start with a zero fill indicator and is accumulated in
    // decimal, while the conversion's argument pointer advances separately.
    c.check(0x40cc)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, r8, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x4024)?;
    c.emit(mov_register(C::Al, r10, R::R2))?;
    c.emit(mov(R::R11, 32)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R0, r10, 1)?)?;
    c.emit(cmp(R::R0, 48)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Eq, true, R::R0, R::R2, 2)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R11, R::R0, 48)?)?;
    c.emit(mov_register(C::Eq, r10, R::R2))?;
    c.check(0x40f4)?;
    c.emit(sub(R::R2, R::R0, 48)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R2, R::R2, 255)?)?;
    c.emit(cmp(R::R2, 9)?)?;
    c.branch(C::Hi, 0x411c)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, r6, r6, Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 255)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, R::R2, Shift::Lsl, 1, false)?)?;
    c.emit(sub(r6, R::R0, 48)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R0, r10, 1)?)?;
    c.branch(C::Al, 0x40f4)?;
    c.check(0x411c)?;
    c.emit(data_immediate(C::Al, Op::And, R::R2, R::R0, 255)?)?;
    c.emit(cmp(R::R2, 108)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Eq, true, R::R0, r10, 1)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 255)?)?;
    c.emit(cmp(R::R0, 100)?)?;
    c.branch(C::Eq, 0x41b8)?;
    c.emit(cmp(R::R0, 120)?)?;
    c.branch(C::Eq, 0x4188)?;
    c.emit(cmp(R::R0, 117)?)?;
    c.branch(C::Eq, 0x41dc)?;
    c.emit(cmp(R::R0, 115)?)?;
    c.branch(C::Ne, 0x4024)?;

    // String argument: substitute an explicit missing-string literal and
    // ensure every output byte fits within the caller's bound.
    c.emit(transfer_word_postindexed(C::Al, true, R::R0, r5, 4)?)?;
    c.literal(R::R2, 0x4258)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(mov_register(C::Eq, R::R0, R::R2))?;
    c.check(0x415c)?;
    c.emit(transfer_byte(C::Al, true, R::R2, R::R0, 0)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Ne, 0x4170)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.branch(C::Al, 0x41e8)?;
    c.check(0x4170)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, r8, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x4168)?;
    c.emit(transfer_byte_register(C::Al, false, R::R2, r4, r7, Shift::Lsl, 0)?)?;
    c.emit(add(r7, r7, 1)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.branch(C::Al, 0x415c)?;

    // Convert a 32-bit argument to lowercase hexadecimal by extracting its
    // low nibble and writing output bytes backwards from stack memory.
    c.check(0x4188)?;
    c.emit(transfer_word_postindexed(C::Al, true, R::R12, r5, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.check(0x4190)?;
    c.emit(data_immediate(C::Al, Op::And, R::R2, R::R12, 15)?)?;
    c.emit(add(r3, R::R2, 87)?)?;
    c.emit(cmp(R::R2, 10)?)?;
    c.emit(data_immediate(C::Cc, Op::Orr, r3, R::R2, 48)?)?;
    c.emit(mov_shifted_register(C::Al, R::R2, R::R12, Shift::Lsr, 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r9, R::R12, Shift::Lsr, 4, false)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r3, R::R0, -1)?)?;
    c.emit(mov_register(C::Al, R::R12, R::R2))?;
    c.branch(C::Ne, 0x4190)?;
    c.branch(C::Al, 0x41e4)?;

    // Signed and unsigned decimal arguments use the shared division helper.
    c.check(0x41b8)?;
    c.emit(transfer_word_postindexed(C::Al, true, R::R0, r5, 4)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ge, 0x41e0)?;
    c.emit(data_immediate(C::Al, Op::Rsb, R::R0, R::R0, 0)?)?;
    c.call(0x425c)?;
    c.emit(load(R::R1, R::SP, 0)?)?;
    c.emit(mov(R::R2, 45)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, R::R2, R::R0, -1)?)?;
    c.branch(C::Al, 0x41e8)?;
    c.check(0x41dc)?;
    c.emit(transfer_word_postindexed(C::Al, true, R::R0, r5, 4)?)?;
    c.check(0x41e0)?;
    c.call(0x425c)?;
    c.check(0x41e4)?;
    c.emit(load(R::R1, R::SP, 0)?)?;

    // Copy left padding and converted result, enforcing both the requested
    // field width and the actual output limit.
    c.check(0x41e8)?;
    c.emit(data_register(C::Al, Op::Sub, R::R12, R::R1, R::R0, Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x4240)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r6, R::R12, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x4218)?;
    c.emit(data_register(C::Al, Op::Sub, r3, r8, r7, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R2, r6, R::R12, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Cc, R::R2, r3))?;
    c.emit(mov(r3, 0)?)?;
    c.check(0x420c)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Lt, 0x4248)?;
    c.emit(load(R::R1, R::SP, 0)?)?;
    c.check(0x4218)?;
    c.emit(data_register(C::Al, Op::Sub, R::R2, r8, r7, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R12, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Cc, R::R12, R::R2))?;
    c.emit(data_register(C::Al, Op::Add, r3, R::R0, R::R12, Shift::Lsl, 0, false)?)?;
    c.check(0x4228)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r3, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x4240)?;
    c.emit(transfer_byte_postindexed(C::Al, true, R::R2, R::R0, 1)?)?;
    c.emit(transfer_byte_register(C::Al, false, R::R2, r4, r7, Shift::Lsl, 0)?)?;
    c.emit(add(r7, r7, 1)?)?;
    c.branch(C::Al, 0x4228)?;
    c.check(0x4240)?;
    c.emit(add(R::R2, r10, 1)?)?;
    c.branch(C::Al, 0x4018)?;
    c.check(0x4248)?;
    c.emit(transfer_byte_register(C::Al, false, R::R11, r4, r7, Shift::Lsl, 0)?)?;
    c.emit(add(r3, r3, 1)?)?;
    c.emit(add(r7, r7, 1)?)?;
    c.branch(C::Al, 0x420c)?;
    c.emit(0x000e_2938)?; // Pointer to diagnostic fallback string.
    c.check(0x425c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_previous_formatter() { assert!(extend(&mut Vec::new()).is_err()); }
}
