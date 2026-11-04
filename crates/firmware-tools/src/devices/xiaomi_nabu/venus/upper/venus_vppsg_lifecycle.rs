use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register, mov_register, push_registers, transfer_byte,
    transfer_multiple, transfer_word, transfer_word_postindexed, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x74ab0;
pub const END: usize = 0x74be4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 292;
pub const DATA_BYTES: usize = 16;

const CONTEXT_TABLE_SLOT: usize = 0x74b0c;
const CONTEXT_TABLE: u32 = 0x10ceac;
const SOURCE_SLOT: usize = 0x74bd8;
const SOURCE: u32 = 0xf35fa;
const DESTROY_FORMAT_SLOT: usize = 0x74bdc;
const DESTROY_FORMAT: u32 = 0xf3630;
const NEWLINE_SLOT: usize = 0x74be0;
const NEWLINE: u32 = 0xe2788;
const CHECK_CONTEXT: usize = 0x17b0;
const RELEASE: usize = 0x2f1c;
const CLEANUP_NESTED: usize = 0x35bec;
const LOG: usize = 0x2ec94;
const SAVED_REGISTERS: u16 = 0x4bf0;

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
            return Err(format!("VppSg address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        let expected = match self.at {
            CONTEXT_TABLE_SLOT => Some(CONTEXT_TABLE),
            SOURCE_SLOT => Some(SOURCE),
            DESTROY_FORMAT_SLOT => Some(DESTROY_FORMAT),
            NEWLINE_SLOT => Some(NEWLINE),
            _ => None,
        };
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || if data { expected != Some(value) } else { expected.is_some() }
        {
            return Err(format!("invalid VppSg {} address {:#x}", if data { "data" } else { "code" }, self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn literal_data(&mut self, address: usize, value: u32) -> Result<(), String> {
        self.check(address)?;
        self.word(value, true)
    }

    fn immediate(&mut self, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn mov(&mut self, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(C::Al, r(rd), r(rm)))
    }

    fn word_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn byte_load(&mut self, condition: C, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(condition, true, r(rd), r(rn), offset)?)
    }

    fn literal(&mut self, rd: u8, address: usize) -> Result<(), String> {
        self.word_transfer(true, rd, 15, address as i32 - self.at as i32 - 8)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn pop(&mut self, condition: C, registers: u16) -> Result<(), String> {
        self.emit(transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }
}

fn initialize_context(c: &mut TypedA32) -> Result<(), String> {
    c.check(START)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.word_transfer(true, 2, 0, 0x10)?;
    c.literal(3, CONTEXT_TABLE_SLOT)?;
    c.word_transfer(false, 1, 0, 0x18)?;
    c.word_transfer(true, 1, 0, 0x78)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(3), r(1), Shift::Lsl, 3, false)?)?;
    c.immediate(C::Al, Op::Add, 3, 1, 0x44)?;
    c.word_transfer(false, 3, 0, 8)?;
    c.byte_load(C::Al, 3, 2, 0x15e)?;
    c.immediate(C::Al, Op::Cmp, 0, 3, 2)?;
    c.branch(C::Cc, 0x74ae8, false)?;
    c.byte_load(C::Al, 2, 2, 0x159)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, 0)?;
    c.branch(C::Eq, 0x74afc, false)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0x3c)?;
    c.word_transfer(false, 1, 0, 0xc)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.word_transfer(false, 1, 0, 0)?;
    c.emit(bx(C::Al, R::LR))?;
    c.immediate(C::Al, Op::Cmp, 0, 3, 2)?;
    c.branch(C::Ne, 0x74af0, false)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0x34)?;
    c.check(0x74b08)?;
    c.branch(C::Al, 0x74aec, false)?;
    c.literal_data(CONTEXT_TABLE_SLOT, CONTEXT_TABLE)
}

