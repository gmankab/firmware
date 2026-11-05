use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch, data_immediate, data_register, data_register_shifted_by_register, load_signed_byte, mov_register, mov_shifted_register, push_registers, transfer_byte, transfer_multiple, transfer_word, transfer_word_postindexed, transfer_word_preindexed_writeback
};

pub const START: usize = 0x7ba5c;
pub const END: usize = 0x7c25c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2032;
pub const DATA_BYTES: usize = 16;
pub const FUNCTION_STARTS: [usize; 2] = [START, 0x7c0fc];

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("HEVC reference sets at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..0x7c24c).contains(&at) && at % 4 == 0
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0x7c24c => Some(0xf42b7),
            0x7c250 => Some(0xf42f0),
            0x7c254 => Some(0xe2788),
            0x7c258 => Some(0xf4260),
            _ => None,
        }
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) { return Err("HEVC instruction outside code".into()); }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) { return Err("HEVC literal mismatch".into()); }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        let valid = matches!((self.at, rd.number(), target),
            (0x7c13c, 1, 0x7c24c)
            | (0x7c140, 2, 0x7c250)
            | (0x7c150, 1, 0x7c254)
            | (0x7c15c, 2, 0x7c258)
        );
        if !valid { return Err("invalid HEVC literal load".into()); }
        self.emit(transfer_word(C::Al, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7ba9c, C::Al, 0x79db8, true)
            | (0x7baac, C::Cc, 0x7be18, false)
            | (0x7bad4, C::Al, 0x798f8, true)
            | (0x7baec, C::Eq, 0x7bcf4, false)
            | (0x7bb0c, C::Al, 0x798f8, true)
            | (0x7bb30, C::Al, 0x7c0fc, true)
            | (0x7bb68, C::Ne, 0x7bb54, false)
            | (0x7bb74, C::Al, 0x79db8, true)
            | (0x7bb7c, C::Eq, 0x7bdec, false)
            | (0x7bbc8, C::Eq, 0x7bcdc, false)
            | (0x7bbdc, C::Al, 0x798f8, true)
            | (0x7bbe8, C::Lt, 0x7bc10, false)
            | (0x7bbfc, C::Ne, 0x7bc10, false)
            | (0x7bc0c, C::Al, 0x7bc1c, false)
            | (0x7bc20, C::Al, 0x798f8, true)
            | (0x7bc44, C::Eq, 0x7bc7c, false)
            | (0x7bc5c, C::Al, 0x798f8, true)
            | (0x7bc68, C::Ne, 0x7bcbc, false)
            | (0x7bc78, C::Al, 0x7bcdc, false)
            | (0x7bca4, C::Al, 0x798f8, true)
            | (0x7bcb8, C::Eq, 0x7bcd4, false)
            | (0x7bcc4, C::Al, 0x79db8, true)
            | (0x7bcd0, C::Al, 0x7bcdc, false)
            | (0x7bcec, C::Hi, 0x7bbbc, false)
            | (0x7bcf0, C::Al, 0x7bdec, false)
            | (0x7bcfc, C::Eq, 0x7bd5c, false)
            | (0x7bd08, C::Cs, 0x7bdb8, false)
            | (0x7bd1c, C::Al, 0x798f8, true)
            | (0x7bd40, C::Cc, 0x7bd2c, false)
            | (0x7bd48, C::Eq, 0x7bdec, false)
            | (0x7bd54, C::Al, 0x798f8, true)
            | (0x7bd58, C::Al, 0x7bdec, false)
            | (0x7bd6c, C::Al, 0x798f8, true)
            | (0x7bd7c, C::Al, 0x798f8, true)
            | (0x7bd88, C::Al, 0x79db8, true)
            | (0x7bd94, C::Al, 0x79db8, true)
            | (0x7bda0, C::Al, 0x79db8, true)
            | (0x7bdb0, C::Al, 0x798f8, true)
            | (0x7bdb4, C::Al, 0x7bdec, false)
            | (0x7bdcc, C::Al, 0x798f8, true)
            | (0x7bde8, C::Al, 0x7c0fc, true)
            | (0x7be04, C::Eq, 0x7be18, false)
            | (0x7be14, C::Al, 0x798f8, true)
            | (0x7be24, C::Eq, 0x7be4c, false)
            | (0x7be38, C::Al, 0x798f8, true)
            | (0x7be48, C::Al, 0x798f8, true)
            | (0x7be54, C::Eq, 0x7bfcc, false)
            | (0x7be68, C::Al, 0x798f8, true)
            | (0x7be78, C::Al, 0x79db8, true)
            | (0x7be84, C::Ne, 0x7be9c, false)
            | (0x7be94, C::Al, 0x79db8, true)
            | (0x7bea0, C::Eq, 0x7bfcc, false)
            | (0x7beac, C::Eq, 0x7bed8, false)
            | (0x7beb8, C::Al, 0x7ba08, true)
            | (0x7bec0, C::Cc, 0x7bed8, false)
            | (0x7bed0, C::Al, 0x798f8, true)
            | (0x7bedc, C::Ne, 0x7bfa8, false)
            | (0x7bee8, C::Eq, 0x7bf94, false)
            | (0x7bef4, C::Al, 0x7ba08, true)
            | (0x7befc, C::Cc, 0x7bf94, false)
            | (0x7bf08, C::Eq, 0x7bf84, false)
            | (0x7bf18, C::Al, 0x798f8, true)
            | (0x7bf2c, C::Cc, 0x7bf94, false)
            | (0x7bf44, C::Ne, 0x7bf38, false)
            | (0x7bf50, C::Eq, 0x7bf94, false)
            | (0x7bf68, C::Al, 0x798f8, true)
            | (0x7bf7c, C::Cc, 0x7bf5c, false)
            | (0x7bf80, C::Al, 0x7bf94, false)
            | (0x7bf90, C::Al, 0x798f8, true)
            | (0x7bfa0, C::Al, 0x798f8, true)
            | (0x7bfb4, C::Eq, 0x7bfcc, false)
            | (0x7bfc8, C::Al, 0x798f8, true)
            | (0x7bfd8, C::Eq, 0x7c040, false)
            | (0x7bfe4, C::Ne, 0x7bffc, false)
            | (0x7bff4, C::Al, 0x798f8, true)
            | (0x7c000, C::Eq, 0x7c040, false)
            | (0x7c00c, C::Eq, 0x7c028, false)
            | (0x7c014, C::Ne, 0x7c040, false)
            | (0x7c020, C::Hi, 0x7c034, false)
            | (0x7c024, C::Al, 0x7c040, false)
            | (0x7c030, C::Cc, 0x7c040, false)
            | (0x7c03c, C::Al, 0x79db8, true)
            | (0x7c048, C::Eq, 0x7c060, false)
            | (0x7c05c, C::Al, 0x79db8, true)
            | (0x7c088, C::Al, 0x79df0, true)
            | (0x7c098, C::Eq, 0x7c0c0, false)
            | (0x7c0ac, C::Ne, 0x7c0c0, false)
            | (0x7c0bc, C::Al, 0x798f8, true)
            | (0x7c0c8, C::Eq, 0x7c0d8, false)
            | (0x7c0d4, C::Al, 0x79db8, true)
            | (0x7c0e4, C::Al, 0x79c18, true)
            | (0x7c0ec, C::Al, 0x79c98, true)
            | (0x7c11c, C::Eq, 0x7c130, false)
            | (0x7c12c, C::Al, 0x798f8, true)
            | (0x7c138, C::Eq, 0x7c174, false)
            | (0x7c14c, C::Al, 0x2ec94, true)
            | (0x7c158, C::Al, 0x2ec94, true)
            | (0x7c170, C::Al, 0x17b0, false)
            | (0x7c17c, C::Al, 0x79db8, true)
            | (0x7c188, C::Al, 0x79db8, true)
            | (0x7c190, C::Eq, 0x7c1ec, false)
            | (0x7c1b0, C::Al, 0x79db8, true)
            | (0x7c1bc, C::Eq, 0x7c1cc, false)
            | (0x7c1c8, C::Al, 0x7c1d4, false)
            | (0x7c1d8, C::Al, 0x798f8, true)
            | (0x7c1e8, C::Ne, 0x7c1a0, false)
            | (0x7c1f4, C::Eq, 0x7c244, false)
            | (0x7c214, C::Al, 0x79db8, true)
            | (0x7c228, C::Al, 0x798f8, true)
            | (0x7c240, C::Cc, 0x7c200, false)
        );
        if !valid { return Err("invalid HEVC reference set branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn slice_reference_setup(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7ba5c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7ba60)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.check(0x7ba64)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x2c)?)?;
    c.check(0x7ba68)?;
    c.emit(mov_register(C::Al, r(10), r(1)))?;
    c.check(0x7ba6c)?;
    c.emit(mov_register(C::Al, r(8), r(0)))?;
    c.check(0x7ba70)?;
    c.emit(mov_register(C::Al, r(5), r(3)))?;
    c.check(0x7ba74)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x4cc)?)?;
    c.check(0x7ba78)?;
    c.emit(transfer_byte(C::Al, true, r(4), r(8), 0x720)?)?;
    c.check(0x7ba7c)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(8), r(0), Shift::Lsl, 2, false)?)?;
    c.check(0x7ba80)?;
    c.emit(transfer_word(C::Al, true, r(6), r(0), 0x74c)?)?;
    c.check(0x7ba84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7ba88)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(2), 0xc)?)?;
    c.check(0x7ba8c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x20)?)?;
    c.check(0x7ba90)?;
    c.emit(transfer_word(C::Al, false, r(2), r(11), -0x20)?)?;
    c.check(0x7ba94)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(1), r(0), 0x2)?)?;
    c.check(0x7ba98)?;
    c.emit(mov_register(C::Al, r(0), r(2)))?;
    c.check(0x7ba9c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7baa0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x280)?)?;
    c.check(0x7baa4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x13)?)?;
    c.check(0x7baa8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7baac)?;
    c.branch(C::Cc, 0x7be18, false)?;
    c.check(0x7bab0)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 0xc)?)?;
    c.check(0x7bab4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x10)?)?;
    c.check(0x7bab8)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(2), r(0), 0x0)?)?;
    c.check(0x7babc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x720)?)?;
    c.check(0x7bac0)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, r(0), false)?)?;
    c.check(0x7bac4)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 0x26c)?)?;
    c.check(0x7bac8)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.check(0x7bacc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x4)?)?;
    c.check(0x7bad0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bad4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bad8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x10c)?)?;
    c.check(0x7badc)?;
    c.emit(transfer_byte(C::Al, true, r(9), r(10), 0x4cc)?)?;
    c.check(0x7bae0)?;
    c.emit(transfer_word(C::Al, false, r(10), r(13), 0x1c)?)?;
    c.check(0x7bae4)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(6).bit() | r(8).bit(), BlockMode::IncrementBefore, false)?)?;
    c.check(0x7bae8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7baec)?;
    c.branch(C::Eq, 0x7bcf4, false)?;
    c.check(0x7baf0)?;
    c.emit(transfer_word(C::Al, true, r(7), r(11), -0x20)?)?;
    c.check(0x7baf4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x4)?)?;
    c.check(0x7baf8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bafc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bb00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7bb04)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x24)?)?;
    c.check(0x7bb08)?;
    c.emit(mov_register(C::Al, r(0), r(7)))?;
    c.check(0x7bb0c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bb10)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(8), r(9), Shift::Lsl, 2, false)?)?;
    c.check(0x7bb14)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0xc)?)?;
    c.check(0x7bb18)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x1dc)?)?;
    c.check(0x7bb1c)?;
    c.emit(mov_register(C::Al, r(3), r(7)))?;
    c.check(0x7bb20)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x74c)?)?;
    c.check(0x7bb24)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 0x678)?)?;
    c.check(0x7bb28)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 0x0)?)?;
    c.check(0x7bb2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bb30)?;
    c.branch(C::Al, 0x7c0fc, true)?;
    c.check(0x7bb34)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.check(0x7bb38)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(1), r(0), 0x248)?)?;
    c.check(0x7bb3c)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(1), Shift::Lsl, 3, false)?)?;
    c.check(0x7bb40)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.check(0x7bb44)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x30)?)?;
    c.check(0x7bb48)?;
    c.emit(load_signed_byte(C::Al, r(1), r(1), 0x73)?)?;
    c.check(0x7bb4c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x18)?)?;
    c.check(0x7bb50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bb54)?;
    c.emit(transfer_word_postindexed(C::Al, true, r(2), r(0), 0x4)?)?;
    c.check(0x7bb58)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.check(0x7bb5c)?;
    c.emit(data_immediate(C::Al, Op::Cmn, r(0), r(2), 0x1)?)?;
    c.check(0x7bb60)?;
    c.emit(data_immediate(C::Ne, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7bb64)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4)?)?;
    c.check(0x7bb68)?;
    c.branch(C::Ne, 0x7bb54, false)?;
    c.check(0x7bb6c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bb70)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7bb74)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7bb78)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7bb7c)?;
    c.branch(C::Eq, 0x7bdec, false)?;
    c.check(0x7bb80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.check(0x7bb84)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x0)?)?;
    c.check(0x7bb88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x1)?)?;
    c.check(0x7bb8c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(10), r(0), 0x0)?)?;
    c.check(0x7bb90)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7bb94)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, r(4), false)?)?;
    c.check(0x7bb98)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bb9c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x14)?)?;
    c.check(0x7bba0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x18)?)?;
    c.check(0x7bba4)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x20)?)?;
    c.check(0x7bba8)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.check(0x7bbac)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(8), r(0), Shift::Lsl, 3, false)?)?;
    c.check(0x7bbb0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x290)?)?;
    c.check(0x7bbb4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x10)?)?;
    c.check(0x7bbb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7bbbc)?;
    c.emit(data_register(C::Al, Op::Add, r(4), r(8), r(0), Shift::Lsl, 2, false)?)?;
    c.check(0x7bbc0)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(0), r(4), 0x278)?)?;
    c.check(0x7bbc4)?;
    c.emit(data_immediate(C::Al, Op::Cmn, r(0), r(0), 0x1)?)?;
    c.check(0x7bbc8)?;
    c.branch(C::Eq, 0x7bcdc, false)?;
    c.check(0x7bbcc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x14)?)?;
    c.check(0x7bbd0)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x24)?)?;
    c.check(0x7bbd4)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7bbd8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bbdc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bbe0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x18)?)?;
    c.check(0x7bbe4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bbe8)?;
    c.branch(C::Lt, 0x7bc10, false)?;
    c.check(0x7bbec)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x10)?)?;
    c.check(0x7bbf0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x0)?)?;
    c.check(0x7bbf4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0x0)?)?;
    c.check(0x7bbf8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7bbfc)?;
    c.branch(C::Ne, 0x7bc10, false)?;
    c.check(0x7bc00)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bc04)?;
    c.emit(mov_register(C::Al, r(5), r(7)))?;
    c.check(0x7bc08)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7bc0c)?;
    c.branch(C::Al, 0x7bc1c, false)?;
    c.check(0x7bc10)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bc14)?;
    c.emit(mov_register(C::Al, r(5), r(7)))?;
    c.check(0x7bc18)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bc1c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bc20)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bc24)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x1c)?)?;
    c.check(0x7bc28)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 0x24)?)?;
    c.check(0x7bc2c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.check(0x7bc30)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(9), 0xff)?)?;
    c.check(0x7bc34)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x26c)?)?;
    c.check(0x7bc38)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Asr, r(2), false)?)?;
    c.check(0x7bc3c)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, r(2), false)?)?;
    c.check(0x7bc40)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Sub, r(4), r(0), r(1), Shift::Asr, r(2), false)?)?;
    c.check(0x7bc44)?;
    c.branch(C::Eq, 0x7bc7c, false)?;
    c.check(0x7bc48)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bc4c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7bc50)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7bc54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bc58)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7bc5c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bc60)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7bc64)?;
    c.emit(mov_register(C::Al, r(7), r(4)))?;
    c.check(0x7bc68)?;
    c.branch(C::Ne, 0x7bcbc, false)?;
    c.check(0x7bc6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.check(0x7bc70)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x20)?)?;
    c.check(0x7bc74)?;
    c.emit(mov_register(C::Al, r(7), r(4)))?;
    c.check(0x7bc78)?;
    c.branch(C::Al, 0x7bcdc, false)?;
    c.check(0x7bc7c)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0x20)?)?;
    c.check(0x7bc80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7bc84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bc88)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(8), Shift::Lsl, 0, false)?)?;
    c.check(0x7bc8c)?;
    c.emit(data_immediate(C::Cs, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7bc90)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7bc94)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(7), r(0), 0x1)?)?;
    c.check(0x7bc98)?;
    c.emit(data_register(C::Al, Op::And, r(9), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7bc9c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bca0)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.check(0x7bca4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bca8)?;
    c.emit(data_register(C::Al, Op::Sub, r(7), r(4), r(8), Shift::Lsl, 0, true)?)?;
    c.check(0x7bcac)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0xc)?)?;
    c.check(0x7bcb0)?;
    c.emit(mov_register(C::Cc, r(7), r(5)))?;
    c.check(0x7bcb4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.check(0x7bcb8)?;
    c.branch(C::Eq, 0x7bcd4, false)?;
    c.check(0x7bcbc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bcc0)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7bcc4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7bcc8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.check(0x7bccc)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x20)?)?;
    c.check(0x7bcd0)?;
    c.branch(C::Al, 0x7bcdc, false)?;
    c.check(0x7bcd4)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x20)?)?;
    c.check(0x7bcd8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.check(0x7bcdc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(10), 0x1)?)?;
    c.check(0x7bce0)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(10), Shift::Lsl, 16)?)?;
    c.check(0x7bce4)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(1), Shift::Asr, 16)?)?;
    c.check(0x7bce8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(1), Shift::Asr, 16, false)?)?;
    c.check(0x7bcec)?;
    c.branch(C::Hi, 0x7bbbc, false)?;
    c.check(0x7bcf0)?;
    c.branch(C::Al, 0x7bdec, false)?;
    c.check(0x7bcf4)
}

