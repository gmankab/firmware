use arm32::{branch, data_immediate, data_register, store_halfword, transfer_byte,
    transfer_byte_postindexed, transfer_byte_register, transfer_halfword_register,
    transfer_multiple, transfer_word, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x79214;
pub const END: usize = 0x798c8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1672;
pub const DATA_BYTES: usize = 44;

fn r(number: u8) -> R { R::new(number).unwrap() }

fn data_value(at: usize) -> Option<u32> {
    match at {
        0x79304 => Some(0xf3d8b),
        0x79308 => Some(0xf3dcc),
        0x7930c => Some(0xe2788),
        0x79310 => Some(0xf3de3),
        0x79670 => Some(0x105a64),
        0x79674 => Some(0x105ca0),
        0x79678 => Some(0x105b68),
        0x7967c => Some(0x105de0),
        0x79680 => Some(0x1059b0),
        0x797e8 => Some(0x221007),
        0x798c4 => Some(0xf3ce9),
        _ => None,
    }
}

fn code_address(at: usize) -> bool {
    at % 4 == 0 && [(START, 0x79304), (0x79314, 0x79670),
        (0x79684, 0x797e8), (0x797ec, 0x798c4)]
        .iter().any(|&(start, end)| (start..end).contains(&at))
}

struct TypedA32 { at: usize, bytes: Vec<u8> }

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("worker continuation address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !code_address(self.at) {
            return Err("worker continuation instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if data_value(at) != Some(value) {
            return Err("worker continuation protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let expected = match self.at {
            0x7923c => (C::Eq, 0x79264, false),
            0x79254 => (C::Al, 0x2ec94, true),
            0x79260 => (C::Al, 0x2ec94, true),
            0x7926c => (C::Ne, 0x79278, false),
            0x79274 => (C::Al, 0x2f1c, true),
            0x7927c => (C::Al, 0x35bec, true),
            0x79288 => (C::Eq, 0x79290, false),
            0x7928c => (C::Al, 0x524d8, true),
            0x79298 => (C::Eq, 0x792a0, false),
            0x7929c => (C::Al, 0x70f9c, true),
            0x792a8 => (C::Eq, 0x792b0, false),
            0x792ac => (C::Al, 0x62234, true),
            0x792b8 => (C::Eq, 0x792c0, false),
            0x792bc => (C::Al, 0x71250, true),
            0x792c8 => (C::Eq, 0x792d8, false),
            0x792cc => (C::Al, 0x2f1c, true),
            0x792dc => (C::Al, 0x2f1c, true),
            0x792f0 => (C::Al, 0x2ec94, true),
            0x79300 => (C::Al, 0x2ec94, false),
            0x79350 => (C::Al, 0x58f60, true),
            0x79360 => (C::Al, 0x471c, true),
            0x7936c => (C::Eq, 0x79390, false),
            0x793f8 => (C::Ne, 0x7940c, false),
            0x79408 => (C::Al, 0x31ec, true),
            0x7941c => (C::Al, 0x61d7c, true),
            0x79428 => (C::Eq, 0x79448, false),
            0x79434 => (C::Ne, 0x79448, false),
            0x79444 => (C::Al, 0x679e8, true),
            0x79450 => (C::Al, 0x7125c, true),
            0x7945c => (C::Ne, 0x79478, false),
            0x7946c => (C::Al, 0x52304, true),
            0x79498 => (C::Al, 0x2bf4, true),
            0x794a8 => (C::Eq, 0x794f4, false),
            0x794b4 => (C::Lt, 0x794f4, false),
            0x794cc => (C::Ne, 0x794e8, false),
            0x794e0 => (C::Lt, 0x794c0, false),
            0x794e4 => (C::Al, 0x794f4, false),
            0x794f8 => (C::Al, 0x2c34, true),
            0x79518 => (C::Al, 0x31ec, true),
            0x79524 => (C::Eq, 0x795fc, false),
            0x7952c => (C::Ne, 0x7965c, false),
            0x79548 => (C::Eq, 0x7956c, false),
            0x79558 => (C::Ne, 0x7957c, false),
            0x79568 => (C::Al, 0x79578, false),
            0x79578 => (C::Al, 0x471c, true),
            0x79590 => (C::Ne, 0x79540, false),
            0x795ac => (C::Eq, 0x795d0, false),
            0x795bc => (C::Ne, 0x795e0, false),
            0x795cc => (C::Al, 0x795dc, false),
            0x795dc => (C::Al, 0x471c, true),
            0x795f4 => (C::Ne, 0x795a4, false),
            0x795f8 => (C::Al, 0x7965c, false),
            0x79644 => (C::Ne, 0x79610, false),
            0x79658 => (C::Ne, 0x79608, false),
            0x79664 => (C::Al, 0x2014, true),
            0x796a8 => (C::Eq, 0x796f8, false),
            0x796f4 => (C::Al, 0x2a878, true),
            0x79714 => (C::Al, 0x356e8, true),
            0x7971c => (C::Eq, 0x79708, false),
            0x79728 => (C::Al, 0x35d68, true),
            0x79734 => (C::Eq, 0x797e0, false),
            0x7974c => (C::Ne, 0x797bc, false),
            0x79790 => (C::Al, 0x2a878, true),
            0x7979c => (C::Eq, 0x797a8, false),
            0x797a4 => (C::Al, 0x7fd1c, true),
            0x797b0 => (C::Eq, 0x797bc, false),
            0x797b8 => (C::Al, 0x7fd1c, true),
            0x797c8 => (C::Al, 0x31ec, true),
            0x797d0 => (C::Al, 0x35d68, true),
            0x797dc => (C::Ne, 0x79744, false),
            0x79818 => (C::Al, 0x17b0, true),
            0x79838 => (C::Al, 0x17b0, true),
            0x7984c => (C::Al, 0x622f8, true),
            0x79858 => (C::Eq, 0x798bc, false),
            0x79880 => (C::Al, 0x355dc, true),
            0x798a4 => (C::Al, 0x17b0, true),
            0x798b8 => (C::Ne, 0x79874, false),
            _ => return Err("worker continuation invalid branch site".into()),
        };
        if (condition, target, link) != expected {
            return Err("worker continuation invalid branch".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn literal(&mut self, condition: C, destination: R, target: usize) -> Result<(), String> {
        let expected = match self.at {
            0x79240 => (C::Al, r(1), 0x79304),
            0x79244 => (C::Al, r(2), 0x79308),
            0x79258 => (C::Al, r(1), 0x7930c),
            0x792e0 => (C::Al, r(1), 0x79310),
            0x792f4 => (C::Al, r(1), 0x7930c),
            0x79530 => (C::Al, r(5), 0x79670),
            0x79534 => (C::Al, r(10), 0x79674),
            0x79594 => (C::Al, r(5), 0x79678),
            0x79598 => (C::Al, r(10), 0x7967c),
            0x795fc => (C::Al, r(12), 0x79680),
            0x796a0 => (C::Al, r(8), 0x797e8),
            0x797f8 => (C::Al, r(7), 0x798c4),
            0x79888 => (C::Al, r(2), 0x798c4),
            _ => return Err("worker continuation invalid literal site".into()),
        };
        if (condition, destination, target) != expected || data_value(target).is_none() {
            return Err("worker continuation invalid literal".into());
        }
        self.emit(transfer_word(condition, true, destination, R::PC,
            target as i32 - self.at as i32 - 8)?)
    }
}

fn release_worker_resources(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79214)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(0), 0x34)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x79264, false)?;
    c.literal(C::Al, r(1), 0x79304)?;
    c.literal(C::Al, r(2), 0x79308)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x77)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0xa00)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x7930c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x79278, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x14)?)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x640)?)?;
    c.branch(C::Al, 0x35bec, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x79290, false)?;
    c.branch(C::Al, 0x524d8, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x792a0, false)?;
    c.branch(C::Al, 0x70f9c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x24)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x792b0, false)?;
    c.branch(C::Al, 0x62234, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x704)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x792c0, false)?;
    c.branch(C::Al, 0x71250, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x792d8, false)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 0x1a4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2f1c, true)?;
    c.literal(C::Al, r(1), 0x79310)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x7930c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x4830, BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x2ec94, false)?;
    c.check(0x79304)
}

