// SPDX-License-Identifier: GPL-3.0-only
//! Format argument access and widening operations for A32 diagnostic code.
//! A typed function table handles signed/unsigned 8-, 16-, 32- and 64-bit
//! reads. Machine-code fields come from the shared ARM32 encoder.
use arm32::{
    bx, data_immediate, load_halfword, load_signed_byte, load_signed_halfword,
    mov_shifted_register, transfer_byte, transfer_doubleword,
    transfer_word_register, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

/// Read an argument-pointer entry and advance the caller's cursor one word.
fn advance_argument(c: &mut Code<'_>) -> Result<(), String> {
    let r3 = R::new(3)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(add(r3, R::R1, 4)?)?;
    c.emit(store(r3, R::R2, 0)?)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x42e0 };
    let r3 = R::new(3)?;
    c.emit(cmp(R::R1, 20)?)?;
    c.emit(data_immediate(C::Ls, Op::Sub, R::R1, R::R1, 9)?)?;
    c.emit(data_immediate(C::Ls, Op::Cmp, R::R0, R::R1, 9)?)?;
    c.branch(C::Hi, 0x43d8)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r3, R::PC, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::PC, R::R1, r3, Shift::Lsl, 0)?)?;

    // Case labels are addresses rather than encoded instructions.  The
    // compiler-generated table has two aliases for the doubleword case.
    c.check(0x42fc)?;
    for target in [
        0x4324, 0x433c, 0x4354, 0x43bc, 0x436c,
        0x4380, 0x4394, 0x43a8, 0x43bc, 0x43bc,
    ] { c.emit(target)?; }
    c.check(0x4324)?;

    // Word (pointer) argument.
    advance_argument(&mut c)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.branch(C::Al, 0x43d8)?;

    // Signed 32-bit value with explicit upper-word sign extension.
    c.check(0x433c)?;
    advance_argument(&mut c)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(mov_shifted_register(C::Al, r3, R::R2, Shift::Asr, 31)?)?;
    c.branch(C::Al, 0x43d4)?;

    // Unsigned 32-bit value with an all-zero high word.
    c.check(0x4354)?;
    advance_argument(&mut c)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.branch(C::Al, 0x43d4)?;

    // Signed 16-bit arguments widen via sign extension.
    c.check(0x436c)?;
    advance_argument(&mut c)?;
    c.emit(load_signed_halfword(C::Al, R::R2, R::R1, 0)?)?;
    c.branch(C::Al, 0x434c)?;

    // Unsigned 16-bit arguments widen with a zero upper word.
    c.check(0x4380)?;
    advance_argument(&mut c)?;
    c.emit(load_halfword(C::Al, R::R2, R::R1, 0)?)?;
    c.branch(C::Al, 0x4364)?;

    // Signed 8-bit and unsigned 8-bit variants.
    c.check(0x4394)?;
    advance_argument(&mut c)?;
    c.emit(load_signed_byte(C::Al, R::R2, R::R1, 0)?)?;
    c.branch(C::Al, 0x434c)?;
    c.check(0x43a8)?;
    advance_argument(&mut c)?;
    c.emit(transfer_byte(C::Al, true, R::R2, R::R1, 0)?)?;
    c.branch(C::Al, 0x4364)?;

    // 64-bit arguments are fetched from an aligned pair and written as a pair.
    c.check(0x43bc)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(add(R::R1, R::R1, 7)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R1, R::R1, 7)?)?;
    c.emit(add(r3, R::R1, 8)?)?;
    c.emit(store(r3, R::R2, 0)?)?;
    c.emit(transfer_doubleword(C::Al, true, R::R2, R::R1, 0)?)?;
    c.check(0x43d4)?;
    c.emit(transfer_doubleword(C::Al, false, R::R2, R::R0, 0)?)?;
    c.check(0x43d8)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x43dc)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_previous_helper() { assert!(extend(&mut Vec::new()).is_err()); }
}
