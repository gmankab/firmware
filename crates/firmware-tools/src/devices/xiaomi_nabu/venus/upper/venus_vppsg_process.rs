use arm32::{branch, data_immediate, data_register, transfer_byte, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x73e10;
pub const END: usize = 0x74ab0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 3136;
pub const DATA_BYTES: usize = 96;
pub const POOL_START: usize = 0x74a50;
const STACK_BYTES: u32 = 0x1ec;
const SAVED_REGISTERS: u16 = 0x4ff0;
const RESTORED_REGISTERS: u16 = 0x8ff0;

fn r(number: u8) -> R { R::new(number).unwrap() }

fn pool_value(at: usize) -> Option<u32> {
    match at {
        0x74a50 => Some(0x000f35fa),
        0x74a54 => Some(0xe00e00c0),
        0x74a58 => Some(0x000f3780),
        0x74a5c => Some(0x000e2788),
        0x74a60 => Some(0x000f37cc),
        0x74a64 => Some(0x000f3809),
        0x74a68 => Some(0x000f3836),
        0x74a6c => Some(0x000f3857),
        0x74a70 => Some(0x000f3876),
        0x74a74 => Some(0x000f3895),
        0x74a78 => Some(0x000f38e5),
        0x74a7c => Some(0x00104090),
        0x74a80 => Some(0x00104094),
        0x74a84 => Some(0x000f3930),
        0x74a88 => Some(0x000f394c),
        0x74a8c => Some(0x000f395c),
        0x74a90 => Some(0x000f399c),
        0x74a94 => Some(0x00221007),
        0x74a98 => Some(0x000f39ea),
        0x74a9c => Some(0x000f3681),
        0x74aa0 => Some(0x000f36b3),
        0x74aa4 => Some(0x000f36f0),
        0x74aa8 => Some(0x000f372d),
        0x74aac => Some(0x000f3654),
        _ => None,
    }
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("encVPPSGProcess address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || (self.at >= POOL_START) != data {
            return Err(format!("invalid encVPPSGProcess extent {:#x}", self.at));
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
        if pool_value(at) != Some(value) {
            return Err(format!("invalid encVPPSGProcess literal {at:#x}"));
        }
        self.word(value, true)
    }

    fn literal(&mut self, condition: C, rd: R, target: usize) -> Result<(), String> {
        if pool_value(target).is_none() {
            return Err(format!("invalid encVPPSGProcess pool target {target:#x}"));
        }
        self.emit(transfer_word(condition, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if target % 4 != 0 || (POOL_START..END).contains(&target) {
            return Err(format!("invalid encVPPSGProcess branch target {target:#x}"));
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn select_process_kind(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x73e10)?;
    c.emit(transfer_multiple(C::Al, false, r(13), SAVED_REGISTERS, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), STACK_BYTES)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x284)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2000)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x2c)?)?;
    c.branch(C::Ge, 0x73e4c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Eq, 0x73e60, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1000)?)?;
    c.branch(C::Ne, 0x73e70, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd)?)?;
    c.branch(C::Al, 0x73e7c, false)?;
    c.branch(C::Eq, 0x73e68, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4000)?)?;
    c.branch(C::Ne, 0x73e70, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe)?)?;
    c.branch(C::Al, 0x73e7c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb)?)?;
    c.branch(C::Al, 0x73e7c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xc)?)?;
    c.branch(C::Al, 0x73e7c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x8000)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0xc)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(4), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3084, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x293c, true)?;
    Ok(())
}

fn dispatch_context_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x73e94)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x6)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x5)?)?;
    c.branch(C::Cs, 0x73f08, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x74a48, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x73ef8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x114)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x100)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x3c)?)?;
    c.branch(C::Al, 0x80928, true)?;
    c.literal(C::Al, r(2), 0x74a50)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x194)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x14)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.branch(C::Al, 0x74a48, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x5)?)?;
    c.branch(C::Ne, 0x74a48, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1800)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x280)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x14)?)?;
    c.branch(C::Al, 0x74a30, false)?;
    Ok(())
}

fn acquire_output_object(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x73f30)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x640)?)?;
    c.branch(C::Al, 0x35d44, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(C::Eq, 0x74088, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x73f94, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x73f94, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x40)?)?;
    c.branch(C::Al, 0x35c14, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x73f94, false)?;
    c.literal(C::Al, r(1), 0x74a9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 0xb8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x140)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x742b8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74298, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x7c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(2), r(11), 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(13), 0x34)?)?;
    c.branch(C::Al, 0x7f990, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7440c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x10)?)?;
    c.emit(transfer_byte(C::Al, false, r(10), r(1), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x30)?)?;
    c.branch(C::Al, 0x8025c, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x30)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x34)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x70)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x73a1c, true)?;
    c.branch(C::Al, 0x742b8, false)?;
    Ok(())
}

