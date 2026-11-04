use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    branch_exchange_with_link, bx, data_immediate, data_register,
    data_register_shifted_by_register, load_halfword, mov_register, push_registers,
    store_halfword_postindexed, transfer_byte, transfer_multiple, transfer_word,
    transfer_word_register, transfer_word_register_preindexed_writeback,
};

pub const START: usize = 0xd3e0;
pub const END: usize = 0xd790;
#[cfg(test)]
const FILE_MAPPING_DELTA: usize = 0x4000;
const CODE_BYTES: usize = 888;
const DATA_BYTES: usize = 56;
const HEAP_DIAGNOSTIC: u32 = 0xe37d7;
const INTERRUPT_DIAGNOSTIC: u32 = 0xe380f;
const CALLBACK_TABLE: u32 = 0x103c00;
const INTERRUPT_LOCK: u32 = 0x103d00;
const INTERRUPT_LOCK_STORAGE: u32 = 0x103d08;
const INTERRUPT_PENDING: u32 = 0xe00df000;
const INTERRUPT_ENABLE: u32 = 0xe00df010;
const INTERRUPT_DISABLE: u32 = 0xe00df014;
const CAPTURE_FAILURE: u32 = 0x86644;
const REPORT_FAILURE: u32 = 0x17b0;
const CLEAR_MEMORY: u32 = 0x31ec;
const BEGIN_HEAP_ITERATION: u32 = 0xce84;
const NEXT_HEAP_BLOCK: u32 = 0xcf24;
const HEAP_BLOCK_SIZE: u32 = 0xcdc0;
const CREATE_INTERRUPT_LOCK: u32 = 0x58d8;
const ENTER_INTERRUPT_LOCK: u32 = 0x5958;
const LEAVE_INTERRUPT_LOCK: u32 = 0x59e8;
const PROTECTED_DATA: [(usize, u32); 14] = [
    (0xd530, HEAP_DIAGNOSTIC),
    (0xd5e4, CALLBACK_TABLE),
    (0xd5f8, INTERRUPT_DIAGNOSTIC),
    (0xd648, INTERRUPT_LOCK),
    (0xd64c, INTERRUPT_LOCK_STORAGE),
    (0xd650, INTERRUPT_ENABLE),
    (0xd6b8, INTERRUPT_LOCK),
    (0xd6bc, CALLBACK_TABLE),
    (0xd700, INTERRUPT_LOCK),
    (0xd704, CALLBACK_TABLE),
    (0xd744, INTERRUPT_LOCK),
    (0xd748, INTERRUPT_ENABLE),
    (0xd788, INTERRUPT_LOCK),
    (0xd78c, INTERRUPT_DISABLE),
];

fn r(number: u8) -> R { R::new(number).unwrap() }

