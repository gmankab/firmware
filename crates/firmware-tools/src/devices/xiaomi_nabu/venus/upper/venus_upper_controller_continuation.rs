// SPDX-License-Identifier: GPL-3.0-only
use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, mov_register,
    mov_shifted_register, multiply_accumulate, multiply_long, multiply_register,
    transfer_word, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x68000;
pub const END: usize = 0x680cc;
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
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;

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

    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 60)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R2, 72)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r3, 255)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xff00)?)?;
    c.emit(mov_shifted_register(C::Al, r4, R::R0, Shift::Lsl, 16)?)?;
    c.emit(mov_shifted_register(C::Al, r5, r4, Shift::Asr, 16)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0)?)?;
    c.branch(C::Lt, 0x67ee4)?;
    c.emit(transfer_word(C::Al, true, r8, r9, 0x834)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 100)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r8, 100)?)?;
    c.emit(multiply_register(C::Al, R::R0, R::R2, r3)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 52)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, r4, Shift::Asr, 16, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, R::R2, r4, Shift::Asr, 14, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::R2, 32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0)?)?;
    c.branch(C::Eq, 0x69b10)?;
    c.emit(transfer_word(C::Al, true, r7, R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R1, R::R1, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r5, r5, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(multiply_register(C::Al, r4, r7, R::R0)?)?;
    c.emit(transfer_word(C::Al, false, r4, R::R2, -24)?)?;
    c.emit(multiply_accumulate(C::Al, r4, r7, R::R0, r3)?)?;
    c.emit(data_register(C::Al, Op::Rsb, R::R0, R::R0, R::R0, Shift::Lsl, 4, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R2, R::R2, 4)?)?;
    c.emit(transfer_word(C::Al, true, r3, R::PC, 0xfec)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 3)?)?;
    c.emit(multiply_long(C::Al, false, r7, r6, R::R0, r3)?)?;
    c.emit(mov_register(C::Al, r3, r4))?;
    c.emit(mov_shifted_register(C::Al, R::R0, r6, Shift::Lsr, 5)?)?;
    c.branch(C::Gt, 0x68084)?;
    c.branch(C::Al, 0x67ee4)?;

    if c.at != END {
        return Err(format!("controller continuation ended at 0x{:x}, expected 0x{END:x}", c.at));
    }
    Ok(c.bytes)
}
