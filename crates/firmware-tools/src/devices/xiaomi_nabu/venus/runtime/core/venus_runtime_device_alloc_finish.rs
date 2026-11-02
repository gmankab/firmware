// SPDX-License-Identifier: GPL-3.0-only
//! Finalize event/DMA descriptor allocation and rollback on failure. The
//! physical-address packing is represented with typed bitfield operations.
use arm32::{
    data_immediate, data_register, mov_shifted_register,
    transfer_word_postindexed,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x6710 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.emit(load(R::R1, R::SP, 36)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(load(R::R2, R::R0, 20)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, R::R2, R::R1,
        Shift::Lsl, 8, false)?)?;
    c.emit(store(R::R2, r9, 0)?)?;
    c.branch(C::Eq, 0x6750)?;
    c.emit(load(R::R2, R::R0, 0)?)?;
    c.emit(load(r3, r6, 88)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R2,
        Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R2, 16646144)?)?;
    c.emit(data_register(C::Al, Op::And, R::R2, R::R2, R::R1,
        Shift::Lsl, 17, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r3, R::R1,
        Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R1, R::R1, 16646144)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, R::R1, R::R2,
        Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.check(0x6750)?;
    c.emit(load(R::R1, R::R0, 16)?)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(store(R::R1, R::R0, 16)?)?;
    c.emit(load(R::R0, r6, 120)?)?;
    c.call(0x59e8)?;
    c.emit(mov(r7, 0)?)?;
    c.branch(C::Al, 0x6704)?;
    c.check(0x676c)?;
    c.emit(load(R::R0, r7, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r7, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.emit(load(R::R2, R::R11, -32)?)?;
    c.emit(load(R::R0, r6, 88)?)?;
    c.emit(transfer_word_postindexed(C::Al, true, r3, R::R2, 12)?)?;
    c.emit(store(R::R2, R::SP, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, R::R0, r3,
        Shift::Lsl, 8, false)?)?;
    c.emit(mov(R::R0, 3200)?)?;
    c.emit(store(R::R1, R::SP, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(mov(r3, 4096)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 524288)?)?;
    c.call(0x573c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x67e4)?;
    c.emit(load(R::R1, r10, 4)?)?;
    c.emit(load(R::R0, r10, 8)?)?;
    c.emit(load(R::R2, R::SP, 12)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    c.call(0x69fc)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.call(0x5838)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.call(0x5838)?;
    c.emit(load(R::R0, r6, 120)?)?;
    c.call(0x59e8)?;
    c.branch(C::Al, 0x6704)?;
    c.check(0x67e4)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.emit(add(R::R1, R::SP, 16)?)?;
    c.emit(load(R::R0, R::R0, 12)?)?;
    c.call(0x5888)?;
    c.emit(load(R::R0, R::R11, -32)?)?;
    c.emit(load(R::R1, R::SP, 16)?)?;
    c.emit(cmp(r8, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, r5, r8, 28)?)?;
    c.emit(store(R::R1, R::R0, 20)?)?;
    c.emit(store(R::R0, r5, 0)?)?;
    c.branch(C::Al, 0x66e0)?;
    c.check(0x6810)?;
    Ok(())
}
