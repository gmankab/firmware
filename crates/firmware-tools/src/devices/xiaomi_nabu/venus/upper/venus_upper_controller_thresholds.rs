use arm32::{
    Condition as C, DataOp as Op, Register as R, Shift, branch, data_immediate, data_register,
    mov_register, mov_shifted_register, multiply_accumulate, multiply_long, transfer_byte,
    transfer_doubleword, transfer_word,
};

pub const START: usize = 0x69498;
pub const END: usize = 0x697a0;
pub const FILE_BASE: usize = 0x4000;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn new() -> Self {
        Self {
            at: START,
            bytes: Vec::with_capacity(END - START),
        }
    }

    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at == expected {
            Ok(())
        } else {
            Err(format!(
                "controller thresholds 0x{:x} != 0x{expected:x}",
                self.at
            ))
        }
    }

    fn emit(&mut self, word: u32) -> Result<(), String> {
        if self.at < START || self.at >= END || self.at % 4 != 0 {
            return Err(format!("invalid partial A32 address 0x{:x}", self.at));
        }
        self.bytes.extend(word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn r(n: u8) -> R {
    R::new(n).unwrap()
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new();
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(7), 0x778)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(7), 0x77c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(7), 0x278)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(7), 0xc)?)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), R::SP, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0)?)?;
    c.branch(C::Eq, 0x69630, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(7), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.branch(C::Eq, 0x696f4, false)?;
    c.emit(transfer_word(C::Al, true, R::LR, r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), R::LR, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(3), R::LR, 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), R::LR, 0x1c8)?)?;
    c.emit(transfer_word(C::Al, false, R::LR, R::SP, 0x38)?)?;
    c.emit(multiply_long(C::Al, false, r(6), r(7), r(3), r(1))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 13)?)?;
    c.emit(multiply_long(C::Al, false, r(3), r(5), r(6), r(1))?)?;
    c.emit(multiply_accumulate(C::Al, r(12), r(7), r(1), r(5))?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(3), Shift::Lsr, 4)?)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(3),
        r(1),
        r(12),
        Shift::Lsl,
        28,
        false,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 3)?)?;
    c.emit(mov_shifted_register(C::Al, r(10), r(12), Shift::Lsr, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), R::R0, 0)?)?;
    c.emit(multiply_long(C::Al, false, r(5), r(4), r(6), r(1))?)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(1),
        r(7),
        r(7),
        Shift::Lsl,
        1,
        false,
    )?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::SP, 0x20)?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(7), Shift::Lsl, 1)?)?;
    c.emit(transfer_word(C::Al, false, r(4), R::SP, 0x2c)?)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 0x30)?)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(1),
        r(1),
        r(6),
        Shift::Lsr,
        31,
        false,
    )?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::SP, 0x1c)?)?;
    for (offset, slot) in [(0x110, 0x18), (0x108, 0x14), (0x100, 0x10)] {
        c.emit(data_immediate(C::Al, Op::Add, r(1), R::LR, offset)?)?;
        c.emit(transfer_word(C::Al, false, r(1), R::SP, slot)?)?;
    }
    c.emit(data_immediate(C::Al, Op::Add, r(1), R::LR, 0xf8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, R::LR, R::R0, 7)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::SP, 0xc)?)?;
    c.check(0x6954c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(9), 0)?)?;
    c.branch(C::Eq, 0x695b0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 2)?)?;
    c.emit(mov_register(C::Al, r(4), r(3)))?;
    c.emit(mov_register(C::Al, r(5), r(10)))?;
    for (reg, offset) in [
        (12, -0xd0),
        (12, -0xcc),
        (12, -0xc8),
        (12, -0xc4),
        (6, -0xc0),
        (7, -0xbc),
        (6, -0xb8),
        (7, -0xb4),
        (10, -0xac),
        (3, -0xb0),
        (10, -0xa4),
        (3, -0xa8),
    ] {
        c.emit(transfer_word(C::Al, false, r(reg), r(8), offset)?)?;
    }
    c.emit(transfer_byte(C::Al, false, R::LR, r(8), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(8), -0x9c)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(8), -0xa0)?)?;
    c.branch(C::Eq, 0x69600, false)?;
    c.branch(C::Al, 0x69608, false)?;
    c.check(0x695a8)?;
    c.emit(0x000f2788)?;
    c.emit(0x000f27b8)?;
    c.check(0x695b0)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x3c)?)?;
    c.emit(transfer_word(C::Al, true, r(5), R::SP, 0x1c)?)?;
    c.emit(transfer_word(C::Al, true, r(4), R::SP, 0x20)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 1)?)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x2c)?)?;
    c.emit(data_register(
        C::Hi,
        Op::Add,
        r(5),
        r(1),
        r(4),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(transfer_word(C::Al, true, r(4), R::SP, 0x30)?)?;
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0xc)?)?;
    c.emit(mov_shifted_register(C::Ls, r(4), r(6), Shift::Lsl, 1)?)?;
    c.emit(transfer_doubleword(C::Al, false, r(4), r(1), 0)?)?;
    for (slot, reg) in [(0x10, 4), (0x14, 6), (0x18, 6)] {
        c.emit(transfer_word(C::Al, true, r(1), R::SP, slot)?)?;
        c.emit(transfer_doubleword(C::Al, false, r(reg), r(1), 0)?)?;
    }
    c.emit(transfer_word(C::Al, true, r(1), R::SP, 0x38)?)?;
    c.emit(transfer_byte(C::Al, false, R::LR, r(1), 0x1c8)?)?;
    c.emit(mov_register(C::Al, r(1), r(2)))?;
    c.check(0x69600)?;
    c.emit(mov_register(C::Al, r(2), r(1)))?;
    c.check(0x69604)?;
    c.emit(transfer_doubleword(C::Al, false, r(4), r(8), -0xa0)?)?;
    c.check(0x69608)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(9), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0xd8)?)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(9),
        r(0),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Ne, 0x6954c, false)?;
    c.branch(C::Al, 0x696f4, false)?;
    c.check(0x6961c)?;
    for pointer in [0x000f27a0, 0x000f27be, 0x000f27c4, 0x000f27ca, 0x000f27d0] {
        c.emit(pointer)?;
    }
    c.check(0x69630)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(7), 0x27)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(7), 0x60)?)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(7), 0x25)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), R::R0, 13)?)?;
    c.emit(multiply_long(C::Al, false, r(2), r(1), r(3), r(6))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), R::R0, 3)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 1)?)?;
    c.emit(multiply_long(C::Al, false, r(8), r(6), r(2), r(3))?)?;
    c.emit(mov_shifted_register(C::Al, r(3), r(1), Shift::Lsl, 1)?)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(5),
        r(1),
        r(1),
        Shift::Lsl,
        1,
        false,
    )?)?;
    c.emit(mov_shifted_register(C::Ls, r(8), r(2), Shift::Lsl, 1)?)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(3),
        r(3),
        r(2),
        Shift::Lsr,
        31,
        false,
    )?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(4), 1)?)?;
    c.emit(data_register(
        C::Hi,
        Op::Add,
        r(3),
        r(6),
        r(5),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(multiply_long(C::Al, false, r(6), r(5), r(2), r(12))?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0)?)?;
    c.emit(multiply_accumulate(C::Al, r(4), r(1), r(12), r(5))?)?;
    c.emit(mov_shifted_register(C::Al, r(6), r(6), Shift::Lsr, 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 7)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    for (reg, offset) in [
        (8, 0x680),
        (3, 0x684),
        (2, 0x688),
        (1, 0x68c),
        (2, 0x690),
        (1, 0x694),
    ] {
        c.emit(transfer_word(C::Al, false, r(reg), r(7), offset)?)?;
    }
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(6),
        r(6),
        r(4),
        Shift::Lsl,
        28,
        false,
    )?)?;
    c.emit(mov_shifted_register(C::Al, r(12), r(4), Shift::Lsr, 4)?)?;
    for (reg, offset) in [(6, 0x698), (12, 0x69c), (6, 0x6a0), (12, 0x6a4)] {
        c.emit(transfer_word(C::Al, false, r(reg), r(7), offset)?)?;
    }
    c.emit(transfer_byte(C::Al, false, r(5), r(7), 0x748)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(7), 0x2d8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(5), 2)?)?;
    c.branch(C::Eq, 0x696dc, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(5), 1)?)?;
    c.branch(C::Eq, 0x696ec, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(5), 0)?)?;
    c.emit(transfer_word(C::Eq, true, r(1), R::SP, 0x3c)?)?;
    c.emit(transfer_word(C::Eq, false, r(8), r(1), 0x6a8)?)?;
    c.emit(transfer_word(C::Eq, false, r(3), r(1), 0x6ac)?)?;
    c.branch(C::Al, 0x696f4, false)?;
    c.check(0x696dc)?;
    c.emit(transfer_word(C::Al, false, r(6), r(7), 0x6a8)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(7), 0x6ac)?)?;
    c.branch(C::Al, 0x696f4, false)?;
    c.check(0x696e8)?;
    c.emit((-1000i32) as u32)?;
    c.check(0x696ec)?;
    c.emit(transfer_word(C::Al, false, r(2), r(7), 0x6a8)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(7), 0x6ac)?)?;
    c.check(0x696f4)?;
    c.emit(transfer_word(C::Al, true, r(8), R::SP, 0x3c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x6b0)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x2f8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x26)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x400)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x688)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(8), 0x5cc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x14)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0x5c8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, false, r(9), r(8), 0x5d4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0x5d0)?)?;
    c.emit(transfer_byte(C::Al, false, r(9), r(8), 0xa84)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(8), 0x27)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(8), 0xa86)?)?;
    c.emit(transfer_byte(C::Al, true, r(10), r(8), 0x2c)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0xa90)?)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(4),
        r(10),
        r(7),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(4),
        r(0),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(mov_register(C::Hi, r(4), r(0)))?;
    c.emit(transfer_byte(C::Al, false, r(4), r(8), 0xa8d)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(8), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0xa8f)?)?;
    c.emit(data_register(
        C::Al,
        Op::Sub,
        r(5),
        r(7),
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(5),
        r(0),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(mov_register(C::Lt, r(5), r(0)))?;
    for (reg, offset) in [
        (5, 0xa8b),
        (4, 0xa8e),
        (5, 0xa8c),
        (5, 0xa89),
        (4, 0xa8a),
        (0, 0xa91),
    ] {
        c.emit(transfer_byte(C::Al, false, r(reg), r(8), offset)?)?;
    }
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(8), 0x2f4)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x800)?)?;
    c.branch(C::Al, 0x31ec, true)?;
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
    fn fixed_runtime_extent() {
        assert_eq!(START, 0x69498);
        assert_eq!(END, 0x697a0);
        assert_eq!(fragment().unwrap().len(), 776);
        assert_eq!(END - START - 32, 744);
    }

    #[test]
    fn literal_pools_preserved() {
        let bytes = fragment().unwrap();
        for (at, value) in [
            (0x695a8, 0xf2788),
            (0x695ac, 0xf27b8),
            (0x6961c, 0xf27a0),
            (0x69620, 0xf27be),
            (0x69624, 0xf27c4),
            (0x69628, 0xf27ca),
            (0x6962c, 0xf27d0),
            (0x696e8, (-1000i32) as u32),
        ] {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn negative_pair_store_and_clear_calls() {
        let bytes = fragment().unwrap();
        assert_eq!(
            word(&bytes, 0x69604),
            transfer_doubleword(C::Al, false, r(4), r(8), -0xa0).unwrap()
        );
        for at in [0x69708, 0x6971c, 0x6979c] {
            assert_eq!(
                word(&bytes, at),
                branch(C::Al, at as u32, 0x31ec, true).unwrap()
            );
        }
    }

    #[test]
    fn fixed_writer_rejects_invalid_addresses() {
        for at in [START - 4, START + 1, END] {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(c.emit(mov_register(C::Al, R::R0, R::R0)).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        assert!(Code::new().check(END).is_err());
    }

    #[test]
    fn exact_pinned_runtime_bytes() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE"))
        else {
            return;
        };
        let source = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(source.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&source), identity.sha256);
        assert_eq!(
            fragment().unwrap(),
            source[START + FILE_BASE..END + FILE_BASE]
        );
    }
}
