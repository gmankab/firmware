use arm32::{branch, data_immediate, data_register, data_register_shifted_by_register, multiply_register, transfer_multiple, transfer_word, transfer_word_register, transfer_word_register_preindexed_writeback, BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x120f4;
pub const END: usize = 0x12e5c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const DATA_BYTES: usize = 96;
pub const CODE_BYTES: usize = END - START - DATA_BYTES;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("descriptor dispatch address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0x126e8 => Some(0xe3dfc),
            0x126ec => Some(0xe2788),
            0x126f0 => Some(0xe3d16),
            0x12780 => Some(0x12790),
            0x12784 => Some(0x12810),
            0x12788 => Some(0x1284c),
            0x1278c => Some(0x128a0),
            0x12800 => Some(0x12798),
            0x12804 => Some(0x12818),
            0x12808 => Some(0x12854),
            0x1280c => Some(0x128e4),
            0x12c40 => Some(0xffffede8),
            0x12c44 => Some(0xe3d16),
            0x12c48 => Some(0xe414f),
            0x12c4c => Some(0xe2788),
            0x12c50 => Some(0xe4121),
            0x12c54 => Some(0xe40f4),
            0x12c58 => Some(0xe40c7),
            0x12c5c => Some(0xe417d),
            0x12c60 => Some(0xe41c7),
            0x12e4c => Some(0xfbfeeffc),
            0x12e50 => Some(0x7ffff),
            0x12e54 => Some(0xe3d16),
            0x12e58 => Some(0xffffec2c),
            _ => None,
        }
    }

    fn word(&mut self, value: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err("invalid descriptor dispatch address".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if Self::data_value(self.at).is_some() {
            return Err("instruction overlaps protected descriptor DATA".into());
        }
        self.word(instruction)
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) {
            return Err("incorrect protected descriptor DATA".into());
        }
        self.word(value)
    }

    fn literal(&mut self, condition: C, destination: R, at: usize) -> Result<(), String> {
        if Self::data_value(at).is_none() || (0x12780..0x12790).contains(&at) || (0x12800..0x12810).contains(&at) {
            return Err("invalid descriptor literal address".into());
        }
        self.emit(transfer_word(condition, true, destination, R::PC,
            at as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        if target % 4 != 0 || Self::data_value(target as usize).is_some() {
            return Err("invalid descriptor branch target".into());
        }
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
}

fn write_descriptor_planes(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x120f4)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 144)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 76)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(2), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x12564, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 28)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 12)?)?;
    c.emit(transfer_word_register(C::Al, true, r(7), r(0), r(1), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 816)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x122b0, false)?;
    c.literal(C::Al, r(2), 0x126f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(4), r(6), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(6), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x22c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x12)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(3), 16)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x1c00)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 3384)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(0), 3376)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(0), 3380)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 3388)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(3), r(3), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(7), r(3), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x228)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x1c00)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(3), 8)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(7), r(8), r(7), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(2), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x208)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(6), r(4), r(6), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(6), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(6), 8)?)?;
    c.literal(C::Al, r(1), 0x126e8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(6), 12)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(6), 0x126ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0x126e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x123f0, false)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(6), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(6), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x12)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 16)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1c00)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(4), r(2), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(0), 3376)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 3384)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(7), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(7), r(3), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x28c)?)?;
    c.literal(C::Al, r(1), 0x126e8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(2), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(3), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(2), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(3), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(2), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(3), 12)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(5), 0x126ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0x126e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 3440)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x126dc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 816)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x12598, false)?;
    c.literal(C::Al, r(2), 0x126f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1a)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x208)?)?;
    c.literal(C::Al, r(8), 0x126e8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(3), r(3), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(0), 3376)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 3384)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(0), 3388)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(4), r(3), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x13)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 16)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(3), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x228)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(6), r(3), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(3), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x22c)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(7), r(10), r(7), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(3), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(3), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(3), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(7), 12)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(7), 0x126ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(1), 12)?)?;
    c.branch(C::Al, 0x126c0, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x12118, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 60)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 68)?)?;
    c.emit(multiply_register(C::Al, r(7), r(1), r(2))?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x126dc, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 72)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(multiply_register(C::Al, r(6), r(1), r(2))?)?;
    c.branch(C::Al, 0x1211c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.literal(C::Al, r(8), 0x126e8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(0), 3384)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(4), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 16)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(10), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(9), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(7), 0x126ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x126e8, 0xe3dfc)?;
    c.data(0x126ec, 0xe2788)?;
    c.data(0x126f0, 0xe3d16)?;
    Ok(())
}