fn prepare_input_layout(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74004)?;
    c.emit(transfer_word(C::Al, true, r(7), r(4), 0x30)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x160)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(7), 0x18)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7403c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Eq, false, r(10), r(7), 0x74)?)?;
    c.emit(transfer_byte(C::Ne, false, r(0), r(9), 0x2)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x24)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x413)?)?;
    c.emit(transfer_byte(C::Al, true, r(10), r(4), 0x3de)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x78)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x21)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x1)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x1c)?)?;
    c.branch(C::Eq, 0x740ac, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x1)?)?;
    c.branch(C::Ne, 0x740d4, false)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(0), 0x10)?)?;
    c.branch(C::Al, 0x740d8, false)?;
    c.literal(C::Al, r(1), 0x74aac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.branch(C::Al, 0x743e8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x134)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x740d8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x3b0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.emit(transfer_word(C::Cs, true, r(1), r(4), 0x298)?)?;
    c.emit(data_register(C::Cs, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 28, false)?)?;
    c.emit(data_immediate(C::Cs, Op::Orr, r(3), r(0), 0x108)?)?;
    c.branch(C::Al, 0x740d8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xab8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x153)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x740f0, false)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x40)?)?;
    c.branch(C::Al, 0x74108, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0xcc8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74108, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xd00)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Orr, r(3), r(3), 0x40)?)?;
    Ok(())
}

fn configure_processing(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74108)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x130)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 0x3c)?)?;
    c.branch(C::Eq, 0x744e4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x298)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x28744, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xab8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x153)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x74174, false)?;
    c.literal(C::Al, r(0), 0x74a54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x7418c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0xcc8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7418c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xd00)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74164, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.literal(C::Al, r(6), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x741ac, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x1a8)?)?;
    c.literal(C::Al, r(1), 0x74a58)?;
    c.branch(C::Al, 0x741b8, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x284)?)?;
    c.literal(C::Al, r(1), 0x74a60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn transform_input_objects(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x741cc)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0x28)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.branch(C::Al, 0x73bb8, true)?;
    c.branch(C::Al, 0x26754, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x413)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74444, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x624)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Cc, 0x7443c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0xc)?)?;
    c.branch(C::Al, 0x294c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0xc)?)?;
    c.branch(C::Al, 0x293c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x12c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x52510, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x74434, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x12c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(0), r(6), Shift::Lsl, 2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x110)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb68)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x64)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x48)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0x194)?)?;
    c.branch(C::Al, 0x74c64, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x12c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0x2)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(6), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x110)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x624)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0x74208, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x74438, false)?;
    Ok(())
}

fn complete_output_fields(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74298)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x1b8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x114)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x11c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x73a1c, true)?;
    c.emit(transfer_word(C::Al, false, r(10), r(5), 0x114)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x743e8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x74350, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(0), r(2), 0x110)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x74338, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0xb8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x74338, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x7c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(5), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x38)?)?;
    c.branch(C::Al, 0x7f990, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x740a0, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x30)?)?;
    c.branch(C::Al, 0x8025c, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 0x1a8)?)?;
    c.literal(C::Al, r(1), 0x74aa4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x74350, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x134)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x74350, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x298)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Cc, 0x743e8, false)?;
    Ok(())
}

fn assign_output_buffer(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74350)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x128)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x743e8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x743e8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x614)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x614)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(4), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(1), 0x8e0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(5), 0x128)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x5fc)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(C::Al, r(1), 0x74aa8)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x5fc)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x614)?)?;
    c.branch(C::Al, 0x81850, true)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 0x614)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x128)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x743e8, false)?;
    c.literal(C::Al, r(2), 0x74a50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xde)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    Ok(())
}

fn commit_output_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x743e8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x18)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(4), 0xd00)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.branch(C::Al, 0x7497c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x10)?)?;
    c.literal(C::Al, r(1), 0x74aa0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x740a4, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x194)?)?;
    Ok(())
}

