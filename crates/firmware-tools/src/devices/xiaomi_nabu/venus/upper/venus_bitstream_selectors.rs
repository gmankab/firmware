use arm32::{
    branch, bx, data_immediate, data_register, data_register_shifted_by_register,
    load_halfword, mov_register, mov_shifted_register, multiply_register,
    push_registers, store_halfword, transfer_byte, transfer_byte_register,
    transfer_multiple, transfer_word, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x79e48;
pub const END: usize = 0x7a234;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 904;
pub const DATA_BYTES: usize = 100;

fn r(n: u8) -> R { R::new(n).unwrap() }

fn selector_target(selector: usize) -> usize {
    match selector {
        1 => 0x7a110, 2 => 0x7a16c, 3 => 0x7a180, 4 => 0x7a194,
        10 => 0x7a1a8, 12 => 0x7a1b4, 15 => 0x7a1c0, 16 => 0x7a1cc,
        18 => 0x7a1d8, 20 => 0x7a1e4, 24 => 0x7a1f0,
        _ => 0x7a22c,
    }
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("bitstream selector address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && ((START..0x79fd4).contains(&at)
            || (0x79fd8..0x7a0b0).contains(&at) || (0x7a110..END).contains(&at))
    }

    fn data_value(at: usize) -> Option<u32> {
        if at == 0x79fd4 { return Some(0x1fffe); }
        if at % 4 == 0 && (0x7a0b0..0x7a110).contains(&at) {
            return Some(selector_target((at - 0x7a0b0) / 4 + 1) as u32);
        }
        None
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("bitstream selector instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) {
            return Err("bitstream selector protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        if self.at != 0x79ed0 || rd != r(7) || target != 0x79fd4 {
            return Err("bitstream selector invalid literal".into());
        }
        self.emit(transfer_word(C::Al, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x79ebc, C::Hi, 0x79fc4, false)
            | (0x79ee0, C::Eq, 0x79f0c, false)
            | (0x79eec, C::Ne, 0x79f14, false)
            | (0x79ef4, C::Ne, 0x79f28, false)
            | (0x79f00, C::Eq, 0x79f7c, false)
            | (0x79f08, C::Al, 0x79f8c, false)
            | (0x79f10, C::Al, 0x79fa8, false)
            | (0x79f1c, C::Eq, 0x79f40, false)
            | (0x79f24, C::Al, 0x79f50, false)
            | (0x79f3c, C::Al, 0x79fa8, false)
            | (0x79f4c, C::Al, 0x79fd8, true)
            | (0x79f58, C::Eq, 0x79f6c, false)
            | (0x79f64, C::Al, 0x79fd8, true)
            | (0x79f78, C::Al, 0x79fa8, false)
            | (0x79f88, C::Al, 0x79fd8, true)
            | (0x79f94, C::Eq, 0x79fa4, false)
            | (0x79fa0, C::Al, 0x79fd8, true)
            | (0x79fc0, C::Cc, 0x79ed0, false)
            | (0x7a00c, C::Ne, 0x7a034, false)
            | (0x7a030, C::Al, 0x7a038, false)
            | (0x7a040, C::Ne, 0x79ff0, false)
            | (0x7a078, C::Al, 0x79e74, true)
            | (0x7a084, C::Gt, 0x7a068, false)
            | (0x7a094, C::Gt, 0x7a11c, false)
            | (0x7a0a0, C::Hi, 0x7a22c, false)
            | (0x7a118, C::Al, 0x7a174, false)
            | (0x7a120, C::Le, 0x7a150, false)
            | (0x7a128, C::Eq, 0x7a1fc, false)
            | (0x7a130, C::Eq, 0x7a208, false)
            | (0x7a138, C::Ne, 0x7a22c, false)
            | (0x7a148, C::Ne, 0x7a22c, false)
            | (0x7a154, C::Eq, 0x7a21c, false)
            | (0x7a15c, C::Ne, 0x7a22c, false)
            | (0x7a168, C::Al, 0x7a210, false)
            | (0x7a17c, C::Al, 0x7a22c, false)
            | (0x7a1a0, C::Eq, 0x7a14c, false)
            | (0x7a1a4, C::Al, 0x7a22c, false)
            | (0x7a1b0, C::Al, 0x7a224, false)
            | (0x7a1bc, C::Al, 0x7a224, false)
            | (0x7a1c8, C::Al, 0x7a224, false)
            | (0x7a1d4, C::Al, 0x7a224, false)
            | (0x7a1e0, C::Al, 0x7a224, false)
            | (0x7a1ec, C::Al, 0x7a224, false)
            | (0x7a1f8, C::Al, 0x7a224, false)
            | (0x7a204, C::Al, 0x7a210, false)
            | (0x7a214, C::Eq, 0x7a14c, false)
            | (0x7a218, C::Al, 0x7a22c, false)
            | (0x7a228, C::Eq, 0x7a14c, false)
        );
        if !valid { return Err("bitstream selector invalid branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn initialize(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79e48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.check(0x79e4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xff)?)?;
    c.check(0x79e50)?;
    c.emit(store_halfword(C::Al, r(2), r(0), 0x0)?)?;
    c.check(0x79e54)?;
    c.emit(store_halfword(C::Al, r(3), r(0), 0x2)?)?;
    c.check(0x79e58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.check(0x79e5c)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0x4)?)?;
    c.check(0x79e60)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0x8)?)?;
    c.check(0x79e64)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0xc)?)?;
    c.check(0x79e68)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x10)?)?;
    c.check(0x79e6c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x14)?)?;
    c.check(0x79e70)?;
    c.emit(bx(C::Al, r(14)))?;
    c.check(0x79e74)
}

