// SPDX-License-Identifier: GPL-3.0-only
//! Venus service initialization with interface probing and cleanup on failure.
//! The service vtable is addressed symbolically through typed A32 load and
//! indirect-call operations; invalid object handles are not dereferenced.
use arm32::{
    branch_exchange_with_link, data_immediate, data_register, mov_register,
    push_registers, transfer_multiple, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

fn bic_tag(c: &mut Code<'_>, src: R) -> Result<(), String> {
    c.emit(data_immediate(C::Al, Op::Bic, R::R0, src, 1)?)
}
fn compare_register(c: &mut Code<'_>, lhs: R, rhs: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::Cmp, R::R0, lhs, rhs, Shift::Lsl, 0, false)?)
}
fn pop_frame(c: &mut Code<'_>, saved: u16) -> Result<(), String> {
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true)?)
}
pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x5214 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit();
    // Instantiate service, call start hook for classes which support it,
    // and validate the accepted context window (under 64 KiB).
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(sub(R::SP, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, r4, r3))?;
    c.emit(mov(r7, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.emit(store(r7, r4, 0)?)?;
    c.call(0x513c)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Eq, 0x5250)?;
    c.emit(store(r7, r4, 0)?)?;
    c.branch(C::Al, 0x52ec)?;
    c.check(0x5250)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(load(R::R1, R::R0, 8)?)?;
    c.emit(load(R::R1, R::R1, 12)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(cmp(R::R1, 1)?)?;
    c.branch(C::Ne, 0x5298)?;
    bic_tag(&mut c, R::R0)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(R::R1, R::R1, 8)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Eq, 0x5294)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x51cc)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x52e8)?;
    c.check(0x5294)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.check(0x5298)?;
    bic_tag(&mut c, R::R0)?;
    c.emit(mov(R::R2, 40)?)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(r3, R::R1, 24)?)?;
    c.emit(mov_register(C::Al, R::R1, R::SP))?;
    c.emit(branch_exchange_with_link(C::Al, r3)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x52ec)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    compare_register(&mut c, R::R0, r5)?;
    c.branch(C::Cc, 0x52d8)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r5, Shift::Lsl, 0, false)?)?;
    c.emit(mov(r6, 0)?)?;
    c.emit(cmp(R::R0, 65536)?)?;
    c.branch(C::Cc, 0x52ec)?;
    c.check(0x52d8)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x51cc)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(r6, 0x8000_0007)?)?;
    c.check(0x52e8)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.check(0x52ec)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(sub(R::SP, R::R11, 16)?)?;
    pop_frame(&mut c, saved)?;
    c.check(0x52f8)?;

    // Resolve a device service by identifier, publish its typed handle,
    // and run an optional initializer. Abort via the common release method.
    let small_saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, small_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 8)?)?;
    c.literal(R::R2, 0x53b8)?;
    c.emit(mov_register(C::Al, r4, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(add(R::R1, R::SP, 4)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(store(R::R0, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x8490)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x5338)?;
    c.emit(mov(R::R0, 0x8000_0006)?)?;
    c.branch(C::Al, 0x53b0)?;
    c.check(0x5338)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.emit(load(R::R0, R::R0, 12)?)?;
    c.emit(load(r3, R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(branch_exchange_with_link(C::Al, r3)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x53b0)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(load(R::R1, R::R0, 8)?)?;
    c.emit(load(R::R1, R::R1, 12)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(cmp(R::R1, 1)?)?;
    c.branch(C::Ne, 0x53ac)?;
    bic_tag(&mut c, R::R0)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(R::R1, R::R1, 8)?)?;
    c.emit(branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x53b0)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x51cc)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.branch(C::Al, 0x53b0)?;
    c.check(0x53ac)?;
    c.emit(mov(R::R0, 0)?)?;
    c.check(0x53b0)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    pop_frame(&mut c, small_saved)?;
    c.emit(0x0010_36f4)?;
    c.check(0x53bc)?;

    // Validate the new device service context using its external descriptor.
    // This mirrors the 40-byte context size and handles cleanup on failure.
    c.emit(push_registers(C::Al, small_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(sub(R::SP, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, r4, r3))?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.call(0x52f8)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x53f0)?;
    c.emit(mov(R::R1, 0)?)?;
    c.emit(store(R::R1, r4, 0)?)?;
    c.branch(C::Al, 0x544c)?;
    c.check(0x53f0)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.emit(mov(R::R2, 40)?)?;
    bic_tag(&mut c, R::R0)?;
    c.emit(load(R::R1, R::R0, 4)?)?;
    c.emit(load(r3, R::R1, 24)?)?;
    c.emit(mov_register(C::Al, R::R1, R::SP))?;
    c.emit(branch_exchange_with_link(C::Al, r3)?)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Ne, 0x544c)?;
    c.emit(load(R::R0, R::SP, 4)?)?;
    compare_register(&mut c, R::R0, r5)?;
    c.branch(C::Cc, 0x5438)?;
    c.emit(data_register(C::Al, Op::Eor, R::R1, R::R0, r5, Shift::Lsl, 0, false)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(R::R1, 65536)?)?;
    c.branch(C::Cc, 0x544c)?;
    c.check(0x5438)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x51cc)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(store(R::R0, r4, 0)?)?;
    c.emit(mov(R::R0, 0x8000_0007)?)?;
    c.check(0x544c)?;
    c.emit(sub(R::SP, R::R11, 8)?)?;
    pop_frame(&mut c, small_saved)?;
    c.check(0x5454)?;
    Ok(())
}
