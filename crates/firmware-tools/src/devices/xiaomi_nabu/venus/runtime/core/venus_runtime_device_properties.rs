// SPDX-License-Identifier: GPL-3.0-only
//! Resolve device properties and allocate runtime memory regions, with
//! deterministic ARM32 encoding of fallback and cleanup control paths.
use arm32::{
    data_immediate, mov_register, push_registers, transfer_multiple,
    BlockMode, Condition as C, DataOp as Op, Register as R,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

/// Ask the public property service to populate a four-word result on stack.
/// Destination and label location are structured runtime metadata.
fn lookup(c: &mut Code<'_>, object: R, key_at: usize, result_sp: u32)
    -> Result<(), String>
{
    let r3 = R::new(3)?;
    c.literal(R::R1, key_at)?;
    c.emit(add(r3, R::SP, result_sp)?)?;
    c.emit(mov_register(C::Al, R::R0, object))?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x883c)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x63d8 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 16)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(r5, R::R0, 12)?)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.emit(add(r4, r5, 16)?)?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.call(0x83e0)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6468)?;
    lookup(&mut c, r4, 0x6528, 4)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6468)?;
    c.emit(load(R::R0, R::SP, 12)?)?;
    c.literal(R::R1, 0x652c)?;
    c.emit(add(r3, R::SP, 4)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(R::R0, r5, 88)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6468)?;
    c.emit(load(R::R0, R::SP, 12)?)?;
    c.literal(R::R1, 0x6530)?;
    c.emit(add(r3, R::SP, 4)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(R::R0, r5, 92)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x6478)?;
    c.check(0x6468)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.check(0x646c)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x6478)?;
    c.emit(load(R::R0, R::SP, 12)?)?;
    c.literal(R::R1, 0x6534)?;
    c.emit(add(r3, R::SP, 4)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(add(R::R0, R::R0, 15)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, R::R0, 15)?)?;
    c.emit(store(R::R0, r5, 96)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x64fc)?;
    c.check(0x64a8)?;
    c.emit(store(r6, r5, 104)?)?;
    c.emit(store(r6, r5, 108)?)?;
    c.check(0x64b0)?;
    c.emit(add(R::R0, r5, 100)?)?;
    c.emit(load(R::R1, r5, 96)?)?;
    c.call(0x6914)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x646c)?;
    c.emit(load(R::R1, r5, 108)?)?;
    c.emit(add(R::R0, r5, 112)?)?;
    c.call(0x6914)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x646c)?;
    c.emit(add(R::R1, r5, 120)?)?;
    c.emit(mov(R::R0, 16)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x58d8)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mvn, r4, R::R0, 0)?)?;
    c.branch(C::Al, 0x646c)?;
    c.check(0x64fc)?;
    c.emit(load(R::R0, R::SP, 12)?)?;
    c.literal(R::R1, 0x6538)?;
    c.emit(add(r3, R::SP, 4)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(store(R::R0, r5, 92)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x6468)?;
    c.emit(load(r6, R::SP, 12)?)?;
    c.branch(C::Al, 0x64ac)?;
    c.check(0x6528)?;
    for property in [0x000e_3478, 0x000e_3489, 0x000e_3497,
                     0x000e_34a5, 0x000e_34b7] {
        c.emit(property)?;
    }
    c.check(0x653c)?;
    Ok(())
}
