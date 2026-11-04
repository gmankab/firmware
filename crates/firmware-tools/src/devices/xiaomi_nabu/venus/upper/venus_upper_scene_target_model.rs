use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register, multiply_accumulate,
    multiply_long, multiply_register, transfer_byte, transfer_byte_preindexed_writeback,
    transfer_doubleword, transfer_multiple, transfer_word, unsigned_multiply_long_accumulate,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6f2d0;
pub const END: usize = 0x6f61c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 788;
pub const DATA_BYTES: usize = 56;
const POOL_START: usize = 0x6f5e4;
const RATE_CONTROL_SOURCE: u32 = 0xf2e4c;
const NEWLINE: u32 = 0xe2788;
const SCENE_CHANGED: u32 = 0xf30f2;
const INTERNAL_PSNR: u32 = 0xf2e89;
const MODEL_PARAMETERS: u32 = 0xf3113;
const TARGET_PSNR_QP: u32 = 0xf3148;
const FIXED_POINT_LIMIT_MINUS_ONE: u32 = 0x007ffffe;
const FIXED_POINT_LIMIT: u32 = 0x007fffff;
const QUALITY_MODEL_TABLE: u32 = 0x10e084;
const QUANTIZER_MODEL_TABLE: u32 = 0x10e304;
const QUANTIZER_ASCENDING_TABLE: u32 = 0xf32c4;
const QUANTIZER_DESCENDING_TABLE: u32 = 0xf32cc;
const QUANTIZER_DIAGNOSTIC: u32 = 0xf2fa1;
const BYTE_LANE_ONES: u32 = 0x01010101;
const TRACE: u32 = 0x2ec94;
const DIVIDE_U64: u32 = 0x8187c;
const COMMIT_TARGET: u32 = 0x6d6a4;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct Code { at: usize, bytes: Vec<u8> }

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("scene target address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || (self.at >= POOL_START) != data {
            return Err(format!("scene target output kind or address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> { self.word(instruction, false) }
    fn data(&mut self, value: u32) -> Result<(), String> { self.word(value, true) }
    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
}
fn scene_change_and_fixed_point_psnr(c: &mut Code) -> Result<(), String> {
    c.check(0x6f2d0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x64c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x6f370, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x2f4)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(11), -0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(11), -0x48)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x2d8)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(transfer_word(C::Al, true, r(14), r(4), 0x2d8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(4), r(14), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x634)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x640)?)?;
    c.emit(multiply_register(C::Al, r(2), r(1), r(5))?)?;
    c.emit(data_register(C::Al, Op::Rsb, r(0), r(2), r(0), Shift::Lsl, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsr, 10, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x9d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x6f364, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x2a8)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0x94)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x28c)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(transfer_word(C::Al, true, r(14), r(4), 0x2d8)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(11), -0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(11), -0x44)?)?;
    c.branch(C::Al, 0x6f388, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(4), r(14), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x634)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x640)?)?;
    c.emit(multiply_register(C::Al, r(2), r(1), r(3))?)?;
    c.emit(data_register(C::Al, Op::Rsb, r(0), r(2), r(0), Shift::Lsl, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsr, 10, false)?)?;
    Ok(())
}

fn clamp_target_qp_and_trace(c: &mut Code) -> Result<(), String> {
    c.check(0x6f388)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 0x64c)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(0), r(5), 0xff, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(4), 0x98)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 0x630)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, r(0), r(5), 0x2d0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(5), r(14), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(1), 0x364)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(1), 0x370)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x238)?)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(0).bit() | r(12).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(transfer_word(C::Al, true, r(4), r(15), 0x21c)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x21c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -0x40)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, TRACE, true)?;
    Ok(())
}

fn select_frame_model(c: &mut Code) -> Result<(), String> {
    c.check(0x6f3f4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x6f42c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x6f440, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x6f5ac, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -0x3c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x2c)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(0), r(1), 0x284, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(10), r(0), 0x800, false)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 0xa80)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0x38)?)?;
    c.branch(C::Al, 0x6f458, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -0x3c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x2c)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(0), r(1), 0x3a4, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(4), r(1), 0x4e0, false)?)?;
    c.branch(C::Al, 0x6f450, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(11), -0x3c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x2c)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(0), r(1), 0x114, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(4), r(1), 0x3f8, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(10), r(0), 0xc00, false)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(1), 0xa80)?)?;
    Ok(())
}