fn dispatch_descriptor_tail(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x126f4)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x1c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2ac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(6), r(0), Shift::Lsl, 0)?)?;
    c.literal(C::Al, r(0), 0x12c40)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x12760, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1f0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(2), r(8), r(2), Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, r(2), r(2), r(3), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x2)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(3), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(2), r(3), r(3), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(10), r(2), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x94)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(2), 0xc00)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 68)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(2), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x3)?)?;
    c.branch(C::Hi, 0x127cc, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(15), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(2), r(3), Shift::Lsl, 0)?)?;
    c.data(0x12780, 0x12790)?;
    c.data(0x12784, 0x12810)?;
    c.data(0x12788, 0x1284c)?;
    c.data(0x1278c, 0x128a0)?;
    c.emit(transfer_word(C::Al, true, r(7), r(10), 2452)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(1), 0x12c50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 24)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x12c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1640)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 1708)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 1904)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1912)?)?;
    c.branch(C::Al, 0x12884, false)?;
    c.literal(C::Al, r(2), 0x12c44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x7a)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.branch(C::Hi, 0x12bf4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(15), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(0), r(1), Shift::Lsl, 0)?)?;
    c.data(0x12800, 0x12798)?;
    c.data(0x12804, 0x12818)?;
    c.data(0x12808, 0x12854)?;
    c.data(0x1280c, 0x128e4)?;
    c.emit(transfer_word(C::Al, true, r(7), r(10), 2452)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(1), 0x12c54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 24)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x12c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1636)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 1704)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 1832)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1840)?)?;
    c.branch(C::Al, 0x12884, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(10), 2452)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(1), 0x12c58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 24)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x12c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 1632)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 1700)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 1760)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1768)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(8), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 16)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x1295c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x128dc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1f0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(8), r(0), Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(1), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(10), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x94)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xc00)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(1), 0x12c48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x12c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0x6c)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x12934, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1f0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(1), r(8), r(1), Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, r(1), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.emit(data_immediate(C::Cc, Op::Add, r(0), r(10), 0x70)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 2200)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 1600)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 16)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 12)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(5), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(10), 2192)?)?;
    c.emit(data_register(C::Al, Op::Add, r(8), r(3), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 16)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x7)?)?;
    c.branch(C::Eq, 0x12ab8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(2), 0x12c44)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 4)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2e8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x204)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x9600)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(5), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x238)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(4), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x284)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1680)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x8000)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0x12c5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x12c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x12ab8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x204)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x33)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9600)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x284)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(1), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1680)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x8000)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(2), 0x12c44)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xf2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x204)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x42)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x284)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1680)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x9600)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x8000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(5), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x238)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(10), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0x12c60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x12c4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x12be8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x204)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x43)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 16)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.literal(C::Al, r(2), 0x12c44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xdf)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 16)?)?;
    c.branch(C::Al, 0x1295c, false)?;
    c.data(0x12c40, 0xffffede8)?;
    c.data(0x12c44, 0xe3d16)?;
    c.data(0x12c48, 0xe414f)?;
    c.data(0x12c4c, 0xe2788)?;
    c.data(0x12c50, 0xe4121)?;
    c.data(0x12c54, 0xe40f4)?;
    c.data(0x12c58, 0xe40c7)?;
    c.data(0x12c5c, 0xe417d)?;
    c.data(0x12c60, 0xe41c7)?;
    Ok(())
}

