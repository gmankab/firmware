use arm32::{branch, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0xfa0c;
pub const END: usize = 0xfce4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const POOL_START: usize = 0xfcdc;
const LITERALS: [(usize, u32); 2] = [(0xfcdc, 0xe3dfc), (0xfce0, 0xe2788)];

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, address: usize) -> Result<(), String> {
        if self.at == address { Ok(()) } else {
            Err(format!("descriptor-context cursor {:#x} != {address:#x}", self.at))
        }
    }

    fn word(&mut self, value: u32) {
        self.bytes.extend(value.to_le_bytes());
        self.at += 4;
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..POOL_START).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("invalid descriptor-context instruction at {:#x}", self.at));
        }
        self.word(instruction);
        Ok(())
    }

    fn data(&mut self, address: usize, value: u32) -> Result<(), String> {
        if self.at != address || !LITERALS.contains(&(address, value)) {
            return Err("invalid descriptor-context literal".into());
        }
        self.word(value);
        Ok(())
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }

    fn literal(&mut self, dst: R, target: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| address == target) {
            return Err("invalid descriptor-context literal target".into());
        }
        self.emit(transfer_word(C::Al, true, dst, R::PC, target as i32 - self.at as i32 - 8)?)
    }
}

fn r(n: u8) -> R { R::new(n).unwrap() }
fn imm(c: &mut Code, op: Op, dst: R, src: R, value: u32) -> Result<(), String> {
    c.emit(data_immediate(C::Al, op, dst, src, value)?)
}
fn set(c: &mut Code, dst: R, value: u32) -> Result<(), String> {
    imm(c, Op::Mov, dst, R::R0, value)
}
fn or(c: &mut Code, dst: R, value: u32) -> Result<(), String> {
    imm(c, Op::Orr, dst, dst, value)
}
fn mov(c: &mut Code, dst: R, src: R) -> Result<(), String> {
    c.emit(mov_register(C::Al, dst, src))
}
fn add(c: &mut Code, dst: R, left: R, right: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::Add, dst, left, right, Shift::Lsl, 0, false)?)
}
fn load(c: &mut Code, dst: R, base: R, offset: i32) -> Result<(), String> {
    c.emit(transfer_word(C::Al, true, dst, base, offset)?)
}
fn store(c: &mut Code, src: R, base: R, offset: i32) -> Result<(), String> {
    c.emit(transfer_word(C::Al, false, src, base, offset)?)
}
fn context_pointer(c: &mut Code, dst: R, base: R, offset: R) -> Result<(), String> {
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, dst, base, offset, Shift::Lsl, 0)?)
}
fn aligned_size(c: &mut Code, size: R, record: R) -> Result<(), String> {
    imm(c, Op::Add, r(0), size, 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    or(c, r(0), 1)?;
    store(c, r(0), record, 12)
}
fn saved() -> u16 {
    r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit()
        | r(9).bit() | R::R11.bit() | R::LR.bit()
}

fn first_pair(c: &mut Code) -> Result<(), String> {
    set(c, r(0), 0x228)?;
    mov(c, r(1), r(4))?;
    set(c, r(3), 0x20c)?;
    set(c, R::LR, 0x22c)?;
    load(c, r(2), r(4), 0xca4)?;
    load(c, r(12), r(4), 0xca0)?;
    load(c, r(9), r(4), 0xca8)?;
    load(c, r(5), r(4), 0x6c)?;
    load(c, r(8), r(4), 0xcac)?;
    set(c, r(7), 0x208)?;
    or(c, r(0), 0x1c00)?;
    or(c, r(3), 0x9400)?;
    or(c, R::LR, 0x1c00)?;
    or(c, r(7), 0x9400)?;
    context_pointer(c, r(0), r(1), r(0))?;
    add(c, r(5), r(9), r(5))?;
    store(c, r(3), r(0), 0)?;
    mov(c, r(3), r(4))?;
    context_pointer(c, r(6), r(3), R::LR)?;
    set(c, R::LR, 0x22)?;
    or(c, R::LR, 0x1000)?;
    store(c, R::LR, r(6), 0)?;
    store(c, r(7), r(0), 4)?;
    add(c, r(7), r(5), r(12))?;
    add(c, r(7), r(7), r(8))?;
    store(c, r(7), r(6), 4)?;
    set(c, r(7), 0x28c)?;
    or(c, r(7), 0x9400)?;
    store(c, r(7), r(0), 8)?;
    imm(c, Op::Add, r(7), r(0), 16)?;
    store(c, R::LR, r(6), 8)?;
    store(c, r(7), r(1), 0)?;
    set(c, r(1), 0x288)?;
    or(c, r(1), 0x9400)?;
    store(c, r(1), r(0), 12)?;
    imm(c, Op::Add, r(0), r(6), 16)?;
    store(c, r(0), r(3), 0)?;
    aligned_size(c, r(2), r(6))?;
    c.branch(C::Al, 0xfb64, false)?;
    c.check(0xfad0)?;
    set(c, r(0), 0x228)?;
    mov(c, r(1), r(4))?;
    set(c, r(3), 0x20c)?;
    load(c, r(2), r(4), 0xca0)?;
    load(c, r(12), r(4), 0xca8)?;
    load(c, R::LR, r(4), 0x6c)?;
    mov(c, r(5), r(4))?;
    set(c, r(6), 0x22)?;
    set(c, r(7), 0x208)?;
    or(c, r(0), 0x1c00)?;
    or(c, r(3), 0x9400)?;
    or(c, r(6), 0x1000)?;
    or(c, r(7), 0x9400)?;
    context_pointer(c, r(0), r(1), r(0))?;
    store(c, r(3), r(0), 0)?;
    set(c, r(3), 0x22c)?;
    or(c, r(3), 0x1c00)?;
    context_pointer(c, r(3), r(5), r(3))?;
    store(c, r(6), r(3), 0)?;
    store(c, r(7), r(0), 4)?;
    add(c, r(7), r(12), R::LR)?;
    store(c, r(7), r(3), 4)?;
    set(c, r(7), 0x28c)?;
    or(c, r(7), 0x9400)?;
    store(c, r(7), r(0), 8)?;
    imm(c, Op::Add, r(7), r(0), 16)?;
    store(c, r(6), r(3), 8)?;
    store(c, r(7), r(1), 0)?;
    set(c, r(1), 0x288)?;
    or(c, r(1), 0x9400)?;
    store(c, r(1), r(0), 12)?;
    imm(c, Op::Add, r(0), r(3), 16)?;
    store(c, r(0), r(5), 0)?;
    aligned_size(c, r(2), r(3))?;
    c.check(0xfb64)
}

fn second_pair(c: &mut Code) -> Result<(), String> {
    set(c, r(1), 0x228)?;
    mov(c, r(7), r(4))?;
    set(c, r(5), 0x20c)?;
    load(c, R::LR, r(4), 0xca0)?;
    load(c, r(3), r(4), 0xcac)?;
    add(c, r(0), r(2), r(0))?;
    set(c, r(12), 0x23)?;
    set(c, r(6), 0x208)?;
    or(c, r(1), 0x1c00)?;
    or(c, r(5), 0x9400)?;
    or(c, r(12), 0x1000)?;
    or(c, r(6), 0x9400)?;
    context_pointer(c, r(1), r(7), r(1))?;
    add(c, r(0), r(0), R::LR)?;
    mov(c, r(2), r(3))?;
    store(c, r(5), r(1), 0)?;
    set(c, r(5), 0x22c)?;
    or(c, r(5), 0x1c00)?;
    context_pointer(c, r(5), r(4), r(5))?;
    store(c, r(12), r(5), 0)?;
    store(c, r(6), r(1), 4)?;
    store(c, r(0), r(5), 4)?;
    second_tail(c, r(5), r(7), r(12), r(3), 0xfc20)?;
    c.branch(C::Al, 0xfcc0, false)?;
    c.check(0xfc3c)?;
    set(c, r(1), 0x228)?;
    mov(c, r(3), r(4))?;
    set(c, r(7), 0x20c)?;
    set(c, r(6), 0x23)?;
    set(c, r(5), 0x208)?;
    or(c, r(1), 0x1c00)?;
    or(c, r(7), 0x9400)?;
    or(c, r(6), 0x1000)?;
    or(c, r(5), 0x9400)?;
    context_pointer(c, r(1), r(3), r(1))?;
    store(c, r(7), r(1), 0)?;
    set(c, r(7), 0x22c)?;
    or(c, r(7), 0x1c00)?;
    context_pointer(c, r(7), r(4), r(7))?;
    store(c, r(6), r(7), 0)?;
    store(c, r(5), r(1), 4)?;
    store(c, r(0), r(7), 4)?;
    second_tail(c, r(7), r(3), r(6), r(2), 0xfca8)?;
    c.check(0xfcc0)
}

fn second_tail(c: &mut Code, record: R, slot: R, tag: R, size: R, literal_at: usize) -> Result<(), String> {
    set(c, r(0), 0x28c)?;
    or(c, r(0), 0x9400)?;
    store(c, r(0), r(1), 8)?;
    imm(c, Op::Add, r(0), r(1), 16)?;
    store(c, tag, record, 8)?;
    store(c, r(0), slot, 0)?;
    set(c, r(0), 0x288)?;
    or(c, r(0), 0x9400)?;
    store(c, r(0), r(1), 12)?;
    imm(c, Op::Add, r(0), record, 16)?;
    c.check(literal_at)?;
    c.literal(r(1), 0xfcdc)?;
    store(c, r(0), r(4), 0)?;
    aligned_size(c, size, record)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(push_registers(C::Al, saved())?)?;
    imm(&mut c, Op::Add, R::R11, R::SP, 24)?;
    mov(&mut c, r(4), r(1))?;
    load(&mut c, r(0), r(4), 0xa58)?;
    load(&mut c, r(0), r(0), 0x330)?;
    imm(&mut c, Op::Cmp, r(0), r(0), 0)?;
    c.branch(C::Eq, 0xfad0, false)?;
    first_pair(&mut c)?;
    for address in [0xfcdc, 0xfce0] {
        c.literal(r(1), address)?;
        set(&mut c, r(0), 1)?;
        c.branch(C::Al, 0x2ec94, true)?;
    }
    load(&mut c, r(0), r(4), 0xc98)?;
    imm(&mut c, Op::Cmp, r(0), r(0), 1)?;
    c.branch(C::Ne, 0xfcd4, false)?;
    load(&mut c, r(1), r(4), 0xa58)?;
    load(&mut c, r(2), r(4), 0xca8)?;
    load(&mut c, r(0), r(4), 0x6c)?;
    load(&mut c, r(1), r(1), 0x330)?;
    imm(&mut c, Op::Cmp, r(0), r(1), 0)?;
    c.branch(C::Eq, 0xfc3c, false)?;
    second_pair(&mut c)?;
    set(&mut c, r(0), 1)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0xfce0)?;
    set(&mut c, r(0), 1)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xfcd4)?;
    set(&mut c, r(0), 1)?;
    c.check(0xfcd8)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, (saved() & !R::LR.bit()) | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(POOL_START)?;
    for (address, value) in LITERALS { c.data(address, value)?; }
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn structure() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 0x2d8);
        for (address, value) in LITERALS { assert_eq!(word(&bytes, address), value); }
        assert_eq!(word(&bytes, 0xfcd8), transfer_multiple(C::Al, true, R::SP,
            (saved() & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true).unwrap());
        for (at, dst, base, offset) in [(0xfa60, 0, 1, 0), (0xfa70, 6, 3, 14),
            (0xfb04, 0, 1, 0), (0xfb14, 3, 5, 3), (0xfbd0, 1, 7, 1),
            (0xfbe8, 5, 4, 5), (0xfc60, 1, 3, 1), (0xfc70, 7, 4, 7)] {
            assert_eq!(word(&bytes, at), transfer_word_register_preindexed_writeback(
                C::Al, true, r(dst), r(base), r(offset), Shift::Lsl, 0).unwrap());
        }
        for (at, dst, src) in [(0xfa9c, 7, 0), (0xfab4, 0, 6), (0xfb34, 7, 0),
            (0xfb4c, 0, 3), (0xfc04, 0, 1), (0xfc1c, 0, 5), (0xfc8c, 0, 1), (0xfca4, 0, 7)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Add, r(dst), r(src), 16).unwrap());
        }
        for (at, target, condition) in [(0xfa24, 0xfad0, C::Eq), (0xfacc, 0xfb64, C::Al),
            (0xfb84, 0xfcd4, C::Ne), (0xfb9c, 0xfc3c, C::Eq), (0xfc38, 0xfcc0, C::Al)] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        for (at, target) in [(0xfb64, 0xfcdc), (0xfb70, 0xfce0), (0xfc20, 0xfcdc),
            (0xfca8, 0xfcdc), (0xfcc8, 0xfce0)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(1), R::PC,
                target as i32 - at as i32 - 8).unwrap());
        }
    }

    #[test]
    fn exact_data_guards() {
        let mut c = Code { at: POOL_START, bytes: Vec::new() };
        assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
        assert!(c.data(POOL_START, 0).is_err());
        assert!(c.data(0xfce0, 0xe2788).is_err());
        assert!(c.bytes.is_empty());
        c.data(POOL_START, 0xe3dfc).unwrap();
        c.data(0xfce0, 0xe2788).unwrap();
        assert!(c.data(END, 0).is_err());
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.data(START, 0xe3dfc).is_err());
        assert!(c.data(POOL_START, 0xe3dfc).is_err());
        assert!(c.literal(r(1), START).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_original_byte_equality() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let original = std::fs::read(path).unwrap();
        assert_eq!(original.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&original),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), original[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
