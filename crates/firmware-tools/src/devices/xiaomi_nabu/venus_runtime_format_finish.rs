// SPDX-License-Identifier: GPL-3.0-only
//! Final conversion dispatch, diagnostics flushing and internal literal data.
//! The jump table names function entry offsets instead of embedding machine
//! opcodes; ARM instructions use typed public ARM architecture encodings.
use arm32::{
    data_immediate, data_register, load_halfword, mov_register,
    mov_shifted_register, transfer_byte, transfer_multiple,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};
use super::venus_runtime_services::Code;
use super::venus_runtime_tail::{add, cmp, load, mov, store};

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let mut c = Code { bytes, at: 0x3e58 };
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;

    // Copy a fixed diagnostic header from a platform string into a temporary
    // stack buffer, then append the conversion character and flush to sink.
    c.literal(R::R0, 0x3fc8)?;
    c.emit(add(R::R1, R::SP, 120)?)?;
    c.emit(mov(r9, 18)?)?;
    c.emit(mov_register(C::Al, R::R2, R::R1))?;
    let copied = r3.bit() | r4.bit() | R::R12.bit() | R::LR.bit();
    c.emit(transfer_multiple(C::Al, true, R::R0, copied, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R2, copied, BlockMode::IncrementAfter, true)?)?;
    c.emit(load_halfword(C::Al, r3, R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 2)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, R::R2, 2)?)?;
    c.emit(load(R::R0, R::SP, 48)?)?;
    c.emit(arm32::store_halfword(C::Al, r3, R::R2, 0)?)?;
    c.emit(mov(R::R2, 18)?)?;
    c.emit(transfer_byte(C::Al, false, r6, R::SP, 121)?)?;
    c.call(0x4290)?;
    c.branch(C::Al, 0x3e4c)?;

    // End of format input: visit unresolved positional arguments, invoking
    // the output service for each present entry.
    c.check(0x3e94)?;
    c.emit(load(r5, R::SP, 176)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.emit(mov_register(C::Al, R::R0, r4))?;
    c.branch(C::Ne, 0x3f0c)?;
    c.emit(load(R::R0, R::SP, 44)?)?;
    c.emit(cmp(R::R0, 0)?)?;
    c.branch(C::Eq, 0x391c)?;
    c.emit(mov(r4, 1)?)?;
    c.check(0x3eb4)?;
    c.emit(cmp(r4, 9)?)?;
    c.branch(C::Gt, 0x3ef4)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, r5, r4, Shift::Lsl, 2)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Eq, 0x3ef4)?;
    c.emit(load(R::R0, R::SP, 28)?)?;
    c.emit(load(R::R2, R::SP, 32)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, r4, Shift::Lsl, 3, false)?)?;
    c.call(0x42e0)?;
    c.emit(add(r4, r4, 1)?)?;
    c.branch(C::Al, 0x3eb4)?;
    c.check(0x3ee0)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, r5, r4, Shift::Lsl, 2)?)?;
    c.emit(add(r4, r4, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.emit(cmp(R::R1, 0)?)?;
    c.branch(C::Ne, 0x3f0c)?;
    c.check(0x3ef4)?;
    c.emit(cmp(r4, 9)?)?;
    c.branch(C::Le, 0x3ee0)?;
    c.emit(mov(R::R0, 1)?)?;
    c.branch(C::Al, 0x3f0c)?;
    c.check(0x3f04)?;
    c.emit(store(r6, R::SP, 116)?)?;
    c.check(0x3f08)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0)?)?;
    c.check(0x3f0c)?;
    c.emit(add(R::SP, R::SP, 140)?)?;
    let saved = r4.bit() | r5.bit() | r6.bit() | r7.bit()
        | r8.bit() | r9.bit() | r10.bit() | R::R11.bit() | R::PC.bit();
    c.emit(transfer_multiple(C::Al, true, R::SP, saved,
        BlockMode::IncrementAfter, true)?)?;

    // 38-way conversion jump table. All entries are source-level symbolic
    // destinations within this same program. These are data pointers, not
    // instructions. Most variants take the common field-width path.
    c.check(0x3f14)?;
    c.literal(r9, 0x3fc4)?;
    c.emit(mov_shifted_register(C::Al, R::R2, R::R0, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r3, R::PC, 4)?)?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(transfer_word_register(C::Al, true, R::PC, R::R2, r3, Shift::Lsl, 0)?)?;
    c.check(0x3f28)?;
    // Index 0 is vector output; indices 1..37 map to conversion handlers.
    let dispatch: [u32; 38] = [
        0x3d70, 0x3c84, 0x3c84, 0x3c84, 0x3c84, 0x3a24,
        0x3c84, 0x3c84, 0x3c84, 0x3c84, 0x3c84, 0x3c84,
        0x3c84, 0x3c84, 0x3e58, 0x3c84, 0x3c08, 0x3aec,
        0x3e58, 0x3e58, 0x3e58, 0x3c84, 0x3aec, 0x3c84,
        0x3c84, 0x3c84, 0x3c28, 0x3990, 0x3a84, 0x3a0c,
        0x3c84, 0x3c84, 0x3c3c, 0x3c84, 0x3b50, 0x3c84,
        0x3c84, 0x3a24,
    ];
    for destination in dispatch { c.emit(destination)?; }
    c.check(0x3fc0)?;
    // OEM diagnostic format/data addresses in the original image's literal
    // pool. They are treated as typed pointers rather than instruction data.
    for address in [0x000e_2949, 0x000e_293f, 0x000e_2b2c,
                    0x000e_2938, 0x000e_2b19] { c.emit(address)?; }
    c.check(0x3fd4)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extension_requires_first_converter_pass() { assert!(extend(&mut Vec::new()).is_err()); }
}
