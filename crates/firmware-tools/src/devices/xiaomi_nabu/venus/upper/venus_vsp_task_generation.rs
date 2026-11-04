use arm32::{
    branch, data_immediate, data_register, mov_register, mov_shifted_register,
    multiply_register, push_registers, transfer_byte, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x753f4;
pub const END: usize = 0x7562c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 552;
pub const DATA_BYTES: usize = 16;
pub const POOL_START: usize = 0x7561c;
const ERROR_FORMAT_ADDRESS: usize = 0x7561c;
const ERROR_FORMAT_POINTER: u32 = 0xec05d;
const TASK_NAME_ADDRESS: usize = 0x75620;
const TASK_NAME_POINTER: u32 = 0xf3a29;
const NEWLINE_ADDRESS: usize = 0x75624;
const NEWLINE_POINTER: u32 = 0xe2788;
const SOURCE_PATH_ADDRESS: usize = 0x75628;
const SOURCE_PATH_POINTER: u32 = 0xf35fa;
const OBTAIN_OBJECT: u32 = 0x35c14;
const COPY_FIELDS: u32 = 0x471c;
const QUEUE_OBJECT: u32 = 0x35ca0;
const QUEUE_NOTIFICATION: u32 = 0x80928;
const DIAGNOSTIC: u32 = 0x2ec94;
const FAILURE_TAIL: u32 = 0x17b0;
const MAIN_RETURN: usize = 0x755fc;
const FAILURE_PATH: usize = 0x75600;
const RESTORE_STACK: u32 = 0x755f8;
const OPTIONAL_FIELDS_END: u32 = 0x75590;
const COPY_LOOP: u32 = 0x75530;
const COPY_LOOP_END: u32 = 0x75558;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VSP task address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("VSP task output address 0x{:x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if self.at >= POOL_START {
            return Err("instruction overlaps VSP task protected DATA".into());
        }
        self.word(instruction)
    }

    fn data(&mut self, address: usize, value: u32) -> Result<(), String> {
        self.check(address)?;
        let expected = match address {
            ERROR_FORMAT_ADDRESS => ERROR_FORMAT_POINTER,
            TASK_NAME_ADDRESS => TASK_NAME_POINTER,
            NEWLINE_ADDRESS => NEWLINE_POINTER,
            SOURCE_PATH_ADDRESS => SOURCE_PATH_POINTER,
            _ => return Err("not VSP task protected DATA".into()),
        };
        if value != expected {
            return Err("incorrect VSP task protected DATA".into());
        }
        self.word(value)
    }

    fn branch(&mut self, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target, link)?)
    }

    fn literal(&mut self, destination: R, address: usize) -> Result<(), String> {
        if !matches!(address, ERROR_FORMAT_ADDRESS | TASK_NAME_ADDRESS | NEWLINE_ADDRESS | SOURCE_PATH_ADDRESS) {
            return Err("not a VSP task literal".into());
        }
        self.emit(transfer_word(C::Al, true, destination, R::PC,
            address as i32 - self.at as i32 - 8)?)
    }
}

fn r(number: u8) -> R { R::new(number).unwrap() }
fn imm(c: &mut Code, op: Op, dst: R, src: R, value: u32) -> Result<(), String> {
    c.emit(data_immediate(C::Al, op, dst, src, value)?)
}
fn set(c: &mut Code, dst: R, value: u32) -> Result<(), String> {
    imm(c, Op::Mov, dst, R::R0, value)
}
fn cmp(c: &mut Code, src: R, value: u32) -> Result<(), String> {
    imm(c, Op::Cmp, R::R0, src, value)
}
fn mov(c: &mut Code, dst: R, src: R) -> Result<(), String> {
    c.emit(mov_register(C::Al, dst, src))
}
fn load(c: &mut Code, dst: R, base: R, offset: i32) -> Result<(), String> {
    c.emit(transfer_word(C::Al, true, dst, base, offset)?)
}
fn store(c: &mut Code, src: R, base: R, offset: i32) -> Result<(), String> {
    c.emit(transfer_word(C::Al, false, src, base, offset)?)
}
fn copy_word(c: &mut Code, src: R, dst: R, from: i32, to: i32) -> Result<(), String> {
    load(c, R::R1, src, from)?;
    store(c, R::R1, dst, to)
}
fn copy_byte(c: &mut Code, tmp: R, src: R, dst: R, from: i32, to: i32) -> Result<(), String> {
    c.emit(transfer_byte(C::Al, true, tmp, src, from)?)?;
    c.emit(transfer_byte(C::Al, false, tmp, dst, to)?)
}
fn saved() -> u16 {
    [4, 5, 6, 7, 8, 9, 11].into_iter().fold(0, |mask, n| mask | r(n).bit())
}
fn restore(c: &mut Code, return_register: R) -> Result<(), String> {
    c.emit(transfer_multiple(C::Al, true, R::SP, saved() | return_register.bit(),
        BlockMode::IncrementAfter, true)?)
}

