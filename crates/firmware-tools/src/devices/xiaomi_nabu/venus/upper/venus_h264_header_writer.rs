use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register,
    load_halfword, load_signed_byte_register, mov_register, mov_shifted_register,
    push_registers, transfer_byte, transfer_halfword_register, transfer_multiple,
    transfer_word, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7a3ec;
pub const END: usize = 0x7aaa0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1712;
pub const DATA_BYTES: usize = 4;
const POOL_START: usize = 0x7aa9c;
const ASSERTION_SOURCE: u32 = 0xf4215;

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("H264 header address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && (START..POOL_START).contains(&at)
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("H264 header instruction outside CODE".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if at != POOL_START || value != ASSERTION_SOURCE {
            return Err("H264 header protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, dst: R, target: usize) -> Result<(), String> {
        if !matches!(self.at, 0x7a67c | 0x7a6dc) || dst != r(2) || target != POOL_START {
            return Err("H264 header invalid literal".into());
        }
        self.emit(transfer_word(C::Al, true, dst, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn jump(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7a420, C::Al, 0x79b6c, true)
            | (0x7a430, C::Al, 0x798f8, true)
            | (0x7a44c, C::Al, 0x798f8, true)
            | (0x7a494, C::Al, 0x798f8, true)
            | (0x7a4a4, C::Al, 0x798f8, true)
            | (0x7a4b0, C::Al, 0x79db8, true)
            | (0x7a4bc, C::Eq, 0x7a4cc, false)
            | (0x7a4c8, C::Eq, 0x7a550, false)
            | (0x7a4d4, C::Al, 0x79db8, true)
            | (0x7a4e4, C::Al, 0x79db8, true)
            | (0x7a4f0, C::Al, 0x79db8, true)
            | (0x7a500, C::Al, 0x798f8, true)
            | (0x7a50c, C::Ne, 0x7a540, false)
            | (0x7a51c, C::Al, 0x798f8, true)
            | (0x7a52c, C::Al, 0x798f8, true)
            | (0x7a538, C::Ne, 0x7a520, false)
            | (0x7a53c, C::Al, 0x7a550, false)
            | (0x7a54c, C::Al, 0x798f8, true)
            | (0x7a558, C::Al, 0x79db8, true)
            | (0x7a564, C::Al, 0x79db8, true)
            | (0x7a570, C::Ne, 0x7a580, false)
            | (0x7a57c, C::Al, 0x79db8, true)
            | (0x7a588, C::Al, 0x79db8, true)
            | (0x7a598, C::Eq, 0x7a5b0, false)
            | (0x7a5a0, C::Ne, 0x7a5b8, false)
            | (0x7a5ac, C::Al, 0x7a5c0, false)
            | (0x7a5b4, C::Eq, 0x7aa94, false)
            | (0x7a5c4, C::Al, 0x798f8, true)
            | (0x7a5d4, C::Al, 0x79db8, true)
            | (0x7a5e4, C::Al, 0x79db8, true)
            | (0x7a5f4, C::Al, 0x798f8, true)
            | (0x7a604, C::Al, 0x798f8, true)
            | (0x7a610, C::Eq, 0x7a628, false)
            | (0x7a620, C::Al, 0x798f8, true)
            | (0x7a624, C::Al, 0x7a64c, false)
            | (0x7a640, C::Al, 0x798f8, true)
            | (0x7a648, C::Eq, 0x7a708, false)
            | (0x7a658, C::Al, 0x79db8, true)
            | (0x7a664, C::Ne, 0x7a67c, false)
            | (0x7a678, C::Al, 0x7a6a4, false)
            | (0x7a694, C::Al, 0x17b0, true)
            | (0x7a6a8, C::Al, 0x79db8, true)
            | (0x7a6b8, C::Al, 0x79db8, true)
            | (0x7a6c4, C::Ne, 0x7a6dc, false)
            | (0x7a6d8, C::Al, 0x7a700, false)
            | (0x7a6f0, C::Al, 0x17b0, true)
            | (0x7a704, C::Al, 0x79db8, true)
            | (0x7a710, C::Eq, 0x7aa28, false)
            | (0x7a724, C::Al, 0x798f8, true)
            | (0x7a738, C::Al, 0x798f8, true)
            | (0x7a74c, C::Eq, 0x7a7ac, false)
            | (0x7a768, C::Al, 0x7a08c, true)
            | (0x7a77c, C::Al, 0x798f8, true)
            | (0x7a784, C::Ne, 0x7a7ac, false)
            | (0x7a798, C::Al, 0x798f8, true)
            | (0x7a7a8, C::Al, 0x798f8, true)
            | (0x7a7c0, C::Al, 0x798f8, true)
            | (0x7a7d4, C::Al, 0x798f8, true)
            | (0x7a7e0, C::Eq, 0x7a8a0, false)
            | (0x7a80c, C::Al, 0x798f8, true)
            | (0x7a828, C::Al, 0x798f8, true)
            | (0x7a838, C::Al, 0x798f8, true)
            | (0x7a844, C::Ne, 0x7a8a0, false)
            | (0x7a868, C::Al, 0x798f8, true)
            | (0x7a880, C::Al, 0x798f8, true)
            | (0x7a89c, C::Al, 0x798f8, true)
            | (0x7a8b4, C::Al, 0x798f8, true)
            | (0x7a8c0, C::Eq, 0x7a8e8, false)
            | (0x7a8d0, C::Al, 0x798f8, true)
            | (0x7a8e0, C::Al, 0x798f8, true)
            | (0x7a8f4, C::Al, 0x798f8, true)
            | (0x7a914, C::Eq, 0x7a978, false)
            | (0x7a928, C::Al, 0x798f8, true)
            | (0x7a938, C::Al, 0x798f8, true)
            | (0x7a950, C::Al, 0x798f8, true)
            | (0x7a964, C::Al, 0x798f8, true)
            | (0x7a974, C::Al, 0x798f8, true)
            | (0x7a988, C::Al, 0x798f8, true)
            | (0x7a998, C::Al, 0x798f8, true)
            | (0x7a9a8, C::Al, 0x798f8, true)
            | (0x7a9bc, C::Al, 0x798f8, true)
            | (0x7a9c8, C::Eq, 0x7aa38, false)
            | (0x7a9d8, C::Al, 0x798f8, true)
            | (0x7a9e4, C::Al, 0x79db8, true)
            | (0x7a9f0, C::Al, 0x79db8, true)
            | (0x7a9fc, C::Al, 0x79db8, true)
            | (0x7aa08, C::Al, 0x79db8, true)
            | (0x7aa14, C::Al, 0x79db8, true)
            | (0x7aa20, C::Al, 0x79db8, true)
            | (0x7aa24, C::Al, 0x7aa38, false)
            | (0x7aa34, C::Al, 0x798f8, true)
            | (0x7aa44, C::Al, 0x798f8, true)
            | (0x7aa50, C::Al, 0x79c18, true)
            | (0x7aa58, C::Al, 0x79c98, true)
            | (0x7aa68, C::Ne, 0x7aa74, false)
            | (0x7aa90, C::Al, 0x79b34, false)
            | (0x7aa98, C::Al, 0x7a59c, false)
        );
        if !valid { return Err("H264 header invalid branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn sequence_profile(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a3ec)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7a3f0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.check(0x7a3f4)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x14)?)?;
    c.check(0x7a3f8)?;
    c.emit(transfer_word(C::Al, false, r(2), R::SP, 0x4)?)?;
    c.check(0x7a3fc)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7a400)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0x7a404)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.check(0x7a408)?;
    c.emit(mov_register(C::Al, r(10), r(3)))?;
    c.check(0x7a40c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.check(0x7a410)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.check(0x7a414)?;
    c.emit(transfer_word(C::Al, true, r(9), r(4), 0x20)?)?;
    c.check(0x7a418)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(8))?)?;
    c.check(0x7a41c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a420)?;
    c.jump(C::Al, 0x79b6c, true)?;
    c.check(0x7a424)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a428)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x67)?)?;
    c.check(0x7a42c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a430)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a434)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2ec)?)?;
    c.check(0x7a438)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x64)?)?;
    c.check(0x7a43c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a440)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a444)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a448)?;
    c.emit(transfer_word(C::Eq, true, r(1), r(5), 0x8)?)?;
    c.check(0x7a44c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a450)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2db)?)?;
    c.check(0x7a454)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2dc)?)?;
    c.check(0x7a458)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x2dd)?)?;
    c.check(0x7a45c)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(5), 0x2de)?)?;
    c.check(0x7a460)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7a464)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x80)?)?;
    c.check(0x7a468)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a46c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2df)?)?;
    c.check(0x7a470)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x40)?)?;
    c.check(0x7a474)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7a478)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a47c)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x10)?)?;
    c.check(0x7a480)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.check(0x7a484)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x8)?)?;
    c.check(0x7a488)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a48c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a490)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x4)?)?;
    c.check(0x7a494)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a498)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0xc)?)?;
    c.check(0x7a49c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a4a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a4a4)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a4a8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a4ac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a4b0)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a4b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8)?)?;
    c.check(0x7a4b8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x64)?)?;
    c.check(0x7a4bc)?;
    c.jump(C::Eq, 0x7a4cc, false)?;
    c.check(0x7a4c0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2ec)?)?;
    c.check(0x7a4c4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a4c8)?;
    c.jump(C::Eq, 0x7a550, false)?;
    c.check(0x7a4cc)
}

