// SPDX-License-Identifier: GPL-3.0-only
//! Resource-bank switching, protection-key handshakes, and indexed bit reads.
//! Two hardware commands use common instruction-level generators; their
//! signatures are described as device-specific command data.
use arm32::{
    bx, data_immediate, data_register, data_register_shifted_by_register,
    transfer_word_register, Condition as C, DataOp as Op, Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{cmp, load, mov, store, sub};

const COUNT: u32 = 0x0010_3944;
const TABLE: u32 = 0x0010_3948;
const ACTIVE: u32 = 0x0010_cfa8;
const UNLOCK: u32 = 0xc5ac_ce55;
const LOCK: u32 = 0x3a53_31aa;

/// Select an indexed controller bank and write its handshake token.
fn select_bank(c: &mut Code<'_>, token: u32) -> Result<(), String> {
    let start = c.at;
    c.literal(R::R1, start + 0x34)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0,
        Shift::Lsl, 0, false)?)?;
    c.emit(bx(C::Ls, R::LR))?;
    c.literal(R::R1, start + 0x38)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R1,
        R::R0, Shift::Lsl, 2)?)?;
    c.literal(R::R1, start + 0x3c)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.literal(R::R1, start + 0x40)?;
    c.emit(store(R::R1, R::R0, 0xfb0)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [COUNT, TABLE, ACTIVE, token] { c.emit(data)?; }
    Ok(())
}

/// Write a handshake key to all currently registered banks, last to first.
fn set_all_banks(c: &mut Code<'_>, token: u32) -> Result<(), String> {
    let start = c.at;
    c.literal(R::R0, start + 0x44)?;
    c.emit(load(R::R0, R::R0, 0)?)?;
    c.emit(sub(R::R0, R::R0, 1)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(bx(C::Lt, R::LR))?;
    c.literal(R::R1, start + 0x48)?;
    c.literal(R::R2, start + 0x4c)?;
    c.emit(load(R::R12, R::R1, 0)?)?;
    c.check(start + 0x20)?;
    c.emit(transfer_word_register(C::Al, true, R::new(3)?, R::R12,
        R::R0, Shift::Lsl, 2)?)?;
    c.emit(sub(R::R0, R::R0, 1)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(store(R::new(3)?, R::R2, 0)?)?;
    c.emit(load(R::new(3)?, R::R2, 0)?)?;
    c.literal(R::R1, start + 0x50)?;
    c.emit(store(R::R1, R::new(3)?, 0xfb0)?)?;
    c.branch(C::Ge, start + 0x20)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [COUNT, TABLE, ACTIVE, token] { c.emit(data)?; }
    Ok(())
}

fn read_bank_bit(c: &mut Code<'_>) -> Result<(), String> {
    let start = c.at;
    c.literal(R::R1, start + 0x28)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R1,
        R::R0, Shift::Lsl, 2)?)?;
    c.literal(R::R1, start + 0x2c)?;
    c.emit(store(R::R0, R::R1, 0)?)?;
    c.emit(load(R::R0, R::R1, 0)?)?;
    c.emit(mov(R::R1, 1)?)?;
    c.emit(load(R::R0, R::R0, 0xfb4)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R1, R::R0,
        Shift::Lsr, 1, false)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.emit(TABLE)?;
    c.emit(ACTIVE)?;
    Ok(())
}

fn read_indexed_flag(c: &mut Code<'_>) -> Result<(), String> {
    c.literal(R::R2, 0x781c)?;
    c.emit(load(R::new(3)?, R::R2, 0)?)?;
    c.emit(mov(R::R2, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::new(3)?, R::R1,
        Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x7814)?;
    c.literal(R::R2, 0x7820)?;
    c.emit(load(R::R2, R::R2, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R2,
        R::R1, Shift::Lsl, 2)?)?;
    c.literal(R::R2, 0x7824)?;
    c.emit(store(R::R1, R::R2, 0)?)?;
    c.emit(load(R::R1, R::R2, 0)?)?;
    c.emit(mov(R::R2, 1)?)?;
    c.emit(load(R::R1, R::R1, 0)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::And, R::R1, R::R1,
        R::R2, Shift::Lsl, R::R0, false)?)?;
    c.emit(mov(R::R2, 255)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 3840)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1, R::R2,
        Shift::Lsl, 0, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, R::R2, R::R0,
        R::R1, Shift::Lsr, R::R0, false)?)?;
    c.check(0x7814)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R2, 255)?)?;
    c.emit(bx(C::Al, R::LR))?;
    for data in [COUNT, TABLE, ACTIVE] { c.emit(data)?; }
    Ok(())
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x766c };
    select_bank(&mut c, UNLOCK)?;
    c.check(0x76b0)?;
    set_all_banks(&mut c, UNLOCK)?;
    c.check(0x7704)?;
    set_all_banks(&mut c, LOCK)?;
    c.check(0x7758)?;
    select_bank(&mut c, LOCK)?;
    c.check(0x779c)?;
    read_bank_bit(&mut c)?;
    c.check(0x77cc)?;
    read_indexed_flag(&mut c)?;
    c.check(0x7828)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_bank_keys_share_a_checked_encoder() {
        let mut code = vec![0; 0x766c - 0x1000];
        extend(&mut code).unwrap();
        assert_eq!(code.len(), 0x7828 - 0x1000);
        assert_eq!(u32::from_le_bytes(code[0x7700-0x1000..0x7704-0x1000].try_into().unwrap()), UNLOCK);
        assert_eq!(u32::from_le_bytes(code[0x7754-0x1000..0x7758-0x1000].try_into().unwrap()), LOCK);
    }
    #[test]
    fn reject_noncontiguous_bank_code() {
        assert!(extend(&mut Vec::new()).is_err());
    }
}
