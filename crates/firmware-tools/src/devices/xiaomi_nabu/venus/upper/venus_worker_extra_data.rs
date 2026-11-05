use arm32::{
    branch, data_immediate, data_register, mov_register, push_registers, transfer_byte,
    transfer_multiple, transfer_word, transfer_word_preindexed_writeback, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x761e0;
pub const END: usize = 0x76610;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const DATA_BYTES: usize = 36;
pub const CODE_BYTES: usize = END - START - DATA_BYTES;
pub const PARSER_START: usize = START;
pub const CONSTRUCTOR_START: usize = 0x76460;

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("worker extra-data address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0x76444 => Some(0xf3ce9),
            0x76448 => Some(0x7f100007),
            0x7644c => Some(0xf3d61),
            0x76450 => Some(0xf3d47),
            0x76454 => Some(0xe2788),
            0x76458 => Some(0x7f100003),
            0x7645c => Some(0xf3d20),
            0x76608 => Some(0xf3ce9),
            0x7660c => Some(0x76610),
            _ => None,
        }
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && ((START..0x76444).contains(&at)
            || (CONSTRUCTOR_START..0x76608).contains(&at))
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("worker extra-data instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) {
            return Err("worker extra-data protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: u8, target: usize) -> Result<(), String> {
        let valid = if self.at < CONSTRUCTOR_START {
            (0x76444..0x76460).contains(&target)
        } else {
            (0x76608..END).contains(&target)
        };
        if !valid || Self::data_value(target).is_none() {
            return Err("worker extra-data invalid literal".into());
        }
        self.load(rd, 15, target as i32 - self.at as i32 - 8)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = if link {
            if self.at < CONSTRUCTOR_START {
                matches!(target, 0x17b0 | 0x2ec94)
            } else {
                matches!(target, 0x17b0 | 0x2efc | 0x31ec | 0x61c78 | 0x67888
                    | 0x522d4 | 0x71214 | 0x35b30)
            }
        } else if self.at == 0x76440 {
            condition == C::Al && target == 0x2038
        } else {
            Self::is_code(target) && (self.at < CONSTRUCTOR_START) == (target < CONSTRUCTOR_START)
        };
        if !valid {
            return Err("worker extra-data invalid branch".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn imm(&mut self, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }
    fn mov(&mut self, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(C::Al, r(rd), r(rm)))
    }
    fn set(&mut self, rd: u8, value: u32) -> Result<(), String> {
        self.imm(C::Al, Op::Mov, rd, 0, value)
    }
    fn cmp(&mut self, rn: u8, value: u32) -> Result<(), String> {
        self.imm(C::Al, Op::Cmp, 0, rn, value)
    }
    fn reg(&mut self, op: Op, rd: u8, rn: u8, rm: u8) -> Result<(), String> {
        self.emit(data_register(C::Al, op, r(rd), r(rn), r(rm), Shift::Lsl, 0, false)?)
    }
    fn load(&mut self, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, true, r(rd), r(rn), offset)?)
    }
    fn store(&mut self, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, false, r(rd), r(rn), offset)?)
    }
    fn byte(&mut self, condition: C, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(condition, load, r(rd), r(rn), offset)?)
    }
    fn pop(&mut self, registers: u16) -> Result<(), String> {
        self.emit(transfer_multiple(C::Al, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }
}

fn parser_entry(c: &mut TypedA32) -> Result<(), String> {
    c.check(START)?;
    c.emit(push_registers(C::Al, 0x4ff0)?)?;
    c.imm(C::Al, Op::Add, 11, 13, 0x1c)?;
    c.imm(C::Al, Op::Sub, 13, 13, 0xc)?;
    c.store(0, 13, 8)?;
    c.literal(5, 0x76444)?;
    c.set(7, 0xd6)?;
    c.set(9, 0xdb)?;
    c.set(10, 0xe)?;
    c.mov(4, 2)?;
    c.mov(6, 1)?;
    c.load(8, 0, 0x38)?;
    c.imm(C::Al, Op::Orr, 7, 7, 0x200)?;
    c.imm(C::Al, Op::Orr, 9, 9, 0x200)?;
    c.imm(C::Al, Op::Orr, 10, 10, 0x7000000)?;
    c.branch(C::Al, 0x76250, false)
}

fn unknown_tag_logging(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7621c)?;
    c.literal(1, 0x7644c)?;
    c.literal(2, 0x76450)?;
    c.store(0, 13, 0)?;
    c.set(0, 8)?;
    c.set(3, 0x330)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(1, 0x76454)?;
    c.set(0, 8)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.set(1, 0x31)?;
    c.set(0, 0)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0x300)?;
    c.branch(C::Al, 0x76380, false)
}

