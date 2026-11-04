use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, mov_register, multiply_long,
    multiply_register, push_registers, signed_multiply_accumulate_bottom_halves, transfer_byte,
    transfer_byte_postindexed, transfer_byte_preindexed_writeback, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_postindexed, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6ac0c;
pub const END: usize = 0x6af2c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = END - START;
pub const DATA_BYTES: usize = 0;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!(
                "weighted metrics instruction outside range: {:#x}",
                self.at
            ));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!(
                "weighted metrics address {:#x} != {expected:#x}",
                self.at
            ));
        }
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn call_float(&mut self, target: u32) -> Result<(), String> {
        self.emit(branch_exchange_immediate(self.at as u32, target)?)
    }

    fn immediate(&mut self, condition: C, op: Op, rd: R, rn: R, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, rd, rn, value)?)
    }

    fn mov(&mut self, condition: C, rd: R, rm: R) -> Result<(), String> {
        self.emit(mov_register(condition, rd, rm))
    }

    fn load(&mut self, condition: C, rd: R, rn: R, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(condition, true, rd, rn, offset)?)
    }

    fn store(&mut self, rd: R, rn: R, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, false, rd, rn, offset)?)
    }

    fn byte(&mut self, load: bool, rd: R, rn: R, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(C::Al, load, rd, rn, offset)?)
    }

    fn commit64(&mut self, rn: R, offset: i32) -> Result<(), String> {
        self.emit(transfer_doubleword(C::Al, false, r(4), rn, offset)?)
    }
}