fn high_profile(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a4cc)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2e0)?)?;
    c.check(0x7a4d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a4d4)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a4d8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a4dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a4e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7a4e4)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a4e8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a4ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a4f0)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a4f4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a4f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a4fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a500)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a504)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x166)?)?;
    c.check(0x7a508)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7a50c)?;
    c.jump(C::Ne, 0x7a540, false)?;
    c.check(0x7a510)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a514)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a518)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a51c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a520)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a524)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a528)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a52c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a530)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7a534)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x8)?)?;
    c.check(0x7a538)?;
    c.jump(C::Ne, 0x7a520, false)?;
    c.check(0x7a53c)?;
    c.jump(C::Al, 0x7a550, false)?;
    c.check(0x7a540)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a544)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a548)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a54c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a550)
}

fn sequence_dimensions(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a550)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d8)?)?;
    c.check(0x7a554)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a558)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a55c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x190)?)?;
    c.check(0x7a560)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a564)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a568)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x190)?)?;
    c.check(0x7a56c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a570)?;
    c.jump(C::Ne, 0x7a580, false)?;
    c.check(0x7a574)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a578)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a57c)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a580)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x165)?)?;
    c.check(0x7a584)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a588)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a58c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x82c)?)?;
    c.check(0x7a590)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x254)?)?;
    c.check(0x7a594)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7a598)?;
    c.jump(C::Eq, 0x7a5b0, false)?;
    c.check(0x7a59c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a5a0)?;
    c.jump(C::Ne, 0x7a5b8, false)?;
    c.check(0x7a5a4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a5a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a5ac)?;
    c.jump(C::Al, 0x7a5c0, false)?;
    c.check(0x7a5b0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a5b4)?;
    c.jump(C::Eq, 0x7aa94, false)?;
    c.check(0x7a5b8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a5bc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a5c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a5c4)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a5c8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x22c)?)?;
    c.check(0x7a5cc)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7a5d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a5d4)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a5d8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x230)?)?;
    c.check(0x7a5dc)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7a5e0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a5e4)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a5e8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a5ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a5f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a5f4)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a5f8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a5fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a600)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a604)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a608)
}

