use arm32::{
    branch, branch_exchange_with_link, data_immediate, data_immediate_with_flags,
    data_register, load_halfword, multiply_long, multiply_register, store_halfword,
    transfer_byte, transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xb1b0;
pub const END: usize = 0xb884;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1724;
pub const DATA_BYTES: usize = 24;
const CALLBACK_COMPARE: u32 = 0x0000_d2a8;
const DIAGNOSTIC_ADDRESS: u32 = 0x000e_37d7;
const COOKIE_TABLE: u32 = 0x0010_3ae8;
const FAILURE_CALLBACK_SLOT: u32 = 0x0010_3ba4;
const DATA_WORDS: [(usize, u32); 6] = [
    (0xb284, CALLBACK_COMPARE),
    (0xb288, DIAGNOSTIC_ADDRESS),
    (0xb874, DIAGNOSTIC_ADDRESS),
    (0xb878, CALLBACK_COMPARE),
    (0xb87c, COOKIE_TABLE),
    (0xb880, FAILURE_CALLBACK_SLOT),
];

fn r(number: u8) -> R {
    R::new(number).expect("A32 register")
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("heap allocation at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !((START..0xb284).contains(&self.at) || (0xb28c..0xb874).contains(&self.at))
            || self.at % 4 != 0
        {
            return Err(format!("heap allocation instruction outside code: {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, expected: usize, value: u32) -> Result<(), String> {
        if !DATA_WORDS.contains(&(expected, value)) {
            return Err(format!("heap allocation invalid data pair: {expected:#x} = {value:#x}"));
        }
        self.check(expected)?;
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }

    fn literal(&mut self, condition: C, destination: R, target: usize) -> Result<(), String> {
        self.emit(transfer_word(condition, true, destination, R::PC,
            target as i32 - self.at as i32 - 8)?)
    }
}

fn allocate_zeroed(c: &mut Code) -> Result<(), String> {
    c.check(0xb1b0)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(7), r(6), Shift::Lsr, 10, false)?)?;
    c.branch(C::Ne, 0xb1d8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(5), 0x400000)?)?;
    c.branch(C::Cc, 0xb1ec, false)?;
    c.emit(multiply_long(C::Al, false, R::R0, r(1), r(6), r(5))?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Ne, 0xb27c, false)?;
    c.branch(C::Al, 0x86670, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Eq, 0xb224, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(8), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.literal(C::Ne, r(1), 0xb284)?;
    c.emit(data_register(C::Ne, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xb224, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb288)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x95)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x500)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x86670, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, R::R0, r(8), 36)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Eq, 0xb248, false)?;
    c.literal(C::Al, r(1), 0xb284)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0xb27c, false)?;
    c.emit(multiply_register(C::Al, r(4), r(5), r(6))?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xb28c, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0x0)?)?;
    c.branch(C::Eq, 0xb278, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.branch(C::Al, 0xb27c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xb284)
}

