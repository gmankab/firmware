// SPDX-License-Identifier: GPL-3.0-only
//! Diagnostic output emission and width/padding calculation for formatted
//! values. Call addresses identify external Venus service operations; each
//! encoded instruction is generated from public, typed A32 operands.
use arm32::{
    data_immediate, data_register, mov_register, transfer_byte, Condition as C,
    DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3be0 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;

    // Integer conversion width: account for sign or a single leading zero
    // before updating the remaining available output space.
    c.emit(load(r3, R::SP, 20)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R1, r3, r10, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R0, r3))?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::transfer_word(C::Le, false, R::R1, R::SP, 40)?)?;
    c.branch(C::Al, 0x3c84)?;

    // Single-character diagnostic conversion, then forward into shared
    // output padding/width logic.
    c.check(0x3c08)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::SP, 104)?)?;
    c.emit(load(r10, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, R::R11, r4))?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::SP, 103)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(store(R::R0, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.branch(C::Al, 0x3c80)?;

    // An error object supplies a format string; if the supplied pointer is
    // null, fall back to a named literal string.
    c.check(0x3c28)?;
    c.call(0x4a6c)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.call(0x4ae0)?;
    c.emit(mov_register(C::Al, r10, R::R0))?;
    c.branch(C::Al, 0x3c4c)?;
    c.check(0x3c3c)?;
    c.emit(load(r10, R::SP, 104)?)?;
    c.literal(R::R0, 0x3fcc)?;
    c.emit(cmp(r10, 0)?)?;
    c.emit(mov_register(C::Eq, r10, R::R0))?;
    c.check(0x3c4c)?;
    c.emit(load(r5, R::SP, 40)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(R::R0, R::SP, 12)?)?;
    c.emit(mov_register(C::Al, R::R0, r10))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x4660)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.emit(mov_register(C::Al, R::R11, r4))?;
    c.emit(data_register(C::Ne, Op::Sub, R::R1, R::R0, r10, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Eq, Op::Add, R::R0, r10, r5, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R1, R::SP, 40)?)?;
    c.check(0x3c80)?;
    c.literal(r9, 0x3fc4)?;

    // Write left padding, optional sign and zero-fill, formatted data,
    // and finally any right padding. All lengths are computed as registers.
    c.check(0x3c84)?;
    c.emit(load(r7, R::SP, 40)?)?;
    c.emit(load(r5, R::SP, 12)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R0, r10, Shift::Lsl, 0, false)?)?;
    c.emit(store(r9, R::SP, 4)?)?;
    c.emit(load(r9, R::SP, 24)?)?;
    c.emit(load(r6, R::SP, 48)?)?;
    c.emit(mov(R::R1, 32)?)?;
    c.emit(store(R::R11, R::SP, 0)?)?;
    c.emit(store(R::R0, R::SP, 8)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, r7, R::R0))?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(data_register(C::Al, Op::Add, r4, r5, r7, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r9, r4, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r3, r4))?;
    c.emit(mov_register(C::Lt, r9, r4))?;
    c.emit(mov_register(C::Al, R::R2, r9))?;
    c.call(0x43dc)?;
    c.emit(load(R::R1, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x4290)?;
    c.emit(data_immediate(C::Al, Op::Eor, R::R0, R::R11, 65536)?)?;
    c.emit(mov(R::R1, 48)?)?;
    c.emit(mov_register(C::Al, R::R2, r9))?;
    c.emit(mov_register(C::Al, r3, r4))?;
    c.emit(store(R::R0, R::SP, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x43dc)?;
    c.emit(load(r5, R::SP, 8)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, r7))?;
    c.emit(mov(r7, 137)?)?;
    c.emit(mov(R::R1, 48)?)?;
    c.emit(store(R::R0, R::SP, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(data_immediate(C::Al, Op::Orr, r7, r7, 75776)?)?;
    c.emit(mov_register(C::Al, r3, r5))?;
    c.call(0x43dc)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(mov_register(C::Al, R::R1, r10))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x4290)?;
    c.emit(data_immediate(C::Al, Op::Eor, R::R0, R::R11, 8192)?)?;
    c.emit(mov(R::R1, 32)?)?;
    c.emit(mov_register(C::Al, R::R2, r9))?;
    c.emit(mov_register(C::Al, r3, r4))?;
    c.emit(store(R::R0, R::SP, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x43dc)?;
    c.branch(C::Al, 0x3e4c)?;
    c.check(0x3d50)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reject_noncontiguous_source() { assert!(extend(&mut Vec::new()).is_err()); }
}
