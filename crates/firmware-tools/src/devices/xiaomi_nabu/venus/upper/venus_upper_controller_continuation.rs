// SPDX-License-Identifier: GPL-3.0-only
use arm32::{
    branch, branch_exchange_immediate, data_immediate, mov_shifted_register,
    transfer_word, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x68000;
pub const END: usize = 0x68038;
pub const FILE_MAPPING_DELTA: usize = 0x4000;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("controller continuation output address 0x{:x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, false)?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r8 = R::new(8)?;

    c.branch(C::Al, 0x6800c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r4, 28)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r4, 32)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r8, 72)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 92)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r4, 56)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r5, r5, 1)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 52)?)?;
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, r5, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x67fd4)?;
    c.branch(C::Al, 0x67f54)?;

    if c.at != END {
        return Err(format!("controller continuation ended at 0x{:x}, expected 0x{END:x}", c.at));
    }
    Ok(c.bytes)
}
