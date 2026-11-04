use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, mov_register, push_registers, transfer_multiple,
    transfer_word, transfer_word_register_preindexed_writeback,
};

pub const START: usize = 0xfce4;
pub const END: usize = 0xffbc;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const POOL_START: usize = 0xffb4;
const ARGUMENT_LOG: u32 = 0xe3dfc;
const LENGTH_LOG: u32 = 0xe2788;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("descriptor writer at {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, word: u32) -> Result<(), String> {
        if !(START..POOL_START).contains(&self.at) || self.at % 4 != 0 {
            return Err("descriptor instruction outside code".into());
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal_value(at: usize) -> Option<u32> {
        match at {
            0xffb4 => Some(ARGUMENT_LOG),
            0xffb8 => Some(LENGTH_LOG),
            _ => None,
        }
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::literal_value(at) != Some(value) {
            return Err("descriptor literal mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, dst: R, at: usize) -> Result<(), String> {
        if Self::literal_value(at).is_none() {
            return Err("invalid descriptor literal target".into());
        }
        self.emit(transfer_word(C::Al, true, dst, R::PC, at as i32 - self.at as i32 - 8)?)
    }

    fn jump(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if (link && target != 0x2ec94)
            || (!link && (!(START..POOL_START).contains(&target) || target % 4 != 0))
        {
            return Err("invalid descriptor branch target".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn r(n: u8) -> R {
    R::new(n).unwrap()
}

fn saved(returning: bool) -> u16 {
    (4..=11).fold(if returning { R::PC.bit() } else { R::LR.bit() }, |mask, n| mask | r(n).bit())
}

fn entry(c: &mut Code) -> Result<(), String> {
    c.check(0xfce4)?;
    c.emit(push_registers(C::Al, saved(false))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 4)?)?;
    c.emit(mov_register(C::Al, r(9), r(0)))?;
    c.emit(mov_register(C::Al, r(6), r(2)))?;
    c.emit(mov_register(C::Al, r(8), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xa58)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x330)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.jump(C::Eq, 0xfdac, false)?;
    Ok(())
}

fn extended_first_record(c: &mut Code) -> Result<(), String> {
    c.check(0xfd0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(6), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(6), 0xc)?)?;
    c.emit(mov_register(C::Al, r(4), r(9)))?;
    c.emit(data_immediate(C::Al, Op::Mov, R::LR, R::R0, 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::LR, R::LR, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_multiple(C::Al, true, r(10), r(2).bit() | r(7).bit() | r(10).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(4), r(3), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::LR, r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(8), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(5), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(7), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(3), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, R::LR, r(3), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(3), 0xc)?)?;
    c.jump(C::Al, 0xfe3c, false)?;
    Ok(())
}

fn simple_first_record(c: &mut Code) -> Result<(), String> {
    c.check(0xfdac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), R::R0, 0x22c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, R::LR, r(6), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0)?)?;
    c.emit(mov_register(C::Al, r(5), r(9)))?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(7), r(5), r(12), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), R::R0, 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(12), r(12), 0x1000)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(8), R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(7), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(7), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0xc)?)?;
    Ok(())
}

fn log_first_record(c: &mut Code) -> Result<(), String> {
    c.check(0xfe3c)?;
    c.literal(r(1), 0xffb4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0xffb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 1)?)?;
    c.jump(C::Ne, 0xffa8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xa58)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x330)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.jump(C::Eq, 0xff10, false)?;
    Ok(())
}

fn extended_second_record(c: &mut Code) -> Result<(), String> {
    c.check(0xfe74)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(transfer_word(C::Al, true, r(7), r(6), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x208)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(8), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(5), r(5), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(2), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(6), r(9), r(6), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(6), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x288)?)?;
    c.emit(mov_register(C::Al, r(2), r(3)))?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x10)?)?;
    c.literal(r(1), 0xffb4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0xc)?)?;
    c.jump(C::Al, 0xff94, false)?;
    Ok(())
}

fn simple_second_record(c: &mut Code) -> Result<(), String> {
    c.check(0xff10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x228)?)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x23)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x9400)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x9400)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(9), r(3), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), R::R0, 0x28c)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(3), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(3), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x288)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x9400)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(3), 0x10)?)?;
    c.literal(r(1), 0xffb4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(2), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(3), 0xc)?)?;
    Ok(())
}