fn cleanup_context(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x74b10)?;
    c.emit(push_registers(C::Al, SAVED_REGISTERS)?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x18)?;
    c.literal(2, SOURCE_SLOT)?;
    c.mov(4, 0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x1f)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x600)?;
    c.immediate(C::Ne, Op::Mov, 0, 0, 1)?;
    c.branch(C::Al, CHECK_CONTEXT, true)?;
    c.word_transfer(true, 8, 4, 0x30)?;
    c.immediate(C::Al, Op::Cmp, 0, 8, 0)?;
    c.pop(C::Eq, (SAVED_REGISTERS & !R::LR.bit()) | R::PC.bit())?;
    c.word_transfer(true, 0, 4, 0x118)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x74b54, false)?;
    c.word_transfer(true, 0, 8, 4)?;
    c.branch(C::Al, RELEASE, true)?;
    c.immediate(C::Al, Op::Add, 0, 8, 0x1c)?;
    c.branch(C::Al, CLEANUP_NESTED, true)?;
    c.byte_load(C::Al, 0, 4, 0x3de)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x74b70, false)?;
    c.immediate(C::Al, Op::Add, 0, 8, 0x40)?;
    c.branch(C::Al, CLEANUP_NESTED, true)?;
    c.byte_load(C::Al, 0, 4, 0x413)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.byte_load(C::Ne, 0, 4, 0x3df)?;
    c.byte_load(C::Ne, 1, 4, 0x3e0)?;
    c.emit(data_register(C::Ne, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.immediate(C::Ne, Op::Add, 0, 0, 1)?;
    c.emit(data_immediate_with_flags(C::Ne, Op::And, r(6), r(0), 0xff, true)?)?;
    c.branch(C::Eq, 0x74bac, false)?;
    c.immediate(C::Al, Op::Add, 7, 8, 0x68)?;
    c.immediate(C::Al, Op::Mov, 5, 0, 0)?;
    c.check(0x74b98)?;
    c.emit(transfer_word_postindexed(C::Al, true, r(0), r(7), 4)?)?;
    c.branch(C::Al, RELEASE, true)?;
    c.immediate(C::Al, Op::Add, 5, 5, 1)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x74b98, false)?;
    c.mov(0, 8)?;
    c.branch(C::Al, RELEASE, true)?;
    c.literal(1, DESTROY_FORMAT_SLOT)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.word_transfer(false, 0, 4, 0x30)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 4)?;
    c.branch(C::Al, LOG, true)?;
    c.literal(1, NEWLINE_SLOT)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 4)?;
    c.pop(C::Al, SAVED_REGISTERS)?;
    c.branch(C::Al, LOG, false)?;
    c.literal_data(SOURCE_SLOT, SOURCE)?;
    c.literal_data(DESTROY_FORMAT_SLOT, DESTROY_FORMAT)?;
    c.literal_data(NEWLINE_SLOT, NEWLINE)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    initialize_context(&mut c)?;
    cleanup_context(&mut c)?;
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
    fn exact_extent_and_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 308);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(CODE_BYTES + DATA_BYTES, bytes.len());
        for (at, value) in [
            (CONTEXT_TABLE_SLOT, CONTEXT_TABLE),
            (SOURCE_SLOT, SOURCE),
            (DESTROY_FORMAT_SLOT, DESTROY_FORMAT),
            (NEWLINE_SLOT, NEWLINE),
        ] {
            assert_eq!(word(&bytes, at), value);
        }
        assert_eq!(word(&bytes, 0x74b08), branch(C::Al, 0x74b08, 0x74aec, false).unwrap());
    }

    #[test]
    fn protected_data_and_code() {
        for at in (START..END).step_by(4) {
            let data = matches!(at, CONTEXT_TABLE_SLOT | SOURCE_SLOT | DESTROY_FORMAT_SLOT | NEWLINE_SLOT);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(bx(C::Al, R::LR)).is_err(), data);
            for (slot, value) in [
                (CONTEXT_TABLE_SLOT, CONTEXT_TABLE),
                (SOURCE_SLOT, SOURCE),
                (DESTROY_FORMAT_SLOT, DESTROY_FORMAT),
                (NEWLINE_SLOT, NEWLINE),
            ] {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.literal_data(at, value).is_ok(), at == slot);
                if at == slot {
                    assert_eq!(c.at, at + 4);
                    assert_eq!(c.bytes, value.to_le_bytes());
                } else {
                    assert_eq!(c.at, at);
                    assert!(c.bytes.is_empty());
                }
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.literal_data(at, value ^ 1).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
        }
        let mut c = TypedA32 { at: END, bytes: Vec::new() };
        assert!(c.emit(bx(C::Al, R::LR)).is_err());
        assert!(c.literal_data(END, NEWLINE).is_err());
    }

    #[test]
    fn ands_flags_and_consumers() {
        for (condition, expected) in [(C::Al, 0xe21060ff), (C::Ne, 0x121060ff)] {
            let mut c = TypedA32 { at: START, bytes: Vec::new() };
            c.emit(data_immediate_with_flags(condition, Op::And, r(6), r(0), 0xff, true).unwrap()).unwrap();
            assert_eq!(word(&c.bytes, START), expected);
            assert_ne!(expected & (1 << 20), 0);
            assert_eq!(data_immediate(condition, Op::And, r(6), r(0), 0xff).unwrap(), expected & !(1 << 20));
        }
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x74b74), 0xe3500000);
        assert_eq!(word(&bytes, 0x74b78), 0x15d403df);
        assert_eq!(word(&bytes, 0x74b7c), 0x15d413e0);
        assert_eq!(word(&bytes, 0x74b80), 0x10800001);
        assert_eq!(word(&bytes, 0x74b84), 0x12800001);
        assert_eq!(word(&bytes, 0x74b88), 0x121060ff);
        assert_eq!(word(&bytes, 0x74b8c), branch(C::Eq, 0x74b8c, 0x74bac, false).unwrap());
        assert_eq!(word(&bytes, 0x74ba4), 0xe1550006);
        assert_eq!(word(&bytes, 0x74ba8), branch(C::Ne, 0x74ba8, 0x74b98, false).unwrap());
        for (at, rd, slot) in [
            (0x74ab8, 3, CONTEXT_TABLE_SLOT),
            (0x74b18, 2, SOURCE_SLOT),
            (0x74bb4, 1, DESTROY_FORMAT_SLOT),
            (0x74bc8, 1, NEWLINE_SLOT),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(rd), R::PC, slot as i32 - at as i32 - 8).unwrap());
        }
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
