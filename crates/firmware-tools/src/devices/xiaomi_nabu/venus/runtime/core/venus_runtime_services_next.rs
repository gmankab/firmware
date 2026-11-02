// SPDX-License-Identifier: GPL-3.0-only
//! Venus ARM32 cache control, memory diagnostics and service registration.
use arm32::{
    bx, data_immediate, data_register, mcr, mrc, mov_register, push_registers,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op, Register as R,
    Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

fn register_coprocessor_control(c: &mut Code<'_>, clear: bool) -> Result<(), String> {
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.call(0x2110)?;
    c.emit(mrc(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    c.emit(data_immediate(C::Al,
        if clear { Op::Bic } else { Op::Orr }, R::R0, R::R0, 1 << 14)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x20c0 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r7 = R::new(7)?;

    // Three direct CP15 synchronization operations.
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 7, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 10, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 10, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 0, 4)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x20e8)?;

    // Restore the MMU control bits and permit the resulting coprocessor
    // state to settle. ARM NOPs are represented by MOV r0,r0.
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 1)?)?;
    c.emit(mrc(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 4096)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R0, Shift::Lsl, 14, false)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    for _ in 0..3 { c.emit(mov_register(C::Al, R::R0, R::R0))?; }
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x2110)?;

    // Poll architectural flags for CP15 cache synchronization completion.
    c.emit(mrc(C::Al, 15, 0, R::PC, 7, 14, 3)?)?;
    c.branch(C::Ne, 0x2110)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 5, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mcr(C::Al, 15, 0, R::R0, 7, 7, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x212c)?;
    register_coprocessor_control(&mut c, false)?;
    c.check(0x2148)?;
    register_coprocessor_control(&mut c, true)?;
    c.check(0x2164)?;

    // Probe each memory controller region in a bounded loop.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov(r4, 1808)?)?;
    c.emit(mov(R::R0, 0)?)?;
    for (dest, address) in [
        (R::R12, 0x1000_0000), (R::LR, 0x1100_0000),
        (r3, 0x1200_0000), (R::R1, 0x1300_0000),
        (R::R2, 0x1400_0000),
    ] { c.emit(mov(dest, address)?)?; }
    c.emit(data_immediate(C::Al, Op::Orr, r4, r4, 8192)?)?;
    for source in [R::R12, R::LR, r3, R::R1, R::R2] {
        c.emit(load(r5, source, 0)?)?;
    }
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x218c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x21b0)?;
    c.emit(load(R::R0, R::R0, 0x4c)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x21b8)?;

    // Service registration: construct the application-specific parameter
    // record on the stack, invoke setup, and test the returned mapping.
    c.emit(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 32)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.literal(R::R0, 0x2258)?;
    c.emit(store(R::R2, R::SP, 4)?)?;
    c.emit(store(R::R1, R::SP, 8)?)?;
    c.emit(add(R::R1, r4, 72)?)?;
    c.emit(store(R::R0, R::SP, 12)?)?;
    c.emit(mov(R::R0, 8192)?)?;
    c.emit(store(r4, R::SP, 16)?)?;
    c.emit(store(R::R0, R::SP, 20)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 24)?)?;
    c.emit(store(R::R0, R::SP, 28)?)?;
    c.emit(add(R::R0, R::SP, 4)?)?;
    c.call(0x29b4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2214)?;
    c.literal(R::R2, 0x225c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 76)?)?;
    c.call(0x17b0)?;
    c.check(0x2214)?;
    c.emit(load(R::R1, r4, 0x48)?)?;
    c.emit(add(R::R0, r4, 76)?)?;
    c.call(0x2c44)?;
    c.literal(R::R2, 0x225c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 80)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x2b8c)?;
    c.emit(add(R::R0, r4, 4)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 64)?)?;
    c.call(0x31ec)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(0x0000_2260)?; // Service table next routine address
    c.emit(0x000e_27f9)?; // Runtime diagnostics label
    c.check(0x2260)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_previous_runtime() { assert!(extend(&mut vec![]).is_err()); }
}
