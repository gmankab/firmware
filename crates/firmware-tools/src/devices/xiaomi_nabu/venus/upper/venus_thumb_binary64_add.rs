pub const START: usize = 0x818a0;
pub const END: usize = 0x81d78;
pub const CODE_BYTES: usize = 1218;
pub const DATA_BYTES: usize = 20;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 2;
const POOLS: [(usize, u32); 5] = [
    (0x81c80, 0x7fffffff), (0x81c84, 0x7ff00000),
    (0x81d6c, 0x000fffff), (0x81d70, 0x000007fe), (0x81d74, 0x7ff80000),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Adc, Sbc, Tst, Cmp, Orr, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Hs = 2, Lo = 3, Hi = 8, Ls = 9, Ge = 10, Le = 13 }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(bool, u16),
    Stack(bool, R, u16), Address(R, u16), Literal(R, usize),
    Alu(Alu, R, R), Shift(Shift, R, R, u8),
    Mov(R, u16), Compare(R, u16), Subtract(R, u16),
    Small(bool, R, R, u8), Arithmetic(bool, R, R, R),
    Branch(Option<Condition>, usize), Call(usize), Align,
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
                    Alu::And => 0, Alu::Eor => 1, Alu::Lsl => 2, Alu::Lsr => 3,
                    Alu::Adc => 5, Alu::Sbc => 6, Alu::Tst => 8, Alu::Cmp => 10,
                    Alu::Orr => 12, Alu::Mvn => 15,
                };
                0x4000 | (op << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Shift(kind, dst, src, amount) => {
                let amount = match kind {
                    Shift::Lsl => { checked(amount < 32)?; amount },
                    Shift::Lsr => { checked((1..=32).contains(&amount))?; amount % 32 },
                };
                (if matches!(kind, Shift::Lsr) { 0x0800 } else { 0 })
                    | ((amount as u16) << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Mov(r, value) | Self::Compare(r, value) | Self::Subtract(r, value) => {
                checked(value <= 255)?;
                let base = match self { Self::Mov(..) => 0x2000, Self::Compare(..) => 0x2800, _ => 0x3800 };
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
            Self::Call(target) => {
                let delta = displacement(at, target, -16_777_216, 16_777_214)? as u32;
                let s = (delta >> 24) & 1;
                let j1 = (!(delta >> 23) ^ s) & 1;
                let j2 = (!(delta >> 22) ^ s) & 1;
                let first = 0xf000 | (s << 10) | ((delta >> 12) & 0x3ff);
                let second = 0xd000 | (j1 << 13) | (j2 << 11) | ((delta >> 1) & 0x7ff);
                return Ok([ (first as u16).to_le_bytes(), (second as u16).to_le_bytes() ].concat());
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
        checked(matches!(instruction, I::Align) == (at == 0x81c7e))?;
        checked(at + bytes.len() <= 0x81c7e || at >= 0x81c80 || matches!(instruction, I::Align))?;
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
    c.emit(0x818a0, I::Push(0x40f0))?;
    c.emit(0x818a2, I::StackAdjust(true, 0x4c))?;
    c.emit(0x818a4, I::Stack(false, R3, 0x30))?;
    c.emit(0x818a6, I::Stack(false, R2, 0x28))?;
    c.emit(0x818a8, I::Push(0x2))?;
    c.emit(0x818aa, I::Pop(0x10))?;
    c.emit(0x818ac, I::Literal(R2, 0x81c80))?;
    c.emit(0x818ae, I::Stack(false, R4, 0x24))?;
    c.emit(0x818b0, I::Stack(false, R2, 0x34))?;
    c.emit(0x818b2, I::Alu(Alu::And, R4, R2))?;
    c.emit(0x818b4, I::Mov(R3, 0))?;
    c.emit(0x818b6, I::Alu(Alu::Mvn, R1, R3))?;
    c.emit(0x818b8, I::Stack(false, R0, 0x2c))?;
    c.emit(0x818ba, I::Small(true, R0, R0, 1))?;
    c.emit(0x818bc, I::Push(0x10))?;
    c.emit(0x818be, I::Pop(0x80))?;
    c.emit(0x818c0, I::Stack(false, R1, 0x18))?;
    c.emit(0x818c2, I::Alu(Alu::Adc, R7, R1))?;
    c.emit(0x818c4, I::Small(true, R2, R3, 2))?;
    c.emit(0x818c6, I::Mov(R1, 1))?;
    c.emit(0x818c8, I::Alu(Alu::Cmp, R0, R2))?;
    c.emit(0x818ca, I::Push(0x2))?;
    c.emit(0x818cc, I::Pop(0x40))?;
    c.emit(0x818ce, I::Branch(Some(Condition::Hi), 0x818d4))?;
    c.emit(0x818d0, I::Push(0x8))?;
    c.emit(0x818d2, I::Pop(0x40))?;
    c.emit(0x818d4, I::Literal(R0, 0x81c84))?;
    c.emit(0x818d6, I::Stack(false, R0, 0x20))?;
    c.emit(0x818d8, I::Small(true, R5, R0, 1))?;
    c.emit(0x818da, I::Alu(Alu::Cmp, R7, R5))?;
    c.emit(0x818dc, I::Push(0x2))?;
    c.emit(0x818de, I::Pop(0x1))?;
    c.emit(0x818e0, I::Branch(Some(Condition::Hi), 0x818e6))?;
    c.emit(0x818e2, I::Push(0x8))?;
    c.emit(0x818e4, I::Pop(0x1))?;
    c.emit(0x818e6, I::Alu(Alu::Cmp, R7, R5))?;
    c.emit(0x818e8, I::Branch(Some(Condition::Eq), 0x818ee))?;
    c.emit(0x818ea, I::Push(0x1))?;
    c.emit(0x818ec, I::Pop(0x40))?;
    c.emit(0x818ee, I::Stack(true, R3, 0x30))?;
    c.emit(0x818f0, I::Stack(true, R0, 0x34))?;
    c.emit(0x818f2, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x818f4, I::Shift(Shift::Lsl, R0, R1, 19))?;
    c.emit(0x818f6, I::Stack(false, R0, 0x1c))?;
    c.emit(0x818f8, I::Compare(R6, 0))?;
    c.emit(0x818fa, I::Branch(Some(Condition::Ne), 0x8195c))?;
    c.emit(0x818fc, I::Mov(R1, 0))?;
    c.emit(0x818fe, I::Alu(Alu::Mvn, R7, R1))?;
    c.emit(0x81900, I::Stack(true, R0, 0x28))?;
    c.emit(0x81902, I::Small(true, R2, R0, 1))?;
    c.emit(0x81904, I::Alu(Alu::Adc, R7, R3))?;
    c.emit(0x81906, I::Mov(R6, 1))?;
    c.emit(0x81908, I::Alu(Alu::Cmp, R7, R5))?;
    c.emit(0x8190a, I::Push(0x40))?;
    c.emit(0x8190c, I::Pop(0x1))?;
    c.emit(0x8190e, I::Branch(Some(Condition::Hi), 0x81914))?;
    c.emit(0x81910, I::Push(0x2))?;
    c.emit(0x81912, I::Pop(0x1))?;
    c.emit(0x81914, I::Small(false, R2, R2, 1))?;
    c.emit(0x81916, I::Branch(Some(Condition::Eq), 0x8191c))?;
    c.emit(0x81918, I::Push(0x2))?;
    c.emit(0x8191a, I::Pop(0x40))?;
    c.emit(0x8191c, I::Alu(Alu::Cmp, R7, R5))?;
    c.emit(0x8191e, I::Branch(Some(Condition::Eq), 0x81924))?;
    c.emit(0x81920, I::Push(0x1))?;
    c.emit(0x81922, I::Pop(0x40))?;
    c.emit(0x81924, I::Compare(R6, 0))?;
    c.emit(0x81926, I::Stack(true, R6, 0x20))?;
    c.emit(0x81928, I::Stack(true, R7, 0x28))?;
    c.emit(0x8192a, I::Branch(Some(Condition::Eq), 0x81a14))?;
    c.emit(0x8192c, I::Mov(R5, 1))?;
    c.emit(0x8192e, I::Mov(R2, 0))?;
    c.emit(0x81930, I::Alu(Alu::Cmp, R3, R6))?;
    c.emit(0x81932, I::Push(0x20))?;
    c.emit(0x81934, I::Pop(0x2))?;
    c.emit(0x81936, I::Branch(Some(Condition::Ls), 0x8193c))?;
    c.emit(0x81938, I::Push(0x4))?;
    c.emit(0x8193a, I::Pop(0x2))?;
    c.emit(0x8193c, I::Compare(R7, 0))?;
    c.emit(0x8193e, I::Branch(Some(Condition::Eq), 0x81944))?;
    c.emit(0x81940, I::Push(0x4))?;
    c.emit(0x81942, I::Pop(0x20))?;
    c.emit(0x81944, I::Alu(Alu::Cmp, R3, R6))?;
    c.emit(0x81946, I::Branch(Some(Condition::Eq), 0x8194c))?;
    c.emit(0x81948, I::Push(0x2))?;
    c.emit(0x8194a, I::Pop(0x20))?;
    c.emit(0x8194c, I::Compare(R5, 0))?;
    c.emit(0x8194e, I::Branch(Some(Condition::Ne), 0x81990))?;
    c.emit(0x81950, I::Stack(true, R1, 0x30))?;
    c.emit(0x81952, I::Stack(true, R0, 0x1c))?;
    c.emit(0x81954, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x81956, I::Push(0x80))?;
    c.emit(0x81958, I::Pop(0x1))?;
    c.emit(0x8195a, I::Branch(None, 0x81d5c))?;
    c.emit(0x8195c, I::Mov(R5, 1))?;
    c.emit(0x8195e, I::Mov(R2, 0))?;
    c.emit(0x81960, I::Stack(true, R6, 0x20))?;
    c.emit(0x81962, I::Alu(Alu::Cmp, R4, R6))?;
    c.emit(0x81964, I::Push(0x20))?;
    c.emit(0x81966, I::Pop(0x2))?;
    c.emit(0x81968, I::Branch(Some(Condition::Ls), 0x8196e))?;
    c.emit(0x8196a, I::Push(0x4))?;
    c.emit(0x8196c, I::Pop(0x2))?;
    c.emit(0x8196e, I::Stack(true, R0, 0x2c))?;
    c.emit(0x81970, I::Compare(R0, 0))?;
    c.emit(0x81972, I::Stack(true, R7, 0x28))?;
    c.emit(0x81974, I::Branch(Some(Condition::Eq), 0x8197a))?;
    c.emit(0x81976, I::Push(0x4))?;
    c.emit(0x81978, I::Pop(0x20))?;
    c.emit(0x8197a, I::Alu(Alu::Cmp, R4, R6))?;
    c.emit(0x8197c, I::Branch(Some(Condition::Eq), 0x81982))?;
    c.emit(0x8197e, I::Push(0x2))?;
    c.emit(0x81980, I::Pop(0x20))?;
    c.emit(0x81982, I::Compare(R5, 0))?;
    c.emit(0x81984, I::Branch(Some(Condition::Ne), 0x8192c))?;
    c.emit(0x81986, I::Stack(true, R1, 0x24))?;
    c.emit(0x81988, I::Stack(true, R0, 0x1c))?;
    c.emit(0x8198a, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x8198c, I::Stack(true, R0, 0x2c))?;
    c.emit(0x8198e, I::Branch(None, 0x81d5c))?;
    c.emit(0x81990, I::Push(0x10))?;
    c.emit(0x81992, I::Pop(0x1))?;
    c.emit(0x81994, I::Alu(Alu::Eor, R0, R6))?;
    c.emit(0x81996, I::Stack(true, R5, 0x2c))?;
    c.emit(0x81998, I::Alu(Alu::Orr, R0, R5))?;
    c.emit(0x8199a, I::Compare(R0, 0))?;
    c.emit(0x8199c, I::Branch(Some(Condition::Ne), 0x819ca))?;
    c.emit(0x8199e, I::Mov(R0, 0))?;
    c.emit(0x819a0, I::Stack(true, R1, 0x18))?;
    c.emit(0x819a2, I::Small(false, R1, R1, 1))?;
    c.emit(0x819a4, I::Stack(true, R2, 0x34))?;
    c.emit(0x819a6, I::Alu(Alu::Adc, R2, R0))?;
    c.emit(0x819a8, I::Stack(true, R0, 0x24))?;
    c.emit(0x819aa, I::Stack(true, R3, 0x30))?;
    c.emit(0x819ac, I::Alu(Alu::Eor, R3, R0))?;
    c.emit(0x819ae, I::Alu(Alu::Eor, R3, R2))?;
    c.emit(0x819b0, I::Alu(Alu::Eor, R7, R5))?;
    c.emit(0x819b2, I::Alu(Alu::Eor, R7, R1))?;
    c.emit(0x819b4, I::Alu(Alu::Orr, R7, R3))?;
    c.emit(0x819b6, I::Compare(R7, 0))?;
    c.emit(0x819b8, I::Push(0x20))?;
    c.emit(0x819ba, I::Pop(0x40))?;
    c.emit(0x819bc, I::Push(0x1))?;
    c.emit(0x819be, I::Pop(0x2))?;
    c.emit(0x819c0, I::Branch(Some(Condition::Eq), 0x819c4))?;
    c.emit(0x819c2, I::Branch(None, 0x81d64))?;
    c.emit(0x819c4, I::Mov(R0, 0))?;
    c.emit(0x819c6, I::Literal(R1, 0x81d74))?;
    c.emit(0x819c8, I::Branch(None, 0x81d5c))?;
    c.emit(0x819ca, I::Push(0x8))?;
    c.emit(0x819cc, I::Pop(0x1))?;
    c.emit(0x819ce, I::Alu(Alu::Eor, R0, R6))?;
    c.emit(0x819d0, I::Alu(Alu::Orr, R0, R7))?;
    c.emit(0x819d2, I::Compare(R0, 0))?;
    c.emit(0x819d4, I::Branch(Some(Condition::Ne), 0x819de))?;
    c.emit(0x819d6, I::Push(0x80))?;
    c.emit(0x819d8, I::Pop(0x40))?;
    c.emit(0x819da, I::Stack(true, R1, 0x30))?;
    c.emit(0x819dc, I::Branch(None, 0x81d64))?;
    c.emit(0x819de, I::Push(0x20))?;
    c.emit(0x819e0, I::Pop(0x1))?;
    c.emit(0x819e2, I::Alu(Alu::Orr, R0, R4))?;
    c.emit(0x819e4, I::Compare(R0, 0))?;
    c.emit(0x819e6, I::Branch(Some(Condition::Ne), 0x81a02))?;
    c.emit(0x819e8, I::Alu(Alu::Orr, R3, R7))?;
    c.emit(0x819ea, I::Compare(R3, 0))?;
    c.emit(0x819ec, I::Push(0x80))?;
    c.emit(0x819ee, I::Pop(0x40))?;
    c.emit(0x819f0, I::Stack(true, R1, 0x30))?;
    c.emit(0x819f2, I::Branch(Some(Condition::Eq), 0x819f6))?;
    c.emit(0x819f4, I::Branch(None, 0x81d64))?;
    c.emit(0x819f6, I::Stack(true, R0, 0x2c))?;
    c.emit(0x819f8, I::Alu(Alu::And, R7, R0))?;
    c.emit(0x819fa, I::Stack(true, R0, 0x24))?;
    c.emit(0x819fc, I::Stack(true, R1, 0x30))?;
    c.emit(0x819fe, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x81a00, I::Branch(None, 0x81956))?;
    c.emit(0x81a02, I::Push(0x80))?;
    c.emit(0x81a04, I::Pop(0x1))?;
    c.emit(0x81a06, I::Alu(Alu::Orr, R0, R3))?;
    c.emit(0x81a08, I::Compare(R0, 0))?;
    c.emit(0x81a0a, I::Push(0x20))?;
    c.emit(0x81a0c, I::Pop(0x40))?;
    c.emit(0x81a0e, I::Stack(true, R1, 0x24))?;
    c.emit(0x81a10, I::Branch(Some(Condition::Ne), 0x81a14))?;
    c.emit(0x81a12, I::Branch(None, 0x81d64))?;
    c.emit(0x81a14, I::Mov(R5, 1))?;
    c.emit(0x81a16, I::Mov(R1, 0))?;
    c.emit(0x81a18, I::Stack(true, R0, 0x2c))?;
    c.emit(0x81a1a, I::Alu(Alu::Cmp, R7, R0))?;
    c.emit(0x81a1c, I::Push(0x20))?;
    c.emit(0x81a1e, I::Pop(0x40))?;
    c.emit(0x81a20, I::Branch(Some(Condition::Hi), 0x81a26))?;
    c.emit(0x81a22, I::Push(0x2))?;
    c.emit(0x81a24, I::Pop(0x40))?;
    c.emit(0x81a26, I::Alu(Alu::Cmp, R3, R4))?;
    c.emit(0x81a28, I::Branch(Some(Condition::Hi), 0x81a2e))?;
    c.emit(0x81a2a, I::Push(0x2))?;
    c.emit(0x81a2c, I::Pop(0x20))?;
    c.emit(0x81a2e, I::Alu(Alu::Cmp, R3, R4))?;
    c.emit(0x81a30, I::Branch(Some(Condition::Eq), 0x81a36))?;
    c.emit(0x81a32, I::Push(0x20))?;
    c.emit(0x81a34, I::Pop(0x40))?;
    c.emit(0x81a36, I::Compare(R6, 0))?;
    c.emit(0x81a38, I::Push(0x80))?;
    c.emit(0x81a3a, I::Pop(0x1))?;
    c.emit(0x81a3c, I::Stack(true, R2, 0x24))?;
    c.emit(0x81a3e, I::Branch(Some(Condition::Ne), 0x81a42))?;
    c.emit(0x81a40, I::Stack(true, R0, 0x2c))?;
    c.emit(0x81a42, I::Stack(false, R0, 0x28))?;
    c.emit(0x81a44, I::Stack(false, R0, 0x40))?;
    c.emit(0x81a46, I::Compare(R6, 0))?;
    c.emit(0x81a48, I::Stack(true, R5, 0x30))?;
    c.emit(0x81a4a, I::Branch(Some(Condition::Ne), 0x81a50))?;
    c.emit(0x81a4c, I::Push(0x4))?;
    c.emit(0x81a4e, I::Pop(0x20))?;
    c.emit(0x81a50, I::Literal(R3, 0x81d6c))?;
    c.emit(0x81a52, I::Push(0x20))?;
    c.emit(0x81a54, I::Pop(0x1))?;
    c.emit(0x81a56, I::Alu(Alu::And, R0, R3))?;
    c.emit(0x81a58, I::Stack(false, R0, 0x44))?;
    c.emit(0x81a5a, I::Compare(R6, 0))?;
    c.emit(0x81a5c, I::Branch(Some(Condition::Ne), 0x81a60))?;
    c.emit(0x81a5e, I::Stack(true, R2, 0x30))?;
    c.emit(0x81a60, I::Push(0x4))?;
    c.emit(0x81a62, I::Pop(0x1))?;
    c.emit(0x81a64, I::Alu(Alu::And, R0, R3))?;
    c.emit(0x81a66, I::Stack(false, R0, 0x3c))?;
    c.emit(0x81a68, I::Compare(R6, 0))?;
    c.emit(0x81a6a, I::Branch(Some(Condition::Ne), 0x81a6e))?;
    c.emit(0x81a6c, I::Stack(false, R7, 0x2c))?;
    c.emit(0x81a6e, I::Stack(false, R3, 0xc))?;
    c.emit(0x81a70, I::Stack(true, R0, 0x2c))?;
    c.emit(0x81a72, I::Stack(false, R0, 0x38))?;
    c.emit(0x81a74, I::Shift(Shift::Lsr, R6, R2, 20))?;
    c.emit(0x81a76, I::Literal(R0, 0x81d70))?;
    c.emit(0x81a78, I::Stack(false, R0, 0x1c))?;
    c.emit(0x81a7a, I::Small(false, R0, R0, 1))?;
    c.emit(0x81a7c, I::Alu(Alu::And, R6, R0))?;
    c.emit(0x81a7e, I::Shift(Shift::Lsr, R3, R5, 20))?;
    c.emit(0x81a80, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x81a82, I::Alu(Alu::Mvn, R0, R1))?;
    c.emit(0x81a84, I::Stack(false, R0, 0x10))?;
    c.emit(0x81a86, I::Compare(R3, 0))?;
    c.emit(0x81a88, I::Stack(false, R2, 0x24))?;
    c.emit(0x81a8a, I::Branch(Some(Condition::Ne), 0x81a96))?;
    c.emit(0x81a8c, I::Address(R0, 0x40))?;
    c.emit(0x81a8e, I::Call(0x81d7a))?;
    c.emit(0x81a92, I::Push(0x1))?;
    c.emit(0x81a94, I::Pop(0x8))?;
    c.emit(0x81a96, I::Stack(false, R3, 0x30))?;
    c.emit(0x81a98, I::Mov(R0, 1))?;
    c.emit(0x81a9a, I::Stack(false, R0, 0x8))?;
    c.emit(0x81a9c, I::Shift(Shift::Lsl, R3, R0, 23))?;
    c.emit(0x81a9e, I::Compare(R6, 0))?;
    c.emit(0x81aa0, I::Stack(false, R3, 0x14))?;
    c.emit(0x81aa2, I::Branch(Some(Condition::Ne), 0x81ab0))?;
    c.emit(0x81aa4, I::Address(R0, 0x38))?;
    c.emit(0x81aa6, I::Call(0x81d7a))?;
    c.emit(0x81aaa, I::Stack(true, R3, 0x14))?;
    c.emit(0x81aac, I::Push(0x1))?;
    c.emit(0x81aae, I::Pop(0x40))?;
    c.emit(0x81ab0, I::Stack(true, R0, 0x40))?;
    c.emit(0x81ab2, I::Shift(Shift::Lsl, R2, R0, 3))?;
    c.emit(0x81ab4, I::Stack(true, R1, 0x44))?;
    c.emit(0x81ab6, I::Stack(false, R2, 0x2c))?;
    c.emit(0x81ab8, I::Stack(false, R2, 0x40))?;
    c.emit(0x81aba, I::Shift(Shift::Lsr, R0, R0, 29))?;
    c.emit(0x81abc, I::Shift(Shift::Lsl, R7, R1, 3))?;
    c.emit(0x81abe, I::Alu(Alu::Orr, R7, R0))?;
    c.emit(0x81ac0, I::Alu(Alu::Orr, R7, R3))?;
    c.emit(0x81ac2, I::Stack(false, R7, 0x44))?;
    c.emit(0x81ac4, I::Stack(true, R1, 0x38))?;
    c.emit(0x81ac6, I::Shift(Shift::Lsl, R4, R1, 3))?;
    c.emit(0x81ac8, I::Stack(true, R2, 0x3c))?;
    c.emit(0x81aca, I::Stack(false, R4, 0x38))?;
    c.emit(0x81acc, I::Push(0x8))?;
    c.emit(0x81ace, I::Pop(0x1))?;
    c.emit(0x81ad0, I::Shift(Shift::Lsr, R3, R1, 29))?;
    c.emit(0x81ad2, I::Shift(Shift::Lsl, R1, R2, 3))?;
    c.emit(0x81ad4, I::Alu(Alu::Orr, R1, R3))?;
    c.emit(0x81ad6, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x81ad8, I::Stack(false, R1, 0x3c))?;
    c.emit(0x81ada, I::Mov(R2, 0))?;
    c.emit(0x81adc, I::Stack(true, R0, 0x18))?;
    c.emit(0x81ade, I::Small(false, R3, R0, 1))?;
    c.emit(0x81ae0, I::Stack(true, R0, 0x34))?;
    c.emit(0x81ae2, I::Alu(Alu::Adc, R0, R2))?;
    c.emit(0x81ae4, I::Alu(Alu::And, R0, R5))?;
    c.emit(0x81ae6, I::Stack(false, R0, 0x34))?;
    c.emit(0x81ae8, I::Stack(true, R0, 0x28))?;
    c.emit(0x81aea, I::Alu(Alu::And, R0, R3))?;
    c.emit(0x81aec, I::Stack(true, R3, 0x30))?;
    c.emit(0x81aee, I::Arithmetic(true, R6, R3, R6))?;
    c.emit(0x81af0, I::Stack(true, R3, 0x24))?;
    c.emit(0x81af2, I::Alu(Alu::Eor, R3, R5))?;
    c.emit(0x81af4, I::Compare(R6, 0))?;
    c.emit(0x81af6, I::Branch(Some(Condition::Ne), 0x81afe))?;
    c.emit(0x81af8, I::Push(0x2))?;
    c.emit(0x81afa, I::Pop(0x4))?;
    c.emit(0x81afc, I::Branch(None, 0x81ba0))?;
    c.emit(0x81afe, I::Stack(false, R4, 0x28))?;
    c.emit(0x81b00, I::Compare(R6, 63))?;
    c.emit(0x81b02, I::Branch(Some(Condition::Hi), 0x81b9a))?;
    c.emit(0x81b04, I::Stack(false, R3, 0x24))?;
    c.emit(0x81b06, I::Push(0x2))?;
    c.emit(0x81b08, I::Pop(0x8))?;
    c.emit(0x81b0a, I::Alu(Alu::Lsr, R3, R6))?;
    c.emit(0x81b0c, I::Mov(R4, 0))?;
    c.emit(0x81b0e, I::Push(0x40))?;
    c.emit(0x81b10, I::Pop(0x4))?;
    c.emit(0x81b12, I::Subtract(R2, 32))?;
    c.emit(0x81b14, I::Stack(false, R2, 0x8))?;
    c.emit(0x81b16, I::Compare(R2, 0))?;
    c.emit(0x81b18, I::Stack(false, R4, 0x18))?;
    c.emit(0x81b1a, I::Push(0x10))?;
    c.emit(0x81b1c, I::Pop(0x4))?;
    c.emit(0x81b1e, I::Branch(Some(Condition::Ge), 0x81b24))?;
    c.emit(0x81b20, I::Push(0x8))?;
    c.emit(0x81b22, I::Pop(0x4))?;
    c.emit(0x81b24, I::Stack(false, R2, 0x3c))?;
    c.emit(0x81b26, I::Mov(R3, 32))?;
    c.emit(0x81b28, I::Stack(false, R3, 0x0))?;
    c.emit(0x81b2a, I::Arithmetic(true, R5, R3, R6))?;
    c.emit(0x81b2c, I::Mov(R3, 64))?;
    c.emit(0x81b2e, I::Arithmetic(true, R4, R3, R6))?;
    c.emit(0x81b30, I::Stack(true, R3, 0x28))?;
    c.emit(0x81b32, I::Alu(Alu::Lsl, R3, R4))?;
    c.emit(0x81b34, I::Stack(false, R5, 0x4))?;
    c.emit(0x81b36, I::Compare(R5, 0))?;
    c.emit(0x81b38, I::Stack(true, R5, 0x18))?;
    c.emit(0x81b3a, I::Branch(Some(Condition::Ge), 0x81b40))?;
    c.emit(0x81b3c, I::Push(0x8))?;
    c.emit(0x81b3e, I::Pop(0x20))?;
    c.emit(0x81b40, I::Stack(false, R5, 0x18))?;
    c.emit(0x81b42, I::Push(0x2))?;
    c.emit(0x81b44, I::Pop(0x8))?;
    c.emit(0x81b46, I::Alu(Alu::Lsl, R3, R4))?;
    c.emit(0x81b48, I::Stack(true, R5, 0x0))?;
    c.emit(0x81b4a, I::Arithmetic(true, R4, R5, R4))?;
    c.emit(0x81b4c, I::Stack(false, R4, 0x0))?;
    c.emit(0x81b4e, I::Stack(true, R4, 0x28))?;
    c.emit(0x81b50, I::Stack(true, R5, 0x0))?;
    c.emit(0x81b52, I::Alu(Alu::Lsr, R4, R5))?;
    c.emit(0x81b54, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x81b56, I::Stack(true, R3, 0x28))?;
    c.emit(0x81b58, I::Stack(true, R5, 0x4))?;
    c.emit(0x81b5a, I::Alu(Alu::Lsl, R3, R5))?;
    c.emit(0x81b5c, I::Compare(R5, 0))?;
    c.emit(0x81b5e, I::Branch(Some(Condition::Ge), 0x81b64))?;
    c.emit(0x81b60, I::Push(0x10))?;
    c.emit(0x81b62, I::Pop(0x8))?;
    c.emit(0x81b64, I::Stack(true, R4, 0x18))?;
    c.emit(0x81b66, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x81b68, I::Mov(R3, 1))?;
    c.emit(0x81b6a, I::Compare(R4, 0))?;
    c.emit(0x81b6c, I::Branch(Some(Condition::Ne), 0x81b72))?;
    c.emit(0x81b6e, I::Push(0x10))?;
    c.emit(0x81b70, I::Pop(0x8))?;
    c.emit(0x81b72, I::Stack(false, R3, 0x18))?;
    c.emit(0x81b74, I::Stack(true, R3, 0x28))?;
    c.emit(0x81b76, I::Alu(Alu::Lsr, R3, R6))?;
    c.emit(0x81b78, I::Push(0x2))?;
    c.emit(0x81b7a, I::Pop(0x10))?;
    c.emit(0x81b7c, I::Alu(Alu::Lsl, R4, R5))?;
    c.emit(0x81b7e, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x81b80, I::Stack(true, R3, 0x8))?;
    c.emit(0x81b82, I::Alu(Alu::Lsr, R1, R3))?;
    c.emit(0x81b84, I::Compare(R3, 0))?;
    c.emit(0x81b86, I::Stack(true, R3, 0x24))?;
    c.emit(0x81b88, I::Branch(Some(Condition::Ge), 0x81b8e))?;
    c.emit(0x81b8a, I::Push(0x10))?;
    c.emit(0x81b8c, I::Pop(0x2))?;
    c.emit(0x81b8e, I::Stack(true, R4, 0x18))?;
    c.emit(0x81b90, I::Alu(Alu::Orr, R1, R4))?;
    c.emit(0x81b92, I::Stack(false, R1, 0x38))?;
    c.emit(0x81b94, I::Push(0x2))?;
    c.emit(0x81b96, I::Pop(0x10))?;
    c.emit(0x81b98, I::Branch(None, 0x81ba0))?;
    c.emit(0x81b9a, I::Stack(false, R2, 0x3c))?;
    c.emit(0x81b9c, I::Stack(true, R4, 0x8))?;
    c.emit(0x81b9e, I::Stack(false, R4, 0x38))?;
    c.emit(0x81ba0, I::Stack(false, R0, 0x28))?;
    c.emit(0x81ba2, I::Compare(R3, 0))?;
    c.emit(0x81ba4, I::Branch(Some(Condition::Ge), 0x81c34))?;
    c.emit(0x81ba6, I::Stack(true, R0, 0x2c))?;
    c.emit(0x81ba8, I::Arithmetic(true, R5, R0, R4))?;
    c.emit(0x81baa, I::Alu(Alu::Sbc, R7, R2))?;
    c.emit(0x81bac, I::Stack(false, R7, 0x44))?;
    c.emit(0x81bae, I::Stack(false, R5, 0x40))?;
    c.emit(0x81bb0, I::Push(0x20))?;
    c.emit(0x81bb2, I::Pop(0x2))?;
    c.emit(0x81bb4, I::Alu(Alu::Orr, R1, R7))?;
    c.emit(0x81bb6, I::Mov(R0, 0))?;
    c.emit(0x81bb8, I::Compare(R1, 0))?;
    c.emit(0x81bba, I::Branch(Some(Condition::Ne), 0x81bc2))?;
    c.emit(0x81bbc, I::Push(0x1))?;
    c.emit(0x81bbe, I::Pop(0x2))?;
    c.emit(0x81bc0, I::Branch(None, 0x81d5c))?;
    c.emit(0x81bc2, I::Mov(R0, 1))?;
    c.emit(0x81bc4, I::Mov(R2, 0))?;
    c.emit(0x81bc6, I::Stack(true, R3, 0x14))?;
    c.emit(0x81bc8, I::Alu(Alu::Cmp, R7, R3))?;
    c.emit(0x81bca, I::Push(0x1))?;
    c.emit(0x81bcc, I::Pop(0x2))?;
    c.emit(0x81bce, I::Branch(Some(Condition::Hs), 0x81bd4))?;
    c.emit(0x81bd0, I::Push(0x4))?;
    c.emit(0x81bd2, I::Pop(0x2))?;
    c.emit(0x81bd4, I::Alu(Alu::Cmp, R7, R3))?;
    c.emit(0x81bd6, I::Branch(Some(Condition::Eq), 0x81bdc))?;
    c.emit(0x81bd8, I::Push(0x2))?;
    c.emit(0x81bda, I::Pop(0x1))?;
    c.emit(0x81bdc, I::Compare(R0, 0))?;
    c.emit(0x81bde, I::Branch(Some(Condition::Eq), 0x81bea))?;
    c.emit(0x81be0, I::Push(0x20))?;
    c.emit(0x81be2, I::Pop(0x2))?;
    c.emit(0x81be4, I::Push(0x80))?;
    c.emit(0x81be6, I::Pop(0x20))?;
    c.emit(0x81be8, I::Branch(None, 0x81c4a))?;
    c.emit(0x81bea, I::Push(0x20))?;
    c.emit(0x81bec, I::Pop(0x1))?;
    c.emit(0x81bee, I::Push(0x80))?;
    c.emit(0x81bf0, I::Pop(0x2))?;
    c.emit(0x81bf2, I::Call(0x81dd0))?;
    c.emit(0x81bf6, I::Push(0x1))?;
    c.emit(0x81bf8, I::Pop(0x4))?;
    c.emit(0x81bfa, I::Subtract(R2, 8))?;
    c.emit(0x81bfc, I::Push(0x20))?;
    c.emit(0x81bfe, I::Pop(0x10))?;
    c.emit(0x81c00, I::Alu(Alu::Lsl, R4, R2))?;
    c.emit(0x81c02, I::Mov(R1, 0))?;
    c.emit(0x81c04, I::Push(0x1))?;
    c.emit(0x81c06, I::Pop(0x8))?;
    c.emit(0x81c08, I::Subtract(R3, 40))?;
    c.emit(0x81c0a, I::Compare(R3, 0))?;
    c.emit(0x81c0c, I::Branch(Some(Condition::Ge), 0x81c12))?;
    c.emit(0x81c0e, I::Push(0x10))?;
    c.emit(0x81c10, I::Pop(0x2))?;
    c.emit(0x81c12, I::Stack(false, R1, 0x40))?;
    c.emit(0x81c14, I::Alu(Alu::Lsl, R7, R2))?;
    c.emit(0x81c16, I::Mov(R4, 40))?;
    c.emit(0x81c18, I::Arithmetic(true, R4, R4, R0))?;
    c.emit(0x81c1a, I::Push(0x20))?;
    c.emit(0x81c1c, I::Pop(0x1))?;
    c.emit(0x81c1e, I::Alu(Alu::Lsr, R0, R4))?;
    c.emit(0x81c20, I::Alu(Alu::Orr, R0, R7))?;
    c.emit(0x81c22, I::Alu(Alu::Lsl, R5, R3))?;
    c.emit(0x81c24, I::Compare(R3, 0))?;
    c.emit(0x81c26, I::Branch(Some(Condition::Ge), 0x81c2c))?;
    c.emit(0x81c28, I::Push(0x1))?;
    c.emit(0x81c2a, I::Pop(0x20))?;
    c.emit(0x81c2c, I::Stack(false, R5, 0x44))?;
    c.emit(0x81c2e, I::Stack(true, R6, 0x30))?;
    c.emit(0x81c30, I::Arithmetic(true, R6, R6, R2))?;
    c.emit(0x81c32, I::Branch(None, 0x81c64))?;
    c.emit(0x81c34, I::Stack(true, R0, 0x2c))?;
    c.emit(0x81c36, I::Arithmetic(false, R1, R4, R0))?;
    c.emit(0x81c38, I::Alu(Alu::Adc, R2, R7))?;
    c.emit(0x81c3a, I::Stack(false, R2, 0x44))?;
    c.emit(0x81c3c, I::Stack(false, R1, 0x40))?;
    c.emit(0x81c3e, I::Mov(R0, 1))?;
    c.emit(0x81c40, I::Shift(Shift::Lsl, R3, R0, 24))?;
    c.emit(0x81c42, I::Alu(Alu::Tst, R2, R3))?;
    c.emit(0x81c44, I::Branch(Some(Condition::Ne), 0x81c50))?;
    c.emit(0x81c46, I::Push(0x4))?;
    c.emit(0x81c48, I::Pop(0x20))?;
    c.emit(0x81c4a, I::Stack(true, R0, 0x20))?;
    c.emit(0x81c4c, I::Stack(true, R6, 0x30))?;
    c.emit(0x81c4e, I::Branch(None, 0x81c66))?;
    c.emit(0x81c50, I::Shift(Shift::Lsr, R5, R2, 1))?;
    c.emit(0x81c52, I::Stack(false, R5, 0x44))?;
    c.emit(0x81c54, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x81c56, I::Shift(Shift::Lsr, R3, R1, 1))?;
    c.emit(0x81c58, I::Shift(Shift::Lsl, R1, R2, 31))?;
    c.emit(0x81c5a, I::Alu(Alu::Orr, R1, R3))?;
    c.emit(0x81c5c, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x81c5e, I::Stack(false, R1, 0x40))?;
    c.emit(0x81c60, I::Stack(true, R6, 0x30))?;
    c.emit(0x81c62, I::Small(false, R6, R6, 1))?;
    c.emit(0x81c64, I::Stack(true, R0, 0x20))?;
    c.emit(0x81c66, I::Stack(true, R2, 0x1c))?;
    c.emit(0x81c68, I::Alu(Alu::Cmp, R6, R2))?;
    c.emit(0x81c6a, I::Branch(Some(Condition::Le), 0x81c74))?;
    c.emit(0x81c6c, I::Stack(true, R1, 0x34))?;
    c.emit(0x81c6e, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x81c70, I::Stack(true, R0, 0x28))?;
    c.emit(0x81c72, I::Branch(None, 0x81d5c))?;
    c.emit(0x81c74, I::Compare(R6, 0))?;
    c.emit(0x81c76, I::Branch(Some(Condition::Le), 0x81c88))?;
    c.emit(0x81c78, I::Push(0x20))?;
    c.emit(0x81c7a, I::Pop(0x1))?;
    c.emit(0x81c7c, I::Branch(None, 0x81d12))?;
    c.emit(0x81c7e, I::Align)?;
    c.pool(0x81c80, 0x7fffffff)?;
    c.pool(0x81c84, 0x7ff00000)?;
    c.emit(0x81c88, I::Mov(R0, 30))?;
    c.emit(0x81c8a, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x81c8c, I::Arithmetic(true, R3, R0, R6))?;
    c.emit(0x81c8e, I::Mov(R0, 1))?;
    c.emit(0x81c90, I::Stack(false, R0, 0x2c))?;
    c.emit(0x81c92, I::Arithmetic(true, R4, R0, R6))?;
    c.emit(0x81c94, I::Push(0x20))?;
    c.emit(0x81c96, I::Pop(0x4))?;
    c.emit(0x81c98, I::Alu(Alu::Lsr, R2, R4))?;
    c.emit(0x81c9a, I::Mov(R7, 0))?;
    c.emit(0x81c9c, I::Stack(false, R3, 0x24))?;
    c.emit(0x81c9e, I::Compare(R3, 0))?;
    c.emit(0x81ca0, I::Push(0x80))?;
    c.emit(0x81ca2, I::Pop(0x1))?;
    c.emit(0x81ca4, I::Branch(Some(Condition::Ge), 0x81caa))?;
    c.emit(0x81ca6, I::Push(0x4))?;
    c.emit(0x81ca8, I::Pop(0x1))?;
    c.emit(0x81caa, I::Stack(false, R0, 0x44))?;
    c.emit(0x81cac, I::Mov(R2, 32))?;
    c.emit(0x81cae, I::Arithmetic(true, R6, R2, R4))?;
    c.emit(0x81cb0, I::Mov(R3, 64))?;
    c.emit(0x81cb2, I::Stack(false, R4, 0x20))?;
    c.emit(0x81cb4, I::Arithmetic(true, R3, R3, R4))?;
    c.emit(0x81cb6, I::Push(0x2))?;
    c.emit(0x81cb8, I::Pop(0x10))?;
    c.emit(0x81cba, I::Alu(Alu::Lsl, R4, R3))?;
    c.emit(0x81cbc, I::Compare(R6, 0))?;
    c.emit(0x81cbe, I::Stack(false, R7, 0x30))?;
    c.emit(0x81cc0, I::Branch(Some(Condition::Ge), 0x81cc6))?;
    c.emit(0x81cc2, I::Push(0x10))?;
    c.emit(0x81cc4, I::Pop(0x80))?;
    c.emit(0x81cc6, I::Push(0x20))?;
    c.emit(0x81cc8, I::Pop(0x10))?;
    c.emit(0x81cca, I::Alu(Alu::Lsl, R4, R3))?;
    c.emit(0x81ccc, I::Arithmetic(true, R3, R2, R3))?;
    c.emit(0x81cce, I::Push(0x2))?;
    c.emit(0x81cd0, I::Pop(0x4))?;
    c.emit(0x81cd2, I::Alu(Alu::Lsr, R2, R3))?;
    c.emit(0x81cd4, I::Alu(Alu::Orr, R2, R4))?;
    c.emit(0x81cd6, I::Push(0x2))?;
    c.emit(0x81cd8, I::Pop(0x8))?;
    c.emit(0x81cda, I::Alu(Alu::Lsl, R3, R6))?;
    c.emit(0x81cdc, I::Compare(R6, 0))?;
    c.emit(0x81cde, I::Branch(Some(Condition::Ge), 0x81ce4))?;
    c.emit(0x81ce0, I::Push(0x4))?;
    c.emit(0x81ce2, I::Pop(0x8))?;
    c.emit(0x81ce4, I::Alu(Alu::Orr, R7, R3))?;
    c.emit(0x81ce6, I::Compare(R7, 0))?;
    c.emit(0x81ce8, I::Stack(true, R3, 0x2c))?;
    c.emit(0x81cea, I::Stack(true, R2, 0x20))?;
    c.emit(0x81cec, I::Branch(Some(Condition::Ne), 0x81cf2))?;
    c.emit(0x81cee, I::Push(0x80))?;
    c.emit(0x81cf0, I::Pop(0x8))?;
    c.emit(0x81cf2, I::Alu(Alu::Lsr, R1, R2))?;
    c.emit(0x81cf4, I::Push(0x20))?;
    c.emit(0x81cf6, I::Pop(0x4))?;
    c.emit(0x81cf8, I::Alu(Alu::Lsl, R2, R6))?;
    c.emit(0x81cfa, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x81cfc, I::Stack(true, R1, 0x24))?;
    c.emit(0x81cfe, I::Alu(Alu::Lsr, R5, R1))?;
    c.emit(0x81d00, I::Compare(R1, 0))?;
    c.emit(0x81d02, I::Branch(Some(Condition::Ge), 0x81d08))?;
    c.emit(0x81d04, I::Push(0x4))?;
    c.emit(0x81d06, I::Pop(0x20))?;
    c.emit(0x81d08, I::Alu(Alu::Orr, R5, R3))?;
    c.emit(0x81d0a, I::Stack(false, R5, 0x40))?;
    c.emit(0x81d0c, I::Push(0x20))?;
    c.emit(0x81d0e, I::Pop(0x2))?;
    c.emit(0x81d10, I::Stack(true, R6, 0x30))?;
    c.emit(0x81d12, I::Mov(R2, 7))?;
    c.emit(0x81d14, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x81d16, I::Shift(Shift::Lsr, R4, R0, 3))?;
    c.emit(0x81d18, I::Stack(true, R3, 0xc))?;
    c.emit(0x81d1a, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x81d1c, I::Stack(true, R3, 0x34))?;
    c.emit(0x81d1e, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x81d20, I::Shift(Shift::Lsl, R3, R6, 20))?;
    c.emit(0x81d22, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x81d24, I::Shift(Shift::Lsr, R1, R1, 3))?;
    c.emit(0x81d26, I::Shift(Shift::Lsl, R0, R0, 29))?;
    c.emit(0x81d28, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x81d2a, I::Stack(true, R4, 0x10))?;
    c.emit(0x81d2c, I::Alu(Alu::And, R4, R0))?;
    c.emit(0x81d2e, I::Stack(true, R1, 0x28))?;
    c.emit(0x81d30, I::Alu(Alu::Orr, R4, R1))?;
    c.emit(0x81d32, I::Compare(R2, 5))?;
    c.emit(0x81d34, I::Branch(Some(Condition::Lo), 0x81d42))?;
    c.emit(0x81d36, I::Mov(R1, 0))?;
    c.emit(0x81d38, I::Small(false, R0, R4, 1))?;
    c.emit(0x81d3a, I::Alu(Alu::Adc, R3, R1))?;
    c.emit(0x81d3c, I::Push(0x8))?;
    c.emit(0x81d3e, I::Pop(0x2))?;
    c.emit(0x81d40, I::Branch(None, 0x81d5c))?;
    c.emit(0x81d42, I::Mov(R1, 1))?;
    c.emit(0x81d44, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x81d46, I::Mov(R1, 0))?;
    c.emit(0x81d48, I::Arithmetic(false, R0, R4, R0))?;
    c.emit(0x81d4a, I::Alu(Alu::Adc, R1, R3))?;
    c.emit(0x81d4c, I::Compare(R2, 4))?;
    c.emit(0x81d4e, I::Branch(Some(Condition::Eq), 0x81d54))?;
    c.emit(0x81d50, I::Push(0x8))?;
    c.emit(0x81d52, I::Pop(0x2))?;
    c.emit(0x81d54, I::Compare(R2, 4))?;
    c.emit(0x81d56, I::Branch(Some(Condition::Eq), 0x81d5c))?;
    c.emit(0x81d58, I::Push(0x10))?;
    c.emit(0x81d5a, I::Pop(0x1))?;
    c.emit(0x81d5c, I::Call(0x81d78))?;
    c.emit(0x81d60, I::Push(0x1))?;
    c.emit(0x81d62, I::Pop(0x40))?;
    c.emit(0x81d64, I::Push(0x40))?;
    c.emit(0x81d66, I::Pop(0x1))?;
    c.emit(0x81d68, I::StackAdjust(false, 0x4c))?;
    c.emit(0x81d6a, I::Pop(0x80f0))?;
    c.pool(0x81d6c, 0x000fffff)?;
    c.pool(0x81d70, 0x000007fe)?;
    c.pool(0x81d74, 0x7ff80000)?;
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
    fn extent_digest_pools_and_alignment() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1240);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "7d7681b1db69f810cb54ad1a7b58aa59901f5dfce8e001fe3564a3269d8b50de");
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        assert_eq!(half(&bytes, 0x81c7e), 0x46c0);
        for (at, slot) in [(0x818ac, 0x81c80), (0x818d4, 0x81c84),
            (0x819c6, 0x81d74), (0x81a50, 0x81d6c), (0x81a76, 0x81d70)] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn flag_setting_shifts_and_no_it_blocks() {
        let bytes = fragment().unwrap();
        for (at, expected) in [(0x818a0, 0xb5f0), (0x818a2, 0xb093),
            (0x818b2, 0x4014), (0x818ba, 0x1e40), (0x818c2, 0x414f),
            (0x818f4, 0x04c8), (0x81b0a, 0x40f3), (0x81baa, 0x4197),
            (0x81c42, 0x421a), (0x81c58, 0x07d1), (0x81d3a, 0x414b),
            (0x81d6a, 0xbdf0)] {
            assert_eq!(half(&bytes, at), expected, "{at:#x}");
        }
        for (start, end) in [(START, 0x81c7e), (0x81c88, 0x81d6c)] {
            let mut at = start;
            while at < end {
                let word = half(&bytes, at);
                assert_ne!(word & 0xff00, 0xbf00, "unexpected IT/hint at {at:#x}");
                at += if word & 0xf800 == 0xf000 { 4 } else { 2 };
            }
            assert_eq!(at, end);
        }
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
    }

    #[test]
    fn branch_and_call_targets() {
        let bytes = fragment().unwrap();
        for (at, target, conditional) in [(0x8195a, 0x81d5c, false),
            (0x81984, 0x8192c, true), (0x81c76, 0x81c88, true),
            (0x81c7c, 0x81d12, false)] {
            let word = half(&bytes, at);
            let shift = if conditional { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [(0x81a8e, 0x81d7a), (0x81aa6, 0x81d7a),
            (0x81bf2, 0x81dd0), (0x81d5c, 0x81d78)] {
            let first = u32::from(half(&bytes, at));
            let second = u32::from(half(&bytes, at + 2));
            let s = (first >> 10) & 1;
            let i1 = (!(second >> 13) ^ s) & 1;
            let i2 = (!(second >> 11) ^ s) & 1;
            let value = (s << 24) | (i1 << 23) | (i2 << 22)
                | ((first & 0x3ff) << 12) | ((second & 0x7ff) << 1);
            let delta = ((value << 7) as i32 >> 7) as isize;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
    }

    #[test]
    fn rejected_operands_and_layout_leave_buffer_unchanged() {
        for instruction in [I::Push(0), I::Push(0x8000), I::Pop(0x4000),
            I::StackAdjust(true, 510), I::Stack(true, R0, 2), I::Address(R1, 1024),
            I::Mov(R0, 256), I::Compare(R0, 256), I::Subtract(R0, 256),
            I::Small(true, R0, R1, 8), I::Shift(Shift::Lsl, R0, R1, 32),
            I::Shift(Shift::Lsr, R0, R1, 0), I::Literal(R0, START),
            I::Branch(Some(Condition::Eq), START + 260), I::Branch(None, START + 2052),
            I::Branch(None, START + 1), I::Call(START + 16_777_220)] {
            assert!(instruction.encode(START).is_err(), "{instruction:?}");
        }
        let mut c = Code::new();
        assert!(c.emit(START + 2, I::Mov(R0, 0)).is_err());
        assert!(c.emit(START, I::Align).is_err());
        assert!(c.pool(START, 0).is_err());
        assert_eq!((c.at, c.bytes.len(), c.code, c.data, c.align), (START, 0, 0, 0, 0));
        c.at = 0x81c80;
        assert!(c.emit(c.at, I::Mov(R0, 0)).is_err());
        assert!(c.pool(c.at, 0).is_err());
        assert!(c.bytes.is_empty());
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
