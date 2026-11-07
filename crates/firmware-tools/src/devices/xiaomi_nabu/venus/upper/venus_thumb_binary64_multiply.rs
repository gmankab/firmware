pub const START: usize = 0x83018;
pub const END: usize = 0x83608;
pub const CODE_BYTES: usize = 1472;
pub const DATA_BYTES: usize = 44;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 4;
const ALIGNMENTS: [usize; 2] = [0x83352, 0x8359e];
const POOLS: [(usize, u32); 11] = [
    (0x83354, 0x000fffff),
    (0x83358, 0x000007fd),
    (0x8335c, 0xfffff800),
    (0x83360, 0x7ff00000),
    (0x834d4, 0x7ff80000),
    (0x834d8, 0xfffffc01),
    (0x835a0, 0x55555555),
    (0x835a4, 0x33333333),
    (0x835a8, 0x0f0f0f0f),
    (0x835ac, 0x01010101),
    (0x83604, 0x0000ffff),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Adc, Cmp, Orr, Mul, Mvn }
#[derive(Clone, Copy, Debug)]
enum Shift { Lsl, Lsr }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Hs = 2, Hi = 8, Ls = 9, Ge = 10, Le = 13 }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(bool, u16),
    Stack(bool, R, u16), Address(R, u16), Literal(R, usize),
    Alu(Alu, R, R), Shift(Shift, R, R, u8),
    Add(R, u16), Mov(R, u16), Compare(R, u16), Subtract(R, u16),
    Small(bool, R, R, u8), Arithmetic(bool, R, R, R),
    Branch(Option<Condition>, usize), Call(usize), LoadMultiple(R, u16), Memory(bool, R, R, u16), Align,
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
                    Alu::Adc => 5, Alu::Cmp => 10,
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
        checked(matches!(instruction, I::Align) == ALIGNMENTS.contains(&at))?;
        checked(!ALIGNMENTS.iter().any(|&slot| at < slot && at + bytes.len() > slot))?;
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
    c.emit(0x83018, I::Push(0x40f0))?;
    c.emit(0x8301a, I::StackAdjust(true, 92))?;
    c.emit(0x8301c, I::Push(0x2))?;
    c.emit(0x8301e, I::Pop(0x20))?;
    c.emit(0x83020, I::Stack(false, R0, 60))?;
    c.emit(0x83022, I::Stack(false, R0, 80))?;
    c.emit(0x83024, I::Literal(R1, 0x83354))?;
    c.emit(0x83026, I::Push(0x20))?;
    c.emit(0x83028, I::Pop(0x1))?;
    c.emit(0x8302a, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x8302c, I::Stack(false, R0, 84))?;
    c.emit(0x8302e, I::Push(0x8))?;
    c.emit(0x83030, I::Pop(0x1))?;
    c.emit(0x83032, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x83034, I::Stack(false, R0, 76))?;
    c.emit(0x83036, I::Stack(false, R2, 72))?;
    c.emit(0x83038, I::Push(0x8))?;
    c.emit(0x8303a, I::Pop(0x1))?;
    c.emit(0x8303c, I::Alu(Alu::Eor, R0, R5))?;
    c.emit(0x8303e, I::Mov(R4, 1))?;
    c.emit(0x83040, I::Stack(false, R4, 24))?;
    c.emit(0x83042, I::Shift(Shift::Lsl, R4, R4, 31))?;
    c.emit(0x83044, I::Alu(Alu::And, R0, R4))?;
    c.emit(0x83046, I::Stack(false, R0, 28))?;
    c.emit(0x83048, I::Literal(R7, 0x83358))?;
    c.emit(0x8304a, I::Small(false, R0, R7, 2))?;
    c.emit(0x8304c, I::Stack(false, R3, 56))?;
    c.emit(0x8304e, I::Shift(Shift::Lsr, R3, R3, 20))?;
    c.emit(0x83050, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x83052, I::Stack(false, R5, 44))?;
    c.emit(0x83054, I::Shift(Shift::Lsr, R5, R5, 20))?;
    c.emit(0x83056, I::Alu(Alu::And, R5, R0))?;
    c.emit(0x83058, I::Stack(false, R5, 12))?;
    c.emit(0x8305a, I::Small(true, R0, R5, 1))?;
    c.emit(0x8305c, I::Mov(R5, 0))?;
    c.emit(0x8305e, I::Alu(Alu::Mvn, R6, R5))?;
    c.emit(0x83060, I::Stack(false, R6, 40))?;
    c.emit(0x83062, I::Literal(R6, 0x8335c))?;
    c.emit(0x83064, I::Stack(false, R6, 48))?;
    c.emit(0x83066, I::Literal(R6, 0x83360))?;
    c.emit(0x83068, I::Stack(false, R6, 64))?;
    c.emit(0x8306a, I::Push(0x2))?;
    c.emit(0x8306c, I::Pop(0x40))?;
    c.emit(0x8306e, I::Alu(Alu::Cmp, R0, R7))?;
    c.emit(0x83070, I::Stack(false, R4, 32))?;
    c.emit(0x83072, I::Stack(false, R6, 8))?;
    c.emit(0x83074, I::Stack(false, R5, 68))?;
    c.emit(0x83076, I::Stack(false, R7, 20))?;
    c.emit(0x83078, I::Stack(false, R3, 16))?;
    c.emit(0x8307a, I::Branch(Some(Condition::Hi), 0x83088))?;
    c.emit(0x8307c, I::Small(true, R0, R3, 1))?;
    c.emit(0x8307e, I::Alu(Alu::Cmp, R0, R7))?;
    c.emit(0x83080, I::Push(0x20))?;
    c.emit(0x83082, I::Pop(0x80))?;
    c.emit(0x83084, I::Branch(Some(Condition::Hi), 0x83088))?;
    c.emit(0x83086, I::Branch(None, 0x83222))?;
    c.emit(0x83088, I::Stack(false, R2, 36))?;
    c.emit(0x8308a, I::Mov(R2, 0))?;
    c.emit(0x8308c, I::Alu(Alu::Mvn, R0, R2))?;
    c.emit(0x8308e, I::Small(true, R7, R2, 1))?;
    c.emit(0x83090, I::Alu(Alu::Adc, R0, R4))?;
    c.emit(0x83092, I::Stack(true, R1, 44))?;
    c.emit(0x83094, I::Push(0x2))?;
    c.emit(0x83096, I::Pop(0x10))?;
    c.emit(0x83098, I::Push(0x1))?;
    c.emit(0x8309a, I::Pop(0x8))?;
    c.emit(0x8309c, I::Alu(Alu::And, R4, R0))?;
    c.emit(0x8309e, I::Mov(R5, 1))?;
    c.emit(0x830a0, I::Stack(true, R0, 64))?;
    c.emit(0x830a2, I::Stack(false, R4, 52))?;
    c.emit(0x830a4, I::Alu(Alu::Cmp, R4, R0))?;
    c.emit(0x830a6, I::Push(0x20))?;
    c.emit(0x830a8, I::Pop(0x10))?;
    c.emit(0x830aa, I::Branch(Some(Condition::Ls), 0x830b0))?;
    c.emit(0x830ac, I::Push(0x4))?;
    c.emit(0x830ae, I::Pop(0x10))?;
    c.emit(0x830b0, I::Stack(true, R0, 60))?;
    c.emit(0x830b2, I::Push(0x80))?;
    c.emit(0x830b4, I::Pop(0x40))?;
    c.emit(0x830b6, I::Alu(Alu::And, R0, R7))?;
    c.emit(0x830b8, I::Stack(false, R0, 4))?;
    c.emit(0x830ba, I::Compare(R0, 0))?;
    c.emit(0x830bc, I::Push(0x20))?;
    c.emit(0x830be, I::Pop(0x1))?;
    c.emit(0x830c0, I::Branch(Some(Condition::Eq), 0x830c6))?;
    c.emit(0x830c2, I::Push(0x4))?;
    c.emit(0x830c4, I::Pop(0x1))?;
    c.emit(0x830c6, I::Stack(true, R2, 64))?;
    c.emit(0x830c8, I::Stack(true, R7, 52))?;
    c.emit(0x830ca, I::Alu(Alu::Cmp, R7, R2))?;
    c.emit(0x830cc, I::Push(0x8))?;
    c.emit(0x830ce, I::Pop(0x80))?;
    c.emit(0x830d0, I::Branch(Some(Condition::Eq), 0x830d6))?;
    c.emit(0x830d2, I::Push(0x10))?;
    c.emit(0x830d4, I::Pop(0x1))?;
    c.emit(0x830d6, I::Stack(true, R2, 56))?;
    c.emit(0x830d8, I::Alu(Alu::And, R7, R2))?;
    c.emit(0x830da, I::Stack(true, R3, 36))?;
    c.emit(0x830dc, I::Alu(Alu::And, R6, R3))?;
    c.emit(0x830de, I::Shift(Shift::Lsl, R5, R5, 19))?;
    c.emit(0x830e0, I::Compare(R0, 0))?;
    c.emit(0x830e2, I::Branch(Some(Condition::Ne), 0x830ee))?;
    c.emit(0x830e4, I::Alu(Alu::Orr, R1, R5))?;
    c.emit(0x830e6, I::Push(0x2))?;
    c.emit(0x830e8, I::Pop(0x20))?;
    c.emit(0x830ea, I::Stack(true, R0, 60))?;
    c.emit(0x830ec, I::Branch(None, 0x834cc))?;
    c.emit(0x830ee, I::Stack(false, R6, 0))?;
    c.emit(0x830f0, I::Mov(R2, 1))?;
    c.emit(0x830f2, I::Mov(R4, 0))?;
    c.emit(0x830f4, I::Stack(true, R0, 64))?;
    c.emit(0x830f6, I::Alu(Alu::Cmp, R7, R0))?;
    c.emit(0x830f8, I::Push(0x4))?;
    c.emit(0x830fa, I::Pop(0x1))?;
    c.emit(0x830fc, I::Branch(Some(Condition::Ls), 0x83102))?;
    c.emit(0x830fe, I::Push(0x10))?;
    c.emit(0x83100, I::Pop(0x1))?;
    c.emit(0x83102, I::Stack(true, R6, 0))?;
    c.emit(0x83104, I::Compare(R6, 0))?;
    c.emit(0x83106, I::Branch(Some(Condition::Eq), 0x8310c))?;
    c.emit(0x83108, I::Push(0x10))?;
    c.emit(0x8310a, I::Pop(0x4))?;
    c.emit(0x8310c, I::Stack(true, R4, 64))?;
    c.emit(0x8310e, I::Alu(Alu::Cmp, R7, R4))?;
    c.emit(0x83110, I::Stack(true, R6, 52))?;
    c.emit(0x83112, I::Branch(Some(Condition::Eq), 0x83118))?;
    c.emit(0x83114, I::Push(0x1))?;
    c.emit(0x83116, I::Pop(0x4))?;
    c.emit(0x83118, I::Compare(R2, 0))?;
    c.emit(0x8311a, I::Stack(true, R2, 32))?;
    c.emit(0x8311c, I::Branch(Some(Condition::Ne), 0x83124))?;
    c.emit(0x8311e, I::Stack(true, R1, 56))?;
    c.emit(0x83120, I::Alu(Alu::Orr, R1, R5))?;
    c.emit(0x83122, I::Branch(None, 0x8317c))?;
    c.emit(0x83124, I::Push(0x40))?;
    c.emit(0x83126, I::Pop(0x1))?;
    c.emit(0x83128, I::Alu(Alu::Eor, R0, R4))?;
    c.emit(0x8312a, I::Stack(true, R5, 4))?;
    c.emit(0x8312c, I::Alu(Alu::Orr, R0, R5))?;
    c.emit(0x8312e, I::Literal(R5, 0x834d4))?;
    c.emit(0x83130, I::Compare(R0, 0))?;
    c.emit(0x83132, I::Branch(Some(Condition::Ne), 0x83154))?;
    c.emit(0x83134, I::Stack(true, R4, 0))?;
    c.emit(0x83136, I::Alu(Alu::Orr, R4, R7))?;
    c.emit(0x83138, I::Stack(true, R0, 56))?;
    c.emit(0x8313a, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x8313c, I::Alu(Alu::Eor, R0, R1))?;
    c.emit(0x8313e, I::Compare(R4, 0))?;
    c.emit(0x83140, I::Branch(Some(Condition::Eq), 0x83146))?;
    c.emit(0x83142, I::Push(0x1))?;
    c.emit(0x83144, I::Pop(0x20))?;
    c.emit(0x83146, I::Compare(R4, 0))?;
    c.emit(0x83148, I::Stack(true, R0, 60))?;
    c.emit(0x8314a, I::Branch(Some(Condition::Eq), 0x8314e))?;
    c.emit(0x8314c, I::Branch(None, 0x834cc))?;
    c.emit(0x8314e, I::Push(0x10))?;
    c.emit(0x83150, I::Pop(0x1))?;
    c.emit(0x83152, I::Branch(None, 0x834cc))?;
    c.emit(0x83154, I::Push(0x80))?;
    c.emit(0x83156, I::Pop(0x1))?;
    c.emit(0x83158, I::Alu(Alu::Eor, R0, R4))?;
    c.emit(0x8315a, I::Stack(true, R4, 0))?;
    c.emit(0x8315c, I::Alu(Alu::Orr, R0, R4))?;
    c.emit(0x8315e, I::Compare(R0, 0))?;
    c.emit(0x83160, I::Branch(Some(Condition::Ne), 0x83186))?;
    c.emit(0x83162, I::Stack(true, R4, 4))?;
    c.emit(0x83164, I::Alu(Alu::Orr, R4, R6))?;
    c.emit(0x83166, I::Alu(Alu::And, R1, R2))?;
    c.emit(0x83168, I::Stack(true, R0, 56))?;
    c.emit(0x8316a, I::Alu(Alu::Eor, R1, R0))?;
    c.emit(0x8316c, I::Compare(R4, 0))?;
    c.emit(0x8316e, I::Branch(Some(Condition::Ne), 0x83174))?;
    c.emit(0x83170, I::Push(0x20))?;
    c.emit(0x83172, I::Pop(0x2))?;
    c.emit(0x83174, I::Compare(R4, 0))?;
    c.emit(0x83176, I::Branch(Some(Condition::Ne), 0x8317c))?;
    c.emit(0x83178, I::Push(0x10))?;
    c.emit(0x8317a, I::Pop(0x8))?;
    c.emit(0x8317c, I::Push(0x8))?;
    c.emit(0x8317e, I::Pop(0x1))?;
    c.emit(0x83180, I::Push(0x2))?;
    c.emit(0x83182, I::Pop(0x20))?;
    c.emit(0x83184, I::Branch(None, 0x834cc))?;
    c.emit(0x83186, I::Stack(true, R0, 4))?;
    c.emit(0x83188, I::Alu(Alu::Orr, R0, R6))?;
    c.emit(0x8318a, I::Compare(R0, 0))?;
    c.emit(0x8318c, I::Branch(Some(Condition::Ne), 0x83194))?;
    c.emit(0x8318e, I::Stack(true, R0, 68))?;
    c.emit(0x83190, I::Stack(true, R5, 28))?;
    c.emit(0x83192, I::Branch(None, 0x834cc))?;
    c.emit(0x83194, I::Push(0x10))?;
    c.emit(0x83196, I::Pop(0x1))?;
    c.emit(0x83198, I::Alu(Alu::Orr, R0, R7))?;
    c.emit(0x8319a, I::Compare(R0, 0))?;
    c.emit(0x8319c, I::Branch(Some(Condition::Eq), 0x8318e))?;
    c.emit(0x8319e, I::Mov(R5, 0))?;
    c.emit(0x831a0, I::Stack(true, R0, 40))?;
    c.emit(0x831a2, I::Small(false, R0, R0, 1))?;
    c.emit(0x831a4, I::Stack(true, R2, 8))?;
    c.emit(0x831a6, I::Alu(Alu::Adc, R2, R5))?;
    c.emit(0x831a8, I::Mov(R1, 1))?;
    c.emit(0x831aa, I::Stack(true, R3, 4))?;
    c.emit(0x831ac, I::Alu(Alu::Cmp, R3, R0))?;
    c.emit(0x831ae, I::Push(0x2))?;
    c.emit(0x831b0, I::Pop(0x1))?;
    c.emit(0x831b2, I::Branch(Some(Condition::Hs), 0x831b8))?;
    c.emit(0x831b4, I::Push(0x20))?;
    c.emit(0x831b6, I::Pop(0x1))?;
    c.emit(0x831b8, I::Alu(Alu::Cmp, R6, R2))?;
    c.emit(0x831ba, I::Push(0x40))?;
    c.emit(0x831bc, I::Pop(0x8))?;
    c.emit(0x831be, I::Branch(Some(Condition::Hs), 0x831c4))?;
    c.emit(0x831c0, I::Push(0x20))?;
    c.emit(0x831c2, I::Pop(0x2))?;
    c.emit(0x831c4, I::Stack(false, R7, 60))?;
    c.emit(0x831c6, I::Alu(Alu::Cmp, R3, R2))?;
    c.emit(0x831c8, I::Branch(Some(Condition::Eq), 0x831ce))?;
    c.emit(0x831ca, I::Push(0x2))?;
    c.emit(0x831cc, I::Pop(0x1))?;
    c.emit(0x831ce, I::Compare(R0, 0))?;
    c.emit(0x831d0, I::Push(0x20))?;
    c.emit(0x831d2, I::Pop(0x80))?;
    c.emit(0x831d4, I::Stack(true, R6, 8))?;
    c.emit(0x831d6, I::Branch(Some(Condition::Ne), 0x831e2))?;
    c.emit(0x831d8, I::Address(R0, 80))?;
    c.emit(0x831da, I::Call(0x834dc))?;
    c.emit(0x831de, I::Push(0x1))?;
    c.emit(0x831e0, I::Pop(0x80))?;
    c.emit(0x831e2, I::Stack(true, R0, 40))?;
    c.emit(0x831e4, I::Small(false, R1, R0, 1))?;
    c.emit(0x831e6, I::Push(0x40))?;
    c.emit(0x831e8, I::Pop(0x8))?;
    c.emit(0x831ea, I::Push(0x40))?;
    c.emit(0x831ec, I::Pop(0x4))?;
    c.emit(0x831ee, I::Alu(Alu::Adc, R2, R5))?;
    c.emit(0x831f0, I::Mov(R0, 1))?;
    c.emit(0x831f2, I::Alu(Alu::Cmp, R4, R1))?;
    c.emit(0x831f4, I::Push(0x1))?;
    c.emit(0x831f6, I::Pop(0x2))?;
    c.emit(0x831f8, I::Branch(Some(Condition::Hs), 0x831fe))?;
    c.emit(0x831fa, I::Push(0x20))?;
    c.emit(0x831fc, I::Pop(0x2))?;
    c.emit(0x831fe, I::Stack(true, R6, 60))?;
    c.emit(0x83200, I::Alu(Alu::Cmp, R6, R2))?;
    c.emit(0x83202, I::Branch(Some(Condition::Hs), 0x83208))?;
    c.emit(0x83204, I::Push(0x20))?;
    c.emit(0x83206, I::Pop(0x1))?;
    c.emit(0x83208, I::Alu(Alu::Cmp, R6, R2))?;
    c.emit(0x8320a, I::Branch(Some(Condition::Eq), 0x83210))?;
    c.emit(0x8320c, I::Push(0x1))?;
    c.emit(0x8320e, I::Pop(0x2))?;
    c.emit(0x83210, I::Compare(R1, 0))?;
    c.emit(0x83212, I::Stack(true, R5, 68))?;
    c.emit(0x83214, I::Push(0x8))?;
    c.emit(0x83216, I::Pop(0x40))?;
    c.emit(0x83218, I::Branch(Some(Condition::Ne), 0x83222))?;
    c.emit(0x8321a, I::Address(R0, 72))?;
    c.emit(0x8321c, I::Call(0x834dc))?;
    c.emit(0x83220, I::Arithmetic(false, R7, R0, R7))?;
    c.emit(0x83222, I::Stack(false, R7, 56))?;
    c.emit(0x83224, I::Stack(true, R4, 40))?;
    c.emit(0x83226, I::Small(false, R1, R4, 1))?;
    c.emit(0x83228, I::Alu(Alu::Adc, R6, R5))?;
    c.emit(0x8322a, I::Stack(false, R6, 52))?;
    c.emit(0x8322c, I::Stack(true, R5, 80))?;
    c.emit(0x8322e, I::Push(0x20))?;
    c.emit(0x83230, I::Pop(0x1))?;
    c.emit(0x83232, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x83234, I::Push(0x2))?;
    c.emit(0x83236, I::Pop(0x4))?;
    c.emit(0x83238, I::Stack(false, R2, 44))?;
    c.emit(0x8323a, I::Stack(true, R1, 84))?;
    c.emit(0x8323c, I::Stack(false, R0, 80))?;
    c.emit(0x8323e, I::Alu(Alu::Orr, R1, R6))?;
    c.emit(0x83240, I::Stack(false, R1, 4))?;
    c.emit(0x83242, I::Stack(false, R1, 84))?;
    c.emit(0x83244, I::Stack(true, R0, 76))?;
    c.emit(0x83246, I::Alu(Alu::Orr, R0, R6))?;
    c.emit(0x83248, I::Stack(true, R7, 72))?;
    c.emit(0x8324a, I::Stack(false, R0, 76))?;
    c.emit(0x8324c, I::Shift(Shift::Lsl, R0, R0, 11))?;
    c.emit(0x8324e, I::Push(0x80))?;
    c.emit(0x83250, I::Pop(0x2))?;
    c.emit(0x83252, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x83254, I::Stack(false, R1, 72))?;
    c.emit(0x83256, I::Shift(Shift::Lsr, R6, R1, 21))?;
    c.emit(0x83258, I::Alu(Alu::Orr, R6, R0))?;
    c.emit(0x8325a, I::Alu(Alu::And, R6, R4))?;
    c.emit(0x8325c, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x8325e, I::Push(0x40))?;
    c.emit(0x83260, I::Pop(0x1))?;
    c.emit(0x83262, I::Stack(true, R1, 68))?;
    c.emit(0x83264, I::Push(0x20))?;
    c.emit(0x83266, I::Pop(0x4))?;
    c.emit(0x83268, I::Stack(true, R3, 68))?;
    c.emit(0x8326a, I::Call(0x835b0))?;
    c.emit(0x8326e, I::Push(0x1))?;
    c.emit(0x83270, I::Pop(0x10))?;
    c.emit(0x83272, I::Stack(false, R1, 36))?;
    c.emit(0x83274, I::Shift(Shift::Lsl, R7, R7, 11))?;
    c.emit(0x83276, I::Stack(true, R0, 48))?;
    c.emit(0x83278, I::Alu(Alu::And, R7, R0))?;
    c.emit(0x8327a, I::Push(0x80))?;
    c.emit(0x8327c, I::Pop(0x1))?;
    c.emit(0x8327e, I::Stack(true, R1, 68))?;
    c.emit(0x83280, I::Push(0x20))?;
    c.emit(0x83282, I::Pop(0x4))?;
    c.emit(0x83284, I::Stack(true, R3, 68))?;
    c.emit(0x83286, I::Call(0x835b0))?;
    c.emit(0x8328a, I::Stack(false, R0, 60))?;
    c.emit(0x8328c, I::Push(0x2))?;
    c.emit(0x8328e, I::Pop(0x20))?;
    c.emit(0x83290, I::Push(0x80))?;
    c.emit(0x83292, I::Pop(0x1))?;
    c.emit(0x83294, I::Stack(true, R1, 68))?;
    c.emit(0x83296, I::Stack(true, R7, 4))?;
    c.emit(0x83298, I::Push(0x80))?;
    c.emit(0x8329a, I::Pop(0x4))?;
    c.emit(0x8329c, I::Stack(true, R3, 68))?;
    c.emit(0x8329e, I::Call(0x835b0))?;
    c.emit(0x832a2, I::Stack(false, R1, 0))?;
    c.emit(0x832a4, I::Stack(true, R1, 48))?;
    c.emit(0x832a6, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x832a8, I::Arithmetic(false, R0, R0, R5))?;
    c.emit(0x832aa, I::Stack(true, R5, 68))?;
    c.emit(0x832ac, I::Alu(Alu::Adc, R5, R5))?;
    c.emit(0x832ae, I::Stack(true, R1, 40))?;
    c.emit(0x832b0, I::Alu(Alu::And, R4, R1))?;
    c.emit(0x832b2, I::Arithmetic(false, R4, R0, R4))?;
    c.emit(0x832b4, I::Stack(true, R0, 68))?;
    c.emit(0x832b6, I::Alu(Alu::Adc, R5, R0))?;
    c.emit(0x832b8, I::Push(0x40))?;
    c.emit(0x832ba, I::Pop(0x1))?;
    c.emit(0x832bc, I::Stack(true, R6, 68))?;
    c.emit(0x832be, I::Push(0x40))?;
    c.emit(0x832c0, I::Pop(0x2))?;
    c.emit(0x832c2, I::Push(0x80))?;
    c.emit(0x832c4, I::Pop(0x4))?;
    c.emit(0x832c6, I::Push(0x40))?;
    c.emit(0x832c8, I::Pop(0x8))?;
    c.emit(0x832ca, I::Call(0x835b0))?;
    c.emit(0x832ce, I::Stack(true, R2, 0))?;
    c.emit(0x832d0, I::Arithmetic(false, R0, R2, R0))?;
    c.emit(0x832d2, I::Alu(Alu::Adc, R1, R6))?;
    c.emit(0x832d4, I::Stack(true, R2, 36))?;
    c.emit(0x832d6, I::Arithmetic(false, R0, R0, R2))?;
    c.emit(0x832d8, I::Alu(Alu::Adc, R1, R6))?;
    c.emit(0x832da, I::Arithmetic(false, R7, R0, R5))?;
    c.emit(0x832dc, I::Stack(true, R5, 60))?;
    c.emit(0x832de, I::Alu(Alu::Adc, R1, R6))?;
    c.emit(0x832e0, I::Stack(true, R2, 52))?;
    c.emit(0x832e2, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x832e4, I::Stack(true, R0, 44))?;
    c.emit(0x832e6, I::Alu(Alu::And, R0, R7))?;
    c.emit(0x832e8, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x832ea, I::Stack(true, R2, 48))?;
    c.emit(0x832ec, I::Alu(Alu::And, R5, R2))?;
    c.emit(0x832ee, I::Stack(true, R2, 12))?;
    c.emit(0x832f0, I::Stack(true, R3, 16))?;
    c.emit(0x832f2, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x832f4, I::Stack(true, R3, 56))?;
    c.emit(0x832f6, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x832f8, I::Literal(R3, 0x834d8))?;
    c.emit(0x832fa, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x832fc, I::Compare(R0, 0))?;
    c.emit(0x832fe, I::Branch(Some(Condition::Eq), 0x8330a))?;
    c.emit(0x83300, I::Stack(false, R4, 56))?;
    c.emit(0x83302, I::Push(0x20))?;
    c.emit(0x83304, I::Pop(0x10))?;
    c.emit(0x83306, I::Small(false, R2, R2, 1))?;
    c.emit(0x83308, I::Branch(None, 0x83328))?;
    c.emit(0x8330a, I::Shift(Shift::Lsr, R3, R4, 31))?;
    c.emit(0x8330c, I::Push(0x80))?;
    c.emit(0x8330e, I::Pop(0x1))?;
    c.emit(0x83310, I::Shift(Shift::Lsl, R7, R0, 1))?;
    c.emit(0x83312, I::Alu(Alu::Orr, R7, R3))?;
    c.emit(0x83314, I::Shift(Shift::Lsr, R3, R5, 31))?;
    c.emit(0x83316, I::Shift(Shift::Lsl, R4, R4, 1))?;
    c.emit(0x83318, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x8331a, I::Stack(false, R4, 56))?;
    c.emit(0x8331c, I::Shift(Shift::Lsr, R0, R0, 31))?;
    c.emit(0x8331e, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x83320, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x83322, I::Shift(Shift::Lsl, R5, R5, 1))?;
    c.emit(0x83324, I::Push(0x20))?;
    c.emit(0x83326, I::Pop(0x10))?;
    c.emit(0x83328, I::Stack(true, R5, 24))?;
    c.emit(0x8332a, I::Stack(true, R3, 20))?;
    c.emit(0x8332c, I::Small(false, R3, R3, 1))?;
    c.emit(0x8332e, I::Alu(Alu::Cmp, R2, R3))?;
    c.emit(0x83330, I::Branch(Some(Condition::Le), 0x8333c))?;
    c.emit(0x83332, I::Stack(true, R5, 28))?;
    c.emit(0x83334, I::Stack(true, R1, 64))?;
    c.emit(0x83336, I::Alu(Alu::Orr, R5, R1))?;
    c.emit(0x83338, I::Mov(R0, 0))?;
    c.emit(0x8333a, I::Branch(None, 0x834cc))?;
    c.emit(0x8333c, I::Compare(R2, 0))?;
    c.emit(0x8333e, I::Branch(Some(Condition::Le), 0x83342))?;
    c.emit(0x83340, I::Branch(None, 0x83460))?;
    c.emit(0x83342, I::Mov(R0, 1))?;
    c.emit(0x83344, I::Arithmetic(true, R2, R0, R2))?;
    c.emit(0x83346, I::Compare(R2, 63))?;
    c.emit(0x83348, I::Branch(Some(Condition::Ls), 0x83364))?;
    c.emit(0x8334a, I::Push(0x40))?;
    c.emit(0x8334c, I::Pop(0x1))?;
    c.emit(0x8334e, I::Stack(true, R5, 28))?;
    c.emit(0x83350, I::Branch(None, 0x834cc))?;
    c.emit(0x83352, I::Align)?;
    c.pool(0x83354, 0x000fffff)?;
    c.pool(0x83358, 0x000007fd)?;
    c.pool(0x8335c, 0xfffff800)?;
    c.pool(0x83360, 0x7ff00000)?;
    c.emit(0x83364, I::Stack(false, R0, 36))?;
    c.emit(0x83366, I::Mov(R0, 32))?;
    c.emit(0x83368, I::Stack(false, R0, 20))?;
    c.emit(0x8336a, I::Arithmetic(true, R6, R0, R2))?;
    c.emit(0x8336c, I::Mov(R3, 64))?;
    c.emit(0x8336e, I::Arithmetic(true, R0, R3, R2))?;
    c.emit(0x83370, I::Push(0x80))?;
    c.emit(0x83372, I::Pop(0x20))?;
    c.emit(0x83374, I::Stack(false, R0, 44))?;
    c.emit(0x83376, I::Alu(Alu::Lsl, R5, R0))?;
    c.emit(0x83378, I::Mov(R3, 0))?;
    c.emit(0x8337a, I::Stack(false, R6, 64))?;
    c.emit(0x8337c, I::Compare(R6, 0))?;
    c.emit(0x8337e, I::Push(0x8))?;
    c.emit(0x83380, I::Pop(0x1))?;
    c.emit(0x83382, I::Branch(Some(Condition::Ge), 0x83388))?;
    c.emit(0x83384, I::Push(0x20))?;
    c.emit(0x83386, I::Pop(0x1))?;
    c.emit(0x83388, I::Stack(false, R0, 68))?;
    c.emit(0x8338a, I::Stack(false, R4, 60))?;
    c.emit(0x8338c, I::Push(0x10))?;
    c.emit(0x8338e, I::Pop(0x20))?;
    c.emit(0x83390, I::Alu(Alu::Lsr, R5, R2))?;
    c.emit(0x83392, I::Stack(true, R4, 56))?;
    c.emit(0x83394, I::Push(0x10))?;
    c.emit(0x83396, I::Pop(0x1))?;
    c.emit(0x83398, I::Stack(true, R6, 64))?;
    c.emit(0x8339a, I::Alu(Alu::Lsl, R0, R6))?;
    c.emit(0x8339c, I::Alu(Alu::Orr, R0, R5))?;
    c.emit(0x8339e, I::Stack(false, R2, 52))?;
    c.emit(0x833a0, I::Subtract(R2, 32))?;
    c.emit(0x833a2, I::Alu(Alu::Lsr, R4, R2))?;
    c.emit(0x833a4, I::Compare(R2, 0))?;
    c.emit(0x833a6, I::Branch(Some(Condition::Ge), 0x833ac))?;
    c.emit(0x833a8, I::Push(0x1))?;
    c.emit(0x833aa, I::Pop(0x10))?;
    c.emit(0x833ac, I::Stack(false, R2, 48))?;
    c.emit(0x833ae, I::Stack(true, R0, 68))?;
    c.emit(0x833b0, I::Alu(Alu::Orr, R0, R4))?;
    c.emit(0x833b2, I::Stack(false, R0, 68))?;
    c.emit(0x833b4, I::Stack(true, R2, 60))?;
    c.emit(0x833b6, I::Push(0x4))?;
    c.emit(0x833b8, I::Pop(0x20))?;
    c.emit(0x833ba, I::Stack(true, R6, 44))?;
    c.emit(0x833bc, I::Alu(Alu::Lsl, R5, R6))?;
    c.emit(0x833be, I::Stack(true, R0, 64))?;
    c.emit(0x833c0, I::Compare(R0, 0))?;
    c.emit(0x833c2, I::Stack(false, R3, 40))?;
    c.emit(0x833c4, I::Push(0x4))?;
    c.emit(0x833c6, I::Pop(0x10))?;
    c.emit(0x833c8, I::Branch(Some(Condition::Ge), 0x833cc))?;
    c.emit(0x833ca, I::Stack(false, R5, 40))?;
    c.emit(0x833cc, I::Stack(true, R2, 56))?;
    c.emit(0x833ce, I::Alu(Alu::Lsl, R2, R6))?;
    c.emit(0x833d0, I::Stack(true, R5, 20))?;
    c.emit(0x833d2, I::Arithmetic(true, R6, R5, R6))?;
    c.emit(0x833d4, I::Push(0x10))?;
    c.emit(0x833d6, I::Pop(0x20))?;
    c.emit(0x833d8, I::Stack(false, R6, 20))?;
    c.emit(0x833da, I::Alu(Alu::Lsr, R5, R6))?;
    c.emit(0x833dc, I::Alu(Alu::Orr, R5, R2))?;
    c.emit(0x833de, I::Alu(Alu::Lsl, R4, R0))?;
    c.emit(0x833e0, I::Compare(R0, 0))?;
    c.emit(0x833e2, I::Push(0x1))?;
    c.emit(0x833e4, I::Pop(0x40))?;
    c.emit(0x833e6, I::Branch(Some(Condition::Ge), 0x833ec))?;
    c.emit(0x833e8, I::Push(0x20))?;
    c.emit(0x833ea, I::Pop(0x10))?;
    c.emit(0x833ec, I::Stack(true, R0, 40))?;
    c.emit(0x833ee, I::Alu(Alu::Orr, R0, R4))?;
    c.emit(0x833f0, I::Compare(R0, 0))?;
    c.emit(0x833f2, I::Stack(true, R2, 36))?;
    c.emit(0x833f4, I::Branch(Some(Condition::Ne), 0x833fa))?;
    c.emit(0x833f6, I::Push(0x1))?;
    c.emit(0x833f8, I::Pop(0x4))?;
    c.emit(0x833fa, I::Stack(true, R0, 68))?;
    c.emit(0x833fc, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x833fe, I::Push(0x80))?;
    c.emit(0x83400, I::Pop(0x4))?;
    c.emit(0x83402, I::Stack(true, R4, 52))?;
    c.emit(0x83404, I::Alu(Alu::Lsr, R2, R4))?;
    c.emit(0x83406, I::Push(0x2))?;
    c.emit(0x83408, I::Pop(0x20))?;
    c.emit(0x8340a, I::Alu(Alu::Lsl, R5, R6))?;
    c.emit(0x8340c, I::Alu(Alu::Orr, R5, R2))?;
    c.emit(0x8340e, I::Push(0x2))?;
    c.emit(0x83410, I::Pop(0x40))?;
    c.emit(0x83412, I::Stack(true, R2, 48))?;
    c.emit(0x83414, I::Alu(Alu::Lsr, R6, R2))?;
    c.emit(0x83416, I::Compare(R2, 0))?;
    c.emit(0x83418, I::Branch(Some(Condition::Ge), 0x8341e))?;
    c.emit(0x8341a, I::Push(0x20))?;
    c.emit(0x8341c, I::Pop(0x40))?;
    c.emit(0x8341e, I::Push(0x2))?;
    c.emit(0x83420, I::Pop(0x4))?;
    c.emit(0x83422, I::Stack(true, R5, 44))?;
    c.emit(0x83424, I::Alu(Alu::Lsl, R2, R5))?;
    c.emit(0x83426, I::Push(0x80))?;
    c.emit(0x83428, I::Pop(0x20))?;
    c.emit(0x8342a, I::Stack(true, R4, 20))?;
    c.emit(0x8342c, I::Alu(Alu::Lsr, R5, R4))?;
    c.emit(0x8342e, I::Alu(Alu::Orr, R5, R2))?;
    c.emit(0x83430, I::Stack(true, R2, 64))?;
    c.emit(0x83432, I::Alu(Alu::Lsl, R7, R2))?;
    c.emit(0x83434, I::Compare(R2, 0))?;
    c.emit(0x83436, I::Branch(Some(Condition::Ge), 0x8343c))?;
    c.emit(0x83438, I::Push(0x20))?;
    c.emit(0x8343a, I::Pop(0x80))?;
    c.emit(0x8343c, I::Stack(true, R2, 52))?;
    c.emit(0x8343e, I::Stack(true, R5, 56))?;
    c.emit(0x83440, I::Alu(Alu::Lsr, R5, R2))?;
    c.emit(0x83442, I::Stack(true, R4, 48))?;
    c.emit(0x83444, I::Compare(R4, 0))?;
    c.emit(0x83446, I::Stack(false, R3, 68))?;
    c.emit(0x83448, I::Branch(Some(Condition::Ge), 0x8344c))?;
    c.emit(0x8344a, I::Stack(false, R5, 68))?;
    c.emit(0x8344c, I::Stack(true, R5, 68))?;
    c.emit(0x8344e, I::Alu(Alu::Orr, R7, R5))?;
    c.emit(0x83450, I::Alu(Alu::Lsr, R1, R2))?;
    c.emit(0x83452, I::Compare(R4, 0))?;
    c.emit(0x83454, I::Branch(Some(Condition::Ge), 0x8345a))?;
    c.emit(0x83456, I::Push(0x2))?;
    c.emit(0x83458, I::Pop(0x8))?;
    c.emit(0x8345a, I::Stack(true, R4, 32))?;
    c.emit(0x8345c, I::Stack(true, R5, 24))?;
    c.emit(0x8345e, I::Branch(None, 0x83478))?;
    c.emit(0x83460, I::Push(0x80))?;
    c.emit(0x83462, I::Pop(0x40))?;
    c.emit(0x83464, I::Stack(true, R0, 40))?;
    c.emit(0x83466, I::Alu(Alu::And, R6, R0))?;
    c.emit(0x83468, I::Stack(true, R0, 8))?;
    c.emit(0x8346a, I::Alu(Alu::And, R1, R0))?;
    c.emit(0x8346c, I::Shift(Shift::Lsl, R3, R2, 20))?;
    c.emit(0x8346e, I::Alu(Alu::Orr, R3, R1))?;
    c.emit(0x83470, I::Push(0x10))?;
    c.emit(0x83472, I::Pop(0x1))?;
    c.emit(0x83474, I::Stack(true, R7, 56))?;
    c.emit(0x83476, I::Stack(true, R4, 32))?;
    c.emit(0x83478, I::Mov(R2, 0))?;
    c.emit(0x8347a, I::Alu(Alu::Cmp, R7, R4))?;
    c.emit(0x8347c, I::Push(0x20))?;
    c.emit(0x8347e, I::Pop(0x2))?;
    c.emit(0x83480, I::Branch(Some(Condition::Ls), 0x83486))?;
    c.emit(0x83482, I::Push(0x4))?;
    c.emit(0x83484, I::Pop(0x2))?;
    c.emit(0x83486, I::Compare(R0, 0))?;
    c.emit(0x83488, I::Branch(Some(Condition::Eq), 0x8348e))?;
    c.emit(0x8348a, I::Push(0x4))?;
    c.emit(0x8348c, I::Pop(0x20))?;
    c.emit(0x8348e, I::Alu(Alu::Cmp, R7, R4))?;
    c.emit(0x83490, I::Branch(Some(Condition::Eq), 0x83496))?;
    c.emit(0x83492, I::Push(0x2))?;
    c.emit(0x83494, I::Pop(0x20))?;
    c.emit(0x83496, I::Stack(true, R1, 28))?;
    c.emit(0x83498, I::Alu(Alu::Orr, R3, R1))?;
    c.emit(0x8349a, I::Compare(R5, 0))?;
    c.emit(0x8349c, I::Branch(Some(Condition::Ne), 0x834aa))?;
    c.emit(0x8349e, I::Mov(R1, 0))?;
    c.emit(0x834a0, I::Small(false, R0, R6, 1))?;
    c.emit(0x834a2, I::Alu(Alu::Adc, R3, R1))?;
    c.emit(0x834a4, I::Push(0x8))?;
    c.emit(0x834a6, I::Pop(0x20))?;
    c.emit(0x834a8, I::Branch(None, 0x834cc))?;
    c.emit(0x834aa, I::Mov(R1, 1))?;
    c.emit(0x834ac, I::Alu(Alu::And, R1, R6))?;
    c.emit(0x834ae, I::Mov(R5, 0))?;
    c.emit(0x834b0, I::Push(0x1))?;
    c.emit(0x834b2, I::Pop(0x4))?;
    c.emit(0x834b4, I::Arithmetic(false, R0, R6, R1))?;
    c.emit(0x834b6, I::Alu(Alu::Adc, R5, R3))?;
    c.emit(0x834b8, I::Alu(Alu::Eor, R7, R4))?;
    c.emit(0x834ba, I::Alu(Alu::Orr, R2, R7))?;
    c.emit(0x834bc, I::Compare(R2, 0))?;
    c.emit(0x834be, I::Branch(Some(Condition::Eq), 0x834c4))?;
    c.emit(0x834c0, I::Push(0x8))?;
    c.emit(0x834c2, I::Pop(0x20))?;
    c.emit(0x834c4, I::Compare(R2, 0))?;
    c.emit(0x834c6, I::Branch(Some(Condition::Eq), 0x834cc))?;
    c.emit(0x834c8, I::Push(0x40))?;
    c.emit(0x834ca, I::Pop(0x1))?;
    c.emit(0x834cc, I::Push(0x20))?;
    c.emit(0x834ce, I::Pop(0x2))?;
    c.emit(0x834d0, I::StackAdjust(false, 92))?;
    c.emit(0x834d2, I::Pop(0x80f0))?;
    c.pool(0x834d4, 0x7ff80000)?;
    c.pool(0x834d8, 0xfffffc01)?;
    c.emit(0x834dc, I::Push(0x40f0))?;
    c.emit(0x834de, I::LoadMultiple(R0, 6))?;
    c.emit(0x834e0, I::Mov(R3, 32))?;
    c.emit(0x834e2, I::Subtract(R0, 8))?;
    c.emit(0x834e4, I::Compare(R2, 0))?;
    c.emit(0x834e6, I::Branch(Some(Condition::Eq), 0x83524))?;
    c.emit(0x834e8, I::Compare(R2, 0))?;
    c.emit(0x834ea, I::Branch(Some(Condition::Eq), 0x83560))?;
    c.emit(0x834ec, I::Shift(Shift::Lsr, R3, R2, 1))?;
    c.emit(0x834ee, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x834f0, I::Shift(Shift::Lsr, R4, R3, 2))?;
    c.emit(0x834f2, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x834f4, I::Shift(Shift::Lsr, R3, R4, 4))?;
    c.emit(0x834f6, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x834f8, I::Shift(Shift::Lsr, R4, R3, 8))?;
    c.emit(0x834fa, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x834fc, I::Shift(Shift::Lsr, R3, R4, 16))?;
    c.emit(0x834fe, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x83500, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x83502, I::Shift(Shift::Lsr, R4, R3, 1))?;
    c.emit(0x83504, I::Literal(R5, 0x835a0))?;
    c.emit(0x83506, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x83508, I::Arithmetic(true, R3, R3, R5))?;
    c.emit(0x8350a, I::Literal(R4, 0x835a4))?;
    c.emit(0x8350c, I::Shift(Shift::Lsr, R5, R3, 2))?;
    c.emit(0x8350e, I::Alu(Alu::And, R3, R4))?;
    c.emit(0x83510, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x83512, I::Arithmetic(false, R3, R3, R5))?;
    c.emit(0x83514, I::Shift(Shift::Lsr, R4, R3, 4))?;
    c.emit(0x83516, I::Arithmetic(false, R3, R3, R4))?;
    c.emit(0x83518, I::Literal(R4, 0x835a8))?;
    c.emit(0x8351a, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x8351c, I::Literal(R3, 0x835ac))?;
    c.emit(0x8351e, I::Alu(Alu::Mul, R3, R4))?;
    c.emit(0x83520, I::Shift(Shift::Lsr, R3, R3, 24))?;
    c.emit(0x83522, I::Branch(None, 0x83560))?;
    c.emit(0x83524, I::Compare(R1, 0))?;
    c.emit(0x83526, I::Branch(Some(Condition::Eq), 0x8355e))?;
    c.emit(0x83528, I::Shift(Shift::Lsr, R3, R1, 1))?;
    c.emit(0x8352a, I::Alu(Alu::Orr, R3, R1))?;
    c.emit(0x8352c, I::Shift(Shift::Lsr, R4, R3, 2))?;
    c.emit(0x8352e, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x83530, I::Shift(Shift::Lsr, R3, R4, 4))?;
    c.emit(0x83532, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x83534, I::Shift(Shift::Lsr, R4, R3, 8))?;
    c.emit(0x83536, I::Alu(Alu::Orr, R4, R3))?;
    c.emit(0x83538, I::Shift(Shift::Lsr, R3, R4, 16))?;
    c.emit(0x8353a, I::Alu(Alu::Orr, R3, R4))?;
    c.emit(0x8353c, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x8353e, I::Shift(Shift::Lsr, R4, R3, 1))?;
    c.emit(0x83540, I::Literal(R5, 0x835a0))?;
    c.emit(0x83542, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x83544, I::Arithmetic(true, R3, R3, R5))?;
    c.emit(0x83546, I::Literal(R4, 0x835a4))?;
    c.emit(0x83548, I::Shift(Shift::Lsr, R5, R3, 2))?;
    c.emit(0x8354a, I::Alu(Alu::And, R3, R4))?;
    c.emit(0x8354c, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x8354e, I::Arithmetic(false, R3, R3, R5))?;
    c.emit(0x83550, I::Shift(Shift::Lsr, R4, R3, 4))?;
    c.emit(0x83552, I::Arithmetic(false, R3, R3, R4))?;
    c.emit(0x83554, I::Literal(R4, 0x835a8))?;
    c.emit(0x83556, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x83558, I::Literal(R3, 0x835ac))?;
    c.emit(0x8355a, I::Alu(Alu::Mul, R3, R4))?;
    c.emit(0x8355c, I::Shift(Shift::Lsr, R3, R3, 24))?;
    c.emit(0x8355e, I::Add(R3, 32))?;
    c.emit(0x83560, I::Push(0x8))?;
    c.emit(0x83562, I::Pop(0x20))?;
    c.emit(0x83564, I::Subtract(R5, 11))?;
    c.emit(0x83566, I::Push(0x2))?;
    c.emit(0x83568, I::Pop(0x40))?;
    c.emit(0x8356a, I::Alu(Alu::Lsl, R6, R5))?;
    c.emit(0x8356c, I::Mov(R7, 0))?;
    c.emit(0x8356e, I::Push(0x8))?;
    c.emit(0x83570, I::Pop(0x10))?;
    c.emit(0x83572, I::Subtract(R4, 43))?;
    c.emit(0x83574, I::Compare(R4, 0))?;
    c.emit(0x83576, I::Branch(Some(Condition::Ge), 0x8357c))?;
    c.emit(0x83578, I::Push(0x40))?;
    c.emit(0x8357a, I::Pop(0x80))?;
    c.emit(0x8357c, I::Memory(false, R7, R0, 0))?;
    c.emit(0x8357e, I::Alu(Alu::Lsl, R2, R5))?;
    c.emit(0x83580, I::Mov(R5, 43))?;
    c.emit(0x83582, I::Arithmetic(true, R6, R5, R3))?;
    c.emit(0x83584, I::Push(0x2))?;
    c.emit(0x83586, I::Pop(0x20))?;
    c.emit(0x83588, I::Alu(Alu::Lsr, R5, R6))?;
    c.emit(0x8358a, I::Alu(Alu::Orr, R5, R2))?;
    c.emit(0x8358c, I::Alu(Alu::Lsl, R1, R4))?;
    c.emit(0x8358e, I::Compare(R4, 0))?;
    c.emit(0x83590, I::Branch(Some(Condition::Ge), 0x83596))?;
    c.emit(0x83592, I::Push(0x20))?;
    c.emit(0x83594, I::Pop(0x2))?;
    c.emit(0x83596, I::Memory(false, R1, R0, 4))?;
    c.emit(0x83598, I::Mov(R0, 12))?;
    c.emit(0x8359a, I::Arithmetic(true, R0, R0, R3))?;
    c.emit(0x8359c, I::Pop(0x80f0))?;
    c.emit(0x8359e, I::Align)?;
    c.pool(0x835a0, 0x55555555)?;
    c.pool(0x835a4, 0x33333333)?;
    c.pool(0x835a8, 0x0f0f0f0f)?;
    c.pool(0x835ac, 0x01010101)?;
    c.emit(0x835b0, I::Push(0x40fe))?;
    c.emit(0x835b2, I::Stack(false, R3, 8))?;
    c.emit(0x835b4, I::Stack(false, R1, 4))?;
    c.emit(0x835b6, I::Push(0x1))?;
    c.emit(0x835b8, I::Pop(0x10))?;
    c.emit(0x835ba, I::Literal(R5, 0x83604))?;
    c.emit(0x835bc, I::Push(0x10))?;
    c.emit(0x835be, I::Pop(0x40))?;
    c.emit(0x835c0, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x835c2, I::Push(0x4))?;
    c.emit(0x835c4, I::Pop(0x80))?;
    c.emit(0x835c6, I::Alu(Alu::And, R7, R5))?;
    c.emit(0x835c8, I::Push(0x40))?;
    c.emit(0x835ca, I::Pop(0x1))?;
    c.emit(0x835cc, I::Alu(Alu::Mul, R0, R7))?;
    c.emit(0x835ce, I::Shift(Shift::Lsr, R3, R0, 16))?;
    c.emit(0x835d0, I::Alu(Alu::And, R0, R5))?;
    c.emit(0x835d2, I::Shift(Shift::Lsr, R1, R4, 16))?;
    c.emit(0x835d4, I::Alu(Alu::Mul, R7, R1))?;
    c.emit(0x835d6, I::Arithmetic(false, R7, R3, R7))?;
    c.emit(0x835d8, I::Shift(Shift::Lsr, R3, R2, 16))?;
    c.emit(0x835da, I::Stack(false, R3, 0))?;
    c.emit(0x835dc, I::Alu(Alu::Mul, R6, R3))?;
    c.emit(0x835de, I::Arithmetic(false, R3, R7, R6))?;
    c.emit(0x835e0, I::Shift(Shift::Lsl, R3, R3, 16))?;
    c.emit(0x835e2, I::Alu(Alu::Orr, R0, R3))?;
    c.emit(0x835e4, I::Stack(true, R3, 4))?;
    c.emit(0x835e6, I::Alu(Alu::Mul, R3, R2))?;
    c.emit(0x835e8, I::Stack(true, R2, 8))?;
    c.emit(0x835ea, I::Alu(Alu::Mul, R2, R4))?;
    c.emit(0x835ec, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x835ee, I::Stack(true, R3, 0))?;
    c.emit(0x835f0, I::Alu(Alu::Mul, R1, R3))?;
    c.emit(0x835f2, I::Arithmetic(false, R1, R2, R1))?;
    c.emit(0x835f4, I::Shift(Shift::Lsr, R2, R7, 16))?;
    c.emit(0x835f6, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x835f8, I::Alu(Alu::And, R7, R5))?;
    c.emit(0x835fa, I::Arithmetic(false, R2, R7, R6))?;
    c.emit(0x835fc, I::Shift(Shift::Lsr, R2, R2, 16))?;
    c.emit(0x835fe, I::Arithmetic(false, R1, R1, R2))?;
    c.emit(0x83600, I::StackAdjust(false, 12))?;
    c.emit(0x83602, I::Pop(0x80f0))?;
    c.pool(0x83604, 0x0000ffff)?;
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
        assert_eq!(bytes.len(), 1520);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes), "b07086b9c8761eb6b467dd3f7422dac5f332aabc13220406f7ef55ee1f129730");
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        for at in ALIGNMENTS { assert_eq!(half(&bytes, at), 0x46c0); }
        for (at, slot) in [(0x83024, 0x83354), (0x83048, 0x83358), (0x83062, 0x8335c), (0x83066, 0x83360), (0x8312e, 0x834d4), (0x832f8, 0x834d8), (0x83504, 0x835a0), (0x8350a, 0x835a4), (0x83518, 0x835a8), (0x8351c, 0x835ac), (0x83540, 0x835a0), (0x83546, 0x835a4), (0x83554, 0x835a8), (0x83558, 0x835ac), (0x835ba, 0x83604)] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn independently_decoded_control_flow() {
        let bytes = fragment().unwrap();
        for (at, target, conditional) in [(0x8307a, 0x83088, true), (0x83084, 0x83088, true), (0x83086, 0x83222, false), (0x830aa, 0x830b0, true), (0x830c0, 0x830c6, true), (0x830d0, 0x830d6, true), (0x830e2, 0x830ee, true), (0x830ec, 0x834cc, false), (0x830fc, 0x83102, true), (0x83106, 0x8310c, true), (0x83112, 0x83118, true), (0x8311c, 0x83124, true), (0x83122, 0x8317c, false), (0x83132, 0x83154, true), (0x83140, 0x83146, true), (0x8314a, 0x8314e, true), (0x8314c, 0x834cc, false), (0x83152, 0x834cc, false), (0x83160, 0x83186, true), (0x8316e, 0x83174, true), (0x83176, 0x8317c, true), (0x83184, 0x834cc, false), (0x8318c, 0x83194, true), (0x83192, 0x834cc, false), (0x8319c, 0x8318e, true), (0x831b2, 0x831b8, true), (0x831be, 0x831c4, true), (0x831c8, 0x831ce, true), (0x831d6, 0x831e2, true), (0x831f8, 0x831fe, true), (0x83202, 0x83208, true), (0x8320a, 0x83210, true), (0x83218, 0x83222, true), (0x832fe, 0x8330a, true), (0x83308, 0x83328, false), (0x83330, 0x8333c, true), (0x8333a, 0x834cc, false), (0x8333e, 0x83342, true), (0x83340, 0x83460, false), (0x83348, 0x83364, true), (0x83350, 0x834cc, false), (0x83382, 0x83388, true), (0x833a6, 0x833ac, true), (0x833c8, 0x833cc, true), (0x833e6, 0x833ec, true), (0x833f4, 0x833fa, true), (0x83418, 0x8341e, true), (0x83436, 0x8343c, true), (0x83448, 0x8344c, true), (0x83454, 0x8345a, true), (0x8345e, 0x83478, false), (0x83480, 0x83486, true), (0x83488, 0x8348e, true), (0x83490, 0x83496, true), (0x8349c, 0x834aa, true), (0x834a8, 0x834cc, false), (0x834be, 0x834c4, true), (0x834c6, 0x834cc, true), (0x834e6, 0x83524, true), (0x834ea, 0x83560, true), (0x83522, 0x83560, false), (0x83526, 0x8355e, true), (0x83576, 0x8357c, true), (0x83590, 0x83596, true)] {
            let word = half(&bytes, at);
            assert_eq!(word & (if conditional { 0xf000 } else { 0xf800 }),
                if conditional { 0xd000 } else { 0xe000 });
            let shift = if conditional { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [(0x831da, 0x834dc), (0x8321c, 0x834dc), (0x8326a, 0x835b0), (0x83286, 0x835b0), (0x8329e, 0x835b0), (0x832ca, 0x835b0)] {
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
    fn carry_multiply_and_shift_encodings() {
        let bytes = fragment().unwrap();
        assert_eq!(I::Alu(Alu::Adc, R0, R4).encode(0x83090).unwrap(), bytes[0x83090 - START..0x83092 - START]);
        assert_eq!(I::Alu(Alu::Adc, R2, R5).encode(0x831a6).unwrap(), bytes[0x831a6 - START..0x831a8 - START]);
        assert_eq!(I::Alu(Alu::Adc, R2, R5).encode(0x831ee).unwrap(), bytes[0x831ee - START..0x831f0 - START]);
        assert_eq!(I::Alu(Alu::Adc, R6, R5).encode(0x83228).unwrap(), bytes[0x83228 - START..0x8322a - START]);
        assert_eq!(I::Alu(Alu::Adc, R5, R5).encode(0x832ac).unwrap(), bytes[0x832ac - START..0x832ae - START]);
        assert_eq!(I::Alu(Alu::Adc, R5, R0).encode(0x832b6).unwrap(), bytes[0x832b6 - START..0x832b8 - START]);
        assert_eq!(I::Alu(Alu::Adc, R1, R6).encode(0x832d2).unwrap(), bytes[0x832d2 - START..0x832d4 - START]);
        assert_eq!(I::Alu(Alu::Adc, R1, R6).encode(0x832d8).unwrap(), bytes[0x832d8 - START..0x832da - START]);
        assert_eq!(I::Alu(Alu::Adc, R1, R6).encode(0x832de).unwrap(), bytes[0x832de - START..0x832e0 - START]);
        assert_eq!(I::Alu(Alu::Adc, R3, R1).encode(0x834a2).unwrap(), bytes[0x834a2 - START..0x834a4 - START]);
        assert_eq!(I::Alu(Alu::Adc, R5, R3).encode(0x834b6).unwrap(), bytes[0x834b6 - START..0x834b8 - START]);
        assert_eq!(I::Alu(Alu::Mul, R3, R4).encode(0x8351e).unwrap(), bytes[0x8351e - START..0x83520 - START]);
        assert_eq!(I::Alu(Alu::Mul, R3, R4).encode(0x8355a).unwrap(), bytes[0x8355a - START..0x8355c - START]);
        assert_eq!(I::Alu(Alu::Mul, R0, R7).encode(0x835cc).unwrap(), bytes[0x835cc - START..0x835ce - START]);
        assert_eq!(I::Alu(Alu::Mul, R7, R1).encode(0x835d4).unwrap(), bytes[0x835d4 - START..0x835d6 - START]);
        assert_eq!(I::Alu(Alu::Mul, R6, R3).encode(0x835dc).unwrap(), bytes[0x835dc - START..0x835de - START]);
        assert_eq!(I::Alu(Alu::Mul, R3, R2).encode(0x835e6).unwrap(), bytes[0x835e6 - START..0x835e8 - START]);
        assert_eq!(I::Alu(Alu::Mul, R2, R4).encode(0x835ea).unwrap(), bytes[0x835ea - START..0x835ec - START]);
        assert_eq!(I::Alu(Alu::Mul, R1, R3).encode(0x835f0).unwrap(), bytes[0x835f0 - START..0x835f2 - START]);
        assert_eq!(I::Shift(Shift::Lsl, R0, R1, 0).encode(START).unwrap(), 0x0008u16.to_le_bytes());
        assert_eq!(I::Shift(Shift::Lsr, R0, R1, 32).encode(START).unwrap(), 0x0808u16.to_le_bytes());
        for (start, end) in [(START, 0x83352), (0x83364, 0x834d4), (0x834dc, 0x8359e), (0x835b0, 0x83604)] {
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
        assert!(c.pool(START, 0).is_err());
        for at in [0x83350, 0x83352, 0x83354, 0x834d4, 0x8359c, 0x8359e, 0x835a0, 0x83604, END] {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            let instruction = if at == 0x83350 || at == 0x8359c {
                I::Call(0x834dc)
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
            "b07086b9c8761eb6b467dd3f7422dac5f332aabc13220406f7ef55ee1f129730");
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
