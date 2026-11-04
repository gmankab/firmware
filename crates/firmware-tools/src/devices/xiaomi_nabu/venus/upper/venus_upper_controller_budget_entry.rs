use arm32::{
    Condition as C, DataOp as Op, Register as R, Shift, branch, data_immediate, data_register,
    mov_register, mov_shifted_register, multiply_register, transfer_byte,
    transfer_byte_postindexed, transfer_word,
};

pub const START: usize = 0x690a4;
pub const END: usize = 0x692a0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 432;
pub const DATA_BYTES: usize = 76;

const DIVIDE_BY_100_MAGIC: u32 = 0x51eb851f;
const TABLE_SOURCE_F1CE0: u32 = 0x000f1ce0;
const TABLE_DESTINATION_107918: u32 = 0x00107918;
const TABLE_DESTINATION_107C88: u32 = 0x00107c88;
const TABLE_DESTINATION_107FF8: u32 = 0x00107ff8;
const TABLE_DESTINATION_108368: u32 = 0x00108368;
const TABLE_SOURCE_F2050: u32 = 0x000f2050;
const TABLE_DESTINATION_107760: u32 = 0x00107760;
const TABLE_SOURCE_F2208: u32 = 0x000f2208;
const TABLE_DESTINATION_108520: u32 = 0x00108520;
const TABLE_SOURCE_F23C0: u32 = 0x000f23c0;
const TIER_SOURCE_F2578: u32 = 0x000f2578;
const TIER_SOURCE_F25D0: u32 = 0x000f25d0;
const TIER_SOURCE_F2628: u32 = 0x000f2628;
const TIER_SOURCE_F2680: u32 = 0x000f2680;
const TIER_SOURCE_F26D8: u32 = 0x000f26d8;
const TIER_SOURCE_F2730: u32 = 0x000f2730;
const PSNR_DIAGNOSTIC_STRING: u32 = 0x000f2e89;
const NEWLINE_STRING: u32 = 0x000e2788;

