use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags,
    data_register, data_register_shifted_by_register, multiply_register,
    transfer_byte, transfer_multiple, transfer_word, transfer_word_postindexed,
    transfer_word_register, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xe3f8;
pub const END: usize = 0xee38;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2552;
pub const DATA_BYTES: usize = 72;
pub const POOL_START: usize = 0xee2c;
const REGISTER_DEFAULTS: u32 = 0xe43b4;
const SERVICE_SOURCE: u32 = 0xe3d16;
const REGISTER_TABLE: u32 = 0xe43c4;
const THUMB_SCALE: u32 = 0x83c4c;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("service register address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err("invalid service register output address".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn dispatch_value(at: usize) -> Option<u32> {
        match at {
            0xe4f4 => Some(0xe514),
            0xe4f8 => Some(0xe730),
            0xe4fc => Some(0xe610),
            0xe500 => Some(0xe754),
            0xe504 => Some(0xe610),
            0xe508 => Some(0xe610),
            0xe50c => Some(0xe610),
            0xe510 => Some(0xe7dc),
            0xe6fc => Some(0xe8d8),
            0xe700 => Some(0xe93c),
            0xe704 => Some(0xe9a0),
            0xe708 => Some(0xe718),
            0xe70c => Some(0xe718),
            0xe710 => Some(0xe718),
            0xe714 => Some(0xe718),
            _ => None,
        }
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if self.at >= POOL_START || Self::dispatch_value(self.at).is_some() {
            return Err("instruction overlaps protected service register DATA".into());
        }
        self.word(instruction)
    }

    fn dispatch_target(&mut self, at: usize, target: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::dispatch_value(at) != Some(target) {
            return Err("incorrect service register dispatch target".into());
        }
        self.word(target)
    }

    fn literal_value(at: usize) -> Option<u32> {
        match at {
            0xee2c => Some(REGISTER_DEFAULTS),
            0xee30 => Some(SERVICE_SOURCE),
            0xee34 => Some(REGISTER_TABLE),
            _ => None,
        }
    }

    fn literal_data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::literal_value(at) != Some(value) {
            return Err("incorrect protected service register literal".into());
        }
        self.word(value)
    }

    fn literal(&mut self, condition: C, destination: R, at: usize) -> Result<(), String> {
        if Self::literal_value(at).is_none() {
            return Err("invalid service register literal address".into());
        }
        self.emit(transfer_word(condition, true, destination, R::PC,
            at as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        let at = target as usize;
        if at % 4 != 0 || ((START..END).contains(&at)
            && (at >= POOL_START || Self::dispatch_value(at).is_some())) {
            return Err("invalid service register branch target".into());
        }
        self.emit(branch(condition, self.at as u32, target, link)?)
    }

    fn thumb_call(&mut self, target: u32) -> Result<(), String> {
        if target != THUMB_SCALE || !matches!(self.at, 0xeaec | 0xeb34 | 0xeb98 | 0xebd4) {
            return Err("invalid service register interworking call".into());
        }
        self.emit(branch_exchange_immediate(self.at as u32, target)?)
    }
}

fn select_register_sources(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xe3f8)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x24)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), R::R0, R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Ne, 0xe44c, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 12)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), 68)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, r(8), R::R1, Shift::Lsl, 2)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R1, R::R2, Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x2cc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), R::R1, 0x2a4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), R::R1, 0x3bc)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x394)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), R::R1, 0x1000)?)?;
    c.branch(C::Al, 0xe45c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(4), 0x5c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(4), 0x58)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, r(4), 0x1e8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(4), 0x170)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(6), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), R::R2, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(3), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Eq, 0xe4a0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Ne, 0xe4dc, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2312)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Ne, 0xe548, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Al, 0xe544, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(8), R::R0, Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Ne, 0xe54c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 820)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, R::R1, R::R0, 0x3, true)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R2, R::R0, Shift::Lsr, 1, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2312)?)?;
    c.branch(C::Al, 0xe548, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R0, R::R1, 0x4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x7)?)?;
    c.branch(C::Hi, 0xe610, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::PC, 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::PC, R::R0, R::R2, Shift::Lsl, 0)?)?;
    c.dispatch_target(0xe4f4, 0xe514)?;
    c.dispatch_target(0xe4f8, 0xe730)?;
    c.dispatch_target(0xe4fc, 0xe610)?;
    c.dispatch_target(0xe500, 0xe754)?;
    c.dispatch_target(0xe504, 0xe610)?;
    c.dispatch_target(0xe508, 0xe610)?;
    c.dispatch_target(0xe50c, 0xe610)?;
    c.dispatch_target(0xe510, 0xe7dc)?;
    Ok(())
}

