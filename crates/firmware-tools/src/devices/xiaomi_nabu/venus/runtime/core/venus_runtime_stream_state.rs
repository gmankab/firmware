// SPDX-License-Identifier: GPL-3.0-only
//! Diagnostic stream flag normalization and output-buffer state reset.
//! The subsequent legacy trap encoding is deliberately excluded until its
//! precise processor semantics and provenance are established.
use arm32::{
    bx, data_immediate, data_register, load_signed_byte,
    mov_register, transfer_byte, Condition as C,
    DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4b1c };
    let r3 = R::new(3)?;
    // Normalize a diagnostic stream's signed byte flags, and dispatch
    // between writable initialization and read-only rejection.
    c.emit(load_signed_byte(C::Al, R::R1, R::R0, 74)?)?;
    c.emit(add(R::R2, R::R1, 255)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, R::R1, R::R0, 74)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 8)?)?;
    c.branch(C::Ne, 0x4b64)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(R::R1, R::R0, 4)?)?;
    c.emit(store(R::R1, R::R0, 8)?)?;
    c.emit(load(R::R2, R::R0, 44)?)?;
    c.emit(store(R::R2, R::R0, 28)?)?;
    c.emit(store(R::R2, R::R0, 20)?)?;
    c.emit(load(r3, R::R0, 48)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, R::R2, r3, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R2, R::R0, 16)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x4b64)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 32)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x4b84)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_previous_stream_code() { assert!(extend(&mut Vec::new()).is_err()); }
}
