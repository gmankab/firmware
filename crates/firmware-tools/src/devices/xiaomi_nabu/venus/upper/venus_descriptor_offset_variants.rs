use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, mov_register, multiply_register, push_registers,
    transfer_multiple, transfer_word, transfer_word_register,
    transfer_word_register_preindexed_writeback,
};

pub const START: usize = 0x118b8;
pub const END: usize = 0x120f4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;

fn r(n: u8) -> R { R::new(n).unwrap() }

fn saved(returning: bool) -> u16 {
    (4..=11).fold(if returning { R::PC.bit() } else { R::LR.bit() }, |mask, n| mask | r(n).bit())
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("descriptor offset variants at {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && ((START..0x11b4c).contains(&at)
            || (0x11b58..0x11e00).contains(&at)
            || (0x11e10..0x120e8).contains(&at))
    }

    fn emit(&mut self, word: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("descriptor offset instruction outside code".into());
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal_value(at: usize) -> Option<u32> {
        match at {
            0x11b4c | 0x11e00 | 0x120ec => Some(0xe3dfc),
            0x11b50 | 0x11e04 | 0x120f0 => Some(0xe2788),
            0x11b54 | 0x11e0c => Some(0xffffee6c),
            0x11e08 => Some(0xffffe21c),
            0x120e8 => Some(0xe3d16),
            _ => None,
        }
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::literal_value(at) != Some(value) {
            return Err("descriptor offset literal mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, dst: R, target: usize) -> Result<(), String> {
        if !matches!((self.at, target),
            (0x11994, 0x11b4c)
            | (0x119a0, 0x11b50)
            | (0x11aa0, 0x11b4c)
            | (0x11aac, 0x11b50)
            | (0x11ab8, 0x11b54)
            | (0x11b28, 0x11b4c)
            | (0x11b34, 0x11b50)
            | (0x11c38, 0x11e00)
            | (0x11c44, 0x11e04)
            | (0x11c50, 0x11e08)
            | (0x11d54, 0x11e00)
            | (0x11d60, 0x11e04)
            | (0x11d6c, 0x11e0c)
            | (0x11ddc, 0x11e00)
            | (0x11de8, 0x11e04)
            | (0x11e58, 0x120e8)
            | (0x11f2c, 0x120ec)
            | (0x11f34, 0x120f0)
            | (0x11fa0, 0x120ec)
            | (0x11fac, 0x120f0)
            | (0x11fe4, 0x120ec)
            | (0x12050, 0x120f0)
        ) || Self::literal_value(target).is_none() {
            return Err("invalid descriptor offset literal target".into());
        }
        self.emit(transfer_word(C::Al, true, dst, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn jump(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if !matches!((self.at, condition, target, link),
            (0x118d0, C::Eq, 0x11a10, false)
            | (0x1199c, C::Al, 0x2ec94, true)
            | (0x119a8, C::Al, 0x2ec94, true)
            | (0x119b4, C::Eq, 0x11b40, false)
            | (0x11a0c, C::Al, 0x11b1c, false)
            | (0x11aa8, C::Al, 0x2ec94, true)
            | (0x11ab4, C::Al, 0x2ec94, true)
            | (0x11ac4, C::Eq, 0x11b40, false)
            | (0x11b30, C::Al, 0x2ec94, true)
            | (0x11b3c, C::Al, 0x2ec94, true)
            | (0x11b70, C::Eq, 0x11cc4, false)
            | (0x11c40, C::Al, 0x2ec94, true)
            | (0x11c4c, C::Al, 0x2ec94, true)
            | (0x11c68, C::Eq, 0x11df4, false)
            | (0x11cc0, C::Al, 0x11dd0, false)
            | (0x11d5c, C::Al, 0x2ec94, true)
            | (0x11d68, C::Al, 0x2ec94, true)
            | (0x11d78, C::Eq, 0x11df4, false)
            | (0x11de4, C::Al, 0x2ec94, true)
            | (0x11df0, C::Al, 0x2ec94, true)
            | (0x11e28, C::Ne, 0x120dc, false)
            | (0x11e34, C::Eq, 0x120dc, false)
            | (0x11e94, C::Al, 0x17b0, true)
            | (0x11f30, C::Al, 0x2ec94, true)
            | (0x11f3c, C::Al, 0x2ec94, true)
            | (0x11fa8, C::Al, 0x2ec94, true)
            | (0x11fb4, C::Al, 0x2ec94, true)
            | (0x11fd0, C::Eq, 0x120dc, false)
            | (0x1204c, C::Al, 0x2ec94, true)
            | (0x1205c, C::Al, 0x2ec94, true)
            | (0x120cc, C::Al, 0x2ec94, true)
            | (0x120d8, C::Al, 0x2ec94, true)
        ) {
            return Err("invalid descriptor offset branch".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn indexed_offsets(c: &mut Code) -> Result<(), String> {
    c.check(0x118b8)?;
    c.emit(push_registers(C::Al, saved(false))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.jump(C::Eq, 0x11a10, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(4), 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0x9400)?)?;
    c.emit(transfer_word_register(C::Al, true, r(2), r(0), r(2), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x90)?)?;
    c.emit(multiply_register(C::Al, r(7), r(3), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(7), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(2), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(0), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(7), r(7), Shift::Lsl, 3, false)?)?;
    c.emit(mov_register(C::Al, r(7), r(4)))?;
    c.emit(data_register(C::Al, Op::Add, r(6), r(4), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 0xd30)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(6), 0xd38)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(4), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(9), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(7), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0xc)?)?;
    c.check(0x11994)?;
    c.literal(r(1), 0x11b4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x11b50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0xd28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x0)?)?;
    c.jump(C::Eq, 0x11b40, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(9)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0xff)?)?;
    c.jump(C::Al, 0x11b1c, false)?;
    c.check(0x11a10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(6), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0xca0)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0xca8)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(4), 0x6c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(6), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(4), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(5), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(6), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0xc)?)?;
    c.check(0x11aa0)?;
    c.literal(r(1), 0x11b4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x11b50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(0), 0x11b54)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x0)?)?;
    c.jump(C::Eq, 0x11b40, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(5)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.check(0x11b1c)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0xc)?)?;
    c.literal(r(1), 0x11b4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x11b50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.check(0x11b40)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    c.check(0x11b4c)?;
    c.data(0x11b4c, 0xe3dfc)?;
    c.data(0x11b50, 0xe2788)?;
    c.data(0x11b54, 0xffffee6c)?;
    Ok(())
}