const DATA: &[(usize, &[u32])] = &[
    (0x690a4, &[DIVIDE_BY_100_MAGIC]),
    (0x690c8, &[TABLE_SOURCE_F1CE0, TABLE_DESTINATION_107918]),
    (
        0x690f8,
        &[
            TABLE_DESTINATION_107C88,
            TABLE_DESTINATION_107FF8,
            TABLE_DESTINATION_108368,
            TABLE_SOURCE_F2050,
            TABLE_DESTINATION_107760,
            TABLE_SOURCE_F2208,
            TABLE_DESTINATION_108520,
            TABLE_SOURCE_F23C0,
            TIER_SOURCE_F2578,
            TIER_SOURCE_F25D0,
            TIER_SOURCE_F2628,
            TIER_SOURCE_F2680,
            TIER_SOURCE_F26D8,
            TIER_SOURCE_F2730,
        ],
    ),
    (0x69284, &[PSNR_DIAGNOSTIC_STRING, NEWLINE_STRING]),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

fn is_data(at: usize) -> bool {
    DATA.iter()
        .any(|(start, words)| (*start..*start + words.len() * 4).contains(&at))
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!(
                "budget entry address {:#x}, expected {expected:#x}",
                self.at
            ));
        }
        Ok(())
    }

    fn word(&mut self, word: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("budget entry output address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if is_data(self.at) {
            return Err(format!("budget entry data address {:#x}", self.at));
        }
        self.word(instruction)
    }

    fn data(&mut self, address: usize) -> Result<(), String> {
        self.check(address)?;
        let words = DATA
            .iter()
            .find(|(start, _)| *start == address)
            .ok_or_else(|| format!("unknown budget entry data island {address:#x}"))?
            .1;
        for &word in words {
            self.word(word)?;
        }
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, false)?)
    }

    fn word_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn byte_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn immediate(
        &mut self,
        condition: C,
        op: Op,
        rd: u8,
        rn: u8,
        value: u32,
    ) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn compare(&mut self, left: u8, right: u8) -> Result<(), String> {
        self.emit(data_register(
            C::Al,
            Op::Cmp,
            r(0),
            r(left),
            r(right),
            Shift::Lsl,
            0,
            false,
        )?)
    }

    fn mov(&mut self, condition: C, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(condition, r(rd), r(rm)))
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(END - START),
    };
    c.data(0x690a4)?;
    c.byte_transfer(true, 1, 4, 0x28)?;
    c.word_transfer(true, 3, 13, 0x20)?;
    c.compare(1, 3)?;
    c.branch(C::Cc, 0x6907c)?;
    c.word_transfer(true, 3, 13, 0x18)?;
    c.compare(1, 3)?;
    c.branch(C::Ls, 0x690e0)?;
    c.branch(C::Al, 0x6907c)?;
    c.data(0x690c8)?;
    c.immediate(C::Al, Op::Cmp, 0, 3, 2)?;
    c.branch(C::Eq, 0x690e0)?;
    c.immediate(C::Al, Op::Add, 1, 0, 4)?;
    c.byte_transfer(false, 1, 4, 0x28)?;
    c.check(0x690e0)?;
    c.immediate(C::Al, Op::Tst, 0, 2, 4)?;
    c.branch(C::Ne, 0x69130)?;
    c.check(0x690e8)?;
    c.immediate(C::Al, Op::And, 2, 1, 0xff)?;
    c.immediate(C::Al, Op::Add, 2, 2, 2)?;
    c.byte_transfer(false, 2, 4, 0x29)?;
    c.branch(C::Al, 0x6914c)?;
    c.data(0x690f8)?;
    c.check(0x69130)?;
    c.byte_transfer(true, 2, 4, 0x29)?;
    c.word_transfer(true, 3, 13, 0x20)?;
    c.compare(2, 3)?;
    c.branch(C::Cc, 0x690e8)?;
    c.word_transfer(true, 3, 13, 0x18)?;
    c.compare(2, 3)?;
    c.branch(C::Hi, 0x690e8)?;
    c.check(0x6914c)?;
    c.byte_transfer(true, 3, 4, 0xd1f)?;
    c.immediate(C::Al, Op::And, 7, 1, 0xff)?;
    c.compare(7, 3)?;
    c.branch(C::Cc, 0x6916c)?;
    c.byte_transfer(true, 3, 4, 0xd20)?;
    c.compare(7, 3)?;
    c.mov(C::Hi, 1, 3)?;
    c.mov(C::Al, 3, 1)?;
    c.check(0x6916c)?;
    c.byte_transfer(false, 3, 4, 0x28)?;
    c.immediate(C::Al, Op::And, 3, 2, 0xff)?;
    c.byte_transfer(true, 1, 4, 0xfaf)?;
    c.compare(3, 1)?;
    c.branch(C::Cc, 0x69190)?;
    c.byte_transfer(true, 1, 4, 0xfb0)?;
    c.compare(3, 1)?;
    c.mov(C::Hi, 2, 1)?;
    c.mov(C::Al, 1, 2)?;
    c.check(0x69190)?;
    c.byte_transfer(false, 1, 4, 0x29)?;
    c.word_transfer(true, 1, 4, 0xc)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0)?;
    c.branch(C::Eq, 0x69208)?;
    c.word_transfer(true, 1, 13, 0x34)?;
    c.mov(C::Al, 2, 1)?;
    c.byte_transfer(false, 0, 2, 0)?;
    c.immediate(C::Al, Op::Add, 0, 0, 2)?;
    c.byte_transfer(true, 1, 4, 0xd20)?;
    c.compare(0, 1)?;
    c.mov(C::Hi, 0, 1)?;
    c.byte_transfer(false, 0, 2, 1)?;
    c.word_transfer(true, 1, 4, 0x34)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 2)?;
    c.branch(C::Cc, 0x69208)?;
    c.word_transfer(true, 1, 13, 0x34)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 1)?;
    c.mov(C::Al, 3, 0)?;
    c.immediate(C::Al, Op::Add, 1, 1, 2)?;
    c.check(0x691dc)?;
    c.immediate(C::Al, Op::And, 7, 0, 0xff)?;
    c.byte_transfer(true, 0, 4, 0xd20)?;
    c.immediate(C::Al, Op::And, 3, 3, 0xff)?;
    c.immediate(C::Al, Op::Add, 2, 2, 1)?;
    c.compare(0, 7)?;
    c.immediate(C::Hi, Op::Add, 0, 3, 1)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(0), r(1), 1)?)?;
    c.word_transfer(true, 3, 4, 0x34)?;
    c.compare(2, 3)?;
    c.mov(C::Al, 3, 0)?;
    c.branch(C::Cc, 0x691dc)?;
    c.check(0x69208)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.word_transfer(false, 0, 4, 0x2e0)?;
    c.word_transfer(false, 0, 4, 0x2d8)?;
    c.byte_transfer(true, 0, 4, 0x27)?;
    c.byte_transfer(false, 0, 4, 0x2d0)?;
    c.byte_transfer(true, 0, 4, 0x25)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Eq, 0x6924c)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Ne, 0x6927c)?;
    c.word_transfer(true, 0, 4, 0x5c)?;
    c.word_transfer(true, 1, 4, 0x60)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x3f)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(2),
        r(1),
        r(1),
        Shift::Lsl,
        1,
        false,
    )?)?;
    c.mov(C::Al, 0, 1)?;
    c.emit(mov_shifted_register(C::Hi, r(0), r(2), Shift::Lsr, 1)?)?;
    c.branch(C::Al, 0x6929c)?;
    c.check(0x6924c)?;
    c.word_transfer(true, 0, 4, 0x5c)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x40)?;
    c.branch(C::Cc, 0x6928c)?;
    c.word_transfer(true, 1, 4, 0x60)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(0),
        r(1),
        r(1),
        Shift::Lsl,
        1,
        false,
    )?)?;
    c.word_transfer(false, 0, 4, 0x2e4)?;
    c.word_transfer(true, 2, 4, 0x84)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, 3)?;
    c.immediate(C::Ne, Op::Cmp, 0, 2, 0xff)?;
    c.branch(C::Eq, END)?;
    c.emit(multiply_register(C::Al, r(0), r(1), r(2))?)?;
    c.branch(C::Al, 0x6929c)?;
    c.check(0x6927c)?;
    c.word_transfer(true, 1, 4, 0x60)?;
    c.branch(C::Al, 0x69298)?;
    c.data(0x69284)?;
    c.check(0x6928c)?;
    c.word_transfer(true, 1, 4, 0x60)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x28)?;
    c.branch(C::Cc, 0x69b00)?;
    c.check(0x69298)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Lsl, 1)?)?;
    c.check(0x6929c)?;
    c.word_transfer(false, 0, 4, 0x2e4)?;
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
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE"))
        else {
            return;
        };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(
            crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085"
        );
        assert_eq!(
            fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]
        );
    }

    #[test]
    fn exact_bounds_and_data_islands() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 508);
        assert_eq!(END - START, CODE_BYTES + DATA_BYTES);
        assert_eq!(
            (START..END).step_by(4).filter(|&at| is_data(at)).count() * 4,
            DATA_BYTES
        );
        assert_eq!(
            DATA.iter().map(|(_, words)| words.len() * 4).sum::<usize>(),
            76
        );
        for &(at, words) in DATA {
            for (index, &value) in words.iter().enumerate() {
                assert_eq!(word(&bytes, at + index * 4), value);
            }
        }
        assert_eq!(
            word(&bytes, END - 4),
            transfer_word(C::Al, false, r(0), r(4), 0x2e4).unwrap()
        );
    }

    #[test]
    fn data_and_bounds_reject_instruction_emission() {
        for at in (START..END).step_by(4).filter(|&at| is_data(at)) {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        for at in [START - 4, START + 1, END] {
            assert!(
                Code {
                    at,
                    bytes: Vec::new()
                }
                .word(0)
                .is_err()
            );
        }
        let mut c = Code {
            at: START,
            bytes: Vec::new(),
        };
        assert!(c.data(0x690c8).is_err());
        c.data(START).unwrap();
        assert_eq!(c.at, 0x690a8);
        assert!(c.data(0x690a8).is_err());
    }

    #[test]
    fn fixed_branch_edges() {
        let bytes = fragment().unwrap();
        for (at, condition, target) in [
            (0x690b4, C::Cc, 0x6907c),
            (0x690c0, C::Ls, 0x690e0),
            (0x690c4, C::Al, 0x6907c),
            (0x690d4, C::Eq, 0x690e0),
            (0x690e4, C::Ne, 0x69130),
            (0x690f4, C::Al, 0x6914c),
            (0x6913c, C::Cc, 0x690e8),
            (0x69148, C::Hi, 0x690e8),
            (0x69158, C::Cc, 0x6916c),
            (0x6917c, C::Cc, 0x69190),
            (0x6919c, C::Eq, 0x69208),
            (0x691c8, C::Cc, 0x69208),
            (0x69204, C::Cc, 0x691dc),
            (0x69224, C::Eq, 0x6924c),
            (0x6922c, C::Ne, 0x6927c),
            (0x69248, C::Al, 0x6929c),
            (0x69254, C::Cc, 0x6928c),
            (0x69270, C::Eq, END),
            (0x69278, C::Al, 0x6929c),
            (0x69280, C::Al, 0x69298),
            (0x69294, C::Cc, 0x69b00),
        ] {
            assert!(!is_data(at));
            assert!(!is_data(target));
            assert_eq!(
                word(&bytes, at),
                branch(condition, at as u32, target as u32, false).unwrap()
            );
        }
    }

    #[test]
    fn ramp_and_budget_predicates() {
        let bytes = fragment().unwrap();
        assert_eq!(
            word(&bytes, 0x691f0),
            data_immediate(C::Hi, Op::Add, r(0), r(3), 1).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x691f4),
            transfer_byte_postindexed(C::Al, false, r(0), r(1), 1).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x69244),
            mov_shifted_register(C::Hi, r(0), r(2), Shift::Lsr, 1).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6926c),
            data_immediate(C::Ne, Op::Cmp, r(0), r(2), 0xff).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x69274),
            multiply_register(C::Al, r(0), r(1), r(2)).unwrap()
        );
    }
}
