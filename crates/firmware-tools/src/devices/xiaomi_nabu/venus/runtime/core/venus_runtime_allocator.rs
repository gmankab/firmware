// SPDX-License-Identifier: GPL-3.0-only
//! Venus service allocator, slot lookup, and release entry points.
//! Only public ARM A32 instruction encodings and named device constants are
//! emitted; no vendor instruction arrays are stored here.
use arm32::{
    data_immediate, data_register, data_register_shifted_by_register, mov_register,
    push_registers, transfer_multiple, transfer_word_postindexed,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x250c };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    // Probe memory-bank availability before touching the resource block.
    c.emit(push_registers(C::Al, r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.literal(R::R0, 0x254c)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x252c)?;
    c.call(0x25888)?;
    c.emit(mov_register(C::Al, r5, r4))?;
    c.emit(transfer_word_postindexed(C::Al, true, R::R0, r5, 72)?)?;
    c.call(0x2bcc)?;
    c.emit(add(R::R0, r4, 76)?)?;
    c.call(0x2ce0)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x2b18)?;
    c.emit(0x0010_306c)?; // Per-device MMIO resource table
    c.check(0x2550)?;

    // Resolve an existing slot, otherwise locate the first unused slot in the
    // sixteen-entry resource table. Preserve fallible diagnostic paths.
    c.emit(push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r7, R::R2))?;
    c.literal(R::R2, 0x2664)?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(R::R1, 111)?)?;
    c.emit(mov_register(C::Al, r5, r3))?;
    c.emit(cmp(r6, 16)?)?;
    c.emit(data_immediate(C::Lt, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x2bf4)?;
    c.emit(cmp(r6, 0)?)?;
    c.branch(C::Lt, 0x25d4)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r4, r6, Shift::Lsl, 2, false)?)?;
    c.emit(load(R::R0, R::R0, 4)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_register(C::Ne, Op::Cmp, R::R0, R::R0, r7, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x2618)?;
    c.literal(R::R1, 0x2668)?;
    c.literal(R::R2, 0x266c)?;
    c.emit(mov(R::R0, 8)?)?;
    c.emit(mov(r3, 130)?)?;
    c.call(0x2ec94)?;
    c.literal(R::R1, 0x2670)?;
    c.emit(mov(R::R0, 8)?)?;
    c.call(0x2ec94)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x2c34)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.branch(C::Al, 0x265c)?;
    c.check(0x25d4)?;
    c.emit(add(R::R1, r4, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.check(0x25dc)?;
    c.emit(load(R::R2, R::R1, 0)?)?;
    c.emit(cmp(R::R2, 0)?)?;
    c.branch(C::Eq, 0x2604)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(add(R::R1, R::R1, 4)?)?;
    c.emit(cmp(R::R0, 15)?)?;
    c.branch(C::Lt, 0x25dc)?;
    c.emit(add(r6, R::R0, 1)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x260c)?;
    c.check(0x2604)?;
    c.emit(add(r6, R::R0, 1)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.check(0x260c)?;
    c.literal(R::R2, 0x2664)?;
    c.emit(mov(R::R1, 126)?)?;
    c.call(0x17b0)?;
    c.check(0x2618)?;
    c.literal(R::R2, 0x2664)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r4, r6, Shift::Lsl, 2, false)?)?;
    c.emit(cmp(r6, 16)?)?;
    c.emit(mov(R::R1, 328)?)?;
    c.emit(store(r7, R::R0, 4)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 1)?)?;
    c.call(0x17b0)?;
    c.emit(load(R::R1, r4, 76)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, R::R2,
        R::R0, R::R0, Shift::Lsl, r6, false)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x2924)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x293c)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x2c34)?;
    c.check(0x265c)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    for literal in [0x000e_27f9, 0x000e_282f, 0x000e_284d, 0x000e_2788] {
        c.emit(literal)?;
    }
    c.check(0x2674)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn truncated_prefix_is_rejected() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
