use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register, mov_register,
    push_registers, transfer_multiple, transfer_word, transfer_word_postindexed, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7eb40;
pub const END: usize = 0x7edc8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 620;
pub const DATA_BYTES: usize = 28;

const LITERALS: [(usize, u32); 7] = [
    (0x7ec58, 0xf478b),
    (0x7ec5c, 0xf47b5),
    (0x7ec60, 0xf4804),
    (0x7ec64, 0xe2788),
    (0x7ec68, 0xf481d),
    (0x7ec6c, 0xf4847),
    (0x7edc4, 0xf478b),
];
const ASSERT: usize = 0x17b0;
const LOCK: usize = 0x2bf4;
const UNLOCK: usize = 0x2c34;
const LOG: usize = 0x2ec94;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("VBuffer address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, at: usize, value: u32, data: bool) -> Result<(), String> {
        self.check(at)?;
        let literal = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, v)| v);
        if !(START..END).contains(&at) || at % 4 != 0
            || if data { literal != Some(value) } else { literal.is_some() }
        {
            return Err(format!("invalid VBuffer {} at {at:#x}", if data { "data" } else { "code" }));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.word(at, instruction, false)
    }

    fn immediate(&mut self, at: usize, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(at, data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn mov(&mut self, at: usize, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(at, mov_register(C::Al, r(rd), r(rm)))
    }

    fn cmp(&mut self, at: usize, rn: u8, rm: u8) -> Result<(), String> {
        self.emit(at, data_register(C::Al, Op::Cmp, r(0), r(rn), r(rm), Shift::Lsl, 0, false)?)
    }

    fn transfer(&mut self, at: usize, condition: C, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_word(condition, load, r(rd), r(rn), offset)?)
    }

    fn literal(&mut self, at: usize, rd: u8, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| address == slot) {
            return Err(format!("invalid VBuffer literal {slot:#x}"));
        }
        self.transfer(at, C::Al, true, rd, 15, slot as i32 - at as i32 - 8)
    }

    fn jump(&mut self, at: usize, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target as u32, link)?)
    }

    fn push(&mut self, at: usize, registers: u16) -> Result<(), String> {
        self.emit(at, push_registers(C::Al, registers)?)
    }

    fn pop(&mut self, at: usize, condition: C, registers: u16) -> Result<(), String> {
        self.emit(at, transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }
}

