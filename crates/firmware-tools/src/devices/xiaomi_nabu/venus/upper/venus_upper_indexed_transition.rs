use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags, data_register,
    data_register_shifted_by_register, multiply_accumulate, multiply_long, multiply_register, transfer_byte,
    transfer_doubleword, transfer_multiple, transfer_word, transfer_word_postindexed,
    transfer_word_preindexed_writeback, transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R,
    Shift,
};

pub const START: usize = 0x6c264;
pub const CODE_END: usize = 0x6c95c;
pub const END: usize = 0x6c964;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = CODE_END - START;
pub const DATA_BYTES: usize = END - CODE_END;
pub const LOWER_SEMANTIC_THRESHOLD: u32 = 0x15752a00;
pub const UPPER_SEMANTIC_THRESHOLD: u32 = 0x15846c3f;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if self.at != at || !(START..CODE_END).contains(&at) || at % 4 != 0 {
            return Err(format!("indexed transition instruction address {at:#x}, cursor {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn thresholds(&mut self) -> Result<(), String> {
        if self.at != CODE_END || self.bytes.len() != CODE_BYTES {
            return Err("indexed transition threshold boundary mismatch".into());
        }
        self.bytes.extend_from_slice(&LOWER_SEMANTIC_THRESHOLD.to_le_bytes());
        self.bytes.extend_from_slice(&UPPER_SEMANTIC_THRESHOLD.to_le_bytes());
        self.at = END;
        Ok(())
    }
}

fn r(n: u8) -> R {
    R::new(n).expect("valid indexed transition register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(0x6c264, transfer_word(C::Al, true, R::R1, R::SP, 0x34)?)?;
    c.emit(0x6c268, data_immediate(C::Al, Op::And, R::R0, r(6), 0x7)?)?;
    c.emit(0x6c26c, data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 0, true)?)?;
    c.emit(0x6c270, branch(C::Eq, 0x6c270, 0x6c834, false)?)?;
    c.emit(0x6c274, transfer_word(C::Al, true, r(7), r(9), 0x5c)?)?;
    c.emit(0x6c278, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 2, false)?)?;
    c.emit(0x6c27c, data_register(C::Al, Op::Mov, R::R1, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c280, branch_exchange_immediate(0x6c280, 0x83c4c)?)?;
    c.emit(0x6c284, transfer_word(C::Al, false, R::R0, R::SP, 0x18)?)?;
    c.emit(0x6c288, data_immediate(C::Al, Op::Tst, R::R0, r(6), 0x2)?)?;
    c.emit(0x6c28c, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c290, transfer_word(C::Al, true, R::R0, r(9), 0x2a0)?)?;
    c.emit(0x6c294, transfer_word(C::Eq, true, R::R1, R::SP, 0x24)?)?;
    c.emit(0x6c298, transfer_word(C::Al, false, R::R0, R::R11, -0x48)?)?;
    c.emit(0x6c29c, data_immediate(C::Eq, Op::And, R::R0, r(6), 0x4)?)?;
    c.emit(0x6c2a0, data_register(C::Eq, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 0, true)?)?;
    c.emit(0x6c2a4, branch(C::Eq, 0x6c2a4, 0x6c590, false)?)?;
    c.emit(0x6c2a8, transfer_word(C::Al, true, R::R2, R::SP, 0x18)?)?;
    c.emit(0x6c2ac, transfer_word(C::Al, false, R::R2, r(9), 0x60)?)?;
    c.emit(0x6c2b0, transfer_word(C::Al, true, R::R0, R::SP, 0x28)?)?;
    c.emit(0x6c2b4, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c2b8, branch(C::Eq, 0x6c2b8, 0x6c3bc, false)?)?;
    c.emit(0x6c2bc, data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0x0)?)?;
    c.emit(0x6c2c0, branch(C::Eq, 0x6c2c0, 0x6c440, false)?)?;
    c.emit(0x6c2c4, transfer_word(C::Al, true, R::R1, R::PC, 0xfa0)?)?;
    c.emit(0x6c2c8, data_register(C::Al, Op::Add, R::R0, R::LR, R::LR, Shift::Lsl, 1, false)?)?;
    c.emit(0x6c2cc, data_immediate(C::Al, Op::Mov, r(8), R::R0, 0x0)?)?;
    c.emit(0x6c2d0, data_immediate(C::Al, Op::Mov, r(9), R::R0, 0x0)?)?;
    c.emit(0x6c2d4, data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 1, false)?)?;
    c.emit(0x6c2d8, data_immediate(C::Al, Op::Sub, r(10), R::R0, 0xc)?)?;
    c.emit(0x6c2dc, transfer_word(C::Al, true, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6c2e0, transfer_word(C::Al, false, r(10), R::SP, 0x30)?)?;
    c.emit(0x6c2e4, data_immediate(C::Al, Op::Add, r(6), R::R0, 0x68)?)?;
    c.emit(0x6c2e8, data_immediate(C::Al, Op::Sub, R::R0, R::LR, 0x1)?)?;
    c.emit(0x6c2ec, transfer_word(C::Al, false, R::R0, R::SP, 0x48)?)?;
    c.emit(0x6c2f0, data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0x1)?)?;
    c.emit(0x6c2f4, branch(C::Hi, 0x6c2f4, 0x6c30c, false)?)?;
    c.emit(0x6c2f8, transfer_word(C::Al, true, R::R0, R::R11, -0x3c)?)?;
    c.emit(0x6c2fc, transfer_word(C::Al, true, R::R1, R::SP, 0x48)?)?;
    c.emit(0x6c300, transfer_word(C::Al, true, R::R0, R::R0, 0x6c)?)?;
    c.emit(0x6c304, data_register_shifted_by_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, R::R1, false)?)?;
    c.emit(0x6c308, branch(C::Al, 0x6c308, 0x6c314, false)?)?;
    c.emit(0x6c30c, transfer_word(C::Al, true, R::R0, r(6), -0x4c)?)?;
    c.emit(0x6c310, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    c.emit(0x6c314, transfer_word(C::Al, true, r(7), R::R11, -0x48)?)?;
    c.emit(0x6c318, transfer_word(C::Al, false, R::R0, r(6), -0x48)?)?;
    c.emit(0x6c31c, data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0x0)?)?;
    c.emit(0x6c320, transfer_byte(C::Al, true, R::R1, r(10), 0x0)?)?;
    c.emit(0x6c324, multiply_accumulate(C::Al, r(4), R::R0, R::R1, r(9))?)?;
    c.emit(0x6c328, branch(C::Eq, 0x6c328, 0x6c33c, false)?)?;
    c.emit(0x6c32c, transfer_word(C::Al, true, r(5), R::R11, -0x3c)?)?;
    c.emit(0x6c330, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(0x6c334, data_register(C::Al, Op::Mov, r(9), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c338, branch(C::Al, 0x6c338, 0x6c378, false)?)?;
    c.emit(0x6c33c, transfer_word(C::Al, true, r(5), R::R11, -0x3c)?)?;
    c.emit(0x6c340, transfer_word(C::Al, true, R::R2, r(5), 0x5c)?)?;
    c.emit(0x6c344, transfer_word(C::Al, true, R::R1, r(5), 0x6c)?)?;
    c.emit(0x6c348, multiply_register(C::Al, R::R0, R::R2, r(7))?)?;
    c.emit(0x6c34c, branch_exchange_immediate(0x6c34c, 0x83c4c)?)?;
    c.emit(0x6c350, transfer_word(C::Al, true, R::R1, R::R11, -0x44)?)?;
    c.emit(0x6c354, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 2, false)?)?;
    c.emit(0x6c358, transfer_word(C::Al, false, R::R0, R::R1, 0x50)?)?;
    c.emit(0x6c35c, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(0x6c360, transfer_word(C::Al, true, R::R0, R::SP, 0x30)?)?;
    c.emit(0x6c364, transfer_byte(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c368, data_register(C::Al, Op::Add, r(9), r(4), R::R0, Shift::Lsl, 2, false)?)?;
    c.emit(0x6c36c, transfer_word(C::Al, true, R::R0, r(6), -0x48)?)?;
    c.emit(0x6c370, data_immediate_with_flags(C::Al, Op::Sub, R::R0, R::R0, 0x1, true)?)?;
    c.emit(0x6c374, data_immediate(C::Al, Op::Sbc, R::R1, R::R1, 0x0)?)?;
    c.emit(0x6c378, multiply_long(C::Al, false, r(3), R::R2, R::R0, r(7))?)?;
    c.emit(0x6c37c, multiply_accumulate(C::Al, r(4), R::R1, r(7), R::R2)?)?;
    c.emit(0x6c380, transfer_word(C::Al, true, r(7), r(5), 0x5c)?)?;
    c.emit(0x6c384, transfer_word(C::Al, true, R::R2, r(5), 0x6c)?)?;
    c.emit(0x6c388, multiply_long(C::Al, false, R::R0, r(5), r(3), r(7))?)?;
    c.emit(0x6c38c, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c390, multiply_accumulate(C::Al, R::R1, r(4), r(7), r(5))?)?;
    c.emit(0x6c394, branch(C::Al, 0x6c394, 0x8187c, true)?)?;
    c.emit(0x6c398, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 2, false)?)?;
    c.emit(0x6c39c, data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(0x6c3a0, data_immediate(C::Al, Op::Add, r(10), r(10), 0x1)?)?;
    c.emit(0x6c3a4, data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(0x6c3a8, transfer_word_postindexed(C::Al, false, R::R0, r(6), 0x4)?)?;
    c.emit(0x6c3ac, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c3b0, data_register(C::Al, Op::Cmp, R::R0, r(8), R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c3b4, branch(C::Ne, 0x6c3b4, 0x6c2f0, false)?)?;
    c.emit(0x6c3b8, branch(C::Al, 0x6c3b8, 0x6c444, false)?)?;
    c.emit(0x6c3bc, transfer_word(C::Al, true, r(5), R::R11, -0x48)?)?;
    c.emit(0x6c3c0, data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsr, 2, false)?)?;
    c.emit(0x6c3c4, multiply_register(C::Al, R::R1, r(5), R::R0)?)?;
    c.emit(0x6c3c8, multiply_register(C::Al, R::R0, r(5), r(7))?)?;
    c.emit(0x6c3cc, transfer_word(C::Al, false, R::R1, r(9), 0x2b0)?)?;
    c.emit(0x6c3d0, transfer_word(C::Al, true, r(6), r(9), 0x6c)?)?;
    c.emit(0x6c3d4, data_register(C::Al, Op::Mov, R::R1, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c3d8, branch_exchange_immediate(0x6c3d8, 0x83c4c)?)?;
    c.emit(0x6c3dc, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 2, false)?)?;
    c.emit(0x6c3e0, multiply_long(C::Al, false, r(4), r(8), r(5), r(7))?)?;
    c.emit(0x6c3e4, transfer_word(C::Al, false, R::R0, r(9), 0x2b4)?)?;
    c.emit(0x6c3e8, transfer_word(C::Al, true, R::R2, r(9), 0x64)?)?;
    c.emit(0x6c3ec, multiply_long(C::Al, false, R::R0, r(3), r(4), R::R2)?)?;
    c.emit(0x6c3f0, multiply_accumulate(C::Al, R::R1, r(8), R::R2, r(3))?)?;
    c.emit(0x6c3f4, data_register(C::Al, Op::Mov, R::R2, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c3f8, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c3fc, branch(C::Al, 0x6c3fc, 0x8187c, true)?)?;
    c.emit(0x6c400, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 2, false)?)?;
    c.emit(0x6c404, data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(0x6c408, transfer_word(C::Al, false, R::R0, r(9), 0x2b8)?)?;
    c.emit(0x6c40c, transfer_word(C::Al, true, R::R2, r(9), 0x68)?)?;
    c.emit(0x6c410, multiply_long(C::Al, false, R::R0, r(3), r(4), R::R2)?)?;
    c.emit(0x6c414, multiply_accumulate(C::Al, R::R1, r(8), R::R2, r(3))?)?;
    c.emit(0x6c418, data_register(C::Al, Op::Mov, R::R2, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c41c, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c420, branch(C::Al, 0x6c420, 0x8187c, true)?)?;
    c.emit(0x6c424, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 2, false)?)?;
    c.emit(0x6c428, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c42c, data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(0x6c430, transfer_word(C::Al, false, R::R0, r(9), 0x2bc)?)?;
    c.emit(0x6c434, transfer_word(C::Al, true, R::R0, R::SP, 0x18)?)?;
    c.emit(0x6c438, transfer_word(C::Al, false, R::R0, r(9), 0x60)?)?;
    c.emit(0x6c43c, branch(C::Al, 0x6c43c, 0x6c59c, false)?)?;
    c.emit(0x6c440, data_immediate(C::Al, Op::Mov, r(9), R::R0, 0x0)?)?;
    c.emit(0x6c444, transfer_word(C::Al, true, R::R0, R::SP, 0x24)?)?;
    c.emit(0x6c448, transfer_word(C::Al, true, r(10), R::SP, 0x44)?)?;
    c.emit(0x6c44c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c450, branch(C::Eq, 0x6c450, 0x6c51c, false)?)?;
    c.emit(0x6c454, transfer_word(C::Al, true, R::R0, R::SP, 0x20)?)?;
    c.emit(0x6c458, transfer_byte(C::Al, true, R::R0, R::R0, 0x170)?)?;
    c.emit(0x6c45c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c460, branch(C::Ne, 0x6c460, 0x6c51c, false)?)?;
    c.emit(0x6c464, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3f4)?)?;
    c.emit(0x6c468, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xb0000)?)?;
    c.emit(0x6c46c, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xfc00)?)?;
    c.emit(0x6c470, data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 16, false)?)?;
    c.emit(0x6c474, data_immediate(C::Al, Op::Add, R::R1, R::LR, 0xff)?)?;
    c.emit(0x6c478, data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xff00)?)?;
    c.emit(0x6c47c, data_register(C::Al, Op::Mov, r(5), R::R0, R::R1, Shift::Lsl, 16, false)?)?;
    c.emit(0x6c480, data_register(C::Al, Op::Mov, r(4), R::R0, r(5), Shift::Asr, 16, false)?)?;
    c.emit(0x6c484, data_register(C::Al, Op::Cmp, R::R0, r(4), R::R0, Shift::Asr, 16, false)?)?;
    c.emit(0x6c488, branch(C::Le, 0x6c488, 0x6c51c, false)?)?;
    c.emit(0x6c48c, transfer_word(C::Al, true, R::R0, R::R11, -0x3c)?)?;
    c.emit(0x6c490, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x64)?)?;
    c.emit(0x6c494, transfer_word(C::Al, true, R::R1, R::R0, 0x48)?)?;
    c.emit(0x6c498, multiply_register(C::Al, R::R0, R::R1, R::R2)?)?;
    c.emit(0x6c49c, data_register(C::Al, Op::Mov, R::R1, R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c4a0, branch_exchange_immediate(0x6c4a0, 0x83c4c)?)?;
    c.emit(0x6c4a4, transfer_word(C::Al, true, R::R2, R::R11, -0x44)?)?;
    c.emit(0x6c4a8, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c4ac, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(0x6c4b0, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c4b4, data_register(C::Al, Op::Add, R::R1, R::R1, r(5), Shift::Asr, 16, false)?)?;
    c.emit(0x6c4b8, data_register(C::Al, Op::Add, R::R2, R::R2, r(5), Shift::Asr, 14, false)?)?;
    c.emit(0x6c4bc, data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x20)?)?;
    c.emit(0x6c4c0, data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.emit(0x6c4c4, branch(C::Eq, 0x6c4c4, 0x6c508, false)?)?;
    c.emit(0x6c4c8, transfer_word(C::Al, true, r(7), R::R2, 0x0)?)?;
    c.emit(0x6c4cc, data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    c.emit(0x6c4d0, data_immediate(C::Al, Op::Sub, r(4), r(4), 0x1)?)?;
    c.emit(0x6c4d4, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(0x6c4d8, multiply_register(C::Al, r(6), r(7), R::R0)?)?;
    c.emit(0x6c4dc, transfer_word(C::Al, false, r(6), R::R2, -0x18)?)?;
    c.emit(0x6c4e0, multiply_accumulate(C::Al, r(6), r(7), R::R0, r(3))?)?;
    c.emit(0x6c4e4, data_register(C::Al, Op::Rsb, R::R0, R::R0, R::R0, Shift::Lsl, 4, false)?)?;
    c.emit(0x6c4e8, data_immediate(C::Al, Op::Sub, R::R2, R::R2, 0x4)?)?;
    c.emit(0x6c4ec, transfer_word(C::Al, true, r(3), R::PC, 0xf50)?)?;
    c.emit(0x6c4f0, data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsl, 3, false)?)?;
    c.emit(0x6c4f4, multiply_long(C::Al, false, r(7), r(5), R::R0, r(3))?)?;
    c.emit(0x6c4f8, data_register(C::Al, Op::Mov, r(3), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c4fc, data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsr, 5, false)?)?;
    c.emit(0x6c500, branch(C::Gt, 0x6c500, 0x6c4c0, false)?)?;
    c.emit(0x6c504, branch(C::Al, 0x6c504, 0x6c51c, false)?)?;
    c.emit(0x6c508, transfer_word(C::Al, true, R::R0, R::R11, -0x3c)?)?;
    c.emit(0x6c50c, transfer_word(C::Al, true, R::R1, R::R11, -0x44)?)?;
    c.emit(0x6c510, transfer_word(C::Al, true, R::R0, R::R0, 0x48)?)?;
    c.emit(0x6c514, data_register(C::Al, Op::Sub, R::R0, R::R0, r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c518, transfer_word(C::Al, false, R::R0, R::R1, 0x8)?)?;
    c.emit(0x6c51c, transfer_word(C::Al, true, r(9), R::R11, -0x3c)?)?;
    c.emit(0x6c520, data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0x0)?)?;
    c.emit(0x6c524, branch(C::Eq, 0x6c524, 0x6c590, false)?)?;
    c.emit(0x6c528, transfer_word(C::Al, true, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6c52c, data_immediate(C::Al, Op::Mov, r(6), R::R0, 0x0)?)?;
    c.emit(0x6c530, data_immediate(C::Al, Op::Add, r(4), R::R0, 0x20)?)?;
    c.emit(0x6c534, transfer_word(C::Al, true, R::R0, R::SP, 0x20)?)?;
    c.emit(0x6c538, transfer_byte(C::Al, true, r(8), R::R0, 0x171)?)?;
    c.emit(0x6c53c, data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0x0)?)?;
    c.emit(0x6c540, branch(C::Eq, 0x6c540, 0x6c578, false)?)?;
    c.emit(0x6c544, transfer_word(C::Al, true, R::R1, r(9), 0x6c)?)?;
    c.emit(0x6c548, transfer_word(C::Al, true, R::R2, r(4), -0x18)?)?;
    c.emit(0x6c54c, transfer_word(C::Al, true, R::R0, r(9), 0x5c)?)?;
    c.emit(0x6c550, transfer_word(C::Al, true, r(3), r(4), 0x0)?)?;
    c.emit(0x6c554, multiply_long(C::Al, false, r(7), r(5), R::R2, R::R1)?)?;
    c.emit(0x6c558, multiply_register(C::Al, R::R2, R::R0, r(3))?)?;
    c.emit(0x6c55c, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c560, data_register(C::Al, Op::Mov, R::R1, R::R0, r(5), Shift::Lsl, 2, false)?)?;
    c.emit(0x6c564, data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 2, false)?)?;
    c.emit(0x6c568, data_register(C::Al, Op::Orr, R::R1, R::R1, r(7), Shift::Lsr, 30, false)?)?;
    c.emit(0x6c56c, branch(C::Al, 0x6c56c, 0x8187c, true)?)?;
    c.emit(0x6c570, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c574, branch(C::Al, 0x6c574, 0x6c57c, false)?)?;
    c.emit(0x6c578, transfer_word(C::Al, true, R::R0, r(9), 0x60)?)?;
    c.emit(0x6c57c, data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(0x6c580, transfer_word(C::Al, false, R::R0, r(4), 0x18)?)?;
    c.emit(0x6c584, data_immediate(C::Al, Op::Add, r(4), r(4), 0x4)?)?;
    c.emit(0x6c588, data_register(C::Al, Op::Cmp, R::R0, r(6), R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c58c, branch(C::Ne, 0x6c58c, 0x6c53c, false)?)?;
    c.emit(0x6c590, transfer_word(C::Al, true, R::R0, R::SP, 0x28)?)?;
    c.emit(0x6c594, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c598, branch(C::Ne, 0x6c598, 0x6c5ac, false)?)?;
    c.emit(0x6c59c, transfer_word(C::Al, true, R::R0, R::SP, 0x40)?)?;
    c.emit(0x6c5a0, transfer_byte(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c5a4, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6c5a8, branch(C::Ne, 0x6c5a8, 0x6c724, false)?)?;
    c.emit(0x6c5ac, transfer_word(C::Al, true, R::R0, R::SP, 0x34)?)?;
    c.emit(0x6c5b0, transfer_word(C::Al, true, R::R12, R::R11, -0x44)?)?;
    c.emit(0x6c5b4, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c5b8, branch(C::Eq, 0x6c5b8, 0x6c834, false)?)?;
    c.emit(0x6c5bc, data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0x0)?)?;
    c.emit(0x6c5c0, transfer_word(C::Ne, true, R::R0, R::SP, 0x40)?)?;
    c.emit(0x6c5c4, transfer_word(C::Ne, true, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c5c8, data_immediate_with_flags(C::Ne, Op::And, R::R0, R::R0, 0x1, true)?)?;
    c.emit(0x6c5cc, branch(C::Eq, 0x6c5cc, 0x6c844, false)?)?;
    c.emit(0x6c5d0, data_immediate(C::Al, Op::Add, R::R0, r(9), 0x27c)?)?;
    c.emit(0x6c5d4, data_register(C::Al, Op::Mov, r(8), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c5d8, data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x0)?)?;
    c.emit(0x6c5dc, data_immediate(C::Al, Op::Sub, R::R2, R::R11, 0x38)?)?;
    c.emit(0x6c5e0, data_immediate(C::Al, Op::Mov, r(10), R::R0, 0x0)?)?;
    c.emit(0x6c5e4, data_immediate(C::Al, Op::Add, r(6), R::R0, 0x1000)?)?;
    c.emit(0x6c5e8, data_register(C::Al, Op::Mov, r(3), R::R0, R::R12, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c5ec, data_register(C::Al, Op::Mov, r(4), R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c5f0, data_register(C::Al, Op::Add, r(9), r(3), r(10), Shift::Lsl, 2, false)?)?;
    c.emit(0x6c5f4, transfer_word_postindexed(C::Al, true, R::R2, r(4), 0x4)?)?;
    c.emit(0x6c5f8, transfer_word(C::Al, true, R::R1, r(9), 0x8)?)?;
    c.emit(0x6c5fc, transfer_word(C::Al, true, R::R0, r(9), 0x38)?)?;
    c.emit(0x6c600, transfer_word(C::Al, false, r(4), R::SP, 0x48)?)?;
    c.emit(0x6c604, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c608, data_register(C::Al, Op::Mov, r(5), R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c60c, data_register(C::Cc, Op::Mov, r(5), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c610, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c614, branch(C::Ls, 0x6c614, 0x6c664, false)?)?;
    c.emit(0x6c618, data_register(C::Al, Op::Add, R::R0, r(3), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c61c, transfer_word_preindexed_writeback(C::Al, true, R::R12, R::R0, 0xc8)?)?;
    c.emit(0x6c620, transfer_word(C::Al, true, r(4), R::R0, -0x30)?)?;
    c.emit(0x6c624, transfer_word(C::Al, true, R::R1, R::R0, -0x2c)?)?;
    c.emit(0x6c628, transfer_word(C::Al, true, r(8), R::R11, -0x48)?)?;
    c.emit(0x6c62c, transfer_word(C::Al, false, R::R1, R::SP, 0x38)?)?;
    c.emit(0x6c630, multiply_register(C::Al, R::R2, r(5), r(8))?)?;
    c.emit(0x6c634, data_register(C::Al, Op::Mov, R::R1, R::R0, R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c638, transfer_word(C::Al, true, r(3), R::R0, 0x4)?)?;
    c.emit(0x6c63c, transfer_word(C::Al, true, r(5), R::SP, 0x38)?)?;
    c.emit(0x6c640, data_register(C::Al, Op::Rsb, R::R1, R::R12, R::R2, Shift::Lsr, 6, true)?)?;
    c.emit(0x6c644, data_register(C::Al, Op::Mov, R::R2, R::R0, R::R2, Shift::Lsr, 6, false)?)?;
    c.emit(0x6c648, data_immediate(C::Al, Op::Rsc, r(3), r(3), 0x0)?)?;
    c.emit(0x6c64c, data_register(C::Al, Op::Add, R::R1, R::R1, r(4), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c650, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0x6c654, data_register(C::Al, Op::Adc, r(3), r(3), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c658, transfer_word(C::Al, false, R::R1, R::R0, -0x30)?)?;
    c.emit(0x6c65c, transfer_word(C::Al, false, r(3), R::R0, -0x2c)?)?;
    c.emit(0x6c660, branch(C::Al, 0x6c660, 0x6c6e0, false)?)?;
    c.emit(0x6c664, branch(C::Cs, 0x6c664, 0x6c6ec, false)?)?;
    c.emit(0x6c668, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6c66c, transfer_word(C::Al, false, R::R0, r(6), -0x8)?)?;
    c.emit(0x6c670, transfer_byte(C::Al, true, R::R0, r(8), 0x25)?)?;
    c.emit(0x6c674, transfer_word(C::Al, true, R::R1, r(8), 0x5c)?)?;
    c.emit(0x6c678, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(0x6c67c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    c.emit(0x6c680, data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 0x2)?)?;
    c.emit(0x6c684, data_register_shifted_by_register(C::Al, Op::Mov, R::R2, R::R0, R::R1, Shift::Lsr, R::R0, false)?)?;
    c.emit(0x6c688, transfer_word(C::Al, false, R::R2, r(6), -0x4)?)?;
    c.emit(0x6c68c, transfer_word(C::Al, true, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6c690, data_register(C::Al, Op::Add, r(8), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c694, transfer_word(C::Al, true, R::R0, r(8), 0x98)?)?;
    c.emit(0x6c698, transfer_word(C::Al, true, r(4), r(8), 0x9c)?)?;
    c.emit(0x6c69c, transfer_word(C::Al, true, r(3), r(8), 0xc8)?)?;
    c.emit(0x6c6a0, transfer_word(C::Al, true, R::R1, r(8), 0xcc)?)?;
    c.emit(0x6c6a4, data_register(C::Al, Op::Sub, R::R0, r(3), R::R0, Shift::Lsl, 0, true)?)?;
    c.emit(0x6c6a8, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c6ac, data_register(C::Al, Op::Sbc, R::R1, R::R1, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c6b0, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0x6c6b4, branch(C::Al, 0x6c6b4, 0x8187c, true)?)?;
    c.emit(0x6c6b8, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c6bc, transfer_word(C::Al, false, R::R0, r(6), 0x0)?)?;
    c.emit(0x6c6c0, transfer_word(C::Al, true, R::R1, R::R11, -0x48)?)?;
    c.emit(0x6c6c4, multiply_register(C::Al, R::R0, r(5), R::R1)?)?;
    c.emit(0x6c6c8, data_register(C::Al, Op::Mov, R::R2, R::R0, R::R0, Shift::Lsr, 6, false)?)?;
    c.emit(0x6c6cc, transfer_word(C::Al, false, R::R2, r(8), 0xc8)?)?;
    c.emit(0x6c6d0, transfer_word(C::Al, false, r(4), r(8), 0xcc)?)?;
    c.emit(0x6c6d4, transfer_word(C::Al, true, R::R0, R::R11, -0x44)?)?;
    c.emit(0x6c6d8, data_register(C::Al, Op::Add, R::R0, R::R0, r(10), Shift::Lsl, 3, false)?)?;
    c.emit(0x6c6dc, data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x98)?)?;
    c.emit(
        0x6c6e0,
        transfer_multiple(C::Al, false, R::R0, R::R2.bit() | r(4).bit(), BlockMode::IncrementAfter, false)?,
    )?;
    c.emit(0x6c6e4, transfer_word(C::Al, true, R::R0, r(9), 0x38)?)?;
    c.emit(0x6c6e8, transfer_word(C::Al, true, r(8), R::R11, -0x3c)?)?;
    c.emit(0x6c6ec, transfer_word(C::Al, true, R::R12, R::R11, -0x44)?)?;
    c.emit(0x6c6f0, data_immediate(C::Al, Op::Add, r(10), r(10), 0x1)?)?;
    c.emit(0x6c6f4, data_immediate(C::Al, Op::Add, r(6), r(6), 0xc)?)?;
    c.emit(0x6c6f8, data_register(C::Al, Op::Add, R::R1, R::R12, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c6fc, data_immediate(C::Al, Op::Add, r(7), r(7), 0x8)?)?;
    c.emit(0x6c700, transfer_doubleword(C::Al, true, R::R2, R::R1, 0x98)?)?;
    c.emit(0x6c704, data_register(C::Al, Op::Sub, R::R0, R::R2, R::R0, Shift::Lsl, 0, true)?)?;
    c.emit(0x6c708, data_immediate(C::Al, Op::Sbc, r(3), r(3), 0x0)?)?;
    c.emit(0x6c70c, transfer_word(C::Al, false, R::R0, R::R1, 0x98)?)?;
    c.emit(0x6c710, data_register(C::Al, Op::Cmp, R::R0, r(10), R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c714, transfer_word(C::Al, false, r(3), R::R1, 0x9c)?)?;
    c.emit(0x6c718, transfer_word(C::Al, true, R::R2, R::SP, 0x48)?)?;
    c.emit(0x6c71c, branch(C::Ne, 0x6c71c, 0x6c5e8, false)?)?;
    c.emit(0x6c720, branch(C::Al, 0x6c720, 0x6c820, false)?)?;
    c.emit(0x6c724, transfer_word(C::Al, true, R::R0, R::SP, 0x38)?)?;
    c.emit(0x6c728, transfer_word(C::Al, true, R::R1, R::SP, 0x1c)?)?;
    c.emit(0x6c72c, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c730, branch(C::Cs, 0x6c730, 0x6c770, false)?)?;
    c.emit(0x6c734, data_immediate(C::Al, Op::Add, r(7), r(9), 0x2c0)?)?;
    c.emit(0x6c738, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(
        0x6c73c,
        transfer_multiple(
            C::Al,
            true,
            r(7),
            R::R1.bit() | R::R2.bit() | r(3).bit() | r(7).bit(),
            BlockMode::IncrementAfter,
            false,
        )?,
    )?;
    c.emit(0x6c740, transfer_word(C::Al, true, R::R0, r(9), 0x2b0)?)?;
    c.emit(0x6c744, transfer_word(C::Al, true, r(6), R::SP, 0x18)?)?;
    c.emit(0x6c748, data_register(C::Al, Op::Sub, R::R1, R::R1, r(3), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c74c, data_register(C::Al, Op::Sbc, R::R2, R::R2, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c750, multiply_long(C::Al, false, r(3), r(7), R::R0, r(6))?)?;
    c.emit(0x6c754, data_register(C::Al, Op::Mov, r(3), R::R0, r(3), Shift::Lsr, 6, false)?)?;
    c.emit(0x6c758, data_register(C::Al, Op::Orr, r(3), r(3), r(7), Shift::Lsl, 26, false)?)?;
    c.emit(0x6c75c, data_register(C::Al, Op::Add, R::R1, R::R1, r(3), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c760, data_register(C::Al, Op::Adc, R::R2, R::R2, r(7), Shift::Lsr, 6, false)?)?;
    c.emit(0x6c764, transfer_word(C::Al, false, R::R1, r(9), 0x2c0)?)?;
    c.emit(0x6c768, transfer_word(C::Al, false, R::R2, r(9), 0x2c4)?)?;
    c.emit(0x6c76c, branch(C::Al, 0x6c76c, 0x6c7e4, false)?)?;
    c.emit(0x6c770, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x268)?)?;
    c.emit(0x6c774, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(0x6c778, data_immediate(C::Al, Op::Add, r(7), r(9), 0x2c4)?)?;
    c.emit(0x6c77c, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0x6c780, data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    c.emit(0x6c784, transfer_word_register(C::Al, false, R::R1, r(9), R::R0, Shift::Lsl, 0)?)?;
    c.emit(0x6c788, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    c.emit(0x6c78c, transfer_word(C::Al, true, R::R1, R::SP, 0x10)?)?;
    c.emit(0x6c790, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.emit(0x6c794, transfer_word(C::Al, true, R::R1, r(9), 0x5c)?)?;
    c.emit(0x6c798, data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 0x2)?)?;
    c.emit(0x6c79c, data_register_shifted_by_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsr, R::R0, false)?)?;
    c.emit(0x6c7a0, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x26c)?)?;
    c.emit(0x6c7a4, data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    c.emit(0x6c7a8, transfer_word_register(C::Al, false, R::R1, r(9), R::R0, Shift::Lsl, 0)?)?;
    c.emit(0x6c7ac, transfer_word(C::Al, true, R::R2, r(9), 0x2c0)?)?;
    c.emit(
        0x6c7b0,
        transfer_multiple(C::Al, true, r(7), R::R0.bit() | r(3).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?,
    )?;
    c.emit(0x6c7b4, data_register(C::Al, Op::Sub, r(6), R::R2, r(3), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c7b8, data_register(C::Al, Op::Sbc, R::R0, R::R0, r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c7bc, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c7c0, branch(C::Cs, 0x6c7c0, 0x6c7d0, false)?)?;
    c.emit(0x6c7c4, data_register(C::Al, Op::Sub, R::R0, r(3), R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c7c8, branch_exchange_immediate(0x6c7c8, 0x83c4c)?)?;
    c.emit(0x6c7cc, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c7d0, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x270)?)?;
    c.emit(0x6c7d4, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(0x6c7d8, transfer_word_register(C::Al, false, R::R0, r(9), R::R1, Shift::Lsl, 0)?)?;
    c.emit(0x6c7dc, transfer_word(C::Al, true, R::R0, r(9), 0x2b0)?)?;
    c.emit(0x6c7e0, transfer_word(C::Al, true, r(6), R::SP, 0x18)?)?;
    c.emit(0x6c7e4, multiply_long(C::Al, false, R::R1, R::R2, r(6), R::R0)?)?;
    c.emit(0x6c7e8, data_immediate(C::Al, Op::Mov, r(7), R::R0, 0x0)?)?;
    c.emit(0x6c7ec, data_register(C::Al, Op::Mov, r(8), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c7f0, multiply_accumulate(C::Al, r(3), r(4), R::R0, R::R2)?)?;
    c.emit(0x6c7f4, data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsr, 6, false)?)?;
    c.emit(0x6c7f8, data_register(C::Al, Op::Orr, R::R1, R::R1, r(3), Shift::Lsl, 26, false)?)?;
    c.emit(0x6c7fc, data_register(C::Al, Op::Mov, R::R0, R::R0, r(3), Shift::Lsr, 6, false)?)?;
    c.emit(0x6c800, data_register(C::Al, Op::Sub, R::R2, R::R1, r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c804, data_register(C::Al, Op::Rsc, r(3), r(7), r(3), Shift::Lsr, 6, false)?)?;
    c.emit(0x6c808, transfer_word(C::Al, false, R::R2, r(9), 0x2c0)?)?;
    c.emit(0x6c80c, transfer_word(C::Al, false, r(3), r(9), 0x2c4)?)?;
    c.emit(0x6c810, transfer_word(C::Al, false, R::R1, r(9), 0x2c8)?)?;
    c.emit(0x6c814, transfer_word(C::Al, false, R::R0, r(9), 0x2cc)?)?;
    c.emit(0x6c818, transfer_word(C::Al, false, r(6), r(9), 0x60)?)?;
    c.emit(0x6c81c, transfer_word(C::Al, true, R::R12, R::R11, -0x44)?)?;
    c.emit(0x6c820, transfer_word(C::Al, true, R::R0, R::SP, 0x34)?)?;
    c.emit(0x6c824, transfer_word(C::Al, true, r(10), R::SP, 0x44)?)?;
    c.emit(0x6c828, data_register(C::Al, Op::Mov, r(9), R::R0, r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c82c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c830, branch(C::Ne, 0x6c830, 0x6c844, false)?)?;
    c.emit(0x6c834, transfer_word(C::Al, true, R::R0, R::SP, 0x40)?)?;
    c.emit(0x6c838, transfer_byte(C::Al, true, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c83c, data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.emit(0x6c840, branch(C::Eq, 0x6c840, 0x6c8e0, false)?)?;
    c.emit(0x6c844, transfer_word(C::Al, true, R::R0, r(9), 0x0)?)?;
    c.emit(0x6c848, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.emit(0x6c84c, branch(C::Ne, 0x6c84c, 0x6c8e0, false)?)?;
    c.emit(0x6c850, transfer_word(C::Al, true, R::R0, R::SP, 0x20)?)?;
    c.emit(0x6c854, transfer_byte(C::Al, true, R::R0, R::R0, 0xa8)?)?;
    c.emit(0x6c858, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c85c, branch(C::Ne, 0x6c85c, 0x6c8e0, false)?)?;
    c.emit(0x6c860, data_immediate(C::Al, Op::Add, r(4), r(9), 0x54)?)?;
    c.emit(0x6c864, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x64)?)?;
    c.emit(
        0x6c868,
        transfer_multiple(C::Al, true, r(4), R::R0.bit() | R::R1.bit() | r(4).bit(), BlockMode::IncrementAfter, false)?,
    )?;
    c.emit(0x6c86c, transfer_word(C::Al, true, R::R2, r(9), 0x60)?)?;
    c.emit(0x6c870, multiply_register(C::Al, r(6), R::R0, R::R1)?)?;
    c.emit(0x6c874, multiply_register(C::Al, r(7), R::R2, r(3))?)?;
    c.emit(0x6c878, data_register(C::Al, Op::Add, R::R0, r(7), r(6), Shift::Lsr, 1, false)?)?;
    c.emit(0x6c87c, data_register(C::Al, Op::Mov, R::R1, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c880, branch_exchange_immediate(0x6c880, 0x83c4c)?)?;
    c.emit(0x6c884, data_register(C::Al, Op::Add, R::R1, r(6), r(6), Shift::Lsl, 1, false)?)?;
    c.emit(0x6c888, data_register(C::Al, Op::Mov, R::R2, R::R0, r(4), Shift::Lsr, 2, false)?)?;
    c.emit(0x6c88c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x3)?)?;
    c.emit(0x6c890, data_register(C::Al, Op::Mov, r(3), R::R0, R::R1, Shift::Lsr, 1, false)?)?;
    c.emit(0x6c894, multiply_register(C::Al, R::R1, R::R2, r(3))?)?;
    c.emit(0x6c898, branch(C::Hi, 0x6c898, 0x6c8a8, false)?)?;
    c.emit(0x6c89c, transfer_word(C::Al, true, R::R2, R::SP, 0x14)?)?;
    c.emit(0x6c8a0, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c8a4, branch(C::Hi, 0x6c8a4, 0x6c8b8, false)?)?;
    c.emit(0x6c8a8, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x4)?)?;
    c.emit(0x6c8ac, transfer_word(C::Ls, true, R::R0, R::SP, 0xc)?)?;
    c.emit(0x6c8b0, data_register(C::Ls, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c8b4, branch(C::Cs, 0x6c8b4, 0x6d824, false)?)?;
    c.emit(0x6c8b8, transfer_word(C::Al, true, R::R0, R::SP, 0x20)?)?;
    c.emit(0x6c8bc, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x6e)?)?;
    c.emit(0x6c8c0, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x23)?)?;
    c.emit(0x6c8c4, transfer_word(C::Al, true, R::R0, R::R0, 0x838)?)?;
    c.emit(0x6c8c8, transfer_word(C::Al, false, R::R1, R::R0, 0x350)?)?;
    c.emit(0x6c8cc, transfer_word(C::Al, false, R::R2, R::R0, 0x354)?)?;
    c.emit(0x6c8d0, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x12)?)?;
    c.emit(0x6c8d4, transfer_word(C::Al, false, R::R1, R::R0, 0x358)?)?;
    c.emit(0x6c8d8, transfer_word(C::Al, true, R::LR, R::R11, -0x40)?)?;
    c.emit(0x6c8dc, transfer_word(C::Al, true, R::R12, R::R11, -0x44)?)?;
    c.emit(0x6c8e0, transfer_word(C::Al, true, R::R0, r(9), 0xc)?)?;
    c.emit(0x6c8e4, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0x6c8e8, branch(C::Eq, 0x6c8e8, 0x6c964, false)?)?;
    c.emit(0x6c8ec, transfer_word(C::Al, true, r(6), R::R11, 0x8)?)?;
    c.emit(0x6c8f0, data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0x0)?)?;
    c.emit(0x6c8f4, branch(C::Eq, 0x6c8f4, 0x6c9d4, false)?)?;
    c.emit(0x6c8f8, data_immediate(C::Al, Op::Add, R::R0, r(9), 0x27c)?)?;
    c.emit(0x6c8fc, data_immediate(C::Al, Op::Add, R::R1, R::R12, 0x98)?)?;
    c.emit(0x6c900, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    c.emit(0x6c904, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(0x6c908, data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    c.emit(0x6c90c, transfer_word(C::Al, true, r(7), R::R0, -0x8)?)?;
    c.emit(0x6c910, data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0x1)?)?;
    c.emit(0x6c914, branch(C::Ne, 0x6c914, 0x6c944, false)?)?;
    c.emit(0x6c918, transfer_word(C::Al, true, r(7), R::R0, 0x0)?)?;
    c.emit(0x6c91c, transfer_doubleword(C::Al, true, r(4), R::R1, 0x0)?)?;
    c.emit(0x6c920, data_register(C::Al, Op::Sub, r(4), r(4), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c924, data_register(C::Al, Op::Sbc, r(5), r(5), r(7), Shift::Asr, 31, false)?)?;
    c.emit(0x6c928, transfer_doubleword(C::Al, false, r(4), R::R1, 0x0)?)?;
    c.emit(0x6c92c, transfer_word(C::Al, true, r(7), R::R0, -0x4)?)?;
    c.emit(0x6c930, data_immediate_with_flags(C::Al, Op::Sub, r(7), r(7), 0x1, true)?)?;
    c.emit(0x6c934, transfer_word(C::Al, false, r(7), R::R0, -0x4)?)?;
    c.emit(0x6c938, transfer_word(C::Eq, false, R::R2, R::R0, -0x8)?)?;
    c.emit(0x6c93c, transfer_word(C::Eq, false, R::R2, R::R0, -0x4)?)?;
    c.emit(0x6c940, transfer_word(C::Eq, false, R::R2, R::R0, 0x0)?)?;
    c.emit(0x6c944, data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.emit(0x6c948, data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x8)?)?;
    c.emit(0x6c94c, data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xc)?)?;
    c.emit(0x6c950, data_register(C::Al, Op::Cmp, R::R0, r(3), R::LR, Shift::Lsl, 0, false)?)?;
    c.emit(0x6c954, branch(C::Ne, 0x6c954, 0x6c90c, false)?)?;
    c.emit(0x6c958, branch(C::Al, 0x6c958, 0x6c9d4, false)?)?;
    c.thresholds()?;
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
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE").or_else(|| std::env::var_os("VENUS_REFERENCE"))
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
        let expected = &reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA];
        assert_eq!(bytes.len(), expected.len());
        for (i, (actual, expected)) in bytes.chunks_exact(4).zip(expected.chunks_exact(4)).enumerate() {
            assert_eq!(actual, expected, "runtime address {:#x}", START + i * 4);
        }
        for (at, target) in [(0x6b9cc, CODE_END), (0x6b9d0, CODE_END + 4)] {
            let offset = at + FILE_MAPPING_DELTA;
            let instruction = u32::from_le_bytes(reference[offset..offset + 4].try_into().unwrap());
            assert_eq!(instruction >> 16, 0xe59f);
            assert_eq!(at + 8 + (instruction as usize & 0xfff), target);
        }
    }

    #[test]
    fn exact_extent_and_protected_thresholds() {
        let bytes = fragment().unwrap();
        assert_eq!((bytes.len(), CODE_BYTES, DATA_BYTES), (1792, 1784, 8));
        assert_eq!(word(&bytes, CODE_END), LOWER_SEMANTIC_THRESHOLD);
        assert_eq!(word(&bytes, CODE_END + 4), UPPER_SEMANTIC_THRESHOLD);
        assert_eq!(word(&bytes, CODE_END - 4), branch(C::Al, 0x6c958, 0x6c9d4, false).unwrap());
        for at in [START - 4, START + 1, CODE_END, CODE_END + 4, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0).unwrap()).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, 0).is_err());
        assert!(c.thresholds().is_err());
        let mut c = Code { at: CODE_END, bytes: Vec::new() };
        assert!(c.thresholds().is_err());
        let mut c = Code { at: CODE_END, bytes: vec![0; CODE_BYTES] };
        c.thresholds().unwrap();
        assert_eq!(c.at, END);
        assert!(c.thresholds().is_err());
        assert!(c.emit(END, 0).is_err());
    }

    #[test]
    fn scaled_context_and_indexed_loops() {
        let bytes = fragment().unwrap();
        for (at, rd, rn, offset) in
            [(0x6c274, r(7), r(9), 0x5c), (0x6c300, R::R0, R::R0, 0x6c), (0x6c494, R::R1, R::R0, 0x48)]
        {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, rd, rn, offset).unwrap());
        }
        assert_eq!(
            word(&bytes, 0x6c5f0),
            data_register(C::Al, Op::Add, r(9), r(3), r(10), Shift::Lsl, 2, false).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6c6d8),
            data_register(C::Al, Op::Add, R::R0, R::R0, r(10), Shift::Lsl, 3, false).unwrap()
        );
        for (at, target) in [(0x6c3b4, 0x6c2f0), (0x6c58c, 0x6c53c), (0x6c71c, 0x6c5e8), (0x6c954, 0x6c90c)] {
            assert_eq!(word(&bytes, at), branch(C::Ne, at as u32, target, false).unwrap());
        }
    }

    #[test]
    fn carry_chains_and_predicated_flags() {
        let bytes = fragment().unwrap();
        assert_eq!(
            word(&bytes, 0x6c2a0),
            data_register(C::Eq, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 0, true).unwrap()
        );
        assert_eq!(word(&bytes, 0x6c5c8), data_immediate_with_flags(C::Ne, Op::And, R::R0, R::R0, 1, true).unwrap());
        assert_eq!(
            word(&bytes, 0x6c640),
            data_register(C::Al, Op::Rsb, R::R1, R::R12, R::R2, Shift::Lsr, 6, true).unwrap()
        );
        assert_eq!(word(&bytes, 0x6c648), data_immediate(C::Al, Op::Rsc, r(3), r(3), 0).unwrap());
        assert_eq!(
            word(&bytes, 0x6c64c),
            data_register(C::Al, Op::Add, R::R1, R::R1, r(4), Shift::Lsl, 0, true).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6c654),
            data_register(C::Al, Op::Adc, r(3), r(3), r(5), Shift::Lsl, 0, false).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6c7b8),
            data_register(C::Al, Op::Sbc, R::R0, R::R0, r(7), Shift::Lsl, 0, true).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6c924),
            data_register(C::Al, Op::Sbc, r(5), r(5), r(7), Shift::Asr, 31, false).unwrap()
        );
    }

    #[test]
    fn writeback_pairs_and_countdowns() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6c3a8), transfer_word_postindexed(C::Al, false, R::R0, r(6), 4).unwrap());
        assert_eq!(word(&bytes, 0x6c5f4), transfer_word_postindexed(C::Al, true, R::R2, r(4), 4).unwrap());
        assert_eq!(
            word(&bytes, 0x6c61c),
            transfer_word_preindexed_writeback(C::Al, true, R::R12, R::R0, 0xc8).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6c6e0),
            transfer_multiple(C::Al, false, R::R0, R::R2.bit() | r(4).bit(), BlockMode::IncrementAfter, false).unwrap()
        );
        assert_eq!(word(&bytes, 0x6c91c), transfer_doubleword(C::Al, true, r(4), R::R1, 0).unwrap());
        assert_eq!(word(&bytes, 0x6c928), transfer_doubleword(C::Al, false, r(4), R::R1, 0).unwrap());
        assert_eq!(word(&bytes, 0x6c930), data_immediate_with_flags(C::Al, Op::Sub, r(7), r(7), 1, true).unwrap());
        for (at, offset) in [(0x6c938, -8), (0x6c93c, -4), (0x6c940, 0)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Eq, false, R::R2, R::R0, offset).unwrap());
        }
    }

    #[test]
    fn multiplication_division_and_external_references() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6c378), multiply_long(C::Al, false, r(3), R::R2, R::R0, r(7)).unwrap());
        assert_eq!(word(&bytes, 0x6c37c), multiply_accumulate(C::Al, r(4), R::R1, r(7), R::R2).unwrap());
        for at in [0x6c280, 0x6c34c, 0x6c3d8, 0x6c4a0, 0x6c7c8, 0x6c880] {
            assert_eq!(word(&bytes, at), branch_exchange_immediate(at as u32, 0x83c4c).unwrap());
        }
        for at in [0x6c394, 0x6c3fc, 0x6c420, 0x6c56c, 0x6c6b4] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, 0x8187c, true).unwrap());
        }
        for (at, offset, target) in [(0x6c2c4, 0xfa0, 0x6d26c), (0x6c4ec, 0xf50, 0x6d444)] {
            assert!(target >= END);
            assert_eq!(at + 8 + offset, target);
            let rd = if at == 0x6c2c4 { R::R1 } else { r(3) };
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, rd, R::PC, offset as i32).unwrap());
        }
        assert_eq!(word(&bytes, 0x6c8b4), branch(C::Cs, 0x6c8b4, 0x6d824, false).unwrap());
    }
}
