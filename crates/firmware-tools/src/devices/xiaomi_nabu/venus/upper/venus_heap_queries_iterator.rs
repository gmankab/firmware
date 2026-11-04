use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register, transfer_byte,
    transfer_multiple, transfer_word, BlockMode, Condition as C, DataOp as Op, Register as R,
    Shift,
};

pub const START: usize = 0xcdc0;
pub const END: usize = 0xd040;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 624;
pub const DATA_BYTES: usize = 16;
const SOURCE: u32 = 0x000e_37d7;
const PROTECTED_DATA: [(usize, u32); 4] = [
    (0xce50, SOURCE),
    (0xcee4, SOURCE),
    (0xcf20, SOURCE),
    (0xd03c, SOURCE),
];

fn r(number: u8) -> R {
    R::new(number).expect("A32 register")
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("heap queries iterator at {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !(START..END).contains(&at) || at % 4 != 0
            || PROTECTED_DATA.iter().any(|&(address, _)| address == at)
        {
            return Err(format!("heap queries iterator instruction outside code: {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !PROTECTED_DATA.contains(&(at, value)) {
            return Err(format!("heap queries iterator invalid literal: {at:#x} = {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn jump(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }

    fn literal(&mut self, at: usize, target: usize) -> Result<(), String> {
        if !PROTECTED_DATA.contains(&(target, SOURCE)) {
            return Err(format!("heap queries iterator invalid literal target: {target:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, r(2), R::PC, target as i32 - at as i32 - 8)?)
    }
}

fn usable_size(c: &mut Code) -> Result<(), String> {
    c.emit(0xcdc0, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xcdc4, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0xcdc8, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcdcc, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.jump(0xcdd0, C::Ne, 0xcdec, false)?;
    c.jump(0xcdd4, C::Al, 0x86644, true)?;
    c.literal(0xcdd8, 0xce50)?;
    c.emit(0xcddc, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xaf)?)?;
    c.emit(0xcde0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcde4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.jump(0xcde8, C::Al, 0x17b0, true)?;
    c.emit(0xcdec, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0xcdf0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.jump(0xcdf4, C::Ne, 0xce10, false)?;
    c.jump(0xcdf8, C::Al, 0x86644, true)?;
    c.literal(0xcdfc, 0xce50)?;
    c.emit(0xce00, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xce04, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9b0)?)?;
    c.jump(0xce08, C::Al, 0x17b0, true)?;
    c.emit(0xce0c, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0xce10, data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.jump(0xce14, C::Eq, 0xce34, false)?;
    c.jump(0xce18, C::Al, 0x86644, true)?;
    c.literal(0xce1c, 0xce50)?;
    c.emit(0xce20, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xb1)?)?;
    c.emit(0xce24, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xce28, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.jump(0xce2c, C::Al, 0x17b0, true)?;
    c.emit(0xce30, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0xce34, transfer_byte(C::Al, true, r(2), r(4), 3)?)?;
    c.emit(0xce38, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0xce3c, data_immediate(C::Al, Op::Sub, r(0), r(0), 8)?)?;
    c.emit(0xce40, data_immediate(C::Al, Op::Tst, r(0), r(2), 0xf)?)?;
    c.emit(0xce44, transfer_byte(C::Eq, true, r(1), r(4), 2)?)?;
    c.emit(0xce48, data_register(C::Al, Op::Sub, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xce4c, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xce50, SOURCE)
}

fn accounting_queries(c: &mut Code) -> Result<(), String> {
    c.emit(0xce54, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0xce58, transfer_word(C::Ne, true, r(0), r(0), 0x14)?)?;
    c.emit(0xce5c, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xce60, bx(C::Al, R::LR))?;
    c.emit(0xce64, data_immediate(C::Al, Op::Mov, r(0), r(0), 8)?)?;
    c.emit(0xce68, bx(C::Al, R::LR))?;
    c.emit(0xce6c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0xce70, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xce74, transfer_word(C::Ne, true, r(1), r(0), 0xc)?)?;
    c.emit(0xce78, transfer_word(C::Ne, true, r(0), r(0), 0x14)?)?;
    c.emit(0xce7c, data_register(C::Ne, Op::Add, r(0), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(0xce80, bx(C::Al, R::LR))
}

fn iterator_init(c: &mut Code) -> Result<(), String> {
    c.emit(0xce84, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xce88, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0xce8c, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xce90, data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xce94, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.jump(0xce98, C::Ne, 0xceb4, false)?;
    c.jump(0xce9c, C::Al, 0x86644, true)?;
    c.literal(0xcea0, 0xcee4)?;
    c.emit(0xcea4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xd2)?)?;
    c.emit(0xcea8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xceac, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.jump(0xceb0, C::Al, 0x17b0, true)?;
    c.emit(0xceb4, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0)?)?;
    c.jump(0xceb8, C::Ne, 0xced4, false)?;
    c.jump(0xcebc, C::Al, 0x86644, true)?;
    c.literal(0xcec0, 0xcee4)?;
    c.emit(0xcec4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xd3)?)?;
    c.emit(0xcec8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcecc, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.jump(0xced0, C::Al, 0x17b0, true)?;
    c.emit(0xced4, transfer_word(C::Al, false, r(5), r(4), 0)?)?;
    c.emit(0xced8, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0xcedc, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.jump(0xcee0, C::Al, 0xcee8, false)?;
    c.data(0xcee4, SOURCE)
}

fn iterator_reset(c: &mut Code) -> Result<(), String> {
    c.emit(0xcee8, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xceec, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0xcef0, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcef4, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.jump(0xcef8, C::Ne, 0xcf14, false)?;
    c.jump(0xcefc, C::Al, 0x86644, true)?;
    c.literal(0xcf00, 0xcf20)?;
    c.emit(0xcf04, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe7)?)?;
    c.emit(0xcf08, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcf0c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x900)?)?;
    c.jump(0xcf10, C::Al, 0x17b0, true)?;
    c.emit(0xcf14, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcf18, transfer_word(C::Al, false, r(0), r(4), 4)?)?;
    c.emit(0xcf1c, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xcf20, SOURCE)
}

fn iterator_advance(c: &mut Code) -> Result<(), String> {
    c.emit(0xcf24, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0xcf28, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x10)?)?;
    c.emit(0xcf2c, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcf30, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xcf34, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xcf38, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0)?)?;
    c.jump(0xcf3c, C::Ne, 0xcf58, false)?;
    c.jump(0xcf40, C::Al, 0x86644, true)?;
    c.literal(0xcf44, 0xd03c)?;
    c.emit(0xcf48, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x12)?)?;
    c.emit(0xcf4c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcf50, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xa00)?)?;
    c.jump(0xcf54, C::Al, 0x17b0, true)?;
    c.emit(0xcf58, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.jump(0xcf5c, C::Ne, 0xcf78, false)?;
    c.jump(0xcf60, C::Al, 0x86644, true)?;
    c.literal(0xcf64, 0xd03c)?;
    c.emit(0xcf68, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x13)?)?;
    c.emit(0xcf6c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcf70, data_immediate(C::Al, Op::Orr, r(1), r(1), 0xa00)?)?;
    c.jump(0xcf74, C::Al, 0x17b0, true)?;
    c.emit(0xcf78, transfer_word(C::Al, true, r(1), r(5), 4)?)?;
    c.emit(0xcf7c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.jump(0xcf80, C::Eq, 0xcfa0, false)?;
    c.emit(0xcf84, transfer_byte(C::Al, true, r(0), r(1), 3)?)?;
    c.emit(0xcf88, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xf)?)?;
    c.jump(0xcf8c, C::Hi, 0xd028, false)?;
    c.emit(0xcf90, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.jump(0xcf94, C::Al, 0xbce4, true)?;
    c.emit(0xcf98, transfer_word(C::Al, false, r(0), r(5), 4)?)?;
    c.jump(0xcf9c, C::Al, 0xcfb4, false)?;
    c.emit(0xcfa0, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(0xcfa4, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0xcfa8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0xcfac, transfer_word(C::Al, false, r(0), r(5), 4)?)?;
    c.jump(0xcfb0, C::Eq, 0xd028, false)?;
    c.emit(0xcfb4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0xcfb8, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0xcfbc, data_immediate(C::Al, Op::Mov, r(7), r(0), 1)?)?;
    c.emit(0xcfc0, transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0xcfc4, transfer_word(C::Al, false, r(0), r(4), 4)?)?;
    c.emit(0xcfc8, transfer_word(C::Al, true, r(1), r(5), 4)?)?;
    c.emit(0xcfcc, transfer_multiple(C::Al, true, r(1), r(0).bit() | r(1).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0xcfd0, transfer_multiple(C::Al, false, r(4), r(0).bit() | r(1).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0xcfd4, data_immediate_with_flags(C::Ne, Op::And, r(0), r(0), 0x0f00_0000, true)?)?;
    c.jump(0xcfd8, C::Eq, 0xd034, false)?;
    c.emit(0xcfdc, transfer_word(C::Al, true, r(1), r(5), 4)?)?;
    c.emit(0xcfe0, transfer_byte(C::Al, true, r(0), r(1), 3)?)?;
    c.emit(0xcfe4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xf)?)?;
    c.jump(0xcfe8, C::Hi, 0xd034, false)?;
    c.emit(0xcfec, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.jump(0xcff0, C::Al, 0xbce4, true)?;
    c.emit(0xcff4, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xcff8, transfer_byte(C::Al, true, r(0), r(1), 3)?)?;
    c.emit(0xcffc, data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.jump(0xd000, C::Eq, 0xd034, false)?;
    c.emit(0xd004, transfer_word(C::Al, false, r(1), r(5), 4)?)?;
    c.emit(0xd008, transfer_word(C::Al, true, r(0), r(1), 4)?)?;
    c.emit(0xd00c, transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.emit(0xd010, data_register(C::Al, Op::Add, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0xd014, transfer_word(C::Al, false, r(0), r(4), 4)?)?;
    c.emit(0xd018, transfer_byte(C::Al, true, r(0), r(1), 3)?)?;
    c.emit(0xd01c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x10)?)?;
    c.jump(0xd020, C::Cc, 0xcfec, false)?;
    c.jump(0xd024, C::Al, 0xd034, false)?;
    c.emit(0xd028, data_immediate(C::Al, Op::Mov, r(7), r(0), 0)?)?;
    c.emit(0xd02c, transfer_word(C::Al, false, r(7), r(4), 0)?)?;
    c.emit(0xd030, transfer_word(C::Al, false, r(7), r(4), 4)?)?;
    c.emit(0xd034, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0xd038, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0xd03c, SOURCE)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    usable_size(&mut c)?;
    accounting_queries(&mut c)?;
    iterator_init(&mut c)?;
    iterator_reset(&mut c)?;
    iterator_advance(&mut c)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("heap queries iterator extent mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_extent_and_protected_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(PROTECTED_DATA.len() * 4, DATA_BYTES);
        for (at, value) in PROTECTED_DATA {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        let mut code_words = 0;
        for at in (START..END).step_by(4) {
            if !PROTECTED_DATA.iter().any(|&(address, _)| address == at) {
                let mut c = Code { at, bytes: Vec::new() };
                c.emit(at, bx(C::Al, R::LR)).unwrap();
                assert_eq!(c.at, at + 4);
                assert_eq!(c.bytes.len(), 4);
                code_words += 1;
            }
        }
        assert_eq!(code_words * 4, CODE_BYTES);
    }

    #[test]
    fn exact_literal_guards() {
        for (at, value) in PROTECTED_DATA {
            for wrong in [0, u32::MAX, value ^ 1] {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.data(at, wrong).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
            let mut c = Code { at: at - 4, bytes: Vec::new() };
            assert!(c.data(at, value).is_err());
            assert_eq!(c.at, at - 4);
            assert!(c.bytes.is_empty());
            let mut c = Code { at, bytes: Vec::new() };
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes, value.to_le_bytes());
        }
        for at in (START - 4..=END + 4).step_by(1) {
            if !PROTECTED_DATA.iter().any(|&(address, _)| address == at) {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.data(at, SOURCE).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
        }
    }

    #[test]
    fn code_cursor_extent_and_literal_targets() {
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        for target in [START, 0xce51, 0xcee0, END] {
            assert!(c.literal(START, target).is_err());
        }
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
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
}