fn frame_cropping(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a608)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x4c)?)?;
    c.check(0x7a60c)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.check(0x7a610)?;
    c.jump(C::Eq, 0x7a628, false)?;
    c.check(0x7a614)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a618)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a61c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a620)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a624)?;
    c.jump(C::Al, 0x7a64c, false)?;
    c.check(0x7a628)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x50)?)?;
    c.check(0x7a62c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a630)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(6), r(0), 0xf, true)?)?;
    c.check(0x7a634)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a638)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7a63c)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a640)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a644)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7a648)?;
    c.jump(C::Eq, 0x7a708, false)?;
    c.check(0x7a64c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a650)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a654)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7a658)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a65c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2e0)?)?;
    c.check(0x7a660)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7a664)?;
    c.jump(C::Ne, 0x7a67c, false)?;
    c.check(0x7a668)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x4c)?)?;
    c.check(0x7a66c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x22c)?)?;
    c.check(0x7a670)?;
    c.emit(data_register(C::Al, Op::Rsb, r(0), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7a674)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 1)?)?;
    c.check(0x7a678)?;
    c.jump(C::Al, 0x7a6a4, false)?;
    c.check(0x7a67c)?;
    c.literal(r(2), 0x7aa9c)?;
    c.check(0x7a680)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a684)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4b)?)?;
    c.check(0x7a688)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x1)?)?;
    c.check(0x7a68c)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x100)?)?;
    c.check(0x7a690)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.check(0x7a694)?;
    c.jump(C::Al, 0x17b0, true)?;
    c.check(0x7a698)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x4c)?)?;
    c.check(0x7a69c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x22c)?)?;
    c.check(0x7a6a0)?;
    c.emit(data_register(C::Al, Op::Rsb, r(1), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7a6a4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a6a8)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a6ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a6b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a6b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7a6b8)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a6bc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2e0)?)?;
    c.check(0x7a6c0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7a6c4)?;
    c.jump(C::Ne, 0x7a6dc, false)?;
    c.check(0x7a6c8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x50)?)?;
    c.check(0x7a6cc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x230)?)?;
    c.check(0x7a6d0)?;
    c.emit(data_register(C::Al, Op::Rsb, r(0), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7a6d4)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 1)?)?;
    c.check(0x7a6d8)?;
    c.jump(C::Al, 0x7a700, false)?;
    c.check(0x7a6dc)?;
    c.literal(r(2), 0x7aa9c)?;
    c.check(0x7a6e0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a6e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x15c)?)?;
    c.check(0x7a6e8)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x1)?)?;
    c.check(0x7a6ec)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.check(0x7a6f0)?;
    c.jump(C::Al, 0x17b0, true)?;
    c.check(0x7a6f4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x50)?)?;
    c.check(0x7a6f8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x230)?)?;
    c.check(0x7a6fc)?;
    c.emit(data_register(C::Al, Op::Rsb, r(1), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7a700)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a704)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a708)
}

