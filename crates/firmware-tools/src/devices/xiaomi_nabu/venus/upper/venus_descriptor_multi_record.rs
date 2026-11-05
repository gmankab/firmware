use arm32::{
    branch, data_immediate, data_register, transfer_byte, transfer_multiple,
    transfer_word, transfer_word_register_preindexed_writeback, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x10ee4;
pub const END: usize = 0x118b8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const POOL_START: usize = 0x118b0;
pub const DATA_BYTES: usize = 8;
pub const CODE_BYTES: usize = END - START - DATA_BYTES;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("multi-record address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0x118b0 => Some(0xe3dfc),
            0x118b4 => Some(0xe2788),
            _ => None,
        }
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..POOL_START).contains(&self.at) || self.at % 4 != 0 {
            return Err("multi-record instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) {
            return Err("multi-record protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, destination: R, at: usize) -> Result<(), String> {
        if Self::data_value(at).is_none() {
            return Err("invalid multi-record literal target".into());
        }
        self.emit(transfer_word(C::Al, true, destination, R::PC,
            at as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if (link && target != 0x2ec94)
            || (!link && (!(START..POOL_START).contains(&target) || target % 4 != 0))
        {
            return Err("invalid multi-record branch target".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn entry(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x10ee4)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 820)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 28)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 816)?)?;
    c.emit(data_register(C::Al, Op::Bic, r(10), r(2), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.branch(C::Ne, 0x11040, false)?;
    Ok(())
}

fn primary_extended_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x10f24)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 3384)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(8), 248)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(8), 3376)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(8), 3380)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(8), 3388)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(6), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22c)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(2), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(2), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x28c)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(5), r(7), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(10), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(5), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x10)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(12), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(10), 0x2)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(2), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(2), 12)?)?;
    c.branch(C::Al, 0x11144, false)?;
    Ok(())
}

fn primary_simple_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x11040)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 3376)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(8), 3384)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 248)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(10), 0x2)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(1), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(6), r(3), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(3), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(3), 8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0x10)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(3), 12)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(10), 0x2)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(2), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(2), 12)?)?;
    Ok(())
}

fn primary_logging(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x11144)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 3368)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x113bc, false)?;
    Ok(())
}

fn secondary_mode(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x1116c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 816)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x11298, false)?;
    Ok(())
}

fn secondary_extended_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x1117c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x22c)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(8), 3384)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 3388)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(8), 3376)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(8), 248)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(13), 0)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(5), r(7), r(6), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(10), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(3), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(1), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(5), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x10)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(10), 0x3)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(2), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(2), 12)?)?;
    c.branch(C::Al, 0x113a0, false)?;
    Ok(())
}

fn secondary_simple_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x11298)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(8), 3384)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(8), 248)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(13), 0)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(6), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(10), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(1), 12)?)?;
    c.literal(r(1), 0x118b0)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(10), 0x3)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(2), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(2), 12)?)?;
    Ok(())
}

fn secondary_logging(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x113a0)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn optional_gate(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x113bc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 2648)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 820)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(1), 0x2)?)?;
    c.branch(C::Eq, 0x118a4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 816)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x11514, false)?;
    Ok(())
}

fn optional_extended_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x113d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x22c)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(8), 3448)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(8), 3452)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 3456)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(8), 252)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(8), 3460)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(6), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(5), r(4), r(12), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x12)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(10), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(5), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(5), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x10)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 12)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(5), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
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
    c.emit(transfer_word(C::Al, false, r(10), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 12)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x1163c, false)?;
    Ok(())
}

fn optional_simple_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x11514)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(8), 3448)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 3456)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 252)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(10), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(6), r(3), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(3), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(3), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0x10)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(9), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(3), 12)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(4), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
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
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 12)?)?;
    c.literal(r(1), 0x118b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    Ok(())
}

fn optional_secondary_gate(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x1163c)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 3440)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x118a4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 816)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x11780, false)?;
    Ok(())
}

fn optional_secondary_extended_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x1165c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 3456)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(8), 252)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(8), 3460)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 3448)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(6), r(8), r(6), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(6), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(6), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(6), 12)?)?;
    c.literal(r(6), 0x118b0)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(4), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x11890, false)?;
    Ok(())
}