fn short_term_selection(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7bcf4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x20)?)?;
    c.check(0x7bcf8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bcfc)?;
    c.branch(C::Eq, 0x7bd5c, false)?;
    c.check(0x7bd00)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x71a)?)?;
    c.check(0x7bd04)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(9), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7bd08)?;
    c.branch(C::Cs, 0x7bdb8, false)?;
    c.check(0x7bd0c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bd10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7bd14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bd18)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x1)?)?;
    c.check(0x7bd1c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bd20)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x71a)?)?;
    c.check(0x7bd24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.check(0x7bd28)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(2), r(0), 0x0)?)?;
    c.check(0x7bd2c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.check(0x7bd30)?;
    c.emit(mov_register(C::Al, r(1), r(3)))?;
    c.check(0x7bd34)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(6), r(0), r(4), Shift::Lsl, r(2), false)?)?;
    c.check(0x7bd38)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7bd3c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7bd40)?;
    c.branch(C::Cc, 0x7bd2c, false)?;
    c.check(0x7bd44)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7bd48)?;
    c.branch(C::Eq, 0x7bdec, false)?;
    c.check(0x7bd4c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bd50)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.check(0x7bd54)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bd58)?;
    c.branch(C::Al, 0x7bdec, false)?;
    c.check(0x7bd5c)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -0x20)?)?;
    c.check(0x7bd60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bd64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bd68)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7bd6c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bd70)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7bd74)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bd78)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bd7c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bd80)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7bd84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7bd88)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7bd8c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7bd90)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bd94)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7bd98)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7bd9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bda0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7bda4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7bda8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bdac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bdb0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bdb4)?;
    c.branch(C::Al, 0x7bdec, false)?;
    c.check(0x7bdb8)?;
    c.emit(transfer_word(C::Al, true, r(5), r(11), -0x20)?)?;
    c.check(0x7bdbc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bdc0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bdc4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7bdc8)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7bdcc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bdd0)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(8), r(9), Shift::Lsl, 2, false)?)?;
    c.check(0x7bdd4)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 0x71a)?)?;
    c.check(0x7bdd8)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x1dc)?)?;
    c.check(0x7bddc)?;
    c.emit(mov_register(C::Al, r(3), r(5)))?;
    c.check(0x7bde0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x74c)?)?;
    c.check(0x7bde4)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 0x0)?)?;
    c.check(0x7bde8)?;
    c.branch(C::Al, 0x7c0fc, true)?;
    c.check(0x7bdec)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0x8)?)?;
    c.check(0x7bdf0)?;
    c.emit(transfer_word(C::Al, true, r(10), r(13), 0x1c)?)?;
    c.check(0x7bdf4)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 0x4)?)?;
    c.check(0x7bdf8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7bdfc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x33c)?)?;
    c.check(0x7be00)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7be04)?;
    c.branch(C::Eq, 0x7be18, false)?;
    c.check(0x7be08)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x278)?)?;
    c.check(0x7be0c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7be10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7be14)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7be18)
}

