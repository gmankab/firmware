use arm32::{
    branch, branch_exchange_immediate, bx, data_immediate, data_immediate_with_flags,
    data_register, multiply_register_with_flags, transfer_multiple, transfer_word, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x817b8;
pub const END: usize = 0x818a0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = END - START;
pub const DATA_BYTES: usize = 0;

fn r(n: u8) -> R { R::new(n).unwrap() }

struct Code { at: usize, bytes: Vec<u8> }

impl Code {
    fn new(at: usize) -> Self { Self { at, bytes: Vec::new() } }
    fn emit(&mut self, at: usize, word: u32) -> Result<(), String> {
        if self.at != at || !(START..END).contains(&at) || at % 4 != 0 {
            return Err(format!("invalid arithmetic wrapper address {at:#x}"));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn comparison(c: &mut Code, at: usize, target: u32, condition: C) -> Result<(), String> {
    c.emit(at, transfer_multiple(C::Al, false, R::SP, 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(at + 4, branch_exchange_immediate((at + 4) as u32, target)?)?;
    c.emit(at + 8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(at + 12, branch(condition, (at + 12) as u32, (at + 24) as u32, false)?)?;
    c.emit(at + 16, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0, true)?)?;
    c.emit(at + 20, transfer_multiple(C::Al, true, R::SP, 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(at + 24, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 1, true)?)?;
    c.emit(at + 28, transfer_multiple(C::Al, true, R::SP, 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn divrem(c: &mut Code, at: usize, target: u32) -> Result<(), String> {
    c.emit(at, transfer_multiple(C::Al, false, R::SP, 0x4003, BlockMode::DecrementBefore, true)?)?;
    c.emit(at + 4, branch_exchange_immediate((at + 4) as u32, target)?)?;
    c.emit(at + 8, transfer_multiple(C::Al, true, R::SP, 0x000e, BlockMode::IncrementAfter, true)?)?;
    c.emit(at + 12, multiply_register_with_flags(C::Al, r(2), r(0), r(2), true)?)?;
    c.emit(at + 16, data_register(C::Al, Op::Sub, r(1), r(1), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(at + 20, bx(C::Al, r(3)))?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    comparison(&mut c, 0x817b8, 0x823a0, C::Lt)?;
    comparison(&mut c, 0x817d8, 0x822f8, C::Le)?;
    comparison(&mut c, 0x817f8, 0x82348, C::Ge)?;
    comparison(&mut c, 0x81818, 0x823a8, C::Gt)?;
    divrem(&mut c, 0x81838, 0x82be4)?;
    c.emit(0x81850, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x81854, branch(C::Cc, 0x81854, 0x81870, false)?)?;
    divrem(&mut c, 0x81858, 0x83c4c)?;
    c.emit(0x81870, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(0x81874, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0, true)?)?;
    c.emit(0x81878, bx(C::Al, R::LR))?;
    c.emit(0x8187c, transfer_multiple(C::Al, false, R::SP, 0x4040, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81880, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 16)?)?;
    c.emit(0x81884, data_immediate(C::Al, Op::Add, r(6), R::SP, 8)?)?;
    c.emit(0x81888, transfer_word(C::Al, false, r(6), R::SP, 0)?)?;
    c.emit(0x8188c, branch_exchange_immediate(0x8188c, 0x837f0)?)?;
    c.emit(0x81890, transfer_word(C::Al, true, r(2), R::SP, 8)?)?;
    c.emit(0x81894, transfer_word(C::Al, true, r(3), R::SP, 12)?)?;
    c.emit(0x81898, data_immediate(C::Al, Op::Add, R::SP, R::SP, 16)?)?;
    c.emit(0x8189c, transfer_multiple(C::Al, true, R::SP, 0x8040, BlockMode::IncrementAfter, true)?)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES { return Err("invalid arithmetic wrapper extent".into()); }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn extent_and_sensitive_words() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, CODE_BYTES, DATA_BYTES), (0x817b8, 0x818a0, 232, 0));
        assert_eq!(bytes.len(), END - START);
        for (at, expected) in [
            (0x817bc, 0xfa0002f7), (0x817c4, 0xba000001), (0x817c8, 0xe3b00000),
            (0x817dc, 0xfa0002c5), (0x817e4, 0xda000001),
            (0x817fc, 0xfa0002d1), (0x81804, 0xaa000001),
            (0x8181c, 0xfa0002e1), (0x81824, 0xca000001),
            (0x8183c, 0xfa0004e8), (0x81844, 0xe0120290), (0x81848, 0xe0511002),
            (0x8184c, 0xe12fff13), (0x81854, 0x3a000005), (0x8185c, 0xfa0008fa),
            (0x81864, 0xe0120290), (0x81868, 0xe0511002), (0x81870, 0xe1b01000),
            (0x81874, 0xe3b00000), (0x81878, 0xe12fff1e), (0x8188c, 0xfa0007d7),
            (0x81888, 0xe58d6000), (0x81890, 0xe59d2008), (0x81894, 0xe59d300c),
            (0x8189c, 0xe8bd8040),
        ] { assert_eq!(word(&bytes, at), expected, "{at:#x}"); }
    }

    #[test]
    fn function_boundaries() {
        for (at, target, condition) in [
            (0x817b8, 0x823a0, C::Lt), (0x817d8, 0x822f8, C::Le),
            (0x817f8, 0x82348, C::Ge), (0x81818, 0x823a8, C::Gt),
        ] {
            let mut c = Code::new(at);
            comparison(&mut c, at, target, condition).unwrap();
            assert_eq!((c.at, c.bytes.len()), (at + 32, 32));
        }
        for (at, target) in [(0x81838, 0x82be4), (0x81858, 0x83c4c)] {
            let mut c = Code::new(at);
            divrem(&mut c, at, target).unwrap();
            assert_eq!((c.at, c.bytes.len()), (at + 24, 24));
        }
    }

    #[test]
    fn checked_bounds() {
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code::new(at);
            assert!(c.emit(at, 0).is_err());
            assert_eq!((c.at, c.bytes.len()), (at, 0));
        }
        let mut c = Code::new(START);
        assert!(c.emit(START + 4, 0).is_err());
        assert!(R::new(16).is_err());
        assert!(data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 32, false).is_err());
        assert!(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x12345678).is_err());
        assert!(transfer_word(C::Al, true, r(0), R::SP, 4096).is_err());
        assert!(transfer_multiple(C::Al, false, R::SP, 0, BlockMode::DecrementBefore, true).is_err());
        assert!(branch(C::Al, START as u32, START as u32 + 2, false).is_err());
        assert!(branch_exchange_immediate(START as u32, START as u32 + 1).is_err());
    }

    #[test]
    fn authenticated_same_buffer_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
