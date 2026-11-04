use arm32::{branch, data_immediate, data_register, load_halfword, transfer_byte,
    transfer_doubleword, transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x74df0;
pub const END: usize = 0x753f4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1516;
pub const DATA_BYTES: usize = 24;
pub const POOL_START: usize = 0x753dc;
const STACK_BYTES: u32 = 0x20c;
const FIELD_TAG_SLOT: usize = 0x753dc;
const FIELD_TAG: u32 = 0x00121002;
const PACKING_FLAGS_SLOT: usize = 0x753e0;
const PACKING_FLAGS: u32 = 0x02002001;
const VPPSG_SOURCE_SLOT: usize = 0x753e4;
const VPPSG_SOURCE: u32 = 0x000f35fa;
const REORDER_QUEUE_FULL_FORMAT_SLOT: usize = 0x753e8;
const REORDER_QUEUE_FULL_FORMAT: u32 = 0x000ebffd;
const NEWLINE_SLOT: usize = 0x753ec;
const NEWLINE: u32 = 0x000e2788;
const TASK_TAG_SLOT: usize = 0x753f0;
const TASK_TAG: u32 = 0x00221007;
const BUILD_VPPSG_TASK: usize = 0x753f4;
const TRANSFORM_OBJECT: usize = 0x74c64;
const SUBMIT_FIELDS: usize = 0x2a878;
const OBTAIN_OBJECT: usize = 0x35c14;
const QUEUE_OBJECT: usize = 0x35ca0;
const CHECK: usize = 0x17b0;
const REORDER_QUEUE: usize = 0x80928;
const LOG: usize = 0x2ec94;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VppSg task packing address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || (self.at >= POOL_START) != data {
            return Err(format!("invalid VppSg task packing address {:#x}", self.at));
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
        let expected = match at {
            FIELD_TAG_SLOT => FIELD_TAG,
            PACKING_FLAGS_SLOT => PACKING_FLAGS,
            VPPSG_SOURCE_SLOT => VPPSG_SOURCE,
            REORDER_QUEUE_FULL_FORMAT_SLOT => REORDER_QUEUE_FULL_FORMAT,
            NEWLINE_SLOT => NEWLINE,
            TASK_TAG_SLOT => TASK_TAG,
            _ => return Err(format!("unknown VppSg literal {at:#x}")),
        };
        if value != expected { return Err(format!("incorrect literal at {at:#x}")); }
        self.word(value, true)
    }

    fn literal(&mut self, condition: C, rd: R, at: usize) -> Result<(), String> {
        if !(POOL_START..END).contains(&at) || at % 4 != 0 {
            return Err(format!("invalid VppSg literal target {at:#x}"));
        }
        self.emit(transfer_word(condition, true, rd, R::PC, at as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if target % 4 != 0 || (POOL_START..END).contains(&target) {
            return Err(format!("invalid VppSg branch target {target:#x}"));
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn dispatch_task_mode(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74df0)?;
    c.emit(transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), STACK_BYTES)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(10), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x74e34, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x74e60, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, BUILD_VPPSG_TASK, true)?;
    c.branch(C::Al, 0x74e80, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x110)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74f58, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.branch(C::Ne, 0x74fb0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, BUILD_VPPSG_TASK, true)?;
    c.branch(C::Al, 0x75104, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0xb8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x74e80, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.branch(C::Al, TRANSFORM_OBJECT, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x74ea0, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0xd14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x1a8)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x75104, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75104, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x41c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x75104, false)?;
    Ok(())
}

fn pack_field_notification(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74eb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0xc)?)?;
    c.literal(C::Al, r(1), TASK_TAG_SLOT)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x8)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x120)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x50)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x4c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x48)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x18)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x4c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x18)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x28)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x140)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x144)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x30)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x148)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x34)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x14c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x150)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x3c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x154)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x40)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x158)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0x44)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x20)?)?;
    c.branch(C::Al, SUBMIT_FIELDS, true)?;
    c.branch(C::Al, 0x75104, false)?;
    Ok(())
}