fn reset_worker_and_tables(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79314)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0xa4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(5), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(5), 0x5c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(9), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x264)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(0), 0x400)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x124)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x58f60, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa0)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x82c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x79390, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x834)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x64)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0x60)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x28)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 0x668)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x60)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x688)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(9), 0x4)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(9), 0x6)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(9), 0x19c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x1a0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x1a4)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 0x34)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(9), 0x171)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(9), 0x190)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(9), 0x172)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(9), 0x175)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(9), 0x170)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(9), 0x176)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(9), 0x1dc)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x38)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(9), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x6a)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(9), 0x17a)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(9), 0x194)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(9), 0x198)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(9), 0x17b)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x200)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(7), r(9), r(0))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x7940c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x2b4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6c)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(store_halfword(C::Al, r(7), r(9), 0xf8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x24)?)?;
    c.branch(C::Al, 0x61d7c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x68)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x79448, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x79448, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x679e8, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x704)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x7125c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x2f4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x79478, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x90)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x52304, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x2f4)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x664)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(0), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x664)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(9), 0x6e0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(9), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(4), r(9), 0x7)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x68)?)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.emit(transfer_word(C::Al, false, r(4), r(5), 0x74)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x794f4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 0x6c)?)?;
    c.branch(C::Lt, 0x794f4, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(2), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(3), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(3), 0x1)?)?;
    c.branch(C::Ne, 0x794e8, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Lt, 0x794c0, false)?;
    c.branch(C::Al, 0x794f4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 0x74)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 0x6c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x68)?)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 0x18)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0xa84)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x795fc, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.branch(C::Ne, 0x7965c, false)?;
    c.literal(C::Al, r(5), 0x79670)?;
    c.literal(C::Al, r(10), 0x79674)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(9), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0x15c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7956c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x838)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x7957c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x40)?)?;
    c.branch(C::Al, 0x79578, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x34)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(10), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x5)?)?;
    c.branch(C::Ne, 0x79540, false)?;
    c.literal(C::Al, r(5), 0x79678)?;
    c.literal(C::Al, r(10), 0x7967c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(9), 0x60)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0x15c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x795d0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x838)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0xe0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x795e0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x40)?)?;
    c.branch(C::Al, 0x795dc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x34)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(10), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x6)?)?;
    c.branch(C::Ne, 0x795a4, false)?;
    c.branch(C::Al, 0x7965c, false)?;
    c.literal(C::Al, r(12), 0x79680)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x4c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4c0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(7), 0x838)?)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(6), r(2), 0x1)?)?;
    c.emit(transfer_byte_register(C::Al, true, r(4), r(4), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(6), r(4), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 0x0)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(6), r(4), r(3), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(0), 0x0)?)?;
    c.emit(transfer_byte_register(C::Al, true, r(4), r(6), r(3), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x7f)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(4), r(0), 0x7f)?)?;
    c.emit(transfer_byte_register(C::Al, false, r(4), r(6), r(3), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x34)?)?;
    c.branch(C::Ne, 0x79610, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x3)?)?;
    c.branch(C::Ne, 0x79608, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2014, true)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x79670)
}