fn vui_aspect_ratio(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a708)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a70c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a710)?;
    c.jump(C::Eq, 0x7aa28, false)?;
    c.check(0x7a714)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a718)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a71c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a720)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.check(0x7a724)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a728)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a72c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a730)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(0), 0x1)?)?;
    c.check(0x7a734)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a738)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a73c)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0x4)?)?;
    c.check(0x7a740)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a744)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a748)?;
    c.emit(data_immediate_with_flags(C::Ne, Op::And, r(0), r(0), 0x1, true)?)?;
    c.check(0x7a74c)?;
    c.jump(C::Eq, 0x7a7ac, false)?;
    c.check(0x7a750)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a8)?)?;
    c.check(0x7a754)?;
    c.emit(transfer_word(C::Al, false, r(0), R::SP, 0xc)?)?;
    c.check(0x7a758)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1ac)?)?;
    c.check(0x7a75c)?;
    c.emit(transfer_word(C::Al, false, r(0), R::SP, 0x10)?)?;
    c.check(0x7a760)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), R::SP, 0x8)?)?;
    c.check(0x7a764)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x4)?)?;
    c.check(0x7a768)?;
    c.jump(C::Al, 0x7a08c, true)?;
    c.check(0x7a76c)?;
    c.emit(mov_register(C::Al, r(7), r(0)))?;
    c.check(0x7a770)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a774)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a778)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7a77c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a780)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0xff)?)?;
    c.check(0x7a784)?;
    c.jump(C::Ne, 0x7a7ac, false)?;
    c.check(0x7a788)?;
    c.emit(transfer_word(C::Al, true, r(7), R::SP, 0x4)?)?;
    c.check(0x7a78c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a790)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7a794)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x4)?)?;
    c.check(0x7a798)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a79c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x8)?)?;
    c.check(0x7a7a0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a7a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7a7a8)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a7ac)
}

