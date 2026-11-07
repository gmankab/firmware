pub const START: usize = 0x83608;
pub const END: usize = 0x837f0;
pub const CODE_BYTES: usize = 462;
pub const DATA_BYTES: usize = 24;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 2;
const POOLS: [(usize, u32); 6] = [
    (0x83778, 0x007fffff), (0x8377c, 0x7fc00000),
    (0x837d0, 0x55555555), (0x837d4, 0x33333333),
    (0x837d8, 0x0f0f0f0f), (0x837dc, 0x01010101),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Tst, Cmp, Orr, Mul, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Hs = 2, Lo = 3, Hi = 8, Ls = 9, Ge = 10, Lt = 11, Gt = 12, Le = 13 }
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
        checked(!matches!(instruction, I::Align))?;
        checked(matches!(instruction, I::ZeroAlign) == (at == 0x837ee))?;
        checked(![0x837ee].iter().any(|&slot| at < slot && at + bytes.len() > slot))?;
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
    c.emit(0x83608, I::Push(0x40f0))?;
    c.emit(0x8360a, I::StackAdjust(true, 0x24))?;
    c.emit(0x8360c, I::Literal(R3, 0x83778))?;
    c.emit(0x8360e, I::Push(1))?;
    c.emit(0x83610, I::Pop(4))?;
    c.emit(0x83612, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x83614, I::Stack(false, R2, 0x20))?;
    c.emit(0x83616, I::Push(2))?;
    c.emit(0x83618, I::Pop(4))?;
    c.emit(0x8361a, I::Stack(false, R3, 0x10))?;
    c.emit(0x8361c, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x8361e, I::Stack(false, R2, 0x1c))?;
    c.emit(0x83620, I::Push(2))?;
    c.emit(0x83622, I::Pop(0x20))?;
    c.emit(0x83624, I::Alu(Alu::Eor, R5, R0))?;
    c.emit(0x83626, I::Mov(R2, 1))?;
    c.emit(0x83628, I::Stack(false, R2, 0xc))?;
    c.emit(0x8362a, I::Shift(Shift::Lsl, R7, R2, 31))?;
    c.emit(0x8362c, I::Alu(Alu::And, R5, R7))?;
    c.emit(0x8362e, I::Shift(Shift::Lsr, R6, R1, 23))?;
    c.emit(0x83630, I::Mov(R2, 255))?;
    c.emit(0x83632, I::Alu(Alu::And, R6, R2))?;
    c.emit(0x83634, I::Shift(Shift::Lsr, R4, R0, 23))?;
    c.emit(0x83636, I::Alu(Alu::And, R4, R2))?;
    c.emit(0x83638, I::Small(true, R3, R4, 1))?;
    c.emit(0x8363a, I::Shift(Shift::Lsl, R2, R2, 23))?;
    c.emit(0x8363c, I::Stack(false, R2, 0x18))?;
    c.emit(0x8363e, I::Compare(R3, 253))?;
    c.emit(0x83640, I::Stack(false, R7, 0x14))?;
    c.emit(0x83642, I::Branch(Some(Condition::Hi), 0x8364c))?;
    c.emit(0x83644, I::Small(true, R2, R6, 1))?;
    c.emit(0x83646, I::Mov(R3, 0))?;
    c.emit(0x83648, I::Compare(R2, 254))?;
    c.emit(0x8364a, I::Branch(Some(Condition::Lo), 0x836d4))?;
    c.emit(0x8364c, I::Stack(false, R5, 8))?;
    c.emit(0x8364e, I::Small(true, R3, R7, 1))?;
    c.emit(0x83650, I::Push(2))?;
    c.emit(0x83652, I::Pop(0x80))?;
    c.emit(0x83654, I::Alu(Alu::And, R7, R3))?;
    c.emit(0x83656, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x83658, I::Mov(R2, 1))?;
    c.emit(0x8365a, I::Shift(Shift::Lsl, R2, R2, 22))?;
    c.emit(0x8365c, I::Stack(true, R5, 0x18))?;
    c.emit(0x8365e, I::Alu(Alu::Cmp, R3, R5))?;
    c.emit(0x83660, I::Branch(Some(Condition::Ls), 0x83666))?;
    c.emit(0x83662, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x83664, I::Branch(None, 0x83696))?;
    c.emit(0x83666, I::Alu(Alu::Cmp, R7, R5))?;
    c.emit(0x83668, I::Branch(Some(Condition::Ls), 0x8366e))?;
    c.emit(0x8366a, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x8366c, I::Branch(None, 0x8367e))?;
    c.emit(0x8366e, I::Literal(R2, 0x8377c))?;
    c.emit(0x83670, I::Alu(Alu::Cmp, R3, R5))?;
    c.emit(0x83672, I::Branch(Some(Condition::Ne), 0x83684))?;
    c.emit(0x83674, I::Stack(true, R3, 0x14))?;
    c.emit(0x83676, I::Alu(Alu::And, R1, R3))?;
    c.emit(0x83678, I::Alu(Alu::Eor, R1, R0))?;
    c.emit(0x8367a, I::Compare(R7, 0))?;
    c.emit(0x8367c, I::Branch(Some(Condition::Eq), 0x83770))?;
    c.emit(0x8367e, I::Push(2))?;
    c.emit(0x83680, I::Pop(4))?;
    c.emit(0x83682, I::Branch(None, 0x83770))?;
    c.emit(0x83684, I::Alu(Alu::Cmp, R7, R5))?;
    c.emit(0x83686, I::Branch(Some(Condition::Ne), 0x8369c))?;
    c.emit(0x83688, I::Stack(true, R4, 0x14))?;
    c.emit(0x8368a, I::Alu(Alu::And, R0, R4))?;
    c.emit(0x8368c, I::Alu(Alu::Eor, R0, R1))?;
    c.emit(0x8368e, I::Compare(R3, 0))?;
    c.emit(0x83690, I::Branch(Some(Condition::Ne), 0x83696))?;
    c.emit(0x83692, I::Push(4))?;
    c.emit(0x83694, I::Pop(1))?;
    c.emit(0x83696, I::Push(1))?;
    c.emit(0x83698, I::Pop(4))?;
    c.emit(0x8369a, I::Branch(None, 0x83770))?;
    c.emit(0x8369c, I::Compare(R3, 0))?;
    c.emit(0x8369e, I::Branch(Some(Condition::Ne), 0x836a4))?;
    c.emit(0x836a0, I::Stack(true, R2, 8))?;
    c.emit(0x836a2, I::Branch(None, 0x83770))?;
    c.emit(0x836a4, I::Compare(R7, 0))?;
    c.emit(0x836a6, I::Stack(true, R5, 8))?;
    c.emit(0x836a8, I::Branch(Some(Condition::Eq), 0x8376c))?;
    c.emit(0x836aa, I::Stack(true, R0, 0x10))?;
    c.emit(0x836ac, I::Small(false, R1, R0, 1))?;
    c.emit(0x836ae, I::Mov(R0, 0))?;
    c.emit(0x836b0, I::Stack(false, R0, 4))?;
    c.emit(0x836b2, I::Alu(Alu::Cmp, R3, R1))?;
    c.emit(0x836b4, I::Branch(Some(Condition::Hs), 0x836c2))?;
    c.emit(0x836b6, I::Address(R0, 0x20))?;
    c.emit(0x836b8, I::Stack(false, R1, 0))?;
    c.emit(0x836ba, I::Call(0x83780))?;
    c.emit(0x836be, I::Stack(true, R1, 0))?;
    c.emit(0x836c0, I::Stack(false, R0, 4))?;
    c.emit(0x836c2, I::Alu(Alu::Cmp, R7, R1))?;
    c.emit(0x836c4, I::Branch(Some(Condition::Lo), 0x836ca))?;
    c.emit(0x836c6, I::Stack(true, R3, 4))?;
    c.emit(0x836c8, I::Branch(None, 0x836d4))?;
    c.emit(0x836ca, I::Address(R0, 0x1c))?;
    c.emit(0x836cc, I::Call(0x83780))?;
    c.emit(0x836d0, I::Stack(true, R3, 4))?;
    c.emit(0x836d2, I::Arithmetic(false, R3, R0, R3))?;
    c.emit(0x836d4, I::Stack(false, R3, 4))?;
    c.emit(0x836d6, I::Stack(true, R0, 0x10))?;
    c.emit(0x836d8, I::Small(false, R7, R0, 1))?;
    c.emit(0x836da, I::Stack(true, R2, 0x20))?;
    c.emit(0x836dc, I::Alu(Alu::Orr, R2, R7))?;
    c.emit(0x836de, I::Stack(false, R2, 0x20))?;
    c.emit(0x836e0, I::Stack(true, R0, 0x1c))?;
    c.emit(0x836e2, I::Alu(Alu::Orr, R0, R7))?;
    c.emit(0x836e4, I::Stack(false, R0, 0x1c))?;
    c.emit(0x836e6, I::Shift(Shift::Lsl, R0, R0, 8))?;
    c.emit(0x836e8, I::Mov(R1, 0))?;
    c.emit(0x836ea, I::Push(2))?;
    c.emit(0x836ec, I::Pop(8))?;
    c.emit(0x836ee, I::Call(0x835b0))?;
    c.emit(0x836f2, I::Arithmetic(false, R2, R4, R6))?;
    c.emit(0x836f4, I::Stack(true, R3, 4))?;
    c.emit(0x836f6, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x836f8, I::Subtract(R2, 127))?;
    c.emit(0x836fa, I::Alu(Alu::Tst, R1, R7))?;
    c.emit(0x836fc, I::Branch(Some(Condition::Eq), 0x83702))?;
    c.emit(0x836fe, I::Small(false, R2, R2, 1))?;
    c.emit(0x83700, I::Branch(None, 0x8370a))?;
    c.emit(0x83702, I::Shift(Shift::Lsl, R3, R1, 1))?;
    c.emit(0x83704, I::Shift(Shift::Lsr, R1, R0, 31))?;
    c.emit(0x83706, I::Alu(Alu::Orr, R1, R3))?;
    c.emit(0x83708, I::Shift(Shift::Lsl, R0, R0, 1))?;
    c.emit(0x8370a, I::Stack(true, R7, 0x14))?;
    c.emit(0x8370c, I::Stack(true, R3, 0x18))?;
    c.emit(0x8370e, I::Compare(R2, 255))?;
    c.emit(0x83710, I::Branch(Some(Condition::Lt), 0x83716))?;
    c.emit(0x83712, I::Alu(Alu::Orr, R5, R3))?;
    c.emit(0x83714, I::Branch(None, 0x8376c))?;
    c.emit(0x83716, I::Compare(R2, 0))?;
    c.emit(0x83718, I::Branch(Some(Condition::Gt), 0x8374c))?;
    c.emit(0x8371a, I::Stack(true, R6, 0xc))?;
    c.emit(0x8371c, I::Arithmetic(true, R2, R6, R2))?;
    c.emit(0x8371e, I::Compare(R2, 31))?;
    c.emit(0x83720, I::Branch(Some(Condition::Hi), 0x8376c))?;
    c.emit(0x83722, I::Push(1))?;
    c.emit(0x83724, I::Pop(8))?;
    c.emit(0x83726, I::Alu(Alu::Lsr, R3, R2))?;
    c.emit(0x83728, I::Stack(false, R3, 0x14))?;
    c.emit(0x8372a, I::Mov(R3, 32))?;
    c.emit(0x8372c, I::Arithmetic(true, R3, R3, R2))?;
    c.emit(0x8372e, I::Stack(false, R3, 0x18))?;
    c.emit(0x83730, I::Push(2))?;
    c.emit(0x83732, I::Pop(8))?;
    c.emit(0x83734, I::Stack(true, R4, 0x18))?;
    c.emit(0x83736, I::Alu(Alu::Lsl, R3, R4))?;
    c.emit(0x83738, I::Stack(true, R4, 0x14))?;
    c.emit(0x8373a, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x8373c, I::Stack(true, R4, 0x18))?;
    c.emit(0x8373e, I::Alu(Alu::Lsl, R0, R4))?;
    c.emit(0x83740, I::Branch(Some(Condition::Ne), 0x83746))?;
    c.emit(0x83742, I::Push(1))?;
    c.emit(0x83744, I::Pop(0x40))?;
    c.emit(0x83746, I::Alu(Alu::Orr, R3, R6))?;
    c.emit(0x83748, I::Alu(Alu::Lsr, R1, R2))?;
    c.emit(0x8374a, I::Branch(None, 0x83758))?;
    c.emit(0x8374c, I::Stack(true, R3, 0x10))?;
    c.emit(0x8374e, I::Alu(Alu::And, R1, R3))?;
    c.emit(0x83750, I::Shift(Shift::Lsl, R2, R2, 23))?;
    c.emit(0x83752, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83754, I::Push(1))?;
    c.emit(0x83756, I::Pop(8))?;
    c.emit(0x83758, I::Stack(true, R0, 0xc))?;
    c.emit(0x8375a, I::Alu(Alu::Orr, R5, R1))?;
    c.emit(0x8375c, I::Alu(Alu::Cmp, R3, R7))?;
    c.emit(0x8375e, I::Branch(Some(Condition::Ls), 0x83764))?;
    c.emit(0x83760, I::Small(false, R2, R5, 1))?;
    c.emit(0x83762, I::Branch(None, 0x83770))?;
    c.emit(0x83764, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x83766, I::Arithmetic(false, R2, R5, R1))?;
    c.emit(0x83768, I::Alu(Alu::Cmp, R3, R7))?;
    c.emit(0x8376a, I::Branch(Some(Condition::Eq), 0x83770))?;
    c.emit(0x8376c, I::Push(0x20))?;
    c.emit(0x8376e, I::Pop(4))?;
    c.emit(0x83770, I::Push(4))?;
    c.emit(0x83772, I::Pop(1))?;
    c.emit(0x83774, I::StackAdjust(false, 0x24))?;
    c.emit(0x83776, I::Pop(0x80f0))?;
    c.pool(0x83778, 0x007fffff)?;
    c.pool(0x8377c, 0x7fc00000)?;
    c.emit(0x83780, I::Push(0x4010))?;
    c.emit(0x83782, I::Memory(true, R1, R0, 0))?;
    c.emit(0x83784, I::Mov(R2, 32))?;
    c.emit(0x83786, I::Compare(R1, 0))?;
    c.emit(0x83788, I::Branch(Some(Condition::Eq), 0x837c0))?;
    c.emit(0x8378a, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x8378c, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x8378e, I::Shift(Shift::Lsr, R3, R2, 2))?;
    c.emit(0x83790, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x83792, I::Shift(Shift::Lsr, R2, R3, 4))?;
    c.emit(0x83794, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x83796, I::Shift(Shift::Lsr, R3, R2, 8))?;
    c.emit(0x83798, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x8379a, I::Shift(Shift::Lsr, R2, R3, 16))?;
    c.emit(0x8379c, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x8379e, I::Alu(Alu::Mvn, R2, R2))?;
    c.emit(0x837a0, I::Shift(Shift::Lsr, R3, R2, 1))?;
    c.emit(0x837a2, I::Literal(R4, 0x837d0))?;
    c.emit(0x837a4, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x837a6, I::Arithmetic(true, R2, R2, R4))?;
    c.emit(0x837a8, I::Literal(R3, 0x837d4))?;
    c.emit(0x837aa, I::Shift(Shift::Lsr, R4, R2, 2))?;
    c.emit(0x837ac, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x837ae, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x837b0, I::Arithmetic(false, R2, R2, R4))?;
    c.emit(0x837b2, I::Shift(Shift::Lsr, R3, R2, 4))?;
    c.emit(0x837b4, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x837b6, I::Literal(R3, 0x837d8))?;
    c.emit(0x837b8, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x837ba, I::Literal(R2, 0x837dc))?;
    c.emit(0x837bc, I::Alu(Alu::Mul, R2, R3))?;
    c.emit(0x837be, I::Shift(Shift::Lsr, R2, R2, 24))?;
    c.emit(0x837c0, I::Push(4))?;
    c.emit(0x837c2, I::Pop(8))?;
    c.emit(0x837c4, I::Subtract(R3, 8))?;
    c.emit(0x837c6, I::Alu(Alu::Lsl, R1, R3))?;
    c.emit(0x837c8, I::Memory(false, R1, R0, 0))?;
    c.emit(0x837ca, I::Mov(R0, 9))?;
    c.emit(0x837cc, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x837ce, I::Pop(0x8010))?;
    c.pool(0x837d0, 0x55555555)?;
    c.pool(0x837d4, 0x33333333)?;
    c.pool(0x837d8, 0x0f0f0f0f)?;
    c.pool(0x837dc, 0x01010101)?;
    c.emit(0x837e0, I::Push(0x4080))?;
    c.emit(0x837e2, I::Mov(R2, 1))?;
    c.emit(0x837e4, I::Shift(Shift::Lsl, R2, R2, 31))?;
    c.emit(0x837e6, I::Alu(Alu::Eor, R1, R2))?;
    c.emit(0x837e8, I::Call(0x81e64))?;
    c.emit(0x837ec, I::Pop(0x8080))?;
    c.emit(0x837ee, I::ZeroAlign)?;
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
        assert_eq!(bytes.len(), 488);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        for (start, end, digest) in [
            (START, END, "d64ec77f7601aac9f871cd008dcf90d63cb6ae9bf9567860f9b52299ee91c19e"),
            (START, 0x83778, "6d3df5b0fb54c2cf6f4d058546cecb85361d8da29f9961ffec1616c6d1dd1779"),
            (0x83778, 0x83780, "ceb43700cc3ec44b0eb4d1fc6b4b8aed9d804bdb57ccb5c15d3a7e1669d7c543"),
            (0x83780, 0x837d0, "9dd23d40529d43456346906d8f92d0d58e25541657c156f6f311191da3b76200"),
            (0x837d0, 0x837e0, "7b520efa5bec97643152d100759ea3c6711e85bcb95af6f18463cd1ab759f4b8"),
            (0x837e0, 0x837ee, "bc82fb50b10edd25496cfa621697d0d320d394d9f88233b266857380e9bb23f6"),
            (0x837ee, END, "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        ] {
            assert_eq!(crate::wmfw_sha256::hex_digest(&bytes[start - START..end - START]), digest);
        }
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        assert_eq!(half(&bytes, 0x837ee), 0);
    }

    #[test]
    fn independently_decoded_control_flow_and_literals() {
        let bytes = fragment().unwrap();
        for (at, target, condition) in [
            (0x83642, 0x8364c, Some(8)),
            (0x8364a, 0x836d4, Some(3)),
            (0x83660, 0x83666, Some(9)),
            (0x83664, 0x83696, None),
            (0x83668, 0x8366e, Some(9)),
            (0x8366c, 0x8367e, None),
            (0x83672, 0x83684, Some(1)),
            (0x8367c, 0x83770, Some(0)),
            (0x83682, 0x83770, None),
            (0x83686, 0x8369c, Some(1)),
            (0x83690, 0x83696, Some(1)),
            (0x8369a, 0x83770, None),
            (0x8369e, 0x836a4, Some(1)),
            (0x836a2, 0x83770, None),
            (0x836a8, 0x8376c, Some(0)),
            (0x836b4, 0x836c2, Some(2)),
            (0x836c4, 0x836ca, Some(3)),
            (0x836c8, 0x836d4, None),
            (0x836fc, 0x83702, Some(0)),
            (0x83700, 0x8370a, None),
            (0x83710, 0x83716, Some(11)),
            (0x83714, 0x8376c, None),
            (0x83718, 0x8374c, Some(12)),
            (0x83720, 0x8376c, Some(8)),
            (0x83740, 0x83746, Some(1)),
            (0x8374a, 0x83758, None),
            (0x8375e, 0x83764, Some(9)),
            (0x83762, 0x83770, None),
            (0x8376a, 0x83770, Some(0)),
            (0x83788, 0x837c0, Some(0))
        ] {
            let word = half(&bytes, at);
            let shift = if let Some(condition) = condition {
                assert_eq!(word >> 12, 13);
                assert_eq!((word >> 8) & 15, condition);
                8
            } else {
                assert_eq!(word & 0xf800, 0xe000);
                5
            };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [
            (0x836ba, 0x83780),
            (0x836cc, 0x83780),
            (0x836ee, 0x835b0),
            (0x837e8, 0x81e64)
        ] {
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
        for (at, slot) in [
            (0x8360c, 0x83778),
            (0x8366e, 0x8377c),
            (0x837a2, 0x837d0),
            (0x837a8, 0x837d4),
            (0x837b6, 0x837d8),
            (0x837ba, 0x837dc)
        ] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn flag_setting_carry_shifts_and_rounding_forms() {
        let bytes = fragment().unwrap();
        for (at, word) in [
            (0x8363e, 0x2bfd), (0x83640, 0x9705),
            (0x836fa, 0x4239), (0x83702, 0x004b),
            (0x83704, 0x0fc1), (0x83708, 0x0040),
            (0x83726, 0x40d3), (0x83736, 0x40a3),
            (0x8373e, 0x40a0), (0x83748, 0x40d1),
            (0x8375c, 0x42bb), (0x83760, 0x1c6a),
            (0x83764, 0x4001), (0x83766, 0x186a),
            (0x837c6, 0x4099), (0x837e4, 0x07d2),
        ] {
            assert_eq!(half(&bytes, at), word, "{at:x}");
        }
        assert_eq!(I::Shift(Shift::Lsl, R0, R1, 0).encode(START).unwrap(), 0x0008u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
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
        assert!(c.pool(START, 0).is_err());
        for at in [0x83776, 0x83778, 0x8377e, 0x837ce, 0x837d0, 0x837ec, 0x837ee, END] {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            let instruction = if [0x83776, 0x8377e, 0x837ce, 0x837ec].contains(&at) {
                I::Call(0x83780)
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
            "d64ec77f7601aac9f871cd008dcf90d63cb6ae9bf9567860f9b52299ee91c19e");
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
