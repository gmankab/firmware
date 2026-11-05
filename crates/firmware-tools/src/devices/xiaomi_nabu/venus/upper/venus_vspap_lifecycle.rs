use arm32::{
    branch, bx, data_immediate, data_register, mov_register, push_registers,
    transfer_byte, transfer_multiple, transfer_word, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7562c;
pub const END: usize = 0x75748;
pub const LIFECYCLE_START: usize = 0x76114;
pub const LIFECYCLE_END: usize = 0x761e0;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 276;
pub const DATA_BYTES: usize = 8;
pub const LIFECYCLE_CODE_BYTES: usize = 188;
pub const LIFECYCLE_DATA_BYTES: usize = 16;

const POOLS: [(usize, u32); 6] = [
    (0x75740, 0xf3a34),
    (0x75744, 0x75748),
    (0x76134, 0x10ceac),
    (0x761b0, 0xf3a34),
    (0x761b4, 0xf3a6a),
    (0x761b8, 0xe2788),
];
const CHECK: usize = 0x17b0;
const ALLOCATE: usize = 0x2efc;
const RELEASE: usize = 0x2f1c;
const QUEUE_INITIALIZE: usize = 0x35b30;
const QUEUE_DESTROY: usize = 0x35bec;
const QUEUE_PROCESS: usize = 0x35df4;
const LOG: usize = 0x2ec94;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct TypedA32 {
    at: usize,
    start: usize,
    end: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn new(start: usize, end: usize) -> Result<Self, String> {
        if !matches!((start, end), (START, END) | (LIFECYCLE_START, LIFECYCLE_END)) {
            return Err("invalid VspAp fragment extent".into());
        }
        Ok(Self { at: start, start, end, bytes: Vec::with_capacity(end - start) })
    }

    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VspAp address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        let pool = POOLS.iter().find(|(at, _)| *at == self.at).map(|(_, value)| *value);
        if !(self.start..self.end).contains(&self.at) || self.at % 4 != 0
            || if data { pool != Some(value) } else { pool.is_some() }
        {
            return Err(format!("invalid VspAp {} at {:#x}", if data { "data" } else { "code" }, self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn pool(&mut self, slot: usize, value: u32) -> Result<(), String> {
        self.check(slot)?;
        self.word(value, true)
    }

    fn immediate(&mut self, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn set(&mut self, rd: u8, value: u32) -> Result<(), String> {
        self.immediate(C::Al, Op::Mov, rd, 0, value)
    }

    fn mov(&mut self, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(C::Al, r(rd), r(rm)))
    }

    fn transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn byte_load(&mut self, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(C::Al, true, r(rd), r(rn), offset)?)
    }

    fn literal(&mut self, rd: u8, slot: usize) -> Result<(), String> {
        if !POOLS.iter().any(|(at, _)| *at == slot) || !(self.start..self.end).contains(&slot) {
            return Err("invalid VspAp literal target".into());
        }
        self.transfer(true, rd, 15, slot as i32 - self.at as i32 - 8)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn pop(&mut self, condition: C, registers: u16) -> Result<(), String> {
        self.emit(transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn finish(self) -> Result<Vec<u8>, String> {
        self.check(self.end)?;
        if self.bytes.len() != self.end - self.start {
            return Err("invalid VspAp byte count".into());
        }
        Ok(self.bytes)
    }
}

fn construct(c: &mut TypedA32) -> Result<(), String> {
    c.check(START)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | R::R11.bit() | R::LR.bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x10)?;
    c.immediate(C::Al, Op::Sub, 13, 13, 8)?;
    c.literal(7, 0x75740)?;
    c.mov(6, 0)?;
    c.mov(4, 1)?;
    c.set(1, 0x4d)?;
    c.mov(5, 2)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x200)?;
    c.immediate(C::Ne, Op::Mov, 0, 0, 1)?;
    c.mov(2, 7)?;
    c.branch(C::Al, CHECK, true)?;
    c.byte_load(0, 6, 0x15e)?;
    c.set(1, 0x4e)?;
    c.mov(2, 7)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x200)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.immediate(C::Ne, Op::Mov, 0, 0, 0)?;
    c.branch(C::Al, CHECK, true)?;
    c.set(0, 0x40)?;
    c.branch(C::Al, ALLOCATE, true)?;
    c.mov(7, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 7, 0)?;
    c.branch(C::Eq, 0x75720, false)?;
    c.transfer(false, 6, 7, 0xc)?;
    c.set(0, 1)?;
    c.set(3, 0)?;
    c.byte_load(1, 6, 0x15f)?;
    c.byte_load(2, 6, 0x160)?;
    c.transfer(false, 3, 13, 0)?;
    c.transfer(false, 0, 13, 4)?;
    c.immediate(C::Al, Op::Add, 0, 7, 0x10)?;
    c.set(3, 0)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.immediate(C::Al, Op::Add, 2, 1, 1)?;
    c.set(1, 0x44)?;
    c.branch(C::Al, QUEUE_INITIALIZE, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Eq, 0x756e0, false)?;
    c.literal(2, 0x75740)?;
    c.set(1, 0x61)?;
    c.set(0, 0)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x200)?;
    c.branch(C::Al, CHECK, true)?;
    c.check(0x756e0)?;
    c.transfer(false, 5, 7, 0x3c)?;
    c.transfer(true, 0, 6, 0x2f0)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x75714, false)?;
    c.set(0, 8)?;
    c.branch(C::Al, ALLOCATE, true)?;
    c.literal(2, 0x75740)?;
    c.set(1, 0x69)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.transfer(false, 0, 7, 4)?;
    c.immediate(C::Ne, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x200)?;
    c.branch(C::Al, CHECK, true)?;
    c.check(0x75714)?;
    c.transfer(false, 7, 4, 0)?;
    c.literal(4, 0x75744)?;
    c.branch(C::Al, 0x75734, false)?;
    c.check(0x75720)?;
    c.literal(2, 0x75740)?;
    c.set(0, 0)?;
    c.set(1, 0x254)?;
    c.set(4, 0)?;
    c.branch(C::Al, CHECK, true)?;
    c.check(0x75734)?;
    c.mov(0, 4)?;
    c.immediate(C::Al, Op::Sub, 13, 11, 0x10)?;
    c.pop(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | R::R11.bit() | R::PC.bit())?;
    c.pool(0x75740, 0xf3a34)?;
    c.pool(0x75744, 0x75748)
}

fn initialize(c: &mut TypedA32) -> Result<(), String> {
    c.check(LIFECYCLE_START)?;
    c.transfer(true, 1, 0, 0x3c)?;
    c.literal(2, 0x76134)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 3, false)?)?;
    c.immediate(C::Al, Op::Add, 1, 1, 0x2c)?;
    c.transfer(false, 1, 0, 8)?;
    c.set(1, 0)?;
    c.transfer(false, 1, 0, 0)?;
    c.emit(bx(C::Al, R::LR))?;
    c.pool(0x76134, 0x10ceac)
}

