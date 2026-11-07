use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    branch_exchange_with_link, bx, data_immediate, data_immediate_with_flags,
    data_register, multiply_accumulate, multiply_long, multiply_register, supervisor_call,
    transfer_byte, transfer_byte_postindexed, transfer_byte_register, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_postindexed, transfer_word_register,
};

pub const START: usize = 0x83d2c;
pub const END: usize = 0x84574;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2072;
pub const DATA_BYTES: usize = 48;

const LITERALS: [(usize, u32); 12] = [
    (0x83d64, 0x000f4df0),
    (0x83d68, 0x000f4dcc),
    (0x83d80, 0x0010b488),
    (0x83dbc, 0x0010b488),
    (0x83dc0, 0x4c957f2d),
    (0x83dc4, 0x5851f42d),
    (0x83ffc, 0x001034a8),
    (0x841d8, 0x01010101),
    (0x841dc, 0xfefefeff),
    (0x841e0, 0x80808080),
    (0x84538, 0xfffff001),
    (0x84560, 0x000f4df4),
];

const LITERAL_REFERENCES: [(usize, usize); 12] = [
    (0x83d34, 0x83d64),
    (0x83d3c, 0x83d68),
    (0x83d6c, 0x83d80),
    (0x83d88, 0x83dbc),
    (0x83d90, 0x83dc0),
    (0x83da0, 0x83dc4),
    (0x83e78, 0x83ffc),
    (0x84184, 0x841d8),
    (0x8418c, 0x841dc),
    (0x84190, 0x841e0),
    (0x84510, 0x84538),
    (0x84550, 0x84560),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
    code_bytes: usize,
    data_bytes: usize,
}

