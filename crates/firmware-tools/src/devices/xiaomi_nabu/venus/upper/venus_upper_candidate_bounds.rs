use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register, multiply_accumulate, signed_multiply_accumulate_bottom_halves, multiply_long, multiply_register, transfer_byte, transfer_doubleword, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6d548;
pub const END: usize = 0x6d998;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, word: u32) -> Result<(), String> {
        if self.at < START || self.at >= END || self.at % 4 != 0 {
            return Err(format!("invalid candidate bounds A32 address 0x{:x}", self.at));
        }
        self.bytes.extend(word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("constant A32 register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(data_register(C::Al, Op::Add, r(1), r(12), r(5), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(1), 56)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 37)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 2)?)?;
    c.branch(C::Eq, 0x6d590, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.branch(C::Eq, 0x6d5cc, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Ne, 0x6d600, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 728)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0x6d620, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x6d7cc, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.branch(C::Ne, 0x6d7cc, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6d7cc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 704)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 708)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 728)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(1), r(1), 1, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(3), 0, true)?)?;
    c.branch(C::Lt, 0x6d63c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.branch(C::Eq, 0x6d770, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 64)?)?;
    c.branch(C::Cc, 0x6d790, false)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6d7cc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 728)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0x6d64c, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.branch(C::Hi, 0x6d7cc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 61)?)?;
    c.branch(C::Cc, 0x6d7c4, false)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 1, false)?)?;
    c.branch(C::Al, 0x6d7cc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 728)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Ne, 0x6d7cc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 96)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 61)?)?;
    c.branch(C::Cc, 0x6d79c, false)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 1, false)?)?;
    c.branch(C::Al, 0x6d7f0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x6d7cc, false)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 61)?)?;
    c.branch(C::Cc, 0x6d7ac, false)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 137)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0x6d684, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 4096)?)?;
    c.branch(C::Ne, 0x6d998, false)?;
    c.emit(multiply_long(C::Al, false, r(0), r(6), r(3), r(2))?)?;
    c.emit(multiply_accumulate(C::Al, r(5), r(3), r(1), r(6))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(7), r(2), r(5))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(9), 728)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0)?)?;
    c.branch(C::Ne, 0x6e098, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 64)?)?;
    c.branch(C::Cc, 0x6e098, false)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 37)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 2)?)?;
    c.branch(C::Cc, 0x6d6d4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 137)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x6d6e0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 132)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 4)?)?;
    c.branch(C::Cc, 0x6e098, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0)?)?;
    c.emit(transfer_multiple(C::Al, false, r(0), r(1).bit() | r(2).bit() | r(3).bit(), BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Eq, 0x6d94c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 76)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 721)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 232)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 948)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 1024)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 72)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(9), 722)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 723)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(3), r(4), r(3), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 520)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(3), 13376)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 1024)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(1), r(5), r(3))?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(3), r(4), r(5), r(3))?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 11)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 13)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(3), 4)?)?;
    c.branch(C::Al, 0x6dbb0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.branch(C::Ne, 0x6d844, false)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 11)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 56)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(0), 200)?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(5), r(2), r(12))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 10)?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(3), r(12), r(5))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.branch(C::Al, 0x6d884, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 64)?)?;
    c.branch(C::Cc, 0x6d908, false)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(C::Ls, 0x6d7f4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6d7cc, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Ls, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 41)?)?;
    c.branch(C::Cs, 0x6d920, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 772)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 92)?)?;
    c.emit(multiply_long(C::Al, false, r(2), r(3), r(1), r(0))?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(6), r(2), r(7))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 2304)?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(3), r(7), r(6))?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 3997696)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 1888)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1892)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 7)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(4), Shift::Lsr, 3, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(7), r(0), r(4), Shift::Lsr, 3, false)?)?;
    c.emit(multiply_long(C::Al, false, r(3), r(6), r(0), r(2))?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(6), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsr, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(1), r(0), Shift::Lsl, 29, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6e300, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(2), 2104)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(2), 2696)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 848)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(2), 2700)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 852)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(2), 2704)?)?;
    c.branch(C::Al, 0x6c8d4, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 116)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.branch(C::Eq, 0x6db94, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(8), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 56)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 8)?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(1), r(3), r(2))?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 672)?)?;
    c.emit(multiply_long(C::Al, false, r(12), r(5), r(0), r(2))?)?;
    c.emit(multiply_accumulate(C::Al, r(3), r(1), r(2), r(5))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(12), Shift::Lsr, 6, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(3), Shift::Lsl, 26, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsr, 6, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 56)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(0), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Al, Op::Sbc, r(2), r(1), r(7), Shift::Lsl, 0, true)?)?;
    c.branch(C::Cs, 0x6d8ac, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 116)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, r(2), r(3), 152)?)?;
    c.emit(data_register(C::Ne, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Ne, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_doubleword(C::Ne, false, r(0), r(2), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0x6cce8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 1912)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0x6d8d0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(9), 1912)?)?;
    c.branch(C::Al, 0x6cce8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 72)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 37)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0x6d9f4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Hi, 0x6da1c, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(8), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 2)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 56)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(multiply_register(C::Al, r(1), r(0), r(2))?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(6), Shift::Lsl, 0, true)?)?;
    c.branch(C::Al, 0x6da38, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 40)?)?;
    c.branch(C::Cc, 0x6d920, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(4), Shift::Lsl, 2, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(4), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6d7f4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 37)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Hi, 0x6da58, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.emit(multiply_register(C::Al, r(0), r(2), r(1))?)?;
    c.branch(C::Al, 0x6da64, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 644)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(9), 721)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(0), 2048)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 100)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 72)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x6dba0, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 3350)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 44)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 45)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 2704)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 2703)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(0), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Gt, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6dbac, false)?;
    if c.at != END {
        return Err(format!("candidate bounds ended at 0x{:x}", c.at));
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn exact_code_extent_and_absolute_edges() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1104);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(word(&bytes, 0x6d840), branch(C::Al, 0x6d840, 0x6c8d4, false).unwrap());
        let mut calls = 0;
        let mut earlier_exit = false;
        let mut forward_a = false;
        let mut forward_b = false;
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 != 0x0a00_0000 { continue; }
            let offset = ((instruction << 8) as i32 >> 8) * 4;
            let target = (at as i32 + 8 + offset) as usize;
            if instruction & 0x0100_0000 != 0 {
                assert_eq!(target, 0x8187c);
                calls += 1;
            }
            earlier_exit |= target == 0x6cce8;
            forward_a |= target == 0x6e098;
            forward_b |= target == 0x6e300;
        }
        assert_eq!(calls, 4);
        assert!(earlier_exit && forward_a && forward_b);
    }

    #[test]
    fn optional_pinned_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }
}