fn parser_dispatch(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x76250)?;
    c.imm(C::Al, Op::Tst, 0, 8, 3)?;
    c.set(0, 0)?;
    c.mov(1, 7)?;
    c.mov(2, 5)?;
    c.imm(C::Eq, Op::Mov, 0, 0, 1)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.load(0, 8, 0xc)?;
    c.load(1, 6, 0x320)?;
    c.reg(Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x762cc, false)?;
    c.load(0, 8, 0x10)?;
    c.mov(1, 9)?;
    c.mov(2, 5)?;
    c.cmp(0, 0)?;
    c.imm(C::Ne, Op::Mov, 0, 0, 1)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.load(0, 8, 0x14)?;
    c.literal(1, 0x76458)?;
    c.reg(Op::Cmp, 0, 0, 1)?;
    c.branch(C::Eq, 0x76320, false)?;
    c.reg(Op::Cmp, 0, 0, 10)?;
    c.branch(C::Ne, 0x76350, false)?;
    for (src, dst) in [(0x24, 0x214), (0x28, 0x218), (0x2c, 0x21c), (0x30, 0x220)] {
        c.load(0, 8, src)?;
        c.store(0, 4, dst)?;
    }
    c.branch(C::Al, 0x763f0, false)?;
    c.check(0x762cc)?;
    c.load(1, 6, 0x314)?;
    c.reg(Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x762f0, false)?;
    c.byte(C::Al, true, 0, 6, 0x199)?;
    c.cmp(0, 1)?;
    c.branch(C::Ne, 0x7638c, false)?;
    c.load(0, 8, 0x14)?;
    c.store(0, 4, 0x238)?;
    c.branch(C::Al, 0x763f0, false)?;
    c.cmp(0, 0x12)?;
    c.branch(C::Le, 0x76398, false)?;
    c.cmp(0, 0x13)?;
    c.branch(C::Eq, 0x763d0, false)?;
    c.cmp(0, 0x1a)?;
    c.branch(C::Eq, 0x763dc, false)?;
    c.literal(1, 0x76448)?;
    c.reg(Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x7621c, false)?;
    c.imm(C::Al, Op::Add, 0, 8, 0x14)?;
    c.store(0, 4, 0x244)?;
    c.branch(C::Al, 0x763f0, false)
}

