use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register, load_halfword,
    load_signed_byte_register, mov_register, mov_shifted_register, push_registers,
    transfer_byte, transfer_halfword_register, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x7aaa0;
pub const END: usize = 0x7b160;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1724;
pub const DATA_BYTES: usize = 4;
const POOL_START: usize = 0x7b15c;
const ASSERTION_LOG: u32 = 0xf4215;

fn r(n: u8) -> R { R::new(n).unwrap() }

fn saved() -> u16 {
    (4..=11).fold(R::LR.bit(), |mask, n| mask | r(n).bit())
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("H.264 parameter writer address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && (START..POOL_START).contains(&at)
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("H.264 parameter writer instruction outside CODE".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if at != POOL_START || value != ASSERTION_LOG {
            return Err("H.264 parameter writer protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        if !matches!(self.at, 0x7ad04 | 0x7ad64) || rd != r(2) || target != POOL_START {
            return Err("H.264 parameter writer invalid literal".into());
        }
        self.emit(transfer_word(C::Al, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7aacc, C::Al, 0x79b6c, true)
            | (0x7aadc, C::Al, 0x798f8, true)
            | (0x7aaec, C::Al, 0x798f8, true)
            | (0x7ab34, C::Al, 0x798f8, true)
            | (0x7ab44, C::Al, 0x798f8, true)
            | (0x7ab50, C::Al, 0x79db8, true)
            | (0x7ab5c, C::Al, 0x79db8, true)
            | (0x7ab6c, C::Al, 0x79db8, true)
            | (0x7ab78, C::Al, 0x79db8, true)
            | (0x7ab88, C::Al, 0x798f8, true)
            | (0x7ab94, C::Ne, 0x7abc8, false)
            | (0x7aba4, C::Al, 0x798f8, true)
            | (0x7abb4, C::Al, 0x798f8, true)
            | (0x7abc0, C::Ne, 0x7aba8, false)
            | (0x7abc4, C::Al, 0x7abd8, false)
            | (0x7abd4, C::Al, 0x798f8, true)
            | (0x7abe0, C::Al, 0x79db8, true)
            | (0x7abec, C::Al, 0x79db8, true)
            | (0x7abf8, C::Ne, 0x7ac08, false)
            | (0x7ac04, C::Al, 0x79db8, true)
            | (0x7ac10, C::Al, 0x79db8, true)
            | (0x7ac20, C::Eq, 0x7ac38, false)
            | (0x7ac28, C::Ne, 0x7ac40, false)
            | (0x7ac34, C::Al, 0x7ac48, false)
            | (0x7ac3c, C::Eq, 0x7b154, false)
            | (0x7ac4c, C::Al, 0x798f8, true)
            | (0x7ac5c, C::Al, 0x79db8, true)
            | (0x7ac6c, C::Al, 0x79db8, true)
            | (0x7ac7c, C::Al, 0x798f8, true)
            | (0x7ac8c, C::Al, 0x798f8, true)
            | (0x7ac98, C::Eq, 0x7acb0, false)
            | (0x7aca8, C::Al, 0x798f8, true)
            | (0x7acac, C::Al, 0x7acd4, false)
            | (0x7acc8, C::Al, 0x798f8, true)
            | (0x7acd0, C::Eq, 0x7ad90, false)
            | (0x7ace0, C::Al, 0x79db8, true)
            | (0x7acec, C::Ne, 0x7ad04, false)
            | (0x7ad00, C::Al, 0x7ad2c, false)
            | (0x7ad1c, C::Al, 0x17b0, true)
            | (0x7ad30, C::Al, 0x79db8, true)
            | (0x7ad40, C::Al, 0x79db8, true)
            | (0x7ad4c, C::Ne, 0x7ad64, false)
            | (0x7ad60, C::Al, 0x7ad88, false)
            | (0x7ad78, C::Al, 0x17b0, true)
            | (0x7ad8c, C::Al, 0x79db8, true)
            | (0x7ad98, C::Eq, 0x7b000, false)
            | (0x7adac, C::Al, 0x798f8, true)
            | (0x7adc0, C::Al, 0x798f8, true)
            | (0x7adcc, C::Eq, 0x7ae28, false)
            | (0x7ade8, C::Al, 0x7a08c, true)
            | (0x7adfc, C::Al, 0x798f8, true)
            | (0x7ae04, C::Ne, 0x7ae28, false)
            | (0x7ae14, C::Al, 0x798f8, true)
            | (0x7ae24, C::Al, 0x798f8, true)
            | (0x7ae34, C::Al, 0x798f8, true)
            | (0x7ae48, C::Al, 0x798f8, true)
            | (0x7ae54, C::Eq, 0x7aec4, false)
            | (0x7ae64, C::Al, 0x798f8, true)
            | (0x7ae74, C::Al, 0x798f8, true)
            | (0x7ae84, C::Al, 0x798f8, true)
            | (0x7ae90, C::Ne, 0x7aec4, false)
            | (0x7aea0, C::Al, 0x798f8, true)
            | (0x7aeb0, C::Al, 0x798f8, true)
            | (0x7aec0, C::Al, 0x798f8, true)
            | (0x7aed4, C::Al, 0x798f8, true)
            | (0x7aee8, C::Al, 0x798f8, true)
            | (0x7aef4, C::Eq, 0x7af54, false)
            | (0x7af04, C::Al, 0x798f8, true)
            | (0x7af14, C::Al, 0x798f8, true)
            | (0x7af2c, C::Al, 0x798f8, true)
            | (0x7af40, C::Al, 0x798f8, true)
            | (0x7af50, C::Al, 0x798f8, true)
            | (0x7af60, C::Al, 0x798f8, true)
            | (0x7af70, C::Al, 0x798f8, true)
            | (0x7af80, C::Al, 0x798f8, true)
            | (0x7af94, C::Al, 0x798f8, true)
            | (0x7afa0, C::Eq, 0x7b010, false)
            | (0x7afb0, C::Al, 0x798f8, true)
            | (0x7afbc, C::Al, 0x79db8, true)
            | (0x7afc8, C::Al, 0x79db8, true)
            | (0x7afd4, C::Al, 0x79db8, true)
            | (0x7afe0, C::Al, 0x79db8, true)
            | (0x7afec, C::Al, 0x79db8, true)
            | (0x7aff8, C::Al, 0x79db8, true)
            | (0x7affc, C::Al, 0x7b010, false)
            | (0x7b00c, C::Al, 0x798f8, true)
            | (0x7b018, C::Al, 0x79db8, true)
            | (0x7b024, C::Al, 0x79db8, true)
            | (0x7b030, C::Al, 0x79db8, true)
            | (0x7b03c, C::Al, 0x79db8, true)
            | (0x7b048, C::Al, 0x79db8, true)
            | (0x7b054, C::Al, 0x79db8, true)
            | (0x7b060, C::Al, 0x79db8, true)
            | (0x7b06c, C::Al, 0x79db8, true)
            | (0x7b078, C::Al, 0x79db8, true)
            | (0x7b084, C::Al, 0x79db8, true)
            | (0x7b090, C::Al, 0x79db8, true)
            | (0x7b09c, C::Al, 0x79db8, true)
            | (0x7b0a8, C::Al, 0x79db8, true)
            | (0x7b0b8, C::Al, 0x798f8, true)
            | (0x7b0c4, C::Al, 0x79db8, true)
            | (0x7b0d4, C::Al, 0x798f8, true)
            | (0x7b0e0, C::Al, 0x79db8, true)
            | (0x7b0ec, C::Al, 0x79db8, true)
            | (0x7b0f8, C::Al, 0x79db8, true)
            | (0x7b104, C::Al, 0x79db8, true)
            | (0x7b114, C::Al, 0x798f8, true)
            | (0x7b124, C::Al, 0x798f8, true)
            | (0x7b134, C::Al, 0x798f8, true)
            | (0x7b140, C::Al, 0x79c18, true)
            | (0x7b150, C::Al, 0x79c98, false)
            | (0x7b158, C::Al, 0x7ac24, false)
        );
        if !valid { return Err("H.264 parameter writer invalid branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn profile_and_constraints(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7aaa0)?;
    c.emit(push_registers(C::Al, saved())?)?;
    c.check(0x7aaa4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.check(0x7aaa8)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0xc)?)?;
    c.check(0x7aaac)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7aab0)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0x7aab4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7aab8)?;
    c.emit(mov_register(C::Al, r(8), r(2)))?;
    c.check(0x7aabc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.check(0x7aac0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7aac4)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(1))?)?;
    c.check(0x7aac8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aacc)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7aad0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aad4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6f)?)?;
    c.check(0x7aad8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7aadc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aae0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x8)?)?;
    c.check(0x7aae4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aae8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7aaec)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aaf0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2db)?)?;
    c.check(0x7aaf4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2dc)?)?;
    c.check(0x7aaf8)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(5), 0x2dd)?)?;
    c.check(0x7aafc)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(5), 0x2de)?)?;
    c.check(0x7ab00)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7ab04)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x80)?)?;
    c.check(0x7ab08)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ab0c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2df)?)?;
    c.check(0x7ab10)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x40)?)?;
    c.check(0x7ab14)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7ab18)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7ab1c)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x10)?)?;
    c.check(0x7ab20)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.check(0x7ab24)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x8)?)?;
    c.check(0x7ab28)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ab2c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab30)?;
    c.emit(data_immediate(C::Ne, Op::Orr, r(1), r(1), 0x4)?)?;
    c.check(0x7ab34)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ab38)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0xc)?)?;
    c.check(0x7ab3c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab40)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7ab44)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ab48)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ab50)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ab54)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2e0)?)?;
    c.check(0x7ab58)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab5c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ab60)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ab68)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7ab6c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ab70)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab74)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ab78)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ab7c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ab84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ab88)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ab8c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x166)?)?;
    c.check(0x7ab90)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7ab94)?;
    c.branch(C::Ne, 0x7abc8, false)?;
    c.check(0x7ab98)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ab9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7aba0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7aba4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aba8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7abac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7abb0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7abb4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7abb8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7abbc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x8)?)?;
    c.check(0x7abc0)?;
    c.branch(C::Ne, 0x7aba8, false)?;
    c.check(0x7abc4)?;
    c.branch(C::Al, 0x7abd8, false)?;
    c.check(0x7abc8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7abcc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7abd0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7abd4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7abd8)
}

