use arm32::{
    branch, data_immediate, data_register, transfer_multiple, transfer_word,
    transfer_word_preindexed_writeback, transfer_word_register,
    transfer_word_register_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xee38;
pub const END: usize = 0xfa0c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const POOL_START: usize = 0xf9c8;
pub const DATA_BYTES: usize = 84;
pub const CODE_BYTES: usize = END - START - DATA_BYTES;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("descriptor state address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0xef9c => Some(0xefac),
            0xefa0 => Some(0xf7ec),
            0xefa4 => Some(0xefdc),
            0xefa8 => Some(0xf010),
            0xf9c8 => Some(0xe3d16),
            0xf9cc => Some(0xe3dce),
            0xf9d0 => Some(0xe2788),
            0xf9d4 => Some(0xe3da1),
            0xf9d8 => Some(0xe3d74),
            0xf9dc => Some(0xe3d42),
            0xf9e0 => Some(0xe3dfc),
            0xf9e4 => Some(0xe3dff),
            0xf9e8 => Some(0xe3e46),
            0xf9ec => Some(0xe3e8e),
            0xf9f0 => Some(0xe3ed6),
            0xf9f4 => Some(0xe3f1e),
            0xf9f8 => Some(0xe3f60),
            0xf9fc => Some(0xe3fa8),
            0xfa00 => Some(0xe3fef),
            0xfa04 => Some(0xe4037),
            0xfa08 => Some(0xe407f),
            _ => None,
        }
    }

    fn word(&mut self, value: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err("invalid descriptor state address".into());
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
        if !(POOL_START..END).contains(&at) || Self::data_value(at).is_none() {
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

fn select_descriptor_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xee38)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x24)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 12)?)?;
    c.emit(transfer_word_register(C::Al, true, r(5), r(0), r(1), Shift::Lsl, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1f0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(5), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Hi, 0xee74, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0xef44, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 2304)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0xee98, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(8), r(8), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x98)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(0), 0xc00)?)?;
    c.branch(C::Al, 0xeecc, false)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(8), r(8), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 2184)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(6), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(1), r(7), 3224)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2bc)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 2184)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0xeee0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0xeee0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x2)?)?;
    c.branch(C::Al, 0xeee4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xc5)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(0), r(7), 108)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), -80)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), -92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0xef34, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Cc, 0xef60, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0xef4c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 2452)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(0), 0x1)?)?;
    c.branch(C::Al, 0xef60, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 28)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.branch(C::Al, 0xf060, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.branch(C::Al, 0xee78, false)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 724)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0xef80, false)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2ec)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 68)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.branch(C::Hi, 0xf048, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(15), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(0), r(1), Shift::Lsl, 0)?)?;
    c.data(0xef9c, 0xefac)?;
    c.data(0xefa0, 0xf7ec)?;
    c.data(0xefa4, 0xefdc)?;
    c.data(0xefa8, 0xf010)?;
    Ok(())
}

fn select_descriptor_mode(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xefac)?;
    c.literal(C::Al, r(1), 0xf9dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xf5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, 0xf058, false)?;
    c.literal(C::Al, r(1), 0xf9d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 728)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Hi, 0xf88c, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 1700)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 1768)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0x6e0)?)?;
    c.branch(C::Al, 0xf89c, false)?;
    c.literal(C::Al, r(1), 0xf9cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 728)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Hi, 0xf8f0, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 1704)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 1840)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x328)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.branch(C::Al, 0xf8fc, false)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3d4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    Ok(())
}

fn initialize_descriptor_fields(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xf060)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 16)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3d8)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    Ok(())
}

