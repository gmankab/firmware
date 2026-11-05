use arm32::{
    branch, data_immediate, data_register, transfer_byte, transfer_doubleword,
    transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    transfer_word_register_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x75748;
pub const END: usize = 0x76114;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const POOL_START: usize = 0x760d4;
pub const CODE_BYTES: usize = POOL_START - START;
pub const DATA_BYTES: usize = END - POOL_START;

fn r(number: u8) -> R { R::new(number).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VSPAP process address 0x{:x}, expected 0x{expected:x}", self.at));
        }
        Ok(())
    }

    fn data_value(at: usize) -> Option<u32> {
        match at {
            0x760d4 => Some(0xf3a34),
            0x760d8 => Some(0xf3b2a),
            0x760dc => Some(0xe2788),
            0x760e0 => Some(0xf3b7c),
            0x760e4 => Some(0xf3bb9),
            0x760e8 => Some(0xf3c19),
            0x760ec => Some(0x104098),
            0x760f0 => Some(0xf3c5e),
            0x760f4 => Some(0xf3c9a),
            0x760f8 => Some(0xf3caa),
            0x760fc => Some(0x121002),
            0x76100 => Some(0x2002001),
            0x76104 => Some(0xebffd),
            0x76108 => Some(0xf3abb),
            0x7610c => Some(0xf3afb),
            0x76110 => Some(0xf3a8e),
            _ => None,
        }
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..POOL_START).contains(&self.at) || self.at % 4 != 0 {
            return Err("VSPAP process instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if Self::data_value(at) != Some(value) {
            return Err("VSPAP process protected DATA mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, destination: R, at: usize) -> Result<(), String> {
        if !matches!((self.at, at),
              (0x75810, 0x760d4)
            | (0x75858, 0x7610c)
            | (0x75864, 0x760dc)
            | (0x75900, 0x76108)
            | (0x7590c, 0x760dc)
            | (0x75970, 0x76110)
            | (0x7597c, 0x760dc)
            | (0x75a14, 0x760d8)
            | (0x75a24, 0x760e0)
            | (0x75a30, 0x760dc)
            | (0x75a90, 0x760e4)
            | (0x75ab4, 0x760e8)
            | (0x75ad0, 0x760dc)
            | (0x75adc, 0x760ec)
            | (0x75af4, 0x760f4)
            | (0x75b00, 0x760f0)
            | (0x75b14, 0x760dc)
            | (0x75b34, 0x760ec)
            | (0x75c08, 0x760f4)
            | (0x75c18, 0x760f8)
            | (0x75c2c, 0x760dc)
            | (0x75e10, 0x760fc)
            | (0x75e20, 0x76100)
            | (0x75ed8, 0x760d4)
            | (0x75f04, 0x760d4)
            | (0x75f2c, 0x76104)
            | (0x75f38, 0x760dc)
            | (0x75f44, 0x760d4)
            | (0x76074, 0x760d4)
        ) || Self::data_value(at).is_none() {
            return Err("invalid VSPAP process literal target".into());
        }
        self.emit(transfer_word(C::Al, true, destination, R::PC,
            at as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if !matches!((self.at, condition, target, link),
              (0x75764, C::Ge, 0x75780, false)
            | (0x7576c, C::Eq, 0x75794, false)
            | (0x75774, C::Ne, 0x757a4, false)
            | (0x7577c, C::Al, 0x757b0, false)
            | (0x75780, C::Eq, 0x7579c, false)
            | (0x75788, C::Ne, 0x757a4, false)
            | (0x75790, C::Al, 0x757b0, false)
            | (0x75798, C::Al, 0x757b0, false)
            | (0x757a0, C::Al, 0x757b0, false)
            | (0x757b8, C::Al, 0x3084, true)
            | (0x757c0, C::Al, 0x293c, true)
            | (0x757d0, C::Cs, 0x75838, false)
            | (0x757dc, C::Ne, 0x760cc, false)
            | (0x757f0, C::Ne, 0x75828, false)
            | (0x7580c, C::Al, 0x80928, true)
            | (0x75824, C::Al, 0x17b0, true)
            | (0x75830, C::Al, 0x292c, true)
            | (0x75834, C::Al, 0x760cc, false)
            | (0x7583c, C::Ne, 0x760cc, false)
            | (0x75854, C::Al, 0x7587c, false)
            | (0x75860, C::Al, 0x2ec94, true)
            | (0x7586c, C::Al, 0x2ec94, true)
            | (0x75878, C::Al, 0x7598c, false)
            | (0x75884, C::Eq, 0x7591c, false)
            | (0x7588c, C::Ne, 0x760cc, false)
            | (0x75898, C::Al, 0x35d44, true)
            | (0x758a4, C::Eq, 0x75970, false)
            | (0x758b4, C::Eq, 0x7598c, false)
            | (0x758c4, C::Ne, 0x7598c, false)
            | (0x758d0, C::Ne, 0x7598c, false)
            | (0x758e0, C::Al, 0x7f990, true)
            | (0x758ec, C::Eq, 0x75858, false)
            | (0x758f4, C::Al, 0x8025c, true)
            | (0x75908, C::Al, 0x2ec94, true)
            | (0x75914, C::Al, 0x2ec94, true)
            | (0x75918, C::Al, 0x7598c, false)
            | (0x75944, C::Al, 0x31ec, true)
            | (0x75954, C::Eq, 0x759b4, false)
            | (0x75960, C::Eq, 0x759a8, false)
            | (0x7596c, C::Al, 0x759b4, false)
            | (0x75978, C::Al, 0x2ec94, true)
            | (0x75984, C::Al, 0x2ec94, true)
            | (0x759a4, C::Al, 0x760c4, false)
            | (0x759e0, C::Eq, 0x75a6c, false)
            | (0x759fc, C::Al, 0x28e9c, true)
            | (0x75a08, C::Eq, 0x75a1c, false)
            | (0x75a18, C::Al, 0x75a28, false)
            | (0x75a2c, C::Al, 0x2ec94, true)
            | (0x75a38, C::Al, 0x2ec94, true)
            | (0x75a44, C::Al, 0x293c, true)
            | (0x75a54, C::Al, 0x50c70, true)
            | (0x75a58, C::Al, 0x26754, true)
            | (0x75a68, C::Al, 0x294c, true)
            | (0x75a78, C::Eq, 0x75aa4, false)
            | (0x75a8c, C::Al, 0x26754, true)
            | (0x75aa0, C::Al, 0x75ac0, false)
            | (0x75ab0, C::Al, 0x26754, true)
            | (0x75acc, C::Al, 0x2ec94, true)
            | (0x75ad8, C::Al, 0x2ec94, true)
            | (0x75ae8, C::Eq, 0x75b20, false)
            | (0x75b10, C::Al, 0x2ec94, true)
            | (0x75b1c, C::Al, 0x2ec94, true)
            | (0x75b30, C::Eq, 0x75c44, false)
            | (0x75b44, C::Ne, 0x75bcc, false)
            | (0x75b58, C::Al, 0x51bc4, true)
            | (0x75b64, C::Ne, 0x75b8c, false)
            | (0x75b80, C::Ls, 0x75bb0, false)
            | (0x75b88, C::Al, 0x75bcc, false)
            | (0x75bac, C::Al, 0x75bcc, false)
            | (0x75be4, C::Al, 0x28ff0, true)
            | (0x75bf0, C::Eq, 0x75c44, false)
            | (0x75bfc, C::Eq, 0x75c3c, false)
            | (0x75c28, C::Al, 0x2ec94, true)
            | (0x75c34, C::Al, 0x2ec94, true)
            | (0x75c38, C::Al, 0x75c44, false)
            | (0x75c4c, C::Ne, 0x75dec, false)
            | (0x75c58, C::Al, 0x35c14, true)
            | (0x75c64, C::Eq, 0x75ed8, false)
            | (0x75c74, C::Al, 0x471c, true)
            | (0x75cfc, C::Ne, 0x75d0c, false)
            | (0x75d08, C::Eq, 0x75d90, false)
            | (0x75d20, C::Al, 0x26754, true)
            | (0x75d28, C::Eq, 0x75d68, false)
            | (0x75d64, C::Cc, 0x75d34, false)
            | (0x75da0, C::Eq, 0x75f60, false)
            | (0x75dac, C::Ne, 0x75dd0, false)
            | (0x75dcc, C::Al, 0x471c, true)
            | (0x75dd8, C::Al, 0x35ca0, true)
            | (0x75de4, C::Al, 0x292c, true)
            | (0x75de8, C::Al, 0x75f60, false)
            | (0x75df4, C::Eq, 0x75f60, false)
            | (0x75e8c, C::Eq, 0x75eec, false)
            | (0x75e94, C::Eq, 0x75ef8, false)
            | (0x75e9c, C::Ne, 0x75f04, false)
            | (0x75eb0, C::Eq, 0x75f14, false)
            | (0x75ec8, C::Ne, 0x75f14, false)
            | (0x75ed4, C::Al, 0x75f14, false)
            | (0x75ee4, C::Al, 0x17b0, true)
            | (0x75ee8, C::Al, 0x75f60, false)
            | (0x75ef4, C::Al, 0x75f14, false)
            | (0x75f00, C::Al, 0x75f14, false)
            | (0x75f10, C::Al, 0x17b0, true)
            | (0x75f20, C::Al, 0x80928, true)
            | (0x75f28, C::Eq, 0x75f58, false)
            | (0x75f34, C::Al, 0x2ec94, true)
            | (0x75f40, C::Al, 0x2ec94, true)
            | (0x75f54, C::Al, 0x17b0, true)
            | (0x75f68, C::Eq, 0x75f80, false)
            | (0x75f7c, C::Ne, 0x76088, false)
            | (0x75f8c, C::Al, 0x35c14, true)
            | (0x75f94, C::Eq, 0x7606c, false)
            | (0x7602c, C::Al, 0x35ca0, true)
            | (0x7603c, C::Ne, 0x7607c, false)
            | (0x76058, C::Al, 0x80928, true)
            | (0x76068, C::Al, 0x76074, false)
            | (0x76078, C::Al, 0x17b0, true)
            | (0x76084, C::Al, 0x292c, true)
            | (0x76090, C::Al, 0x35df4, true)
            | (0x7609c, C::Al, 0x292c, true)
            | (0x760a8, C::Al, 0x292c, true)
            | (0x760c8, C::Eq, 0x7587c, false)
        ) {
            return Err("invalid VSPAP process branch edge".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn entry_and_codec(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75748)?;
    c.emit(transfer_multiple(C::Al, false, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x254)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 644)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2000)?)?;
    c.branch(C::Ge, 0x75780, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x2)?)?;
    c.branch(C::Eq, 0x75794, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1000)?)?;
    c.branch(C::Ne, 0x757a4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xd)?)?;
    c.branch(C::Al, 0x757b0, false)?;
    c.branch(C::Eq, 0x7579c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4000)?)?;
    c.branch(C::Ne, 0x757a4, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xe)?)?;
    c.branch(C::Al, 0x757b0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xb)?)?;
    c.branch(C::Al, 0x757b0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xc)?)?;
    c.branch(C::Al, 0x757b0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x8000)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0xc)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(4), 16)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x3084, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 4)?)?;
    c.branch(C::Al, 0x293c, true)?;
    Ok(())
}

