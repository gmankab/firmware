use arm32::{
    branch, data_immediate, mov_register, signed_multiply_accumulate_bottom_halves,
    transfer_byte, transfer_halfword_register, transfer_multiple, transfer_word,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6af2c;
pub const END: usize = 0x6b38c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1100;
pub const DATA_BYTES: usize = 20;
const POOL_START: usize = 0x6b378;
const WRONG_FRAME_FORMAT: u32 = 0xf2fbe;
const FUNCTION_NAME: u32 = 0xf2ff1;
const NEWLINE: u32 = 0xe2788;
const RATE_CONTROL_SOURCE: u32 = 0xf2e4c;
const INVALID_HIERARCHICAL_FRAME: u32 = 0xf300e;

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
            return Err(format!("frame distortion address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || (self.at >= POOL_START) != data
        {
            return Err(format!("frame distortion output kind or address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn pointer(&mut self, value: u32) -> Result<(), String> {
        self.word(value, true)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0xc)?)?;
    c.emit(mov_register(C::Al, r(7), r(0)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb30)?)?;
    c.emit(mov_register(C::Al, r(5), r(3)))?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(11), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(11), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(11), 0x8)?)?;
    c.emit(mov_register(C::Al, r(6), r(1)))?;
    c.emit(mov_register(C::Al, r(10), r(2)))?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(3), r(7), r(0), Shift::Lsl, 0)?)?;
    c.branch(C::Eq, 0x6b024, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x2)?)?;
    c.branch(C::Eq, 0x6b0d8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x1)?)?;
    c.branch(C::Ne, 0x6b14c, false)?;
    c.check(0x6af7c)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x6c0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0x6c4)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(7), 0x712)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 0x700)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x16)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x6e8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x94)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(7), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x318)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(7), r(1))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x720)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x728)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0xd15)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 0x78)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x6b194, false)?;
    c.check(0x6afd8)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x6c8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0x6cc)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(7), 0x713)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 0x704)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1a)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x6ec)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x94)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(7), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x31c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(7), r(1))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x724)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x72c)?)?;
    c.branch(C::Al, 0x6b194, false)?;
    c.check(0x6b024)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x6b0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0x6b4)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(7), 0x710)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 0x6f8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x6e0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0xd15)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 0x78)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x6b070, false)?;
    c.check(0x6b058)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x6b8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0x6bc)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(7), 0x711)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 0x6fc)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x6e0)?)?;
    c.check(0x6b070)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x31c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x28)?)?;
    c.emit(mov_register(C::Al, r(8), r(9)))?;
    c.emit(mov_register(C::Al, r(9), r(10)))?;
    c.emit(mov_register(C::Al, r(10), r(12)))?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1a)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x700)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x318)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x400)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x16)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x700)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0x720)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(mov_register(C::Al, r(12), r(10)))?;
    c.emit(mov_register(C::Al, r(10), r(9)))?;
    c.emit(mov_register(C::Al, r(9), r(8)))?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0x4)?)?;
    c.branch(C::Al, 0x6b194, false)?;
    c.check(0x6b0d8)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x6d0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0x6d4)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(7), 0x714)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 0x708)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x730)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x6f0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x94)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(7), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x32)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x700)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(7), r(1))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x738)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x740)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0xd15)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 0x78)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x6b194, false)?;
    c.check(0x6b130)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x6d8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0x6dc)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(7), 0x715)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(7), 0x70c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x6f0)?)?;
    c.branch(C::Al, 0x6b194, false)?;
    c.check(0x6b14c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x224)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x224)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3e8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(13), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1c00)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x20c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x204)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe9)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1f00)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(12), r(13), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x8)?)?;
    c.check(0x6b194)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x6b330, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x6b254, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x2)?)?;
    c.branch(C::Eq, 0x6b298, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x1)?)?;
    c.branch(C::Ne, 0x6b338, false)?;
    c.check(0x6b1b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x96)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(8), r(0), r(3))?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x140)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x144)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0x180)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x192)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x94)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x198)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xac)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1a0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1a8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(8), r(1), r(3))?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0x60d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x6b330, false)?;
    c.check(0x6b214)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x148)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x14c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0x184)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x193)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9a)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x94)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x19c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xac)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1a4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1ac)?)?;
    c.branch(C::Al, 0x6b330, false)?;
    c.check(0x6b254)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(8), r(0), r(3))?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x130)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x134)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0x178)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x190)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(8), r(1), r(3))?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0x60d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x6b330, false)?;
    c.check(0x6b284)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x138)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x13c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0x17c)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x191)?)?;
    c.branch(C::Al, 0x6b330, false)?;
    c.check(0x6b298)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1b0)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(8), r(0), r(3))?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x150)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x154)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0x188)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x194)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x94)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xb2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1b8)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1c0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r(1), r(8), r(1), r(3))?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0x60d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x6b330, false)?;
    c.check(0x6b2f4)?;
    c.emit(transfer_word(C::Al, false, r(10), r(0), 0x158)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x15c)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(0), 0x18c)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x195)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1b4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x94)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xb6)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(0), r(2))?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1bc)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xb0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1c4)?)?;
    c.check(0x6b330)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x6b338)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x2000)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(15), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x2000)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(POOL_START)?;
    c.pointer(WRONG_FRAME_FORMAT)?;
    c.pointer(FUNCTION_NAME)?;
    c.pointer(NEWLINE)?;
    c.pointer(RATE_CONTROL_SOURCE)?;
    c.pointer(INVALID_HIERARCHICAL_FRAME)?;
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
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn extent_and_semantic_pool() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1120);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        for (at, value) in [
            (0x6b378, WRONG_FRAME_FORMAT), (0x6b37c, FUNCTION_NAME),
            (0x6b380, NEWLINE), (0x6b384, RATE_CONTROL_SOURCE),
            (0x6b388, INVALID_HIERARCHICAL_FRAME),
        ] {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn checked_code_and_data() {
        for at in (START..POOL_START).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.pointer(WRONG_FRAME_FORMAT).is_err());
            assert!(c.bytes.is_empty());
            c.emit(mov_register(C::Al, r(0), r(1))).unwrap();
        }
        for at in (POOL_START..END).step_by(4) {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.bytes.is_empty());
            c.pointer(WRONG_FRAME_FORMAT).unwrap();
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.pointer(WRONG_FRAME_FORMAT).is_err());
            assert!(c.bytes.is_empty());
        }
        assert!(Code { at: START, bytes: Vec::new() }.check(END).is_err());
    }

    #[test]
    fn class_edges_calls_and_error_tail() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x6af68, C::Eq, 0x6b024, false),
            (0x6af70, C::Eq, 0x6b0d8, false),
            (0x6af78, C::Ne, 0x6b14c, false),
            (0x6b0bc, C::Al, 0x31ec, true),
            (0x6b168, C::Al, 0x2ec94, true),
            (0x6b174, C::Al, 0x2ec94, true),
            (0x6b188, C::Al, 0x17b0, true),
            (0x6b19c, C::Eq, 0x6b330, false),
            (0x6b1a4, C::Eq, 0x6b254, false),
            (0x6b1ac, C::Eq, 0x6b298, false),
            (0x6b1b4, C::Ne, 0x6b338, false),
            (0x6b210, C::Ne, 0x6b330, false),
            (0x6b280, C::Ne, 0x6b330, false),
            (0x6b2f0, C::Ne, 0x6b330, false),
            (0x6b34c, C::Al, 0x2ec94, true),
            (0x6b358, C::Al, 0x2ec94, true),
            (0x6b374, C::Al, 0x17b0, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        let saved = r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit()
            | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit();
        assert_eq!(word(&bytes, 0x6b330), data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c).unwrap());
        assert_eq!(word(&bytes, 0x6b334), transfer_multiple(C::Al, true, r(13), saved | r(15).bit(), BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x6b370), transfer_multiple(C::Al, true, r(13), saved | r(14).bit(), BlockMode::IncrementAfter, true).unwrap());
    }

    #[test]
    fn hierarchical_stride_and_halfword_clears() {
        let bytes = fragment().unwrap();
        for (at, destination, stride) in [
            (0x6b1c0, 0, 0), (0x6b204, 1, 1),
            (0x6b260, 0, 0), (0x6b274, 1, 1),
            (0x6b2a0, 0, 0), (0x6b2e4, 1, 1),
        ] {
            assert_eq!(word(&bytes, at), signed_multiply_accumulate_bottom_halves(C::Al, r(destination), r(8), r(stride), r(3)).unwrap());
        }
        for at in [0x6b08c, 0x6b098, 0x6b0a4, 0x6b0b0] {
            assert_eq!(word(&bytes, at), transfer_halfword_register(C::Al, false, r(1), r(7), r(0)).unwrap());
        }
        for at in [0x6b208, 0x6b278, 0x6b2e8] {
            assert_eq!(word(&bytes, at), transfer_byte(C::Al, true, r(1), r(1), 0x60d).unwrap());
        }
    }
}