fn sequence_dimensions(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7abd8)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x2d8)?)?;
    c.check(0x7abdc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7abe0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7abe4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x190)?)?;
    c.check(0x7abe8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7abec)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7abf0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x190)?)?;
    c.check(0x7abf4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7abf8)?;
    c.branch(C::Ne, 0x7ac08, false)?;
    c.check(0x7abfc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ac04)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ac08)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x165)?)?;
    c.check(0x7ac0c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac10)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ac14)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x82c)?)?;
    c.check(0x7ac18)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x254)?)?;
    c.check(0x7ac1c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7ac20)?;
    c.branch(C::Eq, 0x7ac38, false)?;
    c.check(0x7ac24)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ac28)?;
    c.branch(C::Ne, 0x7ac40, false)?;
    c.check(0x7ac2c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ac34)?;
    c.branch(C::Al, 0x7ac48, false)?;
    c.check(0x7ac38)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ac3c)?;
    c.branch(C::Eq, 0x7b154, false)?;
    c.check(0x7ac40)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ac48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ac4c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ac50)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x22c)?)?;
    c.check(0x7ac54)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7ac58)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac5c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ac60)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x230)?)?;
    c.check(0x7ac64)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7ac68)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac6c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ac70)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac74)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ac78)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ac7c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ac80)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ac84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ac88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ac8c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ac90)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x4c)?)?;
    c.check(0x7ac94)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0xf)?)?;
    c.check(0x7ac98)?;
    c.branch(C::Eq, 0x7acb0, false)?;
    c.check(0x7ac9c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aca0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7aca4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7aca8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7acac)?;
    c.branch(C::Al, 0x7acd4, false)?;
    c.check(0x7acb0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x50)?)?;
    c.check(0x7acb4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7acb8)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(6), r(0), 0xf, true)?)?;
    c.check(0x7acbc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7acc0)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7acc4)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7acc8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7accc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7acd0)?;
    c.branch(C::Eq, 0x7ad90, false)?;
    c.check(0x7acd4)
}

