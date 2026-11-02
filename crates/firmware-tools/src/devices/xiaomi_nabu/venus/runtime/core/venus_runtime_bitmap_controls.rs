// SPDX-License-Identifier: GPL-3.0-only
//! Bitmap release operations and platform register-control wrappers.
//! MMIO addresses are device metadata; instruction forms are generic A32.
use arm32::{
    bx, data_immediate, data_register, data_register_shifted_by_register,
    mov_register, push_registers, transfer_multiple, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x69fc };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let saved = r4.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2,
        R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Hi, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(R::R2, 33)?)?;
    c.branch(C::Cc, 0x6a3c)?;
    c.emit(sub(R::R12, R::R2, 33)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(sub(R::LR, R::R2, 32)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R2, R::R12, 31)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1,
        R::R12, Shift::Lsr, 5, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R2, R::LR,
        R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6a40)?;
    c.check(0x6a3c)?;
    c.emit(mov(R::R1, 0)?)?;
    c.check(0x6a40)?;
    c.emit(transfer_word_register(C::Al, true, R::R12, r4,
        R::R1, Shift::Lsl, 2)?)?;
    c.emit(mov(r3, 1)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Bic,
        R::R2, R::R12, r3, Shift::Lsl, R::R2, false)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R2, r4,
        R::R1, Shift::Lsl, 2)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x6a54)?;
    // Record the global context and clear its startup control flag.
    c.literal(R::R1, 0x6a80)?;
    c.emit(mov(r3, 3)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.literal(R::R1, 0x6a84)?;
    c.emit(load(R::R0, R::R0, 3744)?)?;
    c.emit(store(R::R2, R::R1, 0)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, r3, R::R0,
        Shift::Lsr, 22, false)?)?;
    c.emit(store(R::R0, R::R1, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0x0010_3850)?;
    c.emit(0x0010_3854)?;
    c.check(0x6a88)?;
    c.literal(R::R1, 0x6a94)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0x0010_3850)?;
    c.check(0x6a98)?;
    // Read and update a memory controller state word, preserve reserved bits.
    c.literal(R::R0, 0x6ac0)?;
    c.emit(mov(R::R2, 38)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 16711680)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 3712)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1,
        R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 1)?)?;
    c.emit(store(R::R1, R::R0, 3712)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0x0010_3850)?;
    c.check(0x6ac4)?;
    c.literal(R::R0, 0x6af8)?;
    c.emit(mov(R::R2, 38)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 16711680)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 3712)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1,
        R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(load(R::R2, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R2, 3712)?)?;
    c.check(0x6ae4)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, 3712)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R1, 8388608)?)?;
    c.branch(C::Ne, 0x6ae4)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0x0010_3850)?;
    c.check(0x6afc)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_register_accessors_have_exact_layout() {
        let mut bytes = vec![0; 0x69fc - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x6afc - 0x1000);
        let word = |at: usize| {
            let offset = at - 0x1000;
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
        };
        assert_eq!(word(0x6a80), 0x0010_3850);
        assert_eq!(word(0x6a84), 0x0010_3854);
        assert_eq!(word(0x6a94), 0x0010_3850);
        assert_eq!(word(0x6af8), 0x0010_3850);
    }

    #[test]
    fn rejects_bitmap_control_layout_gaps() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
