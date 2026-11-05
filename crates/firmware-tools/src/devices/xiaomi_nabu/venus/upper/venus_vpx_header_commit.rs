use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register,
    data_register_shifted_by_register, load_halfword, load_signed_halfword_register,
    mov_register, transfer_byte, transfer_byte_register, transfer_halfword_register,
    transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7df7c;
pub const END: usize = 0x7e544;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1480;
pub const DATA_BYTES: usize = 0;
pub const FUNCTION_STARTS: [usize; 2] = [START, 0x7e050];
const STUB_CANDIDATE: usize = 0x7e540;

fn r(n: u8) -> R { R::new(n).unwrap() }

const EDGES: &[(usize, C, usize, bool)] = &[
    (0x7dfcc, C::Al, 0x79e48, true),
    (0x7dfdc, C::Al, 0x7d834, true),
    (0x7e04c, C::Al, 0x205c, false),
    (0x7e0a0, C::Al, 0x79e48, true),
    (0x7e0ac, C::Ne, 0x7e180, false),
    (0x7e16c, C::Al, 0x7a048, true),
    (0x7e17c, C::Al, 0x7a048, true),
    (0x7e18c, C::Al, 0x7a048, true),
    (0x7e198, C::Eq, 0x7e30c, false),
    (0x7e1ac, C::Al, 0x7a048, true),
    (0x7e1bc, C::Al, 0x7a048, true),
    (0x7e1d0, C::Al, 0x7a048, true),
    (0x7e1e0, C::Al, 0x7a048, true),
    (0x7e1f0, C::Al, 0x7a048, true),
    (0x7e200, C::Al, 0x7a048, true),
    (0x7e20c, C::Ne, 0x7e1d4, false),
    (0x7e21c, C::Eq, 0x7e288, false),
    (0x7e23c, C::Al, 0x7a048, true),
    (0x7e260, C::Al, 0x7a048, true),
    (0x7e270, C::Al, 0x7a048, true),
    (0x7e280, C::Ne, 0x7e230, false),
    (0x7e284, C::Al, 0x7e2ac, false),
    (0x7e29c, C::Al, 0x7a048, true),
    (0x7e2a8, C::Ne, 0x7e290, false),
    (0x7e2c8, C::Ne, 0x7e2dc, false),
    (0x7e2d8, C::Al, 0x7e2fc, false),
    (0x7e2e8, C::Al, 0x7a048, true),
    (0x7e2fc, C::Al, 0x7a048, true),
    (0x7e308, C::Ne, 0x7e2bc, false),
    (0x7e31c, C::Al, 0x79e74, true),
    (0x7e32c, C::Al, 0x7a048, true),
    (0x7e33c, C::Al, 0x7a048, true),
    (0x7e34c, C::Al, 0x7a048, true),
    (0x7e35c, C::Al, 0x7a048, true),
    (0x7e36c, C::Al, 0x7a048, true),
    (0x7e37c, C::Al, 0x79e74, true),
    (0x7e38c, C::Al, 0x79e74, true),
    (0x7e39c, C::Al, 0x79e74, true),
    (0x7e3ac, C::Al, 0x79e74, true),
    (0x7e3bc, C::Al, 0x79e74, true),
    (0x7e3c8, C::Eq, 0x7e494, false),
    (0x7e3e4, C::Al, 0x7a048, true),
    (0x7e3f8, C::Al, 0x7a048, true),
    (0x7e400, C::Ne, 0x7e420, false),
    (0x7e41c, C::Al, 0x7a048, true),
    (0x7e424, C::Ne, 0x7e444, false),
    (0x7e440, C::Al, 0x7a048, true),
    (0x7e458, C::Al, 0x7a048, true),
    (0x7e468, C::Al, 0x7a048, true),
    (0x7e474, C::Eq, 0x7e4ac, false),
    (0x7e484, C::Eq, 0x7e4ac, false),
    (0x7e490, C::Al, 0x7e4b4, false),
    (0x7e4a8, C::Al, 0x7e4c4, false),
    (0x7e4b8, C::Al, 0x7a048, true),
    (0x7e4c8, C::Al, 0x7a048, true),
    (0x7e534, C::Al, 0x205c, true),
];

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VPX header address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && (START..END).contains(&at)
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("VPX header instruction outside CODE".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if !EDGES.contains(&(self.at, condition, target, link)) {
            return Err("VPX header invalid branch edge".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn commit_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7df7c)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.check(0x7df80)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x10)?)?;
    c.check(0x7df84)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0x7df88)?;
    c.emit(mov_register(C::Al, r(6), r(1)))?;
    c.check(0x7df8c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7df90)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7df94)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x3bc)?)?;
    c.check(0x7df98)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.check(0x7df9c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.check(0x7dfa0)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x8)?)?;
    c.check(0x7dfa4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0xb8)?)?;
    c.check(0x7dfa8)?;
    c.emit(transfer_word(C::Al, true, r(3), r(4), 0x20)?)?;
    c.check(0x7dfac)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.check(0x7dfb0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.check(0x7dfb4)?;
    c.emit(data_register(C::Ne, Op::Orr, r(5), r(2), r(1), Shift::Lsl, 1, false)?)?;
    c.check(0x7dfb8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(5), 0x10)?)?;
    c.check(0x7dfbc)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.check(0x7dfc0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0xf)?)?;
    c.check(0x7dfc4)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0x3cc)?)?;
    c.check(0x7dfc8)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7dfcc)?;
    c.branch(C::Al, 0x79e48, true)?;
    c.check(0x7dfd0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7dfd4)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.check(0x7dfd8)?;
    c.emit(mov_register(C::Al, r(2), r(6)))?;
    c.check(0x7dfdc)?;
    c.branch(C::Al, 0x7d834, true)?;
    c.check(0x7dfe0)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x8)?)?;
    c.check(0x7dfe4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x12)?)?;
    c.check(0x7dfe8)?;
    c.emit(transfer_word(C::Al, true, r(3), r(5), 0xc)?)?;
    c.check(0x7dfec)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(2), r(2), 0x8)?)?;
    c.check(0x7dff0)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, r(2), false)?)?;
    c.check(0x7dff4)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x14)?)?;
    c.check(0x7dff8)?;
    c.emit(transfer_byte_register(C::Al, false, r(1), r(2), r(3), Shift::Lsl, 0)?)?;
    c.check(0x7dffc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x8)?)?;
    c.check(0x7e000)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.check(0x7e004)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x78)?)?;
    c.check(0x7e008)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x3d0)?)?;
    c.check(0x7e00c)?;
    c.emit(load_halfword(C::Al, r(2), r(5), 0x0)?)?;
    c.check(0x7e010)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0x3d8)?)?;
    c.check(0x7e014)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x10)?)?;
    c.check(0x7e018)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0x3dc)?)?;
    c.check(0x7e01c)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x11)?)?;
    c.check(0x7e020)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0x3e0)?)?;
    c.check(0x7e024)?;
    c.emit(load_halfword(C::Al, r(2), r(5), 0x2)?)?;
    c.check(0x7e028)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0x3e4)?)?;
    c.check(0x7e02c)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x13)?)?;
    c.check(0x7e030)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0x3e8)?)?;
    c.check(0x7e034)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x3ec)?)?;
    c.check(0x7e038)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xf)?)?;
    c.check(0x7e03c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x3d4)?)?;
    c.check(0x7e040)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x2f)?)?;
    c.check(0x7e044)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x3bc)?)?;
    c.check(0x7e048)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7e04c)?;
    c.branch(C::Al, 0x205c, false)?;
    Ok(())
}

