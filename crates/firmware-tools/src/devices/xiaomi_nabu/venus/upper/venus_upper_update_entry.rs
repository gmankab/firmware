use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, mov_register,
    mov_shifted_register, multiply_long, multiply_register, store_halfword, transfer_byte,
    transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    transfer_word_register, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6b990;
pub const END: usize = 0x6bc50;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 704;
pub const DATA_BYTES: usize = 0;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, expected: usize, instruction: u32) -> Result<(), String> {
        if self.at != expected || !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("update entry address {:#x}, expected {expected:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(CODE_BYTES) };
    c.emit(0x6b990, transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x6b994, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x6b998, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x7c)?)?;
    c.emit(0x6b99c, mov_register(C::Al, r(10), r(1)))?;
    c.emit(0x6b9a0, mov_register(C::Al, r(9), r(0)))?;
    c.emit(0x6b9a4, transfer_word(C::Al, true, r(4), r(11), 0x8)?)?;
    c.emit(0x6b9a8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6b9ac, transfer_word(C::Al, true, r(0), r(10), 0x48)?)?;
    c.emit(0x6b9b0, transfer_word(C::Al, false, r(0), r(9), 0x9b4)?)?;
    c.emit(0x6b9b4, transfer_word(C::Al, true, r(0), r(10), 0x2c)?)?;
    c.emit(0x6b9b8, transfer_word(C::Al, false, r(0), r(9), 0x78)?)?;
    c.emit(0x6b9bc, transfer_byte(C::Al, false, r(1), r(10), 0x4b1)?)?;
    c.emit(0x6b9c0, data_immediate(C::Al, Op::Add, r(1), r(4), 0x280)?)?;
    c.emit(0x6b9c4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6b9c8, transfer_word(C::Al, false, r(1), r(13), 0x3c)?)?;
    c.emit(0x6b9cc, transfer_word(C::Al, true, r(5), r(15), 0xf88)?)?;
    c.emit(0x6b9d0, transfer_word(C::Al, true, r(6), r(15), 0xf88)?)?;
    c.emit(0x6b9d4, transfer_word(C::Al, false, r(10), r(13), 0x44)?)?;
    c.emit(0x6b9d8, transfer_word(C::Al, false, r(9), r(11), -0x3c)?)?;
    c.emit(0x6b9dc, branch(C::Eq, 0x6b9dc, 0x6bae4, false)?)?;
    c.emit(0x6b9e0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb30)?)?;
    c.emit(0x6b9e4, transfer_word(C::Al, false, r(6), r(13), 0x14)?)?;
    c.emit(0x6b9e8, transfer_word(C::Al, false, r(5), r(13), 0xc)?)?;
    c.emit(0x6b9ec, transfer_word(C::Al, false, r(3), r(13), 0x2c)?)?;
    c.emit(0x6b9f0, mov_register(C::Al, r(1), r(9)))?;
    c.emit(0x6b9f4, data_immediate(C::Al, Op::Sub, r(6), r(11), 0x38)?)?;
    c.emit(0x6b9f8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x18)?)?;
    c.emit(0x6b9fc, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6ba00, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(1), r(0), Shift::Lsl, 0)?)?;
    c.emit(0x6ba04, transfer_word(C::Al, false, r(1), r(13), 0x4c)?)?;
    c.emit(0x6ba08, transfer_word(C::Al, false, r(0), r(11), -0x44)?)?;
    c.emit(0x6ba0c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6ba10, transfer_word(C::Al, true, r(0), r(4), 0x2c)?)?;
    c.emit(0x6ba14, transfer_word(C::Al, true, r(0), r(0), 0x18)?)?;
    c.emit(0x6ba18, transfer_word(C::Al, false, r(0), r(13), 0x20)?)?;
    c.emit(0x6ba1c, mov_register(C::Al, r(0), r(6)))?;
    c.emit(0x6ba20, branch(C::Al, 0x6ba20, 0x31ec, true)?)?;
    c.emit(0x6ba24, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x234)?)?;
    c.emit(0x6ba28, mov_register(C::Al, r(2), r(9)))?;
    c.emit(0x6ba2c, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6ba30, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(2), r(0), Shift::Lsl, 0)?)?;
    c.emit(0x6ba34, transfer_word(C::Al, true, r(1), r(15), 0xff0)?)?;
    c.emit(0x6ba38, transfer_word(C::Al, false, r(2), r(13), 0x40)?)?;
    c.emit(0x6ba3c, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.emit(0x6ba40, transfer_word_register(C::Al, true, r(14), r(2), r(1), Shift::Lsl, 0)?)?;
    c.emit(0x6ba44, data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x258)?)?;
    c.emit(0x6ba48, data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6ba4c, transfer_word_register(C::Ne, true, r(1), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(0x6ba50, transfer_word(C::Ne, false, r(1), r(9), 0x5c)?)?;
    c.emit(0x6ba54, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x4)?)?;
    c.emit(0x6ba58, branch(C::Eq, 0x6ba58, 0x6ba88, false)?)?;
    c.emit(0x6ba5c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x260)?)?;
    c.emit(0x6ba60, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x264)?)?;
    c.emit(0x6ba64, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6ba68, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(0x6ba6c, transfer_word_register(C::Al, true, r(1), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(0x6ba70, transfer_word(C::Al, false, r(1), r(9), 0x64)?)?;
    c.emit(0x6ba74, transfer_word_register(C::Al, true, r(2), r(9), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x6ba78, data_register(C::Al, Op::Add, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6ba7c, transfer_word(C::Al, false, r(2), r(9), 0x68)?)?;
    c.emit(0x6ba80, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x6ba84, transfer_word(C::Al, false, r(1), r(9), 0x6c)?)?;
    c.emit(0x6ba88, transfer_word(C::Al, true, r(1), r(9), 0xc)?)?;
    c.emit(0x6ba8c, transfer_word(C::Al, true, r(12), r(11), -0x44)?)?;
    c.emit(0x6ba90, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x6ba94, transfer_word(C::Al, false, r(1), r(13), 0x28)?)?;
    c.emit(0x6ba98, branch(C::Eq, 0x6ba98, 0x6bc50, false)?)?;
    c.emit(0x6ba9c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6baa0, data_immediate(C::Al, Op::Tst, r(0), r(0), 0x20)?)?;
    c.emit(0x6baa4, transfer_word(C::Al, false, r(1), r(13), 0x24)?)?;
    c.emit(0x6baa8, mov_register(C::Al, r(1), r(0)))?;
    c.emit(0x6baac, branch(C::Eq, 0x6baac, 0x6bad4, false)?)?;
    c.emit(0x6bab0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x238)?)?;
    c.emit(0x6bab4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6bab8, transfer_word_register(C::Al, true, r(14), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(0x6babc, data_immediate(C::Al, Op::Bic, r(1), r(0), 0x20)?)?;
    c.emit(0x6bac0, transfer_word(C::Al, false, r(14), r(9), 0x34)?)?;
    c.emit(0x6bac4, transfer_word(C::Al, true, r(2), r(13), 0x40)?)?;
    c.emit(0x6bac8, transfer_word(C::Al, false, r(1), r(2), 0x0)?)?;
    c.emit(0x6bacc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x6bad0, transfer_word(C::Al, false, r(2), r(13), 0x24)?)?;
    c.emit(0x6bad4, data_immediate(C::Al, Op::Tst, r(0), r(1), 0x1)?)?;
    c.emit(0x6bad8, branch(C::Ne, 0x6bad8, 0x6bc60, false)?)?;
    c.emit(0x6badc, transfer_word(C::Al, true, r(4), r(9), 0x48)?)?;
    c.emit(0x6bae0, branch(C::Al, 0x6bae0, 0x6bcc0, false)?)?;
    c.emit(0x6bae4, transfer_word(C::Al, true, r(0), r(9), 0x0)?)?;
    c.emit(0x6bae8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.emit(0x6baec, transfer_byte(C::Eq, true, r(0), r(4), 0x328)?)?;
    c.emit(0x6baf0, data_immediate(C::Eq, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6baf4, branch(C::Ne, 0x6baf4, 0x6bba0, false)?)?;
    c.emit(0x6baf8, data_immediate(C::Al, Op::Add, r(8), r(9), 0x54)?)?;
    c.emit(0x6bafc, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x64)?)?;
    c.emit(0x6bb00, transfer_multiple(C::Al, true, r(8), r(0).bit() | r(1).bit() | r(8).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x6bb04, transfer_word(C::Al, true, r(2), r(9), 0x60)?)?;
    c.emit(0x6bb08, multiply_register(C::Al, r(4), r(0), r(1))?)?;
    c.emit(0x6bb0c, multiply_register(C::Al, r(7), r(2), r(3))?)?;
    c.emit(0x6bb10, data_register(C::Al, Op::Add, r(0), r(7), r(4), Shift::Lsr, 1, false)?)?;
    c.emit(0x6bb14, mov_register(C::Al, r(1), r(4)))?;
    c.emit(0x6bb18, branch_exchange_immediate(0x6bb18, 0x83c4c)?)?;
    c.emit(0x6bb1c, data_register(C::Al, Op::Add, r(1), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(0x6bb20, mov_register(C::Al, r(2), r(0)))?;
    c.emit(0x6bb24, mov_shifted_register(C::Al, r(0), r(8), Shift::Lsr, 2)?)?;
    c.emit(0x6bb28, mov_shifted_register(C::Al, r(3), r(1), Shift::Lsr, 1)?)?;
    c.emit(0x6bb2c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x3)?)?;
    c.emit(0x6bb30, multiply_register(C::Al, r(1), r(0), r(3))?)?;
    c.emit(0x6bb34, transfer_word(C::Al, true, r(0), r(15), 0xfe0)?)?;
    c.emit(0x6bb38, multiply_long(C::Al, false, r(3), r(7), r(1), r(0))?)?;
    c.emit(0x6bb3c, mov_shifted_register(C::Al, r(0), r(7), Shift::Lsr, 18)?)?;
    c.emit(0x6bb40, branch(C::Hi, 0x6bb40, 0x6bb4c, false)?)?;
    c.emit(0x6bb44, data_register(C::Al, Op::Cmp, r(0), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x6bb48, branch(C::Hi, 0x6bb48, 0x6bb58, false)?)?;
    c.emit(0x6bb4c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x4)?)?;
    c.emit(0x6bb50, data_register(C::Ls, Op::Cmp, r(0), r(1), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x6bb54, branch(C::Cs, 0x6bb54, 0x6c028, false)?)?;
    c.emit(0x6bb58, transfer_word(C::Al, true, r(4), r(11), 0x8)?)?;
    c.emit(0x6bb5c, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x6e)?)?;
    c.emit(0x6bb60, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x23)?)?;
    c.emit(0x6bb64, transfer_word(C::Al, true, r(1), r(4), 0xab8)?)?;
    c.emit(0x6bb68, transfer_word(C::Al, false, r(3), r(1), 0x350)?)?;
    c.emit(0x6bb6c, transfer_word(C::Al, false, r(7), r(1), 0x354)?)?;
    c.emit(0x6bb70, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x12)?)?;
    c.emit(0x6bb74, transfer_word(C::Al, false, r(7), r(1), 0x358)?)?;
    c.emit(0x6bb78, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6bb7c, transfer_word(C::Al, false, r(7), r(13), 0x0)?)?;
    c.emit(0x6bb80, transfer_word(C::Al, false, r(1), r(13), 0x4)?)?;
    c.emit(0x6bb84, transfer_word(C::Al, true, r(1), r(15), 0xfb0)?)?;
    c.emit(0x6bb88, transfer_word(C::Al, false, r(0), r(13), 0x8)?)?;
    c.emit(0x6bb8c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(0x6bb90, branch(C::Al, 0x6bb90, 0x2ec94, true)?)?;
    c.emit(0x6bb94, transfer_word(C::Al, true, r(1), r(15), 0xff4)?)?;
    c.emit(0x6bb98, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(0x6bb9c, branch(C::Al, 0x6bb9c, 0x2ec94, true)?)?;
    c.emit(0x6bba0, transfer_byte(C::Al, true, r(0), r(9), 0x89)?)?;
    c.emit(0x6bba4, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.emit(0x6bba8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(0x6bbac, branch(C::Ne, 0x6bbac, 0x6e8e8, false)?)?;
    c.emit(0x6bbb0, transfer_word(C::Al, true, r(0), r(9), 0x0)?)?;
    c.emit(0x6bbb4, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.emit(0x6bbb8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.emit(0x6bbbc, branch(C::Eq, 0x6bbbc, 0x6e8e8, false)?)?;
    c.emit(0x6bbc0, mov_register(C::Al, r(0), r(9)))?;
    c.emit(0x6bbc4, branch(C::Al, 0x6bbc4, 0x67900, true)?)?;
    c.emit(0x6bbc8, data_register(C::Al, Op::Add, r(1), r(9), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x6bbcc, transfer_byte(C::Al, false, r(0), r(9), 0x64c)?)?;
    c.emit(0x6bbd0, mov_register(C::Al, r(2), r(1)))?;
    c.emit(0x6bbd4, transfer_word_preindexed_writeback(C::Al, true, r(3), r(2), 0x224)?)?;
    c.emit(0x6bbd8, transfer_word(C::Al, false, r(3), r(9), 0x94)?)?;
    c.emit(0x6bbdc, transfer_byte(C::Al, false, r(0), r(9), 0x158)?)?;
    c.emit(0x6bbe0, transfer_byte(C::Al, true, r(0), r(9), 0x9d)?)?;
    c.emit(0x6bbe4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x6bbe8, transfer_word(C::Ne, true, r(0), r(2), 0x0)?)?;
    c.emit(0x6bbec, transfer_word(C::Ne, false, r(0), r(9), 0x94)?)?;
    c.emit(0x6bbf0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x6bbf4, transfer_word(C::Al, false, r(0), r(9), 0x658)?)?;
    c.emit(0x6bbf8, transfer_word(C::Al, false, r(0), r(9), 0x65c)?)?;
    c.emit(0x6bbfc, transfer_word(C::Al, true, r(2), r(1), 0x174)?)?;
    c.emit(0x6bc00, transfer_word(C::Al, false, r(2), r(9), 0x634)?)?;
    c.emit(0x6bc04, transfer_word(C::Al, true, r(2), r(1), 0x1cc)?)?;
    c.emit(0x6bc08, transfer_word(C::Al, false, r(2), r(9), 0x640)?)?;
    c.emit(0x6bc0c, transfer_word(C::Al, true, r(2), r(1), 0x174)?)?;
    c.emit(0x6bc10, transfer_word(C::Al, false, r(2), r(9), 0x638)?)?;
    c.emit(0x6bc14, transfer_word(C::Al, true, r(2), r(1), 0x1cc)?)?;
    c.emit(0x6bc18, transfer_word(C::Al, false, r(2), r(9), 0x644)?)?;
    c.emit(0x6bc1c, transfer_word(C::Al, true, r(2), r(1), 0x174)?)?;
    c.emit(0x6bc20, transfer_word(C::Al, false, r(2), r(9), 0x63c)?)?;
    c.emit(0x6bc24, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x18)?)?;
    c.emit(0x6bc28, transfer_word(C::Al, true, r(1), r(1), 0x1cc)?)?;
    c.emit(0x6bc2c, transfer_word(C::Al, false, r(1), r(9), 0x648)?)?;
    c.emit(0x6bc30, store_halfword(C::Al, r(0), r(9), 0xa0)?)?;
    c.emit(0x6bc34, store_halfword(C::Al, r(0), r(9), 0x9e)?)?;
    c.emit(0x6bc38, store_halfword(C::Al, r(0), r(9), 0xa2)?)?;
    c.emit(0x6bc3c, data_immediate(C::Al, Op::Add, r(0), r(9), 0x15c)?)?;
    c.emit(0x6bc40, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x6bc44, branch(C::Al, 0x6bc44, 0x31ec, true)?)?;
    c.emit(0x6bc48, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.emit(0x6bc4c, branch(C::Al, 0x6bc4c, 0x6e8e8, false)?)?;

    if c.at != END || c.bytes.len() != CODE_BYTES {
        return Err(format!("update entry extent {:#x}, {} bytes", c.at, c.bytes.len()));
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
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn exact_code_extent_and_checked_addresses() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 704);
        assert_eq!(END - START, CODE_BYTES);
        assert_eq!(DATA_BYTES, 0);
        for at in [START - 4, START + 1, END, 0x6c95c, 0x6c960] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, mov_register(C::Al, r(0), r(0))).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn external_pools_and_selected_pending_changes() {
        let bytes = fragment().unwrap();
        for (at, rd, target) in [
            (0x6b9cc, 5, 0x6c95c), (0x6b9d0, 6, 0x6c960),
            (0x6ba34, 1, 0x6ca2c), (0x6bb34, 0, 0x6cb1c),
            (0x6bb84, 1, 0x6cb3c), (0x6bb94, 1, 0x6cb90),
        ] {
            assert!(target >= END);
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(rd), r(15), (target - at - 8) as i32).unwrap());
        }
        assert_eq!(word(&bytes, 0x6ba30), transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(2), r(0), Shift::Lsl, 0).unwrap());
        for (at, mask) in [(0x6ba3c, 2), (0x6ba54, 4), (0x6baa0, 0x20)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Tst, r(0), r(0), mask).unwrap());
        }
        assert_eq!(word(&bytes, 0x6ba50), transfer_word(C::Ne, false, r(1), r(9), 0x5c).unwrap());
        assert_eq!(word(&bytes, 0x6bac8), transfer_word(C::Al, false, r(1), r(2), 0).unwrap());
    }

    #[test]
    fn conditional_flags_carry_and_interworking() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6baf0), data_immediate(C::Eq, Op::Cmp, r(0), r(0), 0).unwrap());
        assert_eq!(word(&bytes, 0x6bb18), branch_exchange_immediate(0x6bb18, 0x83c4c).unwrap());
        assert_eq!(word(&bytes, 0x6bb50), data_register(C::Ls, Op::Cmp, r(0), r(1), r(5), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x6bb54), branch(C::Cs, 0x6bb54, 0x6c028, false).unwrap());
        for at in [0x6bb08, 0x6bb0c, 0x6bb10, 0x6bb30, 0x6bb38, 0x6bb3c] {
            assert_eq!(word(&bytes, at) & (1 << 20), 0);
        }
        for (at, condition, target) in [
            (0x6b9dc, C::Eq, 0x6bae4), (0x6ba98, C::Eq, 0x6bc50),
            (0x6bad8, C::Ne, 0x6bc60), (0x6bae0, C::Al, 0x6bcc0),
            (0x6bbac, C::Ne, 0x6e8e8), (0x6bbbc, C::Eq, 0x6e8e8),
            (0x6bc4c, C::Al, 0x6e8e8),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
    }

    #[test]
    fn indexed_state_and_counter_clears() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6bbc8), data_register(C::Al, Op::Add, r(1), r(9), r(0), Shift::Lsl, 2, false).unwrap());
        assert_eq!(word(&bytes, 0x6bbd4), transfer_word_preindexed_writeback(C::Al, true, r(3), r(2), 0x224).unwrap());
        for (at, offset) in [(0x6bbf4, 0x658), (0x6bbf8, 0x65c)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, false, r(0), r(9), offset).unwrap());
        }
        for (at, offset) in [(0x6bc30, 0xa0), (0x6bc34, 0x9e), (0x6bc38, 0xa2)] {
            assert_eq!(word(&bytes, at), store_halfword(C::Al, r(0), r(9), offset).unwrap());
        }
    }
}
