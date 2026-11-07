pub const START: usize = 0x823b0;
pub const END: usize = 0x82988;
pub const CODE_BYTES: usize = 1442;
pub const DATA_BYTES: usize = 48;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 6;
const ALIGNMENTS: [usize; 2] = [0x8289e, 0x82976];
const ZERO_ALIGNMENT: usize = 0x828b2;
const POOLS: [(usize, u32); 12] = [
    (0x825c4, 0x000fffff),
    (0x825c8, 0x7fffffff),
    (0x825cc, 0x000007fd),
    (0x825d0, 0x7ff00000),
    (0x828a0, 0x7ff80000),
    (0x828a4, 0x7504f333),
    (0x828a8, 0x000003ff),
    (0x828ac, 0xfffffc02),
    (0x82978, 0x55555555),
    (0x8297c, 0x33333333),
    (0x82980, 0x0f0f0f0f),
    (0x82984, 0x01010101),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Adc, Sbc, Neg, Tst, Cmp, Orr, Mul, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Hs = 2, Lo = 3, Hi = 8, Ls = 9, Ge = 10, Lt = 11, Le = 13 }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(bool, u16),
    Stack(bool, R, u16), Address(R, u16), Literal(R, usize),
    Alu(Alu, R, R), Shift(Shift, R, R, u8),
    Add(R, u16), Mov(R, u16), Compare(R, u16), Subtract(R, u16),
    Small(bool, R, R, u8), Arithmetic(bool, R, R, R),
    Branch(Option<Condition>, usize), Call(usize), BxLr, LoadMultiple(R, u16), Memory(bool, R, R, u16), Align, ZeroAlign,
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
                    Alu::Adc => 5, Alu::Sbc => 6, Alu::Neg => 9, Alu::Tst => 8, Alu::Cmp => 10,
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
            Self::Call(target) => {
                let delta = displacement(at, target, -16_777_216, 16_777_214)? as u32;
                let s = (delta >> 24) & 1;
                let j1 = (!(delta >> 23) ^ s) & 1;
                let j2 = (!(delta >> 22) ^ s) & 1;
                let first = 0xf000 | (s << 10) | ((delta >> 12) & 0x3ff);
                let second = 0xd000 | (j1 << 13) | (j2 << 11) | ((delta >> 1) & 0x7ff);
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
        checked(![ALIGNMENTS[0], ZERO_ALIGNMENT, ALIGNMENTS[1]].iter().any(|&slot| at < slot && at + bytes.len() > slot))?;
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
    c.emit(0x823b0, I::Push(0x40f0))?;
    c.emit(0x823b2, I::StackAdjust(true, 92))?;
    c.emit(0x823b4, I::Push(0x8))?;
    c.emit(0x823b6, I::Pop(0x10))?;
    c.emit(0x823b8, I::Push(0x1))?;
    c.emit(0x823ba, I::Pop(0x8))?;
    c.emit(0x823bc, I::Stack(false, R3, 0x50))?;
    c.emit(0x823be, I::Literal(R5, 0x825c4))?;
    c.emit(0x823c0, I::Push(0x2))?;
    c.emit(0x823c2, I::Pop(0x1))?;
    c.emit(0x823c4, I::Alu(Alu::And, R0, R5))?;
    c.emit(0x823c6, I::Stack(false, R0, 0x54))?;
    c.emit(0x823c8, I::Push(0x10))?;
    c.emit(0x823ca, I::Pop(0x1))?;
    c.emit(0x823cc, I::Alu(Alu::And, R0, R5))?;
    c.emit(0x823ce, I::Stack(false, R0, 0x4c))?;
    c.emit(0x823d0, I::Stack(false, R2, 0x48))?;
    c.emit(0x823d2, I::Mov(R6, 0))?;
    c.emit(0x823d4, I::Alu(Alu::Mvn, R0, R6))?;
    c.emit(0x823d6, I::Literal(R7, 0x825c8))?;
    c.emit(0x823d8, I::Stack(false, R0, 0x44))?;
    c.emit(0x823da, I::Small(false, R0, R0, 1))?;
    c.emit(0x823dc, I::Stack(false, R0, 0x40))?;
    c.emit(0x823de, I::Stack(false, R7, 0x3c))?;
    c.emit(0x823e0, I::Alu(Alu::Adc, R7, R6))?;
    c.emit(0x823e2, I::Push(0x10))?;
    c.emit(0x823e4, I::Pop(0x1))?;
    c.emit(0x823e6, I::Alu(Alu::Eor, R4, R1))?;
    c.emit(0x823e8, I::Alu(Alu::And, R4, R7))?;
    c.emit(0x823ea, I::Stack(false, R4, 0x28))?;
    c.emit(0x823ec, I::Stack(false, R2, 0x34))?;
    c.emit(0x823ee, I::Stack(false, R3, 0x38))?;
    c.emit(0x823f0, I::Alu(Alu::Eor, R2, R3))?;
    c.emit(0x823f2, I::Push(0x1))?;
    c.emit(0x823f4, I::Pop(0x8))?;
    c.emit(0x823f6, I::Stack(true, R0, 0x40))?;
    c.emit(0x823f8, I::Alu(Alu::And, R2, R0))?;
    c.emit(0x823fa, I::Stack(false, R2, 0x2c))?;
    c.emit(0x823fc, I::Literal(R0, 0x825cc))?;
    c.emit(0x823fe, I::Small(false, R2, R0, 2))?;
    c.emit(0x82400, I::Stack(false, R3, 0x30))?;
    c.emit(0x82402, I::Shift(Shift::Lsr, R7, R3, 20))?;
    c.emit(0x82404, I::Alu(Alu::And, R7, R2))?;
    c.emit(0x82406, I::Shift(Shift::Lsr, R4, R1, 20))?;
    c.emit(0x82408, I::Alu(Alu::And, R4, R2))?;
    c.emit(0x8240a, I::Stack(false, R4, 0x20))?;
    c.emit(0x8240c, I::Small(true, R2, R4, 1))?;
    c.emit(0x8240e, I::Literal(R4, 0x825d0))?;
    c.emit(0x82410, I::Stack(false, R4, 0x40))?;
    c.emit(0x82412, I::Alu(Alu::Cmp, R2, R0))?;
    c.emit(0x82414, I::Stack(false, R5, 0x1c))?;
    c.emit(0x82416, I::Stack(false, R7, 0x24))?;
    c.emit(0x82418, I::Branch(Some(Condition::Hi), 0x82426))?;
    c.emit(0x8241a, I::Small(true, R2, R7, 1))?;
    c.emit(0x8241c, I::Alu(Alu::Cmp, R2, R0))?;
    c.emit(0x8241e, I::Push(0x40))?;
    c.emit(0x82420, I::Pop(0x1))?;
    c.emit(0x82422, I::Branch(Some(Condition::Hi), 0x82426))?;
    c.emit(0x82424, I::Branch(None, 0x825e2))?;
    c.emit(0x82426, I::Stack(false, R6, 0x10))?;
    c.emit(0x82428, I::Push(0x2))?;
    c.emit(0x8242a, I::Pop(0x80))?;
    c.emit(0x8242c, I::Stack(true, R0, 0x3c))?;
    c.emit(0x8242e, I::Alu(Alu::And, R7, R0))?;
    c.emit(0x82430, I::Mov(R6, 1))?;
    c.emit(0x82432, I::Mov(R4, 0))?;
    c.emit(0x82434, I::Stack(true, R0, 0x40))?;
    c.emit(0x82436, I::Alu(Alu::Cmp, R7, R0))?;
    c.emit(0x82438, I::Push(0x40))?;
    c.emit(0x8243a, I::Pop(0x20))?;
    c.emit(0x8243c, I::Branch(Some(Condition::Ls), 0x82442))?;
    c.emit(0x8243e, I::Push(0x10))?;
    c.emit(0x82440, I::Pop(0x20))?;
    c.emit(0x82442, I::Stack(true, R0, 0x38))?;
    c.emit(0x82444, I::Stack(true, R2, 0x44))?;
    c.emit(0x82446, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x82448, I::Compare(R0, 0))?;
    c.emit(0x8244a, I::Push(0x40))?;
    c.emit(0x8244c, I::Pop(0x4))?;
    c.emit(0x8244e, I::Branch(Some(Condition::Eq), 0x82454))?;
    c.emit(0x82450, I::Push(0x10))?;
    c.emit(0x82452, I::Pop(0x4))?;
    c.emit(0x82454, I::Stack(true, R4, 0x40))?;
    c.emit(0x82456, I::Alu(Alu::Cmp, R7, R4))?;
    c.emit(0x82458, I::Branch(Some(Condition::Eq), 0x8245e))?;
    c.emit(0x8245a, I::Push(0x20))?;
    c.emit(0x8245c, I::Pop(0x4))?;
    c.emit(0x8245e, I::Stack(true, R5, 0x34))?;
    c.emit(0x82460, I::Stack(true, R4, 0x44))?;
    c.emit(0x82462, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x82464, I::Stack(true, R4, 0x30))?;
    c.emit(0x82466, I::Stack(true, R3, 0x3c))?;
    c.emit(0x82468, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x8246a, I::Shift(Shift::Lsl, R3, R6, 19))?;
    c.emit(0x8246c, I::Compare(R2, 0))?;
    c.emit(0x8246e, I::Branch(Some(Condition::Ne), 0x8247a))?;
    c.emit(0x82470, I::Alu(Alu::Orr, R1, R3))?;
    c.emit(0x82472, I::Stack(true, R2, 0x38))?;
    c.emit(0x82474, I::Push(0x2))?;
    c.emit(0x82476, I::Pop(0x20))?;
    c.emit(0x82478, I::Branch(None, 0x8288e))?;
    c.emit(0x8247a, I::Stack(false, R3, 0xc))?;
    c.emit(0x8247c, I::Stack(false, R5, 0x18))?;
    c.emit(0x8247e, I::Mov(R6, 1))?;
    c.emit(0x82480, I::Mov(R2, 0))?;
    c.emit(0x82482, I::Stack(true, R3, 0x40))?;
    c.emit(0x82484, I::Stack(false, R4, 0x14))?;
    c.emit(0x82486, I::Alu(Alu::Cmp, R4, R3))?;
    c.emit(0x82488, I::Push(0x40))?;
    c.emit(0x8248a, I::Pop(0x10))?;
    c.emit(0x8248c, I::Branch(Some(Condition::Ls), 0x82492))?;
    c.emit(0x8248e, I::Push(0x4))?;
    c.emit(0x82490, I::Pop(0x10))?;
    c.emit(0x82492, I::Stack(true, R5, 0x18))?;
    c.emit(0x82494, I::Compare(R5, 0))?;
    c.emit(0x82496, I::Branch(Some(Condition::Eq), 0x8249c))?;
    c.emit(0x82498, I::Push(0x4))?;
    c.emit(0x8249a, I::Pop(0x40))?;
    c.emit(0x8249c, I::Stack(true, R2, 0x14))?;
    c.emit(0x8249e, I::Alu(Alu::Cmp, R2, R3))?;
    c.emit(0x824a0, I::Branch(Some(Condition::Eq), 0x824a6))?;
    c.emit(0x824a2, I::Push(0x10))?;
    c.emit(0x824a4, I::Pop(0x40))?;
    c.emit(0x824a6, I::Compare(R6, 0))?;
    c.emit(0x824a8, I::Branch(Some(Condition::Ne), 0x824b4))?;
    c.emit(0x824aa, I::Stack(true, R5, 0x30))?;
    c.emit(0x824ac, I::Stack(true, R1, 0xc))?;
    c.emit(0x824ae, I::Alu(Alu::Orr, R5, R1))?;
    c.emit(0x824b0, I::Stack(true, R2, 0x34))?;
    c.emit(0x824b2, I::Branch(None, 0x8288e))?;
    c.emit(0x824b4, I::Push(0x80))?;
    c.emit(0x824b6, I::Pop(0x4))?;
    c.emit(0x824b8, I::Alu(Alu::Eor, R2, R3))?;
    c.emit(0x824ba, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x824bc, I::Literal(R5, 0x828a0))?;
    c.emit(0x824be, I::Compare(R2, 0))?;
    c.emit(0x824c0, I::Push(0x8))?;
    c.emit(0x824c2, I::Pop(0x10))?;
    c.emit(0x824c4, I::Branch(Some(Condition::Ne), 0x824fe))?;
    c.emit(0x824c6, I::Mov(R2, 0))?;
    c.emit(0x824c8, I::Stack(true, R0, 0x44))?;
    c.emit(0x824ca, I::Small(false, R0, R0, 1))?;
    c.emit(0x824cc, I::Stack(true, R3, 0x3c))?;
    c.emit(0x824ce, I::Alu(Alu::Adc, R3, R2))?;
    c.emit(0x824d0, I::Stack(true, R2, 0x30))?;
    c.emit(0x824d2, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x824d4, I::Alu(Alu::Eor, R2, R1))?;
    c.emit(0x824d6, I::Push(0x4))?;
    c.emit(0x824d8, I::Pop(0x8))?;
    c.emit(0x824da, I::Stack(true, R2, 0x14))?;
    c.emit(0x824dc, I::Alu(Alu::Eor, R2, R4))?;
    c.emit(0x824de, I::Stack(true, R1, 0x18))?;
    c.emit(0x824e0, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x824e2, I::Compare(R1, 0))?;
    c.emit(0x824e4, I::Branch(Some(Condition::Eq), 0x824ea))?;
    c.emit(0x824e6, I::Push(0x8))?;
    c.emit(0x824e8, I::Pop(0x20))?;
    c.emit(0x824ea, I::Stack(true, R2, 0x34))?;
    c.emit(0x824ec, I::Alu(Alu::And, R2, R0))?;
    c.emit(0x824ee, I::Stack(true, R0, 0x38))?;
    c.emit(0x824f0, I::Alu(Alu::Eor, R2, R0))?;
    c.emit(0x824f2, I::Compare(R1, 0))?;
    c.emit(0x824f4, I::Branch(Some(Condition::Eq), 0x824f8))?;
    c.emit(0x824f6, I::Branch(None, 0x8288e))?;
    c.emit(0x824f8, I::Push(0x2))?;
    c.emit(0x824fa, I::Pop(0x4))?;
    c.emit(0x824fc, I::Branch(None, 0x8288e))?;
    c.emit(0x824fe, I::Stack(true, R3, 0x14))?;
    c.emit(0x82500, I::Push(0x8))?;
    c.emit(0x82502, I::Pop(0x2))?;
    c.emit(0x82504, I::Alu(Alu::Eor, R1, R4))?;
    c.emit(0x82506, I::Stack(true, R2, 0x18))?;
    c.emit(0x82508, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x8250a, I::Compare(R1, 0))?;
    c.emit(0x8250c, I::Branch(Some(Condition::Ne), 0x82514))?;
    c.emit(0x8250e, I::Stack(true, R2, 0x2c))?;
    c.emit(0x82510, I::Stack(true, R5, 0x28))?;
    c.emit(0x82512, I::Branch(None, 0x8288e))?;
    c.emit(0x82514, I::Push(0x1))?;
    c.emit(0x82516, I::Pop(0x2))?;
    c.emit(0x82518, I::Alu(Alu::Orr, R1, R7))?;
    c.emit(0x8251a, I::Compare(R1, 0))?;
    c.emit(0x8251c, I::Branch(Some(Condition::Ne), 0x82540))?;
    c.emit(0x8251e, I::Stack(true, R2, 0x18))?;
    c.emit(0x82520, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x82522, I::Compare(R2, 0))?;
    c.emit(0x82524, I::Stack(true, R1, 0x28))?;
    c.emit(0x82526, I::Branch(Some(Condition::Ne), 0x8252c))?;
    c.emit(0x82528, I::Push(0x20))?;
    c.emit(0x8252a, I::Pop(0x2))?;
    c.emit(0x8252c, I::Compare(R2, 0))?;
    c.emit(0x8252e, I::Stack(true, R0, 0x2c))?;
    c.emit(0x82530, I::Branch(Some(Condition::Ne), 0x82536))?;
    c.emit(0x82532, I::Push(0x4))?;
    c.emit(0x82534, I::Pop(0x1))?;
    c.emit(0x82536, I::Push(0x1))?;
    c.emit(0x82538, I::Pop(0x4))?;
    c.emit(0x8253a, I::Push(0x2))?;
    c.emit(0x8253c, I::Pop(0x20))?;
    c.emit(0x8253e, I::Branch(None, 0x8288e))?;
    c.emit(0x82540, I::Stack(true, R1, 0x18))?;
    c.emit(0x82542, I::Alu(Alu::Orr, R1, R3))?;
    c.emit(0x82544, I::Compare(R1, 0))?;
    c.emit(0x82546, I::Branch(Some(Condition::Ne), 0x82550))?;
    c.emit(0x82548, I::Stack(true, R5, 0x28))?;
    c.emit(0x8254a, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x8254c, I::Stack(true, R2, 0x2c))?;
    c.emit(0x8254e, I::Branch(None, 0x8288e))?;
    c.emit(0x82550, I::Mov(R4, 0))?;
    c.emit(0x82552, I::Stack(true, R1, 0x44))?;
    c.emit(0x82554, I::Small(false, R3, R1, 1))?;
    c.emit(0x82556, I::Stack(true, R5, 0x1c))?;
    c.emit(0x82558, I::Push(0x20))?;
    c.emit(0x8255a, I::Pop(0x4))?;
    c.emit(0x8255c, I::Alu(Alu::Adc, R2, R4))?;
    c.emit(0x8255e, I::Mov(R1, 1))?;
    c.emit(0x82560, I::Alu(Alu::Cmp, R0, R3))?;
    c.emit(0x82562, I::Push(0x2))?;
    c.emit(0x82564, I::Pop(0x1))?;
    c.emit(0x82566, I::Branch(Some(Condition::Hs), 0x8256c))?;
    c.emit(0x82568, I::Push(0x10))?;
    c.emit(0x8256a, I::Pop(0x1))?;
    c.emit(0x8256c, I::Alu(Alu::Cmp, R7, R2))?;
    c.emit(0x8256e, I::Stack(true, R6, 0x10))?;
    c.emit(0x82570, I::Branch(Some(Condition::Hs), 0x82576))?;
    c.emit(0x82572, I::Push(0x10))?;
    c.emit(0x82574, I::Pop(0x2))?;
    c.emit(0x82576, I::Alu(Alu::Cmp, R7, R2))?;
    c.emit(0x82578, I::Branch(Some(Condition::Eq), 0x8257e))?;
    c.emit(0x8257a, I::Push(0x2))?;
    c.emit(0x8257c, I::Pop(0x1))?;
    c.emit(0x8257e, I::Compare(R0, 0))?;
    c.emit(0x82580, I::Push(0x10))?;
    c.emit(0x82582, I::Pop(0x1))?;
    c.emit(0x82584, I::Branch(Some(Condition::Ne), 0x8258c))?;
    c.emit(0x82586, I::Address(R0, 80))?;
    c.emit(0x82588, I::Call(0x828b4))?;
    c.emit(0x8258c, I::Stack(false, R0, 0xc))?;
    c.emit(0x8258e, I::Stack(true, R0, 0x44))?;
    c.emit(0x82590, I::Small(false, R1, R0, 1))?;
    c.emit(0x82592, I::Push(0x20))?;
    c.emit(0x82594, I::Pop(0x4))?;
    c.emit(0x82596, I::Alu(Alu::Adc, R2, R4))?;
    c.emit(0x82598, I::Mov(R0, 1))?;
    c.emit(0x8259a, I::Stack(true, R3, 0x18))?;
    c.emit(0x8259c, I::Alu(Alu::Cmp, R3, R1))?;
    c.emit(0x8259e, I::Push(0x1))?;
    c.emit(0x825a0, I::Pop(0x2))?;
    c.emit(0x825a2, I::Branch(Some(Condition::Hs), 0x825a8))?;
    c.emit(0x825a4, I::Push(0x10))?;
    c.emit(0x825a6, I::Pop(0x2))?;
    c.emit(0x825a8, I::Stack(true, R3, 0x14))?;
    c.emit(0x825aa, I::Alu(Alu::Cmp, R3, R2))?;
    c.emit(0x825ac, I::Branch(Some(Condition::Hs), 0x825b2))?;
    c.emit(0x825ae, I::Push(0x10))?;
    c.emit(0x825b0, I::Pop(0x1))?;
    c.emit(0x825b2, I::Stack(true, R3, 0x14))?;
    c.emit(0x825b4, I::Alu(Alu::Cmp, R3, R2))?;
    c.emit(0x825b6, I::Branch(Some(Condition::Eq), 0x825bc))?;
    c.emit(0x825b8, I::Push(0x1))?;
    c.emit(0x825ba, I::Pop(0x2))?;
    c.emit(0x825bc, I::Compare(R1, 0))?;
    c.emit(0x825be, I::Branch(Some(Condition::Eq), 0x825d4))?;
    c.emit(0x825c0, I::Stack(true, R0, 0xc))?;
    c.emit(0x825c2, I::Branch(None, 0x825e2))?;
    c.pool(0x825c4, 0x000fffff)?;
    c.pool(0x825c8, 0x7fffffff)?;
    c.pool(0x825cc, 0x000007fd)?;
    c.pool(0x825d0, 0x7ff00000)?;
    c.emit(0x825d4, I::Address(R0, 72))?;
    c.emit(0x825d6, I::Call(0x828b4))?;
    c.emit(0x825da, I::Stack(true, R1, 0xc))?;
    c.emit(0x825dc, I::Arithmetic(true, R1, R1, R0))?;
    c.emit(0x825de, I::Push(0x2))?;
    c.emit(0x825e0, I::Pop(0x1))?;
    c.emit(0x825e2, I::Stack(false, R0, 0xc))?;
    c.emit(0x825e4, I::Stack(true, R0, 0x44))?;
    c.emit(0x825e6, I::Small(false, R1, R0, 1))?;
    c.emit(0x825e8, I::Push(0x1))?;
    c.emit(0x825ea, I::Pop(0x80))?;
    c.emit(0x825ec, I::Alu(Alu::Adc, R5, R6))?;
    c.emit(0x825ee, I::Stack(true, R2, 0x50))?;
    c.emit(0x825f0, I::Stack(false, R2, 0x38))?;
    c.emit(0x825f2, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x825f4, I::Stack(false, R2, 0x18))?;
    c.emit(0x825f6, I::Stack(true, R3, 0x54))?;
    c.emit(0x825f8, I::Stack(false, R3, 0x10))?;
    c.emit(0x825fa, I::Stack(false, R2, 0x50))?;
    c.emit(0x825fc, I::Push(0x8))?;
    c.emit(0x825fe, I::Pop(0x4))?;
    c.emit(0x82600, I::Alu(Alu::Orr, R2, R5))?;
    c.emit(0x82602, I::Stack(false, R2, 0x14))?;
    c.emit(0x82604, I::Stack(false, R2, 0x54))?;
    c.emit(0x82606, I::Stack(true, R2, 0x4c))?;
    c.emit(0x82608, I::Alu(Alu::Orr, R2, R5))?;
    c.emit(0x8260a, I::Stack(false, R2, 0x34))?;
    c.emit(0x8260c, I::Stack(true, R0, 0x48))?;
    c.emit(0x8260e, I::Stack(false, R0, 0x3c))?;
    c.emit(0x82610, I::Stack(false, R2, 0x4c))?;
    c.emit(0x82612, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x82614, I::Stack(false, R1, 0x30))?;
    c.emit(0x82616, I::Stack(false, R1, 0x48))?;
    c.emit(0x82618, I::Shift(Shift::Lsl, R0, R2, 11))?;
    c.emit(0x8261a, I::Shift(Shift::Lsr, R4, R1, 21))?;
    c.emit(0x8261c, I::Alu(Alu::Orr, R4, R0))?;
    c.emit(0x8261e, I::Literal(R0, 0x828a4))?;
    c.emit(0x82620, I::Push(0x40))?;
    c.emit(0x82622, I::Pop(0x20))?;
    c.emit(0x82624, I::Arithmetic(true, R2, R0, R4))?;
    c.emit(0x82626, I::Stack(false, R2, 0x8))?;
    c.emit(0x82628, I::Push(0x80))?;
    c.emit(0x8262a, I::Pop(0x40))?;
    c.emit(0x8262c, I::Alu(Alu::And, R4, R6))?;
    c.emit(0x8262e, I::Push(0x10))?;
    c.emit(0x82630, I::Pop(0x1))?;
    c.emit(0x82632, I::Push(0x20))?;
    c.emit(0x82634, I::Pop(0x2))?;
    c.emit(0x82636, I::Push(0x20))?;
    c.emit(0x82638, I::Pop(0x8))?;
    c.emit(0x8263a, I::Call(0x835b0))?;
    c.emit(0x8263e, I::Alu(Alu::Neg, R0, R1))?;
    c.emit(0x82640, I::Alu(Alu::And, R0, R6))?;
    c.emit(0x82642, I::Push(0x20))?;
    c.emit(0x82644, I::Pop(0x2))?;
    c.emit(0x82646, I::Stack(true, R2, 0x8))?;
    c.emit(0x82648, I::Push(0x20))?;
    c.emit(0x8264a, I::Pop(0x8))?;
    c.emit(0x8264c, I::Call(0x835b0))?;
    c.emit(0x82650, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x82652, I::Shift(Shift::Lsr, R6, R0, 31))?;
    c.emit(0x82654, I::Alu(Alu::Orr, R6, R1))?;
    c.emit(0x82656, I::Alu(Alu::And, R6, R7))?;
    c.emit(0x82658, I::Push(0x40))?;
    c.emit(0x8265a, I::Pop(0x1))?;
    c.emit(0x8265c, I::Push(0x20))?;
    c.emit(0x8265e, I::Pop(0x2))?;
    c.emit(0x82660, I::Push(0x10))?;
    c.emit(0x82662, I::Pop(0x4))?;
    c.emit(0x82664, I::Push(0x20))?;
    c.emit(0x82666, I::Pop(0x8))?;
    c.emit(0x82668, I::Call(0x835b0))?;
    c.emit(0x8266c, I::Alu(Alu::Neg, R0, R1))?;
    c.emit(0x8266e, I::Alu(Alu::And, R0, R7))?;
    c.emit(0x82670, I::Push(0x20))?;
    c.emit(0x82672, I::Pop(0x2))?;
    c.emit(0x82674, I::Push(0x40))?;
    c.emit(0x82676, I::Pop(0x4))?;
    c.emit(0x82678, I::Push(0x20))?;
    c.emit(0x8267a, I::Pop(0x8))?;
    c.emit(0x8267c, I::Call(0x835b0))?;
    c.emit(0x82680, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x82682, I::Shift(Shift::Lsr, R6, R0, 31))?;
    c.emit(0x82684, I::Alu(Alu::Orr, R6, R1))?;
    c.emit(0x82686, I::Alu(Alu::And, R6, R7))?;
    c.emit(0x82688, I::Push(0x40))?;
    c.emit(0x8268a, I::Pop(0x1))?;
    c.emit(0x8268c, I::Push(0x20))?;
    c.emit(0x8268e, I::Pop(0x2))?;
    c.emit(0x82690, I::Push(0x10))?;
    c.emit(0x82692, I::Pop(0x4))?;
    c.emit(0x82694, I::Push(0x20))?;
    c.emit(0x82696, I::Pop(0x8))?;
    c.emit(0x82698, I::Call(0x835b0))?;
    c.emit(0x8269c, I::Alu(Alu::Neg, R0, R1))?;
    c.emit(0x8269e, I::Alu(Alu::And, R0, R7))?;
    c.emit(0x826a0, I::Push(0x20))?;
    c.emit(0x826a2, I::Pop(0x2))?;
    c.emit(0x826a4, I::Push(0x40))?;
    c.emit(0x826a6, I::Pop(0x4))?;
    c.emit(0x826a8, I::Push(0x20))?;
    c.emit(0x826aa, I::Pop(0x8))?;
    c.emit(0x826ac, I::Call(0x835b0))?;
    c.emit(0x826b0, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x826b2, I::Shift(Shift::Lsr, R0, R0, 31))?;
    c.emit(0x826b4, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x826b6, I::Small(true, R6, R0, 1))?;
    c.emit(0x826b8, I::Push(0x10))?;
    c.emit(0x826ba, I::Pop(0x1))?;
    c.emit(0x826bc, I::Push(0x20))?;
    c.emit(0x826be, I::Pop(0x2))?;
    c.emit(0x826c0, I::Push(0x40))?;
    c.emit(0x826c2, I::Pop(0x4))?;
    c.emit(0x826c4, I::Push(0x20))?;
    c.emit(0x826c6, I::Pop(0x8))?;
    c.emit(0x826c8, I::Call(0x835b0))?;
    c.emit(0x826cc, I::Push(0x1))?;
    c.emit(0x826ce, I::Pop(0x10))?;
    c.emit(0x826d0, I::Push(0x2))?;
    c.emit(0x826d2, I::Pop(0x80))?;
    c.emit(0x826d4, I::Stack(true, R0, 0x3c))?;
    c.emit(0x826d6, I::Shift(Shift::Lsl, R2, R0, 11))?;
    c.emit(0x826d8, I::Push(0x40))?;
    c.emit(0x826da, I::Pop(0x1))?;
    c.emit(0x826dc, I::Push(0x20))?;
    c.emit(0x826de, I::Pop(0x2))?;
    c.emit(0x826e0, I::Push(0x20))?;
    c.emit(0x826e2, I::Pop(0x8))?;
    c.emit(0x826e4, I::Call(0x835b0))?;
    c.emit(0x826e8, I::Arithmetic(false, R0, R4, R1))?;
    c.emit(0x826ea, I::Alu(Alu::Adc, R7, R5))?;
    c.emit(0x826ec, I::Arithmetic(true, R4, R5, R0))?;
    c.emit(0x826ee, I::Push(0x20))?;
    c.emit(0x826f0, I::Pop(0x1))?;
    c.emit(0x826f2, I::Alu(Alu::Sbc, R0, R7))?;
    c.emit(0x826f4, I::Push(0x20))?;
    c.emit(0x826f6, I::Pop(0x2))?;
    c.emit(0x826f8, I::Push(0x40))?;
    c.emit(0x826fa, I::Pop(0x4))?;
    c.emit(0x826fc, I::Push(0x20))?;
    c.emit(0x826fe, I::Pop(0x8))?;
    c.emit(0x82700, I::Call(0x835b0))?;
    c.emit(0x82704, I::Stack(false, R0, 0x8))?;
    c.emit(0x82706, I::Stack(false, R1, 0x3c))?;
    c.emit(0x82708, I::Stack(true, R7, 0x44))?;
    c.emit(0x8270a, I::Alu(Alu::And, R4, R7))?;
    c.emit(0x8270c, I::Push(0x10))?;
    c.emit(0x8270e, I::Pop(0x1))?;
    c.emit(0x82710, I::Push(0x20))?;
    c.emit(0x82712, I::Pop(0x2))?;
    c.emit(0x82714, I::Push(0x40))?;
    c.emit(0x82716, I::Pop(0x4))?;
    c.emit(0x82718, I::Push(0x20))?;
    c.emit(0x8271a, I::Pop(0x8))?;
    c.emit(0x8271c, I::Call(0x835b0))?;
    c.emit(0x82720, I::Stack(true, R0, 0x8))?;
    c.emit(0x82722, I::Arithmetic(false, R0, R0, R1))?;
    c.emit(0x82724, I::Stack(true, R1, 0x3c))?;
    c.emit(0x82726, I::Alu(Alu::Adc, R1, R5))?;
    c.emit(0x82728, I::Small(true, R2, R0, 2))?;
    c.emit(0x8272a, I::Alu(Alu::Adc, R1, R7))?;
    c.emit(0x8272c, I::Stack(false, R1, 0x3c))?;
    c.emit(0x8272e, I::Alu(Alu::And, R2, R7))?;
    c.emit(0x82730, I::Push(0x4))?;
    c.emit(0x82732, I::Pop(0x1))?;
    c.emit(0x82734, I::Stack(false, R0, 0x8))?;
    c.emit(0x82736, I::Small(true, R2, R7, 3))?;
    c.emit(0x82738, I::Stack(false, R2, 0x4))?;
    c.emit(0x8273a, I::Push(0x20))?;
    c.emit(0x8273c, I::Pop(0x10))?;
    c.emit(0x8273e, I::Alu(Alu::Adc, R4, R7))?;
    c.emit(0x82740, I::Stack(true, R1, 0x38))?;
    c.emit(0x82742, I::Shift(Shift::Lsl, R7, R1, 2))?;
    c.emit(0x82744, I::Alu(Alu::And, R7, R2))?;
    c.emit(0x82746, I::Shift(Shift::Lsr, R2, R1, 30))?;
    c.emit(0x82748, I::Stack(true, R1, 0x10))?;
    c.emit(0x8274a, I::Shift(Shift::Lsl, R6, R1, 2))?;
    c.emit(0x8274c, I::Alu(Alu::Orr, R6, R2))?;
    c.emit(0x8274e, I::Alu(Alu::And, R6, R4))?;
    c.emit(0x82750, I::Push(0x20))?;
    c.emit(0x82752, I::Pop(0x2))?;
    c.emit(0x82754, I::Push(0x80))?;
    c.emit(0x82756, I::Pop(0x4))?;
    c.emit(0x82758, I::Push(0x40))?;
    c.emit(0x8275a, I::Pop(0x8))?;
    c.emit(0x8275c, I::Call(0x835b0))?;
    c.emit(0x82760, I::Stack(false, R1, 0x0))?;
    c.emit(0x82762, I::Stack(true, R0, 0x3c))?;
    c.emit(0x82764, I::Push(0x20))?;
    c.emit(0x82766, I::Pop(0x2))?;
    c.emit(0x82768, I::Push(0x80))?;
    c.emit(0x8276a, I::Pop(0x4))?;
    c.emit(0x8276c, I::Push(0x40))?;
    c.emit(0x8276e, I::Pop(0x8))?;
    c.emit(0x82770, I::Call(0x835b0))?;
    c.emit(0x82774, I::Stack(false, R1, 0x10))?;
    c.emit(0x82776, I::Stack(true, R2, 0x4))?;
    c.emit(0x82778, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x8277a, I::Alu(Alu::And, R4, R1))?;
    c.emit(0x8277c, I::Stack(true, R1, 0x0))?;
    c.emit(0x8277e, I::Arithmetic(false, R0, R1, R0))?;
    c.emit(0x82780, I::Stack(false, R0, 0x4))?;
    c.emit(0x82782, I::Alu(Alu::Adc, R4, R5))?;
    c.emit(0x82784, I::Stack(true, R0, 0x14))?;
    c.emit(0x82786, I::Shift(Shift::Lsl, R0, R0, 2))?;
    c.emit(0x82788, I::Stack(true, R1, 0x18))?;
    c.emit(0x8278a, I::Shift(Shift::Lsr, R7, R1, 30))?;
    c.emit(0x8278c, I::Alu(Alu::Orr, R7, R0))?;
    c.emit(0x8278e, I::Stack(true, R6, 0x44))?;
    c.emit(0x82790, I::Alu(Alu::And, R7, R6))?;
    c.emit(0x82792, I::Stack(true, R0, 0x8))?;
    c.emit(0x82794, I::Push(0x20))?;
    c.emit(0x82796, I::Pop(0x2))?;
    c.emit(0x82798, I::Push(0x80))?;
    c.emit(0x8279a, I::Pop(0x4))?;
    c.emit(0x8279c, I::Push(0x20))?;
    c.emit(0x8279e, I::Pop(0x8))?;
    c.emit(0x827a0, I::Call(0x835b0))?;
    c.emit(0x827a4, I::Stack(false, R1, 0x18))?;
    c.emit(0x827a6, I::Alu(Alu::And, R0, R6))?;
    c.emit(0x827a8, I::Stack(true, R1, 0x4))?;
    c.emit(0x827aa, I::Arithmetic(false, R0, R1, R0))?;
    c.emit(0x827ac, I::Alu(Alu::Adc, R4, R5))?;
    c.emit(0x827ae, I::Stack(true, R0, 0x3c))?;
    c.emit(0x827b0, I::Push(0x20))?;
    c.emit(0x827b2, I::Pop(0x2))?;
    c.emit(0x827b4, I::Push(0x80))?;
    c.emit(0x827b6, I::Pop(0x4))?;
    c.emit(0x827b8, I::Push(0x20))?;
    c.emit(0x827ba, I::Pop(0x8))?;
    c.emit(0x827bc, I::Call(0x835b0))?;
    c.emit(0x827c0, I::Stack(true, R3, 0x34))?;
    c.emit(0x827c2, I::Stack(true, R2, 0x30))?;
    c.emit(0x827c4, I::Push(0x2))?;
    c.emit(0x827c6, I::Pop(0x40))?;
    c.emit(0x827c8, I::Stack(true, R1, 0x18))?;
    c.emit(0x827ca, I::Arithmetic(false, R0, R1, R0))?;
    c.emit(0x827cc, I::Alu(Alu::Adc, R6, R5))?;
    c.emit(0x827ce, I::Stack(true, R1, 0x10))?;
    c.emit(0x827d0, I::Arithmetic(false, R0, R0, R1))?;
    c.emit(0x827d2, I::Alu(Alu::Adc, R6, R5))?;
    c.emit(0x827d4, I::Arithmetic(false, R4, R0, R4))?;
    c.emit(0x827d6, I::Alu(Alu::Adc, R6, R5))?;
    c.emit(0x827d8, I::Shift(Shift::Lsr, R0, R6, 21))?;
    c.emit(0x827da, I::Stack(true, R1, 0x20))?;
    c.emit(0x827dc, I::Stack(true, R5, 0x24))?;
    c.emit(0x827de, I::Arithmetic(true, R1, R1, R5))?;
    c.emit(0x827e0, I::Stack(true, R5, 0xc))?;
    c.emit(0x827e2, I::Arithmetic(false, R5, R5, R1))?;
    c.emit(0x827e4, I::Literal(R7, 0x828a8))?;
    c.emit(0x827e6, I::Compare(R0, 0))?;
    c.emit(0x827e8, I::Branch(Some(Condition::Ne), 0x82804))?;
    c.emit(0x827ea, I::Push(0x10))?;
    c.emit(0x827ec, I::Pop(0x1))?;
    c.emit(0x827ee, I::Push(0x40))?;
    c.emit(0x827f0, I::Pop(0x2))?;
    c.emit(0x827f2, I::Call(0x835b0))?;
    c.emit(0x827f6, I::Stack(true, R2, 0x38))?;
    c.emit(0x827f8, I::Shift(Shift::Lsl, R2, R2, 21))?;
    c.emit(0x827fa, I::Mov(R3, 0))?;
    c.emit(0x827fc, I::Arithmetic(true, R0, R3, R0))?;
    c.emit(0x827fe, I::Alu(Alu::Sbc, R2, R1))?;
    c.emit(0x82800, I::Small(true, R5, R5, 1))?;
    c.emit(0x82802, I::Branch(None, 0x82822))?;
    c.emit(0x82804, I::Shift(Shift::Lsr, R0, R4, 1))?;
    c.emit(0x82806, I::Shift(Shift::Lsl, R4, R6, 31))?;
    c.emit(0x82808, I::Alu(Alu::Orr, R4, R0))?;
    c.emit(0x8280a, I::Shift(Shift::Lsr, R6, R6, 1))?;
    c.emit(0x8280c, I::Push(0x10))?;
    c.emit(0x8280e, I::Pop(0x1))?;
    c.emit(0x82810, I::Push(0x40))?;
    c.emit(0x82812, I::Pop(0x2))?;
    c.emit(0x82814, I::Call(0x835b0))?;
    c.emit(0x82818, I::Stack(true, R2, 0x38))?;
    c.emit(0x8281a, I::Shift(Shift::Lsl, R2, R2, 20))?;
    c.emit(0x8281c, I::Mov(R3, 0))?;
    c.emit(0x8281e, I::Arithmetic(true, R0, R3, R0))?;
    c.emit(0x82820, I::Alu(Alu::Sbc, R2, R1))?;
    c.emit(0x82822, I::Arithmetic(false, R1, R5, R7))?;
    c.emit(0x82824, I::Alu(Alu::Cmp, R5, R7))?;
    c.emit(0x82826, I::Branch(Some(Condition::Le), 0x82832))?;
    c.emit(0x82828, I::Stack(true, R5, 0x28))?;
    c.emit(0x8282a, I::Stack(true, R1, 0x40))?;
    c.emit(0x8282c, I::Alu(Alu::Orr, R5, R1))?;
    c.emit(0x8282e, I::Stack(true, R2, 0x2c))?;
    c.emit(0x82830, I::Branch(None, 0x8288e))?;
    c.emit(0x82832, I::Literal(R3, 0x828ac))?;
    c.emit(0x82834, I::Alu(Alu::Cmp, R5, R3))?;
    c.emit(0x82836, I::Branch(Some(Condition::Ge), 0x8283a))?;
    c.emit(0x82838, I::Branch(None, 0x8250e))?;
    c.emit(0x8283a, I::Shift(Shift::Lsl, R3, R0, 1))?;
    c.emit(0x8283c, I::Mov(R5, 1))?;
    c.emit(0x8283e, I::Mov(R7, 0))?;
    c.emit(0x82840, I::Stack(false, R7, 0x40))?;
    c.emit(0x82842, I::Stack(true, R7, 0x30))?;
    c.emit(0x82844, I::Alu(Alu::Cmp, R3, R7))?;
    c.emit(0x82846, I::Push(0x20))?;
    c.emit(0x82848, I::Pop(0x8))?;
    c.emit(0x8284a, I::Branch(Some(Condition::Hi), 0x8284e))?;
    c.emit(0x8284c, I::Stack(true, R3, 0x40))?;
    c.emit(0x8284e, I::Shift(Shift::Lsr, R7, R0, 31))?;
    c.emit(0x82850, I::Shift(Shift::Lsl, R0, R2, 1))?;
    c.emit(0x82852, I::Alu(Alu::Orr, R0, R7))?;
    c.emit(0x82854, I::Stack(true, R2, 0x34))?;
    c.emit(0x82856, I::Alu(Alu::Cmp, R0, R2))?;
    c.emit(0x82858, I::Branch(Some(Condition::Hi), 0x8285c))?;
    c.emit(0x8285a, I::Stack(true, R5, 0x40))?;
    c.emit(0x8285c, I::Alu(Alu::Cmp, R0, R2))?;
    c.emit(0x8285e, I::Branch(Some(Condition::Eq), 0x82864))?;
    c.emit(0x82860, I::Push(0x20))?;
    c.emit(0x82862, I::Pop(0x8))?;
    c.emit(0x82864, I::Stack(true, R0, 0x44))?;
    c.emit(0x82866, I::Alu(Alu::And, R4, R0))?;
    c.emit(0x82868, I::Stack(true, R0, 0x1c))?;
    c.emit(0x8286a, I::Alu(Alu::And, R6, R0))?;
    c.emit(0x8286c, I::Shift(Shift::Lsl, R0, R1, 20))?;
    c.emit(0x8286e, I::Alu(Alu::Orr, R0, R6))?;
    c.emit(0x82870, I::Small(false, R2, R4, 1))?;
    c.emit(0x82872, I::Stack(true, R5, 0x40))?;
    c.emit(0x82874, I::Alu(Alu::Adc, R5, R0))?;
    c.emit(0x82876, I::Compare(R3, 0))?;
    c.emit(0x82878, I::Branch(Some(Condition::Ne), 0x8287e))?;
    c.emit(0x8287a, I::Push(0x1))?;
    c.emit(0x8287c, I::Pop(0x20))?;
    c.emit(0x8287e, I::Stack(true, R0, 0x28))?;
    c.emit(0x82880, I::Alu(Alu::Orr, R5, R0))?;
    c.emit(0x82882, I::Compare(R3, 0))?;
    c.emit(0x82884, I::Branch(Some(Condition::Ne), 0x8288a))?;
    c.emit(0x82886, I::Push(0x10))?;
    c.emit(0x82888, I::Pop(0x4))?;
    c.emit(0x8288a, I::Stack(true, R0, 0x2c))?;
    c.emit(0x8288c, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x8288e, I::Push(0x4))?;
    c.emit(0x82890, I::Pop(0x1))?;
    c.emit(0x82892, I::Push(0x20))?;
    c.emit(0x82894, I::Pop(0x2))?;
    c.emit(0x82896, I::Call(0x828b0))?;
    c.emit(0x8289a, I::StackAdjust(false, 92))?;
    c.emit(0x8289c, I::Pop(0x80f0))?;
    c.emit(0x8289e, I::Align)?;
    c.pool(0x828a0, 0x7ff80000)?;
    c.pool(0x828a4, 0x7504f333)?;
    c.pool(0x828a8, 0x000003ff)?;
    c.pool(0x828ac, 0xfffffc02)?;
    c.emit(0x828b0, I::BxLr)?;
    c.emit(0x828b2, I::ZeroAlign)?;
    c.emit(0x828b4, I::Push(0x40f0))?;
    c.emit(0x828b6, I::LoadMultiple(R0, 0x6))?;
    c.emit(0x828b8, I::Mov(R3, 32))?;
    c.emit(0x828ba, I::Subtract(R0, 8))?;
    c.emit(0x828bc, I::Compare(R2, 0))?;
    c.emit(0x828be, I::Branch(Some(Condition::Eq), 0x828fc))?;
    c.emit(0x828c0, I::Compare(R2, 0))?;
    c.emit(0x828c2, I::Branch(Some(Condition::Eq), 0x82938))?;
    c.emit(0x828c4, I::Shift(Shift::Lsr, R3, R2, 1))?;
    c.emit(0x828c6, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x828c8, I::Shift(Shift::Lsr, R4, R3, 2))?;
    c.emit(0x828ca, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x828cc, I::Shift(Shift::Lsr, R3, R4, 4))?;
    c.emit(0x828ce, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x828d0, I::Shift(Shift::Lsr, R4, R3, 8))?;
    c.emit(0x828d2, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x828d4, I::Shift(Shift::Lsr, R3, R4, 16))?;
    c.emit(0x828d6, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x828d8, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x828da, I::Shift(Shift::Lsr, R4, R3, 1))?;
    c.emit(0x828dc, I::Literal(R5, 0x82978))?;
    c.emit(0x828de, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x828e0, I::Arithmetic(true, R3, R3, R5))?;
    c.emit(0x828e2, I::Literal(R4, 0x8297c))?;
    c.emit(0x828e4, I::Shift(Shift::Lsr, R5, R3, 2))?;
    c.emit(0x828e6, I::Alu(Alu::And, R3, R4))?;
    c.emit(0x828e8, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x828ea, I::Arithmetic(false, R3, R3, R5))?;
    c.emit(0x828ec, I::Shift(Shift::Lsr, R4, R3, 4))?;
    c.emit(0x828ee, I::Arithmetic(false, R3, R3, R4))?;
    c.emit(0x828f0, I::Literal(R4, 0x82980))?;
    c.emit(0x828f2, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x828f4, I::Literal(R3, 0x82984))?;
    c.emit(0x828f6, I::Alu(Alu::Mul, R3, R4))?;
    c.emit(0x828f8, I::Shift(Shift::Lsr, R3, R3, 24))?;
    c.emit(0x828fa, I::Branch(None, 0x82938))?;
    c.emit(0x828fc, I::Compare(R1, 0))?;
    c.emit(0x828fe, I::Branch(Some(Condition::Eq), 0x82936))?;
    c.emit(0x82900, I::Shift(Shift::Lsr, R3, R1, 1))?;
    c.emit(0x82902, I::Alu(Alu::Orr, R3, R1))?;
    c.emit(0x82904, I::Shift(Shift::Lsr, R4, R3, 2))?;
    c.emit(0x82906, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x82908, I::Shift(Shift::Lsr, R3, R4, 4))?;
    c.emit(0x8290a, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x8290c, I::Shift(Shift::Lsr, R4, R3, 8))?;
    c.emit(0x8290e, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x82910, I::Shift(Shift::Lsr, R3, R4, 16))?;
    c.emit(0x82912, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x82914, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x82916, I::Shift(Shift::Lsr, R4, R3, 1))?;
    c.emit(0x82918, I::Literal(R5, 0x82978))?;
    c.emit(0x8291a, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x8291c, I::Arithmetic(true, R3, R3, R5))?;
    c.emit(0x8291e, I::Literal(R4, 0x8297c))?;
    c.emit(0x82920, I::Shift(Shift::Lsr, R5, R3, 2))?;
    c.emit(0x82922, I::Alu(Alu::And, R3, R4))?;
    c.emit(0x82924, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x82926, I::Arithmetic(false, R3, R3, R5))?;
    c.emit(0x82928, I::Shift(Shift::Lsr, R4, R3, 4))?;
    c.emit(0x8292a, I::Arithmetic(false, R3, R3, R4))?;
    c.emit(0x8292c, I::Literal(R4, 0x82980))?;
    c.emit(0x8292e, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x82930, I::Literal(R3, 0x82984))?;
    c.emit(0x82932, I::Alu(Alu::Mul, R3, R4))?;
    c.emit(0x82934, I::Shift(Shift::Lsr, R3, R3, 24))?;
    c.emit(0x82936, I::Add(R3, 32))?;
    c.emit(0x82938, I::Push(0x8))?;
    c.emit(0x8293a, I::Pop(0x20))?;
    c.emit(0x8293c, I::Subtract(R5, 11))?;
    c.emit(0x8293e, I::Push(0x2))?;
    c.emit(0x82940, I::Pop(0x40))?;
    c.emit(0x82942, I::Alu(Alu::Lsl, R6, R5))?;
    c.emit(0x82944, I::Mov(R7, 0))?;
    c.emit(0x82946, I::Push(0x8))?;
    c.emit(0x82948, I::Pop(0x10))?;
    c.emit(0x8294a, I::Subtract(R4, 43))?;
    c.emit(0x8294c, I::Compare(R4, 0))?;
    c.emit(0x8294e, I::Branch(Some(Condition::Ge), 0x82954))?;
    c.emit(0x82950, I::Push(0x40))?;
    c.emit(0x82952, I::Pop(0x80))?;
    c.emit(0x82954, I::Memory(false, R7, R0, 0))?;
    c.emit(0x82956, I::Alu(Alu::Lsl, R2, R5))?;
    c.emit(0x82958, I::Mov(R5, 43))?;
    c.emit(0x8295a, I::Arithmetic(true, R6, R5, R3))?;
    c.emit(0x8295c, I::Push(0x2))?;
    c.emit(0x8295e, I::Pop(0x20))?;
    c.emit(0x82960, I::Alu(Alu::Lsr, R5, R6))?;
    c.emit(0x82962, I::Alu(Alu::Orr, R5, R2))?;
    c.emit(0x82964, I::Alu(Alu::Lsl, R1, R4))?;
    c.emit(0x82966, I::Compare(R4, 0))?;
    c.emit(0x82968, I::Branch(Some(Condition::Ge), 0x8296e))?;
    c.emit(0x8296a, I::Push(0x20))?;
    c.emit(0x8296c, I::Pop(0x2))?;
    c.emit(0x8296e, I::Memory(false, R1, R0, 4))?;
    c.emit(0x82970, I::Mov(R0, 12))?;
    c.emit(0x82972, I::Arithmetic(true, R0, R0, R3))?;
    c.emit(0x82974, I::Pop(0x80f0))?;
    c.emit(0x82976, I::Align)?;
    c.pool(0x82978, 0x55555555)?;
    c.pool(0x8297c, 0x33333333)?;
    c.pool(0x82980, 0x0f0f0f0f)?;
    c.pool(0x82984, 0x01010101)?;
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
        assert_eq!(bytes.len(), 1496);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "80f982a935f1072b30a3a18cc45d088377aeecd1911c379bf5a2a18fbef90dda");
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        for at in ALIGNMENTS { assert_eq!(half(&bytes, at), 0x46c0); }
        assert_eq!(half(&bytes, 0x828b2), 0x0000);
        for (at, slot) in [(533438, 533956), (533462, 533960), (533500, 533964), (533518, 533968), (533692, 534688), (534046, 534692), (534500, 534696), (534578, 534700), (534748, 534904), (534754, 534908), (534768, 534912), (534772, 534916), (534808, 534904), (534814, 534908), (534828, 534912), (534832, 534916)] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn independently_decoded_control_flow() {
        let bytes = fragment().unwrap();
        for (at, target, conditional) in [(0x82418, 0x82426, true), (0x82422, 0x82426, true), (0x82424, 0x825e2, false), (0x8243c, 0x82442, true), (0x8244e, 0x82454, true), (0x82458, 0x8245e, true), (0x8246e, 0x8247a, true), (0x82478, 0x8288e, false), (0x8248c, 0x82492, true), (0x82496, 0x8249c, true), (0x824a0, 0x824a6, true), (0x824a8, 0x824b4, true), (0x824b2, 0x8288e, false), (0x824c4, 0x824fe, true), (0x824e4, 0x824ea, true), (0x824f4, 0x824f8, true), (0x824f6, 0x8288e, false), (0x824fc, 0x8288e, false), (0x8250c, 0x82514, true), (0x82512, 0x8288e, false), (0x8251c, 0x82540, true), (0x82526, 0x8252c, true), (0x82530, 0x82536, true), (0x8253e, 0x8288e, false), (0x82546, 0x82550, true), (0x8254e, 0x8288e, false), (0x82566, 0x8256c, true), (0x82570, 0x82576, true), (0x82578, 0x8257e, true), (0x82584, 0x8258c, true), (0x825a2, 0x825a8, true), (0x825ac, 0x825b2, true), (0x825b6, 0x825bc, true), (0x825be, 0x825d4, true), (0x825c2, 0x825e2, false), (0x827e8, 0x82804, true), (0x82802, 0x82822, false), (0x82826, 0x82832, true), (0x82830, 0x8288e, false), (0x82836, 0x8283a, true), (0x82838, 0x8250e, false), (0x8284a, 0x8284e, true), (0x82858, 0x8285c, true), (0x8285e, 0x82864, true), (0x82878, 0x8287e, true), (0x82884, 0x8288a, true), (0x828be, 0x828fc, true), (0x828c2, 0x82938, true), (0x828fa, 0x82938, false), (0x828fe, 0x82936, true), (0x8294e, 0x82954, true), (0x82968, 0x8296e, true)] {
            let word = half(&bytes, at);
            assert_eq!(word & (if conditional { 0xf000 } else { 0xf800 }),
                if conditional { 0xd000 } else { 0xe000 });
            let shift = if conditional { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [(0x82588, 0x828b4), (0x825d6, 0x828b4), (0x8263a, 0x835b0), (0x8264c, 0x835b0), (0x82668, 0x835b0), (0x8267c, 0x835b0), (0x82698, 0x835b0), (0x826ac, 0x835b0), (0x826c8, 0x835b0), (0x826e4, 0x835b0), (0x82700, 0x835b0), (0x8271c, 0x835b0), (0x8275c, 0x835b0), (0x82770, 0x835b0), (0x827a0, 0x835b0), (0x827bc, 0x835b0), (0x827f2, 0x835b0), (0x82814, 0x835b0), (0x82896, 0x828b0)] {
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
        assert_eq!(I::Alu(Alu::Sbc, R0, R7).encode(0x826f2).unwrap(), bytes[0x826f2 - START..0x826f4 - START]);
        assert_eq!(I::Alu(Alu::Neg, R0, R1).encode(0x8263e).unwrap(), bytes[0x8263e - START..0x82640 - START]);
        assert_eq!(I::LoadMultiple(R0, 6).encode(0x828b6).unwrap(), bytes[0x828b6 - START..0x828b8 - START]);
        assert_eq!(I::Shift(Shift::Lsl, R0, R1, 0).encode(START).unwrap(), 0x0008u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
        for (start, end) in [(START, 0x825c4), (0x825d4, 0x8289e), (0x828b0, 0x828b2), (0x828b4, 0x82976)] {
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
        for instruction in [I::LoadMultiple(R0, 0), I::LoadMultiple(R0, 1), I::LoadMultiple(R0, 256), I::Add(R0, 256), I::Push(0), I::Push(0x8000), I::Pop(0x4000),
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
        for at in [0x825c2, 0x825c4, 0x8289c, 0x8289e, 0x828b2, 0x82976, 0x82978, END] {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            let instruction = if at == 0x825c2 || at == 0x8289c {
                I::Call(0x828b4)
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
            "80f982a935f1072b30a3a18cc45d088377aeecd1911c379bf5a2a18fbef90dda");
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