fn write_header_and_metadata(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7e050)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.check(0x7e054)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x18)?)?;
    c.check(0x7e058)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x18)?)?;
    c.check(0x7e05c)?;
    c.emit(mov_register(C::Al, r(8), r(0)))?;
    c.check(0x7e060)?;
    c.emit(mov_register(C::Al, r(9), r(1)))?;
    c.check(0x7e064)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e068)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.check(0x7e06c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x60)?)?;
    c.check(0x7e070)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.check(0x7e074)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.check(0x7e078)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x8)?)?;
    c.check(0x7e07c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0xb8)?)?;
    c.check(0x7e080)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 0x0)?)?;
    c.check(0x7e084)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.check(0x7e088)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7e08c)?;
    c.emit(data_register(C::Ne, Op::Orr, r(3), r(4), r(1), Shift::Lsl, 1, false)?)?;
    c.check(0x7e090)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(3), 0x10)?)?;
    c.check(0x7e094)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.check(0x7e098)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0xf)?)?;
    c.check(0x7e09c)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e0a0)?;
    c.branch(C::Al, 0x79e48, true)?;
    c.check(0x7e0a4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x0)?)?;
    c.check(0x7e0a8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e0ac)?;
    c.branch(C::Ne, 0x7e180, false)?;
    c.check(0x7e0b0)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0xc)?)?;
    c.check(0x7e0b4)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x14)?)?;
    c.check(0x7e0b8)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e0bc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x1)?)?;
    c.check(0x7e0c0)?;
    c.emit(transfer_word(C::Al, false, r(2), R::SP, 0xc)?)?;
    c.check(0x7e0c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9d)?)?;
    c.check(0x7e0c8)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0)?)?;
    c.check(0x7e0cc)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0xc)?)?;
    c.check(0x7e0d0)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x14)?)?;
    c.check(0x7e0d4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x1)?)?;
    c.check(0x7e0d8)?;
    c.emit(transfer_word(C::Al, false, r(2), R::SP, 0xc)?)?;
    c.check(0x7e0dc)?;
    c.emit(transfer_byte_register(C::Al, false, r(4), r(1), r(0), Shift::Lsl, 0)?)?;
    c.check(0x7e0e0)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0xc)?)?;
    c.check(0x7e0e4)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x14)?)?;
    c.check(0x7e0e8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x1)?)?;
    c.check(0x7e0ec)?;
    c.emit(transfer_word(C::Al, false, r(2), R::SP, 0xc)?)?;
    c.check(0x7e0f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2a)?)?;
    c.check(0x7e0f4)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0)?)?;
    c.check(0x7e0f8)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0xc)?)?;
    c.check(0x7e0fc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.check(0x7e100)?;
    c.emit(transfer_word(C::Al, true, r(2), R::SP, 0x14)?)?;
    c.check(0x7e104)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7e108)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 0xc)?)?;
    c.check(0x7e10c)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7e110)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.check(0x7e114)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0xc)?)?;
    c.check(0x7e118)?;
    c.emit(transfer_word(C::Al, true, r(2), R::SP, 0x14)?)?;
    c.check(0x7e11c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7e120)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 8, false)?)?;
    c.check(0x7e124)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 0xc)?)?;
    c.check(0x7e128)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7e12c)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0xc)?)?;
    c.check(0x7e130)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x50)?)?;
    c.check(0x7e134)?;
    c.emit(transfer_word(C::Al, true, r(2), R::SP, 0x14)?)?;
    c.check(0x7e138)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7e13c)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 0xc)?)?;
    c.check(0x7e140)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7e144)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x50)?)?;
    c.check(0x7e148)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0xc)?)?;
    c.check(0x7e14c)?;
    c.emit(transfer_word(C::Al, true, r(2), R::SP, 0x14)?)?;
    c.check(0x7e150)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7e154)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 8, false)?)?;
    c.check(0x7e158)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 0xc)?)?;
    c.check(0x7e15c)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7e160)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e164)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e168)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e16c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e170)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e174)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e178)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e17c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e180)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x190)?)?;
    c.check(0x7e184)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e188)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e18c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e190)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x190)?)?;
    c.check(0x7e194)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e198)?;
    c.branch(C::Eq, 0x7e30c, false)?;
    c.check(0x7e19c)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e1a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e1a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e1a8)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e1ac)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e1b0)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e1b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e1b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e1bc)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e1c0)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e1c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e1c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e1cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7e1d0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e1d4)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e1d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e1dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e1e0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e1e4)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e1e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e1ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7)?)?;
    c.check(0x7e1f0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e1f4)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e1f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e1fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e200)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e204)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.check(0x7e208)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x4)?)?;
    c.check(0x7e20c)?;
    c.branch(C::Ne, 0x7e1d4, false)?;
    c.check(0x7e210)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7e214)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4c8)?)?;
    c.check(0x7e218)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e21c)?;
    c.branch(C::Eq, 0x7e288, false)?;
    c.check(0x7e220)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0xcc)?)?;
    c.check(0x7e224)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7e228)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e22c)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x400)?)?;
    c.check(0x7e230)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e234)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e238)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e23c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e240)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7e244)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x194)?)?;
    c.check(0x7e248)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7e24c)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(0), r(4), Shift::Lsl, 0)?)?;
    c.check(0x7e250)?;
    c.emit(data_register(C::Al, Op::Sub, r(5), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.check(0x7e254)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e258)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7e25c)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r(1), r(5), 0x0)?)?;
    c.check(0x7e260)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e264)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsr, 31, false)?)?;
    c.check(0x7e268)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e26c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e270)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e274)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7e278)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x4)?)?;
    c.check(0x7e27c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x4)?)?;
    c.check(0x7e280)?;
    c.branch(C::Ne, 0x7e230, false)?;
    c.check(0x7e284)?;
    c.branch(C::Al, 0x7e2ac, false)?;
    c.check(0x7e288)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7e28c)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e290)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e294)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e298)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e29c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e2a0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.check(0x7e2a4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x4)?)?;
    c.check(0x7e2a8)?;
    c.branch(C::Ne, 0x7e290, false)?;
    c.check(0x7e2ac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xe2)?)?;
    c.check(0x7e2b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7e2b4)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e2b8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x400)?)?;
    c.check(0x7e2bc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7e2c0)?;
    c.emit(transfer_halfword_register(C::Al, true, r(0), r(0), r(7))?)?;
    c.check(0x7e2c4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xff)?)?;
    c.check(0x7e2c8)?;
    c.branch(C::Ne, 0x7e2dc, false)?;
    c.check(0x7e2cc)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e2d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e2d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e2d8)?;
    c.branch(C::Al, 0x7e2fc, false)?;
    c.check(0x7e2dc)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e2e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e2e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e2e8)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e2ec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7e2f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7e2f4)?;
    c.emit(load_signed_halfword_register(C::Al, r(1), r(0), r(7))?)?;
    c.check(0x7e2f8)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e2fc)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e300)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.check(0x7e304)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x3)?)?;
    c.check(0x7e308)?;
    c.branch(C::Ne, 0x7e2bc, false)?;
    c.check(0x7e30c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x198)?)?;
    c.check(0x7e310)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e314)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7e318)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e31c)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7e320)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x194)?)?;
    c.check(0x7e324)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e328)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7e32c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e330)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x19c)?)?;
    c.check(0x7e334)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e338)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7e33c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e340)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e344)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e348)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e34c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e350)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e354)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e358)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7e35c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e360)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x1)?)?;
    c.check(0x7e364)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e368)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7)?)?;
    c.check(0x7e36c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e370)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e374)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e378)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7e37c)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7e380)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e384)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e388)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7e38c)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7e390)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e394)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e398)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7e39c)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7e3a0)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e3a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e3a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7e3ac)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7e3b0)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e3b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e3b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7e3bc)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7e3c0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x0)?)?;
    c.check(0x7e3c4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e3c8)?;
    c.branch(C::Eq, 0x7e494, false)?;
    c.check(0x7e3cc)?;
    c.emit(transfer_word(C::Al, true, r(7), r(8), 0x58)?)?;
    c.check(0x7e3d0)?;
    c.emit(mov_register(C::Al, r(6), R::SP))?;
    c.check(0x7e3d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e3d8)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e3dc)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(7), 0x2)?)?;
    c.check(0x7e3e0)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsr, 1, false)?)?;
    c.check(0x7e3e4)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e3e8)?;
    c.emit(data_immediate(C::Al, Op::And, r(4), r(7), 0x4)?)?;
    c.check(0x7e3ec)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7e3f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e3f4)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsr, 2, false)?)?;
    c.check(0x7e3f8)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e3fc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0x7e400)?;
    c.branch(C::Ne, 0x7e420, false)?;
    c.check(0x7e404)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.check(0x7e408)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(7), 0x10)?)?;
    c.check(0x7e40c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7e410)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(0), r(7), Shift::Lsr, 4, false)?)?;
    c.check(0x7e414)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e418)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e41c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e420)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.check(0x7e424)?;
    c.branch(C::Ne, 0x7e444, false)?;
    c.check(0x7e428)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(0), r(7), 0x80, true)?)?;
    c.check(0x7e42c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7e430)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.check(0x7e434)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(0), r(7), Shift::Lsr, 7, false)?)?;
    c.check(0x7e438)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e43c)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e440)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e444)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(7), 0x1)?)?;
    c.check(0x7e448)?;
    c.emit(mov_register(C::Al, r(7), R::SP))?;
    c.check(0x7e44c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e450)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e454)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.check(0x7e458)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e45c)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.check(0x7e460)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e464)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e468)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e46c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x55)?)?;
    c.check(0x7e470)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e474)?;
    c.branch(C::Eq, 0x7e4ac, false)?;
    c.check(0x7e478)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7e47c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x4e0)?)?;
    c.check(0x7e480)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e484)?;
    c.branch(C::Eq, 0x7e4ac, false)?;
    c.check(0x7e488)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e48c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e490)?;
    c.branch(C::Al, 0x7e4b4, false)?;
    c.check(0x7e494)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x55)?)?;
    c.check(0x7e498)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7e49c)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e4a0)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e4a4)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7e4a8)?;
    c.branch(C::Al, 0x7e4c4, false)?;
    c.check(0x7e4ac)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e4b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7e4b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e4b8)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e4bc)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7e4c0)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7e4c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7e4c8)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7e4cc)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0x8)?)?;
    c.check(0x7e4d0)?;
    c.emit(transfer_byte(C::Al, true, r(2), R::SP, 0x12)?)?;
    c.check(0x7e4d4)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0xc)?)?;
    c.check(0x7e4d8)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(0), r(0), 0x8)?)?;
    c.check(0x7e4dc)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(0), r(0), r(2), Shift::Lsl, r(0), false)?)?;
    c.check(0x7e4e0)?;
    c.emit(transfer_word(C::Al, true, r(2), R::SP, 0x14)?)?;
    c.check(0x7e4e4)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7e4e8)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0x8)?)?;
    c.check(0x7e4ec)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.check(0x7e4f0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x2f)?)?;
    c.check(0x7e4f4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x78)?)?;
    c.check(0x7e4f8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0x74)?)?;
    c.check(0x7e4fc)?;
    c.emit(load_halfword(C::Al, r(2), R::SP, 0x0)?)?;
    c.check(0x7e500)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 0x7c)?)?;
    c.check(0x7e504)?;
    c.emit(transfer_byte(C::Al, true, r(2), R::SP, 0x10)?)?;
    c.check(0x7e508)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 0x80)?)?;
    c.check(0x7e50c)?;
    c.emit(transfer_byte(C::Al, true, r(2), R::SP, 0x11)?)?;
    c.check(0x7e510)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 0x84)?)?;
    c.check(0x7e514)?;
    c.emit(load_halfword(C::Al, r(2), R::SP, 0x2)?)?;
    c.check(0x7e518)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 0x88)?)?;
    c.check(0x7e51c)?;
    c.emit(transfer_byte(C::Al, true, r(2), R::SP, 0x13)?)?;
    c.check(0x7e520)?;
    c.emit(transfer_word(C::Al, false, r(2), r(8), 0x8c)?)?;
    c.check(0x7e524)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0x90)?)?;
    c.check(0x7e528)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xf)?)?;
    c.check(0x7e52c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0x78)?)?;
    c.check(0x7e530)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x60)?)?;
    c.check(0x7e534)?;
    c.branch(C::Al, 0x205c, true)?;
    c.check(0x7e538)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x18)?)?;
    c.check(0x7e53c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn stub_candidate(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7e540)?;
    c.emit(bx(C::Al, R::LR))?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    commit_header(&mut c)?;
    c.check(FUNCTION_STARTS[1])?;
    write_header_and_metadata(&mut c)?;
    c.check(STUB_CANDIDATE)?;
    stub_candidate(&mut c)?;
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
    fn structure_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END), (0x7df7c, 0x7e544));
        assert_eq!(END - START, 1480);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!((CODE_BYTES, DATA_BYTES), (370 * 4, 0));
        assert_eq!((0x7e050 - START) / 4, 53);
        assert_eq!((STUB_CANDIDATE - 0x7e050) / 4, 316);
        assert_eq!((END - STUB_CANDIDATE) / 4, 1);
        assert_eq!(FUNCTION_STARTS, [START, 0x7e050]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 370);
        assert_eq!(EDGES.len(), 56);
        assert!(EDGES.iter().all(|&(_, _, target, _)| target != STUB_CANDIDATE));
        for &(at, target, link) in &[
            (0x7dfdc, 0x7d834, true),
            (0x7e04c, 0x205c, false),
            (0x7e534, 0x205c, true),
        ] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, link).unwrap());
        }
        let mask = [4, 5, 6, 7, 8, 9, 11].into_iter().fold(R::PC.bit(), |mask, n| mask | r(n).bit());
        assert_eq!(word(&bytes, 0x7e53c), transfer_multiple(C::Al, true, R::SP,
            mask, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, STUB_CANDIDATE), bx(C::Al, R::LR));
    }

    #[test]
    fn exact_code_bounds() {
        for at in (START - 4..END + 8).chain([0, usize::MAX]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            let valid = at % 4 == 0 && (START..END).contains(&at);
            assert_eq!(c.emit(bx(C::Al, R::LR)).is_ok(), valid);
            assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
            assert_eq!(c.at, if valid { at + 4 } else { at });
        }
        let c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
    }

    #[test]
    fn exact_branch_guards() {
        let bytes = fragment().unwrap();
        for &(at, condition, target, link) in EDGES {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            c.branch(condition, target, link).unwrap();
            assert_eq!(word(&bytes, at), u32::from_le_bytes(c.bytes.try_into().unwrap()));
            for (source, cc, dest, linked) in [
                (at, condition, target, !link),
                (at, if condition == C::Al { C::Eq } else { C::Al }, target, link),
                (at, condition, target + 4, link),
                (at + 1, condition, target, link),
                (STUB_CANDIDATE, condition, target, link),
                (END, condition, target, link),
            ] {
                let mut c = TypedA32 { at: source, bytes: Vec::new() };
                assert!(c.branch(cc, dest, linked).is_err());
                assert!(c.bytes.is_empty());
                assert_eq!(c.at, source);
            }
        }
    }

    #[test]
    fn authenticated_same_buffer_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