fn adaptive_model_projection(c: &mut Code) -> Result<(), String> {
    c.check(0x6f458)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x6f518, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 0x70)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.branch(C::Eq, 0x6f4d4, false)?;
    c.emit(transfer_word(C::Al, true, r(14), r(4), 0x18)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(14), 0x15)?)?;
    c.branch(C::Cc, 0x6f4d4, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 0x74)?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(5), r(3), r(3))?)?;
    c.emit(unsigned_multiply_long_accumulate(C::Al, r(0), r(5), r(2), r(2))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(5), Shift::Lsl, 24, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x40)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(5), Shift::Lsr, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.branch(C::Eq, 0x6f4d4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x1c)?)?;
    c.emit(multiply_long(C::Al, false, r(12), r(1), r(14), r(3))?)?;
    c.emit(unsigned_multiply_long_accumulate(C::Al, r(12), r(1), r(0), r(2))?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(12), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.branch(C::Eq, 0x6f4d4, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(11), -0x40)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsr, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, DIVIDE_U64, true)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x2)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(10), 0x28c)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x7f)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Ge, Op::Mov, r(2), r(0), 0x7f, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(data_immediate_with_flags(C::Lt, Op::Mov, r(2), r(0), 0x0, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(10), r(2), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -0x3c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x8c)?)?;
    c.emit(multiply_register(C::Al, r(2), r(1), r(0))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(0), r(2), Shift::Lsr, 8, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.branch(C::Al, 0x6f548, false)?;
    Ok(())
}

fn fallback_wide_projection(c: &mut Code) -> Result<(), String> {
    c.check(0x6f518)?;
    c.emit(transfer_multiple(C::Al, true, r(6), r(2).bit() | r(4).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(11), -0x3c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x2c8)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(10), 0x2cc)?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(5), r(1), r(2))?)?;
    c.emit(multiply_accumulate(C::Al, r(3), r(1), r(4), r(5))?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(7), r(2), r(3))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, DIVIDE_U64, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, false)?)?;
    Ok(())
}

fn minimum_target_and_commit(c: &mut Code) -> Result<(), String> {
    c.check(0x6f548)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 0x631)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x64)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x6f564, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x68)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x6f5cc, false)?;
    c.emit(transfer_multiple(C::Al, true, r(6), r(2).bit() | r(12).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0x2c)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(4), r(8), 0x0)?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(1), r(4), r(2))?)?;
    c.emit(multiply_accumulate(C::Al, r(6), r(4), r(12), r(1))?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(5), r(2), r(6))?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, DIVIDE_U64, true)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(0), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(1), 0x0, true)?)?;
    c.branch(C::Cs, COMMIT_TARGET, false)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(1), r(0), 0x1, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(9), 0x631)?)?;
    c.branch(C::Al, COMMIT_TARGET, false)?;
    Ok(())
}

