pub const START: usize = 0x81e64;
pub const END: usize = 0x82100;
pub const CODE_BYTES: usize = 620;
pub const DATA_BYTES: usize = 44;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 4;
const POOLS: [(usize, u32); 11] = [
    (0x82080, 0x7fffffff), (0x82084, 0x007fffff),
    (0x82088, 0x55555555), (0x8208c, 0x33333333),
    (0x82090, 0x0f0f0f0f), (0x82094, 0x01010101),
    (0x82098, 0x7fc00000), (0x820f0, 0x55555555),
    (0x820f4, 0x33333333), (0x820f8, 0x0f0f0f0f),
    (0x820fc, 0x01010101),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Tst, Cmp, Orr, Mul, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Hs = 2, Lo = 3, Hi = 8, Ls = 9, Ge = 10, Lt = 11, Le = 13 }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(bool, u16),
    Stack(bool, R, u16), Address(R, u16), Literal(R, usize),
    Alu(Alu, R, R), Shift(Shift, R, R, u8),
    Mov(R, u16), Compare(R, u16), Subtract(R, u16),
    Small(bool, R, R, u8), Arithmetic(bool, R, R, R),
    Branch(Option<Condition>, usize), Call(usize), BxLr, Memory(bool, R, R, u16), Align, ZeroAlign,
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
                    Alu::Tst => 8, Alu::Cmp => 10,
                    Alu::Orr => 12, Alu::Mul => 13, Alu::Mvn => 15,
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
            Self::BxLr => 0x4700 | (14 << 3),
            Self::Memory(load, r, base, offset) => {
                checked(offset <= 124 && offset % 4 == 0)?;
                (if load { 0x6800 } else { 0x6000 }) | ((offset / 4) << 6) | ((base as u16) << 3) | r as u16
            }
            Self::ZeroAlign => 0,
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
        checked(matches!(instruction, I::Align) == (at == 0x8207e))?;
        checked(matches!(instruction, I::ZeroAlign) == (at == 0x8209e))?;
        checked(![0x8207e, 0x8209e].iter().any(|&slot| at < slot && at + bytes.len() > slot))?;
        if matches!(instruction, I::Align | I::ZeroAlign) { self.align += bytes.len(); } else { self.code += bytes.len(); }
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
    c.emit(0x81e64, I::Push(0x40f0))?;
    c.emit(0x81e66, I::StackAdjust(true, 0x24))?;
    c.emit(0x81e68, I::Literal(R2, 0x82080))?;
    c.emit(0x81e6a, I::Push(0x2))?;
    c.emit(0x81e6c, I::Pop(0x8))?;
    c.emit(0x81e6e, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x81e70, I::Stack(false, R0, 0x18))?;
    c.emit(0x81e72, I::Push(0x1))?;
    c.emit(0x81e74, I::Pop(0x40))?;
    c.emit(0x81e76, I::Stack(false, R2, 0x10))?;
    c.emit(0x81e78, I::Alu(Alu::And, R6, R2))?;
    c.emit(0x81e7a, I::Small(true, R7, R6, 1))?;
    c.emit(0x81e7c, I::Mov(R0, 1))?;
    c.emit(0x81e7e, I::Stack(false, R0, 0x14))?;
    c.emit(0x81e80, I::Shift(Shift::Lsl, R2, R0, 22))?;
    c.emit(0x81e82, I::Mov(R0, 255))?;
    c.emit(0x81e84, I::Stack(false, R0, 0xc))?;
    c.emit(0x81e86, I::Shift(Shift::Lsl, R5, R0, 23))?;
    c.emit(0x81e88, I::Small(true, R4, R5, 2))?;
    c.emit(0x81e8a, I::Alu(Alu::Cmp, R7, R4))?;
    c.emit(0x81e8c, I::Branch(Some(Condition::Hi), 0x81ea0))?;
    c.emit(0x81e8e, I::Small(true, R7, R3, 1))?;
    c.emit(0x81e90, I::Alu(Alu::Cmp, R7, R4))?;
    c.emit(0x81e92, I::Branch(Some(Condition::Ls), 0x81eea))?;
    c.emit(0x81e94, I::Alu(Alu::Cmp, R3, R5))?;
    c.emit(0x81e96, I::Branch(Some(Condition::Ls), 0x81eaa))?;
    c.emit(0x81e98, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x81e9a, I::Push(0x2))?;
    c.emit(0x81e9c, I::Pop(0x1))?;
    c.emit(0x81e9e, I::Branch(None, 0x82076))?;
    c.emit(0x81ea0, I::Alu(Alu::Cmp, R6, R5))?;
    c.emit(0x81ea2, I::Branch(Some(Condition::Ls), 0x81e94))?;
    c.emit(0x81ea4, I::Stack(true, R0, 0x18))?;
    c.emit(0x81ea6, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x81ea8, I::Branch(None, 0x82076))?;
    c.emit(0x81eaa, I::Alu(Alu::Cmp, R6, R5))?;
    c.emit(0x81eac, I::Branch(Some(Condition::Ne), 0x81ec4))?;
    c.emit(0x81eae, I::Stack(true, R2, 0x18))?;
    c.emit(0x81eb0, I::Alu(Alu::Eor, R1, R2))?;
    c.emit(0x81eb2, I::Stack(true, R0, 0x10))?;
    c.emit(0x81eb4, I::Small(false, R0, R0, 1))?;
    c.emit(0x81eb6, I::Alu(Alu::Cmp, R1, R0))?;
    c.emit(0x81eb8, I::Push(0x4))?;
    c.emit(0x81eba, I::Pop(0x1))?;
    c.emit(0x81ebc, I::Branch(Some(Condition::Eq), 0x81ec0))?;
    c.emit(0x81ebe, I::Branch(None, 0x8207a))?;
    c.emit(0x81ec0, I::Literal(R0, 0x82098))?;
    c.emit(0x81ec2, I::Branch(None, 0x82076))?;
    c.emit(0x81ec4, I::Alu(Alu::Cmp, R3, R5))?;
    c.emit(0x81ec6, I::Branch(Some(Condition::Ne), 0x81ece))?;
    c.emit(0x81ec8, I::Push(0x2))?;
    c.emit(0x81eca, I::Pop(0x1))?;
    c.emit(0x81ecc, I::Branch(None, 0x8207a))?;
    c.emit(0x81ece, I::Compare(R6, 0))?;
    c.emit(0x81ed0, I::Branch(Some(Condition::Ne), 0x81ee2))?;
    c.emit(0x81ed2, I::Compare(R3, 0))?;
    c.emit(0x81ed4, I::Push(0x2))?;
    c.emit(0x81ed6, I::Pop(0x1))?;
    c.emit(0x81ed8, I::Branch(Some(Condition::Eq), 0x81edc))?;
    c.emit(0x81eda, I::Branch(None, 0x8207a))?;
    c.emit(0x81edc, I::Stack(true, R0, 0x18))?;
    c.emit(0x81ede, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x81ee0, I::Branch(None, 0x81e9a))?;
    c.emit(0x81ee2, I::Compare(R3, 0))?;
    c.emit(0x81ee4, I::Stack(true, R0, 0x18))?;
    c.emit(0x81ee6, I::Branch(Some(Condition::Ne), 0x81eea))?;
    c.emit(0x81ee8, I::Branch(None, 0x8207a))?;
    c.emit(0x81eea, I::Stack(false, R5, 0x8))?;
    c.emit(0x81eec, I::Alu(Alu::Cmp, R3, R6))?;
    c.emit(0x81eee, I::Push(0x2))?;
    c.emit(0x81ef0, I::Pop(0x10))?;
    c.emit(0x81ef2, I::Branch(Some(Condition::Hi), 0x81ef6))?;
    c.emit(0x81ef4, I::Stack(true, R4, 0x18))?;
    c.emit(0x81ef6, I::Literal(R2, 0x82084))?;
    c.emit(0x81ef8, I::Push(0x10))?;
    c.emit(0x81efa, I::Pop(0x1))?;
    c.emit(0x81efc, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x81efe, I::Stack(false, R0, 0x20))?;
    c.emit(0x81f00, I::Alu(Alu::Cmp, R3, R6))?;
    c.emit(0x81f02, I::Branch(Some(Condition::Hi), 0x81f06))?;
    c.emit(0x81f04, I::Stack(false, R1, 0x18))?;
    c.emit(0x81f06, I::Stack(true, R1, 0x18))?;
    c.emit(0x81f08, I::Push(0x2))?;
    c.emit(0x81f0a, I::Pop(0x1))?;
    c.emit(0x81f0c, I::Stack(false, R2, 0x4))?;
    c.emit(0x81f0e, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x81f10, I::Stack(false, R0, 0x1c))?;
    c.emit(0x81f12, I::Shift(Shift::Lsr, R6, R1, 23))?;
    c.emit(0x81f14, I::Stack(true, R0, 0xc))?;
    c.emit(0x81f16, I::Alu(Alu::And, R6, R0))?;
    c.emit(0x81f18, I::Push(0x10))?;
    c.emit(0x81f1a, I::Pop(0x80))?;
    c.emit(0x81f1c, I::Shift(Shift::Lsr, R4, R4, 23))?;
    c.emit(0x81f1e, I::Alu(Alu::And, R4, R0))?;
    c.emit(0x81f20, I::Compare(R4, 0))?;
    c.emit(0x81f22, I::Branch(Some(Condition::Ne), 0x81f2e))?;
    c.emit(0x81f24, I::Address(R0, 0x20))?;
    c.emit(0x81f26, I::Call(0x820a0))?;
    c.emit(0x81f2a, I::Push(0x1))?;
    c.emit(0x81f2c, I::Pop(0x10))?;
    c.emit(0x81f2e, I::Stack(true, R0, 0x14))?;
    c.emit(0x81f30, I::Shift(Shift::Lsl, R5, R0, 26))?;
    c.emit(0x81f32, I::Compare(R6, 0))?;
    c.emit(0x81f34, I::Stack(false, R5, 0xc))?;
    c.emit(0x81f36, I::Branch(Some(Condition::Ne), 0x81f44))?;
    c.emit(0x81f38, I::Address(R0, 0x1c))?;
    c.emit(0x81f3a, I::Call(0x820a0))?;
    c.emit(0x81f3e, I::Stack(true, R5, 0xc))?;
    c.emit(0x81f40, I::Push(0x1))?;
    c.emit(0x81f42, I::Pop(0x40))?;
    c.emit(0x81f44, I::Stack(true, R0, 0x20))?;
    c.emit(0x81f46, I::Shift(Shift::Lsl, R0, R0, 3))?;
    c.emit(0x81f48, I::Push(0x1))?;
    c.emit(0x81f4a, I::Pop(0x4))?;
    c.emit(0x81f4c, I::Alu(Alu::Orr, R2, R5))?;
    c.emit(0x81f4e, I::Stack(false, R2, 0x20))?;
    c.emit(0x81f50, I::Stack(true, R1, 0x1c))?;
    c.emit(0x81f52, I::Shift(Shift::Lsl, R3, R1, 3))?;
    c.emit(0x81f54, I::Alu(Alu::Orr, R3, R5))?;
    c.emit(0x81f56, I::Stack(false, R3, 0x1c))?;
    c.emit(0x81f58, I::Arithmetic(true, R6, R4, R6))?;
    c.emit(0x81f5a, I::Stack(true, R1, 0x18))?;
    c.emit(0x81f5c, I::Alu(Alu::Eor, R1, R7))?;
    c.emit(0x81f5e, I::Stack(false, R1, 0x18))?;
    c.emit(0x81f60, I::Stack(true, R1, 0x10))?;
    c.emit(0x81f62, I::Small(false, R1, R1, 1))?;
    c.emit(0x81f64, I::Alu(Alu::And, R1, R7))?;
    c.emit(0x81f66, I::Compare(R6, 0))?;
    c.emit(0x81f68, I::Stack(false, R4, 0x10))?;
    c.emit(0x81f6a, I::Branch(Some(Condition::Ne), 0x81f72))?;
    c.emit(0x81f6c, I::Push(0x8))?;
    c.emit(0x81f6e, I::Pop(0x40))?;
    c.emit(0x81f70, I::Branch(None, 0x81f9e))?;
    c.emit(0x81f72, I::Compare(R6, 31))?;
    c.emit(0x81f74, I::Branch(Some(Condition::Hi), 0x81f9a))?;
    c.emit(0x81f76, I::Push(0x8))?;
    c.emit(0x81f78, I::Pop(0x10))?;
    c.emit(0x81f7a, I::Alu(Alu::Lsr, R4, R6))?;
    c.emit(0x81f7c, I::Stack(false, R4, 0x14))?;
    c.emit(0x81f7e, I::Mov(R4, 32))?;
    c.emit(0x81f80, I::Arithmetic(true, R4, R4, R6))?;
    c.emit(0x81f82, I::Mov(R6, 1))?;
    c.emit(0x81f84, I::Alu(Alu::Lsl, R3, R4))?;
    c.emit(0x81f86, I::Branch(Some(Condition::Ne), 0x81f8c))?;
    c.emit(0x81f88, I::Push(0x8))?;
    c.emit(0x81f8a, I::Pop(0x40))?;
    c.emit(0x81f8c, I::Stack(true, R3, 0x14))?;
    c.emit(0x81f8e, I::Alu(Alu::Orr, R3, R6))?;
    c.emit(0x81f90, I::Push(0x8))?;
    c.emit(0x81f92, I::Pop(0x40))?;
    c.emit(0x81f94, I::Stack(false, R6, 0x1c))?;
    c.emit(0x81f96, I::Stack(true, R4, 0x10))?;
    c.emit(0x81f98, I::Branch(None, 0x81f9e))?;
    c.emit(0x81f9a, I::Stack(true, R6, 0x14))?;
    c.emit(0x81f9c, I::Stack(false, R6, 0x1c))?;
    c.emit(0x81f9e, I::Stack(true, R3, 0x18))?;
    c.emit(0x81fa0, I::Compare(R3, 0))?;
    c.emit(0x81fa2, I::Branch(Some(Condition::Ge), 0x81ffc))?;
    c.emit(0x81fa4, I::Arithmetic(true, R2, R2, R6))?;
    c.emit(0x81fa6, I::Stack(false, R2, 0x20))?;
    c.emit(0x81fa8, I::Mov(R0, 0))?;
    c.emit(0x81faa, I::Compare(R2, 0))?;
    c.emit(0x81fac, I::Branch(Some(Condition::Eq), 0x82076))?;
    c.emit(0x81fae, I::Stack(true, R0, 0xc))?;
    c.emit(0x81fb0, I::Alu(Alu::Cmp, R2, R0))?;
    c.emit(0x81fb2, I::Branch(Some(Condition::Hs), 0x8201a))?;
    c.emit(0x81fb4, I::Mov(R0, 32))?;
    c.emit(0x81fb6, I::Compare(R2, 0))?;
    c.emit(0x81fb8, I::Branch(Some(Condition::Eq), 0x81ff2))?;
    c.emit(0x81fba, I::Shift(Shift::Lsr, R0, R2, 1))?;
    c.emit(0x81fbc, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x81fbe, I::Shift(Shift::Lsr, R3, R0, 2))?;
    c.emit(0x81fc0, I::Alu(Alu::Orr, R3, R0))?;
    c.emit(0x81fc2, I::Shift(Shift::Lsr, R0, R3, 4))?;
    c.emit(0x81fc4, I::Alu(Alu::Orr, R0, R3))?;
    c.emit(0x81fc6, I::Shift(Shift::Lsr, R3, R0, 8))?;
    c.emit(0x81fc8, I::Alu(Alu::Orr, R3, R0))?;
    c.emit(0x81fca, I::Shift(Shift::Lsr, R0, R3, 16))?;
    c.emit(0x81fcc, I::Alu(Alu::Orr, R0, R3))?;
    c.emit(0x81fce, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x81fd0, I::Shift(Shift::Lsr, R3, R0, 1))?;
    c.emit(0x81fd2, I::Literal(R4, 0x82088))?;
    c.emit(0x81fd4, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x81fd6, I::Arithmetic(true, R0, R0, R4))?;
    c.emit(0x81fd8, I::Literal(R3, 0x8208c))?;
    c.emit(0x81fda, I::Shift(Shift::Lsr, R4, R0, 2))?;
    c.emit(0x81fdc, I::Alu(Alu::And, R0, R3))?;
    c.emit(0x81fde, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x81fe0, I::Arithmetic(false, R0, R0, R4))?;
    c.emit(0x81fe2, I::Stack(true, R4, 0x10))?;
    c.emit(0x81fe4, I::Shift(Shift::Lsr, R3, R0, 4))?;
    c.emit(0x81fe6, I::Arithmetic(false, R0, R0, R3))?;
    c.emit(0x81fe8, I::Literal(R3, 0x82090))?;
    c.emit(0x81fea, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x81fec, I::Literal(R0, 0x82094))?;
    c.emit(0x81fee, I::Alu(Alu::Mul, R0, R3))?;
    c.emit(0x81ff0, I::Shift(Shift::Lsr, R0, R0, 24))?;
    c.emit(0x81ff2, I::Small(true, R0, R0, 5))?;
    c.emit(0x81ff4, I::Alu(Alu::Lsl, R2, R0))?;
    c.emit(0x81ff6, I::Stack(false, R2, 0x20))?;
    c.emit(0x81ff8, I::Arithmetic(true, R4, R4, R0))?;
    c.emit(0x81ffa, I::Branch(None, 0x8201a))?;
    c.emit(0x81ffc, I::Arithmetic(false, R2, R6, R2))?;
    c.emit(0x81ffe, I::Stack(false, R2, 0x20))?;
    c.emit(0x82000, I::Mov(R3, 1))?;
    c.emit(0x82002, I::Shift(Shift::Lsl, R4, R3, 27))?;
    c.emit(0x82004, I::Alu(Alu::Tst, R2, R4))?;
    c.emit(0x82006, I::Branch(Some(Condition::Ne), 0x8200c))?;
    c.emit(0x82008, I::Stack(true, R4, 0x10))?;
    c.emit(0x8200a, I::Branch(None, 0x8201a))?;
    c.emit(0x8200c, I::Arithmetic(false, R0, R6, R0))?;
    c.emit(0x8200e, I::Alu(Alu::And, R0, R3))?;
    c.emit(0x82010, I::Shift(Shift::Lsr, R2, R2, 1))?;
    c.emit(0x82012, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x82014, I::Stack(false, R2, 0x20))?;
    c.emit(0x82016, I::Stack(true, R4, 0x10))?;
    c.emit(0x82018, I::Small(false, R4, R4, 1))?;
    c.emit(0x8201a, I::Compare(R4, 255))?;
    c.emit(0x8201c, I::Branch(Some(Condition::Lt), 0x82024))?;
    c.emit(0x8201e, I::Stack(true, R0, 0x8))?;
    c.emit(0x82020, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x82022, I::Branch(None, 0x81e9a))?;
    c.emit(0x82024, I::Compare(R4, 0))?;
    c.emit(0x82026, I::Branch(Some(Condition::Le), 0x82030))?;
    c.emit(0x82028, I::Push(0x4))?;
    c.emit(0x8202a, I::Pop(0x1))?;
    c.emit(0x8202c, I::Stack(true, R5, 0x4))?;
    c.emit(0x8202e, I::Branch(None, 0x82052))?;
    c.emit(0x82030, I::Stack(true, R6, 0x4))?;
    c.emit(0x82032, I::Mov(R3, 1))?;
    c.emit(0x82034, I::Arithmetic(true, R4, R3, R4))?;
    c.emit(0x82036, I::Push(0x4))?;
    c.emit(0x82038, I::Pop(0x1))?;
    c.emit(0x8203a, I::Alu(Alu::Lsr, R0, R4))?;
    c.emit(0x8203c, I::Mov(R5, 32))?;
    c.emit(0x8203e, I::Arithmetic(true, R4, R5, R4))?;
    c.emit(0x82040, I::Alu(Alu::Lsl, R2, R4))?;
    c.emit(0x82042, I::Branch(Some(Condition::Ne), 0x82048))?;
    c.emit(0x82044, I::Push(0x4))?;
    c.emit(0x82046, I::Pop(0x8))?;
    c.emit(0x82048, I::Alu(Alu::Orr, R0, R3))?;
    c.emit(0x8204a, I::Stack(false, R0, 0x20))?;
    c.emit(0x8204c, I::Mov(R4, 0))?;
    c.emit(0x8204e, I::Push(0x40))?;
    c.emit(0x82050, I::Pop(0x20))?;
    c.emit(0x82052, I::Shift(Shift::Lsr, R3, R0, 3))?;
    c.emit(0x82054, I::Alu(Alu::And, R5, R3))?;
    c.emit(0x82056, I::Shift(Shift::Lsl, R2, R4, 23))?;
    c.emit(0x82058, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x8205a, I::Alu(Alu::Orr, R2, R5))?;
    c.emit(0x8205c, I::Mov(R1, 7))?;
    c.emit(0x8205e, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x82060, I::Compare(R1, 5))?;
    c.emit(0x82062, I::Branch(Some(Condition::Lo), 0x82068))?;
    c.emit(0x82064, I::Small(false, R0, R2, 1))?;
    c.emit(0x82066, I::Branch(None, 0x82076))?;
    c.emit(0x82068, I::Mov(R0, 1))?;
    c.emit(0x8206a, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x8206c, I::Arithmetic(false, R0, R2, R3))?;
    c.emit(0x8206e, I::Compare(R1, 4))?;
    c.emit(0x82070, I::Branch(Some(Condition::Eq), 0x82076))?;
    c.emit(0x82072, I::Push(0x4))?;
    c.emit(0x82074, I::Pop(0x1))?;
    c.emit(0x82076, I::Call(0x8209c))?;
    c.emit(0x8207a, I::StackAdjust(false, 0x24))?;
    c.emit(0x8207c, I::Pop(0x80f0))?;
    c.emit(0x8207e, I::Align)?;
    c.pool(0x82080, 0x7fffffff)?;
    c.pool(0x82084, 0x007fffff)?;
    c.pool(0x82088, 0x55555555)?;
    c.pool(0x8208c, 0x33333333)?;
    c.pool(0x82090, 0x0f0f0f0f)?;
    c.pool(0x82094, 0x01010101)?;
    c.pool(0x82098, 0x7fc00000)?;
    c.emit(0x8209c, I::BxLr)?;
    c.emit(0x8209e, I::ZeroAlign)?;
    c.emit(0x820a0, I::Push(0x4010))?;
    c.emit(0x820a2, I::Memory(true, R1, R0, 0))?;
    c.emit(0x820a4, I::Mov(R2, 32))?;
    c.emit(0x820a6, I::Compare(R1, 0))?;
    c.emit(0x820a8, I::Branch(Some(Condition::Eq), 0x820e0))?;
    c.emit(0x820aa, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x820ac, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x820ae, I::Shift(Shift::Lsr, R3, R2, 2))?;
    c.emit(0x820b0, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x820b2, I::Shift(Shift::Lsr, R2, R3, 4))?;
    c.emit(0x820b4, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x820b6, I::Shift(Shift::Lsr, R3, R2, 8))?;
    c.emit(0x820b8, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x820ba, I::Shift(Shift::Lsr, R2, R3, 16))?;
    c.emit(0x820bc, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x820be, I::Alu(Alu::Mvn, R2, R2))?;
    c.emit(0x820c0, I::Shift(Shift::Lsr, R3, R2, 1))?;
    c.emit(0x820c2, I::Literal(R4, 0x820f0))?;
    c.emit(0x820c4, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x820c6, I::Arithmetic(true, R2, R2, R4))?;
    c.emit(0x820c8, I::Literal(R3, 0x820f4))?;
    c.emit(0x820ca, I::Shift(Shift::Lsr, R4, R2, 2))?;
    c.emit(0x820cc, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x820ce, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x820d0, I::Arithmetic(false, R2, R2, R4))?;
    c.emit(0x820d2, I::Shift(Shift::Lsr, R3, R2, 4))?;
    c.emit(0x820d4, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x820d6, I::Literal(R3, 0x820f8))?;
    c.emit(0x820d8, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x820da, I::Literal(R2, 0x820fc))?;
    c.emit(0x820dc, I::Alu(Alu::Mul, R2, R3))?;
    c.emit(0x820de, I::Shift(Shift::Lsr, R2, R2, 24))?;
    c.emit(0x820e0, I::Push(0x4))?;
    c.emit(0x820e2, I::Pop(0x8))?;
    c.emit(0x820e4, I::Subtract(R3, 8))?;
    c.emit(0x820e6, I::Alu(Alu::Lsl, R1, R3))?;
    c.emit(0x820e8, I::Memory(false, R1, R0, 0))?;
    c.emit(0x820ea, I::Mov(R0, 9))?;
    c.emit(0x820ec, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x820ee, I::Pop(0x8010))?;
    c.pool(0x820f0, 0x55555555)?;
    c.pool(0x820f4, 0x33333333)?;
    c.pool(0x820f8, 0x0f0f0f0f)?;
    c.pool(0x820fc, 0x01010101)?;
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
    fn extent_digest_pools_alignment_and_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 668);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "beb256522e8b4b5e23d781fd6fff1aada8756d3ba145caed3a9409488f7133a0");
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        assert_eq!(half(&bytes, 0x8207e), 0x46c0);
        assert_eq!(half(&bytes, 0x8209e), 0x0000);
        for (at, slot) in [(0x81e68, 0x82080),
            (0x81ec0, 0x82098),
            (0x81ef6, 0x82084),
            (0x81fd2, 0x82088),
            (0x81fd8, 0x8208c),
            (0x81fe8, 0x82090),
            (0x81fec, 0x82094),
            (0x820c2, 0x820f0),
            (0x820c8, 0x820f4),
            (0x820d6, 0x820f8),
            (0x820da, 0x820fc)] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn independently_decoded_control_flow() {
        let bytes = fragment().unwrap();
        for (at, target, conditional) in [(0x81e8c, 0x81ea0, true),
            (0x81e92, 0x81eea, true),
            (0x81e96, 0x81eaa, true),
            (0x81e9e, 0x82076, false),
            (0x81ea2, 0x81e94, true),
            (0x81ea8, 0x82076, false),
            (0x81eac, 0x81ec4, true),
            (0x81ebc, 0x81ec0, true),
            (0x81ebe, 0x8207a, false),
            (0x81ec2, 0x82076, false),
            (0x81ec6, 0x81ece, true),
            (0x81ecc, 0x8207a, false),
            (0x81ed0, 0x81ee2, true),
            (0x81ed8, 0x81edc, true),
            (0x81eda, 0x8207a, false),
            (0x81ee0, 0x81e9a, false),
            (0x81ee6, 0x81eea, true),
            (0x81ee8, 0x8207a, false),
            (0x81ef2, 0x81ef6, true),
            (0x81f02, 0x81f06, true),
            (0x81f22, 0x81f2e, true),
            (0x81f36, 0x81f44, true),
            (0x81f6a, 0x81f72, true),
            (0x81f70, 0x81f9e, false),
            (0x81f74, 0x81f9a, true),
            (0x81f86, 0x81f8c, true),
            (0x81f98, 0x81f9e, false),
            (0x81fa2, 0x81ffc, true),
            (0x81fac, 0x82076, true),
            (0x81fb2, 0x8201a, true),
            (0x81fb8, 0x81ff2, true),
            (0x81ffa, 0x8201a, false),
            (0x82006, 0x8200c, true),
            (0x8200a, 0x8201a, false),
            (0x8201c, 0x82024, true),
            (0x82022, 0x81e9a, false),
            (0x82026, 0x82030, true),
            (0x8202e, 0x82052, false),
            (0x82042, 0x82048, true),
            (0x82062, 0x82068, true),
            (0x82066, 0x82076, false),
            (0x82070, 0x82076, true),
            (0x820a8, 0x820e0, true)] {
            let word = half(&bytes, at);
            assert_eq!(word & (if conditional { 0xf000 } else { 0xf800 }),
                if conditional { 0xd000 } else { 0xe000 });
            let shift = if conditional { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [(0x81f26, 0x820a0), (0x81f3a, 0x820a0), (0x82076, 0x8209c)] {
            let first = u32::from(half(&bytes, at));
            let second = u32::from(half(&bytes, at + 2));
            assert_eq!(first & 0xf800, 0xf000);
            assert_eq!(second & 0xd000, 0xd000);
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
    fn fixed_words_flags_and_shift_edges() {
        let bytes = fragment().unwrap();
        for (at, word) in [(0x81e64, 0xb5f0), (0x81e66, 0xb089),
            (0x81e6e, 0x4013), (0x81e7a, 0x1e77), (0x81e80, 0x0582),
            (0x81eb4, 0x1c40), (0x81f7a, 0x40f4), (0x81f84, 0x40a3),
            (0x81fee, 0x4358), (0x82004, 0x4222), (0x8203a, 0x40e0),
            (0x82040, 0x40a2), (0x8207c, 0xbdf0), (0x8209c, 0x4770),
            (0x820a2, 0x6801), (0x820dc, 0x435a), (0x820e6, 0x4099),
            (0x820e8, 0x6001), (0x820ee, 0xbd10)] {
            assert_eq!(half(&bytes, at), word, "{at:#x}");
        }
        assert_eq!(I::Shift(Shift::Lsl, R0, R1, 0).encode(START).unwrap(), 0x0008u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
        for (start, end) in [(START, 0x8207e), (0x8209c, 0x8209e), (0x820a0, 0x820f0)] {
            let mut at = start;
            while at < end {
                let word = half(&bytes, at);
                assert_ne!(word & 0xff00, 0xbf00);
                at += if word & 0xf800 == 0xf000 { 4 } else { 2 };
            }
            assert_eq!(at, end);
        }
    }

    #[test]
    fn operand_and_layout_guards_are_atomic() {
        for instruction in [I::Push(0), I::Push(0x8000), I::Pop(0x4000),
            I::StackAdjust(true, 510), I::Stack(true, R0, 2), I::Address(R1, 1024),
            I::Memory(true, R0, R1, 128), I::Memory(false, R0, R1, 2),
            I::Mov(R0, 256), I::Compare(R0, 256), I::Subtract(R0, 256),
            I::Small(true, R0, R1, 8), I::Shift(Shift::Lsl, R0, R1, 32),
            I::Shift(Shift::Lsr, R0, R1, 0), I::Shift(Shift::Lsr, R0, R1, 33),
            I::Literal(R0, START), I::Literal(R0, START + 2048),
            I::Branch(Some(Condition::Eq), START + 260), I::Branch(None, START + 2052),
            I::Branch(None, START + 1), I::Call(START + 16_777_220)] {
            assert!(instruction.encode(START).is_err(), "{instruction:?}");
        }
        let mut c = Code::new();
        assert!(c.emit(START + 2, I::Mov(R0, 0)).is_err());
        assert!(c.emit(START, I::Align).is_err());
        assert!(c.emit(START, I::ZeroAlign).is_err());
        assert!(c.pool(START, 0).is_err());
        for at in [0x8207c, 0x8207e, 0x82080, 0x8209a, 0x8209e, 0x820f0, END] {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            let instruction = if at == 0x8207c || at == 0x8209a {
                I::Call(0x820a0)
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
            "beb256522e8b4b5e23d781fd6fff1aada8756d3ba145caed3a9409488f7133a0");
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