fn parser_updates_and_recovery(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x76320)?;
    c.byte(C::Al, true, 0, 6, 0x180)?;
    c.cmp(0, 1)?;
    c.branch(C::Ne, 0x763f0, false)?;
    c.load(0, 8, 0x24)?;
    c.load(1, 4, 0x24c)?;
    c.reg(Op::Cmp, 0, 1, 0)?;
    c.branch(C::Ne, 0x763b4, false)?;
    c.load(1, 8, 0x28)?;
    c.load(2, 4, 0x250)?;
    c.reg(Op::Cmp, 0, 2, 1)?;
    c.branch(C::Ne, 0x763b8, false)?;
    c.branch(C::Al, 0x763f0, false)?;
    c.literal(1, 0x7645c)?;
    c.literal(2, 0x76450)?;
    c.set(3, 0xf9)?;
    c.set(0, 8)?;
    c.imm(C::Al, Op::Orr, 3, 3, 0x200)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(1, 0x76454)?;
    c.set(0, 8)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.set(1, 0xfa)?;
    c.set(0, 0)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0x200)?;
    c.check(0x76380)?;
    c.mov(2, 5)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x76250, false)?;
    c.set(0, 1)?;
    c.store(0, 4, 0x238)?;
    c.branch(C::Al, 0x76250, false)?;
    c.cmp(0, 6)?;
    c.branch(C::Eq, 0x763fc, false)?;
    c.cmp(0, 0xf)?;
    c.branch(C::Ne, 0x76424, false)?;
    c.imm(C::Al, Op::Add, 0, 8, 0x14)?;
    c.store(0, 4, 0x248)?;
    c.branch(C::Al, 0x763f0, false)?;
    c.load(1, 8, 0x28)?;
    c.check(0x763b8)?;
    c.store(0, 4, 0x24c)?;
    c.store(1, 4, 0x250)?;
    c.load(0, 6, 0x264)?;
    c.imm(C::Al, Op::Orr, 0, 0, 1)?;
    c.store(0, 6, 0x264)?;
    c.branch(C::Al, 0x763f0, false)?;
    c.imm(C::Al, Op::Add, 0, 8, 0x14)?;
    c.store(0, 4, 0x254)?;
    c.branch(C::Al, 0x763f0, false)?;
    c.byte(C::Al, true, 0, 4, 8)?;
    c.imm(C::Al, Op::Orr, 0, 0, 4)?;
    c.byte(C::Al, false, 0, 4, 8)?;
    c.imm(C::Al, Op::Add, 0, 8, 0x14)?;
    c.store(0, 4, 0x258)?;
    c.check(0x763f0)?;
    c.load(0, 8, 0)?;
    c.reg(Op::Add, 8, 8, 0)?;
    c.branch(C::Al, 0x76250, false)?;
    c.imm(C::Al, Op::Add, 0, 8, 0x14)?;
    c.store(0, 4, 0x240)?;
    c.load(0, 8, 0)?;
    c.reg(Op::Add, 8, 8, 0)?;
    c.byte(C::Al, true, 0, 6, 0x18f)?;
    c.cmp(0, 0)?;
    c.byte(C::Ne, true, 0, 4, 8)?;
    c.imm(C::Ne, Op::Orr, 0, 0, 4)?;
    c.byte(C::Ne, false, 0, 4, 8)?;
    c.branch(C::Al, 0x76250, false)?;
    c.check(0x76424)?;
    c.cmp(0, 0)?;
    c.branch(C::Ne, 0x7621c, false)?;
    c.load(0, 13, 8)?;
    c.load(0, 0, 0x38)?;
    c.reg(Op::Sub, 1, 8, 0)?;
    c.imm(C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.pop(0x4ff0)?;
    c.branch(C::Al, 0x2038, false)
}

fn constructor_allocate_and_publish(c: &mut TypedA32) -> Result<(), String> {
    c.check(CONSTRUCTOR_START)?;
    c.emit(push_registers(C::Al, 0x4bf0)?)?;
    c.imm(C::Al, Op::Add, 11, 13, 0x18)?;
    c.imm(C::Al, Op::Sub, 13, 13, 8)?;
    c.mov(8, 2)?;
    c.literal(2, 0x76608)?;
    c.mov(6, 0)?;
    c.mov(4, 1)?;
    c.set(1, 0x12)?;
    c.cmp(6, 0)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0xa00)?;
    c.imm(C::Ne, Op::Mov, 0, 0, 1)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.set(0, 0x318)?;
    c.imm(C::Al, Op::Orr, 0, 0, 0x400)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.mov(5, 0)?;
    c.cmp(5, 0)?;
    c.branch(C::Eq, 0x765d4, false)?;
    c.set(2, 0x318)?;
    c.mov(0, 5)?;
    c.set(1, 0)?;
    c.check(0x764b4)?;
    c.store(5, 4, 0)?;
    c.set(4, 0)?;
    c.imm(C::Al, Op::Orr, 2, 2, 0x400)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.mov(7, 5)?;
    c.store(6, 5, 0x18)?;
    c.store(4, 5, 0x30)?;
    c.store(4, 5, 0x34)?;
    c.set(1, 0)?;
    c.set(2, 0x14)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, false, r(4), r(7), 0xfc)?)?;
    c.imm(C::Al, Op::Sub, 0, 7, 0x20)?;
    c.store(4, 7, 0x1c)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.set(0, 0xd8)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.cmp(0, 0)?;
    c.store(0, 7, 0xa8)?;
    c.branch(C::Eq, 0x765e4, false)
}