fn frame_crop(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7acd4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7acd8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7acdc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7ace0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ace4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2e0)?)?;
    c.check(0x7ace8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7acec)?;
    c.branch(C::Ne, 0x7ad04, false)?;
    c.check(0x7acf0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x4c)?)?;
    c.check(0x7acf4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x22c)?)?;
    c.check(0x7acf8)?;
    c.emit(data_register(C::Al, Op::Rsb, r(0), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7acfc)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 1)?)?;
    c.check(0x7ad00)?;
    c.branch(C::Al, 0x7ad2c, false)?;
    c.check(0x7ad04)?;
    c.literal(r(2), 0x7b15c)?;
    c.check(0x7ad08)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ad0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x7b)?)?;
    c.check(0x7ad10)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(6), r(0), 0x1)?)?;
    c.check(0x7ad14)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.check(0x7ad18)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7ad1c)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0x7ad20)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x4c)?)?;
    c.check(0x7ad24)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x22c)?)?;
    c.check(0x7ad28)?;
    c.emit(data_register(C::Al, Op::Rsb, r(1), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7ad2c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ad30)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ad34)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ad38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ad3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7ad40)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ad44)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2e0)?)?;
    c.check(0x7ad48)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7ad4c)?;
    c.branch(C::Ne, 0x7ad64, false)?;
    c.check(0x7ad50)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x50)?)?;
    c.check(0x7ad54)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x230)?)?;
    c.check(0x7ad58)?;
    c.emit(data_register(C::Al, Op::Rsb, r(0), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7ad5c)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 1)?)?;
    c.check(0x7ad60)?;
    c.branch(C::Al, 0x7ad88, false)?;
    c.check(0x7ad64)?;
    c.literal(r(2), 0x7b15c)?;
    c.check(0x7ad68)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ad6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x28c)?)?;
    c.check(0x7ad70)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(6), r(0), 0x1)?)?;
    c.check(0x7ad74)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7ad78)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0x7ad7c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x50)?)?;
    c.check(0x7ad80)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x230)?)?;
    c.check(0x7ad84)?;
    c.emit(data_register(C::Al, Op::Rsb, r(1), r(0), r(1), Shift::Lsl, 4, false)?)?;
    c.check(0x7ad88)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ad8c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ad90)
}

