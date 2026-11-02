// SPDX-License-Identifier: GPL-3.0-only
//! Early Venus platform-control register accessors.
//!
//! Firmware-specific MMIO offsets and literal addresses are named data;
//! the ARM instructions themselves are assembled from checked ISA fields.
use arm32::{
    bx, data_immediate, data_register, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{load, mov, store};

const CONTROLLER: u32 = 0x0010_3850;
const CONTROL: i32 = 0xe80;
const CONTROL_COMMAND: i32 = 0xe84;
const CONTROL_STATUS: i32 = 0xe8c;
const MASK_HIGH: u32 = 0x00ff_0000;

fn and_shift(c: &mut Code<'_>, rd: R, lhs: R, rhs: R,
             shift: Shift, amount: u8) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::And, rd, lhs, rhs, shift, amount, false)?)
}

fn read_flag(c: &mut Code<'_>, shifted: Option<u8>) -> Result<(), String> {
    let address = c.at + if shifted.is_some() { 24 } else { 20 };
    c.literal(R::R0, address)?;
    if shifted.is_some() { c.emit(mov(R::R1, 1)?)?; }
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, CONTROL)?)?;
    match shifted {
        Some(bits) => and_shift(c, R::R0, R::R1, R::R0, Shift::Lsr, bits)?,
        None => c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 1)?)?,
    }
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    Ok(())
}

/// Preserve selected control-state bits, optionally asserting another bit.
/// Each routine has an inlined constant-address literal pool word.
fn update_control(c: &mut Code<'_>, preserve_low: u32,
                  assert_bit: Option<u32>) -> Result<(), String> {
    let literal = c.at + if assert_bit.is_some() { 40 } else { 36 };
    c.literal(R::R0, literal)?;
    c.emit(mov(R::R2, preserve_low)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, MASK_HIGH)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, CONTROL)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    and_shift(c, R::R1, R::R1, R::R2, Shift::Lsl, 0)?;
    if let Some(bit) = assert_bit {
        c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, bit)?)?;
    }
    c.emit(store(R::R1, R::R0, CONTROL)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    Ok(())
}

fn write_all_ones(c: &mut Code<'_>, offset: i32) -> Result<(), String> {
    let literal = c.at + 20;
    c.literal(R::R0, literal)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, offset)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    Ok(())
}

fn update_config(c: &mut Code<'_>, set_bit: bool) -> Result<(), String> {
    let literal = c.at + if set_bit { 32 } else { 28 };
    c.literal(R::R0, literal)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 0xd64)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 0xf6)?)?;
    if set_bit { c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 1)?)?; }
    c.emit(store(R::R1, R::R0, 0xd64)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6afc };
    read_flag(&mut c, Some(23))?;
    c.check(0x6b18)?;
    read_flag(&mut c, None)?;
    c.check(0x6b30)?;
    // Apply the caller's frame-enable field without overwriting reserved
    // register bits. Controller addresses stay data, not executable words.
    c.emit(mov(R::R1, 0x007f_0000)?)?;
    c.emit(mov(R::new(3)?, 39)?)?;
    and_shift(&mut c, R::R0, R::R1, R::R0, Shift::Lsl, 16)?;
    c.literal(R::R1, 0x6b60)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x0080_0000)?)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, CONTROL)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    and_shift(&mut c, R::R2, R::R2, R::new(3)?, Shift::Lsl, 0)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, R::R1, CONTROL)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6b64)?;
    read_flag(&mut c, Some(5))?;
    c.check(0x6b80)?;
    for (at, mask, set) in [
        (0x6b80, 7, Some(32)),
        (0x6bac, 7, None),
        (0x6bd4, 37, Some(2)),
        (0x6c00, 37, None),
    ] {
        c.check(at)?;
        update_control(&mut c, mask, set)?;
    }
    c.check(0x6c28)?;
    c.literal(R::R0, 0x6c3c)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, CONTROL_COMMAND)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6c40)?;
    c.literal(R::R0, 0x6c50)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(load(R::R0, R::R0, CONTROL_STATUS)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6c54)?;
    c.literal(R::R1, 0x6c6c)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(load(R::R2, R::R2, CONTROL_STATUS)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, CONTROL_STATUS)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x6c70)?;
    write_all_ones(&mut c, 0xd00)?;
    c.check(0x6c88)?;
    write_all_ones(&mut c, 0xd20)?;
    c.check(0x6ca0)?;
    update_config(&mut c, true)?;
    c.check(0x6cc4)?;
    update_config(&mut c, false)?;
    c.check(0x6ce4)?;
    read_flag(&mut c, Some(2))?;
    c.check(0x6d00)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mmio_register_helpers_are_contiguous_and_typed() {
        let mut data = vec![0; 0x6afc - 0x1000];
        extend(&mut data).unwrap();
        assert_eq!(data.len(), 0x6d00 - 0x1000);
        assert_eq!(u32::from_le_bytes(data[0x6b14 - 0x1000..0x6b18 - 0x1000].try_into().unwrap()), CONTROLLER);
        assert_eq!(u32::from_le_bytes(data[0x6cfc - 0x1000..0x6d00 - 0x1000].try_into().unwrap()), CONTROLLER);
    }
    #[test]
    fn mmio_helpers_reject_gaps() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
