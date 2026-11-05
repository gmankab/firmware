use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, transfer_multiple, transfer_word,
    transfer_word_postindexed, transfer_word_register,
};

pub const START: usize = 0x7f3fc;
pub const END: usize = 0x7f7bc;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 940;
pub const DATA_BYTES: usize = 20;
pub const FUNCTION_STARTS: [usize; 2] = [START, 0x7f600];
const PROTECTED_DATA: [(usize, u32); 5] = [
    (0x7f5f0, 0xf478b),
    (0x7f5f4, 0xf4896),
    (0x7f5f8, 0xf48bf),
    (0x7f5fc, 0xe2788),
    (0x7f7b8, 0xf478b),
];
const EDGES: &[(usize, C, usize, bool)] = &[
    (0x7f434, C::Ls, 0x7f4a4, false),
    (0x7f444, C::Al, 0x2efc, true),
    (0x7f450, C::Eq, 0x7f5b4, false),
    (0x7f464, C::Al, 0x31ec, true),
    (0x7f47c, C::Al, 0x471c, true),
    (0x7f484, C::Al, 0x2f1c, true),
    (0x7f49c, C::Cc, 0x7f490, false),
    (0x7f4b0, C::Eq, 0x7f4cc, false),
    (0x7f4d8, C::Eq, 0x7f5a8, false),
    (0x7f4e0, C::Cs, 0x7f5a8, false),
    (0x7f520, C::Al, 0x17b0, true),
    (0x7f55c, C::Al, 0x17b0, true),
    (0x7f5a4, C::Cc, 0x7f500, false),
    (0x7f5c8, C::Al, 0x2ec94, true),
    (0x7f5d4, C::Al, 0x2ec94, true),
    (0x7f5ec, C::Al, 0x17b0, false),
    (0x7f620, C::Al, 0x2bf4, true),
    (0x7f63c, C::Ls, 0x7f6c4, false),
    (0x7f64c, C::Hi, 0x7f640, false),
    (0x7f65c, C::Al, 0x2efc, true),
    (0x7f668, C::Eq, 0x7f7a0, false),
    (0x7f67c, C::Al, 0x31ec, true),
    (0x7f694, C::Al, 0x471c, true),
    (0x7f69c, C::Al, 0x2f1c, true),
    (0x7f6ac, C::Eq, 0x7f6c8, false),
    (0x7f6c0, C::Cc, 0x7f6b4, false),
    (0x7f6d0, C::Eq, 0x7f6e8, false),
    (0x7f6f4, C::Eq, 0x7f78c, false),
    (0x7f6fc, C::Cs, 0x7f78c, false),
    (0x7f738, C::Al, 0x17b0, true),
    (0x7f788, C::Cc, 0x7f718, false),
    (0x7f79c, C::Al, 0x2c34, false),
    (0x7f7b4, C::Al, 0x17b0, false),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VBUF attach address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && ((START..0x7f5f0).contains(&at) || (0x7f600..0x7f7b8).contains(&at))
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !Self::is_code(at) {
            return Err("VBUF attach instruction outside CODE".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !PROTECTED_DATA.contains(&(at, value)) {
            return Err("VBUF attach invalid literal address/value".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, at: usize, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.check(at)?;
        if !EDGES.contains(&(at, condition, target, link)) {
            return Err("VBUF attach invalid branch edge".into());
        }
        self.emit(at, branch(condition, at as u32, target as u32, link)?)
    }
}

fn attach2(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f3fc, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x7f400, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(0x7f404, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x14)?)?;
    c.emit(0x7f408, transfer_word(C::Al, false, r(2), R::SP, 0x4)?)?;
    c.emit(0x7f40c, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f410, data_register(C::Al, Op::Mov, r(5), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f414, transfer_word(C::Al, true, r(2), r(4), 0xc)?)?;
    c.emit(0x7f418, transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(0x7f41c, transfer_word(C::Al, false, r(1), R::SP, 0x10)?)?;
    c.emit(0x7f420, transfer_word(C::Al, true, r(8), r(11), 0xc)?)?;
    c.emit(0x7f424, data_register(C::Al, Op::Add, r(9), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f428, transfer_word(C::Al, true, r(1), r(11), 0x8)?)?;
    c.emit(0x7f42c, data_register(C::Al, Op::Cmp, r(0), r(9), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f430, transfer_word(C::Al, false, r(1), R::SP, 0xc)?)?;
    c.branch(0x7f434, C::Ls, 0x7f4a4, false)?;
    c.emit(0x7f438, data_register(C::Al, Op::Add, r(0), r(9), r(9), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f43c, data_register(C::Al, Op::Mov, r(10), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x7f440, data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f444, C::Al, 0x2efc, true)?;
    c.emit(0x7f448, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f44c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(0x7f450, C::Eq, 0x7f5b4, false)?;
    c.emit(0x7f454, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f458, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x7f45c, data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f460, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.branch(0x7f464, C::Al, 0x31ec, true)?;
    c.emit(0x7f468, transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(0x7f46c, transfer_word(C::Al, true, r(1), r(4), 0x4)?)?;
    c.emit(0x7f470, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f474, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x7f478, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f47c, C::Al, 0x471c, true)?;
    c.emit(0x7f480, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.branch(0x7f484, C::Al, 0x2f1c, true)?;
    c.emit(0x7f488, transfer_word(C::Al, false, r(6), r(4), 0x4)?)?;
    c.emit(0x7f48c, transfer_word(C::Al, false, r(9), r(4), 0x10)?)?;
    c.emit(0x7f490, transfer_word_postindexed(C::Al, false, r(7), r(6), 0x28)?)?;
    c.emit(0x7f494, data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(0x7f498, data_register(C::Al, Op::Cmp, r(0), r(7), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f49c, C::Cc, 0x7f490, false)?;
    c.emit(0x7f4a0, data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f4a4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(0x7f4a8, data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x1)?)?;
    c.emit(0x7f4ac, transfer_word(C::Al, false, r(1), R::SP, 0x8)?)?;
    c.branch(0x7f4b0, C::Eq, 0x7f4cc, false)?;
    c.emit(0x7f4b4, transfer_word(C::Al, true, r(1), r(4), 0x18)?)?;
    c.emit(0x7f4b8, transfer_word(C::Al, true, r(2), R::SP, 0x10)?)?;
    c.emit(0x7f4bc, data_register(C::Al, Op::Add, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f4c0, transfer_word(C::Al, false, r(1), r(4), 0x18)?)?;
    c.emit(0x7f4c4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x7f4c8, transfer_word(C::Al, false, r(1), R::SP, 0x8)?)?;
    c.emit(0x7f4cc, transfer_word(C::Al, true, r(1), R::SP, 0x10)?)?;
    c.emit(0x7f4d0, transfer_word(C::Al, true, r(10), r(4), 0xc)?)?;
    c.emit(0x7f4d4, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(0x7f4d8, C::Eq, 0x7f5a8, false)?;
    c.emit(0x7f4dc, data_register(C::Al, Op::Cmp, r(0), r(10), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f4e0, C::Cs, 0x7f5a8, false)?;
    c.emit(0x7f4e4, data_register(C::Al, Op::Add, r(0), r(10), r(10), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f4e8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x14)?)?;
    c.emit(0x7f4ec, transfer_word(C::Al, true, r(6), R::PC, 0xfc)?)?;
    c.emit(0x7f4f0, transfer_word(C::Al, true, r(9), R::SP, 0xc)?)?;
    c.emit(0x7f4f4, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(0x7f4f8, data_register(C::Al, Op::Add, r(8), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x7f4fc, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(0x7f500, data_register(C::Al, Op::Add, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f504, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5b)?)?;
    c.emit(0x7f508, data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f50c, transfer_word(C::Al, true, r(0), r(0), -0x8)?)?;
    c.emit(0x7f510, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.emit(0x7f514, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f518, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f51c, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(0x7f520, C::Al, 0x17b0, true)?;
    c.emit(0x7f524, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(0x7f528, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x7f52c, data_register(C::Al, Op::Add, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f530, transfer_word(C::Al, false, r(2), r(1), -0x8)?)?;
    c.emit(0x7f534, transfer_word(C::Al, true, r(2), R::SP, 0x8)?)?;
    c.emit(0x7f538, transfer_word(C::Al, false, r(2), r(1), -0x10)?)?;
    c.emit(0x7f53c, data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f540, transfer_word(C::Al, true, r(1), R::SP, 0x4)?)?;
    c.emit(0x7f544, transfer_word_register(C::Al, false, r(1), r(0), r(8), Shift::Lsl, 0)?)?;
    c.emit(0x7f548, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x62)?)?;
    c.emit(0x7f54c, transfer_word(C::Al, true, r(0), r(5), 0x0)?)?;
    c.emit(0x7f550, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.emit(0x7f554, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f558, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(0x7f55c, C::Al, 0x17b0, true)?;
    c.emit(0x7f560, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(0x7f564, transfer_word_postindexed(C::Al, true, r(2), r(5), 0x4)?)?;
    c.emit(0x7f568, data_immediate(C::Al, Op::Add, r(10), r(10), 0x1)?)?;
    c.emit(0x7f56c, data_register(C::Al, Op::Add, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f570, transfer_word(C::Al, false, r(2), r(1), -0x4)?)?;
    c.emit(0x7f574, transfer_word(C::Al, true, r(2), R::SP, 0xc)?)?;
    c.emit(0x7f578, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x7f57c, transfer_word(C::Ne, true, r(2), r(9), 0x0)?)?;
    c.emit(0x7f580, transfer_word(C::Ne, false, r(2), r(1), 0x4)?)?;
    c.emit(0x7f584, transfer_word(C::Al, false, r(4), r(1), 0x10)?)?;
    c.emit(0x7f588, transfer_word(C::Al, true, r(1), R::SP, 0x10)?)?;
    c.emit(0x7f58c, data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f590, transfer_word(C::Cc, true, r(1), r(4), 0x10)?)?;
    c.emit(0x7f594, data_immediate(C::Cc, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(0x7f598, data_immediate(C::Cc, Op::Add, r(9), r(9), 0x4)?)?;
    c.emit(0x7f59c, data_immediate(C::Cc, Op::Add, r(8), r(8), 0x28)?)?;
    c.emit(0x7f5a0, data_register(C::Cc, Op::Cmp, r(0), r(10), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f5a4, C::Cc, 0x7f500, false)?;
    c.emit(0x7f5a8, transfer_word(C::Al, false, r(10), r(4), 0xc)?)?;
    c.emit(0x7f5ac, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(0x7f5b0, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn attach2_allocation_failure(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f5b4, transfer_word(C::Al, true, r(1), R::PC, 0x38)?)?;
    c.emit(0x7f5b8, transfer_word(C::Al, true, r(2), R::PC, 0x38)?)?;
    c.emit(0x7f5bc, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x37)?)?;
    c.emit(0x7f5c0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(0x7f5c4, data_immediate(C::Al, Op::Orr, r(3), r(3), 0x300)?)?;
    c.branch(0x7f5c8, C::Al, 0x2ec94, true)?;
    c.emit(0x7f5cc, transfer_word(C::Al, true, r(1), R::PC, 0x28)?)?;
    c.emit(0x7f5d0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(0x7f5d4, C::Al, 0x2ec94, true)?;
    c.emit(0x7f5d8, transfer_word(C::Al, true, r(2), R::PC, 0x10)?)?;
    c.emit(0x7f5dc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f5e0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x338)?)?;
    c.emit(0x7f5e4, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(0x7f5e8, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(0x7f5ec, C::Al, 0x17b0, false)?;
    Ok(())
}

fn attach_contiguous_locked(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f600, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x7f604, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(0x7f608, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0xc)?)?;
    c.emit(0x7f60c, transfer_word(C::Al, false, r(3), R::SP, 0x4)?)?;
    c.emit(0x7f610, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f614, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f618, data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f61c, transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.branch(0x7f620, C::Al, 0x2bf4, true)?;
    c.emit(0x7f624, transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(0x7f628, transfer_word(C::Al, true, r(7), r(4), 0x10)?)?;
    c.emit(0x7f62c, transfer_word(C::Al, false, r(5), R::SP, 0x8)?)?;
    c.emit(0x7f630, transfer_word(C::Al, true, r(10), r(11), 0x8)?)?;
    c.emit(0x7f634, data_register(C::Al, Op::Add, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f638, data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f63c, C::Ls, 0x7f6c4, false)?;
    c.emit(0x7f640, data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f644, data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(0x7f648, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(0x7f64c, C::Hi, 0x7f640, false)?;
    c.emit(0x7f650, data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f654, data_register(C::Al, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 4, false)?)?;
    c.emit(0x7f658, data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f65c, C::Al, 0x2efc, true)?;
    c.emit(0x7f660, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f664, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(0x7f668, C::Eq, 0x7f7a0, false)?;
    c.emit(0x7f66c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f670, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x7f674, data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f678, data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.branch(0x7f67c, C::Al, 0x31ec, true)?;
    c.emit(0x7f680, transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(0x7f684, transfer_word(C::Al, true, r(1), r(4), 0x4)?)?;
    c.emit(0x7f688, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f68c, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x7f690, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f694, C::Al, 0x471c, true)?;
    c.emit(0x7f698, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.branch(0x7f69c, C::Al, 0x2f1c, true)?;
    c.emit(0x7f6a0, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(0x7f6a4, transfer_word(C::Al, false, r(5), r(4), 0x4)?)?;
    c.emit(0x7f6a8, transfer_word(C::Al, false, r(7), r(4), 0x10)?)?;
    c.branch(0x7f6ac, C::Eq, 0x7f6c8, false)?;
    c.emit(0x7f6b0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f6b4, transfer_word_postindexed(C::Al, false, r(0), r(5), 0x28)?)?;
    c.emit(0x7f6b8, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x7f6bc, data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f6c0, C::Cc, 0x7f6b4, false)?;
    c.emit(0x7f6c4, data_register(C::Al, Op::Mov, r(9), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f6c8, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.emit(0x7f6cc, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x1)?)?;
    c.branch(0x7f6d0, C::Eq, 0x7f6e8, false)?;
    c.emit(0x7f6d4, transfer_word(C::Al, true, r(0), r(4), 0x18)?)?;
    c.emit(0x7f6d8, transfer_word(C::Al, true, r(1), R::SP, 0x8)?)?;
    c.emit(0x7f6dc, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(0x7f6e0, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f6e4, transfer_word(C::Al, false, r(0), r(4), 0x18)?)?;
    c.emit(0x7f6e8, transfer_word(C::Al, true, r(0), R::SP, 0x8)?)?;
    c.emit(0x7f6ec, transfer_word(C::Al, true, r(5), r(4), 0xc)?)?;
    c.emit(0x7f6f0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(0x7f6f4, C::Eq, 0x7f78c, false)?;
    c.emit(0x7f6f8, data_register(C::Al, Op::Cmp, r(0), r(5), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f6fc, C::Cs, 0x7f78c, false)?;
    c.emit(0x7f700, data_register(C::Al, Op::Add, r(0), r(5), r(5), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f704, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x14)?)?;
    c.emit(0x7f708, data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(0x7f70c, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(0x7f710, data_register(C::Al, Op::Add, r(9), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x7f714, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(0x7f718, data_register(C::Al, Op::Add, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f71c, transfer_word(C::Al, true, r(2), R::PC, 0x94)?)?;
    c.emit(0x7f720, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xbe)?)?;
    c.emit(0x7f724, transfer_word(C::Al, true, r(0), r(0), -0x8)?)?;
    c.emit(0x7f728, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.emit(0x7f72c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f730, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f734, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(0x7f738, C::Al, 0x17b0, true)?;
    c.emit(0x7f73c, transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(0x7f740, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x7f744, data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(0x7f748, data_register(C::Al, Op::Add, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f74c, transfer_word(C::Al, false, r(2), r(1), -0x8)?)?;
    c.emit(0x7f750, transfer_word(C::Al, false, r(8), r(1), -0x10)?)?;
    c.emit(0x7f754, transfer_word_register(C::Al, false, r(6), r(0), r(9), Shift::Lsl, 0)?)?;
    c.emit(0x7f758, transfer_word(C::Al, true, r(2), R::SP, 0x4)?)?;
    c.emit(0x7f75c, transfer_word(C::Al, true, r(2), r(2), 0x0)?)?;
    c.emit(0x7f760, data_register(C::Al, Op::Add, r(2), r(10), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f764, transfer_word(C::Al, false, r(2), r(1), -0x4)?)?;
    c.emit(0x7f768, transfer_word(C::Al, false, r(4), r(1), 0x10)?)?;
    c.emit(0x7f76c, transfer_word(C::Al, true, r(1), R::SP, 0x8)?)?;
    c.emit(0x7f770, data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f774, transfer_word(C::Cc, true, r(1), r(4), 0x10)?)?;
    c.emit(0x7f778, data_register(C::Cc, Op::Add, r(10), r(10), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f77c, data_immediate(C::Cc, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(0x7f780, data_immediate(C::Cc, Op::Add, r(9), r(9), 0x28)?)?;
    c.emit(0x7f784, data_register(C::Cc, Op::Cmp, r(0), r(5), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(0x7f788, C::Cc, 0x7f718, false)?;
    c.emit(0x7f78c, transfer_word(C::Al, false, r(5), r(4), 0xc)?)?;
    c.emit(0x7f790, transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(0x7f794, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(0x7f798, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(0x7f79c, C::Al, 0x2c34, false)?;
    Ok(())
}

fn contiguous_allocation_failure(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f7a0, transfer_word(C::Al, true, r(2), R::PC, 0x10)?)?;
    c.emit(0x7f7a4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f7a8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x39c)?)?;
    c.emit(0x7f7ac, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(0x7f7b0, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(0x7f7b4, C::Al, 0x17b0, false)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    attach2(&mut c)?;
    attach2_allocation_failure(&mut c)?;
    for (at, value) in PROTECTED_DATA.into_iter().take(4) {
        c.data(at, value)?;
    }
    c.check(FUNCTION_STARTS[1])?;
    attach_contiguous_locked(&mut c)?;
    contiguous_allocation_failure(&mut c)?;
    c.data(0x7f7b8, 0xf478b)?;
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("VBUF attach extent mismatch".into());
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
    fn structure_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, END - START), (0x7f3fc, 0x7f7bc, 960));
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!((CODE_BYTES, DATA_BYTES), (235 * 4, 5 * 4));
        assert_eq!(FUNCTION_STARTS, [START, 0x7f600]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 235);
        assert_eq!(EDGES.len(), 33);
        for &(at, condition, target, link) in EDGES {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
            assert!(TypedA32::is_code(at));
            if (START..END).contains(&target) {
                assert!(TypedA32::is_code(target));
            }
        }
        assert!(!EDGES.iter().any(|&(at, _, target, _)| at < 0x7f600 && [0x2bf4, 0x2c34].contains(&target)));
        assert!(EDGES.contains(&(0x7f620, C::Al, 0x2bf4, true)));
        assert!(EDGES.contains(&(0x7f79c, C::Al, 0x2c34, false)));
        assert!(EDGES.contains(&(0x7f450, C::Eq, 0x7f5b4, false)));
        assert!(EDGES.contains(&(0x7f668, C::Eq, 0x7f7a0, false)));
        assert_eq!(word(&bytes, 0x7f640), data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x7f644), data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false).unwrap());
        assert_eq!(word(&bytes, 0x7f648), data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false).unwrap());
        assert_eq!(word(&bytes, 0x7f64c), branch(C::Hi, 0x7f64c, 0x7f640, false).unwrap());
        for (at, value) in PROTECTED_DATA {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn exact_code_and_cursor_guards() {
        let instruction = data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap();
        for at in (START - 4..END + 8).chain([0, usize::MAX]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            let valid = at % 4 == 0 && ((START..0x7f5f0).contains(&at) || (0x7f600..0x7f7b8).contains(&at));
            assert_eq!(c.emit(at, instruction).is_ok(), valid);
            assert_eq!(c.at, if valid { at + 4 } else { at });
            assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, instruction).is_err());
        assert!(c.check(END).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn exact_literal_guards() {
        for (at, value) in PROTECTED_DATA {
            for wrong in [0, u32::MAX, value ^ 1, 0xf478b, 0xf4896, 0xf48bf, 0xe2788]
                .into_iter().filter(|wrong| *wrong != value)
            {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.data(at, wrong).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
            for wrong_at in [at - 1, at + 1, START, END] {
                let mut c = TypedA32 { at: wrong_at, bytes: Vec::new() };
                assert!(c.data(wrong_at, value).is_err());
                assert_eq!(c.at, wrong_at);
                assert!(c.bytes.is_empty());
            }
            let mut c = TypedA32 { at: at - 4, bytes: Vec::new() };
            assert!(c.data(at, value).is_err());
            assert_eq!(c.at, at - 4);
            assert!(c.bytes.is_empty());
            let mut c = TypedA32 { at, bytes: Vec::new() };
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes, value.to_le_bytes());
        }
    }

    #[test]
    fn exact_branch_guards() {
        for &(at, condition, target, link) in EDGES {
            for (source, cc, dest, linked) in [
                (at, condition, target, !link),
                (at, if condition == C::Al { C::Eq } else { C::Al }, target, link),
                (at, condition, target + 4, link),
                (at + 1, condition, target, link),
                (0x7f5f0, condition, target, link),
                (0x7f7b8, condition, target, link),
                (END, condition, target, link),
            ] {
                let mut c = TypedA32 { at: source, bytes: Vec::new() };
                assert!(c.branch(source, cc, dest, linked).is_err());
                assert_eq!(c.at, source);
                assert!(c.bytes.is_empty());
            }
            let mut c = TypedA32 { at: at + 4, bytes: Vec::new() };
            assert!(c.branch(at, condition, target, link).is_err());
            assert_eq!(c.at, at + 4);
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn authenticated_same_buffer_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