fn queue_secondary_object(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74f58)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x75104, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75104, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x75104, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, OBTAIN_OBJECT, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75104, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, r(1), 0x8c, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(0), 0x8c, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(1), 0x9c, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(0), 0x9c, BlockMode::IncrementAfter, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, QUEUE_OBJECT, true)?;
    c.branch(C::Al, 0x75104, false)?;
    Ok(())
}

fn pack_reorder_task(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74fb0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0xc8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x8)?)?;
    c.literal(C::Al, r(1), FIELD_TAG_SLOT)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x10)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x2)?)?;
    c.literal(C::Al, r(2), PACKING_FLAGS_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x14)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0xc0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 0xc4)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x28)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x2c)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(10), 0x38)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 0x30)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x34)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x124)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x3c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x40)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x44)?)?;
    Ok(())
}

fn select_task_kind(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75038)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x7508c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x75098, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x750a4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x18)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x7)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x750b8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x30)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x750b8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x9)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x18)?)?;
    c.branch(C::Al, 0x750b8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x18)?)?;
    c.branch(C::Al, 0x750b8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x18)?)?;
    c.branch(C::Al, 0x750b8, false)?;
    c.literal(C::Al, r(2), VPPSG_SOURCE_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, CHECK, true)?;
    Ok(())
}

fn submit_reorder_task(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x750b8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x38)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0xc)?)?;
    c.branch(C::Al, REORDER_QUEUE, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x750fc, false)?;
    c.literal(C::Al, r(1), REORDER_QUEUE_FULL_FORMAT_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, LOG, true)?;
    c.literal(C::Al, r(1), NEWLINE_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, LOG, true)?;
    c.literal(C::Al, r(2), VPPSG_SOURCE_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa7)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, CHECK, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 0x44)?)?;
    Ok(())
}

fn build_output_object(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75104)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x350)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75124, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0xd14)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x1a8)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x753d4, false)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(7), 0x1c)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, OBTAIN_OBJECT, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x751d8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(14), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(7), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, r(2), 0x92, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(3), 0x92, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(2), 0xb2, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(3), 0xb2, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x108)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x4c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x104)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x48)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x10c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x50)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0xc)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x1b0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xf4)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x1ac)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xf0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(12), 0x838)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xa0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x3c)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(12), 0x15e)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x2)?)?;
    c.branch(C::Ne, 0x751ec, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x18)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(8), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x2c)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x30)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x20)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x28)?)?;
    c.branch(C::Al, 0x751f4, false)?;
    c.literal(C::Al, r(2), VPPSG_SOURCE_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x100)?)?;
    c.branch(C::Al, 0x753d0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x24)?)?;
    Ok(())
}

fn pack_output_layout(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x751f4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(transfer_word_register(C::Al, false, r(1), r(0), r(7), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x30)?)?;
    c.emit(transfer_word_register(C::Al, false, r(1), r(0), r(3), Shift::Lsl, 0)?)?;
    c.branch(C::Eq, 0x75230, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0x24)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x28)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(1), r(3), 0x7f)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Cc, Op::Add, r(1), r(3), 0x80)?)?;
    c.emit(data_immediate(C::Cc, Op::Bic, r(1), r(1), 0x7f)?)?;
    c.emit(data_register(C::Cc, Op::Sub, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Cc, false, r(1), r(0), 0x28)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 0x4)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(0), 0x3)?)?;
    c.branch(C::Eq, 0x75260, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x3c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 0x128)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x24)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x28)?)?;
    Ok(())
}

fn copy_output_fields(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75260)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(8), 0xcc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x58)?)?;
    c.emit(transfer_multiple(C::Al, true, r(1), 0xb8, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(2), 0xb8, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(1), 0xb8, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(2), 0xb8, BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(1), 0xf8, BlockMode::IncrementAfter, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(8), 0x104)?)?;
    c.emit(transfer_multiple(C::Al, false, r(2), 0xf8, BlockMode::IncrementAfter, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x84)?)?;
    c.emit(transfer_multiple(C::Al, true, r(1), 0xf8, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(2), 0xf8, BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x100)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x80)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x118)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x98)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x11c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0x9c)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x120)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xa4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x124)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xe4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x128)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xe0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x12c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xd8)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0x130)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xdc)?)?;
    Ok(())
}

