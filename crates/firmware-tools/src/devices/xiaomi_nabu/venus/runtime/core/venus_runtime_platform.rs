// SPDX-License-Identifier: GPL-3.0-only
//! Early platform controller and power-mode wrappers for Venus.
//! Device addresses are constants and ISA instructions are encoded via arm32.
use arm32::{
    bx, data_immediate, data_register, mov_register,
    transfer_word, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{load, mov, store};

const CONTROLLER: u32 = 0x0010_3850;
const DEVICE_STATE: u32 = 0x0010_3854;
const CONTROL: i32 = 0xe80;
const PROTECTED_MASK: u32 = 0x00ff_0000;

fn regop(c: &mut Code<'_>, op: Op, rd: R, rn: R,
         rm: R, shift: Shift, bits: u8) -> Result<(), String> {
    c.emit(data_register(C::Al, op, rd, rn, rm, shift, bits, false)?)
}

/// Perform a hardware register update only for the known controller revision.
fn gated_control_update(c: &mut Code<'_>, set_bit: bool) -> Result<(), String> {
    let at = c.at;
    let skipped = at + if set_bit { 0x3c } else { 0x38 };
    c.literal(R::R0, skipped + 4)?;
    c.emit(load(R::R1, R::R0, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 6)?)?;
    c.emit(transfer_word(C::Eq, true, R::R0, R::R0, 0x34)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, R::R0, R::R0, 3)?)?;
    c.branch(C::Ne, skipped)?;
    c.literal(R::R0, skipped + 8)?;
    c.emit(mov(R::R2, 35)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, PROTECTED_MASK)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, CONTROL)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    regop(c, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0)?;
    if set_bit { c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 4)?)?; }
    c.emit(store(R::R1, R::R0, CONTROL)?)?;
    c.check(skipped)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(DEVICE_STATE)?;
    c.emit(CONTROLLER)?;
    Ok(())
}

/// Write a command word into the controller unlock/handshake register.
fn write_handshake(c: &mut Code<'_>, magic: u32) -> Result<(), String> {
    let at = c.at;
    c.literal(R::R0, at + 20)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.literal(R::R1, at + 24)?;
    c.emit(store(R::R1, R::R0, 0xfb0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.emit(magic)?;
    Ok(())
}

fn set_small_selector(c: &mut Code<'_>, offset: i32) -> Result<(), String> {
    let lit = c.at + 28;
    c.literal(R::R1, lit)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 15)?)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, offset)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, offset)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6d00 };
    gated_control_update(&mut c, true)?;
    c.check(0x6d48)?;
    // A bounded index into the eight-byte platform state entries.
    c.literal(R::R2, 0x6d60)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 11)?)?;
    c.emit(data_register(C::Cc, Op::Add, R::R1, R::R2, R::R0, Shift::Lsl, 3, false)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(DEVICE_STATE)?;
    c.check(0x6d64)?;
    gated_control_update(&mut c, false)?;
    c.check(0x6da8)?;
    // Clamp a command argument before constructing the packed 0xe90 control.
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.branch(C::Ne, 0x6dc0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 27)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, R::R1, R::R0, 27)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 12)?)?;
    c.emit(data_immediate(C::Ls, Op::Mov, R::R1, R::R0, 12)?)?;
    c.check(0x6dc0)?;
    c.emit(mov(R::R2, 4096)?)?;
    regop(&mut c, Op::And, R::R0, R::R2, R::R0, Shift::Lsl, 12)?;
    c.emit(mov(R::R2, 1020)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 3072)?)?;
    regop(&mut c, Op::And, R::R1, R::R2, R::R1, Shift::Lsl, 2)?;
    regop(&mut c, Op::Orr, R::R0, R::R1, R::R0, Shift::Lsl, 0)?;
    c.literal(R::R1, 0x6de8)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, 0xe90)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6dec)?;
    write_handshake(&mut c, 0xc5ac_ce55)?;
    c.check(0x6e08)?;
    write_handshake(&mut c, 0x3a53_31aa)?;
    c.check(0x6e24)?;
    // Read the sleep status bit from the platform wakeup register.
    c.literal(R::R0, 0x6e3c)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 0xfb4)?)?;
    regop(&mut c, Op::And, R::R0, R::R1, R::R0, Shift::Lsr, 1)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6e40)?;
    set_small_selector(&mut c, 0xfa0)?;
    c.check(0x6e60)?;
    set_small_selector(&mut c, 0xfa4)?;
    c.check(0x6e80)?;
    c.emit(mov(R::R0, 15)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x6e88)?;
    c.literal(R::R0, 0x6e9c)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 0xfa4)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 15)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6ea0)?;
    c.literal(R::R1, 0x6eb8)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, 0xe00)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, 0xe00)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6ebc)?;
    c.literal(R::R0, 0x6ee0)?;
    c.emit(mov(R::R2, 0x0ff0_0000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf000_0000)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0xe60)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    regop(&mut c, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0)?;
    c.emit(store(R::R1, R::R0, 0xe60)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6ee4)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_platform_controls_follow_mmio_region() {
        let mut data = vec![0; 0x6d00 - 0x1000];
        extend(&mut data).unwrap();
        assert_eq!(data.len(), 0x6ee4 - 0x1000);
        assert_eq!(&data[0x6e04-0x1000..0x6e08-0x1000], &0xc5ac_ce55u32.to_le_bytes());
        assert_eq!(&data[0x6e20-0x1000..0x6e24-0x1000], &0x3a53_31aau32.to_le_bytes());
    }
    #[test]
    fn rejects_noncontiguous_platform_block() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
