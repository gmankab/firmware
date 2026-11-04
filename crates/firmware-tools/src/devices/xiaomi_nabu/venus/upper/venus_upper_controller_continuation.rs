// SPDX-License-Identifier: GPL-3.0-only
use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, mov_register,
    mov_shifted_register, multiply_accumulate, multiply_long, multiply_register,
    transfer_word, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x68000;
pub const END: usize = 0x68718;
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

    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r8, 0x94)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2000)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, R::R1, r8, 0x9c)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, R::R2, r8, 0x9d)?)?;
    c.branch(C::Ne, 0x6814c)?;
    c.emit(transfer_word(C::Al, true, r6, R::PC, 0xfd8)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::PC, 0xfd8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x370)?)?;
    c.emit(mov_register(C::Al, R::R1, r6))?;
    c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
    for destination_offset in [0xff4, 0xfe8] {
        c.emit(transfer_word(C::Al, true, R::R0, R::PC, destination_offset)?)?;
        c.emit(mov_register(C::Al, R::R1, r6))?;
        c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x370)?)?;
        c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
    }
    for literal_offset in [0xfdc, 0xfd4, 0xfcc] {
        c.emit(transfer_word(C::Al, true, R::R0, R::PC, literal_offset)?)?;
        c.emit(transfer_word(C::Al, true, R::R1, R::PC, literal_offset)?)?;
        c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1b8)?)?;
        c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
    }
    c.emit(transfer_word(C::Al, true, R::R1, r8, 0x54)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r8, 0xa8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0xf00)?)?;
    c.branch(C::Cc, 0x68198)?;
    for (tier, primary, secondary, tertiary, final_source) in [
        (0, 0xf98, 0xf9c, 0xf90, 0xf84),
        (1, 0xf50, 0xf50, 0xf54, 0xf48),
        (2, 0xee8, 0xf0c, 0xf20, 0xf14),
    ] {
        if tier == 1 {
            c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x780)?)?;
            c.branch(C::Cc, 0x681dc)?;
        }
        c.emit(transfer_word(C::Al, true, R::R1, R::PC, primary)?)?;
        c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xb0)?)?;
        c.emit(data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xb0)?)?;
        c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
        c.emit(transfer_word(C::Al, true, R::R1, R::PC, secondary)?)?;
        c.emit(data_immediate(C::Al, Op::Add, R::R0, r8, 0x224)?)?;
        c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x58)?)?;
        c.emit(data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x58)?)?;
        c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
        c.emit(transfer_word(C::Al, true, R::R1, R::PC, tertiary)?)?;
        c.emit(data_immediate(C::Al, Op::Add, R::R0, r8, 0x174)?)?;
        c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x58)?)?;
        c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
        c.emit(transfer_word(C::Al, true, R::R1, R::PC, final_source)?)?;
        if tier != 2 {
            c.branch(C::Al, 0x68214)?;
        }
    }
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r8, 0x1cc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x58)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;

    let r10 = R::new(10)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 34)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r8, 0x98)?)?;
    c.emit(mov_register(C::Al, R::R0, r8))?;
    c.emit(branch(C::Al, c.at as u32, 0x67900, true)?)?;
    c.emit(mov_register(C::Al, r10, R::R0))?;
    c.emit(arm32::transfer_byte(C::Al, false, r10, r8, 0x64c)?)?;
    c.emit(data_register(C::Al, Op::Add, r6, r8, r10, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r6, 0x224)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r8, 0x94)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r10, r8, 0x158)?)?;
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, r8, 0x9d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq, 0x68458)?;
    c.emit(mov_register(C::Al, r3, r8))?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r9, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r4, R::R0, 3)?)?;
    c.emit(transfer_word(C::Al, true, r8, r3, 0x54)?)?;
    c.emit(transfer_word(C::Al, true, r5, r3, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R12, r3, 0x58)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r3, 0x60)?)?;
    c.emit(mov_shifted_register(C::Al, r7, r8, Shift::Lsr, 7)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, r8, Shift::Lsr, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r7, 14)?)?;
    c.emit(transfer_word(C::Al, false, r7, R::SP, 44)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 40)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R2, R::R0, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r7, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r9, R::R0, 3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r9, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0x2000)?)?;
    c.emit(mov_register(C::Eq, r9, R::R2))?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r7, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R2, R::R0, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R2, R::R0, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r7, 14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r7, R::R0, 3)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r7, R::R0, 10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r7, R::R0, 3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0x2000)?)?;
    c.emit(mov_register(C::Eq, r7, R::R2))?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 100)?)?;
    c.emit(multiply_register(C::Al, r3, R::R1, R::R2)?)?;
    c.emit(multiply_register(C::Al, R::R1, r8, R::R12)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r3, R::R1, Shift::Lsr, 1, false)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 15)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0x2000)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 4)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 12)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x68338)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0x2000)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R2, R::R0, 24)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 20)?)?;
    c.emit(data_immediate(C::Ls, Op::Mov, R::R1, R::R0, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0x2000)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R2, R::R0, 16)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r4, R::R1))?;
    c.emit(mov_register(C::Al, R::R2, r4))?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 22)?)?;
    c.emit(mov_register(C::Al, r4, R::R0))?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r5, 0x2000)?)?;
    c.emit(arm32::signed_multiply_bottom_halves(C::Al, R::R1, R::R2, R::R1)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R2, R::R1, 0xfe)?)?;
    c.branch(C::Ne, 0x68360)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r8, 0xf00)?)?;
    c.branch(C::Cc, 0x68388)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xdb4)?)?;
    c.branch(C::Al, 0x683a4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r8, 0xf00)?)?;
    c.branch(C::Cc, 0x68398)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xda0)?)?;
    c.branch(C::Al, 0x683a4)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::PC, 0x6f4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 126)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x17b0, true)?)?;
    c.branch(C::Al, 0x67f54)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r8, 0x780)?)?;
    c.emit(transfer_word(C::Cc, true, R::R1, R::PC, 0xd70)?)?;
    c.emit(transfer_word(C::Cs, true, R::R1, R::PC, 0xd74)?)?;
    c.branch(C::Al, 0x683a4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r8, 0x780)?)?;
    c.emit(transfer_word(C::Cc, true, R::R1, R::PC, 0xd5c)?)?;
    c.emit(transfer_word(C::Cs, true, R::R1, R::PC, 0xd60)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R2, Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::SP, 64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 88)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x471c, true)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R0, r8, 0x780)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 7)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 14)?)?;
    c.branch(C::Hi, 0x68410)?;
    c.emit(transfer_word(C::Al, true, r8, R::SP, 60)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r4, r9, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x6842c)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::SP, 44)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, true, r9, R::SP, 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 14)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::SP, 40)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 6)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 14)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r4, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::SP, 64)?)?;
    c.branch(C::Ls, 0x69b24)?;
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, r10, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, r7, 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r4, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ls, Op::Sub, R::R2, R::R2, 0x37)?)?;
    c.branch(C::Al, 0x6843c)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::SP, 64)?)?;
    c.emit(transfer_word(C::Al, true, r8, R::SP, 60)?)?;
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, r10, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, r7, 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r4, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ls, Op::Sub, R::R2, R::R2, 0x37)?)?;
    c.branch(C::Al, 0x68438)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::SP, 64)?)?;
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, r10, Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R2, R::R0, 0x73)?)?;
    c.emit(transfer_word(C::Al, true, r9, R::SP, 36)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xe40)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, r8, 0x94)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x2ec94, true)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xe34)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x2ec94, true)?)?;

    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x18)?)?;
    for offset in [0x658, 0x65c] {
        c.emit(transfer_word(C::Al, false, R::R0, r8, offset)?)?;
    }
    for offset in [0x634, 0x638, 0x63c] {
        c.emit(transfer_word(C::Al, true, R::R1, r6, 0x174)?)?;
        c.emit(transfer_word(C::Al, false, R::R1, r8, offset)?)?;
        c.emit(transfer_word(C::Al, true, R::R1, r6, 0x1cc)?)?;
        c.emit(transfer_word(C::Al, false, R::R1, r8, offset + 12)?)?;
    }
    for offset in [0xa0, 0x9e, 0xa2] {
        c.emit(arm32::store_halfword(C::Al, R::R0, r8, offset)?)?;
    }
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r8, 0x15c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x31ec, true)?)?;
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, r9, 0x8f8)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, R::R0, r8, 0x27d)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r9, 0x408)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r8, 0x280)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r9, 0x18)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, R::R0, r8, 0x284)?)?;
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, r8, 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.branch(C::Hi, 0x68500)?;
    c.emit(transfer_word(C::Al, true, R::R0, r8, 0x70)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq, 0x684f4)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 0x60)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R1, Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Cc, false, R::R1, r8, 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r8, 0x74)?)?;
    c.branch(C::Al, 0x6853c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r8, 0x74)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq, 0x68524)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 0x48)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R1, Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsr, 1, false)?)?;
    c.emit(mov_shifted_register(C::Cc, R::R0, R::R1, Shift::Lsr, 1)?)?;
    c.emit(transfer_word(C::Cc, false, R::R0, r8, 0x74)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r8, 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq, 0x6853c)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 0x48)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Cc, false, R::R1, r8, 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x234)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r8, 0xa80)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::R2, 4)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, r8, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, r8, 0x23c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1000)?)?;
    c.emit(arm32::transfer_word_postindexed(C::Al, true, r3, R::R2, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 6)?)?;
    c.emit(arm32::transfer_word_postindexed(C::Al, false, r3, R::R1, 4)?)?;
    c.branch(C::Ne, 0x68560)?;
    c.emit(transfer_word(C::Al, true, R::R0, r8, 0x5c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x258)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, r8, R::R1, Shift::Lsl, 0)?)?;
    for (destination, source, value) in [
        (0x260, 0x64, R::R12),
        (0x264, 0x68, R::LR),
        (0x254, 0x74, r3),
        (0x25c, 0x70, r6),
    ] {
        c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, destination)?)?;
        c.emit(transfer_word(C::Al, true, value, r8, source)?)?;
        c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
        c.emit(arm32::transfer_word_register(C::Al, false, value, r8, R::R0, Shift::Lsl, 0)?)?;
    }
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, r8, 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 6)?)?;
    c.branch(C::Cc, 0x685ec)?;
    c.emit(arm32::transfer_byte(C::Al, true, R::R1, r8, 0x89)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 1)?)?;
    c.branch(C::Ne, 0x685f8)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1000)?)?;
    c.branch(C::Eq, 0x685f4)?;
    c.branch(C::Al, 0x68600)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 5)?)?;
    c.branch(C::Ne, 0x68608)?;
    c.emit(arm32::transfer_byte(C::Al, true, R::R1, r8, 0x89)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Ne, 0x68608)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 3)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, R::R0, r8, 0x25)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xf98)?)?;
    c.emit(transfer_word(C::Al, true, r7, R::PC, 0xf98)?)?;
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R1, R::R0, Shift::Lsl, 2)?)?;
    c.emit(arm32::transfer_byte_register(C::Al, true, r7, r7, R::R0, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r8, 0x2a0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xff8)?)?;
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R1, R::R0, Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r8, 0x2a4)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xff0)?)?;
    c.emit(arm32::transfer_byte_register(C::Al, true, R::R1, R::R1, R::R0, Shift::Lsl, 0)?)?;
    for (value, offset) in [(R::R1, 0x2a8), (r7, 0x2a9), (R::R1, 0x2aa)] {
        c.emit(arm32::transfer_byte(C::Al, false, value, r8, offset)?)?;
    }
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xfe0)?)?;
    c.emit(arm32::transfer_byte_register(C::Al, true, r5, R::R1, R::R0, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xfdc)?)?;
    c.emit(arm32::transfer_byte_register(C::Al, true, r7, R::R1, R::R0, Shift::Lsl, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r5, r8, 0x2ab)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r7, r8, 0x2ac)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r8, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1000)?)?;
    c.branch(C::Ne, 0x6868c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 3)?)?;
    c.branch(C::Cc, 0x68674)?;
    c.emit(mov_shifted_register(C::Al, r5, r5, Shift::Lsl, 1)?)?;
    c.emit(mov_shifted_register(C::Al, r7, r7, Shift::Lsl, 1)?)?;
    c.branch(C::Al, 0x68684)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 2)?)?;
    c.branch(C::Eq, 0x6868c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r5, R::R0, 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r7, R::R0, 4)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r5, r8, 0x2ab)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r7, r8, 0x2ac)?)?;
    c.emit(transfer_word(C::Al, true, r4, R::PC, 0xf98)?)?;
    c.emit(mov_register(C::Al, r10, r8))?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r6, 0)?)?;
    c.emit(arm32::transfer_byte_register(C::Al, true, r4, r4, R::R0, Shift::Lsl, 0)?)?;
    c.emit(arm32::transfer_byte(C::Al, false, r4, r8, 0x2ad)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r4, R::R0, 0)?)?;
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, r4, r10, 0x758)?)?;
    c.emit(transfer_word(C::Al, false, r4, r10, 4)?)?;
    c.emit(transfer_word(C::Al, true, r4, r10, -0x710)?)?;
    c.branch(C::Eq, 0x686dc)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x686f4)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, r3, r4, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r4, r3, r4, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r3, R::R2, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r6, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Cs, r3, R::R2))?;
    c.emit(mov_register(C::Cc, r6, r4))?;
    c.branch(C::Al, 0x68710)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r3, r4, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x686fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r6, r3, r4, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r3, R::R0, 0)?)?;
    c.branch(C::Al, 0x68710)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0)?)?;
    c.branch(C::Al, 0x68710)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 1)?)?;
    c.branch(C::Hi, 0x68718)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r3, R::R0, R::R0, Shift::Lsr, 1, true)?)?;
    c.emit(arm32::rotate_right_with_extend(C::Al, r6, r4, false)?)?;
    c.emit(transfer_word(C::Al, false, r6, r10, 0)?)?;
    c.emit(transfer_word(C::Al, false, r3, r10, 4)?)?;

    if c.at != END {
        return Err(format!("controller continuation ended at 0x{:x}, expected 0x{END:x}", c.at));
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appended_controller_extent_and_carry_chain() {
        let bytes = fragment().unwrap();
        assert_eq!(END, 0x68718);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(END - 0x68458, 704);
        let instruction = |at: usize| {
            u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
        };
        assert_eq!(instruction(0x68708), data_register(
            C::Al, Op::Mov, R::new(3).unwrap(), R::R0, R::R0, Shift::Lsr, 1, true,
        ).unwrap());
        assert_eq!(instruction(0x6870c), arm32::rotate_right_with_extend(
            C::Al, R::new(6).unwrap(), R::new(4).unwrap(), false,
        ).unwrap());
    }

    #[test]
    fn appended_controller_matches_optional_canonical_reference() {
        let Some(path) = std::env::var_os("VENUS_REFERENCE") else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        assert_eq!(&bytes[..], &reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
        assert_eq!(&bytes[0x68458 - START..],
            &reference[0x68458 + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
