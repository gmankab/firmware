use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags,
    data_register, multiply_accumulate, multiply_long, transfer_byte,
    transfer_doubleword, transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6f738;
pub const END: usize = 0x70128;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2516;
pub const DATA_BYTES: usize = 28;
const POOL_START: usize = 0x7010c;
const CACHED_FLOAT: u32 = 0x10aed0;
const DIVIDE_BY_FIVE_MAGIC: u32 = 0xcccccccd;
const ONE_POINT_SIX: u64 = 1.6f64.to_bits();
const SIGNED_SELECTOR: u32 = 0x10aed4;
const DIAGNOSTIC_TABLE: u32 = 0xf2d64;
const CACHED_WIDE: u32 = 0x10aec8;
const UNSIGNED_TO_FLOAT: u32 = 0x82f74;
const FLOAT_ADD: u32 = 0x81e64;
const UNSIGNED_TO_DOUBLE: u32 = 0x82ed0;
const DOUBLE_MULTIPLY: u32 = 0x83018;
const DOUBLE_DIVIDE: u32 = 0x823b0;
const DOUBLE_ADD: u32 = 0x818a0;
const DOUBLE_TO_WIDE: u32 = 0x82cb4;
const WIDE_TO_DOUBLE: u32 = 0x82e9c;
const DOUBLE_TO_SIGNED_WIDE: u32 = 0x82c10;
const DOUBLE_GREATER: u32 = 0x81778;
const FLOAT_MULTIPLY: u32 = 0x83608;
const FLOAT_GREATER: u32 = 0x817b8;
const DIVIDE_WIDE: u32 = 0x8187c;
const FLOAT_LESS: u32 = 0x81818;
const DOUBLE_TO_UNSIGNED: u32 = 0x82d30;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct Code { at: usize, bytes: Vec<u8> }

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("adaptive rate address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || (self.at >= POOL_START) != data {
            return Err(format!("adaptive rate output kind or address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> { self.word(instruction, false) }
    fn data(&mut self, value: u32) -> Result<(), String> { self.word(value, true) }
    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
    fn numeric(&mut self, target: u32) -> Result<(), String> {
        self.emit(branch_exchange_immediate(self.at as u32, target)?)
    }
}

fn entry_and_direct_accumulation(c: &mut Code) -> Result<(), String> {
    c.check(0x6f738)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(11), r(13), 0x1c, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(13), r(13), 0x14, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(7), 0x700)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_FLOAT)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x9a8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x0)?)?;
    c.numeric(FLOAT_ADD)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(10), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(15), 0x98c)?)?;
    c.branch(C::Eq, 0x6f7a0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x2c4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Adc, r(1), r(1), 0x0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.branch(C::Al, 0x6fce4, false)?;
    Ok(())
}

fn mode_four_projection(c: &mut Code) -> Result<(), String> {
    c.check(0x6f7a0)?;
    c.emit(multiply_long(C::Al, false, r(1), r(2), r(0), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(2), Shift::Lsr, 2, false)?)?;
    c.branch(C::Ne, 0x6f8a4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0x420000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x10)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x40000000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_WIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(4), r(9), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(5), 0x0, true)?)?;
    c.branch(C::Cs, 0x6f9fc, false)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(8), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(5), 0x0, true)?)?;
    c.branch(C::Cs, 0x6f9fc, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.numeric(WIDE_TO_DOUBLE)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2b0)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x880)?)?;
    c.branch(C::Al, 0x6fb08, false)?;
    Ok(())
}

fn paired_rate_projection(c: &mut Code) -> Result<(), String> {
    c.check(0x6f8a4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 0x8)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x704)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(0), r(4), 0x22, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.branch(C::Hi, 0x6fa14, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x7ec)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(15), 0x7ec)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, DOUBLE_GREATER, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x6fa14, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x7d0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(15), 0x7d0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, DOUBLE_GREATER, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x6fa14, false)?;
    Ok(())
}

fn cached_float_and_selector_decrement(c: &mut Code) -> Result<(), String> {
    c.check(0x6f958)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(1), r(0), 0x3e800000, false)?)?;
    c.numeric(FLOAT_MULTIPLY)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x7a0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(1), r(0), 0x20c0000, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(1), r(1), 0x40000000, false)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(10), 0x0)?)?;
    c.branch(C::Al, FLOAT_GREATER, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x6fb14, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x790)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0xff000000, false)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(15), 0x790)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0x1, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x0)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(8), r(5), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 24, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Asr, 24, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(data_register(C::Gt, Op::Mov, r(3), r(0), r(1), Shift::Asr, 24, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x768)?)?;
    c.emit(transfer_word_register(C::Al, true, r(2), r(0), r(3), Shift::Lsl, 2)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 10, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0x0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(0), r(8), Shift::Lsr, 22, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 10, false)?)?;
    c.branch(C::Al, DIVIDE_WIDE, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(4), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fd34, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fd34, false)?;
    c.emit(transfer_multiple(C::Al, false, r(5), 0x50, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fb9c, false)?;
    Ok(())
}

