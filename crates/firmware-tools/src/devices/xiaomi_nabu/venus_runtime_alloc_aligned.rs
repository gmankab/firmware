// SPDX-License-Identifier: GPL-3.0-only
//! Venus aligned memory allocations and runtime service dispatch table.
//! Allocator algorithms use typed ARM A32 encodings with alignment, checked
//! branch displacement, and structured diagnostic addresses.
use arm32::{
    bx, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2e40 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;

    // Clear an event mask under a mutually exclusive queue-state guard.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x2e78)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.call(0x5958)?;
    c.emit(load(r4, r5, 8)?)?;
    c.emit(data_register(C::Al, Op::Bic, R::R0, r4,
        r6, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, r5, 8)?)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.call(0x59e8)?;
    c.branch(C::Al, 0x2e7c)?;
    c.check(0x2e78)?;
    c.emit(mov(r4, 0)?)?;
    c.check(0x2e7c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2e84)?;

    // Signal a pending hardware event if a valid queue is present.
    c.emit(push_registers(C::Al,
        r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x2eb8)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(mov(R::R1, 1024)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(store(r4, R::SP, 0)?)?;
    c.call(0x553c)?;
    c.branch(C::Al, 0x2ebc)?;
    c.check(0x2eb8)?;
    c.emit(mov(r4, 1)?)?;
    c.check(0x2ebc)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2ec8)?;

    // Cancel the queue event with a second event-mode constant.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(store(R::R1, R::SP, 0)?)?;
    c.emit(mov(R::R1, 512)?)?;
    c.call(0x553c)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::SP, R::R11))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x2efc)?;

    // Minimal heap-object allocation wrapper.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.call(0x57d4)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, R::SP, R::R11))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x2f1c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(bx(C::Eq, R::LR))?;
    c.branch(C::Al, 0x5838)?;
    c.check(0x2f28)?;

    // Allocate a 32-byte-aligned object, storing its original allocation
    // displacement in the word immediately preceding the aligned pointer.
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.literal(R::R2, 0x2f9c)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(R::R1, 66)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 512)?)?;
    c.emit(data_immediate(C::Gt, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(add(R::R0, r5, 35)?)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.call(0x57d4)?;
    c.emit(load(r5, R::SP, 4)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x2f90)?;
    c.literal(R::R2, 0x2f9c)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov(R::R1, 588)?)?;
    c.call(0x17b0)?;
    c.emit(add(R::R0, r5, 35)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r4, R::R0, 31)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r4,
        r5, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, r4, -4)?)?;
    c.check(0x2f90)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.emit(0x000e_290a)?; // Allocator assertion descriptor
    c.check(0x2fa0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn truncated_parent_runtime_is_rejected() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
