use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch, bx,
    data_immediate, data_register, load_signed_byte_register, mov_register,
    mov_shifted_register, push_registers, transfer_byte, transfer_byte_register,
    transfer_multiple, transfer_word,
};

pub const START: usize = 0x7b65c;
pub const END: usize = 0x7ba5c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1024;
pub const DATA_BYTES: usize = 0;
pub const FUNCTION_STARTS: [usize; 6] = [START, 0x7b6dc, 0x7b7ac, 0x7b828, 0x7b8bc, 0x7ba08];

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("HEVC state writer at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..END).contains(&at) && at % 4 == 0
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("HEVC state instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7b664, C::Ne, 0x7b6a4, false)
            | (0x7b678, C::Eq, 0x7b69c, false)
            | (0x7b684, C::Ne, 0x7b6b8, false)
            | (0x7b6a0, C::Al, 0x7b6b8, false)
            | (0x7b70c, C::Al, 0x79b6c, true)
            | (0x7b71c, C::Al, 0x798f8, true)
            | (0x7b72c, C::Al, 0x798f8, true)
            | (0x7b73c, C::Al, 0x798f8, true)
            | (0x7b750, C::Al, 0x798f8, true)
            | (0x7b75c, C::Ne, 0x7b744, false)
            | (0x7b76c, C::Al, 0x798f8, true)
            | (0x7b778, C::Al, 0x79c18, true)
            | (0x7b780, C::Al, 0x79c98, true)
            | (0x7b7a8, C::Al, 0x79b34, false)
            | (0x7b7c8, C::Al, 0x79df0, true)
            | (0x7b7d4, C::Al, 0x79db8, true)
            | (0x7b7e0, C::Eq, 0x7b804, false)
            | (0x7b7f0, C::Al, 0x79df0, true)
            | (0x7b800, C::Al, 0x79df0, true)
            | (0x7b818, C::Al, 0x79c18, true)
            | (0x7b824, C::Al, 0x79c98, false)
            | (0x7b854, C::Al, 0x79b6c, true)
            | (0x7b864, C::Al, 0x798f8, true)
            | (0x7b87c, C::Al, 0x798f8, true)
            | (0x7b888, C::Al, 0x79c18, true)
            | (0x7b890, C::Al, 0x79c98, true)
            | (0x7b8b8, C::Al, 0x79b34, false)
            | (0x7b8e8, C::Al, 0x79b6c, true)
            | (0x7b8f4, C::Eq, 0x7b90c, false)
            | (0x7b908, C::Al, 0x7b910, false)
            | (0x7b918, C::Al, 0x798f8, true)
            | (0x7b928, C::Al, 0x798f8, true)
            | (0x7b938, C::Al, 0x798f8, true)
            | (0x7b948, C::Al, 0x798f8, true)
            | (0x7b958, C::Al, 0x798f8, true)
            | (0x7b968, C::Al, 0x798f8, true)
            | (0x7b978, C::Al, 0x798f8, true)
            | (0x7b988, C::Al, 0x798f8, true)
            | (0x7b998, C::Al, 0x798f8, true)
            | (0x7b9a8, C::Al, 0x798f8, true)
            | (0x7b9b8, C::Al, 0x798f8, true)
            | (0x7b9c8, C::Al, 0x798f8, true)
            | (0x7b9d4, C::Al, 0x79c18, true)
            | (0x7b9dc, C::Al, 0x79c98, true)
            | (0x7ba04, C::Al, 0x79b34, false)
            | (0x7ba14, C::Eq, 0x7ba58, false)
            | (0x7ba54, C::Cc, 0x7ba40, false)
        );
        if !valid { return Err("invalid HEVC state branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn update_picture_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b65c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x1)?)?;
    c.check(0x7b660)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0x7b664)?;
    c.branch(C::Ne, 0x7b6a4, false)?;
    c.check(0x7b668)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b66c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x134)?)?;
    c.check(0x7b670)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 0x6)?)?;
    c.check(0x7b674)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.check(0x7b678)?;
    c.branch(C::Eq, 0x7b69c, false)?;
    c.check(0x7b67c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x1dd)?)?;
    c.check(0x7b680)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7b684)?;
    c.branch(C::Ne, 0x7b6b8, false)?;
    c.check(0x7b688)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x138)?)?;
    c.check(0x7b68c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xff)?)?;
    c.check(0x7b690)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0xff00)?)?;
    c.check(0x7b694)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.check(0x7b698)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.check(0x7b69c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x138)?)?;
    c.check(0x7b6a0)?;
    c.branch(C::Al, 0x7b6b8, false)?;
    c.check(0x7b6a4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x130)?)?;
    c.check(0x7b6a8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0x7b6ac)?;
    c.emit(transfer_word(C::Eq, true, r(1), r(0), 0x134)?)?;
    c.check(0x7b6b0)?;
    c.emit(data_immediate(C::Eq, Op::Add, r(1), r(1), 0x1)?)?;
    c.check(0x7b6b4)?;
    c.emit(transfer_word(C::Eq, false, r(1), r(0), 0x134)?)?;
    c.check(0x7b6b8)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x2)?)?;
    c.check(0x7b6bc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1e)?)?;
    c.check(0x7b6c0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0x7b6c4)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b6c8)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0x130)?)?;
    c.check(0x7b6cc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x26c)?)?;
    c.check(0x7b6d0)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(2), r(1), Shift::Lsl, 1, false)?)?;
    c.check(0x7b6d4)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x13c)?)?;
    c.check(0x7b6d8)?;
    c.emit(bx(C::Al, r(14)))?;
    c.check(0x7b6dc)
}

