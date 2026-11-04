use arm32::{
    branch, data_immediate, data_register, store_halfword_postindexed, transfer_multiple,
    transfer_word, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xaa0c;
pub const END: usize = 0xaba0;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn instruction(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if at != self.at || at % 4 != 0 || !(START..END).contains(&at) {
            return Err(format!("invalid heap bin classifier instruction address 0x{at:x}"));
        }
        self.bytes.extend(instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("constant A32 register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.instruction(0xaa0c, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0xaa10, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.instruction(0xaa14, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaa18, transfer_word(C::Al, true, r(0), r(1), 4)?)?;
    c.instruction(0xaa1c, branch(C::Al, 0xaa1c, 0xaa48, true)?)?;
    c.instruction(0xaa20, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.instruction(0xaa24, transfer_multiple(C::Lt, true, r(13), 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0xaa28, data_register(C::Al, Op::Add, r(1), r(4), r(0), Shift::Lsl, 4, false)?)?;
    c.instruction(0xaa2c, data_immediate(C::Al, Op::Add, r(2), r(0), 1)?)?;
    c.instruction(0xaa30, data_immediate(C::Al, Op::Add, r(1), r(1), 0x38)?)?;
    c.instruction(0xaa34, data_immediate(C::Al, Op::Sub, r(2), r(2), 1)?)?;
    c.instruction(0xaa38, store_halfword_postindexed(C::Al, r(0), r(1), -0x10)?)?;
    c.instruction(0xaa3c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.instruction(0xaa40, branch(C::Gt, 0xaa40, 0xaa34, false)?)?;
    c.instruction(0xaa44, transfer_multiple(C::Al, true, r(13), 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0xaa48, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0xaa4c, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.instruction(0xaa50, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.instruction(0xaa54, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x10)?)?;
    c.instruction(0xaa58, data_immediate(C::Al, Op::Mov, r(2), r(0), 3)?)?;
    c.instruction(0xaa5c, data_immediate(C::Al, Op::Mov, r(3), r(0), 5)?)?;
    c.instruction(0xaa60, data_immediate(C::Al, Op::Mov, r(12), r(0), 0xf)?)?;
    c.instruction(0xaa64, data_immediate(C::Al, Op::Mov, r(14), r(0), 1)?)?;
    c.instruction(0xaa68, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1f)?)?;
    c.instruction(0xaa6c, data_immediate(C::Hi, Op::Mov, r(1), r(0), 1)?)?;
    c.instruction(0xaa70, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x31)?)?;
    c.instruction(0xaa74, data_immediate(C::Al, Op::Orr, r(14), r(14), 0x10000)?)?;
    c.instruction(0xaa78, data_immediate(C::Cc, Op::Mov, r(2), r(0), 2)?)?;
    c.instruction(0xaa7c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x21)?)?;
    c.instruction(0xaa80, data_register(C::Cc, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaa84, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x61)?)?;
    c.instruction(0xaa88, data_immediate(C::Al, Op::Mov, r(1), r(0), 7)?)?;
    c.instruction(0xaa8c, data_immediate(C::Cc, Op::Mov, r(3), r(0), 4)?)?;
    c.instruction(0xaa90, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x91)?)?;
    c.instruction(0xaa94, data_immediate(C::Cc, Op::Mov, r(1), r(0), 6)?)?;
    c.instruction(0xaa98, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x81)?)?;
    c.instruction(0xaa9c, data_register(C::Cc, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaaa0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x51)?)?;
    c.instruction(0xaaa4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0xb)?)?;
    c.instruction(0xaaa8, data_register(C::Cc, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaaac, data_immediate(C::Al, Op::Mov, r(2), r(0), 9)?)?;
    c.instruction(0xaab0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xe1)?)?;
    c.instruction(0xaab4, data_immediate(C::Cc, Op::Mov, r(2), r(0), 8)?)?;
    c.instruction(0xaab8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x120)?)?;
    c.instruction(0xaabc, data_immediate(C::Ls, Op::Mov, r(3), r(0), 0xa)?)?;
    c.instruction(0xaac0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x100)?)?;
    c.instruction(0xaac4, data_register(C::Ls, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaac8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xd)?)?;
    c.instruction(0xaacc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x180)?)?;
    c.instruction(0xaad0, data_immediate(C::Ls, Op::Mov, r(2), r(0), 0xc)?)?;
    c.instruction(0xaad4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x250)?)?;
    c.instruction(0xaad8, data_immediate(C::Ls, Op::Mov, r(12), r(0), 0xe)?)?;
    c.instruction(0xaadc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x200)?)?;
    c.instruction(0xaae0, data_register(C::Ls, Op::Mov, r(12), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaae4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x150)?)?;
    c.instruction(0xaae8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x13)?)?;
    c.instruction(0xaaec, data_register(C::Ls, Op::Mov, r(12), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaaf0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xc1)?)?;
    c.instruction(0xaaf4, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x17)?)?;
    c.instruction(0xaaf8, data_register(C::Cc, Op::Mov, r(12), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xaafc, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11)?)?;
    c.instruction(0xab00, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x400)?)?;
    c.instruction(0xab04, data_immediate(C::Ls, Op::Mov, r(1), r(0), 0x10)?)?;
    c.instruction(0xab08, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x800)?)?;
    c.instruction(0xab0c, data_immediate(C::Ls, Op::Mov, r(2), r(0), 0x12)?)?;
    c.instruction(0xab10, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x600)?)?;
    c.instruction(0xab14, data_register(C::Ls, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab18, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x15)?)?;
    c.instruction(0xab1c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.instruction(0xab20, data_immediate(C::Ls, Op::Mov, r(1), r(0), 0x14)?)?;
    c.instruction(0xab24, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.instruction(0xab28, data_immediate(C::Ls, Op::Mov, r(3), r(0), 0x16)?)?;
    c.instruction(0xab2c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1800)?)?;
    c.instruction(0xab30, data_register(C::Ls, Op::Mov, r(3), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab34, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xc00)?)?;
    c.instruction(0xab38, data_immediate(C::Al, Op::Add, r(1), r(14), 0x8000)?)?;
    c.instruction(0xab3c, data_register(C::Ls, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab40, data_immediate(C::Al, Op::Add, r(2), r(14), 0x10000)?)?;
    c.instruction(0xab44, data_register(C::Al, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab48, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1d)?)?;
    c.instruction(0xab4c, data_immediate(C::Cc, Op::Mov, r(4), r(0), 0x1e)?)?;
    c.instruction(0xab50, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x10000)?)?;
    c.instruction(0xab54, data_immediate(C::Ls, Op::Mov, r(2), r(0), 0x1c)?)?;
    c.instruction(0xab58, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab5c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1b)?)?;
    c.instruction(0xab60, data_register(C::Cs, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab64, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x19)?)?;
    c.instruction(0xab68, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.instruction(0xab6c, data_immediate(C::Ls, Op::Mov, r(4), r(0), 0x18)?)?;
    c.instruction(0xab70, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x8000)?)?;
    c.instruction(0xab74, data_immediate(C::Ls, Op::Mov, r(1), r(0), 0x1a)?)?;
    c.instruction(0xab78, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x6000)?)?;
    c.instruction(0xab7c, data_register(C::Ls, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab80, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xc000)?)?;
    c.instruction(0xab84, data_register(C::Hi, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab88, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3000)?)?;
    c.instruction(0xab8c, data_register(C::Ls, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab90, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x300)?)?;
    c.instruction(0xab94, data_register(C::Ls, Op::Mov, r(1), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab98, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0xab9c, transfer_multiple(C::Al, true, r(13), 0x8890, BlockMode::IncrementAfter, true)?)?;
    if c.at != END || c.bytes.len() != END - START {
        return Err("invalid heap bin classifier extent".into());
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
    fn exact_extent_and_instruction_addresses() {
        assert_eq!(fragment().unwrap().len(), 404);
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.instruction(START + 4, 0).is_err());
        assert!(c.instruction(START + 1, 0).is_err());
        c.instruction(START, data_immediate(C::Al, Op::Mov, r(0), r(0), 0).unwrap()).unwrap();
        assert!(c.instruction(START, 0).is_err());
        let mut end = Code { at: END, bytes: Vec::new() };
        assert!(end.instruction(END, 0).is_err());
    }

    #[test]
    fn backward_halfword_bin_hints() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0xaa1c), branch(C::Al, 0xaa1c, 0xaa48, true).unwrap());
        assert_eq!(word(&bytes, 0xaa24), transfer_multiple(C::Lt, true, r(13), 0x8890, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0xaa28), data_register(C::Al, Op::Add, r(1), r(4), r(0), Shift::Lsl, 4, false).unwrap());
        assert_eq!(word(&bytes, 0xaa30), data_immediate(C::Al, Op::Add, r(1), r(1), 0x38).unwrap());
        assert_eq!(word(&bytes, 0xaa38), store_halfword_postindexed(C::Al, r(0), r(1), -0x10).unwrap());
        assert_eq!(word(&bytes, 0xaa40), branch(C::Gt, 0xaa40, 0xaa34, false).unwrap());
    }

    #[test]
    fn classifier_bin_extremes_and_return() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0xaa50), data_immediate(C::Al, Op::Mov, r(1), r(0), 0).unwrap());
        assert_eq!(word(&bytes, 0xaa68), data_immediate(C::Al, Op::Mov, r(4), r(0), 31).unwrap());
        assert_eq!(word(&bytes, 0xab4c), data_immediate(C::Cc, Op::Mov, r(4), r(0), 30).unwrap());
        assert_eq!(word(&bytes, 0xab98), data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0xab9c), transfer_multiple(C::Al, true, r(13), 0x8890, BlockMode::IncrementAfter, true).unwrap());
    }

    #[test]
    fn optional_pinned_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }
}
