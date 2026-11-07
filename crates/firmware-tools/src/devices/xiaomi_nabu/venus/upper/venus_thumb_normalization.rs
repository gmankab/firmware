pub const START: usize = 0x81d78;
pub const END: usize = 0x81e64;
pub const CODE_BYTES: usize = 220;
pub const DATA_BYTES: usize = 16;
pub const FILE_MAPPING_DELTA: usize = 0x4000;

const POOL_START: usize = 0x81e54;
const POOLS: [(usize, u32); 4] = [
    (0x81e54, 0x5555_5555),
    (0x81e58, 0x3333_3333),
    (0x81e5c, 0x0f0f_0f0f),
    (0x81e60, 0x0101_0101),
];
const BRANCHES: [(usize, usize); 6] = [
    (0x81da6, 0x81dac), (0x81dc0, 0x81dc6),
    (0x81dd8, 0x81e16), (0x81ddc, 0x81e52),
    (0x81e14, 0x81e52), (0x81e18, 0x81e50),
];
const LOADS: [(usize, usize); 8] = [
    (0x81df6, 0x81e54), (0x81dfc, 0x81e58),
    (0x81e0a, 0x81e5c), (0x81e0e, 0x81e60),
    (0x81e32, 0x81e54), (0x81e38, 0x81e58),
    (0x81e46, 0x81e5c), (0x81e4a, 0x81e60),
];

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Reg { R0, R1, R2, R3, R4, R5, R6, R7 }

impl Reg {
    fn bits(self) -> u16 { self as u16 }
}

#[derive(Clone, Copy)]
enum Alu { Ands, Lsls, Lsrs, Orrs, Muls, Mvns }

#[derive(Clone, Copy)]
enum Immediate { Movs, Cmp, Adds, Subs }

#[derive(Clone, Copy)]
enum Condition { Eq, Ge }

#[derive(Clone, Copy)]
enum Instruction {
    BxLr,
    Push(u16),
    Pop(u16),
    Ldm { base: Reg, registers: u8 },
    Immediate { op: Immediate, dst: Reg, value: u8 },
    Alu { op: Alu, dst: Reg, src: Reg },
    Arithmetic { subtract: bool, dst: Reg, lhs: Reg, rhs: Reg },
    Lsr { dst: Reg, src: Reg, amount: u8 },
    Store { src: Reg, base: Reg, offset: u8 },
    AddSp(u16),
    Branch { condition: Option<Condition>, target: usize },
    Call(usize),
    Literal { dst: Reg, slot: usize },
}

