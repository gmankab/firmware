use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    branch_exchange_with_link, data_immediate, data_register, mov_register, transfer_multiple,
    transfer_word, transfer_word_register, transfer_word_register_preindexed_writeback,
};

pub const START: usize = 0xdab4;
pub const END: usize = 0xdf74;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1132;
pub const DATA_BYTES: usize = 84;
pub const POOL_START: usize = 0xdf20;
const POOL: [u32; 21] = [
    0x000e39e4, 0x000e2788, 0x000e3a10, 0x000e3979, 0x000e39af, 0x00103d30, 0x000e3a3b, 0x000e3a68,
    0x04011009, 0x0401100b, 0x0401100d, 0x000e3ada, 0x000e3abe, 0xffffe42c, 0x000e3a92, 0xe0008480,
    0x000e3896, 0x000e3afe, 0x000e3b2c, 0x000e3b59, 0x000e3b8c,
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!(
                "service state dispatch address {:#x}, expected {expected:#x}",
                self.at
            ));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 || (self.at >= POOL_START) != data {
            return Err(format!(
                "invalid service state dispatch address {:#x}",
                self.at
            ));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn literal_data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !(POOL_START..END).contains(&at) || at % 4 != 0 || POOL[(at - POOL_START) / 4] != value {
            return Err(format!("invalid service state literal at {at:#x}"));
        }
        self.word(value, true)
    }

    fn literal(&mut self, condition: C, rd: R, at: usize) -> Result<(), String> {
        if !(POOL_START..END).contains(&at) || at % 4 != 0 {
            return Err(format!("invalid service state literal target {at:#x}"));
        }
        self.emit(transfer_word(
            condition,
            true,
            rd,
            R::PC,
            at as i32 - self.at as i32 - 8,
        )?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if target % 4 != 0 || (POOL_START..END).contains(&target) {
            return Err(format!("invalid service state branch target {target:#x}"));
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn dispatch_entry(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdab4)?;
    c.emit(transfer_multiple(
        C::Al,
        false,
        R::SP,
        0x4bf0,
        BlockMode::DecrementBefore,
        true,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 8)?)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(0), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.branch(C::Eq, 0xdb3c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 7)?)?;
    c.branch(C::Eq, 0xdb48, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 6)?)?;
    c.branch(C::Eq, 0xdaf8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.emit(transfer_word(C::Eq, true, r(0), r(4), 0x2d8)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, r(0), r(0), 2)?)?;
    c.branch(C::Ne, 0xdb80, false)?;
    c.literal(C::Al, r(1), 0xdf2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x25954, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0xdce4, false)?;
    c.literal(C::Al, r(1), 0xdf30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0xdb7c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x18)?)?;
    c.branch(C::Al, 0xdedc, false)?;
    Ok(())
}

fn dispatch_mode_seven(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdb48)?;
    c.literal(C::Al, r(1), 0xdf20)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(6), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x259b4, true)?;
    c.literal(C::Al, r(1), 0xdf28)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn process_service(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdb80)?;
    c.branch(C::Al, 0x25984, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0xdcd4, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x2d8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.literal(C::Al, r(7), 0xdf34)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 2)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.literal(C::Al, r(1), 0xdf38)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(3),
        r(7),
        r(0),
        Shift::Lsl,
        4,
        false,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(6), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x26764, true)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.branch(C::Al, 0x22fac, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x2d8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0xc)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.literal(C::Al, r(1), 0xdf3c)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(3),
        r(7),
        r(0),
        Shift::Lsl,
        4,
        false,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x26754, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1c00)?)?;
    c.emit(transfer_word_register(
        C::Al,
        false,
        r(0),
        r(4),
        r(1),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.literal(C::Al, r(1), 0xdf40)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(0),
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Eq, 0xdcf4, false)?;
    c.literal(C::Al, r(1), 0xdf44)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(0),
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Eq, 0xdd2c, false)?;
    c.literal(C::Al, r(1), 0xdf48)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(0),
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Ne, 0xdd68, false)?;
    Ok(())
}

fn dispatch_callback_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdc44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2ac)?)?;
    c.emit(mov_register(C::Al, r(6), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(
        C::Al,
        true,
        r(0),
        r(6),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.literal(C::Al, r(1), 0xdf54)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(2),
        r(6),
        r(1),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0xdc7c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x38c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1800)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(5),
        r(0),
        Shift::Lsl,
        2,
    )?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(3),
        r(0),
        r(1),
        Shift::Lsl,
        0,
    )?)?;
    c.literal(C::Al, r(1), 0xdf58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2a8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(4),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(6), 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0xdcc8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2b8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(4),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0xdee8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 0x18)?)?;
    c.branch(C::Al, 0xdd78, false)?;
    Ok(())
}

fn clear_pending_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdcd4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x18)?)?;
    c.branch(C::Al, 0x259a4, true)?;
    c.branch(C::Al, 0xdedc, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x18)?)?;
    c.branch(C::Al, 0x25974, true)?;
    c.branch(C::Al, 0xdedc, false)?;
    Ok(())
}

fn dispatch_service_kinds(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdcf4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x38c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1800)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(5),
        r(0),
        Shift::Lsl,
        2,
    )?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(2),
        r(0),
        r(1),
        Shift::Lsl,
        0,
    )?)?;
    c.literal(C::Al, r(1), 0xdf4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0x18)?)?;
    c.branch(C::Al, 0xdd78, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x38c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1800)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(5),
        r(0),
        Shift::Lsl,
        2,
    )?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(2),
        r(0),
        r(1),
        Shift::Lsl,
        0,
    )?)?;
    c.literal(C::Al, r(1), 0xdf50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 2)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 0x18)?)?;
    c.branch(C::Al, 0xdd78, false)?;
    c.literal(C::Al, r(2), 0xdf60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe2)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    Ok(())
}

