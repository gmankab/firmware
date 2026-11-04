use arm32::{
    Condition as C, DataOp as Op, Register as R, Shift,
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags, data_register, multiply_accumulate, multiply_long, multiply_register, transfer_byte, transfer_doubleword, transfer_word, transfer_word_preindexed_writeback,
};

pub const START: usize = 0x6d998;
pub const END: usize = 0x6e098;
const FILE_MAPPING_DELTA: usize = 0x4000;
const CODE_BYTES: usize = 1780;
const DATA_BYTES: usize = 12;
const PROTECTED_DATA: [(usize, u32); 3] = [(0x6dad0, 0xf2e4c), (0x6df28, 0xf329b), (0x6df8c, 0xf3252)];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if self.at != at || !(START..END).contains(&at) || at % 4 != 0
            || self.bytes.len() != at - START
            || PROTECTED_DATA.iter().any(|&(address, _)| address == at)
        {
            return Err(format!("adjustment records address {:#x}, requested {at:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        if self.at != at || !PROTECTED_DATA.contains(&(at, value))
            || self.bytes.len() != at - START
        {
            return Err(format!("invalid adjustment records literal at {at:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(0x6d998, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x6d99c, branch(C::Al, 0x6d99c, 0x67900, true)?)?;
    c.emit(0x6d9a0, data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x6d9a4, data_register(C::Al, Op::Add, r(2), r(4), r(7), Shift::Lsl, 2, false)?)?;
    c.emit(0x6d9a8, transfer_word_preindexed_writeback(C::Al, true, r(3), r(2), 548)?)?;
    c.emit(0x6d9ac, transfer_word(C::Al, false, r(3), r(4), 148)?)?;
    c.emit(0x6d9b0, transfer_byte(C::Al, true, r(1), r(4), 1548)?)?;
    c.emit(0x6d9b4, data_register(C::Al, Op::Add, r(0), r(4), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x6d9b8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6d9bc, transfer_word(C::Al, true, r(12), r(0), 1524)?)?;
    c.emit(0x6d9c0, transfer_word(C::Al, true, r(0), r(0), 1536)?)?;
    c.emit(0x6d9c4, transfer_word(C::Al, false, r(2), r(13), 72)?)?;
    c.emit(0x6d9c8, branch(C::Eq, 0x6d9c8, 0x6dde0, false)?)?;
    c.emit(0x6d9cc, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6d9d0, branch(C::Ne, 0x6d9d0, 0x6de00, false)?)?;
    c.emit(0x6d9d4, transfer_word(C::Al, true, r(2), r(11), -60)?)?;
    c.emit(0x6d9d8, transfer_word(C::Al, true, r(1), r(2), 352)?)?;
    c.emit(0x6d9dc, transfer_word(C::Al, false, r(1), r(2), 348)?)?;
    c.emit(0x6d9e0, transfer_word(C::Al, false, r(12), r(2), 352)?)?;
    c.emit(0x6d9e4, transfer_byte(C::Al, true, r(1), r(2), 159)?)?;
    c.emit(0x6d9e8, transfer_byte(C::Al, false, r(1), r(2), 158)?)?;
    c.emit(0x6d9ec, data_immediate(C::Al, Op::Add, r(1), r(2), 0x9e)?)?;
    c.emit(0x6d9f0, branch(C::Al, 0x6d9f0, 0x6de1c, false)?)?;
    c.emit(0x6d9f4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6d9f8, branch(C::Hi, 0x6d9f8, 0x6dd90, false)?)?;
    c.emit(0x6d9fc, transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(0x6da00, transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.emit(0x6da04, multiply_register(C::Al, r(0), r(1), r(2))?)?;
    c.emit(0x6da08, transfer_doubleword(C::Al, true, r(2), r(4), 0)?)?;
    c.emit(0x6da0c, data_register(C::Al, Op::Sub, r(0), r(2), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x6da10, data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(3), 0x0, true)?)?;
    c.emit(0x6da14, branch(C::Ge, 0x6da14, 0x6cce8, false)?)?;
    c.emit(0x6da18, branch(C::Al, 0x6da18, 0x6da40, false)?)?;
    c.emit(0x6da1c, transfer_word(C::Al, true, r(0), r(9), 672)?)?;
    c.emit(0x6da20, data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x0)?)?;
    c.emit(0x6da24, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsr, 6, false)?)?;
    c.emit(0x6da28, data_register(C::Al, Op::Add, r(1), r(10), r(8), Shift::Lsl, 2, false)?)?;
    c.emit(0x6da2c, transfer_word(C::Al, true, r(1), r(1), 8)?)?;
    c.emit(0x6da30, multiply_register(C::Al, r(2), r(0), r(1))?)?;
    c.emit(0x6da34, data_register(C::Al, Op::Sub, r(0), r(2), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6da38, data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(7), 0x0, true)?)?;
    c.emit(0x6da3c, branch(C::Lt, 0x6da3c, 0x6cce8, false)?)?;
    c.emit(0x6da40, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x6da44, transfer_byte(C::Al, false, r(0), r(9), 1912)?)?;
    c.emit(0x6da48, transfer_word(C::Al, true, r(0), r(9), 1916)?)?;
    c.emit(0x6da4c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x6da50, transfer_word(C::Al, false, r(0), r(9), 1916)?)?;
    c.emit(0x6da54, branch(C::Al, 0x6da54, 0x6cce8, false)?)?;
    c.emit(0x6da58, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6da5c, branch(C::Ne, 0x6da5c, 0x6ddbc, false)?)?;
    c.emit(0x6da60, transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(0x6da64, data_register(C::Al, Op::Sub, r(0), r(6), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x6da68, data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(7), 0x0, true)?)?;
    c.emit(0x6da6c, branch(C::Ge, 0x6da6c, 0x6da84, false)?)?;
    c.emit(0x6da70, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x6da74, transfer_byte(C::Al, false, r(0), r(9), 1912)?)?;
    c.emit(0x6da78, transfer_word(C::Al, true, r(0), r(9), 1916)?)?;
    c.emit(0x6da7c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x6da80, transfer_word(C::Al, false, r(0), r(9), 1916)?)?;
    c.emit(0x6da84, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(6), 0x1, true)?)?;
    c.emit(0x6da88, data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(7), 0x0, true)?)?;
    c.emit(0x6da8c, branch(C::Lt, 0x6da8c, 0x6cce8, false)?)?;
    c.emit(0x6da90, transfer_byte(C::Al, true, r(0), r(9), 37)?)?;
    c.emit(0x6da94, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6da98, branch(C::Hi, 0x6da98, 0x6dad4, false)?)?;
    c.emit(0x6da9c, transfer_word(C::Al, true, r(1), r(9), 712)?)?;
    c.emit(0x6daa0, transfer_word(C::Al, true, r(2), r(9), 716)?)?;
    c.emit(0x6daa4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0xc)?)?;
    c.emit(0x6daa8, multiply_long(C::Al, false, r(0), r(5), r(1), r(3))?)?;
    c.emit(0x6daac, data_register(C::Al, Op::Add, r(1), r(2), r(2), Shift::Lsl, 1, false)?)?;
    c.emit(0x6dab0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa)?)?;
    c.emit(0x6dab4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x6dab8, data_register(C::Al, Op::Add, r(1), r(5), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x6dabc, branch(C::Al, 0x6dabc, 0x8187c, true)?)?;
    c.emit(0x6dac0, data_register(C::Al, Op::Sub, r(2), r(0), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6dac4, data_register(C::Al, Op::Sbc, r(2), r(1), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6dac8, transfer_doubleword(C::Cc, false, r(0), r(4), 0)?)?;
    c.emit(0x6dacc, branch(C::Al, 0x6dacc, 0x6cce8, false)?)?;
    c.literal(0x6dad0, 0xf2e4c)?;
    c.emit(0x6dad4, transfer_word(C::Al, true, r(1), r(9), 116)?)?;
    c.emit(0x6dad8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6dadc, branch(C::Eq, 0x6dadc, 0x6db40, false)?)?;
    c.emit(0x6dae0, transfer_word(C::Al, true, r(1), r(9), 1880)?)?;
    c.emit(0x6dae4, transfer_word(C::Al, true, r(5), r(9), 1884)?)?;
    c.emit(0x6dae8, transfer_word(C::Al, true, r(3), r(9), 716)?)?;
    c.emit(0x6daec, data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsr, 2, false)?)?;
    c.emit(0x6daf0, data_register(C::Al, Op::Mov, r(14), r(0), r(5), Shift::Lsr, 2, false)?)?;
    c.emit(0x6daf4, data_register(C::Al, Op::Orr, r(12), r(2), r(5), Shift::Lsl, 30, false)?)?;
    c.emit(0x6daf8, transfer_word(C::Al, true, r(2), r(9), 712)?)?;
    c.emit(0x6dafc, data_register(C::Al, Op::Add, r(2), r(12), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(0x6db00, data_register(C::Al, Op::Adc, r(3), r(14), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6db04, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6db08, data_register(C::Eq, Op::Orr, r(0), r(1), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x6db0c, branch(C::Ne, 0x6db0c, 0x6db30, false)?)?;
    c.emit(0x6db10, data_immediate(C::Al, Op::Mov, r(12), r(0), 0xb)?)?;
    c.emit(0x6db14, multiply_long(C::Al, false, r(0), r(5), r(2), r(12))?)?;
    c.emit(0x6db18, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa)?)?;
    c.emit(0x6db1c, multiply_accumulate(C::Al, r(1), r(3), r(12), r(5))?)?;
    c.emit(0x6db20, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x6db24, branch(C::Al, 0x6db24, 0x8187c, true)?)?;
    c.emit(0x6db28, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x6db2c, data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6db30, data_register(C::Al, Op::Sub, r(0), r(2), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6db34, data_register(C::Al, Op::Sbc, r(0), r(3), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6db38, transfer_doubleword(C::Cc, false, r(2), r(4), 0)?)?;
    c.emit(0x6db3c, branch(C::Al, 0x6db3c, 0x6cce8, false)?)?;
    c.emit(0x6db40, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6db44, branch(C::Ne, 0x6db44, 0x6cce8, false)?)?;
    c.emit(0x6db48, transfer_word(C::Al, true, r(5), r(9), 712)?)?;
    c.emit(0x6db4c, transfer_word(C::Al, true, r(8), r(9), 716)?)?;
    c.emit(0x6db50, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x69)?)?;
    c.emit(0x6db54, multiply_long(C::Al, false, r(0), r(3), r(5), r(2))?)?;
    c.emit(0x6db58, multiply_accumulate(C::Al, r(1), r(8), r(2), r(3))?)?;
    c.emit(0x6db5c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x64)?)?;
    c.emit(0x6db60, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x6db64, branch(C::Al, 0x6db64, 0x8187c, true)?)?;
    c.emit(0x6db68, data_register(C::Al, Op::Sub, r(0), r(0), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6db6c, data_register(C::Al, Op::Sbc, r(0), r(1), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6db70, branch(C::Cs, 0x6db70, 0x6cce8, false)?)?;
    c.emit(0x6db74, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6c)?)?;
    c.emit(0x6db78, multiply_long(C::Al, false, r(0), r(3), r(5), r(2))?)?;
    c.emit(0x6db7c, multiply_accumulate(C::Al, r(1), r(8), r(2), r(3))?)?;
    c.emit(0x6db80, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x64)?)?;
    c.emit(0x6db84, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x6db88, branch(C::Al, 0x6db88, 0x8187c, true)?)?;
    c.emit(0x6db8c, transfer_doubleword(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0x6db90, branch(C::Al, 0x6db90, 0x6cce8, false)?)?;
    c.emit(0x6db94, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6db98, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6db9c, branch(C::Al, 0x6db9c, 0x6d888, false)?)?;
    c.emit(0x6dba0, transfer_word(C::Al, true, r(0), r(9), 2688)?)?;
    c.emit(0x6dba4, transfer_byte(C::Al, true, r(1), r(0), 7)?)?;
    c.emit(0x6dba8, transfer_byte(C::Al, true, r(0), r(0), 9)?)?;
    c.emit(0x6dbac, transfer_word(C::Al, true, r(2), r(13), 56)?)?;
    c.emit(0x6dbb0, data_immediate(C::Al, Op::And, r(1), r(1), 0xff)?)?;
    c.emit(0x6dbb4, data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.emit(0x6dbb8, data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbbc, transfer_word(C::Al, false, r(2), r(11), -68)?)?;
    c.emit(0x6dbc0, transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.emit(0x6dbc4, data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbc8, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbcc, transfer_word(C::Al, false, r(0), r(11), -64)?)?;
    c.emit(0x6dbd0, transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(0x6dbd4, branch(C::Al, 0x6dbd4, 0x70d64, true)?)?;
    c.emit(0x6dbd8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3f4)?)?;
    c.emit(0x6dbdc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.emit(0x6dbe0, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbe4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xfc00)?)?;
    c.emit(0x6dbe8, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.emit(0x6dbec, data_immediate(C::Al, Op::Add, r(1), r(1), 0xb)?)?;
    c.emit(0x6dbf0, data_register(C::Al, Op::And, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbf4, data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbf8, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dbfc, transfer_word(C::Al, false, r(1), r(11), -72)?)?;
    c.emit(0x6dc00, branch(C::Eq, 0x6dc00, 0x6e098, false)?)?;
    c.emit(0x6dc04, transfer_word(C::Al, true, r(0), r(13), 72)?)?;
    c.emit(0x6dc08, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dc0c, transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.emit(0x6dc10, transfer_word(C::Al, false, r(10), r(13), 44)?)?;
    c.emit(0x6dc14, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(0x6dc18, data_immediate(C::Al, Op::And, r(6), r(0), 0xff)?)?;
    c.emit(0x6dc1c, data_register(C::Al, Op::Sub, r(0), r(4), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6dc20, data_immediate(C::Mi, Op::Rsb, r(0), r(0), 0x0)?)?;
    c.emit(0x6dc24, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x2)?)?;
    c.emit(0x6dc28, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4)?)?;
    c.emit(0x6dc2c, branch(C::Hi, 0x6dc2c, 0x6dd88, false)?)?;
    c.emit(0x6dc30, data_register(C::Al, Op::Cmp, r(0), r(4), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dc34, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6dc38, transfer_word(C::Al, false, r(8), r(13), 28)?)?;
    c.emit(0x6dc3c, transfer_word(C::Al, false, r(1), r(13), 52)?)?;
    c.emit(0x6dc40, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(0x6dc44, data_immediate(C::Ge, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x6dc48, transfer_word(C::Al, false, r(0), r(13), 48)?)?;
    c.emit(0x6dc4c, transfer_word(C::Al, true, r(0), r(15), 4012)?)?;
    c.emit(0x6dc50, transfer_word(C::Al, false, r(6), r(13), 72)?)?;
    c.emit(0x6dc54, multiply_long(C::Al, false, r(1), r(3), r(7), r(0))?)?;
    c.emit(0x6dc58, data_register(C::Al, Op::Mov, r(10), r(0), r(3), Shift::Lsr, 4, false)?)?;
    c.emit(0x6dc5c, data_register(C::Lt, Op::Sub, r(10), r(8), r(3), Shift::Lsr, 4, false)?)?;
    c.emit(0x6dc60, data_register(C::Al, Op::Add, r(0), r(10), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dc64, data_register(C::Al, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dc68, transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(0x6dc6c, transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.emit(0x6dc70, transfer_word(C::Al, false, r(5), r(13), 0)?)?;
    c.emit(0x6dc74, data_register(C::Al, Op::Mov, r(6), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dc78, transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(0x6dc7c, branch(C::Al, 0x6dc7c, 0x70d64, true)?)?;
    c.emit(0x6dc80, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.emit(0x6dc84, branch(C::Lt, 0x6dc84, 0x6dca8, false)?)?;
    c.emit(0x6dc88, data_register(C::Al, Op::Mov, r(12), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dc8c, transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.emit(0x6dc90, transfer_word(C::Al, true, r(6), r(13), 72)?)?;
    c.emit(0x6dc94, branch(C::Eq, 0x6dc94, 0x6dcbc, false)?)?;
    c.emit(0x6dc98, transfer_word(C::Al, true, r(1), r(11), -64)?)?;
    c.emit(0x6dc9c, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dca0, branch(C::Lt, 0x6dca0, 0x6dcbc, false)?)?;
    c.emit(0x6dca4, branch(C::Al, 0x6dca4, 0x6dd2c, false)?)?;
    c.emit(0x6dca8, data_register(C::Al, Op::Mov, r(12), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcac, transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.emit(0x6dcb0, transfer_word(C::Al, true, r(6), r(13), 72)?)?;
    c.emit(0x6dcb4, data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcb8, branch(C::Le, 0x6dcb8, 0x6dd2c, false)?)?;
    c.emit(0x6dcbc, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcc0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6dcc4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x6dcc8, data_immediate(C::Gt, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(0x6dccc, data_register(C::Al, Op::Cmp, r(0), r(4), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcd0, data_immediate(C::Le, Op::Mov, r(3), r(0), 0x1)?)?;
    c.emit(0x6dcd4, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcd8, data_register(C::Al, Op::Orr, r(3), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcdc, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6dce0, data_immediate(C::Lt, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(0x6dce4, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.emit(0x6dce8, data_register(C::Ne, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcec, branch(C::Eq, 0x6dcec, 0x6dd2c, false)?)?;
    c.emit(0x6dcf0, transfer_word(C::Al, true, r(3), r(13), 48)?)?;
    c.emit(0x6dcf4, data_register(C::Al, Op::Orr, r(1), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dcf8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x6dcfc, branch(C::Ne, 0x6dcfc, 0x6dd2c, false)?)?;
    c.emit(0x6dd00, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.emit(0x6dd04, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xff00)?)?;
    c.emit(0x6dd08, data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dd0c, transfer_word(C::Al, true, r(1), r(11), -72)?)?;
    c.emit(0x6dd10, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dd14, branch(C::Eq, 0x6dd14, 0x6dd2c, false)?)?;
    c.emit(0x6dd18, data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(0x6dd1c, data_register(C::Al, Op::Add, r(0), r(12), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dd20, data_immediate(C::Al, Op::And, r(1), r(8), 0xff)?)?;
    c.emit(0x6dd24, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0xf)?)?;
    c.emit(0x6dd28, branch(C::Cc, 0x6dd28, 0x6dc64, false)?)?;
    c.emit(0x6dd2c, transfer_word(C::Al, true, r(10), r(13), 44)?)?;
    c.emit(0x6dd30, data_register(C::Al, Op::Cmp, r(0), r(12), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dd34, branch(C::Ls, 0x6dd34, 0x6ddb4, false)?)?;
    c.emit(0x6dd38, transfer_word(C::Al, true, r(0), r(13), 40)?)?;
    c.emit(0x6dd3c, transfer_word(C::Al, true, r(8), r(13), 28)?)?;
    c.emit(0x6dd40, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6dd44, branch(C::Ne, 0x6dd44, 0x6dd58, false)?)?;
    c.emit(0x6dd48, transfer_doubleword(C::Al, true, r(0), r(10), 0)?)?;
    c.emit(0x6dd4c, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 0x1, true)?)?;
    c.emit(0x6dd50, data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(1), 0x0, true)?)?;
    c.emit(0x6dd54, branch(C::Lt, 0x6dd54, 0x6e098, false)?)?;
    c.emit(0x6dd58, transfer_word(C::Al, true, r(0), r(13), 36)?)?;
    c.emit(0x6dd5c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6dd60, branch(C::Ne, 0x6dd60, 0x6df84, false)?)?;
    c.emit(0x6dd64, transfer_word(C::Al, true, r(1), r(13), 32)?)?;
    c.emit(0x6dd68, transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(0x6dd6c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x78)?)?;
    c.emit(0x6dd70, branch(C::Cc, 0x6dd70, 0x6e08c, false)?)?;
    c.emit(0x6dd74, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(0x6dd78, data_register(C::Al, Op::Mov, r(7), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dd7c, data_register(C::Al, Op::Cmp, r(0), r(12), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(0x6dd80, data_register(C::Hi, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(0x6dd84, branch(C::Al, 0x6dd84, 0x6e098, false)?)?;
    c.emit(0x6dd88, transfer_word(C::Al, true, r(10), r(13), 44)?)?;
    c.emit(0x6dd8c, branch(C::Al, 0x6dd8c, 0x6e098, false)?)?;
    c.emit(0x6dd90, transfer_word(C::Al, true, r(2), r(4), 0)?)?;
    c.emit(0x6dd94, transfer_word(C::Al, true, r(1), r(4), 4)?)?;
    c.emit(0x6dd98, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6dd9c, branch(C::Ne, 0x6dd9c, 0x6df60, false)?)?;
    c.emit(0x6dda0, transfer_word(C::Al, true, r(0), r(11), -64)?)?;
    c.emit(0x6dda4, data_register(C::Al, Op::Sub, r(0), r(2), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x6dda8, data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(1), 0x0, true)?)?;
    c.emit(0x6ddac, branch(C::Ge, 0x6ddac, 0x6cce8, false)?)?;
    c.emit(0x6ddb0, branch(C::Al, 0x6ddb0, 0x6da40, false)?)?;
    c.emit(0x6ddb4, transfer_word(C::Al, true, r(8), r(13), 28)?)?;
    c.emit(0x6ddb8, branch(C::Al, 0x6ddb8, 0x6e098, false)?)?;
    c.emit(0x6ddbc, transfer_word(C::Al, true, r(0), r(9), 672)?)?;
    c.emit(0x6ddc0, transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(0x6ddc4, data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x0)?)?;
    c.emit(0x6ddc8, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsr, 6, false)?)?;
    c.emit(0x6ddcc, multiply_register(C::Al, r(1), r(0), r(2))?)?;
    c.emit(0x6ddd0, data_register(C::Al, Op::Sub, r(0), r(1), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6ddd4, data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(7), 0x0, true)?)?;
    c.emit(0x6ddd8, branch(C::Ge, 0x6ddd8, 0x6da70, false)?)?;
    c.emit(0x6dddc, branch(C::Al, 0x6dddc, 0x6da84, false)?)?;
    c.emit(0x6dde0, transfer_word(C::Al, true, r(2), r(11), -60)?)?;
    c.emit(0x6dde4, transfer_word(C::Al, true, r(1), r(2), 360)?)?;
    c.emit(0x6dde8, transfer_word(C::Al, false, r(1), r(2), 356)?)?;
    c.emit(0x6ddec, transfer_word(C::Al, false, r(12), r(2), 360)?)?;
    c.emit(0x6ddf0, transfer_byte(C::Al, true, r(1), r(2), 161)?)?;
    c.emit(0x6ddf4, transfer_byte(C::Al, false, r(1), r(2), 160)?)?;
    c.emit(0x6ddf8, data_immediate(C::Al, Op::Add, r(1), r(2), 0xa0)?)?;
    c.emit(0x6ddfc, branch(C::Al, 0x6ddfc, 0x6de1c, false)?)?;
    c.emit(0x6de00, transfer_word(C::Al, true, r(2), r(11), -60)?)?;
    c.emit(0x6de04, transfer_word(C::Al, true, r(1), r(2), 368)?)?;
    c.emit(0x6de08, transfer_word(C::Al, false, r(1), r(2), 364)?)?;
    c.emit(0x6de0c, transfer_word(C::Al, false, r(12), r(2), 368)?)?;
    c.emit(0x6de10, transfer_byte(C::Al, true, r(1), r(2), 163)?)?;
    c.emit(0x6de14, transfer_byte(C::Al, false, r(1), r(2), 162)?)?;
    c.emit(0x6de18, data_immediate(C::Al, Op::Add, r(1), r(2), 0xa2)?)?;
    c.emit(0x6de1c, transfer_byte(C::Al, false, r(0), r(1), 1)?)?;
    c.emit(0x6de20, transfer_word(C::Al, true, r(14), r(2), 728)?)?;
    c.emit(0x6de24, transfer_word(C::Al, false, r(3), r(11), -64)?)?;
    c.emit(0x6de28, data_immediate(C::Al, Op::Cmp, r(0), r(14), 0x2)?)?;
    c.emit(0x6de2c, branch(C::Eq, 0x6de2c, 0x6debc, false)?)?;
    c.emit(0x6de30, data_immediate(C::Al, Op::Cmp, r(0), r(14), 0x1)?)?;
    c.emit(0x6de34, branch(C::Eq, 0x6de34, 0x6df2c, false)?)?;
    c.emit(0x6de38, transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.emit(0x6de3c, data_immediate(C::Al, Op::Cmp, r(0), r(14), 0x0)?)?;
    c.emit(0x6de40, branch(C::Ne, 0x6de40, 0x6f2d0, false)?)?;
    c.emit(0x6de44, transfer_word(C::Al, true, r(0), r(4), 348)?)?;
    c.emit(0x6de48, transfer_word(C::Al, true, r(5), r(4), 352)?)?;
    c.emit(0x6de4c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6de50, branch(C::Eq, 0x6de50, 0x6f1d8, false)?)?;
    c.emit(0x6de54, transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.emit(0x6de58, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x6de5c, branch(C::Eq, 0x6de5c, 0x6f2d0, false)?)?;
    c.emit(0x6de60, multiply_register(C::Al, r(2), r(0), r(0))?)?;
    c.emit(0x6de64, multiply_accumulate(C::Al, r(1), r(5), r(5), r(2))?)?;
    c.emit(0x6de68, transfer_word(C::Al, false, r(1), r(11), -72)?)?;
    c.emit(0x6de6c, transfer_byte(C::Al, true, r(2), r(4), 158)?)?;
    c.emit(0x6de70, transfer_byte(C::Al, true, r(1), r(4), 159)?)?;
    c.emit(0x6de74, transfer_word(C::Al, false, r(14), r(13), 52)?)?;
    c.emit(0x6de78, multiply_register(C::Al, r(3), r(0), r(2))?)?;
    c.emit(0x6de7c, transfer_word(C::Al, false, r(1), r(11), -68)?)?;
    c.emit(0x6de80, multiply_accumulate(C::Al, r(0), r(5), r(1), r(3))?)?;
    c.emit(0x6de84, transfer_word(C::Al, true, r(1), r(11), -72)?)?;
    c.emit(0x6de88, transfer_word(C::Al, false, r(12), r(11), -72)?)?;
    c.emit(0x6de8c, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 10, false)?)?;
    c.emit(0x6de90, branch_exchange_immediate(0x6de90, 0x83c4c)?)?;
    c.emit(0x6de94, transfer_word(C::Al, true, r(14), r(13), 52)?)?;
    c.emit(0x6de98, transfer_word(C::Al, true, r(12), r(11), -72)?)?;
    c.emit(0x6de9c, transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.emit(0x6dea0, transfer_word(C::Al, false, r(0), r(4), 1588)?)?;
    c.emit(0x6dea4, multiply_register(C::Al, r(1), r(0), r(5))?)?;
    c.emit(0x6dea8, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 4, false)?)?;
    c.emit(0x6deac, transfer_word(C::Al, true, r(1), r(11), -68)?)?;
    c.emit(0x6deb0, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 6, false)?)?;
    c.emit(0x6deb4, transfer_word(C::Al, false, r(0), r(4), 1600)?)?;
    c.emit(0x6deb8, branch(C::Al, 0x6deb8, 0x6f2d0, false)?)?;
    c.emit(0x6debc, transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.emit(0x6dec0, transfer_word(C::Al, true, r(0), r(4), 364)?)?;
    c.emit(0x6dec4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6dec8, transfer_word(C::Ne, true, r(5), r(4), 368)?)?;
    c.emit(0x6decc, data_immediate(C::Ne, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x6ded0, branch(C::Eq, 0x6ded0, 0x6df90, false)?)?;
    c.emit(0x6ded4, multiply_register(C::Al, r(2), r(0), r(0))?)?;
    c.emit(0x6ded8, multiply_accumulate(C::Al, r(1), r(5), r(5), r(2))?)?;
    c.emit(0x6dedc, transfer_word(C::Al, false, r(1), r(11), -72)?)?;
    c.emit(0x6dee0, transfer_byte(C::Al, true, r(2), r(4), 162)?)?;
    c.emit(0x6dee4, transfer_byte(C::Al, true, r(1), r(4), 163)?)?;
    c.emit(0x6dee8, transfer_word(C::Al, false, r(14), r(13), 52)?)?;
    c.emit(0x6deec, multiply_register(C::Al, r(3), r(0), r(2))?)?;
    c.emit(0x6def0, transfer_word(C::Al, false, r(1), r(11), -68)?)?;
    c.emit(0x6def4, multiply_accumulate(C::Al, r(0), r(5), r(1), r(3))?)?;
    c.emit(0x6def8, transfer_word(C::Al, true, r(1), r(11), -72)?)?;
    c.emit(0x6defc, transfer_word(C::Al, false, r(12), r(11), -72)?)?;
    c.emit(0x6df00, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 10, false)?)?;
    c.emit(0x6df04, branch_exchange_immediate(0x6df04, 0x83c4c)?)?;
    c.emit(0x6df08, transfer_word(C::Al, true, r(14), r(13), 52)?)?;
    c.emit(0x6df0c, transfer_word(C::Al, true, r(12), r(11), -72)?)?;
    c.emit(0x6df10, transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.emit(0x6df14, transfer_word(C::Al, false, r(0), r(4), 1596)?)?;
    c.emit(0x6df18, multiply_register(C::Al, r(1), r(0), r(5))?)?;
    c.emit(0x6df1c, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 4, false)?)?;
    c.emit(0x6df20, transfer_word(C::Al, true, r(1), r(11), -68)?)?;
    c.emit(0x6df24, branch(C::Al, 0x6df24, 0x6e014, false)?)?;
    c.literal(0x6df28, 0xf329b)?;
    c.emit(0x6df2c, transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.emit(0x6df30, transfer_word(C::Al, false, r(7), r(11), -68)?)?;
    c.emit(0x6df34, transfer_word(C::Al, true, r(0), r(4), 356)?)?;
    c.emit(0x6df38, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6df3c, transfer_word(C::Ne, true, r(5), r(4), 360)?)?;
    c.emit(0x6df40, data_immediate(C::Ne, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x6df44, branch(C::Eq, 0x6df44, 0x6e020, false)?)?;
    c.emit(0x6df48, multiply_register(C::Al, r(2), r(0), r(0))?)?;
    c.emit(0x6df4c, transfer_byte(C::Al, true, r(7), r(4), 161)?)?;
    c.emit(0x6df50, multiply_accumulate(C::Al, r(1), r(5), r(5), r(2))?)?;
    c.emit(0x6df54, transfer_byte(C::Al, true, r(2), r(4), 160)?)?;
    c.emit(0x6df58, transfer_word(C::Al, false, r(7), r(11), -72)?)?;
    c.emit(0x6df5c, branch(C::Al, 0x6df5c, 0x6e048, false)?)?;
    c.emit(0x6df60, transfer_word(C::Al, true, r(0), r(9), 672)?)?;
    c.emit(0x6df64, transfer_word(C::Al, true, r(7), r(11), -72)?)?;
    c.emit(0x6df68, data_immediate(C::Al, Op::Mvn, r(3), r(0), 0x0)?)?;
    c.emit(0x6df6c, data_register(C::Al, Op::Add, r(0), r(3), r(0), Shift::Lsr, 6, false)?)?;
    c.emit(0x6df70, multiply_register(C::Al, r(3), r(0), r(7))?)?;
    c.emit(0x6df74, data_register(C::Al, Op::Sub, r(0), r(3), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(0x6df78, data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(1), 0x0, true)?)?;
    c.emit(0x6df7c, branch(C::Lt, 0x6df7c, 0x6cce8, false)?)?;
    c.emit(0x6df80, branch(C::Al, 0x6df80, 0x6da40, false)?)?;
    c.emit(0x6df84, data_register(C::Al, Op::Mov, r(7), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x6df88, branch(C::Al, 0x6df88, 0x6e098, false)?)?;
    c.literal(0x6df8c, 0xf3252)?;
    c.emit(0x6df90, transfer_word(C::Al, true, r(1), r(4), 360)?)?;
    c.emit(0x6df94, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6df98, branch(C::Eq, 0x6df98, 0x6f2d0, false)?)?;
    c.emit(0x6df9c, transfer_word(C::Al, true, r(5), r(4), 368)?)?;
    c.emit(0x6dfa0, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x6dfa4, branch(C::Eq, 0x6dfa4, 0x6f264, false)?)?;
    c.emit(0x6dfa8, multiply_register(C::Al, r(0), r(1), r(1))?)?;
    c.emit(0x6dfac, transfer_word(C::Al, false, r(1), r(11), -68)?)?;
    c.emit(0x6dfb0, multiply_accumulate(C::Al, r(1), r(5), r(5), r(0))?)?;
    c.emit(0x6dfb4, transfer_word(C::Al, false, r(1), r(11), -72)?)?;
    c.emit(0x6dfb8, transfer_byte(C::Al, true, r(0), r(4), 161)?)?;
    c.emit(0x6dfbc, transfer_word(C::Al, false, r(0), r(13), 48)?)?;
    c.emit(0x6dfc0, transfer_byte(C::Al, true, r(0), r(4), 163)?)?;
    c.emit(0x6dfc4, transfer_word(C::Al, true, r(1), r(13), 48)?)?;
    c.emit(0x6dfc8, transfer_word(C::Al, false, r(0), r(13), 52)?)?;
    c.emit(0x6dfcc, transfer_word(C::Al, true, r(0), r(11), -68)?)?;
    c.emit(0x6dfd0, transfer_word(C::Al, false, r(7), r(11), -68)?)?;
    c.emit(0x6dfd4, data_register(C::Al, Op::Mov, r(7), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dfd8, multiply_register(C::Al, r(2), r(0), r(1))?)?;
    c.emit(0x6dfdc, transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.emit(0x6dfe0, multiply_accumulate(C::Al, r(0), r(5), r(1), r(2))?)?;
    c.emit(0x6dfe4, transfer_word(C::Al, true, r(1), r(11), -72)?)?;
    c.emit(0x6dfe8, transfer_word(C::Al, false, r(12), r(11), -72)?)?;
    c.emit(0x6dfec, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 10, false)?)?;
    c.emit(0x6dff0, branch_exchange_immediate(0x6dff0, 0x83c4c)?)?;
    c.emit(0x6dff4, data_register(C::Al, Op::Mov, r(14), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6dff8, transfer_word(C::Al, true, r(7), r(11), -68)?)?;
    c.emit(0x6dffc, transfer_word(C::Al, true, r(12), r(11), -72)?)?;
    c.emit(0x6e000, transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.emit(0x6e004, transfer_word(C::Al, false, r(0), r(4), 1596)?)?;
    c.emit(0x6e008, multiply_register(C::Al, r(1), r(0), r(5))?)?;
    c.emit(0x6e00c, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 4, false)?)?;
    c.emit(0x6e010, transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.emit(0x6e014, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 6, false)?)?;
    c.emit(0x6e018, transfer_word(C::Al, false, r(0), r(4), 1608)?)?;
    c.emit(0x6e01c, branch(C::Al, 0x6e01c, 0x6f2d0, false)?)?;
    c.emit(0x6e020, transfer_word(C::Al, true, r(0), r(4), 368)?)?;
    c.emit(0x6e024, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6e028, transfer_word(C::Ne, true, r(5), r(4), 360)?)?;
    c.emit(0x6e02c, data_immediate(C::Ne, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x6e030, branch(C::Eq, 0x6e030, 0x6f1f4, false)?)?;
    c.emit(0x6e034, transfer_byte(C::Al, true, r(7), r(4), 161)?)?;
    c.emit(0x6e038, multiply_register(C::Al, r(2), r(0), r(0))?)?;
    c.emit(0x6e03c, multiply_accumulate(C::Al, r(1), r(5), r(5), r(2))?)?;
    c.emit(0x6e040, transfer_word(C::Al, false, r(7), r(11), -72)?)?;
    c.emit(0x6e044, transfer_byte(C::Al, true, r(2), r(4), 163)?)?;
    c.emit(0x6e048, multiply_register(C::Al, r(3), r(0), r(2))?)?;
    c.emit(0x6e04c, transfer_word(C::Al, false, r(14), r(13), 52)?)?;
    c.emit(0x6e050, multiply_accumulate(C::Al, r(0), r(5), r(7), r(3))?)?;
    c.emit(0x6e054, data_register(C::Al, Op::Mov, r(7), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x6e058, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 10, false)?)?;
    c.emit(0x6e05c, branch_exchange_immediate(0x6e05c, 0x83c4c)?)?;
    c.emit(0x6e060, transfer_word(C::Al, true, r(14), r(13), 52)?)?;
    c.emit(0x6e064, transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.emit(0x6e068, transfer_word(C::Al, false, r(0), r(4), 1592)?)?;
    c.emit(0x6e06c, multiply_register(C::Al, r(1), r(0), r(5))?)?;
    c.emit(0x6e070, data_register(C::Al, Op::Mov, r(12), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6e074, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 4, false)?)?;
    c.emit(0x6e078, transfer_word(C::Al, true, r(1), r(11), -72)?)?;
    c.emit(0x6e07c, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 6, false)?)?;
    c.emit(0x6e080, transfer_word(C::Al, false, r(0), r(4), 1604)?)?;
    c.emit(0x6e084, transfer_word(C::Al, true, r(7), r(11), -68)?)?;
    c.emit(0x6e088, branch(C::Al, 0x6e088, 0x6f2d0, false)?)?;
    c.emit(0x6e08c, data_register(C::Al, Op::Cmp, r(0), r(12), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x6e090, data_register(C::Al, Op::Mov, r(7), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x6e094, data_register(C::Hi, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 2, false)?)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("invalid adjustment records extent".into());
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
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
        let external = 0x6ec00 + FILE_MAPPING_DELTA;
        assert_eq!(u32::from_le_bytes(reference[external..external + 4].try_into().unwrap()), 0xcccccccd);
    }

    #[test]
    fn exact_extent_and_protected_data() {
        let bytes = fragment().unwrap();
        assert_eq!((bytes.len(), CODE_BYTES, DATA_BYTES), (1792, 1780, 12));
        for (at, value) in PROTECTED_DATA {
            assert_eq!(word(&bytes, at), value);
            let mut c = Code { at, bytes: vec![0; at - START] };
            assert!(c.emit(at, 0).is_err());
            assert!(c.literal(at, value ^ 1).is_err());
            c.literal(at, value).unwrap();
            assert!(c.literal(at, value).is_err());
        }
        for at in [START - 4, START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, 0).is_err());
            assert!(c.literal(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, 0).is_err());
    }

    #[test]
    fn calls_and_bounded_adjustment_loop() {
        let bytes = fragment().unwrap();
        for (at, target) in [(0x6d99c, 0x67900), (0x6dbd4, 0x70d64), (0x6dc7c, 0x70d64),
            (0x6dabc, 0x8187c), (0x6db24, 0x8187c), (0x6db64, 0x8187c), (0x6db88, 0x8187c)] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, true).unwrap());
        }
        for at in [0x6de90, 0x6df04, 0x6dff0, 0x6e05c] {
            assert_eq!(word(&bytes, at), branch_exchange_immediate(at as u32, 0x83c4c).unwrap());
        }
        assert_eq!(word(&bytes, 0x6dd20), data_immediate(C::Al, Op::And, r(1), r(8), 0xff).unwrap());
        assert_eq!(word(&bytes, 0x6dd24), data_immediate(C::Al, Op::Cmp, r(0), r(1), 0xf).unwrap());
        assert_eq!(word(&bytes, 0x6dd28), branch(C::Cc, 0x6dd28, 0x6dc64, false).unwrap());
        assert_eq!(word(&bytes, 0x6dc4c), transfer_word(C::Al, true, r(0), R::PC, 0x6ec00 - 0x6dc54).unwrap());
    }

    #[test]
    fn indexed_state_and_previous_current_copies() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6d9a4), data_register(C::Al, Op::Add, r(2), r(4), r(7), Shift::Lsl, 2, false).unwrap());
        assert_eq!(word(&bytes, 0x6d9a8), transfer_word_preindexed_writeback(C::Al, true, r(3), r(2), 0x224).unwrap());
        for (at, previous, current, byte) in [(0x6d9d8, 0x15c, 0x160, 0x9e),
            (0x6dde4, 0x164, 0x168, 0xa0), (0x6de04, 0x16c, 0x170, 0xa2)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(1), r(2), current).unwrap());
            assert_eq!(word(&bytes, at + 4), transfer_word(C::Al, false, r(1), r(2), previous).unwrap());
            assert_eq!(word(&bytes, at + 8), transfer_word(C::Al, false, r(12), r(2), current).unwrap());
            assert_eq!(word(&bytes, at + 12), transfer_byte(C::Al, true, r(1), r(2), byte + 1).unwrap());
            assert_eq!(word(&bytes, at + 16), transfer_byte(C::Al, false, r(1), r(2), byte).unwrap());
        }
        assert_eq!(word(&bytes, 0x6da0c), data_register(C::Al, Op::Sub, r(0), r(2), r(0), Shift::Lsl, 0, true).unwrap());
        assert_eq!(word(&bytes, 0x6da10), data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(3), 0, true).unwrap());
    }

    #[test]
    fn multiply_accumulate_and_metric_stores() {
        let bytes = fragment().unwrap();
        for at in [0x6de64, 0x6ded8, 0x6df50, 0x6e03c] {
            assert_eq!(word(&bytes, at), multiply_accumulate(C::Al, r(1), r(5), r(5), r(2)).unwrap());
        }
        for (at, offset) in [(0x6dea0, 0x634), (0x6deb4, 0x640), (0x6df14, 0x63c),
            (0x6e004, 0x63c), (0x6e018, 0x648), (0x6e068, 0x638), (0x6e080, 0x644)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, false, r(0), r(4), offset).unwrap());
        }
    }
}