fn encode_probability(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79e74)?;
    c.emit(push_registers(C::Al, 0x4ff0)?)?;
    c.check(0x79e78)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.check(0x79e7c)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.check(0x79e80)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0x79e84)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x79e88)?;
    c.emit(load_halfword(C::Al, r(0), r(4), 0x2)?)?;
    c.check(0x79e8c)?;
    c.emit(load_halfword(C::Ne, r(1), r(4), 0x0)?)?;
    c.check(0x79e90)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), r(0), 0x1)?)?;
    c.check(0x79e94)?;
    c.emit(multiply_register(C::Al, r(7), r(3), r(2))?)?;
    c.check(0x79e98)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x79e9c)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(2), r(7), Shift::Lsr, 8, false)?)?;
    c.check(0x79ea0)?;
    c.emit(data_register(C::Ne, Op::Add, r(1), r(5), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x79ea4)?;
    c.emit(data_register(C::Ne, Op::Sub, r(5), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.check(0x79ea8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xff)?)?;
    c.check(0x79eac)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xff00)?)?;
    c.check(0x79eb0)?;
    c.emit(store_halfword(C::Ne, r(1), r(4), 0x0)?)?;
    c.check(0x79eb4)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x79eb8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x7f)?)?;
    c.check(0x79ebc)?;
    c.branch(C::Hi, 0x79fc4, false)?;
    c.check(0x79ec0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x3f80)?)?;
    c.check(0x79ec4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.check(0x79ec8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.check(0x79ecc)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0xc000)?)?;
    c.check(0x79ed0)?;
    c.literal(r(7), 0x79fd4)?;
    c.check(0x79ed4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x13)?)?;
    c.check(0x79ed8)?;
    c.emit(mov_shifted_register(C::Al, r(6), r(5), Shift::Lsl, 1)?)?;
    c.check(0x79edc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x79ee0)?;
    c.branch(C::Eq, 0x79f0c, false)?;
    c.check(0x79ee4)?;
    c.emit(load_halfword(C::Al, r(0), r(4), 0x0)?)?;
    c.check(0x79ee8)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x200)?)?;
    c.check(0x79eec)?;
    c.branch(C::Ne, 0x79f14, false)?;
    c.check(0x79ef0)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x100)?)?;
    c.check(0x79ef4)?;
    c.branch(C::Ne, 0x79f28, false)?;
    c.check(0x79ef8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x10)?)?;
    c.check(0x79efc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x79f00)?;
    c.branch(C::Eq, 0x79f7c, false)?;
    c.check(0x79f04)?;
    c.emit(transfer_byte(C::Al, false, r(9), r(4), 0x10)?)?;
    c.check(0x79f08)?;
    c.branch(C::Al, 0x79f8c, false)?;
    c.check(0x79f0c)?;
    c.emit(transfer_byte(C::Al, false, r(8), r(4), 0x13)?)?;
    c.check(0x79f10)?;
    c.branch(C::Al, 0x79fa8, false)?;
    c.check(0x79f14)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x10)?)?;
    c.check(0x79f18)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x79f1c)?;
    c.branch(C::Eq, 0x79f40, false)?;
    c.check(0x79f20)?;
    c.emit(transfer_byte(C::Al, false, r(9), r(4), 0x10)?)?;
    c.check(0x79f24)?;
    c.branch(C::Al, 0x79f50, false)?;
    c.check(0x79f28)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 0x11)?)?;
    c.check(0x79f2c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xff00)?)?;
    c.check(0x79f30)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.check(0x79f34)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 0x11)?)?;
    c.check(0x79f38)?;
    c.emit(store_halfword(C::Al, r(0), r(4), 0x0)?)?;
    c.check(0x79f3c)?;
    c.branch(C::Al, 0x79fa8, false)?;
    c.check(0x79f40)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x79f44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x79f48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x79f4c)?;
    c.branch(C::Al, 0x79fd8, true)?;
    c.check(0x79f50)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(4), 0x11)?)?;
    c.check(0x79f54)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x79f58)?;
    c.branch(C::Eq, 0x79f6c, false)?;
    c.check(0x79f5c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x79f60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x79f64)?;
    c.branch(C::Al, 0x79fd8, true)?;
    c.check(0x79f68)?;
    c.emit(transfer_byte(C::Al, false, r(9), r(4), 0x11)?)?;
    c.check(0x79f6c)?;
    c.emit(load_halfword(C::Al, r(0), r(4), 0x0)?)?;
    c.check(0x79f70)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xfe00)?)?;
    c.check(0x79f74)?;
    c.emit(store_halfword(C::Al, r(0), r(4), 0x0)?)?;
    c.check(0x79f78)?;
    c.branch(C::Al, 0x79fa8, false)?;
    c.check(0x79f7c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x79f80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x79f84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x79f88)?;
    c.branch(C::Al, 0x79fd8, true)?;
    c.check(0x79f8c)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(4), 0x11)?)?;
    c.check(0x79f90)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x79f94)?;
    c.branch(C::Eq, 0x79fa4, false)?;
    c.check(0x79f98)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x79f9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x79fa0)?;
    c.branch(C::Al, 0x79fd8, true)?;
    c.check(0x79fa4)?;
    c.emit(transfer_byte(C::Al, false, r(9), r(4), 0x11)?)?;
    c.check(0x79fa8)?;
    c.emit(load_halfword(C::Al, r(0), r(4), 0x0)?)?;
    c.check(0x79fac)?;
    c.emit(data_register(C::Al, Op::And, r(5), r(7), r(5), Shift::Lsl, 1, false)?)?;
    c.check(0x79fb0)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsl, 1)?)?;
    c.check(0x79fb4)?;
    c.emit(store_halfword(C::Al, r(0), r(4), 0x0)?)?;
    c.check(0x79fb8)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(6), r(10), Shift::Lsl, 0, false)?)?;
    c.check(0x79fbc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x80)?)?;
    c.check(0x79fc0)?;
    c.branch(C::Cc, 0x79ed0, false)?;
    c.check(0x79fc4)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(5), 0xff)?)?;
    c.check(0x79fc8)?;
    c.emit(store_halfword(C::Al, r(0), r(4), 0x2)?)?;
    c.check(0x79fcc)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x79fd0)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x79fd4)
}