fn write_filler_payload(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b6dc)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b6e0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7b6e4)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7b6e8)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7b6ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7b6f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.check(0x7b6f4)?;
    c.emit(transfer_word(C::Al, true, r(4), r(5), 0x20)?)?;
    c.check(0x7b6f8)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 0xc)?)?;
    c.check(0x7b6fc)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.check(0x7b700)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7b704)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(8))?)?;
    c.check(0x7b708)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b70c)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7b710)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b714)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6)?)?;
    c.check(0x7b718)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b71c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b720)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b724)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3f)?)?;
    c.check(0x7b728)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b72c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b730)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b734)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x10)?)?;
    c.check(0x7b738)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b73c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b740)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7b744)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b748)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.check(0x7b74c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b750)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b754)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7b758)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x10)?)?;
    c.check(0x7b75c)?;
    c.branch(C::Ne, 0x7b744, false)?;
    c.check(0x7b760)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b764)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b768)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b76c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b770)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b774)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b778)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b77c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b780)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7b784)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x16d)?)?;
    c.check(0x7b788)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b78c)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b790)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7b794)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(8))?)?;
    c.check(0x7b798)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x20)?)?;
    c.check(0x7b79c)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.check(0x7b7a0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b7a4)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b7a8)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(0x7b7ac)
}

fn write_quantizer_parameters(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b7ac)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b7b0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.check(0x7b7b4)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0x7b7b8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(1), 0x1)?)?;
    c.check(0x7b7bc)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7b7c0)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x14)?)?;
    c.check(0x7b7c4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b7c8)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7b7cc)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2c4)?)?;
    c.check(0x7b7d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b7d4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b7d8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2c4)?)?;
    c.check(0x7b7dc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b7e0)?;
    c.branch(C::Eq, 0x7b804, false)?;
    c.check(0x7b7e4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2c8)?)?;
    c.check(0x7b7e8)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 1)?)?;
    c.check(0x7b7ec)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b7f0)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7b7f4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2cc)?)?;
    c.check(0x7b7f8)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 1)?)?;
    c.check(0x7b7fc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b800)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7b804)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0xa8)?)?;
    c.check(0x7b808)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b80c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b810)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b814)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b818)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b81c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b820)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b824)?;
    c.branch(C::Al, 0x79c98, false)?;
    c.check(0x7b828)
}