fn usability_and_aspect_ratio(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7ad90)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7ad94)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ad98)?;
    c.branch(C::Eq, 0x7b000, false)?;
    c.check(0x7ad9c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ada0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ada4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ada8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x1)?)?;
    c.check(0x7adac)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7adb0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7adb4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7adb8)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(0), 0x1)?)?;
    c.check(0x7adbc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7adc0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7adc4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7adc8)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x1)?)?;
    c.check(0x7adcc)?;
    c.branch(C::Eq, 0x7ae28, false)?;
    c.check(0x7add0)?;
    c.emit(transfer_word(C::Al, true, r(6), r(5), 0x1a8)?)?;
    c.check(0x7add4)?;
    c.emit(mov_register(C::Al, r(0), R::SP))?;
    c.check(0x7add8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x4)?)?;
    c.check(0x7addc)?;
    c.emit(transfer_word(C::Al, false, r(6), R::SP, 0x4)?)?;
    c.check(0x7ade0)?;
    c.emit(transfer_word(C::Al, true, r(9), r(5), 0x1ac)?)?;
    c.check(0x7ade4)?;
    c.emit(transfer_word(C::Al, false, r(9), R::SP, 0x8)?)?;
    c.check(0x7ade8)?;
    c.branch(C::Al, 0x7a08c, true)?;
    c.check(0x7adec)?;
    c.emit(mov_register(C::Al, r(7), r(0)))?;
    c.check(0x7adf0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7adf4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7adf8)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7adfc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae00)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0xff)?)?;
    c.check(0x7ae04)?;
    c.branch(C::Ne, 0x7ae28, false)?;
    c.check(0x7ae08)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae0c)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7ae10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7ae14)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae18)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae1c)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.check(0x7ae20)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7ae24)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae28)
}