fn optional_secondary_simple_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x11780)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(8), 3456)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(8), 252)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.literal(r(9), 0x118b0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(8), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 12)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(4), 0x118b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    Ok(())
}

fn final_logging(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x11890)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn finish(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x118a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    entry(&mut c)?;
    primary_extended_records(&mut c)?;
    primary_simple_records(&mut c)?;
    primary_logging(&mut c)?;
    secondary_mode(&mut c)?;
    secondary_extended_records(&mut c)?;
    secondary_simple_records(&mut c)?;
    secondary_logging(&mut c)?;
    optional_gate(&mut c)?;
    optional_extended_records(&mut c)?;
    optional_simple_records(&mut c)?;
    optional_secondary_gate(&mut c)?;
    optional_secondary_extended_records(&mut c)?;
    optional_secondary_simple_records(&mut c)?;
    final_logging(&mut c)?;
    finish(&mut c)?;
    c.check(POOL_START)?;
    c.data(0x118b0, 0xe3dfc)?;
    c.data(0x118b4, 0xe2788)?;
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
    fn structure_and_record_semantics() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2516);
        assert_eq!(CODE_BYTES, 2508);
        assert_eq!(word(&bytes, POOL_START), 0xe3dfc);
        assert_eq!(word(&bytes, POOL_START + 4), 0xe2788);
        let saved = (4..=11).fold(R::PC.bit(), |mask, n| mask | r(n).bit());
        assert_eq!(word(&bytes, 0x118ac), transfer_multiple(C::Al, true,
            R::SP, saved, BlockMode::IncrementAfter, true).unwrap());
        for (at, condition, target, link) in [
            (0x10f20, C::Ne, 0x11040, false),
            (0x10fcc, C::Al, 0x2ec94, true),
            (0x10fd8, C::Al, 0x2ec94, true),
            (0x1103c, C::Al, 0x11144, false),
            (0x110d4, C::Al, 0x2ec94, true),
            (0x110e0, C::Al, 0x2ec94, true),
            (0x11150, C::Al, 0x2ec94, true),
            (0x1115c, C::Al, 0x2ec94, true),
            (0x11168, C::Ne, 0x113bc, false),
            (0x11178, C::Ne, 0x11298, false),
            (0x11224, C::Al, 0x2ec94, true),
            (0x11230, C::Al, 0x2ec94, true),
            (0x11294, C::Al, 0x113a0, false),
            (0x11330, C::Al, 0x2ec94, true),
            (0x1133c, C::Al, 0x2ec94, true),
            (0x113ac, C::Al, 0x2ec94, true),
            (0x113b8, C::Al, 0x2ec94, true),
            (0x113c8, C::Eq, 0x118a4, false),
            (0x113d4, C::Ne, 0x11514, false),
            (0x11484, C::Al, 0x2ec94, true),
            (0x11494, C::Al, 0x2ec94, true),
            (0x11504, C::Al, 0x2ec94, true),
            (0x11510, C::Al, 0x1163c, false),
            (0x115b0, C::Al, 0x2ec94, true),
            (0x115c0, C::Al, 0x2ec94, true),
            (0x11630, C::Al, 0x2ec94, true),
            (0x1163c, C::Al, 0x2ec94, true),
            (0x11648, C::Ne, 0x118a4, false),
            (0x11658, C::Ne, 0x11780, false),
            (0x11700, C::Al, 0x2ec94, true),
            (0x11710, C::Al, 0x2ec94, true),
            (0x1177c, C::Al, 0x11890, false),
            (0x11814, C::Al, 0x2ec94, true),
            (0x11824, C::Al, 0x2ec94, true),
            (0x11894, C::Al, 0x2ec94, true),
            (0x118a0, C::Al, 0x2ec94, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        for (at, dst, base, offset) in [
            (0x10f48, r(0), r(4), r(0)),
            (0x10f70, r(5), r(7), r(1)),
            (0x11060, r(0), r(7), r(0)),
            (0x11084, r(3), r(6), r(3)),
            (0x111b0, r(0), r(4), r(0)),
            (0x111c0, r(5), r(7), r(6)),
            (0x112c0, r(0), r(7), r(0)),
            (0x112d4, r(1), r(6), r(1)),
            (0x11414, r(0), r(7), r(0)),
            (0x11420, r(5), r(4), r(12)),
            (0x11544, r(0), r(7), r(0)),
            (0x11564, r(3), r(6), r(3)),
            (0x1168c, r(0), r(7), r(0)),
            (0x116b0, r(6), r(8), r(6)),
            (0x117ac, r(0), r(7), r(0)),
            (0x117bc, r(1), r(8), r(1)),
        ] {
            assert_eq!(word(&bytes, at), transfer_word_register_preindexed_writeback(
                C::Al, true, dst, base, offset, Shift::Lsl, 0).unwrap());
        }
        for (at, dst, target) in [
            (0x10fac, r(1), 0x118b0),
            (0x10fd0, r(1), 0x118b4),
            (0x110b8, r(1), 0x118b0),
            (0x110d8, r(1), 0x118b4),
            (0x11144, r(1), 0x118b0),
            (0x11154, r(1), 0x118b4),
            (0x11204, r(1), 0x118b0),
            (0x11228, r(1), 0x118b4),
            (0x1132c, r(1), 0x118b0),
            (0x11334, r(1), 0x118b4),
            (0x113a0, r(1), 0x118b0),
            (0x113b0, r(1), 0x118b4),
            (0x11468, r(1), 0x118b0),
            (0x11488, r(5), 0x118b4),
            (0x114fc, r(1), 0x118b0),
            (0x11594, r(1), 0x118b0),
            (0x115b4, r(4), 0x118b4),
            (0x11628, r(1), 0x118b0),
            (0x116f8, r(6), 0x118b0),
            (0x11704, r(4), 0x118b4),
            (0x1179c, r(9), 0x118b0),
            (0x11818, r(4), 0x118b4),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, dst, R::PC,
                target as i32 - at as i32 - 8).unwrap());
        }
        for (at, op, src, value) in [
            (0x10f10, Op::Cmp, r(0), 1),
            (0x11164, Op::Cmp, r(0), 1),
            (0x11174, Op::Cmp, r(0), 1),
            (0x113c4, Op::Tst, r(1), 2),
            (0x113d0, Op::Cmp, r(0), 1),
            (0x11644, Op::Cmp, r(0), 1),
            (0x11654, Op::Cmp, r(0), 1),
        ] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, op, r(0), src, value).unwrap());
        }
        assert_eq!(word(&bytes, 0x10f0c), data_register(C::Al, Op::Bic,
            r(10), r(2), r(1), Shift::Lsl, 3, false).unwrap());
        assert_eq!(word(&bytes, 0x113c0), transfer_byte(C::Al, true, r(1), r(0), 0x334).unwrap());
        for (length, expected) in [(0u32, 1), (1, 0x101), (0x100, 0x101),
            (0xffff_ff00, 0xffff_ff01), (0xffff_ff01, 1), (u32::MAX, 1)] {
            assert_eq!((length.wrapping_add(0xff) & !0xff) | 1, expected);
        }
    }

    #[test]
    fn exact_data_and_control_flow_guards() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), at < POOL_START);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = TypedA32::data_value(at) {
                assert!(c.data(at, value ^ 1).is_err());
                assert!(c.bytes.is_empty());
                c.data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        for at in [START - 4, START + 1, POOL_START + 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, 0xe3dfc).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(POOL_START, 0xe3dfc).is_err());
        assert!(c.data(POOL_START + 4, 0xe2788).is_err());
        for target in [START, POOL_START - 4, POOL_START + 1, END] {
            assert!(c.literal(r(0), target).is_err());
        }
        for target in [START - 4, START + 1, POOL_START, POOL_START + 4, END] {
            assert!(c.branch(C::Al, target, false).is_err());
        }
        for target in [START, 0x2ec90, 0x2ec95] {
            assert!(c.branch(C::Al, target, true).is_err());
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
