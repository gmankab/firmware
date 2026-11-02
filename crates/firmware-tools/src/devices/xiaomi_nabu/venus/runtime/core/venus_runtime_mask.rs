// SPDX-License-Identifier: GPL-3.0-only
//! Venus resource release and interrupt-mask registration routines.
//! ARM32 operations use typed encoders; service-parameter addresses appear
//! only as explicitly named literal-pool data.
use arm32::{
    data_immediate, data_register, data_register_shifted_by_register, mov_register,
    push_registers, transfer_multiple, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2674 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    let preserved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();

    // Release the slot and notify the MMIO-backed pool of the freed bit.
    c.emit(push_registers(C::Al, preserved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.literal(R::R2, 0x26cc)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 328)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(cmp(r4, 16)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov,
        R::R1, R::R0, R::R0, Shift::Lsl, r4, false)?)?;
    c.emit(load(R::R0, r5, 76)?)?;
    c.call(0x2e40)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.call(0x2bf4)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r5, r4, Shift::Lsl, 2, false)?)?;
    c.emit(store(r6, R::R0, 4)?)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, preserved,
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x2c34)?;
    c.emit(0x000e_27f9)?; // Source-code identifier
    c.check(0x26d0)?;

    // Mark an interrupt bit as present in a banked bitmap.
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.literal(r8, 0x2744)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(r7, 0)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov(R::R1, 160)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(cmp(r5, 16)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r7, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.call(0x17b0)?;
    c.emit(cmp(r4, 6)?)?;
    c.emit(mov(R::R1, 161)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.emit(data_immediate(C::Cc, Op::Mov, r6, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x17b0)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov(R::R1, 328)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.call(0x17b0)?;
    c.literal(R::R0, 0x2748)?;
    c.emit(mov(R::R2, 1)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, r4, Shift::Lsl, 2)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr,
        R::R1, R::R1, R::R2, Shift::Lsl, r5, false)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R1, R::R0, r4, Shift::Lsl, 2)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() |
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_27f9)?; // Source-code identifier
    c.emit(0x0010_3484)?; // Shared interrupt-mask table
    c.check(0x274c)?;

    // Atomically reassign a bit between two interrupt banks, clearing the
    // old field and setting the new field after validating both indices.
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | r10.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 28)?)?;
    c.emit(sub(R::SP, R::SP, 4)?)?;
    c.literal(r8, 0x2800)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(r7, 0)?)?;
    c.emit(mov_register(C::Al, r9, R::R2))?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(mov(R::R1, 170)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(cmp(r5, 16)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r7, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.call(0x17b0)?;
    c.emit(cmp(r6, 6)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 171)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(cmp(r9, 6)?)?;
    c.emit(mov(R::R1, 172)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.emit(data_immediate(C::Cc, Op::Mov, r4, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x17b0)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov(R::R1, 328)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.call(0x17b0)?;
    c.literal(r4, 0x2804)?;
    c.emit(mov(r10, 1)?)?;
    c.emit(mov(R::R1, 328)?)?;
    c.emit(mov_register(C::Al, R::R2, r8))?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r4, r6, Shift::Lsl, 2)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Bic,
        R::R0, R::R0, r10, Shift::Lsl, r5, false)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R0, r4, r6, Shift::Lsl, 2)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.call(0x17b0)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r4, r9, Shift::Lsl, 2)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr,
        R::R0, R::R0, r10, Shift::Lsl, r5, false)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R0, r4, r9, Shift::Lsl, 2)?)?;
    c.emit(sub(R::SP, R::R11, 28)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() |
        r10.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_27f9)?; // Source-code identifier
    c.emit(0x0010_3484)?; // Shared interrupt-mask table
    c.check(0x2808)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_preceding_runtime() { assert!(extend(&mut Vec::new()).is_err()); }
}
