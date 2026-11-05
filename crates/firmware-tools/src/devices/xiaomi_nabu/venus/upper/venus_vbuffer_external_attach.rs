use arm32::{
    branch, data_immediate, data_register, mov_register, mov_shifted_register,
    push_registers, transfer_multiple, transfer_word, transfer_word_postindexed,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7edc8;
pub const END: usize = 0x7f3fc;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1552;
pub const DATA_BYTES: usize = 36;
const POOLS: [(usize, u32); 9] = [
    (0x7efe4, 0xf478b),
    (0x7f074, 0xf478b),
    (0x7f114, 0xf478b),
    (0x7f3e4, 0xf4955),
    (0x7f3e8, 0xf4947),
    (0x7f3ec, 0xe2788),
    (0x7f3f0, 0xf478b),
    (0x7f3f4, 0xf497c),
    (0x7f3f8, 0xf4919),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
    code_words: usize,
    data_words: usize,
}

impl TypedA32 {
    fn new() -> Self {
        Self { at: START, bytes: Vec::with_capacity(END - START), code_words: 0, data_words: 0 }
    }

    fn word(&mut self, at: usize, value: u32, data: bool) -> Result<(), String> {
        let pool = POOLS.iter().find(|(slot, _)| *slot == at).map(|(_, value)| *value);
        if at != self.at || !(START..END).contains(&at) || at % 4 != 0
            || if data { pool != Some(value) } else { pool.is_some() }
        {
            return Err(format!("invalid VBUF {} at {at:#x}, cursor {:#x}", if data { "data" } else { "code" }, self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        if data { self.data_words += 1; } else { self.code_words += 1; }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.word(at, instruction, false)
    }

    fn pool(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.word(at, value, true)
    }

    fn literal(&mut self, at: usize, rd: u8, slot: usize) -> Result<(), String> {
        if !POOLS.iter().any(|(pool, _)| *pool == slot) {
            return Err(format!("invalid VBUF literal {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, r(rd), R::PC, slot as i32 - at as i32 - 8)?)
    }

    fn finish(self) -> Result<Vec<u8>, String> {
        if self.at != END || self.bytes.len() != END - START
            || self.code_words * 4 != CODE_BYTES || self.data_words * 4 != DATA_BYTES
        {
            return Err("incomplete VBUF external attach fragment".into());
        }
        Ok(self.bytes)
    }
}

fn attach_single(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7edc8, push_registers(C::Al, 0x4ff0)?)?;
    c.emit(0x7edcc, data_immediate(C::Al, Op::Add, r(11), r(13), 28)?)?;
    c.emit(0x7edd0, data_immediate(C::Al, Op::Sub, r(13), r(13), 20)?)?;
    c.emit(0x7edd4, transfer_word(C::Al, false, r(2), r(13), 16)?)?;
    c.emit(0x7edd8, transfer_word(C::Al, false, r(1), r(13), 12)?)?;
    c.emit(0x7eddc, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7ede0, mov_register(C::Al, r(6), r(3)))?;
    c.emit(0x7ede4, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7ede8, branch(C::Al, 0x7ede8, 0x2bf4, true)?)?;
    c.emit(0x7edec, transfer_word(C::Al, true, r(0), r(11), 16)?)?;
    c.emit(0x7edf0, transfer_word(C::Al, true, r(10), r(4), 16)?)?;
    c.emit(0x7edf4, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0x7edf8, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0)?)?;
    c.emit(0x7edfc, transfer_word(C::Al, true, r(0), r(11), 12)?)?;
    c.emit(0x7ee00, transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(0x7ee04, transfer_word(C::Al, true, r(0), r(11), 8)?)?;
    c.emit(0x7ee08, transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(0x7ee0c, branch(C::Eq, 0x7ee0c, 0x7ee6c, false)?)?;
    c.literal(0x7ee10, 5, 0x7efe4)?;
    c.emit(0x7ee14, data_immediate(C::Al, Op::Mov, r(7), r(0), 82)?)?;
    c.emit(0x7ee18, data_immediate(C::Al, Op::Mov, r(9), r(0), 0)?)?;
    c.emit(0x7ee1c, data_immediate(C::Al, Op::Mov, r(8), r(0), 0)?)?;
    c.emit(0x7ee20, data_immediate(C::Al, Op::Orr, r(7), r(7), 512)?)?;
    c.emit(0x7ee24, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7ee28, data_register(C::Al, Op::Add, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7ee2c, transfer_word(C::Al, true, r(1), r(0), 12)?)?;
    c.emit(0x7ee30, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x7ee34, branch(C::Eq, 0x7ee34, 0x7ee58, false)?)?;
    c.emit(0x7ee38, transfer_word(C::Al, true, r(0), r(0), 16)?)?;
    c.emit(0x7ee3c, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7ee40, branch(C::Ne, 0x7ee40, 0x7ee58, false)?)?;
    c.emit(0x7ee44, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7ee48, mov_register(C::Al, r(1), r(7)))?;
    c.emit(0x7ee4c, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7ee50, branch(C::Al, 0x7ee50, 0x17b0, true)?)?;
    c.emit(0x7ee54, transfer_word(C::Al, true, r(10), r(4), 16)?)?;
    c.emit(0x7ee58, data_immediate(C::Al, Op::Add, r(8), r(8), 1)?)?;
    c.emit(0x7ee5c, data_immediate(C::Al, Op::Add, r(9), r(9), 40)?)?;
    c.emit(0x7ee60, data_register(C::Al, Op::Cmp, r(0), r(8), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x7ee64, branch(C::Cc, 0x7ee64, 0x7ee24, false)?)?;
    c.emit(0x7ee68, branch(C::Al, 0x7ee68, 0x7ee70, false)?)?;
    c.emit(0x7ee6c, data_immediate(C::Al, Op::Mov, r(10), r(0), 0)?)?;
    c.emit(0x7ee70, transfer_word(C::Al, true, r(0), r(4), 12)?)?;
    c.emit(0x7ee74, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(0x7ee78, data_register(C::Al, Op::Cmp, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x7ee7c, branch(C::Ls, 0x7ee7c, 0x7ef0c, false)?)?;
    c.emit(0x7ee80, mov_register(C::Al, r(1), r(10)))?;
    c.emit(0x7ee84, mov_shifted_register(C::Al, r(10), r(1), Shift::Lsl, 1)?)?;
    c.emit(0x7ee88, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(0x7ee8c, branch(C::Hi, 0x7ee8c, 0x7ee80, false)?)?;
    c.emit(0x7ee90, data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x7ee94, mov_shifted_register(C::Al, r(5), r(0), Shift::Lsl, 4)?)?;
    c.emit(0x7ee98, mov_register(C::Al, r(0), r(5)))?;
    c.emit(0x7ee9c, branch(C::Al, 0x7ee9c, 0x2efc, true)?)?;
    c.emit(0x7eea0, mov_register(C::Al, r(7), r(0)))?;
    c.emit(0x7eea4, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0)?)?;
    c.emit(0x7eea8, branch(C::Eq, 0x7eea8, 0x7ef8c, false)?)?;
    c.emit(0x7eeac, mov_register(C::Al, r(0), r(7)))?;
    c.emit(0x7eeb0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x7eeb4, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7eeb8, data_immediate(C::Al, Op::Mov, r(9), r(0), 0)?)?;
    c.emit(0x7eebc, branch(C::Al, 0x7eebc, 0x31ec, true)?)?;
    c.emit(0x7eec0, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7eec4, transfer_word(C::Al, true, r(1), r(4), 4)?)?;
    c.emit(0x7eec8, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x7eecc, mov_shifted_register(C::Al, r(2), r(0), Shift::Lsl, 3)?)?;
    c.emit(0x7eed0, mov_register(C::Al, r(0), r(7)))?;
    c.emit(0x7eed4, branch(C::Al, 0x7eed4, 0x471c, true)?)?;
    c.emit(0x7eed8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7eedc, branch(C::Al, 0x7eedc, 0x2f1c, true)?)?;
    c.emit(0x7eee0, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0)?)?;
    c.emit(0x7eee4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x7eee8, transfer_word(C::Al, false, r(7), r(4), 4)?)?;
    c.emit(0x7eeec, transfer_word(C::Al, false, r(10), r(4), 16)?)?;
    c.emit(0x7eef0, branch(C::Eq, 0x7eef0, 0x7efac, false)?)?;
    c.emit(0x7eef4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7eef8, transfer_word_postindexed(C::Al, false, r(0), r(7), 40)?)?;
    c.emit(0x7eefc, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(0x7ef00, data_register(C::Al, Op::Cmp, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x7ef04, branch(C::Cc, 0x7ef04, 0x7eef8, false)?)?;
    c.emit(0x7ef08, branch(C::Al, 0x7ef08, 0x7ef14, false)?)?;
    c.emit(0x7ef0c, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0)?)?;
    c.emit(0x7ef10, branch(C::Eq, 0x7ef10, 0x7efa4, false)?)?;
    c.emit(0x7ef14, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7ef18, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x7ef1c, data_immediate(C::Al, Op::Add, r(0), r(0), 36)?)?;
    c.emit(0x7ef20, transfer_word(C::Al, true, r(2), r(0), -24)?)?;
    c.emit(0x7ef24, data_immediate(C::Al, Op::Cmp, r(0), r(2), 1)?)?;
    c.emit(0x7ef28, branch(C::Ne, 0x7ef28, 0x7ef40, false)?)?;
    c.emit(0x7ef2c, data_immediate(C::Al, Op::Add, r(1), r(1), 1)?)?;
    c.emit(0x7ef30, data_immediate(C::Al, Op::Add, r(0), r(0), 40)?)?;
    c.emit(0x7ef34, data_register(C::Al, Op::Cmp, r(0), r(1), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x7ef38, branch(C::Cc, 0x7ef38, 0x7ef20, false)?)?;
    c.emit(0x7ef3c, branch(C::Al, 0x7ef3c, 0x7ef84, false)?)?;
    c.emit(0x7ef40, transfer_word(C::Al, true, r(7), r(13), 0)?)?;
    c.emit(0x7ef44, data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x7ef48, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(0x7ef4c, transfer_word(C::Al, true, r(5), r(13), 4)?)?;
    c.emit(0x7ef50, data_immediate(C::Al, Op::Cmp, r(0), r(7), 1)?)?;
    c.emit(0x7ef54, transfer_word(C::Al, true, r(7), r(13), 16)?)?;
    c.emit(0x7ef58, transfer_word(C::Al, false, r(3), r(0), -32)?)?;
    c.emit(0x7ef5c, transfer_word(C::Al, false, r(3), r(0), -28)?)?;
    c.emit(0x7ef60, transfer_word(C::Al, false, r(2), r(0), -24)?)?;
    c.emit(0x7ef64, transfer_word(C::Al, false, r(6), r(0), -20)?)?;
    c.emit(0x7ef68, transfer_word(C::Al, true, r(2), r(13), 12)?)?;
    c.emit(0x7ef6c, mov_register(C::Ne, r(5), r(3)))?;
    c.emit(0x7ef70, mov_register(C::Ne, r(7), r(3)))?;
    c.emit(0x7ef74, data_immediate(C::Al, Op::Sub, r(3), r(0), 16)?)?;
    c.emit(0x7ef78, transfer_multiple(C::Al, false, r(3), 0x00a4, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x7ef7c, transfer_word(C::Al, true, r(2), r(13), 8)?)?;
    c.emit(0x7ef80, transfer_multiple(C::Al, false, r(0), 0x0014, BlockMode::DecrementAfter, false)?)?;
    c.emit(0x7ef84, mov_register(C::Al, r(9), r(10)))?;
    c.emit(0x7ef88, branch(C::Al, 0x7ef88, 0x7efac, false)?)?;
    c.literal(0x7ef8c, 2, 0x7efe4)?;
    c.emit(0x7ef90, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7ef94, data_immediate(C::Al, Op::Mov, r(1), r(0), 620)?)?;
    c.emit(0x7ef98, data_immediate(C::Al, Op::Sub, r(13), r(11), 28)?)?;
    c.emit(0x7ef9c, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7efa0, branch(C::Al, 0x7efa0, 0x17b0, false)?)?;
    c.emit(0x7efa4, data_immediate(C::Al, Op::Mov, r(9), r(0), 0)?)?;
    c.emit(0x7efa8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.literal(0x7efac, 2, 0x7efe4)?;
    c.emit(0x7efb0, data_register(C::Al, Op::Cmp, r(0), r(1), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7efb4, data_immediate(C::Al, Op::Mov, r(1), r(0), 163)?)?;
    c.emit(0x7efb8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7efbc, data_immediate(C::Cc, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x7efc0, data_immediate(C::Al, Op::Orr, r(1), r(1), 512)?)?;
    c.emit(0x7efc4, branch(C::Al, 0x7efc4, 0x17b0, true)?)?;
    c.emit(0x7efc8, transfer_word(C::Al, true, r(0), r(4), 12)?)?;
    c.emit(0x7efcc, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(0x7efd0, transfer_word(C::Al, false, r(0), r(4), 12)?)?;
    c.emit(0x7efd4, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7efd8, data_immediate(C::Al, Op::Sub, r(13), r(11), 28)?)?;
    c.emit(0x7efdc, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7efe0, branch(C::Al, 0x7efe0, 0x2c34, false)?)?;
    c.pool(0x7efe4, 0xf478b)?;
    Ok(())
}

fn mark_external(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7efe8, push_registers(C::Al, 0x48f0)?)?;
    c.emit(0x7efec, data_immediate(C::Al, Op::Add, r(11), r(13), 16)?)?;
    c.emit(0x7eff0, mov_register(C::Al, r(6), r(0)))?;
    c.emit(0x7eff4, mov_register(C::Al, r(5), r(1)))?;
    c.emit(0x7eff8, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0x7effc, transfer_multiple(C::Eq, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f000, transfer_word(C::Al, true, r(0), r(6), 0)?)?;
    c.emit(0x7f004, branch(C::Al, 0x7f004, 0x2bf4, true)?)?;
    c.emit(0x7f008, transfer_word(C::Al, true, r(1), r(6), 16)?)?;
    c.emit(0x7f00c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7f010, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x7f014, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x7f018, branch(C::Eq, 0x7f018, 0x7f050, false)?)?;
    c.emit(0x7f01c, transfer_word(C::Al, true, r(2), r(6), 4)?)?;
    c.emit(0x7f020, data_immediate(C::Al, Op::Add, r(3), r(2), 8)?)?;
    c.emit(0x7f024, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x7f028, transfer_word(C::Al, true, r(4), r(3), 8)?)?;
    c.emit(0x7f02c, data_register(C::Al, Op::Cmp, r(0), r(4), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f030, branch(C::Eq, 0x7f030, 0x7f048, false)?)?;
    c.emit(0x7f034, data_immediate(C::Al, Op::Add, r(2), r(2), 1)?)?;
    c.emit(0x7f038, data_immediate(C::Al, Op::Add, r(3), r(3), 40)?)?;
    c.emit(0x7f03c, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f040, branch(C::Cc, 0x7f040, 0x7f028, false)?)?;
    c.emit(0x7f044, branch(C::Al, 0x7f044, 0x7f050, false)?)?;
    c.emit(0x7f048, data_immediate(C::Al, Op::Mov, r(5), r(0), 1)?)?;
    c.emit(0x7f04c, transfer_word(C::Al, false, r(5), r(3), 0)?)?;
    c.emit(0x7f050, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(0x7f054, 2, 0x7f074)?;
    c.emit(0x7f058, data_immediate(C::Al, Op::Mov, r(1), r(0), 203)?)?;
    c.emit(0x7f05c, data_immediate(C::Cc, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x7f060, data_immediate(C::Al, Op::Orr, r(1), r(1), 512)?)?;
    c.emit(0x7f064, branch(C::Al, 0x7f064, 0x17b0, true)?)?;
    c.emit(0x7f068, transfer_word(C::Al, true, r(0), r(6), 0)?)?;
    c.emit(0x7f06c, transfer_multiple(C::Al, true, R::SP, 0x48f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f070, branch(C::Al, 0x7f070, 0x2c34, false)?)?;
    c.pool(0x7f074, 0xf478b)?;
    Ok(())
}

fn query_external(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f078, push_registers(C::Al, 0x48f0)?)?;
    c.emit(0x7f07c, data_immediate(C::Al, Op::Add, r(11), r(13), 16)?)?;
    c.emit(0x7f080, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7f084, mov_register(C::Al, r(5), r(1)))?;
    c.emit(0x7f088, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.emit(0x7f08c, branch(C::Eq, 0x7f08c, 0x7f0e0, false)?)?;
    c.emit(0x7f090, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f094, branch(C::Al, 0x7f094, 0x2bf4, true)?)?;
    c.emit(0x7f098, transfer_word(C::Al, true, r(1), r(4), 16)?)?;
    c.emit(0x7f09c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7f0a0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x7f0a4, data_immediate(C::Al, Op::Mov, r(6), r(0), 0)?)?;
    c.emit(0x7f0a8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x7f0ac, branch(C::Eq, 0x7f0ac, 0x7f0ec, false)?)?;
    c.emit(0x7f0b0, transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.emit(0x7f0b4, data_immediate(C::Al, Op::Add, r(3), r(2), 8)?)?;
    c.emit(0x7f0b8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x7f0bc, transfer_word(C::Al, true, r(6), r(3), 8)?)?;
    c.emit(0x7f0c0, data_register(C::Al, Op::Cmp, r(0), r(6), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f0c4, branch(C::Eq, 0x7f0c4, 0x7f0e8, false)?)?;
    c.emit(0x7f0c8, data_immediate(C::Al, Op::Add, r(2), r(2), 1)?)?;
    c.emit(0x7f0cc, data_immediate(C::Al, Op::Add, r(3), r(3), 40)?)?;
    c.emit(0x7f0d0, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f0d4, branch(C::Cc, 0x7f0d4, 0x7f0bc, false)?)?;
    c.emit(0x7f0d8, data_immediate(C::Al, Op::Mov, r(6), r(0), 0)?)?;
    c.emit(0x7f0dc, branch(C::Al, 0x7f0dc, 0x7f0ec, false)?)?;
    c.emit(0x7f0e0, data_immediate(C::Al, Op::Mov, r(6), r(0), 0)?)?;
    c.emit(0x7f0e4, branch(C::Al, 0x7f0e4, 0x7f10c, false)?)?;
    c.emit(0x7f0e8, transfer_word(C::Al, true, r(6), r(3), 0)?)?;
    c.emit(0x7f0ec, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(0x7f0f0, 2, 0x7f114)?;
    c.emit(0x7f0f4, data_immediate(C::Al, Op::Mov, r(1), r(0), 242)?)?;
    c.emit(0x7f0f8, data_immediate(C::Cc, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x7f0fc, data_immediate(C::Al, Op::Orr, r(1), r(1), 512)?)?;
    c.emit(0x7f100, branch(C::Al, 0x7f100, 0x17b0, true)?)?;
    c.emit(0x7f104, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f108, branch(C::Al, 0x7f108, 0x2c34, true)?)?;
    c.emit(0x7f10c, mov_register(C::Al, r(0), r(6)))?;
    c.emit(0x7f110, transfer_multiple(C::Al, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.pool(0x7f114, 0xf478b)?;
    Ok(())
}

fn attach_wrapper(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f118, push_registers(C::Al, 0x4800)?)?;
    c.emit(0x7f11c, mov_register(C::Al, r(11), r(13)))?;
    c.emit(0x7f120, data_immediate(C::Al, Op::Sub, r(13), r(13), 16)?)?;
    c.emit(0x7f124, data_immediate(C::Al, Op::Mov, r(12), r(0), 0)?)?;
    c.emit(0x7f128, transfer_word(C::Al, false, r(12), r(13), 8)?)?;
    c.emit(0x7f12c, transfer_word(C::Al, true, r(12), r(11), 8)?)?;
    c.emit(0x7f130, transfer_multiple(C::Al, false, r(13), 0x1008, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x7f134, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(0x7f138, branch(C::Al, 0x7f138, 0x7f144, true)?)?;
    c.emit(0x7f13c, mov_register(C::Al, r(13), r(11)))?;
    c.emit(0x7f140, transfer_multiple(C::Al, true, R::SP, 0x8800, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn attach_ex(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f144, push_registers(C::Al, 0x4ff0)?)?;
    c.emit(0x7f148, data_immediate(C::Al, Op::Add, r(11), r(13), 28)?)?;
    c.emit(0x7f14c, data_immediate(C::Al, Op::Sub, r(13), r(13), 20)?)?;
    c.emit(0x7f150, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7f154, mov_register(C::Al, r(5), r(1)))?;
    c.emit(0x7f158, transfer_word(C::Al, false, r(3), r(13), 4)?)?;
    c.emit(0x7f15c, transfer_word(C::Al, false, r(2), r(13), 12)?)?;
    c.emit(0x7f160, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.emit(0x7f164, branch(C::Eq, 0x7f164, 0x7f384, false)?)?;
    c.emit(0x7f168, transfer_word(C::Al, true, r(0), r(11), 16)?)?;
    c.emit(0x7f16c, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0x7f170, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f174, transfer_word(C::Al, true, r(10), r(11), 12)?)?;
    c.emit(0x7f178, transfer_word(C::Al, true, r(8), r(11), 8)?)?;
    c.emit(0x7f17c, branch(C::Al, 0x7f17c, 0x2bf4, true)?)?;
    c.emit(0x7f180, transfer_word(C::Al, true, r(0), r(4), 12)?)?;
    c.emit(0x7f184, transfer_word(C::Al, true, r(7), r(4), 16)?)?;
    c.emit(0x7f188, transfer_word(C::Al, false, r(5), r(13), 16)?)?;
    c.emit(0x7f18c, data_register(C::Al, Op::Add, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f190, data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f194, branch(C::Ls, 0x7f194, 0x7f254, false)?)?;
    c.literal(0x7f198, 1, 0x7f3e4)?;
    c.literal(0x7f19c, 2, 0x7f3e8)?;
    c.emit(0x7f1a0, data_immediate(C::Al, Op::Mov, r(3), r(0), 42)?)?;
    c.emit(0x7f1a4, data_immediate(C::Al, Op::Mov, r(0), r(0), 8)?)?;
    c.emit(0x7f1a8, data_immediate(C::Al, Op::Orr, r(3), r(3), 1024)?)?;
    c.emit(0x7f1ac, branch(C::Al, 0x7f1ac, 0x2ec94, true)?)?;
    c.literal(0x7f1b0, 1, 0x7f3ec)?;
    c.emit(0x7f1b4, data_immediate(C::Al, Op::Mov, r(0), r(0), 8)?)?;
    c.emit(0x7f1b8, branch(C::Al, 0x7f1b8, 0x2ec94, true)?)?;
    c.emit(0x7f1bc, transfer_word(C::Al, true, r(0), r(4), 12)?)?;
    c.emit(0x7f1c0, transfer_word(C::Al, true, r(1), r(13), 16)?)?;
    c.emit(0x7f1c4, transfer_word(C::Al, true, r(6), r(4), 16)?)?;
    c.emit(0x7f1c8, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f1cc, mov_register(C::Al, r(1), r(6)))?;
    c.emit(0x7f1d0, mov_shifted_register(C::Al, r(6), r(1), Shift::Lsl, 1)?)?;
    c.emit(0x7f1d4, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(0x7f1d8, branch(C::Hi, 0x7f1d8, 0x7f1cc, false)?)?;
    c.emit(0x7f1dc, data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f1e0, mov_shifted_register(C::Al, r(9), r(0), Shift::Lsl, 4)?)?;
    c.emit(0x7f1e4, mov_register(C::Al, r(0), r(9)))?;
    c.emit(0x7f1e8, branch(C::Al, 0x7f1e8, 0x2efc, true)?)?;
    c.emit(0x7f1ec, mov_register(C::Al, r(5), r(0)))?;
    c.emit(0x7f1f0, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0)?)?;
    c.emit(0x7f1f4, branch(C::Eq, 0x7f1f4, 0x7f3b0, false)?)?;
    c.emit(0x7f1f8, mov_register(C::Al, r(0), r(5)))?;
    c.emit(0x7f1fc, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x7f200, mov_register(C::Al, r(2), r(9)))?;
    c.emit(0x7f204, data_immediate(C::Al, Op::Mov, r(7), r(0), 0)?)?;
    c.emit(0x7f208, branch(C::Al, 0x7f208, 0x31ec, true)?)?;
    c.emit(0x7f20c, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7f210, transfer_word(C::Al, true, r(1), r(4), 4)?)?;
    c.emit(0x7f214, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f218, mov_shifted_register(C::Al, r(2), r(0), Shift::Lsl, 3)?)?;
    c.emit(0x7f21c, mov_register(C::Al, r(0), r(5)))?;
    c.emit(0x7f220, branch(C::Al, 0x7f220, 0x471c, true)?)?;
    c.emit(0x7f224, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f228, branch(C::Al, 0x7f228, 0x2f1c, true)?)?;
    c.emit(0x7f22c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0x7f230, transfer_word(C::Al, false, r(5), r(4), 4)?)?;
    c.emit(0x7f234, transfer_word(C::Al, false, r(6), r(4), 16)?)?;
    c.emit(0x7f238, branch(C::Eq, 0x7f238, 0x7f254, false)?)?;
    c.emit(0x7f23c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7f240, transfer_word_postindexed(C::Al, false, r(0), r(5), 40)?)?;
    c.emit(0x7f244, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(0x7f248, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f24c, branch(C::Cc, 0x7f24c, 0x7f240, false)?)?;
    c.emit(0x7f250, mov_register(C::Al, r(7), r(6)))?;
    c.emit(0x7f254, data_immediate(C::Al, Op::Cmp, r(0), r(10), 1)?)?;
    c.emit(0x7f258, transfer_word(C::Al, true, r(1), r(13), 16)?)?;
    c.emit(0x7f25c, data_immediate(C::Al, Op::Mov, r(9), r(0), 1)?)?;
    c.emit(0x7f260, transfer_word(C::Ne, true, r(0), r(4), 24)?)?;
    c.emit(0x7f264, data_immediate(C::Ne, Op::Mov, r(9), r(0), 0)?)?;
    c.emit(0x7f268, data_register(C::Ne, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f26c, transfer_word(C::Ne, false, r(0), r(4), 24)?)?;
    c.emit(0x7f270, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x7f274, transfer_word(C::Al, true, r(10), r(4), 12)?)?;
    c.emit(0x7f278, branch(C::Eq, 0x7f278, 0x7f370, false)?)?;
    c.emit(0x7f27c, data_register(C::Al, Op::Cmp, r(0), r(10), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f280, branch(C::Cs, 0x7f280, 0x7f370, false)?)?;
    c.emit(0x7f284, data_register(C::Al, Op::Add, r(0), r(10), r(10), Shift::Lsl, 2, false)?)?;
    c.emit(0x7f288, data_immediate(C::Al, Op::Mov, r(1), r(0), 20)?)?;
    c.emit(0x7f28c, data_immediate(C::Al, Op::Mov, r(5), r(0), 1)?)?;
    c.emit(0x7f290, mov_register(C::Al, r(7), r(8)))?;
    c.emit(0x7f294, data_register(C::Al, Op::Add, r(6), r(1), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x7f298, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f29c, data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.literal(0x7f2a0, 2, 0x7f3f0)?;
    c.emit(0x7f2a4, data_immediate(C::Al, Op::Mov, r(1), r(0), 94)?)?;
    c.emit(0x7f2a8, transfer_word(C::Al, true, r(0), r(0), -8)?)?;
    c.emit(0x7f2ac, data_immediate(C::Al, Op::Orr, r(1), r(1), 1024)?)?;
    c.emit(0x7f2b0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x7f2b4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7f2b8, data_immediate(C::Eq, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x7f2bc, branch(C::Al, 0x7f2bc, 0x17b0, true)?)?;
    c.emit(0x7f2c0, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f2c4, data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x7f2c8, data_register(C::Al, Op::Add, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f2cc, transfer_word(C::Al, false, r(2), r(1), -8)?)?;
    c.emit(0x7f2d0, transfer_word(C::Al, false, r(9), r(1), -16)?)?;
    c.emit(0x7f2d4, transfer_word(C::Al, true, r(2), r(13), 12)?)?;
    c.emit(0x7f2d8, transfer_word_register(C::Al, false, r(2), r(0), r(6), Shift::Lsl, 0)?)?;
    c.emit(0x7f2dc, transfer_word(C::Al, true, r(2), r(13), 8)?)?;
    c.emit(0x7f2e0, data_immediate(C::Al, Op::Cmp, r(0), r(2), 1)?)?;
    c.emit(0x7f2e4, branch(C::Ne, 0x7f2e4, 0x7f308, false)?)?;
    c.emit(0x7f2e8, transfer_word(C::Al, true, r(2), r(7), 0)?)?;
    c.emit(0x7f2ec, transfer_word(C::Al, false, r(2), r(1), -4)?)?;
    c.emit(0x7f2f0, transfer_word(C::Al, true, r(2), r(7), 4)?)?;
    c.emit(0x7f2f4, data_immediate(C::Al, Op::Add, r(7), r(7), 8)?)?;
    c.emit(0x7f2f8, transfer_word(C::Al, false, r(2), r(1), 4)?)?;
    c.emit(0x7f2fc, transfer_word(C::Al, true, r(2), r(13), 4)?)?;
    c.emit(0x7f300, transfer_word(C::Al, false, r(2), r(1), 8)?)?;
    c.emit(0x7f304, branch(C::Al, 0x7f304, 0x7f344, false)?)?;
    c.emit(0x7f308, transfer_word(C::Al, true, r(1), r(8), 0)?)?;
    c.emit(0x7f30c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x7f310, branch(C::Ne, 0x7f310, 0x7f330, false)?)?;
    c.literal(0x7f314, 2, 0x7f3f0)?;
    c.emit(0x7f318, data_immediate(C::Al, Op::Mov, r(1), r(0), 111)?)?;
    c.emit(0x7f31c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7f320, data_immediate(C::Al, Op::Orr, r(1), r(1), 1024)?)?;
    c.emit(0x7f324, branch(C::Al, 0x7f324, 0x17b0, true)?)?;
    c.emit(0x7f328, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f32c, transfer_word(C::Al, true, r(1), r(8), 0)?)?;
    c.emit(0x7f330, data_register(C::Al, Op::Add, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f334, transfer_word(C::Al, false, r(1), r(2), -4)?)?;
    c.emit(0x7f338, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x7f33c, transfer_word(C::Al, false, r(1), r(2), 4)?)?;
    c.emit(0x7f340, transfer_word(C::Al, false, r(1), r(2), 8)?)?;
    c.emit(0x7f344, data_register(C::Al, Op::Add, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f348, data_immediate(C::Al, Op::Add, r(10), r(10), 1)?)?;
    c.emit(0x7f34c, transfer_word(C::Al, false, r(4), r(1), 16)?)?;
    c.emit(0x7f350, transfer_word(C::Al, true, r(1), r(13), 16)?)?;
    c.emit(0x7f354, data_register(C::Al, Op::Cmp, r(0), r(5), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f358, transfer_word(C::Cc, true, r(1), r(4), 16)?)?;
    c.emit(0x7f35c, data_immediate(C::Cc, Op::Add, r(8), r(8), 4)?)?;
    c.emit(0x7f360, data_immediate(C::Cc, Op::Add, r(5), r(5), 1)?)?;
    c.emit(0x7f364, data_immediate(C::Cc, Op::Add, r(6), r(6), 40)?)?;
    c.emit(0x7f368, data_register(C::Cc, Op::Cmp, r(0), r(10), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f36c, branch(C::Cc, 0x7f36c, 0x7f29c, false)?)?;
    c.emit(0x7f370, transfer_word(C::Al, false, r(10), r(4), 12)?)?;
    c.emit(0x7f374, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f378, data_immediate(C::Al, Op::Sub, r(13), r(11), 28)?)?;
    c.emit(0x7f37c, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f380, branch(C::Al, 0x7f380, 0x2c34, false)?)?;
    c.literal(0x7f384, 1, 0x7f3f8)?;
    c.literal(0x7f388, 2, 0x7f3e8)?;
    c.emit(0x7f38c, data_immediate(C::Al, Op::Mov, r(3), r(0), 30)?)?;
    c.emit(0x7f390, data_immediate(C::Al, Op::Mov, r(0), r(0), 16)?)?;
    c.emit(0x7f394, data_immediate(C::Al, Op::Orr, r(3), r(3), 1024)?)?;
    c.emit(0x7f398, branch(C::Al, 0x7f398, 0x2ec94, true)?)?;
    c.literal(0x7f39c, 1, 0x7f3ec)?;
    c.emit(0x7f3a0, data_immediate(C::Al, Op::Mov, r(0), r(0), 16)?)?;
    c.emit(0x7f3a4, branch(C::Al, 0x7f3a4, 0x2ec94, true)?)?;
    c.emit(0x7f3a8, data_immediate(C::Al, Op::Mov, r(1), r(0), 31)?)?;
    c.emit(0x7f3ac, branch(C::Al, 0x7f3ac, 0x7f3cc, false)?)?;
    c.literal(0x7f3b0, 1, 0x7f3f4)?;
    c.emit(0x7f3b4, data_immediate(C::Al, Op::Mov, r(0), r(0), 4)?)?;
    c.emit(0x7f3b8, branch(C::Al, 0x7f3b8, 0x2ec94, true)?)?;
    c.literal(0x7f3bc, 1, 0x7f3ec)?;
    c.emit(0x7f3c0, data_immediate(C::Al, Op::Mov, r(0), r(0), 4)?)?;
    c.emit(0x7f3c4, branch(C::Al, 0x7f3c4, 0x2ec94, true)?)?;
    c.emit(0x7f3c8, data_immediate(C::Al, Op::Mov, r(1), r(0), 59)?)?;
    c.literal(0x7f3cc, 2, 0x7f3f0)?;
    c.emit(0x7f3d0, data_immediate(C::Al, Op::Orr, r(1), r(1), 1024)?)?;
    c.emit(0x7f3d4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x7f3d8, data_immediate(C::Al, Op::Sub, r(13), r(11), 28)?)?;
    c.emit(0x7f3dc, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f3e0, branch(C::Al, 0x7f3e0, 0x17b0, false)?)?;
    c.pool(0x7f3e4, 0xf4955)?;
    c.pool(0x7f3e8, 0xf4947)?;
    c.pool(0x7f3ec, 0xe2788)?;
    c.pool(0x7f3f0, 0xf478b)?;
    c.pool(0x7f3f4, 0xf497c)?;
    c.pool(0x7f3f8, 0xf4919)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32::new();
    attach_single(&mut c)?;
    mark_external(&mut c)?;
    query_external(&mut c)?;
    attach_wrapper(&mut c)?;
    attach_ex(&mut c)?;
    c.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn extents_counts_and_pools() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 1588);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(CODE_BYTES / 4, 388);
        assert_eq!(DATA_BYTES, POOLS.len() * 4);
        for (slot, value) in POOLS {
            assert_eq!(word(&bytes, slot), value);
        }
        let mut c = TypedA32::new();
        attach_single(&mut c).unwrap();
        assert_eq!(c.at, 0x7efe8);
        mark_external(&mut c).unwrap();
        assert_eq!(c.at, 0x7f078);
        query_external(&mut c).unwrap();
        assert_eq!(c.at, 0x7f118);
        attach_wrapper(&mut c).unwrap();
        assert_eq!(c.at, 0x7f144);
        attach_ex(&mut c).unwrap();
        assert_eq!((c.code_words, c.data_words), (388, 9));
        c.finish().unwrap();
    }

    #[test]
    fn exact_guards_do_not_mutate_on_failure() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let pool = POOLS.iter().find(|(slot, _)| *slot == at);
            let valid = (START..END).contains(&at) && at % 4 == 0;
            for data in [false, true] {
                for value in [0, pool.map_or(0, |(_, value)| *value), pool.map_or(1, |(_, value)| *value ^ 1)] {
                    let mut c = TypedA32::new();
                    c.at = at;
                    let ok = valid && if data { pool.is_some_and(|(_, expected)| *expected == value) } else { pool.is_none() };
                    assert_eq!(c.word(at, value, data).is_ok(), ok);
                    assert_eq!(c.at, at + if ok { 4 } else { 0 });
                    assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
                    assert_eq!(c.code_words, usize::from(ok && !data));
                    assert_eq!(c.data_words, usize::from(ok && data));
                }
            }
        }
        let mut c = TypedA32::new();
        assert!(c.emit(START + 4, mov_register(C::Al, r(0), r(0))).is_err());
        assert!(c.pool(START, 0xf478b).is_err());
        assert!(c.literal(START, 0, END).is_err());
        assert_eq!((c.at, c.bytes.len(), c.code_words, c.data_words), (START, 0, 0, 0));
        assert!(c.finish().is_err());
    }

    #[test]
    fn doubling_assertion_continuations_and_cold_paths() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x7ee50, C::Al, 0x17b0, true),
            (0x7ee8c, C::Hi, 0x7ee80, false),
            (0x7eea8, C::Eq, 0x7ef8c, false),
            (0x7ef10, C::Eq, 0x7efa4, false),
            (0x7efa0, C::Al, 0x17b0, false),
            (0x7efc4, C::Al, 0x17b0, true),
            (0x7f064, C::Al, 0x17b0, true),
            (0x7f100, C::Al, 0x17b0, true),
            (0x7f138, C::Al, 0x7f144, true),
            (0x7f164, C::Eq, 0x7f384, false),
            (0x7f1d8, C::Hi, 0x7f1cc, false),
            (0x7f1f4, C::Eq, 0x7f3b0, false),
            (0x7f2bc, C::Al, 0x17b0, true),
            (0x7f324, C::Al, 0x17b0, true),
            (0x7f36c, C::Cc, 0x7f29c, false),
            (0x7f380, C::Al, 0x2c34, false),
            (0x7f398, C::Al, 0x2ec94, true),
            (0x7f3a4, C::Al, 0x2ec94, true),
            (0x7f3ac, C::Al, 0x7f3cc, false),
            (0x7f3b8, C::Al, 0x2ec94, true),
            (0x7f3c4, C::Al, 0x2ec94, true),
            (0x7f3e0, C::Al, 0x17b0, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        for (at, rd, rn, offset) in [
            (0x7ee54, 10, 4, 16),
            (0x7efc8, 0, 4, 12),
            (0x7f068, 0, 6, 0),
            (0x7f104, 0, 4, 0),
            (0x7f2c0, 0, 4, 4),
            (0x7f328, 0, 4, 4),
            (0x7f32c, 1, 8, 0),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(rd), r(rn), offset).unwrap());
        }
        for (at, rd) in [(0x7ee84, 10), (0x7f1d0, 6)] {
            assert_eq!(word(&bytes, at), mov_shifted_register(C::Al, r(rd), r(1), Shift::Lsl, 1).unwrap());
            assert_eq!(word(&bytes, at + 4), data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 1, false).unwrap());
        }
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