fn append_repeated_bit(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79fd8)?;
    c.emit(push_registers(C::Al, 0x48f0)?)?;
    c.check(0x79fdc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.check(0x79fe0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x79fe4)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x79fe8)?;
    c.emit(transfer_byte(C::Al, true, r(12), r(0), 0x12)?)?;
    c.check(0x79fec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x79ff0)?;
    c.emit(data_immediate(C::Al, Op::And, r(3), r(12), 0xff)?)?;
    c.check(0x79ff4)?;
    c.emit(data_register(C::Al, Op::Add, r(14), r(1), r(3), Shift::Lsl, 1, false)?)?;
    c.check(0x79ff8)?;
    c.emit(transfer_byte(C::Al, false, r(14), r(0), 0x12)?)?;
    c.check(0x79ffc)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 0x8)?)?;
    c.check(0x7a000)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.check(0x7a004)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x8)?)?;
    c.check(0x7a008)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x8)?)?;
    c.check(0x7a00c)?;
    c.branch(C::Ne, 0x7a034, false)?;
    c.check(0x7a010)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.check(0x7a014)?;
    c.emit(transfer_word(C::Al, false, r(12), r(0), 0x8)?)?;
    c.check(0x7a018)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 0xc)?)?;
    c.check(0x7a01c)?;
    c.emit(transfer_word(C::Al, true, r(5), r(0), 0x14)?)?;
    c.check(0x7a020)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(3), 0x1)?)?;
    c.check(0x7a024)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 0xc)?)?;
    c.check(0x7a028)?;
    c.emit(transfer_byte_register(C::Al, false, r(14), r(5), r(3), Shift::Lsl, 0)?)?;
    c.check(0x7a02c)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x12)?)?;
    c.check(0x7a030)?;
    c.branch(C::Al, 0x7a038, false)?;
    c.check(0x7a034)?;
    c.emit(mov_register(C::Al, r(12), r(14)))?;
    c.check(0x7a038)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.check(0x7a03c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.check(0x7a040)?;
    c.branch(C::Ne, 0x79ff0, false)?;
    c.check(0x7a044)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x7a048)
}