fn vui_video_signal(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a7ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a7b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a7b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a7b8)?;
    c.emit(transfer_word(C::Al, false, r(10), R::SP, 0x0)?)?;
    c.check(0x7a7bc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7a7c0)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a7c4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a7c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a7cc)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(6), r(0), Shift::Lsr, 1, false)?)?;
    c.check(0x7a7d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a7d4)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a7d8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a7dc)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.check(0x7a7e0)?;
    c.jump(C::Eq, 0x7a8a0, false)?;
    c.check(0x7a7e4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.check(0x7a7e8)?;
    c.emit(mov_register(C::Al, r(10), r(9)))?;
    c.check(0x7a7ec)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1c0)?)?;
    c.check(0x7a7f0)?;
    c.emit(transfer_word(C::Al, true, r(6), r(5), 0x1c4)?)?;
    c.check(0x7a7f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.check(0x7a7f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7a7fc)?;
    c.emit(transfer_byte(C::Al, true, r(9), r(0), 0x143)?)?;
    c.check(0x7a800)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a804)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.check(0x7a808)?;
    c.emit(mov_register(C::Ne, r(1), r(8)))?;
    c.check(0x7a80c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a810)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.check(0x7a814)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a818)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a81c)?;
    c.emit(mov_register(C::Al, r(9), r(10)))?;
    c.check(0x7a820)?;
    c.emit(mov_register(C::Eq, r(7), r(6)))?;
    c.check(0x7a824)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7a828)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a82c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1c8)?)?;
    c.check(0x7a830)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a834)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a838)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a83c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1c8)?)?;
    c.check(0x7a840)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7a844)?;
    c.jump(C::Ne, 0x7a8a0, false)?;
    c.check(0x7a848)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(5), 0x1cc)?)?;
    c.check(0x7a84c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a850)?;
    c.emit(transfer_multiple(C::Al, true, r(9), r(1).bit() | r(7).bit() | r(9).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0x7a854)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.check(0x7a858)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(0), 0x143)?)?;
    c.check(0x7a85c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a860)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7a864)?;
    c.emit(mov_register(C::Ne, r(1), r(8)))?;
    c.check(0x7a868)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a86c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7a870)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a874)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a878)?;
    c.emit(mov_register(C::Ne, r(7), r(8)))?;
    c.check(0x7a87c)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7a880)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a884)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7a888)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a88c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7a890)?;
    c.emit(mov_register(C::Ne, r(9), r(8)))?;
    c.check(0x7a894)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.check(0x7a898)?;
    c.emit(mov_register(C::Al, r(9), r(10)))?;
    c.check(0x7a89c)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a8a0)
}