fn obtain_and_populate(c: &mut Code) -> Result<(), String> {
    c.emit(push_registers(C::Al, saved() | R::LR.bit())?)?;
    imm(c, Op::Add, R::R11, R::SP, 0x18)?;
    imm(c, Op::Sub, R::SP, R::SP, 0x10)?;
    mov(c, r(4), R::R2)?;
    mov(c, r(7), R::R0)?;
    mov(c, r(8), R::R1)?;
    load(c, R::R0, r(4), 0x30)?;
    imm(c, Op::Add, R::R0, R::R0, 0x40)?;
    c.branch(C::Al, OBTAIN_OBJECT, true)?;
    mov(c, r(6), R::R0)?;
    cmp(c, r(6), 0)?;
    c.branch(C::Eq, FAILURE_PATH as u32, false)?;
    mov(c, R::R0, r(6))?;
    mov(c, R::R1, r(7))?;
    set(c, R::R2, 0x4c)?;
    c.branch(C::Al, COPY_FIELDS, true)?;
    load(c, R::R0, r(8), 0)?;
    set(c, R::R1, 0)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(6), 2)?)?;
    copy_byte(c, R::R0, r(7), r(6), 0xb8, 0xb8)?;
    store(c, R::R1, r(6), 0x170)?;
    cmp(c, R::R0, 1)?;
    for (from, to) in [(0xe4, 0x148), (0xe8, 0x14c), (0xec, 0x150),
        (0xf0, 0x154), (0xc8, 0x180)] {
        copy_word(c, r(7), r(6), from, to)?;
    }
    load(c, R::R1, r(7), 0xc0)?;
    load(c, R::R2, r(7), 0xc4)?;
    store(c, R::R2, r(6), 0x17c)?;
    store(c, R::R1, r(6), 0x178)?;
    c.emit(transfer_doubleword(C::Al, true, R::R2, r(7), 0x38)?)?;
    c.emit(transfer_doubleword(C::Al, false, R::R2, r(6), 0x38)?)?;
    for (from, to) in [(0x44, 0x44), (0x124, 0x184), (0x2c, 0x2c), (0x14, 0x14)] {
        copy_word(c, r(7), r(6), from, to)?;
    }
    copy_byte(c, R::R1, r(7), r(6), 0, 0)?;
    for (from, to) in [(0x1a0, 0x188), (0x1a4, 0x18c), (0x1a8, 0x190)] {
        copy_word(c, r(7), r(6), from, to)?;
    }
    copy_word(c, r(8), r(6), 0x138, 0x194)?;
    copy_byte(c, R::R1, r(7), r(6), 0x1ac, 0x198)?;
    c.branch(C::Ne, OPTIONAL_FIELDS_END, false)
}