fn constructor_services(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x76500)?;
    c.set(1, 0)?;
    c.set(2, 0xd8)?;
    c.imm(C::Al, Op::Sub, 9, 7, 0xd0)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.imm(C::Al, Op::Add, 1, 5, 0x170)?;
    c.mov(0, 6)?;
    c.branch(C::Al, 0x61c78, true)?;
    c.store(0, 5, 0x24)?;
    c.load(0, 6, 0x68)?;
    c.cmp(0, 0)?;
    c.branch(C::Eq, 0x76538, false)?;
    c.mov(0, 6)?;
    c.branch(C::Al, 0x67888, true)?;
    c.mov(4, 0)?;
    c.imm(C::Al, Op::Add, 2, 5, 0x80)?;
    c.mov(0, 6)?;
    c.mov(1, 9)?;
    c.mov(3, 7)?;
    c.store(4, 5, 0x20)?;
    c.branch(C::Al, 0x522d4, true)?;
    c.store(0, 5, 0x28)?;
    c.mov(0, 6)?;
    c.branch(C::Al, 0x71214, true)?;
    c.store(0, 5, 0x704)?;
    c.set(0, 1)?;
    c.set(2, 0)?;
    c.set(3, 0)?;
    c.byte(C::Al, true, 1, 6, 0x15f)?;
    c.store(2, 13, 0)?;
    c.store(0, 13, 4)?;
    c.imm(C::Al, Op::Add, 0, 5, 0x640)?;
    c.imm(C::Al, Op::Add, 2, 1, 2)?;
    c.set(1, 0x1c0)?;
    c.branch(C::Al, 0x35b30, true)?;
    c.cmp(0, 1)?;
    c.branch(C::Eq, 0x765a4, false)?;
    c.literal(2, 0x76608)?;
    c.set(1, 0x25c)?;
    c.set(0, 0)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0x800)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.literal(4, 0x7660c)?;
    c.cmp(8, 1)?;
    c.branch(C::Ne, 0x765fc, false)?;
    c.set(0, 8)?;
    c.branch(C::Al, 0x2efc, true)?;
    c.literal(2, 0x76608)?;
    c.set(1, 0x63)?;
    c.cmp(0, 0)?;
    c.store(0, 5, 0x14)?;
    c.imm(C::Ne, Op::Mov, 0, 0, 1)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0xa00)?;
    c.branch(C::Al, 0x765f8, false)
}

