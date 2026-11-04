use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags, data_register, load_halfword, multiply_accumulate, multiply_register, store_halfword, transfer_byte, transfer_byte_register, transfer_doubleword, transfer_halfword_register, transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6ec04;
pub const END: usize = 0x6f2d0;
const DATA_START: usize = 0x6f2c8;
const FORMAT_TARGET_BITS: u32 = 0x000f30d6;
const NEWLINE_TARGET: u32 = 0x000e2788;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn instruction(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if at != self.at || !(START..DATA_START).contains(&at) || at % 4 != 0 {
            return Err(format!("invalid history/QP instruction address 0x{at:x}"));
        }
        self.bytes.extend(instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn checked_data(&mut self, at: usize, value: u32) -> Result<(), String> {
        let expected = match at {
            DATA_START => FORMAT_TARGET_BITS,
            0x6f2cc => NEWLINE_TARGET,
            _ => return Err(format!("invalid history/QP data address 0x{at:x}")),
        };
        if at != self.at || value != expected {
            return Err(format!("invalid history/QP literal at 0x{at:x}"));
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
    c.instruction(0x6ec04, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.instruction(0x6ec08, data_register(C::Al, Op::Mov, r(14), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec0c, transfer_word_preindexed_writeback(C::Al, false, r(1), r(14), 1952)?)?;
    c.instruction(0x6ec10, transfer_word(C::Al, true, r(10), r(14), -1892)?)?;
    c.instruction(0x6ec14, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x1)?)?;
    c.instruction(0x6ec18, branch(C::Lt, 0x6ec18, 0x6ecd0, false)?)?;
    c.instruction(0x6ec1c, data_immediate(C::Al, Op::Add, r(0), r(9), 0x3ac)?)?;
    c.instruction(0x6ec20, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.instruction(0x6ec24, data_immediate(C::Al, Op::Mvn, r(4), r(0), 0x0)?)?;
    c.instruction(0x6ec28, data_immediate(C::Al, Op::Mvn, r(6), r(0), 0x0)?)?;
    c.instruction(0x6ec2c, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.instruction(0x6ec30, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x6ec34, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.instruction(0x6ec38, data_immediate(C::Al, Op::Add, r(5), r(0), 0x400)?)?;
    c.instruction(0x6ec3c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x39c)?)?;
    c.instruction(0x6ec40, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.instruction(0x6ec44, transfer_halfword_register(C::Al, true, r(9), r(9), r(0))?)?;
    c.instruction(0x6ec48, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec4c, load_halfword(C::Al, r(6), r(5), 0)?)?;
    c.instruction(0x6ec50, data_register(C::Al, Op::Mov, r(7), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec54, data_register(C::Al, Op::Cmp, r(0), r(6), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec58, branch(C::Ls, 0x6ec58, 0x6ec80, false)?)?;
    c.instruction(0x6ec5c, transfer_word(C::Al, true, r(3), r(11), -60)?)?;
    c.instruction(0x6ec60, transfer_word(C::Al, true, r(3), r(3), 1960)?)?;
    c.instruction(0x6ec64, data_register(C::Al, Op::Cmp, r(0), r(7), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec68, branch(C::Ge, 0x6ec68, 0x6efd0, false)?)?;
    c.instruction(0x6ec6c, data_register(C::Al, Op::Cmp, r(0), r(7), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec70, data_immediate(C::Al, Op::Add, r(3), r(7), 0x1)?)?;
    c.instruction(0x6ec74, data_register(C::Ge, Op::Mov, r(12), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec78, data_register(C::Ge, Op::Mov, r(4), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec7c, branch(C::Al, 0x6ec7c, 0x6ecb0, false)?)?;
    c.instruction(0x6ec80, transfer_word(C::Al, true, r(3), r(11), -60)?)?;
    c.instruction(0x6ec84, transfer_word(C::Al, true, r(3), r(3), 1960)?)?;
    c.instruction(0x6ec88, data_register(C::Al, Op::Cmp, r(0), r(7), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ec8c, branch(C::Le, 0x6ec8c, 0x6eca0, false)?)?;
    c.instruction(0x6ec90, transfer_word(C::Al, false, r(2), r(14), 0)?)?;
    c.instruction(0x6ec94, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x6ec98, load_halfword(C::Al, r(6), r(5), 0)?)?;
    c.instruction(0x6ec9c, branch(C::Al, 0x6ec9c, 0x6ecb0, false)?)?;
    c.instruction(0x6eca0, data_register(C::Al, Op::Cmp, r(0), r(7), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eca4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x6eca8, data_immediate(C::Gt, Op::Mvn, r(12), r(0), 0x0)?)?;
    c.instruction(0x6ecac, data_register(C::Gt, Op::Mov, r(4), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ecb0, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ecb4, data_immediate(C::Al, Op::Add, r(5), r(5), 0x2)?)?;
    c.instruction(0x6ecb8, data_register(C::Le, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ecbc, data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.instruction(0x6ecc0, data_register(C::Gt, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ecc4, data_register(C::Al, Op::Cmp, r(0), r(2), r(10), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ecc8, branch(C::Lt, 0x6ecc8, 0x6ec48, false)?)?;
    c.instruction(0x6eccc, branch(C::Al, 0x6eccc, 0x6ecd8, false)?)?;
    c.instruction(0x6ecd0, data_immediate(C::Al, Op::Mvn, r(4), r(0), 0x0)?)?;
    c.instruction(0x6ecd4, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.instruction(0x6ecd8, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.instruction(0x6ecdc, data_register(C::Ne, Op::Mov, r(1), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ece0, data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ece4, transfer_word(C::Al, false, r(1), r(14), 0)?)?;
    c.instruction(0x6ece8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.instruction(0x6ecec, branch(C::Lt, 0x6ecec, 0x6ed10, false)?)?;
    c.instruction(0x6ecf0, transfer_word(C::Al, true, r(5), r(11), -60)?)?;
    c.instruction(0x6ecf4, transfer_word(C::Al, true, r(4), r(13), 68)?)?;
    c.instruction(0x6ecf8, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ecfc, transfer_word_preindexed_writeback(C::Al, true, r(1), r(0), 1956)?)?;
    c.instruction(0x6ed00, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed04, data_immediate(C::Le, Op::Add, r(0), r(5), 0x7a0)?)?;
    c.instruction(0x6ed08, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.instruction(0x6ed0c, branch(C::Al, 0x6ed0c, 0x6ed1c, false)?)?;
    c.instruction(0x6ed10, transfer_word(C::Al, true, r(4), r(13), 68)?)?;
    c.instruction(0x6ed14, transfer_word(C::Al, true, r(5), r(11), -60)?)?;
    c.instruction(0x6ed18, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.instruction(0x6ed1c, transfer_word(C::Al, true, r(7), r(13), 60)?)?;
    c.instruction(0x6ed20, transfer_word(C::Al, false, r(0), r(5), 1952)?)?;
    c.instruction(0x6ed24, transfer_word(C::Al, true, r(6), r(11), 8)?)?;
    c.instruction(0x6ed28, transfer_byte(C::Al, true, r(0), r(5), 48)?)?;
    c.instruction(0x6ed2c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6ed30, branch(C::Eq, 0x6ed30, 0x6ed44, false)?)?;
    c.instruction(0x6ed34, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed38, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed3c, data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed40, branch(C::Al, 0x6ed40, 0x6b8f4, true)?)?;
    c.instruction(0x6ed44, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed48, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed4c, data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed50, branch(C::Al, 0x6ed50, 0x70468, true)?)?;
    c.instruction(0x6ed54, transfer_byte(C::Al, true, r(0), r(5), 48)?)?;
    c.instruction(0x6ed58, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6ed5c, branch(C::Eq, 0x6ed5c, 0x6ed70, false)?)?;
    c.instruction(0x6ed60, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed64, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed68, data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ed6c, branch(C::Al, 0x6ed6c, 0x6b8f4, true)?)?;
    c.instruction(0x6ed70, transfer_byte(C::Al, true, r(0), r(6), 2936)?)?;
    c.instruction(0x6ed74, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6ed78, branch(C::Eq, 0x6ed78, 0x6ee00, false)?)?;
    c.instruction(0x6ed7c, transfer_word(C::Al, true, r(1), r(6), 2732)?)?;
    c.instruction(0x6ed80, transfer_byte(C::Al, true, r(0), r(6), 2937)?)?;
    c.instruction(0x6ed84, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0x6ed88, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x10)?)?;
    c.instruction(0x6ed8c, transfer_byte(C::Al, false, r(0), r(4), 1129)?)?;
    c.instruction(0x6ed90, transfer_word(C::Al, true, r(1), r(6), 2744)?)?;
    c.instruction(0x6ed94, transfer_byte(C::Al, true, r(1), r(1), 224)?)?;
    c.instruction(0x6ed98, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.instruction(0x6ed9c, branch(C::Ne, 0x6ed9c, 0x6edd0, false)?)?;
    c.instruction(0x6eda0, transfer_word(C::Al, true, r(0), r(6), 716)?)?;
    c.instruction(0x6eda4, transfer_word(C::Al, true, r(1), r(15), 2144)?)?;
    c.instruction(0x6eda8, transfer_word(C::Al, true, r(2), r(15), 2144)?)?;
    c.instruction(0x6edac, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x500)?)?;
    c.instruction(0x6edb0, transfer_byte(C::Al, true, r(0), r(6), 1173)?)?;
    c.instruction(0x6edb4, data_register(C::Hi, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6edb8, transfer_word(C::Al, true, r(1), r(4), 44)?)?;
    c.instruction(0x6edbc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6edc0, data_immediate(C::Eq, Op::Add, r(1), r(1), 0x4)?)?;
    c.instruction(0x6edc4, data_immediate(C::Al, Op::And, r(0), r(1), 0x7)?)?;
    c.instruction(0x6edc8, transfer_byte_register(C::Al, true, r(0), r(2), r(0), Shift::Lsl, 0)?)?;
    c.instruction(0x6edcc, transfer_byte(C::Al, false, r(0), r(4), 1129)?)?;
    c.instruction(0x6edd0, transfer_word(C::Al, true, r(1), r(15), 2108)?)?;
    c.instruction(0x6edd4, data_immediate(C::Al, Op::And, r(2), r(0), 0xff)?)?;
    c.instruction(0x6edd8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.instruction(0x6eddc, branch(C::Al, 0x6eddc, 0x2ec94, true)?)?;
    c.instruction(0x6ede0, transfer_word(C::Al, true, r(1), r(15), 2048)?)?;
    c.instruction(0x6ede4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.instruction(0x6ede8, branch(C::Al, 0x6ede8, 0x2ec94, true)?)?;
    c.instruction(0x6edec, transfer_word(C::Al, true, r(0), r(4), 44)?)?;
    c.instruction(0x6edf0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.instruction(0x6edf4, data_register(C::Al, Op::Bic, r(0), r(1), r(0), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6edf8, data_immediate(C::Al, Op::Add, r(0), r(0), 0xff)?)?;
    c.instruction(0x6edfc, transfer_byte(C::Al, false, r(0), r(4), 1128)?)?;
    c.instruction(0x6ee00, transfer_byte(C::Al, true, r(1), r(5), 31)?)?;
    c.instruction(0x6ee04, transfer_byte(C::Al, true, r(0), r(7), 533)?)?;
    c.instruction(0x6ee08, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.instruction(0x6ee0c, branch(C::Eq, 0x6ee0c, 0x6ee58, false)?)?;
    c.instruction(0x6ee10, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6ee14, branch(C::Ne, 0x6ee14, 0x6ee60, false)?)?;
    c.instruction(0x6ee18, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.instruction(0x6ee1c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.instruction(0x6ee20, branch(C::Eq, 0x6ee20, 0x6efe0, false)?)?;
    c.instruction(0x6ee24, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.instruction(0x6ee28, branch(C::Ne, 0x6ee28, 0x6f100, false)?)?;
    c.instruction(0x6ee2c, transfer_byte(C::Al, true, r(0), r(4), 12)?)?;
    c.instruction(0x6ee30, transfer_byte(C::Al, false, r(0), r(4), 238)?)?;
    c.instruction(0x6ee34, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6ee38, data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ee3c, transfer_byte(C::Al, false, r(1), r(4), 239)?)?;
    c.instruction(0x6ee40, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6ee44, data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6ee48, transfer_byte(C::Al, false, r(1), r(4), 240)?)?;
    c.instruction(0x6ee4c, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6ee50, data_register(C::Al, Op::Sub, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ee54, branch(C::Al, 0x6ee54, 0x6f0fc, false)?)?;
    c.instruction(0x6ee58, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6ee5c, branch(C::Eq, 0x6ee5c, 0x6efa0, false)?)?;
    c.instruction(0x6ee60, transfer_word(C::Al, true, r(0), r(5), 728)?)?;
    c.instruction(0x6ee64, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.instruction(0x6ee68, branch(C::Eq, 0x6ee68, 0x6ee84, false)?)?;
    c.instruction(0x6ee6c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.instruction(0x6ee70, branch(C::Ne, 0x6ee70, 0x6ee94, false)?)?;
    c.instruction(0x6ee74, data_immediate(C::Al, Op::Add, r(0), r(5), 0x1f)?)?;
    c.instruction(0x6ee78, data_immediate(C::Al, Op::Add, r(2), r(5), 0xd20)?)?;
    c.instruction(0x6ee7c, data_immediate(C::Al, Op::Add, r(1), r(0), 0xd00)?)?;
    c.instruction(0x6ee80, branch(C::Al, 0x6ee80, 0x6eea0, false)?)?;
    c.instruction(0x6ee84, data_immediate(C::Al, Op::Add, r(0), r(5), 0xaf)?)?;
    c.instruction(0x6ee88, data_immediate(C::Al, Op::Add, r(2), r(5), 0xfb0)?)?;
    c.instruction(0x6ee8c, data_immediate(C::Al, Op::Add, r(1), r(0), 0xf00)?)?;
    c.instruction(0x6ee90, branch(C::Al, 0x6ee90, 0x6eea0, false)?)?;
    c.instruction(0x6ee94, data_immediate(C::Al, Op::Add, r(0), r(5), 0x8f)?)?;
    c.instruction(0x6ee98, data_immediate(C::Al, Op::Add, r(2), r(5), 0xa90)?)?;
    c.instruction(0x6ee9c, data_immediate(C::Al, Op::Add, r(1), r(0), 0xa00)?)?;
    c.instruction(0x6eea0, transfer_word(C::Al, true, r(3), r(6), 2744)?)?;
    c.instruction(0x6eea4, transfer_byte(C::Al, true, r(0), r(4), 12)?)?;
    c.instruction(0x6eea8, transfer_byte(C::Al, true, r(1), r(1), 0)?)?;
    c.instruction(0x6eeac, transfer_byte(C::Al, true, r(2), r(2), 0)?)?;
    c.instruction(0x6eeb0, transfer_byte(C::Al, true, r(7), r(3), 221)?)?;
    c.instruction(0x6eeb4, transfer_byte(C::Al, true, r(3), r(3), 222)?)?;
    c.instruction(0x6eeb8, data_register(C::Al, Op::Sub, r(3), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eebc, data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eec0, data_register(C::Gt, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eec4, data_register(C::Al, Op::Add, r(3), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eec8, data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eecc, data_register(C::Cc, Op::Mov, r(2), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eed0, transfer_byte(C::Al, true, r(3), r(6), 1028)?)?;
    c.instruction(0x6eed4, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x14)?)?;
    c.instruction(0x6eed8, data_immediate(C::Ne, Op::Cmp, r(0), r(3), 0x12)?)?;
    c.instruction(0x6eedc, branch(C::Ne, 0x6eedc, 0x6ef00, false)?)?;
    c.instruction(0x6eee0, data_register(C::Al, Op::Cmp, r(0), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eee4, branch(C::Ne, 0x6eee4, 0x6ef00, false)?)?;
    c.instruction(0x6eee8, transfer_word(C::Al, true, r(3), r(4), 32)?)?;
    c.instruction(0x6eeec, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.instruction(0x6eef0, branch(C::Eq, 0x6eef0, 0x6ef00, false)?)?;
    c.instruction(0x6eef4, transfer_word(C::Al, true, r(3), r(6), 644)?)?;
    c.instruction(0x6eef8, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x2)?)?;
    c.instruction(0x6eefc, data_immediate(C::Eq, Op::Add, r(1), r(1), 0x1)?)?;
    c.instruction(0x6ef00, data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef04, branch(C::Eq, 0x6ef04, 0x6ef88, false)?)?;
    c.instruction(0x6ef08, transfer_word(C::Al, true, r(3), r(5), 704)?)?;
    c.instruction(0x6ef0c, transfer_word(C::Al, true, r(7), r(5), 708)?)?;
    c.instruction(0x6ef10, data_immediate_with_flags(C::Al, Op::Sub, r(3), r(3), 0x1, true)?)?;
    c.instruction(0x6ef14, data_immediate_with_flags(C::Al, Op::Sbc, r(3), r(7), 0x0, true)?)?;
    c.instruction(0x6ef18, branch(C::Lt, 0x6ef18, 0x6ef88, false)?)?;
    c.instruction(0x6ef1c, data_immediate(C::Al, Op::Sub, r(7), r(0), 0x4)?)?;
    c.instruction(0x6ef20, data_register(C::Al, Op::Cmp, r(0), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef24, data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef28, data_register(C::Gt, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef2c, data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef30, data_immediate(C::Al, Op::Add, r(7), r(0), 0x2)?)?;
    c.instruction(0x6ef34, data_register(C::Lt, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef38, transfer_byte(C::Al, false, r(3), r(4), 238)?)?;
    c.instruction(0x6ef3c, data_immediate(C::Al, Op::Sub, r(3), r(0), 0x1)?)?;
    c.instruction(0x6ef40, data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef44, data_register(C::Gt, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef48, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef4c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x5)?)?;
    c.instruction(0x6ef50, data_register(C::Ge, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef54, data_register(C::Al, Op::Cmp, r(0), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef58, transfer_byte(C::Al, false, r(3), r(4), 239)?)?;
    c.instruction(0x6ef5c, data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef60, data_register(C::Gt, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef64, data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef68, data_register(C::Lt, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef6c, data_register(C::Al, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef70, data_register(C::Le, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef74, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef78, transfer_byte(C::Al, false, r(3), r(4), 240)?)?;
    c.instruction(0x6ef7c, data_register(C::Lt, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6ef80, transfer_byte(C::Al, false, r(2), r(4), 241)?)?;
    c.instruction(0x6ef84, branch(C::Al, 0x6ef84, 0x6f100, false)?)?;
    c.instruction(0x6ef88, transfer_word(C::Al, true, r(1), r(15), 1672)?)?;
    c.instruction(0x6ef8c, multiply_register(C::Al, r(2), r(0), r(1))?)?;
    c.instruction(0x6ef90, data_register(C::Al, Op::Mov, r(0), r(0), r(2), Shift::Lsr, 16, false)?)?;
    c.instruction(0x6ef94, store_halfword(C::Al, r(2), r(4), 238)?)?;
    c.instruction(0x6ef98, store_halfword(C::Al, r(0), r(4), 240)?)?;
    c.instruction(0x6ef9c, branch(C::Al, 0x6ef9c, 0x6f100, false)?)?;
    c.instruction(0x6efa0, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.instruction(0x6efa4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.instruction(0x6efa8, branch(C::Ne, 0x6efa8, 0x6f048, false)?)?;
    c.instruction(0x6efac, transfer_word(C::Al, true, r(2), r(6), 2744)?)?;
    c.instruction(0x6efb0, transfer_byte(C::Al, true, r(1), r(4), 12)?)?;
    c.instruction(0x6efb4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x6efb8, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x7f)?)?;
    c.instruction(0x6efbc, transfer_byte(C::Al, true, r(0), r(2), 221)?)?;
    c.instruction(0x6efc0, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6efc4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x7f)?)?;
    c.instruction(0x6efc8, data_immediate(C::Cc, Op::Mov, r(3), r(0), 0x1)?)?;
    c.instruction(0x6efcc, branch(C::Al, 0x6efcc, 0x6f084, false)?)?;
    c.instruction(0x6efd0, data_immediate(C::Al, Op::Cmn, r(0), r(2), 0x1)?)?;
    c.instruction(0x6efd4, transfer_word(C::Al, false, r(2), r(14), 0)?)?;
    c.instruction(0x6efd8, branch(C::Ne, 0x6efd8, 0x6ece8, false)?)?;
    c.instruction(0x6efdc, branch(C::Al, 0x6efdc, 0x6ecd8, false)?)?;
    c.instruction(0x6efe0, transfer_byte(C::Al, true, r(0), r(4), 12)?)?;
    c.instruction(0x6efe4, transfer_byte(C::Al, false, r(0), r(4), 238)?)?;
    c.instruction(0x6efe8, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6efec, data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6eff0, transfer_byte(C::Al, false, r(1), r(4), 239)?)?;
    c.instruction(0x6eff4, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6eff8, data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6effc, transfer_byte(C::Al, false, r(1), r(4), 240)?)?;
    c.instruction(0x6f000, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6f004, data_register(C::Al, Op::Add, r(1), r(1), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6f008, data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f00c, transfer_byte(C::Al, false, r(1), r(4), 241)?)?;
    c.instruction(0x6f010, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6f014, data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsl, 2, false)?)?;
    c.instruction(0x6f018, transfer_byte(C::Al, false, r(1), r(4), 242)?)?;
    c.instruction(0x6f01c, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6f020, data_register(C::Al, Op::Sub, r(1), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f024, transfer_byte(C::Al, false, r(1), r(4), 243)?)?;
    c.instruction(0x6f028, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6f02c, data_register(C::Al, Op::Sub, r(1), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6f030, transfer_byte(C::Al, false, r(1), r(4), 244)?)?;
    c.instruction(0x6f034, transfer_byte(C::Al, true, r(1), r(5), 46)?)?;
    c.instruction(0x6f038, data_register(C::Al, Op::Sub, r(1), r(1), r(1), Shift::Lsl, 2, false)?)?;
    c.instruction(0x6f03c, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f040, transfer_byte(C::Al, false, r(0), r(4), 245)?)?;
    c.instruction(0x6f044, branch(C::Al, 0x6f044, 0x6f100, false)?)?;
    c.instruction(0x6f048, transfer_word(C::Al, true, r(2), r(6), 2744)?)?;
    c.instruction(0x6f04c, transfer_byte(C::Al, true, r(1), r(4), 12)?)?;
    c.instruction(0x6f050, transfer_byte(C::Al, true, r(6), r(6), 988)?)?;
    c.instruction(0x6f054, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.instruction(0x6f058, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.instruction(0x6f05c, transfer_byte(C::Al, true, r(0), r(2), 221)?)?;
    c.instruction(0x6f060, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f064, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x33)?)?;
    c.instruction(0x6f068, data_immediate(C::Cc, Op::Mov, r(7), r(0), 0x1)?)?;
    c.instruction(0x6f06c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3f)?)?;
    c.instruction(0x6f070, data_immediate(C::Cc, Op::Mov, r(3), r(0), 0x1)?)?;
    c.instruction(0x6f074, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.instruction(0x6f078, data_register(C::Eq, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f07c, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x3f)?)?;
    c.instruction(0x6f080, data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x33)?)?;
    c.instruction(0x6f084, transfer_byte(C::Al, true, r(2), r(2), 222)?)?;
    c.instruction(0x6f088, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.instruction(0x6f08c, data_register(C::Eq, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f090, data_immediate(C::Al, Op::Sub, r(7), r(1), 0x4)?)?;
    c.instruction(0x6f094, data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f098, data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f09c, data_register(C::Gt, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0a0, data_register(C::Al, Op::Sub, r(2), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0a4, data_register(C::Al, Op::Bic, r(2), r(2), r(2), Shift::Asr, 31, false)?)?;
    c.instruction(0x6f0a8, data_register(C::Al, Op::Cmp, r(0), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0ac, data_immediate(C::Al, Op::Add, r(7), r(1), 0x2)?)?;
    c.instruction(0x6f0b0, data_register(C::Lt, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0b4, transfer_byte(C::Al, false, r(3), r(4), 238)?)?;
    c.instruction(0x6f0b8, data_immediate(C::Al, Op::Sub, r(3), r(1), 0x1)?)?;
    c.instruction(0x6f0bc, data_register(C::Al, Op::Cmp, r(0), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0c0, data_register(C::Gt, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0c4, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0c8, data_immediate(C::Al, Op::Add, r(1), r(1), 0x5)?)?;
    c.instruction(0x6f0cc, data_register(C::Ge, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0d0, data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0d4, transfer_byte(C::Al, false, r(3), r(4), 239)?)?;
    c.instruction(0x6f0d8, data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0dc, data_register(C::Gt, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0e0, data_register(C::Al, Op::Cmp, r(0), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0e4, data_register(C::Lt, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0e8, data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0ec, data_register(C::Le, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0f0, data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0f4, transfer_byte(C::Al, false, r(3), r(4), 240)?)?;
    c.instruction(0x6f0f8, data_register(C::Lt, Op::Mov, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f0fc, transfer_byte(C::Al, false, r(0), r(4), 241)?)?;
    c.instruction(0x6f100, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.instruction(0x6f104, transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x6f108, transfer_word(C::Al, true, r(12), r(11), -64)?)?;
    c.instruction(0x6f10c, data_register(C::Al, Op::Mov, r(6), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f110, data_register(C::Al, Op::Mov, r(9), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f114, branch(C::Al, 0x6f114, 0x6e638, false)?)?;
    c.instruction(0x6f118, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6)?)?;
    c.instruction(0x6f11c, store_halfword(C::Al, r(1), r(12), 0)?)?;
    c.instruction(0x6f120, data_register(C::Al, Op::Cmp, r(0), r(6), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f124, transfer_word(C::Al, true, r(12), r(11), -64)?)?;
    c.instruction(0x6f128, data_register(C::Ls, Op::Mov, r(6), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f12c, transfer_word(C::Al, true, r(9), r(11), -60)?)?;
    c.instruction(0x6f130, branch(C::Al, 0x6f130, 0x6e638, false)?)?;
    c.instruction(0x6f134, transfer_word(C::Al, true, r(1), r(9), 96)?)?;
    c.instruction(0x6f138, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2c)?)?;
    c.instruction(0x6f13c, branch(C::Cc, 0x6f13c, 0x6f1bc, false)?)?;
    c.instruction(0x6f140, data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6f144, data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f148, branch(C::Ls, 0x6f148, 0x6e288, false)?)?;
    c.instruction(0x6f14c, transfer_doubleword(C::Al, true, r(2), r(10), 0)?)?;
    c.instruction(0x6f150, data_immediate_with_flags(C::Al, Op::Sub, r(1), r(2), 0x1, true)?)?;
    c.instruction(0x6f154, data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(3), 0x0, true)?)?;
    c.instruction(0x6f158, data_register(C::Ge, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f15c, branch(C::Al, 0x6f15c, 0x6e288, false)?)?;
    c.instruction(0x6f160, data_immediate(C::Al, Op::Add, r(0), r(9), 0x308)?)?;
    c.instruction(0x6f164, branch(C::Al, 0x6f164, 0x6f16c, false)?)?;
    c.instruction(0x6f168, data_immediate(C::Al, Op::Add, r(0), r(9), 0x2f8)?)?;
    c.instruction(0x6f16c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.instruction(0x6f170, transfer_byte(C::Al, true, r(1), r(9), 720)?)?;
    c.instruction(0x6f174, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x15)?)?;
    c.instruction(0x6f178, branch(C::Hi, 0x6f178, 0x6f19c, false)?)?;
    c.instruction(0x6f17c, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.instruction(0x6f180, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2e0)?)?;
    c.instruction(0x6f184, branch(C::Cc, 0x6f184, 0x6f19c, false)?)?;
    c.instruction(0x6f188, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3f4)?)?;
    c.instruction(0x6f18c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x278)?)?;
    c.instruction(0x6f190, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xfc00)?)?;
    c.instruction(0x6f194, transfer_halfword_register(C::Al, false, r(1), r(10), r(0))?)?;
    c.instruction(0x6f198, branch(C::Al, 0x6f198, 0x6f1ac, false)?)?;
    c.instruction(0x6f19c, transfer_word(C::Al, true, r(0), r(9), 648)?)?;
    c.instruction(0x6f1a0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x278)?)?;
    c.instruction(0x6f1a4, transfer_halfword_register(C::Al, false, r(0), r(10), r(1))?)?;
    c.instruction(0x6f1a8, transfer_word(C::Al, true, r(1), r(9), 652)?)?;
    c.instruction(0x6f1ac, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x7a)?)?;
    c.instruction(0x6f1b0, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x200)?)?;
    c.instruction(0x6f1b4, transfer_halfword_register(C::Al, false, r(1), r(10), r(0))?)?;
    c.instruction(0x6f1b8, branch(C::Al, 0x6f1b8, 0x6e8bc, false)?)?;
    c.instruction(0x6f1bc, data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6f1c0, branch(C::Ls, 0x6f1c0, 0x6e288, false)?)?;
    c.instruction(0x6f1c4, transfer_multiple(C::Al, true, r(10), 0x9, BlockMode::IncrementAfter, false)?)?;
    c.instruction(0x6f1c8, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 0x1, true)?)?;
    c.instruction(0x6f1cc, data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(3), 0x0, true)?)?;
    c.instruction(0x6f1d0, data_register(C::Ge, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.instruction(0x6f1d4, branch(C::Al, 0x6f1d4, 0x6e288, false)?)?;
    c.instruction(0x6f1d8, transfer_word(C::Al, true, r(4), r(11), -60)?)?;
    c.instruction(0x6f1dc, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.instruction(0x6f1e0, transfer_word(C::Ne, true, r(0), r(4), 1592)?)?;
    c.instruction(0x6f1e4, transfer_word(C::Ne, false, r(0), r(4), 1588)?)?;
    c.instruction(0x6f1e8, transfer_word(C::Ne, true, r(0), r(4), 1604)?)?;
    c.instruction(0x6f1ec, transfer_word(C::Ne, false, r(0), r(4), 1600)?)?;
    c.instruction(0x6f1f0, branch(C::Al, 0x6f1f0, 0x6f2d0, false)?)?;
    c.instruction(0x6f1f4, transfer_word(C::Al, true, r(0), r(4), 352)?)?;
    c.instruction(0x6f1f8, transfer_word(C::Al, true, r(7), r(11), -68)?)?;
    c.instruction(0x6f1fc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6f200, transfer_word(C::Ne, true, r(5), r(4), 360)?)?;
    c.instruction(0x6f204, data_immediate(C::Ne, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.instruction(0x6f208, branch(C::Eq, 0x6f208, 0x6f2d0, false)?)?;
    c.instruction(0x6f20c, multiply_register(C::Al, r(2), r(0), r(0))?)?;
    c.instruction(0x6f210, transfer_byte(C::Al, true, r(7), r(4), 161)?)?;
    c.instruction(0x6f214, multiply_accumulate(C::Al, r(1), r(5), r(5), r(2))?)?;
    c.instruction(0x6f218, transfer_byte(C::Al, true, r(2), r(4), 159)?)?;
    c.instruction(0x6f21c, transfer_word(C::Al, false, r(12), r(11), -72)?)?;
    c.instruction(0x6f220, transfer_word(C::Al, false, r(7), r(13), 52)?)?;
    c.instruction(0x6f224, multiply_register(C::Al, r(3), r(0), r(2))?)?;
    c.instruction(0x6f228, multiply_accumulate(C::Al, r(0), r(5), r(7), r(3))?)?;
    c.instruction(0x6f22c, data_register(C::Al, Op::Mov, r(7), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f230, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 10, false)?)?;
    c.instruction(0x6f234, branch_exchange_immediate(0x6f234, 0x83c4c)?)?;
    c.instruction(0x6f238, data_register(C::Al, Op::Mov, r(14), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x6f23c, transfer_word(C::Al, true, r(7), r(11), -68)?)?;
    c.instruction(0x6f240, transfer_word(C::Al, true, r(12), r(11), -72)?)?;
    c.instruction(0x6f244, transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.instruction(0x6f248, transfer_word(C::Al, false, r(0), r(4), 1592)?)?;
    c.instruction(0x6f24c, multiply_register(C::Al, r(1), r(0), r(5))?)?;
    c.instruction(0x6f250, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 4, false)?)?;
    c.instruction(0x6f254, transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.instruction(0x6f258, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 6, false)?)?;
    c.instruction(0x6f25c, transfer_word(C::Al, false, r(0), r(4), 1604)?)?;
    c.instruction(0x6f260, branch(C::Al, 0x6f260, 0x6f2d0, false)?)?;
    c.instruction(0x6f264, transfer_word(C::Al, true, r(0), r(4), 352)?)?;
    c.instruction(0x6f268, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.instruction(0x6f26c, branch(C::Eq, 0x6f26c, 0x6f2d0, false)?)?;
    c.instruction(0x6f270, multiply_register(C::Al, r(2), r(1), r(1))?)?;
    c.instruction(0x6f274, transfer_word(C::Al, false, r(1), r(11), -68)?)?;
    c.instruction(0x6f278, multiply_accumulate(C::Al, r(5), r(0), r(0), r(2))?)?;
    c.instruction(0x6f27c, transfer_word(C::Al, false, r(5), r(11), -72)?)?;
    c.instruction(0x6f280, transfer_byte(C::Al, true, r(2), r(4), 159)?)?;
    c.instruction(0x6f284, transfer_byte(C::Al, true, r(5), r(4), 161)?)?;
    c.instruction(0x6f288, transfer_word(C::Al, false, r(14), r(13), 52)?)?;
    c.instruction(0x6f28c, multiply_register(C::Al, r(3), r(0), r(2))?)?;
    c.instruction(0x6f290, multiply_accumulate(C::Al, r(0), r(1), r(5), r(3))?)?;
    c.instruction(0x6f294, transfer_word(C::Al, true, r(1), r(11), -72)?)?;
    c.instruction(0x6f298, transfer_word(C::Al, false, r(12), r(11), -72)?)?;
    c.instruction(0x6f29c, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 10, false)?)?;
    c.instruction(0x6f2a0, branch_exchange_immediate(0x6f2a0, 0x83c4c)?)?;
    c.instruction(0x6f2a4, transfer_word(C::Al, true, r(14), r(13), 52)?)?;
    c.instruction(0x6f2a8, transfer_word(C::Al, true, r(12), r(11), -72)?)?;
    c.instruction(0x6f2ac, transfer_word(C::Al, true, r(3), r(11), -64)?)?;
    c.instruction(0x6f2b0, transfer_word(C::Al, false, r(0), r(4), 1596)?)?;
    c.instruction(0x6f2b4, transfer_word(C::Al, true, r(2), r(11), -68)?)?;
    c.instruction(0x6f2b8, multiply_register(C::Al, r(1), r(0), r(2))?)?;
    c.instruction(0x6f2bc, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 4, false)?)?;
    c.instruction(0x6f2c0, data_register(C::Al, Op::Add, r(0), r(0), r(5), Shift::Lsl, 6, false)?)?;
    c.instruction(0x6f2c4, branch(C::Al, 0x6f2c4, 0x6e018, false)?)?;
    c.checked_data(DATA_START, FORMAT_TARGET_BITS)?;
    c.checked_data(0x6f2cc, NEWLINE_TARGET)?;
    if c.at != END || c.bytes.len() != END - START {
        return Err(format!("history/QP candidates ended at 0x{:x}", c.at));
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
    fn exact_extent_and_separate_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1740);
        assert_eq!(DATA_START - START, 1732);
        assert_eq!(END - DATA_START, 8);
        assert_eq!(word(&bytes, DATA_START), FORMAT_TARGET_BITS);
        assert_eq!(word(&bytes, 0x6f2cc), NEWLINE_TARGET);
        let mut c = Code { at: DATA_START, bytes: Vec::new() };
        assert!(c.instruction(DATA_START, 0).is_err());
        assert!(c.checked_data(DATA_START, NEWLINE_TARGET).is_err());
        assert!(c.checked_data(START, FORMAT_TARGET_BITS).is_err());
        c.checked_data(DATA_START, FORMAT_TARGET_BITS).unwrap();
        assert!(c.checked_data(DATA_START, FORMAT_TARGET_BITS).is_err());
        c.checked_data(0x6f2cc, NEWLINE_TARGET).unwrap();
        assert_eq!(c.at, END);
    }

    #[test]
    fn history_candidates_and_external_fallback() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6ec0c), transfer_word_preindexed_writeback(C::Al, false, r(1), r(14), 0x7a0).unwrap());
        assert_eq!(word(&bytes, 0x6ec44), transfer_halfword_register(C::Al, true, r(9), r(9), r(0)).unwrap());
        assert_eq!(word(&bytes, 0x6ec4c), load_halfword(C::Al, r(6), r(5), 0).unwrap());
        for (at, target) in [(0x6ed40, 0x6b8f4), (0x6ed50, 0x70468), (0x6ed6c, 0x6b8f4)] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, true).unwrap());
        }
        for (at, offset) in [(0x6efe4, 0xee), (0x6eff0, 0xef), (0x6effc, 0xf0), (0x6f00c, 0xf1), (0x6f018, 0xf2), (0x6f024, 0xf3), (0x6f030, 0xf4), (0x6f040, 0xf5)] {
            let source = if matches!(offset, 0xee | 0xf5) { r(0) } else { r(1) };
            assert_eq!(word(&bytes, at), transfer_byte(C::Al, false, source, r(4), offset).unwrap());
        }
        let fallback_pool = 0x6ef88 + 8 + 1672;
        assert!(fallback_pool >= END);
        assert_eq!(word(&bytes, 0x6ef88), transfer_word(C::Al, true, r(1), R::PC, 1672).unwrap());
        assert_eq!(word(&bytes, 0x6ef94), store_halfword(C::Al, r(2), r(4), 0xee).unwrap());
        assert_eq!(word(&bytes, 0x6ef98), store_halfword(C::Al, r(0), r(4), 0xf0).unwrap());
    }

    #[test]
    fn carry_chains_return_and_cold_code() {
        let bytes = fragment().unwrap();
        for (at, low, high, dest) in [(0x6ef10, 3, 7, 3), (0x6f150, 2, 3, 1), (0x6f1c8, 0, 3, 0)] {
            assert_eq!(word(&bytes, at), data_immediate_with_flags(C::Al, Op::Sub, r(dest), r(low), 1, true).unwrap());
            assert_eq!(word(&bytes, at + 4), data_immediate_with_flags(C::Al, Op::Sbc, r(dest), r(high), 0, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x6f100), data_immediate(C::Al, Op::Sub, R::SP, r(11), 28).unwrap());
        assert_eq!(word(&bytes, 0x6f104), transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x6f108), transfer_word(C::Al, true, r(12), r(11), -64).unwrap());
        for at in [0x6f234, 0x6f2a0] {
            assert_eq!(word(&bytes, at), branch_exchange_immediate(at as u32, 0x83c4c).unwrap());
        }
        assert_eq!(word(&bytes, 0x6f2c4), branch(C::Al, 0x6f2c4, 0x6e018, false).unwrap());
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
        assert_eq!(u32::from_le_bytes(reference[0x6f618 + 0x4000..0x6f61c + 0x4000].try_into().unwrap()), 0x01010101);
    }
}
