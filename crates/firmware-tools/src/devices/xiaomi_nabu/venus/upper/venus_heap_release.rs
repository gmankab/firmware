use arm32::{
    branch, branch_exchange_with_link, bx, data_immediate, data_register,
    load_halfword, load_halfword_register_preindexed_writeback, multiply_long,
    store_halfword, transfer_byte, transfer_halfword_register, transfer_multiple,
    transfer_word, transfer_word_register, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};

pub const START: usize = 0xbe70;
pub const END: usize = 0xc4cc;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1604;
pub const DATA_BYTES: usize = 24;
const MEMHEAP_SOURCE: u32 = 0x000e_37d7;
const MARKER_TABLE: u32 = 0x0010_3ae8;
const CALLBACK: u32 = 0x0000_d2a8;
const RING_INDEX: u32 = 0x0010_3bf8;
const DIAGNOSTIC_RING: u32 = 0x0010_3ba8;
const DIVIDE_BY_TWENTY: u32 = 0x6666_6667;

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
            return Err(format!("heap release at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !(START..0xc4b4).contains(&at) || at % 4 != 0 {
            return Err(format!("heap release instruction outside code: {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        let expected = match at {
            0xc4b4 => MEMHEAP_SOURCE,
            0xc4b8 => MARKER_TABLE,
            0xc4bc => CALLBACK,
            0xc4c0 => RING_INDEX,
            0xc4c4 => DIAGNOSTIC_RING,
            0xc4c8 => DIVIDE_BY_TWENTY,
            _ => return Err(format!("heap release data outside pool: {at:#x}")),
        };
        if value != expected {
            return Err(format!("heap release invalid data at {at:#x}: {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn jump(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }

    fn literal(&mut self, at: usize, condition: C, destination: R, target: usize) -> Result<(), String> {
        self.emit(at, transfer_word(condition, true, destination, R::PC,
            target as i32 - at as i32 - 8)?)
    }
}

fn validate_heap_and_record_diagnostic(c: &mut Code) -> Result<(), String> {
    c.emit(0xbe70, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xbe74, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x18)?)?;
    c.emit(0xbe78, data_register(C::Al, Op::Mov, r(4), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xbe7c, data_register(C::Al, Op::Mov, r(5), R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xbe80, data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.jump(0xbe84, C::Ne, 0xbea0, false)?;
    c.jump(0xbe88, C::Al, 0x86644, true)?;
    c.literal(0xbe8c, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xbe90, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2cc)?)?;
    c.emit(0xbe94, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xbe98, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    c.jump(0xbe9c, C::Al, 0x17b0, true)?;
    c.emit(0xbea0, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x238)?)?;
    c.emit(0xbea4, data_register(C::Al, Op::Mov, r(7), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.literal(0xbea8, C::Al, r(9), 0xc4b8)?;
    c.emit(0xbeac, transfer_word(C::Al, true, R::R1, r(4), 0)?)?;
    c.emit(0xbeb0, load_halfword_register_preindexed_writeback(C::Al, R::R0, r(7), R::R0)?)?;
    c.emit(0xbeb4, transfer_word_register(C::Al, true, R::R0, r(9), R::R0, Shift::Lsl, 2)?)?;
    c.emit(0xbeb8, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.jump(0xbebc, C::Eq, 0xbed8, false)?;
    c.jump(0xbec0, C::Al, 0x86644, true)?;
    c.literal(0xbec4, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xbec8, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xcf)?)?;
    c.emit(0xbecc, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xbed0, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xbed4, C::Al, 0x17b0, true)?;
    c.jump(0xbed8, C::Al, 0x86670, true)?;
    c.emit(0xbedc, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.jump(0xbee0, C::Eq, 0xbf0c, false)?;
    c.emit(0xbee4, transfer_word(C::Al, true, R::R0, r(4), 36)?)?;
    c.emit(0xbee8, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.literal(0xbeec, C::Ne, R::R1, 0xc4bc)?;
    c.emit(0xbef0, data_register(C::Ne, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.jump(0xbef4, C::Eq, 0xbf0c, false)?;
    c.jump(0xbef8, C::Al, 0x86644, true)?;
    c.literal(0xbefc, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xbf00, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xbf04, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x6d0)?)?;
    c.jump(0xbf08, C::Al, 0x17b0, true)?;
    c.emit(0xbf0c, data_immediate(C::Al, Op::Cmp, R::R0, r(5), 0x0)?)?;
    c.jump(0xbf10, C::Ne, 0xbf88, false)?;
    c.emit(0xbf14, transfer_word(C::Al, true, R::R1, r(4), 36)?)?;
    c.emit(0xbf18, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.jump(0xbf1c, C::Eq, 0xbf28, false)?;
    c.emit(0xbf20, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xbf24, branch_exchange_with_link(C::Al, R::R1)?)?;
    c.jump(0xbf28, C::Al, 0x8e88, true)?;
    c.literal(0xbf2c, C::Al, R::R1, 0xc4c0)?;
    c.literal(0xbf30, C::Al, r(3), 0xc4c4)?;
    c.emit(0xbf34, transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    c.emit(0xbf38, transfer_word_register(C::Al, false, R::R0, r(3), R::R2, Shift::Lsl, 2)?)?;
    c.emit(0xbf3c, data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x1)?)?;
    c.literal(0xbf40, C::Al, R::R2, 0xc4c8)?;
    c.emit(0xbf44, multiply_long(C::Al, true, r(3), r(6), R::R0, R::R2)?)?;
    c.emit(0xbf48, data_register(C::Al, Op::Mov, R::R2, R::R0, r(6), Shift::Asr, 3, false)?)?;
    c.emit(0xbf4c, data_register(C::Al, Op::Add, R::R2, R::R2, r(6), Shift::Lsr, 31, false)?)?;
    c.emit(0xbf50, data_register(C::Al, Op::Add, R::R2, R::R2, R::R2, Shift::Lsl, 2, false)?)?;
    c.emit(0xbf54, data_register(C::Al, Op::Sub, R::R0, R::R0, R::R2, Shift::Lsl, 2, false)?)?;
    c.emit(0xbf58, transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    c.emit(0xbf5c, transfer_word(C::Al, true, R::R1, r(4), 40)?)?;
    c.emit(0xbf60, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.jump(0xbf64, C::Eq, 0xbf70, false)?;
    c.emit(0xbf68, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xbf6c, branch_exchange_with_link(C::Al, R::R1)?)?;
    c.jump(0xbf70, C::Al, 0x86644, true)?;
    c.literal(0xbf74, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xbf78, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xde)?)?;
    c.emit(0xbf7c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xbf80, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xbf84, C::Al, 0x17b0, true)?;
    c.jump(0xbf88, C::Al, 0x86670, true)?;
    c.emit(0xbf8c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xbf90, transfer_word(C::Ne, true, R::R0, r(4), 36)?)?;
    c.emit(0xbf94, data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.jump(0xbf98, C::Eq, 0xbfa8, false)?;
    c.literal(0xbf9c, C::Al, R::R1, 0xc4bc)?;
    c.emit(0xbfa0, data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xbfa4, transfer_multiple(C::Ne, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xbfa8)
}

fn validate_block_and_update_accounting(c: &mut Code) -> Result<(), String> {
    c.emit(0xbfa8, transfer_word(C::Al, true, R::R0, r(4), 4)?)?;
    c.emit(0xbfac, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.jump(0xbfb0, C::Ne, 0xbfcc, false)?;
    c.jump(0xbfb4, C::Al, 0x86644, true)?;
    c.literal(0xbfb8, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xbfbc, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe6)?)?;
    c.emit(0xbfc0, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xbfc4, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xbfc8, C::Al, 0x17b0, true)?;
    c.emit(0xbfcc, transfer_word(C::Al, true, R::R1, r(4), 36)?)?;
    c.emit(0xbfd0, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.jump(0xbfd4, C::Eq, 0xbfe0, false)?;
    c.emit(0xbfd8, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xbfdc, branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(0xbfe0, transfer_word(C::Al, true, R::R0, r(4), 4)?)?;
    c.emit(0xbfe4, data_immediate(C::Al, Op::Sub, r(6), r(5), 0x8)?)?;
    c.emit(0xbfe8, data_register(C::Al, Op::Cmp, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.jump(0xbfec, C::Hi, 0xc000, false)?;
    c.emit(0xbff0, transfer_word(C::Al, true, R::R1, r(4), 16)?)?;
    c.emit(0xbff4, data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 3, false)?)?;
    c.emit(0xbff8, data_register(C::Al, Op::Cmp, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.jump(0xbffc, C::Hi, 0xc018, false)?;
    c.jump(0xc000, C::Al, 0x86644, true)?;
    c.literal(0xc004, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc008, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf5)?)?;
    c.emit(0xc00c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc010, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xc014, C::Al, 0x17b0, true)?;
    c.emit(0xc018, transfer_byte(C::Al, true, R::R0, r(5), -5)?)?;
    c.emit(0xc01c, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0xf)?)?;
    c.jump(0xc020, C::Eq, 0xc03c, false)?;
    c.jump(0xc024, C::Al, 0x86644, true)?;
    c.literal(0xc028, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc02c, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xfa)?)?;
    c.emit(0xc030, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc034, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xc038, C::Al, 0x17b0, true)?;
    c.emit(0xc03c, transfer_word(C::Al, true, R::R0, r(5), -4)?)?;
    c.emit(0xc040, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0xb)?)?;
    c.jump(0xc044, C::Hi, 0xc060, false)?;
    c.jump(0xc048, C::Al, 0x86644, true)?;
    c.literal(0xc04c, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc050, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xfd)?)?;
    c.emit(0xc054, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc058, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xc05c, C::Al, 0x17b0, true)?;
    c.emit(0xc060, transfer_byte(C::Al, true, R::R0, r(5), -6)?)?;
    c.emit(0xc064, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x10)?)?;
    c.jump(0xc068, C::Cc, 0xc084, false)?;
    c.jump(0xc06c, C::Al, 0x86644, true)?;
    c.literal(0xc070, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc074, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xff)?)?;
    c.emit(0xc078, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc07c, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    c.jump(0xc080, C::Al, 0x17b0, true)?;
    c.emit(0xc084, transfer_word(C::Al, true, R::R0, r(5), -4)?)?;
    c.emit(0xc088, transfer_word(C::Al, true, R::R1, r(4), 4)?)?;
    c.emit(0xc08c, transfer_word(C::Al, true, R::R2, r(4), 16)?)?;
    c.emit(0xc090, data_register(C::Al, Op::Add, R::R0, r(6), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc094, data_register(C::Al, Op::Add, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc098, data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.jump(0xc09c, C::Ls, 0xc0b8, false)?;
    c.jump(0xc0a0, C::Al, 0x86644, true)?;
    c.literal(0xc0a4, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc0a8, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2)?)?;
    c.emit(0xc0ac, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc0b0, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    c.jump(0xc0b4, C::Al, 0x17b0, true)?;
    c.emit(0xc0b8, transfer_byte(C::Al, true, R::R0, r(5), -5)?)?;
    c.emit(0xc0bc, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0xf)?)?;
    c.jump(0xc0c0, C::Ne, 0xc3a4, false)?;
    c.emit(0xc0c4, transfer_word(C::Al, true, R::R2, r(5), -4)?)?;
    c.emit(0xc0c8, transfer_byte(C::Al, true, R::R0, r(5), -6)?)?;
    c.emit(0xc0cc, data_immediate(C::Al, Op::Sub, R::R1, R::R2, 0x8)?)?;
    c.emit(0xc0d0, data_register(C::Al, Op::Sub, r(3), R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc0d4, transfer_word(C::Al, true, R::R1, r(4), 20)?)?;
    c.emit(0xc0d8, data_register(C::Al, Op::Cmp, R::R0, r(3), R::R1, Shift::Lsl, 0, false)?)?;
    c.jump(0xc0dc, C::Ls, 0xc104, false)?;
    c.jump(0xc0e0, C::Al, 0x86644, true)?;
    c.literal(0xc0e4, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc0e8, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x30c)?)?;
    c.emit(0xc0ec, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc0f0, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    c.jump(0xc0f4, C::Al, 0x17b0, true)?;
    c.emit(0xc0f8, transfer_word(C::Al, true, R::R1, r(4), 20)?)?;
    c.emit(0xc0fc, transfer_byte(C::Al, true, R::R0, r(5), -6)?)?;
    c.emit(0xc100, transfer_word(C::Al, true, R::R2, r(5), -4)?)?;
    c.emit(0xc104, data_immediate(C::Al, Op::Rsb, R::R2, R::R2, 0x8)?)?;
    c.emit(0xc108, data_register(C::Al, Op::Add, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc10c, data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc110, transfer_word(C::Al, false, R::R0, r(4), 20)?)?;
    c.emit(0xc114, transfer_word(C::Al, true, R::R1, r(4), 16)?)?;
    c.emit(0xc118, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.jump(0xc11c, C::Cs, 0xc138, false)?;
    c.jump(0xc120, C::Al, 0x86644, true)?;
    c.literal(0xc124, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc128, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf)?)?;
    c.emit(0xc12c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc130, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    c.jump(0xc134, C::Al, 0x17b0, true)?;
    c.emit(0xc138, transfer_byte(C::Al, true, R::R0, r(5), -5)?)?;
    c.emit(0xc13c, data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf0)?)?;
    c.emit(0xc140, data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1)?)?;
    c.emit(0xc144, transfer_byte(C::Al, false, R::R0, r(5), -5)?)?;
    c.emit(0xc148, transfer_word(C::Al, true, R::R1, r(4), 4)?)?;
    c.emit(0xc14c, data_register(C::Al, Op::Cmp, R::R0, R::R1, r(6), Shift::Lsl, 0, false)?)?;
    c.jump(0xc150, C::Eq, 0xc160, false)?;
    c.emit(0xc154, transfer_word(C::Al, true, R::R0, r(5), -12)?)?;
    c.emit(0xc158, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0xf)?)?;
    c.jump(0xc15c, C::Eq, 0xc3c0, false)?;
    c.check(0xc160)
}

fn coalesce_forward(c: &mut Code) -> Result<(), String> {
    c.emit(0xc160, transfer_byte(C::Al, true, R::R0, r(6), 3)?)?;
    c.emit(0xc164, data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf0)?)?;
    c.emit(0xc168, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x10)?)?;
    c.jump(0xc16c, C::Eq, 0xc258, false)?;
    c.emit(0xc170, transfer_word(C::Al, true, R::R0, r(6), 4)?)?;
    c.emit(0xc174, transfer_word(C::Al, true, R::R1, r(4), 16)?)?;
    c.emit(0xc178, data_register(C::Al, Op::Add, r(5), r(6), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc17c, transfer_word(C::Al, true, R::R0, r(4), 4)?)?;
    c.emit(0xc180, data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc184, data_register(C::Al, Op::Cmp, R::R0, r(5), R::R0, Shift::Lsl, 0, false)?)?;
    c.jump(0xc188, C::Cs, 0xc258, false)?;
    c.emit(0xc18c, transfer_byte(C::Al, true, R::R0, r(5), 3)?)?;
    c.emit(0xc190, data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf)?)?;
    c.emit(0xc194, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.jump(0xc198, C::Ne, 0xc258, false)?;
    c.emit(0xc19c, load_halfword(C::Al, R::R1, r(5), 0)?)?;
    c.emit(0xc1a0, load_halfword(C::Al, R::R2, r(5), 2)?)?;
    c.emit(0xc1a4, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x234)?)?;
    c.emit(0xc1a8, transfer_halfword_register(C::Al, true, R::R0, r(4), R::R0)?)?;
    c.emit(0xc1ac, data_register(C::Al, Op::Eor, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc1b0, load_halfword(C::Al, R::R2, r(5), 4)?)?;
    c.emit(0xc1b4, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc1b8, load_halfword(C::Al, R::R2, r(5), 6)?)?;
    c.emit(0xc1bc, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc1c0, load_halfword(C::Al, R::R2, r(5), 8)?)?;
    c.emit(0xc1c4, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc1c8, load_halfword(C::Al, R::R2, r(5), 10)?)?;
    c.emit(0xc1cc, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc1d0, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xff)?)?;
    c.emit(0xc1d4, data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xff00)?)?;
    c.emit(0xc1d8, data_register(C::Al, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc1dc, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.jump(0xc1e0, C::Eq, 0xc1fc, false)?;
    c.jump(0xc1e4, C::Al, 0x86644, true)?;
    c.literal(0xc1e8, C::Al, R::R2, 0xc4b4)?;
    c.emit(0xc1ec, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x55)?)?;
    c.emit(0xc1f0, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc1f4, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    c.jump(0xc1f8, C::Al, 0x17b0, true)?;
    c.emit(0xc1fc, transfer_byte(C::Al, true, R::R0, r(4), 560)?)?;
    c.emit(0xc200, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.jump(0xc204, C::Eq, 0xc218, false)?;
    c.emit(0xc208, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc20c, data_register(C::Al, Op::Mov, R::R1, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc210, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    c.jump(0xc214, C::Al, 0xb884, true)?;
    c.emit(0xc218, transfer_word(C::Al, true, R::R0, r(5), 4)?)?;
    c.emit(0xc21c, transfer_word(C::Al, true, R::R1, r(6), 4)?)?;
    c.emit(0xc220, data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc224, transfer_word(C::Al, false, R::R0, r(6), 4)?)?;
    c.emit(0xc228, transfer_byte(C::Al, true, R::R0, r(5), 3)?)?;
    c.emit(0xc22c, transfer_byte(C::Al, true, R::R1, r(6), 3)?)?;
    c.emit(0xc230, data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf0)?)?;
    c.emit(0xc234, data_immediate(C::Al, Op::And, R::R1, R::R1, 0xf)?)?;
    c.emit(0xc238, data_register(C::Al, Op::Orr, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc23c, transfer_byte(C::Al, false, R::R0, r(6), 3)?)?;
    c.emit(0xc240, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc244, transfer_word(C::Al, false, R::R0, r(5), 0)?)?;
    c.emit(0xc248, transfer_word(C::Al, false, R::R0, r(5), 4)?)?;
    c.emit(0xc24c, transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(0xc250, data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x1)?)?;
    c.emit(0xc254, transfer_word(C::Al, false, R::R0, r(4), 12)?)?;
    c.emit(0xc258, transfer_word(C::Al, true, R::R0, r(4), 8)?)?;
    c.emit(0xc25c, data_register(C::Al, Op::Cmp, R::R0, r(6), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc260, transfer_word(C::Cc, false, r(6), r(4), 8)?)?;
    c.emit(0xc264, transfer_byte(C::Al, true, R::R0, r(4), 560)?)?;
    c.emit(0xc268, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.jump(0xc26c, C::Eq, 0xc27c, false)?;
    c.emit(0xc270, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc274, data_register(C::Al, Op::Mov, R::R1, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.jump(0xc278, C::Al, 0xaba0, true)?;
    c.check(0xc27c)
}

fn rebuild_checksum_and_empty_heap(c: &mut Code) -> Result<(), String> {
    c.emit(0xc27c, load_halfword(C::Al, R::R1, r(6), 2)?)?;
    c.emit(0xc280, load_halfword(C::Al, R::R2, r(6), 4)?)?;
    c.emit(0xc284, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x234)?)?;
    c.emit(0xc288, data_register(C::Al, Op::Mov, r(5), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc28c, load_halfword_register_preindexed_writeback(C::Al, R::R0, r(5), R::R0)?)?;
    c.emit(0xc290, data_register(C::Al, Op::Eor, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc294, load_halfword(C::Al, R::R2, r(6), 6)?)?;
    c.emit(0xc298, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc29c, load_halfword(C::Al, R::R2, r(6), 8)?)?;
    c.emit(0xc2a0, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc2a4, load_halfword(C::Al, R::R2, r(6), 10)?)?;
    c.emit(0xc2a8, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc2ac, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x10)?)?;
    c.emit(0xc2b0, data_register(C::Al, Op::Eor, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc2b4, store_halfword(C::Al, R::R0, r(6), 0)?)?;
    c.emit(0xc2b8, transfer_word(C::Al, true, R::R1, r(6), 4)?)?;
    c.emit(0xc2bc, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x10)?)?;
    c.emit(0xc2c0, data_register(C::Al, Op::Add, R::R0, r(6), R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc2c4, load_halfword(C::Ne, R::R2, r(7), 0)?)?;
    c.emit(0xc2c8, data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x8)?)?;
    c.emit(0xc2cc, transfer_word_register(C::Ne, true, R::R2, r(9), R::R2, Shift::Lsl, 2)?)?;
    c.emit(0xc2d0, transfer_word(C::Ne, false, R::R2, R::R0, 0)?)?;
    c.emit(0xc2d4, data_register(C::Ne, Op::Mov, R::R2, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc2d8, transfer_word(C::Al, false, R::R2, R::R0, 4)?)?;
    c.emit(0xc2dc, transfer_word(C::Al, true, R::R0, r(4), 20)?)?;
    c.emit(0xc2e0, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.jump(0xc2e4, C::Ne, 0xc3a4, false)?;
    c.emit(0xc2e8, transfer_word(C::Al, true, R::R0, r(4), 4)?)?;
    c.emit(0xc2ec, transfer_word(C::Al, false, R::R0, r(4), 8)?)?;
    c.emit(0xc2f0, transfer_word(C::Al, true, R::R1, r(4), 16)?)?;
    c.jump(0xc2f4, C::Al, 0xb064, true)?;
    c.emit(0xc2f8, transfer_word(C::Al, true, R::R0, r(4), 4)?)?;
    c.emit(0xc2fc, transfer_byte(C::Al, true, R::R1, R::R0, 3)?)?;
    c.emit(0xc300, data_immediate(C::Al, Op::And, R::R1, R::R1, 0xf)?)?;
    c.emit(0xc304, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x10)?)?;
    c.emit(0xc308, transfer_byte(C::Al, false, R::R1, R::R0, 3)?)?;
    c.emit(0xc30c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(0xc310, transfer_word(C::Al, false, R::R0, r(4), 12)?)?;
    c.emit(0xc314, transfer_byte(C::Al, true, R::R0, r(4), 560)?)?;
    c.emit(0xc318, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.jump(0xc31c, C::Eq, 0xc348, false)?;
    c.emit(0xc320, data_immediate(C::Al, Op::Add, R::R0, r(4), 0x30)?)?;
    c.emit(0xc324, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(0xc328, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x200)?)?;
    c.jump(0xc32c, C::Al, 0x31ec, true)?;
    c.emit(0xc330, transfer_word(C::Al, true, R::R1, r(4), 4)?)?;
    c.emit(0xc334, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.jump(0xc338, C::Al, 0xaa0c, true)?;
    c.emit(0xc33c, transfer_word(C::Al, true, R::R1, r(4), 4)?)?;
    c.emit(0xc340, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.jump(0xc344, C::Al, 0xaba0, true)?;
    c.emit(0xc348, transfer_word(C::Al, true, R::R0, r(4), 4)?)?;
    c.emit(0xc34c, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x10)?)?;
    c.emit(0xc350, transfer_word(C::Al, true, r(3), R::R0, 4)?)?;
    c.emit(0xc354, data_immediate(C::Al, Op::Cmp, R::R0, r(3), 0x10)?)?;
    c.emit(0xc358, data_register(C::Al, Op::Add, R::R1, R::R0, r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xc35c, load_halfword(C::Ne, R::R2, r(7), 0)?)?;
    c.emit(0xc360, data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x8)?)?;
    c.emit(0xc364, transfer_word_register(C::Ne, true, R::R2, r(9), R::R2, Shift::Lsl, 2)?)?;
    c.emit(0xc368, transfer_word(C::Ne, false, R::R2, R::R1, 0)?)?;
    c.emit(0xc36c, transfer_word(C::Ne, true, R::R2, R::R0, 4)?)?;
    c.emit(0xc370, transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    c.emit(0xc374, load_halfword(C::Al, R::R1, R::R0, 2)?)?;
    c.emit(0xc378, load_halfword(C::Al, R::R2, R::R0, 4)?)?;
    c.emit(0xc37c, data_register(C::Al, Op::Eor, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc380, load_halfword(C::Al, R::R2, R::R0, 6)?)?;
    c.emit(0xc384, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc388, load_halfword(C::Al, R::R2, R::R0, 8)?)?;
    c.emit(0xc38c, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc390, load_halfword(C::Al, R::R2, R::R0, 10)?)?;
    c.emit(0xc394, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc398, load_halfword(C::Al, R::R2, r(5), 0)?)?;
    c.emit(0xc39c, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc3a0, store_halfword(C::Al, R::R1, R::R0, 0)?)?;
    c.emit(0xc3a4, transfer_word(C::Al, true, R::R1, r(4), 40)?)?;
    c.emit(0xc3a8, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.jump(0xc3ac, C::Eq, 0xc3bc, false)?;
    c.emit(0xc3b0, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc3b4, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0xc3b8, bx(C::Al, R::R1))?;
    c.emit(0xc3bc, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc3c0)
}

fn coalesce_backward(c: &mut Code) -> Result<(), String> {
    c.emit(0xc3c0, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x10)?)?;
    c.jump(0xc3c4, C::Eq, 0xc3dc, false)?;
    c.emit(0xc3c8, load_halfword(C::Al, R::R2, r(7), 0)?)?;
    c.emit(0xc3cc, transfer_word(C::Al, true, r(3), r(5), -16)?)?;
    c.emit(0xc3d0, transfer_word_register(C::Al, true, R::R2, r(9), R::R2, Shift::Lsl, 2)?)?;
    c.emit(0xc3d4, data_register(C::Al, Op::Cmp, R::R0, r(3), R::R2, Shift::Lsl, 0, false)?)?;
    c.jump(0xc3d8, C::Ne, 0xc160, false)?;
    c.emit(0xc3dc, data_register(C::Al, Op::Sub, r(8), r(6), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xc3e0, data_register(C::Al, Op::Cmp, R::R0, R::R1, r(8), Shift::Lsl, 0, false)?)?;
    c.jump(0xc3e4, C::Hi, 0xc160, false)?;
    c.emit(0xc3e8, transfer_word(C::Al, true, R::R1, r(8), 4)?)?;
    c.emit(0xc3ec, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.jump(0xc3f0, C::Ne, 0xc160, false)?;
    c.emit(0xc3f4, load_halfword(C::Al, R::R2, r(8), 0)?)?;
    c.emit(0xc3f8, load_halfword(C::Al, r(3), r(8), 2)?)?;
    c.emit(0xc3fc, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x234)?)?;
    c.emit(0xc400, transfer_halfword_register(C::Al, true, R::R0, r(4), R::R0)?)?;
    c.emit(0xc404, data_register(C::Al, Op::Eor, R::R2, r(3), R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc408, data_register(C::Al, Op::Eor, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc40c, load_halfword(C::Al, R::R2, r(8), 6)?)?;
    c.emit(0xc410, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc414, load_halfword(C::Al, R::R2, r(8), 8)?)?;
    c.emit(0xc418, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc41c, load_halfword(C::Al, R::R2, r(8), 10)?)?;
    c.emit(0xc420, data_register(C::Al, Op::Eor, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc424, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xff)?)?;
    c.emit(0xc428, data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xff00)?)?;
    c.emit(0xc42c, data_register(C::Al, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xc430, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.jump(0xc434, C::Ne, 0xc160, false)?;
    c.emit(0xc438, transfer_byte(C::Al, true, R::R0, r(8), 3)?)?;
    c.emit(0xc43c, data_immediate(C::Al, Op::And, R::R0, R::R0, 0xf)?)?;
    c.emit(0xc440, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.jump(0xc444, C::Ne, 0xc160, false)?;
    c.emit(0xc448, transfer_byte(C::Al, true, R::R0, r(4), 560)?)?;
    c.emit(0xc44c, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.jump(0xc450, C::Eq, 0xc464, false)?;
    c.emit(0xc454, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc458, data_register(C::Al, Op::Mov, R::R1, R::R0, r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc45c, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    c.jump(0xc460, C::Al, 0xb884, true)?;
    c.emit(0xc464, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xc468, data_register(C::Al, Op::Mov, r(6), R::R0, r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc46c, transfer_word(C::Al, false, R::R0, r(5), -12)?)?;
    c.emit(0xc470, transfer_word(C::Al, false, R::R0, r(5), -16)?)?;
    c.emit(0xc474, transfer_word(C::Al, true, R::R1, r(4), 12)?)?;
    c.emit(0xc478, data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    c.emit(0xc47c, transfer_word(C::Al, false, R::R1, r(4), 12)?)?;
    c.emit(0xc480, transfer_word(C::Al, true, R::R1, r(5), -4)?)?;
    c.emit(0xc484, transfer_word(C::Al, true, R::R2, r(8), 4)?)?;
    c.emit(0xc488, data_register(C::Al, Op::Add, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc48c, transfer_word(C::Al, false, R::R1, r(8), 4)?)?;
    c.emit(0xc490, transfer_byte(C::Al, true, R::R1, r(5), -5)?)?;
    c.emit(0xc494, transfer_byte(C::Al, true, R::R2, r(8), 3)?)?;
    c.emit(0xc498, data_immediate(C::Al, Op::And, R::R1, R::R1, 0xf0)?)?;
    c.emit(0xc49c, data_immediate(C::Al, Op::And, R::R2, R::R2, 0xf)?)?;
    c.emit(0xc4a0, data_register(C::Al, Op::Orr, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xc4a4, transfer_byte(C::Al, false, R::R1, r(8), 3)?)?;
    c.emit(0xc4a8, transfer_word(C::Al, false, R::R0, r(5), -8)?)?;
    c.emit(0xc4ac, transfer_word(C::Al, false, R::R0, r(5), -4)?)?;
    c.jump(0xc4b0, C::Al, 0xc160, false)?;
    c.check(0xc4b4)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    validate_heap_and_record_diagnostic(&mut c)?;
    validate_block_and_update_accounting(&mut c)?;
    coalesce_forward(&mut c)?;
    rebuild_checksum_and_empty_heap(&mut c)?;
    coalesce_backward(&mut c)?;
    c.data(0xc4b4, MEMHEAP_SOURCE)?;
    c.data(0xc4b8, MARKER_TABLE)?;
    c.data(0xc4bc, CALLBACK)?;
    c.data(0xc4c0, RING_INDEX)?;
    c.data(0xc4c4, DIAGNOSTIC_RING)?;
    c.data(0xc4c8, DIVIDE_BY_TWENTY)?;
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("heap release extent mismatch".into());
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
        assert_eq!(END - START, 1628);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(START + FILE_MAPPING_DELTA, 0xfe70);
        assert_eq!(END + FILE_MAPPING_DELTA, 0x104cc);
        for (at, value) in [
            (0xc4b4, MEMHEAP_SOURCE), (0xc4b8, MARKER_TABLE),
            (0xc4bc, CALLBACK), (0xc4c0, RING_INDEX),
            (0xc4c4, DIAGNOSTIC_RING), (0xc4c8, DIVIDE_BY_TWENTY),
        ] {
            assert_eq!(word(&bytes, at), value);
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.data(at, value ^ 1).is_err());
            assert!(c.data(at + 1, value).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes, value.to_le_bytes());
        }
    }

    #[test]
    fn rejects_invalid_emission_positions() {
        for at in [START - 4, START + 1, 0xc4b4, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.data(START, MEMHEAP_SOURCE).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn callbacks_ring_coalescing_and_rebuild_sites() {
        let bytes = fragment().unwrap();
        for (at, expected) in [
            (0xbf24, 0xe12f_ff31), (0xbf6c, 0xe12f_ff31),
            (0xbfdc, 0xe12f_ff31), (0xc3b8, 0xe12f_ff11),
            (0xbf44, 0xe0c6_3290), (0xbf54, 0xe040_0102),
            (0xc110, 0xe584_0014), (0xc15c, 0x0a00_0097),
            (0xc1dc, 0xe151_0000), (0xc214, 0xebff_fd9a),
            (0xc224, 0xe586_0004), (0xc278, 0xebff_fa48),
            (0xc2b4, 0xe1c6_00b0), (0xc2e4, 0x1a00_002e),
            (0xc32c, 0xebff_dbae), (0xc344, 0xebff_fa15),
            (0xc3a0, 0xe1c0_10b0), (0xc434, 0x1aff_ff49),
            (0xc460, 0xebff_fd07), (0xc48c, 0xe588_1004),
            (0xc4b0, 0xeaff_ff2a),
        ] {
            assert_eq!(word(&bytes, at), expected, "instruction at {at:#x}");
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