fn configure_output_modes(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xe514)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(8), R::R0, Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(4), 2312)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.branch(C::Ne, 0xed2c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 820)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, R::R0, R::R0, 0x3, true)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Ne, 0xe60c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(4), 0x114)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, false, r(5), R::R0, 980)?)?;
    c.emit(transfer_word(C::Al, false, r(5), R::R0, 984)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Eq, 0xe5cc, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Eq, 0xe5b0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2344)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R0, 980)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R0, 984)?)?;
    c.branch(C::Al, 0xe5cc, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2344)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 68)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(8), R::R0, Shift::Lsl, 2)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2cc)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(7), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3bc)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(9), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2a4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(5), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x394)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(10), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 16)?)?;
    Ok(())
}

fn configure_second_mode(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xe610)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 20)?)?;
    c.branch(C::Ne, 0xe6cc, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(4), 0x114)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x18)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1f0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2312)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(4), 2316)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(8), R::R0, Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.branch(C::Hi, 0xe6d8, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.branch(C::Ne, 0xe6d8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x218)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(12), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x214)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(5), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2344)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R1, 820)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x3dc)?)?;
    c.literal(C::Al, R::R1, 0xee2c)?;
    c.emit(transfer_multiple(C::Al, true, R::R1, R::R2.bit() | r(3).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R0, R::R2.bit() | r(3).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(12), R::SP, 12)?)?;
    c.emit(transfer_word(C::Al, false, r(12), R::SP, 8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 20)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 16)?)?;
    c.branch(C::Al, 0xea0c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x8)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R0, r(7).bit() | r(9).bit() | r(10).bit(), BlockMode::IncrementAfter, false)?)?;
    c.branch(C::Al, 0xea0c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x8)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R1, r(7).bit() | r(9).bit() | r(10).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x6)?)?;
    c.branch(C::Hi, 0xea0c, false)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, R::PC, 0x0)?)?;
    c.emit(transfer_word_register(C::Al, true, R::PC, R::R1, R::R2, Shift::Lsl, 0)?)?;
    c.dispatch_target(0xe6fc, 0xe8d8)?;
    c.dispatch_target(0xe700, 0xe93c)?;
    c.dispatch_target(0xe704, 0xe9a0)?;
    c.dispatch_target(0xe708, 0xe718)?;
    c.dispatch_target(0xe70c, 0xe718)?;
    c.dispatch_target(0xe710, 0xe718)?;
    c.dispatch_target(0xe714, 0xe718)?;
    Ok(())
}

fn configure_task_variants(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xe718)?;
    c.literal(C::Al, R::R2, 0xee30)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xa9)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0xea0c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 820)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R0, R::R0, 0x3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2312)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2316)?)?;
    c.branch(C::Al, 0xe54c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Eq, 0xe54c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xec)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(8), R::R0, Shift::Lsl, 2)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.branch(C::Hi, 0xe54c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x114)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(9), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x110)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(10), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2344)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2312)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 2648)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R1, 820)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x3dc)?)?;
    c.literal(C::Al, R::R1, 0xee2c)?;
    c.emit(transfer_multiple(C::Al, true, R::R1, R::R2.bit() | r(3).bit() | r(5).bit() | r(6).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, R::R0, R::R2.bit() | r(3).bit() | r(5).bit() | r(6).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0xe54c, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 12)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5b0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, r(8), R::R0, Shift::Lsl, 2)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, R::R0, R::R1, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word_register(C::Al, true, r(7), R::R1, R::R2, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x138)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1400)?)?;
    c.emit(transfer_word_register(C::Al, true, r(5), R::R1, R::R2, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1880)?)?;
    c.emit(transfer_word_register(C::Al, true, r(9), R::R1, R::R2, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1800)?)?;
    c.emit(transfer_word_register(C::Al, true, r(10), R::R1, R::R2, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2344)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), 76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    c.branch(C::Eq, 0xed3c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2312)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, r(5), r(5), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Add, r(12), r(7), r(7), Shift::Lsl, 1, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R0, 820)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r(10), r(10), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(3), R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Hi, false, r(3), r(4), 2324)?)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(9), r(9), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), R::R0, r(3), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(6), r(12), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(3), R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Hi, false, r(3), r(4), 2328)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 4, false)?)?;
    c.branch(C::Ls, 0xe8ac, false)?;
    c.literal(C::Al, R::R2, 0xee30)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xcd)?)?;
    c.emit(transfer_word(C::Al, false, r(12), R::SP, 32)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, true, r(12), R::SP, 32)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::R0, 832)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R0, 832)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(6), r(12), Shift::Lsl, 4, false)?)?;
    c.branch(C::Ls, 0xeda0, false)?;
    c.literal(C::Al, R::R2, 0xee30)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xd2)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R1, 832)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x2)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R1, 832)?)?;
    c.branch(C::Al, 0xeda4, false)?;
    Ok(())
}