fn signal_processing_events(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74444)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 0x40)?)?;
    c.branch(C::Ne, 0x744a8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xab8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x153)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x744a8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3d9)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x74470, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x8)?)?;
    c.branch(C::Al, 0x294c, true)?;
    c.literal(C::Al, r(1), 0x74a6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(6), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x413)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x744e4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0xc)?)?;
    c.branch(C::Al, 0x294c, true)?;
    c.literal(C::Al, r(1), 0x74a70)?;
    c.branch(C::Al, 0x744d0, false)?;
    c.literal(C::Al, r(1), 0x74a64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(6), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x8)?)?;
    c.branch(C::Al, 0x294c, true)?;
    c.literal(C::Al, r(1), 0x74a68)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn log_processing_completion(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x744e4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74510, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(9), 0x1a8)?)?;
    c.branch(C::Al, 0x26754, true)?;
    c.literal(C::Al, r(1), 0x74a74)?;
    c.emit(data_register(C::Al, Op::Sub, r(3), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x74538, false)?;
    c.emit(transfer_word(C::Al, true, r(6), r(9), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 0x284)?)?;
    c.branch(C::Al, 0x26754, true)?;
    c.literal(C::Al, r(1), 0x74a78)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(13), 0x38)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x747c8, false)?;
    Ok(())
}

fn handle_debug_completion(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74558)?;
    c.literal(C::Al, r(0), 0x74a7c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x745e8, false)?;
    c.literal(C::Al, r(1), 0x74a80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.literal(C::Al, r(10), 0x74a88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xba)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x3c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0x0)?)?;
    c.literal(C::Al, r(1), 0x74a84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(6), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x10)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xbb)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x400)?)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x7, BlockMode::IncrementAfter, false)?)?;
    c.literal(C::Al, r(1), 0x74a8c)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x28)?)?;
    c.branch(C::Al, 0x74720, false)?;
    Ok(())
}

fn prepare_hardware_status(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x745e8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3d9)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0xe0000008)?)?;
    c.emit(transfer_word(C::Ne, false, r(10), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xab8)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x28)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x153)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74628, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x60)?)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xe0000000)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.literal(C::Al, r(0), 0x74a7c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74648, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x48)?)?;
    c.branch(C::Al, 0x74720, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x14)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.branch(C::Al, 0x513b8, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x413)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74690, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xb68)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x638)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x60)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x174)?)?;
    Ok(())
}

fn check_output_capacity(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74690)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x746ac, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x640)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x648)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x74700, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x284)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x746fc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x50)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x54)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x698)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsr, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, 0x746ec, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x638)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x58)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsr, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x74a20, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x40)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x3c)?)?;
    c.branch(C::Al, 0x7463c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x638)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x48)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x40)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x3c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Hi, false, r(10), r(13), 0x48)?)?;
    Ok(())
}

fn submit_processing_result(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74720)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x20)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x1c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x28c2c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x747c8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x413)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x747c8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74774, false)?;
    c.literal(C::Al, r(2), 0x74a50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xcb)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(9), 0xec)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x5)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(9), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0x14)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x53a28, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x53b24, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0xc)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 0x0)?)?;
    c.literal(C::Al, r(1), 0x74a90)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn dispatch_completion(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x747c8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0xcc8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x747e0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xd00)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x748e8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xab8)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x153)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x748b4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x74888, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.branch(C::Al, 0x74c64, true)?;
    c.emit(transfer_word(C::Al, false, r(10), r(11), -0x64)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x70)?)?;
    c.literal(C::Al, r(0), 0x74a94)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x6c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x68)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x54)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x50)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x70)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x5c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x60)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x58)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(11), -0x24)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(9), 0x44)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.branch(C::Al, 0x2a878, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x640)?)?;
    c.branch(C::Al, 0x35d68, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1c0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x14)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x4)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(6), r(0), 0x1)?)?;
    c.branch(C::Al, 0x7497c, false)?;
    Ok(())
}

fn advance_output_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x748b4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x748f8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x74df0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x7490c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x34)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x74914, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0xd00)?)?;
    c.branch(C::Al, 0x7497c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74988, false)?;
    c.emit(transfer_word(C::Al, false, r(10), r(9), 0x130)?)?;
    c.branch(C::Al, 0x74888, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xd14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x1a8)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x74958, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x640)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x14)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x4)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, false, r(10), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.branch(C::Al, 0x7497c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 0x1a8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x110)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Eq, Op::Add, r(0), r(9), 0x128)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(7), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x74a30, false)?;
    c.branch(C::Al, 0x74a48, false)?;
    Ok(())
}

fn recycle_input_object(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74988)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x749dc, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x5)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x749d8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.literal(C::Al, r(1), 0x74a98)?;
    c.literal(C::Al, r(2), 0x74a88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x41)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x500)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x74a5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x749dc, false)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x3c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x74df0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x640)?)?;
    c.branch(C::Al, 0x35d68, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1c0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x7487c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x34)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x74884, false)?;
    Ok(())
}

