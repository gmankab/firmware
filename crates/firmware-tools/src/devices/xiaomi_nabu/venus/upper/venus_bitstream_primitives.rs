use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register,
    data_register_shifted_by_register, transfer_byte, transfer_byte_postindexed,
    transfer_doubleword, transfer_multiple, transfer_word, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x798c8;
pub const END: usize = 0x79e48;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1400;
pub const DATA_BYTES: usize = 8;
pub const FUNCTION_STARTS: [usize; 10] = [
    0x798c8, 0x798f8, 0x799a4, 0x79b34, 0x79b6c,
    0x79bd4, 0x79c18, 0x79c98, 0x79db8, 0x79df0,
];

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("bitstream address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0x79c94 | 0x79db4 => Some(0xf41db),
            _ => None,
        }
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && (START..END).contains(&at) && Self::data_value(at).is_none()
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("bitstream instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) {
            return Err("bitstream protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: u8, target: usize) -> Result<(), String> {
        if !matches!((self.at, rd, target),
            (0x79c68, 2, 0x79c94)
            | (0x79ca4, 2, 0x79db4)
            | (0x79d74, 2, 0x79db4)
        ) || Self::data_value(target).is_none() {
            return Err("bitstream invalid literal".into());
        }
        self.emit(transfer_word(C::Al, true, r(rd), R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if !matches!((self.at, condition, target, link),
            (0x79914, C::Lt, 0x79924, false)
            | (0x7991c, C::Al, 0x799a4, true)
            | (0x799b8, C::Eq, 0x79a70, false)
            | (0x799e8, C::Eq, 0x799f4, false)
            | (0x799f0, C::Al, 0x79a38, false)
            | (0x79a00, C::Lt, 0x79a28, false)
            | (0x79a0c, C::Hi, 0x79a34, false)
            | (0x79a24, C::Al, 0x79a48, false)
            | (0x79a54, C::Ne, 0x799e0, false)
            | (0x79a6c, C::Al, 0x79a90, false)
            | (0x79b4c, C::Ne, 0x79b58, false)
            | (0x79b54, C::Al, 0x79b64, false)
            | (0x79bf0, C::Eq, 0x79c04, false)
            | (0x79bfc, C::Al, 0x798f8, true)
            | (0x79c14, C::Al, 0x799a4, false)
            | (0x79c34, C::Eq, 0x79c78, false)
            | (0x79c3c, C::Eq, 0x79c54, false)
            | (0x79c44, C::Ne, 0x79c68, false)
            | (0x79c50, C::Al, 0x79c5c, false)
            | (0x79c60, C::Al, 0x798f8, true)
            | (0x79c64, C::Al, 0x79c78, false)
            | (0x79c74, C::Al, 0x17b0, true)
            | (0x79c90, C::Al, 0x799a4, false)
            | (0x79cc0, C::Al, 0x17b0, true)
            | (0x79cd8, C::Eq, 0x79d74, false)
            | (0x79cfc, C::Eq, 0x79d4c, false)
            | (0x79d08, C::Hi, 0x79d3c, false)
            | (0x79d10, C::Ne, 0x79d3c, false)
            | (0x79d38, C::Al, 0x79d4c, false)
            | (0x79d70, C::Ne, 0x79cf0, false)
            | (0x79d88, C::Al, 0x17b0, true)
            | (0x79dc8, C::Eq, 0x79de8, false)
            | (0x79de4, C::Ne, 0x79dd4, false)
            | (0x79dec, C::Al, 0x798f8, false)
            | (0x79e20, C::Eq, 0x79e40, false)
            | (0x79e3c, C::Ne, 0x79e2c, false)
            | (0x79e44, C::Al, 0x798f8, false)
        ) {
            return Err("bitstream invalid branch".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn initialize_writer(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x798c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 24)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 16)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(0), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 28)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 32)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 36)?)?;
    c.emit(bx(C::Al, r(14)))?;
    Ok(())
}

