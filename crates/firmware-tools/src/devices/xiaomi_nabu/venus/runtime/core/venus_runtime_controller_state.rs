// SPDX-License-Identifier: GPL-3.0-only
//! Indexed controller bitfields, resource status fields, and bulk state reset.
//! Layout is expressed as ARM A32 operations and named MMIO addresses.
use arm32::{
    bx, data_immediate, data_register, data_register_shifted_by_register,
    mov_register, push_registers, transfer_multiple, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{cmp, load, mov, store, sub};

const RESOURCE_COUNT: u32 = 0x0010_3944;
const RESOURCE_TABLE: u32 = 0x0010_3948;
const ACTIVE_RESOURCE: u32 = 0x0010_cfa8;

fn regop(c: &mut Code<'_>, op: Op, dst: R, lhs: R, rhs: R) -> Result<(), String> {
    c.emit(data_register(C::Al, op, dst, lhs, rhs, Shift::Lsl, 0, false)?)
}
fn indexed(c: &mut Code<'_>, dst: R, base: R, index: R) -> Result<(), String> {
    c.emit(transfer_word_register(C::Al, true, dst, base, index, Shift::Lsl, 2)?)
}
fn addresses(c: &mut Code<'_>) -> Result<(), String> {
    for addr in [RESOURCE_COUNT, RESOURCE_TABLE, ACTIVE_RESOURCE] { c.emit(addr)?; }
    Ok(())
}

/// Update one three-bit controller field while preserving higher state bits.
fn update_bitfield(c: &mut Code<'_>) -> Result<(), String> {
    c.literal(R::new(3)?, 0x7a00)?;
    c.emit(load(R::new(3)?, R::new(3)?, 0)?)?;
    regop(c, Op::Cmp, R::R0, R::new(3)?, R::R2)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::new(3)?, 0x7a04)?;
    c.literal(R::R12, 0x7a08)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 7)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 7)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    c.emit(load(R::new(3)?, R::new(3)?, 0)?)?;
    indexed(c, R::R2, R::new(3)?, R::R2)?;
    c.emit(mov(R::new(3)?, 7)?)?;
    c.emit(store(R::R2, R::R12, 0)?)?;
    c.emit(load(R::R2, R::R12, 0)?)?;
    c.emit(load(R::R2, R::R2, 4)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Bic,
        R::R2, R::R2, R::new(3)?, Shift::Lsl, R::R0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff00_0000)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr,
        R::R0, R::R2, R::R1, Shift::Lsl, R::R0, false)?)?;
    c.emit(load(R::R1, R::R12, 0)?)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    addresses(c)
}

/// Set either of the two mirrored 4-bit controller state selectors.
fn set_selector(c: &mut Code<'_>, register_offset: i32, end: usize) -> Result<(), String> {
    c.literal(R::R2, end - 12)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    regop(c, Op::Cmp, R::R0, R::R2, R::R1)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::R2, end - 8)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 15)?)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    indexed(c, R::R1, R::R2, R::R1)?;
    c.literal(R::R2, end - 4)?;
    c.emit(store(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R1, register_offset)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(store(R::R0, R::R1, register_offset)?)?;
    c.emit(bx(C::Al, R::LR))?;
    addresses(c)
}

/// Retrieve the selected object's controller mask, respecting the index bound.
fn read_selector(c: &mut Code<'_>, extended: bool) -> Result<(), String> {
    let start = c.at;
    let end = if extended { start + 68 } else { start + 56 };
    c.literal(R::R1, end - 12)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    regop(c, Op::Cmp, R::R0, R::R1, R::R0)?;
    c.branch(C::Ls, if extended { start + 36 } else { start + 36 })?;
    c.literal(R::R1, end - 8)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    indexed(c, R::R0, R::R1, R::R0)?;
    c.literal(R::R1, end - 4)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    if extended {
        c.literal(R::R0, end - 4)?;
        c.emit(load(R::R0, R::R0, 0)?)?;
        c.emit(load(R::R0, R::R0, 0xfa4)?)?;
        c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 15)?)?;
    } else {
        c.emit(mov(R::R0, 15)?)?;
    }
    c.emit(bx(C::Al, R::LR))?;
    addresses(c)
}

/// Walk every registered controller and clear its active state field.
fn clear_all(c: &mut Code<'_>) -> Result<(), String> {
    let saved = R::R11.bit() | R::LR.bit();
    let restored = R::R11.bit() | R::PC.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.literal(R::LR, 0x7b74)?;
    c.emit(load(R::R0, R::LR, 0)?)?;
    c.emit(sub(R::R1, R::R0, 1)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Lt, 0x7b70)?;
    c.literal(R::R2, 0x7b78)?;
    c.literal(R::new(3)?, 0x7b7c)?;
    c.emit(mov(R::R12, 255)?)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    c.check(0x7b44)?;
    regop(c, Op::Cmp, R::R0, R::R0, R::R1)?;
    c.branch(C::Ls, 0x7b5c)?;
    indexed(c, R::R0, R::R2, R::R1)?;
    c.emit(store(R::R0, R::new(3)?, 0)?)?;
    c.emit(load(R::R0, R::new(3)?, 0)?)?;
    c.emit(store(R::R12, R::R0, 0)?)?;
    c.check(0x7b5c)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(transfer_multiple(C::Lt, true, R::SP, restored,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R0, R::LR, 0)?)?;
    c.branch(C::Al, 0x7b44)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, restored,
        BlockMode::IncrementAfter, true)?)?;
    addresses(c)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x79ac };
    update_bitfield(&mut c)?;
    c.check(0x7a0c)?;
    set_selector(&mut c, 0xfa0, 0x7a54)?;
    c.check(0x7a54)?;
    set_selector(&mut c, 0xfa4, 0x7a9c)?;
    c.check(0x7a9c)?;
    read_selector(&mut c, false)?;
    c.check(0x7ad4)?;
    read_selector(&mut c, true)?;
    c.check(0x7b18)?;
    clear_all(&mut c)?;
    c.check(0x7b80)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn patterned_controller_state_helpers_have_strict_boundaries() {
        let mut bytes = vec![0; 0x79ac - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x7b80 - 0x1000);
    }
    #[test]
    fn controller_helpers_reject_disconnected_layout() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
