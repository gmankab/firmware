use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags,
    data_register, mov_register, mov_shifted_register, rotate_right_with_extend,
    signed_multiply_accumulate_bottom_halves, store_halfword, transfer_byte,
    transfer_byte_register, transfer_doubleword, transfer_multiple, transfer_word,
    transfer_word_preindexed_writeback, transfer_word_register,
    transfer_word_register_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6b38c;
pub const END: usize = 0x6b860;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const DIAGNOSTIC_POINTER_ADDRESS: usize = 0x6b85c;
pub const DIAGNOSTIC_POINTER_DATA: u32 = 0x000f2e4c;
pub const CODE_BYTES: usize = 1232;
pub const DATA_BYTES: usize = 4;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("ring ingestion address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("ring ingestion output address 0x{:x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if self.at >= DIAGNOSTIC_POINTER_ADDRESS {
            return Err("instruction overlaps diagnostic pointer DATA".into());
        }
        self.word(instruction)
    }

    fn data(&mut self, address: usize, value: u32) -> Result<(), String> {
        self.check(DIAGNOSTIC_POINTER_ADDRESS)?;
        self.check(address)?;
        if value != DIAGNOSTIC_POINTER_DATA {
            return Err("incorrect source pathname pointer DATA".into());
        }
        self.word(value)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
}

fn registers() -> [R; 12] {
    std::array::from_fn(|n| R::new(n as u8).unwrap())
}

fn ring_head_and_entry(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11] = registers();
    c.check(0x6b38c)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() | r10.bit() | r11.bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r11, R::SP, 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x24)?)?;
    c.emit(mov_register(C::Al, r4, r0))?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0xb30)?)?;
    c.emit(mov_register(C::Al, r8, r1))?;
    c.emit(transfer_word(C::Al, true, r2, R::PC, 0x4b0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(mov_register(C::Al, r10, r4))?;
    c.emit(mov_register(C::Al, r7, r4))?;
    c.emit(transfer_byte(C::Al, true, r6, r4, 0x1c)?)?;
    c.emit(transfer_byte(C::Al, true, r5, r4, 0x1d)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r9, r10, r0, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x2e0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r1, r7, r0, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0xa)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0xf1)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r1, r1, 0x900)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r0, r7, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r6, r5, r6, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0xa70)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r3, r3, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r1, r0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r0, r0, Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r1, r6, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r1, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r1, r7, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r1, r4, r0, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r2, r8, 0xc)?)?;
    c.emit(transfer_word_register(C::Al, false, r2, r1, r3, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r1, r1, 0x268)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r2, r1, 0x1800)?)?;
    Ok(())
}

fn select_class_working_record(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, _, r4, r5, r6, r7, r8, r9, r10, r11] = registers();
    c.check(0x6b41c)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x2)?)?;
    c.branch(C::Eq, 0x6b46c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x1)?)?;
    c.branch(C::Eq, 0x6b488, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x0)?)?;
    c.branch(C::Ne, 0x6b4c0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r1, r2, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r1, r4, 0xc)?)?;
    c.emit(transfer_word(C::Al, false, r6, R::SP, 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r2, R::SP, 0x18)?)?;
    c.emit(transfer_word(C::Al, false, r10, R::SP, 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x0)?)?;
    c.branch(C::Eq, 0x6b4dc, false)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r2, R::R0, 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r1, r1, r2, r9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r10, r1, 0x3440)?)?;
    c.branch(C::Al, 0x6b4e8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r10, R::SP, 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r6, R::SP, 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r2, R::SP, 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r10, r4, 0x4e0)?)?;
    c.emit(transfer_word(C::Al, false, r1, r2, 0x0)?)?;
    c.branch(C::Al, 0x6b4e8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r1, r2, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r1, r4, 0xc)?)?;
    c.emit(transfer_word(C::Al, false, r6, R::SP, 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r2, R::SP, 0x18)?)?;
    c.emit(transfer_word(C::Al, false, r10, R::SP, 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x0)?)?;
    c.branch(C::Eq, 0x6b4e4, false)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r2, R::R0, 0xe8)?)?;
    c.emit(signed_multiply_accumulate_bottom_halves(C::Al, r1, r1, r2, r9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r1, r1, 0x9b0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r10, r1, 0x3000)?)?;
    c.branch(C::Al, 0x6b4e8, false)?;
    c.emit(transfer_word(C::Al, true, r2, R::PC, 0x394)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r1, r1, 0xa00)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r11, 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() | r10.bit() | r11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x17b0, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r10, r4, 0x310)?)?;
    c.branch(C::Al, 0x6b4e8, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r10, r4, 0x3f8)?)?;
    Ok(())
}

