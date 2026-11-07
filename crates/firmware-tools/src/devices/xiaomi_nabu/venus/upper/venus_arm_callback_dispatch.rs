use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    branch_exchange_with_link, bx, data_immediate, data_immediate_with_flags,
    data_register, data_register_shifted_by_register, store_halfword, transfer_byte,
    transfer_byte_postindexed, transfer_multiple, transfer_word, transfer_word_postindexed,
    transfer_word_register,
};

pub const START: usize = 0x85de0;
pub const END: usize = 0x8634c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1344;
pub const DATA_BYTES: usize = 44;

const LITERALS: [(usize, u32); 11] = [
    (0x85f54, 0x10cfbc),
    (0x85f58, 0x10b565),
    (0x85fe8, 0x10b565),
    (0x85fec, 0x10cfbc),
    (0x85ffc, 0x10cfbc),
    (0x8600c, 0x10cfbc),
    (0x86064, 0x200014e),
    (0x86068, 0x10cff0),
    (0x86078, 0x10cff0),
    (0x86264, 0x10cff0),
    (0x86268, 0xf4e0c),
];

const LITERAL_REFERENCES: [(usize, usize); 11] = [
    (0x85e6c, 0x85f54),
    (0x85f44, 0x85f58),
    (0x85f5c, 0x85fe8),
    (0x85f74, 0x85fec),
    (0x85ff0, 0x85ffc),
    (0x86000, 0x8600c),
    (0x86040, 0x86064),
    (0x86044, 0x86068),
    (0x8606c, 0x86078),
    (0x8608c, 0x86264),
    (0x860f8, 0x86268),
];

const FUNCTIONS: [(usize, fn(&mut Code) -> Result<(), String>); 14] = [
    (0x85de0, function_85de0),
    (0x85e54, function_85e54),
    (0x85e64, function_85e64),
    (0x85f5c, function_85f5c),
    (0x85ff0, function_85ff0),
    (0x86000, function_86000),
    (0x86010, function_86010),
    (0x86038, function_86038),
    (0x8606c, function_8606c),
    (0x8607c, function_8607c),
    (0x8626c, function_8626c),
    (0x86280, function_86280),
    (0x862c4, function_862c4),
    (0x862d4, function_862d4),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
    code_bytes: usize,
    data_bytes: usize,
}

