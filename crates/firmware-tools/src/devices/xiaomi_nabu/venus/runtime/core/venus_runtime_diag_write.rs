// SPDX-License-Identifier: GPL-3.0-only
//! Diagnostic stream writes: chunked padding, record forwarding and
//! callback dispatch. Arithmetic and branches use typed ARM A32 operands.
use arm32::{
    branch_exchange_with_link, bx, data_immediate, data_register,
    load_signed_byte, mov_register, push_registers, transfer_byte_register_offset,
    transfer_multiple, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x43dc };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    let stack_saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | R::LR.bit();

    // Emit a potentially large padding run in 256-byte blocks, with
    // last-block truncation and a flag-dependent bypass.
    c.emit(push_registers(C::Al, stack_saved)?)?;
    c.emit(sub(R::SP, R::SP, 256)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(load(R::R0, R::SP, 280)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 73728)?)?;
    c.branch(C::Ne, 0x4448)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, r3, Shift::Lsl, 0, false)?)?;
    c.branch(C::Le, 0x4448)?;
    c.emit(data_register(C::Al, Op::Sub, r5, R::R2, r3, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r6, R::SP))?;
    c.emit(mov(R::R2, 256)?)?;
    c.check(0x4408)?;
    c.emit(cmp(r5, 256)?)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(mov_register(C::Cc, R::R2, r5))?;
    c.call(0x31ec)?;
    c.check(0x4418)?;
    c.emit(cmp(r5, 256)?)?;
    c.branch(C::Cc, 0x4438)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(mov(R::R2, 256)?)?;
    c.call(0x4290)?;
    c.emit(sub(r5, r5, 256)?)?;
    c.branch(C::Al, 0x4418)?;
    c.check(0x4438)?;
    c.emit(mov_register(C::Al, R::R1, R::SP))?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x4290)?;
    c.check(0x4448)?;
    c.emit(add(R::SP, R::SP, 256)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (stack_saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x4450)?;

    // Stream output through either its direct callback or an internal
    // line-buffered path. A failed callback returns the number actually
    // written; successful writes advance the registered stream cursor.
    let callback_saved = r4.bit() | r5.bit() | r6.bit() | r7.bit()
        | r8.bit() | r9.bit() | r10.bit() | R::LR.bit();
    let callback_ret = (callback_saved & !R::LR.bit()) | R::PC.bit();
    c.emit(push_registers(C::Al, callback_saved)?)?;
    c.emit(mov_register(C::Al, r4, R::R2))?;
    c.emit(mov_register(C::Al, r5, R::R1))?;
    c.emit(mov_register(C::Al, r6, R::R0))?;
    c.emit(load(R::R1, r4, 16)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x4508)?;
    c.emit(load(R::R0, r4, 20)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, r5, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x4494)?;
    c.emit(load(r3, r4, 36)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, callback_saved,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(bx(C::Al, r3))?;
    c.check(0x4494)?;
    c.emit(load_signed_byte(C::Al, R::R1, r4, 75)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Lt, 0x4524)?;
    c.emit(data_register(C::Al, Op::Add, r8, r6, r5, Shift::Lsl, 0, false)?)?;
    c.emit(mov(r9, 0)?)?;
    c.emit(mov(r7, 0)?)?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.check(0x44b0)?;
    c.emit(sub(R::R1, r8, 1)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x4528)?;
    c.emit(transfer_byte_register_offset(C::Al, true,
        r3, R::R1, r7, Shift::Lsl, 0, true)?)?;
    c.emit(add(r7, r7, 1)?)?;
    c.emit(sub(R::R2, R::R2, 1)?)?;
    c.emit(cmp(r3, 10)?)?;
    c.branch(C::Ne, 0x44b4)?;
    c.emit(load(r3, r4, 36)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r5, r7, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(add(r9, R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(mov_register(C::Al, R::R2, r9))?;
    c.emit(branch_exchange_with_link(C::Al, r3)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r9, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Cc, true, R::SP, callback_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(add(R::R0, r8, 1)?)?;
    c.emit(sub(r5, r7, 1)?)?;
    c.emit(data_register(C::Al, Op::Sub, r6, R::R0, r7, Shift::Lsl, 0, false)?)?;
    c.emit(load(R::R0, r4, 20)?)?;
    c.branch(C::Al, 0x4528)?;
    c.check(0x4508)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x4b1c)?;
    c.emit(mov_register(C::Al, R::R1, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x4548)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, callback_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x4524)?;
    c.emit(mov(r9, 0)?)?;
    c.check(0x4528)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.call(0x471c)?;
    c.emit(load(R::R0, r4, 20)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, r5, Shift::Lsl, 0, false)?)?;
    c.emit(store(R::R0, r4, 20)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r9, r5, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, callback_ret,
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x4548)?;
    c.emit(load(R::R1, r4, 16)?)?;
    c.branch(C::Al, 0x446c)?;
    c.check(0x4550)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_preceding_writer() { assert!(extend(&mut Vec::new()).is_err()); }
}
