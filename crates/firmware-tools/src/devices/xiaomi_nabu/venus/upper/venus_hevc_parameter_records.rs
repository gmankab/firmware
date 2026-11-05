use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, load_signed_byte_register, mov_register,
    push_registers, transfer_byte, transfer_multiple, transfer_word,
};

pub const START: usize = 0x7c25c;
pub const END: usize = 0x7c6b8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1116;
pub const DATA_BYTES: usize = 0;
pub const FUNCTION_STARTS: [usize; 4] = [START, 0x7c2e8, 0x7c340, 0x7c51c];

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("HEVC parameter records at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..END).contains(&at) && at % 4 == 0
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("HEVC parameter instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7c290, C::Al, 0x7c2e8, true)
            | (0x7c2a0, C::Al, 0x798f8, true)
            | (0x7c2b0, C::Ne, 0x7c2c4, false)
            | (0x7c2c0, C::Al, 0x798f8, true)
            | (0x7c2cc, C::Al, 0x79db8, true)
            | (0x7c2d8, C::Al, 0x79c18, true)
            | (0x7c2e4, C::Al, 0x79c98, false)
            | (0x7c308, C::Al, 0x798f8, true)
            | (0x7c318, C::Al, 0x798f8, true)
            | (0x7c328, C::Al, 0x798f8, true)
            | (0x7c33c, C::Al, 0x798f8, false)
            | (0x7c360, C::Eq, 0x7c370, false)
            | (0x7c36c, C::Al, 0x7c384, false)
            | (0x7c384, C::Al, 0x79b6c, true)
            | (0x7c398, C::Al, 0x7c2e8, true)
            | (0x7c3a8, C::Al, 0x798f8, true)
            | (0x7c3b8, C::Al, 0x798f8, true)
            | (0x7c3c8, C::Al, 0x798f8, true)
            | (0x7c3dc, C::Al, 0x798f8, true)
            | (0x7c3f4, C::Al, 0x798f8, true)
            | (0x7c408, C::Al, 0x798f8, true)
            | (0x7c414, C::Al, 0x7c51c, true)
            | (0x7c424, C::Al, 0x798f8, true)
            | (0x7c430, C::Ne, 0x7c44c, false)
            | (0x7c43c, C::Al, 0x79db8, true)
            | (0x7c448, C::Al, 0x7c460, false)
            | (0x7c454, C::Al, 0x79db8, true)
            | (0x7c460, C::Al, 0x79db8, true)
            | (0x7c46c, C::Al, 0x79db8, true)
            | (0x7c480, C::Ls, 0x7c428, false)
            | (0x7c490, C::Al, 0x798f8, true)
            | (0x7c49c, C::Al, 0x79db8, true)
            | (0x7c4ac, C::Al, 0x798f8, true)
            | (0x7c4bc, C::Al, 0x798f8, true)
            | (0x7c4cc, C::Al, 0x798f8, true)
            | (0x7c4d8, C::Al, 0x79c18, true)
            | (0x7c4e0, C::Al, 0x79c98, true)
            | (0x7c4f0, C::Ne, 0x7c4f8, false)
            | (0x7c518, C::Al, 0x79b34, false)
            | (0x7c53c, C::Al, 0x798f8, true)
            | (0x7c54c, C::Al, 0x798f8, true)
            | (0x7c55c, C::Al, 0x798f8, true)
            | (0x7c56c, C::Eq, 0x7c584, false)
            | (0x7c580, C::Al, 0x7c5a4, false)
            | (0x7c598, C::Ne, 0x7c5a4, false)
            | (0x7c5ac, C::Al, 0x798f8, true)
            | (0x7c5bc, C::Ne, 0x7c564, false)
            | (0x7c5cc, C::Al, 0x798f8, true)
            | (0x7c5e0, C::Al, 0x798f8, true)
            | (0x7c5f0, C::Al, 0x798f8, true)
            | (0x7c600, C::Al, 0x798f8, true)
            | (0x7c610, C::Al, 0x798f8, true)
            | (0x7c620, C::Al, 0x798f8, true)
            | (0x7c630, C::Al, 0x798f8, true)
            | (0x7c640, C::Al, 0x798f8, true)
            | (0x7c64c, C::Cc, 0x7c6b4, false)
            | (0x7c65c, C::Al, 0x798f8, true)
            | (0x7c66c, C::Al, 0x798f8, true)
            | (0x7c684, C::Gt, 0x7c650, false)
            | (0x7c6a0, C::Al, 0x798f8, true)
            | (0x7c6b0, C::Cc, 0x7c694, false)
        );
        if !valid { return Err("invalid HEVC parameter branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn restore(&mut self, condition: C, registers: u16) -> Result<(), String> {
        let valid = match (self.at, condition) {
            (0x7c2e0, C::Al) => registers == (r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit()),
            (0x7c338, C::Al) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit()),
            (0x7c4f4, C::Al) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit()),
            (0x7c514, C::Al) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit()),
            (0x7c690, C::Hi) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit()),
            (0x7c6b4, C::Al) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit()),
            _ => false,
        };
        if !valid { return Err("invalid HEVC parameter restore".into()); }
        self.emit(transfer_multiple(condition, true, R::SP, registers,
            BlockMode::IncrementAfter, true)?)
    }
}