fn slice_reference_flags(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7be18)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7be1c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x14)?)?;
    c.check(0x7be20)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7be24)?;
    c.branch(C::Eq, 0x7be4c, false)?;
    c.check(0x7be28)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -0x20)?)?;
    c.check(0x7be2c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x279)?)?;
    c.check(0x7be30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7be34)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7be38)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7be3c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x27a)?)?;
    c.check(0x7be40)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7be44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7be48)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7be4c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x20)?)?;
    c.check(0x7be50)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7be54)?;
    c.branch(C::Eq, 0x7bfcc, false)?;
    c.check(0x7be58)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -0x20)?)?;
    c.check(0x7be5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7be60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7be64)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7be68)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7be6c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27d)?)?;
    c.check(0x7be70)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7be74)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7be78)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7be7c)?;
    c.emit(transfer_word(C::Al, true, r(5), r(10), 0x20)?)?;
    c.check(0x7be80)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x2)?)?;
    c.check(0x7be84)?;
    c.branch(C::Ne, 0x7be9c, false)?;
    c.check(0x7be88)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27e)?)?;
    c.check(0x7be8c)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7be90)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7be94)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7be98)?;
    c.emit(transfer_word(C::Al, true, r(5), r(10), 0x20)?)?;
    c.check(0x7be9c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0x7bea0)?;
    c.branch(C::Eq, 0x7bfcc, false)?;
    c.check(0x7bea4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x728)?)?;
    c.check(0x7bea8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7beac)?;
    c.branch(C::Eq, 0x7bed8, false)?;
    c.check(0x7beb0)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7beb4)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7beb8)?;
    c.branch(C::Al, 0x7ba08, true)?;
    c.check(0x7bebc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7bec0)?;
    c.branch(C::Cc, 0x7bed8, false)?;
    c.check(0x7bec4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bec8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7becc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bed0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bed4)?;
    c.emit(transfer_word(C::Al, true, r(5), r(10), 0x20)?)?;
    c.check(0x7bed8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x2)?)?;
    c.check(0x7bedc)?;
    c.branch(C::Ne, 0x7bfa8, false)?;
    c.check(0x7bee0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x728)?)?;
    c.check(0x7bee4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bee8)?;
    c.branch(C::Eq, 0x7bf94, false)?;
    c.check(0x7beec)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7bef0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.check(0x7bef4)?;
    c.branch(C::Al, 0x7ba08, true)?;
    c.check(0x7bef8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7befc)?;
    c.branch(C::Cc, 0x7bf94, false)?;
    c.check(0x7bf00)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27c)?)?;
    c.check(0x7bf04)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bf08)?;
    c.branch(C::Eq, 0x7bf84, false)?;
    c.check(0x7bf0c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bf10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7bf14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bf18)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bf1c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x0)?)?;
    c.check(0x7bf20)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x1)?)?;
    c.check(0x7bf24)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7bf28)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7bf2c)?;
    c.branch(C::Cc, 0x7bf94, false)?;
    c.check(0x7bf30)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7bf34)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7bf38)?;
    c.emit(mov_shifted_register(C::Al, r(2), r(1), Shift::Asr, 1)?)?;
    c.check(0x7bf3c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Asr, 1, false)?)?;
    c.check(0x7bf40)?;
    c.emit(mov_register(C::Al, r(1), r(2)))?;
    c.check(0x7bf44)?;
    c.branch(C::Ne, 0x7bf38, false)?;
    c.check(0x7bf48)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27e)?)?;
    c.check(0x7bf4c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bf50)?;
    c.branch(C::Eq, 0x7bf94, false)?;
    c.check(0x7bf54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.check(0x7bf58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.check(0x7bf5c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bf60)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7bf64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bf68)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bf6c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27e)?)?;
    c.check(0x7bf70)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x1)?)?;
    c.check(0x7bf74)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(5), r(5), 0x1)?)?;
    c.check(0x7bf78)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7bf7c)?;
    c.branch(C::Cc, 0x7bf5c, false)?;
    c.check(0x7bf80)?;
    c.branch(C::Al, 0x7bf94, false)?;
    c.check(0x7bf84)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bf88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bf8c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bf90)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bf94)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bf98)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7bf9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bfa0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bfa4)?;
    c.emit(transfer_word(C::Al, true, r(5), r(10), 0x20)?)?;
    c.check(0x7bfa8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0x7bfac)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(8), 0x726)?)?;
    c.check(0x7bfb0)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bfb4)?;
    c.branch(C::Eq, 0x7bfcc, false)?;
    c.check(0x7bfb8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7bfbc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bfc0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x1b0)?)?;
    c.check(0x7bfc4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bfc8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bfcc)
}