fn encode_fixed_bits(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a048)?;
    c.emit(push_registers(C::Al, 0x48f0)?)?;
    c.check(0x7a04c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.check(0x7a050)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.check(0x7a054)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0x7a058)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.check(0x7a05c)?;
    c.emit(transfer_multiple(C::Lt, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x7a060)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(2), 0x1)?)?;
    c.check(0x7a064)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.check(0x7a068)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(6), 0x2)?)?;
    c.check(0x7a06c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7a070)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::And, r(1), r(7), r(4), Shift::Lsr, r(0), false)?)?;
    c.check(0x7a074)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7a078)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7a07c)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(6), r(6), 0x1)?)?;
    c.check(0x7a080)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x1)?)?;
    c.check(0x7a084)?;
    c.branch(C::Gt, 0x7a068, false)?;
    c.check(0x7a088)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x7a08c)
}

fn select_bitstream_value(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7a08c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x0)?)?;
    c.check(0x7a090)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1f)?)?;
    c.check(0x7a094)?;
    c.branch(C::Gt, 0x7a11c, false)?;
    c.check(0x7a098)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.check(0x7a09c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x17)?)?;
    c.check(0x7a0a0)?;
    c.branch(C::Hi, 0x7a22c, false)?;
    c.check(0x7a0a4)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(1), Shift::Lsl, 2)?)?;
    c.check(0x7a0a8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(15), 0x0)?)?;
    c.check(0x7a0ac)?;
    c.emit(transfer_word_register(C::Al, true, r(15), r(1), r(2), Shift::Lsl, 0)?)?;
    for selector in 1..=24 {
        c.data(0x7a0b0 + (selector - 1) * 4, selector_target(selector) as u32)?;
    }
    c.check(0x7a110)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a114)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7a118)?;
    c.branch(C::Al, 0x7a174, false)?;
    c.check(0x7a11c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x3f)?)?;
    c.check(0x7a120)?;
    c.branch(C::Le, 0x7a150, false)?;
    c.check(0x7a124)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x40)?)?;
    c.check(0x7a128)?;
    c.branch(C::Eq, 0x7a1fc, false)?;
    c.check(0x7a12c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x50)?)?;
    c.check(0x7a130)?;
    c.branch(C::Eq, 0x7a208, false)?;
    c.check(0x7a134)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0xa0)?)?;
    c.check(0x7a138)?;
    c.branch(C::Ne, 0x7a22c, false)?;
    c.check(0x7a13c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a140)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd)?)?;
    c.check(0x7a144)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x63)?)?;
    c.check(0x7a148)?;
    c.branch(C::Ne, 0x7a22c, false)?;
    c.check(0x7a14c)?;
    c.emit(bx(C::Al, r(14)))?;
    c.check(0x7a150)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x20)?)?;
    c.check(0x7a154)?;
    c.branch(C::Eq, 0x7a21c, false)?;
    c.check(0x7a158)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x28)?)?;
    c.check(0x7a15c)?;
    c.branch(C::Ne, 0x7a22c, false)?;
    c.check(0x7a160)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a164)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x5)?)?;
    c.check(0x7a168)?;
    c.branch(C::Al, 0x7a210, false)?;
    c.check(0x7a16c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a170)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.check(0x7a174)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0x7a178)?;
    c.emit(bx(C::Eq, r(14)))?;
    c.check(0x7a17c)?;
    c.branch(C::Al, 0x7a22c, false)?;
    c.check(0x7a180)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a184)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xf)?)?;
    c.check(0x7a188)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.check(0x7a18c)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0xff)?)?;
    c.check(0x7a190)?;
    c.emit(bx(C::Al, r(14)))?;
    c.check(0x7a194)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a198)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe)?)?;
    c.check(0x7a19c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x3)?)?;
    c.check(0x7a1a0)?;
    c.branch(C::Eq, 0x7a14c, false)?;
    c.check(0x7a1a4)?;
    c.branch(C::Al, 0x7a22c, false)?;
    c.check(0x7a1a8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1ac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x3)?)?;
    c.check(0x7a1b0)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1b4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.check(0x7a1bc)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1c0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb)?)?;
    c.check(0x7a1c8)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1cc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0x7a1d4)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1d8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xa)?)?;
    c.check(0x7a1e0)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1e4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x7)?)?;
    c.check(0x7a1ec)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1f0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a1f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x6)?)?;
    c.check(0x7a1f8)?;
    c.branch(C::Al, 0x7a224, false)?;
    c.check(0x7a1fc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a200)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xc)?)?;
    c.check(0x7a204)?;
    c.branch(C::Al, 0x7a210, false)?;
    c.check(0x7a208)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a20c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x9)?)?;
    c.check(0x7a210)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x21)?)?;
    c.check(0x7a214)?;
    c.branch(C::Eq, 0x7a14c, false)?;
    c.check(0x7a218)?;
    c.branch(C::Al, 0x7a22c, false)?;
    c.check(0x7a21c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x4)?)?;
    c.check(0x7a220)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0x7a224)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0xb)?)?;
    c.check(0x7a228)?;
    c.branch(C::Eq, 0x7a14c, false)?;
    c.check(0x7a22c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xff)?)?;
    c.check(0x7a230)?;
    c.emit(bx(C::Al, r(14)))?;
    c.check(0x7a234)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    initialize(&mut c)?;
    encode_probability(&mut c)?;
    c.data(0x79fd4, 0x1fffe)?;
    append_repeated_bit(&mut c)?;
    encode_fixed_bits(&mut c)?;
    select_bitstream_value(&mut c)?;
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
    fn structure_and_complete_selector_tail() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1004);
        assert_eq!(CODE_BYTES, 904);
        assert_eq!(DATA_BYTES, 100);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 226);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::data_value(at).is_some()).count(), 25);
        assert_eq!([0x79e74 - START, 0x79fd8 - 0x79e74, 0x7a048 - 0x79fd8,
            0x7a08c - 0x7a048, END - 0x7a08c], [44, 356, 112, 68, 424]);
        assert_eq!(word(&bytes, 0x79fd4), 0x1fffe);
        let targets = [0x7a110, 0x7a16c, 0x7a180, 0x7a194, 0x7a22c, 0x7a22c,
            0x7a22c, 0x7a22c, 0x7a22c, 0x7a1a8, 0x7a22c, 0x7a1b4,
            0x7a22c, 0x7a22c, 0x7a1c0, 0x7a1cc, 0x7a22c, 0x7a1d8,
            0x7a22c, 0x7a1e4, 0x7a22c, 0x7a22c, 0x7a22c, 0x7a1f0];
        for (i, target) in targets.into_iter().enumerate() {
            assert_eq!(word(&bytes, 0x7a0b0 + i * 4), target);
            assert!(TypedA32::is_code(target as usize));
        }
        assert_eq!(word(&bytes, 0x7a0ac), transfer_word_register(C::Al, true,
            R::PC, r(1), r(2), Shift::Lsl, 0).unwrap());
        assert_eq!(word(&bytes, 0x7a14c), bx(C::Al, R::LR));
        assert_eq!(word(&bytes, 0x7a150), data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x20).unwrap());
        assert_eq!(word(&bytes, 0x7a228), branch(C::Eq, 0x7a228, 0x7a14c, false).unwrap());
        assert_eq!(word(&bytes, END - 4), bx(C::Al, R::LR));
    }

    #[test]
    fn exact_code_data_literal_and_branch_guards() {
        for at in (START - 4..=END + 4).step_by(4).chain([START + 1, 0x79fd5, 0x7a0b1]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(bx(C::Al, R::LR)).is_ok(), TypedA32::is_code(at));
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = TypedA32::data_value(at) {
                assert!(c.data(at, value ^ 1).is_err());
                assert!(c.data(at + 4, value).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
                c.data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        let bytes = fragment().unwrap();
        for at in (START..END).step_by(4) {
            for target in [START, 0x79fd4, 0x79fd8, 0x7a0b0, 0x7a14c, 0x7a22c, END, START + 1] {
                for condition in [C::Al, C::Eq, C::Ne] {
                    for link in [false, true] {
                        let mut c = TypedA32 { at, bytes: Vec::new() };
                        if c.branch(condition, target, link).is_ok() {
                            assert!(TypedA32::is_code(at));
                            assert!(TypedA32::is_code(target));
                            assert_eq!(word(&bytes, at), word(&c.bytes, START));
                        } else {
                            assert_eq!(c.at, at);
                            assert!(c.bytes.is_empty());
                        }
                    }
                }
            }
            for target in [START, 0x79fd4, 0x79fd5, 0x7a0b0, END] {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.literal(r(7), target).is_ok(), at == 0x79ed0 && target == 0x79fd4);
            }
        }
        let mut c = TypedA32 { at: 0x79ed0, bytes: Vec::new() };
        assert!(c.literal(r(6), 0x79fd4).is_err());
        assert!(c.check(END).is_err());
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
