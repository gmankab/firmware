// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller range allocator and mode-selection helpers.
//! This implements the observed arithmetic and branch structure using typed
//! ARM A32 operations; reference firmware instructions are not embedded.
use arm32::{
    branch_exchange_immediate, bx, data_immediate,
    data_register, data_register_shifted_by_register, mov_register,
    mov_shifted_register, multiply_register, push_registers,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

const CONTROLLER: u32 = 0x0010_3850;
const RANGE_CONFIG: i32 = 0xe60;
const MODE_CONFIG: i32 = 0xe64;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6ee4 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() |
        r9.bit() | R::R11.bit() | R::LR.bit();
    let restored = saved & !R::LR.bit() | R::PC.bit();

    // Normalize a requested controller range to a power-of-two sized bank,
    // preserve the caller's mode flags and return its effective address span.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov(r8, 0x0ff0_0000)?)?;
    c.emit(mov_register(C::Al, r9, r3))?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, r5, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r8, r8, 0xf000_0000)?)?;
    c.emit(cmp(R::R2, 3)?)?;
    c.branch(C::Ne, 0x6f44)?;
    c.emit(mov_shifted_register(C::Al, r4, R::R1, Shift::Lsr, 5)?)?;
    c.emit(mov_shifted_register(C::Al, r7, R::R0, Shift::Lsr, 5)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R1, r4, R::R0, Shift::Lsr, 5, false)?)?;
    c.emit(add(r6, R::R1, 1)?)?;
    c.emit(data_register(C::Al, Op::Tst, R::R0, r6, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x6f64)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Eq, 0x6f60)?;
    c.check(0x6f28)?;
    c.emit(sub(R::R0, r6, 1)?)?;
    c.emit(data_register(C::Al, Op::And, r6, R::R0, r6, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Ne, Op::Sub, R::R0, r6, 1)?)?;
    c.emit(data_register(C::Ne, Op::And, R::R0, R::R0, r6, Shift::Lsl, 0, true)?)?;
    c.branch(C::Ne, 0x6f28)?;
    c.emit(mov_shifted_register(C::Al, r6, r6, Shift::Lsl, 1)?)?;
    c.branch(C::Al, 0x6f64)?;
    c.check(0x6f44)?;
    c.literal(R::R0, 0x6fe0)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, RANGE_CONFIG)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1, r8, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R1, R::R0, RANGE_CONFIG)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, restored, BlockMode::IncrementAfter, true)?)?;
    c.check(0x6f60)?;
    c.emit(mov(r6, 0)?)?;
    c.check(0x6f64)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.call(0x81850)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r7, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r6, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, r4, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, R::R1, R::R0, r6,
        Shift::Lsl, R::R2, false)?)?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.check(0x6f94)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(multiply_register(C::Al, R::R2, R::R0, r6)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, r6, Shift::Lsl, 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, r7, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x6f8c)?;
    c.check(0x6fa8)?;
    c.literal(R::R0, 0x6fe0)?;
    c.emit(add(R::R2, r8, 3)?)?;
    c.emit(data_register(C::Al, Op::And, R::R2, r5, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, RANGE_CONFIG)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, r6, r7, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, R::R2, R::R1, Shift::Lsl, 20, false)?)?;
    c.emit(store(R::R1, R::R0, RANGE_CONFIG)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, r7, Shift::Lsl, 5)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R0, r6, Shift::Lsl, 5, false)?)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(transfer_multiple(C::Al, false, r9, R::R0.bit() | R::R1.bit(),
        BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, restored, BlockMode::IncrementAfter, true)?)?;
    c.emit(CONTROLLER)?;
    c.check(0x6fe4)?;

    // The next MMIO accessor clears selected mode bits without disturbing
    // the rest of the controller's persistent configuration.
    c.literal(R::R0, 0x7008)?;
    c.emit(mov(R::R2, 0x003f_8000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x0040_0000)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R1, R::R1, MODE_CONFIG)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R1, R::R0, MODE_CONFIG)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(CONTROLLER)?;
    c.check(0x700c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controller_allocator_and_mode_clear_are_contiguous() {
        let mut bytes = vec![0; 0x6ee4 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x700c - 0x1000);
        assert_eq!(u32::from_le_bytes(bytes[0x6fe0 - 0x1000..0x6fe4 - 0x1000].try_into().unwrap()), CONTROLLER);
    }
    #[test]
    fn controller_allocator_cannot_skip_prior_sections() {
        assert!(extend(&mut vec![]).is_err());
    }
}