fn count_leading_zeros(condition: C, destination: R, source: R) -> u32 {
    ((condition as u32) << 28) | 0x016f0f10 | (destination.number() << 12) | source.number()
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
            return Err(format!("heap interrupt instruction at {at:#x}, cursor {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        if self.at != at || !PROTECTED_DATA.contains(&(at, value))
            || self.bytes.len() != at - START
        {
            return Err(format!("invalid heap interrupt data at {at:#x}"));
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

    fn literal_load(&mut self, at: usize, condition: C, dst: u8, target: usize) -> Result<(), String> {
        if !PROTECTED_DATA.iter().any(|&(address, _)| address == target) {
            return Err(format!("unknown heap interrupt literal {target:#x}"));
        }
        self.emit(at, transfer_word(condition, true, r(dst), R::PC, target as i32 - at as i32 - 8)?)
    }

    fn jump(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }

    fn restore(&mut self, at: usize, condition: C, registers: u16) -> Result<(), String> {
        self.emit(at, transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn assertion(&mut self, at: usize, low: u32, high: u32) -> Result<(), String> {
        self.jump(at, C::Al, CAPTURE_FAILURE, true)?;
        self.literal_load(at + 4, C::Al, 2, 0xd530)?;
        self.immediate(at + 8, C::Al, Op::Mov, 1, 0, low)?;
        self.immediate(at + 12, C::Al, Op::Mov, 0, 0, 0)?;
        self.immediate(at + 16, C::Al, Op::Orr, 1, 1, high)?;
        self.jump(at + 20, C::Al, REPORT_FAILURE, true)
    }
}

fn largest_free_sizes(c: &mut Code) -> Result<(), String> {
    c.emit(0xd3e0, push_registers(C::Al, 0x4bf0)?)?;
    c.immediate(0xd3e4, C::Al, Op::Add, 11, 13, 0x18)?;
    c.immediate(0xd3e8, C::Al, Op::Sub, 13, 13, 0x10)?;
    c.mov(0xd3ec, 8, 0)?;
    c.mov(0xd3f0, 5, 2)?;
    c.mov(0xd3f4, 6, 1)?;
    c.immediate(0xd3f8, C::Al, Op::Cmp, 0, 8, 0)?;
    c.jump(0xd3fc, C::Ne, 0xd418, false)?;
    c.assertion(0xd400, 0xa, 0xb00)?;
    c.immediate(0xd418, C::Al, Op::Cmp, 0, 5, 0)?;
    c.jump(0xd41c, C::Ne, 0xd438, false)?;
    c.assertion(0xd420, 0xb, 0xb00)?;
    c.immediate(0xd438, C::Al, Op::Cmp, 0, 6, 0)?;
    c.jump(0xd43c, C::Ne, 0xd458, false)?;
    c.assertion(0xd440, 0x30c, 0x800)?;
    c.word(0xd458, true, 1, 8, 0x24)?;
    c.immediate(0xd45c, C::Al, Op::Cmp, 0, 1, 0)?;
    c.jump(0xd460, C::Eq, 0xd46c, false)?;
    c.mov(0xd464, 0, 8)?;
    c.emit(0xd468, branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(0xd46c, data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 2, false)?)?;
    c.mov(0xd470, 0, 5)?;
    c.immediate(0xd474, C::Al, Op::Mov, 1, 0, 0)?;
    c.jump(0xd478, C::Al, CLEAR_MEMORY, true)?;
    c.immediate(0xd47c, C::Al, Op::Add, 4, 13, 8)?;
    c.mov(0xd480, 1, 8)?;
    c.mov(0xd484, 0, 4)?;
    c.jump(0xd488, C::Al, BEGIN_HEAP_ITERATION, true)?;
    c.mov(0xd48c, 1, 13)?;
    c.mov(0xd490, 0, 4)?;
    c.immediate(0xd494, C::Al, Op::Mov, 2, 0, 1)?;
    c.jump(0xd498, C::Al, NEXT_HEAP_BLOCK, true)?;
    c.immediate(0xd49c, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xd4a0, C::Eq, 0xd514, false)?;
    c.immediate(0xd4a4, C::Al, Op::Add, 9, 13, 8)?;
    c.mov(0xd4a8, 4, 13)?;
    c.emit(0xd4ac, transfer_byte(C::Al, true, r(0), R::SP, 3)?)?;
    c.immediate(0xd4b0, C::Al, Op::Tst, 0, 0, 0xf)?;
    c.jump(0xd4b4, C::Eq, 0xd4fc, false)?;
    c.mov(0xd4b8, 0, 4)?;
    c.jump(0xd4bc, C::Al, HEAP_BLOCK_SIZE, true)?;
    c.word(0xd4c0, true, 2, 5, 0)?;
    c.immediate(0xd4c4, C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(0xd4c8, C::Al, Op::Cmp, 0, 6, 2)?;
    c.jump(0xd4cc, C::Cc, 0xd4f4, false)?;
    c.immediate(0xd4d0, C::Al, Op::Mov, 3, 0, 1)?;
    c.immediate(0xd4d4, C::Al, Op::Mov, 1, 0, 0)?;
    c.emit(0xd4d8, transfer_word_register(C::Al, true, r(7), r(5), r(3), Shift::Lsl, 2)?)?;
    c.emit(0xd4dc, data_register(C::Al, Op::Cmp, r(0), r(2), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0xd4e0, mov_register(C::Hi, r(1), r(3)))?;
    c.immediate(0xd4e4, C::Al, Op::Add, 3, 3, 1)?;
    c.emit(0xd4e8, mov_register(C::Hi, r(2), r(7)))?;
    c.emit(0xd4ec, data_register(C::Al, Op::Cmp, r(0), r(3), r(6), Shift::Lsl, 0, false)?)?;
    c.jump(0xd4f0, C::Ne, 0xd4d8, false)?;
    c.emit(0xd4f4, data_register(C::Al, Op::Cmp, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0xd4f8, transfer_word_register(C::Hi, false, r(0), r(5), r(1), Shift::Lsl, 2)?)?;
    c.mov(0xd4fc, 0, 9)?;
    c.mov(0xd500, 1, 4)?;
    c.immediate(0xd504, C::Al, Op::Mov, 2, 0, 1)?;
    c.jump(0xd508, C::Al, NEXT_HEAP_BLOCK, true)?;
    c.immediate(0xd50c, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xd510, C::Ne, 0xd4ac, false)?;
    c.word(0xd514, true, 1, 8, 0x28)?;
    c.immediate(0xd518, C::Al, Op::Cmp, 0, 1, 0)?;
    c.jump(0xd51c, C::Eq, 0xd528, false)?;
    c.mov(0xd520, 0, 8)?;
    c.emit(0xd524, branch_exchange_with_link(C::Al, r(1))?)?;
    c.immediate(0xd528, C::Al, Op::Sub, 13, 11, 0x18)?;
    c.restore(0xd52c, C::Al, 0x8bf0)?;
    c.literal(0xd530, HEAP_DIAGNOSTIC)
}

fn constant_sixteen(c: &mut Code) -> Result<(), String> {
    c.immediate(0xd534, C::Al, Op::Mov, 0, 0, 16)?;
    c.emit(0xd538, bx(C::Al, R::LR))
}

fn propagate_bin_hint(c: &mut Code) -> Result<(), String> {
    c.immediate(0xd53c, C::Al, Op::Cmp, 0, 1, 0)?;
    c.jump(0xd540, C::Lt, 0xd580, false)?;
    c.immediate(0xd544, C::Al, Op::Add, 2, 1, 1)?;
    c.immediate(0xd548, C::Al, Op::Cmp, 0, 2, 31)?;
    c.jump(0xd54c, C::Hi, 0xd580, false)?;
    c.emit(0xd550, data_register(C::Al, Op::Add, r(3), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.emit(0xd554, data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 4, false)?)?;
    c.immediate(0xd558, C::Al, Op::Add, 3, 3, 0x38)?;
    c.immediate(0xd55c, C::Al, Op::Add, 12, 0, 0x38)?;
    c.emit(0xd560, load_halfword(C::Al, r(0), r(3), 0)?)?;
    c.emit(0xd564, data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0xd568, bx(C::Ne, R::LR))?;
    c.emit(0xd56c, load_halfword(C::Al, r(0), r(12), 0)?)?;
    c.immediate(0xd570, C::Al, Op::Sub, 2, 2, 1)?;
    c.immediate(0xd574, C::Al, Op::Cmp, 0, 2, 0)?;
    c.emit(0xd578, store_halfword_postindexed(C::Al, r(0), r(3), -0x10)?)?;
    c.jump(0xd57c, C::Gt, 0xd560, false)?;
    c.emit(0xd580, bx(C::Al, R::LR))
}

fn dispatch_interrupt_callbacks(c: &mut Code) -> Result<(), String> {
    c.emit(0xd584, push_registers(C::Al, 0x4830)?)?;
    c.immediate(0xd588, C::Al, Op::Add, 11, 13, 8)?;
    c.immediate(0xd58c, C::Al, Op::Mov, 4, 0, INTERRUPT_PENDING & 0x000fffff)?;
    c.immediate(0xd590, C::Al, Op::Orr, 4, 4, INTERRUPT_PENDING & 0xfff00000)?;
    c.word(0xd594, true, 0, 4, 0)?;
    c.immediate(0xd598, C::Al, Op::Cmp, 0, 0, 0)?;
    c.restore(0xd59c, C::Eq, 0x8830)?;
    c.literal_load(0xd5a0, C::Al, 5, 0xd5e4)?;
    c.immediate(0xd5a4, C::Al, Op::Cmp, 0, 0, 0)?;
    c.emit(0xd5a8, count_leading_zeros(C::Ne, r(0), r(0)))?;
    c.immediate(0xd5ac, C::Eq, Op::Mov, 0, 0, 32)?;
    c.immediate(0xd5b0, C::Al, Op::Cmp, 0, 0, 31)?;
    c.jump(0xd5b4, C::Hi, 0xd5d4, false)?;
    c.immediate(0xd5b8, C::Al, Op::Rsb, 0, 0, 31)?;
    c.emit(0xd5bc, transfer_word_register(C::Al, true, r(1), r(5), r(0), Shift::Lsl, 3)?)?;
    c.immediate(0xd5c0, C::Al, Op::Cmp, 0, 1, 0)?;
    c.jump(0xd5c4, C::Eq, 0xd5d4, false)?;
    c.emit(0xd5c8, data_register(C::Al, Op::Add, r(0), r(5), r(0), Shift::Lsl, 3, false)?)?;
    c.word(0xd5cc, true, 0, 0, 4)?;
    c.emit(0xd5d0, branch_exchange_with_link(C::Al, r(1))?)?;
    c.word(0xd5d4, true, 0, 4, 0)?;
    c.immediate(0xd5d8, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xd5dc, C::Ne, 0xd5a4, false)?;
    c.restore(0xd5e0, C::Al, 0x8830)?;
    c.literal(0xd5e4, CALLBACK_TABLE)
}

fn interrupt_source_diagnostic(c: &mut Code) -> Result<(), String> {
    c.literal_load(0xd5e8, C::Al, 2, 0xd5f8)?;
    c.immediate(0xd5ec, C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(0xd5f0, C::Al, Op::Mov, 1, 0, 0x6d)?;
    c.jump(0xd5f4, C::Al, REPORT_FAILURE, false)?;
    c.literal(0xd5f8, INTERRUPT_DIAGNOSTIC)
}

fn lazily_initialize_interrupts(c: &mut Code) -> Result<(), String> {
    c.literal_load(0xd5fc, C::Al, 0, 0xd648)?;
    c.word(0xd600, true, 0, 0, 0)?;
    c.immediate(0xd604, C::Al, Op::Cmp, 0, 0, 0)?;
    c.jump(0xd608, C::Ne, 0xd640, false)?;
    c.emit(0xd60c, push_registers(C::Al, 0x4800)?)?;
    c.mov(0xd610, 11, 13)?;
    c.literal_load(0xd614, C::Al, 1, 0xd648)?;
    c.literal_load(0xd618, C::Al, 2, 0xd64c)?;
    c.immediate(0xd61c, C::Al, Op::Mov, 0, 0, 0x80)?;
    c.jump(0xd620, C::Al, CREATE_INTERRUPT_LOCK, true)?;
    c.literal_load(0xd624, C::Al, 0, 0xd650)?;
    c.immediate(0xd628, C::Al, Op::Mov, 1, 0, 0)?;
    c.word(0xd62c, false, 1, 0, -4)?;
    c.word(0xd630, true, 1, 0, 0)?;
    c.immediate(0xd634, C::Al, Op::Orr, 1, 1, 0x8000)?;
    c.word(0xd638, false, 1, 0, 0)?;
    c.restore(0xd63c, C::Al, 0x4800)?;
    c.immediate(0xd640, C::Al, Op::Mov, 0, 0, 0)?;
    c.emit(0xd644, bx(C::Al, R::LR))?;
    c.literal(0xd648, INTERRUPT_LOCK)?;
    c.literal(0xd64c, INTERRUPT_LOCK_STORAGE)?;
    c.literal(0xd650, INTERRUPT_ENABLE)
}

fn register_interrupt_callback(c: &mut Code) -> Result<(), String> {
    c.emit(0xd654, push_registers(C::Al, 0x4bf0)?)?;
    c.immediate(0xd658, C::Al, Op::Add, 11, 13, 0x18)?;
    c.literal_load(0xd65c, C::Al, 6, 0xd6b8)?;
    c.mov(0xd660, 8, 3)?;
    c.mov(0xd664, 5, 2)?;
    c.mov(0xd668, 7, 1)?;
    c.word(0xd66c, true, 0, 6, 0)?;
    c.jump(0xd670, C::Al, ENTER_INTERRUPT_LOCK, true)?;
    c.immediate(0xd674, C::Al, Op::Mvn, 4, 0, 0)?;
    c.immediate(0xd678, C::Al, Op::Cmp, 0, 7, 31)?;
    c.jump(0xd67c, C::Hi, 0xd6a8, false)?;
    c.immediate(0xd680, C::Al, Op::Cmp, 0, 5, 0)?;
    c.jump(0xd684, C::Eq, 0xd6a8, false)?;
    c.literal_load(0xd688, C::Al, 0, 0xd6bc)?;
    c.emit(0xd68c, transfer_word_register(C::Al, true, r(1), r(0), r(7), Shift::Lsl, 3)?)?;
    c.immediate(0xd690, C::Al, Op::Cmp, 0, 1, 0)?;
    c.jump(0xd694, C::Ne, 0xd6a8, false)?;
    c.emit(0xd698, data_register(C::Al, Op::Add, r(1), r(0), r(7), Shift::Lsl, 3, false)?)?;
    c.immediate(0xd69c, C::Al, Op::Mov, 4, 0, 0)?;
    c.word(0xd6a0, false, 8, 1, 4)?;
    c.emit(0xd6a4, transfer_word_register(C::Al, false, r(5), r(0), r(7), Shift::Lsl, 3)?)?;
    c.word(0xd6a8, true, 0, 6, 0)?;
    c.jump(0xd6ac, C::Al, LEAVE_INTERRUPT_LOCK, true)?;
    c.mov(0xd6b0, 0, 4)?;
    c.restore(0xd6b4, C::Al, 0x8bf0)?;
    c.literal(0xd6b8, INTERRUPT_LOCK)?;
    c.literal(0xd6bc, CALLBACK_TABLE)
}

fn unregister_interrupt_callback(c: &mut Code) -> Result<(), String> {
    c.emit(0xd6c0, push_registers(C::Al, 0x48f0)?)?;
    c.immediate(0xd6c4, C::Al, Op::Add, 11, 13, 0x10)?;
    c.literal_load(0xd6c8, C::Al, 6, 0xd700)?;
    c.mov(0xd6cc, 5, 1)?;
    c.word(0xd6d0, true, 0, 6, 0)?;
    c.jump(0xd6d4, C::Al, ENTER_INTERRUPT_LOCK, true)?;
    c.immediate(0xd6d8, C::Al, Op::Cmp, 0, 5, 31)?;
    c.immediate(0xd6dc, C::Al, Op::Mvn, 4, 0, 0)?;
    c.literal_load(0xd6e0, C::Ls, 0, 0xd704)?;
    c.immediate(0xd6e4, C::Ls, Op::Mov, 4, 0, 0)?;
    c.emit(0xd6e8, transfer_word_register_preindexed_writeback(C::Ls, false, r(4), r(0), r(5), Shift::Lsl, 3)?)?;
    c.emit(0xd6ec, transfer_word(C::Ls, false, r(4), r(0), 4)?)?;
    c.word(0xd6f0, true, 0, 6, 0)?;
    c.jump(0xd6f4, C::Al, LEAVE_INTERRUPT_LOCK, true)?;
    c.mov(0xd6f8, 0, 4)?;
    c.restore(0xd6fc, C::Al, 0x88f0)?;
    c.literal(0xd700, INTERRUPT_LOCK)?;
    c.literal(0xd704, CALLBACK_TABLE)
}

fn set_mmio_bit(c: &mut Code, start: usize, register: u32, after_write: u32) -> Result<(), String> {
    c.emit(start, push_registers(C::Al, 0x4830)?)?;
    c.immediate(start + 4, C::Al, Op::Add, 11, 13, 8)?;
    c.literal_load(start + 8, C::Al, 5, start + 0x3c)?;
    c.mov(start + 0xc, 4, 1)?;
    c.word(start + 0x10, true, 0, 5, 0)?;
    c.jump(start + 0x14, C::Al, ENTER_INTERRUPT_LOCK, true)?;
    c.literal_load(start + 0x18, C::Al, 0, start + 0x40)?;
    c.immediate(start + 0x1c, C::Al, Op::Mov, 2, 0, 1)?;
    c.word(start + 0x20, true, 1, 0, 0)?;
    c.emit(start + 0x24, data_register_shifted_by_register(C::Al, Op::Orr, r(1), r(1), r(2), Shift::Lsl, r(4), false)?)?;
    c.word(start + 0x28, false, 1, 0, 0)?;
    c.word(start + 0x2c, true, 0, 5, 0)?;
    c.jump(start + 0x30, C::Al, after_write, true)?;
    c.immediate(start + 0x34, C::Al, Op::Mov, 0, 0, 0)?;
    c.restore(start + 0x38, C::Al, 0x8830)?;
    c.literal(start + 0x3c, INTERRUPT_LOCK)?;
    c.literal(start + 0x40, register)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    largest_free_sizes(&mut c)?;
    constant_sixteen(&mut c)?;
    propagate_bin_hint(&mut c)?;
    dispatch_interrupt_callbacks(&mut c)?;
    interrupt_source_diagnostic(&mut c)?;
    lazily_initialize_interrupts(&mut c)?;
    register_interrupt_callback(&mut c)?;
    unregister_interrupt_callback(&mut c)?;
    set_mmio_bit(&mut c, 0xd708, INTERRUPT_ENABLE, LEAVE_INTERRUPT_LOCK)?;
    set_mmio_bit(&mut c, 0xd74c, INTERRUPT_DISABLE, ENTER_INTERRUPT_LOCK)?;
    if c.at != END || c.bytes.len() != CODE_BYTES + DATA_BYTES
        || CODE_BYTES + DATA_BYTES != END - START
    {
        return Err("heap interrupt extent mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

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
        assert_eq!((bytes.len(), CODE_BYTES, DATA_BYTES), (944, 888, 56));
        assert_eq!((START + FILE_MAPPING_DELTA, END + FILE_MAPPING_DELTA), (0x113e0, 0x11790));
        for (at, value) in PROTECTED_DATA {
            assert_eq!(word(&bytes, at), value);
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
        assert!(c.literal_load(START, C::Al, 0, START).is_err());
        assert!(c.literal(START, HEAP_DIAGNOSTIC).is_err());
    }

    #[test]
    fn callback_dispatch_and_mmio_lock_targets() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0xd5a8), count_leading_zeros(C::Ne, r(0), r(0)));
        assert_eq!(word(&bytes, 0xd5d0), branch_exchange_with_link(C::Al, r(1)).unwrap());
        for at in [0xd670, 0xd6d4, 0xd71c, 0xd760, 0xd77c] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, ENTER_INTERRUPT_LOCK, true).unwrap());
        }
        for at in [0xd6ac, 0xd6f4, 0xd738] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, LEAVE_INTERRUPT_LOCK, true).unwrap());
        }
        assert_ne!(word(&bytes, 0xd77c), branch(C::Al, 0xd77c, LEAVE_INTERRUPT_LOCK, true).unwrap());
    }

    #[test]
    fn bin_hint_propagation_and_constant() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0xd534), data_immediate(C::Al, Op::Mov, r(0), r(0), 16).unwrap());
        assert_eq!(word(&bytes, 0xd578), store_halfword_postindexed(C::Al, r(0), r(3), -16).unwrap());
        assert_eq!(word(&bytes, 0xd57c), branch(C::Gt, 0xd57c, 0xd560, false).unwrap());
    }
}
