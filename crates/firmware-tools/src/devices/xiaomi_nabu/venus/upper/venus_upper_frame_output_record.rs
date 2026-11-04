use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register,
    data_register_shifted_by_register, load_halfword, multiply_accumulate,
    multiply_long, multiply_register, signed_multiply_accumulate_bottom_halves,
    signed_multiply_bottom_halves, store_halfword, transfer_byte, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x70468;
pub const END: usize = 0x70d64;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2268;
pub const DATA_BYTES: usize = 32;
const POOL_START: usize = 0x70d44;
const RATE_CONTROL_SOURCE: u32 = 0xf2e4c;
const SCAN_INITIALIZED: u32 = 0x10b480;
const SCAN_WIDTH: u32 = 0x10b481;
const DIVIDE_BY_THREE_RECIPROCAL: u32 = 0xaaaaaaab;
const SCAN_HEIGHT: u32 = 0x10b482;
const DEFAULT_METRIC_SCALE: u32 = 0x00013333;
const DIVIDE_BY_HUNDRED_RECIPROCAL: u32 = 0x51eb851f;
const DIVIDE_BY_TEN_RECIPROCAL: u32 = 0xcccccccd;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("frame output record address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || (self.at >= POOL_START) != data
        {
            return Err(format!("frame output record kind or address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn data(&mut self, value: u32) -> Result<(), String> {
        self.word(value, true)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }

    fn literal(&mut self, condition: C, destination: R, location: usize) -> Result<(), String> {
        if !(POOL_START..END).contains(&location) || location % 4 != 0 {
            return Err(format!("invalid frame output literal {location:#x}"));
        }
        self.emit(transfer_word(condition, true, destination, R::PC,
            location as i32 - self.at as i32 - 8)?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.check(0x70468)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x24)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb30)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(1), r(5), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x704dc, false)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(5), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(6), r(0), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(0), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x70504, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x7057c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x70520, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(6), r(0), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(0), 0x3440)?)?;
    c.branch(C::Al, 0x7058c, false)?;
    c.check(0x704dc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(5), 0xa80)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x70528, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x70530, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x70538, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(5), 0x310)?)?;
    c.branch(C::Al, 0x7058c, false)?;
    c.check(0x70504)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.branch(C::Ne, 0x70554, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(6), r(0), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xf20)?)?;
    c.branch(C::Al, 0x70588, false)?;
    c.check(0x70520)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x87)?)?;
    c.branch(C::Al, 0x7053c, false)?;
    c.check(0x70528)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(5), 0x4e0)?)?;
    c.branch(C::Al, 0x7058c, false)?;
    c.check(0x70530)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(5), 0x3f8)?)?;
    c.branch(C::Al, 0x7058c, false)?;
    c.check(0x70538)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9f)?)?;
    c.check(0x7053c)?;
    c.literal(C::Al, r(2), 0x70d44)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1900)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x70554)?;
    c.literal(C::Al, r(2), 0x70d44)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1980)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(12), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.check(0x7057c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(6), r(0), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x9b0)?)?;
    c.check(0x70588)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(0), 0x3000)?)?;
    c.check(0x7058c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x14)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Cc, 0x705d4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2d0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(12), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(3), 0x9)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(5), 0x785)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x2d)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(3), 0x7)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Gt, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x705e4, false)?;
    c.check(0x705d4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(12), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x9)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(5), 0x785)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 0x7)?)?;
    c.check(0x705e4)?;
    c.emit(data_immediate(C::Al, Op::And, r(3), r(1), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(2), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 0x784)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(2), 0xff)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2d0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Cc, false, r(2), r(5), 0x2d0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(0), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Hi, false, r(1), r(5), 0x2d0)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7061c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1000)?)?;
    c.branch(C::Ne, 0x70808, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x2d8)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(13), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x8)?)?;
    c.emit(transfer_multiple(C::Al, false, r(1), r(4).bit() | r(6).bit() | r(12).bit(), BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Eq, 0x70668, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x7c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x70668, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x708e4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(transfer_byte(C::Eq, true, r(2), r(5), 0x2d2)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x708e4, false)?;
    c.check(0x70668)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(5), 0x9b8)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x38)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 0x9b9)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(5), 0x9ba)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x3c)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 0x9bb)?)?;
    c.emit(store_halfword(C::Al, r(0), r(13), 0x22)?)?;
    c.emit(store_halfword(C::Al, r(0), r(13), 0x20)?)?;
    c.emit(store_halfword(C::Al, r(0), r(13), 0x1e)?)?;
    c.check(0x70694)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x45)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x82)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x100)?)?;
    c.emit(store_halfword(C::Al, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x89)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x700)?)?;
    c.emit(load_halfword(C::Al, r(2), r(13), 0x22)?)?;
    c.emit(load_halfword(C::Al, r(3), r(13), 0x20)?)?;
    c.emit(load_halfword(C::Al, r(6), r(13), 0x1e)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(load_halfword(C::Al, r(0), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x706d4)?;
    c.emit(data_immediate(C::Al, Op::And, r(4), r(14), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x8)?)?;
    c.branch(C::Hi, 0x70754, false)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Tst, r(0), r(12), r(2), Shift::Lsl, r(4), false)?)?;
    c.branch(C::Ne, 0x70718, false)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Tst, r(0), r(10), r(2), Shift::Lsl, r(4), false)?)?;
    c.branch(C::Ne, 0x7073c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x28)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Tst, r(0), r(7), r(2), Shift::Lsl, r(4), false)?)?;
    c.branch(C::Eq, 0x70754, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x2)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(1), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(5), 0x2b)?)?;
    c.emit(transfer_byte(C::Al, true, r(9), r(5), 0x2a)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(4), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x70770, false)?;
    c.check(0x70718)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(1), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(8), r(5), 0x2b)?)?;
    c.emit(transfer_byte(C::Al, true, r(9), r(5), 0x2a)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(8), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(9), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x70778, false)?;
    c.check(0x7073c)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(5), 0x2b)?)?;
    c.emit(transfer_byte(C::Al, true, r(9), r(5), 0x2a)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(4), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x70770, false)?;
    c.check(0x70754)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x3)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(1), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(5), 0x2b)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x18)?)?;
    c.emit(transfer_byte(C::Al, true, r(9), r(5), 0x2a)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(4), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.check(0x70770)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(9), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.check(0x70778)?;
    c.emit(data_immediate(C::Al, Op::Add, r(14), r(14), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(9), r(1), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(14), 0x9)?)?;
    c.branch(C::Ne, 0x706d4, false)?;
    c.check(0x7078c)?;
    c.emit(transfer_word(C::Al, true, r(12), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(12), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0xa8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x26)?)?;
    c.check(0x707a4)?;
    c.emit(load_halfword(C::Al, r(3), r(1), -0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x6)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x0)?)?;
    c.emit(load_halfword(C::Al, r(3), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x4)?)?;
    c.branch(C::Ne, 0x707a4, false)?;
    c.emit(transfer_word(C::Al, true, r(14), r(13), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0xc)?)?;
    c.check(0x707d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x89)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0xdc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x700)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x707e4)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x9)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(1), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(1), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x707e4, false)?;
    c.check(0x70808)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x9b8)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xf6)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x9b9)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xf7)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x9ba)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xf8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x9bb)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xf9)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(5), 0x2e4)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(4), 0x8c)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x70880, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x70888, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x708ac, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x708c4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x70994, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(1), r(2), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x460)?)?;
    c.branch(C::Al, 0x708d8, false)?;
    c.check(0x70880)?;
    c.emit(transfer_word(C::Al, false, r(10), r(4), 0x90)?)?;
    c.branch(C::Al, 0x70a68, false)?;
    c.check(0x70888)?;
    c.literal(C::Al, r(1), 0x70d58)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x7097c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x70970, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x70990, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(5), 0x330)?)?;
    c.branch(C::Al, 0x70980, false)?;
    c.check(0x708ac)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(1), r(2), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3f40)?)?;
    c.branch(C::Al, 0x708dc, false)?;
    c.check(0x708c4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(1), r(2), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9d0)?)?;
    c.check(0x708d8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x3000)?)?;
    c.check(0x708dc)?;
    c.emit(transfer_word_register(C::Al, true, r(9), r(1), r(2), Shift::Lsl, 0)?)?;
    c.branch(C::Al, 0x70994, false)?;
    c.check(0x708e4)?;
    c.literal(C::Al, r(12), 0x70d48)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(12), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x70c20, false)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(5), 0x9b9)?)?;
    c.literal(C::Al, r(1), 0x70d4c)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(5), 0x9b8)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(2), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(5), 0x9b9)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x38)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, 0x70950, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.literal(C::Al, r(7), 0x70d54)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(5), 0x9b8)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 0x9b9)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(5), 0x9bb)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), r(5), 0x9ba)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(7), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(4), r(7), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(5), 0x9bb)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(5), 0x3c)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Ls, false, r(7), r(5), 0x9bb)?)?;
    c.emit(transfer_byte(C::Ls, false, r(12), r(5), 0x9ba)?)?;
    c.check(0x70950)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(3), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Cc, false, r(1), r(5), 0x9b9)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x3c)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x9bb)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Cc, false, r(1), r(5), 0x9bb)?)?;
    c.branch(C::Al, 0x70cf0, false)?;
    c.check(0x70970)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(5), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x400)?)?;
    c.branch(C::Al, 0x70980, false)?;
    c.check(0x7097c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(5), 0x500)?)?;
    c.check(0x70980)?;
    c.emit(transfer_word(C::Al, true, r(9), r(2), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.emit(data_register(C::Eq, Op::Mov, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x70994, false)?;
    c.check(0x70990)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x70994)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(store_halfword(C::Al, r(1), r(4), 0xfa)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x709e4, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(14), 0x1fd)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x70a4c, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x780)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x70a4c, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x70a4c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4)?)?;
    c.branch(C::Cs, 0x70a28, false)?;
    c.branch(C::Al, 0x70a50, false)?;
    c.check(0x709e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x14)?)?;
    c.emit(store_halfword(C::Al, r(1), r(4), 0xfa)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(14), 0x1fd)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x70a4c, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x780)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x70a18, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(14), 0x25c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x70a4c, false)?;
    c.check(0x70a18)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x70a50, false)?;
    c.check(0x70a28)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0xc)?)?;
    c.branch(C::Eq, 0x70bd0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x70bd0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x314)?)?;
    c.branch(C::Al, 0x70bd8, false)?;
    c.check(0x70a4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x70a50)?;
    c.emit(multiply_long(C::Al, false, r(0), r(1), r(10), r(9))?)?;
    c.emit(multiply_accumulate(C::Al, r(2), r(7), r(9), r(1))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 16, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x90)?)?;
    c.check(0x70a64)?;
    c.emit(transfer_word(C::Al, false, r(9), r(4), 0xfc)?)?;
    c.check(0x70a68)?;
    c.emit(transfer_word(C::Al, true, r(0), r(12), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x9)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 0xd9)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x7)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 0xd8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x9c)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x785)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xdb)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x784)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xda)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x7a0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0xa4)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Eq, Op::Mov, r(0), r(0), r(10), Shift::Lsr, 2, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x98)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x788)?)?;
    c.emit(multiply_register(C::Al, r(1), r(10), r(0))?)?;
    c.literal(C::Al, r(0), 0x70d5c)?;
    c.emit(multiply_long(C::Al, false, r(2), r(3), r(1), r(0))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(3), Shift::Lsr, 5, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x94)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2d0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(14), 0x838)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(transfer_byte(C::Eq, true, r(1), r(5), 0x778)?)?;
    c.emit(transfer_byte(C::Eq, false, r(1), r(4), 0x5)?)?;
    c.check(0x70ae0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x5c8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x264)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x5d0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x268)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0xe1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x70b14, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x8)?)?;
    c.branch(C::Al, 0x70b28, false)?;
    c.check(0x70b14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x4)?)?;
    c.check(0x70b28)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(4), 0x104)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(4), 0x105)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(4), 0x106)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 0x107)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 0x100)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x101)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x102)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 0x103)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(14), 0x838)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x70bc8, false)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(5), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(14), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(5), 0x78)?)?;
    c.emit(multiply_register(C::Al, r(6), r(3), r(2))?)?;
    c.emit(multiply_register(C::Al, r(1), r(6), r(0))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 6, false)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x70bb4, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x70bb4, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x2f0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xd)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x2ec)?)?;
    c.emit(multiply_register(C::Al, r(6), r(2), r(3))?)?;
    c.literal(C::Al, r(2), 0x70d60)?;
    c.emit(multiply_long(C::Al, false, r(3), r(5), r(6), r(2))?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(5), Shift::Lsr, 3, false)?)?;
    c.branch(C::Cc, 0x70bc0, false)?;
    c.check(0x70bb4)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(7), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 6, false)?)?;
    c.branch(C::Cs, 0x70bc8, false)?;
    c.check(0x70bc0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x100)?)?;
    c.check(0x70bc8)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x70bcc)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x70bd0)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x3fc)?)?;
    c.check(0x70bd8)?;
    c.emit(transfer_word(C::Al, true, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(5), 0x3c)?)?;
    c.emit(multiply_register(C::Al, r(1), r(3), r(0))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(14), 0x838)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x70d0c, false)?;
    c.emit(store_halfword(C::Al, r(0), r(4), 0xfa)?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(1), r(10), r(9))?)?;
    c.emit(multiply_accumulate(C::Al, r(2), r(7), r(9), r(1))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 16, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(2), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x90)?)?;
    c.branch(C::Al, 0x70d38, false)?;
    c.check(0x70c20)?;
    c.emit(transfer_word(C::Al, true, r(10), r(5), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7b)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xcd)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(5), 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.literal(C::Al, r(14), 0x70d54)?;
    c.literal(C::Al, r(8), 0x70d4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0xc00)?)?;
    c.emit(data_register(C::Eq, Op::Mov, r(7), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(multiply_register(C::Al, r(1), r(10), r(7))?)?;
    c.emit(multiply_register(C::Al, r(7), r(1), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xff00)?)?;
    c.emit(data_register(C::Al, Op::And, r(9), r(1), r(7), Shift::Lsr, 15, false)?)?;
    c.check(0x70c60)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(6), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x70c84, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(14), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(transfer_byte(C::Hi, false, r(1), r(14), 0x0)?)?;
    c.branch(C::Al, 0x70cb4, false)?;
    c.check(0x70c84)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(10), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(transfer_byte(C::Hi, false, r(1), r(8), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0x1)?)?;
    c.literal(C::Al, r(4), 0x70d50)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0xff)?)?;
    c.emit(multiply_long(C::Al, false, r(6), r(7), r(1), r(4))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(7), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(4), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(6), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(14), 0x0)?)?;
    c.check(0x70cb4)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(8), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(4), r(1), 0xff)?)?;
    c.emit(signed_multiply_bottom_halves(C::Al, r(2), r(7), r(4))?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x70cd4, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Eq, Op::Cmp, r(0), r(10), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x70c60, false)?;
    c.check(0x70cd4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 0x9b8)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(5), 0x9b9)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(5), 0x9ba)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(5), 0x9bb)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(12), 0x0)?)?;
    c.check(0x70cf0)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3)?)?;
    c.emit(store_halfword(C::Al, r(0), r(13), 0x20)?)?;
    c.emit(store_halfword(C::Al, r(0), r(13), 0x1e)?)?;
    c.emit(store_halfword(C::Al, r(0), r(13), 0x1c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.branch(C::Al, 0x70694, false)?;
    c.check(0x70d0c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(10), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Sub, r(1), r(10), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(multiply_long(C::Al, false, r(2), r(3), r(1), r(9))?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(7), r(9), r(3))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsr, 16, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x90)?)?;
    c.emit(store_halfword(C::Al, r(0), r(4), 0xfa)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(4), 0xfc)?)?;
    c.check(0x70d38)?;
    c.emit(transfer_word(C::Al, true, r(12), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0xc)?)?;
    c.check(0x70d40)?;
    c.branch(C::Al, 0x70a64, false)?;
    c.check(POOL_START)?;
    c.data(RATE_CONTROL_SOURCE)?;
    c.data(SCAN_INITIALIZED)?;
    c.data(SCAN_WIDTH)?;
    c.data(DIVIDE_BY_THREE_RECIPROCAL)?;
    c.data(SCAN_HEIGHT)?;
    c.data(DEFAULT_METRIC_SCALE)?;
    c.data(DIVIDE_BY_HUNDRED_RECIPROCAL)?;
    c.data(DIVIDE_BY_TEN_RECIPROCAL)?;
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
        assert_eq!(fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn extent_and_named_pool() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2300);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        for (at, value) in [
            (0x70d44, RATE_CONTROL_SOURCE), (0x70d48, SCAN_INITIALIZED),
            (0x70d4c, SCAN_WIDTH), (0x70d50, DIVIDE_BY_THREE_RECIPROCAL),
            (0x70d54, SCAN_HEIGHT), (0x70d58, DEFAULT_METRIC_SCALE),
            (0x70d5c, DIVIDE_BY_HUNDRED_RECIPROCAL),
            (0x70d60, DIVIDE_BY_TEN_RECIPROCAL),
        ] {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn checked_code_data_and_literal_domains() {
        for at in (START..POOL_START).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(RATE_CONTROL_SOURCE).is_err());
            assert!(c.bytes.is_empty());
            c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap()).unwrap();
        }
        for at in (POOL_START..END).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap()).is_err());
            assert!(c.bytes.is_empty());
            c.data(RATE_CONTROL_SOURCE).unwrap();
        }
        for at in [START - 4, START + 1, POOL_START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap()).is_err());
            assert!(c.data(RATE_CONTROL_SOURCE).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        for location in [START, POOL_START - 4, POOL_START + 1, END] {
            assert!(c.literal(C::Al, r(0), location).is_err());
            assert!(c.bytes.is_empty());
        }
        c.literal(C::Al, r(0), POOL_START).unwrap();
    }

    #[test]
    fn inline_indexed_state_qp_and_tag_edges() {
        let bytes = fragment().unwrap();
        for (at, condition, target) in [
            (0x70494, C::Eq, 0x704dc), (0x704b8, C::Eq, 0x70504),
            (0x704c0, C::Eq, 0x7057c), (0x704c8, C::Ne, 0x70520),
            (0x704e8, C::Eq, 0x70528), (0x704f0, C::Eq, 0x70530),
            (0x704f8, C::Ne, 0x70538), (0x7050c, C::Ne, 0x70554),
            (0x70598, C::Cc, 0x705d4), (0x70624, C::Ne, 0x70808),
            (0x70654, C::Eq, 0x708e4), (0x70664, C::Eq, 0x708e4),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x70508), data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000).unwrap());
        assert_eq!(word(&bytes, 0x70620), data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1000).unwrap());
        for (at, condition, source, offset) in [
            (0x705b8, C::Al, 1, 0x785), (0x705f4, C::Al, 2, 0x784),
            (0x70608, C::Cc, 2, 0x2d0), (0x70614, C::Hi, 1, 0x2d0),
        ] {
            assert_eq!(word(&bytes, at), transfer_byte(condition, false, r(source), r(5), offset).unwrap());
        }
    }

    #[test]
    fn nine_categories_six_bound_pairs_and_scan_outputs() {
        let bytes = fragment().unwrap();
        for (at, count, base) in [(0x70784, 9, 14), (0x707ac, 6, 2), (0x707ec, 9, 2)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Cmp, r(0), r(base), count).unwrap());
        }
        for (at, target) in [(0x70788, 0x706d4), (0x707c4, 0x707a4), (0x70804, 0x707e4)] {
            assert_eq!(word(&bytes, at), branch(C::Ne, at as u32, target, false).unwrap());
        }
        for (at, base) in [(0x706e0, 12), (0x706e8, 10), (0x706f4, 7)] {
            assert_eq!(word(&bytes, at), data_register_shifted_by_register(
                C::Al, Op::Tst, r(0), r(base), r(2), Shift::Lsl, r(4), false).unwrap());
        }
        for index in 0..4 {
            let at = 0x70808 + index * 8;
            assert_eq!(word(&bytes, at), transfer_byte(C::Al, true, r(0), r(5), (0x9b8 + index) as i32).unwrap());
            assert_eq!(word(&bytes, at + 4), transfer_byte(C::Al, false, r(0), r(4), (0xf6 + index) as i32).unwrap());
        }
        assert_eq!(word(&bytes, 0x707a4), load_halfword(C::Al, r(3), r(1), -0x18).unwrap());
        assert_eq!(word(&bytes, 0x707b4), load_halfword(C::Al, r(3), r(1), 0).unwrap());
    }

    #[test]
    fn scaled_metrics_descriptor_and_literal_references() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x70a50), multiply_long(C::Al, false, r(0), r(1), r(10), r(9)).unwrap());
        assert_eq!(word(&bytes, 0x70a54), multiply_accumulate(C::Al, r(2), r(7), r(9), r(1)).unwrap());
        assert_eq!(word(&bytes, 0x70ab8), multiply_long(C::Al, false, r(2), r(3), r(1), r(0)).unwrap());
        assert_eq!(word(&bytes, 0x70ae4), transfer_word(C::Al, false, r(1), r(4), 0x264).unwrap());
        assert_eq!(word(&bytes, 0x70aec), transfer_word(C::Al, false, r(1), r(4), 0x268).unwrap());
        for (at, destination, location) in [
            (0x7053c, 2, 0x70d44), (0x70554, 2, 0x70d44),
            (0x70888, 1, 0x70d58), (0x708e4, 12, 0x70d48),
            (0x708f8, 1, 0x70d4c), (0x7091c, 7, 0x70d54),
            (0x70ab4, 0, 0x70d5c), (0x70ba4, 2, 0x70d60),
            (0x70c34, 14, 0x70d54), (0x70c38, 8, 0x70d4c),
            (0x70c98, 4, 0x70d50),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(destination), R::PC,
                location as i32 - at as i32 - 8).unwrap());
        }
    }

    #[test]
    fn main_return_does_not_truncate_cold_blocks() {
        let bytes = fragment().unwrap();
        let saved = (4..=11).fold(0, |mask, n| mask | r(n).bit());
        assert_eq!(word(&bytes, 0x70bcc), transfer_multiple(C::Al, true, R::SP,
            saved | R::PC.bit(), BlockMode::IncrementAfter, true).unwrap());
        for (at, condition, target) in [
            (0x70a34, C::Eq, 0x70bd0), (0x70a3c, C::Ne, 0x70bd0),
            (0x708f0, C::Eq, 0x70c20), (0x7096c, C::Al, 0x70cf0),
            (0x70c00, C::Eq, 0x70d0c), (0x70c1c, C::Al, 0x70d38),
            (0x70cd0, C::Ne, 0x70c60), (0x70d08, C::Al, 0x70694),
            (0x70d40, C::Al, 0x70a64),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        for at in [0x70b78, 0x70bec] {
            assert_eq!(word(&bytes, at), branch_exchange_immediate(at as u32, 0x83c4c).unwrap());
        }
        assert_eq!(word(&bytes, 0x70550), branch(C::Al, 0x70550, 0x17b0, false).unwrap());
        assert_eq!(word(&bytes, 0x7056c), branch(C::Al, 0x7056c, 0x17b0, true).unwrap());
    }
}