fn allocate(c: &mut Code) -> Result<(), String> {
    c.check(0xb28c)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x14)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(5), 0x0)?)?;
    c.branch(C::Ne, 0xb2c0, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x1d4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x86670, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Eq, 0xb2f8, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.literal(C::Ne, r(1), 0xb878)?;
    c.emit(data_register(C::Ne, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xb2f8, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0xd5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x500)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(10), 0x0)?)?;
    c.branch(C::Eq, 0xb868, false)?;
    c.branch(C::Al, 0x86670, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, R::R0, r(5), 36)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Eq, 0xb324, false)?;
    c.literal(C::Al, r(1), 0xb878)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0xb868, false)?;
    c.emit(data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0x8)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x10)?)?;
    c.emit(transfer_word(C::Cs, true, R::R0, r(5), 16)?)?;
    c.emit(data_register(C::Cs, Op::Cmp, R::R0, R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0xb868, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(5), 0x36)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(10), 0x17)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(5), 0x234)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), R::R0, 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(10), R::SP, 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x200)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(6), r(1), 0xf)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, R::R0, r(10), 0xf8)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(5), 0x238)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 16)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0x1)?)?;
    c.branch(C::Ne, 0xb3ec, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Eq, 0xb38c, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(5), 32)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Eq, 0xb3ac, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.branch(C::Eq, 0xb83c, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(4))?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Eq, 0xb3d4, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(5), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Eq, 0xb3ec, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Eq, 0xb400, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    c.literal(C::Al, r(1), 0xb87c)?;
    c.emit(load_halfword(C::Al, R::R0, R::R0, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(1), R::R0, Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xb434, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x19)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(5), 560)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.branch(C::Ne, 0xb4cc, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Ne, 0xb464, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x16c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 8)?)?;
    c.emit(load_halfword(C::Al, r(1), R::R0, 0)?)?;
    c.emit(load_halfword(C::Al, r(2), R::R0, 2)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(2), R::R0, 4)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(2), R::R0, 6)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), R::R0, 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(2), 0xf)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0x1)?)?;
    c.branch(C::Ne, 0xb69c, false)?;
    c.emit(load_halfword(C::Al, r(2), R::R0, 8)?)?;
    c.emit(load_halfword(C::Al, R::R0, R::R0, 10)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xff00)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(1), r(7), 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xb6d0, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x970)?)?;
    c.branch(C::Al, 0xb6c8, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xb884, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(9), 0x0)?)?;
    c.branch(C::Eq, 0xb80c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Ne, 0xb510, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x26)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 12)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0xb5dc, false)?;
    c.emit(data_register(C::Al, Op::Add, r(4), r(9), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xb064, true)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(4), 3)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0xf0)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(4), 3)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(9), 4)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(9), 3)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 12)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(5), 560)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.branch(C::Eq, 0xb578, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xaba0, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x10)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x8)?)?;
    c.branch(C::Eq, 0xb5a8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 16)?)?;
    c.literal(C::Al, r(2), 0xb87c)?;
    c.emit(load_halfword(C::Al, r(1), r(1), 0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(1), r(2), r(1), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::R0, 4)?)?;
    c.emit(load_halfword(C::Al, R::R0, r(4), 2)?)?;
    c.emit(load_halfword(C::Al, r(1), r(4), 4)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(1), r(4), 6)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(1), r(4), 8)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(1), r(4), 10)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(1), r(7), 0)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(store_halfword(C::Al, R::R0, r(4), 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf0)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(9), 3)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 4)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(9), 2)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(5), 560)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0xb60c, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xbce4, true)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 8)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 20)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 16)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0xb644, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x4f)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(9), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Hi, false, R::R0, r(5), 24)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 28)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Cc, false, r(10), r(5), 28)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r(9), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::R0, -8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::R0, -4)?)?;
    c.emit(load_halfword(C::Al, R::R0, r(9), 2)?)?;
    c.emit(load_halfword(C::Al, r(1), r(9), 4)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(1), r(9), 6)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 8)?)?;
    c.emit(load_halfword(C::Al, r(1), r(1), 0)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(store_halfword(C::Al, R::R0, r(9), 0)?)?;
    c.branch(C::Al, 0xb814, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0xff00)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 8)?)?;
    c.emit(load_halfword(C::Al, r(1), r(1), 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xb6d0, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x174)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 8)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(5), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xbce4, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x1)?)?;
    c.branch(C::Lt, 0xb80c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 8)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Ne, 0xb714, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x7e)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(10), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Ne, 0xb738, false)?;
    c.branch(C::Al, 0x86644, true)?;
    c.literal(C::Al, r(2), 0xb874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x7f)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(9), r(5), 8)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 3)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0xf)?)?;
    c.branch(C::Ne, 0xb7c4, false)?;
    c.branch(C::Al, 0xb7f0, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r(2), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(1), 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(10), 3)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 3)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0xf)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, r(1), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(9), 3)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 8)?)?;
    c.emit(load_halfword(C::Al, R::R0, r(1), 2)?)?;
    c.emit(load_halfword(C::Al, r(2), r(1), 4)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, r(2), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(2), r(1), 6)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(2), r(1), 8)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(2), r(1), 10)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(2), r(7), 0)?)?;
    c.emit(data_register(C::Al, Op::Eor, R::R0, R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(store_halfword(C::Al, R::R0, r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xbce4, true)?;
    c.emit(transfer_word(C::Al, true, r(9), r(5), 8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(4), r(4), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, R::R0, r(1), 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0xb7e8, false)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(10), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0xb7e8, false)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(10), 3)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(2), r(2), 0xf, true)?)?;
    c.branch(C::Ne, 0xb74c, false)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0xb4e0, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(5), 8)?)?;
    c.branch(C::Al, 0xbce4, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(4), r(4), 0x1, true)?)?;
    c.branch(C::Gt, 0xb6ec, false)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.branch(C::Eq, 0xb828, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(8), r(8), 0x1, true)?)?;
    c.branch(C::Eq, 0xb840, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.branch(C::Eq, 0xb370, false)?;
    c.branch(C::Al, 0xb840, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.branch(C::Ne, 0xb868, false)?;
    c.literal(C::Al, R::R0, 0xb880)?;
    c.emit(transfer_word(C::Al, true, r(2), R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0x0)?)?;
    c.branch(C::Eq, 0xb868, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_with_link(C::Al, r(2))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xb874)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    allocate_zeroed(&mut c)?;
    c.data(0xb284, CALLBACK_COMPARE)?;
    c.data(0xb288, DIAGNOSTIC_ADDRESS)?;
    allocate(&mut c)?;
    c.data(0xb874, DIAGNOSTIC_ADDRESS)?;
    c.data(0xb878, CALLBACK_COMPARE)?;
    c.data(0xb87c, COOKIE_TABLE)?;
    c.data(0xb880, FAILURE_CALLBACK_SLOT)?;
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("heap allocation extent mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_extent_and_protected_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        for (at, value) in [
            (0xb284, CALLBACK_COMPARE), (0xb288, DIAGNOSTIC_ADDRESS),
            (0xb874, DIAGNOSTIC_ADDRESS), (0xb878, CALLBACK_COMPARE),
            (0xb87c, COOKIE_TABLE), (0xb880, FAILURE_CALLBACK_SLOT),
        ] {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0).unwrap()).is_err());
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn data_requires_exact_address_value_pairs() {
        for (at, value) in DATA_WORDS {
            for wrong in [value ^ 1, 0, u32::MAX, CALLBACK_COMPARE,
                DIAGNOSTIC_ADDRESS, COOKIE_TABLE, FAILURE_CALLBACK_SLOT]
                .into_iter().filter(|wrong| *wrong != value)
            {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.data(at, wrong).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(at + 1, value).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
            let mut misplaced = Code { at: at + 4, bytes: Vec::new() };
            assert!(misplaced.data(at, value).is_err());
            assert_eq!(misplaced.at, at + 4);
            assert!(misplaced.bytes.is_empty());
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes, value.to_le_bytes());
        }
    }

    #[test]
    fn register_blx_sites_and_flag_consumers() {
        let bytes = fragment().unwrap();
        for (at, word) in [
            (0xb388, 0xe12f_ff31u32),
            (0xb3a8, 0xe12f_ff31),
            (0xb3bc, 0xe12f_ff34),
            (0xb3d0, 0xe12f_ff31),
            (0xb3e8, 0xe12f_ff31),
            (0xb3fc, 0xe12f_ff31),
            (0xb824, 0xe12f_ff31),
            (0xb860, 0xe12f_ff32),
            (0xb7e0, 0xe212_200f),
            (0xb7e4, 0x1aff_ffd8),
            (0xb804, 0xe254_4001),
            (0xb808, 0xcaff_ffb7),
            (0xb828, 0xe258_8001),
            (0xb82c, 0x0a00_0003),
        ] {
            assert_eq!(&bytes[at - START..at - START + 4], &word.to_le_bytes(),
                "instruction at {at:#x}");
        }
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