fn write_parameter_record(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c25c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7c260)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.check(0x7c264)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7c268)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7c26c)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7c270)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xc)?)?;
    c.check(0x7c274)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.check(0x7c278)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x20)?)?;
    c.check(0x7c27c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0xa)?)?;
    c.check(0x7c280)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7c284)?;
    c.emit(transfer_byte(C::Eq, true, r(0), r(5), 0x2)?)?;
    c.check(0x7c288)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c28c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x280)?)?;
    c.check(0x7c290)?;
    c.branch(C::Al, 0x7c2e8, true)?;
    c.check(0x7c294)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c298)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c29c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c2a0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c2a4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x280)?)?;
    c.check(0x7c2a8)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x7)?)?;
    c.check(0x7c2ac)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x10)?)?;
    c.check(0x7c2b0)?;
    c.branch(C::Ne, 0x7c2c4, false)?;
    c.check(0x7c2b4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c2b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c2bc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c2c0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c2c4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c2c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c2cc)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c2d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c2d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c2d8)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7c2dc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c2e0)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?;
    c.check(0x7c2e4)?;
    c.branch(C::Al, 0x79c98, false)?;
    Ok(())
}

fn write_nal_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c2e8)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7c2ec)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.check(0x7c2f0)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7c2f4)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7c2f8)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7c2fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c300)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c304)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c308)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c30c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c310)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7c314)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7c318)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c31c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c320)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c324)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7c328)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c32c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x1)?)?;
    c.check(0x7c330)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c334)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7c338)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.check(0x7c33c)?;
    c.branch(C::Al, 0x798f8, false)?;
    Ok(())
}

fn write_video_parameter_set(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c340)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7c344)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7c348)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7c34c)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7c350)?;
    c.emit(mov_register(C::Al, r(7), r(2)))?;
    c.check(0x7c354)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x82c)?)?;
    c.check(0x7c358)?;
    c.emit(transfer_word(C::Al, true, r(8), r(5), 0x20)?)?;
    c.check(0x7c35c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c360)?;
    c.branch(C::Eq, 0x7c370, false)?;
    c.check(0x7c364)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c368)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c36c)?;
    c.branch(C::Al, 0x7c384, false)?;
    c.check(0x7c370)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7c374)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7c378)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7c37c)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(1))?)?;
    c.check(0x7c380)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c384)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7c388)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.check(0x7c38c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c390)?;
    c.emit(mov_register(C::Al, r(2), r(5)))?;
    c.check(0x7c394)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7c398)?;
    c.branch(C::Al, 0x7c2e8, true)?;
    c.check(0x7c39c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c3a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c3a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.check(0x7c3a8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c3ac)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c3b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3)?)?;
    c.check(0x7c3b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7c3b8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c3bc)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c3c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c3c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7c3c8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c3cc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x71b)?)?;
    c.check(0x7c3d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7c3d4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7c3d8)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c3dc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c3e0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x71b)?)?;
    c.check(0x7c3e4)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c3e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c3ec)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7c3f0)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c3f4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c3f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.check(0x7c3fc)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c400)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7c404)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xff00)?)?;
    c.check(0x7c408)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c40c)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7c410)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7c414)?;
    c.branch(C::Al, 0x7c51c, true)?;
    c.check(0x7c418)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c41c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c420)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c424)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c428)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x1e8)?)?;
    c.check(0x7c42c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xc)?)?;
    c.check(0x7c430)?;
    c.branch(C::Ne, 0x7c44c, false)?;
    c.check(0x7c434)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c438)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c43c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c440)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c444)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c448)?;
    c.branch(C::Al, 0x7c460, false)?;
    c.check(0x7c44c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x165)?)?;
    c.check(0x7c450)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c454)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c458)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x719)?)?;
    c.check(0x7c45c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c460)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c464)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c468)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c46c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c470)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x71b)?)?;
    c.check(0x7c474)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.check(0x7c478)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.check(0x7c47c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c480)?;
    c.branch(C::Ls, 0x7c428, false)?;
    c.check(0x7c484)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c488)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c48c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7c490)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c494)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c498)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c49c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c4a0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c4a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c4a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c4ac)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c4b0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c4b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c4b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c4bc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c4c0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c4c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c4c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c4cc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c4d0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c4d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c4d8)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7c4dc)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7c4e0)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7c4e4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7c4e8)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(6), 0x16d)?)?;
    c.check(0x7c4ec)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c4f0)?;
    c.branch(C::Ne, 0x7c4f8, false)?;
    c.check(0x7c4f4)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x7c4f8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7c4fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7c500)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7c504)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(1))?)?;
    c.check(0x7c508)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x20)?)?;
    c.check(0x7c50c)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.check(0x7c510)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.check(0x7c514)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?;
    c.check(0x7c518)?;
    c.branch(C::Al, 0x79b34, false)?;
    Ok(())
}