fn build_scaled_fields(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xe8d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x154)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::SP, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x148)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::SP, 20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x218)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x214)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2344)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    c.emit(multiply_register(C::Al, R::R0, R::R2, R::R1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::SP, 16)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsr, 8, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2448)?)?;
    c.branch(C::Al, 0xe994, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2344)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x194)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(3), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1a0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 16)?)?;
    c.emit(multiply_register(C::Al, R::R1, R::R2, r(3))?)?;
    c.emit(transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x158)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsr, 8, false)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R2, R::R0, R::R2, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, R::SP, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x14c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R0, R::R2, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::SP, 20)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2448)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2316)?)?;
    c.branch(C::Al, 0xea0c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x15c)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2344)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x198)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(7), R::R0, R::R2, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x150)?)?;
    c.emit(transfer_word_register(C::Al, true, r(6), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1a4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R2, R::R0, R::R2, Shift::Lsl, 0)?)?;
    c.emit(transfer_word_register(C::Al, true, r(3), R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(multiply_register(C::Al, R::R1, r(3), r(6))?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R1, Shift::Lsr, 8, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2448)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 728)?)?;
    c.emit(transfer_word(C::Al, false, r(3), R::SP, 12)?)?;
    c.emit(transfer_word(C::Al, false, r(7), R::SP, 8)?)?;
    c.emit(transfer_word(C::Al, false, r(6), R::SP, 16)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, R::SP, 20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x2)?)?;
    c.emit(transfer_word(C::Eq, false, R::R0, r(4), 2312)?)?;
    Ok(())
}

fn pack_register_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xea0c)?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 2648)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(4), 0x920)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(6), r(7), R::R0, Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(8), 824)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(6), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::SP, 0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(8), 828)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(4), 2312)?)?;
    c.emit(transfer_word(C::Al, true, R::LR, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), 2332)?)?;
    c.emit(transfer_multiple(C::Al, true, r(10), R::R1.bit() | r(9).bit() | r(10).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), R::SP, 28)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(6), R::SP, 24)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8c00)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(6), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x22c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(6), r(7), R::R0, Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(6), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(7), R::SP, 32)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(7), 0)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R2, R::R1, Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, r(9), r(10), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, r(3), r(5), Shift::Lsl, 1, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, r(12), Shift::Lsl, 11, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 8, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::SP, 0)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::LR, Shift::Lsl, 10, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 6, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x30)?)?;
    c.emit(data_register(C::Al, Op::And, R::R1, R::R1, R::R2, Shift::Lsl, 4, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(6), 0)?)?;
    Ok(())
}