fn slice_reference_finish(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7bfcc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7bfd0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x33c)?)?;
    c.check(0x7bfd4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7bfd8)?;
    c.branch(C::Eq, 0x7c040, false)?;
    c.check(0x7bfdc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x20)?)?;
    c.check(0x7bfe0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7bfe4)?;
    c.branch(C::Ne, 0x7bffc, false)?;
    c.check(0x7bfe8)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x276)?)?;
    c.check(0x7bfec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7bff0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7bff4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7bff8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x20)?)?;
    c.check(0x7bffc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c000)?;
    c.branch(C::Eq, 0x7c040, false)?;
    c.check(0x7c004)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x276)?)?;
    c.check(0x7c008)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7c00c)?;
    c.branch(C::Eq, 0x7c028, false)?;
    c.check(0x7c010)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c014)?;
    c.branch(C::Ne, 0x7c040, false)?;
    c.check(0x7c018)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27d)?)?;
    c.check(0x7c01c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7c020)?;
    c.branch(C::Hi, 0x7c034, false)?;
    c.check(0x7c024)?;
    c.branch(C::Al, 0x7c040, false)?;
    c.check(0x7c028)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x27e)?)?;
    c.check(0x7c02c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.check(0x7c030)?;
    c.branch(C::Cc, 0x7c040, false)?;
    c.check(0x7c034)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7c038)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c03c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c040)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x20)?)?;
    c.check(0x7c044)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c048)?;
    c.branch(C::Eq, 0x7c060, false)?;
    c.check(0x7c04c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7c050)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x3a4)?)?;
    c.check(0x7c054)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(1), r(0), 0x5)?)?;
    c.check(0x7c058)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7c05c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c060)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7c064)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(10), 0x470)?)?;
    c.check(0x7c068)?;
    c.emit(mov_register(C::Al, r(1), r(0)))?;
    c.check(0x7c06c)?;
    c.emit(transfer_multiple(C::Al, true, r(1), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7c070)?;
    c.emit(transfer_multiple(C::Al, false, r(12), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7c074)?;
    c.emit(transfer_multiple(C::Al, true, r(1), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0x7c078)?;
    c.emit(transfer_multiple(C::Al, false, r(12), r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0x7c07c)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(8), 0x2fa)?)?;
    c.check(0x7c080)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0xc)?)?;
    c.check(0x7c084)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.check(0x7c088)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7c08c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x838)?)?;
    c.check(0x7c090)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x2c)?)?;
    c.check(0x7c094)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7c098)?;
    c.branch(C::Eq, 0x7c0c0, false)?;
    c.check(0x7c09c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x38)?)?;
    c.check(0x7c0a0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7c0a4)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(0), 0x14)?)?;
    c.check(0x7c0a8)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7c0ac)?;
    c.branch(C::Ne, 0x7c0c0, false)?;
    c.check(0x7c0b0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7c0b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c0b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c0bc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c0c0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x14c)?)?;
    c.check(0x7c0c4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c0c8)?;
    c.branch(C::Eq, 0x7c0d8, false)?;
    c.check(0x7c0cc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(11), -0x20)?)?;
    c.check(0x7c0d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c0d4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c0d8)?;
    c.emit(transfer_word(C::Al, true, r(4), r(11), -0x20)?)?;
    c.check(0x7c0dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c0e0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c0e4)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7c0e8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c0ec)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7c0f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7c0f4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x7c0f8)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7c0fc)
}

