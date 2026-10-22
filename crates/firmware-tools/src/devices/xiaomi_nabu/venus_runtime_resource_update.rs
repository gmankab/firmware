// SPDX-License-Identifier: GPL-3.0-only
//! ARM A32 resource-state update. The entry routine snapshots temporary
//! state, performs a bounded resource update, then restores the caller's
//! status and optional lock. All instructions are encoded through typed
//! ARM32 operations; the reference firmware is only read by verification.
use arm32::{
    branch_exchange_with_link, data_immediate, data_register, load_signed_byte,
    mov_register, push_registers, transfer_multiple, Condition as C,
    DataOp as Op, Register as R, Shift, BlockMode,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3410 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit()
        | r8.bit() | r9.bit() | r10.bit() | R::LR.bit();

    // Create a scratch frame and fetch the resource description.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(sub(R::SP, R::SP, 216)?)?;
    c.emit(add(r7, R::SP, 168)?)?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(mov(R::R1, 40)?)?;
    c.emit(store(R::R2, R::SP, 212)?)?;
    c.emit(mov_register(C::Al, R::R0, r7))?;
    c.call(0x4650)?;
    c.emit(load(R::R0, R::SP, 212)?)?;
    c.emit(add(R::R2, R::SP, 208)?)?;
    c.emit(add(r3, R::SP, 88)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(store(r7, R::SP, 0)?)?;
    c.emit(mov(r5, 0)?)?;
    c.emit(store(R::R0, R::SP, 208)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.call(0x3574)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ge, 0x3468)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.branch(C::Al, 0x3568)?;

    // The presence of a live slot dictates whether its existing state must
    // be saved. Read the signed mode byte instead of treating it as unsigned.
    c.check(0x3468)?;
    c.emit(load(R::R0, r4, 76)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Lt, 0x3480)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x4b78)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.check(0x3480)?;
    c.emit(load_signed_byte(C::Al, R::R1, r4, 74)?)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r8, R::R0, 32)?)?;
    c.emit(data_immediate(C::Le, Op::Bic, R::R1, R::R0, 32)?)?;
    c.emit(arm32::transfer_word(C::Le, false, R::R1, r4, 0)?)?;
    c.emit(load(R::R0, r4, 48)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x34c4)?;

    // Existing resource: format a state record and update its status.
    c.emit(add(R::R2, R::SP, 208)?)?;
    c.emit(add(r3, R::SP, 88)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(store(r7, R::SP, 0)?)?;
    c.call(0x3574)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.branch(C::Al, 0x3544)?;

    // Otherwise temporarily install local storage, restore the previous
    // descriptor, and release the temporary callback on exit.
    c.check(0x34c4)?;
    c.emit(load(r9, r4, 44)?)?;
    c.emit(add(R::R0, R::SP, 8)?)?;
    c.emit(mov(R::R1, 80)?)?;
    c.emit(add(R::R2, R::SP, 208)?)?;
    c.emit(add(r3, R::SP, 88)?)?;
    c.emit(store(r7, R::SP, 0)?)?;
    for field in [44, 28, 20] { c.emit(store(R::R0, r4, field)?)?; }
    c.emit(add(R::R0, R::R0, 80)?)?;
    c.emit(store(R::R1, r4, 48)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(store(R::R0, r4, 16)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x3574)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r9, 0)?)?;
    c.branch(C::Eq, 0x3544)?;
    c.emit(load(r3, r4, 36)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r7, 0)?)?;
    c.emit(branch_exchange_with_link(C::Al, r3)?)?;
    c.emit(load(R::R0, r4, 20)?)?;
    c.emit(store(r9, r4, 44)?)?;
    for field in [48, 16, 28, 20] { c.emit(store(r7, r4, field)?)?; }
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mvn, r6, R::R0, 0)?)?;

    // Restore the caller's locked/masked state and return the operation status.
    c.check(0x3544)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 32)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, R::R0, r8, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mvn, r6, R::R0, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.branch(C::Eq, 0x3568)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x4b80)?;
    c.check(0x3568)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(add(R::SP, R::SP, 216)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        saved & !R::LR.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x3574)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_exact_previous_layout() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
