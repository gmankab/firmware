use arm32::{
    branch, data_immediate, data_register, mov_register,
    signed_multiply_accumulate_bottom_halves, transfer_byte,
    transfer_byte_preindexed_writeback, transfer_word, transfer_word_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6e8b4;
pub const END: usize = 0x6ec04;
const CODE_END: usize = 0x6ec00;
const DIVIDE_BY_FIVE_RECIPROCAL: u32 = 0xcccc_cccd;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("QP commit address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..CODE_END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("QP commit instruction outside code at 0x{:x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if (CODE_END..END).contains(&target) {
            return Err("QP commit branch into literal data".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn literal(&mut self, condition: C, destination: R, target: usize) -> Result<(), String> {
        self.emit(transfer_word(condition, true, destination, R::PC,
            target as i32 - self.at as i32 - 8)?)
    }

    fn checked_data(&mut self) -> Result<(), String> {
        self.check(CODE_END)?;
        if self.bytes.len() != CODE_END - START {
            return Err("QP commit code size mismatch".into());
        }
        self.bytes.extend_from_slice(&DIVIDE_BY_FIVE_RECIPROCAL.to_le_bytes());
        self.at += 4;
        self.check(END)
    }
}

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(transfer_word(C::Al, true, r(4), R::R11, 0x8)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), r(9), 0x2d0)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 0xab8)?)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 0x44)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne, 0x6e8e8, false)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x2d2)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d8)?)?;
    c.emit(mov_register(C::Al, R::R0, r(9)))?;
    c.branch(C::Al, 0x70128, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), R::R0, 0)?)?;
    c.check(0x6e8e8)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 0xab8)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq, 0x6e9e8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(4), 0x3dc)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(10), 0xa4)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R0, 0xdf)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.emit(transfer_byte(C::Eq, true, R::R1, r(4), 0x2f4)?)?;
    c.emit(transfer_byte(C::Al, false, R::R1, r(9), 0x2d0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(3), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x7f)?)?;
    c.emit(data_immediate(C::Ne, Op::And, R::R1, R::R1, 0xff)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, R::R1, R::R1, 0x2)?)?;
    c.emit(transfer_byte(C::Ne, false, R::R1, r(9), 0x2d0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 0)?)?;
    c.literal(C::Al, r(6), 0x6f5fc)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(7), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Gt, 0x6e95c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0x2)?)?;
    c.branch(C::Eq, 0x6e9d4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0x2000)?)?;
    c.branch(C::Ne, 0x6e9d8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x33)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(3), R::R0, 0x3f)?)?;
    c.branch(C::Al, 0x6e9d8, false)?;
    c.check(0x6e95c)?;
    c.emit(data_immediate(C::Al, Op::Cmn, R::R0, r(7), 0xf8000001)?)?;
    c.literal(C::Ne, R::R2, 0x6f600)?;
    c.emit(data_register(C::Ne, Op::Cmp, R::R0, r(7), R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x6e9d8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x1f)?)?;
    c.branch(C::Al, 0x6e9d8, false)?;
    c.check(0x6e974)?;
    c.emit(mov_register(C::Al, R::R0, r(9)))?;
    c.emit(transfer_word(C::Al, true, r(4), R::R11, 0x8)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, R::R1, R::R0, 0x2d0)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::R0, 0x38c)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, R::R2, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R0, 0x330)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(12), 0x838)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne, 0x6e8bc, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0x78)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x4)?)?;
    c.branch(C::Cc, 0x6e8bc, false)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 0x780)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne, 0x6e8bc, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.branch(C::Eq, 0x6f160, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Ne, 0x6f168, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(9), 0x700)?)?;
    c.branch(C::Al, 0x6f170, false)?;
    c.check(0x6e9d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x33)?)?;
    c.check(0x6e9d8)?;
    c.emit(data_immediate(C::Al, Op::And, R::R2, r(3), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Hi, false, r(3), r(9), 0x2d0)?)?;
    c.check(0x6e9e8)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(9), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq, 0x6ea6c, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x34)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), r(3), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(3), R::R2, Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0x6ea10, false)?;
    c.check(0x6ea08)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x2d0)?)?;
    c.emit(transfer_byte(C::Al, false, R::R2, r(9), 0x2d1)?)?;
    c.check(0x6ea10)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 0xcc)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(9), 0x780)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.branch(C::Eq, 0x6ea7c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Eq, 0x6eaac, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Ne, 0x6eadc, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1000)?)?;
    c.branch(C::Ne, 0x6ebb0, false)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0xa84)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 0xa89)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0xa8a)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), R::R11, 0x1d)?)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), R::R11, 0x38)?)?;
    c.branch(C::Al, 0x69c3c, true)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 0xa84)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(9), 0x92)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xa00)?)?;
    c.branch(C::Al, 0x6eba4, false)?;
    c.check(0x6ea6c)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x2)?)?;
    c.branch(C::Cc, 0x6ea08, false)?;
    c.branch(C::Al, 0x6ea10, false)?;
    c.check(0x6ea7c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq, 0x6eaf4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xb30)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x400)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(9), R::R0, Shift::Lsl, 0)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, R::R0, R::R1, R::R2, R::R0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x400)?)?;
    c.branch(C::Al, 0x6eaf8, false)?;
    c.check(0x6eaac)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq, 0x6eb34, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xb30)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x400)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(9), R::R0, Shift::Lsl, 0)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, R::R0, R::R1, R::R2, R::R0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), R::R0, 0x400)?)?;
    c.branch(C::Al, 0x6eb38, false)?;
    c.check(0x6eadc)?;
    c.literal(C::Al, R::R2, 0x6f5e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8f)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x6ebb0, false)?;
    c.check(0x6eaf4)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(9), 0xa80)?)?;
    c.check(0x6eaf8)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x2c)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, R::R0, 0x530)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, R::R1, R::R2))?;
    c.emit(transfer_byte(C::Al, false, R::R1, R::R0, 0x52e)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x2d0)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R1, R::R2, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, R::R0, 0x52f)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, R::R1, R::R2))?;
    c.emit(transfer_byte(C::Al, false, R::R1, R::R0, 0x52c)?)?;
    c.branch(C::Al, 0x6ebb0, false)?;
    c.check(0x6eb34)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(9), 0xa80)?)?;
    c.check(0x6eb38)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 0x2d0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2c)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(4), 0x2a0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, R::R1, R::R0))?;
    c.emit(transfer_byte(C::Al, false, R::R1, r(4), 0x29e)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x2d0)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(4), 0x29f)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, R::R0, R::R2))?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(4), 0x29c)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x1000)?)?;
    c.branch(C::Ne, 0x6ebb0, false)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(4), 0x294)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), R::R11, 0x1d)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R1, 0xff)?)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), R::R11, 0x38)?)?;
    c.branch(C::Al, 0x69c3c, true)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(4), 0x294)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(4), 0xa2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x200)?)?;
    c.check(0x6eba4)?;
    c.emit(transfer_byte(C::Al, true, R::R2, R::R11, -0x1d)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, R::R11, -0x38)?)?;
    c.branch(C::Al, 0x69cd8, true)?;
    c.check(0x6ebb0)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2000)?)?;
    c.branch(C::Eq, 0x6ebd4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1000)?)?;
    c.branch(C::Ne, 0x6ebe0, false)?;
    c.literal(C::Al, R::R1, 0x6f608)?;
    c.emit(mov_register(C::Al, R::R0, r(9)))?;
    c.branch(C::Al, 0x70260, true)?;
    c.branch(C::Al, 0x6ebe0, false)?;
    c.check(0x6ebd4)?;
    c.literal(C::Al, R::R1, 0x6f604)?;
    c.emit(mov_register(C::Al, R::R0, r(9)))?;
    c.branch(C::Al, 0x703fc, true)?;
    c.check(0x6ebe0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0x1)?)?;
    c.branch(C::Ne, 0x6ec04, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::LR, r(9), 0x7a0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 0x2)?)?;
    c.branch(C::Al, 0x6ece0, false)?;
    c.checked_data()?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn code_and_checked_data_extents() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(CODE_END - START, 844);
        assert_eq!(END - CODE_END, 4);
        assert_eq!(word(&bytes, CODE_END), DIVIDE_BY_FIVE_RECIPROCAL);
        let mut c = Code { at: CODE_END, bytes: Vec::new() };
        assert!(c.emit(mov_register(C::Al, R::R0, R::R0)).is_err());
        assert!(c.checked_data().is_err());
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.checked_data().is_err());
        assert!(c.branch(C::Al, CODE_END, false).is_err());
    }

    #[test]
    fn branches_exclude_pool_and_preserve_calls() {
        let bytes = fragment().unwrap();
        let mut calls = Vec::new();
        for at in (START..CODE_END).step_by(4) {
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 != 0x0a00_0000 {
                continue;
            }
            let offset = ((instruction << 8) as i32 >> 8) * 4;
            let target = (at as i32 + 8 + offset) as usize;
            assert!(!(CODE_END..END).contains(&target));
            if instruction & 0x0100_0000 != 0 {
                calls.push(target);
            }
        }
        assert_eq!(calls, [0x70128, 0x69c3c, 0x17b0, 0x69c3c, 0x69cd8, 0x70260, 0x703fc]);
        assert_eq!(word(&bytes, CODE_END - 4),
            branch(C::Al, (CODE_END - 4) as u32, 0x6ece0, false).unwrap());
    }

    #[test]
    fn optional_pinned_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }
}