fn pack_optional_fields(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x752d0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(12), 0x215)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x7531c, false)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 0xb4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc0)?)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 0xb6)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc4)?)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 0xb8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xc8)?)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 0xba)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xcc)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x138)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x139)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd1)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x13a)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd2)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x13b)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd3)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(12), 0x15e)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Ne, 0x75390, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd4)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(12), 0x215)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Eq, 0x75390, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(12), 0x216)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0xbc)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xb0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0xc0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xb4)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0xc4)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xb8)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(8), 0xc8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0xbc)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x138)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x139)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd1)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x13a)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd2)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x13b)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0xd3)?)?;
    Ok(())
}

fn queue_and_finish(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75390)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, QUEUE_OBJECT, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x118)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x753d4, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x114)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(11), -0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x100)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(11), 0x28)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(11), -0x24)?)?;
    c.branch(C::Al, REORDER_QUEUE, true)?;
    c.literal(C::Al, r(2), VPPSG_SOURCE_SLOT)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x190)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, CHECK, true)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    dispatch_task_mode(&mut c)?;
    pack_field_notification(&mut c)?;
    queue_secondary_object(&mut c)?;
    pack_reorder_task(&mut c)?;
    select_task_kind(&mut c)?;
    submit_reorder_task(&mut c)?;
    build_output_object(&mut c)?;
    pack_output_layout(&mut c)?;
    copy_output_fields(&mut c)?;
    pack_optional_fields(&mut c)?;
    queue_and_finish(&mut c)?;
    c.literal_data(FIELD_TAG_SLOT, FIELD_TAG)?;
    c.literal_data(PACKING_FLAGS_SLOT, PACKING_FLAGS)?;
    c.literal_data(VPPSG_SOURCE_SLOT, VPPSG_SOURCE)?;
    c.literal_data(REORDER_QUEUE_FULL_FORMAT_SLOT, REORDER_QUEUE_FULL_FORMAT)?;
    c.literal_data(NEWLINE_SLOT, NEWLINE)?;
    c.literal_data(TASK_TAG_SLOT, TASK_TAG)?;
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
    fn exact_extent_and_named_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(POOL_START - START, CODE_BYTES);
        assert_eq!(END - POOL_START, DATA_BYTES);
        for (at, value) in [
            (FIELD_TAG_SLOT, FIELD_TAG),
            (PACKING_FLAGS_SLOT, PACKING_FLAGS),
            (VPPSG_SOURCE_SLOT, VPPSG_SOURCE),
            (REORDER_QUEUE_FULL_FORMAT_SLOT, REORDER_QUEUE_FULL_FORMAT),
            (NEWLINE_SLOT, NEWLINE),
            (TASK_TAG_SLOT, TASK_TAG),
        ] {
            assert_eq!(word(&bytes, at), value);
        }
        assert_eq!(word(&bytes, START + 8),
            data_immediate(C::Al, Op::Sub, R::SP, R::SP, STACK_BYTES).unwrap());
        for (at, target) in [(0x74e2c, BUILD_VPPSG_TASK),
            (0x74e58, BUILD_VPPSG_TASK), (0x74e7c, TRANSFORM_OBJECT)] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target as u32, true).unwrap());
        }
    }

    #[test]
    fn checked_code_and_named_data_boundaries() {
        let instruction = data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap();
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(instruction).is_ok(), at < POOL_START);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal_data(at, FIELD_TAG.wrapping_add(1)).is_err());
            assert!(c.bytes.is_empty());
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(instruction).is_err());
            assert!(c.literal_data(at, FIELD_TAG).is_err());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.literal_data(FIELD_TAG_SLOT, FIELD_TAG).is_err());
        assert!(c.literal(C::Al, r(0), START).is_err());
        assert!(c.literal(C::Al, r(0), POOL_START + 1).is_err());
        assert!(c.branch(C::Al, POOL_START, false).is_err());
        assert!(c.branch(C::Al, START + 1, false).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(reference.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), identity.sha256);
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
