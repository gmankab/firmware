use arm32::{
    branch, branch_exchange_with_link, bx, data_immediate, data_register,
    load_halfword, store_halfword, transfer_byte, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x84c54;
pub const END: usize = 0x850d0;
pub const CODE_BYTES: usize = 1124;
pub const DATA_BYTES: usize = 24;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const LITERALS: [(usize, u32); 6] = [
    (0x84d40, 0x10cfb4), (0x84d44, 0x10b528),
    (0x84db8, 0x10cfb4), (0x84dbc, 0x10b528),
    (0x84ea0, 0x10b568), (0x84ed0, 0x10b568),
];

fn r(number: u8) -> R {
    R::new(number).expect("A32 register")
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
            return Err(format!("ARM handle formatting at {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !(START..END).contains(&at) || at % 4 != 0
            || LITERALS.iter().any(|&(slot, _)| at == slot)
        {
            return Err(format!("ARM handle instruction outside code: {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        self.code_bytes += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !LITERALS.contains(&(at, value)) {
            return Err(format!("ARM handle invalid literal at {at:#x}: {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        self.data_bytes += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, rd: R, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| slot == address) {
            return Err(format!("ARM handle unknown literal: {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, rd, R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn create_handle(c: &mut Code) -> Result<(), String> {
    c.emit(0x84c54, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84c58, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x10)?)?;
    c.emit(0x84c5c, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    c.emit(0x84c60, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.literal(0x84c64, r(0), 0x84d40)?;
    c.emit(0x84c68, data_register(C::Al, Op::Mov, r(6), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84c6c, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x10)?)?;
    c.emit(0x84c70, data_register(C::Al, Op::Mov, r(7), R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84c74, branch(C::Al, 0x84c74, 0x85838, true)?)?;
    c.emit(0x84c78, transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0x84c7c, data_immediate(C::Al, Op::Mov, r(5), R::R0, 1)?)?;
    c.emit(0x84c80, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x84c84, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84c88, branch(C::Eq, 0x84c88, 0x84d34, false)?)?;
    c.emit(0x84c8c, branch(C::Al, 0x84c8c, 0x8606c, true)?)?;
    c.emit(0x84c90, transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(0x84c94, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x84c98, data_immediate(C::Al, Op::Mov, r(5), R::R0, 0)?)?;
    c.emit(0x84c9c, transfer_word(C::Al, true, r(3), r(2), 0x24)?)?;
    c.emit(0x84ca0, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84ca4, data_immediate(C::Al, Op::Mov, r(2), R::R0, 0)?)?;
    c.emit(0x84ca8, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x84cac, transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(0x84cb0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84cb4, branch(C::Eq, 0x84cb4, 0x84cd8, false)?)?;
    c.literal(0x84cb8, r(0), 0x84d40)?;
    c.emit(0x84cbc, branch(C::Al, 0x84cbc, 0x859dc, true)?)?;
    c.literal(0x84cc0, r(0), 0x84d44)?;
    c.emit(0x84cc4, transfer_word(C::Al, false, r(4), r(0), 0)?)?;
    c.emit(0x84cc8, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84ccc, transfer_word(C::Al, false, r(5), r(0), 0)?)?;
    c.emit(0x84cd0, data_immediate(C::Al, Op::Mov, r(5), R::R0, 1)?)?;
    c.emit(0x84cd4, branch(C::Al, 0x84cd4, 0x84d34, false)?)?;
    c.emit(0x84cd8, transfer_word(C::Al, true, r(0), r(1), 0)?)?;
    c.emit(0x84cdc, data_immediate(C::Al, Op::Cmp, R::R0, r(6), 1)?)?;
    c.emit(0x84ce0, transfer_word(C::Al, false, r(6), r(0), 4)?)?;
    c.emit(0x84ce4, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x84ce8, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84cec, transfer_byte(C::Al, false, r(7), r(0), 8)?)?;
    c.emit(0x84cf0, branch(C::Ne, 0x84cf0, 0x84d34, false)?)?;
    c.emit(0x84cf4, data_immediate(C::Al, Op::Add, r(0), R::SP, 4)?)?;
    c.emit(0x84cf8, data_register(C::Al, Op::Mov, r(1), R::R0, R::SP, Shift::Lsl, 0, false)?)?;
    c.emit(0x84cfc, branch(C::Al, 0x84cfc, 0x509c, true)?)?;
    c.emit(0x84d00, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x84d04, transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(0x84d08, data_immediate(C::Al, Op::Mov, r(5), R::R0, 0)?)?;
    c.emit(0x84d0c, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84d10, transfer_byte(C::Al, false, r(0), r(1), 9)?)?;
    c.emit(0x84d14, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x84d18, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84d1c, transfer_byte(C::Al, false, r(5), r(0), 0xa)?)?;
    c.emit(0x84d20, branch(C::Al, 0x84d20, 0x8e88, true)?)?;
    c.emit(0x84d24, transfer_word(C::Al, true, r(0), r(0), 0x64)?)?;
    c.emit(0x84d28, transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(0x84d2c, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84d30, store_halfword(C::Al, r(0), r(1), 0xc)?)?;
    c.emit(0x84d34, data_register(C::Al, Op::Mov, r(0), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x84d38, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x10)?)?;
    c.emit(0x84d3c, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x84d40, 0x10cfb4)?;
    c.data(0x84d44, 0x10b528)?;
    Ok(())
}

fn destroy_handle(c: &mut Code) -> Result<(), String> {
    c.emit(0x84d48, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84d4c, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x84d50, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84d54, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x84d58, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84d5c, branch(C::Eq, 0x84d5c, 0x84db4, false)?)?;
    c.literal(0x84d60, r(0), 0x84db8)?;
    c.emit(0x84d64, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0x84d68, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 1)?)?;
    c.emit(0x84d6c, branch(C::Lt, 0x84d6c, 0x84db4, false)?)?;
    c.emit(0x84d70, branch(C::Al, 0x84d70, 0x8606c, true)?)?;
    c.emit(0x84d74, transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.emit(0x84d78, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x84d7c, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84d80, transfer_word(C::Al, true, r(2), r(2), 0x28)?)?;
    c.emit(0x84d84, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84d88, branch_exchange_with_link(C::Al, r(2))?)?;
    c.emit(0x84d8c, transfer_word(C::Al, true, r(1), r(4), 0)?)?;
    c.literal(0x84d90, r(0), 0x84db8)?;
    c.emit(0x84d94, branch(C::Al, 0x84d94, 0x859dc, true)?)?;
    c.emit(0x84d98, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84d9c, transfer_multiple(C::Ne, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.literal(0x84da0, r(0), 0x84dbc)?;
    c.emit(0x84da4, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x84da8, transfer_word(C::Al, false, r(4), r(0), 0)?)?;
    c.emit(0x84dac, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84db0, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x84db4, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x84db8, 0x10cfb4)?;
    c.data(0x84dbc, 0x10b528)?;
    Ok(())
}

fn check_handle(c: &mut Code) -> Result<(), String> {
    c.emit(0x84dc0, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84dc4, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x84dc8, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    c.emit(0x84dcc, transfer_word(C::Al, false, r(0), R::SP, 4)?)?;
    c.emit(0x84dd0, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x84dd4, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84dd8, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0x84ddc, branch(C::Al, 0x84ddc, 0x852ac, true)?)?;
    c.emit(0x84de0, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0)?)?;
    c.emit(0x84de4, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84de8, branch(C::Eq, 0x84de8, 0x84e08, false)?)?;
    c.emit(0x84dec, branch(C::Al, 0x84dec, 0x8527c, true)?)?;
    c.emit(0x84df0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84df4, branch(C::Eq, 0x84df4, 0x84e08, false)?)?;
    c.emit(0x84df8, branch(C::Al, 0x84df8, 0x850f4, true)?)?;
    c.emit(0x84dfc, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e00, data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0)?)?;
    c.emit(0x84e04, data_immediate(C::Ne, Op::Mov, r(4), R::R0, 1)?)?;
    c.emit(0x84e08, data_register(C::Al, Op::Mov, r(0), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e0c, data_immediate(C::Al, Op::Sub, R::SP, r(11), 8)?)?;
    c.emit(0x84e10, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn set_handle_tag(c: &mut Code) -> Result<(), String> {
    c.emit(0x84e14, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 4)?)?;
    c.emit(0x84e18, transfer_word(C::Al, false, r(0), R::SP, 0)?)?;
    c.emit(0x84e1c, transfer_word(C::Al, true, r(0), R::SP, 0)?)?;
    c.emit(0x84e20, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84e24, store_halfword(C::Al, r(1), r(0), 0xc)?)?;
    c.emit(0x84e28, data_immediate(C::Al, Op::Add, R::SP, R::SP, 4)?)?;
    c.emit(0x84e2c, bx(C::Al, R::LR))?;
    Ok(())
}

fn serialized_create_handle(c: &mut Code) -> Result<(), String> {
    c.emit(0x84e30, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84e34, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x18)?)?;
    c.emit(0x84e38, data_register(C::Al, Op::Mov, r(5), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e3c, data_register(C::Al, Op::Mov, r(6), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e40, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0)?)?;
    c.emit(0x84e44, data_register(C::Al, Op::Mov, r(4), R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e48, data_immediate(C::Al, Op::Mov, r(7), R::R0, 1)?)?;
    c.emit(0x84e4c, data_immediate(C::Al, Op::Cmp, R::R0, r(5), 2)?)?;
    c.emit(0x84e50, transfer_word(C::Al, false, r(0), r(6), 0)?)?;
    c.emit(0x84e54, branch(C::Eq, 0x84e54, 0x84e98, false)?)?;
    c.emit(0x84e58, branch(C::Al, 0x84e58, 0x85728, true)?)?;
    c.literal(0x84e5c, r(8), 0x84ea0)?;
    c.emit(0x84e60, transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(0x84e64, branch(C::Al, 0x84e64, 0x5958, true)?)?;
    c.emit(0x84e68, branch(C::Al, 0x84e68, 0x850f4, true)?)?;
    c.emit(0x84e6c, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84e70, branch(C::Eq, 0x84e70, 0x84e8c, false)?)?;
    c.emit(0x84e74, data_register(C::Al, Op::Mov, r(0), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e78, data_register(C::Al, Op::Mov, r(1), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e7c, data_register(C::Al, Op::Mov, r(2), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e80, branch(C::Al, 0x84e80, 0x84c54, true)?)?;
    c.emit(0x84e84, data_register(C::Al, Op::Mov, r(7), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e88, branch(C::Al, 0x84e88, 0x84e90, false)?)?;
    c.emit(0x84e8c, data_immediate(C::Al, Op::Mov, r(7), R::R0, 1)?)?;
    c.emit(0x84e90, transfer_word(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(0x84e94, branch(C::Al, 0x84e94, 0x59e8, true)?)?;
    c.emit(0x84e98, data_register(C::Al, Op::Mov, r(0), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x84e9c, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x84ea0, 0x10b568)?;
    Ok(())
}

fn serialized_destroy_handle(c: &mut Code) -> Result<(), String> {
    c.emit(0x84ea4, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84ea8, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.literal(0x84eac, r(5), 0x84ed0)?;
    c.emit(0x84eb0, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84eb4, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(0x84eb8, branch(C::Al, 0x84eb8, 0x5958, true)?)?;
    c.emit(0x84ebc, data_register(C::Al, Op::Mov, r(0), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x84ec0, branch(C::Al, 0x84ec0, 0x84d48, true)?)?;
    c.emit(0x84ec4, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(0x84ec8, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x84ecc, branch(C::Al, 0x84ecc, 0x59e8, false)?)?;
    c.data(0x84ed0, 0x10b568)?;
    Ok(())
}

fn format_output(c: &mut Code) -> Result<(), String> {
    c.emit(0x84ed4, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84ed8, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(0x84edc, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0xa4)?)?;
    c.emit(0x84ee0, transfer_word(C::Al, false, r(0), r(11), -0x24)?)?;
    c.emit(0x84ee4, data_register(C::Al, Op::Mov, r(6), R::R0, r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x84ee8, data_register(C::Al, Op::Mov, r(7), R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84eec, data_register(C::Al, Op::Mov, r(4), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84ef0, data_immediate(C::Al, Op::Mov, r(5), R::R0, 1)?)?;
    c.emit(0x84ef4, transfer_word(C::Al, true, r(0), r(11), -0x24)?)?;
    c.emit(0x84ef8, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84efc, branch(C::Eq, 0x84efc, 0x850b4, false)?)?;
    c.emit(0x84f00, transfer_word(C::Al, true, r(0), r(11), -0x24)?)?;
    c.emit(0x84f04, branch(C::Al, 0x84f04, 0x84dc0, true)?)?;
    c.emit(0x84f08, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84f0c, branch(C::Eq, 0x84f0c, 0x84fac, false)?)?;
    c.emit(0x84f10, data_immediate(C::Al, Op::Mov, r(8), R::R0, 0)?)?;
    c.emit(0x84f14, data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0)?)?;
    c.emit(0x84f18, branch(C::Eq, 0x84f18, 0x84fb4, false)?)?;
    c.emit(0x84f1c, data_register(C::Al, Op::Mov, r(9), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x84f20, transfer_word(C::Al, true, r(0), r(11), -0x24)?)?;
    c.emit(0x84f24, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84f28, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0x84f2c, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84f30, branch(C::Eq, 0x84f30, 0x85030, false)?)?;
    c.emit(0x84f34, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 1)?)?;
    c.emit(0x84f38, branch(C::Ne, 0x84f38, 0x85060, false)?)?;
    c.emit(0x84f3c, transfer_word(C::Al, true, r(0), r(11), -0x24)?)?;
    c.emit(0x84f40, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84f44, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84f48, transfer_word(C::Al, true, r(1), r(11), -0x24)?)?;
    c.emit(0x84f4c, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84f50, transfer_byte(C::Al, true, r(2), r(1), 4)?)?;
    c.emit(0x84f54, transfer_word(C::Al, true, r(1), r(11), -0x24)?)?;
    c.emit(0x84f58, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84f5c, transfer_byte(C::Al, true, r(3), r(1), 8)?)?;
    c.emit(0x84f60, transfer_word(C::Al, true, r(1), r(11), -0x24)?)?;
    c.emit(0x84f64, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84f68, transfer_byte(C::Al, true, r(1), r(1), 9)?)?;
    c.emit(0x84f6c, transfer_word(C::Al, true, r(6), r(11), -0x24)?)?;
    c.emit(0x84f70, transfer_word(C::Al, true, r(6), r(6), 0)?)?;
    c.emit(0x84f74, transfer_byte(C::Al, true, r(6), r(6), 0xa)?)?;
    c.emit(0x84f78, transfer_word(C::Al, true, r(5), r(11), -0x24)?)?;
    c.emit(0x84f7c, transfer_word(C::Al, true, r(5), r(5), 0)?)?;
    c.emit(0x84f80, load_halfword(C::Al, r(5), r(5), 0xc)?)?;
    c.emit(0x84f84, transfer_multiple(C::Al, false, R::SP, r(1).bit() | r(6).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x84f88, data_immediate(C::Al, Op::Add, r(1), R::SP, 8)?)?;
    c.emit(0x84f8c, transfer_multiple(C::Al, false, r(1), r(5).bit() | r(7).bit() | r(9).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x84f90, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x80)?)?;
    c.emit(0x84f94, data_immediate(C::Al, Op::Mov, r(7), R::R0, 8)?)?;
    c.emit(0x84f98, data_register(C::Al, Op::And, r(1), r(1), r(4), Shift::Lsl, 6, false)?)?;
    c.emit(0x84f9c, data_register(C::Al, Op::And, r(7), r(7), r(4), Shift::Lsl, 3, false)?)?;
    c.emit(0x84fa0, data_register(C::Al, Op::Orr, r(1), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84fa4, branch(C::Al, 0x84fa4, 0x862d4, true)?)?;
    c.emit(0x84fa8, branch(C::Al, 0x84fa8, 0x850a0, false)?)?;
    c.emit(0x84fac, data_immediate(C::Al, Op::Mov, r(5), R::R0, 0)?)?;
    c.emit(0x84fb0, branch(C::Al, 0x84fb0, 0x850b4, false)?)?;
    c.emit(0x84fb4, data_immediate(C::Al, Op::Add, r(10), r(11), 8)?)?;
    c.emit(0x84fb8, data_immediate(C::Al, Op::Add, r(9), R::SP, 0x18)?)?;
    c.emit(0x84fbc, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x84)?)?;
    c.emit(0x84fc0, data_register(C::Al, Op::Mov, r(2), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x84fc4, data_register(C::Al, Op::Mov, r(0), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x84fc8, data_register(C::Al, Op::Mov, r(3), R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x84fcc, transfer_word(C::Al, false, r(10), R::SP, 0x14)?)?;
    c.emit(0x84fd0, branch(C::Al, 0x84fd0, 0x3fd4, true)?)?;
    c.emit(0x84fd4, data_register(C::Al, Op::Mov, r(7), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84fd8, data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0)?)?;
    c.emit(0x84fdc, branch(C::Lt, 0x84fdc, 0x850c0, false)?)?;
    c.emit(0x84fe0, data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0x84)?)?;
    c.emit(0x84fe4, branch(C::Cc, 0x84fe4, 0x84f20, false)?)?;
    c.emit(0x84fe8, data_immediate(C::Al, Op::Add, r(7), r(7), 1)?)?;
    c.emit(0x84fec, data_immediate(C::Al, Op::Sub, r(1), r(11), 0x20)?)?;
    c.emit(0x84ff0, data_register(C::Al, Op::Mov, r(0), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x84ff4, branch(C::Al, 0x84ff4, 0x57d4, true)?)?;
    c.emit(0x84ff8, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x84ffc, branch(C::Ne, 0x84ffc, 0x850b4, false)?)?;
    c.emit(0x85000, transfer_word(C::Al, true, r(9), r(11), -0x20)?)?;
    c.emit(0x85004, data_immediate(C::Al, Op::Cmp, R::R0, r(9), 0)?)?;
    c.emit(0x85008, branch(C::Eq, 0x85008, 0x850b4, false)?)?;
    c.emit(0x8500c, data_register(C::Al, Op::Mov, r(0), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x85010, data_register(C::Al, Op::Mov, r(1), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x85014, data_register(C::Al, Op::Mov, r(2), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x85018, data_register(C::Al, Op::Mov, r(3), R::R0, r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x8501c, transfer_word(C::Al, false, r(10), R::SP, 0x14)?)?;
    c.emit(0x85020, branch(C::Al, 0x85020, 0x3fd4, true)?)?;
    c.emit(0x85024, data_register(C::Al, Op::Mov, r(7), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85028, data_register(C::Al, Op::Mov, r(8), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x8502c, branch(C::Al, 0x8502c, 0x84f20, false)?)?;
    c.emit(0x85030, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x80)?)?;
    c.emit(0x85034, data_immediate(C::Al, Op::Mov, r(1), R::R0, 8)?)?;
    c.emit(0x85038, data_register(C::Al, Op::Mov, r(2), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x8503c, data_register(C::Al, Op::Mov, r(3), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x85040, data_register(C::Al, Op::And, r(0), r(0), r(4), Shift::Lsl, 6, false)?)?;
    c.emit(0x85044, data_register(C::Al, Op::And, r(1), r(1), r(4), Shift::Lsl, 3, false)?)?;
    c.emit(0x85048, data_register(C::Al, Op::Orr, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8504c, transfer_word(C::Al, true, r(0), r(11), -0x24)?)?;
    c.emit(0x85050, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85054, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85058, branch(C::Al, 0x85058, 0x8607c, true)?)?;
    c.emit(0x8505c, branch(C::Al, 0x8505c, 0x850a0, false)?)?;
    c.emit(0x85060, transfer_word(C::Al, true, r(0), r(11), -0x24)?)?;
    c.emit(0x85064, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85068, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x8506c, transfer_word(C::Al, true, r(1), r(11), -0x24)?)?;
    c.emit(0x85070, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x85074, transfer_byte(C::Al, true, r(2), r(1), 4)?)?;
    c.emit(0x85078, transfer_word(C::Al, true, r(1), r(11), -0x24)?)?;
    c.emit(0x8507c, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x85080, transfer_byte(C::Al, true, r(3), r(1), 8)?)?;
    c.emit(0x85084, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0x80)?)?;
    c.emit(0x85088, transfer_multiple(C::Al, false, R::SP, r(7).bit() | r(9).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x8508c, data_immediate(C::Al, Op::Mov, r(7), R::R0, 8)?)?;
    c.emit(0x85090, data_register(C::Al, Op::And, r(1), r(1), r(4), Shift::Lsl, 6, false)?)?;
    c.emit(0x85094, data_register(C::Al, Op::And, r(7), r(7), r(4), Shift::Lsl, 3, false)?)?;
    c.emit(0x85098, data_register(C::Al, Op::Orr, r(1), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8509c, branch(C::Al, 0x8509c, 0x85d98, true)?)?;
    c.emit(0x850a0, data_immediate(C::Al, Op::Mov, r(5), R::R0, 0)?)?;
    c.emit(0x850a4, data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0)?)?;
    c.emit(0x850a8, branch(C::Eq, 0x850a8, 0x850b4, false)?)?;
    c.emit(0x850ac, data_register(C::Al, Op::Mov, r(0), R::R0, r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x850b0, branch(C::Al, 0x850b0, 0x5838, true)?)?;
    c.emit(0x850b4, data_register(C::Al, Op::Mov, r(0), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x850b8, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(0x850bc, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x850c0, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x21)?)?;
    c.emit(0x850c4, data_immediate(C::Al, Op::Mov, r(7), R::R0, 1)?)?;
    c.emit(0x850c8, transfer_byte(C::Al, false, r(0), R::SP, 0x18)?)?;
    c.emit(0x850cc, branch(C::Al, 0x850cc, 0x84f20, false)?)?;
    Ok(())
}
pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    create_handle(&mut c)?;
    destroy_handle(&mut c)?;
    check_handle(&mut c)?;
    set_handle_tag(&mut c)?;
    serialized_create_handle(&mut c)?;
    serialized_destroy_handle(&mut c)?;
    format_output(&mut c)?;
    c.check(END)?;
    if c.code_bytes != CODE_BYTES || c.data_bytes != DATA_BYTES
        || c.bytes.len() != END - START || c.bytes.len() != CODE_BYTES + DATA_BYTES
    {
        return Err("ARM handle formatting partition mismatch".into());
    }
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    fn target(bytes: &[u8], at: usize) -> usize {
        let instruction = word(bytes, at);
        assert_eq!((instruction >> 25) & 7, 5);
        let displacement = ((instruction << 8) as i32 >> 6) as isize;
        (at as isize + 8 + displacement) as usize
    }

    #[test]
    fn exact_partition_and_guards() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 1148);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(LITERALS.len() * 4, DATA_BYTES);
        assert_eq!(START + FILE_MAPPING_DELTA, 0x88c54);
        assert_eq!(END + FILE_MAPPING_DELTA, 0x890d0);
        for (at, value) in LITERALS {
            assert_eq!(word(&bytes, at), value);
            let mut c = Code::new(at);
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.data(at, value ^ 1).is_err());
            assert!(c.data(at + 4, value).is_err());
            assert_eq!((c.at, c.code_bytes, c.data_bytes), (at, 0, 0));
            assert!(c.bytes.is_empty());
            c.data(at, value).unwrap();
            assert_eq!((c.at, c.code_bytes, c.data_bytes), (at + 4, 0, 4));
        }
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code::new(at);
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.data(at, 0x10cfb4).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code::new(START);
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.data(START, 0x10cfb4).is_err());
        assert!(c.literal(START, R::R0, START).is_err());
        assert_eq!(c.at, START);
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn entry_rooted_partitions() {
        let mut c = Code::new(START);
        for (emit, end, code, data) in [
            (create_handle as fn(&mut Code) -> Result<(), String>, 0x84d48, 236, 8),
            (destroy_handle, 0x84dc0, 112, 8),
            (check_handle, 0x84e14, 84, 0),
            (set_handle_tag, 0x84e30, 28, 0),
            (serialized_create_handle, 0x84ea4, 112, 4),
            (serialized_destroy_handle, 0x84ed4, 44, 4),
            (format_output, END, 508, 0),
        ] {
            let before = (c.code_bytes, c.data_bytes);
            emit(&mut c).unwrap();
            c.check(end).unwrap();
            assert_eq!((c.code_bytes - before.0, c.data_bytes - before.1), (code, data));
        }
    }

    #[test]
    fn literals_flags_indirect_calls_and_output_control_flow() {
        let bytes = fragment().unwrap();
        for (at, slot) in [
            (0x84c64, 0x84d40), (0x84cb8, 0x84d40), (0x84cc0, 0x84d44),
            (0x84d60, 0x84db8), (0x84d90, 0x84db8), (0x84da0, 0x84dbc),
            (0x84e5c, 0x84ea0), (0x84eac, 0x84ed0),
        ] {
            let instruction = word(&bytes, at);
            assert_eq!((instruction >> 16) & 15, 15);
            assert_eq!(at + 8 + (instruction & 0xfff) as usize, slot);
        }
        assert_eq!(word(&bytes, 0x84ca8), 0xe12fff33);
        assert_eq!(word(&bytes, 0x84d88), 0xe12fff32);
        for at in [0x84c84, 0x84cdc, 0x84d98, 0x84df0, 0x84f2c, 0x84fd8, 0x84fe0] {
            assert_ne!(word(&bytes, at) & (1 << 20), 0);
        }
        for at in [0x84ce0, 0x84ce4, 0x84ce8, 0x84cec] {
            assert_eq!(word(&bytes, at) & (1 << 20), if at == 0x84ce4 || at == 0x84ce8 { 1 << 20 } else { 0 });
        }
        for (at, destination) in [
            (0x84e64, 0x5958), (0x84e80, START), (0x84e94, 0x59e8),
            (0x84eb8, 0x5958), (0x84ec0, 0x84d48), (0x84ecc, 0x59e8),
            (0x84f04, 0x84dc0), (0x84fa4, 0x862d4), (0x85058, 0x8607c),
            (0x8509c, 0x85d98), (0x84fdc, 0x850c0), (0x84fe4, 0x84f20),
            (0x8502c, 0x84f20), (0x850cc, 0x84f20),
        ] {
            assert_eq!(target(&bytes, at), destination);
        }
        assert_eq!(word(&bytes, 0x84fdc) >> 28, C::Lt as u32);
        assert_eq!(word(&bytes, 0x84fe4) >> 28, C::Cc as u32);
        assert_eq!(word(&bytes, 0x850bc), 0xe8bd8ff0);
        assert_eq!(word(&bytes, 0x850c0), 0xe3a00021);
        assert_eq!(word(&bytes, 0x850c4), 0xe3a07001);
        assert_eq!(word(&bytes, 0x850c8), 0xe5cd0018);
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
