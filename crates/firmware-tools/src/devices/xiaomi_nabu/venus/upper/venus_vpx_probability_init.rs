use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, mov_register, mov_shifted_register, push_registers,
    transfer_byte, transfer_byte_postindexed, transfer_byte_preindexed_writeback,
    transfer_halfword_register, transfer_multiple, transfer_word,
};

pub const START: usize = 0xc65ac;
pub const END: usize = 0xc6e54;
pub const CODE_END: usize = 0xc6e0c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2144;
pub const DATA_BYTES: usize = 72;
pub const FUNCTION_STARTS: [usize; 1] = [START];
pub const EXTERNAL_TABLE_BASES: [usize; 18] = [
    0xfe25c,
    0xfe40c,
    0xfe5bc,
    0xfe76c,
    0xfe18e,
    0xfe16a,
    0xfe1f6,
    0xfe20c,
    0xfe10a,
    0xfe1f1,
    0xfe1ec,
    0xfe1e8,
    0xfe217,
    0xfe214,
    0xfe20a,
    0xfe206,
    0xfe200,
    0xfe0f5,
];

fn r(n: u8) -> R { R::new(n).unwrap() }

const SAVED_REGISTERS: u16 = 0x4ff0;
const RESTORED_REGISTERS: u16 = 0x8ff0;

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VPX probability initialization at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..CODE_END).contains(&at) && at % 4 == 0
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) { return Err("VPX probability instruction outside code".into()); }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if !matches!((self.at, condition, target, link),
            (0xc65cc, C::Al, 0x31ec, true)
            | (0xc65e0, C::Al, 0x31ec, true)
            | (0xc664c, C::Al, 0x471c, true)
            | (0xc665c, C::Al, 0x471c, true)
            | (0xc666c, C::Al, 0x471c, true)
            | (0xc667c, C::Al, 0x471c, true)
            | (0xc669c, C::Ne, 0xc6640, false)
            | (0xc66b8, C::Ne, 0xc6614, false)
            | (0xc6700, C::Ne, 0xc6604, false)
            | (0xc6720, C::Al, 0x471c, true)
            | (0xc6734, C::Ne, 0xc6714, false)
            | (0xc6750, C::Al, 0x471c, true)
            | (0xc6764, C::Ne, 0xc6744, false)
            | (0xc6778, C::Al, 0x471c, true)
            | (0xc6814, C::Al, 0x471c, true)
            | (0xc6828, C::Ne, 0xc6808, false)
            | (0xc683c, C::Ne, 0xc67fc, false)
            | (0xc69b0, C::Al, 0x471c, true)
            | (0xc69c4, C::Al, 0x471c, true)
            | (0xc69d8, C::Al, 0x471c, true)
            | (0xc69ec, C::Al, 0x471c, true)
            | (0xc6a04, C::Al, 0x471c, true)
            | (0xc6a18, C::Al, 0x471c, true)
            | (0xc6a2c, C::Al, 0x471c, true)
            | (0xc6a78, C::Al, 0x471c, true)
            | (0xc6a88, C::Al, 0x471c, true)
            | (0xc6a9c, C::Ne, 0xc6a6c, false)
            | (0xc6abc, C::Al, 0x471c, true)
            | (0xc6acc, C::Al, 0x471c, true)
            | (0xc6ae0, C::Ne, 0xc6ab0, false)
            | (0xc6b08, C::Al, 0x471c, true)
            | (0xc6b1c, C::Ne, 0xc6afc, false)
            | (0xc6b30, C::Ne, 0xc6af0, false)
            | (0xc6b88, C::Al, 0x471c, true)
            | (0xc6c34, C::Al, 0x471c, true)
            | (0xc6d00, C::Al, 0x471c, true)
            | (0xc6db0, C::Al, 0x471c, true)
            | (0xc6dc0, C::Al, 0x471c, true)
            | (0xc6dd8, C::Al, 0x471c, true)
            | (0xc6de8, C::Al, 0x471c, true)
            | (0xc6e00, C::Al, 0x471c, true)
        ) { return Err("invalid VPX probability branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        let valid = match self.at {
            0xc65e4 => rd == r(0) && target == 0xc6e0c,
            0xc65ec => rd == r(0) && target == 0xc6e10,
            0xc65f4 => rd == r(0) && target == 0xc6e14,
            0xc65fc => rd == r(0) && target == 0xc6e18,
            0xc6704 => rd == r(6) && target == 0xc6e1c,
            0xc6738 => rd == r(6) && target == 0xc6e20,
            0xc6768 => rd == r(1) && target == 0xc6e24,
            0xc677c => rd == r(0) && target == 0xc6e28,
            0xc67f0 => rd == r(7) && target == 0xc6e2c,
            0xc6840 => rd == r(0) && target == 0xc6e30,
            0xc6844 => rd == r(4) && target == 0xc6e3c,
            0xc6878 => rd == r(0) && target == 0xc6e34,
            0xc68b4 => rd == r(0) && target == 0xc6e38,
            0xc6900 => rd == r(0) && target == 0xc6e40,
            0xc6920 => rd == r(0) && target == 0xc6e44,
            0xc6940 => rd == r(0) && target == 0xc6e48,
            0xc6968 => rd == r(0) && target == 0xc6e4c,
            0xc69f0 => rd == r(4) && target == 0xc6e50,
            _ => false,
        };
        if !valid { return Err("invalid VPX probability literal".into()); }
        self.emit(transfer_word(C::Al, true, rd, R::PC, (target - self.at - 8) as i32)?)
    }

    fn restore(&mut self, condition: C, registers: u16) -> Result<(), String> {
        if self.at != 0xc6e08 || condition != C::Al || registers != RESTORED_REGISTERS {
            return Err("invalid VPX probability restore".into());
        }
        self.emit(transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !(CODE_END..END).contains(&at) || at % 4 != 0
            || EXTERNAL_TABLE_BASES[(at - CODE_END) / 4] != value as usize {
            return Err("invalid VPX probability pool word".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn zero_destination_and_stack(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc65ac)?;
    c.emit(push_registers(C::Al, SAVED_REGISTERS)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x34c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0xc00)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xe20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 36)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(13), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xf00)?)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.branch(C::Al, 0x31ec, true)?;
    c.check(0xc65e4)
}