fn video_signal(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7ae28)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ae30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ae34)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae38)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7ae3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ae40)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(10), r(0), Shift::Lsr, 1, false)?)?;
    c.check(0x7ae44)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae48)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae4c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7ae50)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.check(0x7ae54)?;
    c.branch(C::Eq, 0x7aec4, false)?;
    c.check(0x7ae58)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1c0)?)?;
    c.check(0x7ae5c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7ae64)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae68)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1c4)?)?;
    c.check(0x7ae6c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae70)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ae74)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae78)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1c8)?)?;
    c.check(0x7ae7c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ae84)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ae88)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x1c8)?)?;
    c.check(0x7ae8c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7ae90)?;
    c.branch(C::Ne, 0x7aec4, false)?;
    c.check(0x7ae94)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1cc)?)?;
    c.check(0x7ae98)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ae9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7aea0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aea4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1d0)?)?;
    c.check(0x7aea8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aeac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7aeb0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aeb4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1d4)?)?;
    c.check(0x7aeb8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aebc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7aec0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aec4)
}

fn chroma_and_timing(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7aec4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aec8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7aecc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7aed0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.check(0x7aed4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aed8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7aedc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7aee0)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(6), r(0), Shift::Lsr, 2, false)?)?;
    c.check(0x7aee4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aee8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7aeec)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7aef0)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x4)?)?;
    c.check(0x7aef4)?;
    c.branch(C::Eq, 0x7af54, false)?;
    c.check(0x7aef8)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 0x2)?)?;
    c.check(0x7aefc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7af04)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af08)?;
    c.emit(load_halfword(C::Al, r(1), r(8), 0x0)?)?;
    c.check(0x7af0c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7af14)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af18)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xba)?)?;
    c.check(0x7af1c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7af20)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x100)?)?;
    c.check(0x7af24)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(5), r(0))?)?;
    c.check(0x7af28)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af2c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1b8)?)?;
    c.check(0x7af34)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7af38)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(5), r(0))?)?;
    c.check(0x7af3c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af40)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af44)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x1b4)?)?;
    c.check(0x7af48)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7af50)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af54)
}

fn restrictions(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7af54)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7af5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7af60)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af64)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af68)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7af6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7af70)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af74)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af78)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7af7c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7af80)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af84)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7af88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7af8c)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(6), r(0), Shift::Lsr, 3, false)?)?;
    c.check(0x7af90)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7af94)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7af98)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x264)?)?;
    c.check(0x7af9c)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x8)?)?;
    c.check(0x7afa0)?;
    c.branch(C::Eq, 0x7b010, false)?;
    c.check(0x7afa4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7afa8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7afac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7afb0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7afb4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7afb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.check(0x7afbc)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7afc0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7afc4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7afc8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7afcc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7afd0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9)?)?;
    c.check(0x7afd4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7afd8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7afdc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x8)?)?;
    c.check(0x7afe0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7afe4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0x11c)?)?;
    c.check(0x7afe8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7afec)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7aff0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x165)?)?;
    c.check(0x7aff4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7aff8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7affc)?;
    c.branch(C::Al, 0x7b010, false)?;
    c.check(0x7b000)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b004)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b008)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b00c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b010)
}

fn final_parameters(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b010)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b014)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b018)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b01c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b020)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b024)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b028)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b02c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b030)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b034)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b038)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b03c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b040)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b044)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b048)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b04c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b050)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b054)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b058)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b05c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b060)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b064)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b068)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b06c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b070)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b074)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b078)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b07c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b080)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b084)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b088)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b08c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b090)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b094)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b098)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b09c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b0a0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b0a8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b0ac)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 0xc)?)?;
    c.check(0x7b0b0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7b0b8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b0bc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b0c4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b0c8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b0d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7b0d4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b0d8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b0e0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b0e4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b0ec)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b0f0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b0f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b0f8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b0fc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b100)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b104)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7b108)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b10c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b110)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b114)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b118)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b11c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b120)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b124)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b128)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b12c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7b130)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7b134)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7b138)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b13c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7b140)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7b144)
}