fn append_bits(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x798f8)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x20)?)?;
    c.branch(C::Lt, 0x79924, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x799a4, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(7), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(12), r(7), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(5), r(0), 0x20)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(8), r(4), 0)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, r(7), false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(3), r(12), 0x20)?)?;
    c.emit(data_immediate(C::Ge, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(6), r(7), 0x20)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsr, r(3), false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_register_shifted_by_register(C::Ge, Op::Mov, r(3), r(0), r(1), Shift::Lsl, r(6), false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, r(12), false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(2), r(0), r(3), Shift::Lsr, r(0), false)?)?;
    c.emit(data_immediate(C::Ge, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(6), r(0), 0x20)?)?;
    c.emit(data_immediate(C::Ge, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(2), r(2), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsr, r(0), false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr, r(1), r(1), r(3), Shift::Lsl, r(6), false)?)?;
    c.emit(data_register_shifted_by_register(C::Ge, Op::Mov, r(1), r(0), r(3), Shift::Lsr, r(5), false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(4), 0x0006, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 8)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn flush_word(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x799a4)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x79a70, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(1), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x7)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 16)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Tst, r(0), r(4), r(9), Shift::Lsl, r(1), false)?)?;
    c.branch(C::Eq, 0x799f4, false)?;
    c.emit(transfer_word(C::Al, false, r(14), r(0), 20)?)?;
    c.branch(C::Al, 0x79a38, false)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 20)?)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x2)?)?;
    c.branch(C::Lt, 0x79a28, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x3)?)?;
    c.branch(C::Hi, 0x79a34, false)?;
    c.emit(transfer_byte(C::Al, false, r(8), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(0), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(0), 24)?)?;
    c.branch(C::Al, 0x79a48, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, r(7), r(4), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 20)?)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(4), r(2), -1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(12), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(3), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), r(3), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x4)?)?;
    c.branch(C::Ne, 0x799e0, false)?;
    c.emit(transfer_doubleword(C::Al, true, r(4), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(13), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(14), r(13), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(8), r(13), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(13), 3)?)?;
    c.branch(C::Al, 0x79a90, false)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(1), Shift::Lsr, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsr, 16, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsr, 24, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(7), r(0), 0x1f)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(12), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(7), r(12), Shift::Lsl, 3, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsl, r(2), false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Ge, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(3), r(3), r(12), Shift::Lsl, 3, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsr, r(3), false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Orr, r(2), r(3), r(5), Shift::Lsl, r(2), false)?)?;
    c.emit(data_register_shifted_by_register(C::Ge, Op::Mov, r(2), r(0), r(4), Shift::Lsl, r(7), false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 8)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(2), r(12), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 16)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsl, r(12), false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(8), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(14), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x18)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn write_length_prefix(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79b34)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsr, 24, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsr, 16, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(0), 1)?)?;
    c.branch(C::Ne, 0x79b58, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x2)?)?;
    c.branch(C::Al, 0x79b64, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsr, 8, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(0), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x3)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(bx(C::Al, r(14)))?;
    Ok(())
}

fn write_start_code(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79b6c)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(3), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(3), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(3), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(3), 0)?)?;
    c.emit(transfer_word(C::Ne, true, r(2), r(0), 32)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, r(3), r(2), 0x1)?)?;
    c.emit(transfer_word(C::Ne, false, r(3), r(0), 32)?)?;
    c.emit(transfer_byte(C::Ne, false, r(12), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(2), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x20)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 36)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(2), r(0), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 36)?)?;
    c.emit(bx(C::Al, r(14)))?;
    Ok(())
}

fn align_ones(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79bd4)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x7)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(1), r(1), 0x7)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(1), r(0), Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x79c04, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x20)?)?;
    c.emit(transfer_multiple(C::Lt, true, r(13), 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x4890, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x799a4, false)?;
    Ok(())
}

fn align_with_fill(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79c18)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x7)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(2), r(2), 0x7)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(5), r(2), r(0), Shift::Lsl, 0, true)?)?;
    c.branch(C::Eq, 0x79c78, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Eq, 0x79c54, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x79c68, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x79c5c, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.branch(C::Al, 0x79c78, false)?;
    c.literal(2, 0x79c94)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xd9)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, false, r(5), r(4), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x20)?)?;
    c.emit(transfer_multiple(C::Lt, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x4830, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x799a4, false)?;
    c.data(0x79c94, 0xf41db)?;
    Ok(())
}

