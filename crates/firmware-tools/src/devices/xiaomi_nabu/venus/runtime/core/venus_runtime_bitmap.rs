// SPDX-License-Identifier: GPL-3.0-only
//! Bitmap allocation primitives: initialize a bounded bitset and find/set
//! its next free bit, keeping all 32-bit operands and loops explicit.
use arm32::{
    data_immediate, data_register, data_register_shifted_by_register,
    mov_register, mov_shifted_register, push_registers,
    transfer_multiple, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6914 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    let ret = (saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(add(R::R0, R::R1, 31)?)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r5,
        R::R0, Shift::Lsr, 5, false)?)?;
    c.branch(C::Eq, 0x6964)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 5)?)?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.emit(mov_shifted_register(C::Al, r6, R::R0, Shift::Lsl, 2)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x57d4)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r5, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6964)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.emit(mov(r5, 0)?)?;
    c.call(0x31ec)?;
    c.check(0x6964)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x696c)?;
    // Walk a bitset; the first vacant bit below the supplied upper bound is
    // recorded and atomically reflected in its containing word.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(add(r3, R::R1, 31)?)?;
    c.emit(mov(R::LR, 0)?)?;
    c.emit(mov_register(C::Al, R::R12, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::LR,
        r3, Shift::Lsr, 5, false)?)?;
    c.branch(C::Eq, 0x69d4)?;
    c.emit(mov_shifted_register(C::Al, r4, r3, Shift::Lsr, 5)?)?;
    c.emit(mov(r3, 0)?)?;
    c.check(0x6994)?;
    c.emit(transfer_word_register(C::Al, true, r7, R::R12, r3, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmn, R::R0, r7, 1)?)?;
    c.branch(C::Eq, 0x69c4)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(mov_register(C::Al, r6, R::LR))?;
    c.check(0x69a8)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r7, 1)?)?;
    c.branch(C::Eq, 0x69d8)?;
    c.emit(add(r5, r5, 1)?)?;
    c.emit(add(r6, r6, 1)?)?;
    c.emit(mov_shifted_register(C::Al, r7, r7, Shift::Lsr, 1)?)?;
    c.emit(cmp(r5, 32)?)?;
    c.branch(C::Cc, 0x69a8)?;
    c.check(0x69c4)?;
    c.emit(add(r3, r3, 1)?)?;
    c.emit(add(R::LR, R::LR, 32)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3,
        r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x6994)?;
    c.check(0x69d4)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x69d8)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r6,
        R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Cs, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(store(r6, R::R2, 0)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R12, r3,
        Shift::Lsl, 2)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr,
        R::R0, R::R0, R::R1, Shift::Lsl, r5, false)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R0, R::R12, r3,
        Shift::Lsl, 2)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x69fc)?;
    Ok(())
}
