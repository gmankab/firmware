use arm32::{
    branch, bx, data_immediate, data_register, load_halfword, mov_register,
    push_registers, store_halfword, transfer_halfword_register, transfer_multiple,
    transfer_word, transfer_word_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xaba0;
pub const END: usize = 0xadb0;
const FILE_BASE: usize = 0x4000;
const DIAGNOSTIC_ADDRESS: usize = 0xada8;
const HEAP_DIAGNOSTIC: u32 = 0x000e37d7;
const RETURN_ADDRESS: usize = 0xadac;
const BIN_INDEX: usize = 0xaa48;
const DIAGNOSTIC_CONTEXT: usize = 0x86644;
const ASSERTION: usize = 0x17b0;
const CHECKSUM_SEED_OFFSET: u32 = 0x234;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at == expected {
            Ok(())
        } else {
            Err(format!("heap insertion at {:#x}, expected {expected:#x}", self.at))
        }
    }

    fn emit(&mut self, word: u32) -> Result<(), String> {
        if self.at < START || self.at >= END || self.at % 4 != 0
            || self.at == DIAGNOSTIC_ADDRESS
        {
            return Err(format!("invalid heap insertion code address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn diagnostic(&mut self) -> Result<(), String> {
        self.check(DIAGNOSTIC_ADDRESS)?;
        self.bytes.extend_from_slice(&HEAP_DIAGNOSTIC.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn literal(&mut self, destination: R) -> Result<(), String> {
        self.emit(transfer_word(C::Al, true, destination, R::PC,
            DIAGNOSTIC_ADDRESS as i32 - self.at as i32 - 8)?)
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("A32 register")
}

fn insert_free_block(c: &mut Code) -> Result<(), String> {
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit()
        | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit())?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 28)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 20)?)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.emit(transfer_word(C::Al, true, r(6), r(5), 4)?)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.branch(C::Al, BIN_INDEX, true)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(4), r(0), Shift::Lsl, 4, false)?)?;
    c.emit(mov_register(C::Al, r(9), r(2)))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(1), r(9), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0xac64, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), R::SP, 8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), R::SP, 12)?)?;
    c.emit(transfer_word(C::Al, false, r(0), R::SP, 8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::SP, 16)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(8), 8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(2), R::SP, 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0xacbc, false)?;
    c.literal(r(6))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x56)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(10), r(10), 0x300)?)?;
    c.check(0xac08)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, 0xac28, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 16)?)?;
    c.emit(mov_register(C::Al, r(8), r(1)))?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, 0xac40, false)?;
    c.branch(C::Al, DIAGNOSTIC_CONTEXT, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(mov_register(C::Al, r(1), r(10)))?;
    c.emit(mov_register(C::Al, r(2), r(6)))?;
    c.branch(C::Al, ASSERTION, true)?;
    c.emit(transfer_word(C::Al, true, r(8), r(7), 0)?)?;
    c.check(0xac40)?;
    c.emit(mov_register(C::Al, r(7), r(8)))?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(1), r(7), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0xacb8, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(1), 4)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cc, 0xac08, false)?;
    c.branch(C::Al, 0xacbc, false)?;
    c.check(0xac64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(9), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(2), 52)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(2), 60)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 56)?)?;
    c.emit(mov_register(C::Al, r(2), r(0)))?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 4, false)?)?;
    c.check(0xac88)?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(4), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 1)?)?;
    c.emit(store_halfword(C::Al, r(0), r(3), 0)?)?;
    c.branch(C::Lt, 0xada0, false)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(3), r(2), 1)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(4), r(2), Shift::Lsl, 4, false)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(2), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.emit(mov_register(C::Al, r(2), r(3)))?;
    c.branch(C::Eq, 0xac88, false)?;
    c.branch(C::Al, 0xada0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.check(0xacbc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), R::SP, 8)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xad34, false)?;
    c.emit(load_halfword(C::Al, r(2), r(8), 0)?)?;
    c.emit(load_halfword(C::Al, r(3), r(8), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), CHECKSUM_SEED_OFFSET)?)?;
    c.emit(transfer_word(C::Al, true, r(6), R::SP, 4)?)?;
    c.emit(transfer_halfword_register(C::Al, true, r(0), r(4), r(0))?)?;
    c.emit(data_register(C::Al, Op::Eor, r(2), r(3), r(2), Shift::Lsl, 0, false)?)?;
    for offset in [4, 6, 8, 10] {
        c.emit(load_halfword(C::Al, r(3), r(8), offset)?)?;
        c.emit(data_register(C::Al, Op::Eor, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    }
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0xff00)?)?;
    c.emit(data_register(C::Al, Op::And, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Eq, 0xad38, false)?;
    c.branch(C::Al, DIAGNOSTIC_CONTEXT, true)?;
    c.literal(r(2))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x5d)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x300)?)?;
    c.branch(C::Al, ASSERTION, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0)?)?;
    c.branch(C::Al, 0xad38, false)?;
    c.check(0xad34)?;
    c.emit(transfer_word(C::Al, true, r(6), R::SP, 4)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), R::SP, 8)?)?;
    c.emit(transfer_word(C::Eq, false, r(5), r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(transfer_word(C::Eq, false, r(5), r(6), 52)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 8)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(7), 0)?)?;
    c.branch(C::Eq, 0xad94, false)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 2)?)?;
    c.emit(load_halfword(C::Al, r(2), r(8), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), CHECKSUM_SEED_OFFSET)?)?;
    c.emit(transfer_halfword_register(C::Al, true, r(0), r(4), r(0))?)?;
    c.emit(data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    for offset in [6, 8, 10] {
        c.emit(load_halfword(C::Al, r(2), r(8), offset)?)?;
        c.emit(data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    }
    c.emit(data_register(C::Al, Op::Eor, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(store_halfword(C::Al, r(0), r(8), 0)?)?;
    c.check(0xad94)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 60)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 60)?)?;
    c.check(0xada0)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 28)?)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit()
        | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit()
        | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(DIAGNOSTIC_ADDRESS)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut code = Code { at: START, bytes: Vec::with_capacity(END - START) };
    insert_free_block(&mut code)?;
    code.diagnostic()?;
    code.check(RETURN_ADDRESS)?;
    code.emit(bx(C::Al, R::LR))?;
    code.check(END)?;
    Ok(code.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], address: usize) -> u32 {
        u32::from_le_bytes(bytes[address - START..address - START + 4].try_into().unwrap())
    }

    #[test]
    fn layout_and_protected_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 528);
        assert_eq!(bytes.len() - 4, 524);
        assert_eq!(word(&bytes, DIAGNOSTIC_ADDRESS), HEAP_DIAGNOSTIC);
        assert_eq!(word(&bytes, RETURN_ADDRESS), bx(C::Al, R::LR));
        let mut code = Code { at: DIAGNOSTIC_ADDRESS, bytes: Vec::new() };
        assert!(code.emit(bx(C::Al, R::LR)).is_err());
        assert!(code.bytes.is_empty());
        code.diagnostic().unwrap();
        assert!(code.diagnostic().is_err());
        code.at = END;
        assert!(code.emit(bx(C::Al, R::LR)).is_err());
    }

    #[test]
    fn bin_links_count_hints_and_checksum() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0xabbc, C::Al, BIN_INDEX, true),
            (0xabd0, C::Eq, 0xac64, false),
            (0xabf8, C::Cs, 0xacbc, false),
            (0xac5c, C::Cc, 0xac08, false),
            (0xacb0, C::Eq, 0xac88, false),
            (0xad10, C::Eq, 0xad38, false),
            (0xad28, C::Al, ASSERTION, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        for (at, condition, destination, base, offset) in [
            (0xac68, C::Al, r(5), r(9), 0),
            (0xac6c, C::Al, r(5), r(2), 52),
            (0xac78, C::Al, r(1), r(2), 60),
            (0xad44, C::Eq, r(5), r(9), 0),
            (0xad4c, C::Eq, r(5), r(6), 52),
            (0xad9c, C::Al, r(0), r(6), 60),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(condition, false, destination, base, offset).unwrap());
        }
        assert_eq!(word(&bytes, 0xac90), store_halfword(C::Al, r(0), r(3), 0).unwrap());
        assert_eq!(word(&bytes, 0xad90), store_halfword(C::Al, r(0), r(8), 0).unwrap());
        for (at, destination) in [(0xabfc, r(6)), (0xad18, r(2))] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, destination, R::PC,
                DIAGNOSTIC_ADDRESS as i32 - at as i32 - 8).unwrap());
        }
    }

    #[test]
    fn optional_pinned_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_BASE..END + FILE_BASE]);
    }
}