impl Code {
    fn new(at: usize) -> Self {
        Self { at, bytes: Vec::new(), code_bytes: 0, data_bytes: 0 }
    }

    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("runtime address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, at: usize, value: u32, data: bool) -> Result<(), String> {
        self.check(at)?;
        let literal = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, v)| v);
        if !(START..END).contains(&at) || at % 4 != 0
            || if data { literal != Some(value) } else { literal.is_some() }
        {
            return Err(format!("invalid runtime {} at {at:#x}", if data { "data" } else { "code" }));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        if data { self.data_bytes += 4; } else { self.code_bytes += 4; }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.word(at, instruction, false)
    }

    fn literal(&mut self, at: usize, rd: u8, slot: usize) -> Result<(), String> {
        if !LITERAL_REFERENCES.contains(&(at, slot))
            || !LITERALS.iter().any(|&(address, _)| address == slot)
        {
            return Err(format!("invalid runtime literal reference {at:#x} -> {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, r(rd), R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn function_83d2c(c: &mut Code) -> Result<(), String> {
    c.emit(0x83d2c, transfer_multiple(C::Al, false, r(13), 0x40e0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x83d30, data_register(C::Al, Op::Mov, r(14), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.literal(0x83d34, 0, 0x83d64)?;
    c.emit(0x83d38, data_register(C::Al, Op::Mov, r(12), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(0x83d3c, 1, 0x83d68)?;
    c.emit(0x83d40, transfer_word(C::Al, false, r(3), r(13), 0)?)?;
    c.emit(0x83d44, transfer_word(C::Al, false, r(2), r(13), 4)?)?;
    c.emit(0x83d48, data_register(C::Al, Op::Mov, r(2), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(0x83d4c, data_register(C::Al, Op::Mov, r(3), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x83d50, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x83d54, branch(C::Al, 0x83d54, 0x84000, true)?)?;
    c.emit(0x83d58, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x83d5c, branch(C::Al, 0x83d5c, 0x83dc8, true)?)?;
    c.emit(0x83d60, branch(C::Al, 0x83d60, 0x84260, true)?)?;
    c.word(0x83d64, 0xf4df0, true)?;
    c.word(0x83d68, 0xf4dcc, true)?;
    Ok(())
}

fn function_83d6c(c: &mut Code) -> Result<(), String> {
    c.literal(0x83d6c, 1, 0x83d80)?;
    c.emit(0x83d70, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x83d74, data_immediate(C::Al, Op::Sub, r(2), r(0), 0x1)?)?;
    c.emit(0x83d78, transfer_doubleword(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(0x83d7c, bx(C::Al, r(14)))?;
    c.word(0x83d80, 0x10b488, true)?;
    Ok(())
}

fn function_83d84(c: &mut Code) -> Result<(), String> {
    c.emit(0x83d84, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.literal(0x83d88, 0, 0x83dbc)?;
    c.emit(0x83d8c, transfer_doubleword(C::Al, true, r(2), r(0), 0)?)?;
    c.literal(0x83d90, 1, 0x83dc0)?;
    c.emit(0x83d94, multiply_long(C::Al, false, r(12), r(14), r(2), r(1))?)?;
    c.emit(0x83d98, data_immediate_with_flags(C::Al, Op::Add, r(12), r(12), 0x1, true)?)?;
    c.emit(0x83d9c, transfer_word(C::Al, false, r(12), r(0), 0)?)?;
    c.literal(0x83da0, 12, 0x83dc4)?;
    c.emit(0x83da4, multiply_accumulate(C::Al, r(4), r(2), r(12), r(14))?)?;
    c.emit(0x83da8, multiply_accumulate(C::Al, r(2), r(3), r(1), r(4))?)?;
    c.emit(0x83dac, data_immediate(C::Al, Op::Adc, r(1), r(2), 0x0)?)?;
    c.emit(0x83db0, transfer_word(C::Al, false, r(1), r(0), 4)?)?;
    c.emit(0x83db4, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsr, 1, false)?)?;
    c.emit(0x83db8, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.word(0x83dbc, 0x10b488, true)?;
    c.word(0x83dc0, 0x4c957f2d, true)?;
    c.word(0x83dc4, 0x5851f42d, true)?;
    Ok(())
}

fn function_83dc8(c: &mut Code) -> Result<(), String> {
    c.emit(0x83dc8, transfer_multiple(C::Al, false, r(13), 0x41f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x83dcc, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x8)?)?;
    c.emit(0x83dd0, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x83dd4, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x83dd8, branch(C::Eq, 0x83dd8, 0x83e78, false)?)?;
    c.emit(0x83ddc, transfer_word(C::Al, true, r(0), r(5), 76)?)?;
    c.emit(0x83de0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x83de4, branch(C::Lt, 0x83de4, 0x83e98, false)?)?;
    c.emit(0x83de8, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83dec, branch(C::Al, 0x83dec, 0x4b78, true)?)?;
    c.emit(0x83df0, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x83df4, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0x83df8, transfer_word(C::Al, true, r(1), r(5), 28)?)?;
    c.emit(0x83dfc, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e00, branch(C::Ls, 0x83e00, 0x83e24, false)?)?;
    c.emit(0x83e04, transfer_word(C::Al, true, r(3), r(5), 36)?)?;
    c.emit(0x83e08, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e0c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x83e10, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x83e14, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x83e18, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0x83e1c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x83e20, branch(C::Eq, 0x83e20, 0x83fe4, false)?)?;
    c.emit(0x83e24, transfer_multiple(C::Al, true, r(5), 0x3, BlockMode::IncrementBefore, false)?)?;
    c.emit(0x83e28, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e2c, branch(C::Cs, 0x83e2c, 0x83e4c, false)?)?;
    c.emit(0x83e30, transfer_word(C::Al, true, r(7), r(5), 40)?)?;
    c.emit(0x83e34, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x83e38, transfer_word(C::Al, false, r(2), r(13), 0)?)?;
    c.emit(0x83e3c, data_register(C::Al, Op::Sub, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e40, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e44, data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Asr, 31, false)?)?;
    c.emit(0x83e48, branch_exchange_with_link(C::Al, r(7))?)?;
    c.emit(0x83e4c, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x83e50, transfer_word(C::Al, false, r(4), r(5), 16)?)?;
    c.emit(0x83e54, transfer_word(C::Al, false, r(4), r(5), 28)?)?;
    c.emit(0x83e58, transfer_word(C::Al, false, r(4), r(5), 20)?)?;
    c.emit(0x83e5c, transfer_word(C::Al, false, r(4), r(5), 4)?)?;
    c.emit(0x83e60, transfer_word(C::Al, false, r(4), r(5), 8)?)?;
    c.emit(0x83e64, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x83e68, branch(C::Eq, 0x83e68, 0x83fd8, false)?)?;
    c.emit(0x83e6c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e70, branch(C::Al, 0x83e70, 0x4b80, true)?)?;
    c.emit(0x83e74, branch(C::Al, 0x83e74, 0x83fd8, false)?)?;
    c.literal(0x83e78, 0, 0x83ffc)?;
    c.emit(0x83e7c, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x83e80, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x83e84, branch(C::Eq, 0x83e84, 0x83f0c, false)?)?;
    c.emit(0x83e88, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x83e8c, branch(C::Al, 0x83e8c, 0x83dc8, true)?)?;
    c.emit(0x83e90, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x83e94, branch(C::Al, 0x83e94, 0x83f10, false)?)?;
    c.emit(0x83e98, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0x83e9c, transfer_word(C::Al, true, r(1), r(5), 28)?)?;
    c.emit(0x83ea0, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83ea4, branch(C::Ls, 0x83ea4, 0x83ec8, false)?)?;
    c.emit(0x83ea8, transfer_word(C::Al, true, r(3), r(5), 36)?)?;
    c.emit(0x83eac, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83eb0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x83eb4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x83eb8, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x83ebc, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0x83ec0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x83ec4, branch(C::Eq, 0x83ec4, 0x83ff4, false)?)?;
    c.emit(0x83ec8, transfer_multiple(C::Al, true, r(5), 0x3, BlockMode::IncrementBefore, false)?)?;
    c.emit(0x83ecc, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83ed0, branch(C::Cs, 0x83ed0, 0x83ef0, false)?)?;
    c.emit(0x83ed4, transfer_word(C::Al, true, r(7), r(5), 40)?)?;
    c.emit(0x83ed8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x83edc, transfer_word(C::Al, false, r(2), r(13), 0)?)?;
    c.emit(0x83ee0, data_register(C::Al, Op::Sub, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83ee4, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83ee8, data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Asr, 31, false)?)?;
    c.emit(0x83eec, branch_exchange_with_link(C::Al, r(7))?)?;
    c.emit(0x83ef0, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x83ef4, transfer_word(C::Al, false, r(4), r(5), 16)?)?;
    c.emit(0x83ef8, transfer_word(C::Al, false, r(4), r(5), 28)?)?;
    c.emit(0x83efc, transfer_word(C::Al, false, r(4), r(5), 20)?)?;
    c.emit(0x83f00, transfer_word(C::Al, false, r(4), r(5), 4)?)?;
    c.emit(0x83f04, transfer_word(C::Al, false, r(4), r(5), 8)?)?;
    c.emit(0x83f08, branch(C::Al, 0x83f08, 0x83fd8, false)?)?;
    c.emit(0x83f0c, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x83f10, branch(C::Al, 0x83f10, 0x4e24, true)?)?;
    c.emit(0x83f14, transfer_word(C::Al, true, r(5), r(0), 0)?)?;
    c.emit(0x83f18, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x83f1c, branch(C::Eq, 0x83f1c, 0x83fd4, false)?)?;
    c.emit(0x83f20, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.emit(0x83f24, transfer_word(C::Al, true, r(0), r(5), 76)?)?;
    c.emit(0x83f28, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x83f2c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x83f30, branch(C::Lt, 0x83f30, 0x83f40, false)?)?;
    c.emit(0x83f34, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f38, branch(C::Al, 0x83f38, 0x4b78, true)?)?;
    c.emit(0x83f3c, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f40, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0x83f44, transfer_word(C::Al, true, r(1), r(5), 28)?)?;
    c.emit(0x83f48, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f4c, branch(C::Ls, 0x83f4c, 0x83fb8, false)?)?;
    c.emit(0x83f50, transfer_word(C::Al, true, r(3), r(5), 36)?)?;
    c.emit(0x83f54, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f58, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x83f5c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x83f60, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x83f64, transfer_word(C::Al, true, r(0), r(5), 20)?)?;
    c.emit(0x83f68, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x83f6c, branch(C::Eq, 0x83f6c, 0x83fb0, false)?)?;
    c.emit(0x83f70, transfer_multiple(C::Al, true, r(5), 0x3, BlockMode::IncrementBefore, false)?)?;
    c.emit(0x83f74, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f78, branch(C::Cs, 0x83f78, 0x83f94, false)?)?;
    c.emit(0x83f7c, transfer_word(C::Al, true, r(7), r(5), 40)?)?;
    c.emit(0x83f80, data_register(C::Al, Op::Sub, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f84, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83f88, transfer_word(C::Al, false, r(8), r(13), 0)?)?;
    c.emit(0x83f8c, data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Asr, 31, false)?)?;
    c.emit(0x83f90, branch_exchange_with_link(C::Al, r(7))?)?;
    c.emit(0x83f94, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x83f98, transfer_word(C::Al, false, r(0), r(5), 16)?)?;
    c.emit(0x83f9c, transfer_word(C::Al, false, r(0), r(5), 28)?)?;
    c.emit(0x83fa0, transfer_word(C::Al, false, r(0), r(5), 20)?)?;
    c.emit(0x83fa4, transfer_word(C::Al, false, r(0), r(5), 4)?)?;
    c.emit(0x83fa8, transfer_word(C::Al, false, r(0), r(5), 8)?)?;
    c.emit(0x83fac, branch(C::Al, 0x83fac, 0x83fb4, false)?)?;
    c.emit(0x83fb0, data_immediate(C::Al, Op::Mvn, r(0), r(0), 0x0)?)?;
    c.emit(0x83fb4, data_register(C::Al, Op::Orr, r(4), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x83fb8, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x83fbc, branch(C::Eq, 0x83fbc, 0x83fc8, false)?)?;
    c.emit(0x83fc0, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x83fc4, branch(C::Al, 0x83fc4, 0x4b80, true)?)?;
    c.emit(0x83fc8, transfer_word(C::Al, true, r(5), r(5), 56)?)?;
    c.emit(0x83fcc, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x83fd0, branch(C::Ne, 0x83fd0, 0x83f24, false)?)?;
    c.emit(0x83fd4, branch(C::Al, 0x83fd4, 0x4e30, true)?)?;
    c.emit(0x83fd8, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x83fdc, data_immediate(C::Al, Op::Add, r(13), r(13), 0x8)?)?;
    c.emit(0x83fe0, transfer_multiple(C::Al, true, r(13), 0x81f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x83fe4, data_immediate(C::Al, Op::Mvn, r(4), r(0), 0x0)?)?;
    c.emit(0x83fe8, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x83fec, branch(C::Ne, 0x83fec, 0x83e6c, false)?)?;
    c.emit(0x83ff0, branch(C::Al, 0x83ff0, 0x83fd8, false)?)?;
    c.emit(0x83ff4, data_immediate(C::Al, Op::Mvn, r(4), r(0), 0x0)?)?;
    c.emit(0x83ff8, branch(C::Al, 0x83ff8, 0x83fd8, false)?)?;
    c.word(0x83ffc, 0x1034a8, true)?;
    Ok(())
}

fn function_84000(c: &mut Code) -> Result<(), String> {
    c.emit(0x84000, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x8)?)?;
    c.emit(0x84004, transfer_multiple(C::Al, false, r(13), 0x4080, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84008, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x8)?)?;
    c.emit(0x8400c, transfer_word(C::Al, false, r(3), r(13), 20)?)?;
    c.emit(0x84010, transfer_word(C::Al, false, r(2), r(13), 16)?)?;
    c.emit(0x84014, data_immediate(C::Al, Op::Add, r(2), r(13), 0x10)?)?;
    c.emit(0x84018, transfer_word(C::Al, false, r(2), r(13), 4)?)?;
    c.emit(0x8401c, branch(C::Al, 0x8401c, 0x3410, true)?)?;
    c.emit(0x84020, data_immediate(C::Al, Op::Add, r(13), r(13), 0x8)?)?;
    c.emit(0x84024, transfer_multiple(C::Al, true, r(13), 0x4080, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x84028, data_immediate(C::Al, Op::Add, r(13), r(13), 0x8)?)?;
    c.emit(0x8402c, bx(C::Al, r(14)))?;
    Ok(())
}

fn function_84030(c: &mut Code) -> Result<(), String> {
    c.emit(0x84030, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84034, branch(C::Eq, 0x84034, 0x84118, false)?)?;
    c.emit(0x84038, data_register(C::Al, Op::Add, r(3), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x8403c, data_register(C::Al, Op::Cmp, r(0), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84040, data_register(C::Hi, Op::Add, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84044, data_register(C::Hi, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84048, branch(C::Ls, 0x84048, 0x8411c, false)?)?;
    c.emit(0x8404c, data_register(C::Al, Op::Eor, r(3), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84050, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84054, data_immediate(C::Al, Op::And, r(3), r(3), 0x3)?)?;
    c.emit(0x84058, branch(C::Cs, 0x84058, 0x840bc, false)?)?;
    c.emit(0x8405c, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.emit(0x84060, branch(C::Eq, 0x84060, 0x84080, false)?)?;
    c.emit(0x84064, data_register(C::Al, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84068, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x8406c, branch(C::Eq, 0x8406c, 0x84118, false)?)?;
    c.emit(0x84070, transfer_byte_postindexed(C::Al, true, r(12), r(1), 1)?)?;
    c.emit(0x84074, data_immediate(C::Al, Op::Sub, r(2), r(2), 0x1)?)?;
    c.emit(0x84078, transfer_byte_postindexed(C::Al, false, r(12), r(3), 1)?)?;
    c.emit(0x8407c, branch(C::Al, 0x8407c, 0x84068, false)?)?;
    c.emit(0x84080, data_register(C::Al, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84084, data_immediate(C::Al, Op::Tst, r(0), r(3), 0x3)?)?;
    c.emit(0x84088, branch(C::Eq, 0x84088, 0x840a4, false)?)?;
    c.emit(0x8408c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x84090, branch(C::Eq, 0x84090, 0x84118, false)?)?;
    c.emit(0x84094, transfer_byte_postindexed(C::Al, true, r(12), r(1), 1)?)?;
    c.emit(0x84098, data_immediate(C::Al, Op::Sub, r(2), r(2), 0x1)?)?;
    c.emit(0x8409c, transfer_byte_postindexed(C::Al, false, r(12), r(3), 1)?)?;
    c.emit(0x840a0, branch(C::Al, 0x840a0, 0x84084, false)?)?;
    c.emit(0x840a4, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x4)?)?;
    c.emit(0x840a8, branch(C::Cc, 0x840a8, 0x84068, false)?)?;
    c.emit(0x840ac, transfer_word_postindexed(C::Al, true, r(12), r(1), 4)?)?;
    c.emit(0x840b0, data_immediate(C::Al, Op::Sub, r(2), r(2), 0x4)?)?;
    c.emit(0x840b4, transfer_word_postindexed(C::Al, false, r(12), r(3), 4)?)?;
    c.emit(0x840b8, branch(C::Al, 0x840b8, 0x840a4, false)?)?;
    c.emit(0x840bc, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.emit(0x840c0, branch(C::Ne, 0x840c0, 0x84100, false)?)?;
    c.emit(0x840c4, data_register(C::Al, Op::Add, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x840c8, data_immediate(C::Al, Op::Tst, r(0), r(3), 0x3)?)?;
    c.emit(0x840cc, branch(C::Eq, 0x840cc, 0x840e8, false)?)?;
    c.emit(0x840d0, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x840d4, branch(C::Eq, 0x840d4, 0x84118, false)?)?;
    c.emit(0x840d8, data_immediate(C::Al, Op::Sub, r(2), r(2), 0x1)?)?;
    c.emit(0x840dc, transfer_byte_register(C::Al, true, r(3), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x840e0, transfer_byte_register(C::Al, false, r(3), r(0), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x840e4, branch(C::Al, 0x840e4, 0x840c4, false)?)?;
    c.emit(0x840e8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x4)?)?;
    c.emit(0x840ec, branch(C::Cc, 0x840ec, 0x84100, false)?)?;
    c.emit(0x840f0, data_immediate(C::Al, Op::Sub, r(2), r(2), 0x4)?)?;
    c.emit(0x840f4, transfer_word_register(C::Al, true, r(3), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x840f8, transfer_word_register(C::Al, false, r(3), r(0), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x840fc, branch(C::Al, 0x840fc, 0x840e8, false)?)?;
    c.emit(0x84100, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x84104, bx(C::Eq, r(14)))?;
    c.emit(0x84108, data_immediate(C::Al, Op::Sub, r(2), r(2), 0x1)?)?;
    c.emit(0x8410c, transfer_byte_register(C::Al, true, r(3), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x84110, transfer_byte_register(C::Al, false, r(3), r(0), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x84114, branch(C::Al, 0x84114, 0x84100, false)?)?;
    c.emit(0x84118, bx(C::Al, r(14)))?;
    c.emit(0x8411c, branch(C::Al, 0x8411c, 0x471c, false)?)?;
    Ok(())
}

fn function_84120(c: &mut Code) -> Result<(), String> {
    c.emit(0x84120, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84124, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84128, branch(C::Al, 0x84128, 0x84140, true)?)?;
    c.emit(0x8412c, transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84130, data_immediate(C::Al, Op::And, r(2), r(4), 0xff)?)?;
    c.emit(0x84134, data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84138, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8413c, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_84140(c: &mut Code) -> Result<(), String> {
    c.emit(0x84140, transfer_multiple(C::Al, false, r(13), 0x40b0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84144, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84148, data_immediate_with_flags(C::Al, Op::And, r(1), r(1), 0xff, true)?)?;
    c.emit(0x8414c, branch(C::Ne, 0x8414c, 0x84164, false)?)?;
    c.emit(0x84150, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84154, branch(C::Al, 0x84154, 0x3388, true)?)?;
    c.emit(0x84158, data_register(C::Al, Op::Add, r(0), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8415c, branch(C::Al, 0x8415c, 0x841d4, false)?)?;
    c.emit(0x84160, data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.emit(0x84164, data_immediate(C::Al, Op::Tst, r(0), r(4), 0x3)?)?;
    c.emit(0x84168, branch(C::Eq, 0x84168, 0x84184, false)?)?;
    c.emit(0x8416c, transfer_byte(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x84170, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x84174, data_register(C::Ne, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84178, branch(C::Ne, 0x84178, 0x84160, false)?)?;
    c.emit(0x8417c, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84180, branch(C::Al, 0x84180, 0x841d4, false)?)?;
    c.literal(0x84184, 2, 0x841d8)?;
    c.emit(0x84188, multiply_register(C::Al, r(0), r(1), r(2))?)?;
    c.literal(0x8418c, 2, 0x841dc)?;
    c.literal(0x84190, 3, 0x841e0)?;
    c.emit(0x84194, transfer_word(C::Al, true, r(12), r(4), 0)?)?;
    c.emit(0x84198, data_register(C::Al, Op::Add, r(14), r(12), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x8419c, data_register(C::Al, Op::Bic, r(5), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x841a0, data_register(C::Al, Op::Tst, r(0), r(5), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(0x841a4, data_register(C::Eq, Op::Eor, r(12), r(12), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x841a8, data_register(C::Eq, Op::Add, r(14), r(12), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x841ac, data_register(C::Eq, Op::Bic, r(12), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x841b0, data_register(C::Eq, Op::Tst, r(0), r(12), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(0x841b4, branch(C::Ne, 0x841b4, 0x841c0, false)?)?;
    c.emit(0x841b8, data_immediate(C::Al, Op::Add, r(4), r(4), 0x4)?)?;
    c.emit(0x841bc, branch(C::Al, 0x841bc, 0x84194, false)?)?;
    c.emit(0x841c0, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x841c4, transfer_byte_postindexed(C::Al, true, r(2), r(4), 1)?)?;
    c.emit(0x841c8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x841cc, data_register(C::Ne, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x841d0, branch(C::Ne, 0x841d0, 0x841c0, false)?)?;
    c.emit(0x841d4, transfer_multiple(C::Al, true, r(13), 0x80b0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x841d8, 0x1010101, true)?;
    c.word(0x841dc, 0xfefefeff, true)?;
    c.word(0x841e0, 0x80808080, true)?;
    Ok(())
}

fn function_841e4(c: &mut Code) -> Result<(), String> {
    c.emit(0x841e4, transfer_byte(C::Al, true, r(3), r(0), 0)?)?;
    c.emit(0x841e8, transfer_byte(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x841ec, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.emit(0x841f0, branch(C::Eq, 0x841f0, 0x84208, false)?)?;
    c.emit(0x841f4, data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x841f8, branch(C::Ne, 0x841f8, 0x84208, false)?)?;
    c.emit(0x841fc, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x84200, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x84204, branch(C::Al, 0x84204, 0x841e4, false)?)?;
    c.emit(0x84208, data_register(C::Al, Op::Sub, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x8420c, bx(C::Al, r(14)))?;
    Ok(())
}

fn function_84210(c: &mut Code) -> Result<(), String> {
    c.emit(0x84210, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x84214, branch(C::Ne, 0x84214, 0x84220, false)?)?;
    c.emit(0x84218, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8421c, branch(C::Al, 0x8421c, 0x84238, false)?)?;
    c.emit(0x84220, transfer_byte(C::Al, true, r(3), r(0), 0)?)?;
    c.emit(0x84224, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.emit(0x84228, branch(C::Ne, 0x84228, 0x8423c, false)?)?;
    c.emit(0x8422c, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x84230, transfer_byte(C::Al, true, r(0), r(1), 0)?)?;
    c.emit(0x84234, data_register(C::Al, Op::Sub, r(0), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84238, bx(C::Al, r(14)))?;
    c.emit(0x8423c, transfer_byte(C::Al, true, r(12), r(1), 0)?)?;
    c.emit(0x84240, data_immediate(C::Al, Op::Cmp, r(0), r(12), 0x0)?)?;
    c.emit(0x84244, data_immediate_with_flags(C::Ne, Op::Sub, r(2), r(2), 0x1, true)?)?;
    c.emit(0x84248, branch(C::Eq, 0x84248, 0x84230, false)?)?;
    c.emit(0x8424c, data_register(C::Al, Op::Cmp, r(0), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x84250, branch(C::Ne, 0x84250, 0x84230, false)?)?;
    c.emit(0x84254, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x84258, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x8425c, branch(C::Al, 0x8425c, 0x84220, false)?)?;
    Ok(())
}

fn function_84260(c: &mut Code) -> Result<(), String> {
    c.emit(0x84260, transfer_multiple(C::Al, false, r(13), 0x4080, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84264, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x6)?)?;
    c.emit(0x84268, branch(C::Al, 0x84268, 0x84278, true)?)?;
    c.emit(0x8426c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x9)?)?;
    c.emit(0x84270, branch(C::Al, 0x84270, 0x84278, true)?)?;
    c.emit(0x84274, branch(C::Al, 0x84274, 0x84274, false)?)?;
    Ok(())
}

fn function_84278(c: &mut Code) -> Result<(), String> {
    c.emit(0x84278, transfer_multiple(C::Al, false, r(13), 0x40b0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8427c, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x80)?)?;
    c.emit(0x84280, data_register(C::Al, Op::Mov, r(5), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(0x84284, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84288, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x8428c, branch(C::Al, 0x8428c, 0x84550, true)?)?;
    c.emit(0x84290, data_immediate(C::Al, Op::Mov, r(7), r(0), 0xe0)?)?;
    c.emit(0x84294, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84298, supervisor_call(C::Al, 0x0)?)?;
    c.emit(0x8429c, data_immediate(C::Al, Op::Mov, r(7), r(0), 0xee)?)?;
    c.emit(0x842a0, supervisor_call(C::Al, 0x0)?)?;
    c.emit(0x842a4, branch(C::Al, 0x842a4, 0x84510, true)?)?;
    c.emit(0x842a8, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x842ac, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x842b0, branch(C::Al, 0x842b0, 0x84564, true)?)?;
    c.emit(0x842b4, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x842b8, data_immediate(C::Al, Op::Add, r(13), r(13), 0x80)?)?;
    c.emit(0x842bc, transfer_multiple(C::Al, true, r(13), 0x80b0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_842c0(c: &mut Code) -> Result<(), String> {
    c.emit(0x842c0, transfer_multiple(C::Al, false, r(13), 0x40b0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x842c4, data_immediate(C::Al, Op::Add, r(4), r(0), 0x3c)?)?;
    c.emit(0x842c8, data_immediate(C::Al, Op::Mov, r(5), r(0), 0x2)?)?;
    c.emit(0x842cc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x842d0, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(0x842d4, supervisor_call(C::Al, 0x123456)?)?;
    c.emit(0x842d8, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x842dc, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x842e0, transfer_multiple(C::Al, true, r(13), 0x80b0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_842e4(c: &mut Code) -> Result<(), String> {
    c.emit(0x842e4, transfer_multiple(C::Al, false, r(13), 0x47fc, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x842e8, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x842ec, transfer_word(C::Al, true, r(0), r(13), 40)?)?;
    c.emit(0x842f0, data_register(C::Al, Op::Mov, r(5), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x842f4, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x842f8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x842fc, branch(C::Eq, 0x842fc, 0x8433c, false)?)?;
    c.emit(0x84300, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x84304, branch(C::Ne, 0x84304, 0x84358, false)?)?;
    c.emit(0x84308, data_immediate(C::Al, Op::Add, r(5), r(4), 0x3c)?)?;
    c.emit(0x8430c, data_immediate(C::Al, Op::Mov, r(6), r(0), 0xc)?)?;
    c.emit(0x84310, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x84314, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x84318, supervisor_call(C::Al, 0x123456)?)?;
    c.emit(0x8431c, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x84320, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x84324, branch(C::Lt, 0x84324, 0x84330, false)?)?;
    c.emit(0x84328, data_register(C::Al, Op::Mov, r(5), r(0), r(6), Shift::Asr, 31, false)?)?;
    c.emit(0x8432c, branch(C::Al, 0x8432c, 0x84358, false)?)?;
    c.emit(0x84330, data_immediate(C::Al, Op::Mvn, r(8), r(0), 0x0)?)?;
    c.emit(0x84334, data_immediate(C::Al, Op::Mvn, r(5), r(0), 0x0)?)?;
    c.emit(0x84338, branch(C::Al, 0x84338, 0x84394, false)?)?;
    c.emit(0x8433c, transfer_doubleword(C::Al, true, r(8), r(4), 88)?)?;
    c.emit(0x84340, data_register(C::Al, Op::Orr, r(0), r(6), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x84344, branch(C::Ne, 0x84344, 0x84350, false)?)?;
    c.emit(0x84348, data_register(C::Al, Op::Mov, r(5), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x8434c, branch(C::Al, 0x8434c, 0x84394, false)?)?;
    c.emit(0x84350, data_register(C::Al, Op::Add, r(6), r(8), r(6), Shift::Lsl, 0, true)?)?;
    c.emit(0x84354, data_register(C::Al, Op::Adc, r(5), r(9), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x84358, transfer_word(C::Al, true, r(0), r(4), 60)?)?;
    c.emit(0x8435c, data_immediate(C::Al, Op::Mov, r(7), r(0), 0xa)?)?;
    c.emit(0x84360, data_register(C::Al, Op::Mov, r(8), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(0x84364, transfer_multiple(C::Al, false, r(13), 0x41, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x84368, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x8436c, data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, true)?)?;
    c.emit(0x84370, supervisor_call(C::Al, 0x123456)?)?;
    c.emit(0x84374, data_register(C::Al, Op::Mov, r(8), r(0), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x84378, data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.emit(0x8437c, branch(C::Eq, 0x8437c, 0x84388, false)?)?;
    c.emit(0x84380, data_register(C::Al, Op::Mov, r(5), r(0), r(8), Shift::Asr, 31, false)?)?;
    c.emit(0x84384, branch(C::Al, 0x84384, 0x84394, false)?)?;
    c.emit(0x84388, data_register(C::Al, Op::Mov, r(8), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x8438c, transfer_word(C::Al, false, r(6), r(4), 88)?)?;
    c.emit(0x84390, transfer_word(C::Al, false, r(5), r(4), 92)?)?;
    c.emit(0x84394, data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x84398, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x8439c, data_immediate(C::Al, Op::Add, r(13), r(13), 0x8)?)?;
    c.emit(0x843a0, transfer_multiple(C::Al, true, r(13), 0x87f0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_843a4(c: &mut Code) -> Result<(), String> {
    c.emit(0x843a4, transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x843a8, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x24)?)?;
    c.emit(0x843ac, transfer_word(C::Al, true, r(12), r(0), 28)?)?;
    c.emit(0x843b0, data_immediate(C::Al, Op::Add, r(9), r(0), 0x58)?)?;
    c.emit(0x843b4, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x2)?)?;
    c.emit(0x843b8, data_immediate(C::Al, Op::Add, r(6), r(13), 0x14)?)?;
    c.emit(0x843bc, data_immediate(C::Al, Op::Mov, r(11), r(0), 0x5)?)?;
    c.emit(0x843c0, transfer_word(C::Al, false, r(12), r(13), 20)?)?;
    c.emit(0x843c4, transfer_word(C::Al, true, r(3), r(0), 20)?)?;
    c.emit(0x843c8, transfer_word(C::Al, false, r(2), r(13), 0)?)?;
    c.emit(0x843cc, transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(0x843d0, data_register(C::Al, Op::Sub, r(3), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x843d4, data_register(C::Al, Op::Add, r(8), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x843d8, transfer_word(C::Al, false, r(3), r(13), 24)?)?;
    c.emit(0x843dc, transfer_word(C::Al, false, r(1), r(13), 28)?)?;
    c.emit(0x843e0, transfer_word(C::Al, false, r(2), r(13), 32)?)?;
    c.emit(0x843e4, transfer_word(C::Al, true, r(0), r(13), 4)?)?;
    c.emit(0x843e8, transfer_word(C::Al, true, r(4), r(13), 24)?)?;
    c.emit(0x843ec, data_immediate(C::Al, Op::Add, r(5), r(13), 0x8)?)?;
    c.emit(0x843f0, transfer_word(C::Al, true, r(0), r(0), 60)?)?;
    c.emit(0x843f4, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0x843f8, transfer_word(C::Al, true, r(0), r(13), 20)?)?;
    c.emit(0x843fc, transfer_word(C::Al, false, r(0), r(13), 12)?)?;
    c.emit(0x84400, transfer_word(C::Al, false, r(4), r(13), 16)?)?;
    c.emit(0x84404, data_register(C::Al, Op::Mov, r(0), r(0), r(11), Shift::Lsl, 0, true)?)?;
    c.emit(0x84408, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x8440c, supervisor_call(C::Al, 0x123456)?)?;
    c.emit(0x84410, data_register(C::Al, Op::Mov, r(10), r(0), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x84414, transfer_doubleword(C::Al, true, r(0), r(9), 0)?)?;
    c.emit(0x84418, data_register(C::Al, Op::Sub, r(4), r(4), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x8441c, data_register(C::Al, Op::Add, r(0), r(0), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(0x84420, data_register(C::Al, Op::Adc, r(1), r(1), r(4), Shift::Asr, 31, false)?)?;
    c.emit(0x84424, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x2)?)?;
    c.emit(0x84428, transfer_doubleword(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(0x8442c, branch(C::Lt, 0x8442c, 0x84470, false)?)?;
    c.emit(0x84430, transfer_word(C::Al, true, r(0), r(13), 28)?)?;
    c.emit(0x84434, transfer_word(C::Al, true, r(10), r(13), 32)?)?;
    c.emit(0x84438, data_immediate(C::Al, Op::Mov, r(11), r(0), 0x5)?)?;
    c.emit(0x8443c, transfer_word(C::Al, false, r(0), r(13), 12)?)?;
    c.emit(0x84440, transfer_word(C::Al, false, r(10), r(13), 16)?)?;
    c.emit(0x84444, data_register(C::Al, Op::Mov, r(0), r(0), r(11), Shift::Lsl, 0, true)?)?;
    c.emit(0x84448, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x8444c, supervisor_call(C::Al, 0x123456)?)?;
    c.emit(0x84450, data_register(C::Al, Op::Mov, r(11), r(0), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x84454, transfer_doubleword(C::Al, true, r(2), r(9), 0)?)?;
    c.emit(0x84458, data_register(C::Al, Op::Sub, r(0), r(10), r(11), Shift::Lsl, 0, false)?)?;
    c.emit(0x8445c, data_immediate(C::Al, Op::Mov, r(11), r(0), 0x5)?)?;
    c.emit(0x84460, data_register(C::Al, Op::Add, r(2), r(2), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x84464, data_register(C::Al, Op::Add, r(4), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84468, data_register(C::Al, Op::Adc, r(3), r(3), r(0), Shift::Asr, 31, false)?)?;
    c.emit(0x8446c, transfer_doubleword(C::Al, false, r(2), r(9), 0)?)?;
    c.emit(0x84470, data_register(C::Al, Op::Cmp, r(0), r(8), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84474, branch(C::Ne, 0x84474, 0x8449c, false)?)?;
    c.emit(0x84478, transfer_word(C::Al, true, r(2), r(13), 4)?)?;
    c.emit(0x8447c, transfer_word(C::Al, true, r(0), r(2), 44)?)?;
    c.emit(0x84480, transfer_word(C::Al, true, r(1), r(2), 48)?)?;
    c.emit(0x84484, data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84488, transfer_word(C::Al, false, r(1), r(2), 16)?)?;
    c.emit(0x8448c, transfer_word(C::Al, false, r(0), r(2), 28)?)?;
    c.emit(0x84490, transfer_word(C::Al, false, r(0), r(2), 20)?)?;
    c.emit(0x84494, transfer_word(C::Al, true, r(0), r(13), 0)?)?;
    c.emit(0x84498, branch(C::Al, 0x84498, 0x844d4, false)?)?;
    c.emit(0x8449c, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x844a0, branch(C::Ge, 0x844a0, 0x844dc, false)?)?;
    c.emit(0x844a4, transfer_word(C::Al, true, r(2), r(13), 4)?)?;
    c.emit(0x844a8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x844ac, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x2)?)?;
    c.emit(0x844b0, transfer_word(C::Al, false, r(0), r(2), 16)?)?;
    c.emit(0x844b4, transfer_word(C::Al, false, r(0), r(2), 28)?)?;
    c.emit(0x844b8, transfer_word(C::Al, false, r(0), r(2), 20)?)?;
    c.emit(0x844bc, transfer_word(C::Al, true, r(1), r(2), 0)?)?;
    c.emit(0x844c0, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x20)?)?;
    c.emit(0x844c4, transfer_word(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(0x844c8, transfer_word(C::Ne, true, r(0), r(6), 4)?)?;
    c.emit(0x844cc, transfer_word(C::Ne, true, r(1), r(13), 0)?)?;
    c.emit(0x844d0, data_register(C::Ne, Op::Sub, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x844d4, data_immediate(C::Al, Op::Add, r(13), r(13), 0x24)?)?;
    c.emit(0x844d8, transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x844dc, transfer_word(C::Al, true, r(0), r(6), 4)?)?;
    c.emit(0x844e0, data_register(C::Al, Op::Sub, r(8), r(8), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x844e4, data_register(C::Al, Op::Cmp, r(0), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x844e8, data_immediate(C::Hi, Op::Add, r(6), r(6), 0x8)?)?;
    c.emit(0x844ec, data_register(C::Hi, Op::Sub, r(4), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x844f0, data_immediate(C::Hi, Op::Sub, r(7), r(7), 0x1)?)?;
    c.emit(0x844f4, transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(0x844f8, data_register(C::Al, Op::Add, r(0), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x844fc, transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(0x84500, transfer_word(C::Al, true, r(0), r(6), 4)?)?;
    c.emit(0x84504, data_register(C::Al, Op::Sub, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84508, transfer_word(C::Al, false, r(0), r(6), 4)?)?;
    c.emit(0x8450c, branch(C::Al, 0x8450c, 0x843e4, false)?)?;
    Ok(())
}

fn function_84510(c: &mut Code) -> Result<(), String> {
    c.literal(0x84510, 1, 0x84538)?;
    c.emit(0x84514, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84518, bx(C::Cc, r(14)))?;
    c.emit(0x8451c, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84520, data_immediate(C::Al, Op::Rsb, r(4), r(0), 0x0)?)?;
    c.emit(0x84524, branch(C::Al, 0x84524, 0x4a6c, true)?)?;
    c.emit(0x84528, transfer_word(C::Al, false, r(4), r(0), 0)?)?;
    c.emit(0x8452c, data_immediate(C::Al, Op::Mvn, r(0), r(0), 0x0)?)?;
    c.emit(0x84530, transfer_multiple(C::Al, true, r(13), 0x4010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x84534, bx(C::Al, r(14)))?;
    c.word(0x84538, 0xfffff001, true)?;
    Ok(())
}

fn function_8453c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8453c, transfer_multiple(C::Al, false, r(13), 0x4080, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84540, data_immediate(C::Al, Op::Mov, r(7), r(0), 0xaf)?)?;
    c.emit(0x84544, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x8)?)?;
    c.emit(0x84548, supervisor_call(C::Al, 0x0)?)?;
    c.emit(0x8454c, transfer_multiple(C::Al, true, r(13), 0x8080, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_84550(c: &mut Code) -> Result<(), String> {
    c.literal(0x84550, 1, 0x84560)?;
    c.emit(0x84554, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84558, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8455c, branch(C::Al, 0x8455c, 0x8453c, false)?)?;
    c.word(0x84560, 0xf4df4, true)?;
    Ok(())
}

fn function_84564(c: &mut Code) -> Result<(), String> {
    c.emit(0x84564, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84568, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(0x8456c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x84570, branch(C::Al, 0x84570, 0x8453c, false)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    function_83d2c(&mut c)?;
    function_83d6c(&mut c)?;
    function_83d84(&mut c)?;
    function_83dc8(&mut c)?;
    function_84000(&mut c)?;
    function_84030(&mut c)?;
    function_84120(&mut c)?;
    function_84140(&mut c)?;
    function_841e4(&mut c)?;
    function_84210(&mut c)?;
    function_84260(&mut c)?;
    function_84278(&mut c)?;
    function_842c0(&mut c)?;
    function_842e4(&mut c)?;
    function_843a4(&mut c)?;
    function_84510(&mut c)?;
    function_8453c(&mut c)?;
    function_84550(&mut c)?;
    function_84564(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != END - START || c.code_bytes != CODE_BYTES || c.data_bytes != DATA_BYTES {
        return Err("invalid runtime services code/data extent".into());
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
    fn exact_partition_and_fragment_hash() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, FILE_MAPPING_DELTA), (0x83d2c, 0x84574, 0x4000));
        assert_eq!((bytes.len(), CODE_BYTES, DATA_BYTES), (2120, 2072, 48));
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!(LITERALS.len() * 4, DATA_BYTES);
        assert!(LITERALS.windows(2).all(|pair| pair[0].0 < pair[1].0));
        let sections: [(usize, fn(&mut Code) -> Result<(), String>); 19] = [
            (0x83d2c, function_83d2c),
            (0x83d6c, function_83d6c),
            (0x83d84, function_83d84),
            (0x83dc8, function_83dc8),
            (0x84000, function_84000),
            (0x84030, function_84030),
            (0x84120, function_84120),
            (0x84140, function_84140),
            (0x841e4, function_841e4),
            (0x84210, function_84210),
            (0x84260, function_84260),
            (0x84278, function_84278),
            (0x842c0, function_842c0),
            (0x842e4, function_842e4),
            (0x843a4, function_843a4),
            (0x84510, function_84510),
            (0x8453c, function_8453c),
            (0x84550, function_84550),
            (0x84564, function_84564),
        ];
        let mut c = Code::new(START);
        for (index, &(start, emit)) in sections.iter().enumerate() {
            let end = sections.get(index + 1).map_or(END, |&(at, _)| at);
            c.check(start).unwrap();
            let before = (c.code_bytes, c.data_bytes);
            emit(&mut c).unwrap();
            c.check(end).unwrap();
            let data = LITERALS.iter().filter(|&&(at, _)| (start..end).contains(&at)).count() * 4;
            assert_eq!((c.code_bytes - before.0, c.data_bytes - before.1), (end - start - data, data));
            if end == 0x84000 {
                assert_eq!((c.code_bytes, c.data_bytes), (696, 28));
            }
        }
        assert_eq!((c.at, c.code_bytes, c.data_bytes), (END, CODE_BYTES, DATA_BYTES));
        assert_eq!(c.bytes, bytes);
        for (at, value) in LITERALS {
            assert_eq!(word(&bytes, at), value);
        }
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "083626e4bac5e0c0fadc45430826864580a982c51f04fb5d49fddef44f9e9ceb");
    }

    #[test]
    fn branches_and_unresolved_indirect_calls() {
        let bytes = fragment().unwrap();
        let branches = [
            (0x83d54, 0x84000, C::Al, true),
            (0x83d5c, 0x83dc8, C::Al, true),
            (0x83d60, 0x84260, C::Al, true),
            (0x83dd8, 0x83e78, C::Eq, false),
            (0x83de4, 0x83e98, C::Lt, false),
            (0x83dec, 0x4b78, C::Al, true),
            (0x83e00, 0x83e24, C::Ls, false),
            (0x83e20, 0x83fe4, C::Eq, false),
            (0x83e2c, 0x83e4c, C::Cs, false),
            (0x83e68, 0x83fd8, C::Eq, false),
            (0x83e70, 0x4b80, C::Al, true),
            (0x83e74, 0x83fd8, C::Al, false),
            (0x83e84, 0x83f0c, C::Eq, false),
            (0x83e8c, 0x83dc8, C::Al, true),
            (0x83e94, 0x83f10, C::Al, false),
            (0x83ea4, 0x83ec8, C::Ls, false),
            (0x83ec4, 0x83ff4, C::Eq, false),
            (0x83ed0, 0x83ef0, C::Cs, false),
            (0x83f08, 0x83fd8, C::Al, false),
            (0x83f10, 0x4e24, C::Al, true),
            (0x83f1c, 0x83fd4, C::Eq, false),
            (0x83f30, 0x83f40, C::Lt, false),
            (0x83f38, 0x4b78, C::Al, true),
            (0x83f4c, 0x83fb8, C::Ls, false),
            (0x83f6c, 0x83fb0, C::Eq, false),
            (0x83f78, 0x83f94, C::Cs, false),
            (0x83fac, 0x83fb4, C::Al, false),
            (0x83fbc, 0x83fc8, C::Eq, false),
            (0x83fc4, 0x4b80, C::Al, true),
            (0x83fd0, 0x83f24, C::Ne, false),
            (0x83fd4, 0x4e30, C::Al, true),
            (0x83fec, 0x83e6c, C::Ne, false),
            (0x83ff0, 0x83fd8, C::Al, false),
            (0x83ff8, 0x83fd8, C::Al, false),
            (0x8401c, 0x3410, C::Al, true),
            (0x84034, 0x84118, C::Eq, false),
            (0x84048, 0x8411c, C::Ls, false),
            (0x84058, 0x840bc, C::Cs, false),
            (0x84060, 0x84080, C::Eq, false),
            (0x8406c, 0x84118, C::Eq, false),
            (0x8407c, 0x84068, C::Al, false),
            (0x84088, 0x840a4, C::Eq, false),
            (0x84090, 0x84118, C::Eq, false),
            (0x840a0, 0x84084, C::Al, false),
            (0x840a8, 0x84068, C::Cc, false),
            (0x840b8, 0x840a4, C::Al, false),
            (0x840c0, 0x84100, C::Ne, false),
            (0x840cc, 0x840e8, C::Eq, false),
            (0x840d4, 0x84118, C::Eq, false),
            (0x840e4, 0x840c4, C::Al, false),
            (0x840ec, 0x84100, C::Cc, false),
            (0x840fc, 0x840e8, C::Al, false),
            (0x84114, 0x84100, C::Al, false),
            (0x8411c, 0x471c, C::Al, false),
            (0x84128, 0x84140, C::Al, true),
            (0x8414c, 0x84164, C::Ne, false),
            (0x84154, 0x3388, C::Al, true),
            (0x8415c, 0x841d4, C::Al, false),
            (0x84168, 0x84184, C::Eq, false),
            (0x84178, 0x84160, C::Ne, false),
            (0x84180, 0x841d4, C::Al, false),
            (0x841b4, 0x841c0, C::Ne, false),
            (0x841bc, 0x84194, C::Al, false),
            (0x841d0, 0x841c0, C::Ne, false),
            (0x841f0, 0x84208, C::Eq, false),
            (0x841f8, 0x84208, C::Ne, false),
            (0x84204, 0x841e4, C::Al, false),
            (0x84214, 0x84220, C::Ne, false),
            (0x8421c, 0x84238, C::Al, false),
            (0x84228, 0x8423c, C::Ne, false),
            (0x84248, 0x84230, C::Eq, false),
            (0x84250, 0x84230, C::Ne, false),
            (0x8425c, 0x84220, C::Al, false),
            (0x84268, 0x84278, C::Al, true),
            (0x84270, 0x84278, C::Al, true),
            (0x84274, 0x84274, C::Al, false),
            (0x8428c, 0x84550, C::Al, true),
            (0x842a4, 0x84510, C::Al, true),
            (0x842b0, 0x84564, C::Al, true),
            (0x842fc, 0x8433c, C::Eq, false),
            (0x84304, 0x84358, C::Ne, false),
            (0x84324, 0x84330, C::Lt, false),
            (0x8432c, 0x84358, C::Al, false),
            (0x84338, 0x84394, C::Al, false),
            (0x84344, 0x84350, C::Ne, false),
            (0x8434c, 0x84394, C::Al, false),
            (0x8437c, 0x84388, C::Eq, false),
            (0x84384, 0x84394, C::Al, false),
            (0x8442c, 0x84470, C::Lt, false),
            (0x84474, 0x8449c, C::Ne, false),
            (0x84498, 0x844d4, C::Al, false),
            (0x844a0, 0x844dc, C::Ge, false),
            (0x8450c, 0x843e4, C::Al, false),
            (0x84524, 0x4a6c, C::Al, true),
            (0x8455c, 0x8453c, C::Al, false),
            (0x84570, 0x8453c, C::Al, false),
        ];
        assert_eq!(branches.len(), 96);
        for (at, target, condition, link) in branches {
            let instruction = word(&bytes, at);
            assert_eq!((instruction >> 25) & 7, 5);
            assert_eq!(instruction >> 28, condition as u32);
            assert_eq!(instruction & (1 << 24) != 0, link);
            let displacement = ((instruction << 8) as i32 >> 8) * 4;
            assert_eq!((at as i32 + 8 + displacement) as usize, target);
            if (START..END).contains(&target) {
                assert!(!LITERALS.iter().any(|&(slot, _)| slot == target));
            }
        }
        for (at, register) in [
            (0x83e14, 3), (0x83e48, 7), (0x83eb8, 3),
            (0x83eec, 7), (0x83f60, 3), (0x83f90, 7),
        ] {
            assert_eq!(word(&bytes, at), branch_exchange_with_link(C::Al, r(register)).unwrap());
        }
        assert_eq!(word(&bytes, 0x84274), branch(C::Al, 0x84274, 0x84274, false).unwrap());
        assert_eq!(word(&bytes, 0x8402c), bx(C::Al, R::LR));
        assert_eq!(word(&bytes, 0x84570), branch(C::Al, 0x84570, 0x8453c, false).unwrap());
    }

    #[test]
    fn flags_carry_conditions_writeback_and_supervisor_calls() {
        let bytes = fragment().unwrap();
        for (at, condition, flags) in [
            (0x83d98, C::Al, true), (0x83dac, C::Al, false),
            (0x84040, C::Hi, false), (0x84044, C::Hi, true),
            (0x84148, C::Al, true), (0x84174, C::Ne, true),
            (0x841a4, C::Eq, false), (0x841b0, C::Eq, true),
            (0x84244, C::Ne, true), (0x842cc, C::Al, true),
            (0x842d0, C::Al, true), (0x842d8, C::Al, true),
            (0x84340, C::Al, true), (0x84350, C::Al, true),
            (0x84354, C::Al, false), (0x8441c, C::Al, true),
            (0x84420, C::Al, false), (0x84460, C::Al, true),
            (0x84468, C::Al, false), (0x844d0, C::Ne, false),
        ] {
            let instruction = word(&bytes, at);
            assert_eq!(instruction >> 28, condition as u32, "{at:#x}");
            assert_eq!(instruction & (1 << 20) != 0, flags, "{at:#x}");
        }
        for at in [0x84070, 0x84078, 0x84094, 0x8409c, 0x840ac, 0x840b4, 0x841c4] {
            assert_eq!(word(&bytes, at) & (1 << 24), 0);
            assert_eq!(word(&bytes, at) & (1 << 21), 0);
        }
        for at in [0x842d4, 0x84318, 0x84370, 0x8440c, 0x8444c] {
            assert_eq!(word(&bytes, at), supervisor_call(C::Al, 0x123456).unwrap());
        }
        for at in [0x84298, 0x842a0, 0x84548] {
            assert_eq!(word(&bytes, at), supervisor_call(C::Al, 0).unwrap());
        }
    }

    #[test]
    fn literal_references_and_nonmutating_guards() {
        let bytes = fragment().unwrap();
        for (at, slot) in LITERAL_REFERENCES {
            let instruction = word(&bytes, at);
            assert_eq!(instruction & 0xffff_0000, 0xe59f_0000);
            assert_eq!(at + 8 + (instruction & 4095) as usize, slot);
            assert!(LITERALS.iter().any(|&(pool, _)| pool == slot));
            let mut c = Code::new(at);
            c.literal(at, ((instruction >> 12) & 15) as u8, slot).unwrap();
            assert_eq!(c.bytes, instruction.to_le_bytes());
        }
        for &(slot, _) in &LITERALS {
            assert!(LITERAL_REFERENCES.iter().any(|&(_, pool)| pool == slot));
        }
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let pool = LITERALS.iter().find(|&&(slot, _)| slot == at);
            for data in [false, true] {
                for value in [0, pool.map_or(0, |&(_, value)| value), pool.map_or(1, |&(_, value)| value ^ 1)] {
                    let valid = (START..END).contains(&at) && at % 4 == 0
                        && if data { pool.is_some_and(|&(_, expected)| value == expected) } else { pool.is_none() };
                    let mut c = Code::new(at);
                    assert_eq!(c.word(at, value, data).is_ok(), valid);
                    assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes),
                        (at + 4 * usize::from(valid), 4 * usize::from(valid),
                         4 * usize::from(valid && !data), 4 * usize::from(valid && data)));
                }
            }
        }
        for (at, slot) in [(START, 0x83d64), (0x83d34, 0x83d68), (0x83d34, 0x83d65),
            (0x83d34, END), (0x83d64, 0x83d64), (END, 0x84560)] {
            let mut c = Code::new(at);
            assert!(c.literal(at, 0, slot).is_err());
            assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes), (at, 0, 0, 0));
        }
        let mut c = Code::new(START);
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.literal(0x83d34, 0, 0x83d64).is_err());
        assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes), (START, 0, 0, 0));
        assert!(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x12345678).is_err());
        assert!(data_immediate_with_flags(C::Al, Op::Cmp, r(0), r(0), 0, true).is_err());
        assert!(transfer_word(C::Al, true, r(0), R::PC, 4096).is_err());
        assert!(transfer_doubleword(C::Al, true, r(1), r(0), 0).is_err());
        assert!(branch_exchange_with_link(C::Al, R::PC).is_err());
        assert!(supervisor_call(C::Al, 0x1000000).is_err());
        assert!(branch(C::Al, START as u32, START as u32 + 2, false).is_err());
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
