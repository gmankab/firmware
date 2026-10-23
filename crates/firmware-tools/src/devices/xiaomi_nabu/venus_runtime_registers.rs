// SPDX-License-Identifier: GPL-3.0-only
//! Typed ARM controller-register accessors and bounded status polling.
use arm32::{
    bx, data_immediate, data_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{load, mov, store};

const CONTROLLER_POINTER: u32 = 0x0010_cfac;
const MMIO_TEST_TOKEN: u32 = 0xc5ac_ce55;
const MMIO_RESET_TOKEN: u32 = 0x3a53_31aa;
const SHARED_MAPPED_POINTER: u32 = 0x0010_cfb0;

fn exit(c: &mut Code<'_>) -> Result<(), String> { c.emit(bx(C::Al, R::LR)) }
fn controller_addr(c: &mut Code<'_>, rd: R, at: usize) -> Result<(), String> {
    c.literal(rd, at)
}
fn and_register(c: &mut Code<'_>, rd: R, lhs: R, rhs: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::And, rd, lhs, rhs, Shift::Lsl, 0, false)?)
}
fn set_field(c: &mut Code<'_>, offset: i32, end: usize) -> Result<(), String> {
    controller_addr(c, R::R1, end-4)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, offset)?)?;
    exit(c)?;
    c.emit(CONTROLLER_POINTER)
}
fn get_field(c: &mut Code<'_>, offset: i32, end: usize) -> Result<(), String> {
    controller_addr(c, R::R0, end-4)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, offset)?)?;
    exit(c)?;
    c.emit(CONTROLLER_POINTER)
}
fn write_auth_token(c: &mut Code<'_>, token: u32, end: usize) -> Result<(), String> {
    controller_addr(c, R::R0, end-8)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    controller_addr(c, R::R1, end-4)?;
    c.emit(store(R::R1, R::R0, 0xfb0)?)?;
    exit(c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.emit(token)
}
fn write_mode(c: &mut Code<'_>, value: u32, end: usize) -> Result<(), String> {
    controller_addr(c, R::R0, end-4)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 32)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(mov(R::R1, value)?)?;
    c.emit(store(R::R1, R::R0, 32)?)?;
    exit(c)?;
    c.emit(CONTROLLER_POINTER)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7f18 };
    // Publish the active controller pointer for service discovery.
    c.literal(R::R1, 0x7f24)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x7f28)?;
    write_auth_token(&mut c, MMIO_TEST_TOKEN, 0x7f44)?;
    write_auth_token(&mut c, MMIO_RESET_TOKEN, 0x7f60)?;
    c.check(0x7f60)?;
    // Update the controller's low two mode bits.
    controller_addr(&mut c, R::R1, 0x7f7c)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 3)?)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, 40)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, 40)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x7f80)?;
    // Read the two-bit mode selector.
    c.literal(R::R0, 0x7f94)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 40)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 3)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x7f98)?;
    // Read the controller config while masking its inaccessible high bit.
    c.literal(R::R0, 0x7fac)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x8000_0000)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x7fb0)?;
    set_field(&mut c, 24, 0x7fc4)?;
    c.check(0x7fc4)?;
    // Extract only the supported 16-bit controller field.
    c.literal(R::R0, 0x7fe0)?;
    c.emit(mov(R::R1, 255)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xff00)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 24)?)?;
    and_register(&mut c, R::R0, R::R0, R::R1)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x7fe4)?;
    set_field(&mut c, 28, 0x7ff8)?;
    set_field(&mut c, 20, 0x800c)?;
    c.check(0x800c)?;
    c.literal(R::R0, 0x8028)?;
    c.emit(mov(R::R1, 255)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xff00)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 20)?)?;
    and_register(&mut c, R::R0, R::R0, R::R1)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x802c)?;
    set_field(&mut c, 36, 0x8040)?;
    get_field(&mut c, 16, 0x8054)?;
    write_mode(&mut c, 0, 0x8074)?;
    write_mode(&mut c, 1, 0x8094)?;
    c.check(0x8094)?;
    set_field(&mut c, 0x304, 0x80a8)?;
    c.check(0x80a8)?;
    // Set the controller bits required before initiating transfer.
    c.literal(R::R0, 0x80c0)?;
    c.emit(mov(R::R1, 51)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 1024)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, 0x304)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x80c4)?;
    // Retain only safe status bits when setting the handshake-complete bit.
    c.literal(R::R0, 0x80ec)?;
    c.emit(mov(R::R2, 51)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 30_464)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x304)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    and_register(&mut c, R::R1, R::R1, R::R2)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 64)?)?;
    c.emit(store(R::R1, R::R0, 0x304)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x80f0)?;
    // Poll until the controller clears the completion bit.
    c.literal(R::R0, 0x8108)?;
    c.check(0x80f4)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x304)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 64)?)?;
    c.branch(C::Ne, 0x80f4)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x810c)?;
    c.literal(R::R0, 0x8134)?;
    c.emit(mov(R::R2, 114)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 30_464)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0x304)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    and_register(&mut c, R::R1, R::R1, R::R2)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 1)?)?;
    c.emit(store(R::R1, R::R0, 0x304)?)?;
    exit(&mut c)?;
    c.emit(CONTROLLER_POINTER)?;
    c.check(0x8138)?;
    c.literal(R::R1, 0x8144)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    exit(&mut c)?;
    c.emit(SHARED_MAPPED_POINTER)?;
    c.check(0x8148)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controller_register_read_write_helpers_have_no_gaps() {
        let mut bytes = vec![0; 0x7f18 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x8148 - 0x1000);
    }
    #[test]
    fn controller_register_read_write_helpers_are_anchored() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
