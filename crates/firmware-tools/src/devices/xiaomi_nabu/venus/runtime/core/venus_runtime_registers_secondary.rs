// SPDX-License-Identifier: GPL-3.0-only
//! ARM32 register accessors for the companion controller, including status checks.
use arm32::{
    bx, data_immediate, data_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{load, mov, store};

const SECONDARY_CONTROLLER: u32 = 0x0010_cfb0;
const AUTH_TOKEN: u32 = 0xc5ac_ce55;
const RESET_TOKEN: u32 = 0x3a53_31aa;

fn ret(c: &mut Code<'_>) -> Result<(), String> { c.emit(bx(C::Al, R::LR)) }
fn field_write(c: &mut Code<'_>, offset: i32, end: usize) -> Result<(), String> {
    c.literal(R::R1, end-4)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, offset)?)?;
    ret(c)?;
    c.emit(SECONDARY_CONTROLLER)
}
fn field_read(c: &mut Code<'_>, offset: i32, end: usize) -> Result<(), String> {
    c.literal(R::R0, end-4)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, offset)?)?;
    ret(c)?;
    c.emit(SECONDARY_CONTROLLER)
}
fn token_write(c: &mut Code<'_>, token: u32, end: usize) -> Result<(), String> {
    c.literal(R::R0, end-8)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.literal(R::R1, end-4)?;
    c.emit(store(R::R1, R::R0, 0xfb0)?)?;
    ret(c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    c.emit(token)
}
fn config_mode(c: &mut Code<'_>, enabled: u32, end: usize) -> Result<(), String> {
    c.literal(R::R0, end-4)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 32)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(mov(R::R1, enabled)?)?;
    c.emit(store(R::R1, R::R0, 32)?)?;
    ret(c)?;
    c.emit(SECONDARY_CONTROLLER)
}
fn and_register(c: &mut Code<'_>, dst: R, left: R, right: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::And, dst, left, right,
        Shift::Lsl, 0, false)?)
}
fn masked_config(c: &mut Code<'_>, initial: u32, retain: u32,
                 inserted: u32, end: usize) -> Result<(), String> {
    c.literal(R::R0, end - 4)?;
    c.emit(mov(R::R2, initial)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, retain)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x304)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    and_register(c, R::R1, R::R1, R::R2)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, inserted)?)?;
    c.emit(store(R::R1, R::R0, 0x304)?)?;
    ret(c)?;
    c.emit(SECONDARY_CONTROLLER)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x8148 };
    // Read secondary configuration without its inaccessible high bit.
    c.literal(R::R0, 0x815c)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x8000_0000)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    field_write(&mut c, 4, 0x8174)?;
    token_write(&mut c, AUTH_TOKEN, 0x8190)?;
    token_write(&mut c, RESET_TOKEN, 0x81ac)?;
    c.check(0x81ac)?;
    c.literal(R::R1, 0x81c8)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 3)?)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, 40)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, 40)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    c.check(0x81cc)?;
    // Embed a four-bit mode index in the controller's high control nibble.
    c.emit(mov(R::R1, 3840)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R1,
        R::R0, Shift::Lsl, 8, false)?)?;
    c.literal(R::R1, 0x81f4)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, 0x110)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R2, R::R2, 191)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R2,
        R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, R::R1, 0x110)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    c.check(0x81f8)?;
    // Normalize the same register while preserving the masked state.
    c.literal(R::R0, 0x821c)?;
    c.emit(mov(R::R2, 63)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 3840)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x110)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    and_register(&mut c, R::R1, R::R1, R::R2)?;
    c.emit(store(R::R1, R::R0, 0x110)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    c.check(0x8220)?;
    c.literal(R::R0, 0x8240)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x110)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 3968)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 14)?)?;
    c.emit(store(R::R1, R::R0, 0x110)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    c.check(0x8244)?;
    // Write the mapped service identity and clear an obsolete pending value.
    c.literal(R::R1, 0x8268)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R2, 0x118)?)?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.emit(load(R::R0, R::R0, 0x11c)?)?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(R::R1, R::R0, 0x11c)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    field_read(&mut c, 0x118, 0x8280)?;
    config_mode(&mut c, 0, 0x82a0)?;
    config_mode(&mut c, 1, 0x82c0)?;
    field_write(&mut c, 0x304, 0x82d4)?;
    masked_config(&mut c, 51, 14080, 64, 0x8300)?;
    c.check(0x8300)?;
    c.literal(R::R0, 0x8318)?;
    c.check(0x8304)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x304)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 64)?)?;
    c.branch(C::Ne, 0x8304)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    masked_config(&mut c, 114, 14080, 1, 0x8348)?;
    c.check(0x8348)?;
    c.literal(R::R0, 0x8360)?;
    c.emit(mov(R::R1, 51)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 1024)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, 0x304)?)?;
    ret(&mut c)?;
    c.emit(SECONDARY_CONTROLLER)?;
    c.check(0x8364)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn secondary_controller_registers_have_contiguous_encoding() {
        let mut bytes = vec![0; 0x8148-0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x8364-0x1000);
    }
    #[test]
    fn secondary_controller_registration_is_strict() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