fn write_picture_delimiter(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b828)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b82c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7b830)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7b834)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.check(0x7b838)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7b83c)?;
    c.emit(mov_register(C::Al, r(7), r(1)))?;
    c.check(0x7b840)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7b844)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.check(0x7b848)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0x20)?)?;
    c.check(0x7b84c)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(8))?)?;
    c.check(0x7b850)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b854)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7b858)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b85c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9)?)?;
    c.check(0x7b860)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b864)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b868)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x20)?)?;
    c.check(0x7b86c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x10)?)?;
    c.check(0x7b870)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b874)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(0), Shift::Lsl, 5, false)?)?;
    c.check(0x7b878)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b87c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b880)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b884)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b888)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b88c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b890)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7b894)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x16d)?)?;
    c.check(0x7b898)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b89c)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b8a0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7b8a4)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(8))?)?;
    c.check(0x7b8a8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.check(0x7b8ac)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.check(0x7b8b0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7b8b4)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b8b8)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(0x7b8bc)
}

fn write_extension_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b8bc)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7b8c0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7b8c4)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7b8c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.check(0x7b8cc)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7b8d0)?;
    c.emit(mov_register(C::Al, r(7), r(1)))?;
    c.check(0x7b8d4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7b8d8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.check(0x7b8dc)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0x20)?)?;
    c.check(0x7b8e0)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(8))?)?;
    c.check(0x7b8e4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b8e8)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7b8ec)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0x2)?)?;
    c.check(0x7b8f0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b8f4)?;
    c.branch(C::Eq, 0x7b90c, false)?;
    c.check(0x7b8f8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0x1)?)?;
    c.check(0x7b8fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4e)?)?;
    c.check(0x7b900)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7b904)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x6e)?)?;
    c.check(0x7b908)?;
    c.branch(C::Al, 0x7b910, false)?;
    c.check(0x7b90c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe)?)?;
    c.check(0x7b910)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b914)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b918)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b91c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b920)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b924)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b928)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b92c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 0x1)?)?;
    c.check(0x7b930)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b934)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b938)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b93c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 0x64)?)?;
    c.check(0x7b940)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b944)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7b948)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b94c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b950)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b954)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b958)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b95c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b960)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b964)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7b968)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b96c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b970)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b974)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.check(0x7b978)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b97c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 0xa)?)?;
    c.check(0x7b980)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b984)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7b988)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b98c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b990)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b994)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b998)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b99c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b9a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b9a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b9a8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b9ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b9b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b9b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b9b8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b9bc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b9c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3)?)?;
    c.check(0x7b9c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7b9c8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b9cc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b9d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b9d4)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b9d8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b9dc)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7b9e0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x16d)?)?;
    c.check(0x7b9e4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7b9e8)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b9ec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7b9f0)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(8))?)?;
    c.check(0x7b9f4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.check(0x7b9f8)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.check(0x7b9fc)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7ba00)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7ba04)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(0x7ba08)
}

