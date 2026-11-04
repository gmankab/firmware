use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags,
    data_register, load_halfword, mov_register, mov_shifted_register, multiply_accumulate,
    multiply_long, multiply_register, transfer_byte, transfer_doubleword, transfer_multiple,
    transfer_word, transfer_word_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x68d44;
pub const END: usize = 0x690a4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const DIAGNOSTIC_POINTER_ADDRESS: usize = 0x68dcc;
pub const DIAGNOSTIC_POINTER_DATA: u32 = 0x000f2d78;
pub const CODE_BYTES: usize = END - START - DATA_BYTES;
pub const DATA_BYTES: usize = 4;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("aggregate address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("aggregate output address 0x{:x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if self.at == DIAGNOSTIC_POINTER_ADDRESS {
            return Err("instruction overlaps diagnostic pointer DATA".into());
        }
        self.word(instruction)
    }

    fn data(&mut self, address: usize, value: u32) -> Result<(), String> {
        self.check(DIAGNOSTIC_POINTER_ADDRESS)?;
        self.check(address)?;
        self.word(value)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
}

fn source_overrides(c: &mut Code) -> Result<(), String> {
    let r5 = R::new(5)?;
    let r9 = R::new(9)?;
    c.check(0x68d44)?;
    c.emit(transfer_word(C::Al, true, R::R0, r9, 0x838)?)?;
    c.emit(transfer_word(C::Al, true, r5, R::SP, 0x3c)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 0xdf)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq, 0x68da4, false)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r5, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0)?)?;
    c.branch(C::Eq, 0x68d6c, false)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r5, 0x27)?)?;
    c.branch(C::Al, 0x68da4, false)?;
    c.emit(load_halfword(C::Al, R::R0, r9, 0x74)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Asr, 0x10)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 0x23)?)?;
    c.emit(transfer_byte(C::Al, false, R::R1, r5, 0x27)?)?;
    c.emit(load_halfword(C::Al, R::R0, r9, 0x76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(mov_register(C::Eq, R::R0, R::R1))?;
    c.emit(transfer_byte(C::Al, false, R::R0, r5, 0x28)?)?;
    c.emit(load_halfword(C::Al, R::R0, r9, 0x78)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, R::R0, R::R1, 2)?)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r5, 0x29)?)?;
    Ok(())
}

fn select_table(c: &mut Code) -> Result<(), String> {
    let r9 = R::new(9)?;
    c.check(0x68da4)?;
    c.emit(transfer_word(C::Al, true, R::R0, r9, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2000)?)?;
    c.branch(C::Eq, 0x68dc4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1000)?)?;
    c.branch(C::Eq, 0x68dd0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 2)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::PC, 0xdb8)?)?;
    c.branch(C::Al, 0x68dd4, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::PC, 0xda8)?)?;
    c.branch(C::Al, 0x68dd4, false)?;
    c.data(DIAGNOSTIC_POINTER_ADDRESS, DIAGNOSTIC_POINTER_DATA)?;
    Ok(())
}