fn flush_remaining_bytes(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79c98)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.literal(2, 0x79db4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xfb)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x7)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Asr, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(8), r(0), Shift::Asr, 3, false)?)?;
    c.branch(C::Eq, 0x79d74, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 12)?)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(5), r(0), -1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x79d4c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x3)?)?;
    c.branch(C::Hi, 0x79d3c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x79d3c, false)?;
    c.emit(transfer_word(C::Al, false, r(12), r(4), 20)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(14), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 24)?)?;
    c.branch(C::Al, 0x79d4c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, r(2), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 32)?)?;
    c.emit(transfer_byte(C::Al, false, r(5), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(5), r(1), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(4), 8)?)?;
    c.branch(C::Ne, 0x79cf0, false)?;
    c.literal(2, 0x79db4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x120)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, false, r(8), r(4), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(4), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(4), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(4), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(4), 20)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(7), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 36)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 36)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    c.data(0x79db4, 0xf41db)?;
    Ok(())
}

fn write_unsigned_exp_golomb(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79db8)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(2), r(0), 0x0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Add, r(1), r(1), 0x1, true)?)?;
    c.branch(C::Eq, 0x79de8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(3), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(12), r(3), Shift::Lsr, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x2)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x79dd4, false)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x4800, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x798f8, false)?;
    Ok(())
}

fn write_signed_exp_golomb(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79df0)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r(3), r(2), 0x0)?)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(2), r(0), 0x0)?)?;
    c.emit(data_register(C::Lt, Op::Orr, r(1), r(12), r(3), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x79e40, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(3), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(12), r(3), Shift::Lsr, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x2)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x79e2c, false)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x4800, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x798f8, false)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    initialize_writer(&mut c)?;
    append_bits(&mut c)?;
    flush_word(&mut c)?;
    write_length_prefix(&mut c)?;
    write_start_code(&mut c)?;
    align_ones(&mut c)?;
    align_with_fill(&mut c)?;
    flush_remaining_bytes(&mut c)?;
    write_unsigned_exp_golomb(&mut c)?;
    write_signed_exp_golomb(&mut c)?;
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
        assert_eq!(bytes.len(), 1408);
        assert_eq!(CODE_BYTES, 350 * 4);
        assert_eq!(DATA_BYTES, 2 * 4);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 350);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::data_value(at).is_some()).count(), 2);
        assert_eq!(FUNCTION_STARTS.len(), 10);
        for at in FUNCTION_STARTS { assert!(TypedA32::is_code(at)); }
        for at in [0x79c94, 0x79db4] { assert_eq!(word(&bytes, at), 0xf41db); }
        for (at, target) in [(0x79c14, 0x799a4), (0x79c90, 0x799a4),
            (0x79dec, 0x798f8), (0x79e44, 0x798f8)] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x79df0), data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x79dfc), data_immediate(C::Mi, Op::Rsb, r(3), r(2), 0).unwrap());
        assert_eq!(word(&bytes, 0x79e00), transfer_multiple(C::Al, false, R::SP, 0x4800, BlockMode::DecrementBefore, true).unwrap());
    }

    #[test]
    fn exact_guards() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), TypedA32::is_code(at));
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.data(at, 0xf41db).is_ok(), TypedA32::data_value(at).is_some());
        }
        for (at, rd, target) in [(0x79c68, 2, 0x79c94), (0x79ca4, 2, 0x79db4), (0x79d74, 2, 0x79db4)] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal(rd, target + 4).is_err());
            assert!(c.literal(rd + 1, target).is_err());
            assert!(c.literal(rd, if target == 0x79c94 { 0x79db4 } else { 0x79c94 }).is_err());
            assert!(c.bytes.is_empty());
            c.literal(rd, target).unwrap();
        }
        for (at, target) in [(0x79c14, 0x799a4), (0x79c90, 0x799a4),
            (0x79dec, 0x798f8), (0x79e44, 0x798f8)] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.branch(C::Ne, target, false).is_err());
            assert!(c.branch(C::Al, target, true).is_err());
            assert!(c.branch(C::Al, target + 4, false).is_err());
            assert!(c.bytes.is_empty());
            c.branch(C::Al, target, false).unwrap();
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(START + 4).is_err());
        assert!(c.data(0x79c94, 0xf41db).is_err());
        assert!(c.literal(2, 0x79c94).is_err());
        for target in [START, START + 1, 0x79c94, 0x79db4, END, 0x17b0] {
            assert!(c.branch(C::Al, target, false).is_err());
            assert!(c.branch(C::Al, target, true).is_err());
        }
        assert!(c.bytes.is_empty());
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
}