fn finish(c: &mut Code) -> Result<(), String> {
    c.check(0xff94)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0xffb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 1)?)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    entry(&mut c)?;
    extended_first_record(&mut c)?;
    simple_first_record(&mut c)?;
    log_first_record(&mut c)?;
    extended_second_record(&mut c)?;
    simple_second_record(&mut c)?;
    finish(&mut c)?;
    c.check(POOL_START)?;
    c.data(0xffb4, ARGUMENT_LOG)?;
    c.data(0xffb8, LENGTH_LOG)?;
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
        assert_eq!(bytes.len(), 728);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(word(&bytes, 0xffb4), ARGUMENT_LOG);
        assert_eq!(word(&bytes, 0xffb8), LENGTH_LOG);
        assert_eq!(word(&bytes, 0xffb0), transfer_multiple(C::Al, true, R::SP,
            saved(true), BlockMode::IncrementAfter, true).unwrap());
        for (at, condition, target, link) in [
            (0xfd08, C::Eq, 0xfdac, false), (0xfda8, C::Al, 0xfe3c, false),
            (0xfe44, C::Al, 0x2ec94, true), (0xfe50, C::Al, 0x2ec94, true),
            (0xfe5c, C::Ne, 0xffa8, false), (0xfe70, C::Eq, 0xff10, false),
            (0xff0c, C::Al, 0xff94, false), (0xff98, C::Al, 0x2ec94, true),
            (0xffa4, C::Al, 0x2ec94, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        for (at, dst, base, offset) in [
            (0xfd3c, 0, 1, 0), (0xfd50, 3, 4, 3),
            (0xfdd8, 0, 1, 0), (0xfde4, 7, 5, 12),
            (0xfea4, 0, 1, 0), (0xfeb8, 6, 9, 6),
            (0xff34, 0, 1, 0), (0xff44, 3, 9, 3),
        ] {
            assert_eq!(word(&bytes, at), transfer_word_register_preindexed_writeback(
                C::Al, true, r(dst), r(base), r(offset), Shift::Lsl, 0).unwrap());
        }
        for (at, dst, src) in [
            (0xfd78, 7, 0), (0xfd90, 0, 3), (0xfe0c, 3, 0), (0xfe24, 0, 7),
            (0xfed4, 2, 0), (0xfef0, 0, 6), (0xff64, 7, 0), (0xff78, 0, 3),
        ] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Add, r(dst), r(src), 0x10).unwrap());
        }
        for (at, src) in [(0xfd98, 2), (0xfe2c, 2), (0xfefc, 3), (0xff84, 2)] {
            for (offset, op, source, immediate) in [
                (0, Op::Add, r(src), 0xff), (4, Op::Bic, r(0), 0xff),
                (8, Op::Orr, r(0), 1),
            ] {
                assert_eq!(word(&bytes, at + offset),
                    data_immediate(C::Al, op, r(0), source, immediate).unwrap());
            }
        }
        for (length, expected) in [(0u32, 1), (1, 0x101), (0x100, 0x101),
            (0xffff_ff00, 0xffff_ff01), (0xffff_ff01, 1), (u32::MAX, 1)] {
            assert_eq!((length.wrapping_add(0xff) & !0xff) | 1, expected);
        }
        assert_eq!(0x228 | 0x1c00, 0x1e28);
        assert_eq!(0x22c | 0x1c00, 0x1e2c);
    }

    #[test]
    fn exact_data_guards() {
        for at in (START..END).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), at < POOL_START);
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = Code::literal_value(at) {
                assert!(c.data(at, value).is_ok());
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, ARGUMENT_LOG).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(POOL_START, ARGUMENT_LOG).is_err());
        assert!(c.literal(r(0), START).is_err());
        assert!(c.literal(r(0), POOL_START + 1).is_err());
        for target in [START + 1, POOL_START, END] {
            assert!(c.jump(C::Al, target, false).is_err());
        }
        assert!(c.jump(C::Al, START, true).is_err());
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