fn ingest_metrics_and_totals(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, r3, r4, r5, _, _, r8, r9, r10, _] = registers();
    c.check(0x6b4e8)?;
    c.emit(data_immediate(C::Al, Op::Add, r1, r4, 0x2d8)?)?;
    c.emit(mov_register(C::Al, r5, r4))?;
    c.emit(data_immediate(C::Al, Op::Add, r9, r1, 0x1000)?)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r9, r0, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_byte(C::Al, false, r1, r0, 0x795)?)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x8)?)?;
    c.emit(transfer_byte(C::Al, false, r1, r0, 0x794)?)?;
    c.emit(transfer_word(C::Al, true, r1, r8, 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r1, r0, 0x79c)?)?;
    c.emit(transfer_word(C::Al, true, r2, r8, 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r2, r0, 0x7a0)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r2, r1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r2, r10, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r1, r10, 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r0, r10, 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r2, r4, 0x2ec)?)?;
    c.emit(transfer_word(C::Al, false, r1, r4, 0x2f0)?)?;
    c.emit(transfer_word(C::Al, false, r0, r4, 0x2e0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r0, r4, 0x2d4)?)?;
    c.emit(transfer_word(C::Al, true, r3, r10, 0xc8)?)?;
    c.emit(transfer_doubleword(C::Al, false, r2, r10, 0xc8)?)?;
    c.emit(transfer_word(C::Al, true, r2, R::PC, 0x310)?)?;
    c.emit(transfer_word(C::Al, true, r1, r8, 0xc)?)?;
    c.emit(transfer_word(C::Al, false, r1, r10, 0xc0)?)?;
    c.emit(transfer_byte(C::Al, false, r0, r10, 0xd0)?)?;
    c.emit(transfer_byte(C::Al, true, r0, r8, 0x40)?)?;
    c.emit(transfer_byte(C::Al, false, r0, r10, 0xb8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x2dc)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r1, r5, r0, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0xa)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0xa40)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r0, R::R0, 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, false, r5, R::SP, 0x20)?)?;
    Ok(())
}

fn copy_history_and_optional_metrics(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, _] = registers();
    c.check(0x6b580)?;
    c.emit(transfer_word(C::Al, true, r0, r5, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r0, r0, Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r6, r9, r0, Shift::Lsl, 6, false)?)?;
    c.emit(mov_register(C::Al, r9, r0))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r0, r6, 0xc4)?)?;
    c.emit(transfer_word(C::Al, false, r0, r10, 0x60)?)?;
    c.emit(transfer_byte(C::Al, true, r0, r6, 0x8)?)?;
    c.emit(transfer_byte(C::Al, false, r0, r10, 0x68)?)?;
    c.emit(transfer_word(C::Al, true, r0, r4, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r0, 0x1000)?)?;
    c.branch(C::Ne, 0x6b5c0, false)?;
    c.emit(transfer_byte(C::Al, true, r0, r8, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r5, r10, 0x60)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x82f74)?)?;
    c.emit(transfer_word(C::Al, false, r0, r6, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r0, r5, 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r0, r4, 0x8a)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r0, 0x0)?)?;
    c.branch(C::Eq, 0x6b614, false)?;
    c.emit(data_register(C::Al, Op::Add, r1, r4, r9, Shift::Lsl, 6, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r2, r8, 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r3, r10, 0x64)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r6, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r0, r1, 0x38c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r1, r1, 0x3a0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r0, r0, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r1, r1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r5, r2, r6, Shift::Lsl, 2)?)?;
    c.emit(data_register(C::Al, Op::Add, r7, r3, r6, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, false, r5, r7, -0x24)?)?;
    c.emit(transfer_word_register(C::Al, true, r5, r0, r6, Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, false, r5, r7, -0x14)?)?;
    c.emit(transfer_byte_register(C::Al, true, r7, r1, r6, Shift::Lsl, 0)?)?;
    c.emit(transfer_byte_register(C::Al, false, r7, r3, r6, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r6, r6, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r6, 0x4)?)?;
    c.branch(C::Ne, 0x6b5ec, false)?;
    Ok(())
}

