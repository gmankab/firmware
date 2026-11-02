// SPDX-License-Identifier: GPL-3.0-only
//! UTF-8 scalar encoding used by the Venus diagnostics layer. Encodings of
//! one through four bytes are selected from Unicode scalar-value ranges.
//! This is a machine-code-level implementation with typed ARM A32 operands.
use arm32::{
    compiler_trap, data_immediate, data_register, mov_register,
    push_registers, transfer_byte, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{cmp, load, mov, store, sub};

const SURROGATE_RANGE_MASK: u32 = 0xffff_e000;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4b84 };
    let r3 = R::new(3)?;
    let r7 = R::new(7)?;
    c.emit(compiler_trap())?;
    c.check(0x4b88)?;
    c.emit(push_registers(C::Al, r7.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.emit(mov(r3, 1)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x4ca4)?;
    c.emit(cmp(R::R1, 127)?)?;
    c.branch(C::Hi, 0x4bac)?;
    c.check(0x4ba4)?;
    c.emit(transfer_byte(C::Al, false, R::R1, R::R2, 0)?)?;
    c.branch(C::Al, 0x4ca4)?;
    c.check(0x4bac)?;
    c.call(0x4b84)?;
    c.emit(data_register(C::Al, Op::Mov, R::R12, R::R0, R::R0, Shift::Lsl, 0, true)?)?;
    c.emit(load(R::R0, R::R12, -48)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x4be8)?;
    c.emit(mov(R::R0, 191)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 256)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsr, 7, false)?)?;
    c.branch(C::Eq, 0x4ba4)?;
    c.check(0x4bd4)?;
    c.call(0x4a6c)?;
    c.emit(mov(R::R1, 84)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r3, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.branch(C::Al, 0x4ca4)?;

    // Two-byte UTF-8 encoding.
    c.check(0x4be8)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsr, 11, false)?)?;
    c.branch(C::Ne, 0x4c14)?;
    c.emit(mov(R::R0, 192)?)?;
    c.emit(mov(r3, 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsr, 6, false)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R1, 63)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 128)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 1)?)?;
    c.branch(C::Al, 0x4ca4)?;

    // Three-byte encoding with surrogate-range handling.
    c.check(0x4c14)?;
    c.emit(cmp(R::R1, 55296)?)?;
    c.branch(C::Cc, 0x4c2c)?;
    c.literal(R::R0, 0x4cac)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(cmp(R::R0, 57344)?)?;
    c.branch(C::Ne, 0x4c5c)?;
    c.check(0x4c2c)?;
    c.emit(mov(R::R0, 224)?)?;
    c.emit(mov(r3, 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsr, 12, false)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 0)?)?;
    c.emit(mov(R::R0, 63)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R0, R::R1, Shift::Lsr, 6, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 128)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 1)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R1, 63)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 128)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 2)?)?;
    c.branch(C::Al, 0x4ca4)?;

    // Four-byte scalar encoding. Reject numbers outside Unicode range.
    c.check(0x4c5c)?;
    c.emit(sub(R::R0, R::R1, 65536)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, R::R0, Shift::Lsr, 20, false)?)?;
    c.branch(C::Ne, 0x4bd4)?;
    c.emit(mov(R::R0, 240)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsr, 18, false)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 0)?)?;
    c.emit(mov(R::R0, 63)?)?;
    c.emit(data_register(C::Al, Op::And, r3, R::R0, R::R1, Shift::Lsr, 12, false)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R0, R::R1, Shift::Lsr, 6, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r3, r3, 128)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 128)?)?;
    c.emit(transfer_byte(C::Al, false, r3, R::R2, 1)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 2)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R1, 63)?)?;
    c.emit(mov(r3, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 128)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 3)?)?;
    c.check(0x4ca4)?;
    c.emit(mov_register(C::Al, R::R0, r3))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r7.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(SURROGATE_RANGE_MASK)?;
    c.check(0x4cb0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_stream_state() { assert!(extend(&mut Vec::new()).is_err()); }
}
