use arm32::{
    Condition as C, DataOp as Op, Register as R, Shift, branch, data_immediate, data_register,
    mov_register, mov_shifted_register, multiply_long, transfer_byte, transfer_doubleword,
    transfer_word,
};

pub const START: usize = 0x692a0;
pub const END: usize = 0x69498;
pub const FILE_BASE: usize = 0x4000;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at == expected {
            Ok(())
        } else {
            Err(format!(
                "budget records address 0x{:x} != 0x{expected:x}",
                self.at
            ))
        }
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!(
                "invalid budget records instruction address 0x{:x}",
                self.at
            ));
        }
        self.bytes.extend(instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn word(
        &mut self,
        condition: C,
        load: bool,
        rd: u8,
        rn: u8,
        offset: i32,
    ) -> Result<(), String> {
        self.emit(transfer_word(condition, load, r(rd), r(rn), offset)?)
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

    fn mov(&mut self, condition: C, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(condition, r(rd), r(rm)))
    }

    fn shift(&mut self, rd: u8, rm: u8, amount: u8) -> Result<(), String> {
        self.emit(mov_shifted_register(
            C::Al,
            r(rd),
            r(rm),
            Shift::Lsr,
            amount,
        )?)
    }

    fn product(&mut self, lo: u8, hi: u8, rm: u8, rs: u8) -> Result<(), String> {
        self.emit(multiply_long(C::Al, false, r(lo), r(hi), r(rm), r(rs))?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code {
        at: START,
        bytes: Vec::with_capacity(END - START),
    };
    c.word(C::Al, true, 3, 4, 0x64)?;
    c.immediate(C::Al, Op::Add, 2, 4, 0x760)?;
    c.immediate(C::Al, Op::Cmp, 0, 3, 0)?;
    c.branch(C::Ne, 0x692c0, false)?;
    c.word(C::Al, true, 3, 4, 0x68)?;
    c.immediate(C::Al, Op::Cmp, 0, 3, 0)?;
    c.mov(C::Eq, 0, 1)?;
    c.word(C::Eq, false, 1, 4, 0x2e4)?;
    c.check(0x692c0)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(2), 0)?)?;
    c.immediate(C::Al, Op::Mov, 7, 0, 7)?;
    c.immediate(C::Al, Op::Mov, 8, 0, 0)?;
    c.product(6, 5, 2, 7)?;
    c.emit(data_register(
        C::Al,
        Op::Sub,
        r(2),
        r(5),
        r(3),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.shift(3, 6, 3)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(2),
        r(3),
        r(2),
        Shift::Lsl,
        29,
        false,
    )?)?;
    c.mov(C::Al, 3, 9)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(0),
        r(2),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.word(C::Hi, false, 2, 4, 0x2e4)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.word(C::Al, false, 8, 0, 0x2e8)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.immediate(C::Al, Op::Add, 5, 0, 0x310)?;
    c.mov(C::Al, 0, 5)?;
    c.check(0x69300)?;
    c.branch(C::Al, 0x69b80, true)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 1)?;
    c.mov(C::Al, 3, 9)?;
    c.word(C::Al, true, 1, 0, 0x60)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.immediate(C::Al, Op::Add, 6, 0, 0x3f8)?;
    c.mov(C::Al, 0, 6)?;
    c.check(0x69320)?;
    c.branch(C::Al, 0x69b80, true)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 2)?;
    c.mov(C::Al, 3, 9)?;
    c.word(C::Al, true, 1, 0, 0x60)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.immediate(C::Al, Op::Add, 10, 0, 0x4e0)?;
    c.mov(C::Al, 0, 10)?;
    c.check(0x69340)?;
    c.branch(C::Al, 0x69b80, true)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.word(C::Al, true, 2, 13, 0x3c)?;
    c.word(C::Al, true, 0, 0, 0x60)?;
    c.shift(1, 0, 2)?;
    c.word(C::Al, false, 1, 2, 0x2f0)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(2),
        r(0),
        r(0),
        Shift::Lsl,
        1,
        false,
    )?)?;
    c.word(C::Al, true, 3, 13, 0x3c)?;
    c.shift(2, 2, 2)?;
    c.word(C::Al, false, 2, 3, 0x2ec)?;
    c.word(C::Al, true, 3, 13, 0x3c)?;
    c.word(C::Al, false, 0, 3, 0x2fc)?;
    c.word(C::Al, true, 3, 13, 0x3c)?;
    c.word(C::Al, true, 7, 13, 0x3c)?;
    c.word(C::Al, true, 3, 3, 0x330)?;
    c.check(0x6937c)?;
    c.product(4, 12, 3, 1)?;
    c.shift(1, 4, 16)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(1),
        r(1),
        r(12),
        Shift::Lsl,
        16,
        false,
    )?)?;
    c.word(C::Al, false, 1, 7, 0x2f4)?;
    c.product(1, 4, 3, 2)?;
    c.word(C::Al, true, 2, 13, 0x3c)?;
    c.shift(1, 1, 16)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(1),
        r(1),
        r(4),
        Shift::Lsl,
        16,
        false,
    )?)?;
    c.word(C::Al, false, 1, 2, 0x2f8)?;
    c.product(1, 2, 3, 0)?;
    c.shift(0, 1, 16)?;
    c.word(C::Al, true, 1, 13, 0x3c)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(0),
        r(0),
        r(2),
        Shift::Lsl,
        16,
        false,
    )?)?;
    c.word(C::Al, false, 0, 1, 0x300)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.word(C::Al, false, 8, 0, 0x308)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.word(C::Al, false, 8, 0, 0x304)?;
    c.word(C::Al, true, 0, 13, 0x3c)?;
    c.emit(transfer_byte(C::Al, false, r(8), r(0), 0x2d4)?)?;
    c.word(C::Al, true, 7, 13, 0x3c)?;
    c.word(C::Al, true, 0, 7, 0xc)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.word(C::Ne, true, 0, 7, 0x34)?;
    c.immediate(C::Ne, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x69440, false)?;
    c.word(C::Al, true, 0, 13, 0x34)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0)?;
    c.immediate(C::Al, Op::Add, 9, 0, 0x3440)?;
    c.check(0x693f0)?;
    c.mov(C::Al, 0, 9)?;
    c.mov(C::Al, 1, 5)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0xe8)?;
    c.branch(C::Al, 0x471c, true)?;
    c.immediate(C::Al, Op::Add, 0, 9, 0x570)?;
    c.mov(C::Al, 1, 6)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0xe8)?;
    c.branch(C::Al, 0x471c, true)?;
    c.word(C::Al, true, 0, 7, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x2000)?;
    c.branch(C::Ne, 0x6942c, false)?;
    c.immediate(C::Al, Op::Add, 0, 9, 0xae0)?;
    c.mov(C::Al, 1, 10)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0xe8)?;
    c.branch(C::Al, 0x471c, true)?;
    c.check(0x6942c)?;
    c.word(C::Al, true, 0, 7, 0x34)?;
    c.immediate(C::Al, Op::Add, 4, 4, 1)?;
    c.immediate(C::Al, Op::Add, 9, 9, 0xe8)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        r(0),
        r(4),
        r(0),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.branch(C::Cc, 0x693f0, false)?;
    c.check(0x69440)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0x89)?)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, END, false)?;
    c.word(C::Al, true, 0, 7, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x1000)?;
    c.branch(C::Eq, END, false)?;
    c.immediate(C::Al, Op::Add, 0, 7, 0x1e8)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x25)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0)?;
    c.immediate(C::Al, Op::Add, 0, 0, 0x400)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.word(C::Al, true, 0, 7, 0x2b0)?;
    c.word(C::Al, false, 4, 7, 0x610)?;
    c.word(C::Al, false, 4, 7, 0x614)?;
    c.shift(0, 0, 8)?;
    c.word(C::Al, false, 0, 7, 0x618)?;
    for offset in [0x61c, 0x620, 0x624, 0x628, 0x62c] {
        c.word(C::Al, false, 4, 7, offset)?;
    }
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], address: usize) -> u32 {
        let offset = address - START;
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
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
            reference[START + FILE_BASE..END + FILE_BASE]
        );
    }

    #[test]
    fn exact_instruction_bounds() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 504);
        assert_eq!(bytes.len(), 504);
        assert_eq!(bytes.chunks_exact(4).len(), 126);
        assert_eq!(
            word(&bytes, START),
            transfer_word(C::Al, true, r(3), r(4), 0x64).unwrap()
        );
        assert_eq!(
            word(&bytes, END - 4),
            transfer_word(C::Al, false, r(4), r(7), 0x62c).unwrap()
        );
        for at in [START - 4, START + 1, END, END + 4] {
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
    fn fixed_calls_and_control_flow() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x692ac, C::Ne, 0x692c0, false),
            (0x69300, C::Al, 0x69b80, true),
            (0x69320, C::Al, 0x69b80, true),
            (0x69340, C::Al, 0x69b80, true),
            (0x693e0, C::Eq, 0x69440, false),
            (0x693fc, C::Al, 0x471c, true),
            (0x6940c, C::Al, 0x471c, true),
            (0x69418, C::Ne, 0x6942c, false),
            (0x69428, C::Al, 0x471c, true),
            (0x6943c, C::Cc, 0x693f0, false),
            (0x69448, C::Ne, END, false),
            (0x69454, C::Eq, END, false),
            (0x6946c, C::Al, 0x31ec, true),
        ] {
            assert_eq!(
                word(&bytes, at),
                branch(condition, at as u32, target as u32, link).unwrap()
            );
        }
        assert_eq!(
            bytes
                .chunks_exact(4)
                .filter(|b| { u32::from_le_bytes((*b).try_into().unwrap()) >> 24 == 0xeb })
                .count(),
            7
        );
    }

    #[test]
    fn budget_products_copy_stride_and_clearing() {
        let bytes = fragment().unwrap();
        for (at, lo, hi, rm, rs) in [
            (0x692cc, 6, 5, 2, 7),
            (0x6937c, 4, 12, 3, 1),
            (0x6938c, 1, 4, 3, 2),
            (0x693a0, 1, 2, 3, 0),
        ] {
            assert_eq!(
                word(&bytes, at),
                multiply_long(C::Al, false, r(lo), r(hi), r(rm), r(rs)).unwrap()
            );
        }
        assert_eq!(
            word(&bytes, 0x692c0),
            transfer_doubleword(C::Al, true, r(2), r(2), 0).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x692e4),
            transfer_word(C::Hi, false, r(2), r(4), 0x2e4).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x69434),
            data_immediate(C::Al, Op::Add, r(9), r(9), 0xe8).unwrap()
        );
        assert_eq!(
            word(&bytes, 0x69460),
            data_immediate(C::Al, Op::Mov, r(2), r(0), 0x25).unwrap()
        );
        for (at, offset) in [
            (0x69474, 0x610),
            (0x69478, 0x614),
            (0x69484, 0x61c),
            (0x69488, 0x620),
            (0x6948c, 0x624),
            (0x69490, 0x628),
            (0x69494, 0x62c),
        ] {
            assert_eq!(
                word(&bytes, at),
                transfer_word(C::Al, false, r(4), r(7), offset).unwrap()
            );
        }
    }
}