impl Instruction {
    fn encode(self, at: usize) -> Result<Vec<u16>, String> {
        if at & 1 != 0 || !(START..POOL_START).contains(&at) {
            return Err(format!("invalid Thumb instruction address {at:#x}"));
        }
        let word = match self {
            Self::BxLr => 0x4700 | (14 << 3),
            Self::Push(registers) | Self::Pop(registers) => {
                let pop = matches!(self, Self::Pop(_));
                let special = if pop { 1 << 15 } else { 1 << 14 };
                if registers == 0 || registers & !(0xff | special) != 0 {
                    return Err("invalid Thumb stack register list".into());
                }
                (if pop { 0xbc00 } else { 0xb400 }) | (registers & 0xff)
                    | if registers & special != 0 { 0x100 } else { 0 }
            }
            Self::Ldm { base, registers } => {
                if registers == 0 || registers & (1 << base.bits()) != 0 {
                    return Err("LDM requires nonempty list excluding writeback base".into());
                }
                0xc800 | (base.bits() << 8) | u16::from(registers)
            }
            Self::Immediate { op, dst, value } => {
                let operation = match op {
                    Immediate::Movs => 0, Immediate::Cmp => 1,
                    Immediate::Adds => 2, Immediate::Subs => 3,
                };
                0x2000 | (operation << 11) | (dst.bits() << 8) | u16::from(value)
            }
            Self::Alu { op, dst, src } => {
                let operation = match op {
                    Alu::Ands => 0, Alu::Lsls => 2, Alu::Lsrs => 3,
                    Alu::Orrs => 12, Alu::Muls => 13, Alu::Mvns => 15,
                };
                0x4000 | (operation << 6) | (src.bits() << 3) | dst.bits()
            }
            Self::Arithmetic { subtract, dst, lhs, rhs } => {
                0x1800 | (u16::from(subtract) << 9) | (rhs.bits() << 6)
                    | (lhs.bits() << 3) | dst.bits()
            }
            Self::Lsr { dst, src, amount } => {
                if !(1..=32).contains(&amount) { return Err("invalid LSR amount".into()); }
                0x0800 | (u16::from(amount & 31) << 6) | (src.bits() << 3) | dst.bits()
            }
            Self::Store { src, base, offset } => {
                if offset > 124 || offset & 3 != 0 { return Err("invalid STR offset".into()); }
                0x6000 | (u16::from(offset / 4) << 6) | (base.bits() << 3) | src.bits()
            }
            Self::AddSp(bytes) => {
                if bytes > 508 || bytes & 3 != 0 { return Err("invalid ADD SP offset".into()); }
                0xb000 | (bytes / 4)
            }
            Self::Branch { condition, target } => {
                if !BRANCHES.contains(&(at, target)) || target & 1 != 0 {
                    return Err("branch outside declared code edges".into());
                }
                let delta = target as i64 - (at as i64 + 4);
                if let Some(condition) = condition {
                    if !(-256..=254).contains(&delta) { return Err("conditional branch range".into()); }
                    let condition = match condition { Condition::Eq => 0, Condition::Ge => 10 };
                    0xd000 | (condition << 8) | ((delta / 2) as u16 & 0xff)
                } else {
                    if !(-2048..=2046).contains(&delta) { return Err("branch range".into()); }
                    0xe000 | ((delta / 2) as u16 & 0x7ff)
                }
            }
            Self::Call(target) => {
                if (at, target) != (0x81d8c, 0x81dd0) { return Err("undeclared call".into()); }
                let delta = target as i64 - (at as i64 + 4);
                if delta & 1 != 0 || !(-(1 << 24)..(1 << 24)).contains(&delta) {
                    return Err("BL alignment or range".into());
                }
                let bits = delta as u32;
                let s = (bits >> 24) & 1;
                let j1 = (!(bits >> 23) ^ s) & 1;
                let j2 = (!(bits >> 22) ^ s) & 1;
                return Ok(vec![0xf000 | ((s as u16) << 10) | ((bits >> 12) as u16 & 0x3ff),
                    0xd000 | ((j1 as u16) << 13) | ((j2 as u16) << 11) | ((bits >> 1) as u16 & 0x7ff)]);
            }
            Self::Literal { dst, slot } => {
                if !LOADS.contains(&(at, slot)) || !POOLS.iter().any(|&(p, _)| p == slot) {
                    return Err("undeclared literal reference".into());
                }
                let pc = (at + 4) & !3;
                let offset = slot.checked_sub(pc).ok_or("negative literal offset")?;
                if offset > 1020 || offset & 3 != 0 { return Err("literal range or alignment".into()); }
                0x4800 | (dst.bits() << 8) | (offset / 4) as u16
            }
        };
        Ok(vec![word])
    }
}

struct Code {
    bytes: Vec<u8>,
}

impl Code {
    fn new() -> Self { Self { bytes: Vec::with_capacity(END - START) } }
    fn at(&self) -> usize { START + self.bytes.len() }
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at() != at { return Err(format!("Thumb layout: expected {at:#x}, got {:#x}", self.at())); }
        Ok(())
    }
    fn emit(&mut self, at: usize, instruction: Instruction) -> Result<(), String> {
        self.check(at)?;
        let words = instruction.encode(at)?;
        if at + words.len() * 2 > POOL_START { return Err("instruction overlaps pool".into()); }
        for word in words { self.bytes.extend_from_slice(&word.to_le_bytes()); }
        Ok(())
    }
    fn pool(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if at & 3 != 0 || !POOLS.contains(&(at, value)) { return Err("undeclared pool word".into()); }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
}