fn initial_state(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x757c4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x6)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x5)?)?;
    c.branch(C::Cs, 0x75838, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 13)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x760cc, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 13)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x75828, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 276)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 80)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x100)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x50)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 84)?)?;
    c.branch(C::Al, 0x80928, true)?;
    c.literal(r(2), 0x760d4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x21)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 20)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.branch(C::Al, 0x760cc, false)?;
    Ok(())
}

fn task_acquisition(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75838)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x5)?)?;
    c.branch(C::Ne, 0x760cc, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(4), 0x280)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(13), 0x24)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.branch(C::Al, 0x7587c, false)?;
    c.literal(r(1), 0x7610c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.branch(C::Al, 0x7598c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x7591c, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x760cc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x40)?)?;
    c.branch(C::Al, 0x35d44, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(C::Eq, 0x75970, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7598c, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(0), r(2), 356)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x7598c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 184)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x7598c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 124)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(5), 0x168)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x50)?)?;
    c.branch(C::Al, 0x7f990, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 80)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75858, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(5), 0x30)?)?;
    c.branch(C::Al, 0x8025c, true)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 356)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(5), 400)?)?;
    c.literal(r(1), 0x76108)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x7598c, false)?;
    Ok(())
}

fn active_task_setup(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7591c)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2c)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(5), 12)?)?;
    c.emit(transfer_word(C::Al, true, r(10), r(5), 52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 28)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 60)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 24)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x759b4, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x759a8, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 2)?)?;
    c.branch(C::Al, 0x759b4, false)?;
    c.literal(r(1), 0x76110)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 52)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.branch(C::Al, 0x760c4, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 56)?)?;
    Ok(())
}

