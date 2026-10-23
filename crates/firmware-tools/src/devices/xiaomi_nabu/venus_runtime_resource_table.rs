// SPDX-License-Identifier: GPL-3.0-only
//! Registered resource list operations and bounded global service lookup.
use arm32::{
    bx, data_immediate, data_register, mov_register,
    push_registers, transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

const RESOURCE_COUNT: u32 = 0x0010_3944;
const RESOURCE_TABLE: u32 = 0x0010_3948;

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7578 };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let saved = r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit();
    let restored = (saved & !R::LR.bit()) | R::PC.bit();
    // Check a linked resource list; keep the first failed probe, otherwise
    // return the conjunction of every resource's readiness bit.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r5, R::R0))?;
    c.emit(mov(r4, 1)?)?;
    c.check(0x7588)?;
    c.emit(cmp(r5, 0)?)?;
    c.emit(mov_register(C::Eq, R::R0, r4))?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, restored,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R1, r5, 0)?)?;
    c.emit(load(R::R0, r5, 4)?)?;
    c.call(0x77cc)?;
    c.emit(load(r5, r5, 8)?)?;
    c.emit(data_register(C::Al, Op::And, r4, R::R0, r4,
        Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x7588)?;
    c.check(0x75ac)?;
    // Release all associated resource nodes, including callback handles.
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.check(0x75b8)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, restored,
        BlockMode::IncrementAfter, true)?)?;
    c.emit(load(R::R0, r4, 0)?)?;
    c.call(0x75f4)?;
    c.emit(load(R::R1, r4, 0)?)?;
    c.emit(load(R::R0, r4, 4)?)?;
    c.call(0x7828)?;
    c.emit(load(r5, r4, 0)?)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x7a0c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x7758)?;
    c.emit(load(r4, r4, 8)?)?;
    c.branch(C::Al, 0x75b8)?;
    c.check(0x75f4)?;
    // Wait for a resource's completion flag, then deregister it safely.
    let short_saved = r4.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, short_saved)?)?;
    c.emit(add(R::R11, R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.check(0x7600)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x7ad4)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 1)?)?;
    c.branch(C::Eq, 0x7600)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x766c)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R1, r4))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, short_saved,
        BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x7a54)?;
    c.check(0x7628)?;
    // Write service count and its indexed descriptor table.
    c.literal(R::R1, 0x7634)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(RESOURCE_COUNT)?;
    c.check(0x7638)?;
    c.literal(R::R2, 0x7654)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1,
        Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Hi, true, R::R2, R::PC, 12)?)?;
    c.emit(transfer_word(C::Hi, true, R::R2, R::R2, 0)?)?;
    c.emit(transfer_word_register(C::Hi, false, R::R0, R::R2,
        R::R1, Shift::Lsl, 2)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(RESOURCE_COUNT)?;
    c.emit(RESOURCE_TABLE)?;
    c.check(0x765c)?;
    c.literal(R::R1, 0x7668)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(RESOURCE_TABLE)?;
    c.check(0x766c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linked_resources_have_matching_metadata_slots() {
        let mut bytes = vec![0; 0x7578 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x766c - 0x1000);
        assert_eq!(u32::from_le_bytes(bytes[0x7654-0x1000..0x7658-0x1000].try_into().unwrap()), RESOURCE_COUNT);
    }
    #[test]
    fn rejects_missing_resource_prefix() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
