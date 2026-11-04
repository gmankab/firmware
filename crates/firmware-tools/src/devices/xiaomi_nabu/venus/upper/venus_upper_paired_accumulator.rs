use arm32::{
    branch, data_immediate, data_register, load_halfword, multiply_accumulate, multiply_long,
    rotate_right_with_extend, signed_multiply_accumulate_bottom_halves, transfer_byte,
    transfer_doubleword, transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6ccb4;
pub const END: usize = 0x6d0d8;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!(
                "paired accumulator address {:#x} != {expected:#x}",
                self.at
            ));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!(
                "paired accumulator instruction outside range: {:#x}",
                self.at
            ));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn r(n: u8) -> R {
    R::new(n).expect("valid paired accumulator register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(END - START),
    };
    c.check(0x6ccb4)?;
    c.emit(transfer_word(C::Al, false, R::R12, R::SP, 0x40)?)?;
    c.check(0x6ccb8)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 0x2d4)?)?;
    c.check(0x6ccbc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.check(0x6ccc0)?;
    c.branch(C::Ne, 0x6cdb4, false)?;
    c.check(0x6ccc4)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x3c)?)?;
    c.check(0x6ccc8)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 0x213)?)?;
    c.check(0x6cccc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ccd0)?;
    c.branch(C::Eq, 0x6ce70, false)?;
    c.check(0x6ccd4)?;
    c.emit(load_halfword(C::Al, r(3), r(10), 0x2c)?)?;
    c.check(0x6ccd8)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0x60)?)?;
    c.check(0x6ccdc)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R0,
        R::R0,
        r(9),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cce0)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.check(0x6cce4)?;
    c.branch(C::Al, 0x6f738, true)?;
    c.check(0x6cce8)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0x5c)?)?;
    c.check(0x6ccec)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(9), 0x2a9)?)?;
    c.check(0x6ccf0)?;
    c.emit(transfer_word(C::Al, true, r(4), r(9), 0x2d8)?)?;
    c.check(0x6ccf4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x3d)?)?;
    c.check(0x6ccf8)?;
    c.emit(data_immediate(C::Cc, Op::Sub, r(5), r(5), 0x1)?)?;
    c.check(0x6ccfc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.check(0x6cd00)?;
    c.branch(C::Ne, 0x6cd28, false)?;
    c.check(0x6cd04)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0xc)?)?;
    c.check(0x6cd08)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x27)?)?;
    c.check(0x6cd0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.check(0x6cd10)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.check(0x6cd14)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.check(0x6cd18)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    c.check(0x6cd1c)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.check(0x6cd20)?;
    c.emit(data_register(
        C::Al,
        Op::And,
        R::R0,
        R::R0,
        R::R1,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cd24)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(5),
        r(5),
        R::R0,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cd28)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        r(6),
        R::R0,
        r(9),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cd2c)?;
    c.emit(transfer_word_preindexed_writeback(
        C::Al,
        true,
        r(7),
        r(6),
        0x760,
    )?)?;
    c.check(0x6cd30)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(6), -0x718)?)?;
    c.check(0x6cd34)?;
    c.emit(transfer_word(C::Al, true, r(3), r(6), -0x45c)?)?;
    c.check(0x6cd38)?;
    c.emit(multiply_long(C::Al, false, R::R0, R::R1, r(3), R::R2)?)?;
    c.check(0x6cd3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x240)?)?;
    c.check(0x6cd40)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.check(0x6cd44)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf4000)?)?;
    c.check(0x6cd48)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.check(0x6cd4c)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(6), 0x4)?)?;
    c.check(0x6cd50)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        R::R0,
        r(7),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6cd54)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        R::R1,
        R::R2,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cd58)?;
    c.emit(transfer_doubleword(C::Al, false, R::R0, r(6), 0x0)?)?;
    c.check(0x6cd5c)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(6), -0x73c)?)?;
    c.check(0x6cd60)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x1)?)?;
    c.check(0x6cd64)?;
    c.branch(C::Ne, 0x6cd9c, false)?;
    c.check(0x6cd68)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0x60)?)?;
    c.check(0x6cd6c)?;
    c.emit(multiply_long(C::Al, false, r(3), r(7), R::R2, r(5))?)?;
    c.check(0x6cd70)?;
    c.emit(data_register(
        C::Al,
        Op::Sub,
        R::R2,
        R::R0,
        r(3),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6cd74)?;
    c.emit(data_register(
        C::Al,
        Op::Sbc,
        R::R2,
        R::R1,
        r(7),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6cd78)?;
    c.branch(C::Cs, 0x6cd9c, false)?;
    c.check(0x6cd7c)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(9), 0x778)?)?;
    c.check(0x6cd80)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x1)?)?;
    c.check(0x6cd84)?;
    c.branch(C::Eq, 0x6cd9c, false)?;
    c.check(0x6cd88)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.check(0x6cd8c)?;
    c.emit(transfer_byte(C::Al, false, R::R2, r(9), 0x778)?)?;
    c.check(0x6cd90)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0x77c)?)?;
    c.check(0x6cd94)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x1)?)?;
    c.check(0x6cd98)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(9), 0x77c)?)?;
    c.check(0x6cd9c)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(9), 0x758)?)?;
    c.check(0x6cda0)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x75c)?)?;
    c.check(0x6cda4)?;
    c.emit(data_register(
        C::Al,
        Op::Sub,
        R::R0,
        R::R2,
        R::R0,
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6cda8)?;
    c.emit(data_register(
        C::Al,
        Op::Sbc,
        R::R0,
        r(3),
        R::R1,
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6cdac)?;
    c.emit(transfer_doubleword(C::Cc, false, R::R2, r(6), 0x0)?)?;
    c.check(0x6cdb0)?;
    c.branch(C::Al, 0x6cdb8, false)?;
    c.check(0x6cdb4)?;
    c.emit(transfer_word(C::Al, true, r(4), r(9), 0x2d8)?)?;
    c.check(0x6cdb8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x310)?)?;
    c.check(0x6cdbc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x2)?)?;
    c.check(0x6cdc0)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 0x38)?)?;
    c.check(0x6cdc4)?;
    c.branch(C::Eq, 0x6ce00, false)?;
    c.check(0x6cdc8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x1)?)?;
    c.check(0x6cdcc)?;
    c.branch(C::Eq, 0x6ce2c, false)?;
    c.check(0x6cdd0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.check(0x6cdd4)?;
    c.branch(C::Ne, 0x6ce58, false)?;
    c.check(0x6cdd8)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0xc)?)?;
    c.check(0x6cddc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6cde0)?;
    c.branch(C::Eq, 0x6cf3c, false)?;
    c.check(0x6cde4)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x4c)?)?;
    c.check(0x6cde8)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d2)?)?;
    c.check(0x6cdec)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xe8)?)?;
    c.check(0x6cdf0)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.check(0x6cdf4)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        R::R0,
        R::R1,
        R::R2,
        R::R0,
    )?)?;
    c.check(0x6cdf8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), R::R0, 0x3440)?)?;
    c.check(0x6cdfc)?;
    c.branch(C::Al, 0x6cf3c, false)?;
    c.check(0x6ce00)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0xc)?)?;
    c.check(0x6ce04)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ce08)?;
    c.branch(C::Eq, 0x6cf30, false)?;
    c.check(0x6ce0c)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x4c)?)?;
    c.check(0x6ce10)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d2)?)?;
    c.check(0x6ce14)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xe8)?)?;
    c.check(0x6ce18)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ce1c)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        R::R0,
        R::R1,
        R::R2,
        R::R0,
    )?)?;
    c.check(0x6ce20)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf20)?)?;
    c.check(0x6ce24)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), R::R0, 0x3000)?)?;
    c.check(0x6ce28)?;
    c.branch(C::Al, 0x6cf3c, false)?;
    c.check(0x6ce2c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0xc)?)?;
    c.check(0x6ce30)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ce34)?;
    c.branch(C::Eq, 0x6cf38, false)?;
    c.check(0x6ce38)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x4c)?)?;
    c.check(0x6ce3c)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d2)?)?;
    c.check(0x6ce40)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xe8)?)?;
    c.check(0x6ce44)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ce48)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        R::R0,
        R::R1,
        R::R2,
        R::R0,
    )?)?;
    c.check(0x6ce4c)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x9b0)?)?;
    c.check(0x6ce50)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), R::R0, 0x3000)?)?;
    c.check(0x6ce54)?;
    c.branch(C::Al, 0x6cf3c, false)?;
    c.check(0x6ce58)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::PC, 0xc70)?)?;
    c.check(0x6ce5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xaf)?)?;
    c.check(0x6ce60)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ce64)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x2300)?)?;
    c.check(0x6ce68)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0x6ce6c)?;
    c.branch(C::Al, 0x6cf3c, false)?;
    c.check(0x6ce70)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        r(4),
        R::R0,
        r(9),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6ce74)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.check(0x6ce78)?;
    c.emit(transfer_word_preindexed_writeback(
        C::Al,
        true,
        r(6),
        r(4),
        0x2c0,
    )?)?;
    c.check(0x6ce7c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), -0x260)?)?;
    c.check(0x6ce80)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), -0x278)?)?;
    c.check(0x6ce84)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R11, -0x40)?)?;
    c.check(0x6ce88)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R0,
        R::R0,
        R::R2,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6ce8c)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0x44)?)?;
    c.check(0x6ce90)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R11, -0x48)?)?;
    c.check(0x6ce94)?;
    c.emit(multiply_long(C::Al, false, R::R0, R::R1, r(5), R::R2)?)?;
    c.check(0x6ce98)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x240)?)?;
    c.check(0x6ce9c)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf4000)?)?;
    c.check(0x6cea0)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.check(0x6cea4)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 0x4c)?)?;
    c.check(0x6cea8)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(6),
        R::R0,
        r(6),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6ceac)?;
    c.emit(transfer_word(C::Al, true, r(10), R::R2, 0x0)?)?;
    c.check(0x6ceb0)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r(4), -0x18)?)?;
    c.check(0x6ceb4)?;
    c.emit(transfer_word(C::Al, false, R::R2, R::R11, -0x44)?)?;
    c.check(0x6ceb8)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), 0x4)?)?;
    c.check(0x6cebc)?;
    c.emit(transfer_byte(C::Al, true, r(8), r(4), 0x12)?)?;
    c.check(0x6cec0)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        r(7),
        R::R1,
        R::R2,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cec4)?;
    c.emit(transfer_doubleword(C::Al, false, r(6), r(4), 0x0)?)?;
    c.check(0x6cec8)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), -0x2b4)?)?;
    c.check(0x6cecc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6ced0)?;
    c.branch(C::Eq, 0x6d400, false)?;
    c.check(0x6ced4)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(9), 0x7d)?)?;
    c.check(0x6ced8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.check(0x6cedc)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 0x48)?)?;
    c.check(0x6cee0)?;
    c.branch(C::Ne, 0x6d448, false)?;
    c.check(0x6cee4)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        r(10),
        r(8),
        Shift::Lsl,
        2,
        false,
    )?)?;
    c.check(0x6cee8)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x5c)?)?;
    c.check(0x6ceec)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x38)?)?;
    c.check(0x6cef0)?;
    c.emit(multiply_long(C::Al, false, R::R1, R::R2, R::R0, r(5))?)?;
    c.check(0x6cef4)?;
    c.emit(multiply_long(C::Al, false, R::R0, r(7), R::R1, r(3))?)?;
    c.check(0x6cef8)?;
    c.emit(multiply_accumulate(C::Al, R::R1, R::R2, r(3), r(7))?)?;
    c.check(0x6cefc)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x240)?)?;
    c.check(0x6cf00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.check(0x6cf04)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf4000)?)?;
    c.check(0x6cf08)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.check(0x6cf0c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R2,
        r(10),
        r(8),
        Shift::Lsl,
        3,
        false,
    )?)?;
    c.check(0x6cf10)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R0,
        R::R0,
        R::R0,
        Shift::Lsr,
        2,
        false,
    )?)?;
    c.check(0x6cf14)?;
    c.emit(transfer_doubleword(C::Al, true, r(6), R::R2, 0x98)?)?;
    c.check(0x6cf18)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        R::R0,
        R::R0,
        R::R1,
        Shift::Lsl,
        30,
        false,
    )?)?;
    c.check(0x6cf1c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(6),
        R::R0,
        r(6),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6cf20)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R0,
        R::R0,
        R::R1,
        Shift::Lsr,
        2,
        false,
    )?)?;
    c.check(0x6cf24)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        r(7),
        R::R0,
        r(7),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cf28)?;
    c.emit(transfer_doubleword(C::Al, false, r(6), R::R2, 0x98)?)?;
    c.check(0x6cf2c)?;
    c.branch(C::Al, 0x6d4fc, false)?;
    c.check(0x6cf30)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x4e0)?)?;
    c.check(0x6cf34)?;
    c.branch(C::Al, 0x6cf3c, false)?;
    c.check(0x6cf38)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x3f8)?)?;
    c.check(0x6cf3c)?;
    c.emit(transfer_word(C::Al, true, r(6), r(5), 0xbc)?)?;
    c.check(0x6cf40)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 0xc0)?)?;
    c.check(0x6cf44)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(6),
        r(7),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cf48)?;
    c.branch(C::Ne, 0x6cf78, false)?;
    c.check(0x6cf4c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 0xcc)?)?;
    c.check(0x6cf50)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xfd0)?)?;
    c.check(0x6cf54)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R2,
        R::R0,
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cf58)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        r(3),
        R::R0,
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cf5c)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 0x1c)?)?;
    c.check(0x6cf60)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 0xc8)?)?;
    c.check(0x6cf64)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 0x18)?)?;
    c.check(0x6cf68)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 0x4)?)?;
    c.check(0x6cf6c)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 0xd8)?)?;
    c.check(0x6cf70)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    c.check(0x6cf74)?;
    c.branch(C::Al, 0x6cfec, false)?;
    c.check(0x6cf78)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(5), 0xe0)?)?;
    c.check(0x6cf7c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(5), 0xc8)?)?;
    c.check(0x6cf80)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(5), 0xc4)?)?;
    c.check(0x6cf84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), R::R0, 0x0)?)?;
    c.check(0x6cf88)?;
    c.emit(transfer_word(C::Al, true, r(8), r(5), 0xdc)?)?;
    c.check(0x6cf8c)?;
    c.emit(transfer_word(C::Al, true, r(10), r(5), 0xd4)?)?;
    c.check(0x6cf90)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        r(3),
        R::R0,
        r(9),
        Shift::Lsr,
        1,
        true,
    )?)?;
    c.check(0x6cf94)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 0x1c)?)?;
    c.check(0x6cf98)?;
    c.emit(rotate_right_with_extend(C::Al, R::R0, R::R2, false)?)?;
    c.check(0x6cf9c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        R::R0,
        R::R1,
        Shift::Lsl,
        16,
        true,
    )?)?;
    c.check(0x6cfa0)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        r(3),
        R::R1,
        Shift::Lsr,
        16,
        false,
    )?)?;
    c.check(0x6cfa4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.check(0x6cfa8)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.check(0x6cfac)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        r(4),
        R::R0,
        R::R0,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cfb0)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R1,
        R::R0,
        r(9),
        Shift::Lsr,
        1,
        true,
    )?)?;
    c.check(0x6cfb4)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R2,
        R::R0,
        r(8),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cfb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.check(0x6cfbc)?;
    c.emit(transfer_word(C::Al, false, r(4), r(5), 0x18)?)?;
    c.check(0x6cfc0)?;
    c.emit(rotate_right_with_extend(C::Al, R::R0, r(8), false)?)?;
    c.check(0x6cfc4)?;
    c.emit(transfer_word(C::Al, true, r(9), R::R11, -0x3c)?)?;
    c.check(0x6cfc8)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        R::R0,
        r(10),
        Shift::Lsl,
        16,
        true,
    )?)?;
    c.check(0x6cfcc)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        R::R1,
        r(10),
        Shift::Lsr,
        16,
        false,
    )?)?;
    c.check(0x6cfd0)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.check(0x6cfd4)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, 0xfb0)?)?;
    c.check(0x6cfd8)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 0xd8)?)?;
    c.check(0x6cfdc)?;
    c.emit(transfer_word(C::Al, false, r(4), R::SP, 0x0)?)?;
    c.check(0x6cfe0)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    c.check(0x6cfe4)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R2,
        R::R0,
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cfe8)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        r(3),
        R::R0,
        r(7),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6cfec)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0x6cff0)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::PC, -0x468)?)?;
    c.check(0x6cff4)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    c.check(0x6cff8)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0x6cffc)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0xc)?)?;
    c.check(0x6d000)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6d004)?;
    c.branch(C::Eq, 0x6d024, false)?;
    c.check(0x6d008)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x4c)?)?;
    c.check(0x6d00c)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r(9), 0x2d2)?)?;
    c.check(0x6d010)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xd8)?)?;
    c.check(0x6d014)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.check(0x6d018)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        R::R0,
        R::R1,
        R::R2,
        R::R0,
    )?)?;
    c.check(0x6d01c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), R::R0, 0xf8)?)?;
    c.check(0x6d020)?;
    c.branch(C::Al, 0x6d02c, false)?;
    c.check(0x6d024)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(9), 0x278)?)?;
    c.check(0x6d028)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), R::R0, 0x400)?)?;
    c.check(0x6d02c)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(9), 0x2d8)?)?;
    c.check(0x6d030)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.check(0x6d034)?;
    c.branch(C::Eq, 0x6d064, false)?;
    c.check(0x6d038)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.check(0x6d03c)?;
    c.branch(C::Eq, 0x6d084, false)?;
    c.check(0x6d040)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.check(0x6d044)?;
    c.branch(C::Ne, 0x6d0a4, false)?;
    c.check(0x6d048)?;
    c.emit(transfer_multiple(
        C::Al,
        true,
        r(4),
        R::R0.bit() | R::R1.bit() | R::R2.bit() | r(3).bit(),
        BlockMode::IncrementAfter,
        false,
    )?)?;
    c.check(0x6d04c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        R::R2,
        R::R0,
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6d050)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        r(3),
        R::R1,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6d054)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R1,
        R::R0,
        R::R1,
        Shift::Lsr,
        1,
        true,
    )?)?;
    c.check(0x6d058)?;
    c.emit(rotate_right_with_extend(C::Al, R::R0, R::R0, false)?)?;
    c.check(0x6d05c)?;
    c.emit(transfer_doubleword(C::Al, false, R::R0, r(4), 0x8)?)?;
    c.check(0x6d060)?;
    c.branch(C::Al, 0x6d0d4, false)?;
    c.check(0x6d064)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(4), 0x20)?)?;
    c.check(0x6d068)?;
    c.emit(transfer_multiple(
        C::Al,
        true,
        r(3),
        R::R0.bit() | R::R1.bit() | R::R2.bit() | r(3).bit(),
        BlockMode::IncrementAfter,
        false,
    )?)?;
    c.check(0x6d06c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        R::R2,
        R::R0,
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6d070)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        r(3),
        R::R1,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6d074)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R1,
        R::R0,
        R::R1,
        Shift::Lsr,
        1,
        true,
    )?)?;
    c.check(0x6d078)?;
    c.emit(rotate_right_with_extend(C::Al, R::R0, R::R0, false)?)?;
    c.check(0x6d07c)?;
    c.emit(transfer_doubleword(C::Al, false, R::R0, r(4), 0x28)?)?;
    c.check(0x6d080)?;
    c.branch(C::Al, 0x6d0d4, false)?;
    c.check(0x6d084)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(4), 0x10)?)?;
    c.check(0x6d088)?;
    c.emit(transfer_multiple(
        C::Al,
        true,
        r(3),
        R::R0.bit() | R::R1.bit() | R::R2.bit() | r(3).bit(),
        BlockMode::IncrementAfter,
        false,
    )?)?;
    c.check(0x6d08c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        R::R2,
        R::R0,
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6d090)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        r(3),
        R::R1,
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6d094)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R1,
        R::R0,
        R::R1,
        Shift::Lsr,
        1,
        true,
    )?)?;
    c.check(0x6d098)?;
    c.emit(rotate_right_with_extend(C::Al, R::R0, R::R0, false)?)?;
    c.check(0x6d09c)?;
    c.emit(transfer_doubleword(C::Al, false, R::R0, r(4), 0x18)?)?;
    c.check(0x6d0a0)?;
    c.branch(C::Al, 0x6d0d4, false)?;
    c.check(0x6d0a4)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::PC, 0xa24)?)?;
    c.check(0x6d0a8)?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 0x0)?)?;
    c.check(0x6d0ac)?;
    c.emit(transfer_multiple(
        C::Al,
        true,
        r(4),
        r(6).bit() | r(7).bit(),
        BlockMode::IncrementBefore,
        false,
    )?)?;
    c.check(0x6d0b0)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0xc)?)?;
    c.check(0x6d0b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x9a)?)?;
    c.check(0x6d0b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.check(0x6d0bc)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1100)?)?;
    c.check(0x6d0c0)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0x6d0c4)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        R::R0,
        r(7),
        r(8),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.check(0x6d0c8)?;
    c.emit(data_register(
        C::Al,
        Op::Adc,
        R::R1,
        r(5),
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.check(0x6d0cc)?;
    c.emit(data_register(
        C::Al,
        Op::Mov,
        R::R1,
        R::R0,
        R::R1,
        Shift::Lsr,
        1,
        true,
    )?)?;
    c.check(0x6d0d0)?;
    c.emit(rotate_right_with_extend(C::Al, R::R0, R::R0, false)?)?;
    c.check(0x6d0d4)?;
    c.emit(transfer_doubleword(C::Al, false, R::R0, r(4), 0x30)?)?;
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
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE"))
        else {
            return;
        };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(
            crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085"
        );
        let bytes = fragment().unwrap();
        let expected = &reference[START + 0x4000..END + 0x4000];
        assert_eq!(bytes.len(), expected.len());
        for (index, (actual, expected)) in bytes
            .chunks_exact(4)
            .zip(expected.chunks_exact(4))
            .enumerate()
        {
            assert_eq!(actual, expected, "instruction at {:#x}", START + index * 4);
        }
    }

    #[test]
    fn exact_extent_and_emission_bounds() {
        assert_eq!(END - START, 1060);
        assert_eq!(fragment().unwrap().len(), 1060);
        for at in [START - 4, START + 1, END] {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(c.emit(arm32::mov_register(C::Al, R::R0, R::R0)).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn absolute_calls_and_exits() {
        let bytes = fragment().unwrap();
        for (at, target) in [
            (0x6cfec, 0x2ec94),
            (0x6cff8, 0x2ec94),
            (0x6cd48, 0x8187c),
            (0x6ce68, 0x17b0),
            (0x6d0c0, 0x17b0),
        ] {
            assert_eq!(
                word(&bytes, at),
                branch(C::Al, at as u32, target, true).unwrap()
            );
        }
        for (at, condition, target) in [
            (0x6ced0, C::Eq, 0x6d400),
            (0x6cee0, C::Ne, 0x6d448),
            (0x6cf2c, C::Al, 0x6d4fc),
        ] {
            assert_eq!(
                word(&bytes, at),
                branch(condition, at as u32, target, false).unwrap()
            );
        }
    }

    #[test]
    fn paired_average_carry_and_stores() {
        let bytes = fragment().unwrap();
        for (at, offset) in [(0x6d048, 8), (0x6d068, 0x28), (0x6d088, 0x18)] {
            assert_eq!(
                word(&bytes, at + 4),
                data_register(C::Al, Op::Add, R::R0, R::R2, R::R0, Shift::Lsl, 0, true).unwrap()
            );
            assert_eq!(
                word(&bytes, at + 8),
                data_register(C::Al, Op::Adc, R::R1, r(3), R::R1, Shift::Lsl, 0, false).unwrap()
            );
            assert_eq!(
                word(&bytes, at + 12),
                data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsr, 1, true).unwrap()
            );
            assert_eq!(
                word(&bytes, at + 16),
                rotate_right_with_extend(C::Al, R::R0, R::R0, false).unwrap()
            );
            assert_eq!(
                word(&bytes, at + 20),
                transfer_doubleword(C::Al, false, R::R0, r(4), offset).unwrap()
            );
        }
    }
}