fn write_profile_tier_level(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c51c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7c520)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.check(0x7c524)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.check(0x7c528)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0x7c52c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c530)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7c534)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7c538)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c53c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c540)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c544)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c548)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c54c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c550)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x8)?)?;
    c.check(0x7c554)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c558)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5)?)?;
    c.check(0x7c55c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c560)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xff)?)?;
    c.check(0x7c564)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x82c)?)?;
    c.check(0x7c568)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c56c)?;
    c.branch(C::Eq, 0x7c584, false)?;
    c.check(0x7c570)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(7), 0xff)?)?;
    c.check(0x7c574)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c578)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7c57c)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c580)?;
    c.branch(C::Al, 0x7c5a4, false)?;
    c.check(0x7c584)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8)?)?;
    c.check(0x7c588)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c58c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.check(0x7c590)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c594)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x2)?)?;
    c.check(0x7c598)?;
    c.branch(C::Ne, 0x7c5a4, false)?;
    c.check(0x7c59c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7c5a0)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c5a4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c5a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c5ac)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c5b0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7c5b4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7c5b8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x20)?)?;
    c.check(0x7c5bc)?;
    c.branch(C::Ne, 0x7c564, false)?;
    c.check(0x7c5c0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c5c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c5c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c5cc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c5d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c5d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c5d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c5dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7c5e0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c5e4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c5e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c5ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c5f0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c5f4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c5f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c5fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c600)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c604)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c608)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c60c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7c610)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c614)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c618)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c61c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7c620)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c624)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c628)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c62c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xc)?)?;
    c.check(0x7c630)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c634)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0xc)?)?;
    c.check(0x7c638)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c63c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7c640)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c644)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x71b)?)?;
    c.check(0x7c648)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7c64c)?;
    c.branch(C::Cc, 0x7c6b4, false)?;
    c.check(0x7c650)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c654)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c658)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c65c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c660)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c664)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c668)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c66c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c670)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x71b)?)?;
    c.check(0x7c674)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7c678)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(7), r(0), 0x1)?)?;
    c.check(0x7c67c)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(6), 0xff)?)?;
    c.check(0x7c680)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c684)?;
    c.branch(C::Gt, 0x7c650, false)?;
    c.check(0x7c688)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(7), 0xff)?)?;
    c.check(0x7c68c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x7)?)?;
    c.check(0x7c690)?;
    c.restore(C::Hi, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x7c694)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c698)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c69c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7c6a0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c6a4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7c6a8)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(7), 0xff)?)?;
    c.check(0x7c6ac)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x8)?)?;
    c.check(0x7c6b0)?;
    c.branch(C::Cc, 0x7c694, false)?;
    c.check(0x7c6b4)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit())?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    write_parameter_record(&mut c)?;
    write_nal_header(&mut c)?;
    write_video_parameter_set(&mut c)?;
    write_profile_tier_level(&mut c)?;
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
        assert_eq!((START, END), (0x7c25c, 0x7c6b8));
        assert_eq!(bytes.len(), 1116);
        assert_eq!((CODE_BYTES, DATA_BYTES), (1116, 0));
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(FUNCTION_STARTS, [START, 0x7c2e8, 0x7c340, 0x7c51c]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 279);
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        for (writer, end) in [
            (write_parameter_record as fn(&mut TypedA32) -> Result<(), String>, 0x7c2e8),
            (write_nal_header, 0x7c340),
            (write_video_parameter_set, 0x7c51c),
            (write_profile_tier_level, END),
        ] {
            writer(&mut c).unwrap();
            c.check(end).unwrap();
        }
        assert_eq!(c.bytes, bytes);
        let mut branches = 0;
        let mut calls = 0;
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            if (instruction >> 25) & 7 == 5 {
                branches += 1;
                calls += usize::from(instruction & (1 << 24) != 0);
            }
        }
        assert_eq!((branches, calls), (61, 44));
        for (at, target) in [(0x7c2e4, 0x79c98), (0x7c33c, 0x798f8), (0x7c518, 0x79b34)] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, false).unwrap());
        }
        for (at, condition, registers) in [
            (0x7c4f4, C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | R::R11.bit() | R::PC.bit()),
            (0x7c690, C::Hi, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | R::R11.bit() | R::PC.bit()),
            (0x7c6b4, C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | R::R11.bit() | R::PC.bit()),
        ] {
            assert_eq!(word(&bytes, at), transfer_multiple(condition, true, R::SP, registers,
                BlockMode::IncrementAfter, true).unwrap());
        }
    }

    #[test]
    fn exact_guards() {
        let bytes = fragment().unwrap();
        let conditions = [C::Eq, C::Ne, C::Cs, C::Cc, C::Mi, C::Pl, C::Hi, C::Ls,
            C::Ge, C::Lt, C::Gt, C::Le, C::Al];
        let mut accepted_branches = 0;
        let mut accepted_restores = 0;
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            c.emit(mov_register(C::Al, r(0), r(0))).unwrap();
            assert_eq!(c.bytes.len(), 4);
            let instruction = word(&bytes, at);
            let is_branch = (instruction >> 25) & 7 == 5;
            let displacement = ((instruction << 8) as i32 >> 6) as isize;
            let target = (at as isize + 8 + displacement) as usize;
            for condition in conditions {
                for link in [false, true] {
                    let expected = is_branch && instruction >> 28 == condition as u32
                        && (instruction & (1 << 24) != 0) == link;
                    let mut c = TypedA32 { at, bytes: Vec::new() };
                    assert_eq!(c.branch(condition, target, link).is_ok(), expected);
                    if expected {
                        accepted_branches += 1;
                        assert_eq!(c.bytes, instruction.to_le_bytes());
                    } else {
                        assert!(c.bytes.is_empty());
                    }
                    for invalid in [target.wrapping_add(4), START + 1, END, usize::MAX] {
                        let mut c = TypedA32 { at, bytes: Vec::new() };
                        assert!(c.branch(condition, invalid, link).is_err());
                        assert!(c.bytes.is_empty());
                    }
                }
                let registers = instruction as u16;
                let expected = [0x7c2e0, 0x7c338, 0x7c4f4, 0x7c514, 0x7c690, 0x7c6b4].contains(&at)
                    && instruction >> 28 == condition as u32;
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.restore(condition, registers).is_ok(), expected);
                if expected {
                    accepted_restores += 1;
                    assert_eq!(c.bytes, instruction.to_le_bytes());
                } else {
                    assert!(c.bytes.is_empty());
                }
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.restore(condition, registers ^ r(0).bit()).is_err());
                assert!(c.bytes.is_empty());
            }
        }
        assert_eq!((accepted_branches, accepted_restores), (61, 6));
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.branch(C::Al, 0x798f8, true).is_err());
            assert!(c.restore(C::Al, R::PC.bit()).is_err());
            assert!(c.check(START).is_err());
            assert!(c.bytes.is_empty());
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