fn build_coefficient_probabilities(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc65e4)?;
    c.literal(r(0), 0xc6e0c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 60)?)?;
    c.literal(r(0), 0xc6e10)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 56)?)?;
    c.literal(r(0), 0xc6e14)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 52)?)?;
    c.literal(r(0), 0xc6e18)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 40)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 64)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 60)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 68)?)?;
    c.emit(data_register(C::Al, Op::Add, r(9), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 56)?)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 52)?)?;
    c.emit(data_register(C::Al, Op::Add, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 48)?)?;
    c.emit(data_register(C::Al, Op::Add, r(5), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x300)?)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x600)?)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x900)?)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(9), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x6)?)?;
    c.branch(C::Ne, 0xc6640, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 64)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 68)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(10), 0x6c)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0xc0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0xc6614, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xd8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x180)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xd8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xd8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 56)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 60)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xd8)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 60)?)?;
    c.branch(C::Ne, 0xc6604, false)?;
    c.check(0xc6704)
}

fn build_mode_and_motion_probabilities(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc6704)?;
    c.literal(r(6), 0xc6e1c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(13), 0x48)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(10), 0xc00)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x5)?)?;
    c.branch(C::Ne, 0xc6714, false)?;
    c.literal(r(6), 0xc6e20)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(10), 0xca0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x12)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x2)?)?;
    c.branch(C::Ne, 0xc6744, false)?;
    c.literal(r(1), 0xc6e24)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xce0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 60)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.literal(r(0), 0xc6e28)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(10), 0xd00)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.emit(mov_register(C::Al, r(2), r(0)))?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 5)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(0), 2)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, true, r(3), r(2), 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(3), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xee)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0xc00)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(10), r(3))?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(6), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0xec)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(3), r(3), r(7), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xea)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0xc00)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0xc00)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(10), r(6))?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(3), r(10), r(7))?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(2), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(2), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(3), r(0), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 68)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(2), r(7), r(2), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xcf0)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(2), r(10), r(7))?)?;
    c.literal(r(7), 0xc6e2c)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(1), r(2), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 64)?)?;
    c.emit(mov_register(C::Al, r(4), r(7)))?;
    c.emit(mov_register(C::Al, r(6), r(8)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x3)?)?;
    c.branch(C::Ne, 0xc6808, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(9), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x60)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x2)?)?;
    c.branch(C::Ne, 0xc67fc, false)?;
    c.literal(r(0), 0xc6e30)?;
    c.literal(r(4), 0xc6e3c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(13), 3596)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(3), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(2), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(1), r(0), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 3592)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xc5)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(0), 0xd00)?)?;
    c.literal(r(0), 0xc6e34)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 40)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(0), 3)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(5), r(3), r(7), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(2), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(5), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(1), r(6), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(6), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(6), 0)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(13), 3598)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(13), 3601)?)?;
    c.literal(r(0), 0xc6e38)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(5), r(3), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1cc)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(1), r(2), Shift::Lsl, 8, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(4), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(4), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xc00)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(5), r(10), r(0))?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xca)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xd00)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(7), r(10), r(0))?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xde0)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(0), r(10), r(1))?)?;
    c.literal(r(0), 0xc6e40)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(13), 3626)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(13), 3628)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(13), 3627)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(13), 3629)?)?;
    c.literal(r(0), 0xc6e44)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 1)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe6)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xd00)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 20)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(10), r(0))?)?;
    c.literal(r(0), 0xc6e48)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(3), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(2), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(1), r(0), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 3632)?)?;
    c.literal(r(0), 0xc6e4c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 5)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(0), 2)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xdf0)?)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(10), r(2))?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(1), r(1), r(2), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(3), r(0), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(1), r(0), Shift::Lsl, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x3)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 3636)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xe00)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 56)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xe20)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x13)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x11)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 52)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xe40)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 48)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xe60)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x34)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x11)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 44)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.literal(r(4), 0xc6e50)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xe80)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xea0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0xa)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xb)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.check(0xc6a1c)
}

