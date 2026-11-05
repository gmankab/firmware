use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
    branch, data_immediate, data_register, mov_register, multiply_register,
    push_registers, transfer_multiple, transfer_word, transfer_word_register_preindexed_writeback,
};

pub const START: usize = 0x7e544;
pub const END: usize = 0x7eb40;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1524;
pub const DATA_BYTES: usize = 8;
const POOLS: [(usize, u32); 2] = [(0x7e780, 0xf478b), (0x7eb08, 0xf478b)];

fn r(number: u8) -> R {
    R::new(number).expect("valid ARM register")
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn new() -> Self {
        Self { at: START, bytes: Vec::with_capacity(END - START) }
    }

    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("vbuffer address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        let pool = POOLS.iter().find(|(at, _)| *at == self.at).map(|(_, value)| *value);
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || if data { pool != Some(value) } else { pool.is_some() }
        {
            return Err(format!("invalid vbuffer {} at {:#x}", if data { "data" } else { "code" }, self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn pool(&mut self, slot: usize, value: u32) -> Result<(), String> {
        self.check(slot)?;
        self.word(value, true)
    }

    fn immediate(&mut self, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn register(&mut self, condition: C, op: Op, rd: u8, rn: u8, rm: u8, amount: u8) -> Result<(), String> {
        self.emit(data_register(condition, op, r(rd), r(rn), r(rm), Shift::Lsl, amount, false)?)
    }

    fn mov(&mut self, condition: C, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(condition, r(rd), r(rm)))
    }

    fn transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn literal(&mut self, rd: u8, slot: usize) -> Result<(), String> {
        if !POOLS.iter().any(|(at, _)| *at == slot) {
            return Err("invalid vbuffer literal target".into());
        }
        self.transfer(true, rd, 15, slot as i32 - self.at as i32 - 8)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn pop(&mut self, condition: C, registers: u16) -> Result<(), String> {
        self.emit(transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn finish(self) -> Result<Vec<u8>, String> {
        self.check(END)?;
        if self.bytes.len() != END - START {
            return Err("invalid vbuffer byte count".into());
        }
        Ok(self.bytes)
    }
}

fn create_or_reinitialize(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7e544)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x1c)?;
    c.immediate(C::Al, Op::Sub, 13, 13, 4)?;
    c.mov(C::Al, 7, 2)?;
    c.mov(C::Al, 6, 3)?;
    c.mov(C::Al, 5, 1)?;
    c.mov(C::Al, 10, 0)?;
    c.transfer(true, 4, 7, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0)?;
    c.branch(C::Eq, 0x7e688, false)?;
    c.transfer(true, 0, 4, 0xc)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Ne, 0x7e770, false)?;
    c.immediate(C::Al, Op::Add, 0, 5, 8)?;
    c.immediate(C::Al, Op::Tst, 0, 5, 7)?;
    c.immediate(C::Ne, Op::Bic, 5, 0, 7)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x7e730, false)?;
    c.transfer(true, 2, 4, 0x28)?;
    c.transfer(true, 0, 4, 0x20)?;
    c.emit(multiply_register(C::Al, r(1), r(5), r(10))?)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, 0)?;
    c.mov(C::Ne, 2, 10)?;
    c.register(C::Al, Op::Add, 2, 2, 2, 1)?;
    c.register(C::Al, Op::Add, 0, 0, 2, 3)?;
    c.transfer(false, 2, 13, 0)?;
    c.branch(C::Al, 0x7e784, true)?;
    c.transfer(true, 1, 4, 0x24)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0x24)?;
    c.branch(C::Eq, 0x7e71c, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 10, 0)?;
    c.branch(C::Eq, 0x7e768, false)?;
    c.immediate(C::Al, Op::Mov, 9, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 8, 0, 0)?;
    c.transfer(true, 0, 4, 0x20)?;
    c.transfer(true, 1, 13, 0)?;
    c.register(C::Al, Op::Add, 0, 0, 1, 3)?;
    c.transfer(true, 1, 0, 0x10)?;
    c.transfer(true, 6, 0, 0xc)?;
    c.register(C::Al, Op::Add, 2, 5, 6, 0)?;
    c.register(C::Al, Op::Add, 1, 1, 5, 0)?;
    c.transfer(false, 2, 0, 0xc)?;
    c.transfer(false, 1, 0, 0x10)?;
    c.literal(2, 0x7e780)?;
    c.transfer(true, 0, 0, 0x14)?;
    c.register(C::Al, Op::Cmp, 0, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x2f8)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(C::Ls, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x400)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.transfer(true, 0, 4, 4)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0)?;
    c.register(C::Al, Op::Add, 1, 0, 9, 0)?;
    c.transfer(false, 6, 1, 0x10)?;
    c.branch(C::Eq, 0x7e71c, false)?;
    c.transfer(true, 1, 4, 0xc)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0xc)?;
    c.transfer(true, 1, 4, 0x14)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0x14)?;
    c.transfer(true, 1, 4, 0x18)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0x18)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, false, r(8), r(0), r(9), Shift::Lsl, 0)?)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Add, 8, 8, 1)?;
    c.immediate(C::Al, Op::Add, 9, 9, 0x28)?;
    c.transfer(false, 1, 0, 4)?;
    c.transfer(false, 1, 0, 8)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 1)?;
    c.register(C::Al, Op::Cmp, 0, 8, 10, 0)?;
    c.transfer(false, 1, 0, 0xc)?;
    c.transfer(false, 5, 0, 0x14)?;
    c.transfer(false, 4, 0, 0x24)?;
    c.branch(C::Cc, 0x7e5d8, false)?;
    c.branch(C::Al, 0x7e768, false)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x34)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.mov(C::Al, 4, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0)?;
    c.branch(C::Eq, 0x7e71c, false)?;
    c.mov(C::Al, 0, 4)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x34)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.register(C::Al, Op::Add, 0, 10, 10, 2)?;
    c.transfer(false, 6, 4, 0x2c)?;
    c.transfer(false, 10, 4, 0x10)?;
    c.register(C::Al, Op::Mov, 0, 0, 0, 3)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.transfer(false, 0, 4, 4)?;
    c.branch(C::Eq, 0x7e71c, false)?;
    c.transfer(true, 1, 4, 0x10)?;
    c.register(C::Al, Op::Add, 1, 1, 1, 2)?;
    c.register(C::Al, Op::Mov, 2, 0, 1, 3)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 1)?;
    c.branch(C::Ne, 0x7e70c, false)?;
    c.transfer(true, 1, 4, 0x28)?;
    c.register(C::Al, Op::Add, 2, 10, 10, 1)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x18)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0)?;
    c.register(C::Ne, Op::Mov, 0, 0, 2, 3)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.transfer(false, 0, 4, 0x20)?;
    c.branch(C::Eq, 0x7e71c, false)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x2b8c, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7e778, false)?;
    c.transfer(false, 4, 7, 0)?;
    c.mov(C::Al, 0, 7)?;
    c.immediate(C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.pop(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x7e7d4, false)?;
    c.transfer(true, 0, 4, 0x10)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7e768, false)?;
    c.transfer(true, 1, 4, 4)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 3, 0, 0)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0xc)?;
    c.transfer(false, 3, 1, -12)?;
    c.immediate(C::Al, Op::Add, 3, 3, 1)?;
    c.transfer(false, 2, 1, 0)?;
    c.transfer(false, 2, 1, -8)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0x28)?;
    c.register(C::Al, Op::Cmp, 0, 3, 0, 0)?;
    c.branch(C::Cc, 0x7e74c, false)?;
    c.transfer(false, 5, 4, 0x1c)?;
    c.transfer(false, 10, 4, 8)?;
    c.immediate(C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.pop(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit())?;
    c.transfer(false, 4, 7, 0)?;
    c.branch(C::Al, 0x7e56c, false)?;
    c.pool(0x7e780, 0xf478b)?;
    c.check(0x7e784)
}