fn destroy(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x76138)?;
    let saved = r(4).bit() | r(5).bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 8)?;
    c.literal(2, 0x761b0)?;
    c.mov(4, 0)?;
    c.set(1, 0x81)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x200)?;
    c.immediate(C::Ne, Op::Mov, 0, 0, 1)?;
    c.branch(C::Al, CHECK, true)?;
    c.transfer(true, 5, 4, 0x34)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0)?;
    c.pop(C::Eq, (saved & !R::LR.bit()) | R::PC.bit())?;
    c.transfer(true, 0, 4, 0x118)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x7617c, false)?;
    c.transfer(true, 0, 5, 4)?;
    c.branch(C::Al, RELEASE, true)?;
    c.check(0x7617c)?;
    c.immediate(C::Al, Op::Add, 0, 5, 0x10)?;
    c.branch(C::Al, QUEUE_DESTROY, true)?;
    c.mov(0, 5)?;
    c.branch(C::Al, RELEASE, true)?;
    c.literal(1, 0x761b4)?;
    c.set(0, 0)?;
    c.transfer(false, 0, 4, 0x34)?;
    c.set(0, 4)?;
    c.branch(C::Al, LOG, true)?;
    c.literal(1, 0x761b8)?;
    c.set(0, 4)?;
    c.pop(C::Al, saved)?;
    c.branch(C::Al, LOG, false)?;
    c.pool(0x761b0, 0xf3a34)?;
    c.pool(0x761b4, 0xf3a6a)?;
    c.pool(0x761b8, 0xe2788)
}

fn drain(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x761bc)?;
    let saved = r(4).bit() | r(7).bit() | R::R11.bit() | R::LR.bit();
    c.emit(push_registers(C::Al, saved)?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 8)?;
    c.transfer(true, 0, 0, 0x34)?;
    c.immediate(C::Al, Op::Add, 4, 0, 0x10)?;
    c.check(0x761cc)?;
    c.mov(0, 4)?;
    c.branch(C::Al, QUEUE_PROCESS, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Ne, 0x761cc, false)?;
    c.pop(C::Al, (saved & !R::LR.bit()) | R::PC.bit())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32::new(START, END)?;
    construct(&mut c)?;
    c.finish()
}

