// SPDX-License-Identifier: GPL-3.0-only
//! Venus platform power/mode register operations.
use arm32::{
    bx, data_immediate, data_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{cmp, load, mov, store};

const CONTROLLER: u32 = 0x0010_3850;
const POWER_MODE: i32 = 0xe64;
const D64_CONTROL: i32 = 0xd64;
const E70_CONTROL: i32 = 0xe70;

/// Update a device control register preserving selected bits and optionally
/// setting a new bit. The caller retains the exact firmware MMIO ordering.
fn masked_write(c: &mut Code<'_>, offset: i32, preserve: u32,
                set: Option<u32>) -> Result<(), String> {
    let literal = c.at + if set.is_some() { 32 } else { 28 };
    c.literal(R::R0, literal)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, offset)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, preserve)?)?;
    if let Some(bit) = set {
        c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, bit)?)?;
    }
    c.emit(store(R::R1, R::R0, offset)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x700c };
    // Choose between resetting a controller bank, configuring its 15-bit
    // range, and leaving the previous state untouched.
    c.emit(mov(R::R2, 0x003f_8000)?)?;
    c.emit(cmp(R::R1, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x0040_0000)?)?;
    c.branch(C::Eq, 0x7040)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Ne, 0x7068)?;
    c.literal(R::R0, 0x706c)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, POWER_MODE)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R1, R::R0, POWER_MODE)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x7040)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R0, 64)?)?;
    c.emit(cmp(R::R1, 64)?)?;
    c.emit(bx(C::Ne, R::LR))?;
    c.literal(R::R1, 0x706c)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R2, R::R0, Shift::Lsl, 15, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 1)?)?;
    c.emit(load(R::new(3)?, R::R1, 0)?)?;
    c.emit(load(R::new(3)?, R::new(3)?, POWER_MODE)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(store(R::R0, R::R1, POWER_MODE)?)?;
    c.check(0x7068)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x7070)?;
    // Short mode toggles share one checked generator with distinct register
    // masks. This avoids duplicating each instruction sequence.
    for (start, offset, preserve, set) in [
        (0x7070, E70_CONTROL, 15, Some(16)),
        (0x7094, E70_CONTROL, 30, Some(1)),
        (0x70b8, E70_CONTROL, 30, None),
        (0x70d8, D64_CONTROL, 0xf3, Some(4)),
        (0x70fc, D64_CONTROL, 0xf5, Some(2)),
        (0x7120, D64_CONTROL, 0x77, None),
        (0x7140, D64_CONTROL, 0xe7, None),
    ] {
        c.check(start)?;
        masked_write(&mut c, offset, preserve, set)?;
    }
    c.check(0x7160)?;
    // Program a startup range value, followed by a protected power-mode
    // enable. Literal address remains device-specific non-executable data.
    c.literal(R::R0, 0x7194)?;
    c.emit(mov(R::R2, 5632)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(store(R::R2, R::R1, 0xe90)?)?;
    c.emit(mov(R::R2, 35)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x00ff_0000)?)?;
    c.emit(load(R::R1, R::R1, 0xe80)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 4)?)?;
    c.emit(store(R::R1, R::R0, 0xe80)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x7198)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conditionally_masked_modes_are_contiguous() {
        let mut bytes = vec![0; 0x700c - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x7198 - 0x1000);
        assert_eq!(u32::from_le_bytes(bytes[0x7194 - 0x1000..0x7198 - 0x1000].try_into().unwrap()), CONTROLLER);
    }
    #[test]
    fn mode_helpers_reject_missing_previous_code() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
