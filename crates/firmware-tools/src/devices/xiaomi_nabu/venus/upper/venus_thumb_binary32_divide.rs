pub const START: usize = 0x82988;
pub const END: usize = 0x82be4;
pub const CODE_BYTES: usize = 570;
pub const DATA_BYTES: usize = 32;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 2;
const POOLS: [(usize, u32); 8] = [
    (0x82b74, 0x007fffff), (0x82b78, 0x7fffffff),
    (0x82b7c, 0x7fc00000), (0x82b80, 0x7504f333),
    (0x82bd4, 0x55555555), (0x82bd8, 0x33333333),
    (0x82bdc, 0x0f0f0f0f), (0x82be0, 0x01010101),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Lsl, Lsr, Tst, Neg, Cmp, Orr, Mul, Mvn }
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
    Branch(Option<Condition>, usize), Call(usize), BxLr, Memory(bool, R, R, u16), Align,
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
                    Alu::Tst => 8, Alu::Neg => 9, Alu::Cmp => 10,
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
        checked(matches!(instruction, I::Align) == (at == 0x82b72))?;
        checked(![0x82b72].iter().any(|&slot| at < slot && at + bytes.len() > slot))?;
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
    c.emit(0x82988, I::Push(0x40f0))?;
    c.emit(0x8298a, I::StackAdjust(true, 52))?;
    c.emit(0x8298c, I::Literal(R3, 0x82b74))?;
    c.emit(0x8298e, I::Push(0x1))?;
    c.emit(0x82990, I::Pop(0x4))?;
    c.emit(0x82992, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x82994, I::Stack(false, R2, 48))?;
    c.emit(0x82996, I::Push(0x2))?;
    c.emit(0x82998, I::Pop(0x4))?;
    c.emit(0x8299a, I::Stack(false, R3, 36))?;
    c.emit(0x8299c, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x8299e, I::Stack(false, R2, 44))?;
    c.emit(0x829a0, I::Push(0x2))?;
    c.emit(0x829a2, I::Pop(0x4))?;
    c.emit(0x829a4, I::Alu(Alu::Eor, R2, R0))?;
    c.emit(0x829a6, I::Literal(R3, 0x82b78))?;
    c.emit(0x829a8, I::Small(false, R7, R3, 1))?;
    c.emit(0x829aa, I::Alu(Alu::And, R2, R7))?;
    c.emit(0x829ac, I::Stack(false, R2, 32))?;
    c.emit(0x829ae, I::Shift(Shift::Lsr, R6, R1, 23))?;
    c.emit(0x829b0, I::Mov(R2, 255))?;
    c.emit(0x829b2, I::Alu(Alu::And, R6, R2))?;
    c.emit(0x829b4, I::Shift(Shift::Lsr, R5, R0, 23))?;
    c.emit(0x829b6, I::Alu(Alu::And, R5, R2))?;
    c.emit(0x829b8, I::Stack(false, R5, 24))?;
    c.emit(0x829ba, I::Small(true, R5, R5, 1))?;
    c.emit(0x829bc, I::Mov(R4, 0))?;
    c.emit(0x829be, I::Stack(false, R4, 28))?;
    c.emit(0x829c0, I::Alu(Alu::Mvn, R4, R4))?;
    c.emit(0x829c2, I::Stack(false, R4, 20))?;
    c.emit(0x829c4, I::Shift(Shift::Lsl, R2, R2, 23))?;
    c.emit(0x829c6, I::Stack(false, R2, 40))?;
    c.emit(0x829c8, I::Compare(R5, 253))?;
    c.emit(0x829ca, I::Branch(Some(Condition::Hi), 0x829d2))?;
    c.emit(0x829cc, I::Small(true, R2, R6, 1))?;
    c.emit(0x829ce, I::Compare(R2, 254))?;
    c.emit(0x829d0, I::Branch(Some(Condition::Lo), 0x82a4e))?;
    c.emit(0x829d2, I::Push(0x2))?;
    c.emit(0x829d4, I::Pop(0x20))?;
    c.emit(0x829d6, I::Alu(Alu::And, R5, R3))?;
    c.emit(0x829d8, I::Alu(Alu::And, R3, R0))?;
    c.emit(0x829da, I::Mov(R2, 1))?;
    c.emit(0x829dc, I::Shift(Shift::Lsl, R2, R2, 22))?;
    c.emit(0x829de, I::Stack(true, R4, 40))?;
    c.emit(0x829e0, I::Alu(Alu::Cmp, R3, R4))?;
    c.emit(0x829e2, I::Branch(Some(Condition::Ls), 0x829e8))?;
    c.emit(0x829e4, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x829e6, I::Branch(None, 0x82a1c))?;
    c.emit(0x829e8, I::Alu(Alu::Cmp, R5, R4))?;
    c.emit(0x829ea, I::Branch(Some(Condition::Ls), 0x829f0))?;
    c.emit(0x829ec, I::Alu(Alu::Orr, R1, R2))?;
    c.emit(0x829ee, I::Branch(None, 0x82a00))?;
    c.emit(0x829f0, I::Literal(R2, 0x82b7c))?;
    c.emit(0x829f2, I::Alu(Alu::Cmp, R3, R4))?;
    c.emit(0x829f4, I::Branch(Some(Condition::Ne), 0x82a06))?;
    c.emit(0x829f6, I::Alu(Alu::And, R1, R7))?;
    c.emit(0x829f8, I::Alu(Alu::Eor, R1, R0))?;
    c.emit(0x829fa, I::Alu(Alu::Cmp, R5, R4))?;
    c.emit(0x829fc, I::Branch(Some(Condition::Ne), 0x82a00))?;
    c.emit(0x829fe, I::Branch(None, 0x82b6a))?;
    c.emit(0x82a00, I::Push(0x2))?;
    c.emit(0x82a02, I::Pop(0x4))?;
    c.emit(0x82a04, I::Branch(None, 0x82b6a))?;
    c.emit(0x82a06, I::Alu(Alu::Cmp, R5, R4))?;
    c.emit(0x82a08, I::Branch(Some(Condition::Ne), 0x82a0e))?;
    c.emit(0x82a0a, I::Stack(true, R2, 32))?;
    c.emit(0x82a0c, I::Branch(None, 0x82b6a))?;
    c.emit(0x82a0e, I::Compare(R3, 0))?;
    c.emit(0x82a10, I::Branch(Some(Condition::Ne), 0x82a22))?;
    c.emit(0x82a12, I::Compare(R5, 0))?;
    c.emit(0x82a14, I::Stack(true, R0, 32))?;
    c.emit(0x82a16, I::Branch(Some(Condition::Ne), 0x82a1c))?;
    c.emit(0x82a18, I::Push(0x4))?;
    c.emit(0x82a1a, I::Pop(0x1))?;
    c.emit(0x82a1c, I::Push(0x1))?;
    c.emit(0x82a1e, I::Pop(0x4))?;
    c.emit(0x82a20, I::Branch(None, 0x82b6a))?;
    c.emit(0x82a22, I::Compare(R5, 0))?;
    c.emit(0x82a24, I::Branch(Some(Condition::Ne), 0x82a2c))?;
    c.emit(0x82a26, I::Stack(true, R2, 32))?;
    c.emit(0x82a28, I::Alu(Alu::Orr, R2, R4))?;
    c.emit(0x82a2a, I::Branch(None, 0x82b6a))?;
    c.emit(0x82a2c, I::Stack(true, R0, 36))?;
    c.emit(0x82a2e, I::Small(false, R7, R0, 1))?;
    c.emit(0x82a30, I::Mov(R4, 0))?;
    c.emit(0x82a32, I::Alu(Alu::Cmp, R3, R7))?;
    c.emit(0x82a34, I::Branch(Some(Condition::Hs), 0x82a40))?;
    c.emit(0x82a36, I::Address(R0, 48))?;
    c.emit(0x82a38, I::Call(0x82b84))?;
    c.emit(0x82a3c, I::Push(0x1))?;
    c.emit(0x82a3e, I::Pop(0x10))?;
    c.emit(0x82a40, I::Alu(Alu::Cmp, R5, R7))?;
    c.emit(0x82a42, I::Branch(Some(Condition::Hs), 0x82a4c))?;
    c.emit(0x82a44, I::Address(R0, 44))?;
    c.emit(0x82a46, I::Call(0x82b84))?;
    c.emit(0x82a4a, I::Arithmetic(true, R4, R4, R0))?;
    c.emit(0x82a4c, I::Stack(false, R4, 28))?;
    c.emit(0x82a4e, I::Stack(true, R0, 36))?;
    c.emit(0x82a50, I::Small(false, R0, R0, 1))?;
    c.emit(0x82a52, I::Stack(true, R1, 48))?;
    c.emit(0x82a54, I::Stack(false, R1, 12))?;
    c.emit(0x82a56, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x82a58, I::Stack(false, R1, 8))?;
    c.emit(0x82a5a, I::Stack(false, R1, 48))?;
    c.emit(0x82a5c, I::Stack(true, R1, 44))?;
    c.emit(0x82a5e, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x82a60, I::Stack(false, R1, 44))?;
    c.emit(0x82a62, I::Stack(false, R1, 4))?;
    c.emit(0x82a64, I::Shift(Shift::Lsl, R2, R1, 8))?;
    c.emit(0x82a66, I::Stack(false, R2, 16))?;
    c.emit(0x82a68, I::Literal(R0, 0x82b80))?;
    c.emit(0x82a6a, I::Arithmetic(true, R7, R0, R2))?;
    c.emit(0x82a6c, I::Mov(R5, 0))?;
    c.emit(0x82a6e, I::Push(0x80))?;
    c.emit(0x82a70, I::Pop(0x1))?;
    c.emit(0x82a72, I::Push(0x20))?;
    c.emit(0x82a74, I::Pop(0x2))?;
    c.emit(0x82a76, I::Push(0x20))?;
    c.emit(0x82a78, I::Pop(0x8))?;
    c.emit(0x82a7a, I::Call(0x835b0))?;
    c.emit(0x82a7e, I::Alu(Alu::Neg, R0, R1))?;
    c.emit(0x82a80, I::Stack(true, R4, 20))?;
    c.emit(0x82a82, I::Alu(Alu::And, R0, R4))?;
    c.emit(0x82a84, I::Push(0x20))?;
    c.emit(0x82a86, I::Pop(0x2))?;
    c.emit(0x82a88, I::Push(0x80))?;
    c.emit(0x82a8a, I::Pop(0x4))?;
    c.emit(0x82a8c, I::Push(0x20))?;
    c.emit(0x82a8e, I::Pop(0x8))?;
    c.emit(0x82a90, I::Call(0x835b0))?;
    c.emit(0x82a94, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x82a96, I::Shift(Shift::Lsr, R7, R0, 31))?;
    c.emit(0x82a98, I::Alu(Alu::Orr, R7, R1))?;
    c.emit(0x82a9a, I::Alu(Alu::And, R7, R4))?;
    c.emit(0x82a9c, I::Push(0x80))?;
    c.emit(0x82a9e, I::Pop(0x1))?;
    c.emit(0x82aa0, I::Push(0x20))?;
    c.emit(0x82aa2, I::Pop(0x2))?;
    c.emit(0x82aa4, I::Stack(true, R2, 16))?;
    c.emit(0x82aa6, I::Push(0x20))?;
    c.emit(0x82aa8, I::Pop(0x8))?;
    c.emit(0x82aaa, I::Call(0x835b0))?;
    c.emit(0x82aae, I::Alu(Alu::Neg, R0, R1))?;
    c.emit(0x82ab0, I::Alu(Alu::And, R0, R4))?;
    c.emit(0x82ab2, I::Push(0x20))?;
    c.emit(0x82ab4, I::Pop(0x2))?;
    c.emit(0x82ab6, I::Push(0x80))?;
    c.emit(0x82ab8, I::Pop(0x4))?;
    c.emit(0x82aba, I::Push(0x20))?;
    c.emit(0x82abc, I::Pop(0x8))?;
    c.emit(0x82abe, I::Call(0x835b0))?;
    c.emit(0x82ac2, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x82ac4, I::Shift(Shift::Lsr, R7, R0, 31))?;
    c.emit(0x82ac6, I::Alu(Alu::Orr, R7, R1))?;
    c.emit(0x82ac8, I::Alu(Alu::And, R7, R4))?;
    c.emit(0x82aca, I::Push(0x80))?;
    c.emit(0x82acc, I::Pop(0x1))?;
    c.emit(0x82ace, I::Push(0x20))?;
    c.emit(0x82ad0, I::Pop(0x2))?;
    c.emit(0x82ad2, I::Stack(true, R2, 16))?;
    c.emit(0x82ad4, I::Push(0x20))?;
    c.emit(0x82ad6, I::Pop(0x8))?;
    c.emit(0x82ad8, I::Call(0x835b0))?;
    c.emit(0x82adc, I::Alu(Alu::Neg, R0, R1))?;
    c.emit(0x82ade, I::Alu(Alu::And, R0, R4))?;
    c.emit(0x82ae0, I::Push(0x20))?;
    c.emit(0x82ae2, I::Pop(0x2))?;
    c.emit(0x82ae4, I::Push(0x80))?;
    c.emit(0x82ae6, I::Pop(0x4))?;
    c.emit(0x82ae8, I::Push(0x20))?;
    c.emit(0x82aea, I::Pop(0x8))?;
    c.emit(0x82aec, I::Call(0x835b0))?;
    c.emit(0x82af0, I::Shift(Shift::Lsl, R1, R1, 1))?;
    c.emit(0x82af2, I::Shift(Shift::Lsr, R0, R0, 31))?;
    c.emit(0x82af4, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x82af6, I::Small(true, R0, R0, 2))?;
    c.emit(0x82af8, I::Stack(true, R1, 8))?;
    c.emit(0x82afa, I::Shift(Shift::Lsl, R2, R1, 1))?;
    c.emit(0x82afc, I::Push(0x20))?;
    c.emit(0x82afe, I::Pop(0x2))?;
    c.emit(0x82b00, I::Push(0x20))?;
    c.emit(0x82b02, I::Pop(0x8))?;
    c.emit(0x82b04, I::Call(0x835b0))?;
    c.emit(0x82b08, I::Shift(Shift::Lsr, R2, R1, 24))?;
    c.emit(0x82b0a, I::Stack(true, R0, 24))?;
    c.emit(0x82b0c, I::Arithmetic(true, R0, R0, R6))?;
    c.emit(0x82b0e, I::Stack(true, R3, 28))?;
    c.emit(0x82b10, I::Arithmetic(false, R0, R3, R0))?;
    c.emit(0x82b12, I::Compare(R2, 0))?;
    c.emit(0x82b14, I::Branch(Some(Condition::Ne), 0x82b28))?;
    c.emit(0x82b16, I::Push(0x2))?;
    c.emit(0x82b18, I::Pop(0x4))?;
    c.emit(0x82b1a, I::Stack(true, R4, 4))?;
    c.emit(0x82b1c, I::Alu(Alu::Mul, R2, R4))?;
    c.emit(0x82b1e, I::Stack(true, R3, 12))?;
    c.emit(0x82b20, I::Shift(Shift::Lsl, R3, R3, 24))?;
    c.emit(0x82b22, I::Arithmetic(true, R3, R3, R2))?;
    c.emit(0x82b24, I::Small(true, R0, R0, 1))?;
    c.emit(0x82b26, I::Branch(None, 0x82b38))?;
    c.emit(0x82b28, I::Shift(Shift::Lsr, R1, R1, 1))?;
    c.emit(0x82b2a, I::Push(0x2))?;
    c.emit(0x82b2c, I::Pop(0x4))?;
    c.emit(0x82b2e, I::Stack(true, R4, 4))?;
    c.emit(0x82b30, I::Alu(Alu::Mul, R2, R4))?;
    c.emit(0x82b32, I::Stack(true, R3, 12))?;
    c.emit(0x82b34, I::Shift(Shift::Lsl, R3, R3, 23))?;
    c.emit(0x82b36, I::Arithmetic(true, R3, R3, R2))?;
    c.emit(0x82b38, I::Compare(R0, 128))?;
    c.emit(0x82b3a, I::Branch(Some(Condition::Lt), 0x82b42))?;
    c.emit(0x82b3c, I::Stack(true, R0, 40))?;
    c.emit(0x82b3e, I::Stack(true, R2, 32))?;
    c.emit(0x82b40, I::Branch(None, 0x82b68))?;
    c.emit(0x82b42, I::Mov(R2, 125))?;
    c.emit(0x82b44, I::Alu(Alu::Mvn, R2, R2))?;
    c.emit(0x82b46, I::Alu(Alu::Cmp, R0, R2))?;
    c.emit(0x82b48, I::Branch(Some(Condition::Ge), 0x82b4c))?;
    c.emit(0x82b4a, I::Branch(None, 0x82a0a))?;
    c.emit(0x82b4c, I::Stack(true, R2, 36))?;
    c.emit(0x82b4e, I::Alu(Alu::And, R1, R2))?;
    c.emit(0x82b50, I::Mov(R2, 127))?;
    c.emit(0x82b52, I::Shift(Shift::Lsl, R2, R2, 23))?;
    c.emit(0x82b54, I::Shift(Shift::Lsl, R0, R0, 23))?;
    c.emit(0x82b56, I::Arithmetic(false, R0, R0, R2))?;
    c.emit(0x82b58, I::Alu(Alu::Orr, R0, R1))?;
    c.emit(0x82b5a, I::Small(false, R2, R0, 1))?;
    c.emit(0x82b5c, I::Shift(Shift::Lsl, R1, R3, 1))?;
    c.emit(0x82b5e, I::Alu(Alu::Cmp, R1, R4))?;
    c.emit(0x82b60, I::Branch(Some(Condition::Hi), 0x82b66))?;
    c.emit(0x82b62, I::Push(0x1))?;
    c.emit(0x82b64, I::Pop(0x4))?;
    c.emit(0x82b66, I::Stack(true, R0, 32))?;
    c.emit(0x82b68, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x82b6a, I::Push(0x4))?;
    c.emit(0x82b6c, I::Pop(0x1))?;
    c.emit(0x82b6e, I::StackAdjust(false, 52))?;
    c.emit(0x82b70, I::Pop(0x80f0))?;
    c.emit(0x82b72, I::Align)?;
    c.pool(0x82b74, 0x007fffff)?;
    c.pool(0x82b78, 0x7fffffff)?;
    c.pool(0x82b7c, 0x7fc00000)?;
    c.pool(0x82b80, 0x7504f333)?;
    c.emit(0x82b84, I::Push(0x4010))?;
    c.emit(0x82b86, I::Memory(true, R1, R0, 0))?;
    c.emit(0x82b88, I::Mov(R2, 32))?;
    c.emit(0x82b8a, I::Compare(R1, 0))?;
    c.emit(0x82b8c, I::Branch(Some(Condition::Eq), 0x82bc4))?;
    c.emit(0x82b8e, I::Shift(Shift::Lsr, R2, R1, 1))?;
    c.emit(0x82b90, I::Alu(Alu::Orr, R2, R1))?;
    c.emit(0x82b92, I::Shift(Shift::Lsr, R3, R2, 2))?;
    c.emit(0x82b94, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x82b96, I::Shift(Shift::Lsr, R2, R3, 4))?;
    c.emit(0x82b98, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x82b9a, I::Shift(Shift::Lsr, R3, R2, 8))?;
    c.emit(0x82b9c, I::Alu(Alu::Orr, R3, R2))?;
    c.emit(0x82b9e, I::Shift(Shift::Lsr, R2, R3, 16))?;
    c.emit(0x82ba0, I::Alu(Alu::Orr, R2, R3))?;
    c.emit(0x82ba2, I::Alu(Alu::Mvn, R2, R2))?;
    c.emit(0x82ba4, I::Shift(Shift::Lsr, R3, R2, 1))?;
    c.emit(0x82ba6, I::Literal(R4, 0x82bd4))?;
    c.emit(0x82ba8, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x82baa, I::Arithmetic(true, R2, R2, R4))?;
    c.emit(0x82bac, I::Literal(R3, 0x82bd8))?;
    c.emit(0x82bae, I::Shift(Shift::Lsr, R4, R2, 2))?;
    c.emit(0x82bb0, I::Alu(Alu::And, R2, R3))?;
    c.emit(0x82bb2, I::Alu(Alu::And, R4, R3))?;
    c.emit(0x82bb4, I::Arithmetic(false, R2, R2, R4))?;
    c.emit(0x82bb6, I::Shift(Shift::Lsr, R3, R2, 4))?;
    c.emit(0x82bb8, I::Arithmetic(false, R2, R2, R3))?;
    c.emit(0x82bba, I::Literal(R3, 0x82bdc))?;
    c.emit(0x82bbc, I::Alu(Alu::And, R3, R2))?;
    c.emit(0x82bbe, I::Literal(R2, 0x82be0))?;
    c.emit(0x82bc0, I::Alu(Alu::Mul, R2, R3))?;
    c.emit(0x82bc2, I::Shift(Shift::Lsr, R2, R2, 24))?;
    c.emit(0x82bc4, I::Push(0x4))?;
    c.emit(0x82bc6, I::Pop(0x8))?;
    c.emit(0x82bc8, I::Subtract(R3, 8))?;
    c.emit(0x82bca, I::Alu(Alu::Lsl, R1, R3))?;
    c.emit(0x82bcc, I::Memory(false, R1, R0, 0))?;
    c.emit(0x82bce, I::Mov(R0, 9))?;
    c.emit(0x82bd0, I::Arithmetic(true, R0, R0, R2))?;
    c.emit(0x82bd2, I::Pop(0x8010))?;
    c.pool(0x82bd4, 0x55555555)?;
    c.pool(0x82bd8, 0x33333333)?;
    c.pool(0x82bdc, 0x0f0f0f0f)?;
    c.pool(0x82be0, 0x01010101)?;
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
        assert_eq!(bytes.len(), 604);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        for (start, end, digest) in [
            (START, END, "3a188140f14ba01f64771964558da06e561bb705c402da0d6d13d352df04198e"),
            (START, 0x82b72, "422bc1a46a93d529fae3e08376b7d4868e0bb2d16aaedd3e61e26e21b5a03a74"),
            (0x82b72, 0x82b74, "3e39ed78e50103716c788d4147df413950e5eba73c6e33acd923f15865795676"),
            (0x82b74, 0x82b84, "f5569c8f9bff68c22379dd61cce99a773bf3f28302c856bb7f63b273b780c743"),
            (0x82b84, 0x82bd4, "9dd23d40529d43456346906d8f92d0d58e25541657c156f6f311191da3b76200"),
            (0x82bd4, END, "7b520efa5bec97643152d100759ea3c6711e85bcb95af6f18463cd1ab759f4b8"),
        ] {
            assert_eq!(crate::wmfw_sha256::hex_digest(&bytes[start - START..end - START]), digest);
        }
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        assert_eq!(half(&bytes, 0x82b72), 0x46c0);
    }

    #[test]
    fn independently_decoded_control_flow_and_literals() {
        let bytes = fragment().unwrap();
        for (at, target, conditional) in [(0x829ca, 0x829d2, true),
            (0x829d0, 0x82a4e, true),
            (0x829e2, 0x829e8, true),
            (0x829e6, 0x82a1c, false),
            (0x829ea, 0x829f0, true),
            (0x829ee, 0x82a00, false),
            (0x829f4, 0x82a06, true),
            (0x829fc, 0x82a00, true),
            (0x829fe, 0x82b6a, false),
            (0x82a04, 0x82b6a, false),
            (0x82a08, 0x82a0e, true),
            (0x82a0c, 0x82b6a, false),
            (0x82a10, 0x82a22, true),
            (0x82a16, 0x82a1c, true),
            (0x82a20, 0x82b6a, false),
            (0x82a24, 0x82a2c, true),
            (0x82a2a, 0x82b6a, false),
            (0x82a34, 0x82a40, true),
            (0x82a42, 0x82a4c, true),
            (0x82b14, 0x82b28, true),
            (0x82b26, 0x82b38, false),
            (0x82b3a, 0x82b42, true),
            (0x82b40, 0x82b68, false),
            (0x82b48, 0x82b4c, true),
            (0x82b4a, 0x82a0a, false),
            (0x82b60, 0x82b66, true),
            (0x82b8c, 0x82bc4, true)] {
            let word = half(&bytes, at);
            assert_eq!(word & (if conditional { 0xf000 } else { 0xf800 }),
                if conditional { 0xd000 } else { 0xe000 });
            let shift = if conditional { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [(0x82a38, 0x82b84),
            (0x82a46, 0x82b84),
            (0x82a7a, 0x835b0),
            (0x82a90, 0x835b0),
            (0x82aaa, 0x835b0),
            (0x82abe, 0x835b0),
            (0x82ad8, 0x835b0),
            (0x82aec, 0x835b0),
            (0x82b04, 0x835b0)] {
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
        for (at, slot) in [(0x8298c, 0x82b74),
            (0x829a6, 0x82b78),
            (0x829f0, 0x82b7c),
            (0x82a68, 0x82b80),
            (0x82ba6, 0x82bd4),
            (0x82bac, 0x82bd8),
            (0x82bba, 0x82bdc),
            (0x82bbe, 0x82be0)] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn flag_setting_and_wrapping_instruction_forms() {
        let bytes = fragment().unwrap();
        for (at, instruction) in [
            (0x829a8, I::Small(false, R7, R3, 1)),
            (0x829c0, I::Alu(Alu::Mvn, R4, R4)),
            (0x82a7e, I::Alu(Alu::Neg, R0, R1)),
            (0x82aae, I::Alu(Alu::Neg, R0, R1)),
            (0x82adc, I::Alu(Alu::Neg, R0, R1)),
            (0x82b1c, I::Alu(Alu::Mul, R2, R4)),
            (0x82b30, I::Alu(Alu::Mul, R2, R4)),
            (0x82b54, I::Shift(Shift::Lsl, R0, R0, 23)),
            (0x82bca, I::Alu(Alu::Lsl, R1, R3)),
        ] {
            assert_eq!(half(&bytes, at).to_le_bytes().as_slice(), instruction.encode(at).unwrap());
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
        for at in [0x82b70, 0x82b72, 0x82b74, 0x82b82, 0x82bd2, 0x82bd4, END] {
            c.at = at;
            let before = (c.at, c.bytes.clone(), c.code, c.data, c.align);
            let instruction = if [0x82b70, 0x82b82, 0x82bd2].contains(&at) {
                I::Call(0x82b84)
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
            "3a188140f14ba01f64771964558da06e561bb705c402da0d6d13d352df04198e");
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
