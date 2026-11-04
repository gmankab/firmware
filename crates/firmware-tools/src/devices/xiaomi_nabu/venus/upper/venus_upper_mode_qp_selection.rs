use arm32::{
    branch, data_immediate, data_register, data_register_shifted_by_register, mov_register,
    signed_multiply_accumulate_bottom_halves, store_halfword, transfer_byte,
    transfer_byte_register, transfer_halfword_register, transfer_word,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6e348;
pub const END: usize = 0x6e8b4;

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
            return Err(format!("mode QP selection address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("mode QP selection output address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.check(0x6e348)?;
    c.emit(transfer_word(C::Al, true, r(10), r(4), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x2d8)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(10), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e3c8, false)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e418, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e458, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(9), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x510)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(9), r(0))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e49c, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(3), r(2), r(3), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(3), 0x400)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(2), r(3), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x3440)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x38)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e4a0, false)?)?;
    c.check(0x6e3c8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e480, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e480, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(2), r(0), r(1))?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(2), r(3), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xf20)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x3000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(1), 0x400)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e488, false)?)?;
    c.check(0x6e418)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e4c8, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xe8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(2), r(3), r(2), r(1))?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(3), r(7), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x9b0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x3000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(1), 0x400)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e4d0, false)?)?;
    c.check(0x6e458)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), -2448)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x56)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(mov_register(C::Al, r(4), r(12)))?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1600)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x17b0, true)?)?;
    c.emit(mov_register(C::Al, r(12), r(4)))?;
    c.emit(mov_register(C::Al, r(7), r(5)))?;
    c.emit(branch(C::Al, c.at as u32, 0x6e6e0, false)?)?;
    c.check(0x6e480)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x4e0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(9), 0xa80)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 0xff)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e4fc, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(9), 0x29)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e530, false)?)?;
    c.check(0x6e49c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(9), 0xa80)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(5), 0xff)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e564, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e630, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e634, false)?)?;
    c.check(0x6e4c8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(9), 0x3f8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(9), 0xa80)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(5), 0xff)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e600, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e66c, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e670, false)?)?;
    c.check(0x6e4fc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0xd0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e494, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x124)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(9), 0x4e0)?)?;
    c.emit(mov_register(C::Al, r(5), r(12)))?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x400)?)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.emit(branch(C::Al, c.at as u32, 0x71ed0, true)?)?;
    c.emit(mov_register(C::Al, r(12), r(5)))?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x6e530)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2c)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 0x530)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r(0), r(1)))?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x52d)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 0x52f)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(6), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, r(0), r(1)))?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x52b)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e6e0, false)?)?;
    c.check(0x6e564)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(2), 0xd0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e4ac, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(8), 0x4)?)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.emit(transfer_word(C::Al, false, r(12), r(11), -64)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x71ed0, true)?)?;
    c.emit(mov_register(C::Al, r(4), r(9)))?;
    c.emit(mov_register(C::Al, r(9), r(0)))?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x620)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6f108, false)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(branch(C::Al, c.at as u32, 0x728cc, true)?)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(4), 0x29c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6f118, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(store_halfword(C::Al, r(1), r(12), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6f120, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0xff00)?)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr, r(1), r(1), r(4), Shift::Lsl, r(5), false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(0), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(branch(C::Hi, c.at as u32, 0x6e5e4, false)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6f11c, false)?)?;
    c.check(0x6e600)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(2), 0xd0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e4dc, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x294)?)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.emit(mov_register(C::Al, r(5), r(12)))?;
    c.emit(branch(C::Al, c.at as u32, 0x71ed0, true)?)?;
    c.emit(mov_register(C::Al, r(12), r(5)))?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.emit(branch(C::Al, c.at as u32, 0x6e6b0, false)?)?;
    c.check(0x6e630)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x27)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(0), 0x0)?)?;
    c.check(0x6e638)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2c)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r(0), r(1)))?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0xd)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0xf)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(6), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, r(0), r(1)))?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0xb)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e6e0, false)?)?;
    c.check(0x6e66c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x28)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x621)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e6ac, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x294)?)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.emit(transfer_word(C::Al, false, r(10), r(11), -64)?)?;
    c.emit(mov_register(C::Al, r(10), r(12)))?;
    c.emit(branch(C::Al, c.at as u32, 0x71ed0, true)?)?;
    c.emit(mov_register(C::Al, r(12), r(10)))?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -64)?)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Ls, r(6), r(5)))?;
    c.emit(branch(C::Al, c.at as u32, 0x6e6b0, false)?)?;
    c.emit(mov_register(C::Al, r(6), r(5)))?;
    c.check(0x6e6b0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2c)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 0x2a0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r(0), r(1)))?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x29d)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 0x29f)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(6), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Lt, r(0), r(1)))?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x29b)?)?;
    c.check(0x6e6e0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e7c0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e7c4, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x7d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e7c0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e76c, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(branch(C::Ls, c.at as u32, 0x6e7c0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3b4)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(3), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(0), r(2), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2ec)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_byte_register(C::Al, true, r(1), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r(3), r(1)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe9)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1200)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(3), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x2d2)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(1), r(2), r(0))?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e7a0, false)?)?;
    c.check(0x6e76c)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2ec)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(3), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(2), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte_register(C::Al, true, r(1), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, r(2), r(1)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe9)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1200)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(3), 0x0)?)?;
    c.check(0x6e7a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xeb)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xe7)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1200)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1200)?)?;
    c.emit(transfer_byte_register(C::Al, true, r(1), r(0), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Ls, r(1), r(6)))?;
    c.emit(transfer_byte_register(C::Al, false, r(1), r(0), r(2), Shift::Lsl, 0)?)?;
    c.check(0x6e7c0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e89c, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x7d)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0xff)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e89c, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e888, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(1), 0xff)?)?;
    c.emit(branch(C::Al, c.at as u32, 0x6e81c, false)?)?;
    c.check(0x6e7f8)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(3), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(2), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(2), r(1), 0xff)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(3), 0x1)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d2)?)?;
    c.check(0x6e81c)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x34)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(branch(C::Hi, c.at as u32, 0x6e7f8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e888, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(6), 0xff)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0xff)?)?;
    c.emit(branch(C::Eq, c.at as u32, 0x6e888, false)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(0), 0xff)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(6), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0xff)?)?;
    c.check(0x6e858)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(7), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(3), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(2), 0x1)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(3), r(7), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(mov_register(C::Al, r(0), r(3)))?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e858, false)?)?;
    c.check(0x6e888)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x4c)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2d2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), r(0), 0x1)?)?;
    c.check(0x6e89c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x89)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e8b4, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.emit(branch(C::Ne, c.at as u32, 0x6e974, false)?)?;

    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], address: usize) -> u32 {
        let offset = address - START;
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(reference.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), identity.sha256);
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }

    #[test]
    fn exact_bounds_and_emission_guards() {
        assert_eq!(END - START, 1388);
        assert_eq!(fragment().unwrap().len(), END - START);
        for at in [START - 4, START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn selector_calls_and_cold_path_edges() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x6e358, C::Eq, 0x6e3c8, false),
            (0x6e364, C::Eq, 0x6e418, false),
            (0x6e36c, C::Ne, 0x6e458, false),
            (0x6e524, C::Al, 0x71ed0, true),
            (0x6e588, C::Al, 0x71ed0, true),
            (0x6e59c, C::Eq, 0x6f108, false),
            (0x6e5a4, C::Al, 0x728cc, true),
            (0x6e5b8, C::Eq, 0x6f118, false),
            (0x6e5cc, C::Eq, 0x6f120, false),
            (0x6e5fc, C::Al, 0x6f11c, false),
            (0x6e620, C::Al, 0x71ed0, true),
            (0x6e690, C::Al, 0x71ed0, true),
            (0x6e8a4, C::Ne, END, false),
            (0x6e8b0, C::Ne, 0x6e974, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x6e638), transfer_byte(C::Al, true, r(0), r(9), 0x2c).unwrap());
    }

    #[test]
    fn signed_record_stride_clamps_and_indexed_history() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6e34c), transfer_word(C::Al, true, r(0), r(9), 0x2d8).unwrap());
        for (at, dst, left, stride, base) in [
            (0x6e3a8, 3, 2, 3, 1), (0x6e3fc, 0, 2, 0, 1),
            (0x6e400, 1, 2, 3, 1), (0x6e43c, 2, 3, 2, 1),
            (0x6e440, 1, 3, 7, 1), (0x6e738, 0, 0, 2, 1),
            (0x6e764, 0, 1, 2, 0),
        ] {
            assert_eq!(word(&bytes, at), signed_multiply_accumulate_bottom_halves(C::Al, r(dst), r(left), r(stride), r(base)).unwrap());
        }
        for (at, base, upper, lower) in [
            (0x6e544, 4, 0x52d, 0x52b),
            (0x6e64c, 8, 0xd, 0xb),
            (0x6e6c4, 4, 0x29d, 0x29b),
        ] {
            assert_eq!(word(&bytes, at - 4), mov_register(C::Hi, r(0), r(1)));
            assert_eq!(word(&bytes, at), transfer_byte(C::Al, false, r(0), r(base), upper).unwrap());
            assert_eq!(word(&bytes, at + 20), mov_register(C::Lt, r(0), r(1)));
            assert_eq!(word(&bytes, at + 24), transfer_byte(C::Al, false, r(0), r(base), lower).unwrap());
        }
        for (at, condition, target) in [
            (0x6e824, C::Hi, 0x6e7f8), (0x6e884, C::Ne, 0x6e858),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x6e83c), transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0).unwrap());
        assert_eq!(word(&bytes, 0x6e898), transfer_byte(C::Al, false, r(6), r(0), 1).unwrap());
    }
}
