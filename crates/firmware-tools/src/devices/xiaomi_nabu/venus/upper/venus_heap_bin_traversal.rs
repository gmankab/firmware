use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register, load_halfword,
    load_halfword_register_preindexed_writeback, store_halfword, transfer_byte,
    transfer_halfword_register, transfer_multiple, transfer_word,
    transfer_word_preindexed_writeback, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xb884;
pub const END: usize = 0xbe70;
const HEAP_DIAGNOSTIC_SOURCE: u32 = 0x000e37d7;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn instruction(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if at != self.at
            || at % 4 != 0
            || !matches!(at, 0xb884..0xbce0 | 0xbce4..0xbe68 | 0xbe6c..0xbe70)
        {
            return Err(format!("invalid heap traversal instruction address {at:#x}"));
        }
        self.bytes.extend(instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn checked_data(&mut self, at: usize, value: u32) -> Result<(), String> {
        if at != self.at || !matches!(at, 0xbce0 | 0xbe68) || value != HEAP_DIAGNOSTIC_SOURCE {
            return Err(format!("invalid heap traversal data at {at:#x}"));
        }
        self.bytes.extend(value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("constant A32 register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(END - START),
    };
    c.instruction(0xb884, transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0xb888, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.instruction(0xb88c, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x1c)?)?;
    c.instruction(0xb890, data_register(C::Al, Op::Mov, r(7), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb894, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb898, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb89c, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.instruction(0xb8a0, branch(C::Eq, 0xb8a0, 0xb9f0, false)?)?;
    c.instruction(0xb8a4, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb8a8, branch(C::Al, 0xb8a8, 0xaa48, true)?)?;
    c.instruction(0xb8ac, data_immediate(C::Al, Op::Add, r(9), r(5), 0x30)?)?;
    c.instruction(0xb8b0, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.instruction(0xb8b4, transfer_word(C::Al, false, r(4), r(13), 24)?)?;
    c.instruction(0xb8b8, transfer_word(C::Al, false, r(4), r(13), 20)?)?;
    c.instruction(0xb8bc, transfer_word(C::Al, false, r(4), r(13), 16)?)?;
    c.instruction(0xb8c0, data_register(C::Al, Op::Add, r(0), r(9), r(0), Shift::Lsl, 4, false)?)?;
    c.instruction(0xb8c4, data_register(C::Al, Op::Mov, r(10), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb8c8, load_halfword(C::Al, r(6), r(0), 8)?)?;
    c.instruction(0xb8cc, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(10), r(6), Shift::Lsl, 4)?)?;
    c.instruction(0xb8d0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0xb8d4, branch(C::Eq, 0xb8d4, 0xbcd4, false)?)?;
    c.instruction(0xb8d8, data_register(C::Al, Op::Add, r(8), r(5), r(6), Shift::Lsl, 4, false)?)?;
    c.instruction(0xb8dc, data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb8e0, transfer_word_preindexed_writeback(C::Al, true, r(0), r(8), 52)?)?;
    c.instruction(0xb8e4, load_halfword(C::Al, r(1), r(0), 0)?)?;
    c.instruction(0xb8e8, load_halfword(C::Al, r(2), r(0), 2)?)?;
    c.instruction(0xb8ec, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb8f0, load_halfword(C::Al, r(2), r(0), 4)?)?;
    c.instruction(0xb8f4, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb8f8, load_halfword(C::Al, r(2), r(0), 6)?)?;
    c.instruction(0xb8fc, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb900, load_halfword(C::Al, r(2), r(0), 8)?)?;
    c.instruction(0xb904, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb908, load_halfword(C::Al, r(2), r(0), 10)?)?;
    c.instruction(0xb90c, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb910, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.instruction(0xb914, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.instruction(0xb918, data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb91c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x234)?)?;
    c.instruction(0xb920, load_halfword_register_preindexed_writeback(C::Al, r(2), r(3), r(2))?)?;
    c.instruction(0xb924, data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb928, transfer_word(C::Al, false, r(3), r(13), 8)?)?;
    c.instruction(0xb92c, branch(C::Eq, 0xb92c, 0xb94c, false)?)?;
    c.instruction(0xb930, branch(C::Al, 0xb930, 0x86644, true)?)?;
    c.instruction(0xb934, transfer_word(C::Al, true, r(2), r(15), 932)?)?;
    c.instruction(0xb938, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9f)?)?;
    c.instruction(0xb93c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xb940, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.instruction(0xb944, branch(C::Al, 0xb944, 0x17b0, true)?)?;
    c.instruction(0xb948, transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.instruction(0xb94c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x1e)?)?;
    c.instruction(0xb950, branch(C::Hi, 0xb950, 0xb968, false)?)?;
    c.instruction(0xb954, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.instruction(0xb958, data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb95c, data_register(C::Cc, Op::Add, r(0), r(9), r(6), Shift::Lsl, 4, false)?)?;
    c.instruction(0xb960, load_halfword(C::Cc, r(6), r(0), 24)?)?;
    c.instruction(0xb964, data_register(C::Cc, Op::Add, r(10), r(9), r(6), Shift::Lsl, 4, false)?)?;
    c.instruction(0xb968, transfer_word(C::Al, true, r(0), r(10), 0)?)?;
    c.instruction(0xb96c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0xb970, transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.instruction(0xb974, branch(C::Eq, 0xb974, 0xbcd4, false)?)?;
    c.instruction(0xb978, transfer_word(C::Al, false, r(6), r(13), 4)?)?;
    c.instruction(0xb97c, transfer_word(C::Al, true, r(6), r(15), 860)?)?;
    c.instruction(0xb980, data_immediate(C::Al, Op::Mov, r(9), r(0), 0xad)?)?;
    c.instruction(0xb984, transfer_word(C::Al, false, r(10), r(13), 12)?)?;
    c.instruction(0xb988, data_immediate(C::Al, Op::Add, r(10), r(13), 0x10)?)?;
    c.instruction(0xb98c, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb990, data_immediate(C::Al, Op::Add, r(8), r(10), 0x8)?)?;
    c.instruction(0xb994, data_immediate(C::Al, Op::Orr, r(9), r(9), 0x300)?)?;
    c.instruction(0xb998, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.instruction(0xb99c, data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9a0, branch(C::Cs, 0xb9a0, 0xbad0, false)?)?;
    c.instruction(0xb9a4, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.instruction(0xb9a8, data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9ac, branch(C::Hi, 0xb9ac, 0xb9c4, false)?)?;
    c.instruction(0xb9b0, transfer_word(C::Al, true, r(1), r(5), 16)?)?;
    c.instruction(0xb9b4, data_register(C::Al, Op::Mov, r(10), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9b8, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.instruction(0xb9bc, data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9c0, branch(C::Hi, 0xb9c0, 0xb9dc, false)?)?;
    c.instruction(0xb9c4, branch(C::Al, 0xb9c4, 0x86644, true)?)?;
    c.instruction(0xb9c8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xb9cc, data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9d0, data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9d4, branch(C::Al, 0xb9d4, 0x17b0, true)?)?;
    c.instruction(0xb9d8, transfer_word(C::Al, true, r(10), r(8), 0)?)?;
    c.instruction(0xb9dc, data_register(C::Al, Op::Mov, r(8), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.instruction(0xb9e0, transfer_word_preindexed_writeback(C::Al, true, r(4), r(8), 8)?)?;
    c.instruction(0xb9e4, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.instruction(0xb9e8, branch(C::Ne, 0xb9e8, 0xb998, false)?)?;
    c.instruction(0xb9ec, branch(C::Al, 0xb9ec, 0xbac8, false)?)?;
    c.instruction(0xb9f0, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.instruction(0xb9f4, branch(C::Eq, 0xb9f4, 0xbac8, false)?)?;
    c.instruction(0xb9f8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.instruction(0xb9fc, branch(C::Al, 0xb9fc, 0xaa48, true)?)?;
    c.instruction(0xba00, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.instruction(0xba04, data_register(C::Al, Op::Add, r(3), r(5), r(0), Shift::Lsl, 4, false)?)?;
    c.instruction(0xba08, data_immediate(C::Al, Op::Add, r(8), r(13), 0x10)?)?;
    c.instruction(0xba0c, transfer_word(C::Al, false, r(2), r(13), 20)?)?;
    c.instruction(0xba10, transfer_word(C::Al, false, r(2), r(13), 16)?)?;
    c.instruction(0xba14, data_register(C::Al, Op::Mov, r(9), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba18, data_immediate(C::Al, Op::Add, r(7), r(8), 0x8)?)?;
    c.instruction(0xba1c, transfer_word_preindexed_writeback(C::Al, true, r(1), r(9), 48)?)?;
    c.instruction(0xba20, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.instruction(0xba24, transfer_word(C::Al, false, r(3), r(13), 12)?)?;
    c.instruction(0xba28, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0xba2c, transfer_word(C::Al, false, r(1), r(13), 24)?)?;
    c.instruction(0xba30, data_immediate(C::Eq, Op::Mov, r(2), r(0), 0x1)?)?;
    c.instruction(0xba34, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0xba38, data_register(C::Ne, Op::Cmp, r(0), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba3c, branch(C::Eq, 0xba3c, 0xbaa0, false)?)?;
    c.instruction(0xba40, transfer_word(C::Al, true, r(6), r(15), 664)?)?;
    c.instruction(0xba44, data_immediate(C::Al, Op::Mov, r(10), r(0), 0xe2)?)?;
    c.instruction(0xba48, data_immediate(C::Al, Op::Orr, r(10), r(10), 0x300)?)?;
    c.instruction(0xba4c, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.instruction(0xba50, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba54, branch(C::Hi, 0xba54, 0xba6c, false)?)?;
    c.instruction(0xba58, transfer_word(C::Al, true, r(2), r(5), 16)?)?;
    c.instruction(0xba5c, data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba60, data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 3, false)?)?;
    c.instruction(0xba64, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba68, branch(C::Hi, 0xba68, 0xba84, false)?)?;
    c.instruction(0xba6c, branch(C::Al, 0xba6c, 0x86644, true)?)?;
    c.instruction(0xba70, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xba74, data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba78, data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba7c, branch(C::Al, 0xba7c, 0x17b0, true)?)?;
    c.instruction(0xba80, transfer_word(C::Al, true, r(8), r(7), 0)?)?;
    c.instruction(0xba84, data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba88, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.instruction(0xba8c, transfer_word_preindexed_writeback(C::Al, true, r(1), r(7), 8)?)?;
    c.instruction(0xba90, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0xba94, data_immediate(C::Eq, Op::Mov, r(2), r(0), 0x1)?)?;
    c.instruction(0xba98, data_register(C::Ne, Op::Cmp, r(0), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xba9c, branch(C::Ne, 0xba9c, 0xba4c, false)?)?;
    c.instruction(0xbaa0, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.instruction(0xbaa4, branch(C::Eq, 0xbaa4, 0xbbc8, false)?)?;
    c.instruction(0xbaa8, branch(C::Al, 0xbaa8, 0x86644, true)?)?;
    c.instruction(0xbaac, transfer_word(C::Al, true, r(2), r(15), 556)?)?;
    c.instruction(0xbab0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5)?)?;
    c.instruction(0xbab4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbab8, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.instruction(0xbabc, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbac0, branch(C::Al, 0xbac0, 0x17b0, true)?)?;
    c.instruction(0xbac4, branch(C::Al, 0xbac4, 0xbcd4, false)?)?;
    c.instruction(0xbac8, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.instruction(0xbacc, branch(C::Al, 0xbacc, 0xbcd4, false)?)?;
    c.instruction(0xbad0, load_halfword(C::Al, r(1), r(4), 0)?)?;
    c.instruction(0xbad4, load_halfword(C::Al, r(2), r(4), 2)?)?;
    c.instruction(0xbad8, transfer_word(C::Al, true, r(6), r(13), 8)?)?;
    c.instruction(0xbadc, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbae0, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbae4, load_halfword(C::Al, r(1), r(4), 6)?)?;
    c.instruction(0xbae8, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbaec, load_halfword(C::Al, r(1), r(4), 8)?)?;
    c.instruction(0xbaf0, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbaf4, load_halfword(C::Al, r(1), r(4), 10)?)?;
    c.instruction(0xbaf8, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbafc, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.instruction(0xbb00, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xff00)?)?;
    c.instruction(0xbb04, data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb08, load_halfword(C::Al, r(1), r(6), 0)?)?;
    c.instruction(0xbb0c, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb10, branch(C::Eq, 0xbb10, 0xbb2c, false)?)?;
    c.instruction(0xbb14, branch(C::Al, 0xbb14, 0x86644, true)?)?;
    c.instruction(0xbb18, transfer_word(C::Al, true, r(2), r(15), 448)?)?;
    c.instruction(0xbb1c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xb6)?)?;
    c.instruction(0xbb20, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbb24, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.instruction(0xbb28, branch(C::Al, 0xbb28, 0x17b0, true)?)?;
    c.instruction(0xbb2c, transfer_word(C::Al, true, r(1), r(13), 12)?)?;
    c.instruction(0xbb30, transfer_word(C::Al, true, r(0), r(1), 0)?)?;
    c.instruction(0xbb34, data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb38, transfer_word(C::Eq, true, r(0), r(8), 0)?)?;
    c.instruction(0xbb3c, transfer_word(C::Eq, true, r(0), r(0), 8)?)?;
    c.instruction(0xbb40, transfer_word(C::Eq, false, r(0), r(1), 0)?)?;
    c.instruction(0xbb44, transfer_word(C::Al, true, r(0), r(1), 4)?)?;
    c.instruction(0xbb48, data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb4c, branch(C::Ne, 0xbb4c, 0xbb64, false)?)?;
    c.instruction(0xbb50, transfer_word(C::Al, true, r(0), r(10), 4)?)?;
    c.instruction(0xbb54, transfer_word(C::Al, true, r(1), r(13), 12)?)?;
    c.instruction(0xbb58, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0xbb5c, data_register(C::Ne, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb60, transfer_word(C::Al, false, r(0), r(1), 4)?)?;
    c.instruction(0xbb64, transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.instruction(0xbb68, transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.instruction(0xbb6c, transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.instruction(0xbb70, data_immediate(C::Al, Op::Add, r(0), r(13), 0x10)?)?;
    c.instruction(0xbb74, data_register(C::Al, Op::Cmp, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb78, branch(C::Eq, 0xbb78, 0xbbac, false)?)?;
    c.instruction(0xbb7c, load_halfword(C::Al, r(0), r(10), 2)?)?;
    c.instruction(0xbb80, load_halfword(C::Al, r(1), r(10), 4)?)?;
    c.instruction(0xbb84, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb88, load_halfword(C::Al, r(1), r(10), 6)?)?;
    c.instruction(0xbb8c, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb90, load_halfword(C::Al, r(1), r(10), 8)?)?;
    c.instruction(0xbb94, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbb98, load_halfword(C::Al, r(1), r(10), 10)?)?;
    c.instruction(0xbb9c, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbba0, load_halfword(C::Al, r(1), r(6), 0)?)?;
    c.instruction(0xbba4, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbba8, store_halfword(C::Al, r(0), r(10), 0)?)?;
    c.instruction(0xbbac, transfer_word(C::Al, true, r(1), r(13), 12)?)?;
    c.instruction(0xbbb0, transfer_word(C::Al, true, r(0), r(1), 12)?)?;
    c.instruction(0xbbb4, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 0x1, true)?)?;
    c.instruction(0xbbb8, transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.instruction(0xbbbc, branch(C::Ne, 0xbbbc, 0xbccc, false)?)?;
    c.instruction(0xbbc0, transfer_word(C::Al, true, r(1), r(13), 4)?)?;
    c.instruction(0xbbc4, branch(C::Al, 0xbbc4, 0xbcc4, false)?)?;
    c.instruction(0xbbc8, transfer_word(C::Al, true, r(1), r(7), 0)?)?;
    c.instruction(0xbbcc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.instruction(0xbbd0, data_register(C::Al, Op::Mov, r(6), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbbd4, load_halfword_register_preindexed_writeback(C::Al, r(0), r(6), r(0))?)?;
    c.instruction(0xbbd8, load_halfword(C::Al, r(2), r(1), 0)?)?;
    c.instruction(0xbbdc, load_halfword(C::Al, r(3), r(1), 2)?)?;
    c.instruction(0xbbe0, data_register(C::Al, Op::Eor, r(2), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbbe4, load_halfword(C::Al, r(3), r(1), 4)?)?;
    c.instruction(0xbbe8, data_register(C::Al, Op::Eor, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbbec, load_halfword(C::Al, r(3), r(1), 6)?)?;
    c.instruction(0xbbf0, data_register(C::Al, Op::Eor, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbbf4, load_halfword(C::Al, r(3), r(1), 8)?)?;
    c.instruction(0xbbf8, load_halfword(C::Al, r(1), r(1), 10)?)?;
    c.instruction(0xbbfc, data_register(C::Al, Op::Eor, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc00, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc04, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.instruction(0xbc08, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.instruction(0xbc0c, data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc10, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc14, branch(C::Eq, 0xbc14, 0xbc30, false)?)?;
    c.instruction(0xbc18, branch(C::Al, 0xbc18, 0x86644, true)?)?;
    c.instruction(0xbc1c, transfer_word(C::Al, true, r(2), r(15), 188)?)?;
    c.instruction(0xbc20, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe9)?)?;
    c.instruction(0xbc24, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbc28, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.instruction(0xbc2c, branch(C::Al, 0xbc2c, 0x17b0, true)?)?;
    c.instruction(0xbc30, transfer_word(C::Al, true, r(0), r(9), 0)?)?;
    c.instruction(0xbc34, transfer_word(C::Al, true, r(2), r(13), 12)?)?;
    c.instruction(0xbc38, data_register(C::Al, Op::Cmp, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc3c, transfer_word(C::Eq, true, r(0), r(7), 0)?)?;
    c.instruction(0xbc40, transfer_word(C::Eq, true, r(0), r(0), 8)?)?;
    c.instruction(0xbc44, transfer_word(C::Eq, false, r(0), r(9), 0)?)?;
    c.instruction(0xbc48, data_register(C::Al, Op::Mov, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc4c, transfer_word_preindexed_writeback(C::Al, true, r(1), r(0), 52)?)?;
    c.instruction(0xbc50, data_register(C::Al, Op::Cmp, r(0), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc54, branch(C::Ne, 0xbc54, 0xbc68, false)?)?;
    c.instruction(0xbc58, transfer_word(C::Al, true, r(1), r(8), 4)?)?;
    c.instruction(0xbc5c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0xbc60, data_register(C::Ne, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc64, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.instruction(0xbc68, transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.instruction(0xbc6c, transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.instruction(0xbc70, transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.instruction(0xbc74, data_immediate(C::Al, Op::Add, r(0), r(13), 0x10)?)?;
    c.instruction(0xbc78, data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc7c, branch(C::Eq, 0xbc7c, 0xbcb0, false)?)?;
    c.instruction(0xbc80, load_halfword(C::Al, r(0), r(8), 2)?)?;
    c.instruction(0xbc84, load_halfword(C::Al, r(1), r(8), 4)?)?;
    c.instruction(0xbc88, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc8c, load_halfword(C::Al, r(1), r(8), 6)?)?;
    c.instruction(0xbc90, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc94, load_halfword(C::Al, r(1), r(8), 8)?)?;
    c.instruction(0xbc98, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbc9c, load_halfword(C::Al, r(1), r(8), 10)?)?;
    c.instruction(0xbca0, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbca4, load_halfword(C::Al, r(1), r(6), 0)?)?;
    c.instruction(0xbca8, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbcac, store_halfword(C::Al, r(0), r(8), 0)?)?;
    c.instruction(0xbcb0, transfer_word(C::Al, true, r(0), r(2), 60)?)?;
    c.instruction(0xbcb4, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 0x1, true)?)?;
    c.instruction(0xbcb8, transfer_word(C::Al, false, r(0), r(2), 60)?)?;
    c.instruction(0xbcbc, branch(C::Ne, 0xbcbc, 0xbccc, false)?)?;
    c.instruction(0xbcc0, transfer_word(C::Al, true, r(1), r(13), 8)?)?;
    c.instruction(0xbcc4, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbcc8, branch(C::Al, 0xbcc8, 0xd53c, true)?)?;
    c.instruction(0xbccc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbcd0, transfer_word(C::Al, false, r(0), r(4), 8)?)?;
    c.instruction(0xbcd4, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbcd8, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.instruction(0xbcdc, transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.checked_data(0xbce0, HEAP_DIAGNOSTIC_SOURCE)?;
    c.instruction(0xbce4, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0xbce8, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.instruction(0xbcec, data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbcf0, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbcf4, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.instruction(0xbcf8, branch(C::Ne, 0xbcf8, 0xbd14, false)?)?;
    c.instruction(0xbcfc, branch(C::Al, 0xbcfc, 0x86644, true)?)?;
    c.instruction(0xbd00, transfer_word(C::Al, true, r(2), r(15), 352)?)?;
    c.instruction(0xbd04, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x63)?)?;
    c.instruction(0xbd08, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbd0c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbd10, branch(C::Al, 0xbd10, 0x17b0, true)?)?;
    c.instruction(0xbd14, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.instruction(0xbd18, branch(C::Ne, 0xbd18, 0xbd34, false)?)?;
    c.instruction(0xbd1c, branch(C::Al, 0xbd1c, 0x86644, true)?)?;
    c.instruction(0xbd20, transfer_word(C::Al, true, r(2), r(15), 320)?)?;
    c.instruction(0xbd24, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x64)?)?;
    c.instruction(0xbd28, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbd2c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbd30, branch(C::Al, 0xbd30, 0x17b0, true)?)?;
    c.instruction(0xbd34, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.instruction(0xbd38, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0xbd3c, branch(C::Ne, 0xbd3c, 0xbd5c, false)?)?;
    c.instruction(0xbd40, branch(C::Al, 0xbd40, 0x86644, true)?)?;
    c.instruction(0xbd44, transfer_word(C::Al, true, r(2), r(15), 284)?)?;
    c.instruction(0xbd48, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x65)?)?;
    c.instruction(0xbd4c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbd50, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbd54, branch(C::Al, 0xbd54, 0x17b0, true)?)?;
    c.instruction(0xbd58, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.instruction(0xbd5c, transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.instruction(0xbd60, transfer_word(C::Al, true, r(3), r(4), 16)?)?;
    c.instruction(0xbd64, data_register(C::Al, Op::Add, r(1), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbd68, data_register(C::Al, Op::Add, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbd6c, data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbd70, branch(C::Ls, 0xbd70, 0xbd90, false)?)?;
    c.instruction(0xbd74, branch(C::Al, 0xbd74, 0x86644, true)?)?;
    c.instruction(0xbd78, transfer_word(C::Al, true, r(2), r(15), 232)?)?;
    c.instruction(0xbd7c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x66)?)?;
    c.instruction(0xbd80, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbd84, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbd88, branch(C::Al, 0xbd88, 0x17b0, true)?)?;
    c.instruction(0xbd8c, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.instruction(0xbd90, data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.instruction(0xbd94, branch(C::Eq, 0xbd94, 0xbdb0, false)?)?;
    c.instruction(0xbd98, branch(C::Al, 0xbd98, 0x86644, true)?)?;
    c.instruction(0xbd9c, transfer_word(C::Al, true, r(2), r(15), 196)?)?;
    c.instruction(0xbda0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x67)?)?;
    c.instruction(0xbda4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbda8, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbdac, branch(C::Al, 0xbdac, 0x17b0, true)?)?;
    c.instruction(0xbdb0, transfer_byte(C::Al, true, r(0), r(5), 3)?)?;
    c.instruction(0xbdb4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x10)?)?;
    c.instruction(0xbdb8, transfer_word(C::Cc, true, r(0), r(5), 4)?)?;
    c.instruction(0xbdbc, data_register(C::Cc, Op::Add, r(5), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbdc0, transfer_word(C::Cs, true, r(5), r(4), 4)?)?;
    c.instruction(0xbdc4, load_halfword(C::Al, r(0), r(5), 0)?)?;
    c.instruction(0xbdc8, load_halfword(C::Al, r(1), r(5), 2)?)?;
    c.instruction(0xbdcc, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbdd0, load_halfword(C::Al, r(1), r(5), 4)?)?;
    c.instruction(0xbdd4, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbdd8, load_halfword(C::Al, r(1), r(5), 6)?)?;
    c.instruction(0xbddc, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbde0, transfer_byte(C::Al, true, r(1), r(5), 3)?)?;
    c.instruction(0xbde4, data_immediate(C::Al, Op::And, r(1), r(1), 0xf)?)?;
    c.instruction(0xbde8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.instruction(0xbdec, branch(C::Ne, 0xbdec, 0xbe28, false)?)?;
    c.instruction(0xbdf0, load_halfword(C::Al, r(2), r(5), 8)?)?;
    c.instruction(0xbdf4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x234)?)?;
    c.instruction(0xbdf8, transfer_halfword_register(C::Al, true, r(1), r(4), r(1))?)?;
    c.instruction(0xbdfc, data_register(C::Al, Op::Eor, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe00, load_halfword(C::Al, r(2), r(5), 10)?)?;
    c.instruction(0xbe04, data_register(C::Al, Op::Eor, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe08, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.instruction(0xbe0c, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.instruction(0xbe10, data_register(C::Al, Op::And, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe14, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe18, branch(C::Eq, 0xbe18, 0xbe60, false)?)?;
    c.instruction(0xbe1c, branch(C::Al, 0xbe1c, 0x86644, true)?)?;
    c.instruction(0xbe20, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6e)?)?;
    c.instruction(0xbe24, branch(C::Al, 0xbe24, 0xbe50, false)?)?;
    c.instruction(0xbe28, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x36)?)?;
    c.instruction(0xbe2c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.instruction(0xbe30, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.instruction(0xbe34, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.instruction(0xbe38, transfer_halfword_register(C::Al, true, r(1), r(4), r(1))?)?;
    c.instruction(0xbe3c, data_register(C::Al, Op::And, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe40, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe44, branch(C::Eq, 0xbe44, 0xbe60, false)?)?;
    c.instruction(0xbe48, branch(C::Al, 0xbe48, 0x86644, true)?)?;
    c.instruction(0xbe4c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x71)?)?;
    c.instruction(0xbe50, transfer_word(C::Al, true, r(2), r(15), 16)?)?;
    c.instruction(0xbe54, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.instruction(0xbe58, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0xbe5c, branch(C::Al, 0xbe5c, 0x17b0, true)?)?;
    c.instruction(0xbe60, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0xbe64, transfer_multiple(C::Al, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.checked_data(0xbe68, HEAP_DIAGNOSTIC_SOURCE)?;
    c.instruction(0xbe6c, bx(C::Al, r(14)))?;

    if c.at != END || c.bytes.len() != END - START {
        return Err("invalid heap traversal extent".into());
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
    fn exact_extent_and_protected_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1516);
        assert_eq!(bytes.len() - 8, 1508);
        for at in [0xbce0, 0xbe68] {
            assert_eq!(word(&bytes, at), HEAP_DIAGNOSTIC_SOURCE);
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.instruction(at, bx(C::Al, r(14))).is_err());
            assert!(c.checked_data(at, HEAP_DIAGNOSTIC_SOURCE ^ 1).is_err());
            c.checked_data(at, HEAP_DIAGNOSTIC_SOURCE).unwrap();
            assert!(c.checked_data(at, HEAP_DIAGNOSTIC_SOURCE).is_err());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.instruction(START + 4, bx(C::Al, r(14))).is_err());
        assert!(c.checked_data(START, HEAP_DIAGNOSTIC_SOURCE).is_err());
        let mut c = Code { at: END, bytes: Vec::new() };
        assert!(c.instruction(END, bx(C::Al, r(14))).is_err());
    }

    #[test]
    fn writeback_forward_call_and_standalone_return() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0xb8cc), transfer_word_register_preindexed_writeback(
            C::Al, true, r(0), r(10), r(6), Shift::Lsl, 4).unwrap());
        assert_eq!(word(&bytes, 0xb920), load_halfword_register_preindexed_writeback(
            C::Al, r(2), r(3), r(2)).unwrap());
        assert_eq!(word(&bytes, 0xbcc8), branch(C::Al, 0xbcc8, 0xd53c, true).unwrap());
        assert_eq!(word(&bytes, 0xbe6c), bx(C::Al, r(14)));
    }

    #[test]
    fn optional_canonical_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(reference.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), identity.sha256);
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }
}
