use arm32::{
    branch, bx, data_immediate, data_immediate_with_flags, data_register, mov_register,
    push_registers, transfer_multiple, transfer_word, transfer_word_postindexed, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7fd1c;
pub const END: usize = 0x804fc;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1928;
pub const DATA_BYTES: usize = 88;

const LITERALS: [(usize, u32); 22] = [
    (0x7feb8, 0xf478b),
    (0x7febc, 0xf4a0f),
    (0x7fec0, 0xf4a55),
    (0x7fec4, 0xe2788),
    (0x7fec8, 0xf4a69),
    (0x7fecc, 0xf4a9c),
    (0x7ffb4, 0xf478b),
    (0x7ffb8, 0xf4ad9),
    (0x7ffbc, 0xf4b0e),
    (0x7ffc0, 0xe2788),
    (0x7ffc4, 0xf4b1b),
    (0x8001c, 0xf4b5d),
    (0x80020, 0xe2788),
    (0x8011c, 0xf478b),
    (0x80120, 0xf4b7c),
    (0x80124, 0xe2788),
    (0x80200, 0xf478b),
    (0x80244, 0xf478b),
    (0x803b0, 0xf478b),
    (0x803b4, 0xf499d),
    (0x803b8, 0xf4bac),
    (0x803bc, 0xe2788),
];
const ASSERT: usize = 0x17b0;
const LOCK: usize = 0x2bf4;
const UNLOCK: usize = 0x2c34;
const LOG: usize = 0x2ec94;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("VBuffer address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, at: usize, value: u32, data: bool) -> Result<(), String> {
        self.check(at)?;
        let literal = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, v)| v);
        if !(START..END).contains(&at) || at % 4 != 0
            || if data { literal != Some(value) } else { literal.is_some() }
        {
            return Err(format!("invalid VBuffer {} at {at:#x}", if data { "data" } else { "code" }));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.word(at, instruction, false)
    }

    fn immediate(&mut self, at: usize, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(at, data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn mov(&mut self, at: usize, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(at, mov_register(C::Al, r(rd), r(rm)))
    }

    fn cmp(&mut self, at: usize, rn: u8, rm: u8) -> Result<(), String> {
        self.emit(at, data_register(C::Al, Op::Cmp, r(0), r(rn), r(rm), Shift::Lsl, 0, false)?)
    }

    fn transfer(&mut self, at: usize, condition: C, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_word(condition, load, r(rd), r(rn), offset)?)
    }

    fn register(&mut self, at: usize, condition: C, op: Op, rd: u8, rn: u8, rm: u8, shift: u8) -> Result<(), String> {
        self.emit(at, data_register(condition, op, r(rd), r(rn), r(rm), Shift::Lsl, shift, false)?)
    }

    fn literal(&mut self, at: usize, rd: u8, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| address == slot) {
            return Err(format!("invalid VBuffer literal {slot:#x}"));
        }
        self.transfer(at, C::Al, true, rd, 15, slot as i32 - at as i32 - 8)
    }

    fn jump(&mut self, at: usize, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target as u32, link)?)
    }

    fn push(&mut self, at: usize, registers: u16) -> Result<(), String> {
        self.emit(at, push_registers(C::Al, registers)?)
    }

    fn pop(&mut self, at: usize, condition: C, registers: u16) -> Result<(), String> {
        self.emit(at, transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }
}