fn pack_destination_probabilities(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc6a1c)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 36)?)?;
    c.emit(mov_register(C::Al, r(1), r(10)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xc00)?)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(0), 0xc00)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0x1ec)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(0), 0xc00)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xe3)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xd00)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(7), r(5), Shift::Lsl, 16, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(6), 0xc10)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xde0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xdc0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(7), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(4), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x5)?)?;
    c.branch(C::Ne, 0xc6a6c, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(10), 0xa9)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(6), 0xcb0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(0), 0xc00)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(7), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(4), 0x9)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x2)?)?;
    c.branch(C::Ne, 0xc6ab0, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(10), 0xd00)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(6), 0xce0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.emit(mov_register(C::Al, r(4), r(8)))?;
    c.emit(mov_register(C::Al, r(7), r(5)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x3)?)?;
    c.branch(C::Ne, 0xc6afc, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(10), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x30)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x60)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x2)?)?;
    c.branch(C::Ne, 0xc6af0, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(13), 0x48)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(2), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(2), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(2), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 3393)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3392)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(4), 3394)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xec0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7)?)?;
    c.emit(transfer_halfword_register(C::Al, true, r(0), r(5), r(0))?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3395)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 8)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(13), 3850)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 3397)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0xc3)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3396)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x46)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0xd00)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0xe00)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(4), 0x5a)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0xd00)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(2), Shift::Lsr, 8)?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(2), Shift::Lsr, 24)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3409)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(2), r(0), 3408)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 3)?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(2), Shift::Lsr, 16)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 20)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Lsr, 8)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3413)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 3412)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 16)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(2), Shift::Lsr, 8)?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(2), Shift::Lsr, 24)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3415)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(2), r(0), 3414)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 3)?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(2), Shift::Lsr, 16)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 5)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3423)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 4)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3422)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(9), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(9), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(6), r(2), r(3), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 3419)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(6), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(0), r(7), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0xd60)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(7), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 60)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 40)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(7), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(7), 3)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 3435)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(6), r(2), r(3), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(7), 4)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(6), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 3438)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(4), 0x6a)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0xd00)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(0), r(7), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(7), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(7), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(13), 12)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(7), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(7), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(7), 3)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 3441)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0xd70)?)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(7), 4)?)?;
    c.emit(transfer_byte(C::Al, false, r(7), r(4), 3444)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(2), r(3), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(7), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(0), r(1), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(1), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 68)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(1), r(4), 3445)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 64)?)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(2), r(4), 4)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(2), Shift::Lsr, 24)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 3)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(2), Shift::Lsr, 16)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 2)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(2), Shift::Lsr, 8)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 1)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Lsr, 24)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), -1)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Lsr, 16)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), -2)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Lsr, 8)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), -3)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x7)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 24)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 28)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(3), 5)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 22)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(3), 4)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 21)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(3), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(3), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(3), 2)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(3), 3)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(2), r(3), Shift::Lsl, 8, false)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 18)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(4), 0x11)?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(7), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(0), r(1), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(1), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(4), 0x17)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(13), 3826)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 27)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xa6)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0xe00)?)?;
    c.emit(transfer_halfword_register(C::Al, true, r(0), r(5), r(0))?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 8)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 24)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2a8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0xc00)?)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(5), r(1))?)?;
    c.emit(data_register(C::Al, Op::Orr, r(0), r(0), r(1), Shift::Lsl, 16, false)?)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(0), r(2), 2)?)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Lsr, 8)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(2), 1)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(2), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 32)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(2), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(2), 1)?)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(2), 2)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(4), 40)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 39)?)?;
    c.emit(transfer_byte(C::Al, false, r(2), r(4), 41)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x37)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 56)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x47)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(13), 3704)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 87)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x67)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 48)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 44)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x77)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(13), 3768)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0xee0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x20)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 88)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x87)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.restore(C::Al, RESTORED_REGISTERS)?;
    c.check(0xc6e0c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    zero_destination_and_stack(&mut c)?;
    build_coefficient_probabilities(&mut c)?;
    build_mode_and_motion_probabilities(&mut c)?;
    pack_destination_probabilities(&mut c)?;
    for (index, value) in EXTERNAL_TABLE_BASES.iter().enumerate() {
        c.data(CODE_END + index * 4, *value as u32)?;
    }
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    fn cursor(at: usize) -> TypedA32 { TypedA32 { at, bytes: Vec::new() } }

    #[test]
    fn structure_and_role() {
        let bytes = fragment().unwrap();
        assert_eq!((START, CODE_END, END), (0xc65ac, 0xc6e0c, 0xc6e54));
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (2144, 72, 2216));
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!(FUNCTION_STARTS, [START]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 536);
        let mut c = cursor(START);
        zero_destination_and_stack(&mut c).unwrap();
        build_coefficient_probabilities(&mut c).unwrap();
        build_mode_and_motion_probabilities(&mut c).unwrap();
        pack_destination_probabilities(&mut c).unwrap();
        c.check(CODE_END).unwrap();
        assert_eq!(c.bytes, bytes[..CODE_BYTES]);
        assert_eq!(word(&bytes, START), push_registers(C::Al, SAVED_REGISTERS).unwrap());
        for (at, rd, value) in [(0xc65b4, R::SP, 0x34c), (0xc65b8, R::SP, 0xc00)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Sub, rd, R::SP, value).unwrap());
        }
        for (at, value) in [(0xc65c0, 0xe20), (0xc65d8, 0xf00)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Mov, r(2), r(0), value).unwrap());
        }
        assert_eq!(word(&bytes, 0xc65cc), branch(C::Al, 0xc65cc, 0x31ec, true).unwrap());
        assert_eq!(word(&bytes, 0xc65e0), branch(C::Al, 0xc65e0, 0x31ec, true).unwrap());
        assert_eq!(word(&bytes, 0xc6e08), transfer_multiple(C::Al, true, R::SP,
            RESTORED_REGISTERS, BlockMode::IncrementAfter, true).unwrap());
        for (index, value) in EXTERNAL_TABLE_BASES.iter().enumerate() {
            assert_eq!(word(&bytes, CODE_END + index * 4), *value as u32);
            let mut c = cursor(CODE_END + index * 4);
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
        }
    }

    #[test]
    fn exact_control_flow_guards() {
        let bytes = fragment().unwrap();
        let conditions = [C::Eq, C::Ne, C::Cs, C::Cc, C::Mi, C::Pl, C::Hi,
            C::Ls, C::Ge, C::Lt, C::Gt, C::Le, C::Al];
        let (mut branches, mut calls, mut restores) = (0, 0, 0);
        for at in (START..CODE_END).step_by(4) {
            let instruction = word(&bytes, at);
            let is_branch = (instruction >> 25) & 7 == 5;
            let displacement = ((instruction << 8) as i32 >> 6) as isize;
            let target = (at as isize + 8 + displacement) as usize;
            for condition in conditions {
                for link in [false, true] {
                    let expected = is_branch && instruction >> 28 == condition as u32
                        && (instruction & (1 << 24) != 0) == link;
                    let mut c = cursor(at);
                    assert_eq!(c.branch(condition, target, link).is_ok(), expected);
                    if expected {
                        branches += 1;
                        calls += usize::from(link);
                        assert_eq!(c.bytes, instruction.to_le_bytes());
                    } else { assert!(c.bytes.is_empty()); }
                    for bad in [target.wrapping_add(4), START + 1, END, usize::MAX] {
                        let mut c = cursor(at);
                        assert!(c.branch(condition, bad, link).is_err());
                        assert!(c.bytes.is_empty());
                    }
                }
                for registers in [RESTORED_REGISTERS, RESTORED_REGISTERS ^ r(0).bit()] {
                    let expected = at == 0xc6e08 && condition == C::Al && registers == RESTORED_REGISTERS;
                    let mut c = cursor(at);
                    assert_eq!(c.restore(condition, registers).is_ok(), expected);
                    if expected { restores += 1; assert_eq!(c.bytes, instruction.to_le_bytes()); }
                    else { assert!(c.bytes.is_empty()); }
                }
            }
        }
        assert_eq!((branches, calls, restores), (41, 30, 1));
        for at in [START - 4, START + 1, CODE_END, END - 1, END, usize::MAX] {
            let mut c = cursor(at);
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.branch(C::Al, 0x471c, true).is_err());
            assert!(c.restore(C::Al, RESTORED_REGISTERS).is_err());
            assert!(c.check(START).is_err());
            assert!(c.bytes.is_empty());
        }
    }

    #[test]
    fn exact_literal_and_pool_guards() {
        let bytes = fragment().unwrap();
        let mut loads = 0;
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            let is_literal = at < CODE_END && instruction & 0xffff0000 == 0xe59f0000;
            let target = at + 8 + (instruction & 0xfff) as usize;
            let rd = r(((instruction >> 12) & 15) as u8);
            for destination in (0..16).map(r) {
                for address in [target, target + 4, usize::MAX] {
                    let expected = is_literal && destination == rd && address == target;
                    let mut c = cursor(at);
                    assert_eq!(c.literal(destination, address).is_ok(), expected);
                    if expected { loads += 1; assert_eq!(c.bytes, instruction.to_le_bytes()); }
                    else { assert!(c.bytes.is_empty()); }
                }
            }
            for (index, value) in EXTERNAL_TABLE_BASES.iter().enumerate() {
                let site = CODE_END + index * 4;
                let mut c = cursor(at);
                assert_eq!(c.data(site, *value as u32).is_ok(), at == site);
                let mut c = cursor(at);
                assert!(c.data(site, *value as u32 ^ 1).is_err());
                assert!(c.bytes.is_empty());
            }
        }
        assert_eq!(loads, 18);
        for at in [START, CODE_END - 1, CODE_END + 1, END, usize::MAX] {
            let mut c = cursor(at);
            assert!(c.data(at, EXTERNAL_TABLE_BASES[0] as u32).is_err());
            assert!(c.bytes.is_empty());
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
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