fn invalid_model_and_floor(c: &mut Code) -> Result<(), String> {
    c.check(0x6f5ac)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x30)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(1), r(0), 0x86, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Mov, r(7), r(0), 0x0, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Orr, r(1), r(1), 0x1300, false)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(9), r(11), -0x3c)?)?;
    c.branch(C::Al, COMMIT_TARGET, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x60)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(13), 0x2c)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, COMMIT_TARGET, false)?;
    c.branch(C::Al, 0x6f59c, false)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    scene_change_and_fixed_point_psnr(&mut c)?;
    clamp_target_qp_and_trace(&mut c)?;
    select_frame_model(&mut c)?;
    adaptive_model_projection(&mut c)?;
    fallback_wide_projection(&mut c)?;
    minimum_target_and_commit(&mut c)?;
    invalid_model_and_floor(&mut c)?;
    c.check(POOL_START)?;
    c.data(RATE_CONTROL_SOURCE)?;
    c.data(NEWLINE)?;
    c.data(SCENE_CHANGED)?;
    c.data(INTERNAL_PSNR)?;
    c.data(MODEL_PARAMETERS)?;
    c.data(TARGET_PSNR_QP)?;
    c.data(FIXED_POINT_LIMIT_MINUS_ONE)?;
    c.data(FIXED_POINT_LIMIT)?;
    c.data(QUALITY_MODEL_TABLE)?;
    c.data(QUANTIZER_MODEL_TABLE)?;
    c.data(QUANTIZER_ASCENDING_TABLE)?;
    c.data(QUANTIZER_DESCENDING_TABLE)?;
    c.data(QUANTIZER_DIAGNOSTIC)?;
    c.data(BYTE_LANE_ONES)?;
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
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        for at in (START..END).step_by(4) {
            assert_eq!(word(&bytes, at), u32::from_le_bytes(
                reference[at + FILE_MAPPING_DELTA..at + FILE_MAPPING_DELTA + 4]
                    .try_into().unwrap()), "at {at:#x}");
        }
    }

    #[test]
    fn extent_and_semantic_pool() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 844);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        for (index, value) in [
            RATE_CONTROL_SOURCE, NEWLINE, SCENE_CHANGED, INTERNAL_PSNR,
            MODEL_PARAMETERS, TARGET_PSNR_QP, FIXED_POINT_LIMIT_MINUS_ONE,
            FIXED_POINT_LIMIT, QUALITY_MODEL_TABLE, QUANTIZER_MODEL_TABLE,
            QUANTIZER_ASCENDING_TABLE, QUANTIZER_DESCENDING_TABLE,
            QUANTIZER_DIAGNOSTIC, BYTE_LANE_ONES,
        ].into_iter().enumerate() {
            assert_eq!(word(&bytes, POOL_START + index * 4), value);
        }
    }

    #[test]
    fn checked_code_and_data() {
        for at in (START..POOL_START).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(SCENE_CHANGED).is_err());
            assert!(c.bytes.is_empty());
            c.emit(arm32::mov_register(C::Al, r(0), r(1))).unwrap();
        }
        for at in (POOL_START..END).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(arm32::mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.bytes.is_empty());
            c.data(SCENE_CHANGED).unwrap();
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(arm32::mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.data(SCENE_CHANGED).is_err());
            assert!(c.bytes.is_empty());
        }
        assert!(Code { at: START, bytes: Vec::new() }.check(END).is_err());
    }

    #[test]
    fn wide_accumulation_and_live_flags() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6f48c), unsigned_multiply_long_accumulate(
            C::Al, r(0), r(5), r(2), r(2)).unwrap());
        assert_eq!(word(&bytes, 0x6f4b4), unsigned_multiply_long_accumulate(
            C::Al, r(12), r(1), r(0), r(2)).unwrap());
        assert_eq!(word(&bytes, 0x6f4b8), data_register(
            C::Al, Op::Orr, r(0), r(12), r(1), Shift::Lsl, 0, true).unwrap());
        assert_eq!(word(&bytes, 0x6f588), data_register(
            C::Al, Op::Sub, r(2), r(0), r(7), Shift::Lsl, 0, true).unwrap());
        assert_eq!(word(&bytes, 0x6f594), data_immediate_with_flags(
            C::Al, Op::Sbc, r(1), r(1), 0, true).unwrap());
        for at in [0x6f4d0, 0x6f53c, 0x6f584] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, DIVIDE_U64, true).unwrap());
        }
        for (at, condition) in [(0x6f598, C::Cs), (0x6f5a8, C::Al),
            (0x6f5c8, C::Al), (0x6f5dc, C::Ls)] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, COMMIT_TARGET, false).unwrap());
        }
    }

    #[test]
    fn fixed_point_prediction_and_target_qp() {
        let bytes = fragment().unwrap();
        for at in [0x6f328, 0x6f380] {
            assert_eq!(word(&bytes, at), data_register(
                C::Al, Op::Rsb, r(0), r(2), r(0), Shift::Lsl, 4, false).unwrap());
            assert_eq!(word(&bytes, at + 4), data_register(
                C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsr, 10, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x6f3a0), transfer_byte(
            C::Al, false, r(7), r(4), 0x630).unwrap());
    }
}
