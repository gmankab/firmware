// SPDX-License-Identifier: GPL-3.0-only
//! Controller I/O transfer routines with counted word writes and fault rollback.
use arm32::{
    bx, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word, transfer_word_postindexed,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

fn return_frame(c: &mut Code<'_>, regs: u16) -> Result<(), String> {
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (regs & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7e2c };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    // Transfer an aligned sequence of controller words to a destination buffer.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::new(3)?))?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.call(0x7ff8)?;
    c.emit(load(r6, R::R11, 8)?)?;
    c.emit(mov(r7, 0)?)?;
    c.emit(cmp(r5, 5)?)?;
    c.branch(C::Lt, 0x7e70)?;
    c.emit(sub(r5, r5, 4)?)?;
    c.emit(mov(r7, 0)?)?;
    c.check(0x7e5c)?;
    c.call(0x8040)?;
    c.emit(add(r7, r7, 4)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, R::R0, r4, 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r7, r5, Shift::Lsl, 0, false)?)?;
    c.branch(C::Lt, 0x7e5c)?;
    c.check(0x7e70)?;
    c.emit(cmp(r6, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(transfer_word(C::Ne, false, r7, r6, 0)?)?;
    return_frame(&mut c, saved)?;
    c.check(0x7e80)?;
    // Apply a controller configuration; await readiness when requested.
    let short_saved = r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, short_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.call(0x80c4)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, r4, 1)?)?;
    c.branch(C::Eq, 0x7e9c)?;
    c.call(0x80f0)?;
    c.emit(mov(R::R0, 0)?)?;
    return_frame(&mut c, short_saved)?;
    c.check(0x7ea4)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x7eb4)?;
    // Map controller MMIO, falling back to the diagnostic address provider.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 32)?)?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(mov(R::new(3)?, 4096)?)?;
    c.emit(store(r5, R::SP, 0)?)?;
    c.emit(store(R::R0, R::SP, 4)?)?;
    c.emit(mov(R::R0, 3200)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 524_288)?)?;
    c.call(0x573c)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Ne, 0x7f0c)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(add(R::R1, R::SP, 12)?)?;
    c.call(0x5888)?;
    c.emit(load(R::R0, R::SP, 12)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    return_frame(&mut c, saved)?;
    c.check(0x7f18)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controller_io_has_no_missing_layout_bytes() {
        let mut bytes = vec![0; 0x7e2c-0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x7f18-0x1000);
    }
    #[test]
    fn controller_io_rejects_missing_predecessor() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