fn drain_worker_buffers(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x79684)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x64)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(4), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x1a4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x34)?)?;
    c.literal(C::Al, r(8), 0x797e8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x796f8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 0x18)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(4), 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x50)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x28)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x28)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x30)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x60)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.branch(C::Al, 0x2a878, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x4d0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(13), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(13), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(0), 0x1000)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x356e8, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x79708, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x640)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35d68, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.branch(C::Eq, 0x797e0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x50)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x130)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x797bc, false)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x30)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x4c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x28)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0x60)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.branch(C::Al, 0x2a878, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x110)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x797a8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x7c)?)?;
    c.branch(C::Al, 0x7fd1c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x114)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x797bc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x7c)?)?;
    c.branch(C::Al, 0x7fd1c, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1c0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35d68, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.branch(C::Ne, 0x79744, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x797e8)
}

fn populate_worker_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x797ec)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.literal(C::Al, r(7), 0x798c4)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x72)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xb00)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(10), r(5), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x75)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xb00)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(5), r(10), 0x18)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x24)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(4), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x622f8, true)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 0x38c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.branch(C::Eq, 0x798bc, false)?;
    c.emit(transfer_word(C::Al, true, r(5), r(5), 0x41c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x134)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x2b4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(9), r(9), 0x400)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x100)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x355dc, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x24)?)?;
    c.literal(C::Al, r(2), 0x798c4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, false, r(0), r(1), r(8), Shift::Lsl, 0)?)?;
    c.emit(transfer_word_register(C::Al, false, r(0), r(1), r(9), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x8e)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xb00)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(9), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x48)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(7), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x79874, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.check(0x798c4)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    release_worker_resources(&mut c)?;
    c.data(0x79304, 0xf3d8b)?;
    c.data(0x79308, 0xf3dcc)?;
    c.data(0x7930c, 0xe2788)?;
    c.data(0x79310, 0xf3de3)?;
    reset_worker_and_tables(&mut c)?;
    c.data(0x79670, 0x105a64)?;
    c.data(0x79674, 0x105ca0)?;
    c.data(0x79678, 0x105b68)?;
    c.data(0x7967c, 0x105de0)?;
    c.data(0x79680, 0x1059b0)?;
    drain_worker_buffers(&mut c)?;
    c.data(0x797e8, 0x221007)?;
    populate_worker_records(&mut c)?;
    c.data(0x798c4, 0xf3ce9)?;
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
        assert_eq!(bytes.len(), 1716);
        assert_eq!(CODE_BYTES, 1672);
        assert_eq!(DATA_BYTES, 44);
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!((START..END).step_by(4).filter(|&at| code_address(at)).count(), 418);
        assert_eq!((START..END).step_by(4).filter(|&at| data_value(at).is_some()).count(), 11);
        for (start, end, words) in [(START, 0x79304, 60), (0x79314, 0x79670, 215),
            (0x79684, 0x797e8, 89), (0x797ec, 0x798c4, 54)] {
            assert_eq!((start..end).step_by(4).filter(|&at| code_address(at)).count(), words);
        }
        for at in (START..END).step_by(4) {
            if let Some(value) = data_value(at) {
                assert_eq!(word(&bytes, at), value);
                continue;
            }
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 == 0x0a00_0000 {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                let condition = match instruction >> 28 {
                    0 => C::Eq, 1 => C::Ne, 11 => C::Lt, 14 => C::Al,
                    other => panic!("unexpected branch condition {other}"),
                };
                let offset = ((instruction << 8) as i32 >> 6) as isize;
                let target = (at as isize + 8 + offset) as usize;
                let link = instruction & (1 << 24) != 0;
                assert!(c.branch(C::Mi, target, link).is_err());
                assert!(c.branch(condition, target + 4, link).is_err());
                assert!(c.branch(condition, target, !link).is_err());
                assert!(c.bytes.is_empty());
                c.branch(condition, target, link).unwrap();
                assert_eq!(word(&bytes, at), u32::from_le_bytes(c.bytes.try_into().unwrap()));
            }
            if instruction & 0x0f7f_0000 == 0x051f_0000 {
                let destination = r(((instruction >> 12) & 15) as u8);
                let offset = (instruction & 0xfff) as usize;
                let target = if instruction & (1 << 23) != 0 { at + 8 + offset } else { at + 8 - offset };
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.literal(C::Ne, destination, target).is_err());
                assert!(c.literal(C::Al, r((destination.number() as u8 + 1) % 16), target).is_err());
                assert!(c.literal(C::Al, destination, target + 4).is_err());
                assert!(c.bytes.is_empty());
                c.literal(C::Al, destination, target).unwrap();
                assert_eq!(word(&bytes, at), u32::from_le_bytes(c.bytes.try_into().unwrap()));
            }
        }
        assert_eq!(word(&bytes, 0x79300), branch(C::Al, 0x79300, 0x2ec94, false).unwrap());
        assert_eq!(word(&bytes, 0x79634), data_immediate(C::Cs, Op::Mov, r(4), r(0), 0x7f).unwrap());
        assert_eq!(word(&bytes, 0x79614), transfer_byte_postindexed(C::Al, true, r(6), r(2), 1).unwrap());
        assert_eq!(word(&bytes, 0x793ec), transfer_halfword_register(C::Al, false, r(7), r(9), r(0)).unwrap());
        assert_eq!(word(&bytes, 0x79894), transfer_word_register(C::Al, false, r(0), r(1), r(9), Shift::Lsl, 0).unwrap());
    }

    #[test]
    fn exact_code_data_and_site_guards() {
        for at in (START - 4..END + 8).step_by(4).chain([START + 1, 0x79305, 0x79671, 0x797e9, 0x798c5]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), code_address(at));
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.check(at + 4).is_err());
            assert!(c.data(at + 4, data_value(at + 4).unwrap_or(0)).is_err());
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = data_value(at) {
                assert!(c.data(at, value ^ 1).is_err());
                assert!(c.emit(0).is_err());
                assert!(c.bytes.is_empty());
                c.data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.branch(C::Al, END, false).is_err());
            assert!(c.literal(C::Al, r(0), END).is_err());
            assert!(c.bytes.is_empty());
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