fn debit_counters_and_commit(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, r3, r4, _, r6, r7, r8, _, r10, _] = registers();
    c.check(0x6b614)?;
    c.emit(transfer_word(C::Al, true, r0, R::SP, 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r7, r4, 0x2c0)?)?;
    c.emit(transfer_word(C::Al, true, r1, r10, 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r3, r4, 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r6, r4, 0x2c4)?)?;
    c.emit(transfer_word(C::Al, true, r2, r0, 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r0, r8, 0x8)?)?;
    c.emit(data_register(C::Al, Op::Sub, r7, r7, r1, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r6, r6, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r3, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r7, r4, 0x2c0)?)?;
    c.emit(transfer_word(C::Al, false, r6, r4, 0x2c4)?)?;
    c.branch(C::Eq, 0x6b65c, false)?;
    c.emit(data_register(C::Al, Op::Add, r2, r2, r0, Shift::Lsl, 3, false)?)?;
    c.emit(transfer_doubleword(C::Al, true, r6, r2, 0x98)?)?;
    c.emit(data_register(C::Al, Op::Sub, r6, r6, r1, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r7, r7, 0x0)?)?;
    c.emit(transfer_doubleword(C::Al, false, r6, r2, 0x98)?)?;
    c.emit(transfer_word(C::Al, true, r2, r4, 0x760)?)?;
    c.emit(transfer_word(C::Al, true, r3, r4, 0x764)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r7, R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r2, r2, r1, Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r3, r3, 0x0, true)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, r7, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r7, 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r2, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r3, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r2, r4, 0x760)?)?;
    c.emit(transfer_word(C::Al, false, r3, r4, 0x764)?)?;
    c.emit(transfer_word(C::Al, true, r2, R::SP, 0x18)?)?;
    c.emit(transfer_byte(C::Al, true, r3, r8, 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r2, r2, 0x0)?)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r0.bit() | r10.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(mov_register(C::Al, r0, r4))?;
    c.branch(C::Al, 0x6ac0c, true)?;
    Ok(())
}

fn advance_history_and_totals(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, r3, r4, _, r6, r7, r8, r9, r10, _] = registers();
    c.check(0x6b6a0)?;
    c.emit(transfer_word(C::Al, true, r2, R::SP, 0x20)?)?;
    c.emit(transfer_word(C::Al, true, r1, R::SP, 0x1c)?)?;
    c.emit(data_register(C::Al, Op::Add, r9, r4, r9, Shift::Lsl, 6, false)?)?;
    c.emit(transfer_word(C::Al, true, r0, r2, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r0, r0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r0, r1, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0x1300)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r0, r2, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x2fc)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r0, r9, r0, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r0, r10, 0x10)?)?;
    c.emit(transfer_word_register(C::Al, true, r1, r9, r1, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r1, r10, 0xc)?)?;
    c.emit(transfer_word(C::Al, false, r1, r10, 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r0, r4, 0x2f4)?)?;
    c.emit(transfer_word(C::Al, false, r1, r4, 0x2f8)?)?;
    c.emit(transfer_word(C::Al, false, r1, r4, 0x300)?)?;
    c.emit(transfer_byte(C::Al, true, r0, r4, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r0, 0x0)?)?;
    c.branch(C::Ne, 0x6b73c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x364)?)?;
    c.emit(transfer_byte(C::Al, true, r7, r8, 0x1)?)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r6, r8, 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r3, r9, r0, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x360)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r2, r9, r0, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x374)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r0, r9, r0, Shift::Lsl, 0)?)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r0.bit() | r7.bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(mov_register(C::Al, r0, r4))?;
    c.emit(transfer_word(C::Al, false, r6, R::SP, 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r10, R::SP, 0xc)?)?;
    c.branch(C::Al, 0x6af2c, true)?;
    Ok(())
}