fn invoke_service_callback(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdd78)?;
    c.branch(C::Al, 0x2693c, true)?;
    c.branch(C::Al, 0x259a4, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 8)?)?;
    c.literal(C::Al, r(1), 0xdf48)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(0),
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Ne, 0xddf4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2a8)?)?;
    c.emit(mov_register(C::Al, r(6), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(
        C::Al,
        true,
        r(1),
        r(6),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0xddf4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2ac)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(4),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0xddf4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2b8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(4),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Eq, 0xdddc, false)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.branch(C::Al, 0xd9cc, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x18)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x1c)?)?;
    Ok(())
}

fn validate_and_finish(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xddf4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x8c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.branch(C::Ne, 0xde44, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x208)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x388)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1c00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1800)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(1),
        r(4),
        r(0),
        Shift::Lsl,
        0,
    )?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(5),
        r(0),
        Shift::Lsl,
        2,
    )?)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(0),
        r(0),
        r(2),
        Shift::Lsl,
        0,
    )?)?;
    c.branch(C::Al, 0x7fd1c, true)?;
    c.literal(C::Al, r(2), 0xdf60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(0), r(1)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x104)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 7)?)?;
    c.branch(C::Ne, 0xde8c, false)?;
    c.literal(C::Al, r(1), 0xdf64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(5), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x259c4, true)?;
    c.literal(C::Al, r(1), 0xdf68)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 6)?)?;
    c.branch(C::Eq, 0xdea4, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.emit(transfer_word(C::Eq, true, r(0), r(4), 0x2d8)?)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, r(0), r(0), 2)?)?;
    c.branch(C::Ne, 0xdedc, false)?;
    c.literal(C::Al, r(1), 0xdf6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(4), 0xdf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x25974, true)?;
    c.literal(C::Al, r(1), 0xdf70)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x18)?)?;
    c.emit(transfer_multiple(
        C::Al,
        true,
        R::SP,
        0x8bf0,
        BlockMode::IncrementAfter,
        true,
    )?)?;
    Ok(())
}

fn cold_copy_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xdee8)?;
    c.literal(C::Al, r(2), 0xdf5c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x21c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 9)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), -0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(2), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 4)?)?;
    c.branch(C::Ne, 0xdef8, false)?;
    c.branch(C::Al, 0xdd78, false)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 {
        at: START,
        bytes: Vec::with_capacity(END - START),
    };
    dispatch_entry(&mut c)?;
    dispatch_mode_seven(&mut c)?;
    process_service(&mut c)?;
    dispatch_callback_state(&mut c)?;
    clear_pending_state(&mut c)?;
    dispatch_service_kinds(&mut c)?;
    invoke_service_callback(&mut c)?;
    validate_and_finish(&mut c)?;
    cold_copy_state(&mut c)?;
    c.check(POOL_START)?;
    for (index, value) in POOL.into_iter().enumerate() {
        c.literal_data(POOL_START + index * 4, value)?;
    }
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
    fn exact_extent_pool_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1216);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        for (index, value) in POOL.into_iter().enumerate() {
            assert_eq!(word(&bytes, POOL_START + index * 4), value);
        }
        assert_eq!(
            word(&bytes, 0xdde0),
            branch_exchange_with_link(C::Al, r(1)).unwrap()
        );
        for (at, target, condition) in [
            (0xdcc4, 0xdee8, C::Ne),
            (0xdf18, 0xdef8, C::Ne),
            (0xdf1c, 0xdd78, C::Al),
        ] {
            assert_eq!(
                word(&bytes, at),
                branch(condition, at as u32, target as u32, false).unwrap()
            );
        }
        assert_eq!(
            word(&bytes, 0xdee4),
            transfer_multiple(C::Al, true, R::SP, 0x8bf0, BlockMode::IncrementAfter, true).unwrap()
        );
        assert_eq!(
            word(&bytes, 0xdee8),
            transfer_word(C::Al, true, r(2), R::PC, 0x6c).unwrap()
        );
    }

    #[test]
    fn checked_code_and_pool_boundaries() {
        let instruction = mov_register(C::Al, r(0), r(0));
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 {
                at,
                bytes: Vec::new(),
            };
            assert_eq!(c.emit(instruction).is_ok(), at < POOL_START);
            let mut c = TypedA32 {
                at,
                bytes: Vec::new(),
            };
            assert_eq!(c.literal_data(at, POOL[0]).is_ok(), at == POOL_START);
            let mut c = TypedA32 {
                at,
                bytes: Vec::new(),
            };
            assert!(c.literal_data(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = TypedA32 {
                at,
                bytes: Vec::new(),
            };
            assert!(c.emit(instruction).is_err());
            assert!(c.literal_data(at, POOL[0]).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 {
            at: START,
            bytes: Vec::new(),
        };
        assert!(c.check(END).is_err());
        assert!(c.literal_data(POOL_START, POOL[0]).is_err());
        for target in [START, POOL_START + 1, END] {
            assert!(c.literal(C::Al, r(0), target).is_err());
        }
        for target in (POOL_START..END).step_by(4).chain([START + 1]) {
            assert!(c.branch(C::Al, target, false).is_err());
        }
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE"))
        else {
            return;
        };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(
            crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085"
        );
        assert_eq!(
            fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]
        );
    }
}
