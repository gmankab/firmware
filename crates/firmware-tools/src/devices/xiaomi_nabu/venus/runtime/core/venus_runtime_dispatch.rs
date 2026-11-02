// SPDX-License-Identifier: GPL-3.0-only
//! Venus resource discovery and dispatch initialization. This is a bounded
//! fragment of a larger interrupt-processing loop, not a complete driver.
use arm32::{
    data_immediate, mov_register, push_registers,
    Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2260 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() |
        r9.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.literal(R::R0, 0x24b0)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2284)?;
    c.call(0x257c0)?;
    c.emit(load(R::R0, r4, 0x48)?)?;
    c.call(0x3078)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x2370)?;
    c.check(0x2298)?;

    // Check eight ordered component labels; each failure has a dedicated
    // status slot, while the success path continues into queue processing.
    const SERVICE_LABELS: [usize; 8] = [
        0x24b4, 0x24bc, 0x24c4, 0x24cc,
        0x24d4, 0x24dc, 0x24e4, 0x24ec,
    ];
    const FAILURE_LABELS: [usize; 8] = [
        0x239c, 0x23a4, 0x23ac, 0x23b4,
        0x23bc, 0x23c4, 0x23cc, 0x23d4,
    ];
    for (index, (label, failure)) in SERVICE_LABELS.into_iter()
        .zip(FAILURE_LABELS).enumerate() {
        c.check(0x2298 + 24 * index)?;
        c.literal(R::R1, label)?;
        c.emit(mov_register(C::Al, R::R0, r5))?;
        c.emit(mov(R::R2, 7)?)?;
        c.call(0x325c)?;
        c.emit(cmp(R::R0, 0)?)?;
        c.branch(C::Eq, failure)?;
    }
    c.check(0x2358)?;
    c.literal(R::R1, 0x24f4)?;
    c.literal(R::R2, 0x24f8)?;
    c.emit(store(r5, R::SP, 0)?)?;
    c.emit(mov(R::R0, 8)?)?;
    c.emit(mov(R::new(3)?, 248)?)?;
    c.branch(C::Al, 0x2384)?;
    c.check(0x2370)?;
    c.literal(R::R1, 0x2500)?;
    c.literal(R::R2, 0x24f8)?;
    c.emit(mov(R::new(3)?, 2)?)?;
    c.emit(mov(R::R0, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 256)?)?;
    c.check(0x2384)?;
    c.call(0x2ec94)?;
    c.literal(R::R1, 0x24fc)?;
    c.emit(mov(R::R0, 8)?)?;
    c.call(0x2ec94)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x23dc)?;
    c.check(0x239c)?;

    // Each failed label test names the corresponding diagnostic state slot.
    for (at, literal) in [
        (0x239c, 0x24b8), (0x23a4, 0x24c0),
        (0x23ac, 0x24c8), (0x23b4, 0x24d0),
        (0x23bc, 0x24d8), (0x23c4, 0x24e0),
        (0x23cc, 0x24e8),
    ] {
        c.check(at)?;
        c.literal(R::R0, literal)?;
        c.branch(C::Al, 0x23d8)?;
    }
    c.check(0x23d4)?;
    c.literal(R::R0, 0x24f0)?;
    c.emit(store(r5, R::R0, 0)?)?;
    c.call(0x307c)?;
    c.literal(r9, 0x2504)?;
    c.emit(mov(r8, 255)?)?;
    c.emit(add(r6, r4, 4)?)?;
    c.emit(mov(r5, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r8, r8, 65280)?)?;
    c.check(0x23f4)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_prefix_fails_loudly() { assert!(extend(&mut vec![]).is_err()); }
}
