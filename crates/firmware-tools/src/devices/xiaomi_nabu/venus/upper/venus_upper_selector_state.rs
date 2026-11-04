use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags,
    data_register, data_register_shifted_by_register, load_halfword,
    load_halfword_register_preindexed_writeback, mov_register,
    signed_multiply_accumulate_bottom_halves, store_halfword, transfer_byte,
    transfer_word, transfer_word_register_preindexed_writeback,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6c964;
pub const END: usize = 0x6ccb4;
const CODE_BYTES: usize = 828;
const DATA_BYTES: usize = 20;
const SIGNED_OFFSET: u32 = (-4608_i32) as u32;
const ARITHMETIC_MAGIC: u32 = 0x431bde83;
const DIAGNOSTIC: u32 = 0x000f2f51;
const NEWLINE: u32 = 0x000e2788;
const DIVIDE_CONSTANT: u32 = 0xcccccccd;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

fn island(at: usize) -> Option<u32> {
    match at {
        0x6ca2c => Some(SIGNED_OFFSET),
        0x6cb1c => Some(ARITHMETIC_MAGIC),
        0x6cb3c => Some(DIAGNOSTIC),
        0x6cb90 => Some(NEWLINE),
        0x6cc20 => Some(DIVIDE_CONSTANT),
        _ => None,
    }
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
    code_bytes: usize,
    data_bytes: usize,
}