fn restore_tail_and_cold(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7b144)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7b148)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.check(0x7b14c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7b150)?;
    c.branch(C::Al, 0x79c98, false)?;
    c.check(0x7b154)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x10c)?)?;
    c.check(0x7b158)?;
    c.branch(C::Al, 0x7ac24, false)?;
    c.check(0x7b15c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    profile_and_constraints(&mut c)?;
    sequence_dimensions(&mut c)?;
    frame_crop(&mut c)?;
    usability_and_aspect_ratio(&mut c)?;
    video_signal(&mut c)?;
    chroma_and_timing(&mut c)?;
    restrictions(&mut c)?;
    final_parameters(&mut c)?;
    restore_tail_and_cold(&mut c)?;
    c.data(POOL_START, ASSERTION_LOG)?;
    c.check(END)?;
    if c.bytes.len() != END - START { return Err("H.264 parameter writer length mismatch".into()); }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn structure_and_arithmetic() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1728);
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!(word(&bytes, POOL_START), ASSERTION_LOG);
        assert_eq!(saved(), 0x4ff0);
        assert_eq!(word(&bytes, 0x7b14c), transfer_multiple(C::Al, true, R::SP,
            saved(), BlockMode::IncrementAfter, true).unwrap());
        for (at, condition, target) in [
            (0x7ac3c, C::Eq, 0x7b154),
            (0x7b150, C::Al, 0x79c98),
            (0x7b158, C::Al, 0x7ac24),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x7b154), transfer_word(C::Al, true, r(0), r(5), 0x10c).unwrap());
        for at in [0x7ad04, 0x7ad64] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(2), R::PC,
                POOL_START as i32 - at as i32 - 8).unwrap());
        }
        assert_eq!(word(&bytes, 0x7acb8), data_immediate_with_flags(
            C::Al, Op::And, r(6), r(0), 15, true).unwrap());
        for (at, dst) in [(0x7acf8, 0), (0x7ad28, 1), (0x7ad58, 0), (0x7ad84, 1)] {
            assert_eq!(word(&bytes, at), data_register(C::Al, Op::Rsb, r(dst),
                r(0), r(1), Shift::Lsl, 4, false).unwrap());
        }
        for (dimension, pixels, expected) in [(120u32, 1920u32, 0u32),
            (120, 1918, 2), (0, 1, u32::MAX), (0x1000_0000, 2, u32::MAX - 1)] {
            let crop = dimension.wrapping_shl(4).wrapping_sub(pixels);
            assert_eq!(crop, expected);
            assert_eq!(crop >> 1, expected / 2);
        }
        assert_eq!(0x11c | 0x400, 0x51c);
        assert_eq!(0x7b | 0x200, 0x27b);
        assert_eq!(0xba | 0x100, 0x1ba);
    }

    #[test]
    fn exact_code_data_and_control_guards() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(mov_register(C::Al, r(0), r(0))).is_ok(), at < POOL_START);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.data(at, ASSERTION_LOG).is_ok(), at == POOL_START);
        }
        for at in [START - 4, START + 1, POOL_START + 1, END] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.data(at, ASSERTION_LOG).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(POOL_START, ASSERTION_LOG).is_err());
        assert!(c.literal(r(2), POOL_START).is_err());
        for at in [0x7ad04, 0x7ad64] {
            c.at = at;
            assert!(c.literal(r(1), POOL_START).is_err());
            assert!(c.literal(r(2), POOL_START + 1).is_err());
        }
        c.at = 0x7b150;
        for (condition, target, link) in [(C::Eq, 0x79c98, false),
            (C::Al, 0x79c98, true), (C::Al, POOL_START, false),
            (C::Al, 0x79c99, false), (C::Al, END, false)] {
            assert!(c.branch(condition, target, link).is_err());
        }
        c.at = 0x7b158;
        assert!(c.branch(C::Al, 0x79c98, false).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