fn vui_colour_and_timing(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a8a0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a8a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.check(0x7a8a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a8ac)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(6), r(0), Shift::Lsr, 4, false)?)?;
    c.check(0x7a8b0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a8b4)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a8b8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a8bc)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x10)?)?;
    c.check(0x7a8c0)?;
    c.jump(C::Eq, 0x7a8e8, false)?;
    c.check(0x7a8c4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x19a)?)?;
    c.check(0x7a8c8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a8cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7a8d0)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a8d4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x19a)?)?;
    c.check(0x7a8d8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a8dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7a8e0)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a8e4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a8e8)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(6), r(0), Shift::Lsr, 2, false)?)?;
    c.check(0x7a8ec)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a8f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a8f4)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a8f8)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0x4)?)?;
    c.check(0x7a8fc)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 0x0)?)?;
    c.check(0x7a900)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.check(0x7a904)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.check(0x7a908)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7a90c)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a910)?;
    c.emit(data_immediate_with_flags(C::Ne, Op::And, r(0), r(0), 0x4, true)?)?;
    c.check(0x7a914)?;
    c.jump(C::Eq, 0x7a978, false)?;
    c.check(0x7a918)?;
    c.emit(transfer_word(C::Al, true, r(6), R::SP, 0x4)?)?;
    c.check(0x7a91c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a920)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7a924)?;
    c.emit(load_halfword(C::Al, r(1), r(6), 0x2)?)?;
    c.check(0x7a928)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a92c)?;
    c.emit(load_halfword(C::Al, r(1), r(6), 0x0)?)?;
    c.check(0x7a930)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a934)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7a938)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a93c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xba)?)?;
    c.check(0x7a940)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7a944)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x100)?)?;
    c.check(0x7a948)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(5), r(0))?)?;
    c.check(0x7a94c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a950)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a954)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1b8)?)?;
    c.check(0x7a958)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7a95c)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(5), r(0))?)?;
    c.check(0x7a960)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a964)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a968)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1b4)?)?;
    c.check(0x7a96c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a970)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a974)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a978)
}

fn vui_restrictions(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a978)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a97c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a980)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a984)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.check(0x7a988)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a98c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a990)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a994)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a998)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a99c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a9a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7a9a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a9a8)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a9ac)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a9b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a9b4)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(6), r(0), Shift::Lsr, 3, false)?)?;
    c.check(0x7a9b8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a9bc)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a9c0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7a9c4)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x8)?)?;
    c.check(0x7a9c8)?;
    c.jump(C::Eq, 0x7aa38, false)?;
    c.check(0x7a9cc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a9d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7a9d8)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7a9dc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a9e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.check(0x7a9e4)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a9e8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a9ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7a9f0)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7a9f4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7a9f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9)?)?;
    c.check(0x7a9fc)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7aa00)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x8)?)?;
    c.check(0x7aa08)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7aa0c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x11c)?)?;
    c.check(0x7aa10)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa14)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7aa18)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x165)?)?;
    c.check(0x7aa1c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa20)?;
    c.jump(C::Al, 0x79db8, true)?;
    c.check(0x7aa24)?;
    c.jump(C::Al, 0x7aa38, false)?;
    c.check(0x7aa28)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7aa30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7aa34)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7aa38)
}

fn finish_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7aa38)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7aa40)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7aa44)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.check(0x7aa48)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7aa50)?;
    c.jump(C::Al, 0x79c18, true)?;
    c.check(0x7aa54)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aa58)?;
    c.jump(C::Al, 0x79c98, true)?;
    c.check(0x7aa5c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.check(0x7aa60)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(5), 0x16d)?)?;
    c.check(0x7aa64)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7aa68)?;
    c.jump(C::Ne, 0x7aa74, false)?;
    c.check(0x7aa6c)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.check(0x7aa70)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7aa74)
}

fn alternate_epilogue(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7aa74)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.check(0x7aa78)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(8))?)?;
    c.check(0x7aa7c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.check(0x7aa80)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.check(0x7aa84)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.check(0x7aa88)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.check(0x7aa8c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7aa90)?;
    c.jump(C::Al, 0x79b34, false)?;
    c.check(0x7aa94)
}