impl Code {
    fn append(&mut self, at: usize, word: u32) -> Result<(), String> {
        if at != self.at || !(START..END).contains(&at) || at % 4 != 0 {
            return Err(format!("selector state address {:#x}, requested {at:#x}", self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if island(at).is_some() {
            return Err(format!("selector state instruction at literal {at:#x}"));
        }
        self.append(at, instruction)?;
        self.code_bytes += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        if island(at) != Some(value) {
            return Err(format!("selector state invalid literal at {at:#x}"));
        }
        self.append(at, value)?;
        self.data_bytes += 4;
        Ok(())
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(END - START),
        code_bytes: 0,
        data_bytes: 0,
    };
    c.emit(0x6c964, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x268)?)?;
    c.emit(0x6c968, mov_register(C::Al, r(0), r(9)))?;
    c.emit(0x6c96c, transfer_word(C::Al, true, r(6), r(11), 0x8)?)?;
    c.emit(0x6c970, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6c974, transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(0x6c978, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6c97c, branch(C::Ne, 0x6c97c, 0x6c9d4, false)?)?;
    c.emit(0x6c980, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x270)?)?;
    c.emit(0x6c984, mov_register(C::Al, r(1), r(9)))?;
    c.emit(0x6c988, transfer_word(C::Al, true, r(3), r(9), 0x2c0)?)?;
    c.emit(0x6c98c, transfer_word(C::Al, true, r(7), r(9), 0x2c4)?)?;
    c.emit(0x6c990, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(0x6c994, transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x6c998, data_register(C::Al, Op::Sub, r(3), r(3), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c99c, transfer_word(C::Al, false, r(3), r(9), 0x2c0)?)?;
    c.emit(0x6c9a0, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x26c)?)?;
    c.emit(0x6c9a4, data_register(C::Al, Op::Sbc, r(2), r(7), r(2), Shift::Asr, 31, false)?)?;
    c.emit(0x6c9a8, transfer_word(C::Al, false, r(2), r(9), 0x2c4)?)?;
    c.emit(0x6c9ac, data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.emit(0x6c9b0, mov_register(C::Al, r(2), r(9)))?;
    c.emit(0x6c9b4, transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(2), r(3), Shift::Lsl, 0)?)?;
    c.emit(0x6c9b8, data_immediate_with_flags(C::Al, Op::Sub, r(3), r(3), 0x1, true)?)?;
    c.emit(0x6c9bc, transfer_word(C::Al, false, r(3), r(2), 0x0)?)?;
    c.emit(0x6c9c0, branch(C::Ne, 0x6c9c0, 0x6c9d4, false)?)?;
    c.emit(0x6c9c4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x6c9c8, transfer_word(C::Al, false, r(3), r(0), 0x0)?)?;
    c.emit(0x6c9cc, transfer_word(C::Al, false, r(3), r(2), 0x0)?)?;
    c.emit(0x6c9d0, transfer_word(C::Al, false, r(3), r(1), 0x0)?)?;
    c.emit(0x6c9d4, transfer_word(C::Al, true, r(1), r(13), 0x40)?)?;
    c.emit(0x6c9d8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6c9dc, transfer_word(C::Al, false, r(0), r(1), 0x0)?)?;
    c.emit(0x6c9e0, transfer_byte(C::Al, true, r(0), r(6), 0x33c)?)?;
    c.emit(0x6c9e4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6c9e8, branch(C::Eq, 0x6c9e8, 0x6ca08, false)?)?;
    c.emit(0x6c9ec, transfer_word(C::Al, true, r(0), r(6), 0x2c)?)?;
    c.emit(0x6c9f0, transfer_word(C::Al, true, r(1), r(6), 0x304)?)?;
    c.emit(0x6c9f4, transfer_word(C::Al, true, r(0), r(0), 0x24)?)?;
    c.emit(0x6c9f8, transfer_word(C::Al, true, r(3), r(0), 0x6d4)?)?;
    c.emit(0x6c9fc, transfer_word(C::Al, true, r(2), r(0), 0x5c)?)?;
    c.emit(0x6ca00, mov_register(C::Al, r(0), r(9)))?;
    c.emit(0x6ca04, branch(C::Al, 0x6ca04, 0x6f61c, true)?)?;
    c.emit(0x6ca08, transfer_word(C::Al, true, r(0), r(13), 0x2c)?)?;
    c.emit(0x6ca0c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6ca10, branch(C::Eq, 0x6ca10, 0x6ca30, false)?)?;
    c.emit(0x6ca14, transfer_word(C::Al, true, r(1), r(9), 0x5c)?)?;
    c.emit(0x6ca18, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x900)?)?;
    c.emit(0x6ca1c, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x3d0000)?)?;
    c.emit(0x6ca20, branch_exchange_immediate(0x6ca20, 0x83c4c)?)?;
    c.emit(0x6ca24, transfer_word(C::Al, false, r(0), r(9), 0x304)?)?;
    c.emit(0x6ca28, branch(C::Al, 0x6ca28, 0x6ca3c, false)?)?;
    c.literal(0x6ca2c, SIGNED_OFFSET)?;
    c.emit(0x6ca30, transfer_word(C::Al, true, r(0), r(10), 0x128)?)?;
    c.emit(0x6ca34, transfer_word(C::Al, false, r(0), r(9), 0x304)?)?;
    c.emit(0x6ca38, transfer_word(C::Al, false, r(0), r(9), 0x308)?)?;
    c.emit(0x6ca3c, transfer_byte(C::Al, true, r(0), r(10), 0xa)?)?;
    c.emit(0x6ca40, transfer_byte(C::Al, false, r(0), r(9), 0x2d2)?)?;
    c.emit(0x6ca44, transfer_byte(C::Al, true, r(1), r(10), 0xb)?)?;
    c.emit(0x6ca48, transfer_byte(C::Al, false, r(1), r(9), 0x2d3)?)?;
    c.emit(0x6ca4c, transfer_word(C::Al, true, r(1), r(9), 0x2d8)?)?;
    c.emit(0x6ca50, transfer_word(C::Al, false, r(1), r(9), 0x2dc)?)?;
    c.emit(0x6ca54, transfer_word(C::Al, true, r(2), r(10), 0x20)?)?;
    c.emit(0x6ca58, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x2)?)?;
    c.emit(0x6ca5c, transfer_word(C::Ls, false, r(2), r(9), 0x2d8)?)?;
    c.emit(0x6ca60, mov_register(C::Ls, r(1), r(2)))?;
    c.emit(0x6ca64, transfer_word(C::Al, true, r(2), r(9), 0xc)?)?;
    c.emit(0x6ca68, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x6ca6c, branch(C::Eq, 0x6ca6c, 0x6cab0, false)?)?;
    c.emit(0x6ca70, transfer_word(C::Al, true, r(2), r(13), 0x4c)?)?;
    c.emit(0x6ca74, transfer_word(C::Al, true, r(3), r(9), 0x0)?)?;
    c.emit(0x6ca78, transfer_word(C::Al, true, r(2), r(2), 0x0)?)?;
    c.emit(0x6ca7c, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x2000)?)?;
    c.emit(0x6ca80, branch(C::Ne, 0x6ca80, 0x6cad8, false)?)?;
    c.emit(0x6ca84, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6ca88, branch(C::Ne, 0x6ca88, 0x6cc30, false)?)?;
    c.emit(0x6ca8c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.emit(0x6ca90, branch(C::Eq, 0x6ca90, 0x6cc18, false)?)?;
    c.emit(0x6ca94, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6ca98, branch(C::Eq, 0x6ca98, 0x6cc24, false)?)?;
    c.emit(0x6ca9c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6caa0, branch(C::Ne, 0x6caa0, 0x6cc30, false)?)?;
    c.emit(0x6caa4, data_immediate(C::Al, Op::Add, r(1), r(2), 0x20c)?)?;
    c.emit(0x6caa8, data_immediate(C::Al, Op::Add, r(1), r(1), 0x400)?)?;
    c.emit(0x6caac, branch(C::Al, 0x6caac, 0x6cc2c, false)?)?;
    c.emit(0x6cab0, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.emit(0x6cab4, branch(C::Eq, 0x6cab4, 0x6cb40, false)?)?;
    c.emit(0x6cab8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6cabc, branch(C::Eq, 0x6cabc, 0x6cb94, false)?)?;
    c.emit(0x6cac0, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6cac4, branch(C::Ne, 0x6cac4, 0x6cbb4, false)?)?;
    c.emit(0x6cac8, data_immediate(C::Al, Op::Add, r(0), r(9), 0x284)?)?;
    c.emit(0x6cacc, data_immediate(C::Al, Op::Add, r(1), r(0), 0x800)?)?;
    c.emit(0x6cad0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6cad4, branch(C::Al, 0x6cad4, 0x6cb4c, false)?)?;
    c.emit(0x6cad8, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3b4)?)?;
    c.emit(0x6cadc, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6cae0, data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(0x6cae4, signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(0), r(3), r(2))?)?;
    c.emit(0x6cae8, data_immediate(C::Al, Op::Add, r(2), r(0), 0x208)?)?;
    c.emit(0x6caec, data_immediate(C::Al, Op::Add, r(4), r(2), 0x400)?)?;
    c.emit(0x6caf0, branch(C::Eq, 0x6caf0, 0x6cb0c, false)?)?;
    c.emit(0x6caf4, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6caf8, branch(C::Ne, 0x6caf8, 0x6cb20, false)?)?;
    c.emit(0x6cafc, data_immediate(C::Al, Op::Add, r(0), r(0), 0x20c)?)?;
    c.emit(0x6cb00, data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.emit(0x6cb04, transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(0x6cb08, branch(C::Al, 0x6cb08, 0x6cc40, false)?)?;
    c.emit(0x6cb0c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x9c)?)?;
    c.emit(0x6cb10, data_immediate(C::Al, Op::Add, r(0), r(0), 0x800)?)?;
    c.emit(0x6cb14, transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(0x6cb18, branch(C::Al, 0x6cb18, 0x6cc40, false)?)?;
    c.literal(0x6cb1c, ARITHMETIC_MAGIC)?;
    c.emit(0x6cb20, transfer_word(C::Al, true, r(2), r(15), 0xfa8)?)?;
    c.emit(0x6cb24, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5d)?)?;
    c.emit(0x6cb28, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6cb2c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6cb30, branch(C::Al, 0x6cb30, 0x17b0, true)?)?;
    c.emit(0x6cb34, transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(0x6cb38, branch(C::Al, 0x6cb38, 0x6cc40, false)?)?;
    c.literal(0x6cb3c, DIAGNOSTIC)?;
    c.emit(0x6cb40, data_immediate(C::Al, Op::Add, r(0), r(9), 0x3a4)?)?;
    c.emit(0x6cb44, data_immediate(C::Al, Op::Add, r(1), r(0), 0xc00)?)?;
    c.emit(0x6cb48, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(0x6cb4c, transfer_word(C::Al, false, r(1), r(9), 0xa80)?)?;
    c.emit(0x6cb50, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(0x6cb54, transfer_byte(C::Al, true, r(1), r(1), 0x1)?)?;
    c.emit(0x6cb58, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6cb5c, data_immediate(C::Ne, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cb60, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6cb64, branch(C::Ne, 0x6cb64, 0x6cc50, false)?)?;
    c.emit(0x6cb68, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x29c)?)?;
    c.emit(0x6cb6c, mov_register(C::Al, r(0), r(9)))?;
    c.emit(0x6cb70, load_halfword_register_preindexed_writeback(C::Al, r(1), r(0), r(1))?)?;
    c.emit(0x6cb74, data_immediate(C::Al, Op::Tst, r(0), r(1), 0x4)?)?;
    c.emit(0x6cb78, branch(C::Eq, 0x6cb78, 0x6cc50, false)?)?;
    c.emit(0x6cb7c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xfa)?)?;
    c.emit(0x6cb80, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cb84, transfer_byte(C::Al, false, r(12), r(10), 0x4b1)?)?;
    c.emit(0x6cb88, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.emit(0x6cb8c, branch(C::Al, 0x6cb8c, 0x6cc0c, false)?)?;
    c.literal(0x6cb90, NEWLINE)?;
    c.emit(0x6cb94, data_immediate(C::Al, Op::Add, r(0), r(9), 0x114)?)?;
    c.emit(0x6cb98, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(0x6cb9c, data_immediate(C::Al, Op::Add, r(0), r(0), 0xc00)?)?;
    c.emit(0x6cba0, transfer_word(C::Al, false, r(0), r(9), 0xa80)?)?;
    c.emit(0x6cba4, transfer_byte(C::Al, true, r(0), r(9), 0xd15)?)?;
    c.emit(0x6cba8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6cbac, data_immediate(C::Ne, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cbb0, branch(C::Al, 0x6cbb0, 0x6cbe8, false)?)?;
    c.emit(0x6cbb4, transfer_word(C::Al, true, r(2), r(15), 0xf14)?)?;
    c.emit(0x6cbb8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3b)?)?;
    c.emit(0x6cbbc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6cbc0, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6cbc4, branch(C::Al, 0x6cbc4, 0x17b0, true)?)?;
    c.emit(0x6cbc8, transfer_word(C::Al, true, r(0), r(9), 0xa80)?)?;
    c.emit(0x6cbcc, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(0x6cbd0, transfer_byte(C::Al, true, r(0), r(0), 0x1)?)?;
    c.emit(0x6cbd4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6cbd8, transfer_word(C::Al, true, r(0), r(9), 0x2d8)?)?;
    c.emit(0x6cbdc, data_immediate(C::Ne, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cbe0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6cbe4, branch(C::Ne, 0x6cbe4, 0x6cb60, false)?)?;
    c.emit(0x6cbe8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x29c)?)?;
    c.emit(0x6cbec, mov_register(C::Al, r(0), r(9)))?;
    c.emit(0x6cbf0, load_halfword_register_preindexed_writeback(C::Al, r(1), r(0), r(1))?)?;
    c.emit(0x6cbf4, data_immediate(C::Al, Op::Tst, r(0), r(1), 0x2)?)?;
    c.emit(0x6cbf8, branch(C::Eq, 0x6cbf8, 0x6cc50, false)?)?;
    c.emit(0x6cbfc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3fc)?)?;
    c.emit(0x6cc00, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cc04, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xfc00)?)?;
    c.emit(0x6cc08, transfer_byte(C::Al, false, r(12), r(10), 0x4b1)?)?;
    c.emit(0x6cc0c, data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6cc10, store_halfword(C::Al, r(1), r(0), 0)?)?;
    c.emit(0x6cc14, branch(C::Al, 0x6cc14, 0x6cc50, false)?)?;
    c.emit(0x6cc18, data_immediate(C::Al, Op::Add, r(1), r(2), 0x32c)?)?;
    c.emit(0x6cc1c, branch(C::Al, 0x6cc1c, 0x6cc28, false)?)?;
    c.literal(0x6cc20, DIVIDE_CONSTANT)?;
    c.emit(0x6cc24, data_immediate(C::Al, Op::Add, r(1), r(2), 0x9c)?)?;
    c.emit(0x6cc28, data_immediate(C::Al, Op::Add, r(1), r(1), 0x800)?)?;
    c.emit(0x6cc2c, transfer_word(C::Al, false, r(1), r(2), 0x608)?)?;
    c.emit(0x6cc30, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3b4)?)?;
    c.emit(0x6cc34, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(0x6cc38, signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(0), r(1), r(2))?)?;
    c.emit(0x6cc3c, transfer_word(C::Al, true, r(0), r(0), 0x608)?)?;
    c.emit(0x6cc40, transfer_byte(C::Al, true, r(0), r(0), 0x1)?)?;
    c.emit(0x6cc44, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(0x6cc48, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6cc4c, data_immediate(C::Ne, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cc50, transfer_word(C::Al, true, r(0), r(9), 0x34)?)?;
    c.emit(0x6cc54, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6cc58, branch(C::Eq, 0x6cc58, 0x6ccb4, false)?)?;
    c.emit(0x6cc5c, transfer_byte(C::Al, true, r(1), r(9), 0x2d2)?)?;
    c.emit(0x6cc60, data_immediate(C::Al, Op::Add, r(2), r(9), 0x29c)?)?;
    c.emit(0x6cc64, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x6cc68, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x10)?)?;
    c.emit(0x6cc6c, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(0x6cc70, data_immediate(C::Al, Op::And, r(5), r(7), 0xff)?)?;
    c.emit(0x6cc74, data_register(C::Al, Op::Cmp, r(0), r(5), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6cc78, transfer_word(C::Eq, true, r(5), r(9), 0x2d8)?)?;
    c.emit(0x6cc7c, data_immediate(C::Eq, Op::Cmp, r(0), r(5), 0x1)?)?;
    c.emit(0x6cc80, branch(C::Ne, 0x6cc80, 0x6cca4, false)?)?;
    c.emit(0x6cc84, load_halfword(C::Al, r(5), r(2), 0)?)?;
    c.emit(0x6cc88, data_register_shifted_by_register(C::Al, Op::And, r(4), r(5), r(3), Shift::Lsl, r(6), false)?)?;
    c.emit(0x6cc8c, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x6cc90, data_register_shifted_by_register(C::Ne, Op::Mov, r(6), r(0), r(3), Shift::Lsl, r(6), false)?)?;
    c.emit(0x6cc94, data_immediate(C::Ne, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(0x6cc98, data_register(C::Ne, Op::Bic, r(6), r(5), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6cc9c, transfer_byte(C::Ne, false, r(12), r(10), 0x4b1)?)?;
    c.emit(0x6cca0, store_halfword(C::Ne, r(6), r(2), 0)?)?;
    c.emit(0x6cca4, data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(0x6cca8, data_immediate(C::Al, Op::And, r(6), r(7), 0xff)?)?;
    c.emit(0x6ccac, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6ccb0, branch(C::Hi, 0x6ccb0, 0x6cc70, false)?)?;

    if c.at != END || c.bytes.len() != END - START
        || c.code_bytes != CODE_BYTES || c.data_bytes != DATA_BYTES
    {
        return Err("selector state extent mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    fn code(at: usize) -> Code {
        Code { at, bytes: Vec::new(), code_bytes: 0, data_bytes: 0 }
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }

    #[test]
    fn extent_and_protected_single_word_islands() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 848);
        assert_eq!(END - START, CODE_BYTES + DATA_BYTES);
        for (at, value) in [
            (0x6ca2c, (-4608_i32) as u32),
            (0x6cb1c, 0x431bde83),
            (0x6cb3c, 0x000f2f51),
            (0x6cb90, 0x000e2788),
            (0x6cc20, 0xcccccccd),
        ] {
            assert_eq!(word(&bytes, at), value);
            let mut c = code(at);
            assert!(c.emit(at, mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.literal(at, value ^ 1).is_err());
            assert!(c.bytes.is_empty());
            c.literal(at, value).unwrap();
            assert_eq!((c.at, c.code_bytes, c.data_bytes), (at + 4, 0, 4));
            assert!(island(at - 4).is_none());
            assert!(island(at + 4).is_none());
        }
        for at in [START - 4, START + 1, END] {
            let mut c = code(at);
            assert!(c.emit(at, mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = code(START);
        assert!(c.emit(START + 4, mov_register(C::Al, r(0), r(1))).is_err());
        assert!(c.literal(START, SIGNED_OFFSET).is_err());
        assert_eq!((c.at, c.code_bytes, c.data_bytes), (START, 0, 0));
    }

    #[test]
    fn calls_and_selector_edges() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x6ca04, C::Al, 0x6f61c, true),
            (0x6cb30, C::Al, 0x17b0, true),
            (0x6cbc4, C::Al, 0x17b0, true),
            (0x6ca28, C::Al, 0x6ca3c, false),
            (0x6cb18, C::Al, 0x6cc40, false),
            (0x6cb38, C::Al, 0x6cc40, false),
            (0x6cb8c, C::Al, 0x6cc0c, false),
            (0x6cc1c, C::Al, 0x6cc28, false),
            (0x6cc58, C::Eq, END as u32, false),
            (0x6ccb0, C::Hi, 0x6cc70, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x6ca20), branch_exchange_immediate(0x6ca20, 0x83c4c).unwrap());
    }

    #[test]
    fn paired_subtraction_reset_selector_and_halfword_flags() {
        let bytes = fragment().unwrap();
        for (at, instruction) in [
            (0x6c964, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x268).unwrap()),
            (0x6c980, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x270).unwrap()),
            (0x6c988, transfer_word(C::Al, true, r(3), r(9), 0x2c0).unwrap()),
            (0x6c98c, transfer_word(C::Al, true, r(7), r(9), 0x2c4).unwrap()),
            (0x6c998, data_register(C::Al, Op::Sub, r(3), r(3), r(2), Shift::Lsl, 0, true).unwrap()),
            (0x6c9a0, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x26c).unwrap()),
            (0x6c9a4, data_register(C::Al, Op::Sbc, r(2), r(7), r(2), Shift::Asr, 31, false).unwrap()),
            (0x6c9c8, transfer_word(C::Al, false, r(3), r(0), 0).unwrap()),
            (0x6c9cc, transfer_word(C::Al, false, r(3), r(2), 0).unwrap()),
            (0x6c9d0, transfer_word(C::Al, false, r(3), r(1), 0).unwrap()),
            (0x6ca5c, transfer_word(C::Ls, false, r(2), r(9), 0x2d8).unwrap()),
            (0x6cb70, load_halfword_register_preindexed_writeback(C::Al, r(1), r(0), r(1)).unwrap()),
            (0x6cbf0, load_halfword_register_preindexed_writeback(C::Al, r(1), r(0), r(1)).unwrap()),
            (0x6cc10, store_halfword(C::Al, r(1), r(0), 0).unwrap()),
            (0x6cc88, data_register_shifted_by_register(C::Al, Op::And, r(4), r(5), r(3), Shift::Lsl, r(6), false).unwrap()),
            (0x6cca0, store_halfword(C::Ne, r(6), r(2), 0).unwrap()),
        ] {
            assert_eq!(word(&bytes, at), instruction, "{at:#x}");
        }
    }
}
