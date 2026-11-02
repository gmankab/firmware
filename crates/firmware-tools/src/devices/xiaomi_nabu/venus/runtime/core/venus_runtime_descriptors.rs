// SPDX-License-Identifier: GPL-3.0-only
//! Device descriptor discovery, allocation, and cleanup for Venus.
//! Translates the bounded resource-registration algorithm into typed A32
//! instructions. Named constants denote public-facing service identifiers;
//! the original instruction bytes are never copied into this source.
use arm32::{
    data_immediate, data_register, mov_register, mov_shifted_register,
    push_registers, transfer_multiple, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

// Firmware's service labels are data pointers, not executable instructions.
const SERVICE_DESC_A: u32 = 0x000e_3500;
const SERVICE_DESC_B: u32 = 0x000e_350d;
const SERVICE_DESC_C: u32 = 0x000e_3520;

fn compare_register(c: &mut Code<'_>, lhs: R, rhs: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::Cmp, R::R0, lhs, rhs,
        Shift::Lsl, 0, false)?)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x7224 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit() |
        r8.bit() | r9.bit() | r10.bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.emit(add(R::R11, R::SP, 28)?)?;
    c.emit(sub(R::SP, R::SP, 52)?)?;
    c.emit(load(R::R0, R::R0, 8)?)?;
    c.emit(load(r7, R::R0, 12)?)?;
    c.emit(load(R::R0, r7, 4)?)?;
    c.emit(add(r5, r7, 16)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x83e0)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x7270)?;
    c.literal(R::R1, 0x73d4)?;
    c.emit(add(r3, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x727c)?;
    c.check(0x7270)?;
    c.emit(mov_register(C::Al, R::R0, r6))?;
    c.emit(sub(R::SP, R::R11, 28)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x727c)?;
    c.emit(load(r4, R::SP, 48)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.call(0x7628)?;
    c.emit(mov_shifted_register(C::Al, R::R0, r4, Shift::Lsl, 2)?)?;
    c.emit(add(R::R1, R::SP, 16)?)?;
    c.call(0x57d4)?;
    c.emit(data_immediate(C::Al, Op::Cmn, R::R0, R::R0, 1)?)?;
    c.branch(C::Eq, 0x7270)?;
    c.emit(load(R::R0, R::SP, 16)?)?;
    c.call(0x765c)?;
    c.literal(R::R1, 0x73d8)?;
    c.emit(add(r3, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x7270)?;
    c.emit(load(R::R0, R::SP, 44)?)?;
    compare_register(&mut c, r4, R::R0)?;
    c.branch(C::Ne, 0x7270)?;
    c.emit(cmp(r4, 0)?)?;
    c.emit(store(r4, R::SP, 12)?)?;
    c.branch(C::Eq, 0x734c)?;
    c.emit(load(r4, R::SP, 48)?)?;
    c.emit(add(r7, r7, 92)?)?;
    c.emit(mov(r8, 0)?)?;
    c.emit(mov(r10, 0)?)?;
    c.emit(store(r8, r7, 0)?)?;
    c.emit(mov(R::R0, 3200)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0)?)?;
    c.emit(mov(r3, 4096)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.check(0x72fc)?;
    c.emit(transfer_word_register(C::Al, true, R::R2, r4, r10,
        Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x0008_0000)?)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r7.bit() | r8.bit(),
        BlockMode::IncrementAfter, false)?)?;
    c.call(0x573c)?;
    c.emit(data_immediate(C::Al, Op::Cmn, R::R0, R::R0, 1)?)?;
    c.branch(C::Eq, 0x7270)?;
    c.emit(load(R::R0, r7, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, r6, r4, r10,
        Shift::Lsl, 2)?)?;
    c.emit(load(r9, r4, 0)?)?;
    c.emit(add(R::R1, R::SP, 20)?)?;
    c.call(0x5888)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r6, r9, Shift::Lsl, 0, false)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsr, 12)?)?;
    c.emit(load(R::R0, R::SP, 20)?)?;
    c.call(0x7638)?;
    c.emit(store(r8, r7, 0)?)?;
    c.emit(add(r10, r10, 1)?)?;
    c.emit(load(R::R0, R::SP, 44)?)?;
    compare_register(&mut c, r10, R::R0)?;
    c.branch(C::Cc, 0x72e8)?;
    c.check(0x734c)?;
    c.literal(R::R1, 0x73dc)?;
    c.emit(add(r3, R::SP, 40)?)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.emit(mov(R::R2, 0)?)?;
    c.call(0x883c)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r6, R::R0, 0)?)?;
    c.branch(C::Ne, 0x7270)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.call(0x7578)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Ne, 0x73cc)?;
    c.emit(load(r4, R::SP, 12)?)?;
    c.emit(cmp(r4, 0)?)?;
    c.branch(C::Eq, 0x73c4)?;
    c.emit(mov(r5, 0)?)?;
    c.check(0x738c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x766c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x7a9c)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x7a54)?;
    c.emit(mov(R::R0, 1)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.call(0x7a0c)?;
    c.emit(mov_register(C::Al, R::R0, r5))?;
    c.call(0x7758)?;
    c.emit(add(r5, r5, 1)?)?;
    compare_register(&mut c, r5, r4)?;
    c.branch(C::Ne, 0x738c)?;
    c.check(0x73c4)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.call(0x75ac)?;
    c.check(0x73cc)?;
    c.emit(mov(r6, 0)?)?;
    c.branch(C::Al, 0x7270)?;
    for service in [SERVICE_DESC_A, SERVICE_DESC_B, SERVICE_DESC_C] {
        c.emit(service)?;
    }
    c.check(0x73e0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn descriptor_registration_code_and_service_ids_are_contiguous() {
        let mut bytes = vec![0; 0x7224 - 0x1000];
        extend(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 0x73e0 - 0x1000);
        assert_eq!(u32::from_le_bytes(bytes[0x73d4-0x1000..0x73d8-0x1000].try_into().unwrap()), SERVICE_DESC_A);
    }
    #[test]
    fn descriptor_registration_rejects_gap() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