fn normalization(c: &mut Code) -> Result<(), String> {
    use Reg::*;
    use Instruction::*;
    c.emit(0x81d7a, Push(0x40f8))?;
    c.emit(0x81d7c, Push(1 << 0))?;
    c.emit(0x81d7e, Pop(1 << 4))?;
    c.emit(0x81d80, Ldm { base: R4, registers: (1 << 5) | (1 << 6) })?;
    c.emit(0x81d82, Push(1 << 5))?;
    c.emit(0x81d84, Pop(1 << 0))?;
    c.emit(0x81d86, Push(1 << 6))?;
    c.emit(0x81d88, Pop(1 << 1))?;
    c.emit(0x81d8a, Immediate { op: self::Immediate::Subs, dst: R4, value: 8 })?;
    c.emit(0x81d8c, Call(0x81dd0))?;
    c.emit(0x81d90, Push(1 << 0))?;
    c.emit(0x81d92, Pop(1 << 2))?;
    c.emit(0x81d94, Immediate { op: self::Immediate::Subs, dst: R2, value: 11 })?;
    c.emit(0x81d96, Push(1 << 5))?;
    c.emit(0x81d98, Pop(1 << 3))?;
    c.emit(0x81d9a, Alu { op: self::Alu::Lsls, dst: R3, src: R2 })?;
    c.emit(0x81d9c, Immediate { op: self::Immediate::Movs, dst: R7, value: 0 })?;
    c.emit(0x81d9e, Push(1 << 0))?;
    c.emit(0x81da0, Pop(1 << 1))?;
    c.emit(0x81da2, Immediate { op: self::Immediate::Subs, dst: R1, value: 43 })?;
    c.emit(0x81da4, Immediate { op: self::Immediate::Cmp, dst: R1, value: 0 })?;
    c.emit(0x81da6, Branch { condition: Some(Condition::Ge), target: 0x81dac })?;
    c.emit(0x81da8, Push(1 << 3))?;
    c.emit(0x81daa, Pop(1 << 7))?;
    c.emit(0x81dac, Store { src: R7, base: R4, offset: 0 })?;
    c.emit(0x81dae, Alu { op: self::Alu::Lsls, dst: R6, src: R2 })?;
    c.emit(0x81db0, Immediate { op: self::Immediate::Movs, dst: R2, value: 43 })?;
    c.emit(0x81db2, Arithmetic { subtract: true, dst: R3, lhs: R2, rhs: R0 })?;
    c.emit(0x81db4, Push(1 << 5))?;
    c.emit(0x81db6, Pop(1 << 2))?;
    c.emit(0x81db8, Alu { op: self::Alu::Lsrs, dst: R2, src: R3 })?;
    c.emit(0x81dba, Alu { op: self::Alu::Orrs, dst: R2, src: R6 })?;
    c.emit(0x81dbc, Alu { op: self::Alu::Lsls, dst: R5, src: R1 })?;
    c.emit(0x81dbe, Immediate { op: self::Immediate::Cmp, dst: R1, value: 0 })?;
    c.emit(0x81dc0, Branch { condition: Some(Condition::Ge), target: 0x81dc6 })?;
    c.emit(0x81dc2, Push(1 << 2))?;
    c.emit(0x81dc4, Pop(1 << 5))?;
    c.emit(0x81dc6, Store { src: R5, base: R4, offset: 4 })?;
    c.emit(0x81dc8, Immediate { op: self::Immediate::Movs, dst: R1, value: 12 })?;
    c.emit(0x81dca, Arithmetic { subtract: true, dst: R0, lhs: R1, rhs: R0 })?;
    c.emit(0x81dcc, AddSp(4))?;
    c.emit(0x81dce, Pop(0x80f0))
}

