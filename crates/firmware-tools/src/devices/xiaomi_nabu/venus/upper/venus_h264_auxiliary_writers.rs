use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_immediate_with_flags, data_register, load_signed_byte_register,
    mov_register, mov_shifted_register, push_registers, transfer_byte,
    transfer_byte_postindexed, transfer_multiple, transfer_word,
};

pub const START: usize = 0x7b160;
pub const END: usize = 0x7b65c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1272;
pub const DATA_BYTES: usize = 4;
pub const FUNCTION_STARTS: [usize; 3] = [START, 0x7b240, 0x7b2a8];
const EXTERNAL_CONSTANT: u32 = 0xf4250;

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("H264 auxiliary writer at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..END).contains(&at) && at % 4 == 0 && at != 0x7b23c
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("H264 auxiliary instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if at != 0x7b23c || value != EXTERNAL_CONSTANT {
            return Err("H264 auxiliary literal mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        if self.at != 0x7b1bc || rd != r(7) || target != 0x7b23c {
            return Err("invalid H264 auxiliary literal load".into());
        }
        self.emit(transfer_word(C::Al, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7b184, C::Al, 0x79b6c, true)
            | (0x7b198, C::Al, 0x798f8, true)
            | (0x7b1a8, C::Al, 0x798f8, true)
            | (0x7b1b8, C::Al, 0x798f8, true)
            | (0x7b1d0, C::Al, 0x798f8, true)
            | (0x7b1dc, C::Ne, 0x7b1c4, false)
            | (0x7b1ec, C::Al, 0x798f8, true)
            | (0x7b204, C::Al, 0x798f8, true)
            | (0x7b214, C::Al, 0x798f8, true)
            | (0x7b224, C::Al, 0x798f8, true)
            | (0x7b238, C::Al, 0x798f8, false)
            | (0x7b28c, C::Al, 0x798f8, true)
            | (0x7b298, C::Al, 0x79c18, true)
            | (0x7b2a4, C::Al, 0x79c98, false)
            | (0x7b2d0, C::Eq, 0x7b2dc, false)
            | (0x7b2d8, C::Al, 0x7b2e8, false)
            | (0x7b2f0, C::Eq, 0x7b308, false)
            | (0x7b304, C::Al, 0x7b310, false)
            | (0x7b310, C::Al, 0x79db8, true)
            | (0x7b31c, C::Al, 0x79db8, true)
            | (0x7b330, C::Al, 0x798f8, true)
            | (0x7b33c, C::Ne, 0x7b34c, false)
            | (0x7b348, C::Al, 0x79db8, true)
            | (0x7b354, C::Ne, 0x7b398, false)
            | (0x7b360, C::Eq, 0x7b384, false)
            | (0x7b370, C::Eq, 0x7b384, false)
            | (0x7b380, C::Al, 0x7b38c, false)
            | (0x7b394, C::Al, 0x798f8, true)
            | (0x7b3a0, C::Ne, 0x7b3b8, false)
            | (0x7b3b0, C::Al, 0x798f8, true)
            | (0x7b3bc, C::Eq, 0x7b498, false)
            | (0x7b3cc, C::Ne, 0x7b3e4, false)
            | (0x7b3dc, C::Al, 0x798f8, true)
            | (0x7b3e0, C::Al, 0x7b404, false)
            | (0x7b3f0, C::Al, 0x798f8, true)
            | (0x7b400, C::Al, 0x79db8, true)
            | (0x7b40c, C::Eq, 0x7b46c, false)
            | (0x7b41c, C::Al, 0x798f8, true)
            | (0x7b428, C::Eq, 0x7b45c, false)
            | (0x7b43c, C::Al, 0x79db8, true)
            | (0x7b448, C::Al, 0x79db8, true)
            | (0x7b458, C::Cc, 0x7b430, false)
            | (0x7b464, C::Al, 0x79db8, true)
            | (0x7b468, C::Al, 0x7b47c, false)
            | (0x7b478, C::Al, 0x798f8, true)
            | (0x7b484, C::Ne, 0x7b498, false)
            | (0x7b494, C::Al, 0x798f8, true)
            | (0x7b49c, C::Eq, 0x7b5ac, false)
            | (0x7b4a8, C::Ne, 0x7b4dc, false)
            | (0x7b4b8, C::Al, 0x798f8, true)
            | (0x7b4cc, C::Eq, 0x7b59c, false)
            | (0x7b4d8, C::Al, 0x7b5a4, false)
            | (0x7b4e4, C::Eq, 0x7b59c, false)
            | (0x7b4f4, C::Al, 0x798f8, true)
            | (0x7b500, C::Eq, 0x7b58c, false)
            | (0x7b518, C::Ne, 0x7b55c, false)
            | (0x7b520, C::Ne, 0x7b57c, false)
            | (0x7b530, C::Ne, 0x7b550, false)
            | (0x7b53c, C::Al, 0x79db8, true)
            | (0x7b548, C::Al, 0x79db8, true)
            | (0x7b554, C::Al, 0x79db8, true)
            | (0x7b558, C::Al, 0x7b570, false)
            | (0x7b560, C::Al, 0x79db8, true)
            | (0x7b56c, C::Eq, 0x7b57c, false)
            | (0x7b578, C::Al, 0x79db8, true)
            | (0x7b588, C::Cc, 0x7b508, false)
            | (0x7b594, C::Al, 0x79db8, true)
            | (0x7b598, C::Al, 0x7b5ac, false)
            | (0x7b5a8, C::Al, 0x798f8, true)
            | (0x7b5b4, C::Ne, 0x7b5d0, false)
            | (0x7b5c0, C::Eq, 0x7b5d0, false)
            | (0x7b5cc, C::Al, 0x79db8, true)
            | (0x7b5f4, C::Al, 0x79df0, true)
            | (0x7b600, C::Al, 0x79db8, true)
            | (0x7b60c, C::Eq, 0x7b630, false)
            | (0x7b61c, C::Al, 0x79df0, true)
            | (0x7b62c, C::Al, 0x79df0, true)
            | (0x7b644, C::Al, 0x79c18, true)
            | (0x7b64c, C::Al, 0x79c98, true)
        );
        if !valid { return Err("invalid H264 auxiliary branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn write_user_data(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b160)?;
    c.check(0x7b160)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b164)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.check(0x7b168)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7b16c)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0x7b170)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7b174)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.check(0x7b178)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7b17c)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(1))?)?;
    c.check(0x7b180)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b184)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7b188)?;
    c.emit(transfer_word(C::Al, true, r(5), r(5), 0x1e0)?)?;
    c.check(0x7b18c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b190)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6)?)?;
    c.check(0x7b194)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b198)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b19c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b1a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5)?)?;
    c.check(0x7b1a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b1a8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b1ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b1b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x14)?)?;
    c.check(0x7b1b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b1b8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b1bc)?;
    c.literal(r(7), 0x7b23c)?;
    c.check(0x7b1c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7b1c4)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(1), r(7), 0x1)?)?;
    c.check(0x7b1c8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b1cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b1d0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b1d4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7b1d8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x10)?)?;
    c.check(0x7b1dc)?;
    c.branch(C::Ne, 0x7b1c4, false)?;
    c.check(0x7b1e0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b1e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x20)?)?;
    c.check(0x7b1e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b1ec)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b1f0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0x7b1f4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b1f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b1fc)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(5), r(0), 0x1)?)?;
    c.check(0x7b200)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7b204)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b208)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b20c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b210)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b214)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b218)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b21c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b220)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b224)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b228)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b22c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x80)?)?;
    c.check(0x7b230)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b234)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b238)?;
    c.branch(C::Al, 0x798f8, false)?;
    c.check(0x7b23c)?;
    c.data(0x7b23c, EXTERNAL_CONSTANT)?;
    c.check(0x7b240)
}

