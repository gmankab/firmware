// SPDX-License-Identifier: GPL-3.0-only
//! Bounded byte search optimized for aligned 32-bit access.
//! The wordwise zero-byte detection constants are standard arithmetic masks,
//! not copied machine instructions. Nonaligned and trailing bytes are
//! processed through ordinary byte comparisons.
use arm32::{
    data_immediate, data_register, mov_register, multiply_register,
    push_registers, transfer_byte, transfer_multiple, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, sub};

const REPEATED_LOW_BYTES: u32 = 0x0101_0101;
const ZERO_BYTE_SUBTRACT: u32 = 0xfefe_feff;
const HIGH_BIT_MASK: u32 = 0x8080_8080;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4660 };
    let r3 = R::new(3)?;
    let r7 = R::new(7)?;
    c.emit(push_registers(C::Al, r7.bit() | R::LR.bit())?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 255)?)?;
    c.check(0x4668)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 3)?)?;
    c.branch(C::Eq, 0x46a0)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Ne, 0x4688)?;
    c.check(0x4678)?;
    c.emit(mov(R::R2, 0)?)?;
    c.check(0x467c)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.emit(mov_register(C::Eq, R::R0, R::R2))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r7.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x4688)?;
    c.emit(transfer_byte(C::Al, true, r3, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x46a8)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.branch(C::Al, 0x4668)?;
    c.check(0x46a0)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x4678)?;
    c.check(0x46a8)?;
    c.emit(transfer_byte(C::Al, true, r3, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x467c)?;
    c.literal(R::R12, 0x4710)?;
    c.emit(multiply_register(C::Al, r3, R::R1, R::R12)?)?;
    c.check(0x46bc)?;
    c.emit(cmp(R::R2, 4)?)?;
    c.branch(C::Cc, 0x46f0)?;
    c.emit(load(R::R12, R::R0, 0)?)?;
    c.literal(R::LR, 0x4714)?;
    c.emit(data_register(C::Al, Op::Eor, R::R12, R::R12, r3, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::LR, R::R12, R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Bic, R::R12, R::LR, R::R12, Shift::Lsl, 0, false)?)?;
    c.literal(R::LR, 0x4718)?;
    c.emit(data_register(C::Al, Op::Tst, R::R0, R::R12, R::LR, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x46f0)?;
    c.emit(sub(R::R2, R::R2, 4)?)?;
    c.emit(add(R::R0, R::R0, 4)?)?;
    c.branch(C::Al, 0x46bc)?;
    c.check(0x46f0)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x4678)?;
    c.emit(transfer_byte(C::Al, true, r3, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x467c)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.branch(C::Al, 0x46f0)?;
    c.check(0x4710)?;
    for mask in [REPEATED_LOW_BYTES, ZERO_BYTE_SUBTRACT, HIGH_BIT_MASK] {
        c.emit(mask)?;
    }
    c.check(0x471c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_buffered_writer() { assert!(extend(&mut Vec::new()).is_err()); }
}