fn submit_task(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x759b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 403)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x31)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(10), 368)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(10), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(5), r(0), 0x11)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 36)?)?;
    c.emit(transfer_word(C::Al, false, r(7), r(13), 40)?)?;
    c.branch(C::Eq, 0x75a6c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 28)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x28e9c, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75a1c, false)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 400)?)?;
    c.literal(r(1), 0x760d8)?;
    c.branch(C::Al, 0x75a28, false)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(6), 4)?)?;
    c.literal(r(1), 0x760e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.branch(C::Al, 0x293c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 8)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.branch(C::Al, 0x50c70, true)?;
    c.branch(C::Al, 0x26754, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.branch(C::Al, 0x294c, true)?;
    Ok(())
}

fn timing_and_diagnostics(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75a6c)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 20)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75aa4, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(8), r(10), 2)?)?;
    c.emit(transfer_word(C::Al, true, r(9), r(10), 400)?)?;
    c.branch(C::Al, 0x26754, true)?;
    c.literal(r(1), 0x760e4)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(9), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x75ac0, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(10), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(6), r(6), 4)?)?;
    c.branch(C::Al, 0x26754, true)?;
    c.literal(r(1), 0x760e8)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x20)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(0), 0x760ec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75b20, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 16)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.literal(r(2), 0x760f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xba)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.literal(r(1), 0x760f0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x100)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    Ok(())
}