fn r(n: u8) -> R {
    R::new(n).expect("valid weighted metrics register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(CODE_BYTES),
    };
    let saved = (4..=11).fold(R::LR.bit(), |mask, n| mask | r(n).bit());
    c.emit(push_registers(C::Al, saved)?)?;
    c.immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?;
    c.immediate(C::Al, Op::Sub, R::SP, R::SP, 0x14)?;
    c.load(C::Al, r(6), R::R11, 0xc)?;
    c.store(r(2), R::SP, 0xc)?;
    c.mov(C::Al, r(9), r(1))?;
    c.immediate(C::Al, Op::Mov, r(1), r(0), 0x2c80000)?;
    c.mov(C::Al, r(8), r(0))?;
    c.mov(C::Al, r(4), r(3))?;
    c.immediate(C::Al, Op::Orr, r(1), r(1), 0x40000000)?;
    c.load(C::Al, r(7), r(6), 0x60)?;
    c.mov(C::Al, r(0), r(7))?;
    c.call_float(0x83608)?;
    c.call_float(0x82d98)?;
    c.emit(multiply_long(C::Al, false, r(2), r(1), r(0), r(9))?)?;
    c.immediate(C::Al, Op::Mov, r(3), r(0), 0)?;
    c.immediate(C::Al, Op::Mov, r(10), r(0), 0)?;
    c.mov(C::Al, r(0), r(2))?;
    c.immediate(C::Al, Op::Mov, r(2), r(0), 100)?;
    c.branch(C::Al, 0x8187c, true)?;
    c.mov(C::Al, r(3), r(1))?;
    c.immediate(C::Al, Op::Mov, r(1), r(0), 0xb30)?;
    c.immediate(C::Al, Op::Add, r(2), r(8), 0x278)?;
    c.immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?;
    c.emit(transfer_word_register(
        C::Al,
        true,
        r(1),
        r(8),
        r(1),
        Shift::Lsl,
        0,
    )?)?;
    c.store(r(1), R::SP, 0x10)?;
    c.byte(true, r(1), r(8), 0x8a)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(1), 0)?;
    c.branch(C::Eq, 0x6acd8, false)?;
    c.mov(C::Al, r(12), r(0))?;
    c.mov(C::Al, R::LR, r(3))?;
    c.mov(C::Al, r(9), r(2))?;
    c.immediate(C::Al, Op::Add, r(0), r(6), 0x40)?;
    c.immediate(C::Al, Op::Add, r(1), r(6), 0x64)?;
    c.immediate(C::Al, Op::Mov, r(4), r(0), 0)?;
    c.immediate(C::Al, Op::Mov, r(5), r(0), 0)?;
    c.check(0x6ac9c)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(2), r(1), 1)?)?;
    c.emit(transfer_word_postindexed(C::Al, true, r(3), r(0), 4)?)?;
    c.immediate(C::Al, Op::Add, r(10), r(10), 1)?;
    c.emit(multiply_register(C::Al, r(7), r(3), r(2))?)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(4),
        r(4),
        r(7),
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.immediate(C::Al, Op::Adc, r(5), r(5), 0)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(10), 4)?;
    c.branch(C::Ne, 0x6ac9c, false)?;
    c.byte(true, r(0), r(8), 0x8b)?;
    c.load(C::Al, r(7), R::SP, 0xc)?;
    c.mov(C::Al, r(2), r(9))?;
    c.immediate(C::Al, Op::Cmp, r(0), r(0), 0)?;
    c.mov(C::Eq, r(5), R::LR)?;
    c.mov(C::Eq, r(4), r(12))?;
    c.branch(C::Al, 0x6ad78, false)?;
    c.check(0x6acd8)?;
    c.mov(C::Al, r(1), r(4))?;
    c.mov(C::Al, R::LR, r(4))?;
    c.emit(multiply_long(C::Al, false, r(4), r(5), r(1), r(9))?)?;
    c.byte(true, r(1), r(8), 0x1f)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(1), 0)?;
    c.branch(C::Eq, 0x6ad74, false)?;
    c.byte(true, r(1), r(8), 0x27c)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(1), 2)?;
    c.branch(C::Ne, 0x6ad74, false)?;
    c.load(C::Al, r(12), r(6), 0)?;
    c.load(C::Al, r(1), r(6), 4)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(1),
        r(1),
        r(1),
        Shift::Lsl,
        1,
        false,
    )?)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(1),
        r(12),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Ls, 0x6ad74, false)?;
    c.store(r(0), R::SP, 8)?;
    c.mov(C::Al, r(0), R::LR)?;
    c.store(r(3), R::SP, 4)?;
    c.mov(C::Al, r(10), r(2))?;
    c.call_float(0x82f74)?;
    c.mov(C::Al, r(1), r(0))?;
    c.mov(C::Al, r(0), r(7))?;
    c.call_float(0x837e0)?;
    c.immediate(C::Al, Op::Mov, r(1), r(0), 0)?;
    c.mov(C::Al, r(7), r(0))?;
    c.branch(C::Al, 0x817b8, true)?;
    c.mov(C::Al, r(9), r(0))?;
    c.immediate(C::Al, Op::Mov, r(0), r(0), 0x80000000)?;
    c.mov(C::Al, r(1), r(7))?;
    c.call_float(0x837e0)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(9), 0)?;
    c.immediate(C::Al, Op::Mov, r(1), r(0), 0x40000000)?;
    c.mov(C::Eq, r(0), r(7))?;
    c.branch(C::Al, 0x81818, true)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(0), 0)?;
    c.load(C::Al, r(7), R::SP, 0xc)?;
    c.mov(C::Al, r(2), r(10))?;
    c.load(C::Ne, r(4), R::SP, 8)?;
    c.load(C::Ne, r(5), R::SP, 4)?;
    c.branch(C::Al, 0x6ad78, false)?;
    c.check(0x6ad74)?;
    c.load(C::Al, r(7), R::SP, 0xc)?;
    c.check(0x6ad78)?;
    c.load(C::Al, r(1), R::SP, 0x10)?;
    c.load(C::Al, r(3), R::R11, 8)?;
    c.immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        r(0),
        r(3),
        r(0),
        r(1),
    )?)?;
    c.immediate(C::Al, Op::Add, r(1), r(0), 0xf8)?;
    c.load(C::Al, r(0), r(8), 0xc)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(0), 0)?;
    c.immediate(C::Eq, Op::Add, r(1), r(2), 0x400)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(7), 2)?;
    c.branch(C::Eq, 0x6add8, false)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(7), 1)?;
    c.branch(C::Eq, 0x6ae04, false)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(7), 0)?;
    c.branch(C::Ne, 0x6ae30, false)?;
    c.byte(true, r(2), r(1), 0xd0)?;
    c.immediate(C::Al, Op::Tst, r(0), r(2), 1)?;
    c.branch(C::Ne, 0x6adc8, false)?;
    c.byte(true, r(3), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(3), 1)?;
    c.branch(C::Ne, 0x6ae2c, false)?;
    c.immediate(C::Al, Op::And, r(2), r(2), 6)?;
    c.commit64(r(1), 8)?;
    c.byte(false, r(2), r(1), 0xd0)?;
    c.branch(C::Al, 0x6ae2c, false)?;
    c.check(0x6add8)?;
    c.byte(true, r(2), r(1), 0xd0)?;
    c.immediate(C::Al, Op::Tst, r(0), r(2), 4)?;
    c.branch(C::Ne, 0x6adf0, false)?;
    c.byte(true, r(3), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(3), 1)?;
    c.branch(C::Ne, 0x6adfc, false)?;
    c.immediate(C::Al, Op::And, r(2), r(2), 3)?;
    c.commit64(r(1), 0x28)?;
    c.byte(false, r(2), r(1), 0xd0)?;
    c.immediate(C::Al, Op::Add, r(1), r(1), 0x20)?;
    c.branch(C::Al, 0x6ae2c, false)?;
    c.check(0x6ae04)?;
    c.byte(true, r(2), r(1), 0xd0)?;
    c.immediate(C::Al, Op::Tst, r(0), r(2), 2)?;
    c.branch(C::Ne, 0x6ae1c, false)?;
    c.byte(true, r(3), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(3), 1)?;
    c.branch(C::Ne, 0x6ae28, false)?;
    c.immediate(C::Al, Op::And, r(2), r(2), 5)?;
    c.commit64(r(1), 0x18)?;
    c.byte(false, r(2), r(1), 0xd0)?;
    c.immediate(C::Al, Op::Add, r(1), r(1), 0x10)?;
    c.check(0x6ae2c)?;
    c.commit64(r(1), 0)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(0), 0)?;
    c.branch(C::Eq, 0x6af1c, false)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(7), 2)?;
    c.branch(C::Eq, 0x6ae94, false)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(7), 1)?;
    c.branch(C::Eq, 0x6aed8, false)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(7), 0)?;
    c.branch(C::Ne, 0x6af1c, false)?;
    c.load(C::Al, r(1), R::SP, 0x10)?;
    c.load(C::Al, r(2), R::R11, 8)?;
    c.immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        r(1),
        r(2),
        r(0),
        r(1),
    )?)?;
    c.commit64(r(1), 0xf8)?;
    c.mov(C::Al, r(0), r(1))?;
    c.emit(transfer_byte_preindexed_writeback(
        C::Al,
        true,
        r(2),
        r(0),
        0x1c8,
    )?)?;
    c.immediate(C::Al, Op::Tst, r(0), r(2), 1)?;
    c.branch(C::Ne, 0x6ae80, false)?;
    c.byte(true, r(3), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(3), 1)?;
    c.branch(C::Ne, 0x6af1c, false)?;
    c.store(r(4), r(1), 0x100)?;
    c.store(r(5), r(1), 0x104)?;
    c.immediate(C::Al, Op::And, r(1), r(2), 6)?;
    c.byte(false, r(1), r(0), 0)?;
    c.branch(C::Al, 0x6af1c, false)?;
    c.check(0x6ae94)?;
    c.load(C::Al, r(1), R::SP, 0x10)?;
    c.load(C::Al, r(2), R::R11, 8)?;
    c.immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        r(0),
        r(2),
        r(0),
        r(1),
    )?)?;
    c.store(r(4), r(0), 0x118)?;
    c.store(r(5), r(0), 0x11c)?;
    c.mov(C::Al, r(1), r(0))?;
    c.emit(transfer_byte_preindexed_writeback(
        C::Al,
        true,
        r(2),
        r(1),
        0x1c8,
    )?)?;
    c.immediate(C::Al, Op::Tst, r(0), r(2), 4)?;
    c.branch(C::Ne, 0x6aec8, false)?;
    c.byte(true, r(3), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(3), 1)?;
    c.branch(C::Ne, 0x6af1c, false)?;
    c.store(r(4), r(0), 0x120)?;
    c.store(r(5), r(0), 0x124)?;
    c.immediate(C::Al, Op::And, r(0), r(2), 3)?;
    c.branch(C::Al, 0x6af18, false)?;
    c.check(0x6aed8)?;
    c.load(C::Al, r(1), R::SP, 0x10)?;
    c.load(C::Al, r(2), R::R11, 8)?;
    c.immediate(C::Al, Op::Mov, r(0), r(0), 0xd8)?;
    c.emit(signed_multiply_accumulate_bottom_halves(
        C::Al,
        r(0),
        r(2),
        r(0),
        r(1),
    )?)?;
    c.store(r(4), r(0), 0x108)?;
    c.store(r(5), r(0), 0x10c)?;
    c.mov(C::Al, r(1), r(0))?;
    c.emit(transfer_byte_preindexed_writeback(
        C::Al,
        true,
        r(2),
        r(1),
        0x1c8,
    )?)?;
    c.immediate(C::Al, Op::Tst, r(0), r(2), 2)?;
    c.branch(C::Ne, 0x6af0c, false)?;
    c.byte(true, r(3), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Cmp, r(0), r(3), 1)?;
    c.branch(C::Ne, 0x6af1c, false)?;
    c.store(r(4), r(0), 0x110)?;
    c.store(r(5), r(0), 0x114)?;
    c.immediate(C::Al, Op::And, r(0), r(2), 5)?;
    c.byte(false, r(0), r(1), 0)?;
    c.check(0x6af1c)?;
    c.immediate(C::Al, Op::Mov, r(0), r(0), 0)?;
    c.byte(false, r(0), r(6), 0xb8)?;
    c.immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?;
    c.emit(transfer_multiple(
        C::Al,
        true,
        R::SP,
        (saved & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter,
        true,
    )?)?;
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
        let bytes = fragment().unwrap();
        let expected = &reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA];
        assert_eq!(bytes.len(), expected.len());
        for (index, (actual, expected)) in bytes
            .chunks_exact(4)
            .zip(expected.chunks_exact(4))
            .enumerate()
        {
            assert_eq!(actual, expected, "instruction at {:#x}", START + index * 4);
        }
    }

    #[test]
    fn complete_function_and_bounds() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 800);
        assert_eq!(CODE_BYTES, 200 * 4);
        assert_eq!(DATA_BYTES, 0);
        for at in [START - 4, START + 1, END] {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
    }

    #[test]
    fn arithmetic_predication_and_commits() {
        let bytes = fragment().unwrap();
        assert_eq!(
            word(&bytes, 0x6ac44),
            multiply_long(C::Al, false, r(2), r(1), r(0), r(9)).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6ace0),
            multiply_long(C::Al, false, r(4), r(5), r(1), r(9)).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6acac),
            data_register(C::Al, Op::Add, r(4), r(4), r(7), Shift::Lsl, 0, true).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6acb0),
            data_immediate(C::Al, Op::Adc, r(5), r(5), 0).unwrap()
        );
        assert_eq!(word(&bytes, 0x6accc), mov_register(C::Eq, r(5), R::LR));
        assert_eq!(
            word(&bytes, 0x6ad68),
            transfer_word(C::Ne, true, r(4), R::SP, 8).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6ad94),
            data_immediate(C::Eq, Op::Add, r(1), r(2), 0x400).unwrap()
        );
        for (at, offset) in [
            (0x6adcc, 8),
            (0x6adf4, 0x28),
            (0x6ae20, 0x18),
            (0x6ae2c, 0),
            (0x6ae60, 0xf8),
        ] {
            assert_eq!(
                word(&bytes, at),
                transfer_doubleword(C::Al, false, r(4), r(1), offset).unwrap()
            );
        }
        assert_eq!(
            word(&bytes, 0x6af20),
            transfer_byte(C::Al, false, r(0), r(6), 0xb8).unwrap()
        );
    }

    #[test]
    fn call_instruction_sets_and_record_stride() {
        let bytes = fragment().unwrap();
        for (at, target) in [
            (0x6ac3c, 0x83608),
            (0x6ac40, 0x82d98),
            (0x6ad20, 0x82f74),
            (0x6ad2c, 0x837e0),
            (0x6ad48, 0x837e0),
        ] {
            assert_eq!(
                word(&bytes, at),
                branch_exchange_immediate(at as u32, target).unwrap()
            );
        }
        for (at, target) in [(0x6ac58, 0x8187c), (0x6ad38, 0x817b8), (0x6ad58, 0x81818)] {
            assert_eq!(
                word(&bytes, at),
                branch(C::Al, at as u32, target, true).unwrap()
            );
        }
        assert_eq!(
            word(&bytes, 0x6ad84),
            signed_multiply_accumulate_bottom_halves(C::Al, r(0), r(3), r(0), r(1)).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6ac9c),
            transfer_byte_postindexed(C::Al, true, r(2), r(1), 1).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x6aca0),
            transfer_word_postindexed(C::Al, true, r(3), r(0), 4).unwrap()
        );
    }
}