fn bounded_field_copies(c: &mut Code) -> Result<(), String> {
    c.check(0x754e4)?;
    for (from, to) in [(0x128, 0x160), (0x110, 0x164), (0x118, 0x168)] {
        load(c, R::R0, r(7), from)?;
        store(c, R::R0, r(6), to)?;
    }
    load(c, R::R0, r(8), 0x10)?;
    store(c, R::R0, r(6), 0xc0)?;
    set(c, R::R0, 2)?;
    load(c, R::R1, r(4), 0x284)?;
    cmp(c, R::R1, 0x2000)?;
    load(c, R::R1, r(4), 0x298)?;
    c.emit(transfer_byte(C::Ne, true, R::R0, r(4), 0x3b0)?)?;
    c.emit(multiply_register(C::Al, R::R2, R::R1, R::R0)?)?;
    cmp(c, R::R2, 0)?;
    c.branch(C::Eq, COPY_LOOP_END, false)?;
    imm(c, Op::Add, R::R1, r(6), 0xc8)?;
    set(c, r(3), 0)?;
    set(c, R::R2, 0)?;
    c.check(COPY_LOOP as usize)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(8), r(3), Shift::Lsl, 2, false)?)?;
    imm(c, Op::Add, R::R2, R::R2, 1)?;
    load(c, r(5), r(5), 0x34)?;
    c.emit(mov_shifted_register(C::Al, r(5), r(5), Shift::Lsr, 3)?)?;
    c.emit(transfer_word_register(C::Al, false, r(5), R::R1, r(3), Shift::Lsl, 2)?)?;
    load(c, r(3), r(4), 0x298)?;
    c.emit(multiply_register(C::Al, r(5), r(3), R::R0)?)?;
    imm(c, Op::And, r(3), R::R2, 0xff)?;
    c.emit(data_register(C::Al, Op::Cmp, R::R0, r(5), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Hi, COPY_LOOP, false)?;
    c.check(COPY_LOOP_END as usize)?;
    c.emit(transfer_byte(C::Al, true, R::R0, r(7), 0x13c)?)?;
    imm(c, Op::Add, R::R1, r(7), 0x50)?;
    set(c, R::R2, 0x50)?;
    c.emit(transfer_byte(C::Al, false, R::R0, r(6), 0x16c)?)?;
    copy_byte(c, R::R0, r(7), r(6), 0x13d, 0x16d)?;
    imm(c, Op::Add, R::R0, r(6), 0x50)?;
    c.branch(C::Al, COPY_FIELDS, true)?;
    for (from, to) in [(0x18, 0x94), (0x1c, 0x98), (0x20, 0x9c)] {
        load(c, R::R0, r(8), from)?;
        store(c, R::R0, r(6), to)?;
    }
    Ok(())
}

