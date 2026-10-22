// SPDX-License-Identifier: GPL-3.0-only
//! Venus power-of-two-aligned allocation and the paired deallocation path.
//! The platform allocator is called by address, while the alignment
//! arithmetic and ARM32 machine code are generated from typed operands.
use arm32::{
    data_immediate, data_register, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2fa0 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    // Caller-provided alignment is required to be a nonzero power of two.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.literal(R::R2, 0x303c)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(sub(r7, r4, 1)?)?;
    c.emit(data_register(C::Al, Op::Tst, R::R0, r7,
        r4, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(cmp(r6, 0)?)?;
    c.emit(data_immediate(C::Gt, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R1,
        R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R1, 99)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 512)?)?;
    c.call(0x17b0)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r6,
        r4, Shift::Lsl, 0, false)?)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(add(R::R0, R::R0, 3)?)?;
    c.call(0x57d4)?;
    c.emit(load(r6, R::SP, 4)?)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Eq, 0x3030)?;
    c.literal(R::R2, 0x303c)?;
    c.emit(mov(R::R1, 109)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 512)?)?;
    c.call(0x17b0)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7,
        r6, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, R::R1, r4, 0)?)?;
    c.emit(add(R::R0, R::R0, 4)?)?;
    c.emit(data_register(C::Al, Op::And, r5, R::R0,
        R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r5,
        r6, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, r5, -4)?)?;
    c.check(0x3030)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_290a)?; // Allocator diagnostic metadata
    c.check(0x3040)?;

    // Release allocated memory after recovering the original base offset.
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R1, R::R0, -4)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R0,
        R::R1, Shift::Lsl, 0, false)?)?;
    c.call(0x5838)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x3070)?;
    // System-service branch vector, encoded as PC-relative A32 branches.
    for target in [0x8e98, 0xe2248, 0x5eac, 0x5ec0, 0x5ed8] {
        c.branch(C::Al, target)?;
    }
    c.check(0x3084)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_prior_runtime_fails() { assert!(extend(&mut Vec::new()).is_err()); }
}
