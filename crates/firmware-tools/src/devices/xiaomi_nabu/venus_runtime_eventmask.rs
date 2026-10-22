// SPDX-License-Identifier: GPL-3.0-only
//! Venus event-mask state update, cancellation, and idle-wait routines.
//! The typed ARM32 instructions reproduce dispatch logic and operate on
//! device-specific queue structures; complete firmware is not yet built.
use arm32::{
    data_register, mov_register, push_registers, transfer_multiple, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x2d20 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    // Update event mask while synchronizing the hardware status resource.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x2d78)?;
    c.emit(load(R::R0, r4, 4)?)?;
    c.call(0x5958)?;
    c.emit(load(R::R0, r4, 8)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0,
        R::R0, r5, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, r4, 8)?)?;
    c.emit(load(R::R0, r4, 4)?)?;
    c.call(0x59e8)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(mov(R::R1, 256)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(store(r4, R::SP, 0)?)?;
    c.call(0x553c)?;
    c.branch(C::Al, 0x2d7c)?;
    c.check(0x2d78)?;
    c.emit(mov(r4, 1)?)?;
    c.check(0x2d7c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2d88)?;

    // Read current event mask with an explicit guard for absent state.
    c.emit(push_registers(C::Al,
        r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x2db0)?;
    c.emit(load(R::R0, r4, 4)?)?;
    c.call(0x5958)?;
    c.emit(transfer_multiple(C::Al, true, r4,
        R::R0.bit() | r4.bit(), BlockMode::IncrementBefore, false)?)?;
    c.call(0x59e8)?;
    c.branch(C::Al, 0x2db4)?;
    c.check(0x2db0)?;
    c.emit(mov(r4, 0)?)?;
    c.check(0x2db4)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2dbc)?;

    // Drain pending events until the requested mask becomes present.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x2e30)?;
    c.check(0x2dd8)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.call(0x5958)?;
    c.emit(transfer_multiple(C::Al, true, r5,
        R::R0.bit() | r6.bit(), BlockMode::IncrementBefore, false)?)?;
    c.call(0x59e8)?;
    c.emit(data_register(C::Al, Op::Tst,
        R::R0, r6, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x2e34)?;
    c.emit(mov(r7, 0)?)?;
    c.check(0x2df4)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.emit(mov(R::R1, 1024)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r3, 0)?)?;
    c.emit(store(r7, R::SP, 0)?)?;
    c.call(0x553c)?;
    c.emit(load(R::R0, r5, 0)?)?;
    c.call(0x5618)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.call(0x5958)?;
    c.emit(transfer_multiple(C::Al, true, r5,
        R::R0.bit() | r6.bit(), BlockMode::IncrementBefore, false)?)?;
    c.call(0x59e8)?;
    c.emit(data_register(C::Al, Op::Tst,
        R::R0, r6, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x2df4)?;
    c.branch(C::Al, 0x2e34)?;
    c.check(0x2e30)?;
    c.emit(mov(r6, 0)?)?;
    c.check(0x2e34)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x2e40)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_gap_is_silently_accepted() { assert!(extend(&mut Vec::new()).is_err()); }
}