fn q16_ratios(c: &mut Code) -> Result<(), String> {
    let [r0, r1, r2, r3, r4, r5, r6, r7, r8, _, r10, _] = registers();
    c.check(0x6b73c)?;
    c.emit(transfer_word(C::Al, true, r7, r10, 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r6, r10, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x10000)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r7, 0x0)?)?;
    c.branch(C::Eq, 0x6b7a8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r6, 0x0)?)?;
    c.branch(C::Eq, 0x6b7a4, false)?;
    c.emit(transfer_word(C::Al, true, r2, r10, 0x14)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r1, R::R0, r0, Shift::Lsr, 1, true)?)?;
    c.emit(rotate_right_with_extend(C::Al, r0, r6, false)?)?;
    c.emit(mov_shifted_register(C::Al, r3, r2, Shift::Lsr, 16)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r0, r2, Shift::Lsl, 16, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r2, r6, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Adc, r1, r3, r1, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0x0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_byte(C::Al, true, r1, r8, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x0)?)?;
    c.branch(C::Ne, 0x6b7a8, false)?;
    c.emit(transfer_word(C::Al, true, r1, r4, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r1, 0x2)?)?;
    c.emit(transfer_word(C::Eq, true, r1, r10, 0x20)?)?;
    c.emit(data_register(C::Eq, Op::Add, r0, r1, r0, Shift::Lsl, 0, false)?)?;
    c.emit(mov_shifted_register(C::Eq, r0, r0, Shift::Lsr, 1)?)?;
    c.branch(C::Al, 0x6b7a8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r6, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r0, r10, 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r5, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r2, r10, 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r1, r10, 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r3, R::R0, r5, Shift::Lsr, 1, true)?)?;
    c.emit(rotate_right_with_extend(C::Al, r0, r2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r2, r2, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r0, r1, Shift::Lsl, 16, true)?)?;
    c.emit(data_register(C::Al, Op::Adc, r1, r3, r1, Shift::Lsr, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0x0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, false, r0, r10, 0xdc)?)?;
    c.emit(data_register(C::Al, Op::Mov, r1, R::R0, r5, Shift::Lsr, 1, true)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r3, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r2, r10, 0xc)?)?;
    c.emit(rotate_right_with_extend(C::Al, r0, r6, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r0, r0, r2, Shift::Lsl, 16, true)?)?;
    c.emit(data_register(C::Al, Op::Adc, r1, r1, r2, Shift::Lsr, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r2, r6, 0x1)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, false, r0, r10, 0xe0)?)?;
    Ok(())
}