fn mode_four_clamp(c: &mut Code) -> Result<(), String> {
    c.check(0x6f9fc)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(9), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(5), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fbf0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6fbfc, false)?;
    Ok(())
}

fn fallback_projection(c: &mut Code) -> Result<(), String> {
    c.check(0x6fa14)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0x420000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x40000000, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_WIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(4), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fcec, false)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fcec, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.numeric(WIDE_TO_DOUBLE)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2b0)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x61c)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x600)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.emit(transfer_multiple(C::Al, false, r(0), 0x50, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

fn selector_increment_and_wide_scale(c: &mut Code) -> Result<(), String> {
    c.check(0x6fb14)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(1), r(0), 0x2100000, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(1), r(1), 0x40000000, false)?)?;
    c.branch(C::Al, FLOAT_LESS, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x6fcdc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x5e8)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x1000000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0x4, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(15), 0x5e4)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 24, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Asr, 24, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x4)?)?;
    c.emit(data_register(C::Lt, Op::Mov, r(3), r(0), r(1), Shift::Asr, 24, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x5c4)?)?;
    c.emit(transfer_multiple(C::Al, true, r(8), 0x202, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word_register(C::Al, true, r(2), r(0), r(3), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0xc)?)?;
    c.emit(multiply_long(C::Al, false, r(3), r(7), r(1), r(2))?)?;
    c.emit(multiply_accumulate(C::Al, r(5), r(9), r(2), r(7))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(3), Shift::Lsr, 10, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(4), r(2), r(5), Shift::Lsl, 22, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(5), Shift::Lsr, 10, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(3), r(4), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Al, Op::Rsc, r(2), r(2), r(5), Shift::Lsr, 10, true)?)?;
    c.branch(C::Cs, 0x6fe20, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(2), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(2), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fe20, false)?;
    c.emit(transfer_multiple(C::Al, false, r(8), 0x50, BlockMode::IncrementAfter, false)?)?;
    Ok(())
}

fn selected_wide_conversion(c: &mut Code) -> Result<(), String> {
    c.check(0x6fb9c)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.numeric(WIDE_TO_DOUBLE)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2b0)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6fc5c, false)?;
    Ok(())
}

fn mode_four_clamped_conversion(c: &mut Code) -> Result<(), String> {
    c.check(0x6fbf0)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(4), r(8), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(5), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fc10, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x520)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 0xc)?)?;
    c.numeric(WIDE_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x2b0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    Ok(())
}

fn commit_quarter_rate_and_return(c: &mut Code) -> Result<(), String> {
    c.check(0x6fc90)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.numeric(WIDE_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 2, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_UNSIGNED)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x60)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0x0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(13), r(11), 0x1c, false)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn cold_projection_bounds(c: &mut Code) -> Result<(), String> {
    c.check(0x6fcec)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(4), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fde4, false)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x40c)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cc, 0x6fc90, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0x40c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.branch(C::Ne, 0x70008, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x0)?)?;
    c.branch(C::Al, 0x6fe04, false)?;
    Ok(())
}

fn cold_selector_decrement_bounds(c: &mut Code) -> Result<(), String> {
    c.check(0x6fd34)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(4), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6fecc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(0), r(6), 0x0, true)?)?;
    c.branch(C::Cc, 0x6fc90, false)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(8), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(9), Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x6ffe8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2b0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(6), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0x8)?)?;
    c.emit(transfer_multiple(C::Al, false, r(5), 0x50, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

fn cold_projection_lower_bound(c: &mut Code) -> Result<(), String> {
    c.check(0x6fde4)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x338)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(0), r(10), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.branch(C::Ne, 0x6ff5c, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x304)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(2), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(2), 0x2c4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(3), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Adc, r(1), r(1), 0x0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(2), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 0x2c4)?)?;
    c.branch(C::Al, 0x6fcdc, false)?;
    Ok(())
}