fn queue_and_diagnose(c: &mut Code) -> Result<(), String> {
    c.check(OPTIONAL_FIELDS_END as usize)?;
    load(c, R::R0, r(4), 0x30)?;
    imm(c, Op::Add, R::R0, R::R0, 0x40)?;
    c.branch(C::Al, QUEUE_OBJECT, true)?;
    load(c, R::R0, r(4), 0x118)?;
    cmp(c, R::R0, 2)?;
    c.branch(C::Ne, RESTORE_STACK, false)?;
    load(c, R::R0, r(4), 0x10)?;
    load(c, R::R1, r(4), 0x114)?;
    store(c, R::R0, R::SP, 4)?;
    set(c, R::R0, 0x168)?;
    imm(c, Op::Orr, R::R0, R::R0, 0x1400)?;
    c.emit(transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    store(c, R::R0, R::SP, 8)?;
    imm(c, Op::Add, R::R0, R::R1, 0x58)?;
    imm(c, Op::Add, R::R1, R::SP, 4)?;
    c.branch(C::Al, QUEUE_NOTIFICATION, true)?;
    cmp(c, R::R0, 1)?;
    c.branch(C::Eq, RESTORE_STACK, false)?;
    c.literal(R::R1, ERROR_FORMAT_ADDRESS)?;
    c.literal(R::R2, TASK_NAME_ADDRESS)?;
    set(c, R::R0, 8)?;
    set(c, r(3), 0x1e4)?;
    c.branch(C::Al, DIAGNOSTIC, true)?;
    c.literal(R::R1, NEWLINE_ADDRESS)?;
    set(c, R::R0, 8)?;
    c.branch(C::Al, DIAGNOSTIC, true)?;
    c.check(RESTORE_STACK as usize)?;
    imm(c, Op::Sub, R::SP, R::R11, 0x18)?;
    c.check(MAIN_RETURN)?;
    restore(c, R::PC)?;
    c.check(FAILURE_PATH)?;
    c.literal(R::R2, SOURCE_PATH_ADDRESS)?;
    set(c, R::R1, 0x9d)?;
    set(c, R::R0, 0)?;
    imm(c, Op::Orr, R::R1, R::R1, 0x100)?;
    imm(c, Op::Sub, R::SP, R::R11, 0x18)?;
    restore(c, R::LR)?;
    c.check(0x75618)?;
    c.branch(C::Al, FAILURE_TAIL, false)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    obtain_and_populate(&mut c)?;
    bounded_field_copies(&mut c)?;
    queue_and_diagnose(&mut c)?;
    c.check(POOL_START)?;
    c.data(ERROR_FORMAT_ADDRESS, ERROR_FORMAT_POINTER)?;
    c.data(TASK_NAME_ADDRESS, TASK_NAME_POINTER)?;
    c.data(NEWLINE_ADDRESS, NEWLINE_POINTER)?;
    c.data(SOURCE_PATH_ADDRESS, SOURCE_PATH_POINTER)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], address: usize) -> u32 {
        u32::from_le_bytes(bytes[address - START..address - START + 4].try_into().unwrap())
    }

    #[test]
    fn extent_literals_and_failure_tail() {
        let bytes = fragment().unwrap();
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (552, 16, 568));
        assert_eq!(START + CODE_BYTES, POOL_START);
        assert_eq!(START + bytes.len(), END);
        for (address, value) in [(ERROR_FORMAT_ADDRESS, ERROR_FORMAT_POINTER),
            (TASK_NAME_ADDRESS, TASK_NAME_POINTER), (NEWLINE_ADDRESS, NEWLINE_POINTER),
            (SOURCE_PATH_ADDRESS, SOURCE_PATH_POINTER)] {
            assert_eq!(word(&bytes, address), value);
        }
        for (at, target) in [(0x755d8, ERROR_FORMAT_ADDRESS), (0x755dc, TASK_NAME_ADDRESS),
            (0x755ec, NEWLINE_ADDRESS), (FAILURE_PATH, SOURCE_PATH_ADDRESS)] {
            assert_eq!(at + 8 + (word(&bytes, at) as usize & 0xfff), target);
        }
        assert_eq!(word(&bytes, MAIN_RETURN), transfer_multiple(C::Al, true, R::SP,
            saved() | R::PC.bit(), BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x75614), transfer_multiple(C::Al, true, R::SP,
            saved() | R::LR.bit(), BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x75618), branch(C::Al, 0x75618, FAILURE_TAIL, false).unwrap());
        assert_eq!(word(&bytes, 0x75420), branch(C::Eq, 0x75420, FAILURE_PATH as u32, false).unwrap());
        assert_eq!(word(&bytes, 0x75554), branch(C::Hi, 0x75554, COPY_LOOP, false).unwrap());
    }

    #[test]
    fn protected_data_rejects_instructions_and_invalid_writes() {
        let mut c = Code { at: POOL_START, bytes: Vec::new() };
        assert!(c.emit(mov_register(C::Al, R::R0, R::R0)).is_err());
        assert!(c.data(ERROR_FORMAT_ADDRESS, 0).is_err());
        assert!(c.data(TASK_NAME_ADDRESS, TASK_NAME_POINTER).is_err());
        assert!(c.bytes.is_empty());
        c.data(ERROR_FORMAT_ADDRESS, ERROR_FORMAT_POINTER).unwrap();
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.data(START, ERROR_FORMAT_POINTER).is_err());
        assert!(c.data(ERROR_FORMAT_ADDRESS, ERROR_FORMAT_POINTER).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_original_byte_equality() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let original = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(original.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&original), identity.sha256);
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes, original[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