fn leading_zero_word(c: &mut Code, start: usize, input: Reg) -> Result<(), String> {
    use Reg::*;
    use Instruction::*;
    c.emit(start, Lsr { dst: R0, src: input, amount: 1 })?;
    c.emit(start + 2, Alu { op: self::Alu::Orrs, dst: R0, src: input })?;
    for (offset, dst, src, amount) in [(4, R1, R0, 2), (8, R0, R1, 4), (12, R1, R0, 8), (16, R0, R1, 16)] {
        c.emit(start + offset, Lsr { dst, src, amount })?;
        c.emit(start + offset + 2, Alu { op: self::Alu::Orrs, dst, src })?;
    }
    c.emit(start + 20, Alu { op: self::Alu::Mvns, dst: R0, src: R0 })?;
    c.emit(start + 22, Lsr { dst: R1, src: R0, amount: 1 })?;
    c.emit(start + 24, Literal { dst: R2, slot: 0x81e54 })?;
    c.emit(start + 26, Alu { op: self::Alu::Ands, dst: R2, src: R1 })?;
    c.emit(start + 28, Arithmetic { subtract: true, dst: R0, lhs: R0, rhs: R2 })?;
    c.emit(start + 30, Literal { dst: R1, slot: 0x81e58 })?;
    c.emit(start + 32, Lsr { dst: R2, src: R0, amount: 2 })?;
    c.emit(start + 34, Alu { op: self::Alu::Ands, dst: R0, src: R1 })?;
    c.emit(start + 36, Alu { op: self::Alu::Ands, dst: R2, src: R1 })?;
    c.emit(start + 38, Arithmetic { subtract: false, dst: R0, lhs: R0, rhs: R2 })?;
    c.emit(start + 40, Lsr { dst: R1, src: R0, amount: 4 })?;
    c.emit(start + 42, Arithmetic { subtract: false, dst: R0, lhs: R0, rhs: R1 })?;
    c.emit(start + 44, Literal { dst: R1, slot: 0x81e5c })?;
    c.emit(start + 46, Alu { op: self::Alu::Ands, dst: R1, src: R0 })?;
    c.emit(start + 48, Literal { dst: R0, slot: 0x81e60 })?;
    c.emit(start + 50, Alu { op: self::Alu::Muls, dst: R0, src: R1 })?;
    c.emit(start + 52, Lsr { dst: R0, src: R0, amount: 24 })
}

