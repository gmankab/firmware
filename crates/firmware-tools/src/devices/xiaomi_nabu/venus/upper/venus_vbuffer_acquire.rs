use arm32::{
    branch, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7f7bc;
pub const END: usize = 0x7fd1c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1304;
pub const DATA_BYTES: usize = 72;
const POOLS: [(usize, u32); 18] = [
    (0x7f888, 0xf478b),
    (0x7f88c, 0xf48cc),
    (0x7f890, 0xf48ed),
    (0x7f894, 0xe2788),
    (0x7f964, 0xf478b),
    (0x7f968, 0xf48cc),
    (0x7f96c, 0xf4905),
    (0x7f970, 0xe2788),
    (0x7fb2c, 0xf478b),
    (0x7fb30, 0xf49d3),
    (0x7fb34, 0xe2788),
    (0x7fb38, 0xf499d),
    (0x7fb3c, 0xf49c8),
    (0x7fd08, 0xf478b),
    (0x7fd0c, 0xf49d3),
    (0x7fd10, 0xe2788),
    (0x7fd14, 0xf499d),
    (0x7fd18, 0xf4a03),
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
            return Err(format!("invalid VBUF acquire {} at {at:#x}, cursor {:#x}", if data { "data" } else { "code" }, self.at));
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
            return Err(format!("invalid VBUF acquire literal {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, r(rd), R::PC, slot as i32 - at as i32 - 8)?)
    }

    fn finish(self) -> Result<Vec<u8>, String> {
        if self.at != END || self.bytes.len() != END - START
            || self.code_words * 4 != CODE_BYTES || self.data_words * 4 != DATA_BYTES
        {
            return Err("incomplete VBUF acquire fragment".into());
        }
        Ok(self.bytes)
    }
}

fn validate_acquired(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f7bc, push_registers(C::Al, 0x4ff0)?)?;
    c.emit(0x7f7c0, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x7f7c4, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(0x7f7c8, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7f7cc, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f7d0, branch(C::Al, 0x7f7d0, 0x2bf4, true)?)?;
    c.emit(0x7f7d4, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7f7d8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f7dc, branch(C::Eq, 0x7f7dc, 0x7f878, false)?)?;
    c.literal(0x7f7e0, 5, 0x7f888)?;
    c.literal(0x7f7e4, 9, 0x7f890)?;
    c.literal(0x7f7e8, 7, 0x7f894)?;
    c.emit(0x7f7ec, data_immediate(C::Al, Op::Mov, r(10), r(0), 0xdf)?)?;
    c.emit(0x7f7f0, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x7f7f4, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(0x7f7f8, data_immediate(C::Al, Op::Orr, r(10), r(10), 0x300)?)?;
    c.emit(0x7f7fc, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f800, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3dc)?)?;
    c.emit(0x7f804, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7f808, transfer_word_register(C::Al, true, r(0), r(0), r(6), Shift::Lsl, 0)?)?;
    c.emit(0x7f80c, data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f810, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f814, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7f818, branch(C::Al, 0x7f818, 0x17b0, true)?)?;
    c.emit(0x7f81c, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f820, data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f824, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0x7f828, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x7f82c, branch(C::Eq, 0x7f82c, 0x7f864, false)?)?;
    c.literal(0x7f830, 1, 0x7f88c)?;
    c.emit(0x7f834, transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(0x7f838, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(0x7f83c, mov_register(C::Al, r(2), r(9)))?;
    c.emit(0x7f840, mov_register(C::Al, r(3), r(10)))?;
    c.emit(0x7f844, branch(C::Al, 0x7f844, 0x2ec94, true)?)?;
    c.emit(0x7f848, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(0x7f84c, mov_register(C::Al, r(1), r(7)))?;
    c.emit(0x7f850, branch(C::Al, 0x7f850, 0x2ec94, true)?)?;
    c.emit(0x7f854, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f858, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3e0)?)?;
    c.emit(0x7f85c, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7f860, branch(C::Al, 0x7f860, 0x17b0, true)?)?;
    c.emit(0x7f864, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7f868, data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(0x7f86c, data_immediate(C::Al, Op::Add, r(6), r(6), 0x28)?)?;
    c.emit(0x7f870, data_register(C::Al, Op::Cmp, r(0), r(8), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f874, branch(C::Cc, 0x7f874, 0x7f7fc, false)?)?;
    c.emit(0x7f878, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f87c, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7f880, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f884, branch(C::Al, 0x7f884, 0x2c34, false)?)?;
    c.pool(0x7f888, 0xf478b)?;
    c.pool(0x7f88c, 0xf48cc)?;
    c.pool(0x7f890, 0xf48ed)?;
    c.pool(0x7f894, 0xe2788)?;
    Ok(())
}

fn validate_available(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f898, push_registers(C::Al, 0x4ff0)?)?;
    c.emit(0x7f89c, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x7f8a0, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(0x7f8a4, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7f8a8, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f8ac, branch(C::Al, 0x7f8ac, 0x2bf4, true)?)?;
    c.emit(0x7f8b0, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7f8b4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f8b8, branch(C::Eq, 0x7f8b8, 0x7f954, false)?)?;
    c.literal(0x7f8bc, 5, 0x7f964)?;
    c.literal(0x7f8c0, 9, 0x7f96c)?;
    c.literal(0x7f8c4, 7, 0x7f970)?;
    c.emit(0x7f8c8, data_immediate(C::Al, Op::Mov, r(10), r(0), 0xf3)?)?;
    c.emit(0x7f8cc, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x7f8d0, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(0x7f8d4, data_immediate(C::Al, Op::Orr, r(10), r(10), 0x300)?)?;
    c.emit(0x7f8d8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f8dc, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3f0)?)?;
    c.emit(0x7f8e0, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7f8e4, transfer_word_register(C::Al, true, r(0), r(0), r(6), Shift::Lsl, 0)?)?;
    c.emit(0x7f8e8, data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f8ec, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f8f0, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7f8f4, branch(C::Al, 0x7f8f4, 0x17b0, true)?)?;
    c.emit(0x7f8f8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7f8fc, data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f900, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0x7f904, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f908, branch(C::Eq, 0x7f908, 0x7f940, false)?)?;
    c.literal(0x7f90c, 1, 0x7f968)?;
    c.emit(0x7f910, transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(0x7f914, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(0x7f918, mov_register(C::Al, r(2), r(9)))?;
    c.emit(0x7f91c, mov_register(C::Al, r(3), r(10)))?;
    c.emit(0x7f920, branch(C::Al, 0x7f920, 0x2ec94, true)?)?;
    c.emit(0x7f924, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(0x7f928, mov_register(C::Al, r(1), r(7)))?;
    c.emit(0x7f92c, branch(C::Al, 0x7f92c, 0x2ec94, true)?)?;
    c.emit(0x7f930, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f934, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3f4)?)?;
    c.emit(0x7f938, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7f93c, branch(C::Al, 0x7f93c, 0x17b0, true)?)?;
    c.emit(0x7f940, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7f944, data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(0x7f948, data_immediate(C::Al, Op::Add, r(6), r(6), 0x28)?)?;
    c.emit(0x7f94c, data_register(C::Al, Op::Cmp, r(0), r(8), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f950, branch(C::Cc, 0x7f950, 0x7f8d8, false)?)?;
    c.emit(0x7f954, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f958, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7f95c, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f960, branch(C::Al, 0x7f960, 0x2c34, false)?)?;
    c.pool(0x7f964, 0xf478b)?;
    c.pool(0x7f968, 0xf48cc)?;
    c.pool(0x7f96c, 0xf4905)?;
    c.pool(0x7f970, 0xe2788)?;
    Ok(())
}

fn acquire_wrapper(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f974, push_registers(C::Al, 0x4800)?)?;
    c.emit(0x7f978, mov_register(C::Al, r(11), r(13)))?;
    c.emit(0x7f97c, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x8)?)?;
    c.emit(0x7f980, data_immediate(C::Al, Op::Add, r(3), r(13), 0x4)?)?;
    c.emit(0x7f984, branch(C::Al, 0x7f984, 0x7f990, true)?)?;
    c.emit(0x7f988, mov_register(C::Al, r(13), r(11)))?;
    c.emit(0x7f98c, transfer_multiple(C::Al, true, R::SP, 0x8800, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn acquire(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7f990, push_registers(C::Al, 0x4ff0)?)?;
    c.emit(0x7f994, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x7f998, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(0x7f99c, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7f9a0, mov_register(C::Al, r(8), r(3)))?;
    c.emit(0x7f9a4, mov_register(C::Al, r(10), r(2)))?;
    c.emit(0x7f9a8, mov_register(C::Al, r(9), r(1)))?;
    c.emit(0x7f9ac, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x7f9b0, branch(C::Eq, 0x7f9b0, 0x7fa74, false)?)?;
    c.emit(0x7f9b4, transfer_word(C::Al, true, r(0), r(4), 48)?)?;
    c.emit(0x7f9b8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f9bc, branch(C::Ne, 0x7f9bc, 0x7f9e4, false)?)?;
    c.emit(0x7f9c0, transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(0x7f9c4, transfer_word(C::Al, true, r(1), r(4), 12)?)?;
    c.emit(0x7f9c8, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7f9cc, branch(C::Ls, 0x7f9cc, 0x7f9e4, false)?)?;
    c.emit(0x7f9d0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7f9d4, transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(0x7f9d8, transfer_word(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(0x7f9dc, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7f9e0, transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7f9e4, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7f9e8, branch(C::Al, 0x7f9e8, 0x2bf4, true)?)?;
    c.emit(0x7f9ec, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7f9f0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7f9f4, branch(C::Eq, 0x7f9f4, 0x7fa54, false)?)?;
    c.emit(0x7f9f8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.literal(0x7f9fc, 5, 0x7fb2c)?;
    c.emit(0x7fa00, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(0x7fa04, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x7fa08, transfer_word_register(C::Al, true, r(0), r(0), r(7), Shift::Lsl, 0)?)?;
    c.emit(0x7fa0c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4c0)?)?;
    c.emit(0x7fa10, mov_register(C::Al, r(2), r(5)))?;
    c.emit(0x7fa14, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fa18, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7fa1c, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fa20, branch(C::Al, 0x7fa20, 0x17b0, true)?)?;
    c.emit(0x7fa24, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fa28, data_register(C::Al, Op::Add, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fa2c, transfer_word(C::Al, true, r(2), r(1), 12)?)?;
    c.emit(0x7fa30, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(0x7fa34, transfer_word(C::Eq, true, r(2), r(1), 4)?)?;
    c.emit(0x7fa38, data_immediate(C::Eq, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x7fa3c, branch(C::Eq, 0x7fa3c, 0x7fab4, false)?)?;
    c.emit(0x7fa40, transfer_word(C::Al, true, r(1), r(4), 16)?)?;
    c.emit(0x7fa44, data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(0x7fa48, data_immediate(C::Al, Op::Add, r(7), r(7), 0x28)?)?;
    c.emit(0x7fa4c, data_register(C::Al, Op::Cmp, r(0), r(6), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fa50, branch(C::Cc, 0x7fa50, 0x7fa08, false)?)?;
    c.emit(0x7fa54, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7fa58, transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(0x7fa5c, transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(0x7fa60, transfer_word(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(0x7fa64, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7fa68, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7fa6c, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fa70, branch(C::Al, 0x7fa70, 0x2c34, false)?)?;
    c.literal(0x7fa74, 1, 0x7fb38)?;
    c.literal(0x7fa78, 2, 0x7fb3c)?;
    c.emit(0x7fa7c, data_immediate(C::Al, Op::Mov, r(3), r(0), 0xa9)?)?;
    c.emit(0x7fa80, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(0x7fa84, data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(0x7fa88, branch(C::Al, 0x7fa88, 0x2ec94, true)?)?;
    c.literal(0x7fa8c, 1, 0x7fb34)?;
    c.emit(0x7fa90, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(0x7fa94, branch(C::Al, 0x7fa94, 0x2ec94, true)?)?;
    c.literal(0x7fa98, 2, 0x7fb2c)?;
    c.emit(0x7fa9c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xaa)?)?;
    c.emit(0x7faa0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7faa4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(0x7faa8, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7faac, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fab0, branch(C::Al, 0x7fab0, 0x17b0, false)?)?;
    c.emit(0x7fab4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fab8, transfer_word(C::Al, false, r(0), r(1), 4)?)?;
    c.emit(0x7fabc, transfer_word(C::Al, true, r(0), r(1), 16)?)?;
    c.emit(0x7fac0, transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(0x7fac4, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fac8, data_register(C::Al, Op::Add, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7facc, transfer_word(C::Al, true, r(0), r(0), 24)?)?;
    c.emit(0x7fad0, transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(0x7fad4, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fad8, data_register(C::Al, Op::Add, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fadc, transfer_word(C::Al, false, r(0), r(9), 0)?)?;
    c.emit(0x7fae0, transfer_word(C::Al, true, r(0), r(4), 24)?)?;
    c.emit(0x7fae4, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(0x7fae8, transfer_word(C::Al, false, r(0), r(4), 24)?)?;
    c.emit(0x7faec, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7faf0, branch(C::Al, 0x7faf0, 0x2c34, true)?)?;
    c.emit(0x7faf4, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7faf8, mov_register(C::Al, r(2), r(6)))?;
    c.emit(0x7fafc, data_register(C::Al, Op::Add, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fb00, transfer_word(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(0x7fb04, transfer_word(C::Al, true, r(3), r(0), 16)?)?;
    c.emit(0x7fb08, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fb0c, transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.literal(0x7fb10, 1, 0x7fb30)?;
    c.emit(0x7fb14, branch(C::Al, 0x7fb14, 0x2ec94, true)?)?;
    c.literal(0x7fb18, 1, 0x7fb34)?;
    c.emit(0x7fb1c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fb20, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7fb24, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fb28, branch(C::Al, 0x7fb28, 0x2ec94, false)?)?;
    c.pool(0x7fb2c, 0xf478b)?;
    c.pool(0x7fb30, 0xf49d3)?;
    c.pool(0x7fb34, 0xe2788)?;
    c.pool(0x7fb38, 0xf499d)?;
    c.pool(0x7fb3c, 0xf49c8)?;
    Ok(())
}

fn acquire_sized(c: &mut TypedA32) -> Result<(), String> {
    c.emit(0x7fb40, push_registers(C::Al, 0x4ff0)?)?;
    c.emit(0x7fb44, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x7fb48, data_immediate(C::Al, Op::Sub, r(13), r(13), 0xc)?)?;
    c.emit(0x7fb4c, mov_register(C::Al, r(4), r(0)))?;
    c.emit(0x7fb50, mov_register(C::Al, r(10), r(2)))?;
    c.emit(0x7fb54, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x7fb58, branch(C::Eq, 0x7fb58, 0x7fc48, false)?)?;
    c.emit(0x7fb5c, transfer_word(C::Al, true, r(0), r(4), 48)?)?;
    c.emit(0x7fb60, transfer_word(C::Al, true, r(8), r(11), 8)?)?;
    c.emit(0x7fb64, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7fb68, branch(C::Ne, 0x7fb68, 0x7fb90, false)?)?;
    c.emit(0x7fb6c, transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.emit(0x7fb70, transfer_word(C::Al, true, r(2), r(4), 12)?)?;
    c.emit(0x7fb74, data_register(C::Al, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fb78, branch(C::Ls, 0x7fb78, 0x7fb90, false)?)?;
    c.emit(0x7fb7c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7fb80, transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(0x7fb84, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x7fb88, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7fb8c, transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fb90, transfer_word(C::Al, false, r(3), r(13), 4)?)?;
    c.emit(0x7fb94, transfer_word(C::Al, false, r(1), r(13), 8)?)?;
    c.emit(0x7fb98, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7fb9c, branch(C::Al, 0x7fb9c, 0x2bf4, true)?)?;
    c.emit(0x7fba0, transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(0x7fba4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x7fba8, branch(C::Eq, 0x7fba8, 0x7fc20, false)?)?;
    c.emit(0x7fbac, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.literal(0x7fbb0, 7, 0x7fd08)?;
    c.emit(0x7fbb4, data_immediate(C::Al, Op::Mov, r(5), r(0), 0x10c)?)?;
    c.emit(0x7fbb8, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x7fbbc, data_immediate(C::Al, Op::Mov, r(9), r(0), 0x18)?)?;
    c.emit(0x7fbc0, data_immediate(C::Al, Op::Orr, r(5), r(5), 0x400)?)?;
    c.emit(0x7fbc4, data_register(C::Al, Op::Add, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fbc8, mov_register(C::Al, r(1), r(5)))?;
    c.emit(0x7fbcc, mov_register(C::Al, r(2), r(7)))?;
    c.emit(0x7fbd0, transfer_word(C::Al, true, r(0), r(0), -24)?)?;
    c.emit(0x7fbd4, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fbd8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7fbdc, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fbe0, branch(C::Al, 0x7fbe0, 0x17b0, true)?)?;
    c.emit(0x7fbe4, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fbe8, data_register(C::Al, Op::Add, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fbec, transfer_word(C::Al, true, r(2), r(1), -12)?)?;
    c.emit(0x7fbf0, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(0x7fbf4, transfer_word(C::Eq, true, r(2), r(1), -20)?)?;
    c.emit(0x7fbf8, data_immediate(C::Eq, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x7fbfc, branch(C::Ne, 0x7fbfc, 0x7fc0c, false)?)?;
    c.emit(0x7fc00, transfer_word(C::Al, true, r(2), r(1), -4)?)?;
    c.emit(0x7fc04, data_register(C::Al, Op::Cmp, r(0), r(2), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fc08, branch(C::Cs, 0x7fc08, 0x7fc88, false)?)?;
    c.emit(0x7fc0c, transfer_word(C::Al, true, r(1), r(4), 16)?)?;
    c.emit(0x7fc10, data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(0x7fc14, data_immediate(C::Al, Op::Add, r(9), r(9), 0x28)?)?;
    c.emit(0x7fc18, data_register(C::Al, Op::Cmp, r(0), r(6), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fc1c, branch(C::Cc, 0x7fc1c, 0x7fbc4, false)?)?;
    c.emit(0x7fc20, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7fc24, transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(0x7fc28, transfer_word(C::Al, true, r(1), r(13), 4)?)?;
    c.emit(0x7fc2c, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x7fc30, transfer_word(C::Al, true, r(1), r(13), 8)?)?;
    c.emit(0x7fc34, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x7fc38, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7fc3c, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7fc40, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fc44, branch(C::Al, 0x7fc44, 0x2c34, false)?)?;
    c.literal(0x7fc48, 1, 0x7fd14)?;
    c.literal(0x7fc4c, 2, 0x7fd18)?;
    c.emit(0x7fc50, data_immediate(C::Al, Op::Mov, r(3), r(0), 0xf5)?)?;
    c.emit(0x7fc54, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(0x7fc58, data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(0x7fc5c, branch(C::Al, 0x7fc5c, 0x2ec94, true)?)?;
    c.literal(0x7fc60, 1, 0x7fd10)?;
    c.emit(0x7fc64, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(0x7fc68, branch(C::Al, 0x7fc68, 0x2ec94, true)?)?;
    c.literal(0x7fc6c, 2, 0x7fd08)?;
    c.emit(0x7fc70, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xf6)?)?;
    c.emit(0x7fc74, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x7fc78, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.emit(0x7fc7c, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7fc80, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fc84, branch(C::Al, 0x7fc84, 0x17b0, false)?)?;
    c.emit(0x7fc88, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fc8c, transfer_word(C::Al, false, r(0), r(1), -20)?)?;
    c.emit(0x7fc90, transfer_word(C::Al, true, r(0), r(1), -8)?)?;
    c.emit(0x7fc94, transfer_word(C::Al, false, r(0), r(10), 0)?)?;
    c.emit(0x7fc98, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fc9c, transfer_word(C::Al, true, r(1), r(13), 4)?)?;
    c.emit(0x7fca0, transfer_word_register(C::Al, true, r(0), r(0), r(9), Shift::Lsl, 0)?)?;
    c.emit(0x7fca4, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x7fca8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fcac, transfer_word(C::Al, true, r(1), r(13), 8)?)?;
    c.emit(0x7fcb0, data_register(C::Al, Op::Add, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fcb4, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x18)?)?;
    c.emit(0x7fcb8, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x7fcbc, transfer_word(C::Al, true, r(0), r(4), 24)?)?;
    c.emit(0x7fcc0, data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(0x7fcc4, transfer_word(C::Al, false, r(0), r(4), 24)?)?;
    c.emit(0x7fcc8, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x7fccc, branch(C::Al, 0x7fccc, 0x2c34, true)?)?;
    c.emit(0x7fcd0, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x7fcd4, mov_register(C::Al, r(2), r(6)))?;
    c.emit(0x7fcd8, data_register(C::Al, Op::Add, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x7fcdc, transfer_word(C::Al, true, r(1), r(0), -20)?)?;
    c.emit(0x7fce0, transfer_word(C::Al, true, r(3), r(0), -8)?)?;
    c.emit(0x7fce4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fce8, transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.literal(0x7fcec, 1, 0x7fd0c)?;
    c.emit(0x7fcf0, branch(C::Al, 0x7fcf0, 0x2ec94, true)?)?;
    c.literal(0x7fcf4, 1, 0x7fd10)?;
    c.emit(0x7fcf8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x7fcfc, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x7fd00, transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x7fd04, branch(C::Al, 0x7fd04, 0x2ec94, false)?)?;
    c.pool(0x7fd08, 0xf478b)?;
    c.pool(0x7fd0c, 0xf49d3)?;
    c.pool(0x7fd10, 0xe2788)?;
    c.pool(0x7fd14, 0xf499d)?;
    c.pool(0x7fd18, 0xf4a03)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32::new();
    validate_acquired(&mut c)?;
    validate_available(&mut c)?;
    acquire_wrapper(&mut c)?;
    acquire(&mut c)?;
    acquire_sized(&mut c)?;
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
        assert_eq!(END - START, 1376);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!((CODE_BYTES / 4, DATA_BYTES / 4), (326, 18));
        for (slot, value) in POOLS {
            assert_eq!(word(&bytes, slot), value);
        }
        let mut c = TypedA32::new();
        validate_acquired(&mut c).unwrap();
        assert_eq!(c.at, 0x7f898);
        validate_available(&mut c).unwrap();
        assert_eq!(c.at, 0x7f974);
        acquire_wrapper(&mut c).unwrap();
        assert_eq!(c.at, 0x7f990);
        acquire(&mut c).unwrap();
        assert_eq!(c.at, 0x7fb40);
        acquire_sized(&mut c).unwrap();
        assert_eq!((c.at, c.code_words, c.data_words), (END, 326, 18));
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
        let mut c = TypedA32::new();
        c.at = END;
        c.bytes.resize(END - START, 0);
        assert!(c.finish().is_err());
    }

    #[test]
    fn error_arms_loops_and_tail_calls() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x7f874, C::Cc, 0x7f7fc, false),
            (0x7f884, C::Al, 0x2c34, false),
            (0x7f950, C::Cc, 0x7f8d8, false),
            (0x7f960, C::Al, 0x2c34, false),
            (0x7f984, C::Al, 0x7f990, true),
            (0x7f9b0, C::Eq, 0x7fa74, false),
            (0x7fa3c, C::Eq, 0x7fab4, false),
            (0x7fa50, C::Cc, 0x7fa08, false),
            (0x7fa70, C::Al, 0x2c34, false),
            (0x7fa88, C::Al, 0x2ec94, true),
            (0x7fa94, C::Al, 0x2ec94, true),
            (0x7fab0, C::Al, 0x17b0, false),
            (0x7fb28, C::Al, 0x2ec94, false),
            (0x7fb58, C::Eq, 0x7fc48, false),
            (0x7fbfc, C::Ne, 0x7fc0c, false),
            (0x7fc08, C::Cs, 0x7fc88, false),
            (0x7fc1c, C::Cc, 0x7fbc4, false),
            (0x7fc44, C::Al, 0x2c34, false),
            (0x7fc5c, C::Al, 0x2ec94, true),
            (0x7fc68, C::Al, 0x2ec94, true),
            (0x7fc84, C::Al, 0x17b0, false),
            (0x7fd04, C::Al, 0x2ec94, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        for (at, rd, slot) in [
            (0x7fa74, 1, 0x7fb38), (0x7fa78, 2, 0x7fb3c),
            (0x7fa98, 2, 0x7fb2c), (0x7fc48, 1, 0x7fd14),
            (0x7fc4c, 2, 0x7fd18), (0x7fc6c, 2, 0x7fd08),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(rd), R::PC, slot - at as i32 - 8).unwrap());
        }
        for (at, offset) in [(0x7fa34, 4), (0x7fbf4, -20)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Eq, true, r(2), r(1), offset).unwrap());
            assert_eq!(word(&bytes, at + 4), data_immediate(C::Eq, Op::Cmp, r(0), r(2), 0).unwrap());
        }
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
