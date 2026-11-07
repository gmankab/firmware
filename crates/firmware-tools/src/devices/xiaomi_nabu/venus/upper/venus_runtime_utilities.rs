use arm32::{
    branch, branch_exchange_immediate, branch_exchange_with_link, bx, data_immediate,
    data_immediate_with_flags, data_register, multiply_accumulate, multiply_register,
    transfer_multiple, transfer_word, BlockMode, Condition as C, DataOp as Op,
    Register as R, Shift,
};

pub const START: usize = 0x80c18;
pub const END: usize = 0x817b8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2716;
pub const DATA_BYTES: usize = 260;

const LITERALS: [(usize, u32); 65] = [
    (0x80c58, 0xf4bef),
    (0x80c5c, 0xf4bf1),
    (0x80c60, 0xf4c18),
    (0x80c8c, 0xf4bef),
    (0x80c90, 0xf4bf1),
    (0x80c94, 0xf4c25),
    (0x80cb8, 0xf4bef),
    (0x80cbc, 0xf4bf1),
    (0x80cc0, 0xf4c2f),
    (0x80cf4, 0xf4bef),
    (0x80cf8, 0xf4bf1),
    (0x80cfc, 0xf4c40),
    (0x80d6c, 0xf4bef),
    (0x80d70, 0xf4bf1),
    (0x80d74, 0xf4c51),
    (0x80dec, 0xf4bef),
    (0x80df0, 0xf4bf1),
    (0x80df4, 0xf4c61),
    (0x80ee4, 0xf4c93),
    (0x80ee8, 0xf4bf1),
    (0x80eec, 0xf4c71),
    (0x80ef0, 0xf4c82),
    (0x80ef4, 0xf4bef),
    (0x80f78, 0xf4d7a),
    (0x80f7c, 0xf4bf1),
    (0x80f80, 0xf4d8c),
    (0x80f84, 0xf4bef),
    (0x81074, 0xf4c93),
    (0x81078, 0xf4bf1),
    (0x8107c, 0xf4cc0),
    (0x81080, 0xf4c82),
    (0x81084, 0xf4bef),
    (0x81168, 0xf4bef),
    (0x8116c, 0xf4bf1),
    (0x81170, 0xf4cd2),
    (0x811d4, 0xf4bef),
    (0x811d8, 0xf4bf1),
    (0x811dc, 0xf4ce2),
    (0x8124c, 0xf4d02),
    (0x81250, 0xf4bf1),
    (0x81254, 0xf4cf2),
    (0x81258, 0xf4bef),
    (0x812d0, 0xf4bef),
    (0x812d4, 0xf4bf1),
    (0x812d8, 0xf4d21),
    (0x81310, 0xf4bef),
    (0x81314, 0xf4bf1),
    (0x81318, 0xf4d31),
    (0x8134c, 0xf4bef),
    (0x81350, 0xf4bf1),
    (0x81354, 0xf4d3e),
    (0x8138c, 0xf4bef),
    (0x81390, 0xf4bf1),
    (0x81394, 0xf4d4b),
    (0x813cc, 0xf4bef),
    (0x813d0, 0xf4bf1),
    (0x813d4, 0xf4d5c),
    (0x81424, 0xf4bef),
    (0x81428, 0xf4bf1),
    (0x8142c, 0xf4d6d),
    (0x815a0, 0xf4d9b),
    (0x815e8, 0xf4d9b),
    (0x81620, 0xf4d9b),
    (0x81658, 0xf4d9b),
    (0x816c8, 0xf4d9b),
];

#[cfg(test)]
const FUNCTIONS: [usize; 35] = [
    0x80c18,
    0x80c64,
    0x80c98,
    0x80cc4,
    0x80d00,
    0x80d78,
    0x80df8,
    0x80ef8,
    0x80f88,
    0x81088,
    0x81174,
    0x811e0,
    0x8125c,
    0x812dc,
    0x8131c,
    0x81358,
    0x81398,
    0x813d8,
    0x81430,
    0x81438,
    0x81440,
    0x81530,
    0x815a4,
    0x815ec,
    0x81624,
    0x8165c,
    0x81664,
    0x8166c,
    0x816cc,
    0x816f8,
    0x81718,
    0x81738,
    0x81758,
    0x81778,
    0x81798,
];

const LITERAL_REFERENCES: [(usize, usize); 99] = [
    (0x80c3c, 0x80c58),
    (0x80c40, 0x80c5c),
    (0x80c44, 0x80c60),
    (0x80c78, 0x80c8c),
    (0x80c7c, 0x80c90),
    (0x80c80, 0x80c94),
    (0x80ca4, 0x80cb8),
    (0x80ca8, 0x80cbc),
    (0x80cac, 0x80cc0),
    (0x80cd8, 0x80cf4),
    (0x80cdc, 0x80cf8),
    (0x80ce0, 0x80cfc),
    (0x80d44, 0x80d6c),
    (0x80d48, 0x80d70),
    (0x80d4c, 0x80d74),
    (0x80d58, 0x80d6c),
    (0x80d5c, 0x80d70),
    (0x80d60, 0x80d74),
    (0x80dac, 0x80dec),
    (0x80db0, 0x80df0),
    (0x80db4, 0x80df4),
    (0x80dc0, 0x80dec),
    (0x80dc4, 0x80df0),
    (0x80dc8, 0x80df4),
    (0x80e3c, 0x80ee4),
    (0x80e40, 0x80ee8),
    (0x80e44, 0x80eec),
    (0x80e7c, 0x80ef4),
    (0x80e80, 0x80ee8),
    (0x80e84, 0x80eec),
    (0x80e94, 0x80ef4),
    (0x80e98, 0x80ee8),
    (0x80e9c, 0x80eec),
    (0x80eb4, 0x80ef0),
    (0x80eb8, 0x80ee8),
    (0x80ebc, 0x80eec),
    (0x80f10, 0x80f78),
    (0x80f14, 0x80f7c),
    (0x80f18, 0x80f80),
    (0x80f58, 0x80f84),
    (0x80f5c, 0x80f7c),
    (0x80f60, 0x80f80),
    (0x80fcc, 0x81074),
    (0x80fd0, 0x81078),
    (0x80fd4, 0x8107c),
    (0x81008, 0x81084),
    (0x8100c, 0x81078),
    (0x81010, 0x8107c),
    (0x81020, 0x81084),
    (0x81024, 0x81078),
    (0x81028, 0x8107c),
    (0x81044, 0x81080),
    (0x81048, 0x81078),
    (0x8104c, 0x8107c),
    (0x810bc, 0x81168),
    (0x810c0, 0x8116c),
    (0x810cc, 0x81168),
    (0x810d0, 0x8116c),
    (0x810dc, 0x81168),
    (0x810e0, 0x8116c),
    (0x810ec, 0x81168),
    (0x810f0, 0x8116c),
    (0x810f8, 0x81170),
    (0x8119c, 0x811d4),
    (0x811a0, 0x811d8),
    (0x811a4, 0x811dc),
    (0x811b0, 0x811d4),
    (0x811b4, 0x811d8),
    (0x811b8, 0x811dc),
    (0x81200, 0x8124c),
    (0x81204, 0x81250),
    (0x81208, 0x81254),
    (0x81228, 0x81258),
    (0x8122c, 0x81250),
    (0x81230, 0x81254),
    (0x812a4, 0x812d0),
    (0x812a8, 0x812d4),
    (0x812ac, 0x812d8),
    (0x812f0, 0x81310),
    (0x812f4, 0x81314),
    (0x812f8, 0x81318),
    (0x81330, 0x8134c),
    (0x81334, 0x81350),
    (0x81338, 0x81354),
    (0x8136c, 0x8138c),
    (0x81370, 0x81390),
    (0x81374, 0x81394),
    (0x813ac, 0x813cc),
    (0x813b0, 0x813d0),
    (0x813b4, 0x813d4),
    (0x81404, 0x81424),
    (0x81408, 0x81428),
    (0x8140c, 0x8142c),
    (0x81554, 0x815a0),
    (0x8156c, 0x815a0),
    (0x815b8, 0x815e8),
    (0x815fc, 0x81620),
    (0x81634, 0x81658),
    (0x81694, 0x816c8),
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
        self.emit(at, transfer_word(C::Al, true, r(rd), R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn function_80c18(c: &mut Code) -> Result<(), String> {
    c.emit(0x80c18, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80c1c, branch(C::Eq, 0x80c1c, 0x80c34, false)?)?;
    c.emit(0x80c20, transfer_word(C::Al, true, r(1), r(0), 0x0)?)?;
    c.emit(0x80c24, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80c28, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x80c2c, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x80c30, bx(C::Al, r(14)))?;
    c.emit(0x80c34, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x80c38, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x80c3c, 0, 0x80c58)?;
    c.literal(0x80c40, 1, 0x80c5c)?;
    c.literal(0x80c44, 3, 0x80c60)?;
    c.emit(0x80c48, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x37)?)?;
    c.emit(0x80c4c, branch(C::Al, 0x80c4c, 0x83d2c, true)?)?;
    c.emit(0x80c50, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x80c54, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x80c58, 0xf4bef, true)?;
    c.word(0x80c5c, 0xf4bf1, true)?;
    c.word(0x80c60, 0xf4c18, true)?;
    Ok(())
}

fn function_80c64(c: &mut Code) -> Result<(), String> {
    c.emit(0x80c64, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80c68, data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x80c6c, transfer_word(C::Ne, false, r(1), r(0), 0x0)?)?;
    c.emit(0x80c70, transfer_word(C::Ne, false, r(1), r(0), 0x4)?)?;
    c.emit(0x80c74, bx(C::Ne, r(14)))?;
    c.literal(0x80c78, 0, 0x80c8c)?;
    c.literal(0x80c7c, 1, 0x80c90)?;
    c.literal(0x80c80, 3, 0x80c94)?;
    c.emit(0x80c84, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4e)?)?;
    c.emit(0x80c88, branch(C::Al, 0x80c88, 0x83d2c, false)?)?;
    c.word(0x80c8c, 0xf4bef, true)?;
    c.word(0x80c90, 0xf4bf1, true)?;
    c.word(0x80c94, 0xf4c25, true)?;
    Ok(())
}

fn function_80c98(c: &mut Code) -> Result<(), String> {
    c.emit(0x80c98, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80c9c, transfer_word(C::Ne, false, r(1), r(0), 0x0)?)?;
    c.emit(0x80ca0, bx(C::Ne, r(14)))?;
    c.literal(0x80ca4, 0, 0x80cb8)?;
    c.literal(0x80ca8, 1, 0x80cbc)?;
    c.literal(0x80cac, 3, 0x80cc0)?;
    c.emit(0x80cb0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x69)?)?;
    c.emit(0x80cb4, branch(C::Al, 0x80cb4, 0x83d2c, false)?)?;
    c.word(0x80cb8, 0xf4bef, true)?;
    c.word(0x80cbc, 0xf4bf1, true)?;
    c.word(0x80cc0, 0xf4c2f, true)?;
    Ok(())
}

