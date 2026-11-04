use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, multiply_accumulate, multiply_long, rotate_right_with_extend, signed_multiply_accumulate_bottom_halves, transfer_byte, transfer_byte_postindexed, transfer_doubleword, transfer_multiple, transfer_word, transfer_word_postindexed, transfer_word_register, unsigned_multiply_long_accumulate,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x70d64;
pub const END: usize = 0x710c0;
const DIVIDE_BY_SIX_MAGIC: u32 = 0xaaaaaaab;
const SIX_ENTRY_LOOKUP: u32 = 0x000f1cc8;
const RELEASE_DIAGNOSTIC_SOURCE: u32 = 0x000f2e4c;
const TABLE_PAIR_FIRST: u32 = 0x000f2bd8;
const TABLE_PAIR_SECOND: u32 = 0x000f2c38;
const TABLE_PAIR_THIRD: u32 = 0x000f2c98;
const TABLE_PAIR_FOURTH: u32 = 0x000f2cf8;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn instruction(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if at != self.at || at % 4 != 0 || !matches!(at, 0x70d64..0x70f94 | 0x70f9c..0x70fec | 0x70ff0..0x710b0) {
            return Err(format!("invalid history lookup instruction address 0x{at:x}"));
        }
        self.bytes.extend(instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn checked_data(&mut self, at: usize, value: u32) -> Result<(), String> {
        let expected = match at {
            0x70f94 => DIVIDE_BY_SIX_MAGIC,
            0x70f98 => SIX_ENTRY_LOOKUP,
            0x70fec => RELEASE_DIAGNOSTIC_SOURCE,
            0x710b0 => TABLE_PAIR_FIRST,
            0x710b4 => TABLE_PAIR_SECOND,
            0x710b8 => TABLE_PAIR_THIRD,
            0x710bc => TABLE_PAIR_FOURTH,
            _ => return Err(format!("invalid history lookup data address 0x{at:x}")),
        };
        if at != self.at || value != expected {
            return Err(format!("invalid history lookup literal at 0x{at:x}"));
        }
        self.bytes.extend(value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("constant A32 register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.instruction(0x70d64, transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x70d68, data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.instruction(0x70d6c, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70d70, data_register(C::Al, Op::Mov, r(8), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70d74, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70d78, transfer_word(C::Al, true, r(7), r(4), 112)?)?;
    c.instruction(0x70d7c, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.instruction(0x70d80, branch(C::Eq, 0x70d80, 0x70dc0, false)?)?;
    c.instruction(0x70d84, transfer_word(C::Al, true, r(3), r(6), 24)?)?;
    c.instruction(0x70d88, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x15)?)?;
    c.instruction(0x70d8c, branch(C::Cc, 0x70d8c, 0x70dc0, false)?)?;
    c.instruction(0x70d90, multiply_long(C::Al, false, r(0), r(1), r(3), r(7))?)?;
    c.instruction(0x70d94, transfer_word(C::Al, true, r(2), r(4), 116)?)?;
    c.instruction(0x70d98, transfer_word(C::Al, true, r(3), r(6), 28)?)?;
    c.instruction(0x70d9c, unsigned_multiply_long_accumulate(C::Al, r(0), r(1), r(3), r(2))?)?;
    c.instruction(0x70da0, data_register(C::Al, Op::Orr, r(3), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.instruction(0x70da4, multiply_long(C::Ne, false, r(3), r(5), r(7), r(7))?)?;
    c.instruction(0x70da8, unsigned_multiply_long_accumulate(C::Ne, r(3), r(5), r(2), r(2))?)?;
    c.instruction(0x70dac, data_register(C::Ne, Op::Mov, r(2), r(0), r(3), Shift::Lsr, 8, false)?)?;
    c.instruction(0x70db0, data_register(C::Ne, Op::Mov, r(3), r(0), r(5), Shift::Lsr, 8, false)?)?;
    c.instruction(0x70db4, data_register(C::Ne, Op::Orr, r(2), r(2), r(5), Shift::Lsl, 24, false)?)?;
    c.instruction(0x70db8, data_register(C::Ne, Op::Orr, r(5), r(2), r(3), Shift::Lsl, 0, true)?)?;
    c.instruction(0x70dbc, branch(C::Ne, 0x70dbc, 0x70e58, false)?)?;
    c.instruction(0x70dc0, transfer_word(C::Al, true, r(6), r(6), 4)?)?;
    c.instruction(0x70dc4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0x70dc8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.instruction(0x70dcc, data_register(C::Al, Op::Cmp, r(0), r(6), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70dd0, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xff00)?)?;
    c.instruction(0x70dd4, branch(C::Cs, 0x70dd4, 0x70e4c, false)?)?;
    c.instruction(0x70dd8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x70ddc, branch(C::Eq, 0x70ddc, 0x70e4c, false)?)?;
    c.instruction(0x70de0, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x4)?)?;
    c.instruction(0x70de4, branch(C::Cc, 0x70de4, 0x70e4c, false)?)?;
    c.instruction(0x70de8, transfer_word(C::Al, true, r(12), r(11), 12)?)?;
    c.instruction(0x70dec, transfer_word(C::Al, true, r(2), r(11), 8)?)?;
    c.instruction(0x70df0, data_register(C::Al, Op::Cmp, r(0), r(2), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70df4, branch(C::Hi, 0x70df4, 0x70e4c, false)?)?;
    c.instruction(0x70df8, data_register(C::Al, Op::Add, r(3), r(4), r(2), Shift::Lsl, 2, false)?)?;
    c.instruction(0x70dfc, data_immediate(C::Al, Op::Sub, r(7), r(2), 0x1)?)?;
    c.instruction(0x70e00, data_immediate(C::Al, Op::Mvn, r(6), r(0), 0x80000000)?)?;
    c.instruction(0x70e04, data_immediate(C::Al, Op::Add, r(3), r(3), 0x8c)?)?;
    c.instruction(0x70e08, transfer_word(C::Al, true, r(2), r(3), 0)?)?;
    c.instruction(0x70e0c, data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.instruction(0x70e10, data_register(C::Al, Op::Sub, r(4), r(2), r(0), Shift::Lsl, 0, true)?)?;
    c.instruction(0x70e14, data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e18, data_register(C::Al, Op::Mov, r(5), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e1c, data_immediate(C::Mi, Op::Rsb, r(5), r(4), 0x0)?)?;
    c.instruction(0x70e20, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.instruction(0x70e24, data_immediate(C::Mi, Op::Rsb, r(2), r(6), 0x0)?)?;
    c.instruction(0x70e28, data_register(C::Al, Op::Cmp, r(0), r(5), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e2c, data_register(C::Lt, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e30, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.instruction(0x70e34, branch(C::Lt, 0x70e34, 0x70e4c, false)?)?;
    c.instruction(0x70e38, data_register(C::Al, Op::Cmp, r(0), r(5), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e3c, data_immediate(C::Al, Op::Add, r(3), r(3), 0x4)?)?;
    c.instruction(0x70e40, data_register(C::Lt, Op::Mov, r(6), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e44, data_register(C::Al, Op::Cmp, r(0), r(7), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e48, branch(C::Lt, 0x70e48, 0x70e08, false)?)?;
    c.instruction(0x70e4c, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 16, false)?)?;
    c.instruction(0x70e50, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Asr, 16, false)?)?;
    c.instruction(0x70e54, transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x70e58, branch(C::Al, 0x70e58, 0x8187c, true)?)?;
    c.instruction(0x70e5c, transfer_word(C::Al, true, r(6), r(6), 4)?)?;
    c.instruction(0x70e60, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e64, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0x70e68, branch(C::Eq, 0x70e68, 0x70dc4, false)?)?;
    c.instruction(0x70e6c, data_register(C::Al, Op::Sub, r(2), r(8), r(6), Shift::Lsl, 0, true)?)?;
    c.instruction(0x70e70, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0x70e74, data_register(C::Hi, Op::Mov, r(0), r(0), r(2), Shift::Lsl, 8, false)?)?;
    c.instruction(0x70e78, branch_exchange_immediate(0x70e78, 0x83c4c)?)?;
    c.instruction(0x70e7c, branch(C::Al, 0x70e7c, 0x70dc8, false)?)?;
    c.instruction(0x70e80, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x70e84, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.instruction(0x70e88, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70e8c, transfer_word(C::Al, true, r(0), r(5), 12)?)?;
    c.instruction(0x70e90, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x70e94, branch(C::Eq, 0x70e94, 0x70eb8, false)?)?;
    c.instruction(0x70e98, data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb30)?)?;
    c.instruction(0x70e9c, transfer_byte(C::Al, true, r(3), r(5), 722)?)?;
    c.instruction(0x70ea0, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.instruction(0x70ea4, transfer_word_register(C::Al, true, r(12), r(5), r(0), Shift::Lsl, 0)?)?;
    c.instruction(0x70ea8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?)?;
    c.instruction(0x70eac, signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(3), r(0), r(12))?)?;
    c.instruction(0x70eb0, data_immediate(C::Al, Op::Add, r(0), r(0), 0x128)?)?;
    c.instruction(0x70eb4, branch(C::Al, 0x70eb4, 0x70ec0, false)?)?;
    c.instruction(0x70eb8, data_immediate(C::Al, Op::Add, r(0), r(5), 0x2a8)?)?;
    c.instruction(0x70ebc, data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.instruction(0x70ec0, transfer_word(C::Al, true, r(3), r(5), 0)?)?;
    c.instruction(0x70ec4, transfer_word(C::Al, true, r(12), r(5), 740)?)?;
    c.instruction(0x70ec8, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x1000)?)?;
    c.instruction(0x70ecc, branch(C::Ne, 0x70ecc, 0x70ef0, false)?)?;
    c.instruction(0x70ed0, transfer_doubleword(C::Al, true, r(0), r(0), 0)?)?;
    c.instruction(0x70ed4, data_immediate(C::Al, Op::Add, r(2), r(12), 0x1)?)?;
    c.instruction(0x70ed8, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x70edc, data_register(C::Al, Op::Add, r(0), r(0), r(12), Shift::Lsr, 1, true)?)?;
    c.instruction(0x70ee0, data_immediate(C::Al, Op::Adc, r(1), r(1), 0x0)?)?;
    c.instruction(0x70ee4, branch(C::Al, 0x70ee4, 0x8187c, true)?)?;
    c.instruction(0x70ee8, data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.instruction(0x70eec, branch(C::Al, 0x70eec, 0x70f8c, false)?)?;
    c.instruction(0x70ef0, transfer_word(C::Al, true, r(0), r(15), 156)?)?;
    c.instruction(0x70ef4, transfer_word(C::Al, true, r(14), r(1), 8)?)?;
    c.instruction(0x70ef8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x6)?)?;
    c.instruction(0x70efc, multiply_long(C::Al, false, r(3), r(4), r(2), r(0))?)?;
    c.instruction(0x70f00, data_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsr, 2, false)?)?;
    c.instruction(0x70f04, transfer_word(C::Al, true, r(4), r(15), 140)?)?;
    c.instruction(0x70f08, data_register(C::Al, Op::Add, r(0), r(3), r(3), Shift::Lsl, 1, false)?)?;
    c.instruction(0x70f0c, data_register(C::Al, Op::Sub, r(0), r(2), r(0), Shift::Lsl, 1, false)?)?;
    c.instruction(0x70f10, transfer_word_register(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 2)?)?;
    c.instruction(0x70f14, branch(C::Cc, 0x70f14, 0x70f48, false)?)?;
    c.instruction(0x70f18, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.instruction(0x70f1c, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70f20, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.instruction(0x70f24, data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsl, 1, false)?)?;
    c.instruction(0x70f28, data_register(C::Al, Op::Cmp, r(0), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70f2c, branch(C::Lt, 0x70f2c, 0x70f20, false)?)?;
    c.instruction(0x70f30, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.instruction(0x70f34, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.instruction(0x70f38, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.instruction(0x70f3c, data_register(C::Al, Op::Cmp, r(0), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70f40, branch(C::Lt, 0x70f40, 0x70f34, false)?)?;
    c.instruction(0x70f44, branch(C::Al, 0x70f44, 0x70f4c, false)?)?;
    c.instruction(0x70f48, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70f4c, data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, true)?)?;
    c.instruction(0x70f50, multiply_long(C::Al, false, r(1), r(2), r(0), r(14))?)?;
    c.instruction(0x70f54, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0x70f58, data_immediate(C::Al, Op::Adc, r(0), r(0), 0x0)?)?;
    c.instruction(0x70f5c, multiply_accumulate(C::Al, r(3), r(0), r(14), r(2))?)?;
    c.instruction(0x70f60, data_register(C::Al, Op::Mov, r(2), r(0), r(3), Shift::Lsr, 1, true)?)?;
    c.instruction(0x70f64, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x70f68, rotate_right_with_extend(C::Al, r(0), r(1), false)?)?;
    c.instruction(0x70f6c, data_register(C::Al, Op::Add, r(0), r(0), r(12), Shift::Lsr, 1, true)?)?;
    c.instruction(0x70f70, data_immediate(C::Al, Op::Adc, r(1), r(2), 0x0)?)?;
    c.instruction(0x70f74, data_immediate(C::Al, Op::Add, r(2), r(12), 0x1)?)?;
    c.instruction(0x70f78, branch(C::Al, 0x70f78, 0x8187c, true)?)?;
    c.instruction(0x70f7c, transfer_byte(C::Al, true, r(1), r(5), 126)?)?;
    c.instruction(0x70f80, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0x70f84, data_immediate(C::Ne, Op::Mov, r(1), r(0), 0xc)?)?;
    c.instruction(0x70f88, branch(C::Al, 0x70f88, 0x67788, true)?)?;
    c.instruction(0x70f8c, data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.instruction(0x70f90, transfer_multiple(C::Al, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.checked_data(0x70f94, DIVIDE_BY_SIX_MAGIC)?;
    c.checked_data(0x70f98, SIX_ENTRY_LOOKUP)?;
    c.instruction(0x70f9c, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x70fa0, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.instruction(0x70fa4, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70fa8, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.instruction(0x70fac, branch(C::Eq, 0x70fac, 0x70fd4, false)?)?;
    c.instruction(0x70fb0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb30)?)?;
    c.instruction(0x70fb4, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.instruction(0x70fb8, transfer_word_register(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.instruction(0x70fbc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x70fc0, branch(C::Eq, 0x70fc0, 0x70fc8, false)?)?;
    c.instruction(0x70fc4, branch(C::Al, 0x70fc4, 0x2f1c, true)?)?;
    c.instruction(0x70fc8, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70fcc, transfer_multiple(C::Al, true, r(13), 0x4890, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x70fd0, branch(C::Al, 0x70fd0, 0x2f1c, false)?)?;
    c.instruction(0x70fd4, transfer_word(C::Al, true, r(2), r(15), 16)?)?;
    c.instruction(0x70fd8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x244)?)?;
    c.instruction(0x70fdc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.instruction(0x70fe0, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.instruction(0x70fe4, transfer_multiple(C::Al, true, r(13), 0x4890, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x70fe8, branch(C::Al, 0x70fe8, 0x17b0, false)?)?;
    c.checked_data(0x70fec, RELEASE_DIAGNOSTIC_SOURCE)?;
    c.instruction(0x70ff0, transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x70ff4, data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.instruction(0x70ff8, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x188)?)?;
    c.instruction(0x70ffc, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71000, transfer_word(C::Al, true, r(1), r(15), 168)?)?;
    c.instruction(0x71004, data_immediate(C::Al, Op::Sub, r(8), r(11), 0x7c)?)?;
    c.instruction(0x71008, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x7100c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5d)?)?;
    c.instruction(0x71010, data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71014, branch(C::Al, 0x71014, 0x471c, true)?)?;
    c.instruction(0x71018, transfer_word(C::Al, true, r(1), r(15), 148)?)?;
    c.instruction(0x7101c, data_immediate(C::Al, Op::Add, r(9), r(13), 0xc4)?)?;
    c.instruction(0x71020, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5d)?)?;
    c.instruction(0x71024, data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71028, branch(C::Al, 0x71028, 0x471c, true)?)?;
    c.instruction(0x7102c, transfer_word(C::Al, true, r(1), r(15), 132)?)?;
    c.instruction(0x71030, data_immediate(C::Al, Op::Add, r(7), r(13), 0x64)?)?;
    c.instruction(0x71034, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5d)?)?;
    c.instruction(0x71038, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x7103c, branch(C::Al, 0x7103c, 0x471c, true)?)?;
    c.instruction(0x71040, transfer_word(C::Al, true, r(1), r(15), 116)?)?;
    c.instruction(0x71044, data_immediate(C::Al, Op::Add, r(5), r(13), 0x4)?)?;
    c.instruction(0x71048, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5d)?)?;
    c.instruction(0x7104c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71050, branch(C::Al, 0x71050, 0x471c, true)?)?;
    c.instruction(0x71054, transfer_word(C::Al, true, r(0), r(6), 2688)?)?;
    c.instruction(0x71058, transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.instruction(0x7105c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71060, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.instruction(0x71064, data_register(C::Eq, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71068, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0x7106c, data_register(C::Eq, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71070, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.instruction(0x71074, data_register(C::Eq, Op::Mov, r(5), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71078, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.instruction(0x7107c, data_register(C::Eq, Op::Mov, r(5), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x71080, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0x71084, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.instruction(0x71088, data_register(C::Eq, Op::Mov, r(5), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0x7108c, transfer_byte_postindexed(C::Al, true, r(2), r(0), 1)?)?;
    c.instruction(0x71090, transfer_byte_postindexed(C::Al, true, r(3), r(5), 1)?)?;
    c.instruction(0x71094, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.instruction(0x71098, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x5d)?)?;
    c.instruction(0x7109c, data_register(C::Al, Op::Orr, r(2), r(3), r(2), Shift::Lsl, 5, false)?)?;
    c.instruction(0x710a0, transfer_word_postindexed(C::Al, false, r(2), r(4), 4)?)?;
    c.instruction(0x710a4, branch(C::Ne, 0x710a4, 0x7108c, false)?)?;
    c.instruction(0x710a8, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x18)?)?;
    c.instruction(0x710ac, transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    c.checked_data(0x710b0, TABLE_PAIR_FIRST)?;
    c.checked_data(0x710b4, TABLE_PAIR_SECOND)?;
    c.checked_data(0x710b8, TABLE_PAIR_THIRD)?;
    c.checked_data(0x710bc, TABLE_PAIR_FOURTH)?;
    if c.at != END || c.bytes.len() != END - START {
        return Err(format!("history lookup helpers ended at 0x{:x}", c.at));
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn exact_extent_and_checked_pools() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 860);
        let pools = [
            (0x70f94, DIVIDE_BY_SIX_MAGIC), (0x70f98, SIX_ENTRY_LOOKUP),
            (0x70fec, RELEASE_DIAGNOSTIC_SOURCE), (0x710b0, TABLE_PAIR_FIRST),
            (0x710b4, TABLE_PAIR_SECOND), (0x710b8, TABLE_PAIR_THIRD),
            (0x710bc, TABLE_PAIR_FOURTH),
        ];
        assert_eq!(bytes.len() - pools.len() * 4, 832);
        for (at, value) in pools {
            assert_eq!(word(&bytes, at), value);
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.instruction(at, 0).is_err());
            assert!(c.checked_data(at, value ^ 1).is_err());
            c.checked_data(at, value).unwrap();
            assert!(c.checked_data(at, value).is_err());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.checked_data(START, DIVIDE_BY_SIX_MAGIC).is_err());
        assert!(c.instruction(START + 4, 0).is_err());
    }

    #[test]
    fn cold_ratio_carry_and_release_paths() {
        let bytes = fragment().unwrap();
        for (at, target, link) in [
            (0x70e58, 0x8187c, true), (0x70e7c, 0x70dc8, false),
            (0x70fc4, 0x2f1c, true), (0x70fd0, 0x2f1c, false),
            (0x70fe8, 0x17b0, false),
        ] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x70e78), branch_exchange_immediate(0x70e78, 0x83c4c).unwrap());
        assert_eq!(word(&bytes, 0x70f4c), data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, true).unwrap());
        assert_eq!(word(&bytes, 0x70f60), data_register(C::Al, Op::Mov, r(2), r(0), r(3), Shift::Lsr, 1, true).unwrap());
        assert_eq!(word(&bytes, 0x70f68), rotate_right_with_extend(C::Al, r(0), r(1), false).unwrap());
        assert_eq!(word(&bytes, 0x70f70), data_immediate(C::Al, Op::Adc, r(1), r(2), 0).unwrap());
    }

    #[test]
    fn four_93_byte_input_tables_and_93_packed_output_words() {
        let bytes = fragment().unwrap();
        for at in [0x7100c, 0x71020, 0x71034, 0x71048] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Mov, r(2), r(0), 93).unwrap());
            assert_eq!(word(&bytes, at + 8), branch(C::Al, (at + 8) as u32, 0x471c, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x71098), data_immediate(C::Al, Op::Cmp, r(0), r(1), 93).unwrap());
        assert_eq!(word(&bytes, 0x7109c), data_register(C::Al, Op::Orr, r(2), r(3), r(2), Shift::Lsl, 5, false).unwrap());
        assert_eq!(word(&bytes, 0x710a0), transfer_word_postindexed(C::Al, false, r(2), r(4), 4).unwrap());
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
