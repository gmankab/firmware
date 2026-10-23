// SPDX-License-Identifier: GPL-3.0-only
//! Driver state polling, diagnostics sentinel and descriptor lookup.
//! The sentinel uses public ARM rotated-immediate encoding; callbacks and
//! control words are described by named register fields, not opcode copies.
use arm32::{
    bx, data_immediate, data_immediate_with_flags, mov_register, push_registers,
    transfer_byte, transfer_multiple, BlockMode, Condition as C,
    DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5618 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let ret = (saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.call(0x8e88)?;
    c.emit(store(R::R0, r4, 4)?)?;
    c.branch(C::Al, 0x5650)?;
    c.check(0x563c)?;
    c.emit(mov(R::R0, 1)?)?;
    c.call(0x95f0)?;
    c.emit(load(R::R0, r4, 4)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.call(0x94f4)?;
    c.check(0x5650)?;
    c.emit(load(R::R0, r4, 8)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x563c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x5670)?;
    // A completion guard normalizes a negative state via the shared
    // diagnostics service, otherwise recording the magic completion value.
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 1)?)?;
    c.branch(C::Ne, 0x56a4)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Lt, 0x5694)?;
    c.emit(mov(R::R1, 0x00ad_0000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xde00_0000)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.branch(C::Al, 0x56a4)?;
    c.check(0x5694)?;
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x5838)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::LR.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x56a4)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    // Optional pair of service parameters after checking the descriptor flag.
    c.emit(mov_register(C::Al, R::R12, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R12, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.emit(transfer_byte(C::Al, true, r3, R::R12, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r3, 32)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_multiple(C::Ne, false, R::R12, R::R1.bit() | R::R2.bit(),
        BlockMode::IncrementBefore, false)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x56d0)?;
    // Find a matching descriptor in a table with stride 16. Count is
    // decremented with flags, ensuring a bounded search.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(mov_register(C::Al, R::R12, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R12, 0)?)?;
    c.branch(C::Eq, 0x5710)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, R::R2, R::PC, 0x44)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, R::R2, R::R2, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Eq, 0x5710)?;
    c.emit(load(R::LR, R::R2, 0)?)?;
    c.emit(cmp(R::LR, 0)?)?;
    c.emit(arm32::transfer_word(C::Ne, true, R::R2, R::R2, 4)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.branch(C::Ne, 0x5720)?;
    c.check(0x5710)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, R::LR, R::LR, 1, true)?)?;
    c.emit(add(R::R2, R::R2, 16)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x5720)?;
    c.emit(load(r3, R::R2, 4)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, r3, R::R12,
        arm32::Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x5714)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R2, R::R1, 4)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x0010_34b0)?; // Table root pointer
    c.check(0x573c)?;
    Ok(())
}
