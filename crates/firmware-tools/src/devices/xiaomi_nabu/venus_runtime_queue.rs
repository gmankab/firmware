// SPDX-License-Identifier: GPL-3.0-only
//! Continuation of Venus interrupt-bit selection and service dispatch.
//! Each operation is represented by typed ARM32 operands; the trailing table
//! contains device-owned service identifiers and status-object addresses.
use arm32::{
    data_immediate, data_immediate_with_flags, data_register,
    data_register_shifted_by_register, mov_register, transfer_multiple,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store, sub};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x23f4 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;

    // Fetch a pending interrupt mask and intersect it with the per-bank
    // availability list; if no bit matches, the loop restarts after polling.
    c.emit(load(R::R0, r4, 0x4c)?)?;
    c.emit(mov_register(C::Al, R::R1, r8))?;
    c.call(0x2dbc)?;
    c.emit(load(R::R0, r4, 0x4c)?)?;
    c.call(0x2d88)?;
    c.emit(mov(R::R1, 6)?)?;
    c.emit(mov_register(C::Al, R::R2, r9))?;
    c.emit(load(r3, R::R2, 0)?)?;
    c.emit(data_register(C::Al, Op::And, r7, r3, R::R0, Shift::Lsl, 0, true)?)?;
    c.branch(C::Ne, 0x242c)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, R::R1, R::R1, 1, true)?)?;
    c.emit(add(R::R2, R::R2, 4)?)?;
    c.branch(C::Ne, 0x2410)?;
    c.branch(C::Al, 0x23f4)?;
    c.check(0x242c)?;

    // Find the highest set service bit using ARM32 register-specified shifts.
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(mov(R::R1, 31)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Tst,
        R::R0, r7, r5, Shift::Lsl, R::R1, false)?)?;
    c.branch(C::Ne, 0x244c)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(sub(R::R1, R::R1, 1)?)?;
    c.emit(cmp(R::R0, 31)?)?;
    c.branch(C::Lt, 0x2434)?;
    c.emit(add(R::R0, R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, R::R0, R::R0, 31)?)?;
    c.emit(cmp(R::R0, 16)?)?;
    c.branch(C::Cs, 0x2494)?;
    c.check(0x245c)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, r6, R::R0, Shift::Lsl, 2)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Bic,
        r7, r7, r5, Shift::Lsl, R::R0, false)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x247c)?;
    c.emit(store(R::R0, r4, 0x44)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r6, R::R0, Shift::Lsl, 2)?)?;
    c.call(0x30d8)?;
    c.branch(C::Al, 0x2488)?;
    c.check(0x247c)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov,
        R::R1, R::R0, r5, Shift::Lsl, R::R0, false)?)?;
    c.emit(load(R::R0, r4, 0x4c)?)?;
    c.call(0x2e40)?;
    c.emit(cmp(r7, 0)?)?;
    c.branch(C::Ne, 0x242c)?;
    c.branch(C::Al, 0x23f4)?;
    c.check(0x2494)?;

    // Unsupported service numbers enter the common diagnostics path.
    c.literal(R::R2, 0x2508)?;
    c.emit(mov(R::R1, 30)?)?;
    c.emit(mov(R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 256)?)?;
    c.emit(sub(R::SP, R::R11, 24)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() |
        R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x17b0)?;
    c.check(0x24b0)?;

    // Labels and status-storage addresses alternate through the eight
    // recognized services. The final fields are additional diagnostic labels.
    const SERVICE_DATA: [u32; 23] = [
        0x0010_306c, 0x000e_2511, 0x0010_e050,
        0x000e_2518, 0x0010_e060, 0x000e_251f,
        0x0010_e040, 0x000e_2526, 0x0010_e020,
        0x000e_252d, 0x0010_e030, 0x000e_2534,
        0x0010_e010, 0x000e_253b, 0x0010_e000,
        0x000e_2542, 0x0010_dff0, 0x000e_286e,
        0x000e_28a0, 0x000e_2788, 0x000e_28b9,
        0x0010_3484, 0x000e_27f9,
    ];
    for value in SERVICE_DATA { c.emit(value)?; }
    c.check(0x250c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_unverified_previous_code_is_accepted() { assert!(extend(&mut vec![]).is_err()); }
}