fn snapshot_and_three_quarters(c: &mut Code) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r5 = R::new(5)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    c.check(0x68dd0)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::PC, 0xda0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r9, 0x518)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 3)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r5, 0x48)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r5, 0x750)?)?;
    c.emit(transfer_doubleword(C::Al, true, R::R0, r10, 0)?)?;
    c.emit(multiply_long(C::Al, false, r3, r7, R::R0, R::R2)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R1, R::R1, Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_shifted_register(C::Al, R::R2, r3, Shift::Lsr, 2)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsr, 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R2, R::R0, Shift::Lsl, 30, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::PC, 0xd40)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r5, 0x760)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r5, 0x764)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xd1)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x1c)?)?;
    c.emit(transfer_word(C::Al, true, r8, r5, 0x2a0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    c.emit(transfer_word(C::Al, true, r7, R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r5, 0x6c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    Ok(())
}

fn aggregate_products_and_quotients(c: &mut Code) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    c.check(0x68e30)?;
    c.emit(transfer_word(C::Al, true, r4, r5, 0x5c)?)?;
    c.emit(mov_register(C::Al, r6, r5))?;
    c.emit(multiply_register(C::Al, R::R0, r4, r8)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsr, 2)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::SP, 0x38)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r6, 0x2b0)?)?;
    c.emit(transfer_word(C::Al, true, r5, r6, 0x6c)?)?;
    c.emit(mov_register(C::Al, R::R1, r5))?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(multiply_long(C::Al, false, r9, r10, r4, r8)?)?;
    c.emit(mov_register(C::Al, r4, r8))?;
    c.emit(transfer_word(C::Al, false, R::R0, r6, 0x2b4)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r6, 0x64)?)?;
    c.emit(multiply_long(C::Al, false, R::R0, r3, r9, R::R2)?)?;
    c.emit(multiply_accumulate(C::Al, R::R1, r10, R::R2, r3)?)?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r6, 0x2b8)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r6, 0x68)?)?;
    c.emit(multiply_long(C::Al, false, R::R0, r3, r9, R::R2)?)?;
    c.emit(multiply_accumulate(C::Al, R::R1, r10, R::R2, r3)?)?;
    c.emit(mov_register(C::Al, R::R2, r5))?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r6, 0x2bc)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r6, 0x60)?)?;
    c.emit(transfer_word(C::Al, true, r3, R::SP, 0x38)?)?;
    c.emit(multiply_long(C::Al, false, R::R1, R::R2, R::R0, r3)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 6)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R2, Shift::Lsr, 6)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, R::R1, R::R2, Shift::Lsl, 26, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r6, 0x2c4)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r6, 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r6, 0x2cc)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r6, 0x2c8)?)?;
    Ok(())
}

fn per_entry(c: &mut Code) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    c.check(0x68ed4)?;
    c.emit(transfer_word(C::Al, true, R::R0, r6, 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.branch(C::Eq, 0x69038, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r9, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r8, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r10, R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, R::R0, 0x7d)?)?;
    c.emit(transfer_word(C::Al, false, r4, R::SP, 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r10, 0)?)?;
    c.branch(C::Eq, 0x68f68, false)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 0x3c)?)?;
    c.emit(data_register(C::Al, Op::Add, r5, r7, r9, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R2, 0xc)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r5, 0x50)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7, r10, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x20)?)?;
    c.emit(multiply_long(C::Al, false, R::R1, r3, R::R0, r4)?)?;
    c.emit(mov_register(C::Al, r4, r7))?;
    c.emit(transfer_word(C::Al, true, r7, R::R2, 0x5c)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::R2, 0x6c)?)?;
    c.emit(multiply_long(C::Al, false, R::R0, r6, R::R1, r7)?)?;
    c.emit(multiply_accumulate(C::Al, R::R1, r3, r7, r6)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0)?)?;
    c.emit(mov_register(C::Al, r7, r4))?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 0x38)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, r5, 0x80)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 1)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, R::R1, r5, 0x68)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    c.branch(C::Al, 0x68fc0, false)?;
    c.emit(transfer_word(C::Al, true, r5, R::SP, 0x3c)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r5, 0x5c)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r5, 0x6c)?)?;
    c.emit(multiply_register(C::Al, R::R0, R::R2, r4)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r7, 0x50)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r7, 0x20)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, R::R0, R::R0, 1, true)?)?;
    c.emit(multiply_long(C::Al, false, R::R1, R::R2, R::R0, r4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, R::R0, R::R0, 0)?)?;
    c.emit(multiply_accumulate(C::Al, r3, R::R0, r4, R::R2)?)?;
    c.emit(transfer_word(C::Al, true, r4, r5, 0x5c)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r5, 0x6c)?)?;
    c.emit(multiply_long(C::Al, false, R::R0, r5, R::R1, r4)?)?;
    c.emit(multiply_accumulate(C::Al, R::R1, r3, r4, r5)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r7, 0x68)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, r7, r10, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r5, R::SP, 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::R0, 0x38)?)?;
    c.emit(mov_register(C::Al, r4, r5))?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Cc, R::R1, R::R2))?;
    c.emit(multiply_long(C::Al, false, R::R2, r3, R::R1, r5)?)?;
    c.emit(mov_shifted_register(C::Al, R::R1, R::R2, Shift::Lsr, 6)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R2, R::R1, r3, Shift::Lsl, 26, false)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r7, r8, Shift::Lsl, 0, false)?)?;
    c.emit(mov_shifted_register(C::Al, r3, r3, Shift::Lsr, 6)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false, R::R2, R::R1, 0x98)?)?;
    c.emit(transfer_word(C::Al, false, r3, R::R1, 4)?)?;
    c.emit(transfer_doubleword(C::Al, false, R::R2, R::R1, 0x30)?)?;
    c.emit(transfer_word(C::Al, true, r6, R::SP, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r6, 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, r10, 0)?)?;
    c.branch(C::Eq, 0x6901c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x38)?)?;
    c.emit(data_register(C::Al, Op::Sub, R::R0, R::R2, R::R0, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r3, r3, 0)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R1, R::R0.bit() | r3.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r10, r10, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r9, r9, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r8, r8, 8)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 0x34)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r10, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x68efc, false)?;
    Ok(())
}

