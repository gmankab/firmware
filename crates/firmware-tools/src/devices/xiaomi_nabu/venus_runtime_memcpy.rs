// SPDX-License-Identifier: GPL-3.0-only
//! ARM32 memory-copy implementation, including alignment and common
//! sixteen-byte bulk transfers. All emitted instructions are typed.
use arm32::{
    data_immediate, mov_register, transfer_byte_postindexed,
    transfer_word_postindexed, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x471c };
    let r3 = R::new(3)?;
    c.emit(mov_register(C::Al, r3, R::R0))?;
    c.check(0x4720)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.emit(arm32::data_immediate_with_flags(C::Ne, Op::And,
        R::R12, R::R1, 3, true)?)?;
    c.branch(C::Eq, 0x473c)?;
    c.emit(transfer_byte_postindexed(C::Al, true, R::R12, R::R1, 1)?)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, R::R12, r3, 1)?)?;
    c.branch(C::Al, 0x4720)?;
    c.check(0x473c)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r3, 3)?)?;
    c.branch(C::Ne, 0x47ac)?;
    c.check(0x4744)?;
    c.emit(cmp(R::R2, 16)?)?;
    c.branch(C::Cc, 0x477c)?;
    // Four adjacent words make one cache-friendly 16-byte transfer.
    for offset in [0, 4, 8] {
        c.emit(load(R::R12, R::R1, offset)?)?;
        if offset == 0 { c.emit(sub(R::R2, R::R2, 16)?)?; }
        c.emit(store(R::R12, r3, offset)?)?;
    }
    c.emit(load(R::R12, R::R1, 12)?)?;
    c.emit(add(R::R1, R::R1, 16)?)?;
    c.emit(store(R::R12, r3, 12)?)?;
    c.emit(add(r3, r3, 16)?)?;
    c.branch(C::Al, 0x4744)?;
    c.check(0x477c)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R2, 8)?)?;
    c.branch(C::Eq, 0x479c)?;
    c.emit(load(R::R12, R::R1, 0)?)?;
    c.emit(store(R::R12, r3, 0)?)?;
    c.emit(load(R::R12, R::R1, 4)?)?;
    c.emit(add(R::R1, R::R1, 8)?)?;
    c.emit(store(R::R12, r3, 4)?)?;
    c.emit(add(r3, r3, 8)?)?;
    c.check(0x479c)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R2, 4)?)?;
    c.emit(transfer_word_postindexed(C::Ne, true, R::R12, R::R1, 4)?)?;
    c.emit(transfer_word_postindexed(C::Ne, false, R::R12, r3, 4)?)?;
    c.branch(C::Al, 0x4a3c)?;
    c.check(0x47ac)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_memory_scan() { assert!(extend(&mut Vec::new()).is_err()); }
}
