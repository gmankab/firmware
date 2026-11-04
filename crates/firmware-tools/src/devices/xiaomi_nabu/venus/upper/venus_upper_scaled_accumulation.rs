use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch, data_immediate,
    data_immediate_with_flags, data_register, multiply_accumulate, multiply_long,
    signed_multiply_accumulate_bottom_halves, transfer_byte, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
};

pub const START: usize = 0x6d0d8;
pub const END: usize = 0x6d548;
pub const CODE_BYTES: usize = 1128;
pub const DATA_BYTES: usize = 8;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if at != self.at
            || !(START..END).contains(&at)
            || at % 4 != 0
            || matches!(at, 0x6d26c | 0x6d444)
        {
            return Err(format!("scaled accumulation instruction address {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, word: u32) -> Result<(), String> {
        if at != self.at || !matches!((at, word), (0x6d26c, 0x000f2d78) | (0x6d444, 0x51eb851f)) {
            return Err(format!(
                "scaled accumulation protected data address {at:#x}"
            ));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("valid scaled accumulation register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(END - START),
    };
    c.emit(0x6d0d8, transfer_word(C::Al, true, R::R0, r(9), 0xc)?)?;
    c.emit(0x6d0dc, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6d0e0, transfer_word(C::Al, false, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6d0e4, branch(C::Eq, 0x6d0e4, 0x6d13c, false)?)?;
    c.emit(0x6d0e8, transfer_word(C::Al, true, R::R0, R::SP, 0x4c)?)?;
    c.emit(0x6d0ec, transfer_byte(C::Al, true, R::R1, r(9), 0x7d)?)?;
    c.emit(0x6d0f0, transfer_byte(C::Al, true, r(5), r(9), 0x2d2)?)?;
    c.emit(0x6d0f4, transfer_word(C::Al, true, R::R12, R::R0, 0x0)?)?;
    c.emit(0x6d0f8, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(0x6d0fc, branch(C::Eq, 0x6d0fc, 0x6d174, false)?)?;
    c.emit(
        0x6d100,
        data_register(C::Al, Op::Add, R::R0, R::R12, r(5), Shift::Lsl, 3, false)?,
    )?;
    c.emit(0x6d104, transfer_word(C::Al, true, r(8), R::R0, 0x98)?)?;
    c.emit(0x6d108, transfer_word(C::Al, true, R::R1, R::R0, 0x9c)?)?;
    c.emit(
        0x6d10c,
        data_immediate_with_flags(C::Al, Op::Rsb, R::R0, r(8), 0x0, true)?,
    )?;
    c.emit(
        0x6d110,
        data_immediate_with_flags(C::Al, Op::Rsc, R::R0, R::R1, 0x0, true)?,
    )?;
    c.emit(0x6d114, branch(C::Ge, 0x6d114, 0x6d250, false)?)?;
    c.emit(0x6d118, transfer_word(C::Al, true, R::R0, r(9), 0x758)?)?;
    c.emit(0x6d11c, transfer_word(C::Al, true, r(3), r(9), 0x48)?)?;
    c.emit(0x6d120, transfer_word(C::Al, true, R::R2, r(9), 0x75c)?)?;
    c.emit(
        0x6d124,
        data_register(C::Al, Op::Sub, R::R0, R::R0, r(3), Shift::Lsr, 3, true)?,
    )?;
    c.emit(
        0x6d128,
        data_immediate_with_flags(C::Al, Op::Sbc, R::R0, R::R2, 0x0, true)?,
    )?;
    c.emit(0x6d12c, branch(C::Cs, 0x6d12c, 0x6d270, false)?)?;
    c.emit(
        0x6d130,
        data_register(C::Al, Op::Add, R::R0, R::R12, r(5), Shift::Lsl, 2, false)?,
    )?;
    c.emit(0x6d134, transfer_word(C::Al, true, R::R0, R::R0, 0x38)?)?;
    c.emit(0x6d138, branch(C::Al, 0x6d138, 0x6d548, false)?)?;
    c.emit(
        0x6d13c,
        data_register(C::Al, Op::Mov, r(10), R::R0, r(9), Shift::Lsl, 0, false)?,
    )?;
    c.emit(
        0x6d140,
        transfer_word_preindexed_writeback(C::Al, true, R::R2, r(10), 0x2c0)?,
    )?;
    c.emit(0x6d144, transfer_word(C::Al, true, R::R1, r(10), 0x4)?)?;
    c.emit(
        0x6d148,
        data_immediate_with_flags(C::Al, Op::Rsb, R::R0, R::R2, 0x0, true)?,
    )?;
    c.emit(
        0x6d14c,
        data_immediate_with_flags(C::Al, Op::Rsc, R::R0, R::R1, 0x0, true)?,
    )?;
    c.emit(0x6d150, branch(C::Ge, 0x6d150, 0x6d260, false)?)?;
    c.emit(0x6d154, transfer_word(C::Al, true, R::R0, r(9), 0x758)?)?;
    c.emit(0x6d158, transfer_word(C::Al, true, r(7), r(9), 0x48)?)?;
    c.emit(0x6d15c, transfer_word(C::Al, true, r(3), r(9), 0x75c)?)?;
    c.emit(
        0x6d160,
        data_register(C::Al, Op::Sub, R::R0, R::R0, r(7), Shift::Lsr, 3, true)?,
    )?;
    c.emit(
        0x6d164,
        data_immediate_with_flags(C::Al, Op::Sbc, R::R0, r(3), 0x0, true)?,
    )?;
    c.emit(0x6d168, branch(C::Cs, 0x6d168, 0x6d2e8, false)?)?;
    c.emit(0x6d16c, transfer_word(C::Al, true, r(7), r(9), 0x60)?)?;
    c.emit(0x6d170, branch(C::Al, 0x6d170, 0x6d6a4, false)?)?;
    c.emit(0x6d174, transfer_word(C::Al, false, r(5), R::SP, 0x34)?)?;
    c.emit(0x6d178, transfer_word(C::Al, true, R::R0, r(9), 0x34)?)?;
    c.emit(0x6d17c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6d180, transfer_word(C::Al, false, R::R0, R::SP, 0x48)?)?;
    c.emit(0x6d184, branch(C::Eq, 0x6d184, 0x6d370, false)?)?;
    c.emit(0x6d188, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6d18c, data_immediate(C::Al, Op::Mov, r(6), R::R0, 0x0)?)?;
    c.emit(0x6d190, data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x0)?)?;
    c.emit(0x6d194, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6d198, transfer_word(C::Al, false, R::R12, R::R11, -0x48)?)?;
    c.emit(0x6d19c, transfer_word(C::Al, false, R::R0, R::R11, -0x40)?)?;
    c.emit(0x6d1a0, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xd8)?)?;
    c.emit(
        0x6d1a4,
        multiply_accumulate(C::Al, r(7), r(6), R::R0, R::R12)?,
    )?;
    c.emit(
        0x6d1a8,
        data_register(C::Al, Op::Add, r(6), R::R12, r(6), Shift::Lsl, 2, false)?,
    )?;
    c.emit(0x6d1ac, transfer_word(C::Al, true, r(4), r(7), 0x100)?)?;
    c.emit(0x6d1b0, transfer_word(C::Al, true, R::R12, r(6), 0x50)?)?;
    c.emit(0x6d1b4, transfer_word(C::Al, true, R::R1, r(7), 0x104)?)?;
    c.emit(0x6d1b8, transfer_word(C::Al, true, R::R2, r(7), 0x110)?)?;
    c.emit(0x6d1bc, transfer_word(C::Al, true, r(9), r(6), 0x68)?)?;
    c.emit(0x6d1c0, transfer_word(C::Al, true, r(10), r(7), 0x114)?)?;
    c.emit(
        0x6d1c4,
        multiply_long(C::Al, false, R::LR, r(8), r(4), R::R12)?,
    )?;
    c.emit(
        0x6d1c8,
        multiply_accumulate(C::Al, r(4), R::R1, R::R12, r(8))?,
    )?;
    c.emit(
        0x6d1cc,
        data_register(C::Al, Op::Mov, R::R0, R::R0, R::LR, Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d1d0,
        data_register(C::Al, Op::Mov, R::R1, R::R0, r(4), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d1d4,
        data_register(C::Al, Op::Orr, R::R0, R::R0, r(4), Shift::Lsl, 26, false)?,
    )?;
    c.emit(0x6d1d8, transfer_word(C::Al, true, r(4), R::R11, -0x40)?)?;
    c.emit(
        0x6d1dc,
        data_register(C::Al, Op::Add, R::R0, R::R0, r(4), Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d1e0,
        data_register(C::Al, Op::Adc, R::R1, R::R1, r(3), Shift::Lsl, 0, false)?,
    )?;
    c.emit(
        0x6d1e4,
        multiply_long(C::Al, false, r(3), r(4), R::R2, r(9))?,
    )?;
    c.emit(
        0x6d1e8,
        multiply_accumulate(C::Al, R::R2, r(10), r(9), r(4))?,
    )?;
    c.emit(
        0x6d1ec,
        data_register(C::Al, Op::Mov, r(3), R::R0, r(3), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d1f0,
        data_register(C::Al, Op::Orr, r(3), r(3), R::R2, Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d1f4,
        data_register(C::Al, Op::Add, r(4), R::R0, r(3), Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d1f8, transfer_word(C::Al, true, R::R0, R::R11, -0x44)?)?;
    c.emit(
        0x6d1fc,
        data_register(C::Al, Op::Adc, r(3), R::R1, R::R2, Shift::Lsr, 6, false)?,
    )?;
    c.emit(0x6d200, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.emit(0x6d204, branch(C::Ne, 0x6d204, 0x6d230, false)?)?;
    c.emit(0x6d208, transfer_word(C::Al, true, R::R0, r(7), 0x120)?)?;
    c.emit(0x6d20c, transfer_word(C::Al, true, R::R2, r(6), 0x80)?)?;
    c.emit(0x6d210, transfer_word(C::Al, true, R::R1, r(7), 0x124)?)?;
    c.emit(
        0x6d214,
        multiply_long(C::Al, false, r(7), r(6), R::R0, R::R2)?,
    )?;
    c.emit(
        0x6d218,
        multiply_accumulate(C::Al, R::R0, R::R1, R::R2, r(6))?,
    )?;
    c.emit(
        0x6d21c,
        data_register(C::Al, Op::Mov, R::R2, R::R0, r(7), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d220,
        data_register(C::Al, Op::Mov, R::R1, R::R0, R::R0, Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d224,
        data_register(C::Al, Op::Orr, R::R0, R::R2, R::R0, Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d228,
        data_register(C::Al, Op::Add, r(4), R::R0, r(4), Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d22c,
        data_register(C::Al, Op::Adc, r(3), R::R1, r(3), Shift::Lsl, 0, false)?,
    )?;
    c.emit(0x6d230, transfer_word(C::Al, false, r(4), R::R11, -0x40)?)?;
    c.emit(0x6d234, transfer_word(C::Al, true, R::R0, R::SP, 0x48)?)?;
    c.emit(0x6d238, transfer_word(C::Al, true, R::R12, R::R11, -0x48)?)?;
    c.emit(0x6d23c, data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(0x6d240, data_immediate(C::Al, Op::And, r(6), r(5), 0xff)?)?;
    c.emit(
        0x6d244,
        data_register(C::Al, Op::Cmp, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?,
    )?;
    c.emit(0x6d248, branch(C::Hi, 0x6d248, 0x6d1a0, false)?)?;
    c.emit(0x6d24c, branch(C::Al, 0x6d24c, 0x6d37c, false)?)?;
    c.emit(
        0x6d250,
        data_register(C::Al, Op::Add, R::R0, R::R12, r(5), Shift::Lsl, 2, false)?,
    )?;
    c.emit(0x6d254, transfer_word(C::Al, true, R::R0, R::R0, 0x38)?)?;
    c.emit(
        0x6d258,
        data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 3, false)?,
    )?;
    c.emit(0x6d25c, branch(C::Al, 0x6d25c, 0x6d548, false)?)?;
    c.emit(0x6d260, transfer_word(C::Al, true, R::R0, r(9), 0x60)?)?;
    c.emit(
        0x6d264,
        data_register(C::Al, Op::Mov, r(7), R::R0, R::R0, Shift::Lsr, 3, false)?,
    )?;
    c.emit(0x6d268, branch(C::Al, 0x6d268, 0x6d6a4, false)?)?;
    c.data(0x6d26c, 0x000f2d78)?;
    c.emit(0x6d270, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xd8)?)?;
    c.emit(
        0x6d274,
        data_register(C::Al, Op::Mov, R::R2, R::R0, R::R12, Shift::Lsl, 0, false)?,
    )?;
    c.emit(
        0x6d278,
        signed_multiply_accumulate_bottom_halves(C::Al, R::R0, r(5), R::R0, R::R12)?,
    )?;
    c.emit(
        0x6d27c,
        data_register(C::Al, Op::Add, r(7), R::R2, r(5), Shift::Lsl, 2, false)?,
    )?;
    c.emit(0x6d280, transfer_word(C::Al, true, R::R12, R::R0, 0x100)?)?;
    c.emit(0x6d284, transfer_word(C::Al, true, R::LR, R::R0, 0x104)?)?;
    c.emit(0x6d288, transfer_word(C::Al, true, r(6), R::R0, 0x110)?)?;
    c.emit(0x6d28c, transfer_word(C::Al, true, r(10), R::R0, 0x114)?)?;
    c.emit(0x6d290, transfer_word(C::Al, false, R::R2, R::R11, -0x48)?)?;
    c.emit(0x6d294, transfer_word(C::Al, false, r(5), R::SP, 0x34)?)?;
    c.emit(0x6d298, transfer_word(C::Al, true, R::R2, r(7), 0x68)?)?;
    c.emit(0x6d29c, transfer_word(C::Al, true, r(4), r(7), 0x50)?)?;
    c.emit(
        0x6d2a0,
        multiply_long(C::Al, false, r(9), r(3), r(6), R::R2)?,
    )?;
    c.emit(
        0x6d2a4,
        multiply_accumulate(C::Al, r(6), r(10), R::R2, r(3))?,
    )?;
    c.emit(
        0x6d2a8,
        multiply_long(C::Al, false, R::R2, r(5), R::R12, r(4))?,
    )?;
    c.emit(
        0x6d2ac,
        multiply_accumulate(C::Al, r(3), R::LR, r(4), r(5))?,
    )?;
    c.emit(
        0x6d2b0,
        data_register(C::Al, Op::Mov, R::R2, R::R0, R::R2, Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d2b4,
        data_register(C::Al, Op::Mov, r(5), R::R0, r(9), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d2b8,
        data_register(C::Al, Op::Mov, r(10), R::R0, r(6), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d2bc,
        data_register(C::Al, Op::Orr, R::R2, R::R2, r(3), Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d2c0,
        data_register(C::Al, Op::Orr, r(6), r(5), r(6), Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d2c4,
        data_register(C::Al, Op::Add, R::R2, r(6), R::R2, Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d2c8,
        data_register(C::Al, Op::Adc, r(3), r(10), r(3), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d2cc,
        data_register(C::Al, Op::Orr, r(6), R::R2, r(3), Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d2d0, transfer_word(C::Ne, true, r(6), R::R0, 0x128)?)?;
    c.emit(0x6d2d4, transfer_word(C::Ne, true, r(5), R::R0, 0x12c)?)?;
    c.emit(
        0x6d2d8,
        data_register(C::Ne, Op::Orr, R::R0, r(6), r(5), Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d2dc, branch(C::Ne, 0x6d2dc, 0x6d52c, false)?)?;
    c.emit(0x6d2e0, transfer_word(C::Al, true, R::R0, r(7), 0x38)?)?;
    c.emit(0x6d2e4, branch(C::Al, 0x6d2e4, 0x6d53c, false)?)?;
    c.emit(
        0x6d2e8,
        data_immediate(C::Al, Op::Add, R::R12, r(9), 0x2b4)?,
    )?;
    c.emit(
        0x6d2ec,
        transfer_multiple(
            C::Al,
            true,
            R::R12,
            R::R0.bit() | r(3).bit() | R::R12.bit(),
            BlockMode::IncrementAfter,
            false,
        )?,
    )?;
    c.emit(0x6d2f0, transfer_word(C::Al, true, r(6), r(9), 0x680)?)?;
    c.emit(0x6d2f4, transfer_word(C::Al, true, r(5), r(9), 0x684)?)?;
    c.emit(
        0x6d2f8,
        multiply_long(C::Al, false, r(4), r(7), r(6), R::R0)?,
    )?;
    c.emit(
        0x6d2fc,
        multiply_accumulate(C::Al, r(6), r(5), R::R0, r(7))?,
    )?;
    c.emit(0x6d300, transfer_word(C::Al, true, r(7), r(9), 0x690)?)?;
    c.emit(0x6d304, transfer_word(C::Al, true, r(5), r(9), 0x694)?)?;
    c.emit(
        0x6d308,
        data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d30c,
        data_register(C::Al, Op::Orr, R::LR, R::R0, r(6), Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d310,
        multiply_long(C::Al, false, r(4), R::R0, r(7), r(3))?,
    )?;
    c.emit(
        0x6d314,
        multiply_accumulate(C::Al, r(7), r(5), r(3), R::R0)?,
    )?;
    c.emit(
        0x6d318,
        data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d31c,
        data_register(C::Al, Op::Orr, R::R0, R::R0, r(7), Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d320,
        data_register(C::Al, Op::Mov, r(3), R::R0, r(7), Shift::Lsr, 6, false)?,
    )?;
    c.emit(0x6d324, transfer_word(C::Al, true, r(7), r(9), 0x6a0)?)?;
    c.emit(
        0x6d328,
        data_register(C::Al, Op::Add, R::R0, R::R0, R::LR, Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d32c,
        data_register(C::Al, Op::Adc, r(3), r(3), r(6), Shift::Lsr, 6, false)?,
    )?;
    c.emit(0x6d330, transfer_word(C::Al, true, r(6), r(9), 0x6a4)?)?;
    c.emit(
        0x6d334,
        multiply_long(C::Al, false, r(5), r(4), r(7), R::R12)?,
    )?;
    c.emit(
        0x6d338,
        multiply_accumulate(C::Al, r(7), r(6), R::R12, r(4))?,
    )?;
    c.emit(
        0x6d33c,
        data_register(C::Al, Op::Mov, r(6), R::R0, r(5), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d340,
        data_register(C::Al, Op::Mov, r(4), R::R0, r(9), Shift::Lsl, 0, false)?,
    )?;
    c.emit(
        0x6d344,
        data_register(C::Al, Op::Orr, r(6), r(6), r(7), Shift::Lsl, 26, false)?,
    )?;
    c.emit(
        0x6d348,
        data_register(C::Al, Op::Add, r(9), R::R0, r(6), Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d34c,
        data_register(C::Al, Op::Adc, r(8), r(3), r(7), Shift::Lsr, 6, false)?,
    )?;
    c.emit(
        0x6d350,
        data_register(C::Al, Op::Orr, R::R0, r(9), r(8), Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d354,
        data_register(C::Ne, Op::Mov, r(6), R::R0, r(4), Shift::Lsl, 0, false)?,
    )?;
    c.emit(
        0x6d358,
        transfer_word_preindexed_writeback(C::Ne, true, r(3), r(6), 0x6a8)?,
    )?;
    c.emit(0x6d35c, transfer_word(C::Ne, true, r(7), r(6), 0x4)?)?;
    c.emit(
        0x6d360,
        data_register(C::Ne, Op::Orr, R::R0, r(3), r(7), Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d364, branch(C::Ne, 0x6d364, 0x6d66c, false)?)?;
    c.emit(0x6d368, transfer_word(C::Al, true, r(7), r(4), 0x60)?)?;
    c.emit(0x6d36c, branch(C::Al, 0x6d36c, 0x6d6a0, false)?)?;
    c.emit(0x6d370, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6d374, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6d378, transfer_word(C::Al, false, R::R0, R::R11, -0x40)?)?;
    c.emit(0x6d37c, transfer_word(C::Al, true, r(9), R::R11, -0x3c)?)?;
    c.emit(0x6d380, transfer_word(C::Al, true, r(7), r(9), 0x2c0)?)?;
    c.emit(0x6d384, transfer_word(C::Al, true, R::R1, r(9), 0x2c4)?)?;
    c.emit(
        0x6d388,
        data_immediate_with_flags(C::Al, Op::Rsb, R::R0, r(7), 0x0, true)?,
    )?;
    c.emit(
        0x6d38c,
        data_immediate_with_flags(C::Al, Op::Rsc, R::R0, R::R1, 0x0, true)?,
    )?;
    c.emit(0x6d390, branch(C::Ge, 0x6d390, 0x6d3f4, false)?)?;
    c.emit(0x6d394, transfer_word(C::Al, true, R::R0, r(9), 0x758)?)?;
    c.emit(0x6d398, transfer_word(C::Al, true, r(6), r(9), 0x48)?)?;
    c.emit(0x6d39c, transfer_word(C::Al, true, R::R2, r(9), 0x75c)?)?;
    c.emit(0x6d3a0, transfer_word(C::Al, true, r(5), R::SP, 0x34)?)?;
    c.emit(
        0x6d3a4,
        data_register(C::Al, Op::Sub, R::R0, R::R0, r(6), Shift::Lsr, 3, true)?,
    )?;
    c.emit(
        0x6d3a8,
        data_immediate_with_flags(C::Al, Op::Sbc, R::R0, R::R2, 0x0, true)?,
    )?;
    c.emit(0x6d3ac, branch(C::Cc, 0x6d3ac, 0x6d3ec, false)?)?;
    c.emit(0x6d3b0, transfer_word(C::Al, true, R::R2, R::R11, -0x40)?)?;
    c.emit(
        0x6d3b4,
        data_register(C::Al, Op::Orr, R::R0, R::R2, r(3), Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d3b8, data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0xd8)?)?;
    c.emit(
        0x6d3bc,
        signed_multiply_accumulate_bottom_halves(C::Ne, R::R0, r(5), R::R0, R::R12)?,
    )?;
    c.emit(0x6d3c0, transfer_word(C::Ne, true, r(8), R::R0, 0x128)?)?;
    c.emit(0x6d3c4, transfer_word(C::Ne, true, R::LR, R::R0, 0x12c)?)?;
    c.emit(
        0x6d3c8,
        data_register(C::Ne, Op::Orr, R::R0, r(8), R::LR, Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d3cc, branch(C::Eq, 0x6d3cc, 0x6d3ec, false)?)?;
    c.emit(
        0x6d3d0,
        multiply_long(C::Al, false, R::R0, r(6), r(8), r(7))?,
    )?;
    c.emit(
        0x6d3d4,
        multiply_accumulate(C::Al, r(4), r(8), R::R1, r(6))?,
    )?;
    c.emit(
        0x6d3d8,
        multiply_accumulate(C::Al, R::R1, R::LR, r(7), r(4))?,
    )?;
    c.emit(
        0x6d3dc,
        data_register(C::Al, Op::Mov, r(4), R::R0, R::R12, Shift::Lsl, 0, false)?,
    )?;
    c.emit(0x6d3e0, branch(C::Al, 0x6d3e0, 0x8187c, true)?)?;
    c.emit(
        0x6d3e4,
        data_register(C::Al, Op::Mov, R::R12, R::R0, r(4), Shift::Lsl, 0, false)?,
    )?;
    c.emit(0x6d3e8, branch(C::Al, 0x6d3e8, 0x6d548, false)?)?;
    c.emit(0x6d3ec, transfer_word(C::Al, true, R::R0, r(9), 0x60)?)?;
    c.emit(0x6d3f0, branch(C::Al, 0x6d3f0, 0x6d548, false)?)?;
    c.emit(0x6d3f4, transfer_word(C::Al, true, R::R0, r(9), 0x60)?)?;
    c.emit(
        0x6d3f8,
        data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 3, false)?,
    )?;
    c.emit(0x6d3fc, branch(C::Al, 0x6d3fc, 0x6d544, false)?)?;
    c.emit(0x6d400, transfer_word(C::Al, true, R::R0, r(9), 0x2d8)?)?;
    c.emit(0x6d404, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6d408, branch(C::Ne, 0x6d408, 0x6d420, false)?)?;
    c.emit(0x6d40c, transfer_word(C::Al, true, R::R0, r(9), 0x5c)?)?;
    c.emit(0x6d410, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x27)?)?;
    c.emit(0x6d414, transfer_word(C::Al, true, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6d418, data_immediate(C::Hi, Op::Add, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6d41c, transfer_word(C::Al, false, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6d420, transfer_byte(C::Al, true, R::R0, r(9), 0x24)?)?;
    c.emit(0x6d424, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6d428, branch(C::Ne, 0x6d428, 0x6da84, false)?)?;
    c.emit(0x6d42c, transfer_byte(C::Al, true, R::R0, r(9), 0x778)?)?;
    c.emit(0x6d430, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6d434, branch(C::Ne, 0x6d434, 0x6d930, false)?)?;
    c.emit(0x6d438, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6d43c, transfer_byte(C::Al, false, R::R0, r(9), 0x778)?)?;
    c.emit(0x6d440, branch(C::Al, 0x6d440, 0x6da84, false)?)?;
    c.data(0x6d444, 0x51eb851f)?;
    c.emit(0x6d448, transfer_word(C::Al, true, R::R0, R::R11, -0x40)?)?;
    c.emit(0x6d44c, transfer_word(C::Al, true, r(3), r(9), 0x5c)?)?;
    c.emit(
        0x6d450,
        multiply_long(C::Al, false, R::R1, R::R2, r(5), R::R0)?,
    )?;
    c.emit(
        0x6d454,
        multiply_long(C::Al, false, R::R0, r(7), R::R1, r(3))?,
    )?;
    c.emit(
        0x6d458,
        multiply_accumulate(C::Al, R::R1, R::R2, r(3), r(7))?,
    )?;
    c.emit(
        0x6d45c,
        data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x240)?,
    )?;
    c.emit(0x6d460, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(
        0x6d464,
        data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf4000)?,
    )?;
    c.emit(0x6d468, branch(C::Al, 0x6d468, 0x8187c, true)?)?;
    c.emit(
        0x6d46c,
        data_register(C::Al, Op::Add, r(5), r(10), r(8), Shift::Lsl, 3, false)?,
    )?;
    c.emit(
        0x6d470,
        data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 2, false)?,
    )?;
    c.emit(
        0x6d474,
        transfer_word_preindexed_writeback(C::Al, true, R::R2, r(5), 0x98)?,
    )?;
    c.emit(
        0x6d478,
        data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?,
    )?;
    c.emit(0x6d47c, transfer_word(C::Al, true, r(3), r(5), 0x4)?)?;
    c.emit(
        0x6d480,
        data_register(C::Al, Op::Add, r(6), R::R0, R::R2, Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d484,
        data_register(C::Al, Op::Mov, R::R0, R::R0, R::R1, Shift::Lsr, 2, false)?,
    )?;
    c.emit(
        0x6d488,
        data_register(C::Al, Op::Adc, r(7), R::R0, r(3), Shift::Lsl, 0, false)?,
    )?;
    c.emit(0x6d48c, transfer_doubleword(C::Al, false, r(6), r(5), 0x0)?)?;
    c.emit(0x6d490, transfer_byte(C::Al, true, R::R0, r(9), 0x25)?)?;
    c.emit(0x6d494, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.emit(0x6d498, branch(C::Ne, 0x6d498, 0x6d4fc, false)?)?;
    c.emit(0x6d49c, transfer_word(C::Al, true, R::R1, r(9), 0x2c8)?)?;
    c.emit(0x6d4a0, transfer_word(C::Al, true, R::R2, r(9), 0x2cc)?)?;
    c.emit(
        0x6d4a4,
        data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x69)?,
    )?;
    c.emit(
        0x6d4a8,
        multiply_long(C::Al, false, R::R0, r(3), R::R1, R::R12)?,
    )?;
    c.emit(0x6d4ac, transfer_word(C::Al, false, R::R1, R::SP, 0x38)?)?;
    c.emit(0x6d4b0, transfer_word(C::Al, false, R::R2, R::SP, 0x34)?)?;
    c.emit(
        0x6d4b4,
        multiply_accumulate(C::Al, R::R1, R::R2, R::R12, r(3))?,
    )?;
    c.emit(0x6d4b8, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x64)?)?;
    c.emit(0x6d4bc, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6d4c0, branch(C::Al, 0x6d4c0, 0x8187c, true)?)?;
    c.emit(0x6d4c4, transfer_doubleword(C::Al, true, R::R2, r(4), 0x0)?)?;
    c.emit(
        0x6d4c8,
        data_register(C::Al, Op::Sub, R::R0, R::R0, R::R2, Shift::Lsl, 0, true)?,
    )?;
    c.emit(
        0x6d4cc,
        data_register(C::Al, Op::Sbc, R::R0, R::R1, r(3), Shift::Lsl, 0, true)?,
    )?;
    c.emit(0x6d4d0, branch(C::Cs, 0x6d4d0, 0x6d4fc, false)?)?;
    c.emit(0x6d4d4, transfer_word(C::Al, true, R::R1, R::SP, 0x38)?)?;
    c.emit(0x6d4d8, transfer_word(C::Al, true, r(7), R::SP, 0x34)?)?;
    c.emit(0x6d4dc, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x6c)?)?;
    c.emit(
        0x6d4e0,
        multiply_long(C::Al, false, R::R0, r(3), R::R1, R::R2)?,
    )?;
    c.emit(
        0x6d4e4,
        multiply_accumulate(C::Al, R::R1, r(7), R::R2, r(3))?,
    )?;
    c.emit(0x6d4e8, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x64)?)?;
    c.emit(0x6d4ec, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6d4f0, branch(C::Al, 0x6d4f0, 0x8187c, true)?)?;
    c.emit(
        0x6d4f4,
        transfer_doubleword(C::Al, false, R::R0, r(4), 0x0)?,
    )?;
    c.emit(0x6d4f8, transfer_doubleword(C::Al, true, r(6), r(5), 0x0)?)?;
    c.emit(
        0x6d4fc,
        data_immediate_with_flags(C::Al, Op::Sub, R::R0, r(6), 0x1, true)?,
    )?;
    c.emit(
        0x6d500,
        data_immediate_with_flags(C::Al, Op::Sbc, R::R0, r(7), 0x0, true)?,
    )?;
    c.emit(0x6d504, branch(C::Lt, 0x6d504, 0x6d8ac, false)?)?;
    c.emit(0x6d508, transfer_byte(C::Al, true, R::R0, r(9), 0x25)?)?;
    c.emit(
        0x6d50c,
        data_register(C::Al, Op::Add, r(3), r(10), r(8), Shift::Lsl, 3, false)?,
    )?;
    c.emit(0x6d510, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6d514, branch(C::Hi, 0x6d514, 0x6d740, false)?)?;
    c.emit(
        0x6d518,
        transfer_doubleword(C::Al, true, R::R0, r(3), 0xc8)?,
    )?;
    c.emit(
        0x6d51c,
        data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsl, 1, false)?,
    )?;
    c.emit(
        0x6d520,
        data_register(C::Al, Op::Orr, R::R1, R::R1, R::R0, Shift::Lsr, 31, false)?,
    )?;
    c.emit(
        0x6d524,
        data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?,
    )?;
    c.emit(0x6d528, branch(C::Al, 0x6d528, 0x6d888, false)?)?;
    c.emit(
        0x6d52c,
        multiply_long(C::Al, false, R::R0, r(7), r(6), r(8))?,
    )?;
    c.emit(
        0x6d530,
        multiply_accumulate(C::Al, r(4), r(6), R::R1, r(7))?,
    )?;
    c.emit(
        0x6d534,
        multiply_accumulate(C::Al, R::R1, r(5), r(8), r(4))?,
    )?;
    c.emit(0x6d538, branch(C::Al, 0x6d538, 0x8187c, true)?)?;
    c.emit(0x6d53c, transfer_word(C::Al, true, r(9), R::R11, -0x3c)?)?;
    c.emit(0x6d540, transfer_word(C::Al, true, R::R12, R::R11, -0x48)?)?;
    c.emit(0x6d544, transfer_word(C::Al, true, r(5), R::SP, 0x34)?)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("scaled accumulation length mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert_eq!(actual, expected, "word at {:#x}", START + index * 4);
        }
    }

    #[test]
    fn bounds_and_protected_words() {
        assert_eq!(fragment().unwrap().len(), END - START);
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        for at in [START - 4, START + 1, END, 0x6d26c, 0x6d444] {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(
                c.emit(at, arm32::mov_register(C::Al, R::R0, R::R0))
                    .is_err()
            );
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        for (at, word) in [(0x6d26c, 0x000f2d78), (0x6d444, 0x51eb851f)] {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(c.data(at, word ^ 1).is_err());
            assert!(c.data(at + 4, word).is_err());
            assert!(c.bytes.is_empty());
            c.data(at, word).unwrap();
            assert_eq!(c.bytes, word.to_le_bytes());
        }
    }
}