fn constructor_failure_and_return(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x765d4)?;
    c.set(1, 0x218)?;
    c.set(4, 0)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0x800)?;
    c.branch(C::Al, 0x765f0, false)?;
    c.set(1, 0x37)?;
    c.set(4, 0)?;
    c.imm(C::Al, Op::Orr, 1, 1, 0xa00)?;
    c.literal(2, 0x76608)?;
    c.set(0, 0)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.mov(0, 4)?;
    c.imm(C::Al, Op::Sub, 13, 11, 0x18)?;
    c.pop(0x8bf0)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    parser_entry(&mut c)?;
    unknown_tag_logging(&mut c)?;
    parser_dispatch(&mut c)?;
    parser_updates_and_recovery(&mut c)?;
    for (at, value) in [(0x76444, 0xf3ce9), (0x76448, 0x7f100007),
        (0x7644c, 0xf3d61), (0x76450, 0xf3d47), (0x76454, 0xe2788),
        (0x76458, 0x7f100003), (0x7645c, 0xf3d20)] {
        c.data(at, value)?;
    }
    constructor_allocate_and_publish(&mut c)?;
    constructor_services(&mut c)?;
    constructor_failure_and_return(&mut c)?;
    c.data(0x76608, 0xf3ce9)?;
    c.data(0x7660c, 0x76610)?;
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
    fn structure_and_retail_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1072);
        assert_eq!(CONSTRUCTOR_START - PARSER_START, 640);
        assert_eq!(END - CONSTRUCTOR_START, 432);
        assert_eq!(CODE_BYTES, 1036);
        assert_eq!(DATA_BYTES, 36);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 259);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::data_value(at).is_some()).count(), 9);
        for at in (START..END).step_by(4) {
            if let Some(value) = TypedA32::data_value(at) {
                assert_eq!(word(&bytes, at), value);
            }
        }
        for (at, condition, target, link) in [
            (0x76264, C::Al, 0x17b0, true),
            (0x7628c, C::Al, 0x17b0, true),
            (0x76310, C::Ne, 0x7621c, false),
            (0x7624c, C::Al, 0x76380, false),
            (0x76384, C::Al, 0x17b0, true),
            (0x76388, C::Al, 0x76250, false),
            (0x76394, C::Al, 0x76250, false),
            (0x763f8, C::Al, 0x76250, false),
            (0x76420, C::Al, 0x76250, false),
            (0x76428, C::Ne, 0x7621c, false),
            (0x76440, C::Al, 0x2038, false),
            (0x764a4, C::Eq, 0x765d4, false),
            (0x764fc, C::Eq, 0x765e4, false),
            (0x765a0, C::Al, 0x17b0, true),
            (0x765d0, C::Al, 0x765f8, false),
            (0x765e0, C::Al, 0x765f0, false),
            (0x765f8, C::Al, 0x17b0, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x764b4), transfer_word(C::Al, false, r(5), r(4), 0).unwrap());
        assert_eq!(word(&bytes, 0x764dc), transfer_word_preindexed_writeback(C::Al, false, r(4), r(7), 0xfc).unwrap());
        assert_eq!(word(&bytes, 0x7643c), transfer_multiple(C::Al, true, R::SP, 0x4ff0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x76604), transfer_multiple(C::Al, true, R::SP, 0x8bf0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x765fc), mov_register(C::Al, r(0), r(4)));
        for (at, rd, target) in [(0x761f0, 5, 0x76444), (0x76294, 1, 0x76458),
            (0x76308, 1, 0x76448), (0x76470, 2, 0x76608), (0x765a4, 4, 0x7660c)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(rd), R::PC, target - at as i32 - 8).unwrap());
        }
    }

    #[test]
    fn exact_code_data_and_target_guards() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), TypedA32::is_code(at));
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = TypedA32::data_value(at) {
                assert!(c.data(at, value ^ 1).is_err());
                assert!(c.bytes.is_empty());
                c.data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
        }
        for at in [START - 4, START + 1, 0x76445, 0x76609, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, 0xf3ce9).is_err());
            assert!(c.bytes.is_empty());
        }
        for at in [START, CONSTRUCTOR_START] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.check(END).is_err());
            assert!(c.data(0x76444, 0xf3ce9).is_err());
            for target in [START - 4, START + 1, 0x76444, 0x76608, END, 0x2038] {
                assert!(c.branch(C::Al, target, false).is_err());
            }
            for target in [START, 0x17b4, 0x17b1, 0x2f1c] {
                assert!(c.branch(C::Al, target, true).is_err());
            }
            for target in [START, 0x76445, 0x76609, END] {
                assert!(c.literal(0, target).is_err());
            }
            let other_pool = if at == START { 0x76608 } else { 0x76444 };
            assert!(c.literal(0, other_pool).is_err());
            let other_code = if at == START { CONSTRUCTOR_START } else { START };
            assert!(c.branch(C::Al, other_code, false).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: 0x76440, bytes: Vec::new() };
        assert!(c.branch(C::Ne, 0x2038, false).is_err());
        c.branch(C::Al, 0x2038, false).unwrap();
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