fn capacity_and_backpressure(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75b20)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 2)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 16)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(6), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x75c44, false)?;
    c.literal(r(0), 0x760ec)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.branch(C::Ne, 0x75bcc, false)?;
    c.emit(transfer_word(C::Al, true, r(5), r(6), 952)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.branch(C::Al, 0x51bc4, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x75b8c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 52)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1048)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsr, 3, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ls, 0x75bb0, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x75bcc, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 588)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 592)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x75bcc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 952)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 56)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsr, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Hi, Op::Mov, r(0), r(0), 0x1)?)?;
    Ok(())
}

fn backpressure_notification(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75bcc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 24)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(13), 28)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 36)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.branch(C::Al, 0x28ff0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 40)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75c44, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 5)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75c3c, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 32)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.literal(r(2), 0x760f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xd9)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x100)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 56)?)?;
    c.literal(r(1), 0x760f8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.branch(C::Al, 0x75c44, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 40)?)?;
    Ok(())
}

fn output_descriptor(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75c44)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Ne, 0x75dec, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 56)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.branch(C::Al, 0x35c14, true)?;
    c.emit(data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.branch(C::Eq, 0x75ed8, false)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4c)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 356)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 104)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 108)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 360)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 112)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 400)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 256)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 84)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 88)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 92)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 52)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 96)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 56)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 100)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 80)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 20)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 404)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 264)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 376)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 380)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 272)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(5), 276)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Eq, false, r(0), r(5), 88)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 1016)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x75d0c, false)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 1005)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75d90, false)?;
    Ok(())
}

