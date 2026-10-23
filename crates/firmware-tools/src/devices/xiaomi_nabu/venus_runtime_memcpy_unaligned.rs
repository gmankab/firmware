// SPDX-License-Identifier: GPL-3.0-only
//! Word-at-a-time copies for unaligned destinations. Three public ARM32
//! shift-and-merge configurations (8/16/24 bit) share one typed generator.
use arm32::{
    data_immediate, data_register, mov_shifted_register, push_registers,
    transfer_byte, transfer_byte_postindexed, transfer_word_postindexed,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, store, sub};

fn read_and_advance_words(c: &mut Code<'_>, first: i32, left_shift: u8) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let inverse = 32 - left_shift;
    // The pointer already identifies a partial preceding word. Blend it
    // with four following words while emitting four aligned destination words.
    for (index, offset) in (0..4).map(|i| (i, first + i * 4)) {
        let next = if index % 2 == 0 { R::LR } else { R::R12 };
        let old = if index % 2 == 0 { R::R12 } else { R::LR };
        c.emit(load(next, R::R1, offset)?)?;
        if index == 0 {
            c.emit(sub(R::R2, R::R2, 16)?)?;
        }
        if index == 3 { c.emit(add(R::R1, R::R1, 16)?)?; }
        c.emit(mov_shifted_register(C::Al, r4, next, Shift::Lsl, left_shift)?)?;
        c.emit(data_register(C::Al, Op::Orr, old, r4, old,
            Shift::Lsr, inverse, false)?)?;
        c.emit(store(old, r3, index * 4)?)?;
    }
    c.emit(add(r3, r3, 16)?)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x47ac };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    c.emit(push_registers(C::Al, r4.bit() | R::LR.bit())?)?;
    c.emit(cmp(R::R2, 32)?)?;
    c.branch(C::Cc, 0x4928)?;
    c.emit(data_immediate(C::Al, Op::And, R::R12, r3, 3)?)?;
    c.emit(cmp(R::R12, 3)?)?;
    c.branch(C::Eq, 0x48c4)?;
    c.emit(cmp(R::R12, 2)?)?;
    c.branch(C::Eq, 0x4850)?;
    c.emit(cmp(R::R12, 1)?)?;
    c.branch(C::Ne, 0x4928)?;

    // Destination misalignment of one byte: prime the output with three
    // individual bytes and then combine source words using 8/24-bit shifts.
    c.check(0x47d4)?;
    c.emit(load(R::R12, R::R1, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::LR, R::R1, 1)?)?;
    c.emit(sub(R::R2, R::R2, 3)?)?;
    c.emit(transfer_byte(C::Al, false, R::R12, r3, 0)?)?;
    c.emit(transfer_byte(C::Al, false, R::LR, r3, 1)?)?;
    c.emit(transfer_byte(C::Al, true, R::LR, R::R1, 2)?)?;
    c.emit(add(R::R1, R::R1, 3)?)?;
    c.emit(transfer_byte(C::Al, false, R::LR, r3, 2)?)?;
    c.emit(add(r3, r3, 3)?)?;
    c.check(0x47f8)?;
    c.emit(cmp(R::R2, 17)?)?;
    c.branch(C::Cc, 0x4928)?;
    read_and_advance_words(&mut c, 1, 8)?;
    c.branch(C::Al, 0x47f8)?;

    // Two-byte misalignment: prime two bytes, then use 16/16-bit merging.
    c.check(0x4850)?;
    c.emit(load(R::R12, R::R1, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::LR, R::R1, 1)?)?;
    c.emit(sub(R::R2, R::R2, 2)?)?;
    c.emit(add(R::R1, R::R1, 2)?)?;
    c.emit(transfer_byte(C::Al, false, R::R12, r3, 0)?)?;
    c.emit(transfer_byte(C::Al, false, R::LR, r3, 1)?)?;
    c.emit(add(r3, r3, 2)?)?;
    c.check(0x486c)?;
    c.emit(cmp(R::R2, 18)?)?;
    c.branch(C::Cc, 0x4928)?;
    read_and_advance_words(&mut c, 2, 16)?;
    c.branch(C::Al, 0x486c)?;

    // Three-byte misalignment: emit one byte, then merge 24/8-bit spans.
    c.check(0x48c4)?;
    c.emit(transfer_word_postindexed(C::Al, true, R::R12, R::R1, 1)?)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, R::R12, r3, 1)?)?;
    c.check(0x48d0)?;
    c.emit(cmp(R::R2, 19)?)?;
    c.branch(C::Cc, 0x4928)?;
    read_and_advance_words(&mut c, 3, 24)?;
    c.branch(C::Al, 0x48d0)?;
    c.check(0x4928)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_a_gap_before_unaligned_copy() { assert!(extend(&mut Vec::new()).is_err()); }
}
