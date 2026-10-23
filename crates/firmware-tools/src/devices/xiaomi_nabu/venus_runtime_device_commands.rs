// SPDX-License-Identifier: GPL-3.0-only
//! Runtime command dispatch: typed ARM32 jump table with validated branches.
//! Command variants call independent device service operations and share a
//! single status/result release path.
use arm32::{
    data_immediate, mov_register, mov_shifted_register,
    push_registers, transfer_multiple, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x61e8 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 16)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(mov_register(C::Al, r5, R::R2))?;
    c.emit(mov_register(C::Al, r6, R::R1))?;
    c.emit(load(r7, R::R0, 12)?)?;
    c.emit(load(R::R0, r7, 88)?)?;
    c.call(0x631c)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r4, R::R0, 0)?)?;
    c.emit(cmp(r6, 9)?)?;
    c.branch(C::Hi, 0x6308)?;
    c.emit(mov_shifted_register(C::Al, R::R0, r6, Shift::Lsl, 2)?)?;
    c.emit(add(R::R1, R::PC, 0)?)?;
    c.emit(transfer_word_register(C::Al, true,
        R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.check(0x6220)?;
    for target in [0x6248, 0x6308, 0x6308, 0x625c, 0x626c,
                   0x627c, 0x6288, 0x62a0, 0x62b0, 0x62c4] {
        c.emit(target)?;
    }
    c.check(0x6248)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x62e0)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.call(0x6ea0)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x625c)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x62f0)?;
    c.call(0x6b80)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x626c)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x62f8)?;
    c.call(0x6bd4)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x627c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x6c54)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x6288)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x6308)?;
    c.call(0x6c40)?;
    c.call(0x6c54)?;
    c.branch(C::Al, 0x6308)?;
    c.check(0x62a0)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x6300)?;
    c.call(0x6d00)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x62b0)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x6304)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.branch(C::Al, 0x62d4)?;
    c.check(0x62c4)?;
    c.emit(mov(r4, 0)?)?;
    c.emit(cmp(r5, 0)?)?;
    c.branch(C::Eq, 0x6308)?;
    c.emit(mov(R::R0, 1)?)?;
    c.check(0x62d4)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x6da8)?;
    c.branch(C::Al, 0x6308)?;
    c.check(0x62e0)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(mov(r4, 0)?)?;
    c.call(0x6ea0)?;
    c.branch(C::Al, 0x6308)?;
    c.check(0x62f0)?;
    c.call(0x6bac)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x62f8)?;
    c.call(0x6c00)?;
    c.branch(C::Al, 0x6304)?;
    c.check(0x6300)?;
    c.call(0x6d64)?;
    c.check(0x6304)?;
    c.emit(mov(r4, 0)?)?;
    c.check(0x6308)?;
    c.emit(load(R::R0, r7, 88)?)?;
    c.call(0x6e60)?;
    c.call(0x6e08)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x631c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_command_targets_have_stable_checked_offsets() {
        let mut bytes = vec![0; 0x61e8 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x631c - 0x1000);
        let table = &bytes[(0x6220 - 0x1000)..(0x6248 - 0x1000)];
        let entries: Vec<u32> = table.chunks_exact(4)
            .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
            .collect();
        assert_eq!(entries, [0x6248, 0x6308, 0x6308, 0x625c, 0x626c,
                             0x627c, 0x6288, 0x62a0, 0x62b0, 0x62c4]);
    }

    #[test]
    fn command_dispatch_rejects_layout_gaps() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
