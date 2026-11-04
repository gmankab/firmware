use arm32::{
    branch, branch_exchange_immediate, branch_exchange_with_link, bx, data_immediate,
    data_register, transfer_byte, transfer_multiple, transfer_word, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xd040;
pub const END: usize = 0xd3e0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 876;
pub const DATA_BYTES: usize = 52;
const PROTECTED_DATA: [(usize, u32); 13] = [
    (0xd24c, 0x000e37d7),
    (0xd29c, 0x0000d2a8),
    (0xd2a0, 0x0000d2c4),
    (0xd2a4, 0x000e37d7),
    (0xd2c0, 0x00103bfc),
    (0xd2d8, 0x00103bfc),
    (0xd328, 0x0000d334),
    (0xd32c, 0x0000d338),
    (0xd330, 0x000e37d7),
    (0xd388, 0x0000a99c),
    (0xd38c, 0x0000a9d4),
    (0xd390, 0x000e37d7),
    (0xd3dc, 0x000e37d7),
];

fn r(number: u8) -> R {
    R::new(number).expect("A32 register")
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("heap statistics callbacks at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !(START..END).contains(&at) || at % 4 != 0
            || PROTECTED_DATA.iter().any(|&(address, _)| address == at)
        {
            return Err(format!("heap statistics instruction outside code: {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !PROTECTED_DATA.contains(&(at, value)) {
            return Err(format!("heap statistics invalid literal at {at:#x}: {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn collect_heap_statistics(c: &mut Code) -> Result<(), String> {
    c.emit(0xd040, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xd044, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x10)?)?;
    c.emit(0xd048, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x10)?)?;
    c.emit(0xd04c, data_register(C::Al, Op::Mov, r(4), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xd050, data_register(C::Al, Op::Mov, r(5), R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xd054, data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x0)?)?;
    c.emit(0xd058, branch(C::Ne, 0xd058, 0xd074, false)?)?;
    c.emit(0xd05c, branch(C::Al, 0xd05c, 0x86644, true)?)?;
    c.emit(0xd060, transfer_word(C::Al, true, R::R2, R::PC, 0xd24c - 0xd060 - 8)?)?;
    c.emit(0xd064, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x52)?)?;
    c.emit(0xd068, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd06c, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd070, branch(C::Al, 0xd070, 0x17b0, true)?)?;
    c.emit(0xd074, data_immediate(C::Al, Op::Cmp, R::R0, r(5), 0x0)?)?;
    c.emit(0xd078, branch(C::Ne, 0xd078, 0xd094, false)?)?;
    c.emit(0xd07c, branch(C::Al, 0xd07c, 0x86644, true)?)?;
    c.emit(0xd080, transfer_word(C::Al, true, R::R2, R::PC, 0xd24c - 0xd080 - 8)?)?;
    c.emit(0xd084, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x53)?)?;
    c.emit(0xd088, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd08c, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd090, branch(C::Al, 0xd090, 0x17b0, true)?)?;
    c.emit(0xd094, transfer_word(C::Al, true, R::R1, r(4), 36)?)?;
    c.emit(0xd098, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(0xd09c, branch(C::Eq, 0xd09c, 0xd0a8, false)?)?;
    c.emit(0xd0a0, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd0a4, branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(0xd0a8, data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xd0ac, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(0xd0b0, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x28)?)?;
    c.emit(0xd0b4, branch(C::Al, 0xd0b4, 0x31ec, true)?)?;
    c.emit(0xd0b8, data_immediate(C::Al, Op::Add, r(6), R::SP, 0x8)?)?;
    c.emit(0xd0bc, data_register(C::Al, Op::Mov, R::R1, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd0c0, data_register(C::Al, Op::Mov, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xd0c4, branch(C::Al, 0xd0c4, 0xce84, true)?)?;
    c.emit(0xd0c8, transfer_word(C::Al, true, R::R0, r(4), 24)?)?;
    c.emit(0xd0cc, data_register(C::Al, Op::Mov, R::R1, R::R0, R::SP, Shift::Lsl, 0, false)?)?;
    c.emit(0xd0d0, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.emit(0xd0d4, transfer_word(C::Al, false, R::R0, r(5), 32)?)?;
    c.emit(0xd0d8, transfer_word(C::Al, true, R::R0, r(4), 28)?)?;
    c.emit(0xd0dc, transfer_word(C::Al, false, R::R0, r(5), 36)?)?;
    c.emit(0xd0e0, data_register(C::Al, Op::Mov, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xd0e4, branch(C::Al, 0xd0e4, 0xcf24, true)?)?;
    c.emit(0xd0e8, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd0ec, branch(C::Eq, 0xd0ec, 0xd180, false)?)?;
    c.emit(0xd0f0, data_register(C::Al, Op::Mov, r(6), R::R0, R::SP, Shift::Lsl, 0, false)?)?;
    c.emit(0xd0f4, data_immediate(C::Al, Op::Add, r(7), R::SP, 0x8)?)?;
    c.emit(0xd0f8, data_register(C::Al, Op::Mov, R::R0, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xd0fc, branch(C::Al, 0xd0fc, 0xcdc0, true)?)?;
    c.emit(0xd100, transfer_word(C::Al, true, R::R1, r(5), 4)?)?;
    c.emit(0xd104, data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1)?)?;
    c.emit(0xd108, transfer_word(C::Al, false, R::R1, r(5), 4)?)?;
    c.emit(0xd10c, transfer_word(C::Al, true, R::R1, r(5), 12)?)?;
    c.emit(0xd110, data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x8)?)?;
    c.emit(0xd114, transfer_word(C::Al, false, R::R1, r(5), 12)?)?;
    c.emit(0xd118, transfer_byte(C::Al, true, R::R1, R::SP, 3)?)?;
    c.emit(0xd11c, data_immediate(C::Al, Op::Tst, R::R0, R::R1, 0xf)?)?;
    c.emit(0xd120, branch(C::Eq, 0xd120, 0xd140, false)?)?;
    c.emit(0xd124, transfer_word(C::Al, true, R::R1, r(5), 20)?)?;
    c.emit(0xd128, data_register(C::Al, Op::Add, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xd12c, transfer_word(C::Al, false, R::R1, r(5), 20)?)?;
    c.emit(0xd130, transfer_word(C::Al, true, R::R1, r(5), 28)?)?;
    c.emit(0xd134, data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xd138, transfer_word(C::Hi, false, R::R0, r(5), 28)?)?;
    c.emit(0xd13c, branch(C::Al, 0xd13c, 0xd168, false)?)?;
    c.emit(0xd140, transfer_word(C::Al, true, R::R1, r(5), 8)?)?;
    c.emit(0xd144, data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1)?)?;
    c.emit(0xd148, transfer_word(C::Al, false, R::R1, r(5), 8)?)?;
    c.emit(0xd14c, transfer_word(C::Al, true, R::R1, r(5), 16)?)?;
    c.emit(0xd150, data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xd154, transfer_word(C::Al, false, R::R0, r(5), 16)?)?;
    c.emit(0xd158, transfer_word(C::Al, true, R::R0, r(5), 24)?)?;
    c.emit(0xd15c, transfer_byte(C::Al, true, R::R1, R::SP, 2)?)?;
    c.emit(0xd160, data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xd164, transfer_word(C::Al, false, R::R0, r(5), 24)?)?;
    c.emit(0xd168, data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0xd16c, data_register(C::Al, Op::Mov, R::R1, R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0xd170, data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.emit(0xd174, branch(C::Al, 0xd174, 0xcf24, true)?)?;
    c.emit(0xd178, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd17c, branch(C::Ne, 0xd17c, 0xd0f8, false)?)?;
    c.emit(0xd180, data_immediate(C::Al, Op::Add, r(3), r(5), 0xc)?)?;
    c.emit(0xd184, transfer_multiple(C::Al, true, r(3), R::R0.bit() | R::R1.bit() | R::R2.bit() | r(3).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0xd188, data_register(C::Al, Op::Add, R::R2, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(0xd18c, data_register(C::Al, Op::Add, R::R2, R::R2, r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0xd190, data_register(C::Al, Op::Add, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xd194, transfer_word(C::Al, false, R::R0, r(5), 0)?)?;
    c.emit(0xd198, transfer_word(C::Al, true, R::R0, r(4), 20)?)?;
    c.emit(0xd19c, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xd1a0, branch(C::Eq, 0xd1a0, 0xd1bc, false)?)?;
    c.emit(0xd1a4, branch(C::Al, 0xd1a4, 0x86644, true)?)?;
    c.emit(0xd1a8, transfer_word(C::Al, true, R::R2, R::PC, 0xd24c - 0xd1a8 - 8)?)?;
    c.emit(0xd1ac, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x77)?)?;
    c.emit(0xd1b0, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd1b4, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd1b8, branch(C::Al, 0xd1b8, 0x17b0, true)?)?;
    c.emit(0xd1bc, transfer_word(C::Al, true, R::R0, r(5), 4)?)?;
    c.emit(0xd1c0, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd1c4, branch(C::Ne, 0xd1c4, 0xd1e4, false)?)?;
    c.emit(0xd1c8, branch(C::Al, 0xd1c8, 0x86644, true)?)?;
    c.emit(0xd1cc, transfer_word(C::Al, true, R::R2, R::PC, 0xd24c - 0xd1cc - 8)?)?;
    c.emit(0xd1d0, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x278)?)?;
    c.emit(0xd1d4, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd1d8, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    c.emit(0xd1dc, branch(C::Al, 0xd1dc, 0x17b0, true)?)?;
    c.emit(0xd1e0, transfer_word(C::Al, true, R::R0, r(5), 4)?)?;
    c.emit(0xd1e4, transfer_word(C::Al, true, R::R1, r(4), 12)?)?;
    c.emit(0xd1e8, data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(0xd1ec, branch(C::Ls, 0xd1ec, 0xd208, false)?)?;
    c.emit(0xd1f0, branch(C::Al, 0xd1f0, 0x86644, true)?)?;
    c.emit(0xd1f4, transfer_word(C::Al, true, R::R2, R::PC, 0xd24c - 0xd1f4 - 8)?)?;
    c.emit(0xd1f8, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x79)?)?;
    c.emit(0xd1fc, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd200, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd204, branch(C::Al, 0xd204, 0x17b0, true)?)?;
    c.emit(0xd208, transfer_word(C::Al, true, R::R0, r(4), 16)?)?;
    c.emit(0xd20c, transfer_word(C::Al, true, R::R1, r(5), 0)?)?;
    c.emit(0xd210, data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(0xd214, branch(C::Eq, 0xd214, 0xd230, false)?)?;
    c.emit(0xd218, branch(C::Al, 0xd218, 0x86644, true)?)?;
    c.emit(0xd21c, transfer_word(C::Al, true, R::R2, R::PC, 0xd24c - 0xd21c - 8)?)?;
    c.emit(0xd220, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7a)?)?;
    c.emit(0xd224, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd228, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd22c, branch(C::Al, 0xd22c, 0x17b0, true)?)?;
    c.emit(0xd230, transfer_word(C::Al, true, R::R1, r(4), 40)?)?;
    c.emit(0xd234, data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(0xd238, branch(C::Eq, 0xd238, 0xd244, false)?)?;
    c.emit(0xd23c, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd240, branch_exchange_with_link(C::Al, R::R1)?)?;
    c.emit(0xd244, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x10)?)?;
    c.emit(0xd248, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd24c, 0x000e37d7)?;
    Ok(())
}

fn install_statistics_callbacks(c: &mut Code) -> Result<(), String> {
    c.emit(0xd250, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xd254, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x8)?)?;
    c.emit(0xd258, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd25c, branch(C::Eq, 0xd25c, 0xd278, false)?)?;
    c.emit(0xd260, transfer_word(C::Al, true, R::R1, R::PC, 0xd29c - 0xd260 - 8)?)?;
    c.emit(0xd264, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x1)?)?;
    c.emit(0xd268, transfer_word(C::Al, false, R::R1, R::R0, 36)?)?;
    c.emit(0xd26c, transfer_word(C::Al, true, R::R1, R::PC, 0xd2a0 - 0xd26c - 8)?)?;
    c.emit(0xd270, transfer_word(C::Al, false, R::R1, R::R0, 40)?)?;
    c.emit(0xd274, branch(C::Al, 0xd274, 0xd294, false)?)?;
    c.emit(0xd278, branch(C::Al, 0xd278, 0x86644, true)?)?;
    c.emit(0xd27c, transfer_word(C::Al, true, R::R2, R::PC, 0xd2a4 - 0xd27c - 8)?)?;
    c.emit(0xd280, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x93)?)?;
    c.emit(0xd284, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd288, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0xd28c, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd290, branch(C::Al, 0xd290, 0x17b0, true)?)?;
    c.emit(0xd294, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd298, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd29c, 0x0000d2a8)?;
    c.data(0xd2a0, 0x0000d2c4)?;
    c.data(0xd2a4, 0x000e37d7)?;
    Ok(())
}

fn capture_callback_state(c: &mut Code) -> Result<(), String> {
    c.emit(0xd2a8, transfer_multiple(C::Al, false, R::SP, r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xd2ac, data_register(C::Al, Op::Mov, r(11), R::R0, R::SP, Shift::Lsl, 0, false)?)?;
    c.emit(0xd2b0, branch_exchange_immediate(0xd2b0, 0xe2198)?)?;
    c.emit(0xd2b4, transfer_word(C::Al, true, R::R1, R::PC, 0xd2c0 - 0xd2b4 - 8)?)?;
    c.emit(0xd2b8, transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    c.emit(0xd2bc, transfer_multiple(C::Al, true, R::SP, r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd2c0, 0x00103bfc)?;
    Ok(())
}

fn conditional_callback_tail(c: &mut Code) -> Result<(), String> {
    c.emit(0xd2c4, transfer_word(C::Al, true, R::R0, R::PC, 0xd2d8 - 0xd2c4 - 8)?)?;
    c.emit(0xd2c8, transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    c.emit(0xd2cc, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd2d0, bx(C::Ne, R::LR))?;
    c.emit(0xd2d4, branch(C::Al, 0xd2d4, 0x1029c4, false)?)?;
    c.data(0xd2d8, 0x00103bfc)?;
    Ok(())
}

fn install_noop_callbacks(c: &mut Code) -> Result<(), String> {
    c.emit(0xd2dc, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xd2e0, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x8)?)?;
    c.emit(0xd2e4, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd2e8, branch(C::Eq, 0xd2e8, 0xd304, false)?)?;
    c.emit(0xd2ec, transfer_word(C::Al, true, R::R1, R::PC, 0xd328 - 0xd2ec - 8)?)?;
    c.emit(0xd2f0, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x1)?)?;
    c.emit(0xd2f4, transfer_word(C::Al, false, R::R1, R::R0, 36)?)?;
    c.emit(0xd2f8, transfer_word(C::Al, true, R::R1, R::PC, 0xd32c - 0xd2f8 - 8)?)?;
    c.emit(0xd2fc, transfer_word(C::Al, false, R::R1, R::R0, 40)?)?;
    c.emit(0xd300, branch(C::Al, 0xd300, 0xd320, false)?)?;
    c.emit(0xd304, branch(C::Al, 0xd304, 0x86644, true)?)?;
    c.emit(0xd308, transfer_word(C::Al, true, R::R2, R::PC, 0xd330 - 0xd308 - 8)?)?;
    c.emit(0xd30c, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xaf)?)?;
    c.emit(0xd310, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd314, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0xd318, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd31c, branch(C::Al, 0xd31c, 0x17b0, true)?)?;
    c.emit(0xd320, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd324, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd328, 0x0000d334)?;
    c.data(0xd32c, 0x0000d338)?;
    c.data(0xd330, 0x000e37d7)?;
    Ok(())
}

fn noop_callbacks(c: &mut Code) -> Result<(), String> {
    c.emit(0xd334, bx(C::Al, R::LR))?;
    c.emit(0xd338, bx(C::Al, R::LR))?;
    Ok(())
}

fn install_heap_callbacks(c: &mut Code) -> Result<(), String> {
    c.emit(0xd33c, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xd340, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x8)?)?;
    c.emit(0xd344, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd348, branch(C::Eq, 0xd348, 0xd364, false)?)?;
    c.emit(0xd34c, transfer_word(C::Al, true, R::R1, R::PC, 0xd388 - 0xd34c - 8)?)?;
    c.emit(0xd350, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x1)?)?;
    c.emit(0xd354, transfer_word(C::Al, false, R::R1, R::R0, 36)?)?;
    c.emit(0xd358, transfer_word(C::Al, true, R::R1, R::PC, 0xd38c - 0xd358 - 8)?)?;
    c.emit(0xd35c, transfer_word(C::Al, false, R::R1, R::R0, 40)?)?;
    c.emit(0xd360, branch(C::Al, 0xd360, 0xd380, false)?)?;
    c.emit(0xd364, branch(C::Al, 0xd364, 0x86644, true)?)?;
    c.emit(0xd368, transfer_word(C::Al, true, R::R2, R::PC, 0xd390 - 0xd368 - 8)?)?;
    c.emit(0xd36c, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xcb)?)?;
    c.emit(0xd370, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd374, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0xd378, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    c.emit(0xd37c, branch(C::Al, 0xd37c, 0x17b0, true)?)?;
    c.emit(0xd380, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd384, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd388, 0x0000a99c)?;
    c.data(0xd38c, 0x0000a9d4)?;
    c.data(0xd390, 0x000e37d7)?;
    Ok(())
}

fn clear_callbacks(c: &mut Code) -> Result<(), String> {
    c.emit(0xd394, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xd398, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x8)?)?;
    c.emit(0xd39c, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd3a0, branch(C::Eq, 0xd3a0, 0xd3b8, false)?)?;
    c.emit(0xd3a4, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(0xd3a8, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x1)?)?;
    c.emit(0xd3ac, transfer_word(C::Al, false, R::R1, R::R0, 36)?)?;
    c.emit(0xd3b0, transfer_word(C::Al, false, R::R1, R::R0, 40)?)?;
    c.emit(0xd3b4, branch(C::Al, 0xd3b4, 0xd3d4, false)?)?;
    c.emit(0xd3b8, branch(C::Al, 0xd3b8, 0x86644, true)?)?;
    c.emit(0xd3bc, transfer_word(C::Al, true, R::R2, R::PC, 0xd3dc - 0xd3bc - 8)?)?;
    c.emit(0xd3c0, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2e4)?)?;
    c.emit(0xd3c4, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(0xd3c8, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(0xd3cc, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    c.emit(0xd3d0, branch(C::Al, 0xd3d0, 0x17b0, true)?)?;
    c.emit(0xd3d4, data_register(C::Al, Op::Mov, R::R0, R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xd3d8, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd3dc, 0x000e37d7)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    collect_heap_statistics(&mut c)?;
    install_statistics_callbacks(&mut c)?;
    capture_callback_state(&mut c)?;
    conditional_callback_tail(&mut c)?;
    install_noop_callbacks(&mut c)?;
    noop_callbacks(&mut c)?;
    install_heap_callbacks(&mut c)?;
    clear_callbacks(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("heap statistics callbacks extent mismatch".into());
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
    fn exact_extent_and_literal_guards() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 928);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(START + FILE_MAPPING_DELTA, 0x11040);
        assert_eq!(END + FILE_MAPPING_DELTA, 0x113e0);
        assert_eq!(PROTECTED_DATA.len() * 4, DATA_BYTES);
        for (at, value) in PROTECTED_DATA {
            assert_eq!(word(&bytes, at), value);
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.data(at, value ^ 1).is_err());
            assert!(c.data(at + 1, value).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes, value.to_le_bytes());
        }
    }

    #[test]
    fn rejects_invalid_emission_positions() {
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.data(at, 0xe37d7).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.data(START, 0xe37d7).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn statistics_install_clear_and_callback_control_flow() {
        let bytes = fragment().unwrap();
        for (at, expected) in [
            (0xd0a4, 0xe12fff31),
            (0xd184, 0xe893000f),
            (0xd240, 0xe12fff31),
            (0xd268, 0xe5801024),
            (0xd270, 0xe5801028),
            (0xd2b0, 0xfa0353b8),
            (0xd2b8, 0xe5810000),
            (0xd2d0, 0x112fff1e),
            (0xd2d4, 0xea03d5ba),
            (0xd2f4, 0xe5801024),
            (0xd2fc, 0xe5801028),
            (0xd334, 0xe12fff1e),
            (0xd338, 0xe12fff1e),
            (0xd354, 0xe5801024),
            (0xd35c, 0xe5801028),
            (0xd3ac, 0xe5801024),
            (0xd3b0, 0xe5801028),
        ] {
            assert_eq!(word(&bytes, at), expected, "instruction at {at:#x}");
        }
        assert_eq!(word(&bytes, 0xd2b0), branch_exchange_immediate(0xd2b0, 0xe2198).unwrap());
        assert_eq!(word(&bytes, 0xd2d4), branch(C::Al, 0xd2d4, 0x1029c4, false).unwrap());
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
