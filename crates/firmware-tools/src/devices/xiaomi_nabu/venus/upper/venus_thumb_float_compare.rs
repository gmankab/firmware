pub const START: usize = 0x82100;
pub const END: usize = 0x823b0;
pub const CODE_BYTES: usize = 658;
pub const DATA_BYTES: usize = 24;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const ALIGN_BYTES: usize = 6;
const ALIGNMENTS: [usize; 3] = [0x821f2, 0x82342, 0x82392];
const POOLS: [(usize, u32); 6] = [
    (0x821f4, 0x7fffffff), (0x821f8, 0x7ff00000),
    (0x822d8, 0x7fffffff), (0x822dc, 0x7ff00000),
    (0x82344, 0x7fffffff), (0x82394, 0x7fffffff),
];

#[derive(Clone, Copy, Debug)]
enum R { R0, R1, R2, R3, R4, R5, R6, R7 }
use R::*;

#[derive(Clone, Copy, Debug)]
enum Alu { And, Eor, Cmp, Orr, Mvn }
#[derive(Clone, Copy, Debug)]
enum Condition { Eq = 0, Ne = 1, Lo = 3, Hi = 8, Ge = 10, Lt = 11, Gt = 12 }
#[derive(Clone, Copy, Debug)]
enum I {
    Push(u16), Pop(u16), StackAdjust(u16), Stack(bool, R, u16),
    Literal(R, usize), Alu(Alu, R, R), Lsl(R, R, u8),
    Mov(R, u16), Compare(R, u16), Branch(Option<Condition>, usize), Call(usize), Align,
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
        checked(at <= u32::MAX as usize && at % 2 == 0)?;
        let word = match self {
            Self::Push(mask) | Self::Pop(mask) => {
                let pop = matches!(self, Self::Pop(_));
                let extra = if pop { 0x8000 } else { 0x4000 };
                checked(mask != 0 && mask & !(0xff | extra) == 0)?;
                (if pop { 0xbc00 } else { 0xb400 }) | (mask & 0xff) | if mask & extra != 0 { 0x100 } else { 0 }
            }
            Self::StackAdjust(bytes) => {
                checked(bytes <= 508 && bytes % 4 == 0)?;
                0xb000 | bytes / 4
            }
            Self::Stack(load, r, offset) => {
                checked(offset <= 1020 && offset % 4 == 0)?;
                (if load { 0x9800 } else { 0x9000 }) | ((r as u16) << 8) | offset / 4
            }
            Self::Literal(r, target) => {
                let base = (at + 4) & !3;
                checked(target <= u32::MAX as usize && target % 4 == 0 && target >= base && target - base <= 1020)?;
                0x4800 | ((r as u16) << 8) | ((target - base) / 4) as u16
            }
            Self::Alu(op, dst, src) => {
                let op = match op { Alu::And => 0, Alu::Eor => 1, Alu::Cmp => 10, Alu::Orr => 12, Alu::Mvn => 15 };
                0x4000 | (op << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Lsl(dst, src, amount) => {
                checked(amount < 32)?;
                ((amount as u16) << 6) | ((src as u16) << 3) | dst as u16
            }
            Self::Mov(r, value) | Self::Compare(r, value) => {
                checked(value <= 255)?;
                (if matches!(self, Self::Mov(..)) { 0x2000 } else { 0x2800 }) | ((r as u16) << 8) | value
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
                return Ok([(first as u16).to_le_bytes(), (second as u16).to_le_bytes()].concat());
            }
            Self::Align => 0x4600 | (8 << 4) | (8 << 3),
        };
        Ok(word.to_le_bytes().to_vec())
    }
}

struct Code { at: usize, bytes: Vec<u8>, code: usize, data: usize, align: usize }
impl Code {
    fn new() -> Self { Self { at: START, bytes: Vec::new(), code: 0, data: 0, align: 0 } }
    fn emit(&mut self, at: usize, instruction: I) -> Result<(), String> {
        let bytes = instruction.encode(at)?;
        checked(at == self.at && at >= START && at + bytes.len() <= END)?;
        checked(!POOLS.iter().any(|&(slot, _)| at < slot + 4 && at + bytes.len() > slot))?;
        checked(matches!(instruction, I::Align) == ALIGNMENTS.contains(&at))?;
        checked(!ALIGNMENTS.iter().any(|&slot| at < slot + 2 && at + bytes.len() > slot && !matches!(instruction, I::Align)))?;
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
    c.emit(0x82100, I::Push(0x40fe))?;
    c.emit(0x82102, I::Stack(false, R3, 8))?;
    c.emit(0x82104, I::Stack(false, R2, 4))?;
    c.emit(0x82106, I::Mov(R3, 1))?;
    c.emit(0x82108, I::Mov(R2, 0))?;
    c.emit(0x8210a, I::Stack(false, R0, 0))?;
    c.emit(0x8210c, I::Compare(R0, 0))?;
    c.emit(0x8210e, I::Push(0x8))?;
    c.emit(0x82110, I::Pop(0x10))?;
    c.emit(0x82112, I::Branch(Some(Condition::Ne), 0x82118))?;
    c.emit(0x82114, I::Push(0x4))?;
    c.emit(0x82116, I::Pop(0x10))?;
    c.emit(0x82118, I::Literal(R5, 0x821f4))?;
    c.emit(0x8211a, I::Push(0x2))?;
    c.emit(0x8211c, I::Pop(0x40))?;
    c.emit(0x8211e, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x82120, I::Literal(R7, 0x821f8))?;
    c.emit(0x82122, I::Alu(Alu::Cmp, R6, R7))?;
    c.emit(0x82124, I::Push(0x8))?;
    c.emit(0x82126, I::Pop(0x1))?;
    c.emit(0x82128, I::Branch(Some(Condition::Hi), 0x8212e))?;
    c.emit(0x8212a, I::Push(0x4))?;
    c.emit(0x8212c, I::Pop(0x1))?;
    c.emit(0x8212e, I::Alu(Alu::Cmp, R6, R7))?;
    c.emit(0x82130, I::Branch(Some(Condition::Eq), 0x82136))?;
    c.emit(0x82132, I::Push(0x1))?;
    c.emit(0x82134, I::Pop(0x10))?;
    c.emit(0x82136, I::Stack(true, R0, 8))?;
    c.emit(0x82138, I::Alu(Alu::And, R5, R0))?;
    c.emit(0x8213a, I::Compare(R4, 0))?;
    c.emit(0x8213c, I::Branch(Some(Condition::Ne), 0x821ea))?;
    c.emit(0x8213e, I::Mov(R3, 1))?;
    c.emit(0x82140, I::Mov(R0, 0))?;
    c.emit(0x82142, I::Alu(Alu::Cmp, R5, R7))?;
    c.emit(0x82144, I::Push(0x8))?;
    c.emit(0x82146, I::Pop(0x10))?;
    c.emit(0x82148, I::Branch(Some(Condition::Hi), 0x8214e))?;
    c.emit(0x8214a, I::Push(0x1))?;
    c.emit(0x8214c, I::Pop(0x10))?;
    c.emit(0x8214e, I::Stack(true, R0, 4))?;
    c.emit(0x82150, I::Compare(R0, 0))?;
    c.emit(0x82152, I::Push(0x8))?;
    c.emit(0x82154, I::Pop(0x4))?;
    c.emit(0x82156, I::Branch(Some(Condition::Ne), 0x8215a))?;
    c.emit(0x82158, I::Stack(true, R2, 4))?;
    c.emit(0x8215a, I::Alu(Alu::Cmp, R5, R7))?;
    c.emit(0x8215c, I::Branch(Some(Condition::Eq), 0x82162))?;
    c.emit(0x8215e, I::Push(0x10))?;
    c.emit(0x82160, I::Pop(0x4))?;
    c.emit(0x82162, I::Compare(R2, 0))?;
    c.emit(0x82164, I::Branch(Some(Condition::Ne), 0x821ea))?;
    c.emit(0x82166, I::Stack(true, R0, 4))?;
    c.emit(0x82168, I::Stack(true, R2, 0))?;
    c.emit(0x8216a, I::Alu(Alu::Orr, R0, R2))?;
    c.emit(0x8216c, I::Alu(Alu::Orr, R5, R6))?;
    c.emit(0x8216e, I::Alu(Alu::Orr, R5, R0))?;
    c.emit(0x82170, I::Mov(R3, 0))?;
    c.emit(0x82172, I::Compare(R5, 0))?;
    c.emit(0x82174, I::Branch(Some(Condition::Eq), 0x821ea))?;
    c.emit(0x82176, I::Stack(true, R7, 8))?;
    c.emit(0x82178, I::Push(0x80))?;
    c.emit(0x8217a, I::Pop(0x1))?;
    c.emit(0x8217c, I::Alu(Alu::And, R0, R1))?;
    c.emit(0x8217e, I::Mov(R4, 0))?;
    c.emit(0x82180, I::Alu(Alu::Mvn, R3, R4))?;
    c.emit(0x82182, I::Compare(R0, 0))?;
    c.emit(0x82184, I::Branch(Some(Condition::Lt), 0x821b0))?;
    c.emit(0x82186, I::Mov(R4, 1))?;
    c.emit(0x82188, I::Mov(R2, 0))?;
    c.emit(0x8218a, I::Stack(true, R0, 0))?;
    c.emit(0x8218c, I::Stack(true, R5, 4))?;
    c.emit(0x8218e, I::Alu(Alu::Cmp, R0, R5))?;
    c.emit(0x82190, I::Push(0x10))?;
    c.emit(0x82192, I::Pop(0x20))?;
    c.emit(0x82194, I::Branch(Some(Condition::Lo), 0x8219a))?;
    c.emit(0x82196, I::Push(0x4))?;
    c.emit(0x82198, I::Pop(0x20))?;
    c.emit(0x8219a, I::Alu(Alu::Cmp, R1, R7))?;
    c.emit(0x8219c, I::Branch(Some(Condition::Lt), 0x821a2))?;
    c.emit(0x8219e, I::Push(0x4))?;
    c.emit(0x821a0, I::Pop(0x10))?;
    c.emit(0x821a2, I::Alu(Alu::Cmp, R1, R7))?;
    c.emit(0x821a4, I::Branch(Some(Condition::Eq), 0x821aa))?;
    c.emit(0x821a6, I::Push(0x10))?;
    c.emit(0x821a8, I::Pop(0x20))?;
    c.emit(0x821aa, I::Compare(R5, 0))?;
    c.emit(0x821ac, I::Branch(Some(Condition::Eq), 0x821d6))?;
    c.emit(0x821ae, I::Branch(None, 0x821ea))?;
    c.emit(0x821b0, I::Mov(R5, 1))?;
    c.emit(0x821b2, I::Stack(true, R0, 0))?;
    c.emit(0x821b4, I::Stack(true, R2, 4))?;
    c.emit(0x821b6, I::Alu(Alu::Cmp, R0, R2))?;
    c.emit(0x821b8, I::Push(0x20))?;
    c.emit(0x821ba, I::Pop(0x40))?;
    c.emit(0x821bc, I::Branch(Some(Condition::Hi), 0x821c2))?;
    c.emit(0x821be, I::Push(0x10))?;
    c.emit(0x821c0, I::Pop(0x40))?;
    c.emit(0x821c2, I::Alu(Alu::Cmp, R1, R7))?;
    c.emit(0x821c4, I::Branch(Some(Condition::Gt), 0x821ca))?;
    c.emit(0x821c6, I::Push(0x10))?;
    c.emit(0x821c8, I::Pop(0x20))?;
    c.emit(0x821ca, I::Alu(Alu::Cmp, R1, R7))?;
    c.emit(0x821cc, I::Branch(Some(Condition::Eq), 0x821d2))?;
    c.emit(0x821ce, I::Push(0x20))?;
    c.emit(0x821d0, I::Pop(0x40))?;
    c.emit(0x821d2, I::Compare(R6, 0))?;
    c.emit(0x821d4, I::Branch(Some(Condition::Ne), 0x821ea))?;
    c.emit(0x821d6, I::Stack(true, R0, 0))?;
    c.emit(0x821d8, I::Stack(true, R2, 4))?;
    c.emit(0x821da, I::Alu(Alu::Eor, R0, R2))?;
    c.emit(0x821dc, I::Stack(true, R2, 8))?;
    c.emit(0x821de, I::Alu(Alu::Eor, R1, R2))?;
    c.emit(0x821e0, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x821e2, I::Mov(R3, 0))?;
    c.emit(0x821e4, I::Compare(R1, 0))?;
    c.emit(0x821e6, I::Branch(Some(Condition::Eq), 0x821ea))?;
    c.emit(0x821e8, I::Mov(R3, 1))?;
    c.emit(0x821ea, I::Push(0x8))?;
    c.emit(0x821ec, I::Pop(0x1))?;
    c.emit(0x821ee, I::StackAdjust(12))?;
    c.emit(0x821f0, I::Pop(0x80f0))?;
    c.emit(0x821f2, I::Align)?;
    c.pool(0x821f4, 0x7fffffff)?;
    c.pool(0x821f8, 0x7ff00000)?;
    c.emit(0x821fc, I::Push(0x40fe))?;
    c.emit(0x821fe, I::Stack(false, R3, 8))?;
    c.emit(0x82200, I::Stack(false, R2, 4))?;
    c.emit(0x82202, I::Mov(R5, 1))?;
    c.emit(0x82204, I::Mov(R3, 0))?;
    c.emit(0x82206, I::Compare(R0, 0))?;
    c.emit(0x82208, I::Push(0x20))?;
    c.emit(0x8220a, I::Pop(0x4))?;
    c.emit(0x8220c, I::Branch(Some(Condition::Ne), 0x82212))?;
    c.emit(0x8220e, I::Push(0x8))?;
    c.emit(0x82210, I::Pop(0x4))?;
    c.emit(0x82212, I::Literal(R6, 0x822d8))?;
    c.emit(0x82214, I::Push(0x2))?;
    c.emit(0x82216, I::Pop(0x10))?;
    c.emit(0x82218, I::Alu(Alu::And, R4, R6))?;
    c.emit(0x8221a, I::Literal(R7, 0x822dc))?;
    c.emit(0x8221c, I::Alu(Alu::Cmp, R4, R7))?;
    c.emit(0x8221e, I::Branch(Some(Condition::Hi), 0x82224))?;
    c.emit(0x82220, I::Push(0x8))?;
    c.emit(0x82222, I::Pop(0x20))?;
    c.emit(0x82224, I::Alu(Alu::Cmp, R4, R7))?;
    c.emit(0x82226, I::Branch(Some(Condition::Eq), 0x8222c))?;
    c.emit(0x82228, I::Push(0x20))?;
    c.emit(0x8222a, I::Pop(0x4))?;
    c.emit(0x8222c, I::Stack(true, R5, 8))?;
    c.emit(0x8222e, I::Alu(Alu::And, R6, R5))?;
    c.emit(0x82230, I::Alu(Alu::Mvn, R3, R3))?;
    c.emit(0x82232, I::Compare(R2, 0))?;
    c.emit(0x82234, I::Branch(Some(Condition::Eq), 0x8223c))?;
    c.emit(0x82236, I::Push(0x8))?;
    c.emit(0x82238, I::Pop(0x10))?;
    c.emit(0x8223a, I::Branch(None, 0x822d0))?;
    c.emit(0x8223c, I::Stack(false, R3, 0))?;
    c.emit(0x8223e, I::Mov(R2, 1))?;
    c.emit(0x82240, I::Mov(R3, 0))?;
    c.emit(0x82242, I::Alu(Alu::Cmp, R6, R7))?;
    c.emit(0x82244, I::Push(0x4))?;
    c.emit(0x82246, I::Pop(0x20))?;
    c.emit(0x82248, I::Branch(Some(Condition::Hi), 0x8224e))?;
    c.emit(0x8224a, I::Push(0x8))?;
    c.emit(0x8224c, I::Pop(0x20))?;
    c.emit(0x8224e, I::Stack(true, R3, 4))?;
    c.emit(0x82250, I::Compare(R3, 0))?;
    c.emit(0x82252, I::Branch(Some(Condition::Ne), 0x82258))?;
    c.emit(0x82254, I::Push(0x8))?;
    c.emit(0x82256, I::Pop(0x4))?;
    c.emit(0x82258, I::Alu(Alu::Cmp, R6, R7))?;
    c.emit(0x8225a, I::Branch(Some(Condition::Eq), 0x82260))?;
    c.emit(0x8225c, I::Push(0x20))?;
    c.emit(0x8225e, I::Pop(0x4))?;
    c.emit(0x82260, I::Compare(R2, 0))?;
    c.emit(0x82262, I::Branch(Some(Condition::Eq), 0x82268))?;
    c.emit(0x82264, I::Stack(true, R4, 0))?;
    c.emit(0x82266, I::Branch(None, 0x822d0))?;
    c.emit(0x82268, I::Push(0x8))?;
    c.emit(0x8226a, I::Pop(0x4))?;
    c.emit(0x8226c, I::Alu(Alu::Orr, R2, R0))?;
    c.emit(0x8226e, I::Alu(Alu::Orr, R6, R4))?;
    c.emit(0x82270, I::Alu(Alu::Orr, R6, R2))?;
    c.emit(0x82272, I::Mov(R4, 0))?;
    c.emit(0x82274, I::Compare(R6, 0))?;
    c.emit(0x82276, I::Stack(true, R5, 8))?;
    c.emit(0x82278, I::Branch(Some(Condition::Eq), 0x822d0))?;
    c.emit(0x8227a, I::Push(0x20))?;
    c.emit(0x8227c, I::Pop(0x4))?;
    c.emit(0x8227e, I::Alu(Alu::And, R2, R1))?;
    c.emit(0x82280, I::Compare(R2, 0))?;
    c.emit(0x82282, I::Branch(Some(Condition::Lt), 0x8229a))?;
    c.emit(0x82284, I::Mov(R2, 1))?;
    c.emit(0x82286, I::Mov(R4, 0))?;
    c.emit(0x82288, I::Alu(Alu::Cmp, R0, R3))?;
    c.emit(0x8228a, I::Push(0x4))?;
    c.emit(0x8228c, I::Pop(0x8))?;
    c.emit(0x8228e, I::Branch(Some(Condition::Lo), 0x82294))?;
    c.emit(0x82290, I::Push(0x10))?;
    c.emit(0x82292, I::Pop(0x8))?;
    c.emit(0x82294, I::Alu(Alu::Cmp, R1, R5))?;
    c.emit(0x82296, I::Branch(Some(Condition::Ge), 0x822ae))?;
    c.emit(0x82298, I::Branch(None, 0x822b2))?;
    c.emit(0x8229a, I::Mov(R2, 1))?;
    c.emit(0x8229c, I::Mov(R4, 0))?;
    c.emit(0x8229e, I::Alu(Alu::Cmp, R0, R3))?;
    c.emit(0x822a0, I::Push(0x4))?;
    c.emit(0x822a2, I::Pop(0x8))?;
    c.emit(0x822a4, I::Branch(Some(Condition::Hi), 0x822aa))?;
    c.emit(0x822a6, I::Push(0x10))?;
    c.emit(0x822a8, I::Pop(0x8))?;
    c.emit(0x822aa, I::Alu(Alu::Cmp, R1, R5))?;
    c.emit(0x822ac, I::Branch(Some(Condition::Gt), 0x822b2))?;
    c.emit(0x822ae, I::Push(0x10))?;
    c.emit(0x822b0, I::Pop(0x4))?;
    c.emit(0x822b2, I::Alu(Alu::Cmp, R1, R5))?;
    c.emit(0x822b4, I::Branch(Some(Condition::Eq), 0x822ba))?;
    c.emit(0x822b6, I::Push(0x4))?;
    c.emit(0x822b8, I::Pop(0x8))?;
    c.emit(0x822ba, I::Compare(R3, 0))?;
    c.emit(0x822bc, I::Stack(true, R4, 0))?;
    c.emit(0x822be, I::Stack(true, R2, 4))?;
    c.emit(0x822c0, I::Branch(Some(Condition::Ne), 0x822d0))?;
    c.emit(0x822c2, I::Alu(Alu::Eor, R0, R2))?;
    c.emit(0x822c4, I::Alu(Alu::Eor, R1, R5))?;
    c.emit(0x822c6, I::Alu(Alu::Orr, R1, R0))?;
    c.emit(0x822c8, I::Mov(R4, 0))?;
    c.emit(0x822ca, I::Compare(R1, 0))?;
    c.emit(0x822cc, I::Branch(Some(Condition::Eq), 0x822d0))?;
    c.emit(0x822ce, I::Mov(R4, 1))?;
    c.emit(0x822d0, I::Push(0x10))?;
    c.emit(0x822d2, I::Pop(0x1))?;
    c.emit(0x822d4, I::StackAdjust(12))?;
    c.emit(0x822d6, I::Pop(0x80f0))?;
    c.pool(0x822d8, 0x7fffffff)?;
    c.pool(0x822dc, 0x7ff00000)?;
    c.emit(0x822e0, I::Push(0x4080))?;
    c.emit(0x822e2, I::Call(0x82100))?;
    c.emit(0x822e6, I::Pop(0x8080))?;
    c.emit(0x822e8, I::Push(0x4080))?;
    c.emit(0x822ea, I::Call(0x82100))?;
    c.emit(0x822ee, I::Pop(0x8080))?;
    c.emit(0x822f0, I::Push(0x4080))?;
    c.emit(0x822f2, I::Call(0x821fc))?;
    c.emit(0x822f6, I::Pop(0x8080))?;
    c.emit(0x822f8, I::Push(0x40b0))?;
    c.emit(0x822fa, I::Push(0x1))?;
    c.emit(0x822fc, I::Pop(0x4))?;
    c.emit(0x822fe, I::Literal(R0, 0x82344))?;
    c.emit(0x82300, I::Push(0x2))?;
    c.emit(0x82302, I::Pop(0x10))?;
    c.emit(0x82304, I::Alu(Alu::And, R4, R0))?;
    c.emit(0x82306, I::Alu(Alu::And, R0, R2))?;
    c.emit(0x82308, I::Mov(R3, 255))?;
    c.emit(0x8230a, I::Lsl(R5, R3, 23))?;
    c.emit(0x8230c, I::Mov(R3, 1))?;
    c.emit(0x8230e, I::Alu(Alu::Cmp, R0, R5))?;
    c.emit(0x82310, I::Branch(Some(Condition::Hi), 0x8233c))?;
    c.emit(0x82312, I::Alu(Alu::Cmp, R4, R5))?;
    c.emit(0x82314, I::Branch(Some(Condition::Hi), 0x8233c))?;
    c.emit(0x82316, I::Alu(Alu::Orr, R4, R0))?;
    c.emit(0x82318, I::Mov(R0, 0))?;
    c.emit(0x8231a, I::Compare(R4, 0))?;
    c.emit(0x8231c, I::Branch(Some(Condition::Eq), 0x82340))?;
    c.emit(0x8231e, I::Push(0x2))?;
    c.emit(0x82320, I::Pop(0x20))?;
    c.emit(0x82322, I::Alu(Alu::And, R5, R2))?;
    c.emit(0x82324, I::Mov(R4, 0))?;
    c.emit(0x82326, I::Alu(Alu::Mvn, R0, R4))?;
    c.emit(0x82328, I::Compare(R5, 0))?;
    c.emit(0x8232a, I::Branch(Some(Condition::Lt), 0x82332))?;
    c.emit(0x8232c, I::Alu(Alu::Cmp, R2, R1))?;
    c.emit(0x8232e, I::Branch(Some(Condition::Ge), 0x82336))?;
    c.emit(0x82330, I::Branch(None, 0x82340))?;
    c.emit(0x82332, I::Alu(Alu::Cmp, R2, R1))?;
    c.emit(0x82334, I::Branch(Some(Condition::Gt), 0x82340))?;
    c.emit(0x82336, I::Push(0x10))?;
    c.emit(0x82338, I::Pop(0x1))?;
    c.emit(0x8233a, I::Branch(Some(Condition::Eq), 0x82340))?;
    c.emit(0x8233c, I::Push(0x8))?;
    c.emit(0x8233e, I::Pop(0x1))?;
    c.emit(0x82340, I::Pop(0x80b0))?;
    c.emit(0x82342, I::Align)?;
    c.pool(0x82344, 0x7fffffff)?;
    c.emit(0x82348, I::Push(0x4070))?;
    c.emit(0x8234a, I::Push(0x1))?;
    c.emit(0x8234c, I::Pop(0x4))?;
    c.emit(0x8234e, I::Literal(R4, 0x82394))?;
    c.emit(0x82350, I::Push(0x2))?;
    c.emit(0x82352, I::Pop(0x20))?;
    c.emit(0x82354, I::Alu(Alu::And, R5, R4))?;
    c.emit(0x82356, I::Alu(Alu::And, R4, R2))?;
    c.emit(0x82358, I::Mov(R0, 255))?;
    c.emit(0x8235a, I::Lsl(R6, R0, 23))?;
    c.emit(0x8235c, I::Mov(R3, 0))?;
    c.emit(0x8235e, I::Alu(Alu::Mvn, R0, R3))?;
    c.emit(0x82360, I::Alu(Alu::Cmp, R4, R6))?;
    c.emit(0x82362, I::Branch(Some(Condition::Hi), 0x82390))?;
    c.emit(0x82364, I::Alu(Alu::Cmp, R5, R6))?;
    c.emit(0x82366, I::Branch(Some(Condition::Hi), 0x82390))?;
    c.emit(0x82368, I::Alu(Alu::Orr, R5, R4))?;
    c.emit(0x8236a, I::Compare(R5, 0))?;
    c.emit(0x8236c, I::Branch(Some(Condition::Ne), 0x82374))?;
    c.emit(0x8236e, I::Push(0x8))?;
    c.emit(0x82370, I::Pop(0x1))?;
    c.emit(0x82372, I::Branch(None, 0x82390))?;
    c.emit(0x82374, I::Push(0x2))?;
    c.emit(0x82376, I::Pop(0x10))?;
    c.emit(0x82378, I::Alu(Alu::And, R4, R2))?;
    c.emit(0x8237a, I::Compare(R4, 0))?;
    c.emit(0x8237c, I::Branch(Some(Condition::Lt), 0x82384))?;
    c.emit(0x8237e, I::Alu(Alu::Cmp, R2, R1))?;
    c.emit(0x82380, I::Branch(Some(Condition::Ge), 0x82388))?;
    c.emit(0x82382, I::Branch(None, 0x82390))?;
    c.emit(0x82384, I::Alu(Alu::Cmp, R2, R1))?;
    c.emit(0x82386, I::Branch(Some(Condition::Gt), 0x82390))?;
    c.emit(0x82388, I::Push(0x8))?;
    c.emit(0x8238a, I::Pop(0x1))?;
    c.emit(0x8238c, I::Branch(Some(Condition::Eq), 0x82390))?;
    c.emit(0x8238e, I::Mov(R0, 1))?;
    c.emit(0x82390, I::Pop(0x8070))?;
    c.emit(0x82392, I::Align)?;
    c.pool(0x82394, 0x7fffffff)?;
    c.emit(0x82398, I::Push(0x4080))?;
    c.emit(0x8239a, I::Call(0x822f8))?;
    c.emit(0x8239e, I::Pop(0x8080))?;
    c.emit(0x823a0, I::Push(0x4080))?;
    c.emit(0x823a2, I::Call(0x822f8))?;
    c.emit(0x823a6, I::Pop(0x8080))?;
    c.emit(0x823a8, I::Push(0x4080))?;
    c.emit(0x823aa, I::Call(0x82348))?;
    c.emit(0x823ae, I::Pop(0x8080))?;
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
        assert_eq!(bytes.len(), 688);
        assert_eq!(CODE_BYTES + DATA_BYTES + ALIGN_BYTES, END - START);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "e50cf384ea4096eb1e34f0afddf9b7511da0a893897eb61e57e17e9cdd6ba914");
        for (at, value) in POOLS {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
        }
        for at in ALIGNMENTS { assert_eq!(half(&bytes, at), 0x46c0); }
        for (at, slot) in [(0x82118, 0x821f4), (0x82120, 0x821f8),
            (0x82212, 0x822d8), (0x8221a, 0x822dc),
            (0x822fe, 0x82344), (0x8234e, 0x82394)] {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 255) * 4, slot);
        }
    }

    #[test]
    fn fixed_words_flags_and_unordered_results() {
        let bytes = fragment().unwrap();
        for (at, expected) in [(0x82100, 0xb5fe), (0x82106, 0x2301),
            (0x8211e, 0x402e), (0x8216a, 0x4310), (0x82180, 0x43e3),
            (0x821da, 0x4050), (0x821ee, 0xb003), (0x821f0, 0xbdf0),
            (0x821fc, 0xb5fe), (0x82230, 0x43db), (0x8223c, 0x9300),
            (0x822c4, 0x4069), (0x822f8, 0xb5b0), (0x8230a, 0x05dd),
            (0x8230c, 0x2301), (0x82326, 0x43e0), (0x8232c, 0x428a),
            (0x82336, 0xb410), (0x82338, 0xbc01), (0x8233a, 0xd001),
            (0x82348, 0xb570), (0x8235a, 0x05c6), (0x8235e, 0x43d8),
            (0x82388, 0xb408), (0x8238a, 0xbc01), (0x8238c, 0xd000),
            (0x82390, 0xbd70)] {
            assert_eq!(half(&bytes, at), expected, "{at:#x}");
        }
        for (start, end) in [(START, 0x821f2), (0x821fc, 0x822d8),
            (0x822e0, 0x82342), (0x82348, 0x82392), (0x82398, END)] {
            let mut at = start;
            while at < end {
                let word = half(&bytes, at);
                assert_ne!(word & 0xff00, 0xbf00, "unexpected IT/hint at {at:#x}");
                at += if word & 0xf800 == 0xf000 { 4 } else { 2 };
            }
            assert_eq!(at, end);
        }
    }

    #[test]
    fn branches_and_wrapper_calls() {
        let bytes = fragment().unwrap();
        for (at, target, condition) in [(0x8213c, 0x821ea, Some(1)),
            (0x82184, 0x821b0, Some(11)), (0x82194, 0x8219a, Some(3)),
            (0x821ae, 0x821ea, None), (0x8223a, 0x822d0, None),
            (0x82296, 0x822ae, Some(10)), (0x822ac, 0x822b2, Some(12)),
            (0x82310, 0x8233c, Some(8)), (0x8233a, 0x82340, Some(0)),
            (0x82362, 0x82390, Some(8)), (0x8238c, 0x82390, Some(0))] {
            let word = half(&bytes, at);
            if let Some(condition) = condition {
                assert_eq!(word & 0xff00, 0xd000 | condition << 8);
            } else { assert_eq!(word & 0xf800, 0xe000); }
            let shift = if condition.is_some() { 8 } else { 5 };
            let delta = ((word << shift) as i16 >> shift) as isize * 2;
            assert_eq!((at as isize + 4 + delta) as usize, target);
        }
        for (at, target) in [(0x822e2, 0x82100), (0x822ea, 0x82100),
            (0x822f2, 0x821fc), (0x8239a, 0x822f8),
            (0x823a2, 0x822f8), (0x823aa, 0x82348)] {
            assert_eq!(half(&bytes, at - 2), 0xb580);
            assert_eq!(half(&bytes, at + 4), 0xbd80);
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
    fn rejected_operands_and_layout_leave_buffer_unchanged() {
        for instruction in [I::Push(0), I::Push(0x8000), I::Pop(0x4000),
            I::StackAdjust(510), I::Stack(true, R0, 2), I::Stack(false, R1, 1024),
            I::Mov(R0, 256), I::Compare(R0, 256), I::Lsl(R0, R1, 32),
            I::Literal(R0, START), I::Literal(R0, START + 6),
            I::Literal(R0, START + 1028),
            I::Branch(Some(Condition::Eq), START + 260),
            I::Branch(Some(Condition::Ne), START - 254),
            I::Branch(None, START + 2052), I::Branch(None, START - 2046),
            I::Branch(None, START + 1), I::Call(START + 16_777_220)] {
            assert!(instruction.encode(START).is_err(), "{instruction:?}");
        }
        assert!(I::Mov(R0, 0).encode(START + 1).is_err());
        let mut c = Code::new();
        assert!(c.emit(START + 2, I::Mov(R0, 0)).is_err());
        assert!(c.emit(START, I::Align).is_err());
        assert!(c.pool(START, 0).is_err());
        assert_eq!((c.at, c.bytes.len(), c.code, c.data, c.align), (START, 0, 0, 0, 0));
        for at in ALIGNMENTS {
            c.at = at;
            assert!(c.emit(at, I::Mov(R0, 0)).is_err());
            assert!(c.emit(at - 2, I::Call(START)).is_err());
        }
        for (at, value) in POOLS {
            c.at = at;
            assert!(c.emit(at, I::Mov(R0, 0)).is_err());
            assert!(c.pool(at, value ^ 1).is_err());
        }
        c.at = END;
        assert!(c.emit(END, I::Mov(R0, 0)).is_err());
        assert_eq!((c.bytes.len(), c.code, c.data, c.align), (0, 0, 0, 0));
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
