pub const START: usize = 0x82be4;
pub const END: usize = 0x83018;
pub const CODE_BYTES: usize = 962;
pub const DATA_BYTES: usize = 100;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 14;
const ALIGNMENTS: [usize; 6] = [0x82caa, 0x82d26, 0x82d8a, 0x82dde, 0x82e86, 0x82ec2];
const ZERO_ALIGNMENT: usize = 0x82c5a;
const POOLS: [(usize, u32); 25] = [
    (0x82cac, 0x007fffff),
    (0x82cb0, 0x7fffffff),
    (0x82d28, 0x3df00000),
    (0x82d2c, 0xc1f00000),
    (0x82d8c, 0x000fffff),
    (0x82d90, 0x000007ff),
    (0x82d94, 0xfffffc01),
    (0x82de0, 0x007fffff),
    (0x82e88, 0x55555555),
    (0x82e8c, 0x33333333),
    (0x82e90, 0x0f0f0f0f),
    (0x82e94, 0x01010101),
    (0x82e98, 0x0000041e),
    (0x82ec4, 0x45300000),
    (0x82ec8, 0xc5300000),
    (0x82ecc, 0x43300000),
    (0x82f60, 0x55555555),
    (0x82f64, 0x33333333),
    (0x82f68, 0x0f0f0f0f),
    (0x82f6c, 0x01010101),
    (0x82f70, 0x0000041e),
    (0x83008, 0x55555555),
    (0x8300c, 0x33333333),
    (0x83010, 0x0f0f0f0f),
    (0x83014, 0x01010101),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Adc, Sbc, Cmp, Orr, Mul, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr, Asr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Lo = 3, Hi = 8, Ls = 9, Ge = 10, Lt = 11, Gt = 12 }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(bool, u16),
    Stack(bool, R, u16), Address(R, u16), Literal(R, usize),
    Alu(Alu, R, R), Shift(Shift, R, R, u8),
    Add(R, u16), Mov(R, u16), Compare(R, u16), Subtract(R, u16),
    Small(bool, R, R, u8), Arithmetic(bool, R, R, R),
    Branch(Option<Condition>, usize), Call(usize), CallArm(usize), BxLr, LoadMultiple(R, u16), Memory(bool, R, R, u16), Align, ZeroAlign,
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
        checked(at <= u32::MAX as usize - 4 && at % 2 == 0)?;
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
                    Alu::Adc => 5, Alu::Sbc => 6, Alu::Cmp => 10,
                    Alu::Orr => 12, Alu::Mul => 13, Alu::Mvn => 15,
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
            Self::Add(r, value) | Self::Mov(r, value) | Self::Compare(r, value) | Self::Subtract(r, value) => {
                checked(value <= 255)?;
                let base = match self { Self::Add(..) => 0x3000, Self::Mov(..) => 0x2000, Self::Compare(..) => 0x2800, _ => 0x3800 };
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
            Self::Call(target) | Self::CallArm(target) => {
                let arm = matches!(self, Self::CallArm(_));
                checked(target <= u32::MAX as usize && target % 2 == 0 && (!arm || target % 4 == 0))?;
                let base = if arm { (at + 4) & !3 } else { at + 4 };
                let delta = target as i64 - base as i64;
                checked((-16_777_216..=16_777_214).contains(&delta))?;
                let delta = delta as u32;
                let s = (delta >> 24) & 1;
                let j1 = (!(delta >> 23) ^ s) & 1;
                let j2 = (!(delta >> 22) ^ s) & 1;
                let first = 0xf000 | (s << 10) | ((delta >> 12) & 0x3ff);
                let second = (if arm { 0xc000 } else { 0xd000 }) | (j1 << 13) | (j2 << 11) | ((delta >> 1) & 0x7ff);
                return Ok([ (first as u16).to_le_bytes(), (second as u16).to_le_bytes() ].concat());
            }
            Self::LoadMultiple(base, mask) => {
                checked(mask != 0 && mask <= 255 && mask & (1 << base as u16) == 0)?;
                0xc800 | ((base as u16) << 8) | mask
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
        checked(matches!(instruction, I::Align) == ALIGNMENTS.contains(&at))?;
        checked(matches!(instruction, I::ZeroAlign) == (at == ZERO_ALIGNMENT))?;
        checked(!ALIGNMENTS.iter().chain(std::iter::once(&ZERO_ALIGNMENT)).any(|&slot| at < slot && at + bytes.len() > slot))?;
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
    c.emit(0x82be4, I::Push(0x40b0))?;
    c.emit(0x82be6, I::Push(0x2))?;
    c.emit(0x82be8, I::Pop(0x10))?;
    c.emit(0x82bea, I::Push(0x1))?;
    c.emit(0x82bec, I::Pop(0x20))?;
    c.emit(0x82bee, I::Shift(Shift::Asr, R0, R5, 31))?;
    c.emit(0x82bf0, I::Push(0x1))?;
    c.emit(0x82bf2, I::Pop(0x2))?;
    c.emit(0x82bf4, I::Alu(Alu::Eor, R1, R5))?;
    c.emit(0x82bf6, I::Arithmetic(true, R0, R1, R0))?;
    c.emit(0x82bf8, I::Shift(Shift::Asr, R1, R4, 31))?;
    c.emit(0x82bfa, I::Push(0x2))?;
    c.emit(0x82bfc, I::Pop(0x4))?;
    c.emit(0x82bfe, I::Alu(Alu::Eor, R2, R4))?;
    c.emit(0x82c00, I::Arithmetic(true, R1, R2, R1))?;
    c.emit(0x82c02, I::Call(0x83c4c))?;
    c.emit(0x82c06, I::Alu(Alu::Eor, R4, R5))?;
    c.emit(0x82c08, I::Shift(Shift::Asr, R1, R4, 31))?;
    c.emit(0x82c0a, I::Alu(Alu::Eor, R0, R1))?;
    c.emit(0x82c0c, I::Arithmetic(true, R0, R0, R1))?;
    c.emit(0x82c0e, I::Pop(0x80b0))?;
    c.emit(0x82c10, I::Push(0x4070))?;
    c.emit(0x82c12, I::Push(0x2))?;
    c.emit(0x82c14, I::Pop(0x20))?;
    c.emit(0x82c16, I::Push(0x1))?;
    c.emit(0x82c18, I::Pop(0x40))?;
    c.emit(0x82c1a, I::Mov(R4, 0))?;
    c.emit(0x82c1c, I::Push(0x10))?;
    c.emit(0x82c1e, I::Pop(0x4))?;
    c.emit(0x82c20, I::Push(0x10))?;
    c.emit(0x82c22, I::Pop(0x8))?;
    c.emit(0x82c24, I::CallArm(0x81718))?;
    c.emit(0x82c28, I::Compare(R0, 0))?;
    c.emit(0x82c2a, I::Branch(Some(Condition::Eq), 0x82c44))?;
    c.emit(0x82c2c, I::Mov(R0, 1))?;
    c.emit(0x82c2e, I::Shift(Shift::Lsl, R0, R0, 31))?;
    c.emit(0x82c30, I::Alu(Alu::Eor, R5, R0))?;
    c.emit(0x82c32, I::Push(0x40))?;
    c.emit(0x82c34, I::Pop(0x1))?;
    c.emit(0x82c36, I::Push(0x20))?;
    c.emit(0x82c38, I::Pop(0x2))?;
    c.emit(0x82c3a, I::Call(0x82cb4))?;
    c.emit(0x82c3e, I::Arithmetic(true, R0, R4, R0))?;
    c.emit(0x82c40, I::Alu(Alu::Sbc, R4, R1))?;
    c.emit(0x82c42, I::Branch(None, 0x82c54))?;
    c.emit(0x82c44, I::Push(0x40))?;
    c.emit(0x82c46, I::Pop(0x1))?;
    c.emit(0x82c48, I::Push(0x20))?;
    c.emit(0x82c4a, I::Pop(0x2))?;
    c.emit(0x82c4c, I::Call(0x82cb4))?;
    c.emit(0x82c50, I::Push(0x2))?;
    c.emit(0x82c52, I::Pop(0x10))?;
    c.emit(0x82c54, I::Push(0x10))?;
    c.emit(0x82c56, I::Pop(0x2))?;
    c.emit(0x82c58, I::Pop(0x8070))?;
    c.emit(0x82c5a, I::ZeroAlign)?;
    c.emit(0x82c5c, I::Literal(R1, 0x82cac))?;
    c.emit(0x82c5e, I::Push(0x1))?;
    c.emit(0x82c60, I::Pop(0x4))?;
    c.emit(0x82c62, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x82c64, I::Small(false, R1, R1, 1))?;
    c.emit(0x82c66, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x82c68, I::Shift(Shift::Lsr, R2, R0, 23))?;
    c.emit(0x82c6a, I::Mov(R3, 255))?;
    c.emit(0x82c6c, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x82c6e, I::Shift(Shift::Asr, R0, R0, 31))?;
    c.emit(0x82c70, I::Mov(R2, 1))?;
    c.emit(0x82c72, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x82c74, I::Mov(R0, 0))?;
    c.emit(0x82c76, I::Compare(R3, 127))?;
    c.emit(0x82c78, I::Branch(Some(Condition::Lo), 0x82ca8))?;
    c.emit(0x82c7a, I::Push(0x8))?;
    c.emit(0x82c7c, I::Pop(0x1))?;
    c.emit(0x82c7e, I::Subtract(R0, 127))?;
    c.emit(0x82c80, I::Compare(R0, 32))?;
    c.emit(0x82c82, I::Branch(Some(Condition::Lo), 0x82c92))?;
    c.emit(0x82c84, I::Literal(R0, 0x82cb0))?;
    c.emit(0x82c86, I::Small(false, R1, R0, 1))?;
    c.emit(0x82c88, I::Compare(R2, 1))?;
    c.emit(0x82c8a, I::Branch(Some(Condition::Eq), 0x82ca8))?;
    c.emit(0x82c8c, I::Push(0x2))?;
    c.emit(0x82c8e, I::Pop(0x1))?;
    c.emit(0x82c90, I::Branch(None, 0x82ca8))?;
    c.emit(0x82c92, I::Compare(R3, 149))?;
    c.emit(0x82c94, I::Branch(Some(Condition::Hi), 0x82c9e))?;
    c.emit(0x82c96, I::Mov(R0, 150))?;
    c.emit(0x82c98, I::Arithmetic(true, R0, R0, R3))?;
    c.emit(0x82c9a, I::Alu(Alu::Lsr, R1, R0))?;
    c.emit(0x82c9c, I::Branch(None, 0x82ca2))?;
    c.emit(0x82c9e, I::Subtract(R3, 150))?;
    c.emit(0x82ca0, I::Alu(Alu::Lsl, R1, R3))?;
    c.emit(0x82ca2, I::Alu(Alu::Mul, R2, R1))?;
    c.emit(0x82ca4, I::Push(0x4))?;
    c.emit(0x82ca6, I::Pop(0x1))?;
    c.emit(0x82ca8, I::BxLr)?;
    c.emit(0x82caa, I::Align)?;
    c.pool(0x82cac, 0x007fffff)?;
    c.pool(0x82cb0, 0x7fffffff)?;
    c.emit(0x82cb4, I::Push(0x40f8))?;
    c.emit(0x82cb6, I::Push(0x2))?;
    c.emit(0x82cb8, I::Pop(0x10))?;
    c.emit(0x82cba, I::Push(0x1))?;
    c.emit(0x82cbc, I::Pop(0x20))?;
    c.emit(0x82cbe, I::Mov(R7, 0))?;
    c.emit(0x82cc0, I::Push(0x80))?;
    c.emit(0x82cc2, I::Pop(0x4))?;
    c.emit(0x82cc4, I::Push(0x80))?;
    c.emit(0x82cc6, I::Pop(0x8))?;
    c.emit(0x82cc8, I::CallArm(0x81738))?;
    c.emit(0x82ccc, I::Compare(R0, 0))?;
    c.emit(0x82cce, I::Push(0x80))?;
    c.emit(0x82cd0, I::Pop(0x40))?;
    c.emit(0x82cd2, I::Branch(Some(Condition::Ne), 0x82d1a))?;
    c.emit(0x82cd4, I::Mov(R7, 0))?;
    c.emit(0x82cd6, I::Literal(R3, 0x82d28))?;
    c.emit(0x82cd8, I::Push(0x20))?;
    c.emit(0x82cda, I::Pop(0x1))?;
    c.emit(0x82cdc, I::Push(0x10))?;
    c.emit(0x82cde, I::Pop(0x2))?;
    c.emit(0x82ce0, I::Push(0x80))?;
    c.emit(0x82ce2, I::Pop(0x4))?;
    c.emit(0x82ce4, I::Call(0x83018))?;
    c.emit(0x82ce8, I::Call(0x82d30))?;
    c.emit(0x82cec, I::Push(0x1))?;
    c.emit(0x82cee, I::Pop(0x40))?;
    c.emit(0x82cf0, I::Call(0x82ed0))?;
    c.emit(0x82cf4, I::Literal(R3, 0x82d2c))?;
    c.emit(0x82cf6, I::Push(0x80))?;
    c.emit(0x82cf8, I::Pop(0x4))?;
    c.emit(0x82cfa, I::Call(0x83018))?;
    c.emit(0x82cfe, I::Push(0x1))?;
    c.emit(0x82d00, I::Pop(0x4))?;
    c.emit(0x82d02, I::Push(0x2))?;
    c.emit(0x82d04, I::Pop(0x8))?;
    c.emit(0x82d06, I::Push(0x20))?;
    c.emit(0x82d08, I::Pop(0x1))?;
    c.emit(0x82d0a, I::Push(0x10))?;
    c.emit(0x82d0c, I::Pop(0x2))?;
    c.emit(0x82d0e, I::Call(0x818a0))?;
    c.emit(0x82d12, I::Call(0x82d30))?;
    c.emit(0x82d16, I::Push(0x1))?;
    c.emit(0x82d18, I::Pop(0x80))?;
    c.emit(0x82d1a, I::Push(0x80))?;
    c.emit(0x82d1c, I::Pop(0x1))?;
    c.emit(0x82d1e, I::Push(0x40))?;
    c.emit(0x82d20, I::Pop(0x2))?;
    c.emit(0x82d22, I::StackAdjust(false, 4))?;
    c.emit(0x82d24, I::Pop(0x80f0))?;
    c.emit(0x82d26, I::Align)?;
    c.pool(0x82d28, 0x3df00000)?;
    c.pool(0x82d2c, 0xc1f00000)?;
    c.emit(0x82d30, I::Push(0x40b0))?;
    c.emit(0x82d32, I::Push(0x1))?;
    c.emit(0x82d34, I::Pop(0x4))?;
    c.emit(0x82d36, I::Mov(R0, 0))?;
    c.emit(0x82d38, I::Alu(Alu::Mvn, R4, R0))?;
    c.emit(0x82d3a, I::Literal(R3, 0x82d8c))?;
    c.emit(0x82d3c, I::Small(false, R4, R4, 1))?;
    c.emit(0x82d3e, I::Push(0x8))?;
    c.emit(0x82d40, I::Pop(0x20))?;
    c.emit(0x82d42, I::Alu(Alu::Adc, R5, R0))?;
    c.emit(0x82d44, I::Alu(Alu::And, R3, R1))?;
    c.emit(0x82d46, I::Alu(Alu::Orr, R3, R5))?;
    c.emit(0x82d48, I::Alu(Alu::Orr, R4, R2))?;
    c.emit(0x82d4a, I::Shift(Shift::Lsr, R2, R1, 20))?;
    c.emit(0x82d4c, I::Literal(R5, 0x82d90))?;
    c.emit(0x82d4e, I::Alu(Alu::And, R5, R2))?;
    c.emit(0x82d50, I::Literal(R2, 0x82d94))?;
    c.emit(0x82d52, I::Arithmetic(false, R2, R5, R2))?;
    c.emit(0x82d54, I::Compare(R1, 0))?;
    c.emit(0x82d56, I::Branch(Some(Condition::Lt), 0x82d88))?;
    c.emit(0x82d58, I::Compare(R2, 0))?;
    c.emit(0x82d5a, I::Branch(Some(Condition::Lt), 0x82d88))?;
    c.emit(0x82d5c, I::Mov(R0, 0))?;
    c.emit(0x82d5e, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x82d60, I::Compare(R2, 31))?;
    c.emit(0x82d62, I::Branch(Some(Condition::Hi), 0x82d88))?;
    c.emit(0x82d64, I::Mov(R0, 52))?;
    c.emit(0x82d66, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x82d68, I::Alu(Alu::Lsr, R4, R0))?;
    c.emit(0x82d6a, I::Mov(R1, 32))?;
    c.emit(0x82d6c, I::Arithmetic(true, R1, R1, R0))?;
    c.emit(0x82d6e, I::Push(0x8))?;
    c.emit(0x82d70, I::Pop(0x1))?;
    c.emit(0x82d72, I::Alu(Alu::Lsl, R0, R1))?;
    c.emit(0x82d74, I::Alu(Alu::Orr, R0, R4))?;
    c.emit(0x82d76, I::Mov(R1, 20))?;
    c.emit(0x82d78, I::Arithmetic(true, R1, R1, R2))?;
    c.emit(0x82d7a, I::Alu(Alu::Lsr, R3, R1))?;
    c.emit(0x82d7c, I::Compare(R1, 0))?;
    c.emit(0x82d7e, I::Branch(Some(Condition::Ge), 0x82d84))?;
    c.emit(0x82d80, I::Push(0x1))?;
    c.emit(0x82d82, I::Pop(0x8))?;
    c.emit(0x82d84, I::Push(0x8))?;
    c.emit(0x82d86, I::Pop(0x1))?;
    c.emit(0x82d88, I::Pop(0x80b0))?;
    c.emit(0x82d8a, I::Align)?;
    c.pool(0x82d8c, 0x000fffff)?;
    c.pool(0x82d90, 0x000007ff)?;
    c.pool(0x82d94, 0xfffffc01)?;
    c.emit(0x82d98, I::Push(0x4010))?;
    c.emit(0x82d9a, I::Push(0x1))?;
    c.emit(0x82d9c, I::Pop(0x2))?;
    c.emit(0x82d9e, I::Literal(R0, 0x82de0))?;
    c.emit(0x82da0, I::Push(0x2))?;
    c.emit(0x82da2, I::Pop(0x8))?;
    c.emit(0x82da4, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x82da6, I::Small(false, R2, R0, 1))?;
    c.emit(0x82da8, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x82daa, I::Shift(Shift::Lsr, R0, R1, 23))?;
    c.emit(0x82dac, I::Mov(R3, 255))?;
    c.emit(0x82dae, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x82db0, I::Push(0x8))?;
    c.emit(0x82db2, I::Pop(0x10))?;
    c.emit(0x82db4, I::Subtract(R4, 127))?;
    c.emit(0x82db6, I::Mov(R0, 0))?;
    c.emit(0x82db8, I::Compare(R1, 0))?;
    c.emit(0x82dba, I::Branch(Some(Condition::Lt), 0x82ddc))?;
    c.emit(0x82dbc, I::Compare(R3, 127))?;
    c.emit(0x82dbe, I::Branch(Some(Condition::Lo), 0x82ddc))?;
    c.emit(0x82dc0, I::Mov(R0, 0))?;
    c.emit(0x82dc2, I::Alu(Alu::Mvn, R0, R0))?;
    c.emit(0x82dc4, I::Compare(R4, 31))?;
    c.emit(0x82dc6, I::Branch(Some(Condition::Hi), 0x82ddc))?;
    c.emit(0x82dc8, I::Compare(R3, 149))?;
    c.emit(0x82dca, I::Branch(Some(Condition::Hi), 0x82dd4))?;
    c.emit(0x82dcc, I::Mov(R0, 150))?;
    c.emit(0x82dce, I::Arithmetic(true, R0, R0, R3))?;
    c.emit(0x82dd0, I::Alu(Alu::Lsr, R2, R0))?;
    c.emit(0x82dd2, I::Branch(None, 0x82dd8))?;
    c.emit(0x82dd4, I::Subtract(R3, 150))?;
    c.emit(0x82dd6, I::Alu(Alu::Lsl, R2, R3))?;
    c.emit(0x82dd8, I::Push(0x4))?;
    c.emit(0x82dda, I::Pop(0x1))?;
    c.emit(0x82ddc, I::Pop(0x8010))?;
    c.emit(0x82dde, I::Align)?;
    c.pool(0x82de0, 0x007fffff)?;
    c.emit(0x82de4, I::Push(0x40f8))?;
    c.emit(0x82de6, I::Push(0x1))?;
    c.emit(0x82de8, I::Pop(0x2))?;
    c.emit(0x82dea, I::Mov(R0, 0))?;
    c.emit(0x82dec, I::Compare(R1, 0))?;
    c.emit(0x82dee, I::Branch(Some(Condition::Ne), 0x82df6))?;
    c.emit(0x82df0, I::Push(0x1))?;
    c.emit(0x82df2, I::Pop(0x2))?;
    c.emit(0x82df4, I::Branch(None, 0x82e82))?;
    c.emit(0x82df6, I::Mov(R0, 1))?;
    c.emit(0x82df8, I::Stack(false, R0, 0))?;
    c.emit(0x82dfa, I::Shift(Shift::Lsl, R0, R0, 31))?;
    c.emit(0x82dfc, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x82dfe, I::Shift(Shift::Asr, R4, R1, 31))?;
    c.emit(0x82e00, I::Arithmetic(false, R2, R1, R4))?;
    c.emit(0x82e02, I::Alu(Alu::Eor, R2, R4))?;
    c.emit(0x82e04, I::Mov(R1, 32))?;
    c.emit(0x82e06, I::Compare(R2, 0))?;
    c.emit(0x82e08, I::Push(0x2))?;
    c.emit(0x82e0a, I::Pop(0x10))?;
    c.emit(0x82e0c, I::Branch(Some(Condition::Eq), 0x82e44))?;
    c.emit(0x82e0e, I::Shift(Shift::Lsr, R4, R2, 1))?;
    c.emit(0x82e10, I::Alu(Alu::Orr, R4, R2))?;
    c.emit(0x82e12, I::Shift(Shift::Lsr, R5, R4, 2))?;
    c.emit(0x82e14, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x82e16, I::Shift(Shift::Lsr, R4, R5, 4))?;
    c.emit(0x82e18, I::Alu(Alu::Orr, R4, R5))?;
    c.emit(0x82e1a, I::Shift(Shift::Lsr, R5, R4, 8))?;
    c.emit(0x82e1c, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x82e1e, I::Shift(Shift::Lsr, R4, R5, 16))?;
    c.emit(0x82e20, I::Alu(Alu::Orr, R4, R5))?;
    c.emit(0x82e22, I::Alu(Alu::Mvn, R4, R4))?;
    c.emit(0x82e24, I::Shift(Shift::Lsr, R5, R4, 1))?;
    c.emit(0x82e26, I::Literal(R6, 0x82e88))?;
    c.emit(0x82e28, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x82e2a, I::Arithmetic(true, R4, R4, R6))?;
    c.emit(0x82e2c, I::Literal(R5, 0x82e8c))?;
    c.emit(0x82e2e, I::Shift(Shift::Lsr, R6, R4, 2))?;
    c.emit(0x82e30, I::Alu(Alu::And, R4, R5))?;
    c.emit(0x82e32, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x82e34, I::Arithmetic(false, R4, R4, R6))?;
    c.emit(0x82e36, I::Shift(Shift::Lsr, R5, R4, 4))?;
    c.emit(0x82e38, I::Arithmetic(false, R4, R4, R5))?;
    c.emit(0x82e3a, I::Literal(R5, 0x82e90))?;
    c.emit(0x82e3c, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x82e3e, I::Literal(R4, 0x82e94))?;
    c.emit(0x82e40, I::Alu(Alu::Mul, R4, R5))?;
    c.emit(0x82e42, I::Shift(Shift::Lsr, R4, R4, 24))?;
    c.emit(0x82e44, I::Mov(R5, 31))?;
    c.emit(0x82e46, I::Arithmetic(true, R6, R5, R4))?;
    c.emit(0x82e48, I::Mov(R5, 20))?;
    c.emit(0x82e4a, I::Arithmetic(true, R5, R5, R6))?;
    c.emit(0x82e4c, I::Push(0x4))?;
    c.emit(0x82e4e, I::Pop(0x80))?;
    c.emit(0x82e50, I::Alu(Alu::Lsl, R7, R5))?;
    c.emit(0x82e52, I::Mov(R3, 52))?;
    c.emit(0x82e54, I::Arithmetic(true, R6, R3, R6))?;
    c.emit(0x82e56, I::Arithmetic(true, R3, R1, R6))?;
    c.emit(0x82e58, I::Push(0x4))?;
    c.emit(0x82e5a, I::Pop(0x2))?;
    c.emit(0x82e5c, I::Alu(Alu::Lsr, R1, R3))?;
    c.emit(0x82e5e, I::Compare(R5, 0))?;
    c.emit(0x82e60, I::Branch(Some(Condition::Ge), 0x82e66))?;
    c.emit(0x82e62, I::Push(0x2))?;
    c.emit(0x82e64, I::Pop(0x80))?;
    c.emit(0x82e66, I::Stack(true, R1, 0))?;
    c.emit(0x82e68, I::Shift(Shift::Lsl, R1, R1, 20))?;
    c.emit(0x82e6a, I::Alu(Alu::Eor, R1, R7))?;
    c.emit(0x82e6c, I::Literal(R3, 0x82e98))?;
    c.emit(0x82e6e, I::Arithmetic(true, R3, R3, R4))?;
    c.emit(0x82e70, I::Shift(Shift::Lsl, R3, R3, 20))?;
    c.emit(0x82e72, I::Arithmetic(false, R1, R1, R3))?;
    c.emit(0x82e74, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x82e76, I::Alu(Alu::Lsl, R2, R6))?;
    c.emit(0x82e78, I::Mov(R0, 0))?;
    c.emit(0x82e7a, I::Compare(R5, 0))?;
    c.emit(0x82e7c, I::Branch(Some(Condition::Ge), 0x82e82))?;
    c.emit(0x82e7e, I::Push(0x4))?;
    c.emit(0x82e80, I::Pop(0x1))?;
    c.emit(0x82e82, I::StackAdjust(false, 4))?;
    c.emit(0x82e84, I::Pop(0x80f0))?;
    c.emit(0x82e86, I::Align)?;
    c.pool(0x82e88, 0x55555555)?;
    c.pool(0x82e8c, 0x33333333)?;
    c.pool(0x82e90, 0x0f0f0f0f)?;
    c.pool(0x82e94, 0x01010101)?;
    c.pool(0x82e98, 0x0000041e)?;
    c.emit(0x82e9c, I::Push(0x40b0))?;
    c.emit(0x82e9e, I::Push(0x1))?;
    c.emit(0x82ea0, I::Pop(0x10))?;
    c.emit(0x82ea2, I::Mov(R0, 1))?;
    c.emit(0x82ea4, I::Shift(Shift::Lsl, R2, R0, 20))?;
    c.emit(0x82ea6, I::Literal(R5, 0x82ec4))?;
    c.emit(0x82ea8, I::Literal(R3, 0x82ec8))?;
    c.emit(0x82eaa, I::Push(0x2))?;
    c.emit(0x82eac, I::Pop(0x1))?;
    c.emit(0x82eae, I::Push(0x20))?;
    c.emit(0x82eb0, I::Pop(0x2))?;
    c.emit(0x82eb2, I::Call(0x818a0))?;
    c.emit(0x82eb6, I::Literal(R3, 0x82ecc))?;
    c.emit(0x82eb8, I::Push(0x10))?;
    c.emit(0x82eba, I::Pop(0x4))?;
    c.emit(0x82ebc, I::Call(0x818a0))?;
    c.emit(0x82ec0, I::Pop(0x80b0))?;
    c.emit(0x82ec2, I::Align)?;
    c.pool(0x82ec4, 0x45300000)?;
    c.pool(0x82ec8, 0xc5300000)?;
    c.pool(0x82ecc, 0x43300000)?;
    c.emit(0x82ed0, I::Push(0x4070))?;
    c.emit(0x82ed2, I::Mov(R2, 0))?;
    c.emit(0x82ed4, I::Compare(R0, 0))?;
    c.emit(0x82ed6, I::Branch(Some(Condition::Ne), 0x82ede))?;
    c.emit(0x82ed8, I::Push(0x4))?;
    c.emit(0x82eda, I::Pop(0x2))?;
    c.emit(0x82edc, I::Branch(None, 0x82f5a))?;
    c.emit(0x82ede, I::Mov(R1, 32))?;
    c.emit(0x82ee0, I::Compare(R0, 0))?;
    c.emit(0x82ee2, I::Push(0x2))?;
    c.emit(0x82ee4, I::Pop(0x4))?;
    c.emit(0x82ee6, I::Branch(Some(Condition::Eq), 0x82f1e))?;
    c.emit(0x82ee8, I::Shift(Shift::Lsr, R2, R0, 1))?;
    c.emit(0x82eea, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x82eec, I::Shift(Shift::Lsr, R3, R2, 2))?;
    c.emit(0x82eee, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x82ef0, I::Shift(Shift::Lsr, R2, R3, 4))?;
    c.emit(0x82ef2, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x82ef4, I::Shift(Shift::Lsr, R3, R2, 8))?;
    c.emit(0x82ef6, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x82ef8, I::Shift(Shift::Lsr, R2, R3, 16))?;
    c.emit(0x82efa, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x82efc, I::Alu(Alu::Mvn, R2, R2))?;
    c.emit(0x82efe, I::Shift(Shift::Lsr, R3, R2, 1))?;
    c.emit(0x82f00, I::Literal(R4, 0x82f60))?;
    c.emit(0x82f02, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x82f04, I::Arithmetic(true, R2, R2, R4))?;
    c.emit(0x82f06, I::Literal(R3, 0x82f64))?;
    c.emit(0x82f08, I::Shift(Shift::Lsr, R4, R2, 2))?;
    c.emit(0x82f0a, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x82f0c, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x82f0e, I::Arithmetic(false, R2, R2, R4))?;
    c.emit(0x82f10, I::Shift(Shift::Lsr, R3, R2, 4))?;
    c.emit(0x82f12, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x82f14, I::Literal(R3, 0x82f68))?;
    c.emit(0x82f16, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x82f18, I::Literal(R2, 0x82f6c))?;
    c.emit(0x82f1a, I::Alu(Alu::Mul, R2, R3))?;
    c.emit(0x82f1c, I::Shift(Shift::Lsr, R2, R2, 24))?;
    c.emit(0x82f1e, I::Mov(R3, 31))?;
    c.emit(0x82f20, I::Arithmetic(true, R4, R3, R2))?;
    c.emit(0x82f22, I::Mov(R3, 20))?;
    c.emit(0x82f24, I::Arithmetic(true, R3, R3, R4))?;
    c.emit(0x82f26, I::Push(0x1))?;
    c.emit(0x82f28, I::Pop(0x20))?;
    c.emit(0x82f2a, I::Alu(Alu::Lsl, R5, R3))?;
    c.emit(0x82f2c, I::Mov(R6, 52))?;
    c.emit(0x82f2e, I::Arithmetic(true, R4, R6, R4))?;
    c.emit(0x82f30, I::Arithmetic(true, R6, R1, R4))?;
    c.emit(0x82f32, I::Push(0x1))?;
    c.emit(0x82f34, I::Pop(0x2))?;
    c.emit(0x82f36, I::Alu(Alu::Lsr, R1, R6))?;
    c.emit(0x82f38, I::Compare(R3, 0))?;
    c.emit(0x82f3a, I::Branch(Some(Condition::Ge), 0x82f40))?;
    c.emit(0x82f3c, I::Push(0x2))?;
    c.emit(0x82f3e, I::Pop(0x20))?;
    c.emit(0x82f40, I::Mov(R1, 1))?;
    c.emit(0x82f42, I::Shift(Shift::Lsl, R1, R1, 20))?;
    c.emit(0x82f44, I::Alu(Alu::Eor, R1, R5))?;
    c.emit(0x82f46, I::Literal(R5, 0x82f70))?;
    c.emit(0x82f48, I::Arithmetic(true, R2, R5, R2))?;
    c.emit(0x82f4a, I::Shift(Shift::Lsl, R2, R2, 20))?;
    c.emit(0x82f4c, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x82f4e, I::Alu(Alu::Lsl, R0, R4))?;
    c.emit(0x82f50, I::Mov(R2, 0))?;
    c.emit(0x82f52, I::Compare(R3, 0))?;
    c.emit(0x82f54, I::Branch(Some(Condition::Ge), 0x82f5a))?;
    c.emit(0x82f56, I::Push(0x1))?;
    c.emit(0x82f58, I::Pop(0x4))?;
    c.emit(0x82f5a, I::Push(0x4))?;
    c.emit(0x82f5c, I::Pop(0x1))?;
    c.emit(0x82f5e, I::Pop(0x8070))?;
    c.pool(0x82f60, 0x55555555)?;
    c.pool(0x82f64, 0x33333333)?;
    c.pool(0x82f68, 0x0f0f0f0f)?;
    c.pool(0x82f6c, 0x01010101)?;
    c.pool(0x82f70, 0x0000041e)?;
    c.emit(0x82f74, I::Push(0x40b0))?;
    c.emit(0x82f76, I::Mov(R1, 0))?;
    c.emit(0x82f78, I::Compare(R0, 0))?;
    c.emit(0x82f7a, I::Branch(Some(Condition::Eq), 0x83002))?;
    c.emit(0x82f7c, I::Mov(R2, 1))?;
    c.emit(0x82f7e, I::Shift(Shift::Lsl, R1, R2, 23))?;
    c.emit(0x82f80, I::Mov(R3, 32))?;
    c.emit(0x82f82, I::Compare(R0, 0))?;
    c.emit(0x82f84, I::Branch(Some(Condition::Eq), 0x82fbc))?;
    c.emit(0x82f86, I::Shift(Shift::Lsr, R3, R0, 1))?;
    c.emit(0x82f88, I::Alu(Alu::Orr, R3, R0))?;
    c.emit(0x82f8a, I::Shift(Shift::Lsr, R4, R3, 2))?;
    c.emit(0x82f8c, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x82f8e, I::Shift(Shift::Lsr, R3, R4, 4))?;
    c.emit(0x82f90, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x82f92, I::Shift(Shift::Lsr, R4, R3, 8))?;
    c.emit(0x82f94, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x82f96, I::Shift(Shift::Lsr, R3, R4, 16))?;
    c.emit(0x82f98, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x82f9a, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x82f9c, I::Shift(Shift::Lsr, R4, R3, 1))?;
    c.emit(0x82f9e, I::Literal(R5, 0x83008))?;
    c.emit(0x82fa0, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x82fa2, I::Arithmetic(true, R3, R3, R5))?;
    c.emit(0x82fa4, I::Literal(R4, 0x8300c))?;
    c.emit(0x82fa6, I::Shift(Shift::Lsr, R5, R3, 2))?;
    c.emit(0x82fa8, I::Alu(Alu::And, R3, R4))?;
    c.emit(0x82faa, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x82fac, I::Arithmetic(false, R3, R3, R5))?;
    c.emit(0x82fae, I::Shift(Shift::Lsr, R4, R3, 4))?;
    c.emit(0x82fb0, I::Arithmetic(false, R3, R3, R4))?;
    c.emit(0x82fb2, I::Literal(R4, 0x83010))?;
    c.emit(0x82fb4, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x82fb6, I::Literal(R3, 0x83014))?;
    c.emit(0x82fb8, I::Alu(Alu::Mul, R3, R4))?;
    c.emit(0x82fba, I::Shift(Shift::Lsr, R3, R3, 24))?;
    c.emit(0x82fbc, I::Mov(R4, 31))?;
    c.emit(0x82fbe, I::Arithmetic(true, R3, R4, R3))?;
    c.emit(0x82fc0, I::Compare(R3, 23))?;
    c.emit(0x82fc2, I::Branch(Some(Condition::Gt), 0x82fce))?;
    c.emit(0x82fc4, I::Mov(R2, 23))?;
    c.emit(0x82fc6, I::Arithmetic(true, R2, R2, R3))?;
    c.emit(0x82fc8, I::Alu(Alu::Lsl, R0, R2))?;
    c.emit(0x82fca, I::Alu(Alu::Eor, R0, R1))?;
    c.emit(0x82fcc, I::Branch(None, 0x82ff8))?;
    c.emit(0x82fce, I::Mov(R4, 55))?;
    c.emit(0x82fd0, I::Arithmetic(true, R5, R4, R3))?;
    c.emit(0x82fd2, I::Push(0x1))?;
    c.emit(0x82fd4, I::Pop(0x10))?;
    c.emit(0x82fd6, I::Alu(Alu::Lsl, R4, R5))?;
    c.emit(0x82fd8, I::Push(0x8))?;
    c.emit(0x82fda, I::Pop(0x20))?;
    c.emit(0x82fdc, I::Subtract(R5, 23))?;
    c.emit(0x82fde, I::Alu(Alu::Lsr, R0, R5))?;
    c.emit(0x82fe0, I::Alu(Alu::Eor, R1, R0))?;
    c.emit(0x82fe2, I::Shift(Shift::Lsl, R5, R2, 31))?;
    c.emit(0x82fe4, I::Alu(Alu::Cmp, R4, R5))?;
    c.emit(0x82fe6, I::Branch(Some(Condition::Ls), 0x82fec))?;
    c.emit(0x82fe8, I::Small(false, R0, R1, 1))?;
    c.emit(0x82fea, I::Branch(None, 0x82ff8))?;
    c.emit(0x82fec, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x82fee, I::Arithmetic(false, R0, R1, R0))?;
    c.emit(0x82ff0, I::Alu(Alu::Cmp, R4, R5))?;
    c.emit(0x82ff2, I::Branch(Some(Condition::Eq), 0x82ff8))?;
    c.emit(0x82ff4, I::Push(0x2))?;
    c.emit(0x82ff6, I::Pop(0x1))?;
    c.emit(0x82ff8, I::Shift(Shift::Lsl, R1, R3, 23))?;
    c.emit(0x82ffa, I::Arithmetic(false, R0, R1, R0))?;
    c.emit(0x82ffc, I::Mov(R1, 127))?;
    c.emit(0x82ffe, I::Shift(Shift::Lsl, R1, R1, 23))?;
    c.emit(0x83000, I::Arithmetic(false, R1, R0, R1))?;
    c.emit(0x83002, I::Push(0x2))?;
    c.emit(0x83004, I::Pop(0x1))?;
    c.emit(0x83006, I::Pop(0x80b0))?;
    c.pool(0x83008, 0x55555555)?;
    c.pool(0x8300c, 0x33333333)?;
    c.pool(0x83010, 0x0f0f0f0f)?;
    c.pool(0x83014, 0x01010101)?;
    checked(c.at == END && c.bytes.len() == END - START)?;
    checked(c.code == CODE_BYTES && c.data == DATA_BYTES && c.align == ALIGN_BYTES)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    const BRANCHES: &[(usize, usize, Option<u16>)] = &[
        (0x82c2a, 0x82c44, Some(0)),
        (0x82c42, 0x82c54, None),
        (0x82c78, 0x82ca8, Some(3)),
        (0x82c82, 0x82c92, Some(3)),
        (0x82c8a, 0x82ca8, Some(0)),
        (0x82c90, 0x82ca8, None),
        (0x82c94, 0x82c9e, Some(8)),
        (0x82c9c, 0x82ca2, None),
        (0x82cd2, 0x82d1a, Some(1)),
        (0x82d56, 0x82d88, Some(11)),
        (0x82d5a, 0x82d88, Some(11)),
        (0x82d62, 0x82d88, Some(8)),
        (0x82d7e, 0x82d84, Some(10)),
        (0x82dba, 0x82ddc, Some(11)),
        (0x82dbe, 0x82ddc, Some(3)),
        (0x82dc6, 0x82ddc, Some(8)),
        (0x82dca, 0x82dd4, Some(8)),
        (0x82dd2, 0x82dd8, None),
        (0x82dee, 0x82df6, Some(1)),
        (0x82df4, 0x82e82, None),
        (0x82e0c, 0x82e44, Some(0)),
        (0x82e60, 0x82e66, Some(10)),
        (0x82e7c, 0x82e82, Some(10)),
        (0x82ed6, 0x82ede, Some(1)),
        (0x82edc, 0x82f5a, None),
        (0x82ee6, 0x82f1e, Some(0)),
        (0x82f3a, 0x82f40, Some(10)),
        (0x82f54, 0x82f5a, Some(10)),
        (0x82f7a, 0x83002, Some(0)),
        (0x82f84, 0x82fbc, Some(0)),
        (0x82fc2, 0x82fce, Some(12)),
        (0x82fcc, 0x82ff8, None),
        (0x82fe6, 0x82fec, Some(9)),
        (0x82fea, 0x82ff8, None),
        (0x82ff2, 0x82ff8, Some(0)),
    ];
    const CALLS: &[(usize, usize, bool)] = &[
        (0x82c02, 0x83c4c, false),
        (0x82c24, 0x81718, true),
        (0x82c3a, 0x82cb4, false),
        (0x82c4c, 0x82cb4, false),
        (0x82cc8, 0x81738, true),
        (0x82ce4, 0x83018, false),
        (0x82ce8, 0x82d30, false),
        (0x82cf0, 0x82ed0, false),
        (0x82cfa, 0x83018, false),
        (0x82d0e, 0x818a0, false),
        (0x82d12, 0x82d30, false),
        (0x82eb2, 0x818a0, false),
        (0x82ebc, 0x818a0, false),
    ];
    const LITERALS: &[(usize, usize)] = &[
        (0x82c5c, 0x82cac),
        (0x82c84, 0x82cb0),
        (0x82cd6, 0x82d28),
        (0x82cf4, 0x82d2c),
        (0x82d3a, 0x82d8c),
        (0x82d4c, 0x82d90),
        (0x82d50, 0x82d94),
        (0x82d9e, 0x82de0),
        (0x82e26, 0x82e88),
        (0x82e2c, 0x82e8c),
        (0x82e3a, 0x82e90),
        (0x82e3e, 0x82e94),
        (0x82e6c, 0x82e98),
        (0x82ea6, 0x82ec4),
        (0x82ea8, 0x82ec8),
        (0x82eb6, 0x82ecc),
        (0x82f00, 0x82f60),
        (0x82f06, 0x82f64),
        (0x82f14, 0x82f68),
        (0x82f18, 0x82f6c),
        (0x82f46, 0x82f70),
        (0x82f9e, 0x83008),
        (0x82fa4, 0x8300c),
        (0x82fb2, 0x83010),
        (0x82fb6, 0x83014),
    ];

    fn half(bytes: &[u8], at: usize) -> u16 {
        u16::from_le_bytes(bytes[at - START..at - START + 2].try_into().unwrap())
    }

    #[test]
    fn partition_digest_and_literals() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1076);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "8cb9f1988762540be8b5c8dafff50c999f24644dacaa25a9545090dde55e0193");
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        for at in ALIGNMENTS { assert_eq!(half(&bytes, at), 0x46c0); }
        assert_eq!(half(&bytes, ZERO_ALIGNMENT), 0);
        for &(at, target) in LITERALS {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, target);
        }
        let mut at = START;
        let (mut code, mut data, mut align) = (0, 0, 0);
        while at < END {
            if POOLS.iter().any(|&(slot, _)| slot == at) {
                data += 4;
                at += 4;
            } else if ALIGNMENTS.contains(&at) || at == ZERO_ALIGNMENT {
                align += 2;
                at += 2;
            } else {
                let word = half(&bytes, at);
                assert_ne!(word & 0xff00, 0xbf00);
                let size = if word & 0xf800 == 0xf000 { 4 } else { 2 };
                code += size;
                at += size;
            }
        }
        assert_eq!((at, code, data, align), (END, CODE_BYTES, DATA_BYTES, ALIGN_BYTES));
        assert_eq!(half(&bytes, 0x82d98), 0xb510);
    }

    #[test]
    fn branches_calls_and_arm_mode() {
        let bytes = fragment().unwrap();
        for &(at, target, condition) in BRANCHES {
            let word = half(&bytes, at);
            if let Some(condition) = condition {
                assert_eq!(word & 0xff00, 0xd000 | condition << 8);
            } else { assert_eq!(word & 0xf800, 0xe000); }
            let shift = if condition.is_some() { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for &(at, target, arm) in CALLS {
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
            let base = if arm { (at + 4) & !3 } else { at + 4 };
            assert_eq!((base as isize + delta) as usize, target);
        }
        assert_eq!(I::CallArm(0x81718).encode(0x82c26).unwrap(),
            I::CallArm(0x81718).encode(0x82c24).unwrap());
        assert_ne!(I::Call(0x81718).encode(0x82c24).unwrap(),
            I::CallArm(0x81718).encode(0x82c24).unwrap());
    }

    #[test]
    fn flags_carry_and_shift_edges() {
        let bytes = fragment().unwrap();
        for (at, word) in [(0x82c40, 0x418c), (0x82d3c, 0x1c64),
            (0x82d42, 0x4145), (0x82bee, 0x17e8)] {
            assert_eq!(half(&bytes, at), word);
        }
        assert_eq!(I::Shift(Shift::Lsl, R0, R1, 0).encode(START).unwrap(), 0x0008u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Asr, R0, R1, 32).encode(START).unwrap(), 0x1008u16.to_le_bytes());
    }

    #[test]
    fn operand_and_atomic_layout_guards() {
        for instruction in [I::Push(0), I::Push(0x8000), I::Pop(0x4000),
            I::StackAdjust(true, 510), I::Stack(true, R0, 2), I::Address(R1, 1024),
            I::Memory(true, R0, R1, 128), I::Memory(false, R0, R1, 2),
            I::Mov(R0, 256), I::Add(R0, 256), I::Compare(R0, 256), I::Subtract(R0, 256),
            I::Small(true, R0, R1, 8), I::Shift(Shift::Lsl, R0, R1, 32),
            I::Shift(Shift::Lsr, R0, R1, 0), I::Shift(Shift::Asr, R0, R1, 33),
            I::Literal(R0, START), I::Literal(R0, START + 2048), I::Literal(R0, START + 6),
            I::Branch(Some(Condition::Eq), START + 260), I::Branch(None, START + 2052),
            I::Branch(None, START + 1), I::Call(START + 16_777_220),
            I::CallArm(START + 2), I::CallArm(START + 16_777_220),
            I::LoadMultiple(R0, 0), I::LoadMultiple(R0, 1), I::LoadMultiple(R0, 256)] {
            assert!(instruction.encode(START).is_err(), "{instruction:?}");
        }
        assert!(I::Mov(R0, 0).encode(START + 1).is_err());
        assert!(I::Mov(R0, 0).encode(usize::MAX).is_err());
        let mut c = Code::new();
        assert!(c.emit(START + 2, I::Mov(R0, 0)).is_err());
        assert!(c.emit(START, I::Align).is_err());
        assert!(c.emit(START, I::ZeroAlign).is_err());
        assert!(c.pool(START, 0).is_err());
        for at in POOLS.iter().map(|&(at, _)| at)
            .chain(ALIGNMENTS).chain([ZERO_ALIGNMENT, END]) {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            assert!(c.emit(at, I::Mov(R0, 0)).is_err());
            assert!(c.pool(at, 0).is_err());
            assert_eq!((c.at, c.bytes.clone(), c.code, c.data, c.align), before);
            if at < END {
                c.at = at - 2;
                assert!(c.emit(at - 2, I::Call(START)).is_err());
            }
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
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