fn cold_dimension_load(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7aa94)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x10c)?)?;
    c.check(0x7aa98)?;
    c.jump(C::Al, 0x7a59c, false)?;
    c.check(0x7aa9c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    sequence_profile(&mut c)?;
    high_profile(&mut c)?;
    sequence_dimensions(&mut c)?;
    frame_cropping(&mut c)?;
    vui_aspect_ratio(&mut c)?;
    vui_video_signal(&mut c)?;
    vui_colour_and_timing(&mut c)?;
    vui_restrictions(&mut c)?;
    finish_header(&mut c)?;
    alternate_epilogue(&mut c)?;
    cold_dimension_load(&mut c)?;
    c.data(POOL_START, ASSERTION_SOURCE)?;
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
        assert_eq!(bytes.len(), 1716);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(CODE_BYTES + DATA_BYTES, bytes.len());
        assert_eq!(word(&bytes, POOL_START), ASSERTION_SOURCE);
        for (at, condition, target, link) in [
            (0x7a5b4, C::Eq, 0x7aa94, false),
            (0x7aa68, C::Ne, 0x7aa74, false),
            (0x7aa90, C::Al, 0x79b34, false),
            (0x7aa98, C::Al, 0x7a59c, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        let saved = (4..=11).fold(0, |mask, n| mask | r(n).bit());
        for (at, last) in [(0x7aa70, R::PC), (0x7aa8c, r(14))] {
            assert_eq!(word(&bytes, at), transfer_multiple(C::Al, true, R::SP,
                saved | last.bit(), BlockMode::IncrementAfter, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x7aa94),
            transfer_word(C::Al, true, r(0), r(5), 0x10c).unwrap());
        for at in [0x7a67c, 0x7a6dc] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(2), R::PC,
                POOL_START as i32 - at as i32 - 8).unwrap());
        }
    }

    #[test]
    fn exact_guards() {
        for at in (START - 4..=END + 4).step_by(4).chain([START + 1, POOL_START + 1]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(mov_register(C::Al, r(0), r(0))).is_ok(),
                (START..POOL_START).contains(&at) && at % 4 == 0);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.data(at, ASSERTION_SOURCE).is_ok(), at == POOL_START);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, ASSERTION_SOURCE ^ 1).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(POOL_START, ASSERTION_SOURCE).is_err());
        assert!(c.literal(r(2), POOL_START).is_err());
        assert!(c.jump(C::Al, START, false).is_err());
        assert!(c.bytes.is_empty());
        for at in [0x7a67c, 0x7a6dc] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal(r(1), POOL_START).is_err());
            assert!(c.literal(r(2), POOL_START + 4).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: 0x7aa90, bytes: Vec::new() };
        for (condition, target, link) in [
            (C::Ne, 0x79b34, false), (C::Al, 0x79b34, true),
            (C::Al, POOL_START, false), (C::Al, 0x79b35, false),
        ] {
            assert!(c.jump(condition, target, link).is_err());
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn arithmetic_semantics() {
        for (dimension, cropped, half) in [
            (120u32, 1920u32, 0u32), (120, 1918, 1),
            (0, 2, 0x7fff_ffff), (0x1000_0000, 0, 0),
        ] {
            let difference = dimension.wrapping_shl(4).wrapping_sub(cropped);
            assert_eq!(difference >> 1, half);
        }
        for flags in 0u32..32 {
            for bit in [0, 1, 2, 3, 4] {
                assert_eq!(1 & (flags >> bit), u32::from(flags & (1 << bit) != 0));
            }
        }
        assert_eq!(0x11c | 0x400, 0x51c);
        assert_eq!((0u32).wrapping_sub(1), u32::MAX);
        assert_eq!((0x80u8 as i8) as i32, -128);
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
