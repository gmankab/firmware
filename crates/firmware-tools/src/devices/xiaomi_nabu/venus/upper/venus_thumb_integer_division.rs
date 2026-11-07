pub const START: usize = 0x837f0;
pub const END: usize = 0x83d2c;
pub const CODE_BYTES: usize = 1306;
pub const DATA_BYTES: usize = 32;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 2;
const POOLS: [(usize, u32); 8] = [
    (0x83c3c, 0x55555555),
    (0x83c40, 0x33333333),
    (0x83c44, 0x0f0f0f0f),
    (0x83c48, 0x01010101),
    (0x83d1c, 0x55555555),
    (0x83d20, 0x33333333),
    (0x83d24, 0x0f0f0f0f),
    (0x83d28, 0x01010101),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Lsl, Lsr, Adc, Sbc, Tst, Orr, Mul, Bic, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr, Asr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Lo = 3, Hi = 8 }
#[derive(Clone, Copy, Debug)]
enum CallMode { Thumb, Arm }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(bool, u16),
    Stack(bool, R, u16), Address(R, u16), Literal(R, usize),
    Alu(Alu, R, R), Shift(Shift, R, R, u8),
    Mov(R, u16), Compare(R, u16), Add(R, u16), Subtract(R, u16),
    Small(bool, R, R, u8), Arithmetic(bool, R, R, R),
    Branch(Option<Condition>, usize), Call(CallMode, usize), Memory(bool, R, R, u16), Align,
}

fn checked(valid: bool) -> Result<(), String> {
    if valid { Ok(()) } else { Err("invalid Thumb operand or layout".into()) }
}

fn displacement(at: usize, target: usize, min: i64, max: i64) -> Result<i64, String> {
    checked(at <= u32::MAX as usize && target <= u32::MAX as usize && at % 2 == 0 && target % 2 == 0)?;
    let delta = target as i64 - at as i64 - 4;
    checked((min..=max).contains(&delta))?;
    Ok(delta)
}

