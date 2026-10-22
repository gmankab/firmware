// SPDX-License-Identifier: GPL-3.0-only
//! ARM32 diagnostic-format conversion selection and argument decoding.
//! The jump table contains explicit branch destinations, while the public
//! ARM32 ISA encoders construct instruction bytes from typed operands.
use arm32::{
    data_immediate, data_register, mov_register, mov_shifted_register,
    multiply_accumulate, transfer_byte, transfer_byte_register,
    transfer_byte_preindexed_writeback, transfer_word_register,
    transfer_word_register_preindexed_writeback,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x378c };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r9 = R::new(9)?;

    // Parse conversion precision, supporting literal lengths, named argument
    // words and dynamic width after a '*' marker.
    c.emit(add(R::R0, R::SP, 116)?)?;
    c.call(0x42b0)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(mov(r3, 58)?)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Lt, 0x3f08)?;
    c.check(0x37a4)?;
    c.emit(load(R::R0, R::SP, 116)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 0)?)?;
    c.emit(store(R::R2, R::SP, 40)?)?;
    c.emit(cmp(R::R1, 46)?)?;
    c.branch(C::Ne, 0x3874)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R2, R::R1, 1)?)?;
    c.emit(cmp(R::R2, 42)?)?;
    c.branch(C::Ne, 0x3860)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R2, R::R1, 2)?)?;
    c.emit(sub(R::R2, R::R2, 48)?)?;
    c.emit(cmp(R::R2, 9)?)?;
    c.branch(C::Hi, 0x3820)?;
    c.emit(transfer_byte(C::Al, true, r3, R::R0, 3)?)?;
    c.emit(cmp(r3, 36)?)?;
    c.emit(mov(r3, 58)?)?;
    c.branch(C::Ne, 0x3820)?;
    c.emit(load(R::R1, R::SP, 176)?)?;
    c.emit(mov(r3, 10)?)?;
    c.emit(transfer_word_register(C::Al, false, r3, R::R1, R::R2, Shift::Lsl, 2)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 2)?)?;
    c.emit(load(R::R2, R::SP, 28)?)?;
    c.emit(add(R::R0, R::R0, 4)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R2, R::R1, Shift::Lsl, 3, false)?)?;
    c.emit(load(R::R1, R::R1, -384)?)?;
    c.emit(store(R::R0, R::SP, 116)?)?;
    c.emit(store(R::R1, R::SP, 40)?)?;
    c.branch(C::Al, 0x3874)?;
    c.check(0x3820)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Ne, 0x3f08)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x383c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x3854)?;
    c.check(0x383c)?;
    c.emit(load(r3, R::SP, 32)?)?;
    c.emit(load(R::R0, r3, 0)?)?;
    c.emit(add(R::R2, R::R0, 4)?)?;
    c.emit(store(R::R2, r3, 0)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.check(0x3854)?;
    c.emit(store(R::R0, R::SP, 40)?)?;
    c.emit(store(R::R1, R::SP, 116)?)?;
    c.branch(C::Al, 0x3874)?;
    c.check(0x3860)?;
    c.emit(add(R::R0, R::SP, 116)?)?;
    c.emit(store(R::R1, R::SP, 116)?)?;
    c.call(0x42b0)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(store(R::R0, R::SP, 40)?)?;
    c.check(0x3874)?;
    c.emit(load(R::R0, R::SP, 116)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(r6, R::SP, 24)?)?;
    c.emit(store(r5, R::SP, 44)?)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;

    // Dispatch a format conversion using a small character map. The MLA
    // encoding is explicit, including the addend register used as a row
    // offset in the conversion-character translation table.
    c.check(0x388c)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r6, 0)?)?;
    c.emit(sub(R::R0, R::R0, 65)?)?;
    c.emit(cmp(R::R0, 57)?)?;
    c.branch(C::Hi, 0x3f04)?;
    c.emit(multiply_accumulate(C::Al, R::R1, r5, r3, r9)?)?;
    c.emit(transfer_byte_register(C::Al, true, R::R1, R::R1, R::R0, Shift::Lsl, 0)?)?;
    c.emit(add(R::R0, r6, 1)?)?;
    c.emit(sub(R::R2, R::R1, 1)?)?;
    c.emit(cmp(R::R2, 8)?)?;
    c.branch(C::Cc, 0x3884)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(store(R::R0, R::SP, 116)?)?;
    c.branch(C::Eq, 0x3f08)?;
    c.emit(cmp(R::R1, 19)?)?;
    c.branch(C::Ne, 0x38d4)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Lt, 0x38f8)?;
    c.branch(C::Al, 0x3f08)?;
    c.check(0x38d4)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Lt, 0x3910)?;
    c.emit(load(R::R0, R::SP, 176)?)?;
    c.emit(transfer_word_register(C::Al, false, R::R1, R::R0, r4, Shift::Lsl, 2)?)?;
    c.emit(load(R::R0, R::SP, 28)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, R::R1,
        R::R0, r4, Shift::Lsl, 3)?)?;
    c.emit(load(R::R0, R::R0, 4)?)?;
    c.emit(store(R::R0, R::SP, 108)?)?;
    c.emit(store(R::R1, R::SP, 104)?)?;
    c.check(0x38f8)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(load(r4, R::SP, 36)?)?;
    c.emit(mov(r9, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x35c0)?;
    c.branch(C::Al, 0x3930)?;
    c.check(0x3910)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x3924)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x3f0c)?;
    c.check(0x3924)?;
    c.emit(load(R::R2, R::SP, 32)?)?;
    c.emit(add(R::R0, R::SP, 104)?)?;
    c.call(0x42e0)?;
    c.check(0x3930)?;
    c.emit(transfer_byte(C::Al, true, r6, r6, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x3948)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, r6, 15)?)?;
    c.emit(cmp(R::R0, 3)?)?;
    c.emit(data_immediate(C::Eq, Op::And, r6, r6, 223)?)?;
    c.check(0x3948)?;
    c.emit(load(R::R1, R::SP, 20)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r4, R::R11, 65536)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R11, 8192)?)?;
    c.emit(sub(R::R0, r6, 83)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov_register(C::Ne, R::R11, r4))?;
    c.emit(cmp(R::R0, 37)?)?;
    c.emit(store(R::R2, R::SP, 12)?)?;
    c.branch(C::Ls, 0x3f14)?;
    c.emit(sub(R::R0, r6, 69)?)?;
    c.emit(cmp(R::R0, 3)?)?;
    c.branch(C::Cc, 0x3e58)?;
    c.emit(cmp(r6, 65)?)?;
    c.branch(C::Eq, 0x3e58)?;
    c.emit(cmp(r6, 67)?)?;
    c.branch(C::Eq, 0x3d50)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.branch(C::Al, 0x3c80)?;

    // Eight known conversion-result destinations. Table entries are
    // symbolic runtime addresses rather than opaque opcode bytes.
    c.check(0x3990)?;
    c.emit(load(r4, R::SP, 36)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, r5, 255)?)?;
    c.emit(mov(r9, 0)?)?;
    c.emit(mov(r3, 58)?)?;
    c.emit(cmp(R::R0, 7)?)?;
    c.branch(C::Hi, 0x35c0)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, R::PC, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;
    for target in [0x39ec, 0x39ec, 0x39f8, 0x39d4,
                   0x39e0, 0x35c0, 0x39ec, 0x39f8] { c.emit(target)?; }
    c.check(0x39d4)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_prior_arg_scanner() { assert!(extend(&mut Vec::new()).is_err()); }
}
