// SPDX-License-Identifier: GPL-3.0-only
//! Runtime diagnostic-name lookups, code-to-message table traversal, and
//! small error-status wrappers. External addresses are typed metadata values.
use arm32::{
    bx, data_register, mov_register, push_registers,
    transfer_byte, transfer_byte_postindexed, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, sub};

const ERROR_CODE_TABLE: u32 = 0x000e_2bd0;
const ERROR_STRINGS: u32 = 0x000e_2c28;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4a6c };
    let r3 = R::new(3)?;
    let r7 = R::new(7)?;
    let saved = r7.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.call(0x4b84)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R0, Shift::Lsl, 0, true)?)?;
    c.emit(sub(R::R0, R::R1, 164)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r7.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    // Error number -> diagnostic message table lookup. Strings are walked
    // sequentially until the desired indexed record has been reached.
    c.check(0x4a80)?;
    c.literal(r3, 0x4ad8)?;
    c.emit(mov(R::R2, 0)?)?;
    c.check(0x4a88)?;
    c.emit(cmp(R::R2, 87)?)?;
    c.branch(C::Ne, 0x4a98)?;
    c.emit(mov(R::R2, 87)?)?;
    c.branch(C::Al, 0x4aa4)?;
    c.check(0x4a98)?;
    c.emit(transfer_byte(C::Al, true, R::R12, r3, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R12, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x4acc)?;
    c.check(0x4aa4)?;
    c.literal(R::R0, 0x4adc)?;
    c.check(0x4aa8)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x4ac4)?;
    c.check(0x4ab0)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r3, R::R0, 1)?)?;
    c.emit(cmp(r3, 0)?)?;
    c.branch(C::Ne, 0x4ab0)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.branch(C::Al, 0x4aa8)?;
    c.check(0x4ac4)?;
    c.emit(load(R::R1, R::R1, 20)?)?;
    c.branch(C::Al, 0x4b04)?;
    c.check(0x4acc)?;
    c.emit(add(R::R2, R::R2, 1)?)?;
    c.emit(add(r3, r3, 1)?)?;
    c.branch(C::Al, 0x4a88)?;
    c.emit(ERROR_CODE_TABLE)?;
    c.emit(ERROR_STRINGS)?;
    c.check(0x4ae0)?;
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.call(0x4b84)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R0, Shift::Lsl, 0, true)?)?;
    c.emit(load(R::R1, R::R1, -48)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        saved, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x4a80)?;
    c.check(0x4b00)?;
    c.emit(bx(C::Al, R::LR))?;
    c.branch(C::Al, 0x4b00)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(mov(R::R2, 0)?)?;
    c.branch(C::Al, 0x4b88)?;
    c.check(0x4b1c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_previous_memory_copy() { assert!(extend(&mut Vec::new()).is_err()); }
}
