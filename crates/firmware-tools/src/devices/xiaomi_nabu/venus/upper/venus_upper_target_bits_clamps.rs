use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register, multiply_accumulate,
    multiply_long, multiply_register, transfer_byte, transfer_doubleword, transfer_multiple,
    transfer_word, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6e098;
pub const END: usize = 0x6e348;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, word: u32) -> Result<(), String> {
        if self.at < START || self.at >= END || self.at % 4 != 0 {
            return Err(format!("invalid target bits clamps A32 address 0x{:x}", self.at));
        }
        self.bytes.extend(word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("constant A32 register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(transfer_byte(C::Al, true, r(4), r(9), 37)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 2)?)?;
    c.branch(C::Eq, 0x6e0cc, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 1)?)?;
    c.branch(C::Eq, 0x6e104, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.branch(C::Ne, 0x6e134, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0)?)?;
    c.branch(C::Eq, 0x6e174, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 1)?)?;
    c.branch(C::Ne, 0x6e25c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.branch(C::Al, 0x6e164, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0)?)?;
    c.branch(C::Eq, 0x6e194, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 96)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 64)?)?;
    c.branch(C::Cc, 0x6e1e4, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x6e25c, false)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(10), 0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(1), r(2), 1, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(3), 0, true)?)?;
    c.emit(data_register(C::Ge, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6e25c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0)?)?;
    c.branch(C::Eq, 0x6e1c0, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 1)?)?;
    c.branch(C::Ne, 0x6e25c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 61)?)?;
    c.branch(C::Cc, 0x6e164, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.branch(C::Al, 0x6e25c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 100)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0x6e14c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x6e254, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0)?)?;
    c.branch(C::Ne, 0x6e25c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6e288, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x6e25c, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6e25c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 61)?)?;
    c.branch(C::Cc, 0x6e244, false)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsr, 1, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.branch(C::Al, 0x6e288, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 120)?)?;
    c.branch(C::Cc, 0x6e200, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 1, false)?)?;
    c.branch(C::Ls, 0x6e288, false)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(10), 0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(1), r(2), 1, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(3), 0, true)?)?;
    c.branch(C::Ge, 0x6e23c, false)?;
    c.branch(C::Al, 0x6e288, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 92)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 61)?)?;
    c.branch(C::Cc, 0x6e228, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 132)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 255)?)?;
    c.branch(C::Eq, 0x6e288, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 96)?)?;
    c.emit(multiply_register(C::Al, r(0), r(2), r(1))?)?;
    c.branch(C::Al, 0x6e244, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(C::Ls, 0x6e25c, false)?;
    c.emit(transfer_multiple(C::Al, true, r(10), r(0).bit() | r(3).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 1, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(3), 0, true)?)?;
    c.emit(data_register(C::Ge, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6e25c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 64)?)?;
    c.branch(C::Cc, 0x6f134, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 2, false)?)?;
    c.branch(C::Ls, 0x6e288, false)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(10), 0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(1), r(2), 1, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(3), 0, true)?)?;
    c.emit(data_register(C::Ge, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.branch(C::Al, 0x6e288, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 41)?)?;
    c.branch(C::Cc, 0x6e244, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 1, false)?)?;
    c.branch(C::Ls, 0x6e288, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.branch(C::Al, 0x6e288, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x6e288, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6e288, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0)?)?;
    c.branch(C::Eq, 0x6e288, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 772)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 92)?)?;
    c.emit(multiply_long(C::Al, false, r(1), r(2), r(0), r(7))?)?;
    c.emit(multiply_long(C::Al, false, r(0), r(7), r(1), r(3))?)?;
    c.emit(multiply_accumulate(C::Al, r(1), r(2), r(3), r(7))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 576)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(2), r(2), 999424)?)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 2, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(0), r(1), Shift::Lsl, 30, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 96)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 5)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 3, false)?)?;
    c.branch(C::Eq, 0x6e2b8, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x6e2b8, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 137)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.branch(C::Ne, 0x6e2b8, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 4096)?)?;
    c.branch(C::Ne, 0x6e2d0, false)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Cs, 0x6e2d4, false)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 137)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(data_register(C::Eq, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x6e2d4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 1888)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 1892)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 7)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Cc, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(multiply_long(C::Al, false, r(6), r(5), r(1), r(3))?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(5), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsr, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 29, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Hi, Op::Mov, r(7), r(0), r(6), Shift::Lsr, 3, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 137)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0x6e328, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 100)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Ne, 0x6e328, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 104)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Eq, true, r(7), r(9), 96)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 3992)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(9), 740)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(15), 3976)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    if c.at != END {
        return Err(format!("target bits clamps ended at 0x{:x}, expected 0x{END:x}", c.at));
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
    fn extent_dispatch_and_commit() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 688);
        assert_eq!(bytes.len(), END - START);
        assert_eq!(word(&bytes, START), transfer_byte(C::Al, true, r(4), r(9), 0x25).unwrap());
        assert_eq!(word(&bytes, 0x6e0b4), data_immediate(C::Al, Op::Cmp, r(0), r(8), 0).unwrap());
        assert_eq!(word(&bytes, 0x6e288), transfer_word(C::Al, true, r(0), r(9), 0x60).unwrap());
        assert_eq!(word(&bytes, 0x6e334), transfer_word(C::Al, false, r(7), r(9), 0x2e4).unwrap());
        assert_eq!(word(&bytes, 0x6e328), transfer_word(C::Al, true, r(1), R::PC, 0x6f2c8 - (0x6e328 + 8)).unwrap());
        assert_eq!(word(&bytes, 0x6e33c), transfer_word(C::Al, true, r(1), R::PC, 0x6f2cc - (0x6e33c + 8)).unwrap());
    }

    #[test]
    fn signed_wide_thresholds_preserve_borrow_and_flags() {
        let bytes = fragment().unwrap();
        for at in [0x6e0f4, 0x6e1b0, 0x6e218] {
            assert_eq!(word(&bytes, at), data_immediate_with_flags(C::Al, Op::Sub, r(1), r(2), 1, true).unwrap());
            assert_eq!(word(&bytes, at + 4), data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(3), 0, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x6e1f0), data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 1, true).unwrap());
        assert_eq!(word(&bytes, 0x6e1f4), data_immediate_with_flags(C::Al, Op::Sbc, r(0), r(3), 0, true).unwrap());
        for (at, condition, target) in [
            (0x6e1b8, C::Ge, 0x6e23c),
            (0x6e204, C::Cc, 0x6f134),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x6e0fc), data_register(C::Ge, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x6e1f8), data_register(C::Ge, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 1, false).unwrap());
        assert_eq!(word(&bytes, 0x6e220), data_register(C::Ge, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 2, false).unwrap());
    }

    #[test]
    fn absolute_control_flow() {
        let bytes = fragment().unwrap();
        let mut external = Vec::new();
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 != 0x0a00_0000 {
                continue;
            }
            let offset = ((instruction << 8) as i32 >> 8) * 4;
            let target = (at as i32 + 8 + offset) as usize;
            let link = instruction & 0x0100_0000 != 0;
            if !(START..END).contains(&target) {
                external.push((at, target, link));
            } else {
                assert!(!link);
                assert_eq!((target - START) % 4, 0);
            }
        }
        assert_eq!(external, [
            (0x6e204, 0x6f134, false),
            (0x6e27c, 0x8187c, true),
            (0x6e338, 0x2ec94, true),
            (0x6e344, 0x2ec94, true),
        ]);
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