fn reference_set_entry(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c0fc)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7c100)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.check(0x7c104)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.check(0x7c108)?;
    c.emit(transfer_word(C::Al, true, r(8), r(11), 0x8)?)?;
    c.check(0x7c10c)?;
    c.emit(mov_register(C::Al, r(4), r(3)))?;
    c.check(0x7c110)?;
    c.emit(mov_register(C::Al, r(9), r(1)))?;
    c.check(0x7c114)?;
    c.emit(mov_register(C::Al, r(10), r(0)))?;
    c.check(0x7c118)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7c11c)?;
    c.branch(C::Eq, 0x7c130, false)?;
    c.check(0x7c120)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x9)?)?;
    c.check(0x7c124)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c128)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c12c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c130)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x9)?)?;
    c.check(0x7c134)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c138)?;
    c.branch(C::Eq, 0x7c174, false)?;
    c.check(0x7c13c)
}

fn cold_diagnostic_tail(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c13c)?;
    c.literal(r(1), 0x7c24c)?;
    c.check(0x7c140)?;
    c.literal(r(2), 0x7c250)?;
    c.check(0x7c144)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0x7c148)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x45)?)?;
    c.check(0x7c14c)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0x7c150)?;
    c.literal(r(1), 0x7c254)?;
    c.check(0x7c154)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0x7c158)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0x7c15c)?;
    c.literal(r(2), 0x7c258)?;
    c.check(0x7c160)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7c164)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x46)?)?;
    c.check(0x7c168)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x7c16c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7c170)?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x7c174)
}

