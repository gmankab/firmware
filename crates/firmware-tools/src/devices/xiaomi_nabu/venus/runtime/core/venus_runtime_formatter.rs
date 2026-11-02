// SPDX-License-Identifier: GPL-3.0-only
//! Early argument scanner of the Venus formatted-diagnostics routine.
//! Models stack frame initialization, bounded copies, escaped percent signs,
//! argument selection and flags. The complex conversion dispatcher following
//! this prefix is intentionally not yet marked as reconstructed.
use arm32::{
    data_immediate, data_register, data_register_shifted_by_register,
    mov_register, push_registers, transfer_byte,
    transfer_byte_preindexed_writeback, transfer_word_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3574 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;

    // Establish formatter scratch space and argument-word state.
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit()
        | r8.bit() | r9.bit() | r10.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(sub(R::SP, R::SP, 140)?)?;
    c.emit(store(R::R0, R::SP, 48)?)?;
    c.emit(add(R::R0, R::SP, 64)?)?;
    c.emit(mov(r7, 137)?)?;
    c.emit(store(R::R1, R::SP, 116)?)?;
    c.emit(store(r3, R::SP, 28)?)?;
    c.emit(mov(r8, 1)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(mov(r9, 0)?)?;
    c.emit(store(R::R2, R::SP, 32)?)?;
    c.emit(add(R::R1, R::R0, 39)?)?;
    c.emit(add(R::R0, R::R0, 40)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r7, r7, 75776)?)?;
    c.emit(store(R::R0, R::SP, 20)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R1, R::SP, 16)?)?;
    c.emit(store(R::R0, R::SP, 44)?)?;

    // Overflow-aware count accumulation; use error bookkeeping for exceeded
    // formatted-output capacity and continue scanning format characters.
    c.check(0x35c0)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Lt, 0x35f4)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0x8000_0000)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R0, r4, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r9, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Le, 0x35f0)?;
    c.call(0x4a6c)?;
    c.emit(mov(R::R1, 75)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.branch(C::Al, 0x35f4)?;
    c.check(0x35f0)?;
    c.emit(data_register(C::Al, Op::Add, r4, r9, r4, Shift::Lsl, 0, false)?)?;
    c.check(0x35f4)?;
    c.emit(load(r10, R::SP, 116)?)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r10, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x3e94)?;
    c.emit(mov_register(C::Al, r5, r10))?;

    // A doubled percent is a literal percent, not a conversion specifier.
    c.check(0x360c)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r5, 0)?)?;
    c.emit(cmp(R::R1, 37)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne, 0x3644)?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.check(0x3620)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r5, 0)?)?;
    c.emit(cmp(R::R1, 37)?)?;
    c.emit(transfer_byte(C::Eq, true, R::R1, r5, 1)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, R::R0, R::R1, 37)?)?;
    c.branch(C::Ne, 0x3650)?;
    c.emit(add(r5, r5, 2)?)?;
    c.emit(add(R::R2, R::R2, 1)?)?;
    c.emit(store(r5, R::SP, 116)?)?;
    c.branch(C::Al, 0x3620)?;
    c.check(0x3644)?;
    c.emit(add(r5, r5, 1)?)?;
    c.emit(store(r5, R::SP, 116)?)?;
    c.branch(C::Al, 0x360c)?;

    // Copy already-scanned plain text and parse positional arguments (%N$).
    c.check(0x3650)?;
    c.emit(data_register(C::Al, Op::Sub, r9, R::R2, r10, Shift::Lsl, 0, false)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x366c)?;
    c.emit(mov_register(C::Al, R::R1, r10))?;
    c.emit(mov_register(C::Al, R::R2, r9))?;
    c.call(0x4290)?;
    c.emit(mov(r3, 58)?)?;
    c.check(0x366c)?;
    c.emit(cmp(r9, 0)?)?;
    c.branch(C::Ne, 0x35c0)?;
    c.emit(store(r4, R::SP, 36)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R1, R::R0, 1)?)?;
    c.emit(sub(r4, R::R1, 48)?)?;
    c.emit(cmp(r4, 9)?)?;
    c.branch(C::Ls, 0x369c)?;
    c.literal(r9, 0x3fc0)?;
    c.emit(load(r5, R::SP, 44)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.branch(C::Al, 0x36b8)?;
    c.check(0x369c)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r5, 2)?)?;
    c.literal(r9, 0x3fc0)?;
    c.emit(cmp(R::R1, 36)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, R::R0, r5, 3)?)?;
    c.emit(load(r5, R::SP, 44)?)?;
    c.emit(data_immediate(C::Ne, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r5, R::R0, 1)?)?;

    // Parse formatting flags as a bitset, using the public A32 register-shift
    // instruction rather than a generic numerical encoding table.
    c.check(0x36b8)?;
    c.emit(mov(R::R11, 0)?)?;
    c.emit(store(R::R0, R::SP, 116)?)?;
    c.check(0x36c0)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 0)?)?;
    c.emit(sub(R::R2, R::R1, 32)?)?;
    c.emit(cmp(R::R2, 31)?)?;
    c.branch(C::Hi, 0x36e8)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Tst, R::R0, r7,
        r8, Shift::Lsl, R::R2, false)?)?;
    c.branch(C::Eq, 0x36e8)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, R::R1, R::R0,
        r8, Shift::Lsl, R::R2, false)?)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R11, R::R1, R::R11, Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x36bc)?;
    c.check(0x36e8)?;
    c.emit(cmp(R::R1, 42)?)?;
    c.branch(C::Ne, 0x378c)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R2, R::R1, 1)?)?;
    c.emit(sub(R::R2, R::R2, 48)?)?;
    c.emit(cmp(R::R2, 9)?)?;
    c.branch(C::Hi, 0x3740)?;
    c.emit(transfer_byte(C::Al, true, r3, R::R0, 2)?)?;
    c.emit(cmp(r3, 36)?)?;
    c.emit(mov(r3, 58)?)?;
    c.branch(C::Ne, 0x3740)?;
    c.emit(load(R::R1, R::SP, 176)?)?;
    c.emit(mov(r3, 10)?)?;
    c.emit(mov(r5, 1)?)?;
    c.emit(transfer_word_register(C::Al, false, r3, R::R1, R::R2, Shift::Lsl, 2)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 1)?)?;
    c.emit(load(R::R2, R::SP, 28)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R2, R::R1, Shift::Lsl, 3, false)?)?;
    c.emit(load(r6, R::R1, -384)?)?;
    c.emit(add(R::R1, R::R0, 3)?)?;
    c.branch(C::Al, 0x3778)?;
    c.check(0x3740)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Ne, 0x3f08)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x3760)?;
    c.emit(mov(r6, 0)?)?;
    c.branch(C::Al, 0x3778)?;
    c.check(0x3760)?;
    c.emit(load(r3, R::SP, 32)?)?;
    c.emit(load(R::R0, r3, 0)?)?;
    c.emit(add(R::R2, R::R0, 4)?)?;
    c.emit(store(R::R2, r3, 0)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(load(r6, R::R0, 0)?)?;
    c.check(0x3778)?;
    c.emit(cmp(r6, 0)?)?;
    c.emit(store(R::R1, R::SP, 116)?)?;
    c.emit(data_immediate(C::Lt, Op::Orr, R::R11, R::R11, 8192)?)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r6, r6, 0)?)?;
    c.branch(C::Al, 0x37a4)?;
    c.check(0x378c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_gaps() { assert!(extend(&mut Vec::new()).is_err()); }
}
