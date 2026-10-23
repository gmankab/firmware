// SPDX-License-Identifier: GPL-3.0-only
//! Flush and shutdown of three diagnostic-stream classes. The repeated
//! callback chain is expressed once in Rust, and only its class-specific
//! destination addresses and finalization behaviors vary.
use arm32::{
    branch_exchange_with_link, bx, data_register, mov_register,
    mov_shifted_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

/// Flush all pending data from one stream descriptor to its registered writer.
fn flush_pending(c: &mut Code<'_>, skip_at: usize) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    c.emit(load(R::R0, r4, 76)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Lt, skip_at)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x4b78)?;
    c.check(skip_at)?;
    c.emit(load(R::R0, r4, 20)?)?;
    c.emit(load(R::R1, r4, 28)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, skip_at + 36)?;
    c.emit(load(r3, r4, 36)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(branch_exchange_with_link(C::Al, r3)?)?;
    c.check(skip_at + 36)?;
    Ok(())
}

/// If a buffer retains a partial record, pass its length to the record
/// callback. The first class additionally walks a linked descriptor list.
fn flush_partial(c: &mut Code<'_>, first_class: bool,
                 done: usize) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    c.emit(transfer_multiple(C::Al, true, r4, R::R0.bit() | R::R1.bit(),
        BlockMode::IncrementBefore, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, done)?;
    let callback = if first_class { r5 } else { r6 };
    c.emit(load(callback, r4, 40)?)?;
    if first_class {
        c.emit(data_register(C::Al, Op::Sub, R::R2, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
        c.emit(mov_register(C::Al, R::R0, r4))?;
        c.emit(store(r6, R::SP, 0)?)?;
    } else {
        c.emit(mov(R::R2, 1)?)?;
        c.emit(store(R::R2, R::SP, 0)?)?;
        c.emit(data_register(C::Al, Op::Sub, R::R2, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
        c.emit(mov_register(C::Al, R::R0, r4))?;
    }
    c.emit(mov_shifted_register(C::Al, r3, R::R2, Shift::Asr, 31)?)?;
    c.emit(branch_exchange_with_link(C::Al, callback)?)?;
    c.check(done)?;
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4cb0 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.call(0x4e24)?;
    c.emit(load(r4, R::R0, 0)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x4d34)?;
    c.emit(mov(r6, 1)?)?;

    // Linked-list descriptors, following each release callback in order.
    c.check(0x4ccc)?;
    flush_pending(&mut c, 0x4ce0)?;
    flush_partial(&mut c, true, 0x4d28)?;
    c.emit(load(r4, r4, 56)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Ne, 0x4ccc)?;

    // Second class has one descriptor rather than a list.
    c.check(0x4d34)?;
    c.literal(R::R0, 0x4e1c)?;
    c.emit(load(r4, R::R0, 0)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x4da4)?;
    flush_pending(&mut c, 0x4d58)?;
    flush_partial(&mut c, false, 0x4da4)?;

    // Third stream class points to a separately maintained descriptor.
    c.check(0x4da4)?;
    c.literal(R::R0, 0x4e20)?;
    c.emit(load(r4, R::R0, 0)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x4e14)?;
    flush_pending(&mut c, 0x4dc8)?;
    flush_partial(&mut c, false, 0x4e14)?;
    c.emit(add(R::SP, R::SP, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    // Two runtime stream roots intentionally alias the same descriptor.
    c.emit(0x0010_34a8)?;
    c.emit(0x0010_34a8)?;

    // Return a pointer to the root of the stream table and a small stub.
    c.literal(R::R0, 0x4e2c)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(0x0010_34ac)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(0x4e34)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_utf8_runtime() { assert!(extend(&mut Vec::new()).is_err()); }
}