fn negative_references(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c174)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c178)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.check(0x7c17c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c180)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x1)?)?;
    c.check(0x7c184)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c188)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c18c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.check(0x7c190)?;
    c.branch(C::Eq, 0x7c1ec, false)?;
    c.check(0x7c194)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(10), 0xb)?)?;
    c.check(0x7c198)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7c19c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0x7c1a0)?;
    c.emit(load_signed_byte(C::Al, r(0), r(7), -0x7)?)?;
    c.check(0x7c1a4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(5), 0x1)?)?;
    c.check(0x7c1a8)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c1ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c1b0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c1b4)?;
    c.emit(load_signed_byte(C::Al, r(5), r(7), -0x7)?)?;
    c.check(0x7c1b8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.check(0x7c1bc)?;
    c.branch(C::Eq, 0x7c1cc, false)?;
    c.check(0x7c1c0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c1c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c1c8)?;
    c.branch(C::Al, 0x7c1d4, false)?;
    c.check(0x7c1cc)?;
    c.emit(load_signed_byte(C::Al, r(1), r(7), 0x0)?)?;
    c.check(0x7c1d0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c1d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c1d8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c1dc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7c1e0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7c1e4)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(6), r(9), Shift::Lsl, 0, false)?)?;
    c.check(0x7c1e8)?;
    c.branch(C::Ne, 0x7c1a0, false)?;
    c.check(0x7c1ec)
}