fn initialize_backing(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7e784)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(C::Al, 4, 1)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x18)?;
    c.mov(C::Al, 5, 0)?;
    c.immediate(C::Al, Op::Mov, 6, 0, 0)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.transfer(false, 0, 5, 4)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x2f28, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.transfer(false, 0, 5, 8)?;
    c.immediate(C::Ne, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Ne, Op::Add, 2, 5, 0xc)?;
    c.immediate(C::Ne, Op::Mov, 6, 0, 1)?;
    c.emit(transfer_multiple(C::Ne, false, r(2), r(0).bit() | r(1).bit() | r(4).bit(), BlockMode::IncrementAfter, false)?)?;
    c.mov(C::Al, 0, 6)?;
    c.pop(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x7e7d4)
}

fn teardown(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7e7d4)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x18)?;
    c.mov(C::Al, 8, 0)?;
    c.transfer(true, 9, 8, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 9, 0)?;
    c.pop(C::Eq, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit())?;
    c.transfer(true, 0, 9, 4)?;
    c.transfer(true, 1, 9, 0x20)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.immediate(C::Eq, Op::Mov, 4, 0, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0)?;
    c.branch(C::Ne, 0x7e820, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7e814, false)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 1)?;
    c.transfer(false, 0, 9, 4)?;
    c.transfer(true, 0, 9, 0)?;
    c.branch(C::Al, 0x2bcc, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 1)?;
    c.branch(C::Eq, 0x7e8a8, false)?;
    c.transfer(true, 0, 9, 0x28)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7e850, false)?;
    c.transfer(true, 7, 9, 0x10)?;
    c.transfer(true, 0, 9, 0x20)?;
    c.immediate(C::Al, Op::Cmp, 0, 7, 0)?;
    c.branch(C::Ne, 0x7e858, false)?;
    c.branch(C::Al, 0x7e89c, false)?;
    c.transfer(true, 0, 9, 0x20)?;
    c.immediate(C::Al, Op::Mov, 7, 0, 1)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 5, 0, 0)?;
    c.register(C::Al, Op::Add, 6, 0, 4, 0)?;
    c.transfer(true, 0, 6, 4)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7e878, false)?;
    c.immediate(C::Al, Op::Add, 0, 6, 8)?;
    c.branch(C::Al, 0x3040, true)?;
    c.mov(C::Al, 0, 6)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x18)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.transfer(true, 0, 9, 0x20)?;
    c.immediate(C::Al, Op::Add, 5, 5, 1)?;
    c.immediate(C::Al, Op::Add, 4, 4, 0x18)?;
    c.register(C::Al, Op::Cmp, 0, 5, 7, 0)?;
    c.branch(C::Ne, 0x7e860, false)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.transfer(true, 0, 9, 4)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.mov(C::Al, 0, 9)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.transfer(false, 0, 8, 0)?;
    c.pop(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x7e8bc)
}

