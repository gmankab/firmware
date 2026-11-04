use arm32::{branch, branch_exchange_immediate, data_immediate, data_immediate_with_flags,
    data_register, mov_register, mov_shifted_register, multiply_long, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    transfer_word_register, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift};

pub const START: usize = 0x6beb0;
pub const END: usize = 0x6c264;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 948;

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if self.at != at || !(START..END).contains(&at) || at % 4 != 0 {
            return Err(format!("state smoothing address {:#x}, requested {at:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, at: usize, condition: C, target: u32, link: bool) -> Result<(), String> {
        self.emit(at, branch(condition, at as u32, target, link)?)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(CODE_BYTES) };
    c.emit(0x6beb0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2bc)?)?;
    c.emit(0x6beb4, mov_register(C::Al, r(5), r(9)))?;
    c.emit(0x6beb8, data_immediate(C::Al, Op::Tst, r(0), r(6), 8)?)?;
    c.emit(0x6bebc, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6bec0, transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(5), r(0), Shift::Lsl, 0)?)?;
    c.branch(0x6bec4, C::Ne, 0x6bef0, false)?;
    c.emit(0x6bec8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.branch(0x6becc, C::Ne, 0x6c264, false)?;
    c.emit(0x6bed0, mov_register(C::Al, r(4), r(9)))?;
    c.emit(0x6bed4, transfer_word_preindexed_writeback(C::Al, true, r(14), r(4), 0x760)?)?;
    c.emit(0x6bed8, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x6bedc, transfer_word(C::Al, true, r(12), r(4), -4)?)?;
    c.emit(0x6bee0, transfer_word(C::Al, false, r(0), r(11), -72)?)?;
    c.emit(0x6bee4, mov_register(C::Al, r(0), r(4)))?;
    c.emit(0x6bee8, transfer_word_preindexed_writeback(C::Al, true, r(7), r(0), -8)?)?;
    c.branch(0x6beec, C::Al, 0x6bf38, false)?;
    c.emit(0x6bef0, mov_register(C::Al, r(4), r(9)))?;
    c.emit(0x6bef4, data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.emit(0x6bef8, transfer_word_preindexed_writeback(C::Al, true, r(14), r(4), 0x760)?)?;
    c.emit(0x6befc, mov_register(C::Al, r(0), r(4)))?;
    c.emit(0x6bf00, transfer_word(C::Al, true, r(12), r(4), -4)?)?;
    c.emit(0x6bf04, transfer_word(C::Al, true, r(2), r(4), 4)?)?;
    c.emit(0x6bf08, transfer_word_preindexed_writeback(C::Al, true, r(7), r(0), -8)?)?;
    c.emit(0x6bf0c, transfer_word(C::Al, false, r(2), r(11), -72)?)?;
    c.branch(0x6bf10, C::Eq, 0x6bf38, false)?;
    c.emit(0x6bf14, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(0x6bf18, C::Ne, 0x6bf80, false)?;
    c.emit(0x6bf1c, transfer_word(C::Al, true, r(1), r(9), 0x74)?)?;
    c.emit(0x6bf20, transfer_word(C::Al, true, r(2), r(13), 0x38)?)?;
    c.emit(0x6bf24, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(0x6bf28, data_register(C::Al, Op::Sub, r(8), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6bf2c, transfer_word(C::Al, false, r(8), r(0), 0)?)?;
    c.emit(0x6bf30, transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.branch(0x6bf34, C::Al, 0x6bf88, false)?;
    c.emit(0x6bf38, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2c4)?)?;
    c.emit(0x6bf3c, mov_register(C::Al, r(1), r(9)))?;
    c.emit(0x6bf40, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x1000)?)?;
    c.emit(0x6bf44, transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(1), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x6bf48, data_register(C::Al, Op::Sub, r(8), r(7), r(2), Shift::Lsl, 0, true)?)?;
    c.emit(0x6bf4c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x12c0)?)?;
    c.emit(0x6bf50, data_immediate(C::Al, Op::Sbc, r(3), r(12), 0)?)?;
    c.emit(0x6bf54, transfer_word(C::Al, false, r(8), r(0), 0)?)?;
    c.emit(0x6bf58, transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(0x6bf5c, transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(9), r(2), Shift::Lsl, 0)?)?;
    c.emit(0x6bf60, data_immediate_with_flags(C::Al, Op::Sub, r(2), r(2), 1, true)?)?;
    c.emit(0x6bf64, transfer_word(C::Al, false, r(2), r(9), 0)?)?;
    c.branch(0x6bf68, C::Ne, 0x6bf88, false)?;
    c.emit(0x6bf6c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x6bf70, transfer_word(C::Al, false, r(2), r(5), 0)?)?;
    c.emit(0x6bf74, transfer_word(C::Al, false, r(2), r(9), 0)?)?;
    c.emit(0x6bf78, transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.branch(0x6bf7c, C::Al, 0x6bf88, false)?;
    c.emit(0x6bf80, mov_register(C::Al, r(8), r(7)))?;
    c.emit(0x6bf84, mov_register(C::Al, r(3), r(12)))?;
    c.emit(0x6bf88, data_immediate(C::Al, Op::Mov, r(0), r(0), 3)?)?;
    c.emit(0x6bf8c, data_register(C::Al, Op::Sub, r(1), r(8), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6bf90, multiply_long(C::Al, false, r(2), r(5), r(1), r(0))?)?;
    c.emit(0x6bf94, data_register(C::Al, Op::Sbc, r(0), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x6bf98, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(0x6bf9c, mov_shifted_register(C::Al, r(2), r(2), Shift::Lsr, 2)?)?;
    c.emit(0x6bfa0, data_register(C::Al, Op::Add, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x6bfa4, mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 2)?)?;
    c.emit(0x6bfa8, data_register(C::Al, Op::Orr, r(0), r(2), r(0), Shift::Lsl, 0x1e, false)?)?;
    c.emit(0x6bfac, transfer_word(C::Al, true, r(2), r(11), -72)?)?;
    c.emit(0x6bfb0, data_register(C::Al, Op::Add, r(0), r(0), r(14), Shift::Lsl, 0, true)?)?;
    c.emit(0x6bfb4, data_register(C::Al, Op::Adc, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6bfb8, data_immediate_with_flags(C::Al, Op::Rsb, r(3), r(0), 0, true)?)?;
    c.emit(0x6bfbc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x6bfc0, data_immediate_with_flags(C::Al, Op::Rsc, r(3), r(1), 0, true)?)?;
    c.emit(0x6bfc4, data_immediate(C::Lt, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x6bfc8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.emit(0x6bfcc, mov_register(C::Eq, r(1), r(2)))?;
    c.emit(0x6bfd0, mov_register(C::Ne, r(2), r(0)))?;
    c.emit(0x6bfd4, transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.emit(0x6bfd8, transfer_word(C::Al, false, r(1), r(4), 4)?)?;
    c.branch(0x6bfdc, C::Al, 0x6c254, false)?;
    c.emit(0x6bfe0, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2c8)?)?;
    c.emit(0x6bfe4, data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(0x6bfe8, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6bfec, transfer_word_register(C::Al, false, r(1), r(9), r(0), Shift::Lsl, 0)?)?;
    c.emit(0x6bff0, transfer_word(C::Al, true, r(0), r(9), 0x5c)?)?;
    c.emit(0x6bff4, mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 3)?)?;
    c.emit(0x6bff8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2cc)?)?;
    c.emit(0x6bffc, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6c000, transfer_word_register(C::Al, false, r(1), r(9), r(0), Shift::Lsl, 0)?)?;
    c.emit(0x6c004, data_register(C::Al, Op::Sub, r(0), r(2), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c008, mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 1)?)?;
    c.emit(0x6c00c, branch_exchange_immediate(0x6c00c, 0x83c4c)?)?;
    c.emit(0x6c010, transfer_word(C::Al, true, r(4), r(13), 0x38)?)?;
    c.emit(0x6c014, transfer_word(C::Al, true, r(14), r(11), -64)?)?;
    c.emit(0x6c018, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2d0)?)?;
    c.emit(0x6c01c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6c020, transfer_word_register(C::Al, false, r(0), r(9), r(1), Shift::Lsl, 0)?)?;
    c.branch(0x6c024, C::Al, 0x6c0ac, false)?;
    c.emit(0x6c028, transfer_word(C::Al, true, r(4), r(11), 8)?)?;
    c.emit(0x6c02c, transfer_word(C::Al, true, r(1), r(4), 0xab8)?)?;
    c.emit(0x6c030, transfer_word(C::Al, true, r(3), r(4), 0xd08)?)?;
    c.emit(0x6c034, transfer_word(C::Al, false, r(3), r(1), 0x350)?)?;
    c.emit(0x6c038, transfer_word(C::Al, true, r(7), r(4), 0xd0c)?)?;
    c.emit(0x6c03c, transfer_word(C::Al, false, r(7), r(1), 0x354)?)?;
    c.emit(0x6c040, transfer_word(C::Al, true, r(7), r(4), 0xd10)?)?;
    c.branch(0x6c044, C::Al, 0x6bb74, false)?;
    c.emit(0x6c048, data_register(C::Al, Op::Sub, r(3), r(4), r(3), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c04c, data_immediate(C::Al, Op::Mov, r(9), r(0), 0)?)?;
    c.emit(0x6c050, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x6c054, data_immediate(C::Al, Op::Rsc, r(1), r(1), 0)?)?;
    c.emit(0x6c058, transfer_word(C::Al, false, r(9), r(5), 0)?)?;
    c.emit(0x6c05c, transfer_multiple(C::Al, false, r(7), r(4).bit() | r(9).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x6c060, data_immediate(C::Al, Op::Mov, r(7), r(0), 3)?)?;
    c.emit(0x6c064, multiply_long(C::Al, false, r(5), r(4), r(3), r(7))?)?;
    c.emit(0x6c068, data_register(C::Al, Op::Add, r(1), r(1), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(0x6c06c, data_register(C::Al, Op::Add, r(1), r(4), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c070, mov_shifted_register(C::Al, r(7), r(5), Shift::Lsr, 2)?)?;
    c.emit(0x6c074, mov_shifted_register(C::Al, r(3), r(1), Shift::Asr, 2)?)?;
    c.emit(0x6c078, data_register(C::Al, Op::Orr, r(1), r(7), r(1), Shift::Lsl, 0x1e, false)?)?;
    c.emit(0x6c07c, data_register(C::Al, Op::Add, r(1), r(1), r(12), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c080, data_register(C::Al, Op::Adc, r(3), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c084, data_immediate_with_flags(C::Al, Op::Rsb, r(2), r(1), 0, true)?)?;
    c.emit(0x6c088, data_immediate_with_flags(C::Al, Op::Rsc, r(2), r(3), 0, true)?)?;
    c.emit(0x6c08c, data_immediate(C::Lt, Op::Mov, r(9), r(0), 1)?)?;
    c.emit(0x6c090, data_immediate(C::Al, Op::Cmp, r(0), r(9), 0)?)?;
    c.emit(0x6c094, mov_register(C::Eq, r(3), r(9)))?;
    c.emit(0x6c098, mov_register(C::Ne, r(9), r(1)))?;
    c.emit(0x6c09c, transfer_word(C::Al, false, r(9), r(0), 0)?)?;
    c.emit(0x6c0a0, transfer_word(C::Al, false, r(3), r(0), 4)?)?;
    c.emit(0x6c0a4, transfer_word(C::Al, true, r(9), r(11), -60)?)?;
    c.emit(0x6c0a8, transfer_word(C::Al, true, r(4), r(13), 0x38)?)?;
    c.emit(0x6c0ac, data_immediate(C::Al, Op::Tst, r(0), r(6), 0x10)?)?;
    c.branch(0x6c0b0, C::Eq, 0x6c12c, false)?;
    c.emit(0x6c0b4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x25c)?)?;
    c.emit(0x6c0b8, transfer_word(C::Al, true, r(3), r(9), 0x75c)?)?;
    c.emit(0x6c0bc, data_immediate(C::Al, Op::Add, r(0), r(9), 0x2c8)?)?;
    c.emit(0x6c0c0, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6c0c4, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1000)?)?;
    c.emit(0x6c0c8, transfer_word_register(C::Al, true, r(5), r(9), r(1), Shift::Lsl, 0)?)?;
    c.emit(0x6c0cc, transfer_word(C::Al, true, r(1), r(9), 0x758)?)?;
    c.emit(0x6c0d0, data_register(C::Al, Op::Sub, r(2), r(5), r(1), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c0d4, data_immediate_with_flags(C::Al, Op::Rsc, r(2), r(3), 0, true)?)?;
    c.branch(0x6c0d8, C::Cs, 0x6c120, false)?;
    c.emit(0x6c0dc, data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x6c0e0, transfer_word(C::Al, false, r(2), r(0), 0)?)?;
    c.emit(0x6c0e4, transfer_word(C::Al, true, r(0), r(9), 0x5c)?)?;
    c.emit(0x6c0e8, mov_shifted_register(C::Al, r(2), r(0), Shift::Lsr, 3)?)?;
    c.emit(0x6c0ec, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2cc)?)?;
    c.emit(0x6c0f0, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6c0f4, transfer_word_register(C::Al, false, r(2), r(9), r(0), Shift::Lsl, 0)?)?;
    c.emit(0x6c0f8, data_register(C::Al, Op::Sub, r(0), r(1), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c0fc, data_immediate(C::Al, Op::Sbc, r(1), r(3), 0)?)?;
    c.emit(0x6c100, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.branch(0x6c104, C::Al, 0x8187c, true)?;
    c.emit(0x6c108, transfer_word(C::Al, true, r(4), r(13), 0x38)?)?;
    c.emit(0x6c10c, transfer_word(C::Al, true, r(14), r(11), -64)?)?;
    c.emit(0x6c110, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2d0)?)?;
    c.emit(0x6c114, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1000)?)?;
    c.emit(0x6c118, transfer_word_register(C::Al, false, r(0), r(9), r(1), Shift::Lsl, 0)?)?;
    c.branch(0x6c11c, C::Al, 0x6c128, false)?;
    c.emit(0x6c120, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x6c124, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x6c128, transfer_word(C::Al, false, r(5), r(9), 0x70)?)?;
    c.emit(0x6c12c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2c8)?)?;
    c.emit(0x6c130, mov_register(C::Al, r(5), r(9)))?;
    c.emit(0x6c134, data_immediate(C::Al, Op::Tst, r(0), r(6), 0x10)?)?;
    c.emit(0x6c138, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x1000)?)?;
    c.emit(0x6c13c, transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(5), r(0), Shift::Lsl, 0)?)?;
    c.branch(0x6c140, C::Ne, 0x6c170, false)?;
    c.emit(0x6c144, transfer_word(C::Al, true, r(12), r(11), -68)?)?;
    c.emit(0x6c148, data_immediate(C::Al, Op::Cmp, r(0), r(2), 1)?)?;
    c.branch(0x6c14c, C::Ne, 0x6c264, false)?;
    c.emit(0x6c150, mov_register(C::Al, r(1), r(9)))?;
    c.emit(0x6c154, transfer_word_preindexed_writeback(C::Al, true, r(14), r(1), 0x760)?)?;
    c.emit(0x6c158, transfer_word(C::Al, true, r(0), r(1), 4)?)?;
    c.emit(0x6c15c, transfer_word(C::Al, true, r(12), r(1), -4)?)?;
    c.emit(0x6c160, transfer_word(C::Al, false, r(0), r(11), -72)?)?;
    c.emit(0x6c164, mov_register(C::Al, r(0), r(1)))?;
    c.emit(0x6c168, transfer_word_preindexed_writeback(C::Al, true, r(7), r(0), -8)?)?;
    c.branch(0x6c16c, C::Al, 0x6c1ac, false)?;
    c.emit(0x6c170, mov_register(C::Al, r(1), r(9)))?;
    c.emit(0x6c174, data_immediate(C::Al, Op::Cmp, r(0), r(2), 1)?)?;
    c.emit(0x6c178, transfer_word_preindexed_writeback(C::Al, true, r(14), r(1), 0x760)?)?;
    c.emit(0x6c17c, mov_register(C::Al, r(0), r(1)))?;
    c.emit(0x6c180, transfer_word(C::Al, true, r(12), r(1), -4)?)?;
    c.emit(0x6c184, transfer_word(C::Al, true, r(3), r(1), 4)?)?;
    c.emit(0x6c188, transfer_word_preindexed_writeback(C::Al, true, r(7), r(0), -8)?)?;
    c.emit(0x6c18c, transfer_word(C::Al, false, r(3), r(11), -72)?)?;
    c.branch(0x6c190, C::Eq, 0x6c1ac, false)?;
    c.emit(0x6c194, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.branch(0x6c198, C::Ne, 0x6c1fc, false)?;
    c.emit(0x6c19c, transfer_word(C::Al, true, r(4), r(9), 0x70)?)?;
    c.emit(0x6c1a0, data_immediate(C::Al, Op::Mov, r(9), r(0), 0)?)?;
    c.emit(0x6c1a4, transfer_multiple(C::Al, false, r(0), r(4).bit() | r(9).bit(), BlockMode::IncrementAfter, false)?)?;
    c.branch(0x6c1a8, C::Al, 0x6c204, false)?;
    c.emit(0x6c1ac, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x2d0)?)?;
    c.emit(0x6c1b0, mov_register(C::Al, r(2), r(9)))?;
    c.emit(0x6c1b4, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x2cc)?)?;
    c.emit(0x6c1b8, mov_register(C::Al, r(3), r(9)))?;
    c.emit(0x6c1bc, data_immediate(C::Al, Op::Orr, r(4), r(4), 0x1000)?)?;
    c.emit(0x6c1c0, data_immediate(C::Al, Op::Orr, r(8), r(8), 0x1000)?)?;
    c.emit(0x6c1c4, transfer_word_register_preindexed_writeback(C::Al, true, r(4), r(2), r(4), Shift::Lsl, 0)?)?;
    c.emit(0x6c1c8, data_register(C::Al, Op::Sub, r(4), r(7), r(4), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c1cc, data_immediate(C::Al, Op::Sbc, r(9), r(12), 0)?)?;
    c.emit(0x6c1d0, transfer_multiple(C::Al, false, r(0), r(4).bit() | r(9).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x6c1d4, mov_register(C::Al, r(0), r(3)))?;
    c.emit(0x6c1d8, transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(0), r(8), Shift::Lsl, 0)?)?;
    c.emit(0x6c1dc, data_immediate_with_flags(C::Al, Op::Sub, r(3), r(3), 1, true)?)?;
    c.emit(0x6c1e0, transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.branch(0x6c1e4, C::Ne, 0x6c204, false)?;
    c.emit(0x6c1e8, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(0x6c1ec, transfer_word(C::Al, false, r(3), r(5), 0)?)?;
    c.emit(0x6c1f0, transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(0x6c1f4, transfer_word(C::Al, false, r(3), r(2), 0)?)?;
    c.branch(0x6c1f8, C::Al, 0x6c204, false)?;
    c.emit(0x6c1fc, mov_register(C::Al, r(4), r(7)))?;
    c.emit(0x6c200, mov_register(C::Al, r(9), r(12)))?;
    c.emit(0x6c204, data_immediate(C::Al, Op::Mov, r(0), r(0), 3)?)?;
    c.emit(0x6c208, data_register(C::Al, Op::Sub, r(2), r(4), r(7), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c20c, multiply_long(C::Al, false, r(3), r(4), r(2), r(0))?)?;
    c.emit(0x6c210, data_register(C::Al, Op::Sbc, r(0), r(9), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c214, data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 1, false)?)?;
    c.emit(0x6c218, mov_shifted_register(C::Al, r(3), r(3), Shift::Lsr, 2)?)?;
    c.emit(0x6c21c, data_register(C::Al, Op::Add, r(0), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c220, mov_shifted_register(C::Al, r(2), r(0), Shift::Asr, 2)?)?;
    c.emit(0x6c224, data_register(C::Al, Op::Orr, r(0), r(3), r(0), Shift::Lsl, 0x1e, false)?)?;
    c.emit(0x6c228, transfer_word(C::Al, true, r(3), r(11), -72)?)?;
    c.emit(0x6c22c, data_register(C::Al, Op::Add, r(0), r(0), r(14), Shift::Lsl, 0, true)?)?;
    c.emit(0x6c230, data_register(C::Al, Op::Adc, r(3), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6c234, data_immediate_with_flags(C::Al, Op::Rsb, r(7), r(0), 0, true)?)?;
    c.emit(0x6c238, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x6c23c, data_immediate_with_flags(C::Al, Op::Rsc, r(7), r(3), 0, true)?)?;
    c.emit(0x6c240, data_immediate(C::Lt, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x6c244, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.emit(0x6c248, mov_register(C::Eq, r(3), r(2)))?;
    c.emit(0x6c24c, mov_register(C::Ne, r(2), r(0)))?;
    c.emit(0x6c250, transfer_doubleword(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(0x6c254, transfer_word(C::Al, true, r(9), r(11), -60)?)?;
    c.emit(0x6c258, transfer_word(C::Al, true, r(14), r(11), -64)?)?;
    c.emit(0x6c25c, transfer_word(C::Al, true, r(12), r(11), -68)?)?;
    c.emit(0x6c260, transfer_word(C::Al, true, r(4), r(13), 0x38)?)?;
    if c.at != END || c.bytes.len() != CODE_BYTES {
        return Err("state smoothing extent mismatch".into());
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
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn fixed_extent_and_address_guards() {
        assert_eq!(fragment().unwrap().len(), CODE_BYTES);
        assert_eq!(END - START, CODE_BYTES);
        for at in [START - 4, START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, mov_register(C::Al, r(0), r(1))).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, mov_register(C::Al, r(0), r(1))).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn fragment_edges_and_out_of_line_entries() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x6becc, C::Ne, 0x6c264, false),
            (0x6bfdc, C::Al, 0x6c254, false),
            (0x6c024, C::Al, 0x6c0ac, false),
            (0x6c044, C::Al, 0x6bb74, false),
            (0x6c0d8, C::Cs, 0x6c120, false),
            (0x6c104, C::Al, 0x8187c, true),
            (0x6c14c, C::Ne, 0x6c264, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x6c00c), branch_exchange_immediate(0x6c00c, 0x83c4c).unwrap());
        assert_eq!(word(&bytes, 0x6c048), data_register(C::Al, Op::Sub, r(3), r(4), r(3), Shift::Lsl, 0, true).unwrap());
    }

    #[test]
    fn carry_chains_and_nonnegative_clamps() {
        let bytes = fragment().unwrap();
        for (at, op, rd, rn, rm, flags) in [
            (0x6bf48, Op::Sub, 8, 7, 2, true),
            (0x6bf94, Op::Sbc, 0, 3, 12, false),
            (0x6bfb0, Op::Add, 0, 0, 14, true),
            (0x6bfb4, Op::Adc, 1, 1, 2, false),
            (0x6c07c, Op::Add, 1, 1, 12, true),
            (0x6c080, Op::Adc, 3, 3, 2, false),
            (0x6c22c, Op::Add, 0, 0, 14, true),
            (0x6c230, Op::Adc, 3, 2, 3, false),
        ] {
            assert_eq!(word(&bytes, at), data_register(C::Al, op, r(rd), r(rn), r(rm), Shift::Lsl, 0, flags).unwrap());
        }
        for (at, rd, rn) in [(0x6bfc0, 3, 1), (0x6c088, 2, 3), (0x6c0d4, 2, 3), (0x6c23c, 7, 3)] {
            assert_eq!(word(&bytes, at), data_immediate_with_flags(C::Al, Op::Rsc, r(rd), r(rn), 0, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x6c054), data_immediate(C::Al, Op::Rsc, r(1), r(1), 0).unwrap());
        for (at, rd) in [(0x6bfc4, 2), (0x6c08c, 9), (0x6c240, 2)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Lt, Op::Mov, r(rd), r(0), 1).unwrap());
        }
    }

    #[test]
    fn paired_values_countdowns_and_three_quarter_shifts() {
        let bytes = fragment().unwrap();
        for (at, rd, rn) in [(0x6bed4, 14, 4), (0x6bef8, 14, 4), (0x6c154, 14, 1), (0x6c178, 14, 1)] {
            assert_eq!(word(&bytes, at), transfer_word_preindexed_writeback(C::Al, true, r(rd), r(rn), 0x760).unwrap());
        }
        for (at, rd) in [(0x6bf60, 2), (0x6c1dc, 3)] {
            assert_eq!(word(&bytes, at), data_immediate_with_flags(C::Al, Op::Sub, r(rd), r(rd), 1, true).unwrap());
        }
        for (at, rd, rm) in [(0x6bfa4, 1, 0), (0x6c074, 3, 1), (0x6c220, 2, 0)] {
            assert_eq!(word(&bytes, at), mov_shifted_register(C::Al, r(rd), r(rm), Shift::Asr, 2).unwrap());
        }
        assert_eq!(word(&bytes, 0x6c250), transfer_doubleword(C::Al, false, r(2), r(1), 0).unwrap());
    }
}
