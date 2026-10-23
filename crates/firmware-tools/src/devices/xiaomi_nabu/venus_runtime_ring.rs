// SPDX-License-Identifier: GPL-3.0-only
//! Diagnostic circular-buffer writer and simple output status helpers.
//! The two cases update different output banks, then share the wraparound
//! copy algorithm. Branch destinations are modeled as ARM virtual addresses.
use arm32::{
    bx, data_immediate, data_register, mov_register, mov_shifted_register,
    push_registers, transfer_multiple, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4fb8 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 24)?)?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov_register(C::Al, r8, R::R1))?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(sub(r4, r8, 1)?)?;
    c.call(0x3388)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r4, 1)?)?;
    c.branch(C::Hi, 0x5008)?;
    c.emit(cmp(r6, 256)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r6, R::R0, 256)?)?;
    c.call(0x8e98)?;
    c.literal(R::R0, 0x5090)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r4, R::R1, r6, Shift::Lsl, 24, false)?)?;
    c.emit(store(r4, R::R0, 0)?)?;
    c.call(0xe2248)?;
    c.literal(r7, 0x5094)?;
    c.branch(C::Al, 0x5024)?;
    c.check(0x5008)?;
    c.literal(R::R0, 0x5088)?;
    c.literal(r7, 0x508c)?;
    c.emit(cmp(r6, 256)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r6, R::R0, 256)?)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r4, R::R1, r6, Shift::Lsl, 24, false)?)?;
    c.emit(store(r4, R::R0, 0)?)?;
    c.check(0x5024)?;
    c.emit(mov_shifted_register(C::Al, r4, r4, Shift::Lsr, 24)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r4, r6, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x5058)?;
    c.emit(data_register(C::Al, Op::Sub, r6, r6, r4, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.emit(data_immediate(C::Al, Op::Rsb, R::R0, r6, 256)?)?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7, R::R0, Shift::Lsl, 0, false)?)?;
    c.call(0x471c)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r5, r6, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.branch(C::Al, 0x5068)?;
    c.check(0x5058)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r4, r6, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7, R::R0, Shift::Lsl, 0, false)?)?;
    c.check(0x5068)?;
    c.call(0x471c)?;
    c.emit(cmp(r8, 1)?)?;
    c.emit(transfer_multiple(C::Ne, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.literal(R::R2, 0x5098)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 179)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        saved, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x17b0)?;
    c.check(0x5088)?;
    // Output bank descriptors and the log error label.
    for value in [0x0010_36bc, 0x0010_35bc, 0x0010_35b8,
                  0x0010_34b5, 0x000e_3334] { c.emit(value)?; }
    c.check(0x509c)?;
    // Set a pair of output parameter words when the caller supplies pointers.
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 14)?)?;
    c.emit(data_immediate(C::Ne, Op::Orr, R::R2, R::R2, 8960)?)?;
    c.emit(arm32::transfer_word(C::Ne, false, R::R2, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 9)?)?;
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x50c0)?;
    Ok(())
}