fn positive_references(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c1ec)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x1)?)?;
    c.check(0x7c1f0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c1f4)?;
    c.branch(C::Eq, 0x7c244, false)?;
    c.check(0x7c1f8)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(10), 0x0)?)?;
    c.check(0x7c1fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7c200)?;
    c.emit(data_register(C::Al, Op::Add, r(7), r(10), r(5), Shift::Lsl, 0, false)?)?;
    c.check(0x7c204)?;
    c.emit(data_register(C::Al, Op::Mvn, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.check(0x7c208)?;
    c.emit(load_signed_byte(C::Al, r(0), r(7), 0x4)?)?;
    c.check(0x7c20c)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c210)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c214)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c218)?;
    c.emit(load_signed_byte(C::Al, r(1), r(7), 0xb)?)?;
    c.check(0x7c21c)?;
    c.emit(load_signed_byte(C::Al, r(6), r(7), 0x4)?)?;
    c.check(0x7c220)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c224)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c228)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c22c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0x0)?)?;
    c.check(0x7c230)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 0x1)?)?;
    c.check(0x7c234)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.check(0x7c238)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c23c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c240)?;
    c.branch(C::Cc, 0x7c200, false)?;
    c.check(0x7c244)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x7c248)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7c24c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    slice_reference_setup(&mut c)?;
    short_term_selection(&mut c)?;
    slice_reference_flags(&mut c)?;
    slice_reference_finish(&mut c)?;
    reference_set_entry(&mut c)?;
    cold_diagnostic_tail(&mut c)?;
    negative_references(&mut c)?;
    positive_references(&mut c)?;
    c.data(0x7c24c, 0xf42b7)?;
    c.data(0x7c250, 0xf42f0)?;
    c.data(0x7c254, 0xe2788)?;
    c.data(0x7c258, 0xf4260)?;
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
    fn structure_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 2048);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(FUNCTION_STARTS, [0x7ba5c, 0x7c0fc]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count() * 4, CODE_BYTES);
        for (at, value) in [(0x7c24c, 0xf42b7), (0x7c250, 0xf42f0), (0x7c254, 0xe2788), (0x7c258, 0xf4260)] {
            assert_eq!(word(&bytes, at), value);
        }
        for (at, rd, target) in [(0x7c13c, 1, 0x7c24c), (0x7c140, 2, 0x7c250), (0x7c150, 1, 0x7c254), (0x7c15c, 2, 0x7c258)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, r(rd), R::PC, target - at as i32 - 8).unwrap());
        }
        assert_eq!(word(&bytes, 0x7c138), branch(C::Eq, 0x7c138, 0x7c174, false).unwrap());
        for at in [0x7c14c, 0x7c158] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, 0x2ec94, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x7c170), branch(C::Al, 0x7c170, 0x17b0, false).unwrap());
        let saved = (4..=11).fold(R::LR.bit(), |mask, n| mask | r(n).bit());
        assert_eq!(word(&bytes, 0x7c16c), transfer_multiple(C::Al, true, R::SP, saved, BlockMode::IncrementAfter, true).unwrap());
        for at in [0x7c0f8, 0x7c248] {
            assert_eq!(word(&bytes, at), transfer_multiple(C::Al, true, R::SP, (saved & !R::LR.bit()) | R::PC.bit(), BlockMode::IncrementAfter, true).unwrap());
        }
        for at in [0x7bb30, 0x7bde8] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, 0x7c0fc, true).unwrap());
        }
    }

    #[test]
    fn exact_code_data_and_target_guards() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), at < 0x7c24c);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = TypedA32::data_value(at) {
                c.data(at, value).unwrap();
            }
        }
        for at in [START - 4, START + 1, 0x7c24d, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, 0xf42b7).is_err());
            assert!(c.bytes.is_empty());
        }
        for (at, rd, target) in [(0x7c13c, 1, 0x7c24c), (0x7c140, 2, 0x7c250), (0x7c150, 1, 0x7c254), (0x7c15c, 2, 0x7c258)] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal(r(0), target).is_err());
            assert!(c.literal(r(rd), target + 4).is_err());
            assert!(c.data(target, TypedA32::data_value(target).unwrap()).is_err());
            assert!(c.check(END).is_err());
            assert!(c.bytes.is_empty());
            c.literal(r(rd), target).unwrap();
        }
        for (at, condition, target, link) in [(0x7c170, C::Al, 0x17b0, false), (0x7c14c, C::Al, 0x2ec94, true), (0x7c138, C::Eq, 0x7c174, false), (0x7bb30, C::Al, 0x7c0fc, true)] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.branch(condition, target, !link).is_err());
            assert!(c.branch(C::Ne, target, link).is_err());
            for invalid in [START + 1, 0x7c24c, END, target + 4] {
                assert!(c.branch(condition, invalid, link).is_err());
            }
            assert!(c.bytes.is_empty());
            c.branch(condition, target, link).unwrap();
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
