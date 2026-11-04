use arm32::{
    branch, bx, data_immediate, data_register, mov_register, transfer_multiple, transfer_word,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xd790;
pub const END: usize = 0xdab4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 760;
pub const DATA_BYTES: usize = 44;
const PROTECTED_DATA: [(usize, u32); 11] = [
    (0xd898, 0xe3869),
    (0xd89c, 0xe3881),
    (0xd8a0, 0xe2788),
    (0xd8a4, 0xe3896),
    (0xd8a8, 0xe38c5),
    (0xd8ac, 0xe38eb),
    (0xd97c, 0xe3911),
    (0xd980, 0xe2788),
    (0xd984, 0xe3933),
    (0xd988, 0xe395a),
    (0xd9c8, 0xe0008480),
];
const SECTIONS: [(usize, usize, usize, usize); 6] = [
    (START, 0xd8b0, 264, 24),
    (0xd8b0, 0xd98c, 204, 16),
    (0xd98c, 0xd9cc, 60, 4),
    (0xd9cc, 0xdaa4, 216, 0),
    (0xdaa4, 0xdaac, 8, 0),
    (0xdaac, END, 8, 0),
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
            return Err(format!("service queue cursor {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !(START..END).contains(&at) || at % 4 != 0
            || PROTECTED_DATA.iter().any(|&(address, _)| address == at)
        {
            return Err(format!("service queue instruction outside code: {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !PROTECTED_DATA.contains(&(at, value)) {
            return Err(format!("service queue invalid literal: {at:#x} = {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn immediate(&mut self, at: usize, op: Op, dst: u8, src: u8, value: u32) -> Result<(), String> {
        self.emit(at, data_immediate(C::Al, op, r(dst), r(src), value)?)
    }

    fn mov(&mut self, at: usize, dst: u8, src: u8) -> Result<(), String> {
        self.emit(at, mov_register(C::Al, r(dst), r(src)))
    }

    fn register(&mut self, at: usize, op: Op, dst: u8, src: u8, rhs: u8) -> Result<(), String> {
        self.emit(at, data_register(C::Al, op, r(dst), r(src), r(rhs), Shift::Lsl, 0, false)?)
    }

    fn word(&mut self, at: usize, load: bool, value: u8, base: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_word(C::Al, load, r(value), r(base), offset)?)
    }

    fn literal(&mut self, at: usize, dst: u8, target: usize) -> Result<(), String> {
        if !PROTECTED_DATA.iter().any(|&(address, _)| address == target) {
            return Err(format!("service queue invalid literal target: {target:#x}"));
        }
        self.word(at, true, dst, 15, target as i32 - at as i32 - 8)
    }

    fn jump(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }

    fn stack(&mut self, at: usize, restore: bool, registers: u16) -> Result<(), String> {
        self.emit(at, transfer_multiple(C::Al, restore, R::SP, registers,
            if restore { BlockMode::IncrementAfter } else { BlockMode::DecrementBefore }, true)?)
    }

    fn pool(&mut self, start: usize, end: usize) -> Result<(), String> {
        for (at, value) in PROTECTED_DATA {
            if (start..end).contains(&at) {
                self.data(at, value)?;
            }
        }
        self.check(end)
    }
}

fn queue_selection(c: &mut Code) -> Result<(), String> {
    c.stack(0xd790, false, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit())?;
    c.immediate(0xd794, Op::Add, 11, 13, 8)?;
    c.immediate(0xd798, Op::Sub, 13, 13, 8)?;
    c.mov(0xd79c, 4, 0)?;
    c.immediate(0xd7a0, Op::Cmp, 0, 2, 0)?;
    c.word(0xd7a4, false, 1, 13, 4)?;
    c.jump(0xd7a8, C::Eq, 0xd7c8, false)?;
    c.immediate(0xd7ac, Op::Cmp, 0, 2, 2)?;
    c.jump(0xd7b0, C::Eq, 0xd7d4, false)?;
    c.immediate(0xd7b4, Op::Cmp, 0, 2, 1)?;
    c.jump(0xd7b8, C::Ne, 0xd7e0, false)?;
    c.word(0xd7bc, true, 0, 4, 0x8c)?;
    c.word(0xd7c0, true, 5, 0, 8)?;
    c.jump(0xd7c4, C::Al, 0xd818, false)?;
    c.word(0xd7c8, true, 0, 4, 0x8c)?;
    c.word(0xd7cc, true, 5, 0, 12)?;
    c.jump(0xd7d0, C::Al, 0xd818, false)?;
    c.word(0xd7d4, true, 0, 4, 0x8c)?;
    c.word(0xd7d8, true, 5, 0, 16)?;
    c.jump(0xd7dc, C::Al, 0xd818, false)?;
    c.word(0xd7e0, false, 2, 13, 0)?;
    c.literal(0xd7e4, 1, 0xd898)?;
    c.literal(0xd7e8, 2, 0xd89c)?;
    c.immediate(0xd7ec, Op::Mov, 0, 0, 8)?;
    c.immediate(0xd7f0, Op::Mov, 3, 0, 0x26)?;
    c.jump(0xd7f4, C::Al, 0x2ec94, true)?;
    c.literal(0xd7f8, 1, 0xd8a0)?;
    c.immediate(0xd7fc, Op::Mov, 0, 0, 8)?;
    c.jump(0xd800, C::Al, 0x2ec94, true)?;
    c.literal(0xd804, 2, 0xd8a4)?;
    c.immediate(0xd808, Op::Mov, 0, 0, 0)?;
    c.immediate(0xd80c, Op::Mov, 1, 0, 0x27)?;
    c.immediate(0xd810, Op::Mov, 5, 0, 0)?;
    c.jump(0xd814, C::Al, 0x17b0, true)?;
    c.immediate(0xd818, Op::Add, 1, 13, 4)?;
    c.mov(0xd81c, 0, 5)?;
    c.jump(0xd820, C::Al, 0x80928, true)?;
    c.immediate(0xd824, Op::Cmp, 0, 0, 1)?;
    c.jump(0xd828, C::Ne, 0xd85c, false)?;
    c.word(0xd82c, true, 2, 13, 4)?;
    c.literal(0xd830, 1, 0xd8ac)?;
    c.immediate(0xd834, Op::Mov, 0, 0, 1)?;
    c.word(0xd838, true, 3, 2, 12)?;
    c.jump(0xd83c, C::Al, 0x2ec94, true)?;
    c.literal(0xd840, 1, 0xd8a0)?;
    c.immediate(0xd844, Op::Mov, 0, 0, 1)?;
    c.jump(0xd848, C::Al, 0x2ec94, true)?;
    c.word(0xd84c, true, 0, 4, 0x8c)?;
    c.word(0xd850, true, 0, 0, 20)?;
    c.jump(0xd854, C::Al, 0x292c, true)?;
    c.jump(0xd858, C::Al, 0xd88c, false)?;
    c.literal(0xd85c, 1, 0xd8a8)?;
    c.literal(0xd860, 2, 0xd89c)?;
    c.immediate(0xd864, Op::Mov, 0, 0, 8)?;
    c.immediate(0xd868, Op::Mov, 3, 0, 0x2c)?;
    c.jump(0xd86c, C::Al, 0x2ec94, true)?;
    c.literal(0xd870, 1, 0xd8a0)?;
    c.immediate(0xd874, Op::Mov, 0, 0, 8)?;
    c.jump(0xd878, C::Al, 0x2ec94, true)?;
    c.literal(0xd87c, 2, 0xd8a4)?;
    c.immediate(0xd880, Op::Mov, 0, 0, 0)?;
    c.immediate(0xd884, Op::Mov, 1, 0, 0x2d)?;
    c.jump(0xd888, C::Al, 0x17b0, true)?;
    c.immediate(0xd88c, Op::Mov, 0, 0, 0)?;
    c.immediate(0xd890, Op::Sub, 13, 11, 8)?;
    c.stack(0xd894, true, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit())?;
    c.pool(0xd898, 0xd8b0)
}

fn cleanup(c: &mut Code) -> Result<(), String> {
    let saved = r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit();
    c.stack(0xd8b0, false, saved | R::LR.bit())?;
    c.immediate(0xd8b4, Op::Add, 11, 13, 16)?;
    c.immediate(0xd8b8, Op::Sub, 13, 13, 8)?;
    c.mov(0xd8bc, 4, 0)?;
    c.mov(0xd8c0, 6, 2)?;
    c.mov(0xd8c4, 5, 3)?;
    c.mov(0xd8c8, 7, 1)?;
    c.word(0xd8cc, true, 0, 4, 0x8c)?;
    c.word(0xd8d0, true, 2, 0, 28)?;
    c.immediate(0xd8d4, Op::Cmp, 0, 2, 0)?;
    c.jump(0xd8d8, C::Eq, 0xd974, false)?;
    c.literal(0xd8dc, 1, 0xd97c)?;
    c.immediate(0xd8e0, Op::Mov, 0, 0, 2)?;
    c.jump(0xd8e4, C::Al, 0x2ec94, true)?;
    c.literal(0xd8e8, 1, 0xd980)?;
    c.immediate(0xd8ec, Op::Mov, 0, 0, 2)?;
    c.jump(0xd8f0, C::Al, 0x2ec94, true)?;
    c.word(0xd8f4, true, 0, 4, 0x8c)?;
    c.word(0xd8f8, true, 0, 0, 28)?;
    c.word(0xd8fc, true, 0, 0, 12)?;
    c.register(0xd900, Op::Cmp, 0, 0, 7)?;
    c.jump(0xd904, C::Ne, 0xd974, false)?;
    c.literal(0xd908, 1, 0xd984)?;
    c.literal(0xd90c, 2, 0xd988)?;
    c.immediate(0xd910, Op::Mov, 0, 0, 8)?;
    c.immediate(0xd914, Op::Mov, 3, 0, 0x45)?;
    c.word(0xd918, false, 7, 13, 0)?;
    c.word(0xd91c, false, 7, 13, 4)?;
    c.jump(0xd920, C::Al, 0x2ec94, true)?;
    c.literal(0xd924, 1, 0xd980)?;
    c.immediate(0xd928, Op::Mov, 0, 0, 8)?;
    c.jump(0xd92c, C::Al, 0x2ec94, true)?;
    c.immediate(0xd930, Op::Cmp, 0, 6, 0)?;
    c.jump(0xd934, C::Eq, 0xd950, false)?;
    c.word(0xd938, true, 0, 4, 0x8c)?;
    c.immediate(0xd93c, Op::Mov, 2, 0, 7)?;
    c.immediate(0xd940, Op::Orr, 2, 2, 0x2000)?;
    c.word(0xd944, true, 1, 0, 28)?;
    c.mov(0xd948, 0, 4)?;
    c.jump(0xd94c, C::Al, 0x24f90, true)?;
    c.immediate(0xd950, Op::Cmp, 0, 5, 0)?;
    c.jump(0xd954, C::Eq, 0xd968, false)?;
    c.word(0xd958, true, 1, 4, 0x8c)?;
    c.word(0xd95c, true, 0, 4, 0xb4)?;
    c.word(0xd960, true, 1, 1, 28)?;
    c.jump(0xd964, C::Al, 0x7fd1c, true)?;
    c.word(0xd968, true, 0, 4, 0x8c)?;
    c.immediate(0xd96c, Op::Mov, 1, 0, 0)?;
    c.word(0xd970, false, 1, 0, 28)?;
    c.immediate(0xd974, Op::Sub, 13, 11, 16)?;
    c.stack(0xd978, true, saved | R::PC.bit())?;
    c.pool(0xd97c, 0xd98c)
}

fn nine_word_mmio_copy(c: &mut Code) -> Result<(), String> {
    c.literal(0xd98c, 2, 0xd9c8)?;
    c.immediate(0xd990, Op::Add, 0, 1, 0x1f8)?;
    c.immediate(0xd994, Op::Mov, 1, 0, 0)?;
    c.immediate(0xd998, Op::Add, 0, 0, 0x400)?;
    c.word(0xd99c, true, 3, 2, 0)?;
    c.immediate(0xd9a0, Op::Add, 1, 1, 1)?;
    c.immediate(0xd9a4, Op::Cmp, 0, 1, 9)?;
    c.word(0xd9a8, false, 3, 0, 0)?;
    c.word(0xd9ac, true, 3, 2, 0x40)?;
    c.immediate(0xd9b0, Op::Add, 2, 2, 4)?;
    c.word(0xd9b4, false, 3, 0, 0x24)?;
    c.immediate(0xd9b8, Op::Add, 0, 0, 4)?;
    c.jump(0xd9bc, C::Ne, 0xd99c, false)?;
    c.immediate(0xd9c0, Op::Mov, 0, 0, 1)?;
    c.emit(0xd9c4, bx(C::Al, R::LR))?;
    c.pool(0xd9c8, 0xd9cc)
}

fn unpack(c: &mut Code) -> Result<(), String> {
    let saved = r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit()
        | r(8).bit() | r(9).bit() | r(11).bit();
    c.stack(0xd9cc, false, saved | R::LR.bit())?;
    c.immediate(0xd9d0, Op::Add, 11, 13, 24)?;
    c.immediate(0xd9d4, Op::Sub, 13, 13, 0xa8)?;
    c.immediate(0xd9d8, Op::Add, 0, 13, 24)?;
    c.immediate(0xd9dc, Op::Mov, 3, 0, 0xff)?;
    c.mov(0xd9e0, 8, 1)?;
    c.immediate(0xd9e4, Op::Add, 1, 13, 0x60)?;
    c.immediate(0xd9e8, Op::Mov, 2, 0, 0)?;
    c.immediate(0xd9ec, Op::Mov, 6, 0, 0)?;
    c.immediate(0xd9f0, Op::Add, 0, 0, 0x24)?;
    c.immediate(0xd9f4, Op::Add, 1, 1, 0x24)?;
    c.immediate(0xd9f8, Op::Orr, 3, 3, 0xff00)?;
    c.register(0xd9fc, Op::Add, 5, 8, 2)?;
    c.immediate(0xda00, Op::Add, 6, 6, 1)?;
    c.word(0xda04, true, 4, 5, 0x61c)?;
    c.immediate(0xda08, Op::Cmp, 0, 6, 9)?;
    c.register(0xda0c, Op::And, 7, 4, 3)?;
    c.emit(0xda10, data_register(C::Al, Op::Mov, r(4), r(0), r(4), Shift::Lsr, 16, false)?)?;
    c.emit(0xda14, transfer_word_register(C::Al, false, r(7), r(1), r(2), Shift::Lsl, 0)?)?;
    c.register(0xda18, Op::Sub, 7, 1, 2)?;
    c.word(0xda1c, false, 4, 7, -4)?;
    c.word(0xda20, true, 7, 5, 0x5f8)?;
    c.register(0xda24, Op::And, 5, 7, 3)?;
    c.emit(0xda28, data_register(C::Al, Op::Mov, r(7), r(0), r(7), Shift::Lsr, 16, false)?)?;
    c.emit(0xda2c, transfer_word_register(C::Al, false, r(5), r(0), r(2), Shift::Lsl, 0)?)?;
    c.register(0xda30, Op::Sub, 5, 0, 2)?;
    c.immediate(0xda34, Op::Add, 2, 2, 4)?;
    c.word(0xda38, false, 7, 5, -4)?;
    c.jump(0xda3c, C::Ne, 0xd9fc, false)?;
    c.immediate(0xda40, Op::Add, 5, 13, 0x60)?;
    c.immediate(0xda44, Op::Add, 6, 13, 24)?;
    c.immediate(0xda48, Op::Add, 2, 13, 20)?;
    c.immediate(0xda4c, Op::Add, 3, 13, 16)?;
    c.word(0xda50, false, 8, 13, 0)?;
    c.mov(0xda54, 0, 5)?;
    c.mov(0xda58, 1, 6)?;
    c.jump(0xda5c, C::Al, 0x1a4d4, true)?;
    c.word(0xda60, true, 0, 13, 20)?;
    c.immediate(0xda64, Op::Add, 2, 13, 12)?;
    c.immediate(0xda68, Op::Add, 3, 13, 8)?;
    c.mov(0xda6c, 1, 6)?;
    c.word(0xda70, false, 0, 8, 0x940)?;
    c.word(0xda74, true, 0, 13, 16)?;
    c.word(0xda78, false, 0, 8, 0x944)?;
    c.mov(0xda7c, 0, 5)?;
    c.word(0xda80, false, 8, 13, 0)?;
    c.jump(0xda84, C::Al, 0x1a670, true)?;
    c.word(0xda88, true, 0, 13, 12)?;
    c.word(0xda8c, false, 0, 8, 0x94c)?;
    c.word(0xda90, true, 0, 13, 8)?;
    c.word(0xda94, false, 0, 8, 0x950)?;
    c.immediate(0xda98, Op::Mov, 0, 0, 1)?;
    c.immediate(0xda9c, Op::Sub, 13, 11, 24)?;
    c.stack(0xdaa0, true, saved | R::PC.bit())?;
    c.check(0xdaa4)
}

fn return_one(c: &mut Code, start: usize) -> Result<(), String> {
    c.immediate(start, Op::Mov, 0, 0, 1)?;
    c.emit(start + 4, bx(C::Al, R::LR))
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    queue_selection(&mut c)?;
    cleanup(&mut c)?;
    nine_word_mmio_copy(&mut c)?;
    unpack(&mut c)?;
    return_one(&mut c, 0xdaa4)?;
    return_one(&mut c, 0xdaac)?;
    c.check(END)?;
    let mut code_total = 0;
    let mut data_total = 0;
    for (start, end, code, data) in SECTIONS {
        let actual_data = PROTECTED_DATA.iter().filter(|&&(at, _)| (start..end).contains(&at)).count() * 4;
        if actual_data != data || end - start != code + data {
            return Err(format!("service queue section extent mismatch: {start:#x}"));
        }
        code_total += code;
        data_total += data;
    }
    if code_total != CODE_BYTES || data_total != DATA_BYTES || c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("service queue extent mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_sections_and_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(PROTECTED_DATA.len() * 4, DATA_BYTES);
        let mut next = START;
        for (start, end, code, data) in SECTIONS {
            assert_eq!(start, next);
            let actual_data = PROTECTED_DATA.iter().filter(|&&(at, _)| (start..end).contains(&at)).count() * 4;
            assert_eq!(actual_data, data);
            assert_eq!(end - start - actual_data, code);
            next = end;
        }
        assert_eq!(next, END);
        for (at, value) in PROTECTED_DATA {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        for start in [0xdaa4, 0xdaac] {
            assert_eq!(&bytes[start - START..start - START + 4], &data_immediate(C::Al, Op::Mov, r(0), r(0), 1).unwrap().to_le_bytes());
            assert_eq!(&bytes[start - START + 4..start - START + 8], &bx(C::Al, R::LR).to_le_bytes());
        }
    }

    #[test]
    fn exact_code_and_data_guards() {
        let mut code_bytes = 0;
        for at in START - 4..=END + 4 {
            let literal = PROTECTED_DATA.iter().find(|&&(address, _)| address == at);
            let valid_code = (START..END).contains(&at) && at % 4 == 0 && literal.is_none();
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(at, bx(C::Al, R::LR)).is_ok(), valid_code);
            assert_eq!(c.at, at + if valid_code { 4 } else { 0 });
            assert_eq!(c.bytes.len(), if valid_code { 4 } else { 0 });
            if valid_code { code_bytes += 4; }
            let mut c = Code { at, bytes: Vec::new() };
            let value = literal.map_or(0, |&(_, value)| value);
            assert_eq!(c.data(at, value).is_ok(), literal.is_some());
            assert_eq!(c.at, at + if literal.is_some() { 4 } else { 0 });
            assert_eq!(c.bytes.len(), if literal.is_some() { 4 } else { 0 });
        }
        assert_eq!(code_bytes, CODE_BYTES);
        for (at, value) in PROTECTED_DATA {
            for wrong in [0, u32::MAX, value ^ 1] {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.data(at, wrong).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
            let mut c = Code { at: at - 4, bytes: Vec::new() };
            assert!(c.data(at, value).is_err());
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert_eq!(c.at, at - 4);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        for target in [START, 0xd899, 0xd978, END] {
            assert!(c.literal(START, 2, target).is_err());
        }
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn branches_and_literal_loads() {
        let bytes = fragment().unwrap();
        let word = |at| u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap());
        for (at, dst, target) in [(0xd7e4, 1, 0xd898), (0xd830, 1, 0xd8ac),
            (0xd860, 2, 0xd89c), (0xd8dc, 1, 0xd97c), (0xd908, 1, 0xd984),
            (0xd90c, 2, 0xd988), (0xd98c, 2, 0xd9c8)] {
            assert_eq!(word(at), transfer_word(C::Al, true, r(dst), R::PC, target as i32 - at as i32 - 8).unwrap());
        }
        for (at, condition, target, link) in [(0xd7a8, C::Eq, 0xd7c8, false),
            (0xd820, C::Al, 0x80928, true), (0xd904, C::Ne, 0xd974, false),
            (0xd94c, C::Al, 0x24f90, true), (0xd964, C::Al, 0x7fd1c, true),
            (0xd9bc, C::Ne, 0xd99c, false), (0xda3c, C::Ne, 0xd9fc, false),
            (0xda5c, C::Al, 0x1a4d4, true), (0xda84, C::Al, 0x1a670, true)] {
            assert_eq!(word(at), branch(condition, at as u32, target, link).unwrap());
        }
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