impl Code {
    fn new(at: usize) -> Self {
        Self { at, bytes: Vec::new(), code_bytes: 0, data_bytes: 0 }
    }

    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("runtime address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, at: usize, value: u32, data: bool) -> Result<(), String> {
        self.check(at)?;
        let literal = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, v)| v);
        if !(START..END).contains(&at) || at % 4 != 0
            || if data { literal != Some(value) } else { literal.is_some() }
        {
            return Err(format!("invalid runtime {} at {at:#x}", if data { "data" } else { "code" }));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        if data { self.data_bytes += 4; } else { self.code_bytes += 4; }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.word(at, instruction, false)
    }

    fn literal(&mut self, at: usize, rd: u8, slot: usize) -> Result<(), String> {
        if !LITERAL_REFERENCES.contains(&(at, slot))
            || !LITERALS.iter().any(|&(address, _)| address == slot)
        {
            return Err(format!("invalid runtime literal reference {at:#x} -> {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, R::new(rd)?, R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn function_85de0(c: &mut Code) -> Result<(), String> {
    c.emit(0x85de0, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85de4, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(0x85de8, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x50)?)?;
    c.emit(0x85dec, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0xf)?)?;
    c.emit(0x85df0, transfer_word(C::Al, false, r(1), r(13), 16)?)?;
    c.emit(0x85df4, data_immediate(C::Cs, Op::Mov, r(2), r(0), 0xf)?)?;
    c.emit(0x85df8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x85dfc, branch(C::Eq, 0x85dfc, 0x85e20, false)?)?;
    c.emit(0x85e00, data_immediate(C::Al, Op::Add, r(1), r(13), 0x10)?)?;
    c.emit(0x85e04, data_immediate(C::Al, Op::Mov, r(14), r(0), 0x0)?)?;
    c.emit(0x85e08, data_immediate(C::Al, Op::Add, r(12), r(1), 0x4)?)?;
    c.emit(0x85e0c, transfer_word_postindexed(C::Al, true, r(1), r(3), 4)?)?;
    c.emit(0x85e10, data_immediate(C::Al, Op::Add, r(14), r(14), 0x1)?)?;
    c.emit(0x85e14, data_register(C::Al, Op::Cmp, r(0), r(14), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85e18, transfer_word_postindexed(C::Al, false, r(1), r(12), 4)?)?;
    c.emit(0x85e1c, branch(C::Cc, 0x85e1c, 0x85e0c, false)?)?;
    c.emit(0x85e20, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x4)?)?;
    c.emit(0x85e24, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x85e28, data_immediate(C::Al, Op::Add, r(12), r(13), 0x10)?)?;
    c.emit(0x85e2c, data_register(C::Al, Op::Add, r(2), r(3), r(2), Shift::Lsl, 2, false)?)?;
    c.emit(0x85e30, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.emit(0x85e34, transfer_multiple(C::Al, false, r(13), 0x1004, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x85e38, transfer_word(C::Al, false, r(1), r(13), 8)?)?;
    c.emit(0x85e3c, transfer_word(C::Al, false, r(1), r(13), 12)?)?;
    c.emit(0x85e40, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x85e44, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.emit(0x85e48, branch(C::Al, 0x85e48, 0x85ba0, true)?)?;
    c.emit(0x85e4c, data_register(C::Al, Op::Mov, r(13), r(0), r(11), Shift::Lsl, 0, false)?)?;
    c.emit(0x85e50, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_85e54(c: &mut Code) -> Result<(), String> {
    c.emit(0x85e54, transfer_multiple(C::Al, false, r(13), 0x40f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85e58, transfer_multiple(C::Al, true, r(1), 0xf0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x85e5c, transfer_multiple(C::Al, false, r(0), 0xf0, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x85e60, transfer_multiple(C::Al, true, r(13), 0x80f0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_85e64(c: &mut Code) -> Result<(), String> {
    c.emit(0x85e64, transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85e68, data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.literal(0x85e6c, 5, 0x85f54)?;
    c.emit(0x85e70, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x85e74, transfer_word(C::Al, false, r(4), r(5), 0)?)?;
    c.emit(0x85e78, transfer_word(C::Al, false, r(4), r(5), 4)?)?;
    c.emit(0x85e7c, transfer_word(C::Al, false, r(4), r(5), 8)?)?;
    c.emit(0x85e80, branch(C::Al, 0x85e80, 0x8606c, true)?)?;
    c.emit(0x85e84, transfer_word(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(0x85e88, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x85e8c, transfer_word(C::Al, true, r(3), r(1), 36)?)?;
    c.emit(0x85e90, data_immediate(C::Al, Op::Add, r(1), r(5), 0xc)?)?;
    c.emit(0x85e94, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x85e98, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x85e9c, branch(C::Ne, 0x85e9c, 0x85f44, false)?)?;
    c.emit(0x85ea0, branch(C::Al, 0x85ea0, 0x8606c, true)?)?;
    c.emit(0x85ea4, transfer_word(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(0x85ea8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x85eac, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x85eb0, transfer_word(C::Al, true, r(3), r(1), 36)?)?;
    c.emit(0x85eb4, data_immediate(C::Al, Op::Add, r(1), r(5), 0x10)?)?;
    c.emit(0x85eb8, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x85ebc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x85ec0, branch(C::Eq, 0x85ec0, 0x85edc, false)?)?;
    c.emit(0x85ec4, branch(C::Al, 0x85ec4, 0x8606c, true)?)?;
    c.emit(0x85ec8, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x85ecc, transfer_word(C::Al, true, r(1), r(5), 12)?)?;
    c.emit(0x85ed0, transfer_word(C::Al, true, r(2), r(2), 40)?)?;
    c.emit(0x85ed4, branch_exchange_with_link(C::Al, r(2))?)?;
    c.emit(0x85ed8, branch(C::Al, 0x85ed8, 0x85f44, false)?)?;
    c.emit(0x85edc, data_immediate(C::Al, Op::Add, r(4), r(5), 0x14)?)?;
    c.emit(0x85ee0, data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.emit(0x85ee4, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(0x85ee8, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x85eec, branch(C::Al, 0x85eec, 0x8606c, true)?)?;
    c.emit(0x85ef0, transfer_word(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(0x85ef4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x85ef8, transfer_word(C::Al, true, r(3), r(1), 36)?)?;
    c.emit(0x85efc, data_immediate(C::Al, Op::Add, r(1), r(4), 0x4)?)?;
    c.emit(0x85f00, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x85f04, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x85f08, branch(C::Eq, 0x85f08, 0x85f20, false)?)?;
    c.emit(0x85f0c, transfer_word(C::Al, false, r(7), r(4), 0)?)?;
    c.emit(0x85f10, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.emit(0x85f14, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x85f18, transfer_word(C::Al, false, r(0), r(5), 4)?)?;
    c.emit(0x85f1c, branch(C::Al, 0x85f1c, 0x85f24, false)?)?;
    c.emit(0x85f20, transfer_word(C::Al, false, r(8), r(4), 0)?)?;
    c.emit(0x85f24, data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(0x85f28, data_immediate(C::Al, Op::Add, r(4), r(4), 0x8)?)?;
    c.emit(0x85f2c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x4)?)?;
    c.emit(0x85f30, branch(C::Ne, 0x85f30, 0x85eec, false)?)?;
    c.emit(0x85f34, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.emit(0x85f38, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x85f3c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4)?)?;
    c.emit(0x85f40, data_immediate(C::Lt, Op::Mov, r(4), r(0), 0x1)?)?;
    c.literal(0x85f44, 0, 0x85f58)?;
    c.emit(0x85f48, transfer_byte(C::Al, false, r(4), r(0), 0)?)?;
    c.emit(0x85f4c, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85f50, transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x85f54, 0x10cfbc, true)?;
    c.word(0x85f58, 0x10b565, true)?;
    Ok(())
}

fn function_85f5c(c: &mut Code) -> Result<(), String> {
    c.literal(0x85f5c, 0, 0x85fe8)?;
    c.emit(0x85f60, transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x85f64, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.emit(0x85f68, bx(C::Ne, r(14)))?;
    c.emit(0x85f6c, transfer_multiple(C::Al, false, r(13), 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85f70, data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.literal(0x85f74, 4, 0x85fec)?;
    c.emit(0x85f78, data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(0x85f7c, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.emit(0x85f80, transfer_byte(C::Al, false, r(5), r(0), 0)?)?;
    c.emit(0x85f84, data_immediate(C::Al, Op::Add, r(6), r(4), 0x18)?)?;
    c.emit(0x85f88, transfer_word(C::Al, false, r(5), r(6), -4)?)?;
    c.emit(0x85f8c, transfer_word(C::Al, true, r(0), r(6), 0)?)?;
    c.emit(0x85f90, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x85f94, branch(C::Eq, 0x85f94, 0x85fac, false)?)?;
    c.emit(0x85f98, branch(C::Al, 0x85f98, 0x8606c, true)?)?;
    c.emit(0x85f9c, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x85fa0, transfer_word(C::Al, true, r(1), r(6), 0)?)?;
    c.emit(0x85fa4, transfer_word(C::Al, true, r(2), r(2), 40)?)?;
    c.emit(0x85fa8, branch_exchange_with_link(C::Al, r(2))?)?;
    c.emit(0x85fac, data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.emit(0x85fb0, data_immediate(C::Al, Op::Add, r(6), r(6), 0x8)?)?;
    c.emit(0x85fb4, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x4)?)?;
    c.emit(0x85fb8, branch(C::Ne, 0x85fb8, 0x85f88, false)?)?;
    c.emit(0x85fbc, branch(C::Al, 0x85fbc, 0x8606c, true)?)?;
    c.emit(0x85fc0, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x85fc4, transfer_word(C::Al, true, r(1), r(4), 12)?)?;
    c.emit(0x85fc8, transfer_word(C::Al, true, r(2), r(2), 40)?)?;
    c.emit(0x85fcc, branch_exchange_with_link(C::Al, r(2))?)?;
    c.emit(0x85fd0, branch(C::Al, 0x85fd0, 0x8606c, true)?)?;
    c.emit(0x85fd4, transfer_word(C::Al, true, r(1), r(0), 4)?)?;
    c.emit(0x85fd8, transfer_word(C::Al, true, r(2), r(1), 40)?)?;
    c.emit(0x85fdc, transfer_word(C::Al, true, r(1), r(4), 16)?)?;
    c.emit(0x85fe0, transfer_multiple(C::Al, true, r(13), 0x48f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x85fe4, bx(C::Al, r(2)))?;
    c.word(0x85fe8, 0x10b565, true)?;
    c.word(0x85fec, 0x10cfbc, true)?;
    Ok(())
}

fn function_85ff0(c: &mut Code) -> Result<(), String> {
    c.literal(0x85ff0, 0, 0x85ffc)?;
    c.emit(0x85ff4, transfer_word(C::Al, true, r(0), r(0), 16)?)?;
    c.emit(0x85ff8, bx(C::Al, r(14)))?;
    c.word(0x85ffc, 0x10cfbc, true)?;
    Ok(())
}

fn function_86000(c: &mut Code) -> Result<(), String> {
    c.literal(0x86000, 0, 0x8600c)?;
    c.emit(0x86004, transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.emit(0x86008, bx(C::Al, r(14)))?;
    c.word(0x8600c, 0x10cfbc, true)?;
    Ok(())
}

fn function_86010(c: &mut Code) -> Result<(), String> {
    c.emit(0x86010, data_immediate(C::Al, Op::Add, r(2), r(0), 0xc)?)?;
    c.emit(0x86014, data_immediate(C::Al, Op::Bic, r(3), r(1), 0xf)?)?;
    c.emit(0x86018, data_immediate_with_flags(C::Al, Op::And, r(1), r(1), 0xf, true)?)?;
    c.emit(0x8601c, data_immediate(C::Al, Op::Bic, r(2), r(2), 0xf)?)?;
    c.emit(0x86020, transfer_word(C::Al, false, r(2), r(0), 48)?)?;
    c.emit(0x86024, transfer_word(C::Al, false, r(3), r(0), 52)?)?;
    c.emit(0x86028, transfer_word(C::Al, false, r(1), r(0), 56)?)?;
    c.emit(0x8602c, data_immediate(C::Ne, Op::Rsb, r(1), r(1), 0x10)?)?;
    c.emit(0x86030, transfer_word(C::Al, false, r(1), r(0), 60)?)?;
    c.emit(0x86034, bx(C::Al, r(14)))?;
    Ok(())
}

fn function_86038(c: &mut Code) -> Result<(), String> {
    c.emit(0x86038, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8603c, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x86040, 1, 0x86064)?;
    c.literal(0x86044, 3, 0x86068)?;
    c.emit(0x86048, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x8604c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x86050, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x10000)?)?;
    c.emit(0x86054, branch(C::Al, 0x86054, 0x5214, true)?)?;
    c.emit(0x86058, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x8605c, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x86060, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x86064, 0x200014e, true)?;
    c.word(0x86068, 0x10cff0, true)?;
    Ok(())
}

fn function_8606c(c: &mut Code) -> Result<(), String> {
    c.literal(0x8606c, 0, 0x86078)?;
    c.emit(0x86070, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x86074, bx(C::Al, r(14)))?;
    c.word(0x86078, 0x10cff0, true)?;
    Ok(())
}

fn function_8607c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8607c, transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x86080, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x86084, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4c)?)?;
    c.emit(0x86088, data_register(C::Al, Op::Mov, r(9), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.literal(0x8608c, 0, 0x86264)?;
    c.emit(0x86090, data_register(C::Al, Op::Mov, r(6), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x86094, data_register(C::Al, Op::Mov, r(10), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x86098, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8609c, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x860a0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x860a4, branch(C::Eq, 0x860a4, 0x8625c, false)?)?;
    c.emit(0x860a8, data_immediate(C::Al, Op::Add, r(0), r(13), 0xc)?)?;
    c.emit(0x860ac, data_immediate_with_flags(C::Al, Op::And, r(2), r(6), 0xf, true)?)?;
    c.emit(0x860b0, data_immediate(C::Al, Op::Bic, r(12), r(6), 0xf)?)?;
    c.emit(0x860b4, data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x860b8, data_immediate(C::Al, Op::Add, r(0), r(0), 0xc)?)?;
    c.emit(0x860bc, data_immediate(C::Al, Op::Bic, r(1), r(0), 0xf)?)?;
    c.emit(0x860c0, data_immediate(C::Ne, Op::Rsb, r(3), r(3), 0x10)?)?;
    c.emit(0x860c4, transfer_word(C::Al, false, r(1), r(13), 60)?)?;
    c.emit(0x860c8, transfer_word(C::Al, false, r(12), r(13), 64)?)?;
    c.emit(0x860cc, transfer_word(C::Al, false, r(2), r(13), 68)?)?;
    c.emit(0x860d0, transfer_word(C::Al, false, r(3), r(13), 72)?)?;
    c.emit(0x860d4, branch(C::Eq, 0x860d4, 0x86188, false)?)?;
    c.emit(0x860d8, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x10)?)?;
    c.emit(0x860dc, branch(C::Cc, 0x860dc, 0x86180, false)?)?;
    c.emit(0x860e0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x90)?)?;
    c.emit(0x860e4, transfer_word(C::Al, false, r(9), r(13), 4)?)?;
    c.emit(0x860e8, data_immediate(C::Al, Op::Sub, r(8), r(10), 0x10)?)?;
    c.emit(0x860ec, transfer_word(C::Al, false, r(4), r(13), 0)?)?;
    c.emit(0x860f0, data_register(C::Al, Op::Bic, r(2), r(2), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x860f4, data_register(C::Al, Op::Orr, r(0), r(2), r(9), Shift::Lsl, 0, false)?)?;
    c.literal(0x860f8, 9, 0x86268)?;
    c.emit(0x860fc, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0x86100, branch(C::Al, 0x86100, 0x86110, false)?)?;
    c.emit(0x86104, transfer_word(C::Al, true, r(1), r(13), 60)?)?;
    c.emit(0x86108, transfer_word(C::Al, true, r(12), r(13), 64)?)?;
    c.emit(0x8610c, data_immediate(C::Al, Op::Sub, r(8), r(8), 0x10)?)?;
    c.emit(0x86110, data_immediate(C::Al, Op::Add, r(2), r(12), 0x4)?)?;
    c.emit(0x86114, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x86118, data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8611c, transfer_word(C::Al, true, r(0), r(13), 68)?)?;
    c.emit(0x86120, transfer_multiple(C::Al, true, r(2), 0x4010, BlockMode::DecrementAfter, false)?)?;
    c.emit(0x86124, data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.emit(0x86128, data_immediate(C::Al, Op::Add, r(2), r(2), 0x4)?)?;
    c.emit(0x8612c, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x4)?)?;
    c.emit(0x86130, transfer_word_register(C::Al, true, r(5), r(9), r(0), Shift::Lsl, 2)?)?;
    c.emit(0x86134, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x86138, data_immediate(C::Al, Op::Rsb, r(0), r(0), 0x4)?)?;
    c.emit(0x8613c, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 3, false)?)?;
    c.emit(0x86140, data_register_shifted_by_register(C::Al, Op::And, r(4), r(5), r(4), Shift::Lsr, r(6), false)?)?;
    c.emit(0x86144, data_register(C::Al, Op::Mvn, r(5), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x86148, data_register_shifted_by_register(C::Al, Op::And, r(0), r(5), r(14), Shift::Lsl, r(0), false)?)?;
    c.emit(0x8614c, data_register(C::Al, Op::Orr, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x86150, transfer_word_postindexed(C::Al, false, r(0), r(7), 4)?)?;
    c.emit(0x86154, branch(C::Ne, 0x86154, 0x8611c, false)?)?;
    c.emit(0x86158, data_immediate(C::Al, Op::Add, r(0), r(12), 0x10)?)?;
    c.emit(0x8615c, transfer_word(C::Al, false, r(0), r(13), 64)?)?;
    c.emit(0x86160, transfer_word(C::Al, true, r(0), r(13), 8)?)?;
    c.emit(0x86164, branch(C::Al, 0x86164, 0x862c4, true)?)?;
    c.emit(0x86168, data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x10)?)?;
    c.emit(0x8616c, branch(C::Cs, 0x8616c, 0x86104, false)?)?;
    c.emit(0x86170, transfer_word(C::Al, true, r(12), r(13), 64)?)?;
    c.emit(0x86174, transfer_word(C::Al, true, r(2), r(13), 68)?)?;
    c.emit(0x86178, transfer_multiple(C::Al, true, r(13), 0x210, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x8617c, data_immediate(C::Al, Op::And, r(10), r(10), 0xf)?)?;
    c.emit(0x86180, data_register(C::Al, Op::Add, r(7), r(12), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x86184, branch(C::Al, 0x86184, 0x861dc, false)?)?;
    c.emit(0x86188, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x10)?)?;
    c.emit(0x8618c, branch(C::Cc, 0x8618c, 0x861d8, false)?)?;
    c.emit(0x86190, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x90)?)?;
    c.emit(0x86194, data_register(C::Al, Op::Bic, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x86198, data_register(C::Al, Op::Orr, r(5), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x8619c, data_immediate(C::Al, Op::Sub, r(0), r(10), 0x10)?)?;
    c.emit(0x861a0, data_immediate(C::Al, Op::Bic, r(8), r(0), 0xf)?)?;
    c.emit(0x861a4, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0x861a8, data_register(C::Al, Op::Add, r(0), r(6), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x861ac, data_immediate(C::Al, Op::Add, r(7), r(0), 0x10)?)?;
    c.emit(0x861b0, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x861b4, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x861b8, branch(C::Al, 0x861b8, 0x862c4, true)?)?;
    c.emit(0x861bc, data_immediate(C::Al, Op::Sub, r(10), r(10), 0x10)?)?;
    c.emit(0x861c0, data_immediate(C::Al, Op::Add, r(6), r(6), 0x10)?)?;
    c.emit(0x861c4, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0xf)?)?;
    c.emit(0x861c8, branch(C::Hi, 0x861c8, 0x861b0, false)?)?;
    c.emit(0x861cc, transfer_word(C::Al, true, r(0), r(13), 8)?)?;
    c.emit(0x861d0, data_register(C::Al, Op::Sub, r(10), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x861d4, branch(C::Al, 0x861d4, 0x861dc, false)?)?;
    c.emit(0x861d8, data_register(C::Al, Op::Mov, r(7), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x861dc, data_immediate(C::Al, Op::Add, r(0), r(13), 0xc)?)?;
    c.emit(0x861e0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x861e4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x30)?)?;
    c.emit(0x861e8, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x861ec, branch(C::Al, 0x861ec, 0x31ec, true)?)?;
    c.emit(0x861f0, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.emit(0x861f4, branch(C::Eq, 0x861f4, 0x8625c, false)?)?;
    c.emit(0x861f8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x90)?)?;
    c.emit(0x861fc, data_register(C::Al, Op::Bic, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x86200, data_register(C::Al, Op::Orr, r(5), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x86204, transfer_word(C::Al, true, r(0), r(13), 60)?)?;
    c.emit(0x86208, data_immediate_with_flags(C::Al, Op::Sub, r(10), r(10), 0x1, true)?)?;
    c.emit(0x8620c, transfer_word(C::Al, false, r(6), r(0), 0)?)?;
    c.emit(0x86210, transfer_byte_postindexed(C::Al, true, r(1), r(7), 1)?)?;
    c.emit(0x86214, transfer_byte(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x86218, branch(C::Eq, 0x86218, 0x86248, false)?)?;
    c.emit(0x8621c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x86220, transfer_word(C::Al, true, r(1), r(13), 60)?)?;
    c.emit(0x86224, transfer_byte_postindexed(C::Al, true, r(2), r(7), 1)?)?;
    c.emit(0x86228, data_immediate_with_flags(C::Al, Op::Sub, r(10), r(10), 0x1, true)?)?;
    c.emit(0x8622c, data_register(C::Al, Op::Add, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x86230, transfer_byte(C::Al, false, r(2), r(1), 1)?)?;
    c.emit(0x86234, branch(C::Eq, 0x86234, 0x86248, false)?)?;
    c.emit(0x86238, data_immediate(C::Al, Op::Add, r(1), r(0), 0x1)?)?;
    c.emit(0x8623c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xe)?)?;
    c.emit(0x86240, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x86244, branch(C::Lt, 0x86244, 0x86220, false)?)?;
    c.emit(0x86248, transfer_word(C::Al, true, r(1), r(13), 60)?)?;
    c.emit(0x8624c, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x86250, branch(C::Al, 0x86250, 0x862c4, true)?)?;
    c.emit(0x86254, data_immediate(C::Al, Op::Cmp, r(0), r(10), 0x0)?)?;
    c.emit(0x86258, branch(C::Ne, 0x86258, 0x86204, false)?)?;
    c.emit(0x8625c, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x86260, transfer_multiple(C::Al, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x86264, 0x10cff0, true)?;
    c.word(0x86268, 0xf4e0c, true)?;
    Ok(())
}

fn function_8626c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8626c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.emit(0x86270, data_immediate(C::Al, Op::Orr, r(0), r(0), 0x80)?)?;
    c.emit(0x86274, data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(0x86278, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x8627c, bx(C::Al, r(14)))?;
    Ok(())
}

fn function_86280(c: &mut Code) -> Result<(), String> {
    c.emit(0x86280, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x86284, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(0x86288, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x20)?)?;
    c.emit(0x8628c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.emit(0x86290, data_register(C::Al, Op::Orr, r(1), r(2), r(1), Shift::Lsl, 8, false)?)?;
    c.emit(0x86294, data_register(C::Al, Op::Mov, r(2), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(0x86298, data_immediate(C::Al, Op::Add, r(2), r(2), 0xf)?)?;
    c.emit(0x8629c, data_immediate(C::Al, Op::Bic, r(3), r(2), 0xf)?)?;
    c.emit(0x862a0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x40)?)?;
    c.emit(0x862a4, transfer_multiple(C::Al, false, r(3), 0x6, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x862a8, transfer_word(C::Al, false, r(2), r(3), 8)?)?;
    c.emit(0x862ac, transfer_word(C::Al, false, r(2), r(3), 12)?)?;
    c.emit(0x862b0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x18)?)?;
    c.emit(0x862b4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.emit(0x862b8, branch(C::Al, 0x862b8, 0x8607c, true)?)?;
    c.emit(0x862bc, data_register(C::Al, Op::Mov, r(13), r(0), r(11), Shift::Lsl, 0, false)?)?;
    c.emit(0x862c0, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_862c4(c: &mut Code) -> Result<(), String> {
    c.emit(0x862c4, transfer_multiple(C::Al, false, r(13), 0x40f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x862c8, transfer_multiple(C::Al, true, r(1), 0xf0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x862cc, transfer_multiple(C::Al, false, r(0), 0xf0, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x862d0, transfer_multiple(C::Al, true, r(13), 0x80f0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_862d4(c: &mut Code) -> Result<(), String> {
    c.emit(0x862d4, transfer_multiple(C::Al, false, r(13), 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x862d8, data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.emit(0x862dc, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x18)?)?;
    c.emit(0x862e0, data_register(C::Al, Op::Mov, r(4), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x862e4, data_register(C::Al, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x862e8, data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x862ec, data_register(C::Al, Op::Mov, r(7), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x862f0, branch(C::Al, 0x862f0, 0x8606c, true)?)?;
    c.emit(0x862f4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x862f8, branch(C::Eq, 0x862f8, 0x86344, false)?)?;
    c.emit(0x862fc, transfer_word(C::Al, true, r(0), r(11), 8)?)?;
    c.emit(0x86300, transfer_word(C::Al, true, r(2), r(11), 16)?)?;
    c.emit(0x86304, transfer_word(C::Al, true, r(3), r(11), 12)?)?;
    c.emit(0x86308, transfer_word(C::Al, true, r(12), r(11), 24)?)?;
    c.emit(0x8630c, transfer_word(C::Al, true, r(1), r(11), 20)?)?;
    c.emit(0x86310, transfer_byte(C::Al, false, r(0), r(13), 20)?)?;
    c.emit(0x86314, transfer_byte(C::Al, false, r(3), r(13), 21)?)?;
    c.emit(0x86318, store_halfword(C::Al, r(2), r(13), 22)?)?;
    c.emit(0x8631c, data_immediate(C::Al, Op::Add, r(0), r(13), 0x14)?)?;
    c.emit(0x86320, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.emit(0x86324, transfer_multiple(C::Al, false, r(13), 0x1002, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x86328, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x8632c, data_register(C::Al, Op::Mov, r(3), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x86330, transfer_word(C::Al, false, r(2), r(13), 8)?)?;
    c.emit(0x86334, transfer_word(C::Al, false, r(0), r(13), 12)?)?;
    c.emit(0x86338, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x8633c, data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x86340, branch(C::Al, 0x86340, 0x85ba0, true)?)?;
    c.emit(0x86344, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x10)?)?;
    c.emit(0x86348, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    for (at, function) in FUNCTIONS {
        c.check(at)?;
        function(&mut c)?;
    }
    c.check(END)?;
    if (c.code_bytes, c.data_bytes, c.bytes.len()) != (CODE_BYTES, DATA_BYTES, END - START) {
        return Err("callback dispatch code/data partition mismatch".into());
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
    fn exact_partition_and_entry_extents() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, FILE_MAPPING_DELTA), (0x85de0, 0x8634c, 0x4000));
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (1344, 44, 1388));
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!(LITERALS.len() * 4, DATA_BYTES);
        assert!(LITERALS.windows(2).all(|w| w[0].0 < w[1].0));
        let mut c = Code::new(START);
        for (index, (start, function)) in FUNCTIONS.into_iter().enumerate() {
            let end = FUNCTIONS.get(index + 1).map_or(END, |&(at, _)| at);
            assert_eq!(c.at, start);
            assert!(!LITERALS.iter().any(|&(at, _)| at == start));
            let before = (c.code_bytes, c.data_bytes);
            function(&mut c).unwrap();
            let data = LITERALS.iter().filter(|&&(at, _)| (start..end).contains(&at)).count() * 4;
            assert_eq!(c.at, end);
            assert_eq!((c.code_bytes - before.0, c.data_bytes - before.1), (end - start - data, data));
        }
        assert_eq!((c.at, c.code_bytes, c.data_bytes), (END, CODE_BYTES, DATA_BYTES));
        assert_eq!(c.bytes, bytes);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "ee4856121b64247eaeaaead17996ab0f5e2a8d3251ce86a9444cb8c999e479c3");
    }

    #[test]
    fn literal_values_and_pc_relative_targets() {
        let bytes = fragment().unwrap();
        for (slot, value) in LITERALS {
            assert_eq!(word(&bytes, slot), value);
            assert!(LITERAL_REFERENCES.iter().any(|&(_, target)| target == slot));
        }
        for (index, &(at, slot)) in LITERAL_REFERENCES.iter().enumerate() {
            assert!(!LITERAL_REFERENCES[..index].iter().any(|&(site, _)| site == at));
            let instruction = word(&bytes, at);
            assert_eq!(instruction & 0xffff_0000, 0xe59f_0000);
            assert_eq!(at + 8 + (instruction & 0xfff) as usize, slot);
            assert!(LITERALS.iter().any(|&(pool, _)| pool == slot));
            let mut c = Code::new(at);
            c.literal(at, ((instruction >> 12) & 15) as u8, slot).unwrap();
            assert_eq!(c.bytes, instruction.to_le_bytes());
        }
    }

    #[test]
    fn branches_and_unresolved_indirect_dispatch() {
        let bytes = fragment().unwrap();
        let expected = [
            (0x85dfc, 0x85e20, false, 0x0),
            (0x85e1c, 0x85e0c, false, 0x3),
            (0x85e48, 0x85ba0, true, 0xe),
            (0x85e80, 0x8606c, true, 0xe),
            (0x85e9c, 0x85f44, false, 0x1),
            (0x85ea0, 0x8606c, true, 0xe),
            (0x85ec0, 0x85edc, false, 0x0),
            (0x85ec4, 0x8606c, true, 0xe),
            (0x85ed8, 0x85f44, false, 0xe),
            (0x85eec, 0x8606c, true, 0xe),
            (0x85f08, 0x85f20, false, 0x0),
            (0x85f1c, 0x85f24, false, 0xe),
            (0x85f30, 0x85eec, false, 0x1),
            (0x85f94, 0x85fac, false, 0x0),
            (0x85f98, 0x8606c, true, 0xe),
            (0x85fb8, 0x85f88, false, 0x1),
            (0x85fbc, 0x8606c, true, 0xe),
            (0x85fd0, 0x8606c, true, 0xe),
            (0x86054, 0x5214, true, 0xe),
            (0x860a4, 0x8625c, false, 0x0),
            (0x860d4, 0x86188, false, 0x0),
            (0x860dc, 0x86180, false, 0x3),
            (0x86100, 0x86110, false, 0xe),
            (0x86154, 0x8611c, false, 0x1),
            (0x86164, 0x862c4, true, 0xe),
            (0x8616c, 0x86104, false, 0x2),
            (0x86184, 0x861dc, false, 0xe),
            (0x8618c, 0x861d8, false, 0x3),
            (0x861b8, 0x862c4, true, 0xe),
            (0x861c8, 0x861b0, false, 0x8),
            (0x861d4, 0x861dc, false, 0xe),
            (0x861ec, 0x31ec, true, 0xe),
            (0x861f4, 0x8625c, false, 0x0),
            (0x86218, 0x86248, false, 0x0),
            (0x86234, 0x86248, false, 0x0),
            (0x86244, 0x86220, false, 0xb),
            (0x86250, 0x862c4, true, 0xe),
            (0x86258, 0x86204, false, 0x1),
            (0x862b8, 0x8607c, true, 0xe),
            (0x862f0, 0x8606c, true, 0xe),
            (0x862f8, 0x86344, false, 0x0),
            (0x86340, 0x85ba0, true, 0xe),
        ];
        let mut count = 0;
        for at in (START..END).step_by(4) {
            if LITERALS.iter().any(|&(slot, _)| slot == at) { continue; }
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 != 0x0a00_0000 { continue; }
            count += 1;
            let target = (at as i64 + 8 + (((instruction << 8) as i32 >> 8) as i64 * 4)) as usize;
            assert!(expected.contains(&(at, target, instruction & (1 << 24) != 0, instruction >> 28)));
            if (START..END).contains(&target) {
                assert!(!LITERALS.iter().any(|&(slot, _)| slot == target));
            }
        }
        assert_eq!(count, expected.len());
        for (at, register) in [(0x85e94, 3), (0x85eb8, 3), (0x85ed4, 2),
            (0x85f00, 3), (0x85fa8, 2), (0x85fcc, 2)] {
            assert_eq!(word(&bytes, at), branch_exchange_with_link(C::Al, r(register)).unwrap());
        }
        assert_eq!(word(&bytes, 0x85f68), bx(C::Ne, r(14)));
        assert_eq!(word(&bytes, 0x85fe4), bx(C::Al, r(2)));
        assert_eq!(word(&bytes, 0x85fe0), transfer_multiple(C::Al, true, r(13), 0x48f0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(&bytes[0x85e54-START..0x85e64-START], &bytes[0x862c4-START..0x862d4-START]);
    }

    #[test]
    fn flags_and_predication() {
        let bytes = fragment().unwrap();
        for (at, flags, condition) in [
            (0x85de4, 0, 0xe),
            (0x85de8, 0, 0xe),
            (0x85dec, 1, 0xe),
            (0x85df4, 0, 0x2),
            (0x85df8, 1, 0xe),
            (0x85e00, 0, 0xe),
            (0x85e04, 0, 0xe),
            (0x85e08, 0, 0xe),
            (0x85e10, 0, 0xe),
            (0x85e14, 1, 0xe),
            (0x85e20, 0, 0xe),
            (0x85e24, 0, 0xe),
            (0x85e28, 0, 0xe),
            (0x85e2c, 0, 0xe),
            (0x85e30, 0, 0xe),
            (0x85e40, 0, 0xe),
            (0x85e44, 0, 0xe),
            (0x85e4c, 0, 0xe),
            (0x85e68, 0, 0xe),
            (0x85e70, 0, 0xe),
            (0x85e88, 0, 0xe),
            (0x85e90, 0, 0xe),
            (0x85e98, 1, 0xe),
            (0x85ea8, 0, 0xe),
            (0x85eac, 0, 0xe),
            (0x85eb4, 0, 0xe),
            (0x85ebc, 1, 0xe),
            (0x85edc, 0, 0xe),
            (0x85ee0, 0, 0xe),
            (0x85ee4, 0, 0xe),
            (0x85ee8, 0, 0xe),
            (0x85ef4, 0, 0xe),
            (0x85efc, 0, 0xe),
            (0x85f04, 1, 0xe),
            (0x85f14, 0, 0xe),
            (0x85f24, 0, 0xe),
            (0x85f28, 0, 0xe),
            (0x85f2c, 1, 0xe),
            (0x85f38, 0, 0xe),
            (0x85f3c, 1, 0xe),
            (0x85f40, 0, 0xb),
            (0x85f4c, 0, 0xe),
            (0x85f64, 1, 0xe),
            (0x85f70, 0, 0xe),
            (0x85f78, 0, 0xe),
            (0x85f7c, 0, 0xe),
            (0x85f84, 0, 0xe),
            (0x85f90, 1, 0xe),
            (0x85fac, 0, 0xe),
            (0x85fb0, 0, 0xe),
            (0x85fb4, 1, 0xe),
            (0x86010, 0, 0xe),
            (0x86014, 0, 0xe),
            (0x86018, 1, 0xe),
            (0x8601c, 0, 0xe),
            (0x8602c, 0, 0x1),
            (0x8603c, 0, 0xe),
            (0x86048, 0, 0xe),
            (0x8604c, 0, 0xe),
            (0x86050, 0, 0xe),
            (0x86058, 1, 0xe),
            (0x8605c, 0, 0x1),
            (0x86080, 0, 0xe),
            (0x86084, 0, 0xe),
            (0x86088, 0, 0xe),
            (0x86090, 0, 0xe),
            (0x86094, 0, 0xe),
            (0x86098, 0, 0xe),
            (0x860a0, 1, 0xe),
            (0x860a8, 0, 0xe),
            (0x860ac, 1, 0xe),
            (0x860b0, 0, 0xe),
            (0x860b4, 0, 0xe),
            (0x860b8, 0, 0xe),
            (0x860bc, 0, 0xe),
            (0x860c0, 0, 0x1),
            (0x860d8, 1, 0xe),
            (0x860e0, 0, 0xe),
            (0x860e8, 0, 0xe),
            (0x860f0, 0, 0xe),
            (0x860f4, 0, 0xe),
            (0x8610c, 0, 0xe),
            (0x86110, 0, 0xe),
            (0x86114, 0, 0xe),
            (0x86118, 0, 0xe),
            (0x86124, 0, 0xe),
            (0x86128, 0, 0xe),
            (0x8612c, 1, 0xe),
            (0x86134, 0, 0xe),
            (0x86138, 0, 0xe),
            (0x8613c, 0, 0xe),
            (0x86140, 0, 0xe),
            (0x86144, 0, 0xe),
            (0x86148, 0, 0xe),
            (0x8614c, 0, 0xe),
            (0x86158, 0, 0xe),
            (0x86168, 1, 0xe),
            (0x8617c, 0, 0xe),
            (0x86180, 0, 0xe),
            (0x86188, 1, 0xe),
            (0x86190, 0, 0xe),
            (0x86194, 0, 0xe),
            (0x86198, 0, 0xe),
            (0x8619c, 0, 0xe),
            (0x861a0, 0, 0xe),
            (0x861a8, 0, 0xe),
            (0x861ac, 0, 0xe),
            (0x861b0, 0, 0xe),
            (0x861b4, 0, 0xe),
            (0x861bc, 0, 0xe),
            (0x861c0, 0, 0xe),
            (0x861c4, 1, 0xe),
            (0x861d0, 0, 0xe),
            (0x861d8, 0, 0xe),
            (0x861dc, 0, 0xe),
            (0x861e0, 0, 0xe),
            (0x861e4, 0, 0xe),
            (0x861e8, 0, 0xe),
            (0x861f0, 1, 0xe),
            (0x861f8, 0, 0xe),
            (0x861fc, 0, 0xe),
            (0x86200, 0, 0xe),
            (0x86208, 1, 0xe),
            (0x8621c, 0, 0xe),
            (0x86228, 1, 0xe),
            (0x8622c, 0, 0xe),
            (0x86238, 0, 0xe),
            (0x8623c, 1, 0xe),
            (0x86240, 0, 0xe),
            (0x8624c, 0, 0xe),
            (0x86254, 1, 0xe),
            (0x8625c, 0, 0xe),
            (0x8626c, 0, 0xe),
            (0x86270, 0, 0xe),
            (0x86274, 0, 0xe),
            (0x86284, 0, 0xe),
            (0x86288, 0, 0xe),
            (0x8628c, 0, 0xe),
            (0x86290, 0, 0xe),
            (0x86294, 0, 0xe),
            (0x86298, 0, 0xe),
            (0x8629c, 0, 0xe),
            (0x862a0, 0, 0xe),
            (0x862b0, 0, 0xe),
            (0x862b4, 0, 0xe),
            (0x862bc, 0, 0xe),
            (0x862d8, 0, 0xe),
            (0x862dc, 0, 0xe),
            (0x862e0, 0, 0xe),
            (0x862e4, 0, 0xe),
            (0x862e8, 0, 0xe),
            (0x862ec, 0, 0xe),
            (0x862f4, 1, 0xe),
            (0x8631c, 0, 0xe),
            (0x86320, 0, 0xe),
            (0x86328, 0, 0xe),
            (0x8632c, 0, 0xe),
            (0x86338, 0, 0xe),
            (0x8633c, 0, 0xe),
            (0x86344, 0, 0xe),
        ] {
            assert_eq!((word(&bytes, at) >> 20) & 1, flags);
            assert_eq!(word(&bytes, at) >> 28, condition);
        }
    }
    #[test]
    fn exact_partition_and_literal_guards_are_atomic() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let pool = LITERALS.iter().find(|&&(slot, _)| slot == at);
            for data in [false, true] {
                for value in [0, pool.map_or(0, |&(_, v)| v), pool.map_or(1, |&(_, v)| v ^ 1)] {
                    let valid = (START..END).contains(&at) && at % 4 == 0
                        && if data { pool.is_some_and(|&(_, v)| v == value) } else { pool.is_none() };
                    let mut c = Code::new(at);
                    assert_eq!(c.word(at, value, data).is_ok(), valid);
                    assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes),
                        (at + 4 * usize::from(valid), 4 * usize::from(valid),
                         4 * usize::from(valid && !data), 4 * usize::from(valid && data)));
                }
            }
        }
        for (at, slot, rd) in [(START, 0x85f54, 0), (0x85e6c, 0x85f58, 1),
            (0x85e6c, 0x85f55, 1), (0x85e6c, END, 1), (0x85f54, 0x85f54, 1),
            (END, 0x85f54, 1), (0x85e6c, 0x85f54, 16)] {
            let mut c = Code::new(at);
            assert!(c.literal(at, rd, slot).is_err());
            assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes), (at, 0, 0, 0));
        }
        let mut c = Code::new(START);
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert_eq!((c.at, c.bytes.len()), (START, 0));
        assert!(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x12345678).is_err());
        assert!(data_immediate_with_flags(C::Al, Op::Cmp, r(0), r(0), 0, true).is_err());
        assert!(transfer_word(C::Al, true, r(0), R::PC, 4096).is_err());
        assert!(branch(C::Al, START as u32, START as u32 + 2, false).is_err());
        assert!(branch_exchange_with_link(C::Al, R::PC).is_err());
    }

    #[test]
    fn authenticated_same_buffer_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        let bytes = fragment().unwrap();
        assert_eq!(bytes, reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "ee4856121b64247eaeaaead17996ab0f5e2a8d3251ce86a9444cb8c999e479c3");
    }
}
