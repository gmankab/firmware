use arm32::{
    Condition as C, DataOp as Op, Register as R, Shift, branch, data_immediate, data_register,
    mov_register, store_halfword, transfer_byte, transfer_word,
};

pub const START: usize = 0x68718;
pub const END: usize = 0x68920;
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
                "controller parameters 0x{:x} != 0x{expected:x}",
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
    c.check(START)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x7a)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x26)?)?;
    c.emit(data_register(
        C::Al,
        Op::Orr,
        r(0),
        r(12),
        R::LR,
        Shift::Lsl,
        0,
        true,
    )?)?;
    c.emit(transfer_byte(C::Al, false, r(5), r(8), 0x2c)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(8), 0x2d)?)?;
    c.branch(C::Ne, 0x68744, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x100)?)?;
    c.emit(store_halfword(C::Al, r(0), r(8), 0x2c)?)?;
    c.check(0x68744)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x11c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, r(0), r(5), 2)?)?;
    c.emit(transfer_byte(C::Ne, false, r(0), r(8), 0x2c)?)?;
    c.emit(data_immediate(C::Ne, Op::Add, r(0), r(7), 2)?)?;
    c.emit(transfer_byte(C::Ne, false, r(0), r(8), 0x2d)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 0x30)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x74)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x27)?)?;
    c.emit(mov_register(C::Al, r(2), r(0)))?;
    c.emit(transfer_byte(C::Eq, true, r(0), r(9), 0x76)?)?;
    c.emit(transfer_byte(C::Eq, true, r(2), r(9), 0x78)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x28)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(8), 0x29)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(8), 0x78)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x1fc)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(8), 0x1f)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x1fd)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(8), 0x27c)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x838)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(2), 0x58)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(8), 0x23)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(2), 0x5c)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(8), 0x22)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x68)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 2)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), R::R0, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x2000)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x24)?)?;
    c.branch(C::Eq, 0x687d8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x7f)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0x1000)?)?;
    c.branch(C::Eq, 0x68800, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(1), 2)?)?;
    c.branch(C::Ne, 0x687ec, false)?;
    c.check(0x687d8)?;
    c.emit(transfer_word(C::Al, true, r(0), R::SP, 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), R::R0, 0x33)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(5), R::R0, 0x3f)?)?;
    c.branch(C::Al, 0x68800, false)?;
    c.check(0x687ec)?;
    c.emit(transfer_word(C::Al, true, r(2), R::PC, 0x278)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x23c)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), R::R0, 0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0x68800)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0xca)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), R::R0, 0x284)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(4), r(4), 0x800)?)?;
    c.emit(data_register(
        C::Al,
        Op::Add,
        r(12),
        r(8),
        r(4),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(mov_register(C::Al, r(4), r(8)))?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), R::R0, 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), R::SP, 0x20)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x2a)?)?;
    c.emit(transfer_byte(C::Al, false, r(5), r(8), 0x2b)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(9), 0x186)?)?;
    c.emit(transfer_byte(C::Al, false, r(6), r(8), 0xa90)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(6), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(9), 0x183)?)?;
    c.emit(mov_register(C::Eq, r(6), r(5)))?;
    c.emit(transfer_byte(C::Al, false, r(7), r(8), 0xa8f)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 0x187)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(8), 0xd20)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 0x184)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(8), 0xd1f)?)?;
    c.emit(transfer_byte(C::Al, true, R::LR, r(9), 0x188)?)?;
    c.emit(transfer_byte(C::Al, false, R::LR, r(8), 0xfb0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x185)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0xfaf)?)?;
    c.emit(transfer_byte(C::Eq, false, r(5), r(8), 0xa90)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0)?)?;
    c.emit(transfer_word(C::Eq, true, r(7), R::SP, 0x20)?)?;
    c.emit(transfer_byte(C::Eq, false, r(7), r(8), 0xa8f)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(3), 0)?)?;
    c.emit(transfer_byte(C::Eq, false, r(5), r(8), 0xd20)?)?;
    c.emit(mov_register(C::Eq, r(3), r(5)))?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0)?)?;
    c.emit(transfer_word(C::Eq, true, r(2), R::SP, 0x20)?)?;
    c.emit(transfer_byte(C::Eq, false, r(2), r(8), 0xd1f)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, R::LR, 0)?)?;
    c.emit(transfer_byte(C::Eq, false, r(5), r(8), 0xfb0)?)?;
    c.emit(mov_register(C::Eq, R::LR, r(5)))?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(8), R::PC, 0xe44)?)?;
    c.emit(transfer_word(C::Eq, true, r(0), R::SP, 0x20)?)?;
    c.emit(transfer_byte(C::Eq, false, r(0), r(4), 0xfaf)?)?;
    c.check(0x688a8)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(6),
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(mov_register(C::Cs, r(6), r(5)))?;
    c.emit(transfer_byte(C::Al, false, r(6), r(4), 0xa90)?)?;
    c.emit(transfer_word(C::Al, true, r(6), R::SP, 0x20)?)?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(7),
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(mov_register(C::Ls, r(7), r(6)))?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(3),
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(mov_register(C::Cs, r(3), r(5)))?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(2),
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 0xa8f)?)?;
    c.emit(mov_register(C::Ls, r(2), r(6)))?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        R::LR,
        r(1),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(4), 0xd20)?)?;
    c.emit(mov_register(C::Cc, r(5), R::LR))?;
    c.emit(data_register(
        C::Al,
        Op::Cmp,
        R::R0,
        r(0),
        r(6),
        Shift::Lsl,
        0,
        false,
    )?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(4), 0xd1f)?)?;
    c.emit(transfer_word(C::Al, false, r(1), R::SP, 0x18)?)?;
    c.emit(mov_register(C::Ls, r(0), r(6)))?;
    c.emit(transfer_byte(C::Al, false, r(5), r(4), 0xfb0)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0xfaf)?)?;
    c.emit(transfer_word(C::Al, false, r(12), r(4), 0xa80)?)?;
    c.check(0x688fc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0xac)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 0x1e)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x2000)?)?;
    c.branch(C::Eq, 0x68984, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0x1000)?)?;
    c.branch(C::Eq, 0x689e8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, R::R0, r(0), 2)?)?;
    c.branch(C::Ne, 0x68d44, false)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_runtime_extent() {
        assert_eq!(START, 0x68718);
        assert_eq!(END, 0x68920);
        assert_eq!(fragment().unwrap().len(), 520);
    }

    #[test]
    fn exact_pinned_runtime_bytes() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let source = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(source.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&source), identity.sha256);
        assert_eq!(
            fragment().unwrap(),
            source[START + FILE_BASE..END + FILE_BASE]
        );
    }

    #[test]
    fn fixed_writer_rejects_invalid_addresses() {
        for at in [START - 4, START + 1, END] {
            let mut c = Code {
                at,
                bytes: Vec::new(),
            };
            assert!(c.emit(mov_register(C::Al, R::R0, R::R0)).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        assert!(Code::new().check(END).is_err());
    }

    #[test]
    fn dispatch_targets_remain_outside_fragment() {
        let bytes = fragment().unwrap();
        for (at, condition, target) in [
            (0x6890c, C::Eq, 0x68984),
            (0x68914, C::Eq, 0x689e8),
            (0x6891c, C::Ne, 0x68d44),
        ] {
            assert!(target >= END);
            let offset = at - START;
            assert_eq!(
                &bytes[offset..offset + 4],
                &branch(condition, at as u32, target as u32, false)
                    .unwrap()
                    .to_le_bytes(),
            );
        }
    }
}
