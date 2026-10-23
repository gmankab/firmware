// SPDX-License-Identifier: GPL-3.0-only
//! Decimal digit emission and bounded decimal-input parsing.
//! Instructions are constructed from typed ARM A32 operands, not raw opcode
//! arrays. The API is shared through the device runtime's address-aware
//! builder so branch destinations remain checked.
use arm32::{
    bx, data_immediate, data_register, mov_register,
    push_registers, transfer_byte, transfer_byte_preindexed_writeback,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x425c };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let saved = r4.bit() | r5.bit() | R::new(7)?.bit() | R::LR.bit();

    // Convert the unsigned argument into decimal digits, writing from the
    // buffer's far end towards its beginning.  The division service returns
    // a quotient and remainder in R0/R1 respectively.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.check(0x4268)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov(R::R1, 10)?)?;
    c.call(0x81850)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 48)?)?;
    c.emit(cmp(r5, 9)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, R::R1, r4, -1)?)?;
    c.branch(C::Hi, 0x4268)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    // Cheap control-character guard used by the diagnostic writer.  The
    // branch is a tail-call and retains the original argument registers.
    c.check(0x4290)?;
    c.emit(mov_register(C::Al, r3, R::R0))?;
    c.emit(transfer_byte(C::Al, true, R::R0, r3, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 32)?)?;
    c.emit(bx(C::Ne, R::LR))?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, R::R1, R::R2))?;
    c.emit(mov_register(C::Al, R::R2, r3))?;
    c.branch(C::Al, 0x4450)?;

    // Parse a run of decimal digits from a pointer-to-pointer.  Updating the
    // pointer during parsing is part of the public parsing semantics.
    c.check(0x42b0)?;
    c.emit(mov(R::R1, 0)?)?;
    c.check(0x42b4)?;
    c.emit(load(r3, R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r3, 0)?)?;
    c.emit(sub(R::R2, R::R2, 48)?)?;
    c.emit(cmp(R::R2, 9)?)?;
    c.emit(mov_register(C::Hi, R::R0, R::R1))?;
    c.emit(bx(C::Hi, R::LR))?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R1, Shift::Lsl, 2, false)?)?;
    c.emit(add(r3, r3, 1)?)?;
    c.emit(store(r3, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R2, R::R1, Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x42b4)?;
    c.check(0x42e0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_preceding_formatter() { assert!(extend(&mut Vec::new()).is_err()); }
}
