use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register, mov_register,
    mov_shifted_register, multiply_long, transfer_byte, transfer_word,
    transfer_word_postindexed, transfer_word_preindexed_writeback, transfer_word_register,
    transfer_word_register_preindexed_writeback, Condition as C, DataOp as Op,
    Register as R, Shift,
};

pub const START: usize = 0x6bc50;
pub const END: usize = 0x6beb0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 608;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if self.at < START || self.at >= END || self.at % 4 != 0 {
            return Err(format!("allocation transition instruction outside extent: {:#x}", self.at));
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
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0x6bca0, false)?;
    c.emit(transfer_word(C::Al, true, r(4), r(9), 0x48)?)?;
    c.branch(C::Al, 0x6bcac, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(14), 0)?)?;
    c.branch(C::Eq, 0x6bcb8, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(9), 0x23c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(12), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_postindexed(C::Al, true, r(7), r(1), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(2), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(5), r(6), 4)?)?;
    c.emit(data_register(C::Al, Op::Add, r(4), r(7), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(7), r(2), 4)?)?;
    c.branch(C::Ne, 0x6bc7c, false)?;
    c.branch(C::Al, 0x6bcc0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x23c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(4), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x24)?)?;
    c.branch(C::Al, 0x6bcc0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(14), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0x21)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Ne, false, r(4), r(9), 0x48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x34)?)?;
    c.emit(mov_register(C::Ne, r(2), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x40)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x1c)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x25)?)?;
    c.emit(transfer_word(C::Al, false, r(14), r(11), -0x40)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x10)?)?;
    c.branch(C::Cc, 0x6bd50, false)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 9)?)?;
    c.branch(C::Eq, 0x6beb0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x6bda8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x254)?)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(data_register(C::Al, Op::Add, r(3), r(4), r(4), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(3), Shift::Lsr, 1, false)?)?;
    c.branch(C::Cs, 0x6bda8, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.branch(C::Ne, 0x6beb0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x23c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, false, r(4), r(9), r(2), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.branch(C::Al, 0x6beb0, false)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 0x11)?)?;
    c.branch(C::Eq, 0x6c12c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x70)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x6be1c, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x25c)?)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(15), 0x6cc20 - (0x6bd74 + 8))?)?;
    c.emit(multiply_long(C::Al, false, r(7), r(5), r(4), r(3))?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(5), Shift::Lsr, 3, false)?)?;
    c.branch(C::Cs, 0x6c0ac, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.branch(C::Ne, 0x6c12c, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x23c)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, false, r(3), r(9), r(2), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.branch(C::Al, 0x6c12c, false)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 8)?)?;
    c.branch(C::Eq, 0x6beb0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x254)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x758)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(9), 0x75c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x2bc)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, true, r(5), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(5), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(2), r(1), r(3), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(2), r(7), 0, true)?)?;
    c.branch(C::Cs, 0x6bea0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x5c)?)?;
    c.emit(mov_shifted_register(C::Al, r(2), r(0), Shift::Lsr, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x12c0)?)?;
    c.emit(transfer_word_register(C::Al, false, r(2), r(9), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(3), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r(1), r(7), 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 0x38)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(11), -0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2c4)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, false, r(0), r(9), r(1), Shift::Lsl, 0)?)?;
    c.branch(C::Al, 0x6bea8, false)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(6), 1)?)?;
    c.branch(C::Eq, 0x6c0ac, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x1c)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(4), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x6bfe0, false)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(9), 0x2c8)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(12), r(0), 0x760)?)?;
    c.emit(mov_shifted_register(C::Al, r(4), r(2), Shift::Lsr, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1000)?)?;
    c.emit(mov_register(C::Al, r(7), r(0)))?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), -4)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(3), r(7), -8)?)?;
    c.emit(data_register(C::Al, Op::Rsb, r(2), r(3), r(2), Shift::Lsr, 1, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Rsc, r(2), r(1), 0, true)?)?;
    c.branch(C::Cs, 0x6c048, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x5c)?)?;
    c.emit(mov_shifted_register(C::Al, r(2), r(0), Shift::Lsr, 3)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2cc)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, false, r(2), r(9), r(0), Shift::Lsl, 0)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(3), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sbc, r(1), r(1), 0)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2d0)?)?;
    c.emit(transfer_word(C::Al, true, r(14), r(11), -0x40)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(transfer_word_register(C::Al, false, r(0), r(9), r(1), Shift::Lsl, 0)?)?;
    c.branch(C::Al, 0x6c0a8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(9), 0x74)?)?;
    c.emit(transfer_word(C::Al, true, r(12), r(11), -0x44)?)?;
    if c.at != END || c.bytes.len() != CODE_BYTES {
        return Err(format!("allocation transition ended at {:#x}", c.at));
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
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn extent_and_checked_emission() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), CODE_BYTES);
        assert_eq!(CODE_BYTES, END - START);
        for at in [START - 4, START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn cross_boundary_branches_and_calls() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x6bd04, C::Eq, 0x6beb0, false),
            (0x6bd38, C::Ne, 0x6beb0, false),
            (0x6bd4c, C::Al, 0x6beb0, false),
            (0x6bd54, C::Eq, 0x6c12c, false),
            (0x6bd80, C::Cs, 0x6c0ac, false),
            (0x6bd8c, C::Ne, 0x6c12c, false),
            (0x6bda4, C::Al, 0x6c12c, false),
            (0x6bdac, C::Eq, 0x6beb0, false),
            (0x6be00, C::Al, 0x8187c, true),
            (0x6be20, C::Eq, 0x6c0ac, false),
            (0x6be2c, C::Cs, 0x6bfe0, false),
            (0x6be5c, C::Cs, 0x6c048, false),
            (0x6be88, C::Al, 0x8187c, true),
            (0x6be9c, C::Al, 0x6c0a8, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
    }

    #[test]
    fn carry_chains_and_conditional_context_update() {
        let bytes = fragment().unwrap();
        for (at, sets_flags) in [
            (0x6bccc, true), (0x6bcd0, false), (0x6bcd4, false), (0x6bcd8, false),
            (0x6bdd0, true), (0x6bdd4, true), (0x6bdf4, true), (0x6bdf8, false),
            (0x6bdfc, false), (0x6be54, true), (0x6be58, true),
            (0x6be7c, true), (0x6be80, false), (0x6be84, false),
        ] {
            assert_eq!(word(&bytes, at) & (1 << 20) != 0, sets_flags);
        }
        assert_eq!(word(&bytes, 0x6bcd0), transfer_word(C::Ne, false, r(4), r(9), 0x48).unwrap());
        assert_eq!(word(&bytes, 0x6bcd8), mov_register(C::Ne, r(2), r(1)));
    }


}