fn cold_selector_increment_upper_bound(c: &mut Code) -> Result<(), String> {
    c.check(0x6fe20)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(4), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(2), r(6), 0x0, true)?)?;
    c.branch(C::Cs, 0x6ffdc, false)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(5), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(2), r(6), 0x0, true)?)?;
    c.branch(C::Cc, 0x6fc90, false)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(1), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(9), Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x6ffe8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2b0)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(6), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.emit(transfer_multiple(C::Al, false, r(8), 0x60, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

fn cold_selector_decrement_lower_bound(c: &mut Code) -> Result<(), String> {
    c.check(0x6fecc)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Eor, r(0), r(8), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(9), Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x6ffe8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2b0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(6), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0xc)?)?;
    c.emit(transfer_multiple(C::Al, false, r(5), 0x50, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

fn cold_projection_lower_conversion(c: &mut Code) -> Result<(), String> {
    c.check(0x6ff5c)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2b0)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(6), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(10), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(10), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(15), 0x134)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

fn cold_selector_increment_lower_bound(c: &mut Code) -> Result<(), String> {
    c.check(0x6ffdc)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(9), Shift::Lsl, 0, true)?)?;
    c.branch(C::Ne, 0x70090, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x2c4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Adc, r(1), r(1), 0x0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.branch(C::Al, 0x6fcdc, false)?;
    Ok(())
}

fn cold_projection_upper_conversion(c: &mut Code) -> Result<(), String> {
    c.check(0x70008)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(transfer_word(C::Al, true, r(9), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x2b0)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(6), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(15), 0xa0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 0x2c4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(0), 0x50, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

fn cold_selector_increment_lower_conversion(c: &mut Code) -> Result<(), String> {
    c.check(0x70090)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2b0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(UNSIGNED_TO_DOUBLE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_DIVIDE)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_MULTIPLY)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(3), r(0), 0xfe00000, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(6), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(3), r(3), 0x30000000, false)?)?;
    c.numeric(DOUBLE_ADD)?;
    c.numeric(DOUBLE_TO_SIGNED_WIDE)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x2c4)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0xc)?)?;
    c.emit(transfer_multiple(C::Al, false, r(8), 0x50, BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0x6fc90, false)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    entry_and_direct_accumulation(&mut c)?;
    mode_four_projection(&mut c)?;
    paired_rate_projection(&mut c)?;
    cached_float_and_selector_decrement(&mut c)?;
    mode_four_clamp(&mut c)?;
    fallback_projection(&mut c)?;
    selector_increment_and_wide_scale(&mut c)?;
    selected_wide_conversion(&mut c)?;
    mode_four_clamped_conversion(&mut c)?;
    commit_quarter_rate_and_return(&mut c)?;
    cold_projection_bounds(&mut c)?;
    cold_selector_decrement_bounds(&mut c)?;
    cold_projection_lower_bound(&mut c)?;
    cold_selector_increment_upper_bound(&mut c)?;
    cold_selector_decrement_lower_bound(&mut c)?;
    cold_projection_lower_conversion(&mut c)?;
    cold_selector_increment_lower_bound(&mut c)?;
    cold_projection_upper_conversion(&mut c)?;
    cold_selector_increment_lower_conversion(&mut c)?;
    c.check(POOL_START)?;
    c.data(CACHED_FLOAT)?;
    c.data(DIVIDE_BY_FIVE_MAGIC)?;
    c.data(ONE_POINT_SIX as u32)?;
    c.data((ONE_POINT_SIX >> 32) as u32)?;
    c.data(SIGNED_SELECTOR)?;
    c.data(DIAGNOSTIC_TABLE)?;
    c.data(CACHED_WIDE)?;
    c.check(END)?;
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
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        for at in (START..END).step_by(4) {
            assert_eq!(word(&bytes, at), u32::from_le_bytes(
                reference[at + FILE_MAPPING_DELTA..at + FILE_MAPPING_DELTA + 4]
                    .try_into().unwrap()), "at {at:#x}");
        }
    }

    #[test]
    fn extent_and_named_pool() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2544);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        for (index, value) in [CACHED_FLOAT, DIVIDE_BY_FIVE_MAGIC,
            ONE_POINT_SIX as u32, (ONE_POINT_SIX >> 32) as u32,
            SIGNED_SELECTOR, DIAGNOSTIC_TABLE, CACHED_WIDE].into_iter().enumerate() {
            assert_eq!(word(&bytes, POOL_START + index * 4), value);
        }
        assert_eq!(ONE_POINT_SIX, 0x3ff999999999999a);
    }

    #[test]
    fn protected_code_and_data() {
        for at in (START..POOL_START).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(CACHED_FLOAT).is_err());
            assert!(c.bytes.is_empty());
            c.emit(arm32::mov_register(C::Al, r(0), r(1))).unwrap();
        }
        for at in (POOL_START..END).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(arm32::mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.bytes.is_empty());
            c.data(CACHED_FLOAT).unwrap();
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(arm32::mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.data(CACHED_FLOAT).is_err());
            assert!(c.bytes.is_empty());
        }
        assert!(Code { at: START, bytes: Vec::new() }.check(END).is_err());
    }

    #[test]
    fn direct_accumulation_return_and_cold_continuations() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6f774), data_immediate(
            C::Al, Op::Tst, r(0), r(6), 3).unwrap());
        assert_eq!(word(&bytes, 0x6f78c), data_register(
            C::Al, Op::Add, r(0), r(0), r(5), Shift::Lsl, 0, true).unwrap());
        assert_eq!(word(&bytes, 0x6f790), data_immediate_with_flags(
            C::Al, Op::Adc, r(1), r(1), 0, false).unwrap());
        assert_eq!(word(&bytes, 0x6f79c), branch(
            C::Al, 0x6f79c, 0x6fce4, false).unwrap());
        assert_eq!(word(&bytes, 0x6fce8), transfer_multiple(
            C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true).unwrap());
        for (at, target) in [(0x6fa70, 0x6fcec), (0x6f9e0, 0x6fd34),
            (0x6fb84, 0x6fe20)] {
            assert_eq!(word(&bytes, at), branch(C::Cs, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x70108), branch(
            C::Al, 0x70108, 0x6fc90, false).unwrap());
    }
}