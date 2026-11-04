use arm32::{
    branch, data_immediate, mov_register, push_registers, transfer_multiple, transfer_word,
    transfer_word_postindexed, transfer_word_register, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};

pub const START: usize = 0xdf74;
pub const END: usize = 0xe3f8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1092;
pub const DATA_BYTES: usize = 64;

const LITERALS: [(usize, u32); 16] = [
    (0xe01c, 0xe3bda), (0xe020, 0xe2788), (0xe024, 0xe3bc3),
    (0xe144, 0x10dfa0), (0xe148, 0xe2573), (0xe14c, 0xe3c03),
    (0xe150, 0xe2788), (0xe154, 0xe3c21), (0xe158, 0xe3896),
    (0xe3d0, 0xe028), (0xe3d4, 0xe3896), (0xe3d8, 0xe3c4a),
    (0xe3dc, 0xe3c83), (0xe3e0, 0xe2788), (0xe3e4, 0xe3c9d),
    (0xe3e8, 0xe3cd9),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("service loop address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        let expected = LITERALS.iter().find(|&&(at, _)| at == self.at).map(|&(_, v)| v);
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || if data { expected != Some(value) } else { expected.is_some() }
        {
            return Err(format!("invalid service loop word at {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        self.word(instruction, false)
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        self.word(value, true)
    }

    fn immediate(&mut self, at: usize, condition: C, op: Op, dst: u8, src: u8, value: u32) -> Result<(), String> {
        self.emit(at, data_immediate(condition, op, r(dst), r(src), value)?)
    }

    fn mov(&mut self, at: usize, dst: u8, src: u8) -> Result<(), String> {
        self.emit(at, mov_register(C::Al, r(dst), r(src)))
    }

    fn transfer(&mut self, at: usize, load: bool, value: u8, base: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_word(C::Al, load, r(value), r(base), offset)?)
    }

    fn load_literal(&mut self, at: usize, dst: u8, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| address == slot) {
            return Err(format!("invalid service loop literal slot {slot:#x}"));
        }
        self.transfer(at, true, dst, 15, slot as i32 - at as i32 - 8)
    }

    fn jump(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }

    fn pop(&mut self, at: usize, registers: u16) -> Result<(), String> {
        self.emit(at, transfer_multiple(C::Al, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn pool(&mut self, start: usize, end: usize) -> Result<(), String> {
        self.check(start)?;
        for &(at, value) in &LITERALS {
            if (start..end).contains(&at) {
                self.literal(at, value)?;
            }
        }
        self.check(end)
    }
}

fn acquire(c: &mut Code) -> Result<(), String> {
    c.emit(0xdf74, push_registers(C::Al, 0x48f0)?)?;
    c.immediate(0xdf78, C::Al, Op::Add, 11, 13, 0x10)?;
    c.immediate(0xdf7c, C::Al, Op::Sub, 13, 13, 8)?;
    c.immediate(0xdf80, C::Al, Op::Add, 2, 13, 4)?;
    c.immediate(0xdf84, C::Al, Op::Mov, 1, 0, 1)?;
    c.mov(0xdf88, 5, 0)?;
    c.immediate(0xdf8c, C::Al, Op::Mov, 6, 0, 1)?;
    c.jump(0xdf90, C::Al, 0x253fc, true)?;
    c.mov(0xdf94, 4, 0)?;
    c.immediate(0xdf98, C::Al, Op::Cmp, 0, 4, 1)?;
    c.jump(0xdf9c, C::Ne, 0xdfbc, false)?;
    c.load_literal(0xdfa0, 1, 0xe024)?;
    c.immediate(0xdfa4, C::Al, Op::Mov, 0, 0, 2)?;
    c.jump(0xdfa8, C::Al, 0x2ec94, true)?;
    c.load_literal(0xdfac, 1, 0xe020)?;
    c.immediate(0xdfb0, C::Al, Op::Mov, 0, 0, 2)?;
    c.jump(0xdfb4, C::Al, 0x2ec94, true)?;
    c.jump(0xdfb8, C::Al, 0xe010, false)?;
    c.transfer(0xdfbc, true, 0, 5, 0x8c)?;
    c.transfer(0xdfc0, true, 2, 13, 4)?;
    c.load_literal(0xdfc4, 1, 0xe01c)?;
    c.transfer(0xdfc8, false, 2, 0, 0x1c)?;
    c.immediate(0xdfcc, C::Al, Op::Mov, 0, 0, 1)?;
    c.transfer(0xdfd0, true, 3, 2, 0xc)?;
    c.jump(0xdfd4, C::Al, 0x2ec94, true)?;
    c.load_literal(0xdfd8, 1, 0xe020)?;
    c.immediate(0xdfdc, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0xdfe0, C::Al, 0x2ec94, true)?;
    c.jump(0xdfe4, C::Al, 0x26754, true)?;
    c.transfer(0xdfe8, true, 1, 5, 0x8c)?;
    c.immediate(0xdfec, C::Al, Op::Mov, 3, 0, 0x264)?;
    c.immediate(0xdff0, C::Al, Op::Orr, 3, 3, 0x1c00)?;
    c.transfer(0xdff4, true, 2, 1, 0x1c)?;
    c.emit(0xdff8, transfer_word_register(C::Al, false, r(0), r(2), r(3), Shift::Lsl, 0)?)?;
    c.immediate(0xdffc, C::Al, Op::Mov, 0, 0, 0x204)?;
    c.immediate(0xe000, C::Al, Op::Mov, 3, 0, 0)?;
    c.immediate(0xe004, C::Al, Op::Orr, 0, 0, 0x1c00)?;
    c.emit(0xe008, transfer_word_register(C::Al, false, r(3), r(2), r(0), Shift::Lsl, 0)?)?;
    c.transfer(0xe00c, false, 6, 1, 0x18)?;
    c.mov(0xe010, 0, 4)?;
    c.immediate(0xe014, C::Al, Op::Sub, 13, 11, 0x10)?;
    c.pop(0xe018, 0x88f0)?;
    c.pool(0xe01c, 0xe028)
}

fn service_loop(c: &mut Code) -> Result<(), String> {
    c.emit(0xe028, push_registers(C::Al, 0x4ff0)?)?;
    c.immediate(0xe02c, C::Al, Op::Add, 11, 13, 0x1c)?;
    c.immediate(0xe030, C::Al, Op::Sub, 13, 13, 4)?;
    c.mov(0xe034, 4, 0)?;
    c.load_literal(0xe038, 0, 0xe144)?;
    c.load_literal(0xe03c, 1, 0xe148)?;
    c.transfer(0xe040, true, 8, 4, 0x8c)?;
    c.transfer(0xe044, false, 1, 0, 0)?;
    c.jump(0xe048, C::Al, 0x307c, true)?;
    c.load_literal(0xe04c, 6, 0xe150)?;
    c.load_literal(0xe050, 7, 0xe158)?;
    c.immediate(0xe054, C::Al, Op::Mov, 9, 0, 0)?;
    c.immediate(0xe058, C::Al, Op::Mov, 10, 0, 0)?;
    c.transfer(0xe05c, false, 9, 8, 0x18)?;
    c.load_literal(0xe060, 1, 0xe14c)?;
    c.immediate(0xe064, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0xe068, C::Al, 0x2ec94, true)?;
    c.immediate(0xe06c, C::Al, Op::Mov, 0, 0, 1)?;
    c.mov(0xe070, 1, 6)?;
    c.jump(0xe074, C::Al, 0x2ec94, true)?;
    c.transfer(0xe078, true, 0, 8, 0x14)?;
    c.jump(0xe07c, C::Al, 0x294c, true)?;
    c.transfer(0xe080, true, 0, 8, 0x14)?;
    c.jump(0xe084, C::Al, 0x293c, true)?;
    c.mov(0xe088, 5, 10)?;
    c.transfer(0xe08c, true, 0, 8, 0x18)?;
    c.immediate(0xe090, C::Al, Op::Cmp, 0, 0, 2)?;
    c.jump(0xe094, C::Eq, 0xe0b8, false)?;
    c.immediate(0xe098, C::Al, Op::Cmp, 0, 0, 1)?;
    c.jump(0xe09c, C::Eq, 0xe10c, false)?;
    c.immediate(0xe0a0, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xe0a4, C::Ne, 0xe11c, false)?;
    c.mov(0xe0a8, 0, 4)?;
    c.jump(0xe0ac, C::Al, START as u32, true)?;
    c.mov(0xe0b0, 10, 0)?;
    c.jump(0xe0b4, C::Al, 0xe134, false)?;
    c.jump(0xe0b8, C::Al, 0x26754, true)?;
    c.transfer(0xe0bc, true, 1, 4, 0x8c)?;
    c.immediate(0xe0c0, C::Al, Op::Mov, 2, 0, 0x26c)?;
    c.immediate(0xe0c4, C::Al, Op::Orr, 2, 2, 0x1c00)?;
    c.transfer(0xe0c8, true, 1, 1, 0x1c)?;
    c.emit(0xe0cc, transfer_word_register(C::Al, false, r(0), r(1), r(2), Shift::Lsl, 0)?)?;
    c.mov(0xe0d0, 0, 4)?;
    c.jump(0xe0d4, C::Al, 0x1a7fc, true)?;
    c.mov(0xe0d8, 10, 0)?;
    c.transfer(0xe0dc, true, 0, 4, 0x8c)?;
    c.load_literal(0xe0e0, 1, 0xe154)?;
    c.transfer(0xe0e4, true, 2, 0, 0x1c)?;
    c.immediate(0xe0e8, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0xe0ec, C::Al, 0x2ec94, true)?;
    c.immediate(0xe0f0, C::Al, Op::Mov, 0, 0, 1)?;
    c.mov(0xe0f4, 1, 6)?;
    c.jump(0xe0f8, C::Al, 0x2ec94, true)?;
    c.transfer(0xe0fc, true, 0, 4, 0x8c)?;
    c.transfer(0xe100, false, 9, 0, 0x18)?;
    c.transfer(0xe104, false, 9, 0, 0x1c)?;
    c.jump(0xe108, C::Al, 0xe134, false)?;
    c.mov(0xe10c, 0, 4)?;
    c.jump(0xe110, C::Al, 0xdab4, true)?;
    c.immediate(0xe114, C::Al, Op::Mov, 5, 0, 0)?;
    c.jump(0xe118, C::Al, 0xe08c, false)?;
    c.immediate(0xe11c, C::Al, Op::Mov, 1, 0, 0x61)?;
    c.immediate(0xe120, C::Al, Op::Mov, 0, 0, 0)?;
    c.mov(0xe124, 2, 7)?;
    c.immediate(0xe128, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.jump(0xe12c, C::Al, 0x17b0, true)?;
    c.mov(0xe130, 10, 5)?;
    c.immediate(0xe134, C::Al, Op::Mov, 5, 0, 0)?;
    c.immediate(0xe138, C::Al, Op::Cmp, 0, 10, 0)?;
    c.jump(0xe13c, C::Eq, 0xe08c, false)?;
    c.jump(0xe140, C::Al, 0xe060, false)?;
    c.pool(0xe144, 0xe15c)
}

fn setup(c: &mut Code) -> Result<(), String> {
    c.emit(0xe15c, push_registers(C::Al, 0x4830)?)?;
    c.immediate(0xe160, C::Al, Op::Add, 11, 13, 8)?;
    c.immediate(0xe164, C::Al, Op::Sub, 13, 13, 0x28)?;
    c.mov(0xe168, 4, 0)?;
    c.load_literal(0xe16c, 0, 0xe3d0)?;
    c.transfer(0xe170, false, 2, 13, 0xc)?;
    c.transfer(0xe174, false, 1, 13, 0x10)?;
    c.immediate(0xe178, C::Al, Op::Mov, 5, 0, 0)?;
    c.transfer(0xe17c, false, 0, 13, 0x14)?;
    c.immediate(0xe180, C::Al, Op::Mov, 0, 0, 0x2000)?;
    c.transfer(0xe184, false, 4, 13, 0x18)?;
    c.transfer(0xe188, false, 0, 13, 0x1c)?;
    c.transfer(0xe18c, false, 5, 13, 0x20)?;
    c.transfer(0xe190, false, 5, 13, 0x24)?;
    c.immediate(0xe194, C::Al, Op::Add, 0, 13, 0xc)?;
    c.transfer(0xe198, true, 1, 4, 0x8c)?;
    c.jump(0xe19c, C::Al, 0x29b4, true)?;
    c.immediate(0xe1a0, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xe1a4, C::Eq, 0xe1bc, false)?;
    c.load_literal(0xe1a8, 2, 0xe3d4)?;
    c.immediate(0xe1ac, C::Al, Op::Mov, 1, 0, 0x87)?;
    c.immediate(0xe1b0, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0xe1b4, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.jump(0xe1b8, C::Al, 0x17b0, true)?;
    c.transfer(0xe1bc, true, 0, 4, 0x8c)?;
    c.emit(0xe1c0, transfer_word_postindexed(C::Al, true, r(1), r(0), 4)?)?;
    c.jump(0xe1c4, C::Al, 0x2c44, true)?;
    c.load_literal(0xe1c8, 2, 0xe3d4)?;
    c.immediate(0xe1cc, C::Al, Op::Cmp, 0, 0, 0)?;
    c.immediate(0xe1d0, C::Al, Op::Mov, 1, 0, 0x8b)?;
    c.immediate(0xe1d4, C::Eq, Op::Mov, 5, 0, 1)?;
    c.immediate(0xe1d8, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.mov(0xe1dc, 0, 5)?;
    c.jump(0xe1e0, C::Al, 0x17b0, true)?;
    c.immediate(0xe1e4, C::Al, Op::Mov, 0, 0, 8)?;
    c.jump(0xe1e8, C::Al, 0x2efc, true)?;
    c.transfer(0xe1ec, true, 1, 4, 0x8c)?;
    c.immediate(0xe1f0, C::Al, Op::Cmp, 0, 0, 0)?;
    c.transfer(0xe1f4, false, 0, 1, 0x14)?;
    c.jump(0xe1f8, C::Ne, 0xe214, false)?;
    c.load_literal(0xe1fc, 2, 0xe3d4)?;
    c.immediate(0xe200, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0xe204, C::Al, Op::Mov, 1, 0, 0x190)?;
    c.jump(0xe208, C::Al, 0x17b0, true)?;
    c.transfer(0xe20c, true, 1, 4, 0x8c)?;
    c.transfer(0xe210, true, 0, 1, 0x14)?;
    c.transfer(0xe214, true, 1, 1, 4)?;
    c.immediate(0xe218, C::Al, Op::Mov, 2, 0, 1)?;
    c.jump(0xe21c, C::Al, 0x2924, true)?;
    c.transfer(0xe220, true, 0, 4, 0x8c)?;
    c.transfer(0xe224, true, 0, 0, 0x14)?;
    c.jump(0xe228, C::Al, 0x293c, true)?;
    setup_queue(c, 0xe22c, 8, 0x97, 0xe2b4, 0xe3d8, 0x1a0, 0xa1)?;
    setup_queue(c, 0xe2b4, 0xc, 0xa5, 0xe340, 0xe3e4, 0x1ae, 0xaf)?;
    setup_queue(c, 0xe340, 0x10, 0xb3, 0xe3c8, 0xe3e8, 0x1bc, 0xbd)?;
    c.immediate(0xe3c8, C::Al, Op::Sub, 13, 11, 8)?;
    c.pop(0xe3cc, 0x8830)?;
    c.pool(0xe3d0, 0xe3ec)
}

fn setup_queue(c: &mut Code, start: usize, field: i32, line: u32, next: u32, format: usize, failure_line: u32, assert_line: u32) -> Result<(), String> {
    c.immediate(start, C::Al, Op::Mov, 0, 0, 0x38)?;
    c.jump(start + 4, C::Al, 0x2efc, true)?;
    c.transfer(start + 8, true, 1, 4, 0x8c)?;
    c.load_literal(start + 12, 2, 0xe3d4)?;
    c.transfer(start + 16, false, 0, 1, field)?;
    c.immediate(start + 20, C::Al, Op::Mov, 1, 0, line)?;
    c.immediate(start + 24, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.jump(start + 28, C::Al, 0x17b0, true)?;
    c.transfer(start + 32, true, 1, 4, 0x8c)?;
    c.immediate(start + 36, C::Al, Op::Mov, 3, 0, 0x20)?;
    c.transfer(start + 40, true, 0, 1, field)?;
    c.transfer(start + 44, true, 1, 1, 0x14)?;
    c.transfer(start + 48, true, 2, 1, 0)?;
    c.transfer(start + 52, true, 1, 1, 4)?;
    c.transfer(start + 56, false, 2, 13, 0)?;
    c.transfer(start + 60, false, 1, 13, 4)?;
    c.immediate(start + 64, C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(start + 68, C::Al, Op::Mov, 2, 0, 4)?;
    c.jump(start + 72, C::Al, 0x8080c, true)?;
    c.immediate(start + 76, C::Al, Op::Cmp, 0, 0, 1)?;
    c.jump(start + 80, C::Eq, next, false)?;
    c.load_literal(start + 84, 1, format)?;
    c.load_literal(start + 88, 2, 0xe3dc)?;
    let mut at = start + 92;
    if failure_line == 0x1ae {
        c.immediate(at, C::Al, Op::Mov, 3, 0, 0xae)?;
        c.immediate(at + 4, C::Al, Op::Mov, 0, 0, 8)?;
        c.immediate(at + 8, C::Al, Op::Orr, 3, 3, 0x100)?;
        at += 12;
    } else {
        c.immediate(at, C::Al, Op::Mov, 0, 0, 8)?;
        c.immediate(at + 4, C::Al, Op::Mov, 3, 0, failure_line)?;
        at += 8;
    }
    c.jump(at, C::Al, 0x2ec94, true)?;
    c.load_literal(at + 4, 1, 0xe3e0)?;
    c.immediate(at + 8, C::Al, Op::Mov, 0, 0, 8)?;
    c.jump(at + 12, C::Al, 0x2ec94, true)?;
    c.load_literal(at + 16, 2, 0xe3d4)?;
    c.immediate(at + 20, C::Al, Op::Mov, 1, 0, assert_line)?;
    c.immediate(at + 24, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(at + 28, C::Al, Op::Orr, 1, 1, 0x100)?;
    c.jump(at + 32, C::Al, 0x17b0, true)?;
    c.check(next as usize)
}

fn tail_wrapper(c: &mut Code) -> Result<(), String> {
    c.transfer(0xe3ec, true, 0, 0, 0x8c)?;
    c.transfer(0xe3f0, true, 0, 0, 0)?;
    c.jump(0xe3f4, C::Al, 0x2b58, false)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    acquire(&mut c)?;
    service_loop(&mut c)?;
    setup(&mut c)?;
    tail_wrapper(&mut c)?;
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
    fn extent_literals_and_entries() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!((CODE_BYTES, DATA_BYTES), (168 + 284 + 628 + 12, 12 + 24 + 28));
        for &(at, value) in &LITERALS {
            assert_eq!(word(&bytes, at), value);
        }
        assert_eq!(word(&bytes, 0xe3d0), 0xe028);
        for (at, registers) in [(START, 0x48f0), (0xe028, 0x4ff0), (0xe15c, 0x4830)] {
            assert_eq!(word(&bytes, at), push_registers(C::Al, registers).unwrap());
        }
        for (at, target, condition) in [
            (0xe118, 0xe08c, C::Al), (0xe13c, 0xe08c, C::Eq),
            (0xe140, 0xe060, C::Al), (0xe3f4, 0x2b58, C::Al),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        for at in (0xe028..0xe144).step_by(4) {
            let instruction = word(&bytes, at);
            assert_ne!(instruction & 0x0fff8000, 0x08bd8000);
            assert_ne!(instruction, arm32::bx(C::Al, R::LR));
        }
    }

    #[test]
    fn exact_literal_guards() {
        for at in (START..END).step_by(4) {
            let expected = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, value)| value);
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(at, mov_register(C::Al, r(0), r(0))).is_err(), expected.is_some());
            for &(_, value) in &LITERALS {
                let mut c = Code { at, bytes: Vec::new() };
                let ok = expected == Some(value);
                assert_eq!(c.literal(at, value).is_ok(), ok);
                assert_eq!(c.at, at + if ok { 4 } else { 0 });
                assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
            }
            if let Some(value) = expected {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.literal(at, value ^ 1).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
        }
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.literal(at, 0xe028).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, mov_register(C::Al, r(0), r(0))).is_err());
        assert!(c.load_literal(START, 0, 0xe3d1).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn literal_loads_and_sensitive_operations() {
        let bytes = fragment().unwrap();
        for (at, dst, slot) in [
            (0xdfa0, 1, 0xe024), (0xdfac, 1, 0xe020), (0xdfc4, 1, 0xe01c),
            (0xdfd8, 1, 0xe020), (0xe038, 0, 0xe144), (0xe03c, 1, 0xe148),
            (0xe04c, 6, 0xe150), (0xe050, 7, 0xe158), (0xe060, 1, 0xe14c),
            (0xe0e0, 1, 0xe154), (0xe16c, 0, 0xe3d0),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(dst), R::PC, slot as i32 - at as i32 - 8).unwrap());
        }
        assert_eq!(word(&bytes, 0xe1c0), transfer_word_postindexed(C::Al, true, r(1), r(0), 4).unwrap());
        assert_eq!(word(&bytes, 0xe1d4), data_immediate(C::Eq, Op::Mov, r(5), r(0), 1).unwrap());
        for (at, dst, base, index) in [(0xdff8, 0, 2, 3), (0xe008, 3, 2, 0), (0xe0cc, 0, 1, 2)] {
            assert_eq!(word(&bytes, at), transfer_word_register(C::Al, false, r(dst), r(base), r(index), Shift::Lsl, 0).unwrap());
        }
        for at in [0xe274, 0xe2fc, 0xe388] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, 0x8080c, true).unwrap());
        }
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