fn write_descriptor_control(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x12c64)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(1), 2648)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(2), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(3), 868)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x8c00)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x22c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(2), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(2), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(14), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0)?)?;
    c.branch(C::Al, 0x12cc4, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn write_descriptor_masks(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x12cc4)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.literal(C::Al, r(1), 0x12e4c)?;
    c.literal(C::Al, r(3), 0x12e50)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 2648)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x12)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 928)?)?;
    c.branch(C::Hi, 0x12d18, false)?;
    c.emit(data_register(C::Al, Op::And, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x44000)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::And, r(2), r(7), r(3), Shift::Lsr, r(2), false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.branch(C::Eq, 0x12d18, false)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x10)?)?;
    c.branch(C::Al, 0x12d24, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Eq, Op::Orr, r(1), r(1), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(8), 0x12e54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(7), r(2), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x8c00)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(4), r(2), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(0), 932)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(0), 936)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(0), 940)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Tst, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xc00)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xc90)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x8000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(9), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.literal(C::Al, r(0), 0x12e58)?;
    c.emit(transfer_word_register(C::Al, true, r(6), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(6), 0x3b0)?)?;
    c.emit(transfer_multiple(C::Al, true, r(2), r(0).bit() | r(1).bit() | r(2).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Tst, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xeb)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xc00)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(6), 0x3b0)?)?;
    c.emit(transfer_multiple(C::Al, true, r(2), r(0).bit() | r(1).bit() | r(2).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(3), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x94)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x8c00)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(3), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(3), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x12e4c, 0xfbfeeffc)?;
    c.data(0x12e50, 0x7ffff)?;
    c.data(0x12e54, 0xe3d16)?;
    c.data(0x12e58, 0xffffec2c)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    write_descriptor_planes(&mut c)?;
    dispatch_descriptor_tail(&mut c)?;
    write_descriptor_control(&mut c)?;
    write_descriptor_masks(&mut c)?;
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
    fn extent_functions_and_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 3432);
        assert_eq!(DATA_BYTES, 96);
        assert_eq!(CODE_BYTES, 3336);
        let data_count = (START..END).step_by(4).filter(|&at| {
            if let Some(value) = TypedA32::data_value(at) {
                assert_eq!(word(&bytes, at), value);
                true
            } else { false }
        }).count();
        assert_eq!(data_count * 4, DATA_BYTES);
        assert_eq!(bytes.len() / 4 - data_count, 834);
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        write_descriptor_planes(&mut c).unwrap();
        c.check(0x126f4).unwrap();
        dispatch_descriptor_tail(&mut c).unwrap();
        c.check(0x12c64).unwrap();
        write_descriptor_control(&mut c).unwrap();
        c.check(0x12cc4).unwrap();
        write_descriptor_masks(&mut c).unwrap();
        c.check(END).unwrap();
        assert_eq!(c.bytes, bytes);
        for at in [0x126e4, 0x12bf0, 0x12e48] {
            assert_eq!(word(&bytes, at), transfer_multiple(C::Al, true, R::SP,
                (4..=11).fold(R::PC.bit(), |mask, n| mask | r(n).bit()),
                BlockMode::IncrementAfter, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x12cc0), transfer_multiple(C::Al, true, R::SP,
            r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true).unwrap());
        for (at, condition, target, link) in [
            (0x127f0, C::Hi, 0x12bf4, false),
            (0x12c3c, C::Al, 0x1295c, false),
            (0x12cb8, C::Al, 0x12cc4, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x12bf4), transfer_word(C::Al, true, r(2), R::PC, 0x48).unwrap());
        for at in (0x12bf4..0x12c40).step_by(4) {
            assert!(TypedA32::data_value(at).is_none());
        }
        for (at, base, index) in [(0x1277c, r(2), r(3)), (0x127fc, r(0), r(1))] {
            assert_eq!(word(&bytes, at), transfer_word_register(C::Al, true,
                R::PC, base, index, Shift::Lsl, 0).unwrap());
        }
    }

    #[test]
    fn exact_data_and_address_guards() {
        for at in (START..END).step_by(4) {
            let value = TypedA32::data_value(at);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), value.is_none());
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = value {
                assert!(c.data(at, value).is_ok());
                let mut c = TypedA32 { at: START, bytes: Vec::new() };
                assert!(c.branch(C::Al, at as u32, false).is_err());
                assert!(c.bytes.is_empty());
            }
        }
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(0x126e8, 0xe3dfc).is_err());
        for at in [START, 0x12780, 0x12784, 0x12788, 0x1278c,
            0x12800, 0x12804, 0x12808, 0x1280c, 0x12e4d, END] {
            assert!(c.literal(C::Al, r(0), at).is_err());
        }
        assert!(c.branch(C::Al, START as u32 + 1, false).is_err());
        assert!(c.bytes.is_empty());
        for at in (START..END).step_by(4).filter(|&at| {
            TypedA32::data_value(at).is_some()
                && !(0x12780..0x12790).contains(&at)
                && !(0x12800..0x12810).contains(&at)
        }) {
            let mut c = TypedA32 { at: START, bytes: Vec::new() };
            assert!(c.literal(C::Al, r(0), at).is_ok());
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
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