fn previous_offsets(c: &mut Code) -> Result<(), String> {
    c.check(0x11b58)?;
    c.emit(push_registers(C::Al, saved(false))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.jump(C::Eq, 0x11cc4, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(mov_register(C::Al, r(7), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0x9400)?)?;
    c.emit(transfer_word_register(C::Al, true, r(2), r(0), r(2), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x90)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(2), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(0), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(4), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(9), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(7), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0xc)?)?;
    c.check(0x11c38)?;
    c.literal(r(1), 0x11e00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x11e04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(0), 0x11e08)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), -0x44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x0)?)?;
    c.jump(C::Eq, 0x11df4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(9)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0xff)?)?;
    c.jump(C::Al, 0x11dd0, false)?;
    c.check(0x11cc4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(6), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0xca0)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0xca8)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(4), 0x6c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(6), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(4), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(5), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(6), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0xc)?)?;
    c.check(0x11d54)?;
    c.literal(r(1), 0x11e00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x11e04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(0), 0x11e0c)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x0)?)?;
    c.jump(C::Eq, 0x11df4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(5)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xff)?)?;
    c.check(0x11dd0)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0xc)?)?;
    c.literal(r(1), 0x11e00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x11e04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.check(0x11df4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    c.check(0x11e00)?;
    c.data(0x11e00, 0xe3dfc)?;
    c.data(0x11e04, 0xe2788)?;
    c.data(0x11e08, 0xffffe21c)?;
    c.data(0x11e0c, 0xffffee6c)?;
    Ok(())
}

fn paired_offsets(c: &mut Code) -> Result<(), String> {
    c.check(0x11e10)?;
    c.emit(push_registers(C::Al, saved(false))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0xc)?)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.jump(C::Ne, 0x120dc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x0)?)?;
    c.jump(C::Eq, 0x120dc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x90)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(5), 0x1c)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(5), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0xc)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(3), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(0), r(2), Shift::Lsl, 2)?)?;
    c.literal(r(2), 0x120e8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(3), r(3), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x1cc)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(0), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.jump(C::Al, 0x17b0, true)?;
    c.check(0x11e98)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(7), r(5)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(mov_register(C::Al, r(9), r(5)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(7), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(6), r(4), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(10)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 0xc)?)?;
    c.literal(r(1), 0x120ec)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x120f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.check(0x11f40)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(mov_register(C::Al, r(6), r(4)))?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(10)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(1), 0xc)?)?;
    c.literal(r(1), 0x120ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x120f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x0)?)?;
    c.jump(C::Eq, 0x120dc, false)?;
    c.check(0x11fd4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x13)?)?;
    c.literal(r(8), 0x120ec)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0xff)?)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 0xc)?)?;
    c.emit(mov_register(C::Al, r(1), r(8)))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(10), 0x120f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(mov_register(C::Al, r(1), r(10)))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.check(0x12060)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(1), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x10)?)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(1), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(mov_register(C::Al, r(1), r(8)))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(mov_register(C::Al, r(1), r(10)))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.check(0x120dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    c.check(0x120e8)?;
    c.data(0x120e8, 0xe3d16)?;
    c.data(0x120ec, 0xe3dfc)?;
    c.data(0x120f0, 0xe2788)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    indexed_offsets(&mut c)?;
    previous_offsets(&mut c)?;
    paired_offsets(&mut c)?;
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
    fn structure_and_offsets() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2108);
        assert_eq!(bytes.len(), END - START);
        for (entry, exit, stack) in [
            (START, 0x11b48, 4), (0x11b58, 0x11dfc, 4), (0x11e10, 0x120e4, 12),
        ] {
            assert_eq!(word(&bytes, entry), push_registers(C::Al, saved(false)).unwrap());
            assert_eq!(word(&bytes, entry + 8), data_immediate(C::Al, Op::Sub, R::SP, R::SP, stack).unwrap());
            assert_eq!(word(&bytes, exit), transfer_multiple(C::Al, true, R::SP,
                saved(true), BlockMode::IncrementAfter, true).unwrap());
        }
        for (at, expected) in [
            (0x11b4c, 0xe3dfc), (0x11b50, 0xe2788), (0x11b54, 0xffffee6c),
            (0x11e00, 0xe3dfc), (0x11e04, 0xe2788), (0x11e08, 0xffffe21c),
            (0x11e0c, 0xffffee6c), (0x120e8, 0xe3d16), (0x120ec, 0xe3dfc),
            (0x120f0, 0xe2788),
        ] {
            assert_eq!(word(&bytes, at), expected);
        }
        assert_eq!(word(&bytes, 0x118f4), multiply_register(C::Al, r(7), r(3), r(1)).unwrap());
        assert_eq!(word(&bytes, 0x11b78), data_immediate(C::Al, Op::Sub, r(1), r(1), 1).unwrap());
        assert_eq!(word(&bytes, 0x11c60), transfer_word(C::Al, true, r(0), r(0), -0x44).unwrap());
        assert_eq!(word(&bytes, 0x11e94), branch(C::Al, 0x11e94, 0x17b0, true).unwrap());
        for at in [0x11988, 0x11a94, 0x11b1c, 0x11c2c, 0x11d48, 0x11dd0, 0x11f1c, 0x12038] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff).unwrap());
        }
        for (length, expected) in [(0u32, 1), (1, 0x101), (0x100, 0x101),
            (0xffff_ff00, 0xffff_ff01), (0xffff_ff01, 1), (u32::MAX, 1)] {
            assert_eq!((length.wrapping_add(0xff) & !0xff) | 1, expected);
        }
        let mut calls = 0;
        for at in (START..END).step_by(4).filter(|at| Code::is_code(*at)) {
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 == 0x0a00_0000 {
                let displacement = ((instruction << 8) as i32 >> 6) as i64;
                let target = (at as i64 + 8 + displacement) as usize;
                if instruction & 0x0100_0000 != 0 {
                    calls += 1;
                    assert!(matches!(target, 0x17b0 | 0x2ec94));
                } else {
                    assert!(Code::is_code(target));
                    assert!(if at < 0x11b4c { (START..0x11b4c).contains(&target) }
                        else if at < 0x11e00 { (0x11b58..0x11e00).contains(&target) }
                        else { (0x11e10..0x120e8).contains(&target) });
                }
            }
        }
        assert_eq!(calls, 21);
    }

    #[test]
    fn exact_guards() {
        for at in START - 4..=END + 4 {
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), Code::is_code(at));
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = Code::literal_value(at) {
                assert!(c.data(at, value ^ 1).is_err());
                assert!(c.bytes.is_empty());
                assert!(c.data(at, value).is_ok());
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(0x11b4c, 0xe3dfc).is_err());
        assert!(c.literal(r(0), 0x11b4c).is_err());
        for target in [START, START + 1, 0x11b4c, END, 0x17b0, 0x2ec94] {
            assert!(c.jump(C::Al, target, false).is_err());
            assert!(c.jump(C::Al, target, true).is_err());
        }
        assert!(c.bytes.is_empty());
        let mut c = Code { at: 0x11994, bytes: Vec::new() };
        assert!(c.literal(r(1), 0x11b50).is_err());
        assert!(c.literal(r(1), 0x11b4d).is_err());
        assert!(c.literal(r(1), 0x11b4c).is_ok());
        let mut c = Code { at: 0x118d0, bytes: Vec::new() };
        assert!(c.jump(C::Ne, 0x11a10, false).is_err());
        assert!(c.jump(C::Eq, 0x11a10, true).is_err());
        assert!(c.bytes.is_empty());
        assert!(c.jump(C::Eq, 0x11a10, false).is_ok());
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