fn leading_zero64(c: &mut Code) -> Result<(), String> {
    use Reg::*;
    use Instruction::*;
    c.emit(0x81dd0, Push(1 << 0))?;
    c.emit(0x81dd2, Pop(1 << 2))?;
    c.emit(0x81dd4, Immediate { op: self::Immediate::Movs, dst: R0, value: 32 })?;
    c.emit(0x81dd6, Immediate { op: self::Immediate::Cmp, dst: R1, value: 0 })?;
    c.emit(0x81dd8, Branch { condition: Some(Condition::Eq), target: 0x81e16 })?;
    c.emit(0x81dda, Immediate { op: self::Immediate::Cmp, dst: R1, value: 0 })?;
    c.emit(0x81ddc, Branch { condition: Some(Condition::Eq), target: 0x81e52 })?;
    leading_zero_word(c, 0x81dde, R1)?;
    c.emit(0x81e14, Branch { condition: None, target: 0x81e52 })?;
    c.emit(0x81e16, Immediate { op: self::Immediate::Cmp, dst: R2, value: 0 })?;
    c.emit(0x81e18, Branch { condition: Some(Condition::Eq), target: 0x81e50 })?;
    leading_zero_word(c, 0x81e1a, R2)?;
    c.emit(0x81e50, Immediate { op: self::Immediate::Adds, dst: R0, value: 32 })?;
    c.emit(0x81e52, BxLr)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new();
    c.emit(START, Instruction::BxLr)?;
    c.check(0x81d7a)?;
    normalization(&mut c)?;
    c.check(0x81dd0)?;
    leading_zero64(&mut c)?;
    c.check(POOL_START)?;
    if c.bytes.len() != CODE_BYTES { return Err("Thumb code extent mismatch".into()); }
    for (at, value) in POOLS { c.pool(at, value)?; }
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES { return Err("Thumb fragment extent mismatch".into()); }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn half(bytes: &[u8], at: usize) -> u16 {
        u16::from_le_bytes(bytes[at - START..at - START + 2].try_into().unwrap())
    }

    #[test]
    fn extents_and_fixed_architectural_words() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, FILE_MAPPING_DELTA), (0x81d78, 0x81e64, 0x4000));
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (220, 16, 236));
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!((0x81d7a - START, 0x81dd0 - 0x81d7a, POOL_START - 0x81dd0), (2, 86, 132));
        for (at, word) in [
            (0x81d78, 0x4770), (0x81d7a, 0xb5f8), (0x81d80, 0xcc60),
            (0x81d8c, 0xf000), (0x81d8e, 0xf820), (0x81d9a, 0x4093),
            (0x81da4, 0x2900), (0x81da6, 0xda01), (0x81dbe, 0x2900),
            (0x81dc0, 0xda01), (0x81dcc, 0xb001), (0x81dce, 0xbdf0),
            (0x81dd0, 0xb401), (0x81dd8, 0xd01d), (0x81ddc, 0xd039),
            (0x81df2, 0x43c0), (0x81e10, 0x4348), (0x81e14, 0xe01d),
            (0x81e18, 0xd01a), (0x81e50, 0x3020), (0x81e52, 0x4770),
        ] { assert_eq!(half(&bytes, at), word, "{at:#x}"); }
        for (at, value) in POOLS {
            assert_eq!(at & 3, 0);
            assert_eq!(u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap()), value);
        }
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "ed348d0b01a1f9e19eed94706a64088aa1734fa0858dd349943909bc231c1d19");
    }

    #[test]
    fn decode_control_flow_and_literal_addresses() {
        let bytes = fragment().unwrap();
        for (at, target) in BRANCHES {
            let word = half(&bytes, at);
            let delta = if word & 0xf800 == 0xe000 {
                ((word << 5) as i16 >> 4) as isize
            } else {
                assert_eq!(word & 0xf000, 0xd000);
                assert_eq!((word >> 8) & 15, if at < 0x81dd0 { 10 } else { 0 });
                (word as u8 as i8 as isize) * 2
            };
            assert_eq!((at as isize + 4 + delta) as usize, target);
            assert!((START..POOL_START).contains(&target));
        }
        let first = half(&bytes, 0x81d8c);
        let second = half(&bytes, 0x81d8e);
        let s = u32::from((first >> 10) & 1);
        let i1 = !(u32::from(second >> 13) ^ s) & 1;
        let i2 = !(u32::from(second >> 11) ^ s) & 1;
        let bits = (s << 24) | (i1 << 23) | (i2 << 22)
            | (u32::from(first & 0x3ff) << 12) | (u32::from(second & 0x7ff) << 1);
        let delta = ((bits << 7) as i32) >> 7;
        assert_eq!((0x81d90_i32 + delta) as usize, 0x81dd0);
        for (at, slot) in LOADS {
            let word = half(&bytes, at);
            assert_eq!(word & 0xf800, 0x4800);
            assert_eq!(((at + 4) & !3) + usize::from(word & 0xff) * 4, slot);
        }
        let mut at = START;
        let mut count = 0;
        while at < POOL_START {
            let word = half(&bytes, at);
            assert!(word & 0xff00 != 0xbf00);
            at += if word & 0xf800 >= 0xe800 { 4 } else { 2 };
            count += 1;
        }
        assert_eq!((at, count), (POOL_START, 109));
    }

    #[test]
    fn checked_encoders_and_pool_guards() {
        use Instruction::*;
        use Reg::*;
        for instruction in [Push(0), Push(1 << 15), Pop(1 << 14),
            Ldm { base: R4, registers: 0 }, Ldm { base: R4, registers: 1 << 4 },
            Lsr { dst: R0, src: R1, amount: 0 }, Lsr { dst: R0, src: R1, amount: 33 },
            Store { src: R0, base: R1, offset: 2 }, Store { src: R0, base: R1, offset: 128 },
            AddSp(2), AddSp(512), Call(0x81dd2),
            Branch { condition: None, target: POOL_START },
            Literal { dst: R0, slot: 0x81e54 },
        ] { assert!(instruction.encode(START).is_err()); }
        assert!(BxLr.encode(START + 1).is_err());
        assert!(BxLr.encode(POOL_START).is_err());
        assert_eq!(Lsr { dst: R0, src: R1, amount: 32 }.encode(START).unwrap(), vec![0x0808]);
        let mut c = Code::new();
        assert!(c.emit(START + 2, BxLr).is_err());
        assert!(c.pool(START, POOLS[0].1).is_err());
        assert!(c.bytes.is_empty());
        c.emit(START, BxLr).unwrap();
        normalization(&mut c).unwrap();
        assert_eq!(c.at(), 0x81dd0);
        leading_zero64(&mut c).unwrap();
        assert_eq!(c.at(), POOL_START);
        let before = c.bytes.clone();
        assert!(c.emit(POOL_START, BxLr).is_err());
        assert!(c.pool(POOL_START, 0).is_err());
        assert_eq!(c.bytes, before);
        for (at, value) in POOLS { c.pool(at, value).unwrap(); }
        assert_eq!(c.at(), END);
        assert!(c.pool(END, 0).is_err());
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