fn scale_output_dimensions(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xeabc)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2332)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 20)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::SP, 16)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(7), R::R0, 0x12)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(8), 820)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), R::R0, R::R2, Shift::Lsr, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Eq, true, R::R0, r(4), 2324)?)?;
    c.emit(data_register(C::Eq, Op::Sub, R::R0, r(7), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, R::R0, r(5), R::R1, Shift::Lsl, R::R0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    c.thumb_call(0x83c4c)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 2340)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 48)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x40000)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.branch(C::Ne, 0xeb3c, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 56)?)?;
    c.emit(transfer_word(C::Al, true, r(9), R::SP, 12)?)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Eq, 0xeb44, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Ne, 0xeb44, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::SP, 20)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, R::R0, r(5), R::R0, Shift::Lsl, r(7), false)?)?;
    c.thumb_call(0x83c4c)?;
    c.branch(C::Al, 0xeb44, false)?;
    c.emit(transfer_word(C::Al, true, r(9), R::SP, 12)?)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 8)?)?;
    c.emit(transfer_word(C::Al, true, r(3), R::SP, 24)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 28)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), R::R0, r(10), Shift::Lsr, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, r(3), 0x8)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R2, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xcb0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8000)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(3), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R1, r(6), 0x8)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, R::SP, 32)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R2, 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(6), 4)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2336)?)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(7), R::R0, 0x12)?)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(8), 820)?)?;
    c.emit(data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x2)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Eq, true, R::R0, r(4), 2328)?)?;
    c.emit(data_register(C::Eq, Op::Sub, R::R0, r(7), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, R::R0, r(5), r(9), Shift::Lsl, R::R0, false)?)?;
    c.thumb_call(0x83c4c)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 2344)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 48)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x40000)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    c.branch(C::Ne, 0xebd8, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Eq, 0xebd8, false)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 76)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.branch(C::Ne, 0xebd8, false)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, R::R0, r(5), r(9), Shift::Lsl, r(7), false)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R1, R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.thumb_call(0x83c4c)?;
    Ok(())
}

fn append_register_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xebd8)?;
    c.emit(transfer_word(C::Al, true, r(3), R::SP, 24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(3), 8)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(6), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, r(3), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(10), R::SP, 28)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(8), 988)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(8), 992)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xb8)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(3), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, r(6), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(3), R::SP, 32)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(3), 0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(6), 12)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(8), 980)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(8), 984)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x7)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    c.branch(C::Ne, 0xec48, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(6), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(9), R::R0, 0x0)?)?;
    Ok(())
}