fn write_access_unit_delimiter(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b240)?;
    c.check(0x7b240)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b244)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.check(0x7b248)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7b24c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7b250)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xc)?)?;
    c.check(0x7b254)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), 0x1)?)?;
    c.check(0x7b258)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0x2)?)?;
    c.check(0x7b25c)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7b260)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(2), r(1), 0xff, true)?)?;
    c.check(0x7b264)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5)?)?;
    c.check(0x7b268)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b26c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b270)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x40)?)?;
    c.check(0x7b274)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b278)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x60)?)?;
    c.check(0x7b27c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7b280)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b284)?;
    c.emit(data_register(C::Ne, Op::Orr, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7b288)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b28c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b290)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b294)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b298)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b29c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b2a0)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b2a4)?;
    c.branch(C::Al, 0x79c98, false)?;
    c.check(0x7b2a8)
}

fn write_slice_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b2a8)?;
    c.check(0x7b2a8)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b2ac)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.check(0x7b2b0)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.check(0x7b2b4)?;
    c.emit(mov_register(C::Al, r(10), r(2)))?;
    c.check(0x7b2b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7b2bc)?;
    c.emit(mov_register(C::Al, r(9), r(1)))?;
    c.check(0x7b2c0)?;
    c.emit(mov_register(C::Al, r(8), r(0)))?;
    c.check(0x7b2c4)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(10), 0xc)?)?;
    c.check(0x7b2c8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2)?)?;
    c.check(0x7b2cc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b2d0)?;
    c.branch(C::Eq, 0x7b2dc, false)?;
    c.check(0x7b2d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.check(0x7b2d8)?;
    c.branch(C::Al, 0x7b2e8, false)?;
    c.check(0x7b2dc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1)?)?;
    c.check(0x7b2e0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b2e4)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(4), r(0), 0x1)?)?;
    c.check(0x7b2e8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x20)?)?;
    c.check(0x7b2ec)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b2f0)?;
    c.branch(C::Eq, 0x7b308, false)?;
    c.check(0x7b2f4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b2f8)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b2fc)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b300)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x2)?)?;
    c.check(0x7b304)?;
    c.branch(C::Al, 0x7b310, false)?;
    c.check(0x7b308)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b30c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b310)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b314)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b318)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b31c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b320)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x2d8)?)?;
    c.check(0x7b324)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x134)?)?;
    c.check(0x7b328)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x4)?)?;
    c.check(0x7b32c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b330)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b334)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1)?)?;
    c.check(0x7b338)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b33c)?;
    c.branch(C::Ne, 0x7b34c, false)?;
    c.check(0x7b340)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x138)?)?;
    c.check(0x7b344)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b348)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b34c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x190)?)?;
    c.check(0x7b350)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b354)?;
    c.branch(C::Ne, 0x7b398, false)?;
    c.check(0x7b358)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x82c)?)?;
    c.check(0x7b35c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b360)?;
    c.branch(C::Eq, 0x7b384, false)?;
    c.check(0x7b364)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7b368)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0xe0)?)?;
    c.check(0x7b36c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b370)?;
    c.branch(C::Eq, 0x7b384, false)?;
    c.check(0x7b374)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x13c)?)?;
    c.check(0x7b378)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x7c)?)?;
    c.check(0x7b37c)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(1), r(0), Shift::Lsl, 2, false)?)?;
    c.check(0x7b380)?;
    c.branch(C::Al, 0x7b38c, false)?;
    c.check(0x7b384)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x13c)?)?;
    c.check(0x7b388)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(0), 0x1f)?)?;
    c.check(0x7b38c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b390)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5)?)?;
    c.check(0x7b394)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b398)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x20)?)?;
    c.check(0x7b39c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7b3a0)?;
    c.branch(C::Ne, 0x7b3b8, false)?;
    c.check(0x7b3a4)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b3a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b3ac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b3b0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b3b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x20)?)?;
    c.check(0x7b3b8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b3bc)?;
    c.branch(C::Eq, 0x7b498, false)?;
    c.check(0x7b3c0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x9)?)?;
    c.check(0x7b3c4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x2d9)?)?;
    c.check(0x7b3c8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7b3cc)?;
    c.branch(C::Ne, 0x7b3e4, false)?;
    c.check(0x7b3d0)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b3d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b3d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b3dc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b3e0)?;
    c.branch(C::Al, 0x7b404, false)?;
    c.check(0x7b3e4)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b3e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b3ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b3f0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b3f4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x9)?)?;
    c.check(0x7b3f8)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7b3fc)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b400)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b404)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x144)?)?;
    c.check(0x7b408)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b40c)?;
    c.branch(C::Eq, 0x7b46c, false)?;
    c.check(0x7b410)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b414)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b418)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b41c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b420)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x144)?)?;
    c.check(0x7b424)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b428)?;
    c.branch(C::Eq, 0x7b45c, false)?;
    c.check(0x7b42c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0x7b430)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(9), r(5), Shift::Lsl, 3, false)?)?;
    c.check(0x7b434)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b438)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x150)?)?;
    c.check(0x7b43c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b440)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x154)?)?;
    c.check(0x7b444)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b448)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b44c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x144)?)?;
    c.check(0x7b450)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.check(0x7b454)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7b458)?;
    c.branch(C::Cc, 0x7b430, false)?;
    c.check(0x7b45c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b460)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3)?)?;
    c.check(0x7b464)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b468)?;
    c.branch(C::Al, 0x7b47c, false)?;
    c.check(0x7b46c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b470)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b474)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b478)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b47c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x20)?)?;
    c.check(0x7b480)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7b484)?;
    c.branch(C::Ne, 0x7b498, false)?;
    c.check(0x7b488)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b48c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b490)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b494)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b498)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.check(0x7b49c)?;
    c.branch(C::Eq, 0x7b5ac, false)?;
    c.check(0x7b4a0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1)?)?;
    c.check(0x7b4a4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b4a8)?;
    c.branch(C::Ne, 0x7b4dc, false)?;
    c.check(0x7b4ac)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b4b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b4b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b4b8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b4bc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x10c)?)?;
    c.check(0x7b4c0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b4c4)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(8), 0x110)?)?;
    c.check(0x7b4c8)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b4cc)?;
    c.branch(C::Eq, 0x7b59c, false)?;
    c.check(0x7b4d0)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b4d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b4d8)?;
    c.branch(C::Al, 0x7b5a4, false)?;
    c.check(0x7b4dc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x14c)?)?;
    c.check(0x7b4e0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b4e4)?;
    c.branch(C::Eq, 0x7b59c, false)?;
    c.check(0x7b4e8)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b4ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b4f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b4f4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b4f8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x14c)?)?;
    c.check(0x7b4fc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b500)?;
    c.branch(C::Eq, 0x7b58c, false)?;
    c.check(0x7b504)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7b508)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(9), r(7), Shift::Lsl, 3, false)?)?;
    c.check(0x7b50c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x119)?)?;
    c.check(0x7b510)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x170)?)?;
    c.check(0x7b514)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b518)?;
    c.branch(C::Ne, 0x7b55c, false)?;
    c.check(0x7b51c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x6)?)?;
    c.check(0x7b520)?;
    c.branch(C::Ne, 0x7b57c, false)?;
    c.check(0x7b524)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1)?)?;
    c.check(0x7b528)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6)?)?;
    c.check(0x7b52c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b530)?;
    c.branch(C::Ne, 0x7b550, false)?;
    c.check(0x7b534)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b538)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b53c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b540)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b544)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b548)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b54c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x170)?)?;
    c.check(0x7b550)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b554)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b558)?;
    c.branch(C::Al, 0x7b570, false)?;
    c.check(0x7b55c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b560)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b564)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x170)?)?;
    c.check(0x7b568)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x5)?)?;
    c.check(0x7b56c)?;
    c.branch(C::Eq, 0x7b57c, false)?;
    c.check(0x7b570)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x174)?)?;
    c.check(0x7b574)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b578)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b57c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x14c)?)?;
    c.check(0x7b580)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7b584)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7b588)?;
    c.branch(C::Cc, 0x7b508, false)?;
    c.check(0x7b58c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b590)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b594)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b598)?;
    c.branch(C::Al, 0x7b5ac, false)?;
    c.check(0x7b59c)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b5a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b5a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b5a8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b5ac)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0xa8)?)?;
    c.check(0x7b5b0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b5b4)?;
    c.branch(C::Ne, 0x7b5d0, false)?;
    c.check(0x7b5b8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x20)?)?;
    c.check(0x7b5bc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b5c0)?;
    c.branch(C::Eq, 0x7b5d0, false)?;
    c.check(0x7b5c4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0xa9)?)?;
    c.check(0x7b5c8)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b5cc)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b5d0)?;
    c.emit(mov_register(C::Al, r(1), r(10)))?;
    c.check(0x7b5d4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x470)?)?;
    c.check(0x7b5d8)?;
    c.emit(transfer_multiple(C::Al, true, r(1), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b5dc)?;
    c.emit(transfer_multiple(C::Al, false, r(0), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b5e0)?;
    c.emit(transfer_multiple(C::Al, true, r(1), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0x7b5e4)?;
    c.emit(transfer_multiple(C::Al, false, r(0), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0x7b5e8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0xc)?)?;
    c.check(0x7b5ec)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x14)?)?;
    c.check(0x7b5f0)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b5f4)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7b5f8)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x2c4)?)?;
    c.check(0x7b5fc)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b600)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b604)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x2c4)?)?;
    c.check(0x7b608)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b60c)?;
    c.branch(C::Eq, 0x7b630, false)?;
    c.check(0x7b610)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x2c8)?)?;
    c.check(0x7b614)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 1)?)?;
    c.check(0x7b618)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b61c)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7b620)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x2cc)?)?;
    c.check(0x7b624)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 1)?)?;
    c.check(0x7b628)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b62c)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7b630)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0xa8)?)?;
    c.check(0x7b634)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b638)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b63c)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b640)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b644)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b648)?;
    c.emit(mov_register(C::Al, r(0), r(10)))?;
    c.check(0x7b64c)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7b650)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7b654)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x7b658)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b65c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    write_user_data(&mut c)?;
    write_access_unit_delimiter(&mut c)?;
    write_slice_header(&mut c)?;
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
    fn structure_and_arithmetic() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 1276);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(FUNCTION_STARTS, [0x7b160, 0x7b240, 0x7b2a8]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count() * 4, CODE_BYTES);
        assert_eq!(word(&bytes, 0x7b23c), EXTERNAL_CONSTANT);
        assert_eq!(word(&bytes, 0x7b1bc), transfer_word(C::Al, true, r(7), R::PC, 0x78).unwrap());
        for (at, target) in [(0x7b238, 0x798f8), (0x7b2a4, 0x79c98)] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x7b658), transfer_multiple(C::Al, true, R::SP,
            (4..=11).fold(R::PC.bit(), |mask, n| mask | r(n).bit()),
            BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x7b260), data_immediate_with_flags(C::Al, Op::And, r(2), r(1), 0xff, true).unwrap());
        for value in [0u32, 1, 31, 32, 0x8000_0000, u32::MAX] {
            assert_eq!((value << 2) & 0x7c, (value & 0x1f) << 2);
            assert_eq!((value as i32 >> 1) as u32, (value >> 1) | (value & 0x8000_0000));
            assert_eq!(value.wrapping_sub(0x14).wrapping_add(0x14), value);
        }
        assert_eq!(0x11c | 0x400, 0x51c);
        assert_eq!(2 * 6 * 4, 48);
        for (at, op, dst, src, value) in [
            (0x7b5d4, Op::Add, 0, 9, 0x470),
            (0x7b5ec, Op::Sub, 1, 0, 0x14),
        ] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, op, r(dst), r(src), value).unwrap());
        }
        for at in [0x7b614, 0x7b624] {
            assert_eq!(word(&bytes, at), mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 1).unwrap());
        }
    }

    #[test]
    fn exact_code_data_and_target_guards() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), at != 0x7b23c);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.data(at, EXTERNAL_CONSTANT).is_ok(), at == 0x7b23c);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        for at in [START - 4, START + 1, 0x7b23d, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, EXTERNAL_CONSTANT).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: 0x7b1bc, bytes: Vec::new() };
        assert!(c.literal(r(6), 0x7b23c).is_err());
        assert!(c.literal(r(7), 0x7b240).is_err());
        assert!(c.check(END).is_err());
        assert!(c.data(0x7b23c, EXTERNAL_CONSTANT).is_err());
        assert!(c.bytes.is_empty());
        for (at, target) in [(0x7b238, 0x798f8), (0x7b2a4, 0x79c98)] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.branch(C::Al, target, true).is_err());
            assert!(c.branch(C::Eq, target, false).is_err());
            for invalid in [START + 1, 0x7b23c, END, EXTERNAL_CONSTANT as usize, target + 4] {
                assert!(c.branch(C::Al, invalid, false).is_err());
            }
            assert!(c.bytes.is_empty());
            c.branch(C::Al, target, false).unwrap();
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
