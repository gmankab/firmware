// SPDX-License-Identifier: GPL-3.0-only
//! Runtime service handles and callback table traversal.
//! Handles carry a tagged low bit; dispatch checks the associated descriptor
//! before calling a service and releases partially initialized objects.
use arm32::{
    branch_exchange_with_link, bx, data_immediate, data_register, mov_register,
    push_registers, transfer_multiple, transfer_word_postindexed,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x50c0 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r12 = R::R12;
    let saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    let return_regs = (saved & !R::LR.bit()) | R::PC.bit();

    // Initialize an object, query its class, and conditionally call its
    // start method. Roll back the allocated object if that callback fails.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.call(0x513c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x50e8)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, return_regs,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x50e8)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(load(R::R1, R::R0, 8)?)?;
    c.emit(load(R::R1, R::R1, 12)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(cmp(R::R1, 1)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Ne, true, R::SP, return_regs,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, R::R0, 1)?)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(R::R1, R::R1, 8)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, return_regs,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x51cc)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, return_regs,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x513c)?;

    // Resolve a service implementation from a table of named descriptors.
    // Each group describes zero or more interfaces, checked by identity.
    let resolve_saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, resolve_saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r12, R::R1))?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.literal(R::R0, 0x51c8)?;
    c.emit(load(R::R2, R::R0, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r12, 0)?)?;
    c.emit(load(R::LR, R::R2, 0)?)?;
    c.emit(cmp(R::LR, 0)?)?;
    c.branch(C::Eq, 0x51ac)?;
    c.emit(load(R::R2, R::R2, 4)?)?;
    c.check(0x516c)?;
    c.emit(transfer_word_postindexed(C::Al, true, r3, R::R2, 4)?)?;
    c.emit(load(r4, r3, 4)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x51a0)?;
    c.emit(load(r5, r3, 8)?)?;
    c.emit(mov(r6, 0)?)?;
    c.check(0x5184)?;
    c.emit(load(r7, r5, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x51b4)?;
    c.emit(add(r6, r6, 1)?)?;
    c.emit(add(r5, r5, 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r6, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x5184)?;
    c.check(0x51a0)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::LR, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x516c)?;
    c.check(0x51ac)?;
    c.emit(mov(R::R0, 0x8000_0006)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (resolve_saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x51b4)?;
    c.emit(load(r3, r3, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, r12))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        resolve_saved, BlockMode::IncrementAfter, true)?)?;
    c.emit(bx(C::Al, r3))?;
    c.emit(0x0010_cfa0)?;
    c.check(0x51cc)?;

    // Release a typed handle. The optional callback precedes the generic
    // release method, and the tagged pointer bit is removed only for access.
    let release_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, release_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(load(R::R0, r4, 8)?)?;
    c.emit(load(R::R0, R::R0, 12)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 1)?)?;
    c.branch(C::Ne, 0x51fc)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, r4, 1)?)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(R::R1, R::R1, 12)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R1)?)?;
    c.check(0x51fc)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, r4, 1)?)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(R::R1, R::R1, 4)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (release_saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x5214)?;
    Ok(())
}
