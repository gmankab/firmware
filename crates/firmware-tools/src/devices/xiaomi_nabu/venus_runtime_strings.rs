// SPDX-License-Identifier: GPL-3.0-only
//! Venus length-bounded string copy and optimized byte-length routines.
//! Both algorithms are implemented with checked public ARM A32 encodings.
//! The wordwise zero-byte tests use typed arithmetic and named constants.
use arm32::{
    bx, data_immediate, data_register, mov_register, push_registers,
    transfer_byte, transfer_byte_postindexed, transfer_multiple,
    transfer_word_postindexed, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3294 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r12 = R::R12;

    // Copy a bounded, NUL-terminated string, initially bytewise if the
    // source and destination have incompatible word-alignment addresses.
    c.emit(push_registers(C::Al, r4.bit() | R::LR.bit())?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Ne, 0x32a8)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.branch(C::Al, 0x336c)?;
    c.check(0x32a8)?;
    c.emit(sub(r3, R::R2, 1)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R2,
        R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R2, 3)?)?;
    c.branch(C::Eq, 0x32e0)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.check(0x32bc)?;
    c.emit(cmp(r3, 0)?)?;
    c.emit(transfer_byte(C::Ne, true, r12, R::R1, 0)?)?;
    c.emit(transfer_byte(C::Ne, false, r12, R::R2, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, r12, 0)?)?;
    c.branch(C::Eq, 0x3364)?;
    c.emit(add(R::R2, R::R2, 1)?)?;
    c.emit(add(R::R1, R::R1, 1)?)?;
    c.emit(sub(r3, r3, 1)?)?;
    c.branch(C::Al, 0x32bc)?;
    c.check(0x32e0)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.check(0x32e4)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 3)?)?;
    c.branch(C::Eq, 0x331c)?;
    c.emit(cmp(r3, 0)?)?;
    c.branch(C::Ne, 0x32fc)?;
    c.emit(mov(r3, 0)?)?;
    c.branch(C::Al, 0x32bc)?;
    c.check(0x32fc)?;
    c.emit(transfer_byte(C::Al, true, r12, R::R1, 0)?)?;
    c.emit(cmp(r12, 0)?)?;
    c.emit(transfer_byte(C::Al, false, r12, R::R2, 0)?)?;
    c.branch(C::Eq, 0x3324)?;
    c.emit(add(R::R2, R::R2, 1)?)?;
    c.emit(add(R::R1, R::R1, 1)?)?;
    c.emit(sub(r3, r3, 1)?)?;
    c.branch(C::Al, 0x32e4)?;
    c.check(0x331c)?;
    c.emit(cmp(r3, 0)?)?;
    c.branch(C::Eq, 0x32f4)?;
    c.check(0x3324)?;
    c.emit(transfer_byte(C::Al, true, r12, R::R1, 0)?)?;
    c.emit(cmp(r12, 0)?)?;
    c.branch(C::Eq, 0x32bc)?;
    c.emit(cmp(r3, 4)?)?;
    c.branch(C::Cc, 0x32bc)?;
    c.emit(load(r12, R::R1, 0)?)?;
    c.literal(R::LR, 0x3380)?;
    c.literal(r4, 0x3384)?;
    c.emit(data_register(C::Al, Op::Add, R::LR,
        r12, R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Bic, R::LR,
        R::LR, r12, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Tst, R::R0,
        R::LR, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x32bc)?;
    c.emit(transfer_word_postindexed(C::Al, false, r12, R::R2, 4)?)?;
    c.emit(add(R::R1, R::R1, 4)?)?;
    c.emit(sub(r3, r3, 4)?)?;
    c.branch(C::Al, 0x3330)?;
    c.check(0x3364)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(transfer_byte(C::Al, false, r3, R::R2, 0)?)?;
    c.check(0x336c)?;
    c.emit(data_register(C::Al, Op::Sub, r4,
        R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.call(0x3388)?;
    c.emit(data_register(C::Al, Op::Add, R::R0,
        r4, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // Public parallel-zero-byte detection constants for 32-bit words.
    c.emit(0xfefe_feff)?;
    c.emit(0x8080_8080)?;
    c.check(0x3388)?;

    // Word-oriented C-string byte length with unaligned head and byte tail.
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.check(0x338c)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 3)?)?;
    c.branch(C::Eq, 0x33a8)?;
    c.emit(transfer_byte(C::Al, true, R::R2, R::R1, 0)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x33e0)?;
    c.emit(add(R::R1, R::R1, 1)?)?;
    c.branch(C::Al, 0x338c)?;
    c.check(0x33a8)?;
    c.emit(push_registers(C::Al, R::new(7)?.bit() | R::LR.bit())?)?;
    c.literal(r3, 0x33e8)?;
    c.check(0x33b0)?;
    c.emit(mov_register(C::Al, R::R2, R::R1))?;
    c.emit(transfer_word_postindexed(C::Al, true, r12, R::R1, 4)?)?;
    c.literal(R::LR, 0x33ec)?;
    c.emit(data_register(C::Al, Op::Add, R::LR,
        r12, R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Bic, r12,
        r3, r12, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Tst, R::R0,
        r12, R::LR, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x33b0)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::new(7)?.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x33d0)?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.emit(transfer_byte_postindexed(C::Al, true, r3, R::R2, 1)?)?;
    c.emit(cmp(r3, 0)?)?;
    c.branch(C::Ne, 0x33d0)?;
    c.check(0x33e0)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0,
        R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0x8080_8080)?;
    c.emit(0xfefe_feff)?;
    c.check(0x33f0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_complete_preceding_runtime() { assert!(extend(&mut Vec::new()).is_err()); }
}