fn release_by_handle(c: &mut Code) -> Result<(), String> {
    c.immediate(0x7fd1c, C::Al, Op::Mov, 2, 0, 0x0)?;
    c.immediate(0x7fd20, C::Al, Op::Mov, 3, 0, 0x0)?;
    c.jump(0x7fd24, C::Al, 0x7fd28, false)?;
    Ok(())
}
fn release_by_handle_with_state(c: &mut Code) -> Result<(), String> {
    c.push(0x7fd28, 0x4ff0)?;
    c.immediate(0x7fd2c, C::Al, Op::Add, 11, 13, 0x1c)?;
    c.immediate(0x7fd30, C::Al, Op::Sub, 13, 13, 0x14)?;
    c.transfer(0x7fd34, C::Al, false, 2, 13, 16)?;
    c.mov(0x7fd38, 4, 0)?;
    c.mov(0x7fd3c, 9, 3)?;
    c.mov(0x7fd40, 6, 1)?;
    c.transfer(0x7fd44, C::Al, true, 0, 4, 16)?;
    c.immediate(0x7fd48, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x7fd4c, C::Eq, 0x7fdac, false)?;
    c.transfer(0x7fd50, C::Al, true, 0, 4, 4)?;
    c.literal(0x7fd54, 7, 0x7feb8)?;
    c.immediate(0x7fd58, C::Al, Op::Mov, 10, 0, 0x154)?;
    c.immediate(0x7fd5c, C::Al, Op::Mov, 8, 0, 0x0)?;
    c.immediate(0x7fd60, C::Al, Op::Mov, 5, 0, 0x0)?;
    c.immediate(0x7fd64, C::Al, Op::Orr, 10, 10, 0x400)?;
    c.emit(0x7fd68, transfer_word_register(C::Al, true, r(0), r(0), r(8), Shift::Lsl, 0)?)?;
    c.mov(0x7fd6c, 1, 10)?;
    c.mov(0x7fd70, 2, 7)?;
    c.cmp(0x7fd74, 0, 5)?;
    c.immediate(0x7fd78, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x7fd7c, C::Eq, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x7fd80, C::Al, ASSERT, true)?;
    c.transfer(0x7fd84, C::Al, true, 0, 4, 4)?;
    c.register(0x7fd88, C::Al, Op::Add, 1, 0, 8, 0)?;
    c.transfer(0x7fd8c, C::Al, true, 2, 1, 16)?;
    c.cmp(0x7fd90, 2, 6)?;
    c.jump(0x7fd94, C::Eq, 0x7fdec, false)?;
    c.transfer(0x7fd98, C::Al, true, 1, 4, 16)?;
    c.immediate(0x7fd9c, C::Al, Op::Add, 5, 5, 0x1)?;
    c.immediate(0x7fda0, C::Al, Op::Add, 8, 8, 0x28)?;
    c.cmp(0x7fda4, 5, 1)?;
    c.jump(0x7fda8, C::Cc, 0x7fd68, false)?;
    c.literal(0x7fdac, 1, 0x7fecc)?;
    c.literal(0x7fdb0, 2, 0x7fec0)?;
    c.immediate(0x7fdb4, C::Al, Op::Mov, 3, 0, 0x76)?;
    c.immediate(0x7fdb8, C::Al, Op::Mov, 0, 0, 0x8)?;
    c.immediate(0x7fdbc, C::Al, Op::Orr, 3, 3, 0x500)?;
    c.jump(0x7fdc0, C::Al, LOG, true)?;
    c.literal(0x7fdc4, 1, 0x7fec4)?;
    c.immediate(0x7fdc8, C::Al, Op::Mov, 0, 0, 0x8)?;
    c.jump(0x7fdcc, C::Al, LOG, true)?;
    c.literal(0x7fdd0, 2, 0x7feb8)?;
    c.immediate(0x7fdd4, C::Al, Op::Mov, 1, 0, 0x77)?;
    c.immediate(0x7fdd8, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x7fddc, C::Al, Op::Mov, 6, 0, 0x0)?;
    c.immediate(0x7fde0, C::Al, Op::Orr, 1, 1, 0x500)?;
    c.jump(0x7fde4, C::Al, ASSERT, true)?;
    c.jump(0x7fde8, C::Al, 0x7feac, false)?;
    c.transfer(0x7fdec, C::Al, true, 0, 1, 4)?;
    c.immediate(0x7fdf0, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x7fdf4, C::Ne, 0x7fe3c, false)?;
    c.literal(0x7fdf8, 1, 0x7febc)?;
    c.literal(0x7fdfc, 2, 0x7fec0)?;
    c.immediate(0x7fe00, C::Al, Op::Mov, 3, 0, 0x5b)?;
    c.immediate(0x7fe04, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.emit(0x7fe08, transfer_multiple(C::Al, false, R::SP, 0x60, BlockMode::IncrementAfter, false)?)?;
    c.transfer(0x7fe0c, C::Al, false, 0, 13, 8)?;
    c.immediate(0x7fe10, C::Al, Op::Orr, 3, 3, 0x500)?;
    c.immediate(0x7fe14, C::Al, Op::Mov, 0, 0, 0x8)?;
    c.jump(0x7fe18, C::Al, LOG, true)?;
    c.literal(0x7fe1c, 1, 0x7fec4)?;
    c.immediate(0x7fe20, C::Al, Op::Mov, 0, 0, 0x8)?;
    c.jump(0x7fe24, C::Al, LOG, true)?;
    c.literal(0x7fe28, 2, 0x7feb8)?;
    c.immediate(0x7fe2c, C::Al, Op::Mov, 1, 0, 0x15c)?;
    c.immediate(0x7fe30, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x7fe34, C::Al, Op::Orr, 1, 1, 0x400)?;
    c.jump(0x7fe38, C::Al, ASSERT, true)?;
    c.transfer(0x7fe3c, C::Al, true, 0, 4, 0)?;
    c.jump(0x7fe40, C::Al, LOCK, true)?;
    c.transfer(0x7fe44, C::Al, true, 0, 4, 4)?;
    c.immediate(0x7fe48, C::Al, Op::Cmp, 0, 9, 0x1)?;
    c.register(0x7fe4c, C::Al, Op::Add, 0, 0, 8, 0)?;
    c.transfer(0x7fe50, C::Al, true, 1, 0, 4)?;
    c.immediate(0x7fe54, C::Al, Op::Sub, 6, 1, 0x1)?;
    c.transfer(0x7fe58, C::Al, false, 6, 0, 4)?;
    c.transfer(0x7fe5c, C::Eq, true, 1, 13, 16)?;
    c.transfer(0x7fe60, C::Eq, false, 1, 0, 32)?;
    c.immediate(0x7fe64, C::Al, Op::Cmp, 0, 6, 0x0)?;
    c.transfer(0x7fe68, C::Eq, true, 0, 4, 24)?;
    c.immediate(0x7fe6c, C::Eq, Op::Add, 0, 0, 0x1)?;
    c.transfer(0x7fe70, C::Eq, false, 0, 4, 24)?;
    c.transfer(0x7fe74, C::Al, true, 0, 4, 0)?;
    c.jump(0x7fe78, C::Al, UNLOCK, true)?;
    c.transfer(0x7fe7c, C::Al, true, 0, 4, 4)?;
    c.mov(0x7fe80, 2, 5)?;
    c.register(0x7fe84, C::Al, Op::Add, 0, 0, 8, 0)?;
    c.transfer(0x7fe88, C::Al, true, 1, 0, 4)?;
    c.transfer(0x7fe8c, C::Al, true, 3, 0, 16)?;
    c.immediate(0x7fe90, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x7fe94, C::Al, false, 1, 13, 0)?;
    c.literal(0x7fe98, 1, 0x7fec8)?;
    c.jump(0x7fe9c, C::Al, LOG, true)?;
    c.literal(0x7fea0, 1, 0x7fec4)?;
    c.immediate(0x7fea4, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x7fea8, C::Al, LOG, true)?;
    c.mov(0x7feac, 0, 6)?;
    c.immediate(0x7feb0, C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.pop(0x7feb4, C::Al, 0x8ff0)?;
    c.word(0x7feb8, 0xf478b, true)?;
    c.word(0x7febc, 0xf4a0f, true)?;
    c.word(0x7fec0, 0xf4a55, true)?;
    c.word(0x7fec4, 0xe2788, true)?;
    c.word(0x7fec8, 0xf4a69, true)?;
    c.word(0x7fecc, 0xf4a9c, true)?;
    Ok(())
}
fn release_record(c: &mut Code) -> Result<(), String> {
    c.push(0x7fed0, 0x48f0)?;
    c.immediate(0x7fed4, C::Al, Op::Add, 11, 13, 0x10)?;
    c.immediate(0x7fed8, C::Al, Op::Sub, 13, 13, 0x8)?;
    c.mov(0x7fedc, 5, 0)?;
    c.transfer(0x7fee0, C::Al, true, 0, 5, 12)?;
    c.transfer(0x7fee4, C::Al, true, 4, 5, 36)?;
    c.immediate(0x7fee8, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x7feec, C::Ne, 0x7ff04, false)?;
    c.literal(0x7fef0, 2, 0x7ffb4)?;
    c.immediate(0x7fef4, C::Al, Op::Mov, 1, 0, 0x8e)?;
    c.immediate(0x7fef8, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x7fefc, C::Al, Op::Orr, 1, 1, 0x500)?;
    c.jump(0x7ff00, C::Al, ASSERT, true)?;
    c.transfer(0x7ff04, C::Al, true, 0, 5, 4)?;
    c.immediate(0x7ff08, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x7ff0c, C::Ne, 0x7ff58, false)?;
    c.transfer(0x7ff10, C::Al, true, 0, 5, 16)?;
    c.immediate(0x7ff14, C::Al, Op::Mov, 1, 0, 0x0)?;
    c.literal(0x7ff18, 2, 0x7ffbc)?;
    c.immediate(0x7ff1c, C::Al, Op::Mov, 3, 0, 0x19c)?;
    c.transfer(0x7ff20, C::Al, false, 1, 13, 0)?;
    c.literal(0x7ff24, 1, 0x7ffb8)?;
    c.immediate(0x7ff28, C::Al, Op::Orr, 3, 3, 0x400)?;
    c.transfer(0x7ff2c, C::Al, false, 0, 13, 4)?;
    c.immediate(0x7ff30, C::Al, Op::Mov, 0, 0, 0x10)?;
    c.jump(0x7ff34, C::Al, LOG, true)?;
    c.literal(0x7ff38, 1, 0x7ffc0)?;
    c.immediate(0x7ff3c, C::Al, Op::Mov, 0, 0, 0x10)?;
    c.jump(0x7ff40, C::Al, LOG, true)?;
    c.literal(0x7ff44, 2, 0x7ffb4)?;
    c.immediate(0x7ff48, C::Al, Op::Mov, 1, 0, 0x9d)?;
    c.immediate(0x7ff4c, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x7ff50, C::Al, Op::Orr, 1, 1, 0x500)?;
    c.jump(0x7ff54, C::Al, ASSERT, true)?;
    c.transfer(0x7ff58, C::Al, true, 0, 4, 0)?;
    c.jump(0x7ff5c, C::Al, LOCK, true)?;
    c.transfer(0x7ff60, C::Al, true, 0, 5, 4)?;
    c.emit(0x7ff64, data_immediate_with_flags(C::Al, Op::Sub, r(6), r(0), 1, true)?)?;
    c.transfer(0x7ff68, C::Al, false, 6, 5, 4)?;
    c.transfer(0x7ff6c, C::Eq, true, 0, 4, 24)?;
    c.immediate(0x7ff70, C::Eq, Op::Add, 0, 0, 0x1)?;
    c.transfer(0x7ff74, C::Eq, false, 0, 4, 24)?;
    c.transfer(0x7ff78, C::Al, true, 0, 4, 0)?;
    c.jump(0x7ff7c, C::Al, UNLOCK, true)?;
    c.transfer(0x7ff80, C::Al, true, 0, 5, 4)?;
    c.transfer(0x7ff84, C::Al, true, 3, 5, 16)?;
    c.literal(0x7ff88, 1, 0x7ffc4)?;
    c.mov(0x7ff8c, 2, 4)?;
    c.transfer(0x7ff90, C::Al, false, 0, 13, 0)?;
    c.immediate(0x7ff94, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x7ff98, C::Al, LOG, true)?;
    c.literal(0x7ff9c, 1, 0x7ffc0)?;
    c.immediate(0x7ffa0, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x7ffa4, C::Al, LOG, true)?;
    c.mov(0x7ffa8, 0, 6)?;
    c.immediate(0x7ffac, C::Al, Op::Sub, 13, 11, 0x10)?;
    c.pop(0x7ffb0, C::Al, 0x88f0)?;
    c.word(0x7ffb4, 0xf478b, true)?;
    c.word(0x7ffb8, 0xf4ad9, true)?;
    c.word(0x7ffbc, 0xf4b0e, true)?;
    c.word(0x7ffc0, 0xe2788, true)?;
    c.word(0x7ffc4, 0xf4b1b, true)?;
    Ok(())
}
fn retain_record(c: &mut Code) -> Result<(), String> {
    c.push(0x7ffc8, 0x48f0)?;
    c.immediate(0x7ffcc, C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(0x7ffd0, 4, 0)?;
    c.transfer(0x7ffd4, C::Al, true, 6, 4, 36)?;
    c.transfer(0x7ffd8, C::Al, true, 0, 6, 0)?;
    c.jump(0x7ffdc, C::Al, LOCK, true)?;
    c.transfer(0x7ffe0, C::Al, true, 0, 4, 4)?;
    c.immediate(0x7ffe4, C::Al, Op::Add, 5, 0, 0x1)?;
    c.transfer(0x7ffe8, C::Al, false, 5, 4, 4)?;
    c.transfer(0x7ffec, C::Al, true, 0, 6, 0)?;
    c.jump(0x7fff0, C::Al, UNLOCK, true)?;
    c.transfer(0x7fff4, C::Al, true, 3, 4, 4)?;
    c.transfer(0x7fff8, C::Al, true, 2, 4, 16)?;
    c.literal(0x7fffc, 1, 0x8001c)?;
    c.immediate(0x80000, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x80004, C::Al, LOG, true)?;
    c.literal(0x80008, 1, 0x80020)?;
    c.immediate(0x8000c, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x80010, C::Al, LOG, true)?;
    c.mov(0x80014, 0, 5)?;
    c.pop(0x80018, C::Al, 0x88f0)?;
    c.word(0x8001c, 0xf4b5d, true)?;
    c.word(0x80020, 0xe2788, true)?;
    Ok(())
}
fn retain_by_handle(c: &mut Code) -> Result<(), String> {
    c.push(0x80024, 0x4bf0)?;
    c.immediate(0x80028, C::Al, Op::Add, 11, 13, 0x18)?;
    c.immediate(0x8002c, C::Al, Op::Sub, 13, 13, 0x8)?;
    c.mov(0x80030, 4, 0)?;
    c.mov(0x80034, 6, 1)?;
    c.transfer(0x80038, C::Al, true, 0, 4, 16)?;
    c.immediate(0x8003c, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x80040, C::Eq, 0x800a0, false)?;
    c.transfer(0x80044, C::Al, true, 0, 4, 4)?;
    c.literal(0x80048, 9, 0x8011c)?;
    c.immediate(0x8004c, C::Al, Op::Mov, 8, 0, 0xd7)?;
    c.immediate(0x80050, C::Al, Op::Mov, 7, 0, 0x0)?;
    c.immediate(0x80054, C::Al, Op::Mov, 5, 0, 0x0)?;
    c.immediate(0x80058, C::Al, Op::Orr, 8, 8, 0x500)?;
    c.emit(0x8005c, transfer_word_register(C::Al, true, r(0), r(0), r(7), Shift::Lsl, 0)?)?;
    c.mov(0x80060, 1, 8)?;
    c.mov(0x80064, 2, 9)?;
    c.cmp(0x80068, 0, 5)?;
    c.immediate(0x8006c, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x80070, C::Eq, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x80074, C::Al, ASSERT, true)?;
    c.transfer(0x80078, C::Al, true, 0, 4, 4)?;
    c.register(0x8007c, C::Al, Op::Add, 1, 0, 7, 0)?;
    c.transfer(0x80080, C::Al, true, 1, 1, 16)?;
    c.cmp(0x80084, 1, 6)?;
    c.jump(0x80088, C::Eq, 0x800bc, false)?;
    c.transfer(0x8008c, C::Al, true, 1, 4, 16)?;
    c.immediate(0x80090, C::Al, Op::Add, 5, 5, 0x1)?;
    c.immediate(0x80094, C::Al, Op::Add, 7, 7, 0x28)?;
    c.cmp(0x80098, 5, 1)?;
    c.jump(0x8009c, C::Cc, 0x8005c, false)?;
    c.literal(0x800a0, 2, 0x8011c)?;
    c.immediate(0x800a4, C::Al, Op::Mov, 1, 0, 0xee)?;
    c.immediate(0x800a8, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x800ac, C::Al, Op::Mov, 6, 0, 0x0)?;
    c.immediate(0x800b0, C::Al, Op::Orr, 1, 1, 0x500)?;
    c.jump(0x800b4, C::Al, ASSERT, true)?;
    c.jump(0x800b8, C::Al, 0x80110, false)?;
    c.transfer(0x800bc, C::Al, true, 0, 4, 0)?;
    c.jump(0x800c0, C::Al, LOCK, true)?;
    c.transfer(0x800c4, C::Al, true, 0, 4, 4)?;
    c.register(0x800c8, C::Al, Op::Add, 0, 0, 7, 0)?;
    c.transfer(0x800cc, C::Al, true, 1, 0, 4)?;
    c.immediate(0x800d0, C::Al, Op::Add, 6, 1, 0x1)?;
    c.transfer(0x800d4, C::Al, false, 6, 0, 4)?;
    c.transfer(0x800d8, C::Al, true, 0, 4, 0)?;
    c.jump(0x800dc, C::Al, UNLOCK, true)?;
    c.transfer(0x800e0, C::Al, true, 0, 4, 4)?;
    c.mov(0x800e4, 2, 5)?;
    c.register(0x800e8, C::Al, Op::Add, 0, 0, 7, 0)?;
    c.transfer(0x800ec, C::Al, true, 1, 0, 4)?;
    c.transfer(0x800f0, C::Al, true, 3, 0, 16)?;
    c.immediate(0x800f4, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x800f8, C::Al, false, 1, 13, 0)?;
    c.literal(0x800fc, 1, 0x80120)?;
    c.jump(0x80100, C::Al, LOG, true)?;
    c.literal(0x80104, 1, 0x80124)?;
    c.immediate(0x80108, C::Al, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x8010c, C::Al, LOG, true)?;
    c.mov(0x80110, 0, 6)?;
    c.immediate(0x80114, C::Al, Op::Sub, 13, 11, 0x18)?;
    c.pop(0x80118, C::Al, 0x8bf0)?;
    c.word(0x8011c, 0xf478b, true)?;
    c.word(0x80120, 0xf4b7c, true)?;
    c.word(0x80124, 0xe2788, true)?;
    Ok(())
}
fn get_size(c: &mut Code) -> Result<(), String> {
    c.transfer(0x80128, C::Al, true, 0, 0, 20)?;
    c.transfer(0x8012c, C::Al, false, 0, 1, 0)?;
    c.emit(0x80130, bx(C::Al, r(14)))?;
    Ok(())
}
fn get_size_and_stride(c: &mut Code) -> Result<(), String> {
    c.transfer(0x80134, C::Al, true, 3, 0, 20)?;
    c.transfer(0x80138, C::Al, false, 3, 1, 0)?;
    c.transfer(0x8013c, C::Al, true, 0, 0, 28)?;
    c.transfer(0x80140, C::Al, false, 0, 2, 0)?;
    c.emit(0x80144, bx(C::Al, r(14)))?;
    Ok(())
}
fn get_stride(c: &mut Code) -> Result<(), String> {
    c.transfer(0x80148, C::Al, true, 0, 0, 28)?;
    c.emit(0x8014c, bx(C::Al, r(14)))?;
    Ok(())
}
fn query_size(c: &mut Code) -> Result<(), String> {
    c.transfer(0x80150, C::Al, true, 0, 0, 20)?;
    c.transfer(0x80154, C::Al, false, 0, 1, 0)?;
    c.emit(0x80158, bx(C::Al, r(14)))?;
    Ok(())
}
fn query_size_and_stride(c: &mut Code) -> Result<(), String> {
    c.transfer(0x8015c, C::Al, true, 3, 0, 20)?;
    c.transfer(0x80160, C::Al, false, 3, 1, 0)?;
    c.transfer(0x80164, C::Al, true, 0, 0, 28)?;
    c.transfer(0x80168, C::Al, false, 0, 2, 0)?;
    c.emit(0x8016c, bx(C::Al, r(14)))?;
    Ok(())
}
fn get_reference_count(c: &mut Code) -> Result<(), String> {
    c.push(0x80170, 0x4830)?;
    c.immediate(0x80174, C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(0x80178, 4, 0)?;
    c.transfer(0x8017c, C::Al, true, 5, 4, 36)?;
    c.transfer(0x80180, C::Al, true, 0, 5, 0)?;
    c.jump(0x80184, C::Al, LOCK, true)?;
    c.transfer(0x80188, C::Al, true, 0, 5, 0)?;
    c.transfer(0x8018c, C::Al, true, 4, 4, 4)?;
    c.jump(0x80190, C::Al, UNLOCK, true)?;
    c.mov(0x80194, 0, 4)?;
    c.pop(0x80198, C::Al, 0x8830)?;
    Ok(())
}
fn get_capacity(c: &mut Code) -> Result<(), String> {
    c.transfer(0x8019c, C::Al, true, 0, 0, 16)?;
    c.transfer(0x801a0, C::Al, false, 0, 1, 0)?;
    c.emit(0x801a4, bx(C::Al, r(14)))?;
    Ok(())
}
fn get_capacity_and_available(c: &mut Code) -> Result<(), String> {
    c.transfer(0x801a8, C::Al, true, 3, 0, 16)?;
    c.transfer(0x801ac, C::Al, false, 3, 1, 0)?;
    c.transfer(0x801b0, C::Al, true, 0, 0, 24)?;
    c.transfer(0x801b4, C::Al, false, 0, 2, 0)?;
    c.emit(0x801b8, bx(C::Al, r(14)))?;
    Ok(())
}
fn get_indexed_handle(c: &mut Code) -> Result<(), String> {
    c.push(0x801bc, 0x48f0)?;
    c.immediate(0x801c0, C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(0x801c4, 5, 0)?;
    c.register(0x801c8, C::Al, Op::Add, 6, 2, 2, 2)?;
    c.literal(0x801cc, 2, 0x80200)?;
    c.mov(0x801d0, 4, 1)?;
    c.immediate(0x801d4, C::Al, Op::Mov, 1, 0, 0x83)?;
    c.transfer(0x801d8, C::Al, true, 0, 5, 4)?;
    c.immediate(0x801dc, C::Al, Op::Orr, 1, 1, 0x600)?;
    c.register(0x801e0, C::Al, Op::Add, 0, 0, 6, 3)?;
    c.transfer(0x801e4, C::Al, true, 0, 0, 12)?;
    c.jump(0x801e8, C::Al, ASSERT, true)?;
    c.transfer(0x801ec, C::Al, true, 0, 5, 4)?;
    c.register(0x801f0, C::Al, Op::Add, 0, 0, 6, 3)?;
    c.transfer(0x801f4, C::Al, true, 0, 0, 16)?;
    c.transfer(0x801f8, C::Al, false, 0, 4, 0)?;
    c.pop(0x801fc, C::Al, 0x88f0)?;
    c.word(0x80200, 0xf478b, true)?;
    Ok(())
}
fn set_indexed_handle(c: &mut Code) -> Result<(), String> {
    c.push(0x80204, 0x48f0)?;
    c.immediate(0x80208, C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(0x8020c, 5, 0)?;
    c.register(0x80210, C::Al, Op::Add, 6, 2, 2, 2)?;
    c.literal(0x80214, 2, 0x80244)?;
    c.mov(0x80218, 4, 1)?;
    c.immediate(0x8021c, C::Al, Op::Mov, 1, 0, 0x93)?;
    c.transfer(0x80220, C::Al, true, 0, 5, 4)?;
    c.immediate(0x80224, C::Al, Op::Orr, 1, 1, 0x600)?;
    c.register(0x80228, C::Al, Op::Add, 0, 0, 6, 3)?;
    c.transfer(0x8022c, C::Al, true, 0, 0, 12)?;
    c.jump(0x80230, C::Al, ASSERT, true)?;
    c.transfer(0x80234, C::Al, true, 0, 5, 4)?;
    c.register(0x80238, C::Al, Op::Add, 0, 0, 6, 3)?;
    c.transfer(0x8023c, C::Al, false, 4, 0, 16)?;
    c.pop(0x80240, C::Al, 0x88f0)?;
    c.word(0x80244, 0xf478b, true)?;
    Ok(())
}
fn get_size_and_capacity(c: &mut Code) -> Result<(), String> {
    c.transfer(0x80248, C::Al, true, 3, 0, 20)?;
    c.transfer(0x8024c, C::Al, false, 3, 2, 0)?;
    c.transfer(0x80250, C::Al, true, 0, 0, 16)?;
    c.transfer(0x80254, C::Al, false, 0, 1, 0)?;
    c.emit(0x80258, bx(C::Al, r(14)))?;
    Ok(())
}
fn get_state(c: &mut Code) -> Result<(), String> {
    c.transfer(0x8025c, C::Al, true, 0, 0, 32)?;
    c.transfer(0x80260, C::Al, false, 0, 1, 0)?;
    c.emit(0x80264, bx(C::Al, r(14)))?;
    Ok(())
}
fn reset_references(c: &mut Code) -> Result<(), String> {
    c.transfer(0x80268, C::Al, true, 1, 0, 16)?;
    c.immediate(0x8026c, C::Al, Op::Cmp, 0, 1, 0x0)?;
    c.jump(0x80270, C::Eq, 0x80298, false)?;
    c.transfer(0x80274, C::Al, true, 2, 0, 4)?;
    c.immediate(0x80278, C::Al, Op::Mov, 12, 0, 0x0)?;
    c.immediate(0x8027c, C::Al, Op::Mov, 3, 0, 0x0)?;
    c.immediate(0x80280, C::Al, Op::Add, 2, 2, 0x4)?;
    c.immediate(0x80284, C::Al, Op::Add, 3, 3, 0x1)?;
    c.emit(0x80288, transfer_word_postindexed(C::Al, false, r(12), r(2), 40)?)?;
    c.cmp(0x8028c, 3, 1)?;
    c.jump(0x80290, C::Cc, 0x80284, false)?;
    c.jump(0x80294, C::Al, 0x8029c, false)?;
    c.immediate(0x80298, C::Al, Op::Mov, 1, 0, 0x0)?;
    c.transfer(0x8029c, C::Al, false, 1, 0, 24)?;
    c.emit(0x802a0, bx(C::Al, r(14)))?;
    Ok(())
}
fn count_available(c: &mut Code) -> Result<(), String> {
    c.push(0x802a4, 0x4bf0)?;
    c.immediate(0x802a8, C::Al, Op::Add, 11, 13, 0x18)?;
    c.mov(0x802ac, 4, 0)?;
    c.immediate(0x802b0, C::Al, Op::Cmp, 0, 4, 0x0)?;
    c.jump(0x802b4, C::Eq, 0x8035c, false)?;
    c.transfer(0x802b8, C::Al, true, 0, 4, 48)?;
    c.immediate(0x802bc, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x802c0, C::Ne, 0x802d8, false)?;
    c.transfer(0x802c4, C::Al, true, 0, 4, 8)?;
    c.transfer(0x802c8, C::Al, true, 1, 4, 12)?;
    c.immediate(0x802cc, C::Al, Op::Mov, 5, 0, 0x0)?;
    c.cmp(0x802d0, 0, 1)?;
    c.jump(0x802d4, C::Hi, 0x803a8, false)?;
    c.transfer(0x802d8, C::Al, true, 0, 4, 0)?;
    c.jump(0x802dc, C::Al, LOCK, true)?;
    c.transfer(0x802e0, C::Al, true, 0, 4, 16)?;
    c.immediate(0x802e4, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x802e8, C::Eq, 0x8039c, false)?;
    c.transfer(0x802ec, C::Al, true, 0, 4, 4)?;
    c.literal(0x802f0, 9, 0x803b0)?;
    c.immediate(0x802f4, C::Al, Op::Mov, 8, 0, 0x338)?;
    c.immediate(0x802f8, C::Al, Op::Mov, 6, 0, 0x0)?;
    c.immediate(0x802fc, C::Al, Op::Mov, 5, 0, 0x0)?;
    c.immediate(0x80300, C::Al, Op::Mov, 7, 0, 0x0)?;
    c.immediate(0x80304, C::Al, Op::Orr, 8, 8, 0x400)?;
    c.emit(0x80308, transfer_word_register(C::Al, true, r(0), r(0), r(6), Shift::Lsl, 0)?)?;
    c.mov(0x8030c, 1, 8)?;
    c.mov(0x80310, 2, 9)?;
    c.cmp(0x80314, 0, 7)?;
    c.immediate(0x80318, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x8031c, C::Eq, Op::Mov, 0, 0, 0x1)?;
    c.jump(0x80320, C::Al, ASSERT, true)?;
    c.transfer(0x80324, C::Al, true, 0, 4, 4)?;
    c.register(0x80328, C::Al, Op::Add, 1, 0, 6, 0)?;
    c.transfer(0x8032c, C::Al, true, 2, 1, 12)?;
    c.immediate(0x80330, C::Al, Op::Cmp, 0, 2, 0x1)?;
    c.jump(0x80334, C::Ne, 0x80344, false)?;
    c.transfer(0x80338, C::Al, true, 1, 1, 4)?;
    c.immediate(0x8033c, C::Al, Op::Cmp, 0, 1, 0x0)?;
    c.immediate(0x80340, C::Eq, Op::Add, 5, 5, 0x1)?;
    c.transfer(0x80344, C::Al, true, 1, 4, 16)?;
    c.immediate(0x80348, C::Al, Op::Add, 7, 7, 0x1)?;
    c.immediate(0x8034c, C::Al, Op::Add, 6, 6, 0x28)?;
    c.cmp(0x80350, 7, 1)?;
    c.jump(0x80354, C::Cc, 0x80308, false)?;
    c.jump(0x80358, C::Al, 0x803a0, false)?;
    c.literal(0x8035c, 1, 0x803b4)?;
    c.literal(0x80360, 2, 0x803b8)?;
    c.immediate(0x80364, C::Al, Op::Mov, 3, 0, 0x25)?;
    c.immediate(0x80368, C::Al, Op::Mov, 0, 0, 0x10)?;
    c.immediate(0x8036c, C::Al, Op::Orr, 3, 3, 0x700)?;
    c.jump(0x80370, C::Al, LOG, true)?;
    c.literal(0x80374, 1, 0x803bc)?;
    c.immediate(0x80378, C::Al, Op::Mov, 0, 0, 0x10)?;
    c.jump(0x8037c, C::Al, LOG, true)?;
    c.literal(0x80380, 2, 0x803b0)?;
    c.immediate(0x80384, C::Al, Op::Mov, 1, 0, 0x26)?;
    c.immediate(0x80388, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(0x8038c, C::Al, Op::Mov, 5, 0, 0x0)?;
    c.immediate(0x80390, C::Al, Op::Orr, 1, 1, 0x700)?;
    c.jump(0x80394, C::Al, ASSERT, true)?;
    c.jump(0x80398, C::Al, 0x803a8, false)?;
    c.immediate(0x8039c, C::Al, Op::Mov, 5, 0, 0x0)?;
    c.transfer(0x803a0, C::Al, true, 0, 4, 0)?;
    c.jump(0x803a4, C::Al, UNLOCK, true)?;
    c.mov(0x803a8, 0, 5)?;
    c.pop(0x803ac, C::Al, 0x8bf0)?;
    c.word(0x803b0, 0xf478b, true)?;
    c.word(0x803b4, 0xf499d, true)?;
    c.word(0x803b8, 0xf4bac, true)?;
    c.word(0x803bc, 0xe2788, true)?;
    Ok(())
}
fn initialize(c: &mut Code) -> Result<(), String> {
    c.push(0x803c0, 0x4bf0)?;
    c.immediate(0x803c4, C::Al, Op::Add, 11, 13, 0x18)?;
    c.mov(0x803c8, 7, 2)?;
    c.mov(0x803cc, 5, 3)?;
    c.mov(0x803d0, 4, 1)?;
    c.mov(0x803d4, 8, 0)?;
    c.transfer(0x803d8, C::Al, true, 6, 7, 0)?;
    c.immediate(0x803dc, C::Al, Op::Cmp, 0, 6, 0x0)?;
    c.jump(0x803e0, C::Eq, 0x80440, false)?;
    c.transfer(0x803e4, C::Al, true, 0, 6, 12)?;
    c.immediate(0x803e8, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.pop(0x803ec, C::Ne, 0x8bf0)?;
    c.immediate(0x803f0, C::Al, Op::Add, 0, 4, 0x8)?;
    c.immediate(0x803f4, C::Al, Op::Tst, 0, 4, 0x7)?;
    c.immediate(0x803f8, C::Ne, Op::Bic, 4, 0, 0x7)?;
    c.transfer(0x803fc, C::Al, true, 0, 6, 16)?;
    c.immediate(0x80400, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x80404, C::Eq, 0x80434, false)?;
    c.transfer(0x80408, C::Al, true, 1, 6, 4)?;
    c.immediate(0x8040c, C::Al, Op::Mov, 2, 0, 0x0)?;
    c.immediate(0x80410, C::Al, Op::Mov, 3, 0, 0x0)?;
    c.immediate(0x80414, C::Al, Op::Add, 1, 1, 0xc)?;
    c.transfer(0x80418, C::Al, false, 3, 1, -12)?;
    c.immediate(0x8041c, C::Al, Op::Add, 3, 3, 0x1)?;
    c.transfer(0x80420, C::Al, false, 2, 1, 0)?;
    c.transfer(0x80424, C::Al, false, 2, 1, -8)?;
    c.immediate(0x80428, C::Al, Op::Add, 1, 1, 0x28)?;
    c.cmp(0x8042c, 3, 0)?;
    c.jump(0x80430, C::Cc, 0x80418, false)?;
    c.transfer(0x80434, C::Al, false, 4, 6, 28)?;
    c.transfer(0x80438, C::Al, false, 8, 6, 8)?;
    c.pop(0x8043c, C::Al, 0x8bf0)?;
    c.immediate(0x80440, C::Al, Op::Mov, 0, 0, 0x34)?;
    c.jump(0x80444, C::Al, 0x2efc, true)?;
    c.mov(0x80448, 6, 0)?;
    c.immediate(0x8044c, C::Al, Op::Cmp, 0, 6, 0x0)?;
    c.jump(0x80450, C::Eq, 0x80498, false)?;
    c.mov(0x80454, 0, 6)?;
    c.immediate(0x80458, C::Al, Op::Mov, 1, 0, 0x0)?;
    c.immediate(0x8045c, C::Al, Op::Mov, 2, 0, 0x34)?;
    c.jump(0x80460, C::Al, 0x31ec, true)?;
    c.immediate(0x80464, C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.transfer(0x80468, C::Al, false, 8, 6, 16)?;
    c.transfer(0x8046c, C::Al, false, 5, 6, 4)?;
    c.jump(0x80470, C::Eq, 0x80498, false)?;
    c.register(0x80474, C::Al, Op::Add, 0, 8, 8, 2)?;
    c.immediate(0x80478, C::Al, Op::Mov, 1, 0, 0x0)?;
    c.register(0x8047c, C::Al, Op::Mov, 2, 0, 0, 3)?;
    c.mov(0x80480, 0, 5)?;
    c.jump(0x80484, C::Al, 0x31ec, true)?;
    c.mov(0x80488, 0, 6)?;
    c.jump(0x8048c, C::Al, 0x2b8c, true)?;
    c.immediate(0x80490, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.jump(0x80494, C::Eq, 0x804a8, false)?;
    c.transfer(0x80498, C::Al, false, 6, 7, 0)?;
    c.mov(0x8049c, 0, 7)?;
    c.pop(0x804a0, C::Al, 0x4bf0)?;
    c.jump(0x804a4, C::Al, 0x804b0, false)?;
    c.transfer(0x804a8, C::Al, false, 6, 7, 0)?;
    c.jump(0x804ac, C::Al, 0x803e4, false)?;
    Ok(())
}
fn teardown(c: &mut Code) -> Result<(), String> {
    c.push(0x804b0, 0x4830)?;
    c.immediate(0x804b4, C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(0x804b8, 4, 0)?;
    c.transfer(0x804bc, C::Al, true, 5, 4, 0)?;
    c.immediate(0x804c0, C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.pop(0x804c4, C::Eq, 0x8830)?;
    c.transfer(0x804c8, C::Al, true, 0, 5, 4)?;
    c.immediate(0x804cc, C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.immediate(0x804d0, C::Ne, Op::Mov, 0, 0, 0x0)?;
    c.transfer(0x804d4, C::Ne, false, 0, 5, 4)?;
    c.transfer(0x804d8, C::Al, true, 0, 5, 0)?;
    c.jump(0x804dc, C::Al, 0x2bcc, true)?;
    c.mov(0x804e0, 0, 5)?;
    c.jump(0x804e4, C::Al, 0x2f1c, true)?;
    c.immediate(0x804e8, C::Al, Op::Mov, 0, 0, 0x0)?;
    c.transfer(0x804ec, C::Al, false, 0, 4, 0)?;
    c.pop(0x804f0, C::Al, 0x8830)?;
    Ok(())
}
fn set_mode(c: &mut Code) -> Result<(), String> {
    c.transfer(0x804f4, C::Al, false, 1, 0, 48)?;
    c.emit(0x804f8, bx(C::Al, r(14)))?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    release_by_handle(&mut c)?;
    release_by_handle_with_state(&mut c)?;
    release_record(&mut c)?;
    retain_record(&mut c)?;
    retain_by_handle(&mut c)?;
    get_size(&mut c)?;
    get_size_and_stride(&mut c)?;
    get_stride(&mut c)?;
    query_size(&mut c)?;
    query_size_and_stride(&mut c)?;
    get_reference_count(&mut c)?;
    get_capacity(&mut c)?;
    get_capacity_and_available(&mut c)?;
    get_indexed_handle(&mut c)?;
    set_indexed_handle(&mut c)?;
    get_size_and_capacity(&mut c)?;
    get_state(&mut c)?;
    reset_references(&mut c)?;
    count_available(&mut c)?;
    initialize(&mut c)?;
    teardown(&mut c)?;
    set_mode(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != END - START || CODE_BYTES + DATA_BYTES != END - START {
        return Err("invalid VBuffer extent".into());
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
    fn extent_and_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2016);
        assert_eq!(CODE_BYTES + DATA_BYTES, bytes.len());
        for (at, value) in LITERALS {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn exact_code_data_guards() {
        for at in (START..END).step_by(4) {
            let literal = LITERALS.iter().find(|&&(slot, _)| slot == at);
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(at, bx(C::Al, R::LR)).is_err(), literal.is_some());
            for (_, value) in LITERALS {
                let valid = literal.is_some_and(|&(_, expected)| expected == value);
                let mut c = Code { at, bytes: Vec::new() };
                assert_eq!(c.word(at, value, true).is_ok(), valid);
                assert_eq!(c.at, at + if valid { 4 } else { 0 });
                assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
            }
            if let Some(&(_, value)) = literal {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.word(at, value ^ 1, true).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
        }
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.word(at, 0xf478b, true).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.literal(START, 0, START).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn boundaries_flags_and_failure_tails() {
        let bytes = fragment().unwrap();
        assert_eq!(CODE_BYTES, 482 * 4);
        assert_eq!(DATA_BYTES, 22 * 4);
        for (at, condition, target, link) in [
            (0x7fd24, C::Al, 0x7fd28, false),
            (0x7fd80, C::Al, ASSERT, true),
            (0x7fda8, C::Cc, 0x7fd68, false),
            (0x8009c, C::Cc, 0x8005c, false),
            (0x80290, C::Cc, 0x80284, false),
            (0x802b4, C::Eq, 0x8035c, false),
            (0x80354, C::Cc, 0x80308, false),
            (0x80398, C::Al, 0x803a8, false),
            (0x80450, C::Eq, 0x80498, false),
            (0x80470, C::Eq, 0x80498, false),
            (0x80494, C::Eq, 0x804a8, false),
            (0x804a4, C::Al, 0x804b0, false),
            (0x804ac, C::Al, 0x803e4, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x7ff64), data_immediate_with_flags(C::Al, Op::Sub, r(6), r(0), 1, true).unwrap());
        assert_eq!(word(&bytes, 0x803ec), transfer_multiple(C::Ne, true, R::SP, 0x8bf0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x804a0), transfer_multiple(C::Al, true, R::SP, 0x4bf0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x804f8), bx(C::Al, R::LR));
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