fn bounded_offset_records(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75d0c)?;
    c.emit(transfer_word(C::Al, true, r(6), r(13), 60)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(4), 1948)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x10)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x10)?)?;
    c.emit(data_register(C::Cs, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x26754, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x75d68, false)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x78)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(10), 396)?)?;
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, r(3), r(2), r(1), Shift::Lsl, 3)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(3), r(3), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(2), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(3), r(10), 396)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 64)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x4)?)?;
    c.branch(C::Cc, 0x75d34, false)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 248)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 392)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 120)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 120)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 392)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 184)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 184)?)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 12)?)?;
    Ok(())
}

fn descriptor_queue(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75d90)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(10), 184)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75f60, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 644)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.branch(C::Ne, 0x75dd0, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 116)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(1), 0x1)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 96)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 116)?)?;
    c.branch(C::Al, 0x471c, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 56)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.branch(C::Al, 0x35ca0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 56)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.branch(C::Al, 0x75f60, false)?;
    Ok(())
}

fn message_pack(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75dec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 356)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75f60, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 384)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 84)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4c)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 80)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 88)?)?;
    c.literal(r(1), 0x760fc)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 92)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 96)?)?;
    c.literal(r(1), 0x76100)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 36)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(data_register(C::Ne, Op::Mov, r(1), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 100)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 376)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(10), 380)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 120)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 124)?)?;
    c.emit(transfer_doubleword(C::Al, true, r(2), r(10), 56)?)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 128)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(13), 132)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 68)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 136)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 388)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 140)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 144)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 20)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 152)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 44)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 108)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 112)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 148)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Eq, 0x75eec, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x75ef8, false)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Ne, 0x75f04, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 104)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 7)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75f14, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2000)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x75f14, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x9)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 104)?)?;
    c.branch(C::Al, 0x75f14, false)?;
    Ok(())
}

fn cold_message_paths(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75ed8)?;
    c.literal(r(2), 0x760d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa5)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.branch(C::Al, 0x75f60, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 104)?)?;
    c.branch(C::Al, 0x75f14, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 104)?)?;
    c.branch(C::Al, 0x75f14, false)?;
    c.literal(r(2), 0x760d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    Ok(())
}

fn message_dispatch(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75f14)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 56)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x50)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 12)?)?;
    c.branch(C::Al, 0x80928, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.branch(C::Eq, 0x75f58, false)?;
    c.literal(r(1), 0x76104)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(1), 0x760dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(r(2), 0x760d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x22)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x100)?)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(10), 68)?)?;
    Ok(())
}

fn completion_gate(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75f60)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(4), 848)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x75f80, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 3348)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 400)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(1), 0x1)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.branch(C::Ne, 0x76088, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 52)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(0), 0x10)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35c14, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.branch(C::Eq, 0x7606c, false)?;
    Ok(())
}

fn completion_record(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x75f98)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_multiple(C::Al, true, r(2), r(3).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, false, r(1), r(3).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(transfer_multiple(C::Al, true, r(2), r(3).bit() | r(4).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_multiple(C::Al, false, r(1), r(3).bit() | r(4).bit() | r(6).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 36)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 2)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 40)?)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(0), 3)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 52)?)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 28)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(13), 56)?)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 32)?)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_byte(C::Al, true, r(3), r(10), 408)?)?;
    c.emit(transfer_byte(C::Al, false, r(3), r(0), 64)?)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(3), r(2), 0x7f)?)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(data_immediate(C::Cc, Op::Add, r(2), r(2), 0x80)?)?;
    c.emit(data_immediate(C::Cc, Op::Bic, r(2), r(2), 0x7f)?)?;
    c.emit(data_register(C::Cc, Op::Sub, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Cc, false, r(1), r(0), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 44)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 36)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 64)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 40)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 68)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 72)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 76)?)?;
    c.emit(transfer_word(C::Al, false, r(1), r(0), 52)?)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.branch(C::Al, 0x35ca0, true)?;
    Ok(())
}