pub fn lifecycle_fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32::new(LIFECYCLE_START, LIFECYCLE_END)?;
    initialize(&mut c)?;
    destroy(&mut c)?;
    drain(&mut c)?;
    c.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], start: usize, at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - start..at - start + 4].try_into().unwrap())
    }

    #[test]
    fn extents_counts_and_pools() {
        for (start, end, code, data, bytes) in [
            (START, END, CODE_BYTES, DATA_BYTES, fragment().unwrap()),
            (LIFECYCLE_START, LIFECYCLE_END, LIFECYCLE_CODE_BYTES, LIFECYCLE_DATA_BYTES, lifecycle_fragment().unwrap()),
        ] {
            assert_eq!(bytes.len(), end - start);
            assert_eq!(code + data, bytes.len());
            let pools: Vec<_> = POOLS.iter().filter(|(at, _)| (start..end).contains(at)).collect();
            assert_eq!(pools.len() * 4, data);
            assert_eq!((end - start) / 4 - pools.len(), code / 4);
            for &(at, value) in pools {
                assert_eq!(word(&bytes, start, at), value);
            }
        }
        assert_eq!(END - START, 284);
        assert_eq!(LIFECYCLE_END - LIFECYCLE_START, 204);
        assert!(END < LIFECYCLE_START);
    }

    #[test]
    fn guards_reject_without_mutating() {
        for (start, end) in [(START, END), (LIFECYCLE_START, LIFECYCLE_END)] {
            for at in (start..end).step_by(4).chain([start - 4, start + 1, end, end + 4, END + 4]) {
                let pool = POOLS.iter().find(|(slot, _)| *slot == at);
                let valid = (start..end).contains(&at) && at % 4 == 0;
                let mut c = TypedA32::new(start, end).unwrap();
                c.at = at;
                let ok = valid && pool.is_none();
                assert_eq!(c.emit(bx(C::Al, R::LR)).is_ok(), ok);
                assert_eq!(c.at, at + if ok { 4 } else { 0 });
                assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
                for (slot, value) in POOLS {
                    for candidate in [value, value ^ 1] {
                        let mut c = TypedA32::new(start, end).unwrap();
                        c.at = at;
                        let ok = valid && at == slot && candidate == value;
                        assert_eq!(c.pool(slot, candidate).is_ok(), ok);
                        assert_eq!(c.at, at + if ok { 4 } else { 0 });
                        assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
                    }
                }
            }
            let mut c = TypedA32::new(start, end).unwrap();
            assert!(c.literal(0, end).is_err());
            assert_eq!(c.at, start);
            assert!(c.bytes.is_empty());
            assert!(c.finish().is_err());
        }
        assert!(TypedA32::new(START, LIFECYCLE_END).is_err());
    }

    #[test]
    fn ownership_returns_tail_call_and_process_pointer() {
        let constructor = fragment().unwrap();
        let lifecycle = lifecycle_fragment().unwrap();
        for (bytes, start, at, condition, target, link) in [
            (&constructor, START, 0x7568c, C::Eq, 0x75720, false),
            (&constructor, START, 0x756c0, C::Al, QUEUE_INITIALIZE, true),
            (&constructor, START, 0x756c8, C::Eq, 0x756e0, false),
            (&constructor, START, 0x756ec, C::Ne, 0x75714, false),
            (&constructor, START, 0x7571c, C::Al, 0x75734, false),
            (&lifecycle, LIFECYCLE_START, 0x76170, C::Ne, 0x7617c, false),
            (&lifecycle, LIFECYCLE_START, 0x76178, C::Al, RELEASE, true),
            (&lifecycle, LIFECYCLE_START, 0x76180, C::Al, QUEUE_DESTROY, true),
            (&lifecycle, LIFECYCLE_START, 0x76188, C::Al, RELEASE, true),
            (&lifecycle, LIFECYCLE_START, 0x761ac, C::Al, LOG, false),
            (&lifecycle, LIFECYCLE_START, 0x761d0, C::Al, QUEUE_PROCESS, true),
            (&lifecycle, LIFECYCLE_START, 0x761d8, C::Ne, 0x761cc, false),
        ] {
            assert_eq!(word(bytes, start, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        for (at, rd, rn, offset) in [(0x75704, 0, 7, 4), (0x75714, 7, 4, 0)] {
            assert_eq!(word(&constructor, START, at), transfer_word(C::Al, false, r(rd), r(rn), offset).unwrap());
        }
        assert_eq!(word(&constructor, START, 0x7572c), data_immediate(C::Al, Op::Mov, r(4), r(0), 0).unwrap());
        assert_eq!(word(&constructor, START, 0x75734), mov_register(C::Al, r(0), r(4)));
        assert_eq!(word(&constructor, START, 0x75718), transfer_word(C::Al, true, r(4), R::PC, 0x24).unwrap());
        assert_eq!(word(&lifecycle, LIFECYCLE_START, 0x76194), transfer_word(C::Al, false, r(0), r(4), 0x34).unwrap());
        assert_eq!(word(&lifecycle, LIFECYCLE_START, 0x761c4), transfer_word(C::Al, true, r(0), r(0), 0x34).unwrap());
        assert_eq!(word(&lifecycle, LIFECYCLE_START, 0x761c8), data_immediate(C::Al, Op::Add, r(4), r(0), 0x10).unwrap());
        assert_eq!(word(&lifecycle, LIFECYCLE_START, 0x761cc), mov_register(C::Al, r(0), r(4)));
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        for (start, end, bytes) in [
            (START, END, fragment().unwrap()),
            (LIFECYCLE_START, LIFECYCLE_END, lifecycle_fragment().unwrap()),
        ] {
            assert_eq!(bytes, reference[start + FILE_MAPPING_DELTA..end + FILE_MAPPING_DELTA]);
        }
    }
}