fn clear_and_clamp_tail(c: &mut Code) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r9 = R::new(9)?;
    c.check(0x69038)?;
    c.emit(transfer_word(C::Al, true, r4, R::SP, 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r4, 0x294)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r4, 0x298)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r4, 0xa8f)?)?;
    c.emit(transfer_byte(C::Al, true, R::R1, r4, 0x27)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x69068, false)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r4, 0xa90)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Hi, R::R1, R::R0))?;
    c.emit(mov_register(C::Al, R::R0, R::R1))?;
    c.emit(transfer_byte(C::Al, false, R::R0, r4, 0x27)?)?;
    c.emit(transfer_byte(C::Al, true, R::R2, r4, 0x26)?)?;
    c.emit(transfer_word(C::Al, true, r9, R::SP, 0x24)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R2, 2)?)?;
    c.branch(C::Ne, 0x690a8, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, R::R0, 2)?)?;
    c.emit(transfer_byte(C::Al, false, R::R1, r4, 0x28)?)?;
    c.emit(transfer_word(C::Al, true, r3, r4, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r3, 0x1000)?)?;
    c.branch(C::Ne, 0x690e0, false)?;
    c.emit(transfer_byte(C::Al, true, r3, r4, 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r3, 1)?)?;
    c.branch(C::Hi, 0x690d0, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, R::R0, 8)?)?;
    c.branch(C::Al, 0x690dc, false)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    source_overrides(&mut c)?;
    select_table(&mut c)?;
    snapshot_and_three_quarters(&mut c)?;
    aggregate_products_and_quotients(&mut c)?;
    per_entry(&mut c)?;
    clear_and_clamp_tail(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_extent_and_diagnostic_data() {
        let bytes = fragment().unwrap();
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (860, 4, 864));
        let offset = DIAGNOSTIC_POINTER_ADDRESS - START;
        assert_eq!(&bytes[offset..offset + DATA_BYTES], &DIAGNOSTIC_POINTER_DATA.to_le_bytes());
        assert_eq!(START + bytes.len(), END);
    }

    #[test]
    fn authenticated_original_byte_equality() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let original = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(original.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&original), identity.sha256);
        let bytes = fragment().unwrap();
        assert_eq!(bytes, original[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
        assert_eq!(&original[END + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA + 4], &0x51eb851fu32.to_le_bytes());
    }
}
