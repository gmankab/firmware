// SPDX-License-Identifier: GPL-3.0-only
//! Venus optimized memory-fill primitive using public ARM A32 encodings.
//! Byte, halfword and bulk-word stores are generated through typed fields,
//! preserving unaligned and tail handling without vendor opcode arrays.
use arm32::{
    data_immediate, data_immediate_with_flags, data_register, mov_register,
    push_registers, store_halfword_postindexed, transfer_byte,
    transfer_byte_postindexed, transfer_multiple, transfer_word_postindexed,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, mov, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x31ec };
    let r3 = R::new(3)?;
    let r12 = R::R12;

    // Fill a buffer, aligning its head and writing four word-sized values per
    // pair of STM instructions. The tail is handled by bit tests.
    c.emit(push_registers(C::Al, R::R0.bit() | R::LR.bit())?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r3, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r3, r3, 3)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0,
        r3, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r3, R::R2))?;
    for amount in [8, 16] {
        c.emit(data_register(C::Al, Op::Orr, R::R1,
            R::R1, R::R1, Shift::Lsl, amount, false)?)?;
    }
    c.emit(data_register(C::Al, Op::Mov, r12, R::R0,
        r3, Shift::Lsl, 31, true)?)?;
    c.emit(transfer_byte_postindexed(C::Mi, false, R::R1, R::R0, 1)?)?;
    c.emit(store_halfword_postindexed(C::Cs, R::R1, R::R0, 2)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R2, R::R2,
        r3, Shift::Lsl, 0, true)?)?;
    c.emit(mov_register(C::Al, r3, R::R1))?;
    c.emit(mov_register(C::Al, r12, R::R1))?;
    c.emit(mov_register(C::Al, R::LR, R::R1))?;
    c.check(0x3224)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, R::R2, R::R2, 32, true)?)?;
    let wide_mask = R::R1.bit() | r3.bit() | r12.bit() | R::LR.bit();
    for _ in 0..2 {
        c.emit(transfer_multiple(C::Cs, false, R::R0, wide_mask,
            BlockMode::IncrementAfter, true)?)?;
    }
    c.emit(data_immediate_with_flags(C::Cs, Op::Sub, R::R2, R::R2, 32, true)?)?;
    c.branch(C::Cs, 0x3228)?;
    c.emit(data_register(C::Al, Op::Mov, R::R2, R::R0,
        R::R2, Shift::Lsl, 28, true)?)?;
    c.emit(transfer_multiple(C::Cs, false, R::R0, wide_mask,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Mi, false, R::R0,
        R::R1.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R2, R::R0,
        R::R2, Shift::Lsl, 2, true)?)?;
    c.emit(transfer_word_postindexed(C::Cs, false, R::R1, R::R0, 4)?)?;
    c.emit(store_halfword_postindexed(C::Mi, R::R1, R::R0, 2)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R2, R::R0,
        R::R2, Shift::Lsl, 2, true)?)?;
    c.emit(transfer_byte(C::Cs, false, R::R1, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R0.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x325c)?;

    // Standard unsigned byte comparison, returning the unsigned-byte delta
    // at the first mismatch. The zero-length case returns zero immediately.
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Ne, 0x326c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x3290)?;
    c.check(0x326c)?;
    c.emit(arm32::transfer_byte(C::Al, true, r3, R::R1, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al, true, r12, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r12,
        r3, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x328c)?;
    c.emit(add(R::R1, R::R1, 1)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.branch(C::Al, 0x325c)?;
    c.check(0x328c)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r12,
        r3, Shift::Lsl, 0, false)?)?;
    c.emit(arm32::bx(C::Al, R::LR))?;
    c.check(0x3294)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_image_is_rejected() { assert!(extend(&mut Vec::new()).is_err()); }
}