fn completion_notification(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x76030)?;
    c.emit(transfer_word(C::Al, true, r(4), r(13), 12)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 280)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x2)?)?;
    c.branch(C::Ne, 0x7607c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 16)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 276)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 80)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(1), 0x100)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(13), 0x50)?)?;
    c.emit(transfer_word(C::Al, false, r(4), r(13), 84)?)?;
    c.branch(C::Al, 0x80928, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x96)?)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.branch(C::Al, 0x76074, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x6c)?)?;
    c.literal(r(2), 0x760d4)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 44)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 20)?)?;
    c.branch(C::Al, 0x292c, true)?;
    Ok(())
}

fn release_and_loop(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x76088)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 48)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x40)?)?;
    c.branch(C::Al, 0x35df4, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 48)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 32)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.branch(C::Al, 0x292c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 0)?)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(10), 2)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(6), r(0), 0x1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.branch(C::Eq, 0x7587c, false)?;
    Ok(())
}

fn epilogue(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x760cc)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(transfer_multiple(C::Al, true, r(13), r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    entry_and_codec(&mut c)?;
    initial_state(&mut c)?;
    task_acquisition(&mut c)?;
    active_task_setup(&mut c)?;
    submit_task(&mut c)?;
    timing_and_diagnostics(&mut c)?;
    capacity_and_backpressure(&mut c)?;
    backpressure_notification(&mut c)?;
    output_descriptor(&mut c)?;
    bounded_offset_records(&mut c)?;
    descriptor_queue(&mut c)?;
    message_pack(&mut c)?;
    cold_message_paths(&mut c)?;
    message_dispatch(&mut c)?;
    completion_gate(&mut c)?;
    completion_record(&mut c)?;
    completion_notification(&mut c)?;
    release_and_loop(&mut c)?;
    epilogue(&mut c)?;
    c.check(POOL_START)?;
    c.data(0x760d4, 0xf3a34)?;
    c.data(0x760d8, 0xf3b2a)?;
    c.data(0x760dc, 0xe2788)?;
    c.data(0x760e0, 0xf3b7c)?;
    c.data(0x760e4, 0xf3bb9)?;
    c.data(0x760e8, 0xf3c19)?;
    c.data(0x760ec, 0x104098)?;
    c.data(0x760f0, 0xf3c5e)?;
    c.data(0x760f4, 0xf3c9a)?;
    c.data(0x760f8, 0xf3caa)?;
    c.data(0x760fc, 0x121002)?;
    c.data(0x76100, 0x2002001)?;
    c.data(0x76104, 0xebffd)?;
    c.data(0x76108, 0xf3abb)?;
    c.data(0x7610c, 0xf3afb)?;
    c.data(0x76110, 0xf3a8e)?;
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
    fn structure_and_cold_code() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2508);
        assert_eq!(CODE_BYTES, 2444);
        assert_eq!(DATA_BYTES, 64);
        for at in (POOL_START..END).step_by(4) {
            assert_eq!(Some(word(&bytes, at)), TypedA32::data_value(at));
        }
        for (at, condition, target, link) in [
            (0x75854, C::Al, 0x7587c, false),
            (0x75860, C::Al, 0x2ec94, true),
            (0x758ec, C::Eq, 0x75858, false),
            (0x75c64, C::Eq, 0x75ed8, false),
            (0x75ee4, C::Al, 0x17b0, true),
            (0x75f10, C::Al, 0x17b0, true),
            (0x760c8, C::Eq, 0x7587c, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        let saved = (4..=11).fold(R::PC.bit(), |mask, n| mask | r(n).bit());
        assert_eq!(word(&bytes, 0x760d0), transfer_multiple(C::Al, true,
            R::SP, saved, BlockMode::IncrementAfter, true).unwrap());
    }

    #[test]
    fn indexed_writeback_resets_base_and_unsigned_count() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x75d18), data_immediate(C::Al, Op::Cmp, r(0), r(6), 16).unwrap());
        assert_eq!(word(&bytes, 0x75d1c), data_register(C::Cs, Op::Mov,
            r(6), r(0), r(0), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x75d34), data_register(C::Al, Op::Mov,
            r(2), r(0), r(7), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x75d3c), transfer_word_register_preindexed_writeback(
            C::Al, true, r(3), r(2), r(1), Shift::Lsl, 3).unwrap());
        assert_eq!(word(&bytes, 0x75d50), transfer_word(C::Al, true, r(2), r(2), 4).unwrap());
        assert_eq!(word(&bytes, 0x75d64), branch(C::Cc, 0x75d64, 0x75d34, false).unwrap());
        for count in [0u32, 1, 15, 16, 17, 0x8000_0000, u32::MAX] {
            let bounded = if count >= 16 { 16 } else { count };
            assert_eq!(bounded, count.min(16));
            let base = 0xffff_fff8u32;
            for i in 0..bounded {
                let mut r2 = base;
                r2 = r2.wrapping_add(i << 3);
                assert_eq!(r2, base.wrapping_add(8 * i));
                assert_eq!(r2.wrapping_add(4), base.wrapping_add(8 * i + 4));
            }
        }
    }

    #[test]
    fn conditional_alignment_wraps_in_32_bits() {
        let bytes = fragment().unwrap();
        for (at, op, dst, src, value) in [
            (0x75fec, Op::Add, r(2), r(2), 128),
            (0x75ff0, Op::Bic, r(2), r(2), 127),
        ] {
            assert_eq!(word(&bytes, at), data_immediate(C::Cc, op, dst, src, value).unwrap());
        }
        assert_eq!(word(&bytes, 0x75ff4), data_register(C::Cc, Op::Sub,
            r(1), r(2), r(1), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x75ff8), transfer_word(C::Cc, false, r(1), r(0), 32).unwrap());
        for (prefix, length, expected) in [
            (0u32, 0u32, 0u32), (0, 1, 128), (1, 127, 127),
            (1, 128, 255), (0, 0xffff_ffff, 0),
            (0xffff_ffff, 2, 129), (0xffff_ff80, 128, 128),
        ] {
            let sum = prefix.wrapping_add(length);
            let result = if (sum & !127) < sum {
                (sum.wrapping_add(128) & !127).wrapping_sub(prefix)
            } else { length };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn exact_guards_reject_without_emission() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), at < POOL_START);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            if let Some(value) = TypedA32::data_value(at) {
                assert!(c.data(at, value ^ 1).is_err());
                assert!(c.bytes.is_empty());
                c.data(at, value).unwrap();
                assert_eq!(c.bytes, value.to_le_bytes());
            }
            let mut c = TypedA32 { at, bytes: Vec::new() };
            for target in [START - 4, START + 1, POOL_START, END, 0x17b1] {
                assert!(c.branch(C::Al, target, false).is_err());
                assert!(c.branch(C::Al, target, true).is_err());
            }
            for target in [START, POOL_START - 4, POOL_START + 1, END] {
                assert!(c.literal(r(0), target).is_err());
            }
            assert!(c.bytes.is_empty());
        }
        for at in [START - 4, START + 1, POOL_START + 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(0).is_err());
            assert!(c.data(at, 0xf3a34).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
        assert!(c.data(POOL_START, 0xf3a34).is_err());
        assert!(c.literal(r(2), 0x760d4).is_err());
        assert!(c.branch(C::Al, 0x3084, true).is_err());
        assert!(c.bytes.is_empty());
        let mut c = TypedA32 { at: 0x757b8, bytes: Vec::new() };
        assert!(c.branch(C::Ne, 0x3084, true).is_err());
        assert!(c.branch(C::Al, 0x3084, false).is_err());
        c.branch(C::Al, 0x3084, true).unwrap();
    }

    #[test]
    fn authenticated_same_buffer_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let actual = fragment().unwrap();
        let expected = &reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA];
        for (index, (actual, expected)) in actual.chunks_exact(4).zip(expected.chunks_exact(4)).enumerate() {
            assert_eq!(actual, expected, "runtime 0x{:x}", START + index * 4);
        }
        assert_eq!(actual, expected);
    }
}