fn append_register_tables(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xec48)?;
    c.literal(C::Al, R::R2, 0xee30)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x9a)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(6), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::SP, 32)?)?;
    c.literal(C::Al, r(8), 0xee34)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), R::R0, 0)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r(8), r(6), Shift::Lsl, 5, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), R::R0, 0xc50)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, r(3), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x8000)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(6), r(3), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x4)?)?;
    c.emit(transfer_word_postindexed(C::Al, true, r(5), R::R1, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x8)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(5), R::R0, 4)?)?;
    c.branch(C::Ne, 0xec8c, false)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(10), 0)?)?;
    c.literal(C::Al, R::R2, 0xee30)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(7), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x9e)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(9), 0x8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, true, r(5), R::SP, 32)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(5), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), R::R0, 0xc70)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R1, r(8), r(9), Shift::Lsl, 5, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x8000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R2, r(3), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R0, R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(6), r(3), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x4)?)?;
    c.emit(transfer_word_postindexed(C::Al, true, r(5), R::R1, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0x8)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(5), R::R0, 4)?)?;
    c.branch(C::Ne, 0xecf4, false)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, R::R0, r(7), 0x20)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, R::SP, 32)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn cold_output_modes(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xed2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.branch(C::Al, 0xe548, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x2dc)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2312)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1800)?)?;
    c.emit(transfer_word(C::Al, true, R::R1, r(4), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R2, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.emit(transfer_word(C::Al, false, R::R2, r(4), 2316)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), 72)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R0, R::R0, R::R2, Shift::Lsl, 2, false)?)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R0, r(3), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    c.branch(C::Eq, 0xee10, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::R0, R::R1, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0xedb8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2340)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2344)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2312)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2316)?)?;
    c.branch(C::Al, 0xedb8, false)?;
    c.emit(transfer_word(C::Al, true, R::R0, R::R0, 832)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::And, R::R1, R::R0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::And, R::R0, R::R2, R::R0, Shift::Lsr, 1, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, r(4), 2332)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2336)?)?;
    c.emit(data_register(C::Al, Op::Add, R::R2, r(5), r(5), Shift::Lsl, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(5), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(6), r(7), r(7), Shift::Lsl, 2, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, R::R2, R::R0, R::R2, Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, R::R1, R::R0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), R::R0, r(6), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, R::R2, r(10), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R2, R::R0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(7), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, R::R2, r(4), 2648)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(3), R::R0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(6), r(9), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Cc, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_register(C::Al, Op::Orr, R::R0, r(3), R::R0, Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R2, 980)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, R::R2, 984)?)?;
    c.branch(C::Al, 0xe54c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2324)?)?;
    c.emit(transfer_word(C::Al, false, R::R0, r(4), 2328)?)?;
    c.emit(transfer_word(C::Al, true, R::R0, r(4), 2648)?)?;
    c.emit(transfer_word(C::Al, false, R::R1, R::R0, 820)?)?;
    c.branch(C::Al, 0xedb8, false)?;
    Ok(())
}
pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    select_register_sources(&mut c)?;
    configure_output_modes(&mut c)?;
    configure_second_mode(&mut c)?;
    configure_task_variants(&mut c)?;
    build_scaled_fields(&mut c)?;
    pack_register_header(&mut c)?;
    scale_output_dimensions(&mut c)?;
    append_register_records(&mut c)?;
    append_register_tables(&mut c)?;
    cold_output_modes(&mut c)?;
    c.literal_data(0xee2c, REGISTER_DEFAULTS)?;
    c.literal_data(0xee30, SERVICE_SOURCE)?;
    c.literal_data(0xee34, REGISTER_TABLE)?;
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
    fn extent_literals_and_dispatch_tables() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2624);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        let data_bytes = (START..END).step_by(4).filter(|&at|
            TypedA32::literal_value(at).or(TypedA32::dispatch_value(at)).is_some()).count() * 4;
        assert_eq!(data_bytes, DATA_BYTES);
        assert_eq!(END - START - data_bytes, CODE_BYTES);
        for at in (START..END).step_by(4) {
            if let Some(value) = TypedA32::literal_value(at).or(TypedA32::dispatch_value(at)) {
                assert_eq!(word(&bytes, at), value);
            }
        }
        assert_eq!(word(&bytes, 0xed28), transfer_multiple(C::Al, true, R::SP,
            (4..=11).fold(R::PC.bit(), |mask, n| mask | r(n).bit()),
            BlockMode::IncrementAfter, true).unwrap());
        for (at, condition, target) in [(0xe534, C::Ne, 0xed2c),
            (0xe82c, C::Eq, 0xed3c), (0xe8d4, C::Al, 0xeda4),
            (0xe8b0, C::Ls, 0xeda0), (0xee28, C::Al, 0xedb8),
            (0xeca4, C::Ne, 0xec8c), (0xed0c, C::Ne, 0xecf4)] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        for at in [0xeaec, 0xeb34, 0xeb98, 0xebd4] {
            assert_eq!(word(&bytes, at), branch_exchange_immediate(at as u32, THUMB_SCALE).unwrap());
        }
        assert_eq!(word(&bytes, 0xea20), transfer_word_register_preindexed_writeback(
            C::Al, true, r(6), r(7), R::R0, Shift::Lsl, 0).unwrap());
        for at in [0xec8c, 0xecf4] {
            assert_eq!(word(&bytes, at), transfer_word_postindexed(C::Al, false, r(6), r(3), 4).unwrap());
            assert_eq!(word(&bytes, at + 12), transfer_word_postindexed(C::Al, true, r(5), R::R1, 4).unwrap());
            assert_eq!(word(&bytes, at + 20), transfer_word_postindexed(C::Al, false, r(5), R::R0, 4).unwrap());
        }
    }

    #[test]
    fn protected_data_and_alignment() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), at < POOL_START && TypedA32::dispatch_value(at).is_none());
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal_data(at, 0).is_err());
            assert!(c.dispatch_target(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.literal_data(at, REGISTER_DEFAULTS).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.literal_data(POOL_START, REGISTER_DEFAULTS).is_err());
        assert!(c.literal(C::Al, R::R0, START).is_err());
        assert!(c.literal(C::Al, R::R0, POOL_START + 1).is_err());
        for target in [POOL_START as u32, 0xe4f4, 0xe6fc, START as u32 + 1] {
            assert!(c.branch(C::Al, target, false).is_err());
        }
        assert!(c.thumb_call(THUMB_SCALE).is_err());
        assert!(c.bytes.is_empty());
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