fn write_primary_descriptors(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xf0d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(6), r(1), Shift::Lsl, 0)?)?;
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
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0xf9e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(7), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn write_secondary_descriptors(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xf190)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0xf250, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0xf9e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(9), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn write_extended_descriptors(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xf250)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.branch(C::Eq, 0xf518, false)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe5)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.literal(C::Al, r(9), 0xf9e0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 4)?)?;
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
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(5), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
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
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 4)?)?;
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
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0xf518, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.literal(C::Al, r(8), 0xf9e0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(9), r(9), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(13), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(5), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn write_optional_descriptors(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xf518)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(C::Eq, 0xf7e0, false)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 16)?)?;
    c.literal(C::Al, r(2), 0xf9c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xfa)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 4)?)?;
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
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0xf9e0)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(10), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 4)?)?;
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
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.literal(C::Al, r(1), 0xf9e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xfa00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.branch(C::Eq, 0xf7e0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.literal(C::Al, r(8), 0xf9e0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(10), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xfa04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xfa08)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn cold_descriptor_modes(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xf7ec)?;
    c.literal(C::Al, r(1), 0xf9d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xf9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Hi, 0xf820, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, r(7), r(6), 0x70)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(8), r(8), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 1768)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 1700)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(6), r(3), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(3), 3232)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(3), 3240)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 12)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(7), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(6), 728)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 1760)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x2)?)?;
    c.branch(C::Cs, 0xf970, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 24)?)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(1).bit() | r(2).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 16)?)?;
    c.branch(C::Al, 0xf0a8, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 1984)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 2252)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0x3b8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 1704)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 1840)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(6), 1832)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0xf99c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.branch(C::Al, 0xf0a0, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 2056)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 2256)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x800)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 1708)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 1912)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(6), 1904)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(5), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(8), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 32)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(10), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(0), 0)?)?;
    c.branch(C::Al, 0xf0a8, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(6), 1984)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(6), 2252)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(6), 1976)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(10), r(4), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xf0a8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 2056)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(6), 2256)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(6), 2048)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(10), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xf0a8, false)?;
    Ok(())
}
pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    select_descriptor_state(&mut c)?;
    select_descriptor_mode(&mut c)?;
    initialize_descriptor_fields(&mut c)?;
    write_primary_descriptors(&mut c)?;
    write_secondary_descriptors(&mut c)?;
    write_extended_descriptors(&mut c)?;
    write_optional_descriptors(&mut c)?;
    cold_descriptor_modes(&mut c)?;
    c.data(0xf9c8, 0xe3d16)?;
    c.data(0xf9cc, 0xe3dce)?;
    c.data(0xf9d0, 0xe2788)?;
    c.data(0xf9d4, 0xe3da1)?;
    c.data(0xf9d8, 0xe3d74)?;
    c.data(0xf9dc, 0xe3d42)?;
    c.data(0xf9e0, 0xe3dfc)?;
    c.data(0xf9e4, 0xe3dff)?;
    c.data(0xf9e8, 0xe3e46)?;
    c.data(0xf9ec, 0xe3e8e)?;
    c.data(0xf9f0, 0xe3ed6)?;
    c.data(0xf9f4, 0xe3f1e)?;
    c.data(0xf9f8, 0xe3f60)?;
    c.data(0xf9fc, 0xe3fa8)?;
    c.data(0xfa00, 0xe3fef)?;
    c.data(0xfa04, 0xe4037)?;
    c.data(0xfa08, 0xe407f)?;
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
    fn extent_data_return_and_cold_code() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 3028);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        let mut data_bytes = 0;
        for at in (START..END).step_by(4) {
            if let Some(value) = TypedA32::data_value(at) {
                assert_eq!(word(&bytes, at), value);
                data_bytes += 4;
            }
        }
        assert_eq!(data_bytes, DATA_BYTES);
        assert_eq!(word(&bytes, 0xee98), data_register(C::Al, Op::Orr,
            r(0), r(8), r(8), Shift::Lsl, 3, false).unwrap());
        assert_eq!(word(&bytes, 0xf7e8), transfer_multiple(C::Al, true, R::SP,
            (4..=11).fold(R::PC.bit(), |mask, n| mask | r(n).bit()),
            BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0xf7ec), transfer_word(C::Al, true,
            r(1), R::PC, 0xf9d8 - 0xf7ec - 8).unwrap());
        for (at, condition, target) in [(0xef44 + 4, C::Al, 0xee78),
            (0xeffc, C::Hi, 0xf88c), (0xf030, C::Hi, 0xf8f0),
            (0xf858, C::Cs, 0xf970), (0xf8c0, C::Cs, 0xf99c),
            (0xf888, C::Al, 0xf0a8), (0xf8ec, C::Al, 0xf0a0),
            (0xf96c, C::Al, 0xf0a8), (0xf998, C::Al, 0xf0a8),
            (0xf9c4, C::Al, 0xf0a8)] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        for at in (0xf7ec..POOL_START).step_by(4) {
            assert!(TypedA32::data_value(at).is_none());
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(word(&bytes, at)).is_ok());
        }
    }

    #[test]
    fn protected_data_alignment_and_targets() {
        for at in (START..END).step_by(4) {
            let value = TypedA32::data_value(at);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), value.is_none());
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = value {
                assert!(c.data(at, value).is_ok());
            }
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(POOL_START, 0xe3d16).is_err());
        for at in [START, 0xef9c, POOL_START + 1, END] {
            assert!(c.literal(C::Al, r(0), at).is_err());
        }
        for target in [START + 1, 0xef9c, 0xefa0, 0xefa4, 0xefa8, POOL_START, END - 4] {
            assert!(c.branch(C::Al, target as u32, false).is_err());
        }
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
