// SPDX-License-Identifier: GPL-3.0-only
//! Checked ARM32 interface-table lookup with ordered service fallbacks.
use arm32::{
    data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

const INVALID_INTERFACE_DIAGNOSTIC: u32 = 0x000e_3584;

fn pop(c: &mut Code<'_>, regs: u16, cond: C, final_return: bool) -> Result<(), String> {
    c.emit(transfer_multiple(cond, true, R::SP,
        if final_return { (regs & !R::LR.bit()) | R::PC.bit() } else { regs },
        BlockMode::IncrementAfter, true)?)
}
fn regcmp(c: &mut Code<'_>, lhs: R, rhs: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::Cmp, R::R0, lhs, rhs, Shift::Lsl, 0, false)?)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x8364 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let short = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    let long = r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    // Find a service by its interface tag and return two associated pointers.
    c.emit(push_registers(C::Al, short)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, R::R12, R::R0))?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(transfer_word(C::Ne, true, R::LR, R::R0, 20)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::LR, 0)?)?;
    c.branch(C::Eq, 0x83a8)?;
    c.emit(add(r3, R::R0, 28)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.check(0x838c)?;
    c.emit(load(r4, r3, -4)?)?;
    regcmp(&mut c, r4, R::R12)?;
    c.branch(C::Eq, 0x83c8)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(add(r3, r3, 8)?)?;
    regcmp(&mut c, R::R0, R::LR)?;
    c.branch(C::Cc, 0x838c)?;
    c.check(0x83a8)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R1, R::R12))?;
    c.emit(store(R::R0, R::R2, 0)?)?;
    c.emit(store(R::R0, R::R2, 4)?)?;
    c.literal(R::R0, 0x83dc)?;
    c.call(0x8bac)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    pop(&mut c, short, C::Al, true)?;
    c.check(0x83c8)?;
    c.emit(store(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R0, r3, 0)?)?;
    c.emit(store(R::R0, R::R2, 4)?)?;
    c.emit(mov(R::R0, 0)?)?;
    pop(&mut c, short, C::Al, true)?;
    c.emit(INVALID_INTERFACE_DIAGNOSTIC)?;
    c.check(0x83e0)?;
    // Probe the default interface. Fall back to the alternate service source.
    c.emit(push_registers(C::Al, long)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov(r6, 0)?)?;
    c.call(0x8bdc)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x8444)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.call(0x8364)?;
    c.emit(mov_register(C::Al, r7, R::R0))?;
    c.emit(cmp(r7, 0)?)?;
    c.branch(C::Eq, 0x8450)?;
    c.emit(mov(R::R0, 1)?)?;
    c.call(0x8bdc)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x844c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    pop(&mut c, long, C::Al, false)?;
    c.branch(C::Al, 0x8364)?;
    c.check(0x8444)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.branch(C::Al, 0x8450)?;
    c.emit(mov_register(C::Al, r6, r7))?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    pop(&mut c, long, C::Al, true)?;
    c.check(0x8458)?;
    // Select a service source explicitly or return an error if unavailable.
    let saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.call(0x8bdc)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mvn, R::R0, R::R0, 0)?)?;
    pop(&mut c, saved, C::Eq, true)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    pop(&mut c, saved, C::Al, false)?;
    c.branch(C::Al, 0x8364)?;
    c.check(0x8490)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lookup_code_has_bounded_dispatch_and_fallbacks() {
        let mut bytes = vec![0; 0x8364-0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x8490-0x1000);
    }
    #[test]
    fn interface_lookup_requires_preceding_image() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