fn grow_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7e8bc)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x1c)?;
    c.immediate(C::Al, Op::Sub, 13, 13, 4)?;
    c.mov(C::Al, 4, 2)?;
    c.mov(C::Al, 6, 0)?;
    c.mov(C::Al, 9, 1)?;
    c.transfer(true, 0, 4, 0)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.transfer(true, 5, 4, 0xc)?;
    c.register(C::Al, Op::Cmp, 0, 5, 6, 0)?;
    c.branch(C::Cs, 0x7eaa8, false)?;
    c.transfer(true, 0, 4, 0x10)?;
    c.immediate(C::Al, Op::Mov, 7, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 8, 0, 0)?;
    c.register(C::Al, Op::Sub, 0, 0, 5, 0)?;
    c.register(C::Al, Op::Cmp, 0, 0, 6, 0)?;
    c.branch(C::Cs, 0x7e948, false)?;
    c.register(C::Al, Op::Add, 0, 6, 6, 2)?;
    c.register(C::Al, Op::Mov, 0, 0, 0, 3)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.mov(C::Al, 7, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 7, 0)?;
    c.branch(C::Eq, 0x7eaa8, false)?;
    c.transfer(true, 0, 4, 0xc)?;
    c.transfer(true, 1, 4, 4)?;
    c.register(C::Al, Op::Add, 0, 0, 0, 2)?;
    c.register(C::Al, Op::Mov, 2, 0, 0, 3)?;
    c.mov(C::Al, 0, 7)?;
    c.branch(C::Al, 0x471c, true)?;
    c.transfer(true, 0, 4, 4)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.transfer(false, 7, 4, 4)?;
    c.register(C::Al, Op::Sub, 8, 6, 5, 0)?;
    c.mov(C::Al, 7, 6)?;
    c.transfer(false, 6, 4, 0x10)?;
    c.immediate(C::Al, Op::Add, 0, 9, 8)?;
    c.immediate(C::Al, Op::Tst, 0, 9, 7)?;
    c.immediate(C::Ne, Op::Bic, 9, 0, 7)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x7eab8, false)?;
    c.transfer(true, 0, 4, 0x24)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x18)?;
    c.register(C::Al, Op::Add, 0, 0, 0, 1)?;
    c.register(C::Al, Op::Add, 0, 1, 0, 3)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.mov(C::Al, 6, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0)?;
    c.branch(C::Eq, 0x7eaa8, false)?;
    c.transfer(true, 0, 4, 0x24)?;
    c.transfer(true, 1, 4, 0x20)?;
    c.register(C::Al, Op::Add, 0, 0, 0, 1)?;
    c.register(C::Al, Op::Mov, 2, 0, 0, 3)?;
    c.mov(C::Al, 0, 6)?;
    c.branch(C::Al, 0x471c, true)?;
    c.transfer(true, 0, 4, 0x20)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.transfer(false, 6, 4, 0x20)?;
    c.transfer(true, 0, 4, 0x24)?;
    c.immediate(C::Al, Op::Add, 1, 0, 1)?;
    c.register(C::Al, Op::Add, 5, 0, 0, 1)?;
    c.transfer(false, 1, 4, 0x24)?;
    c.emit(multiply_register(C::Al, r(1), r(8), r(9))?)?;
    c.register(C::Al, Op::Add, 0, 6, 5, 3)?;
    c.branch(C::Al, 0x7e784, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7eaa8, false)?;
    c.transfer(true, 0, 4, 0xc)?;
    c.transfer(true, 1, 4, 0x14)?;
    c.literal(2, 0x7eb08)?;
    c.register(C::Al, Op::Cmp, 0, 0, 1, 0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x3a)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(C::Eq, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x100)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.transfer(true, 10, 4, 0xc)?;
    c.register(C::Al, Op::Cmp, 0, 10, 7, 0)?;
    c.branch(C::Cs, 0x7eaf4, false)?;
    c.register(C::Al, Op::Add, 0, 10, 10, 2)?;
    c.register(C::Al, Op::Mov, 8, 0, 0, 3)?;
    c.transfer(true, 0, 4, 0x20)?;
    c.register(C::Al, Op::Add, 0, 0, 5, 3)?;
    c.transfer(true, 1, 0, 0x10)?;
    c.transfer(true, 6, 0, 0xc)?;
    c.register(C::Al, Op::Add, 2, 9, 6, 0)?;
    c.register(C::Al, Op::Add, 1, 1, 9, 0)?;
    c.transfer(false, 2, 0, 0xc)?;
    c.transfer(false, 1, 0, 0x10)?;
    c.literal(2, 0x7eb08)?;
    c.transfer(true, 0, 0, 0x14)?;
    c.register(C::Al, Op::Cmp, 0, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x2f8)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(C::Ls, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x400)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.transfer(true, 0, 4, 4)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0)?;
    c.register(C::Al, Op::Add, 1, 0, 8, 0)?;
    c.transfer(false, 6, 1, 0x10)?;
    c.branch(C::Eq, 0x7eaa8, false)?;
    c.transfer(true, 1, 4, 0xc)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0xc)?;
    c.transfer(true, 1, 4, 0x14)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0x14)?;
    c.transfer(true, 1, 4, 0x18)?;
    c.immediate(C::Al, Op::Add, 1, 1, 1)?;
    c.transfer(false, 1, 4, 0x18)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, false, r(10), r(0), r(8), Shift::Lsl, 0)?)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 1)?;
    c.immediate(C::Al, Op::Add, 10, 10, 1)?;
    c.immediate(C::Al, Op::Add, 8, 8, 0x28)?;
    c.transfer(false, 1, 0, 0xc)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.register(C::Al, Op::Cmp, 0, 10, 7, 0)?;
    c.transfer(false, 1, 0, 4)?;
    c.transfer(false, 9, 0, 0x14)?;
    c.transfer(false, 4, 0, 0x24)?;
    c.branch(C::Cc, 0x7ea00, false)?;
    c.branch(C::Al, 0x7eaf4, false)?;
    c.transfer(true, 0, 4, 0)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.branch(C::Al, 0x7eb00, false)?;
    c.transfer(true, 0, 4, 0xc)?;
    c.register(C::Al, Op::Cmp, 0, 0, 7, 0)?;
    c.branch(C::Cs, 0x7eaf4, false)?;
    c.transfer(true, 2, 4, 4)?;
    c.register(C::Al, Op::Add, 1, 0, 0, 2)?;
    c.register(C::Al, Op::Add, 1, 2, 1, 3)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0xc)?;
    c.transfer(false, 0, 1, -12)?;
    c.immediate(C::Al, Op::Add, 0, 0, 1)?;
    c.transfer(false, 2, 1, 0)?;
    c.transfer(false, 2, 1, -8)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0x28)?;
    c.register(C::Al, Op::Cmp, 0, 0, 7, 0)?;
    c.branch(C::Ne, 0x7ead8, false)?;
    c.transfer(true, 0, 4, 0)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.pop(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit())?;
    c.pool(0x7eb08, 0xf478b)?;
    c.check(0x7eb0c)
}