fn clear_capacity_flags(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74a20)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x40)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x3c)?)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x38)?)?;
    c.branch(C::Al, 0x74720, false)?;
    Ok(())
}

fn poll_context_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74a30)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x74004, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x73f30, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), RESTORED_REGISTERS, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    select_process_kind(&mut c)?;
    dispatch_context_state(&mut c)?;
    acquire_output_object(&mut c)?;
    prepare_input_layout(&mut c)?;
    configure_processing(&mut c)?;
    transform_input_objects(&mut c)?;
    complete_output_fields(&mut c)?;
    assign_output_buffer(&mut c)?;
    commit_output_state(&mut c)?;
    signal_processing_events(&mut c)?;
    log_processing_completion(&mut c)?;
    handle_debug_completion(&mut c)?;
    prepare_hardware_status(&mut c)?;
    check_output_capacity(&mut c)?;
    submit_processing_result(&mut c)?;
    dispatch_completion(&mut c)?;
    advance_output_state(&mut c)?;
    recycle_input_object(&mut c)?;
    clear_capacity_flags(&mut c)?;
    poll_context_state(&mut c)?;
    c.check(POOL_START)?;
    if c.bytes.len() != CODE_BYTES { return Err("incorrect process code extent".into()); }
    for at in (POOL_START..END).step_by(4) {
        c.literal_data(at, pool_value(at).ok_or("missing process literal")?)?;
    }
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES { return Err("incorrect process extent".into()); }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn exact_code_and_data_extents() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 3232);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        assert_eq!(CODE_BYTES + DATA_BYTES, bytes.len());
        for at in (POOL_START..END).step_by(4) {
            assert_eq!(word(&bytes, at), pool_value(at).unwrap());
        }
    }

    #[test]
    fn instruction_hazards_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, START), transfer_multiple(C::Al, false, R::SP,
            SAVED_REGISTERS, BlockMode::DecrementBefore, true).unwrap());
        assert_eq!(word(&bytes, START + 8),
            data_immediate(C::Al, Op::Sub, R::SP, R::SP, STACK_BYTES).unwrap());
        assert_eq!(word(&bytes, 0x73e30), branch(C::Ge, 0x73e30, 0x73e4c, false).unwrap());
        assert_eq!(word(&bytes, 0x73e7c),
            transfer_doubleword(C::Al, true, r(2), r(4), 0x10).unwrap());
        assert_eq!(word(&bytes, 0x742d4),
            transfer_word_preindexed_writeback(C::Al, true, r(0), r(2), 0x110).unwrap());
        assert_eq!(word(&bytes, 0x745bc), transfer_multiple(C::Al, false, R::SP,
            7, BlockMode::IncrementAfter, false).unwrap());
        assert_eq!(word(&bytes, 0x74a4c), transfer_multiple(C::Al, true, R::SP,
            RESTORED_REGISTERS, BlockMode::IncrementAfter, true).unwrap());
        for at in (START..POOL_START).step_by(4) {
            let instruction = word(&bytes, at);
            assert_ne!(instruction >> 28, 15);
            assert_ne!(instruction & 0x0fff_fff0, 0x012f_ff30);
            if instruction & 0x0e00_0000 == 0x0a00_0000 {
                let displacement = ((instruction << 8) as i32 >> 6) as isize;
                let target = (at as isize + 8 + displacement) as usize;
                assert_eq!(target % 4, 0);
                assert!(!(POOL_START..END).contains(&target));
                if instruction & 0x0100_0000 == 0 {
                    assert!((START..POOL_START).contains(&target));
                }
            }
            if instruction & 0x0f7f_0000 == 0x051f_0000 {
                let displacement = (instruction & 0xfff) as usize;
                let target = if instruction & 0x0080_0000 != 0 {
                    at + 8 + displacement
                } else {
                    at + 8 - displacement
                };
                assert!(pool_value(target).is_some());
            }
        }
    }

    #[test]
    fn exact_guards_reject_without_emitting() {
        let instruction = data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap();
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(instruction).is_ok(), at < POOL_START);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal_data(at, pool_value(at).unwrap_or(0).wrapping_add(1)).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = pool_value(at) {
                c.literal_data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(instruction).is_err());
            assert!(c.literal_data(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.literal_data(POOL_START, pool_value(POOL_START).unwrap()).is_err());
        for target in [START, POOL_START + 1, END] {
            assert!(c.literal(C::Al, r(0), target).is_err());
        }
        for target in [START + 1, POOL_START, END - 4] {
            assert!(c.branch(C::Al, target, false).is_err());
        }
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1159200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
