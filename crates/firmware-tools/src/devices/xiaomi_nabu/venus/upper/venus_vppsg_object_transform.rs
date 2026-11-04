use arm32::{
    branch, data_immediate, data_register, transfer_byte, transfer_doubleword,
    transfer_multiple, transfer_word, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};

pub const START: usize = 0x74be4;
pub const END: usize = 0x74df0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 524;
pub const DATA_BYTES: usize = 0;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VPPsg object transform address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("invalid VPPsg object transform address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(CODE_BYTES) };
    let saved = r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit();
    c.emit(transfer_multiple(C::Al, false, r(13), saved | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(4), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(6), 0x1c)?)?;
    c.check(0x74bf8)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0x74bf8, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0x3de)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0x74c60, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(6), 0x40)?)?;
    c.check(0x74c18)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35d68, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(transfer_multiple(C::Eq, true, r(13), saved | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x170)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0x74c4c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x164)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0x74c4c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x7c)?)?;
    c.branch(C::Al, 0x7fd1c, true)?;
    c.check(0x74c4c)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1a0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.branch(C::Al, 0x74c18, false)?;
    c.check(0x74c60)?;
    c.emit(transfer_multiple(C::Al, true, r(13), saved | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x74c64)?;
    let saved = saved | r(8).bit() | r(9).bit();
    c.emit(transfer_multiple(C::Al, false, r(13), saved | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(8), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.branch(C::Al, 0x35c14, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(5), 0x110)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x74dec, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4c)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, false, r(4), r(6), 0x50)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x110)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x68)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x6c)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x118)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x54)?)?;
    for (source, destination) in [(0x10, 0x58), (0x13c, 0xfc), (0x138, 0x108), (0x18, 0x5c), (0x1c, 0x60), (0x20, 0x64)] {
        c.emit(transfer_word(C::Al, true, r(0), r(7), source)?)?;
        c.emit(transfer_word(C::Al, false, r(0), r(6), destination)?)?;
    }
    c.check(0x74cfc)?;
    c.emit(transfer_doubleword(C::Al, true, r(0), r(5), 0xc0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x110)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 0x114)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x100)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x413)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(7), 0x15c)?)?;
    c.emit(transfer_word(C::Ne, false, r(0), r(6), 0x104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x3f8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0x74d38, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x3ed)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x74db0, false)?;
    c.check(0x74d38)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x134)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x74d8c, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(8), 0x79c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0x78)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.check(0x74d58)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 0x1a4)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(7), r(4), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(2), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(5), 0x1a4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(4), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(1), 0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 4)?)?;
    c.branch(C::Cc, 0x74d58, false)?;
    c.check(0x74d8c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0xf8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x78)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0x78)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1a0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0xb8)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 0xb8)?)?;
    c.check(0x74db0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x284)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x74ddc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x60)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x74)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.check(0x74ddc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), saved | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.branch(C::Al, 0x35ca0, false)?;
    c.check(0x74dec)?;
    c.emit(transfer_multiple(C::Al, true, r(13), saved | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extent() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
    }

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn conditional_compare_chain() {
        let bytes = fragment().unwrap();
        for (at, expected) in [
            (0x74c8c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0).unwrap()),
            (0x74c90, transfer_word(C::Ne, true, r(0), r(5), 0x110).unwrap()),
            (0x74c94, data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0).unwrap()),
            (0x74c98, branch(C::Eq, 0x74c98, 0x74dec, false).unwrap()),
        ] {
            assert_eq!(word(&bytes, at), expected, "{at:#x}");
        }
    }

    #[test]
    fn doubleword_load_and_paired_stores() {
        let bytes = fragment().unwrap();
        for (at, expected) in [
            (0x74cfc, transfer_doubleword(C::Al, true, r(0), r(5), 0xc0).unwrap()),
            (0x74d00, transfer_word(C::Al, false, r(0), r(6), 0x110).unwrap()),
            (0x74d04, transfer_word(C::Al, false, r(1), r(6), 0x114).unwrap()),
        ] {
            assert_eq!(word(&bytes, at), expected, "{at:#x}");
        }
    }

    #[test]
    fn loop_compare_flags_survive_until_backedge() {
        let bytes = fragment().unwrap();
        for (at, expected) in [
            (0x74d64, data_register(C::Al, Op::Cmp, r(0), r(3), r(0), Shift::Lsl, 0, false).unwrap()),
            (0x74d68, data_register(C::Al, Op::Sub, r(7), r(4), r(7), Shift::Lsl, 0, false).unwrap()),
            (0x74d6c, transfer_word(C::Al, false, r(7), r(1), 0).unwrap()),
            (0x74d70, transfer_word(C::Al, true, r(7), r(2), 4).unwrap()),
            (0x74d74, transfer_word(C::Al, true, r(4), r(5), 0x1a4).unwrap()),
            (0x74d78, data_immediate(C::Al, Op::Add, r(2), r(2), 8).unwrap()),
            (0x74d7c, data_register(C::Al, Op::Add, r(7), r(4), r(7), Shift::Lsl, 0, false).unwrap()),
            (0x74d80, transfer_word(C::Al, false, r(7), r(1), 0x40).unwrap()),
            (0x74d84, data_immediate(C::Al, Op::Add, r(1), r(1), 4).unwrap()),
            (0x74d88, branch(C::Cc, 0x74d88, 0x74d58, false).unwrap()),
        ] {
            assert_eq!(word(&bytes, at), expected, "{at:#x}");
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
        assert_eq!(fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