fn destroy_backing(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7eb0c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 4)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x7eb2c, false)?;
    c.immediate(C::Al, Op::Add, 0, 4, 8)?;
    c.branch(C::Al, 0x3040, true)?;
    c.mov(C::Al, 0, 4)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x18)?;
    c.pop(C::Al, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x31ec, false)?;
    c.check(0x7eb40)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32::new();
    create_or_reinitialize(&mut c)?;
    initialize_backing(&mut c)?;
    teardown(&mut c)?;
    grow_locked(&mut c)?;
    destroy_backing(&mut c)?;
    c.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn extents_pools_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1532);
        assert_eq!(END - START, CODE_BYTES + DATA_BYTES);
        assert_eq!(CODE_BYTES, 381 * 4);
        assert_eq!(DATA_BYTES, POOLS.len() * 4);
        for (slot, value) in POOLS {
            assert_eq!(word(&bytes, slot), value);
        }
        for (at, condition, target, link) in [
            (0x7e5b0, C::Al, 0x7e784, true),
            (0x7e5c4, C::Eq, 0x7e71c, false),
            (0x7e698, C::Eq, 0x7e71c, false),
            (0x7e6c8, C::Eq, 0x7e71c, false),
            (0x7e708, C::Eq, 0x7e71c, false),
            (0x7e72c, C::Al, 0x7e7d4, false),
            (0x7e8d8, C::Al, 0x2bf4, true),
            (0x7e914, C::Eq, 0x7eaa8, false),
            (0x7e97c, C::Eq, 0x7eaa8, false),
            (0x7e9bc, C::Al, 0x7e784, true),
            (0x7e9c4, C::Eq, 0x7eaa8, false),
            (0x7eaac, C::Al, 0x2c34, true),
            (0x7eaf8, C::Al, 0x2c34, true),
            (0x7eb3c, C::Al, 0x31ec, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
    }

    #[test]
    fn guards_reject_without_mutation() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let valid = (START..END).contains(&at) && at % 4 == 0;
            let pool = POOLS.iter().any(|(slot, _)| *slot == at);
            let mut c = TypedA32::new();
            c.at = at;
            let ok = valid && !pool;
            assert_eq!(c.emit(mov_register(C::Al, R::R0, R::R0)).is_ok(), ok);
            assert_eq!(c.at, at + if ok { 4 } else { 0 });
            assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
            for (slot, value) in POOLS {
                for candidate in [value, value ^ 1] {
                    let mut c = TypedA32::new();
                    c.at = at;
                    let ok = valid && at == slot && candidate == value;
                    assert_eq!(c.pool(slot, candidate).is_ok(), ok);
                    assert_eq!(c.at, at + if ok { 4 } else { 0 });
                    assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
                }
            }
        }
        let mut c = TypedA32::new();
        assert!(c.literal(0, START).is_err());
        assert!(c.pool(START, 0xf478b).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
        assert!(c.finish().is_err());
        let mut c = TypedA32::new();
        c.at = END;
        assert!(c.finish().is_err());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let path = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE"));
        let Some(path) = path else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
