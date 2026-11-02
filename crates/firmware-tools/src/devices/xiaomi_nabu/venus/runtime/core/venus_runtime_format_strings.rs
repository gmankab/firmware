// SPDX-License-Identifier: GPL-3.0-only
//! Length-limited vector string output in the Venus diagnostic formatter.
//! Iterates through strings, bounds the total length, and copies each item
//! into the shared output sink with typed ARM32 instruction encoders.
use arm32::{data_immediate, data_register, mov_register, Condition as C,
           DataOp as Op, Register as R, Shift};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3d50 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r9 = R::new(9)?;

    // Initialize a descriptor for vector-valued diagnostic output.
    c.emit(load(R::R0, R::SP, 104)?)?;
    c.emit(store(R::R0, R::SP, 56)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 60)?)?;
    c.emit(add(R::R0, R::SP, 56)?)?;
    c.emit(store(R::R0, R::SP, 104)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(store(R::R0, R::SP, 40)?)?;
    c.emit(load(r4, R::SP, 104)?)?;
    c.emit(load(r7, R::SP, 40)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(add(r6, R::SP, 52)?)?;
    c.emit(mov_register(C::Al, r5, r4))?;
    c.emit(mov_register(C::Al, r9, R::R1))?;

    // Bound the count of descriptor entries. Failure in the string-length
    // service skips copying rather than walking past the end of the array.
    c.check(0x3d88)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, r9, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x3dc0)?;
    c.emit(load(R::R1, r5, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x3dc0)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x4b08)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Lt, 0x3f08)?;
    c.emit(data_register(C::Al, Op::Sub, R::R2, r7, r9, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R0, r9, Shift::Lsl, 0, false)?)?;
    c.emit(add(r5, r5, 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x3d84)?;

    // Pre-padding based on the total emitted size.
    c.check(0x3dc0)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(load(R::R2, R::SP, 24)?)?;
    c.emit(mov(R::R1, 32)?)?;
    c.emit(mov_register(C::Al, r3, r9))?;
    c.emit(store(R::R11, R::SP, 0)?)?;
    c.call(0x43dc)?;
    c.emit(mov(r7, 137)?)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r7, r7, 75776)?)?;
    c.check(0x3de4)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r5, r9, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x3e24)?;
    c.emit(load(R::R1, r4, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x3e24)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.call(0x4b08)?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.emit(data_register(C::Al, Op::Add, r5, R::R2, r5, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r5, r9, Shift::Lsl, 0, false)?)?;
    c.branch(C::Gt, 0x3e24)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.call(0x4290)?;
    c.emit(add(r4, r4, 4)?)?;
    c.branch(C::Al, 0x3de4)?;

    // Post-padding, then restore conversion-specific state before the next
    // formatting operation.
    c.check(0x3e24)?;
    c.emit(data_immediate(C::Al, Op::Eor, R::R0, R::R11, 8192)?)?;
    c.emit(load(r4, R::SP, 24)?)?;
    c.emit(mov(R::R1, 32)?)?;
    c.emit(mov_register(C::Al, r3, r9))?;
    c.emit(store(R::R0, R::SP, 0)?)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.call(0x43dc)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r4, r9, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Gt, r9, r4))?;
    c.check(0x3e4c)?;
    c.emit(load(r4, R::SP, 36)?)?;
    c.emit(mov(r3, 58)?)?;
    c.branch(C::Al, 0x35c0)?;
    c.check(0x3e58)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_runtime_prefix() { assert!(extend(&mut Vec::new()).is_err()); }
}
