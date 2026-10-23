// SPDX-License-Identifier: GPL-3.0-only
//! Buffered memory-copy adapters for the diagnostic output stream.
use arm32::{
    data_register, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4610 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(load(R::R0, r5, 16)?)?;
    c.emit(load(r3, r5, 20)?)?;
    c.emit(data_register(C::Al, Op::Sub, r6, R::R0, r3, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R0, r3))?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r6, r4, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r6, r4))?;
    c.emit(mov_register(C::Al, R::R2, r6))?;
    c.call(0x471c)?;
    c.emit(load(R::R0, r5, 20)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, r6, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, r5, 20)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    // Expose zero-fill through the shared typed memory routine.
    c.emit(mov_register(C::Al, R::R2, R::R1))?;
    c.emit(mov(R::R1, 0)?)?;
    c.branch(C::Al, 0x31ec)?;
    c.branch(C::Al, 0x471c)?;
    c.check(0x4660)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_predecessor_is_an_error() { assert!(extend(&mut Vec::new()).is_err()); }
}