fn release(c: &mut Code) -> Result<(), String> {
    c.push(0x7eb40, 0x48f0)?;
    c.immediate(0x7eb44, C::Al, Op::Add, 11, 13, 0x10)?;
    c.immediate(0x7eb48, C::Al, Op::Sub, 13, 13, 8)?;
    c.mov(0x7eb4c, 5, 0)?;
    c.transfer(0x7eb50, C::Al, true, 0, 5, 0xc)?;
    c.transfer(0x7eb54, C::Al, true, 4, 5, 0x24)?;
    c.immediate(0x7eb58, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0x7eb5c, C::Ne, 0x7eb74, false)?;
    c.literal(0x7eb60, 2, 0x7ec58)?;
    c.immediate(0x7eb64, C::Al, Op::Mov, 1, 0, 0xa6)?;
    c.immediate(0x7eb68, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0x7eb6c, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.jump(0x7eb70, C::Al, ASSERT, true)?;
    c.transfer(0x7eb74, C::Al, true, 0, 5, 4)?;
    c.immediate(0x7eb78, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0x7eb7c, C::Ne, 0x7ebc4, false)?;
    c.transfer(0x7eb80, C::Al, true, 0, 5, 0x10)?;
    c.immediate(0x7eb84, C::Al, Op::Mov, 1, 0, 0)?;
    c.literal(0x7eb88, 2, 0x7ec60)?;
    c.immediate(0x7eb8c, C::Al, Op::Mov, 3, 0, 0x1b4)?;
    c.transfer(0x7eb90, C::Al, false, 1, 13, 0)?;
    c.literal(0x7eb94, 1, 0x7ec5c)?;
    c.transfer(0x7eb98, C::Al, false, 0, 13, 4)?;
    c.immediate(0x7eb9c, C::Al, Op::Mov, 0, 0, 0x10)?;
    c.jump(0x7eba0, C::Al, LOG, true)?;
    c.literal(0x7eba4, 1, 0x7ec64)?;
    c.immediate(0x7eba8, C::Al, Op::Mov, 0, 0, 0x10)?;
    c.jump(0x7ebac, C::Al, LOG, true)?;
    c.literal(0x7ebb0, 2, 0x7ec58)?;
    c.immediate(0x7ebb4, C::Al, Op::Mov, 1, 0, 0xb5)?;
    c.immediate(0x7ebb8, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0x7ebbc, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.jump(0x7ebc0, C::Al, ASSERT, true)?;
    c.transfer(0x7ebc4, C::Al, true, 0, 4, 0)?;
    c.jump(0x7ebc8, C::Al, LOCK, true)?;
    c.transfer(0x7ebcc, C::Al, true, 0, 5, 4)?;
    c.emit(0x7ebd0, data_immediate_with_flags(C::Al, Op::Sub, r(6), r(0), 1, true)?)?;
    c.transfer(0x7ebd4, C::Al, false, 6, 5, 4)?;
    c.jump(0x7ebd8, C::Ne, 0x7ec1c, false)?;
    c.transfer(0x7ebdc, C::Al, true, 0, 4, 0x18)?;
    c.literal(0x7ebe0, 1, 0x7ec68)?;
    c.immediate(0x7ebe4, C::Al, Op::Add, 0, 0, 1)?;
    c.transfer(0x7ebe8, C::Al, false, 0, 4, 0x18)?;
    c.immediate(0x7ebec, C::Al, Op::Mov, 0, 0, 0)?;
    c.transfer(0x7ebf0, C::Al, false, 0, 5, 0xc)?;
    c.transfer(0x7ebf4, C::Al, false, 0, 5, 0x10)?;
    c.transfer(0x7ebf8, C::Al, false, 0, 5, 0x18)?;
    c.transfer(0x7ebfc, C::Al, true, 0, 4, 0xc)?;
    c.immediate(0x7ec00, C::Al, Op::Sub, 0, 0, 1)?;
    c.transfer(0x7ec04, C::Al, false, 0, 4, 0xc)?;
    c.immediate(0x7ec08, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0x7ec0c, C::Al, LOG, true)?;
    c.literal(0x7ec10, 1, 0x7ec64)?;
    c.immediate(0x7ec14, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0x7ec18, C::Al, LOG, true)?;
    c.transfer(0x7ec1c, C::Al, true, 0, 4, 0)?;
    c.jump(0x7ec20, C::Al, UNLOCK, true)?;
    c.transfer(0x7ec24, C::Al, true, 0, 5, 4)?;
    c.transfer(0x7ec28, C::Al, true, 3, 5, 0x10)?;
    c.literal(0x7ec2c, 1, 0x7ec6c)?;
    c.mov(0x7ec30, 2, 4)?;
    c.transfer(0x7ec34, C::Al, false, 0, 13, 0)?;
    c.immediate(0x7ec38, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0x7ec3c, C::Al, LOG, true)?;
    c.literal(0x7ec40, 1, 0x7ec64)?;
    c.immediate(0x7ec44, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0x7ec48, C::Al, LOG, true)?;
    c.mov(0x7ec4c, 0, 6)?;
    c.immediate(0x7ec50, C::Al, Op::Sub, 13, 11, 0x10)?;
    c.pop(0x7ec54, C::Al, 0x88f0)?;
    c.check(0x7ec58)?;
    for &(at, value) in &LITERALS[..6] {
        c.word(at, value, true)?;
    }
    c.check(0x7ec70)
}

fn find_release(c: &mut Code) -> Result<(), String> {
    c.immediate(0x7ec70, C::Al, Op::Cmp, 0, 0, 0)?;
    c.transfer(0x7ec74, C::Ne, true, 12, 0, 0x10)?;
    c.immediate(0x7ec78, C::Ne, Op::Cmp, 0, 12, 0)?;
    c.jump(0x7ec7c, C::Eq, 0x7ecb8, false)?;
    c.transfer(0x7ec80, C::Al, true, 0, 0, 4)?;
    c.immediate(0x7ec84, C::Al, Op::Mov, 3, 0, 0)?;
    c.transfer(0x7ec88, C::Al, true, 2, 0, 0x10)?;
    c.cmp(0x7ec8c, 2, 1)?;
    c.jump(0x7ec90, C::Eq, 0x7eca8, false)?;
    c.immediate(0x7ec94, C::Al, Op::Add, 3, 3, 1)?;
    c.immediate(0x7ec98, C::Al, Op::Add, 0, 0, 0x28)?;
    c.cmp(0x7ec9c, 3, 12)?;
    c.jump(0x7eca0, C::Cc, 0x7ec88, false)?;
    c.jump(0x7eca4, C::Al, 0x7ecb8, false)?;
    c.push(0x7eca8, 0x4800)?;
    c.mov(0x7ecac, 11, 13)?;
    c.jump(0x7ecb0, C::Al, START, true)?;
    c.pop(0x7ecb4, C::Al, 0x4800)?;
    c.immediate(0x7ecb8, C::Al, Op::Mov, 0, 0, 0)?;
    c.emit(0x7ecbc, bx(C::Al, R::LR))?;
    c.check(0x7ecc0)
}

fn reset(c: &mut Code) -> Result<(), String> {
    c.push(0x7ecc0, 0x4830)?;
    c.immediate(0x7ecc4, C::Al, Op::Add, 11, 13, 8)?;
    c.mov(0x7ecc8, 5, 0)?;
    c.immediate(0x7eccc, C::Al, Op::Cmp, 0, 5, 0)?;
    c.pop(0x7ecd0, C::Eq, 0x8830)?;
    c.transfer(0x7ecd4, C::Al, true, 0, 5, 0)?;
    c.jump(0x7ecd8, C::Al, LOCK, true)?;
    c.transfer(0x7ecdc, C::Al, true, 0, 5, 0x10)?;
    c.immediate(0x7ece0, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0x7ece4, C::Eq, 0x7ed14, false)?;
    c.transfer(0x7ece8, C::Al, true, 2, 5, 4)?;
    c.transfer(0x7ecec, C::Al, true, 1, 5, 0x18)?;
    c.immediate(0x7ecf0, C::Al, Op::Mov, 3, 0, 0)?;
    c.immediate(0x7ecf4, C::Al, Op::Mov, 4, 0, 0)?;
    c.immediate(0x7ecf8, C::Al, Op::Add, 2, 2, 0xc)?;
    c.immediate(0x7ecfc, C::Al, Op::Add, 4, 4, 1)?;
    c.emit(0x7ed00, transfer_word_postindexed(C::Al, false, r(3), r(2), 0x28)?)?;
    c.immediate(0x7ed04, C::Al, Op::Sub, 1, 1, 1)?;
    c.cmp(0x7ed08, 4, 0)?;
    c.jump(0x7ed0c, C::Cc, 0x7ecfc, false)?;
    c.transfer(0x7ed10, C::Al, false, 1, 5, 0x18)?;
    c.immediate(0x7ed14, C::Al, Op::Mov, 0, 0, 0)?;
    c.transfer(0x7ed18, C::Al, false, 0, 5, 0xc)?;
    c.transfer(0x7ed1c, C::Al, true, 0, 5, 0)?;
    c.pop(0x7ed20, C::Al, 0x4830)?;
    c.jump(0x7ed24, C::Al, UNLOCK, false)?;
    c.check(0x7ed28)
}

fn find_clear(c: &mut Code) -> Result<(), String> {
    c.push(0x7ed28, 0x48f0)?;
    c.immediate(0x7ed2c, C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(0x7ed30, 6, 0)?;
    c.mov(0x7ed34, 5, 1)?;
    c.immediate(0x7ed38, C::Al, Op::Cmp, 0, 6, 0)?;
    c.pop(0x7ed3c, C::Eq, 0x88f0)?;
    c.transfer(0x7ed40, C::Al, true, 0, 6, 0)?;
    c.jump(0x7ed44, C::Al, LOCK, true)?;
    c.transfer(0x7ed48, C::Al, true, 1, 6, 0x10)?;
    c.immediate(0x7ed4c, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0x7ed50, C::Al, Op::Mov, 2, 0, 0)?;
    c.immediate(0x7ed54, C::Al, Op::Cmp, 0, 1, 0)?;
    c.jump(0x7ed58, C::Eq, 0x7eda0, false)?;
    c.transfer(0x7ed5c, C::Al, true, 2, 6, 4)?;
    c.immediate(0x7ed60, C::Al, Op::Add, 3, 2, 8)?;
    c.immediate(0x7ed64, C::Al, Op::Mov, 2, 0, 0)?;
    c.transfer(0x7ed68, C::Al, true, 4, 3, 8)?;
    c.cmp(0x7ed6c, 4, 5)?;
    c.jump(0x7ed70, C::Eq, 0x7ed88, false)?;
    c.immediate(0x7ed74, C::Al, Op::Add, 2, 2, 1)?;
    c.immediate(0x7ed78, C::Al, Op::Add, 3, 3, 0x28)?;
    c.cmp(0x7ed7c, 2, 1)?;
    c.jump(0x7ed80, C::Cc, 0x7ed68, false)?;
    c.jump(0x7ed84, C::Al, 0x7eda0, false)?;
    c.immediate(0x7ed88, C::Al, Op::Mov, 5, 0, 0)?;
    c.transfer(0x7ed8c, C::Al, false, 5, 3, 0)?;
    c.transfer(0x7ed90, C::Al, false, 5, 3, 4)?;
    c.transfer(0x7ed94, C::Al, true, 3, 6, 0xc)?;
    c.immediate(0x7ed98, C::Al, Op::Sub, 3, 3, 1)?;
    c.transfer(0x7ed9c, C::Al, false, 3, 6, 0xc)?;
    c.cmp(0x7eda0, 2, 1)?;
    c.literal(0x7eda4, 2, 0x7edc4)?;
    c.immediate(0x7eda8, C::Al, Op::Mov, 1, 0, 0x29)?;
    c.immediate(0x7edac, C::Cc, Op::Mov, 0, 0, 1)?;
    c.immediate(0x7edb0, C::Al, Op::Orr, 1, 1, 0x200)?;
    c.jump(0x7edb4, C::Al, ASSERT, true)?;
    c.transfer(0x7edb8, C::Al, true, 0, 6, 0)?;
    c.pop(0x7edbc, C::Al, 0x48f0)?;
    c.jump(0x7edc0, C::Al, UNLOCK, false)?;
    c.word(0x7edc4, 0xf478b, true)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    release(&mut c)?;
    find_release(&mut c)?;
    reset(&mut c)?;
    find_clear(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != END - START || CODE_BYTES + DATA_BYTES != END - START {
        return Err("invalid VBuffer extent".into());
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
    fn extent_and_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 648);
        assert_eq!(CODE_BYTES + DATA_BYTES, bytes.len());
        for (at, value) in LITERALS {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn exact_code_data_guards() {
        for at in (START..END).step_by(4) {
            let literal = LITERALS.iter().find(|&&(slot, _)| slot == at);
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(at, bx(C::Al, R::LR)).is_err(), literal.is_some());
            for (_, value) in LITERALS {
                let valid = literal.is_some_and(|&(_, expected)| expected == value);
                let mut c = Code { at, bytes: Vec::new() };
                assert_eq!(c.word(at, value, true).is_ok(), valid);
                assert_eq!(c.at, at + if valid { 4 } else { 0 });
                assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
            }
            if let Some(&(_, value)) = literal {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.word(at, value ^ 1, true).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
        }
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.word(at, 0xf478b, true).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.literal(START, 0, START).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn conditional_entry_flags_and_tail_calls() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x7ec70), data_immediate(C::Al, Op::Cmp, r(0), r(0), 0).unwrap());
        assert_eq!(word(&bytes, 0x7ec74), transfer_word(C::Ne, true, R::R12, r(0), 0x10).unwrap());
        assert_eq!(word(&bytes, 0x7ec78), data_immediate(C::Ne, Op::Cmp, r(0), R::R12, 0).unwrap());
        assert_eq!(word(&bytes, 0x7eca8), push_registers(C::Al, 0x4800).unwrap());
        assert_eq!(word(&bytes, 0x7ebd0), data_immediate_with_flags(C::Al, Op::Sub, r(6), r(0), 1, true).unwrap());
        assert_eq!(word(&bytes, 0x7ed00), transfer_word_postindexed(C::Al, false, r(3), r(2), 0x28).unwrap());
        assert_eq!(word(&bytes, 0x7edac), data_immediate(C::Cc, Op::Mov, r(0), r(0), 1).unwrap());
        for (at, condition, target, link) in [
            (0x7ebd8, C::Ne, 0x7ec1c, false),
            (0x7ec90, C::Eq, 0x7eca8, false),
            (0x7eca0, C::Cc, 0x7ec88, false),
            (0x7ecb0, C::Al, START, true),
            (0x7ed0c, C::Cc, 0x7ecfc, false),
            (0x7ed24, C::Al, UNLOCK, false),
            (0x7ed70, C::Eq, 0x7ed88, false),
            (0x7ed80, C::Cc, 0x7ed68, false),
            (0x7edc0, C::Al, UNLOCK, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
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