fn scaled_halfword_and_return(c: &mut Code) -> Result<(), String> {
    let [r0, r1, _, _, r4, r5, r6, r7, r8, r9, r10, r11] = registers();
    c.check(0x6b7fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r0, R::R0, 0x368)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r0, r0, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r0, r9, r0, Shift::Lsl, 0)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x82f74)?)?;
    c.emit(mov_register(C::Al, r5, r0))?;
    c.emit(mov_register(C::Al, r0, r7))?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x82f74)?)?;
    c.emit(mov_register(C::Al, r1, r0))?;
    c.emit(mov_register(C::Al, r0, r5))?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x837e0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r1, R::R0, 0x2c80000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r1, r1, 0x40000000)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83608)?)?;
    c.emit(mov_register(C::Al, r5, r0))?;
    c.emit(transfer_word(C::Al, true, r0, r4, 0x2e4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r0, r0, 0x1)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x82f74)?)?;
    c.emit(mov_register(C::Al, r1, r0))?;
    c.emit(mov_register(C::Al, r0, r5))?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x82988)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x82c5c)?)?;
    c.emit(store_halfword(C::Al, r0, r10, 0x2a)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r11, 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() | r9.bit() | r10.bit() | r11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    ring_head_and_entry(&mut c)?;
    select_class_working_record(&mut c)?;
    ingest_metrics_and_totals(&mut c)?;
    copy_history_and_optional_metrics(&mut c)?;
    debit_counters_and_commit(&mut c)?;
    advance_history_and_totals(&mut c)?;
    q16_ratios(&mut c)?;
    scaled_halfword_and_return(&mut c)?;
    c.data(DIAGNOSTIC_POINTER_ADDRESS, DIAGNOSTIC_POINTER_DATA)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], address: usize) -> u32 {
        u32::from_le_bytes(bytes[address - START..address - START + 4].try_into().unwrap())
    }

    #[test]
    fn exact_extent_and_source_pathname_data() {
        let bytes = fragment().unwrap();
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (1232, 4, 1236));
        assert_eq!(START + CODE_BYTES, DIAGNOSTIC_POINTER_ADDRESS);
        assert_eq!(START + bytes.len(), END);
        assert_eq!(word(&bytes, DIAGNOSTIC_POINTER_ADDRESS), DIAGNOSTIC_POINTER_DATA);
        for address in [0x6b3a4, 0x6b4c0, 0x6b544] {
            let instruction = word(&bytes, address);
            assert_eq!(address + 8 + (instruction as usize & 0xfff), DIAGNOSTIC_POINTER_ADDRESS);
        }
    }

    #[test]
    fn checked_data_rejects_instructions_and_wrong_pointer() {
        let mut c = Code { at: DIAGNOSTIC_POINTER_ADDRESS, bytes: Vec::new() };
        assert!(c.emit(mov_register(C::Al, R::R0, R::R0)).is_err());
        assert!(c.data(DIAGNOSTIC_POINTER_ADDRESS, 0).is_err());
        assert!(c.data(START, DIAGNOSTIC_POINTER_DATA).is_err());
        assert!(c.bytes.is_empty());
        c.data(DIAGNOSTIC_POINTER_ADDRESS, DIAGNOSTIC_POINTER_DATA).unwrap();
        assert!(c.word(0).is_err());
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.data(DIAGNOSTIC_POINTER_ADDRESS, DIAGNOSTIC_POINTER_DATA).is_err());
    }

    #[test]
    fn calls_and_interworking_targets() {
        let bytes = fragment().unwrap();
        for (address, target) in [(0x6b69c, 0x6ac0c), (0x6b738, 0x6af2c),
            (0x6b77c, 0x8187c), (0x6b7d0, 0x8187c), (0x6b7f4, 0x8187c)] {
            assert_eq!(word(&bytes, address), branch(C::Al, address as u32, target, true).unwrap());
        }
        for (address, target) in [(0x6b5b4, 0x82f74), (0x6b808, 0x82f74),
            (0x6b814, 0x82f74), (0x6b820, 0x837e0), (0x6b82c, 0x83608),
            (0x6b83c, 0x82f74), (0x6b848, 0x82988), (0x6b84c, 0x82c5c)] {
            assert_eq!(word(&bytes, address), branch_exchange_immediate(address as u32, target).unwrap());
        }
    }

    #[test]
    fn carry_borrow_rrx_and_writeback_are_preserved() {
        let bytes = fragment().unwrap();
        for address in [0x6b764, 0x6b7bc, 0x6b7e4] {
            assert_eq!(word(&bytes, address), rotate_right_with_extend(C::Al, R::R0,
                R::new(if address == 0x6b7bc { 2 } else { 6 }).unwrap(), false).unwrap());
        }
        for address in [0x6b760, 0x6b7b8, 0x6b7d8, 0x6b630, 0x6b650,
            0x6b668, 0x6b66c, 0x6b76c, 0x6b7c4, 0x6b7e8] {
            assert_ne!(word(&bytes, address) & (1 << 20), 0);
        }
        for address in [0x6b634, 0x6b654, 0x6b774, 0x6b7c8, 0x6b7ec] {
            assert_eq!(word(&bytes, address) & (1 << 20), 0);
        }
        for address in [0x6b3bc, 0x6b3c8, 0x6b564, 0x6b590] {
            assert_ne!(word(&bytes, address) & (1 << 21), 0);
        }
        assert_eq!(word(&bytes, 0x6b850), store_halfword(C::Al, R::R0, R::new(10).unwrap(), 0x2a).unwrap());
    }

    #[test]
    fn authenticated_original_byte_equality() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let original = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(original.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&original), identity.sha256);
        assert_eq!(identity.sha256, "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        for (index, (actual, expected)) in bytes.iter().zip(
            &original[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]).enumerate() {
            assert_eq!(actual, expected, "runtime byte 0x{:x}", START + index);
        }
        assert_eq!(bytes.len(), END - START);
    }
}