fn count_nonzero_bytes(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7ba08)?;
    c.emit(mov_register(C::Al, r(2), r(0)))?;
    c.check(0x7ba0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7ba10)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7ba14)?;
    c.branch(C::Eq, 0x7ba58, false)?;
    c.check(0x7ba18)?;
    c.emit(transfer_byte(C::Al, true, r(12), r(2), 0x0)?)?;
    c.check(0x7ba1c)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(2), 0x1)?)?;
    c.check(0x7ba20)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(2), 0x2)?)?;
    c.check(0x7ba24)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.check(0x7ba28)?;
    c.emit(data_register(C::Al, Op::Cmn, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7ba2c)?;
    c.emit(bx(C::Eq, r(14)))?;
    c.check(0x7ba30)?;
    c.emit(data_register(C::Al, Op::Add, r(12), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7ba34)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0xb)?)?;
    c.check(0x7ba38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7ba3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.check(0x7ba40)?;
    c.emit(transfer_byte_register(C::Al, true, r(1), r(2), r(3), Shift::Lsl, 0)?)?;
    c.check(0x7ba44)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.check(0x7ba48)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7ba4c)?;
    c.emit(data_immediate(C::Ne, Op::Add, r(0), r(0), 0x1)?)?;
    c.check(0x7ba50)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.check(0x7ba54)?;
    c.branch(C::Cc, 0x7ba40, false)?;
    c.check(0x7ba58)?;
    c.emit(bx(C::Al, r(14)))?;
    c.check(0x7ba5c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    update_picture_state(&mut c)?;
    write_filler_payload(&mut c)?;
    write_quantizer_parameters(&mut c)?;
    write_picture_delimiter(&mut c)?;
    write_extension_header(&mut c)?;
    count_nonzero_bytes(&mut c)?;
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
    fn structure_and_actual_counts() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 1024);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(DATA_BYTES, 0);
        assert_eq!(FUNCTION_STARTS, [0x7b65c, 0x7b6dc, 0x7b7ac, 0x7b828, 0x7b8bc, 0x7ba08]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 256);
        let mut calls = 0;
        let mut internal = 0;
        let mut tails = 0;
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            if (instruction >> 25) & 7 != 5 { continue; }
            let displacement = ((instruction << 8) as i32 >> 6) as isize;
            let target = (at as isize + 8 + displacement) as usize;
            let link = instruction & (1 << 24) != 0;
            if link {
                calls += 1;
                assert!([0x798f8, 0x79b6c, 0x79c18, 0x79c98, 0x79db8, 0x79df0].contains(&target));
            } else if (START..END).contains(&target) {
                internal += 1;
                assert!(TypedA32::is_code(target));
            } else {
                tails += 1;
                assert!([(0x7b7a8, 0x79b34), (0x7b824, 0x79c98),
                    (0x7b8b8, 0x79b34), (0x7ba04, 0x79b34)].contains(&(at, target)));
            }
        }
        assert_eq!((calls, internal, tails), (33, 10, 4));
        for at in [0x7b6d8, 0x7ba58] {
            assert_eq!(word(&bytes, at), bx(C::Al, R::LR));
        }
        assert_eq!(word(&bytes, 0x7ba2c), bx(C::Eq, R::LR));
        assert_eq!(word(&bytes, 0x7ba40), transfer_byte_register(C::Al, true,
            r(1), r(2), r(3), Shift::Lsl, 0).unwrap());
        assert_eq!(word(&bytes, 0x7ba4c), data_immediate(C::Ne, Op::Add, r(0), r(0), 1).unwrap());
        assert_eq!(word(&bytes, 0x7ba34), data_immediate(C::Al, Op::Add, r(2), r(2), 11).unwrap());
        for value in [0u32, 1, 0x7fff, 0xffff, u32::MAX] {
            assert_eq!(value.wrapping_add(1) & 0xffff, (value & 0xffff).wrapping_add(1) & 0xffff);
            assert_eq!((value << 1) & 0x1e, (value & 15) << 1);
        }
    }

    #[test]
    fn exact_code_and_branch_guards() {
        let bytes = fragment().unwrap();
        let conditions = [C::Eq, C::Ne, C::Cs, C::Cc, C::Mi, C::Pl, C::Hi,
            C::Ls, C::Ge, C::Lt, C::Gt, C::Le, C::Al];
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            c.emit(mov_register(C::Al, r(0), r(0))).unwrap();
            assert_eq!(c.at, at + 4);
            let instruction = word(&bytes, at);
            let is_branch = (instruction >> 25) & 7 == 5;
            let displacement = ((instruction << 8) as i32 >> 6) as isize;
            let target = (at as isize + 8 + displacement) as usize;
            for condition in conditions {
                for link in [false, true] {
                    for candidate in [target, target.wrapping_add(4), START + 1, END] {
                        let mut c = TypedA32 { at, bytes: Vec::new() };
                        let expected = is_branch && candidate == target
                            && (condition as u32) == instruction >> 28
                            && link == (instruction & (1 << 24) != 0);
                        assert_eq!(c.branch(condition, candidate, link).is_ok(), expected);
                        assert_eq!(c.bytes.len(), if expected { 4 } else { 0 });
                        assert_eq!(c.at, at + if expected { 4 } else { 0 });
                    }
                }
            }
        }
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(!TypedA32::is_code(at));
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.branch(C::Al, 0x79b34, false).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        let c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
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
