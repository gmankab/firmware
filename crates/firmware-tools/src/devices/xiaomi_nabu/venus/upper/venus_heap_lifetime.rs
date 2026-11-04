use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
    branch, data_immediate, data_register, load_halfword, mov_register, push_registers,
    store_halfword, transfer_byte, transfer_byte_preindexed_writeback, transfer_halfword_register,
    transfer_multiple, transfer_word, transfer_word_register,
};

pub const START: usize = 0xadb0;
pub const END: usize = 0xb1b0;
#[cfg(test)]
const FILE_MAPPING_DELTA: usize = 0x4000;
const CODE_BYTES: usize = 984;
const DATA_BYTES: usize = 40;
const CONTEXT_SIZE: u32 = 0x23c;
const DIAGNOSTIC: u32 = 0xe37d7;
const ACTIVE_HEAP_COUNT: u32 = 0x103ba2;
const HEAP_COOKIES: u32 = 0x103ae8;
const ALLOCATE_BLOCK: u32 = 0xa99c;
const RELEASE_BLOCK: u32 = 0xa9d4;
const FREE_HEAP_IDS: u32 = 0x103b64;
const REPORT_FAILURE: u32 = 0x17b0;
const CAPTURE_FAILURE: u32 = 0x86644;
const CLEAR_MEMORY: u32 = 0x31ec;
const CREATE_ARENA_LOCK: u32 = 0x8634c;
const DESTROY_ARENA_LOCK: u32 = 0x8636c;
const NEXT_COOKIE: u32 = 0x257b0;
const PROTECTED_DATA: [(usize, u32); 10] = [
    (0xb04c, DIAGNOSTIC),
    (0xb050, ACTIVE_HEAP_COUNT),
    (0xb054, HEAP_COOKIES),
    (0xb058, ALLOCATE_BLOCK),
    (0xb05c, RELEASE_BLOCK),
    (0xb060, FREE_HEAP_IDS),
    (0xb0fc, DIAGNOSTIC),
    (0xb1a4, ACTIVE_HEAP_COUNT),
    (0xb1a8, FREE_HEAP_IDS),
    (0xb1ac, DIAGNOSTIC),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if self.at != at || !(START..END).contains(&at) || at % 4 != 0
            || self.bytes.len() != at - START
            || PROTECTED_DATA.iter().any(|&(address, _)| address == at)
        {
            return Err(format!("heap lifetime instruction at {at:#x}, cursor {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        if self.at != at || !PROTECTED_DATA.contains(&(at, value))
            || self.bytes.len() != at - START
        {
            return Err(format!("invalid heap lifetime data at {at:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn immediate(&mut self, at: usize, condition: C, op: Op, dst: u8, src: u8, value: u32) -> Result<(), String> {
        self.emit(at, data_immediate(condition, op, r(dst), r(src), value)?)
    }

    fn mov(&mut self, at: usize, dst: u8, src: u8) -> Result<(), String> {
        self.emit(at, mov_register(C::Al, r(dst), r(src)))
    }

    fn word(&mut self, at: usize, load: bool, value: u8, base: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_word(C::Al, load, r(value), r(base), offset)?)
    }

    fn byte(&mut self, at: usize, load: bool, value: u8, base: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_byte(C::Al, load, r(value), r(base), offset)?)
    }

    fn halfword(&mut self, at: usize, load: bool, value: u8, base: u8, offset: i32) -> Result<(), String> {
        self.emit(at, if load {
            load_halfword(C::Al, r(value), r(base), offset)?
        } else {
            store_halfword(C::Al, r(value), r(base), offset)?
        })
    }

    fn jump(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }

    fn literal_load(&mut self, at: usize, dst: u8, target: usize) -> Result<(), String> {
        if !PROTECTED_DATA.iter().any(|&(address, _)| address == target) {
            return Err(format!("unknown heap lifetime literal {target:#x}"));
        }
        self.word(at, true, dst, 15, target as i32 - at as i32 - 8)
    }

    fn restore(&mut self, at: usize, registers: u16) -> Result<(), String> {
        self.emit(at, transfer_multiple(C::Al, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn assertion(&mut self, at: usize, line: u32, diagnostic: usize) -> Result<(), String> {
        self.jump(at, C::Al, CAPTURE_FAILURE, true)?;
        self.literal_load(at + 4, 2, diagnostic)?;
        self.immediate(at + 8, C::Al, Op::Mov, 1, 0, line & 0xff)?;
        self.immediate(at + 12, C::Al, Op::Mov, 0, 0, 0)?;
        self.immediate(at + 16, C::Al, Op::Orr, 1, 1, line & 0xf00)?;
        self.jump(at + 20, C::Al, REPORT_FAILURE, true)
    }
}

fn initialize_heap(c: &mut Code) -> Result<(), String> {
    c.emit(0xadb0, push_registers(C::Al, 0x4ff0)?)?;
    c.immediate(0xadb4, C::Al, Op::Add, 11, 13, 0x1c)?;
    c.immediate(0xadb8, C::Al, Op::Sub, 13, 13, 4)?;
    c.mov(0xadbc, 4, 0)?;
    c.mov(0xadc0, 5, 3)?;
    c.mov(0xadc4, 6, 2)?;
    c.mov(0xadc8, 7, 1)?;
    c.immediate(0xadcc, C::Al, Op::Cmp, 0, 4, 0)?;
    c.jump(0xadd0, C::Ne, 0xadec, false)?;
    c.assertion(0xadd4, 0x4c5, 0xb04c)?;
    c.literal_load(0xadec, 9, 0xb050)?;
    c.halfword(0xadf0, true, 0, 9, 0)?;
    c.immediate(0xadf4, C::Al, Op::Cmp, 0, 0, 0x1f)?;
    c.jump(0xadf8, C::Cc, 0xae14, false)?;
    c.assertion(0xadfc, 0x4c7, 0xb04c)?;
    c.word(0xae14, true, 0, 4, 0)?;
    c.immediate(0xae18, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xae1c, C::Eq, 0xae38, false)?;
    c.immediate(0xae20, C::Al, Op::Mov, 1, 0, 0x238)?;
    c.literal_load(0xae24, 2, 0xb054)?;
    c.emit(0xae28, transfer_halfword_register(C::Al, true, r(1), r(4), r(1))?)?;
    c.emit(0xae2c, transfer_word_register(C::Al, true, r(1), r(2), r(1), Shift::Lsl, 2)?)?;
    c.emit(0xae30, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.jump(0xae34, C::Eq, 0xb044, false)?;
    c.mov(0xae38, 0, 4)?;
    c.immediate(0xae3c, C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(0xae40, C::Al, Op::Mov, 2, 0, CONTEXT_SIZE)?;
    c.jump(0xae44, C::Al, CLEAR_MEMORY, true)?;
    c.immediate(0xae48, C::Al, Op::Mov, 0, 0, 2)?;
    c.immediate(0xae4c, C::Al, Op::Cmp, 0, 6, 0x20000)?;
    c.immediate(0xae50, C::Cc, Op::Mov, 0, 0, 0)?;
    c.immediate(0xae54, C::Al, Op::Cmp, 0, 7, 0)?;
    c.word(0xae58, false, 0, 4, 0x230)?;
    c.jump(0xae5c, C::Ne, 0xae78, false)?;
    c.assertion(0xae60, 0x4d7, 0xb04c)?;
    c.immediate(0xae78, C::Al, Op::Cmp, 0, 6, 0)?;
    c.word(0xae7c, false, 5, 13, 0)?;
    c.jump(0xae80, C::Eq, 0xae90, false)?;
    c.immediate(0xae84, C::Al, Op::Cmp, 0, 6, 0x1e)?;
    c.jump(0xae88, C::Ls, 0xaea8, false)?;
    c.jump(0xae8c, C::Al, 0xaec0, false)?;
    c.assertion(0xae90, 0x4d8, 0xb04c)?;
    c.assertion(0xaea8, 0x4d9, 0xb04c)?;
    c.immediate(0xaec0, C::Al, Op::Add, 5, 7, 0xe)?;
    c.immediate(0xaec4, C::Al, Op::Sub, 0, 5, 0xe)?;
    c.immediate(0xaec8, C::Al, Op::Add, 5, 5, 1)?;
    c.immediate(0xaecc, C::Al, Op::Tst, 0, 0, 0xf)?;
    c.jump(0xaed0, C::Ne, 0xaec4, false)?;
    c.immediate(0xaed4, C::Al, Op::Sub, 0, 5, 0xf)?;
    c.word(0xaed8, false, 0, 4, 0x2c)?;
    c.jump(0xaedc, C::Al, CREATE_ARENA_LOCK, true)?;
    c.immediate(0xaee0, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xaee4, C::Eq, 0xaf00, false)?;
    c.assertion(0xaee8, 0x4e8, 0xb04c)?;
    c.literal_load(0xaf00, 0, 0xb058)?;
    c.emit(0xaf04, data_register(C::Al, Op::Add, r(10), r(7), r(6), Shift::Lsl, 0, false)?)?;
    c.word(0xaf08, false, 0, 4, 0x24)?;
    c.literal_load(0xaf0c, 0, 0xb05c)?;
    c.word(0xaf10, false, 0, 4, 0x28)?;
    c.immediate(0xaf14, C::Al, Op::Add, 5, 5, 1)?;
    c.immediate(0xaf18, C::Al, Op::Tst, 0, 5, 0xf)?;
    c.jump(0xaf1c, C::Ne, 0xaf14, false)?;
    c.word(0xaf20, false, 5, 4, 4)?;
    c.word(0xaf24, false, 5, 4, 8)?;
    c.literal_load(0xaf28, 6, 0xb060)?;
    c.immediate(0xaf2c, C::Al, Op::Mov, 7, 0, 0)?;
    c.halfword(0xaf30, true, 0, 9, 0)?;
    c.emit(0xaf34, data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 1, false)?)?;
    c.halfword(0xaf38, true, 8, 0, 0)?;
    c.immediate(0xaf3c, C::Al, Op::Mov, 0, 0, 0)?;
    c.jump(0xaf40, C::Al, NEXT_COOKIE, true)?;
    c.literal_load(0xaf44, 1, 0xb054)?;
    c.emit(0xaf48, transfer_word_register(C::Al, false, r(0), r(1), r(8), Shift::Lsl, 2)?)?;
    c.halfword(0xaf4c, true, 0, 9, 0)?;
    c.immediate(0xaf50, C::Al, Op::Mov, 8, 0, 1)?;
    c.emit(0xaf54, data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 1, false)?)?;
    c.halfword(0xaf58, true, 0, 0, 0)?;
    c.emit(0xaf5c, transfer_word_register(C::Al, true, r(1), r(1), r(0), Shift::Lsl, 2)?)?;
    c.word(0xaf60, false, 1, 4, 0)?;
    c.immediate(0xaf64, C::Al, Op::Mov, 1, 0, 0x238)?;
    c.emit(0xaf68, transfer_halfword_register(C::Al, false, r(0), r(4), r(1))?)?;
    c.immediate(0xaf6c, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0xaf70, C::Al, NEXT_COOKIE, true)?;
    c.immediate(0xaf74, C::Al, Op::Mov, 1, 0, 0x234)?;
    c.emit(0xaf78, transfer_halfword_register(C::Al, false, r(0), r(4), r(1))?)?;
    c.immediate(0xaf7c, C::Al, Op::Mov, 0, 0, 1)?;
    c.jump(0xaf80, C::Al, NEXT_COOKIE, true)?;
    c.immediate(0xaf84, C::Al, Op::Mov, 1, 0, 0x36)?;
    c.immediate(0xaf88, C::Al, Op::Orr, 1, 1, 0x200)?;
    c.emit(0xaf8c, transfer_halfword_register(C::Al, false, r(0), r(4), r(1))?)?;
    c.halfword(0xaf90, true, 0, 9, 0)?;
    c.immediate(0xaf94, C::Al, Op::Add, 0, 0, 1)?;
    c.halfword(0xaf98, false, 0, 9, 0)?;
    c.emit(0xaf9c, data_register(C::Al, Op::Sub, r(0), r(10), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0xafa0, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Asr, 31, false)?)?;
    c.emit(0xafa4, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsr, 28, false)?)?;
    c.immediate(0xafa8, C::Al, Op::Bic, 6, 0, 0xf)?;
    c.word(0xafac, true, 0, 4, 4)?;
    c.mov(0xafb0, 1, 6)?;
    c.jump(0xafb4, C::Al, 0xb064, true)?;
    c.word(0xafb8, true, 0, 4, 4)?;
    c.immediate(0xafbc, C::Al, Op::Add, 5, 4, 0x234)?;
    c.byte(0xafc0, true, 1, 0, 3)?;
    c.immediate(0xafc4, C::Al, Op::And, 1, 1, 0xf)?;
    c.immediate(0xafc8, C::Al, Op::Orr, 1, 1, 0x10)?;
    c.byte(0xafcc, false, 1, 0, 3)?;
    c.word(0xafd0, false, 8, 4, 0xc)?;
    c.word(0xafd4, false, 6, 4, 0x10)?;
    c.word(0xafd8, false, 7, 4, 0x14)?;
    c.word(0xafdc, false, 7, 4, 0x18)?;
    c.word(0xafe0, false, 7, 4, 0x1c)?;
    c.word(0xafe4, true, 0, 13, 0)?;
    c.word(0xafe8, false, 0, 4, 0x20)?;
    c.byte(0xafec, true, 0, 4, 0x230)?;
    c.immediate(0xaff0, C::Al, Op::Tst, 0, 0, 2)?;
    c.jump(0xaff4, C::Eq, 0xb010, false)?;
    c.word(0xaff8, true, 1, 4, 4)?;
    c.mov(0xaffc, 0, 4)?;
    c.jump(0xb000, C::Al, 0xaa0c, true)?;
    c.word(0xb004, true, 1, 4, 4)?;
    c.mov(0xb008, 0, 4)?;
    c.jump(0xb00c, C::Al, 0xaba0, true)?;
    c.word(0xb010, true, 0, 4, 4)?;
    c.halfword(0xb014, true, 1, 0, 2)?;
    c.halfword(0xb018, true, 2, 0, 4)?;
    c.emit(0xb01c, data_register(C::Al, Op::Eor, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.halfword(0xb020, true, 2, 0, 6)?;
    c.emit(0xb024, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.halfword(0xb028, true, 2, 0, 8)?;
    c.emit(0xb02c, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.halfword(0xb030, true, 2, 0, 0xa)?;
    c.emit(0xb034, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.halfword(0xb038, true, 2, 5, 0)?;
    c.emit(0xb03c, data_register(C::Al, Op::Eor, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.halfword(0xb040, false, 1, 0, 0)?;
    c.immediate(0xb044, C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.restore(0xb048, 0x8ff0)?;
    for &(at, value) in &PROTECTED_DATA[..6] {
        c.literal(at, value)?;
    }
    Ok(())
}

fn initialize_first_free_header(c: &mut Code) -> Result<(), String> {
    c.emit(0xb064, push_registers(C::Al, 0x4830)?)?;
    c.immediate(0xb068, C::Al, Op::Add, 11, 13, 8)?;
    c.mov(0xb06c, 5, 0)?;
    c.mov(0xb070, 4, 1)?;
    c.immediate(0xb074, C::Al, Op::Cmp, 0, 5, 0)?;
    c.jump(0xb078, C::Ne, 0xb090, false)?;
    c.jump(0xb07c, C::Al, CAPTURE_FAILURE, true)?;
    c.literal_load(0xb080, 2, 0xb0fc)?;
    c.immediate(0xb084, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0xb088, C::Al, Op::Mov, 1, 0, 0x560)?;
    c.jump(0xb08c, C::Al, REPORT_FAILURE, true)?;
    c.immediate(0xb090, C::Al, Op::Cmp, 0, 4, 0xb)?;
    c.jump(0xb094, C::Hi, 0xb0b0, false)?;
    c.assertion(0xb098, 0x561, 0xb0fc)?;
    c.immediate(0xb0b0, C::Al, Op::Mov, 0, 0, 0)?;
    c.mov(0xb0b4, 1, 5)?;
    c.byte(0xb0b8, false, 0, 5, 9)?;
    c.byte(0xb0bc, false, 0, 5, 5)?;
    c.byte(0xb0c0, false, 0, 5, 3)?;
    c.byte(0xb0c4, false, 0, 5, 2)?;
    c.byte(0xb0c8, false, 0, 5, 1)?;
    c.byte(0xb0cc, false, 0, 5, 0)?;
    c.emit(0xb0d0, transfer_byte_preindexed_writeback(C::Al, false, r(0), r(1), 8)?)?;
    c.byte(0xb0d4, false, 0, 1, 3)?;
    c.byte(0xb0d8, false, 0, 1, 2)?;
    c.mov(0xb0dc, 1, 5)?;
    c.emit(0xb0e0, transfer_byte_preindexed_writeback(C::Al, false, r(0), r(1), 4)?)?;
    c.byte(0xb0e4, false, 0, 1, 3)?;
    c.byte(0xb0e8, false, 0, 1, 2)?;
    c.immediate(0xb0ec, C::Al, Op::Mov, 0, 0, 1)?;
    c.byte(0xb0f0, false, 0, 5, 3)?;
    c.word(0xb0f4, false, 4, 1, 0)?;
    c.restore(0xb0f8, 0x8830)?;
    c.literal(0xb0fc, DIAGNOSTIC)
}

fn teardown_heap(c: &mut Code) -> Result<(), String> {
    c.emit(0xb100, push_registers(C::Al, 0x4890)?)?;
    c.immediate(0xb104, C::Al, Op::Add, 11, 13, 8)?;
    c.mov(0xb108, 4, 0)?;
    c.literal_load(0xb10c, 0, 0xb1a4)?;
    c.halfword(0xb110, true, 1, 0, 0)?;
    c.immediate(0xb114, C::Al, Op::Sub, 1, 1, 1)?;
    c.halfword(0xb118, false, 1, 0, 0)?;
    c.immediate(0xb11c, C::Al, Op::Mov, 0, 0, 0xff)?;
    c.immediate(0xb120, C::Al, Op::Orr, 0, 0, 0xff00)?;
    c.emit(0xb124, data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.immediate(0xb128, C::Al, Op::Cmp, 0, 0, 0x1e)?;
    c.jump(0xb12c, C::Hi, 0xb144, false)?;
    c.immediate(0xb130, C::Al, Op::Mov, 1, 0, 0x238)?;
    c.literal_load(0xb134, 2, 0xb1a8)?;
    c.emit(0xb138, transfer_halfword_register(C::Al, true, r(1), r(4), r(1))?)?;
    c.emit(0xb13c, data_register(C::Al, Op::Add, r(0), r(2), r(0), Shift::Lsl, 1, false)?)?;
    c.halfword(0xb140, false, 1, 0, 0)?;
    c.word(0xb144, true, 0, 4, 0x14)?;
    c.immediate(0xb148, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xb14c, C::Eq, 0xb168, false)?;
    c.literal_load(0xb150, 2, 0xb1ac)?;
    c.immediate(0xb154, C::Al, Op::Mov, 1, 0, 0x45)?;
    c.immediate(0xb158, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0xb15c, C::Al, Op::Orr, 1, 1, 0x500)?;
    c.restore(0xb160, 0x4890)?;
    c.jump(0xb164, C::Al, REPORT_FAILURE, false)?;
    c.word(0xb168, true, 0, 4, 0x2c)?;
    c.jump(0xb16c, C::Al, DESTROY_ARENA_LOCK, true)?;
    c.immediate(0xb170, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xb174, C::Eq, 0xb190, false)?;
    c.assertion(0xb178, 0x53e, 0xb1ac)?;
    c.mov(0xb190, 0, 4)?;
    c.immediate(0xb194, C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(0xb198, C::Al, Op::Mov, 2, 0, CONTEXT_SIZE)?;
    c.restore(0xb19c, 0x4890)?;
    c.jump(0xb1a0, C::Al, CLEAR_MEMORY, false)?;
    for &(at, value) in &PROTECTED_DATA[7..] {
        c.literal(at, value)?;
    }
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    initialize_heap(&mut c)?;
    initialize_first_free_header(&mut c)?;
    teardown_heap(&mut c)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES
        || CODE_BYTES + DATA_BYTES != END - START
    {
        return Err("heap lifetime extent mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn extent_and_protected_data() {
        let bytes = fragment().unwrap();
        assert_eq!((bytes.len(), CODE_BYTES, DATA_BYTES), (1024, 984, 40));
        for (at, value) in PROTECTED_DATA {
            assert_eq!(&bytes[at - START..at - START + 4], &value.to_le_bytes());
            let mut c = Code { at, bytes: vec![0; at - START] };
            assert!(c.emit(at, 0).is_err());
            assert!(c.literal(at, value ^ 1).is_err());
            c.literal(at, value).unwrap();
            assert!(c.literal(at, value).is_err());
        }
        for at in [START - 4, START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, 0).is_err());
            assert!(c.literal(at, 0).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, 0).is_err());
    }
}
