// SPDX-License-Identifier: GPL-3.0-only
//! Early Venus device state setup and tagged resource dispatch.
//! Addresses in this module are ARM-mode virtual addresses within the
//! firmware's executable load segment. Data tables remain typed literals.
use arm32::{
    data_immediate, mov_register,
    mov_shifted_register, push_registers, transfer_byte, transfer_multiple,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x4e34 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r12 = R::R12;
    let saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();

    // Initialize the shared device state once, then publish its availability
    // and enroll two stream handlers. The caller may supply an optional
    // register value for an early status/diagnostics slot.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    for (reg, literal) in [(R::R0, 0x4ee4), (R::R1, 0x4ee8), (r5, 0x4eec)] {
        c.literal(reg, literal)?;
    }
    c.emit(store(R::R1, R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r5, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x4e7c)?;
    c.literal(R::R0, 0x4ef0)?;
    c.emit(load(R::R1, R::R0, 0)?)?;
    c.literal(R::R0, 0x4ef4)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.call(0xa8b0)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r5, 0)?)?;
    c.check(0x4e7c)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(load_with(C::Ne, R::R0, r4, 0)?)?;
    c.emit(load_with(C::Ne, R::R1, R::PC, 0x6c)?)?;
    c.emit(store_with(C::Ne, R::R0, R::R1, 0)?)?;
    c.call(0xd5fc)?;
    c.literal(R::R1, 0x4efc)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.call(0xa004)?;
    c.literal(R::R1, 0x4f00)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov(r5, 1)?)?;
    c.call(0xa004)?;
    c.literal(R::R0, 0x4f04)?;
    for reg in [R::R1, R::R2, r3] { c.emit(mov(reg, 0)?)?; }
    for offset in [0, 4, 8, 12] { c.emit(store(r4, R::SP, offset)?)?; }
    c.emit(store(r5, R::R0, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.call(0x9edc)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x4ee4)?;
    // Shared device and log state; these are pointers/constants, not code.
    for literal in [
        0x0010_cfa0, 0x0010_36d4, 0x0010_34b4, 0x48, 0x40,
        0x0010_34b0, 0x0000_d584, 0x0000_d5e8, 0x0010_39f4,
    ] { c.emit(literal)?; }
    c.check(0x4f08)?;
    // Minimal fault-report shim.
    c.literal(R::R2, 0x4f18)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 94)?)?;
    c.branch(C::Al, 0x17b0)?;
    c.emit(0x000e_3334)?;
    c.check(0x4f1c)?;

    // Tagged object routing. The low flag bit selects the wrapped handle;
    // the following four-entry table selects the handler for object type.
    c.emit(push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    c.emit(mov_register(C::Al, R::R11, R::SP))?;
    c.emit(mov_register(C::Al, R::R2, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, R::R2, 1)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x4f78)?;
    c.emit(load(R::LR, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r12, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::LR, 15)?)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(cmp(R::R1, 3)?)?;
    c.branch(C::Hi, 0x4f7c)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 2)?)?;
    c.emit(add(r3, R::PC, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::PC, R::R1, r3, Shift::Lsl, 0)?)?;
    c.check(0x4f58)?;
    for destination in [0x4f68, 0x4f9c, 0x4f84, 0x4f90] { c.emit(destination as u32)?; }
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R2, 1)?)?;
    c.branch(C::Ne, 0x4fa8)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.branch(C::Al, 0x4fb0)?;
    c.emit(mov(r12, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r12))?;
    c.emit(pop_frame()?)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.emit(pop_saved()?)?;
    c.branch(C::Al, 0x584c)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.emit(pop_saved()?)?;
    c.branch(C::Al, 0x5e2c)?;
    c.emit(mov_register(C::Al, R::R0, R::R2))?;
    c.emit(pop_saved()?)?;
    c.branch(C::Al, 0x5a20)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::LR, 32)?)?;
    c.branch(C::Eq, 0x4f7c)?;
    c.emit(pop_saved()?)?;
    c.branch(C::Al, 0x5670)?;
    c.check(0x4fb8)?;
    Ok(())
}

fn load_with(condition: C, rd: R, rn: R, off: i32) -> Result<u32, String> {
    arm32::transfer_word(condition, true, rd, rn, off)
}
fn store_with(condition: C, rd: R, rn: R, off: i32) -> Result<u32, String> {
    arm32::transfer_word(condition, false, rd, rn, off)
}
fn pop_frame() -> Result<u32, String> {
    transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)
}
fn pop_saved() -> Result<u32, String> {
    transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::LR.bit(),
        BlockMode::IncrementAfter, true)
}
