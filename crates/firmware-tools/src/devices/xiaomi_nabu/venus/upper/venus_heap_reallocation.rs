use arm32::{
    branch, branch_exchange_with_link, data_immediate, data_immediate_with_flags, data_register, load_halfword, load_halfword_preindexed_writeback, load_halfword_register_preindexed_writeback, store_halfword, transfer_byte, transfer_halfword_register, transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xc4cc;
pub const END: usize = 0xcdc0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2280;
pub const DATA_BYTES: usize = 12;
const SOURCE: u32 = 0x000e_37d7;
const CALLBACK: u32 = 0x0000_d2a8;
const MARKER_TABLE: u32 = 0x0010_3ae8;
const PROTECTED_DATA: [(usize, u32); 3] = [
    (0xcdb4, SOURCE),
    (0xcdb8, CALLBACK),
    (0xcdbc, MARKER_TABLE),
];

fn r(number: u8) -> R {
    R::new(number).expect("A32 register")
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if self.at != at || !(START..0xcdb4).contains(&at) || at % 4 != 0 {
            return Err(format!("heap reallocation instruction at {at:#x}, cursor {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        if self.at != at || !PROTECTED_DATA.contains(&(at, value)) {
            return Err(format!("heap reallocation invalid data pair: {at:#x} = {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn validate_reallocation(c: &mut Code) -> Result<(), String> {
    c.emit(0xc4cc, transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xc4d0, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0xc4d4, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x24)?)?;
    c.emit(0xc4d8, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc4dc, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc4e0, data_register(C::Al, Op::Mov, r(10), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc4e4, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0xc4e8, branch(C::Ne, 0xc4e8, 0xc504, false)?)?;
    c.emit(0xc4ec, branch(C::Al, 0xc4ec, 0x86644, true)?)?;
    c.emit(0xc4f0, transfer_word(C::Al, true, r(2), r(15), 2236)?)?;
    c.emit(0xc4f4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe5)?)?;
    c.emit(0xc4f8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc4fc, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.emit(0xc500, branch(C::Al, 0xc500, 0x17b0, true)?)?;
    c.emit(0xc504, branch(C::Al, 0xc504, 0x86670, true)?)?;
    c.emit(0xc508, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0xc50c, branch(C::Eq, 0xc50c, 0xc53c, false)?)?;
    c.emit(0xc510, transfer_word(C::Al, true, r(0), r(5), 36)?)?;
    c.emit(0xc514, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0xc518, transfer_word(C::Ne, true, r(1), r(15), 2200)?)?;
    c.emit(0xc51c, data_register(C::Ne, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc520, branch(C::Eq, 0xc520, 0xc53c, false)?)?;
    c.emit(0xc524, branch(C::Al, 0xc524, 0x86644, true)?)?;
    c.emit(0xc528, transfer_word(C::Al, true, r(2), r(15), 2180)?)?;
    c.emit(0xc52c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe6)?)?;
    c.emit(0xc530, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc534, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.emit(0xc538, branch(C::Al, 0xc538, 0x17b0, true)?)?;
    c.emit(0xc53c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x238)?)?;
    c.emit(0xc540, data_register(C::Al, Op::Mov, r(9), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc544, transfer_word(C::Al, true, r(7), r(15), 2160)?)?;
    c.emit(0xc548, transfer_word(C::Al, true, r(1), r(5), 0)?)?;
    c.emit(0xc54c, load_halfword_register_preindexed_writeback(C::Al, r(0), r(9), r(0))?)?;
    c.emit(0xc550, transfer_word_register(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 2)?)?;
    c.emit(0xc554, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc558, branch(C::Eq, 0xc558, 0xc574, false)?)?;
    c.emit(0xc55c, branch(C::Al, 0xc55c, 0x86644, true)?)?;
    c.emit(0xc560, transfer_word(C::Al, true, r(2), r(15), 2124)?)?;
    c.emit(0xc564, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe7)?)?;
    c.emit(0xc568, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc56c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.emit(0xc570, branch(C::Al, 0xc570, 0x17b0, true)?)?;
    c.emit(0xc574, branch(C::Al, 0xc574, 0x86670, true)?)?;
    c.emit(0xc578, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0xc57c, transfer_word(C::Ne, true, r(0), r(5), 36)?)?;
    c.emit(0xc580, data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0xc584, branch(C::Eq, 0xc584, 0xc598, false)?)?;
    c.emit(0xc588, transfer_word(C::Al, true, r(1), r(15), 2088)?)?;
    c.emit(0xc58c, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0xc590, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc594, branch(C::Ne, 0xc594, 0xc5b4, false)?)?;
    c.emit(0xc598, transfer_word(C::Al, true, r(0), r(5), 16)?)?;
    c.emit(0xc59c, transfer_word(C::Al, true, r(1), r(5), 20)?)?;
    c.emit(0xc5a0, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0xc5a4, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x8)?)?;
    c.emit(0xc5a8, data_register(C::Al, Op::Sub, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc5ac, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xc5b0, branch(C::Cs, 0xc5b0, 0xc5c0, false)?)?;
    c.emit(0xc5b4, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc5b8, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0xc5bc, transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0xc5c0, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.emit(0xc5c4, branch(C::Eq, 0xc5c4, 0xc8f4, false)?)?;
    c.emit(0xc5c8, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0xc5cc, branch(C::Eq, 0xc5cc, 0xcd9c, false)?)?;
    Ok(())
}

fn validate_allocated_block(c: &mut Code) -> Result<(), String> {
    c.emit(0xc5d0, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.emit(0xc5d4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0xc5d8, branch(C::Ne, 0xc5d8, 0xc5f4, false)?)?;
    c.emit(0xc5dc, branch(C::Al, 0xc5dc, 0x86644, true)?)?;
    c.emit(0xc5e0, transfer_word(C::Al, true, r(2), r(15), 1996)?)?;
    c.emit(0xc5e4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(0xc5e8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc5ec, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xc5f0, branch(C::Al, 0xc5f0, 0x17b0, true)?)?;
    c.emit(0xc5f4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x36)?)?;
    c.emit(0xc5f8, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc5fc, data_register(C::Al, Op::Mov, r(8), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0xc600, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x200)?)?;
    c.emit(0xc604, load_halfword_register_preindexed_writeback(C::Al, r(0), r(1), r(0))?)?;
    c.emit(0xc608, transfer_word(C::Al, false, r(1), r(13), 32)?)?;
    c.emit(0xc60c, load_halfword_preindexed_writeback(C::Al, r(1), r(8), -8)?)?;
    c.emit(0xc610, load_halfword(C::Al, r(2), r(8), 2)?)?;
    c.emit(0xc614, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc618, load_halfword(C::Al, r(2), r(8), 4)?)?;
    c.emit(0xc61c, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc620, load_halfword(C::Al, r(2), r(8), 6)?)?;
    c.emit(0xc624, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc628, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.emit(0xc62c, data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.emit(0xc630, data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc634, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc638, branch(C::Eq, 0xc638, 0xc654, false)?)?;
    c.emit(0xc63c, branch(C::Al, 0xc63c, 0x86644, true)?)?;
    c.emit(0xc640, transfer_word(C::Al, true, r(2), r(15), 1900)?)?;
    c.emit(0xc644, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xd)?)?;
    c.emit(0xc648, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc64c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xc650, branch(C::Al, 0xc650, 0x17b0, true)?)?;
    c.emit(0xc654, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.emit(0xc658, data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc65c, branch(C::Hi, 0xc65c, 0xc670, false)?)?;
    c.emit(0xc660, transfer_word(C::Al, true, r(1), r(5), 16)?)?;
    c.emit(0xc664, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(0xc668, data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc66c, branch(C::Hi, 0xc66c, 0xc688, false)?)?;
    c.emit(0xc670, branch(C::Al, 0xc670, 0x86644, true)?)?;
    c.emit(0xc674, transfer_word(C::Al, true, r(2), r(15), 1848)?)?;
    c.emit(0xc678, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe)?)?;
    c.emit(0xc67c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc680, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xc684, branch(C::Al, 0xc684, 0x17b0, true)?)?;
    c.emit(0xc688, transfer_byte(C::Al, true, r(0), r(10), -5)?)?;
    c.emit(0xc68c, data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.emit(0xc690, branch(C::Eq, 0xc690, 0xc6b0, false)?)?;
    c.emit(0xc694, branch(C::Al, 0xc694, 0x86644, true)?)?;
    c.emit(0xc698, transfer_word(C::Al, true, r(2), r(15), 1812)?)?;
    c.emit(0xc69c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x12)?)?;
    c.emit(0xc6a0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc6a4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xc6a8, branch(C::Al, 0xc6a8, 0x17b0, true)?)?;
    c.emit(0xc6ac, transfer_byte(C::Al, true, r(0), r(10), -5)?)?;
    c.emit(0xc6b0, data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.emit(0xc6b4, branch(C::Ne, 0xc6b4, 0xc5b4, false)?)?;
    Ok(())
}

fn expand_adjacent(c: &mut Code) -> Result<(), String> {
    c.emit(0xc6b8, data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc6bc, branch(C::Al, 0xc6bc, 0xcdc0, true)?)?;
    c.emit(0xc6c0, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xc6c4, branch(C::Eq, 0xc6c4, 0xcd6c, false)?)?;
    c.emit(0xc6c8, data_register(C::Al, Op::Sub, r(1), r(6), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc6cc, data_immediate(C::Al, Op::Add, r(3), r(6), 0x17)?)?;
    c.emit(0xc6d0, transfer_word(C::Al, false, r(1), r(13), 16)?)?;
    c.emit(0xc6d4, data_immediate(C::Al, Op::Rsb, r(1), r(6), 0xf8)?)?;
    c.emit(0xc6d8, data_immediate(C::Al, Op::Bic, r(2), r(3), 0xf)?)?;
    c.emit(0xc6dc, data_register(C::Al, Op::Add, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc6e0, transfer_word(C::Al, false, r(1), r(13), 20)?)?;
    c.emit(0xc6e4, transfer_word(C::Al, true, r(4), r(10), -4)?)?;
    c.emit(0xc6e8, data_register(C::Al, Op::Cmp, r(0), r(2), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc6ec, branch(C::Ls, 0xc6ec, 0xc908, false)?)?;
    c.emit(0xc6f0, transfer_word(C::Al, false, r(2), r(13), 12)?)?;
    c.emit(0xc6f4, transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(0xc6f8, transfer_word(C::Al, true, r(1), r(5), 36)?)?;
    c.emit(0xc6fc, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0xc700, branch(C::Eq, 0xc700, 0xc70c, false)?)?;
    c.emit(0xc704, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc708, branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(0xc70c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc710, data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc714, branch(C::Al, 0xc714, 0xbce4, true)?)?;
    c.emit(0xc718, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc71c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0xc720, data_register(C::Al, Op::Cmp, r(0), r(1), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc724, branch(C::Ls, 0xc724, 0xcd24, false)?)?;
    c.emit(0xc728, transfer_byte(C::Al, true, r(0), r(1), 3)?)?;
    c.emit(0xc72c, data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.emit(0xc730, branch(C::Eq, 0xc730, 0xcd24, false)?)?;
    c.emit(0xc734, transfer_word(C::Al, true, r(0), r(13), 12)?)?;
    c.emit(0xc738, data_register(C::Al, Op::Sub, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc73c, transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.emit(0xc740, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc744, transfer_word(C::Al, false, r(1), r(13), 4)?)?;
    c.emit(0xc748, branch(C::Al, 0xc748, 0xbce4, true)?)?;
    c.emit(0xc74c, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc750, transfer_byte(C::Al, true, r(0), r(5), 560)?)?;
    c.emit(0xc754, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.emit(0xc758, branch(C::Eq, 0xc758, 0xc76c, false)?)?;
    c.emit(0xc75c, transfer_word(C::Al, true, r(1), r(13), 4)?)?;
    c.emit(0xc760, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc764, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0xc768, branch(C::Al, 0xc768, 0xb884, true)?)?;
    c.emit(0xc76c, transfer_word(C::Al, true, r(3), r(13), 4)?)?;
    c.emit(0xc770, transfer_word(C::Al, true, r(0), r(13), 24)?)?;
    c.emit(0xc774, transfer_word(C::Al, true, r(1), r(3), 4)?)?;
    c.emit(0xc778, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc77c, branch(C::Cs, 0xc77c, 0xc868, false)?)?;
    c.emit(0xc780, data_immediate(C::Al, Op::Add, r(0), r(5), 0x234)?)?;
    c.emit(0xc784, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0xc788, data_register(C::Al, Op::Cmp, r(0), r(4), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xc78c, branch(C::Ls, 0xc78c, 0xc868, false)?)?;
    c.emit(0xc790, transfer_byte(C::Al, true, r(0), r(4), 3)?)?;
    c.emit(0xc794, data_immediate_with_flags(C::Al, Op::And, r(0), r(0), 0xf, true)?)?;
    c.emit(0xc798, branch(C::Eq, 0xc798, 0xc868, false)?)?;
    c.emit(0xc79c, transfer_word(C::Al, true, r(0), r(5), 8)?)?;
    c.emit(0xc7a0, transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.emit(0xc7a4, transfer_byte(C::Al, true, r(0), r(5), 560)?)?;
    c.emit(0xc7a8, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.emit(0xc7ac, branch(C::Eq, 0xc7ac, 0xc7c8, false)?)?;
    c.emit(0xc7b0, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc7b4, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc7b8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0xc7bc, branch(C::Al, 0xc7bc, 0xb884, true)?)?;
    c.emit(0xc7c0, transfer_word(C::Al, true, r(3), r(13), 4)?)?;
    c.emit(0xc7c4, transfer_word(C::Al, true, r(1), r(3), 4)?)?;
    c.emit(0xc7c8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0xc7cc, data_register(C::Al, Op::Mov, r(7), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xc7d0, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc7d4, transfer_word(C::Al, false, r(0), r(3), 4)?)?;
    c.emit(0xc7d8, transfer_byte(C::Al, true, r(1), r(4), 3)?)?;
    c.emit(0xc7dc, transfer_byte(C::Al, true, r(2), r(3), 3)?)?;
    c.emit(0xc7e0, data_immediate(C::Al, Op::And, r(1), r(1), 0xf0)?)?;
    c.emit(0xc7e4, data_immediate(C::Al, Op::And, r(2), r(2), 0xf)?)?;
    c.emit(0xc7e8, data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc7ec, transfer_byte(C::Al, false, r(1), r(3), 3)?)?;
    c.emit(0xc7f0, load_halfword(C::Al, r(1), r(3), 2)?)?;
    c.emit(0xc7f4, data_register(C::Al, Op::Eor, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc7f8, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsr, 16, false)?)?;
    c.emit(0xc7fc, load_halfword(C::Al, r(1), r(3), 8)?)?;
    c.emit(0xc800, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc804, load_halfword(C::Al, r(1), r(3), 10)?)?;
    c.emit(0xc808, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc80c, transfer_word(C::Al, true, r(1), r(13), 8)?)?;
    c.emit(0xc810, load_halfword(C::Al, r(1), r(1), 0)?)?;
    c.emit(0xc814, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc818, data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xc81c, store_halfword(C::Al, r(0), r(3), 0)?)?;
    c.emit(0xc820, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xc824, transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0xc828, transfer_word(C::Al, false, r(0), r(4), 4)?)?;
    c.emit(0xc82c, transfer_word(C::Al, true, r(0), r(5), 12)?)?;
    c.emit(0xc830, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(0xc834, transfer_word(C::Al, false, r(0), r(5), 12)?)?;
    c.emit(0xc838, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc83c, branch(C::Al, 0xc83c, 0xbce4, true)?)?;
    c.emit(0xc840, transfer_word(C::Al, true, r(1), r(13), 28)?)?;
    c.emit(0xc844, data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0xc848, data_register(C::Al, Op::Cmp, r(0), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc84c, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc850, transfer_word(C::Eq, false, r(0), r(5), 8)?)?;
    c.emit(0xc854, transfer_word(C::Al, true, r(1), r(7), 4)?)?;
    c.emit(0xc858, transfer_word(C::Al, true, r(2), r(13), 24)?)?;
    c.emit(0xc85c, transfer_word(C::Al, true, r(7), r(15), 1368)?)?;
    c.emit(0xc860, data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc864, branch(C::Cc, 0xc864, 0xc788, false)?)?;
    c.emit(0xc868, transfer_word(C::Al, true, r(0), r(13), 24)?)?;
    c.emit(0xc86c, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc870, branch(C::Cs, 0xc870, 0xcad8, false)?)?;
    c.emit(0xc874, transfer_byte(C::Al, true, r(0), r(5), 560)?)?;
    c.emit(0xc878, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.emit(0xc87c, branch(C::Eq, 0xc87c, 0xc894, false)?)?;
    c.emit(0xc880, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc884, data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xc888, branch(C::Al, 0xc888, 0xaba0, true)?)?;
    c.emit(0xc88c, transfer_word(C::Al, true, r(3), r(13), 4)?)?;
    c.emit(0xc890, transfer_word(C::Al, true, r(1), r(3), 4)?)?;
    c.emit(0xc894, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x10)?)?;
    c.emit(0xc898, data_register(C::Al, Op::Add, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc89c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(0xc8a0, load_halfword(C::Ne, r(1), r(9), 0)?)?;
    c.emit(0xc8a4, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x8)?)?;
    c.emit(0xc8a8, transfer_word_register(C::Ne, true, r(1), r(7), r(1), Shift::Lsl, 2)?)?;
    c.emit(0xc8ac, transfer_word(C::Ne, false, r(1), r(0), 0)?)?;
    c.emit(0xc8b0, transfer_word(C::Ne, true, r(2), r(3), 4)?)?;
    c.emit(0xc8b4, transfer_word(C::Al, false, r(2), r(0), 4)?)?;
    c.emit(0xc8b8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(0xc8bc, load_halfword(C::Al, r(1), r(3), 2)?)?;
    c.emit(0xc8c0, load_halfword(C::Al, r(2), r(3), 4)?)?;
    c.emit(0xc8c4, transfer_halfword_register(C::Al, true, r(0), r(5), r(0))?)?;
    c.emit(0xc8c8, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8cc, load_halfword(C::Al, r(2), r(3), 6)?)?;
    c.emit(0xc8d0, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8d4, load_halfword(C::Al, r(2), r(3), 8)?)?;
    c.emit(0xc8d8, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8dc, load_halfword(C::Al, r(2), r(3), 10)?)?;
    c.emit(0xc8e0, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8e4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0xc8e8, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8ec, store_halfword(C::Al, r(0), r(3), 0)?)?;
    c.emit(0xc8f0, branch(C::Al, 0xc8f0, 0xcd24, false)?)?;
    Ok(())
}

fn allocate_null(c: &mut Code) -> Result<(), String> {
    c.emit(0xc8f4, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8f8, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xc8fc, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0xc900, transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0xc904, branch(C::Al, 0xc904, 0xb28c, false)?)?;
    Ok(())
}

fn shrink_and_split(c: &mut Code) -> Result<(), String> {
    c.emit(0xc908, transfer_word(C::Al, true, r(1), r(5), 36)?)?;
    c.emit(0xc90c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0xc910, branch(C::Eq, 0xc910, 0xc930, false)?)?;
    c.emit(0xc914, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc918, transfer_word(C::Al, false, r(2), r(13), 12)?)?;
    c.emit(0xc91c, data_register(C::Al, Op::Mov, r(4), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xc920, branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(0xc924, data_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc928, transfer_word(C::Al, true, r(2), r(13), 12)?)?;
    c.emit(0xc92c, transfer_word(C::Al, true, r(4), r(10), -4)?)?;
    c.emit(0xc930, data_register(C::Al, Op::Cmp, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc934, branch(C::Ls, 0xc934, 0xca44, false)?)?;
    c.emit(0xc938, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc93c, data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc940, data_register(C::Al, Op::Mov, r(4), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc944, transfer_word(C::Al, false, r(3), r(13), 24)?)?;
    c.emit(0xc948, branch(C::Al, 0xc948, 0xbce4, true)?)?;
    c.emit(0xc94c, transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.emit(0xc950, data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc954, transfer_word(C::Al, true, r(0), r(10), -4)?)?;
    c.emit(0xc958, data_register(C::Al, Op::Add, r(4), r(8), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc95c, data_register(C::Al, Op::Mov, r(8), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc960, data_register(C::Al, Op::Sub, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc964, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc968, branch(C::Al, 0xc968, 0xb064, true)?)?;
    c.emit(0xc96c, transfer_byte(C::Al, true, r(0), r(4), 3)?)?;
    c.emit(0xc970, transfer_byte(C::Al, true, r(1), r(10), -5)?)?;
    c.emit(0xc974, data_immediate(C::Al, Op::And, r(0), r(0), 0xf)?)?;
    c.emit(0xc978, data_immediate(C::Al, Op::And, r(1), r(1), 0xf0)?)?;
    c.emit(0xc97c, data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc980, transfer_byte(C::Al, false, r(0), r(4), 3)?)?;
    c.emit(0xc984, transfer_word(C::Al, false, r(8), r(10), -4)?)?;
    c.emit(0xc988, transfer_byte(C::Al, true, r(0), r(10), -5)?)?;
    c.emit(0xc98c, data_immediate(C::Al, Op::And, r(0), r(0), 0xf)?)?;
    c.emit(0xc990, transfer_byte(C::Al, false, r(0), r(10), -5)?)?;
    c.emit(0xc994, load_halfword(C::Al, r(0), r(10), -6)?)?;
    c.emit(0xc998, transfer_word(C::Al, true, r(1), r(13), 24)?)?;
    c.emit(0xc99c, data_register(C::Al, Op::Eor, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xc9a0, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsr, 16, false)?)?;
    c.emit(0xc9a4, transfer_word(C::Al, true, r(1), r(13), 32)?)?;
    c.emit(0xc9a8, load_halfword(C::Al, r(1), r(1), 0)?)?;
    c.emit(0xc9ac, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc9b0, store_halfword(C::Al, r(0), r(10), -8)?)?;
    c.emit(0xc9b4, transfer_word(C::Al, true, r(0), r(5), 12)?)?;
    c.emit(0xc9b8, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0xc9bc, transfer_word(C::Al, false, r(0), r(5), 12)?)?;
    c.emit(0xc9c0, transfer_word(C::Al, true, r(0), r(5), 8)?)?;
    c.emit(0xc9c4, transfer_word(C::Al, true, r(1), r(13), 28)?)?;
    c.emit(0xc9c8, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xc9cc, transfer_word(C::Eq, false, r(4), r(5), 8)?)?;
    c.emit(0xc9d0, transfer_byte(C::Al, true, r(0), r(5), 560)?)?;
    c.emit(0xc9d4, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.emit(0xc9d8, branch(C::Eq, 0xc9d8, 0xc9e8, false)?)?;
    c.emit(0xc9dc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xc9e0, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xc9e4, branch(C::Al, 0xc9e4, 0xaba0, true)?)?;
    c.emit(0xc9e8, transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.emit(0xc9ec, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x10)?)?;
    c.emit(0xc9f0, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x10)?)?;
    c.emit(0xc9f4, data_register(C::Al, Op::Add, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xc9f8, load_halfword(C::Ne, r(1), r(9), 0)?)?;
    c.emit(0xc9fc, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x8)?)?;
    c.emit(0xca00, transfer_word_register(C::Ne, true, r(1), r(7), r(1), Shift::Lsl, 2)?)?;
    c.emit(0xca04, transfer_word(C::Ne, false, r(1), r(0), 0)?)?;
    c.emit(0xca08, transfer_word(C::Ne, true, r(1), r(4), 4)?)?;
    c.emit(0xca0c, transfer_word(C::Al, false, r(1), r(0), 4)?)?;
    c.emit(0xca10, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(0xca14, load_halfword(C::Al, r(1), r(4), 2)?)?;
    c.emit(0xca18, load_halfword(C::Al, r(2), r(4), 4)?)?;
    c.emit(0xca1c, transfer_halfword_register(C::Al, true, r(0), r(5), r(0))?)?;
    c.emit(0xca20, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xca24, load_halfword(C::Al, r(2), r(4), 6)?)?;
    c.emit(0xca28, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xca2c, load_halfword(C::Al, r(2), r(4), 8)?)?;
    c.emit(0xca30, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xca34, load_halfword(C::Al, r(2), r(4), 10)?)?;
    c.emit(0xca38, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xca3c, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xca40, store_halfword(C::Al, r(0), r(4), 0)?)?;
    Ok(())
}

fn account_shrink(c: &mut Code) -> Result<(), String> {
    c.emit(0xca44, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0xca48, transfer_word(C::Al, true, r(1), r(13), 16)?)?;
    c.emit(0xca4c, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xca50, transfer_word(C::Al, false, r(0), r(5), 20)?)?;
    c.emit(0xca54, transfer_word(C::Al, true, r(1), r(5), 16)?)?;
    c.emit(0xca58, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xca5c, branch(C::Cs, 0xca5c, 0xca7c, false)?)?;
    c.emit(0xca60, branch(C::Al, 0xca60, 0x86644, true)?)?;
    c.emit(0xca64, transfer_word(C::Al, true, r(2), r(15), 840)?)?;
    c.emit(0xca68, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xf6)?)?;
    c.emit(0xca6c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xca70, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xca74, branch(C::Al, 0xca74, 0x17b0, true)?)?;
    c.emit(0xca78, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0xca7c, transfer_word(C::Al, true, r(1), r(13), 20)?)?;
    c.emit(0xca80, transfer_byte(C::Al, false, r(1), r(10), -6)?)?;
    c.emit(0xca84, load_halfword(C::Al, r(1), r(10), -6)?)?;
    c.emit(0xca88, load_halfword(C::Al, r(2), r(10), -4)?)?;
    c.emit(0xca8c, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xca90, load_halfword(C::Al, r(2), r(10), -2)?)?;
    c.emit(0xca94, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xca98, transfer_word(C::Al, true, r(2), r(13), 32)?)?;
    c.emit(0xca9c, load_halfword(C::Al, r(2), r(2), 0)?)?;
    c.emit(0xcaa0, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcaa4, store_halfword(C::Al, r(1), r(10), -8)?)?;
    c.emit(0xcaa8, transfer_word(C::Al, true, r(1), r(5), 24)?)?;
    c.emit(0xcaac, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcab0, transfer_word(C::Hi, false, r(0), r(5), 24)?)?;
    c.emit(0xcab4, transfer_word(C::Al, true, r(0), r(5), 28)?)?;
    c.emit(0xcab8, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xcabc, transfer_word(C::Cc, false, r(6), r(5), 28)?)?;
    c.emit(0xcac0, transfer_word(C::Al, true, r(1), r(5), 40)?)?;
    c.emit(0xcac4, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0xcac8, branch(C::Eq, 0xcac8, 0xcd6c, false)?)?;
    c.emit(0xcacc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcad0, branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(0xcad4, branch(C::Al, 0xcad4, 0xcd6c, false)?)?;
    Ok(())
}

fn split_adjacent_remainder(c: &mut Code) -> Result<(), String> {
    c.emit(0xcad8, branch(C::Ls, 0xcad8, 0xcc04, false)?)?;
    c.emit(0xcadc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcae0, data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xcae4, branch(C::Al, 0xcae4, 0xbce4, true)?)?;
    c.emit(0xcae8, transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.emit(0xcaec, transfer_word(C::Al, true, r(0), r(13), 4)?)?;
    c.emit(0xcaf0, transfer_word(C::Al, true, r(1), r(13), 24)?)?;
    c.emit(0xcaf4, data_register(C::Al, Op::Add, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcaf8, transfer_word(C::Al, true, r(0), r(13), 4)?)?;
    c.emit(0xcafc, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0xcb00, data_register(C::Al, Op::Sub, r(1), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb04, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb08, branch(C::Al, 0xcb08, 0xb064, true)?)?;
    c.emit(0xcb0c, transfer_word(C::Al, true, r(3), r(13), 4)?)?;
    c.emit(0xcb10, transfer_byte(C::Al, true, r(0), r(4), 3)?)?;
    c.emit(0xcb14, data_register(C::Al, Op::Mov, r(12), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb18, transfer_byte(C::Al, true, r(1), r(3), 3)?)?;
    c.emit(0xcb1c, data_immediate(C::Al, Op::And, r(0), r(0), 0xf)?)?;
    c.emit(0xcb20, data_immediate(C::Al, Op::And, r(1), r(1), 0xf0)?)?;
    c.emit(0xcb24, data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb28, transfer_byte(C::Al, false, r(0), r(4), 3)?)?;
    c.emit(0xcb2c, transfer_word(C::Al, true, r(2), r(13), 24)?)?;
    c.emit(0xcb30, transfer_word(C::Al, false, r(2), r(3), 4)?)?;
    c.emit(0xcb34, transfer_byte(C::Al, true, r(0), r(3), 3)?)?;
    c.emit(0xcb38, data_immediate(C::Al, Op::And, r(0), r(0), 0xf)?)?;
    c.emit(0xcb3c, transfer_byte(C::Al, false, r(0), r(3), 3)?)?;
    c.emit(0xcb40, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(0xcb44, load_halfword(C::Al, r(1), r(3), 2)?)?;
    c.emit(0xcb48, load_halfword_register_preindexed_writeback(C::Al, r(0), r(12), r(0))?)?;
    c.emit(0xcb4c, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb50, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsr, 16, false)?)?;
    c.emit(0xcb54, load_halfword(C::Al, r(2), r(3), 8)?)?;
    c.emit(0xcb58, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb5c, load_halfword(C::Al, r(2), r(3), 10)?)?;
    c.emit(0xcb60, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb64, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb68, store_halfword(C::Al, r(0), r(3), 0)?)?;
    c.emit(0xcb6c, transfer_word(C::Al, true, r(0), r(12), -552)?)?;
    c.emit(0xcb70, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0xcb74, transfer_word(C::Al, false, r(0), r(12), -552)?)?;
    c.emit(0xcb78, transfer_word(C::Al, true, r(0), r(12), -556)?)?;
    c.emit(0xcb7c, transfer_word(C::Al, true, r(1), r(13), 28)?)?;
    c.emit(0xcb80, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb84, transfer_word(C::Eq, false, r(4), r(5), 8)?)?;
    c.emit(0xcb88, transfer_byte(C::Al, true, r(0), r(5), 560)?)?;
    c.emit(0xcb8c, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.emit(0xcb90, branch(C::Eq, 0xcb90, 0xcbac, false)?)?;
    c.emit(0xcb94, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb98, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xcb9c, transfer_word(C::Al, false, r(12), r(13), 28)?)?;
    c.emit(0xcba0, branch(C::Al, 0xcba0, 0xaba0, true)?)?;
    c.emit(0xcba4, transfer_word(C::Al, true, r(12), r(13), 28)?)?;
    c.emit(0xcba8, transfer_word(C::Al, true, r(3), r(13), 4)?)?;
    c.emit(0xcbac, transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.emit(0xcbb0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x10)?)?;
    c.emit(0xcbb4, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x10)?)?;
    c.emit(0xcbb8, data_register(C::Al, Op::Add, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcbbc, load_halfword(C::Ne, r(1), r(9), 0)?)?;
    c.emit(0xcbc0, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x8)?)?;
    c.emit(0xcbc4, transfer_word_register(C::Ne, true, r(1), r(7), r(1), Shift::Lsl, 2)?)?;
    c.emit(0xcbc8, transfer_word(C::Ne, false, r(1), r(0), 0)?)?;
    c.emit(0xcbcc, transfer_word(C::Ne, true, r(1), r(4), 4)?)?;
    c.emit(0xcbd0, transfer_word(C::Al, false, r(1), r(0), 4)?)?;
    c.emit(0xcbd4, load_halfword(C::Al, r(0), r(4), 2)?)?;
    c.emit(0xcbd8, load_halfword(C::Al, r(1), r(4), 4)?)?;
    c.emit(0xcbdc, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcbe0, load_halfword(C::Al, r(1), r(4), 6)?)?;
    c.emit(0xcbe4, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcbe8, load_halfword(C::Al, r(1), r(4), 8)?)?;
    c.emit(0xcbec, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcbf0, load_halfword(C::Al, r(1), r(4), 10)?)?;
    c.emit(0xcbf4, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcbf8, load_halfword(C::Al, r(1), r(12), 0)?)?;
    c.emit(0xcbfc, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcc00, store_halfword(C::Al, r(0), r(4), 0)?)?;
    Ok(())
}

fn merge_adjacent_and_account(c: &mut Code) -> Result<(), String> {
    c.emit(0xcc04, transfer_byte(C::Al, true, r(0), r(3), 3)?)?;
    c.emit(0xcc08, transfer_byte(C::Al, true, r(1), r(10), -5)?)?;
    c.emit(0xcc0c, data_immediate(C::Al, Op::And, r(0), r(0), 0xf0)?)?;
    c.emit(0xcc10, data_immediate(C::Al, Op::And, r(1), r(1), 0xf)?)?;
    c.emit(0xcc14, data_register(C::Al, Op::Orr, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcc18, transfer_byte(C::Al, false, r(0), r(10), -5)?)?;
    c.emit(0xcc1c, transfer_word(C::Al, true, r(0), r(3), 4)?)?;
    c.emit(0xcc20, transfer_word(C::Al, true, r(1), r(10), -4)?)?;
    c.emit(0xcc24, data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcc28, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xcc2c, transfer_word(C::Al, false, r(1), r(10), -4)?)?;
    c.emit(0xcc30, transfer_word(C::Al, false, r(0), r(3), 0)?)?;
    c.emit(0xcc34, transfer_word(C::Al, false, r(0), r(3), 4)?)?;
    c.emit(0xcc38, transfer_word(C::Al, true, r(0), r(5), 12)?)?;
    c.emit(0xcc3c, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(0xcc40, transfer_word(C::Al, false, r(0), r(5), 12)?)?;
    c.emit(0xcc44, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0xcc48, transfer_word(C::Al, true, r(2), r(13), 16)?)?;
    c.emit(0xcc4c, data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcc50, transfer_word(C::Al, false, r(0), r(5), 20)?)?;
    c.emit(0xcc54, transfer_word(C::Al, true, r(2), r(5), 16)?)?;
    c.emit(0xcc58, data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcc5c, branch(C::Cs, 0xcc5c, 0xcc88, false)?)?;
    c.emit(0xcc60, branch(C::Al, 0xcc60, 0x86644, true)?)?;
    c.emit(0xcc64, transfer_word(C::Al, true, r(2), r(15), 328)?)?;
    c.emit(0xcc68, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x99)?)?;
    c.emit(0xcc6c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xcc70, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xcc74, branch(C::Al, 0xcc74, 0x17b0, true)?)?;
    c.emit(0xcc78, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0xcc7c, load_halfword(C::Al, r(2), r(10), -2)?)?;
    c.emit(0xcc80, load_halfword(C::Al, r(1), r(10), -4)?)?;
    c.emit(0xcc84, branch(C::Al, 0xcc84, 0xcc8c, false)?)?;
    c.emit(0xcc88, data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsr, 16, false)?)?;
    c.emit(0xcc8c, transfer_word(C::Al, true, r(3), r(13), 20)?)?;
    c.emit(0xcc90, transfer_byte(C::Al, false, r(3), r(10), -6)?)?;
    c.emit(0xcc94, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x234)?)?;
    c.emit(0xcc98, load_halfword(C::Al, r(4), r(10), -6)?)?;
    c.emit(0xcc9c, transfer_halfword_register(C::Al, true, r(3), r(5), r(3))?)?;
    c.emit(0xcca0, data_register(C::Al, Op::Eor, r(1), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xcca4, transfer_word(C::Al, true, r(4), r(5), 8)?)?;
    c.emit(0xcca8, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xccac, load_halfword(C::Al, r(2), r(10), 0)?)?;
    c.emit(0xccb0, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xccb4, load_halfword(C::Al, r(2), r(10), 2)?)?;
    c.emit(0xccb8, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xccbc, data_register(C::Al, Op::Eor, r(1), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xccc0, store_halfword(C::Al, r(1), r(10), -8)?)?;
    c.emit(0xccc4, transfer_word(C::Al, true, r(1), r(5), 24)?)?;
    c.emit(0xccc8, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcccc, data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0xccd0, transfer_word(C::Hi, false, r(0), r(5), 24)?)?;
    c.emit(0xccd4, transfer_word(C::Al, true, r(0), r(5), 28)?)?;
    c.emit(0xccd8, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xccdc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcce0, transfer_word(C::Cc, false, r(6), r(5), 28)?)?;
    c.emit(0xcce4, branch(C::Al, 0xcce4, 0xbce4, true)?)?;
    c.emit(0xcce8, transfer_word(C::Al, true, r(1), r(13), 4)?)?;
    c.emit(0xccec, data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0xccf0, data_register(C::Al, Op::Cmp, r(0), r(4), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xccf4, transfer_word(C::Eq, false, r(0), r(5), 8)?)?;
    c.emit(0xccf8, transfer_word(C::Al, true, r(0), r(10), -4)?)?;
    c.emit(0xccfc, transfer_word(C::Al, true, r(1), r(13), 12)?)?;
    c.emit(0xcd00, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd04, branch(C::Eq, 0xcd04, 0xcd24, false)?)?;
    c.emit(0xcd08, branch(C::Al, 0xcd08, 0x86644, true)?)?;
    c.emit(0xcd0c, transfer_word(C::Al, true, r(2), r(15), 160)?)?;
    c.emit(0xcd10, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa9)?)?;
    c.emit(0xcd14, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0xcd18, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x800)?)?;
    c.emit(0xcd1c, branch(C::Al, 0xcd1c, 0x17b0, true)?)?;
    c.emit(0xcd20, data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    Ok(())
}

fn unlock_copy_or_free(c: &mut Code) -> Result<(), String> {
    c.emit(0xcd24, transfer_word(C::Al, true, r(1), r(5), 40)?)?;
    c.emit(0xcd28, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0xcd2c, branch(C::Eq, 0xcd2c, 0xcd40, false)?)?;
    c.emit(0xcd30, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd34, data_register(C::Al, Op::Mov, r(4), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd38, branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(0xcd3c, data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd40, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0xcd44, branch(C::Eq, 0xcd44, 0xcd74, false)?)?;
    c.emit(0xcd48, load_halfword(C::Al, r(0), r(10), -6)?)?;
    c.emit(0xcd4c, load_halfword(C::Al, r(1), r(10), -4)?)?;
    c.emit(0xcd50, data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd54, load_halfword(C::Al, r(1), r(10), -2)?)?;
    c.emit(0xcd58, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd5c, transfer_word(C::Al, true, r(1), r(13), 32)?)?;
    c.emit(0xcd60, load_halfword(C::Al, r(1), r(1), 0)?)?;
    c.emit(0xcd64, data_register(C::Al, Op::Eor, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd68, store_halfword(C::Al, r(0), r(10), -8)?)?;
    c.emit(0xcd6c, data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd70, branch(C::Al, 0xcd70, 0xc5b4, false)?)?;
    c.emit(0xcd74, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd78, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd7c, branch(C::Al, 0xcd7c, 0xb28c, true)?)?;
    c.emit(0xcd80, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd84, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0xcd88, branch(C::Eq, 0xcd88, 0xcdac, false)?)?;
    c.emit(0xcd8c, transfer_word(C::Al, true, r(2), r(13), 0)?)?;
    c.emit(0xcd90, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd94, data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0xcd98, branch(C::Al, 0xcd98, 0x471c, true)?)?;
    c.emit(0xcd9c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xcda0, data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0xcda4, branch(C::Al, 0xcda4, 0xbe70, true)?)?;
    c.emit(0xcda8, branch(C::Al, 0xcda8, 0xc5b4, false)?)?;
    c.emit(0xcdac, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0xcdb0, branch(C::Al, 0xcdb0, 0xc5b4, false)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    validate_reallocation(&mut c)?;
    validate_allocated_block(&mut c)?;
    expand_adjacent(&mut c)?;
    allocate_null(&mut c)?;
    shrink_and_split(&mut c)?;
    account_shrink(&mut c)?;
    split_adjacent_remainder(&mut c)?;
    merge_adjacent_and_account(&mut c)?;
    unlock_copy_or_free(&mut c)?;
    c.data(0xcdb4, SOURCE)?;
    c.data(0xcdb8, CALLBACK)?;
    c.data(0xcdbc, MARKER_TABLE)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("heap reallocation extent mismatch".into());
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
        for (at, value) in PROTECTED_DATA {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap()).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn data_requires_exact_address_value_pairs() {
        for (at, value) in PROTECTED_DATA {
            for wrong in [0, u32::MAX, value ^ 1, SOURCE, CALLBACK, MARKER_TABLE]
                .into_iter().filter(|wrong| *wrong != value)
            {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.data(at, wrong).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
            for wrong_at in [at - 1, at + 1, START, END] {
                let mut c = Code { at: wrong_at, bytes: Vec::new() };
                assert!(c.data(wrong_at, value).is_err());
                assert_eq!(c.at, wrong_at);
                assert!(c.bytes.is_empty());
            }
            let mut c = Code { at: at - 4, bytes: Vec::new() };
            assert!(c.data(at, value).is_err());
            assert_eq!(c.at, at - 4);
            assert!(c.bytes.is_empty());
            let mut c = Code { at, bytes: Vec::new() };
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes, value.to_le_bytes());
        }
    }

    #[test]
    fn code_requires_exact_cursor_and_extent() {
        let instruction = data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap();
        for at in [START - 4, START + 1, 0xcdb4, 0xcdb8, 0xcdbc, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, instruction).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, instruction).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