impl I {
    fn encode(self, at: usize) -> Result<Vec<u8>, String> {
        let word = match self {
            Self::Push(mask) | Self::Pop(mask) => {
                let pop = matches!(self, Self::Pop(_));
                let extra = if pop { 0x8000 } else { 0x4000 };
                checked(mask != 0 && mask & !(0xff | extra) == 0)?;
                (if pop { 0xbc00 } else { 0xb400 }) | (mask & 0xff) | if mask & extra != 0 { 0x100 } else { 0 }
            }
            Self::StackAdjust(sub, bytes) => {
                checked(bytes <= 508 && bytes % 4 == 0)?;
                0xb000 | (u16::from(sub) << 7) | bytes / 4
            }
            Self::Stack(load, r, offset) => {
                checked(offset <= 1020 && offset % 4 == 0)?;
                (if load { 0x9800 } else { 0x9000 }) | ((r as u16) << 8) | offset / 4
            }
            Self::Address(r, offset) => {
                checked(offset <= 1020 && offset % 4 == 0)?;
                0xa800 | ((r as u16) << 8) | offset / 4
            }
            Self::Literal(r, target) => {
                let base = (at + 4) & !3;
                checked(at % 2 == 0 && target % 4 == 0 && target >= base && target - base <= 1020)?;
                0x4800 | ((r as u16) << 8) | ((target - base) / 4) as u16
            }
            Self::Alu(op, dst, src) => {
                let op = match op {
                    Alu::And => 0, Alu::Lsl => 2, Alu::Lsr => 3,
                    Alu::Adc => 5, Alu::Sbc => 6, Alu::Tst => 8,
                    Alu::Orr => 12, Alu::Mul => 13, Alu::Bic => 14, Alu::Mvn => 15,
                };
                0x4000 | (op << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Shift(kind, dst, src, amount) => {
                let amount = match kind {
                    Shift::Lsl => { checked(amount < 32)?; amount },
                    Shift::Lsr | Shift::Asr => { checked((1..=32).contains(&amount))?; amount % 32 },
                };
                (match kind { Shift::Lsl => 0, Shift::Lsr => 0x0800, Shift::Asr => 0x1000 })
                    | ((amount as u16) << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Mov(r, value) | Self::Compare(r, value) | Self::Add(r, value) | Self::Subtract(r, value) => {
                checked(value <= 255)?;
                let base = match self { Self::Mov(..) => 0x2000, Self::Compare(..) => 0x2800, Self::Add(..) => 0x3000, _ => 0x3800 };
                base | ((r as u16) << 8) | value
            }
            Self::Small(sub, dst, src, value) => {
                checked(value <= 7)?;
                0x1c00 | (u16::from(sub) << 9) | ((value as u16) << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Arithmetic(sub, dst, lhs, rhs) => {
                0x1800 | (u16::from(sub) << 9) | ((rhs as u16) << 6) | ((lhs as u16) << 3) | dst as u16
            }
            Self::Branch(condition, target) => {
                if let Some(condition) = condition {
                    let delta = displacement(at, target, -256, 254)?;
                    0xd000 | ((condition as u16) << 8) | ((delta / 2) as u16 & 0xff)
                } else {
                    let delta = displacement(at, target, -2048, 2046)?;
                    0xe000 | ((delta / 2) as u16 & 0x7ff)
                }
            }
            Self::Call(mode, target) => {
                let arm = matches!(mode, CallMode::Arm);
                checked(at % 2 == 0 && at <= u32::MAX as usize && (!arm || target % 4 == 0))?;
                let origin = if arm { at & !3 } else { at };
                let delta = displacement(origin, target, -16_777_216, 16_777_214)? as u32;
                let s = (delta >> 24) & 1;
                let j1 = (!(delta >> 23) ^ s) & 1;
                let j2 = (!(delta >> 22) ^ s) & 1;
                let first = 0xf000 | (s << 10) | ((delta >> 12) & 0x3ff);
                let second = (if arm { 0xc000 } else { 0xd000 }) | (j1 << 13) | (j2 << 11) | ((delta >> 1) & 0x7ff);
                return Ok([ (first as u16).to_le_bytes(), (second as u16).to_le_bytes() ].concat());
            }
            Self::Memory(load, r, base, offset) => {
                checked(offset <= 124 && offset % 4 == 0)?;
                (if load { 0x6800 } else { 0x6000 }) | ((offset / 4) << 6) | ((base as u16) << 3) | r as u16
            }
            Self::Align => {
                let dst = 8u16;
                let src = 8u16;
                0x4600 | ((dst & 8) << 4) | (src << 3) | (dst & 7)
            },
        };
        Ok(word.to_le_bytes().to_vec())
    }
}

struct Code { at: usize, bytes: Vec<u8>, code: usize, data: usize, align: usize }
impl Code {
    fn new() -> Self { Self { at: START, bytes: Vec::new(), code: 0, data: 0, align: 0 } }
    fn emit(&mut self, at: usize, instruction: I) -> Result<(), String> {
        let bytes = instruction.encode(at)?;
        checked(at == self.at && at >= START && at % 2 == 0 && at + bytes.len() <= END)?;
        checked(!POOLS.iter().any(|&(slot, _)| at < slot + 4 && at + bytes.len() > slot))?;
        checked(matches!(instruction, I::Align) == (at == 0x83c3a))?;
        checked(![0x83c3a].iter().any(|&slot| at < slot && at + bytes.len() > slot))?;
        if matches!(instruction, I::Align) { self.align += bytes.len(); } else { self.code += bytes.len(); }
        self.at += bytes.len();
        self.bytes.extend(bytes);
        Ok(())
    }
    fn pool(&mut self, at: usize, value: u32) -> Result<(), String> {
        checked(at == self.at && POOLS.contains(&(at, value)))?;
        self.bytes.extend(value.to_le_bytes());
        self.at += 4;
        self.data += 4;
        Ok(())
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new();
    c.emit(0x837f0, I::Push(0x40f0))?;
    c.emit(0x837f2, I::StackAdjust(true, 36))?;
    c.emit(0x837f4, I::Push(0x8))?;
    c.emit(0x837f6, I::Pop(0x20))?;
    c.emit(0x837f8, I::Push(0x2))?;
    c.emit(0x837fa, I::Pop(0x40))?;
    c.emit(0x837fc, I::Push(0x1))?;
    c.emit(0x837fe, I::Pop(0x80))?;
    c.emit(0x83800, I::Mov(R0, 1))?;
    c.emit(0x83802, I::Stack(false, R0, 20))?;
    c.emit(0x83804, I::Alu(Alu::Mvn, R1, R0))?;
    c.emit(0x83806, I::Stack(true, R4, 56))?;
    c.emit(0x83808, I::Compare(R6, 0))?;
    c.emit(0x8380a, I::Stack(false, R2, 24))?;
    c.emit(0x8380c, I::Branch(Some(Condition::Ne), 0x83862))?;
    c.emit(0x8380e, I::Compare(R5, 0))?;
    c.emit(0x83810, I::Branch(Some(Condition::Ne), 0x8383e))?;
    c.emit(0x83812, I::Compare(R4, 0))?;
    c.emit(0x83814, I::Branch(Some(Condition::Eq), 0x8382a))?;
    c.emit(0x83816, I::Mov(R0, 0))?;
    c.emit(0x83818, I::Memory(false, R0, R4, 4))?;
    c.emit(0x8381a, I::Push(0x80))?;
    c.emit(0x8381c, I::Pop(0x1))?;
    c.emit(0x8381e, I::Push(0x4))?;
    c.emit(0x83820, I::Pop(0x2))?;
    c.emit(0x83822, I::Call(CallMode::Arm, 0x81850))?;
    c.emit(0x83826, I::Stack(true, R2, 24))?;
    c.emit(0x83828, I::Memory(false, R1, R4, 0))?;
    c.emit(0x8382a, I::Push(0x80))?;
    c.emit(0x8382c, I::Pop(0x1))?;
    c.emit(0x8382e, I::Push(0x4))?;
    c.emit(0x83830, I::Pop(0x2))?;
    c.emit(0x83832, I::Call(CallMode::Thumb, 0x83c4c))?;
    c.emit(0x83836, I::Push(0x1))?;
    c.emit(0x83838, I::Pop(0x80))?;
    c.emit(0x8383a, I::Mov(R6, 0))?;
    c.emit(0x8383c, I::Branch(None, 0x83c2e))?;
    c.emit(0x8383e, I::Mov(R0, 0))?;
    c.emit(0x83840, I::Compare(R4, 0))?;
    c.emit(0x83842, I::Branch(Some(Condition::Ne), 0x8384e))?;
    c.emit(0x83844, I::Push(0x1))?;
    c.emit(0x83846, I::Pop(0x80))?;
    c.emit(0x83848, I::Push(0x1))?;
    c.emit(0x8384a, I::Pop(0x40))?;
    c.emit(0x8384c, I::Branch(None, 0x83c2e))?;
    c.emit(0x8384e, I::Mov(R0, 0))?;
    c.emit(0x83850, I::Small(false, R1, R1, 1))?;
    c.emit(0x83852, I::Push(0x1))?;
    c.emit(0x83854, I::Pop(0x4))?;
    c.emit(0x83856, I::Alu(Alu::Adc, R2, R2))?;
    c.emit(0x83858, I::Alu(Alu::And, R6, R2))?;
    c.emit(0x8385a, I::Alu(Alu::And, R7, R1))?;
    c.emit(0x8385c, I::Memory(false, R7, R4, 0))?;
    c.emit(0x8385e, I::Memory(false, R6, R4, 4))?;
    c.emit(0x83860, I::Branch(None, 0x83844))?;
    c.emit(0x83862, I::Compare(R2, 0))?;
    c.emit(0x83864, I::Stack(false, R4, 4))?;
    c.emit(0x83866, I::Stack(false, R5, 16))?;
    c.emit(0x83868, I::Stack(false, R1, 0))?;
    c.emit(0x8386a, I::Branch(Some(Condition::Eq), 0x8386e))?;
    c.emit(0x8386c, I::Branch(None, 0x839ba))?;
    c.emit(0x8386e, I::Compare(R5, 0))?;
    c.emit(0x83870, I::Branch(Some(Condition::Ne), 0x83886))?;
    c.emit(0x83872, I::Mov(R7, 0))?;
    c.emit(0x83874, I::Compare(R4, 0))?;
    c.emit(0x83876, I::Branch(Some(Condition::Ne), 0x8387e))?;
    c.emit(0x83878, I::Push(0x80))?;
    c.emit(0x8387a, I::Pop(0x40))?;
    c.emit(0x8387c, I::Branch(None, 0x83c2e))?;
    c.emit(0x8387e, I::Mov(R7, 0))?;
    c.emit(0x83880, I::Memory(false, R7, R4, 0))?;
    c.emit(0x83882, I::Memory(false, R7, R4, 4))?;
    c.emit(0x83884, I::Branch(None, 0x83878))?;
    c.emit(0x83886, I::Compare(R7, 0))?;
    c.emit(0x83888, I::Branch(Some(Condition::Ne), 0x838aa))?;
    c.emit(0x8388a, I::Compare(R4, 0))?;
    c.emit(0x8388c, I::Branch(Some(Condition::Eq), 0x838a0))?;
    c.emit(0x8388e, I::Mov(R0, 0))?;
    c.emit(0x83890, I::Memory(false, R0, R4, 0))?;
    c.emit(0x83892, I::Push(0x40))?;
    c.emit(0x83894, I::Pop(0x1))?;
    c.emit(0x83896, I::Push(0x20))?;
    c.emit(0x83898, I::Pop(0x2))?;
    c.emit(0x8389a, I::Call(CallMode::Arm, 0x81850))?;
    c.emit(0x8389e, I::Memory(false, R1, R4, 4))?;
    c.emit(0x838a0, I::Push(0x40))?;
    c.emit(0x838a2, I::Pop(0x1))?;
    c.emit(0x838a4, I::Push(0x20))?;
    c.emit(0x838a6, I::Pop(0x2))?;
    c.emit(0x838a8, I::Branch(None, 0x83832))?;
    c.emit(0x838aa, I::Small(true, R0, R5, 1))?;
    c.emit(0x838ac, I::Alu(Alu::Tst, R0, R5))?;
    c.emit(0x838ae, I::Branch(Some(Condition::Ne), 0x83904))?;
    c.emit(0x838b0, I::Compare(R4, 0))?;
    c.emit(0x838b2, I::Branch(Some(Condition::Eq), 0x838ce))?;
    c.emit(0x838b4, I::Push(0x2))?;
    c.emit(0x838b6, I::Pop(0x4))?;
    c.emit(0x838b8, I::Mov(R1, 0))?;
    c.emit(0x838ba, I::Small(false, R2, R2, 1))?;
    c.emit(0x838bc, I::Alu(Alu::Adc, R1, R1))?;
    c.emit(0x838be, I::Alu(Alu::And, R7, R2))?;
    c.emit(0x838c0, I::Push(0x1))?;
    c.emit(0x838c2, I::Pop(0x4))?;
    c.emit(0x838c4, I::Alu(Alu::And, R2, R6))?;
    c.emit(0x838c6, I::Alu(Alu::And, R1, R6))?;
    c.emit(0x838c8, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x838ca, I::Memory(false, R7, R4, 0))?;
    c.emit(0x838cc, I::Memory(false, R1, R4, 4))?;
    c.emit(0x838ce, I::Mov(R1, 32))?;
    c.emit(0x838d0, I::Compare(R5, 0))?;
    c.emit(0x838d2, I::Branch(Some(Condition::Eq), 0x838f6))?;
    c.emit(0x838d4, I::Alu(Alu::Bic, R0, R5))?;
    c.emit(0x838d6, I::Shift(Shift::Lsr, R1, R0, 1))?;
    c.emit(0x838d8, I::Literal(R2, 0x83c3c))?;
    c.emit(0x838da, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x838dc, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x838de, I::Literal(R1, 0x83c40))?;
    c.emit(0x838e0, I::Shift(Shift::Lsr, R2, R0, 2))?;
    c.emit(0x838e2, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x838e4, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x838e6, I::Arithmetic(false, R0, R0, R2))?;
    c.emit(0x838e8, I::Shift(Shift::Lsr, R1, R0, 4))?;
    c.emit(0x838ea, I::Arithmetic(false, R0, R0, R1))?;
    c.emit(0x838ec, I::Literal(R1, 0x83c44))?;
    c.emit(0x838ee, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x838f0, I::Literal(R0, 0x83c48))?;
    c.emit(0x838f2, I::Alu(Alu::Mul, R0, R1))?;
    c.emit(0x838f4, I::Shift(Shift::Lsr, R1, R0, 24))?;
    c.emit(0x838f6, I::Alu(Alu::Lsr, R6, R1))?;
    c.emit(0x838f8, I::Mov(R0, 0))?;
    c.emit(0x838fa, I::Push(0x40))?;
    c.emit(0x838fc, I::Pop(0x80))?;
    c.emit(0x838fe, I::Push(0x1))?;
    c.emit(0x83900, I::Pop(0x40))?;
    c.emit(0x83902, I::Branch(None, 0x83c2e))?;
    c.emit(0x83904, I::Mov(R0, 32))?;
    c.emit(0x83906, I::Compare(R5, 0))?;
    c.emit(0x83908, I::Push(0x1))?;
    c.emit(0x8390a, I::Pop(0x2))?;
    c.emit(0x8390c, I::Branch(Some(Condition::Eq), 0x83946))?;
    c.emit(0x8390e, I::Shift(Shift::Lsr, R1, R5, 1))?;
    c.emit(0x83910, I::Alu(Alu::Orr, R1, R5))?;
    c.emit(0x83912, I::Shift(Shift::Lsr, R2, R1, 2))?;
    c.emit(0x83914, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x83916, I::Shift(Shift::Lsr, R1, R2, 4))?;
    c.emit(0x83918, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x8391a, I::Shift(Shift::Lsr, R2, R1, 8))?;
    c.emit(0x8391c, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x8391e, I::Shift(Shift::Lsr, R1, R2, 16))?;
    c.emit(0x83920, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83922, I::Alu(Alu::Mvn, R1, R1))?;
    c.emit(0x83924, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x83926, I::Literal(R3, 0x83c3c))?;
    c.emit(0x83928, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x8392a, I::Arithmetic(true, R1, R1, R3))?;
    c.emit(0x8392c, I::Literal(R2, 0x83c40))?;
    c.emit(0x8392e, I::Shift(Shift::Lsr, R3, R1, 2))?;
    c.emit(0x83930, I::Alu(Alu::And, R1, R2))?;
    c.emit(0x83932, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83934, I::Arithmetic(false, R1, R1, R3))?;
    c.emit(0x83936, I::Shift(Shift::Lsr, R2, R1, 4))?;
    c.emit(0x83938, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x8393a, I::Literal(R2, 0x83c44))?;
    c.emit(0x8393c, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x8393e, I::Literal(R1, 0x83c48))?;
    c.emit(0x83940, I::Alu(Alu::Mul, R1, R2))?;
    c.emit(0x83942, I::Stack(true, R2, 24))?;
    c.emit(0x83944, I::Shift(Shift::Lsr, R1, R1, 24))?;
    c.emit(0x83946, I::Compare(R6, 0))?;
    c.emit(0x83948, I::Branch(Some(Condition::Eq), 0x83982))?;
    c.emit(0x8394a, I::Shift(Shift::Lsr, R0, R6, 1))?;
    c.emit(0x8394c, I::Alu(Alu::Orr, R0, R6))?;
    c.emit(0x8394e, I::Shift(Shift::Lsr, R2, R0, 2))?;
    c.emit(0x83950, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x83952, I::Shift(Shift::Lsr, R0, R2, 4))?;
    c.emit(0x83954, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x83956, I::Shift(Shift::Lsr, R2, R0, 8))?;
    c.emit(0x83958, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x8395a, I::Shift(Shift::Lsr, R0, R2, 16))?;
    c.emit(0x8395c, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x8395e, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x83960, I::Shift(Shift::Lsr, R2, R0, 1))?;
    c.emit(0x83962, I::Literal(R3, 0x83c3c))?;
    c.emit(0x83964, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83966, I::Arithmetic(true, R0, R0, R3))?;
    c.emit(0x83968, I::Literal(R2, 0x83c40))?;
    c.emit(0x8396a, I::Shift(Shift::Lsr, R3, R0, 2))?;
    c.emit(0x8396c, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x8396e, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83970, I::Arithmetic(false, R0, R0, R3))?;
    c.emit(0x83972, I::Shift(Shift::Lsr, R2, R0, 4))?;
    c.emit(0x83974, I::Arithmetic(false, R0, R0, R2))?;
    c.emit(0x83976, I::Literal(R2, 0x83c44))?;
    c.emit(0x83978, I::Alu(Alu::And, R2, R0))?;
    c.emit(0x8397a, I::Literal(R0, 0x83c48))?;
    c.emit(0x8397c, I::Alu(Alu::Mul, R0, R2))?;
    c.emit(0x8397e, I::Stack(true, R2, 24))?;
    c.emit(0x83980, I::Shift(Shift::Lsr, R0, R0, 24))?;
    c.emit(0x83982, I::Arithmetic(true, R0, R1, R0))?;
    c.emit(0x83984, I::Compare(R0, 31))?;
    c.emit(0x83986, I::Branch(Some(Condition::Lo), 0x83998))?;
    c.emit(0x83988, I::Mov(R0, 0))?;
    c.emit(0x8398a, I::Compare(R4, 0))?;
    c.emit(0x8398c, I::Branch(Some(Condition::Ne), 0x83990))?;
    c.emit(0x8398e, I::Branch(None, 0x83844))?;
    c.emit(0x83990, I::Memory(false, R7, R4, 0))?;
    c.emit(0x83992, I::Memory(false, R6, R4, 4))?;
    c.emit(0x83994, I::Mov(R7, 0))?;
    c.emit(0x83996, I::Branch(None, 0x83878))?;
    c.emit(0x83998, I::Small(false, R4, R0, 1))?;
    c.emit(0x8399a, I::Push(0x80))?;
    c.emit(0x8399c, I::Pop(0x2))?;
    c.emit(0x8399e, I::Alu(Alu::Lsr, R1, R4))?;
    c.emit(0x839a0, I::Mov(R3, 31))?;
    c.emit(0x839a2, I::Arithmetic(true, R0, R3, R0))?;
    c.emit(0x839a4, I::Push(0x40))?;
    c.emit(0x839a6, I::Pop(0x20))?;
    c.emit(0x839a8, I::Alu(Alu::Lsl, R5, R0))?;
    c.emit(0x839aa, I::Alu(Alu::Orr, R5, R1))?;
    c.emit(0x839ac, I::Alu(Alu::Lsr, R6, R4))?;
    c.emit(0x839ae, I::Alu(Alu::Lsl, R7, R0))?;
    c.emit(0x839b0, I::Push(0x10))?;
    c.emit(0x839b2, I::Pop(0x1))?;
    c.emit(0x839b4, I::Mov(R1, 0))?;
    c.emit(0x839b6, I::Stack(false, R1, 32))?;
    c.emit(0x839b8, I::Branch(None, 0x83bae))?;
    c.emit(0x839ba, I::Compare(R5, 0))?;
    c.emit(0x839bc, I::Branch(Some(Condition::Eq), 0x839c0))?;
    c.emit(0x839be, I::Branch(None, 0x83afc))?;
    c.emit(0x839c0, I::Small(true, R1, R2, 1))?;
    c.emit(0x839c2, I::Alu(Alu::Tst, R1, R2))?;
    c.emit(0x839c4, I::Branch(Some(Condition::Ne), 0x83a1e))?;
    c.emit(0x839c6, I::Compare(R4, 0))?;
    c.emit(0x839c8, I::Branch(Some(Condition::Eq), 0x839d8))?;
    c.emit(0x839ca, I::Mov(R0, 0))?;
    c.emit(0x839cc, I::Push(0x2))?;
    c.emit(0x839ce, I::Pop(0x4))?;
    c.emit(0x839d0, I::Alu(Alu::And, R2, R7))?;
    c.emit(0x839d2, I::Memory(false, R2, R4, 0))?;
    c.emit(0x839d4, I::Stack(true, R2, 24))?;
    c.emit(0x839d6, I::Memory(false, R0, R4, 4))?;
    c.emit(0x839d8, I::Compare(R2, 1))?;
    c.emit(0x839da, I::Branch(Some(Condition::Ne), 0x839de))?;
    c.emit(0x839dc, I::Branch(None, 0x83c2e))?;
    c.emit(0x839de, I::Mov(R0, 32))?;
    c.emit(0x839e0, I::Compare(R2, 0))?;
    c.emit(0x839e2, I::Push(0x4))?;
    c.emit(0x839e4, I::Pop(0x8))?;
    c.emit(0x839e6, I::Push(0x1))?;
    c.emit(0x839e8, I::Pop(0x4))?;
    c.emit(0x839ea, I::Branch(Some(Condition::Eq), 0x83a0e))?;
    c.emit(0x839ec, I::Alu(Alu::Bic, R1, R3))?;
    c.emit(0x839ee, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x839f0, I::Literal(R3, 0x83c3c))?;
    c.emit(0x839f2, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x839f4, I::Arithmetic(true, R1, R1, R3))?;
    c.emit(0x839f6, I::Literal(R2, 0x83c40))?;
    c.emit(0x839f8, I::Shift(Shift::Lsr, R3, R1, 2))?;
    c.emit(0x839fa, I::Alu(Alu::And, R1, R2))?;
    c.emit(0x839fc, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x839fe, I::Arithmetic(false, R1, R1, R3))?;
    c.emit(0x83a00, I::Shift(Shift::Lsr, R2, R1, 4))?;
    c.emit(0x83a02, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x83a04, I::Literal(R2, 0x83c44))?;
    c.emit(0x83a06, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x83a08, I::Literal(R1, 0x83c48))?;
    c.emit(0x83a0a, I::Alu(Alu::Mul, R1, R2))?;
    c.emit(0x83a0c, I::Shift(Shift::Lsr, R2, R1, 24))?;
    c.emit(0x83a0e, I::Alu(Alu::Lsr, R7, R2))?;
    c.emit(0x83a10, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x83a12, I::Push(0x40))?;
    c.emit(0x83a14, I::Pop(0x2))?;
    c.emit(0x83a16, I::Alu(Alu::Lsl, R1, R0))?;
    c.emit(0x83a18, I::Alu(Alu::Orr, R7, R1))?;
    c.emit(0x83a1a, I::Alu(Alu::Lsr, R6, R2))?;
    c.emit(0x83a1c, I::Branch(None, 0x83c2e))?;
    c.emit(0x83a1e, I::Mov(R4, 32))?;
    c.emit(0x83a20, I::Compare(R2, 0))?;
    c.emit(0x83a22, I::Push(0x10))?;
    c.emit(0x83a24, I::Pop(0x1))?;
    c.emit(0x83a26, I::Branch(Some(Condition::Eq), 0x83a60))?;
    c.emit(0x83a28, I::Shift(Shift::Lsr, R0, R2, 1))?;
    c.emit(0x83a2a, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x83a2c, I::Shift(Shift::Lsr, R1, R0, 2))?;
    c.emit(0x83a2e, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x83a30, I::Shift(Shift::Lsr, R0, R1, 4))?;
    c.emit(0x83a32, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x83a34, I::Shift(Shift::Lsr, R1, R0, 8))?;
    c.emit(0x83a36, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x83a38, I::Shift(Shift::Lsr, R0, R1, 16))?;
    c.emit(0x83a3a, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x83a3c, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x83a3e, I::Shift(Shift::Lsr, R1, R0, 1))?;
    c.emit(0x83a40, I::Literal(R2, 0x83c3c))?;
    c.emit(0x83a42, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x83a44, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x83a46, I::Literal(R1, 0x83c40))?;
    c.emit(0x83a48, I::Shift(Shift::Lsr, R2, R0, 2))?;
    c.emit(0x83a4a, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x83a4c, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x83a4e, I::Arithmetic(false, R0, R0, R2))?;
    c.emit(0x83a50, I::Stack(true, R2, 24))?;
    c.emit(0x83a52, I::Shift(Shift::Lsr, R1, R0, 4))?;
    c.emit(0x83a54, I::Arithmetic(false, R0, R0, R1))?;
    c.emit(0x83a56, I::Literal(R1, 0x83c44))?;
    c.emit(0x83a58, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x83a5a, I::Literal(R0, 0x83c48))?;
    c.emit(0x83a5c, I::Alu(Alu::Mul, R0, R1))?;
    c.emit(0x83a5e, I::Shift(Shift::Lsr, R0, R0, 24))?;
    c.emit(0x83a60, I::Add(R0, 33))?;
    c.emit(0x83a62, I::Compare(R6, 0))?;
    c.emit(0x83a64, I::Push(0x10))?;
    c.emit(0x83a66, I::Pop(0x2))?;
    c.emit(0x83a68, I::Branch(Some(Condition::Eq), 0x83aa2))?;
    c.emit(0x83a6a, I::Shift(Shift::Lsr, R1, R6, 1))?;
    c.emit(0x83a6c, I::Alu(Alu::Orr, R1, R6))?;
    c.emit(0x83a6e, I::Shift(Shift::Lsr, R2, R1, 2))?;
    c.emit(0x83a70, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x83a72, I::Shift(Shift::Lsr, R1, R2, 4))?;
    c.emit(0x83a74, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83a76, I::Shift(Shift::Lsr, R2, R1, 8))?;
    c.emit(0x83a78, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x83a7a, I::Shift(Shift::Lsr, R1, R2, 16))?;
    c.emit(0x83a7c, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83a7e, I::Alu(Alu::Mvn, R1, R1))?;
    c.emit(0x83a80, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x83a82, I::Literal(R3, 0x83c3c))?;
    c.emit(0x83a84, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83a86, I::Arithmetic(true, R1, R1, R3))?;
    c.emit(0x83a88, I::Literal(R2, 0x83c40))?;
    c.emit(0x83a8a, I::Shift(Shift::Lsr, R3, R1, 2))?;
    c.emit(0x83a8c, I::Alu(Alu::And, R1, R2))?;
    c.emit(0x83a8e, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83a90, I::Arithmetic(false, R1, R1, R3))?;
    c.emit(0x83a92, I::Shift(Shift::Lsr, R2, R1, 4))?;
    c.emit(0x83a94, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x83a96, I::Literal(R2, 0x83c44))?;
    c.emit(0x83a98, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x83a9a, I::Literal(R1, 0x83c48))?;
    c.emit(0x83a9c, I::Alu(Alu::Mul, R1, R2))?;
    c.emit(0x83a9e, I::Stack(true, R2, 24))?;
    c.emit(0x83aa0, I::Shift(Shift::Lsr, R1, R1, 24))?;
    c.emit(0x83aa2, I::Arithmetic(true, R0, R0, R1))?;
    c.emit(0x83aa4, I::Mov(R3, 0))?;
    c.emit(0x83aa6, I::Compare(R0, 32))?;
    c.emit(0x83aa8, I::Branch(Some(Condition::Ne), 0x83aae))?;
    c.emit(0x83aaa, I::Stack(false, R3, 32))?;
    c.emit(0x83aac, I::Branch(None, 0x83b8e))?;
    c.emit(0x83aae, I::Compare(R0, 31))?;
    c.emit(0x83ab0, I::Branch(Some(Condition::Hi), 0x83acc))?;
    c.emit(0x83ab2, I::Push(0x80))?;
    c.emit(0x83ab4, I::Pop(0x10))?;
    c.emit(0x83ab6, I::Alu(Alu::Lsr, R4, R0))?;
    c.emit(0x83ab8, I::Mov(R1, 32))?;
    c.emit(0x83aba, I::Arithmetic(true, R1, R1, R0))?;
    c.emit(0x83abc, I::Push(0x40))?;
    c.emit(0x83abe, I::Pop(0x20))?;
    c.emit(0x83ac0, I::Alu(Alu::Lsl, R5, R1))?;
    c.emit(0x83ac2, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x83ac4, I::Alu(Alu::Lsr, R6, R0))?;
    c.emit(0x83ac6, I::Alu(Alu::Lsl, R7, R1))?;
    c.emit(0x83ac8, I::Stack(false, R3, 32))?;
    c.emit(0x83aca, I::Branch(None, 0x83bae))?;
    c.emit(0x83acc, I::Push(0x1))?;
    c.emit(0x83ace, I::Pop(0x2))?;
    c.emit(0x83ad0, I::Subtract(R1, 32))?;
    c.emit(0x83ad2, I::Push(0x80))?;
    c.emit(0x83ad4, I::Pop(0x10))?;
    c.emit(0x83ad6, I::Alu(Alu::Lsr, R4, R1))?;
    c.emit(0x83ad8, I::Stack(false, R4, 32))?;
    c.emit(0x83ada, I::Mov(R4, 64))?;
    c.emit(0x83adc, I::Arithmetic(true, R4, R4, R0))?;
    c.emit(0x83ade, I::Stack(false, R4, 28))?;
    c.emit(0x83ae0, I::Push(0x40))?;
    c.emit(0x83ae2, I::Pop(0x20))?;
    c.emit(0x83ae4, I::Alu(Alu::Lsl, R5, R4))?;
    c.emit(0x83ae6, I::Stack(true, R4, 32))?;
    c.emit(0x83ae8, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x83aea, I::Alu(Alu::Lsr, R6, R1))?;
    c.emit(0x83aec, I::Stack(true, R1, 28))?;
    c.emit(0x83aee, I::Alu(Alu::Lsl, R7, R1))?;
    c.emit(0x83af0, I::Stack(false, R7, 32))?;
    c.emit(0x83af2, I::Push(0x20))?;
    c.emit(0x83af4, I::Pop(0x80))?;
    c.emit(0x83af6, I::Push(0x40))?;
    c.emit(0x83af8, I::Pop(0x20))?;
    c.emit(0x83afa, I::Branch(None, 0x83bb2))?;
    c.emit(0x83afc, I::Mov(R0, 32))?;
    c.emit(0x83afe, I::Compare(R5, 0))?;
    c.emit(0x83b00, I::Push(0x1))?;
    c.emit(0x83b02, I::Pop(0x2))?;
    c.emit(0x83b04, I::Branch(Some(Condition::Eq), 0x83b3e))?;
    c.emit(0x83b06, I::Shift(Shift::Lsr, R1, R5, 1))?;
    c.emit(0x83b08, I::Alu(Alu::Orr, R1, R5))?;
    c.emit(0x83b0a, I::Shift(Shift::Lsr, R2, R1, 2))?;
    c.emit(0x83b0c, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x83b0e, I::Shift(Shift::Lsr, R1, R2, 4))?;
    c.emit(0x83b10, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83b12, I::Shift(Shift::Lsr, R2, R1, 8))?;
    c.emit(0x83b14, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x83b16, I::Shift(Shift::Lsr, R1, R2, 16))?;
    c.emit(0x83b18, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83b1a, I::Alu(Alu::Mvn, R1, R1))?;
    c.emit(0x83b1c, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x83b1e, I::Literal(R3, 0x83c3c))?;
    c.emit(0x83b20, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83b22, I::Arithmetic(true, R1, R1, R3))?;
    c.emit(0x83b24, I::Literal(R2, 0x83c40))?;
    c.emit(0x83b26, I::Shift(Shift::Lsr, R3, R1, 2))?;
    c.emit(0x83b28, I::Alu(Alu::And, R1, R2))?;
    c.emit(0x83b2a, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83b2c, I::Arithmetic(false, R1, R1, R3))?;
    c.emit(0x83b2e, I::Shift(Shift::Lsr, R2, R1, 4))?;
    c.emit(0x83b30, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x83b32, I::Literal(R2, 0x83c44))?;
    c.emit(0x83b34, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x83b36, I::Literal(R1, 0x83c48))?;
    c.emit(0x83b38, I::Alu(Alu::Mul, R1, R2))?;
    c.emit(0x83b3a, I::Stack(true, R2, 24))?;
    c.emit(0x83b3c, I::Shift(Shift::Lsr, R1, R1, 24))?;
    c.emit(0x83b3e, I::Compare(R6, 0))?;
    c.emit(0x83b40, I::Branch(Some(Condition::Eq), 0x83b7a))?;
    c.emit(0x83b42, I::Shift(Shift::Lsr, R0, R6, 1))?;
    c.emit(0x83b44, I::Alu(Alu::Orr, R0, R6))?;
    c.emit(0x83b46, I::Shift(Shift::Lsr, R2, R0, 2))?;
    c.emit(0x83b48, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x83b4a, I::Shift(Shift::Lsr, R0, R2, 4))?;
    c.emit(0x83b4c, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x83b4e, I::Shift(Shift::Lsr, R2, R0, 8))?;
    c.emit(0x83b50, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x83b52, I::Shift(Shift::Lsr, R0, R2, 16))?;
    c.emit(0x83b54, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x83b56, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x83b58, I::Shift(Shift::Lsr, R2, R0, 1))?;
    c.emit(0x83b5a, I::Literal(R3, 0x83c3c))?;
    c.emit(0x83b5c, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83b5e, I::Arithmetic(true, R0, R0, R3))?;
    c.emit(0x83b60, I::Literal(R2, 0x83c40))?;
    c.emit(0x83b62, I::Shift(Shift::Lsr, R3, R0, 2))?;
    c.emit(0x83b64, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x83b66, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x83b68, I::Arithmetic(false, R0, R0, R3))?;
    c.emit(0x83b6a, I::Shift(Shift::Lsr, R2, R0, 4))?;
    c.emit(0x83b6c, I::Arithmetic(false, R0, R0, R2))?;
    c.emit(0x83b6e, I::Literal(R2, 0x83c44))?;
    c.emit(0x83b70, I::Alu(Alu::And, R2, R0))?;
    c.emit(0x83b72, I::Literal(R0, 0x83c48))?;
    c.emit(0x83b74, I::Alu(Alu::Mul, R0, R2))?;
    c.emit(0x83b76, I::Stack(true, R2, 24))?;
    c.emit(0x83b78, I::Shift(Shift::Lsr, R0, R0, 24))?;
    c.emit(0x83b7a, I::Arithmetic(true, R1, R1, R0))?;
    c.emit(0x83b7c, I::Compare(R1, 32))?;
    c.emit(0x83b7e, I::Branch(Some(Condition::Lo), 0x83b82))?;
    c.emit(0x83b80, I::Branch(None, 0x83988))?;
    c.emit(0x83b82, I::Small(false, R0, R1, 1))?;
    c.emit(0x83b84, I::Mov(R4, 32))?;
    c.emit(0x83b86, I::Mov(R3, 0))?;
    c.emit(0x83b88, I::Compare(R0, 32))?;
    c.emit(0x83b8a, I::Stack(false, R3, 32))?;
    c.emit(0x83b8c, I::Branch(Some(Condition::Ne), 0x83b98))?;
    c.emit(0x83b8e, I::Push(0x40))?;
    c.emit(0x83b90, I::Pop(0x20))?;
    c.emit(0x83b92, I::Push(0x10))?;
    c.emit(0x83b94, I::Pop(0x1))?;
    c.emit(0x83b96, I::Branch(None, 0x83bb2))?;
    c.emit(0x83b98, I::Push(0x80))?;
    c.emit(0x83b9a, I::Pop(0x8))?;
    c.emit(0x83b9c, I::Alu(Alu::Lsr, R3, R0))?;
    c.emit(0x83b9e, I::Mov(R4, 31))?;
    c.emit(0x83ba0, I::Arithmetic(true, R1, R4, R1))?;
    c.emit(0x83ba2, I::Push(0x40))?;
    c.emit(0x83ba4, I::Pop(0x20))?;
    c.emit(0x83ba6, I::Alu(Alu::Lsl, R5, R1))?;
    c.emit(0x83ba8, I::Alu(Alu::Orr, R5, R3))?;
    c.emit(0x83baa, I::Alu(Alu::Lsr, R6, R0))?;
    c.emit(0x83bac, I::Alu(Alu::Lsl, R7, R1))?;
    c.emit(0x83bae, I::Push(0x40))?;
    c.emit(0x83bb0, I::Pop(0x8))?;
    c.emit(0x83bb2, I::Mov(R4, 0))?;
    c.emit(0x83bb4, I::Alu(Alu::Mvn, R1, R4))?;
    c.emit(0x83bb6, I::Small(true, R2, R2, 1))?;
    c.emit(0x83bb8, I::Stack(false, R2, 8))?;
    c.emit(0x83bba, I::Stack(true, R2, 16))?;
    c.emit(0x83bbc, I::Alu(Alu::Adc, R1, R2))?;
    c.emit(0x83bbe, I::Stack(false, R1, 12))?;
    c.emit(0x83bc0, I::Compare(R0, 0))?;
    c.emit(0x83bc2, I::Branch(Some(Condition::Eq), 0x83c14))?;
    c.emit(0x83bc4, I::Stack(false, R7, 28))?;
    c.emit(0x83bc6, I::Shift(Shift::Lsr, R7, R7, 31))?;
    c.emit(0x83bc8, I::Shift(Shift::Lsl, R6, R5, 1))?;
    c.emit(0x83bca, I::Alu(Alu::Orr, R6, R7))?;
    c.emit(0x83bcc, I::Shift(Shift::Lsl, R7, R3, 1))?;
    c.emit(0x83bce, I::Shift(Shift::Lsr, R3, R5, 31))?;
    c.emit(0x83bd0, I::Alu(Alu::Orr, R3, R7))?;
    c.emit(0x83bd2, I::Stack(true, R1, 8))?;
    c.emit(0x83bd4, I::Arithmetic(true, R5, R1, R6))?;
    c.emit(0x83bd6, I::Stack(true, R5, 12))?;
    c.emit(0x83bd8, I::Alu(Alu::Sbc, R5, R3))?;
    c.emit(0x83bda, I::Push(0x10))?;
    c.emit(0x83bdc, I::Pop(0x4))?;
    c.emit(0x83bde, I::Shift(Shift::Asr, R4, R5, 31))?;
    c.emit(0x83be0, I::Push(0x10))?;
    c.emit(0x83be2, I::Pop(0x20))?;
    c.emit(0x83be4, I::Push(0x1))?;
    c.emit(0x83be6, I::Pop(0x2))?;
    c.emit(0x83be8, I::Stack(true, R0, 24))?;
    c.emit(0x83bea, I::Alu(Alu::And, R5, R0))?;
    c.emit(0x83bec, I::Push(0x10))?;
    c.emit(0x83bee, I::Pop(0x80))?;
    c.emit(0x83bf0, I::Stack(true, R0, 16))?;
    c.emit(0x83bf2, I::Alu(Alu::And, R7, R0))?;
    c.emit(0x83bf4, I::Arithmetic(true, R5, R6, R5))?;
    c.emit(0x83bf6, I::Alu(Alu::Sbc, R3, R7))?;
    c.emit(0x83bf8, I::Stack(true, R0, 20))?;
    c.emit(0x83bfa, I::Alu(Alu::And, R4, R0))?;
    c.emit(0x83bfc, I::Push(0x2))?;
    c.emit(0x83bfe, I::Pop(0x1))?;
    c.emit(0x83c00, I::Stack(true, R1, 32))?;
    c.emit(0x83c02, I::Shift(Shift::Lsl, R6, R1, 1))?;
    c.emit(0x83c04, I::Alu(Alu::Orr, R2, R6))?;
    c.emit(0x83c06, I::Stack(true, R6, 28))?;
    c.emit(0x83c08, I::Shift(Shift::Lsl, R6, R6, 1))?;
    c.emit(0x83c0a, I::Shift(Shift::Lsr, R7, R1, 31))?;
    c.emit(0x83c0c, I::Alu(Alu::Orr, R7, R6))?;
    c.emit(0x83c0e, I::Small(true, R0, R0, 1))?;
    c.emit(0x83c10, I::Stack(false, R2, 32))?;
    c.emit(0x83c12, I::Branch(None, 0x83bc0))?;
    c.emit(0x83c14, I::Shift(Shift::Lsl, R2, R7, 1))?;
    c.emit(0x83c16, I::Stack(true, R0, 32))?;
    c.emit(0x83c18, I::Shift(Shift::Lsr, R6, R0, 31))?;
    c.emit(0x83c1a, I::Alu(Alu::Orr, R6, R2))?;
    c.emit(0x83c1c, I::Shift(Shift::Lsl, R7, R0, 1))?;
    c.emit(0x83c1e, I::Stack(true, R0, 4))?;
    c.emit(0x83c20, I::Compare(R0, 0))?;
    c.emit(0x83c22, I::Branch(Some(Condition::Eq), 0x83c28))?;
    c.emit(0x83c24, I::Memory(false, R5, R0, 0))?;
    c.emit(0x83c26, I::Memory(false, R3, R0, 4))?;
    c.emit(0x83c28, I::Stack(true, R0, 0))?;
    c.emit(0x83c2a, I::Alu(Alu::And, R7, R0))?;
    c.emit(0x83c2c, I::Alu(Alu::Orr, R7, R4))?;
    c.emit(0x83c2e, I::Push(0x80))?;
    c.emit(0x83c30, I::Pop(0x1))?;
    c.emit(0x83c32, I::Push(0x40))?;
    c.emit(0x83c34, I::Pop(0x2))?;
    c.emit(0x83c36, I::StackAdjust(false, 36))?;
    c.emit(0x83c38, I::Pop(0x80f0))?;
    c.emit(0x83c3a, I::Align)?;
    c.pool(0x83c3c, 0x55555555)?;
    c.pool(0x83c40, 0x33333333)?;
    c.pool(0x83c44, 0x0f0f0f0f)?;
    c.pool(0x83c48, 0x01010101)?;
    c.emit(0x83c4c, I::Push(0x4070))?;
    c.emit(0x83c4e, I::Mov(R2, 0))?;
    c.emit(0x83c50, I::Compare(R1, 0))?;
    c.emit(0x83c52, I::Branch(Some(Condition::Eq), 0x83cde))?;
    c.emit(0x83c54, I::Compare(R0, 0))?;
    c.emit(0x83c56, I::Branch(Some(Condition::Eq), 0x83cde))?;
    c.emit(0x83c58, I::Mov(R3, 32))?;
    c.emit(0x83c5a, I::Compare(R1, 0))?;
    c.emit(0x83c5c, I::Push(0x8))?;
    c.emit(0x83c5e, I::Pop(0x10))?;
    c.emit(0x83c60, I::Branch(Some(Condition::Eq), 0x83c98))?;
    c.emit(0x83c62, I::Shift(Shift::Lsr, R4, R1, 1))?;
    c.emit(0x83c64, I::Alu(Alu::Orr, R4, R1))?;
    c.emit(0x83c66, I::Shift(Shift::Lsr, R5, R4, 2))?;
    c.emit(0x83c68, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x83c6a, I::Shift(Shift::Lsr, R4, R5, 4))?;
    c.emit(0x83c6c, I::Alu(Alu::Orr, R4, R5))?;
    c.emit(0x83c6e, I::Shift(Shift::Lsr, R5, R4, 8))?;
    c.emit(0x83c70, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x83c72, I::Shift(Shift::Lsr, R4, R5, 16))?;
    c.emit(0x83c74, I::Alu(Alu::Orr, R4, R5))?;
    c.emit(0x83c76, I::Alu(Alu::Mvn, R4, R4))?;
    c.emit(0x83c78, I::Shift(Shift::Lsr, R5, R4, 1))?;
    c.emit(0x83c7a, I::Literal(R6, 0x83d1c))?;
    c.emit(0x83c7c, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x83c7e, I::Arithmetic(true, R4, R4, R6))?;
    c.emit(0x83c80, I::Literal(R5, 0x83d20))?;
    c.emit(0x83c82, I::Shift(Shift::Lsr, R6, R4, 2))?;
    c.emit(0x83c84, I::Alu(Alu::And, R4, R5))?;
    c.emit(0x83c86, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x83c88, I::Arithmetic(false, R4, R4, R6))?;
    c.emit(0x83c8a, I::Shift(Shift::Lsr, R5, R4, 4))?;
    c.emit(0x83c8c, I::Arithmetic(false, R4, R4, R5))?;
    c.emit(0x83c8e, I::Literal(R5, 0x83d24))?;
    c.emit(0x83c90, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x83c92, I::Literal(R4, 0x83d28))?;
    c.emit(0x83c94, I::Alu(Alu::Mul, R4, R5))?;
    c.emit(0x83c96, I::Shift(Shift::Lsr, R4, R4, 24))?;
    c.emit(0x83c98, I::Compare(R0, 0))?;
    c.emit(0x83c9a, I::Branch(Some(Condition::Eq), 0x83cd2))?;
    c.emit(0x83c9c, I::Shift(Shift::Lsr, R3, R0, 1))?;
    c.emit(0x83c9e, I::Alu(Alu::Orr, R3, R0))?;
    c.emit(0x83ca0, I::Shift(Shift::Lsr, R5, R3, 2))?;
    c.emit(0x83ca2, I::Alu(Alu::Orr, R5, R3))?;
    c.emit(0x83ca4, I::Shift(Shift::Lsr, R3, R5, 4))?;
    c.emit(0x83ca6, I::Alu(Alu::Orr, R3, R5))?;
    c.emit(0x83ca8, I::Shift(Shift::Lsr, R5, R3, 8))?;
    c.emit(0x83caa, I::Alu(Alu::Orr, R5, R3))?;
    c.emit(0x83cac, I::Shift(Shift::Lsr, R3, R5, 16))?;
    c.emit(0x83cae, I::Alu(Alu::Orr, R3, R5))?;
    c.emit(0x83cb0, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x83cb2, I::Shift(Shift::Lsr, R5, R3, 1))?;
    c.emit(0x83cb4, I::Literal(R6, 0x83d1c))?;
    c.emit(0x83cb6, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x83cb8, I::Arithmetic(true, R3, R3, R6))?;
    c.emit(0x83cba, I::Literal(R5, 0x83d20))?;
    c.emit(0x83cbc, I::Shift(Shift::Lsr, R6, R3, 2))?;
    c.emit(0x83cbe, I::Alu(Alu::And, R3, R5))?;
    c.emit(0x83cc0, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x83cc2, I::Arithmetic(false, R3, R3, R6))?;
    c.emit(0x83cc4, I::Shift(Shift::Lsr, R5, R3, 4))?;
    c.emit(0x83cc6, I::Arithmetic(false, R3, R3, R5))?;
    c.emit(0x83cc8, I::Literal(R5, 0x83d24))?;
    c.emit(0x83cca, I::Alu(Alu::And, R5, R3))?;
    c.emit(0x83ccc, I::Literal(R3, 0x83d28))?;
    c.emit(0x83cce, I::Alu(Alu::Mul, R3, R5))?;
    c.emit(0x83cd0, I::Shift(Shift::Lsr, R3, R3, 24))?;
    c.emit(0x83cd2, I::Arithmetic(true, R4, R4, R3))?;
    c.emit(0x83cd4, I::Compare(R4, 31))?;
    c.emit(0x83cd6, I::Branch(Some(Condition::Hi), 0x83cde))?;
    c.emit(0x83cd8, I::Branch(Some(Condition::Ne), 0x83ce4))?;
    c.emit(0x83cda, I::Push(0x1))?;
    c.emit(0x83cdc, I::Pop(0x4))?;
    c.emit(0x83cde, I::Push(0x4))?;
    c.emit(0x83ce0, I::Pop(0x1))?;
    c.emit(0x83ce2, I::Pop(0x8070))?;
    c.emit(0x83ce4, I::Small(false, R3, R4, 1))?;
    c.emit(0x83ce6, I::Push(0x1))?;
    c.emit(0x83ce8, I::Pop(0x20))?;
    c.emit(0x83cea, I::Alu(Alu::Lsr, R5, R3))?;
    c.emit(0x83cec, I::Mov(R2, 31))?;
    c.emit(0x83cee, I::Arithmetic(true, R2, R2, R4))?;
    c.emit(0x83cf0, I::Alu(Alu::Lsl, R0, R2))?;
    c.emit(0x83cf2, I::Small(true, R4, R1, 1))?;
    c.emit(0x83cf4, I::Mov(R6, 0))?;
    c.emit(0x83cf6, I::Shift(Shift::Lsl, R2, R0, 1))?;
    c.emit(0x83cf8, I::Alu(Alu::Orr, R2, R6))?;
    c.emit(0x83cfa, I::Compare(R3, 0))?;
    c.emit(0x83cfc, I::Branch(Some(Condition::Eq), 0x83cde))?;
    c.emit(0x83cfe, I::Shift(Shift::Lsr, R0, R0, 31))?;
    c.emit(0x83d00, I::Shift(Shift::Lsl, R5, R5, 1))?;
    c.emit(0x83d02, I::Alu(Alu::Orr, R5, R0))?;
    c.emit(0x83d04, I::Arithmetic(true, R0, R4, R5))?;
    c.emit(0x83d06, I::Shift(Shift::Asr, R0, R0, 31))?;
    c.emit(0x83d08, I::Push(0x1))?;
    c.emit(0x83d0a, I::Pop(0x40))?;
    c.emit(0x83d0c, I::Alu(Alu::And, R6, R1))?;
    c.emit(0x83d0e, I::Arithmetic(true, R5, R5, R6))?;
    c.emit(0x83d10, I::Mov(R6, 1))?;
    c.emit(0x83d12, I::Alu(Alu::And, R6, R0))?;
    c.emit(0x83d14, I::Small(true, R3, R3, 1))?;
    c.emit(0x83d16, I::Push(0x4))?;
    c.emit(0x83d18, I::Pop(0x1))?;
    c.emit(0x83d1a, I::Branch(None, 0x83cf6))?;
    c.pool(0x83d1c, 0x55555555)?;
    c.pool(0x83d20, 0x33333333)?;
    c.pool(0x83d24, 0x0f0f0f0f)?;
    c.pool(0x83d28, 0x01010101)?;
    checked((c.at, c.code, c.data, c.align) == (END, CODE_BYTES, DATA_BYTES, ALIGN_BYTES))?;
    Ok(c.bytes)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn half(bytes: &[u8], at: usize) -> u16 {
        u16::from_le_bytes(bytes[at - START..at - START + 2].try_into().unwrap())
    }

    #[test]
    fn extent_digest_partitions_and_pools() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1340);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        for (start, end, digest) in [
            (0x837f0, 0x83d2c, "072b832f47ed230d78a8ff686f5d1dd9c40c55e939fbe60cc926f0ef1c5381e3"),
            (0x837f0, 0x83c3a, "e47865f887dcf8adc5e90b0bb13fab741783a2428de0e191a7686b54088093d1"),
            (0x83c3a, 0x83c3c, "3e39ed78e50103716c788d4147df413950e5eba73c6e33acd923f15865795676"),
            (0x83c3c, 0x83c4c, "7b520efa5bec97643152d100759ea3c6711e85bcb95af6f18463cd1ab759f4b8"),
            (0x83c4c, 0x83d1c, "7c3167d44fa1e4182ee75394cdf41e69da88659d98c01b26eb7c3160ec2431d9"),
            (0x83d1c, 0x83d2c, "7b520efa5bec97643152d100759ea3c6711e85bcb95af6f18463cd1ab759f4b8"),
        ] {
            assert_eq!(crate::wmfw_sha256::hex_digest(&bytes[start - START..end - START]), digest);
        }
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        assert_eq!(half(&bytes, 0x83c3a), 0x46c0);
    }

    #[test]
    fn independently_decoded_control_flow_and_literals() {
        let bytes = fragment().unwrap();
        for (at, target, conditional) in [
            (0x8380c, 0x83862, true),
            (0x83810, 0x8383e, true),
            (0x83814, 0x8382a, true),
            (0x8383c, 0x83c2e, false),
            (0x83842, 0x8384e, true),
            (0x8384c, 0x83c2e, false),
            (0x83860, 0x83844, false),
            (0x8386a, 0x8386e, true),
            (0x8386c, 0x839ba, false),
            (0x83870, 0x83886, true),
            (0x83876, 0x8387e, true),
            (0x8387c, 0x83c2e, false),
            (0x83884, 0x83878, false),
            (0x83888, 0x838aa, true),
            (0x8388c, 0x838a0, true),
            (0x838a8, 0x83832, false),
            (0x838ae, 0x83904, true),
            (0x838b2, 0x838ce, true),
            (0x838d2, 0x838f6, true),
            (0x83902, 0x83c2e, false),
            (0x8390c, 0x83946, true),
            (0x83948, 0x83982, true),
            (0x83986, 0x83998, true),
            (0x8398c, 0x83990, true),
            (0x8398e, 0x83844, false),
            (0x83996, 0x83878, false),
            (0x839b8, 0x83bae, false),
            (0x839bc, 0x839c0, true),
            (0x839be, 0x83afc, false),
            (0x839c4, 0x83a1e, true),
            (0x839c8, 0x839d8, true),
            (0x839da, 0x839de, true),
            (0x839dc, 0x83c2e, false),
            (0x839ea, 0x83a0e, true),
            (0x83a1c, 0x83c2e, false),
            (0x83a26, 0x83a60, true),
            (0x83a68, 0x83aa2, true),
            (0x83aa8, 0x83aae, true),
            (0x83aac, 0x83b8e, false),
            (0x83ab0, 0x83acc, true),
            (0x83aca, 0x83bae, false),
            (0x83afa, 0x83bb2, false),
            (0x83b04, 0x83b3e, true),
            (0x83b40, 0x83b7a, true),
            (0x83b7e, 0x83b82, true),
            (0x83b80, 0x83988, false),
            (0x83b8c, 0x83b98, true),
            (0x83b96, 0x83bb2, false),
            (0x83bc2, 0x83c14, true),
            (0x83c12, 0x83bc0, false),
            (0x83c22, 0x83c28, true),
            (0x83c52, 0x83cde, true),
            (0x83c56, 0x83cde, true),
            (0x83c60, 0x83c98, true),
            (0x83c9a, 0x83cd2, true),
            (0x83cd6, 0x83cde, true),
            (0x83cd8, 0x83ce4, true),
            (0x83cfc, 0x83cde, true),
            (0x83d1a, 0x83cf6, false),
        ] {
            let word = half(&bytes, at);
            assert_eq!(word & (if conditional { 0xf000 } else { 0xf800 }),
                if conditional { 0xd000 } else { 0xe000 });
            let shift = if conditional { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target, arm) in [
            (0x83822, 0x81850, true),
            (0x83832, 0x83c4c, false),
            (0x8389a, 0x81850, true),
] {
            let first = u32::from(half(&bytes, at));
            let second = u32::from(half(&bytes, at + 2));
            assert_eq!(first & 0xf800, 0xf000);
            assert_eq!(second & 0xd000, if arm { 0xc000 } else { 0xd000 });
            if arm { assert_eq!(second & 1, 0); }
            let s = (first >> 10) & 1;
            let i1 = (!(second >> 13) ^ s) & 1;
            let i2 = (!(second >> 11) ^ s) & 1;
            let value = (s << 24) | (i1 << 23) | (i2 << 22)
                | ((first & 0x3ff) << 12) | ((second & 0x7ff) << 1);
            let delta = ((value << 7) as i32 >> 7) as isize;
            let origin = if arm { at & !3 } else { at };
            assert_eq!((origin as isize + 4 + delta) as usize, target);
        }
        for (at, slot) in [
            (0x838d8, 0x83c3c),
            (0x838de, 0x83c40),
            (0x838ec, 0x83c44),
            (0x838f0, 0x83c48),
            (0x83926, 0x83c3c),
            (0x8392c, 0x83c40),
            (0x8393a, 0x83c44),
            (0x8393e, 0x83c48),
            (0x83962, 0x83c3c),
            (0x83968, 0x83c40),
            (0x83976, 0x83c44),
            (0x8397a, 0x83c48),
            (0x839f0, 0x83c3c),
            (0x839f6, 0x83c40),
            (0x83a04, 0x83c44),
            (0x83a08, 0x83c48),
            (0x83a40, 0x83c3c),
            (0x83a46, 0x83c40),
            (0x83a56, 0x83c44),
            (0x83a5a, 0x83c48),
            (0x83a82, 0x83c3c),
            (0x83a88, 0x83c40),
            (0x83a96, 0x83c44),
            (0x83a9a, 0x83c48),
            (0x83b1e, 0x83c3c),
            (0x83b24, 0x83c40),
            (0x83b32, 0x83c44),
            (0x83b36, 0x83c48),
            (0x83b5a, 0x83c3c),
            (0x83b60, 0x83c40),
            (0x83b6e, 0x83c44),
            (0x83b72, 0x83c48),
            (0x83c7a, 0x83d1c),
            (0x83c80, 0x83d20),
            (0x83c8e, 0x83d24),
            (0x83c92, 0x83d28),
            (0x83cb4, 0x83d1c),
            (0x83cba, 0x83d20),
            (0x83cc8, 0x83d24),
            (0x83ccc, 0x83d28),
        ] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn flag_setting_and_wrapping_instruction_forms() {
        let bytes = fragment().unwrap();
        for (at, word) in [
            (0x83850, 0x1c49), (0x83856, 0x4152), (0x838bc, 0x4149),
            (0x83bb6, 0x1e52), (0x83bbc, 0x4151),
            (0x83bd4, 0x1b8d), (0x83bd8, 0x419d),
            (0x83bde, 0x17ec), (0x83bf4, 0x1b75), (0x83bf6, 0x41bb),
            (0x83d06, 0x17c0), (0x83cea, 0x40dd), (0x83cf0, 0x4090),
        ] {
            assert_eq!(half(&bytes, at), word);
        }
        assert_eq!(half(&bytes, 0x83ce2), 0xbd70);
        assert_eq!(half(&bytes, 0x83d1a), 0xe7ec);
        assert_eq!(I::Shift(Shift::Lsl, R0, R1, 0).encode(START).unwrap(), 0x0008u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Asr, R0, R1, 32).encode(START).unwrap(), 0x1008u16.to_le_bytes());
        assert!(I::Call(CallMode::Arm, 0x81850).encode(START + 1).is_err());
    }

    #[test]
    fn operand_and_layout_guards_are_atomic() {
        for instruction in [I::Push(0), I::Push(0x8000), I::Pop(0x4000),
            I::StackAdjust(true, 510), I::Stack(true, R0, 2), I::Address(R1, 1024),
            I::Memory(true, R0, R1, 128), I::Memory(false, R0, R1, 2),
            I::Mov(R0, 256), I::Compare(R0, 256), I::Add(R0, 256), I::Subtract(R0, 256),
            I::Small(true, R0, R1, 8), I::Shift(Shift::Lsl, R0, R1, 32),
            I::Shift(Shift::Lsr, R0, R1, 0), I::Shift(Shift::Lsr, R0, R1, 33),
            I::Shift(Shift::Asr, R0, R1, 0), I::Shift(Shift::Asr, R0, R1, 33),
            I::Call(CallMode::Arm, START + 2),
            I::Literal(R0, START), I::Literal(R0, START + 2048),
            I::Branch(Some(Condition::Eq), START + 260), I::Branch(None, START + 2052),
            I::Branch(None, START + 1), I::Call(CallMode::Thumb, START + 16_777_220)] {
            assert!(instruction.encode(START).is_err(), "{instruction:?}");
        }
        let mut c = Code::new();
        assert!(c.emit(START + 2, I::Mov(R0, 0)).is_err());
        assert!(c.emit(START, I::Align).is_err());
        assert!(c.pool(START, 0).is_err());
        for at in [0x83c38, 0x83c3a, 0x83c3c, 0x83c4a, 0x83d1a, 0x83d1c, END] {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            let instruction = if [0x83c38, 0x83c4a, 0x83d1a].contains(&at) {
                I::Call(CallMode::Thumb, 0x83c4c)
            } else { I::Mov(R0, 0) };
            assert!(c.emit(at, instruction).is_err());
            assert!(c.pool(at, 0).is_err());
            assert_eq!((c.at, c.bytes.clone(), c.code, c.data, c.align), before);
        }
    }

    #[test]
    fn authenticated_same_buffer_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "072b832f47ed230d78a8ff686f5d1dd9c40c55e939fbe60cc926f0ef1c5381e3");
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
