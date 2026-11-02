// SPDX-License-Identifier: GPL-3.0-only
//! Venus indexed resource state bits and controller register writes.
use arm32::{
    bx, data_immediate, data_register, data_register_shifted_by_register,
    mov_register, transfer_word_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{load, mov, store};

const RESOURCE_COUNT: u32 = 0x0010_3944;
const RESOURCE_TABLE: u32 = 0x0010_3948;
const ACTIVE_RESOURCE: u32 = 0x0010_cfa8;

fn register_op(c: &mut Code<'_>, op: Op, dst: R, lhs: R, rhs: R) -> Result<(), String> {
    c.emit(data_register(C::Al, op, dst, lhs, rhs, Shift::Lsl, 0, false)?)
}

fn indexed_resource(c: &mut Code<'_>, target: R, index: R, table: R) -> Result<(), String> {
    c.emit(transfer_word_register(C::Al, true, target, table,
        index, Shift::Lsl, 2)?)
}

/// Reset or clear a single indexed resource's service state word.
fn set_state(c: &mut Code<'_>, state: u32) -> Result<(), String> {
    let at = c.at;
    c.literal(R::R1, at + 0x34)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    register_op(c, Op::Cmp, R::R0, R::R1, R::R0)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::R1, at + 0x38)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    indexed_resource(c, R::R0, R::R0, R::R1)?;
    c.literal(R::R1, at + 0x3c)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.emit(mov(R::R1, state)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [RESOURCE_COUNT, RESOURCE_TABLE, ACTIVE_RESOURCE] {
        c.emit(data)?;
    }
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7828 };
    // Set a single indexed bit in the active device's packed control field.
    c.literal(R::R2, 0x7878)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    register_op(&mut c, Op::Cmp, R::R0, R::R2, R::R1)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::R2, 0x787c)?;
    c.emit(mov(R::new(3)?, 255)?)?;
    c.emit(mov(R::R12, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 3840)?)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    indexed_resource(&mut c, R::R1, R::R1, R::R2)?;
    c.literal(R::R2, 0x7880)?;
    c.emit(store(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Bic, R::R1, R::R1,
        R::R12, Shift::Lsl, R::R0, false)?)?;
    register_op(&mut c, Op::And, R::R1, R::R1, R::new(3)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr, R::R0, R::R1,
        R::R12, Shift::Lsl, R::R0, false)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [RESOURCE_COUNT, RESOURCE_TABLE, ACTIVE_RESOURCE] { c.emit(data)?; }
    c.check(0x7884)?;
    set_state(&mut c, 255)?;
    c.check(0x78c4)?;
    // Return whether an indexed bank has any of its low 8 status bits set.
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0, 0x7904)?;
    c.emit(load(R::R2, R::R0, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    register_op(&mut c, Op::Cmp, R::R0, R::R2, R::R1)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::R2, 0x7908)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    indexed_resource(&mut c, R::R1, R::R1, R::R2)?;
    c.literal(R::R2, 0x790c)?;
    c.emit(store(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 255)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [RESOURCE_COUNT, RESOURCE_TABLE, ACTIVE_RESOURCE] { c.emit(data)?; }
    c.check(0x7910)?;
    // Clear a requested bit while preserving the 12-bit resource mask.
    c.literal(R::R2, 0x7960)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    register_op(&mut c, Op::Cmp, R::R0, R::R2, R::R1)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::R2, 0x7964)?;
    c.emit(mov(R::new(3)?, 255)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 7)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 3840)?)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    indexed_resource(&mut c, R::R1, R::R1, R::R2)?;
    c.literal(R::R2, 0x7968)?;
    c.emit(store(R::R1, R::R2, 0)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Eor, R::R0,
        R::new(3)?, R::R1, Shift::Lsl, R::R0, false)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    register_op(&mut c, Op::And, R::R0, R::R0, R::R1)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [RESOURCE_COUNT, RESOURCE_TABLE, ACTIVE_RESOURCE] { c.emit(data)?; }
    c.check(0x796c)?;
    set_state(&mut c, 0)?;
    c.check(0x79ac)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resource_bit_set_clear_use_checked_a32_fields() {
        let mut bytes = vec![0; 0x7828 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x79ac - 0x1000);
        assert_eq!(u32::from_le_bytes(bytes[0x79a8-0x1000..0x79ac-0x1000].try_into().unwrap()), ACTIVE_RESOURCE);
    }
    #[test]
    fn bit_routines_reject_disconnected_code() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