fn function_80cc4(c: &mut Code) -> Result<(), String> {
    c.emit(0x80cc4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80cc8, transfer_word(C::Ne, true, r(0), r(0), 0x0)?)?;
    c.emit(0x80ccc, bx(C::Ne, r(14)))?;
    c.emit(0x80cd0, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x80cd4, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x80cd8, 0, 0x80cf4)?;
    c.literal(0x80cdc, 1, 0x80cf8)?;
    c.literal(0x80ce0, 3, 0x80cfc)?;
    c.emit(0x80ce4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x81)?)?;
    c.emit(0x80ce8, branch(C::Al, 0x80ce8, 0x83d2c, true)?)?;
    c.emit(0x80cec, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80cf0, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x80cf4, 0xf4bef, true)?;
    c.word(0x80cf8, 0xf4bf1, true)?;
    c.word(0x80cfc, 0xf4c40, true)?;
    Ok(())
}

fn function_80d00(c: &mut Code) -> Result<(), String> {
    c.emit(0x80d00, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x80d04, branch(C::Eq, 0x80d04, 0x80d44, false)?)?;
    c.emit(0x80d08, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80d0c, branch(C::Eq, 0x80d0c, 0x80d58, false)?)?;
    c.emit(0x80d10, transfer_word(C::Al, true, r(2), r(0), 0x0)?)?;
    c.emit(0x80d14, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x80d18, data_immediate(C::Ne, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x80d1c, transfer_multiple(C::Ne, false, r(1), 0xc, BlockMode::IncrementBefore, false)?)?;
    c.emit(0x80d20, transfer_word(C::Ne, false, r(1), r(2), 0x8)?)?;
    c.emit(0x80d24, transfer_word(C::Ne, false, r(1), r(0), 0x0)?)?;
    c.emit(0x80d28, bx(C::Ne, r(14)))?;
    c.emit(0x80d2c, transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(0x80d30, transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.emit(0x80d34, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80d38, transfer_word(C::Al, false, r(0), r(1), 0x4)?)?;
    c.emit(0x80d3c, transfer_word(C::Al, false, r(0), r(1), 0x8)?)?;
    c.emit(0x80d40, bx(C::Al, r(14)))?;
    c.literal(0x80d44, 0, 0x80d6c)?;
    c.literal(0x80d48, 1, 0x80d70)?;
    c.literal(0x80d4c, 3, 0x80d74)?;
    c.emit(0x80d50, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x99)?)?;
    c.emit(0x80d54, branch(C::Al, 0x80d54, 0x83d2c, false)?)?;
    c.literal(0x80d58, 0, 0x80d6c)?;
    c.literal(0x80d5c, 1, 0x80d70)?;
    c.literal(0x80d60, 3, 0x80d74)?;
    c.emit(0x80d64, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xa0)?)?;
    c.emit(0x80d68, branch(C::Al, 0x80d68, 0x83d2c, false)?)?;
    c.word(0x80d6c, 0xf4bef, true)?;
    c.word(0x80d70, 0xf4bf1, true)?;
    c.word(0x80d74, 0xf4c51, true)?;
    Ok(())
}

fn function_80d78(c: &mut Code) -> Result<(), String> {
    c.emit(0x80d78, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x80d7c, branch(C::Eq, 0x80d7c, 0x80dac, false)?)?;
    c.emit(0x80d80, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80d84, branch(C::Eq, 0x80d84, 0x80dc0, false)?)?;
    c.emit(0x80d88, transfer_word(C::Al, true, r(2), r(0), 0x4)?)?;
    c.emit(0x80d8c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x80d90, branch(C::Eq, 0x80d90, 0x80dd4, false)?)?;
    c.emit(0x80d94, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x80d98, transfer_word(C::Al, false, r(3), r(1), 0x4)?)?;
    c.emit(0x80d9c, transfer_word(C::Al, false, r(2), r(1), 0x8)?)?;
    c.emit(0x80da0, transfer_word(C::Al, false, r(1), r(2), 0x4)?)?;
    c.emit(0x80da4, transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.emit(0x80da8, bx(C::Al, r(14)))?;
    c.literal(0x80dac, 0, 0x80dec)?;
    c.literal(0x80db0, 1, 0x80df0)?;
    c.literal(0x80db4, 3, 0x80df4)?;
    c.emit(0x80db8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xc8)?)?;
    c.emit(0x80dbc, branch(C::Al, 0x80dbc, 0x83d2c, false)?)?;
    c.literal(0x80dc0, 0, 0x80dec)?;
    c.literal(0x80dc4, 1, 0x80df0)?;
    c.literal(0x80dc8, 3, 0x80df4)?;
    c.emit(0x80dcc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xcf)?)?;
    c.emit(0x80dd0, branch(C::Al, 0x80dd0, 0x83d2c, false)?)?;
    c.emit(0x80dd4, transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(0x80dd8, transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.emit(0x80ddc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80de0, transfer_word(C::Al, false, r(0), r(1), 0x4)?)?;
    c.emit(0x80de4, transfer_word(C::Al, false, r(0), r(1), 0x8)?)?;
    c.emit(0x80de8, bx(C::Al, r(14)))?;
    c.word(0x80dec, 0xf4bef, true)?;
    c.word(0x80df0, 0xf4bf1, true)?;
    c.word(0x80df4, 0xf4c61, true)?;
    Ok(())
}

fn function_80df8(c: &mut Code) -> Result<(), String> {
    c.emit(0x80df8, transfer_multiple(C::Al, false, r(13), 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x80dfc, data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.emit(0x80e00, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x80e04, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x80e08, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x80e0c, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x80e10, branch(C::Eq, 0x80e10, 0x80e7c, false)?)?;
    c.emit(0x80e14, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x80e18, branch(C::Eq, 0x80e18, 0x80e94, false)?)?;
    c.emit(0x80e1c, transfer_word(C::Al, true, r(0), r(5), 0x0)?)?;
    c.emit(0x80e20, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80e24, branch(C::Eq, 0x80e24, 0x80eac, false)?)?;
    c.emit(0x80e28, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x80e2c, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x80e30, branch(C::Al, 0x80e30, 0x80ef8, true)?)?;
    c.emit(0x80e34, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80e38, branch(C::Ne, 0x80e38, 0x80e54, false)?)?;
    c.literal(0x80e3c, 0, 0x80ee4)?;
    c.literal(0x80e40, 1, 0x80ee8)?;
    c.literal(0x80e44, 3, 0x80eec)?;
    c.emit(0x80e48, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x11)?)?;
    c.emit(0x80e4c, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x80e50, branch(C::Al, 0x80e50, 0x83d2c, true)?)?;
    c.emit(0x80e54, transfer_word(C::Al, false, r(6), r(4), 0x8)?)?;
    c.emit(0x80e58, transfer_word(C::Al, true, r(0), r(6), 0x4)?)?;
    c.emit(0x80e5c, transfer_word(C::Al, false, r(0), r(4), 0x4)?)?;
    c.emit(0x80e60, transfer_word(C::Al, false, r(4), r(6), 0x4)?)?;
    c.emit(0x80e64, transfer_word(C::Al, true, r(0), r(5), 0x4)?)?;
    c.emit(0x80e68, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x80e6c, transfer_word(C::Eq, false, r(4), r(5), 0x4)?)?;
    c.emit(0x80e70, transfer_word(C::Ne, true, r(0), r(4), 0x4)?)?;
    c.emit(0x80e74, transfer_word(C::Ne, false, r(4), r(0), 0x8)?)?;
    c.emit(0x80e78, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.literal(0x80e7c, 0, 0x80ef4)?;
    c.literal(0x80e80, 1, 0x80ee8)?;
    c.literal(0x80e84, 3, 0x80eec)?;
    c.emit(0x80e88, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xf9)?)?;
    c.emit(0x80e8c, transfer_multiple(C::Al, true, r(13), 0x48f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x80e90, branch(C::Al, 0x80e90, 0x83d2c, false)?)?;
    c.literal(0x80e94, 0, 0x80ef4)?;
    c.literal(0x80e98, 1, 0x80ee8)?;
    c.literal(0x80e9c, 3, 0x80eec)?;
    c.emit(0x80ea0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x100)?)?;
    c.emit(0x80ea4, transfer_multiple(C::Al, true, r(13), 0x48f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x80ea8, branch(C::Al, 0x80ea8, 0x83d2c, false)?)?;
    c.emit(0x80eac, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x80eb0, branch(C::Eq, 0x80eb0, 0x80ecc, false)?)?;
    c.literal(0x80eb4, 0, 0x80ef0)?;
    c.literal(0x80eb8, 1, 0x80ee8)?;
    c.literal(0x80ebc, 3, 0x80eec)?;
    c.emit(0x80ec0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7)?)?;
    c.emit(0x80ec4, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x80ec8, branch(C::Al, 0x80ec8, 0x83d2c, true)?)?;
    c.emit(0x80ecc, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80ed0, transfer_word(C::Al, false, r(4), r(5), 0x0)?)?;
    c.emit(0x80ed4, transfer_word(C::Al, false, r(4), r(5), 0x4)?)?;
    c.emit(0x80ed8, transfer_word(C::Al, false, r(0), r(4), 0x4)?)?;
    c.emit(0x80edc, transfer_word(C::Al, false, r(0), r(4), 0x8)?)?;
    c.emit(0x80ee0, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x80ee4, 0xf4c93, true)?;
    c.word(0x80ee8, 0xf4bf1, true)?;
    c.word(0x80eec, 0xf4c71, true)?;
    c.word(0x80ef0, 0xf4c82, true)?;
    c.word(0x80ef4, 0xf4bef, true)?;
    Ok(())
}

fn function_80ef8(c: &mut Code) -> Result<(), String> {
    c.emit(0x80ef8, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x80efc, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x80f00, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x80f04, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x80f08, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x80f0c, branch(C::Ne, 0x80f0c, 0x80f28, false)?)?;
    c.literal(0x80f10, 0, 0x80f78)?;
    c.literal(0x80f14, 1, 0x80f7c)?;
    c.literal(0x80f18, 3, 0x80f80)?;
    c.emit(0x80f1c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xb3)?)?;
    c.emit(0x80f20, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x80f24, branch(C::Al, 0x80f24, 0x83d2c, true)?)?;
    c.emit(0x80f28, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x80f2c, branch(C::Eq, 0x80f2c, 0x80f58, false)?)?;
    c.emit(0x80f30, transfer_word(C::Al, true, r(1), r(5), 0x0)?)?;
    c.emit(0x80f34, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80f38, branch(C::Al, 0x80f38, 0x80f40, false)?)?;
    c.emit(0x80f3c, transfer_word(C::Al, true, r(1), r(1), 0x4)?)?;
    c.emit(0x80f40, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x80f44, transfer_multiple(C::Eq, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x80f48, data_register(C::Al, Op::Cmp, r(0), r(1), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x80f4c, branch(C::Ne, 0x80f4c, 0x80f3c, false)?)?;
    c.emit(0x80f50, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x80f54, transfer_multiple(C::Al, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.literal(0x80f58, 0, 0x80f84)?;
    c.literal(0x80f5c, 1, 0x80f7c)?;
    c.literal(0x80f60, 3, 0x80f80)?;
    c.emit(0x80f64, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xb6)?)?;
    c.emit(0x80f68, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x80f6c, branch(C::Al, 0x80f6c, 0x83d2c, true)?)?;
    c.emit(0x80f70, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x80f74, transfer_multiple(C::Al, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.word(0x80f78, 0xf4d7a, true)?;
    c.word(0x80f7c, 0xf4bf1, true)?;
    c.word(0x80f80, 0xf4d8c, true)?;
    c.word(0x80f84, 0xf4bef, true)?;
    Ok(())
}

fn function_80f88(c: &mut Code) -> Result<(), String> {
    c.emit(0x80f88, transfer_multiple(C::Al, false, r(13), 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x80f8c, data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.emit(0x80f90, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x80f94, data_register(C::Al, Op::Mov, r(6), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x80f98, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x80f9c, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x80fa0, branch(C::Eq, 0x80fa0, 0x81008, false)?)?;
    c.emit(0x80fa4, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x80fa8, branch(C::Eq, 0x80fa8, 0x81020, false)?)?;
    c.emit(0x80fac, transfer_word(C::Al, true, r(0), r(5), 0x0)?)?;
    c.emit(0x80fb0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80fb4, branch(C::Eq, 0x80fb4, 0x8103c, false)?)?;
    c.emit(0x80fb8, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x80fbc, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x80fc0, branch(C::Al, 0x80fc0, 0x80ef8, true)?)?;
    c.emit(0x80fc4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x80fc8, branch(C::Ne, 0x80fc8, 0x80fe0, false)?)?;
    c.literal(0x80fcc, 0, 0x81074)?;
    c.literal(0x80fd0, 1, 0x81078)?;
    c.literal(0x80fd4, 3, 0x8107c)?;
    c.emit(0x80fd8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x14c)?)?;
    c.emit(0x80fdc, branch(C::Al, 0x80fdc, 0x83d2c, true)?)?;
    c.emit(0x80fe0, transfer_word(C::Al, false, r(6), r(4), 0x4)?)?;
    c.emit(0x80fe4, transfer_word(C::Al, true, r(0), r(6), 0x8)?)?;
    c.emit(0x80fe8, transfer_word(C::Al, false, r(0), r(4), 0x8)?)?;
    c.emit(0x80fec, transfer_word(C::Al, false, r(4), r(6), 0x8)?)?;
    c.emit(0x80ff0, transfer_word(C::Al, true, r(0), r(5), 0x0)?)?;
    c.emit(0x80ff4, data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x80ff8, transfer_word(C::Eq, false, r(4), r(5), 0x0)?)?;
    c.emit(0x80ffc, transfer_word(C::Ne, true, r(0), r(4), 0x8)?)?;
    c.emit(0x81000, transfer_word(C::Ne, false, r(4), r(0), 0x4)?)?;
    c.emit(0x81004, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.literal(0x81008, 0, 0x81084)?;
    c.literal(0x8100c, 1, 0x81078)?;
    c.literal(0x81010, 3, 0x8107c)?;
    c.emit(0x81014, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x134)?)?;
    c.emit(0x81018, transfer_multiple(C::Al, true, r(13), 0x48f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x8101c, branch(C::Al, 0x8101c, 0x83d2c, false)?)?;
    c.literal(0x81020, 0, 0x81084)?;
    c.literal(0x81024, 1, 0x81078)?;
    c.literal(0x81028, 3, 0x8107c)?;
    c.emit(0x8102c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3b)?)?;
    c.emit(0x81030, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x81034, transfer_multiple(C::Al, true, r(13), 0x48f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81038, branch(C::Al, 0x81038, 0x83d2c, false)?)?;
    c.emit(0x8103c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x81040, branch(C::Eq, 0x81040, 0x8105c, false)?)?;
    c.literal(0x81044, 0, 0x81080)?;
    c.literal(0x81048, 1, 0x81078)?;
    c.literal(0x8104c, 3, 0x8107c)?;
    c.emit(0x81050, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x42)?)?;
    c.emit(0x81054, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x81058, branch(C::Al, 0x81058, 0x83d2c, true)?)?;
    c.emit(0x8105c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x81060, transfer_word(C::Al, false, r(4), r(5), 0x0)?)?;
    c.emit(0x81064, transfer_word(C::Al, false, r(4), r(5), 0x4)?)?;
    c.emit(0x81068, transfer_word(C::Al, false, r(0), r(4), 0x4)?)?;
    c.emit(0x8106c, transfer_word(C::Al, false, r(0), r(4), 0x8)?)?;
    c.emit(0x81070, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x81074, 0xf4c93, true)?;
    c.word(0x81078, 0xf4bf1, true)?;
    c.word(0x8107c, 0xf4cc0, true)?;
    c.word(0x81080, 0xf4c82, true)?;
    c.word(0x81084, 0xf4bef, true)?;
    Ok(())
}

fn function_81088(c: &mut Code) -> Result<(), String> {
    c.emit(0x81088, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x8108c, branch(C::Eq, 0x8108c, 0x810cc, false)?)?;
    c.emit(0x81090, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81094, branch(C::Eq, 0x81094, 0x810dc, false)?)?;
    c.emit(0x81098, transfer_word(C::Al, true, r(2), r(0), 0x0)?)?;
    c.emit(0x8109c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x810a0, branch(C::Eq, 0x810a0, 0x810ec, false)?)?;
    c.emit(0x810a4, data_register(C::Al, Op::Mov, r(3), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x810a8, data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x810ac, branch(C::Eq, 0x810ac, 0x81104, false)?)?;
    c.emit(0x810b0, transfer_word(C::Al, true, r(3), r(3), 0x4)?)?;
    c.emit(0x810b4, data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x0)?)?;
    c.emit(0x810b8, branch(C::Ne, 0x810b8, 0x810a8, false)?)?;
    c.literal(0x810bc, 0, 0x81168)?;
    c.literal(0x810c0, 1, 0x8116c)?;
    c.emit(0x810c4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x81)?)?;
    c.emit(0x810c8, branch(C::Al, 0x810c8, 0x810f8, false)?)?;
    c.literal(0x810cc, 0, 0x81168)?;
    c.literal(0x810d0, 1, 0x8116c)?;
    c.emit(0x810d4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6b)?)?;
    c.emit(0x810d8, branch(C::Al, 0x810d8, 0x810f8, false)?)?;
    c.literal(0x810dc, 0, 0x81168)?;
    c.literal(0x810e0, 1, 0x8116c)?;
    c.emit(0x810e4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x72)?)?;
    c.emit(0x810e8, branch(C::Al, 0x810e8, 0x810f8, false)?)?;
    c.literal(0x810ec, 0, 0x81168)?;
    c.literal(0x810f0, 1, 0x8116c)?;
    c.emit(0x810f4, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7a)?)?;
    c.literal(0x810f8, 3, 0x81170)?;
    c.emit(0x810fc, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x81100, branch(C::Al, 0x81100, 0x83d2c, false)?)?;
    c.emit(0x81104, transfer_word(C::Al, true, r(3), r(0), 0x4)?)?;
    c.emit(0x81108, data_register(C::Al, Op::Cmp, r(0), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x8110c, branch(C::Eq, 0x8110c, 0x81144, false)?)?;
    c.emit(0x81110, data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x81114, branch(C::Eq, 0x81114, 0x81154, false)?)?;
    c.emit(0x81118, data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8111c, transfer_multiple(C::Ne, true, r(1), 0x5, BlockMode::IncrementBefore, false)?)?;
    c.emit(0x81120, transfer_word(C::Ne, false, r(0), r(2), 0x4)?)?;
    c.emit(0x81124, transfer_word(C::Ne, true, r(0), r(1), 0x4)?)?;
    c.emit(0x81128, transfer_word(C::Ne, false, r(2), r(0), 0x8)?)?;
    c.emit(0x8112c, bx(C::Ne, r(14)))?;
    c.emit(0x81130, transfer_word(C::Al, true, r(1), r(1), 0x8)?)?;
    c.emit(0x81134, transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.emit(0x81138, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8113c, transfer_word(C::Al, false, r(0), r(1), 0x4)?)?;
    c.emit(0x81140, bx(C::Al, r(14)))?;
    c.emit(0x81144, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x81148, transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(0x8114c, transfer_word(C::Al, false, r(1), r(0), 0x4)?)?;
    c.emit(0x81150, bx(C::Al, r(14)))?;
    c.emit(0x81154, transfer_word(C::Al, true, r(1), r(1), 0x4)?)?;
    c.emit(0x81158, transfer_word(C::Al, false, r(1), r(0), 0x0)?)?;
    c.emit(0x8115c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x81160, transfer_word(C::Al, false, r(0), r(1), 0x8)?)?;
    c.emit(0x81164, bx(C::Al, r(14)))?;
    c.word(0x81168, 0xf4bef, true)?;
    c.word(0x8116c, 0xf4bf1, true)?;
    c.word(0x81170, 0xf4cd2, true)?;
    Ok(())
}

fn function_81174(c: &mut Code) -> Result<(), String> {
    c.emit(0x81174, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81178, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x8117c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81180, branch(C::Eq, 0x81180, 0x8119c, false)?)?;
    c.emit(0x81184, transfer_word(C::Al, true, r(4), r(0), 0x0)?)?;
    c.emit(0x81188, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x8118c, branch(C::Eq, 0x8118c, 0x811b0, false)?)?;
    c.emit(0x81190, data_register(C::Al, Op::Mov, r(1), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x81194, branch(C::Al, 0x81194, 0x81088, true)?)?;
    c.emit(0x81198, branch(C::Al, 0x81198, 0x811cc, false)?)?;
    c.literal(0x8119c, 0, 0x811d4)?;
    c.literal(0x811a0, 1, 0x811d8)?;
    c.literal(0x811a4, 3, 0x811dc)?;
    c.emit(0x811a8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1b8)?)?;
    c.emit(0x811ac, branch(C::Al, 0x811ac, 0x811c4, false)?)?;
    c.literal(0x811b0, 0, 0x811d4)?;
    c.literal(0x811b4, 1, 0x811d8)?;
    c.literal(0x811b8, 3, 0x811dc)?;
    c.emit(0x811bc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xbf)?)?;
    c.emit(0x811c0, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x811c4, branch(C::Al, 0x811c4, 0x83d2c, true)?)?;
    c.emit(0x811c8, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x811cc, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x811d0, transfer_multiple(C::Al, true, r(13), 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.word(0x811d4, 0xf4bef, true)?;
    c.word(0x811d8, 0xf4bf1, true)?;
    c.word(0x811dc, 0xf4ce2, true)?;
    Ok(())
}

fn function_811e0(c: &mut Code) -> Result<(), String> {
    c.emit(0x811e0, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x811e4, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x811e8, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x811ec, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x811f0, branch(C::Eq, 0x811f0, 0x81228, false)?)?;
    c.emit(0x811f4, transfer_word(C::Al, true, r(5), r(4), 0x4)?)?;
    c.emit(0x811f8, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x811fc, branch(C::Ne, 0x811fc, 0x81218, false)?)?;
    c.literal(0x81200, 0, 0x8124c)?;
    c.literal(0x81204, 1, 0x81250)?;
    c.literal(0x81208, 3, 0x81254)?;
    c.emit(0x8120c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1e0)?)?;
    c.emit(0x81210, branch(C::Al, 0x81210, 0x83d2c, true)?)?;
    c.emit(0x81214, transfer_word(C::Al, true, r(5), r(4), 0x4)?)?;
    c.emit(0x81218, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x8121c, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x81220, branch(C::Al, 0x81220, 0x81088, true)?)?;
    c.emit(0x81224, branch(C::Al, 0x81224, 0x81244, false)?)?;
    c.literal(0x81228, 0, 0x81258)?;
    c.literal(0x8122c, 1, 0x81250)?;
    c.literal(0x81230, 3, 0x81254)?;
    c.emit(0x81234, data_immediate(C::Al, Op::Mov, r(2), r(0), 0xdb)?)?;
    c.emit(0x81238, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x100)?)?;
    c.emit(0x8123c, branch(C::Al, 0x8123c, 0x83d2c, true)?)?;
    c.emit(0x81240, data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(0x81244, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x81248, transfer_multiple(C::Al, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.word(0x8124c, 0xf4d02, true)?;
    c.word(0x81250, 0xf4bf1, true)?;
    c.word(0x81254, 0xf4cf2, true)?;
    c.word(0x81258, 0xf4bef, true)?;
    Ok(())
}

fn function_8125c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8125c, transfer_multiple(C::Al, false, r(13), 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81260, data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.emit(0x81264, data_register(C::Al, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x81268, data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8126c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81270, branch(C::Eq, 0x81270, 0x812a4, false)?)?;
    c.emit(0x81274, transfer_word(C::Al, true, r(7), r(0), 0x0)?)?;
    c.emit(0x81278, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x8127c, branch(C::Al, 0x8127c, 0x81298, false)?)?;
    c.emit(0x81280, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x81284, data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x81288, branch_exchange_with_link(C::Al, r(5))?)?;
    c.emit(0x8128c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81290, branch(C::Ne, 0x81290, 0x812c4, false)?)?;
    c.emit(0x81294, transfer_word(C::Al, true, r(7), r(7), 0x4)?)?;
    c.emit(0x81298, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(0x8129c, branch(C::Ne, 0x8129c, 0x81280, false)?)?;
    c.emit(0x812a0, branch(C::Al, 0x812a0, 0x812c8, false)?)?;
    c.literal(0x812a4, 0, 0x812d0)?;
    c.literal(0x812a8, 1, 0x812d4)?;
    c.literal(0x812ac, 3, 0x812d8)?;
    c.emit(0x812b0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.emit(0x812b4, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x812b8, branch(C::Al, 0x812b8, 0x83d2c, true)?)?;
    c.emit(0x812bc, data_immediate(C::Al, Op::Mov, r(4), r(0), 0x0)?)?;
    c.emit(0x812c0, branch(C::Al, 0x812c0, 0x812c8, false)?)?;
    c.emit(0x812c4, data_register(C::Al, Op::Mov, r(4), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x812c8, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x812cc, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x812d0, 0xf4bef, true)?;
    c.word(0x812d4, 0xf4bf1, true)?;
    c.word(0x812d8, 0xf4d21, true)?;
    Ok(())
}

fn function_812dc(c: &mut Code) -> Result<(), String> {
    c.emit(0x812dc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x812e0, transfer_word(C::Ne, true, r(0), r(0), 0x0)?)?;
    c.emit(0x812e4, bx(C::Ne, r(14)))?;
    c.emit(0x812e8, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x812ec, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x812f0, 0, 0x81310)?;
    c.literal(0x812f4, 1, 0x81314)?;
    c.literal(0x812f8, 3, 0x81318)?;
    c.emit(0x812fc, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2f)?)?;
    c.emit(0x81300, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x81304, branch(C::Al, 0x81304, 0x83d2c, true)?)?;
    c.emit(0x81308, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8130c, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x81310, 0xf4bef, true)?;
    c.word(0x81314, 0xf4bf1, true)?;
    c.word(0x81318, 0xf4d31, true)?;
    Ok(())
}

fn function_8131c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8131c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81320, transfer_word(C::Ne, true, r(0), r(0), 0x4)?)?;
    c.emit(0x81324, bx(C::Ne, r(14)))?;
    c.emit(0x81328, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8132c, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x81330, 0, 0x8134c)?;
    c.literal(0x81334, 1, 0x81350)?;
    c.literal(0x81338, 3, 0x81354)?;
    c.emit(0x8133c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x248)?)?;
    c.emit(0x81340, branch(C::Al, 0x81340, 0x83d2c, true)?)?;
    c.emit(0x81344, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x81348, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x8134c, 0xf4bef, true)?;
    c.word(0x81350, 0xf4bf1, true)?;
    c.word(0x81354, 0xf4d3e, true)?;
    Ok(())
}

fn function_81358(c: &mut Code) -> Result<(), String> {
    c.emit(0x81358, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x8135c, transfer_word(C::Ne, true, r(0), r(0), 0x4)?)?;
    c.emit(0x81360, bx(C::Ne, r(14)))?;
    c.emit(0x81364, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81368, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x8136c, 0, 0x8138c)?;
    c.literal(0x81370, 1, 0x81390)?;
    c.literal(0x81374, 3, 0x81394)?;
    c.emit(0x81378, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x5f)?)?;
    c.emit(0x8137c, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x81380, branch(C::Al, 0x81380, 0x83d2c, true)?)?;
    c.emit(0x81384, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x81388, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x8138c, 0xf4bef, true)?;
    c.word(0x81390, 0xf4bf1, true)?;
    c.word(0x81394, 0xf4d4b, true)?;
    Ok(())
}

fn function_81398(c: &mut Code) -> Result<(), String> {
    c.emit(0x81398, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x8139c, transfer_word(C::Ne, true, r(0), r(0), 0x8)?)?;
    c.emit(0x813a0, bx(C::Ne, r(14)))?;
    c.emit(0x813a4, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x813a8, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x813ac, 0, 0x813cc)?;
    c.literal(0x813b0, 1, 0x813d0)?;
    c.literal(0x813b4, 3, 0x813d4)?;
    c.emit(0x813b8, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x76)?)?;
    c.emit(0x813bc, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x813c0, branch(C::Al, 0x813c0, 0x83d2c, true)?)?;
    c.emit(0x813c4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x813c8, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x813cc, 0xf4bef, true)?;
    c.word(0x813d0, 0xf4bf1, true)?;
    c.word(0x813d4, 0xf4d5c, true)?;
    Ok(())
}

fn function_813d8(c: &mut Code) -> Result<(), String> {
    c.emit(0x813d8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x813dc, branch(C::Eq, 0x813dc, 0x813fc, false)?)?;
    c.emit(0x813e0, transfer_word(C::Al, true, r(1), r(0), 0x0)?)?;
    c.emit(0x813e4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x813e8, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x813ec, bx(C::Eq, r(14)))?;
    c.emit(0x813f0, transfer_word(C::Al, true, r(1), r(1), 0x4)?)?;
    c.emit(0x813f4, data_immediate(C::Al, Op::Add, r(0), r(0), 0x1)?)?;
    c.emit(0x813f8, branch(C::Al, 0x813f8, 0x813e8, false)?)?;
    c.emit(0x813fc, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81400, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.literal(0x81404, 0, 0x81424)?;
    c.literal(0x81408, 1, 0x81428)?;
    c.literal(0x8140c, 3, 0x8142c)?;
    c.emit(0x81410, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8f)?)?;
    c.emit(0x81414, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x200)?)?;
    c.emit(0x81418, branch(C::Al, 0x81418, 0x83d2c, true)?)?;
    c.emit(0x8141c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x81420, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.word(0x81424, 0xf4bef, true)?;
    c.word(0x81428, 0xf4bf1, true)?;
    c.word(0x8142c, 0xf4d6d, true)?;
    Ok(())
}

fn function_81430(c: &mut Code) -> Result<(), String> {
    c.emit(0x81430, data_immediate(C::Al, Op::Add, r(0), r(0), 0x18)?)?;
    c.emit(0x81434, branch(C::Al, 0x81434, 0x80c18, false)?)?;
    Ok(())
}

fn function_81438(c: &mut Code) -> Result<(), String> {
    c.emit(0x81438, data_immediate(C::Al, Op::Add, r(0), r(0), 0x20)?)?;
    c.emit(0x8143c, branch(C::Al, 0x8143c, 0x80c18, false)?)?;
    Ok(())
}

fn function_81440(c: &mut Code) -> Result<(), String> {
    c.emit(0x81440, transfer_multiple(C::Al, false, r(13), 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81444, data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.emit(0x81448, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8144c, data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.emit(0x81450, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x81454, branch(C::Eq, 0x81454, 0x81528, false)?)?;
    c.emit(0x81458, transfer_word(C::Al, true, r(0), r(1), 0x0)?)?;
    c.emit(0x8145c, transfer_word(C::Al, true, r(8), r(11), 0x8)?)?;
    c.emit(0x81460, transfer_word(C::Al, false, r(0), r(4), 0x0)?)?;
    c.emit(0x81464, transfer_word(C::Al, true, r(0), r(1), 0x4)?)?;
    c.emit(0x81468, transfer_word(C::Al, false, r(0), r(4), 0x4)?)?;
    c.emit(0x8146c, transfer_word(C::Al, true, r(0), r(1), 0x8)?)?;
    c.emit(0x81470, transfer_word(C::Al, false, r(0), r(4), 0x8)?)?;
    c.emit(0x81474, transfer_word(C::Al, true, r(0), r(1), 0xc)?)?;
    c.emit(0x81478, data_immediate(C::Al, Op::Add, r(1), r(4), 0xc)?)?;
    c.emit(0x8147c, transfer_multiple(C::Al, false, r(1), 0xd, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x81480, data_immediate(C::Al, Op::Add, r(0), r(4), 0x18)?)?;
    c.emit(0x81484, branch(C::Al, 0x81484, 0x80c64, true)?)?;
    c.emit(0x81488, data_immediate(C::Al, Op::Add, r(6), r(4), 0x20)?)?;
    c.emit(0x8148c, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x81490, branch(C::Al, 0x81490, 0x80c64, true)?)?;
    c.emit(0x81494, data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x0)?)?;
    c.emit(0x81498, branch(C::Eq, 0x81498, 0x81528, false)?)?;
    c.emit(0x8149c, transfer_word(C::Al, true, r(0), r(4), 0x14)?)?;
    c.emit(0x814a0, transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.emit(0x814a4, data_immediate(C::Al, Op::Add, r(9), r(0), 0xf)?)?;
    c.emit(0x814a8, data_immediate(C::Al, Op::Bic, r(7), r(9), 0x3)?)?;
    c.emit(0x814ac, multiply_register(C::Al, r(0), r(7), r(8))?)?;
    c.emit(0x814b0, branch_exchange_with_link(C::Al, r(1))?)?;
    c.emit(0x814b4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x814b8, transfer_word(C::Al, false, r(0), r(4), 0x28)?)?;
    c.emit(0x814bc, branch(C::Eq, 0x814bc, 0x81528, false)?)?;
    c.emit(0x814c0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x814c4, data_immediate_with_flags(C::Al, Op::Sub, r(2), r(8), 0x1, true)?)?;
    c.emit(0x814c8, transfer_word(C::Al, false, r(0), r(6), 0x0)?)?;
    c.emit(0x814cc, transfer_word(C::Al, false, r(1), r(0), 0x8)?)?;
    c.emit(0x814d0, branch(C::Eq, 0x814d0, 0x81510, false)?)?;
    c.emit(0x814d4, data_immediate(C::Al, Op::Mvn, r(3), r(0), 0x3)?)?;
    c.emit(0x814d8, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x814dc, data_register(C::Al, Op::Add, r(12), r(3), r(8), Shift::Lsl, 2, false)?)?;
    c.emit(0x814e0, data_immediate(C::Al, Op::Add, r(5), r(6), 0xc)?)?;
    c.emit(0x814e4, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x814e8, data_register(C::Al, Op::Add, r(3), r(6), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x814ec, transfer_word(C::Al, false, r(5), r(6), 0x0)?)?;
    c.emit(0x814f0, transfer_word(C::Al, false, r(3), r(6), 0x4)?)?;
    c.emit(0x814f4, transfer_word(C::Al, false, r(6), r(3), 0x8)?)?;
    c.emit(0x814f8, data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x814fc, data_register(C::Al, Op::Mov, r(6), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x81500, branch(C::Ne, 0x81500, 0x814e0, false)?)?;
    c.emit(0x81504, data_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsr, 2, false)?)?;
    c.emit(0x81508, multiply_accumulate(C::Al, r(1), r(2), r(12), r(0))?)?;
    c.emit(0x8150c, branch(C::Al, 0x8150c, 0x81514, false)?)?;
    c.emit(0x81510, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x81514, data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.emit(0x81518, data_immediate(C::Al, Op::Add, r(0), r(1), 0xc)?)?;
    c.emit(0x8151c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x81520, transfer_multiple(C::Al, false, r(1), 0x5, BlockMode::IncrementAfter, false)?)?;
    c.emit(0x81524, transfer_word(C::Al, false, r(1), r(4), 0x24)?)?;
    c.emit(0x81528, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x8152c, transfer_multiple(C::Al, true, r(13), 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_81530(c: &mut Code) -> Result<(), String> {
    c.emit(0x81530, transfer_multiple(C::Al, false, r(13), 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81534, data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.emit(0x81538, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8153c, data_register(C::Al, Op::Mov, r(4), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x81540, data_immediate(C::Al, Op::Add, r(6), r(5), 0x20)?)?;
    c.emit(0x81544, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x81548, branch(C::Al, 0x81548, 0x80c18, true)?)?;
    c.emit(0x8154c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81550, branch(C::Eq, 0x81550, 0x81564, false)?)?;
    c.literal(0x81554, 2, 0x815a0)?;
    c.emit(0x81558, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8155c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x98)?)?;
    c.emit(0x81560, branch(C::Al, 0x81560, 0x17b0, true)?)?;
    c.emit(0x81564, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x81568, branch(C::Al, 0x81568, 0x81174, true)?)?;
    c.literal(0x8156c, 2, 0x815a0)?;
    c.emit(0x81570, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x81574, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xa2)?)?;
    c.emit(0x81578, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.emit(0x8157c, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x81580, branch(C::Al, 0x81580, 0x17b0, true)?)?;
    c.emit(0x81584, transfer_word(C::Al, true, r(3), r(5), 0x10)?)?;
    c.emit(0x81588, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x8158c, data_register(C::Al, Op::Mov, r(1), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x81590, data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x81594, branch_exchange_with_link(C::Al, r(3))?)?;
    c.emit(0x81598, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x8159c, transfer_multiple(C::Al, true, r(13), 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x815a0, 0xf4d9b, true)?;
    Ok(())
}

fn function_815a4(c: &mut Code) -> Result<(), String> {
    c.emit(0x815a4, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x815a8, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x815ac, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x815b0, data_immediate(C::Al, Op::Add, r(0), r(4), 0x18)?)?;
    c.emit(0x815b4, branch(C::Al, 0x815b4, 0x81174, true)?)?;
    c.literal(0x815b8, 2, 0x815e8)?;
    c.emit(0x815bc, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x815c0, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xbe)?)?;
    c.emit(0x815c4, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x815c8, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x815cc, branch(C::Al, 0x815cc, 0x17b0, true)?)?;
    c.emit(0x815d0, data_immediate(C::Al, Op::Add, r(0), r(4), 0x20)?)?;
    c.emit(0x815d4, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x815d8, branch(C::Al, 0x815d8, 0x80d78, true)?)?;
    c.emit(0x815dc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x815e0, transfer_multiple(C::Al, true, r(13), 0x4830, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x815e4, branch(C::Al, 0x815e4, 0x80cc4, false)?)?;
    c.word(0x815e8, 0xf4d9b, true)?;
    Ok(())
}

fn function_815ec(c: &mut Code) -> Result<(), String> {
    c.emit(0x815ec, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x815f0, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x815f4, data_immediate(C::Al, Op::Add, r(0), r(0), 0x18)?)?;
    c.emit(0x815f8, branch(C::Al, 0x815f8, 0x812dc, true)?)?;
    c.literal(0x815fc, 2, 0x81620)?;
    c.emit(0x81600, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x81604, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xd6)?)?;
    c.emit(0x81608, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x8160c, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x81610, branch(C::Al, 0x81610, 0x17b0, true)?)?;
    c.emit(0x81614, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x81618, transfer_multiple(C::Al, true, r(13), 0x4890, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x8161c, branch(C::Al, 0x8161c, 0x80cc4, false)?)?;
    c.word(0x81620, 0xf4d9b, true)?;
    Ok(())
}

fn function_81624(c: &mut Code) -> Result<(), String> {
    c.emit(0x81624, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81628, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x8162c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x18)?)?;
    c.emit(0x81630, branch(C::Al, 0x81630, 0x8131c, true)?)?;
    c.literal(0x81634, 2, 0x81658)?;
    c.emit(0x81638, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8163c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xed)?)?;
    c.emit(0x81640, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x81644, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x81648, branch(C::Al, 0x81648, 0x17b0, true)?)?;
    c.emit(0x8164c, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x81650, transfer_multiple(C::Al, true, r(13), 0x4890, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81654, branch(C::Al, 0x81654, 0x80cc4, false)?)?;
    c.word(0x81658, 0xf4d9b, true)?;
    Ok(())
}

fn function_8165c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8165c, data_immediate(C::Al, Op::Add, r(0), r(0), 0x18)?)?;
    c.emit(0x81660, branch(C::Al, 0x81660, 0x813d8, false)?)?;
    Ok(())
}

fn function_81664(c: &mut Code) -> Result<(), String> {
    c.emit(0x81664, data_immediate(C::Al, Op::Add, r(0), r(0), 0x18)?)?;
    c.emit(0x81668, branch(C::Al, 0x81668, 0x8125c, false)?)?;
    Ok(())
}

fn function_8166c(c: &mut Code) -> Result<(), String> {
    c.emit(0x8166c, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x81670, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x81674, data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x81678, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8167c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x81680, branch(C::Ne, 0x81680, 0x816ac, false)?)?;
    c.emit(0x81684, data_immediate(C::Al, Op::Add, r(0), r(4), 0x18)?)?;
    c.emit(0x81688, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x8168c, data_register(C::Al, Op::Mov, r(2), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x81690, branch(C::Al, 0x81690, 0x8125c, true)?)?;
    c.literal(0x81694, 2, 0x816c8)?;
    c.emit(0x81698, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8169c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x13c)?)?;
    c.emit(0x816a0, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.emit(0x816a4, data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.emit(0x816a8, branch(C::Al, 0x816a8, 0x17b0, true)?)?;
    c.emit(0x816ac, data_immediate(C::Al, Op::Add, r(0), r(4), 0x18)?)?;
    c.emit(0x816b0, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x816b4, branch(C::Al, 0x816b4, 0x81088, true)?)?;
    c.emit(0x816b8, data_immediate(C::Al, Op::Add, r(0), r(4), 0x20)?)?;
    c.emit(0x816bc, data_register(C::Al, Op::Mov, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x816c0, transfer_multiple(C::Al, true, r(13), 0x4830, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x816c4, branch(C::Al, 0x816c4, 0x80d78, false)?)?;
    c.word(0x816c8, 0xf4d9b, true)?;
    Ok(())
}

fn function_816cc(c: &mut Code) -> Result<(), String> {
    c.emit(0x816cc, transfer_multiple(C::Al, false, r(13), 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x816d0, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x816d4, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x816d8, data_immediate(C::Al, Op::Add, r(0), r(4), 0x18)?)?;
    c.emit(0x816dc, branch(C::Al, 0x816dc, 0x80c64, true)?)?;
    c.emit(0x816e0, data_immediate(C::Al, Op::Add, r(0), r(4), 0x20)?)?;
    c.emit(0x816e4, branch(C::Al, 0x816e4, 0x80c64, true)?)?;
    c.emit(0x816e8, transfer_word(C::Al, true, r(1), r(4), 0x8)?)?;
    c.emit(0x816ec, transfer_word(C::Al, true, r(0), r(4), 0x28)?)?;
    c.emit(0x816f0, transfer_multiple(C::Al, true, r(13), 0x4890, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x816f4, bx(C::Al, r(1)))?;
    Ok(())
}

fn function_816f8(c: &mut Code) -> Result<(), String> {
    c.emit(0x816f8, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x816fc, branch_exchange_immediate(0x816fc, 0x822e0)?)?;
    c.emit(0x81700, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81704, branch(C::Eq, 0x81704, 0x81710, false)?)?;
    c.emit(0x81708, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, true)?)?;
    c.emit(0x8170c, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81710, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, true)?)?;
    c.emit(0x81714, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_81718(c: &mut Code) -> Result<(), String> {
    c.emit(0x81718, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8171c, branch_exchange_immediate(0x8171c, 0x822e8)?)?;
    c.emit(0x81720, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81724, branch(C::Lt, 0x81724, 0x81730, false)?)?;
    c.emit(0x81728, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, true)?)?;
    c.emit(0x8172c, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81730, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, true)?)?;
    c.emit(0x81734, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_81738(c: &mut Code) -> Result<(), String> {
    c.emit(0x81738, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8173c, branch_exchange_immediate(0x8173c, 0x82100)?)?;
    c.emit(0x81740, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81744, branch(C::Le, 0x81744, 0x81750, false)?)?;
    c.emit(0x81748, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, true)?)?;
    c.emit(0x8174c, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81750, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, true)?)?;
    c.emit(0x81754, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_81758(c: &mut Code) -> Result<(), String> {
    c.emit(0x81758, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8175c, branch_exchange_immediate(0x8175c, 0x821fc)?)?;
    c.emit(0x81760, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81764, branch(C::Ge, 0x81764, 0x81770, false)?)?;
    c.emit(0x81768, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, true)?)?;
    c.emit(0x8176c, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81770, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, true)?)?;
    c.emit(0x81774, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_81778(c: &mut Code) -> Result<(), String> {
    c.emit(0x81778, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8177c, branch_exchange_immediate(0x8177c, 0x822f0)?)?;
    c.emit(0x81780, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x81784, branch(C::Gt, 0x81784, 0x81790, false)?)?;
    c.emit(0x81788, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, true)?)?;
    c.emit(0x8178c, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x81790, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, true)?)?;
    c.emit(0x81794, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn function_81798(c: &mut Code) -> Result<(), String> {
    c.emit(0x81798, transfer_multiple(C::Al, false, r(13), 0x4010, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8179c, branch_exchange_immediate(0x8179c, 0x82398)?)?;
    c.emit(0x817a0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x817a4, branch(C::Eq, 0x817a4, 0x817b0, false)?)?;
    c.emit(0x817a8, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x0, true)?)?;
    c.emit(0x817ac, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x817b0, data_immediate_with_flags(C::Al, Op::Mov, r(0), r(0), 0x1, true)?)?;
    c.emit(0x817b4, transfer_multiple(C::Al, true, r(13), 0x8010, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    function_80c18(&mut c)?;
    function_80c64(&mut c)?;
    function_80c98(&mut c)?;
    function_80cc4(&mut c)?;
    function_80d00(&mut c)?;
    function_80d78(&mut c)?;
    function_80df8(&mut c)?;
    function_80ef8(&mut c)?;
    function_80f88(&mut c)?;
    function_81088(&mut c)?;
    function_81174(&mut c)?;
    function_811e0(&mut c)?;
    function_8125c(&mut c)?;
    function_812dc(&mut c)?;
    function_8131c(&mut c)?;
    function_81358(&mut c)?;
    function_81398(&mut c)?;
    function_813d8(&mut c)?;
    function_81430(&mut c)?;
    function_81438(&mut c)?;
    function_81440(&mut c)?;
    function_81530(&mut c)?;
    function_815a4(&mut c)?;
    function_815ec(&mut c)?;
    function_81624(&mut c)?;
    function_8165c(&mut c)?;
    function_81664(&mut c)?;
    function_8166c(&mut c)?;
    function_816cc(&mut c)?;
    function_816f8(&mut c)?;
    function_81718(&mut c)?;
    function_81738(&mut c)?;
    function_81758(&mut c)?;
    function_81778(&mut c)?;
    function_81798(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != END - START || c.code_bytes != CODE_BYTES || c.data_bytes != DATA_BYTES {
        return Err("invalid runtime code/data extent".into());
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
    fn extents_pools_and_function_boundaries() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, FILE_MAPPING_DELTA), (0x80c18, 0x817b8, 0x4000));
        assert_eq!(bytes.len(), 2976);
        assert_eq!((CODE_BYTES, DATA_BYTES), (2716, 260));
        assert_eq!(CODE_BYTES + DATA_BYTES, END - START);
        assert_eq!(LITERALS.len() * 4, DATA_BYTES);
        assert!(LITERALS.windows(2).all(|pair| pair[0].0 < pair[1].0));
        assert!(FUNCTIONS.windows(2).all(|pair| pair[0] < pair[1]));
        for (slot, value) in LITERALS {
            assert_eq!(slot % 4, 0);
            assert!((START..END).contains(&slot));
            assert_eq!(word(&bytes, slot), value, "pool {slot:#x}");
        }
        let functions: [fn(&mut Code) -> Result<(), String>; 35] = [
            function_80c18,
            function_80c64,
            function_80c98,
            function_80cc4,
            function_80d00,
            function_80d78,
            function_80df8,
            function_80ef8,
            function_80f88,
            function_81088,
            function_81174,
            function_811e0,
            function_8125c,
            function_812dc,
            function_8131c,
            function_81358,
            function_81398,
            function_813d8,
            function_81430,
            function_81438,
            function_81440,
            function_81530,
            function_815a4,
            function_815ec,
            function_81624,
            function_8165c,
            function_81664,
            function_8166c,
            function_816cc,
            function_816f8,
            function_81718,
            function_81738,
            function_81758,
            function_81778,
            function_81798,
        ];
        let mut c = Code::new(START);
        for (index, function) in functions.into_iter().enumerate() {
            let start = FUNCTIONS[index];
            let end = FUNCTIONS.get(index + 1).copied().unwrap_or(END);
            assert_eq!(c.at, start);
            assert!(!LITERALS.iter().any(|&(slot, _)| slot == start));
            let previous_code = c.code_bytes;
            let previous_data = c.data_bytes;
            function(&mut c).unwrap();
            let data = LITERALS.iter().filter(|&&(slot, _)| (start..end).contains(&slot)).count() * 4;
            assert_eq!(c.at, end, "function {start:#x}");
            assert_eq!(c.data_bytes - previous_data, data);
            assert_eq!(c.code_bytes - previous_code, end - start - data);
            assert_eq!(c.bytes.len(), end - START);
        }
        assert_eq!((c.at, c.code_bytes, c.data_bytes), (END, CODE_BYTES, DATA_BYTES));
        assert_eq!(c.bytes, bytes);
        assert_eq!(crate::wmfw_sha256::hex_digest(&bytes),
            "11c9c4019aaab393e8902030ae53ae5c3eda1203d5014acb3b2477ad26a2c4e2");
    }

    #[test]
    fn literal_loads_decode_to_declared_pools() {
        let bytes = fragment().unwrap();
        assert_eq!(LITERAL_REFERENCES.len(), 99);
        for (index, &(at, slot)) in LITERAL_REFERENCES.iter().enumerate() {
            assert!(!LITERAL_REFERENCES[..index].iter().any(|&(site, _)| site == at));
            assert!(LITERALS.iter().any(|&(pool, _)| pool == slot));
            assert!(!LITERALS.iter().any(|&(pool, _)| pool == at));
            let instruction = word(&bytes, at);
            assert_eq!(instruction & 0xffff_0000, 0xe59f_0000, "literal {at:#x}");
            assert_eq!(at + 8 + (instruction & 0xfff) as usize, slot);
            let mut c = Code::new(at);
            c.literal(at, ((instruction >> 12) & 15) as u8, slot).unwrap();
            assert_eq!(c.bytes, instruction.to_le_bytes());
            assert_eq!((c.at, c.code_bytes, c.data_bytes), (at + 4, 4, 0));
        }
        for (slot, _) in LITERALS {
            assert!(LITERAL_REFERENCES.iter().any(|&(_, pool)| pool == slot));
        }
    }

    #[test]
    fn architectural_flags_writeback_multiply_and_blx_words() {
        let bytes = fragment().unwrap();
        for (at, expected) in [
            (0x80c18, 0xe3500000),
            (0x80c1c, 0x0a000004),
            (0x80c34, 0xe92d4800),
            (0x80c54, 0xe8bd8800),
            (0x80d1c, 0x1981000c),
            (0x8111c, 0x19910005),
            (0x81288, 0xe12fff35),
            (0x8147c, 0xe881000d),
            (0x814ac, 0xe0000897),
            (0x814b0, 0xe12fff31),
            (0x814c4, 0xe2582001),
            (0x81508, 0xe0210c92),
            (0x81520, 0xe8810005),
            (0x81594, 0xe12fff33),
        ] {
            assert_eq!(word(&bytes, at), expected, "instruction {at:#x}");
        }
        for (start, blx, condition) in [
            (0x816f8, 0xfa0002f7, 0x0),
            (0x81718, 0xfa0002f1, 0xb),
            (0x81738, 0xfa00026f, 0xd),
            (0x81758, 0xfa0002a6, 0xa),
            (0x81778, 0xfa0002db, 0xc),
            (0x81798, 0xfa0002fd, 0x0),
        ] {
            assert_eq!(word(&bytes, start), 0xe92d4010);
            assert_eq!(word(&bytes, start + 4), blx);
            assert_eq!(word(&bytes, start + 8), 0xe3500000);
            assert_eq!(word(&bytes, start + 12), (condition << 28) | 0x0a000001);
            assert_eq!(word(&bytes, start + 16), 0xe3b00000);
            assert_eq!(word(&bytes, start + 20), 0xe8bd8010);
            assert_eq!(word(&bytes, start + 24), 0xe3b00001);
            assert_eq!(word(&bytes, start + 28), 0xe8bd8010);
        }
    }

    #[test]
    fn exact_code_data_guards_do_not_mutate_on_failure() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let pool = LITERALS.iter().find(|&&(slot, _)| slot == at);
            for data in [false, true] {
                for value in [0, pool.map_or(0, |&(_, value)| value), pool.map_or(1, |&(_, value)| value ^ 1)] {
                    let valid = (START..END).contains(&at) && at % 4 == 0
                        && if data { pool.is_some_and(|&(_, expected)| expected == value) } else { pool.is_none() };
                    let mut c = Code::new(at);
                    assert_eq!(c.word(at, value, data).is_ok(), valid, "{at:#x} data={data}");
                    assert_eq!(c.at, at + if valid { 4 } else { 0 });
                    assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
                    assert_eq!(c.code_bytes, 4 * usize::from(valid && !data));
                    assert_eq!(c.data_bytes, 4 * usize::from(valid && data));
                }
            }
        }
        let mut c = Code::new(START);
        c.emit(START, 0xe3500000).unwrap();
        let before = (c.at, c.bytes.clone(), c.code_bytes, c.data_bytes);
        assert!(c.emit(START, 0).is_err());
        assert!(c.word(START + 4, 0, true).is_err());
        assert!(c.check(END).is_err());
        assert_eq!((c.at, c.bytes, c.code_bytes, c.data_bytes), before);
    }

    #[test]
    fn rejects_literal_sites_and_unencodable_operands() {
        for (at, slot) in [
            (START, 0x80c58),
            (0x80c3c, 0x80c5c),
            (0x80c3c, 0x80c59),
            (0x80c3c, END),
            (0x80c58, 0x80c58),
            (END, 0x80c58),
        ] {
            let mut c = Code::new(at);
            assert!(c.literal(at, 0, slot).is_err());
            assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes), (at, 0, 0, 0));
        }
        let mut c = Code::new(0x80c38);
        assert!(c.literal(0x80c3c, 0, 0x80c58).is_err());
        assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes), (0x80c38, 0, 0, 0));
        assert!(R::new(16).is_err());
        assert!(R::new(u8::MAX).is_err());
        assert!(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x12345678).is_err());
        assert!(data_immediate_with_flags(C::Al, Op::Cmp, r(0), r(0), 0, true).is_err());
        assert!(data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 32, false).is_err());
        assert!(transfer_word(C::Al, true, r(0), R::PC, 4096).is_err());
        assert!(transfer_word(C::Al, true, r(0), R::PC, -4096).is_err());
        assert!(transfer_multiple(C::Al, false, R::SP, 0, BlockMode::DecrementBefore, true).is_err());
        assert!(branch_exchange_with_link(C::Al, R::PC).is_err());
        assert!(branch(C::Al, START as u32, START as u32 + 2, true).is_err());
        assert!(branch_exchange_immediate(START as u32, START as u32 + 1).is_err());
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
