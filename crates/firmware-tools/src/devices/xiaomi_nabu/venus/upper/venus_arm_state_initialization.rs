use arm32::{
    branch, bx, data_immediate, data_register, data_register_shifted_by_register,
    multiply_register, transfer_byte, transfer_byte_postindexed, transfer_multiple,
    transfer_word, transfer_word_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x850d0;
pub const END: usize = 0x85824;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1744;
pub const DATA_BYTES: usize = 132;

const LITERALS: [(usize, u32); 33] = [
    (0x85100, 0x10b530),
    (0x85110, 0x10b531),
    (0x85130, 0x10b530),
    (0x85134, 0x10b534),
    (0x85154, 0x10b530),
    (0x85158, 0x10b534),
    (0x85198, 0xf4dfc),
    (0x851fc, 0x10b530),
    (0x85200, 0x10b534),
    (0x85274, 0x10b530),
    (0x85278, 0x10b534),
    (0x852a4, 0x10b530),
    (0x852a8, 0x10b534),
    (0x852f0, 0x10b530),
    (0x852f4, 0x10b534),
    (0x85338, 0x10b534),
    (0x853ec, 0x10b534),
    (0x853f0, 0x10b530),
    (0x853f4, 0x10b568),
    (0x8559c, 0x10b530),
    (0x855a0, 0x10b52c),
    (0x855a4, 0x10b534),
    (0x855a8, 0x10b568),
    (0x8564c, 0x10b52c),
    (0x85650, 0x10b534),
    (0x856f4, 0x10b534),
    (0x857bc, 0x10b564),
    (0x857c0, 0x10b568),
    (0x857c4, 0x10b530),
    (0x857c8, 0x10b534),
    (0x85818, 0x10b568),
    (0x8581c, 0x10b530),
    (0x85820, 0x10b531),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("ARM state address {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, at: usize, value: u32, data: bool) -> Result<(), String> {
        self.check(at)?;
        let literal = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, v)| v);
        if !(START..END).contains(&at) || at % 4 != 0
            || if data { literal != Some(value) } else { literal.is_some() }
        {
            return Err(format!("invalid ARM state partition at {at:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.word(at, instruction, false)
    }

    fn literal(&mut self, at: usize, rd: R, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| address == slot) {
            return Err(format!("invalid ARM state literal {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, rd, R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn small_accessors(c: &mut Code) -> Result<(), String> {
    c.emit(0x850d0, data_immediate(C::Al, Op::Sub, r(13), r(13), 4)?)?;
    c.emit(0x850d4, transfer_word(C::Al, false, r(0), r(13), 0)?)?;
    c.emit(0x850d8, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x850dc, data_immediate(C::Al, Op::Add, r(13), r(13), 4)?)?;
    c.emit(0x850e0, bx(C::Al, r(14)))?;
    c.emit(0x850e4, bx(C::Al, r(14)))?;
    c.emit(0x850e8, bx(C::Al, r(14)))?;
    c.emit(0x850ec, data_immediate(C::Al, Op::Mov, r(0), r(0), 128)?)?;
    c.emit(0x850f0, bx(C::Al, r(14)))?;
    c.literal(0x850f4, r(0), 0x85100)?;
    c.emit(0x850f8, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x850fc, bx(C::Al, r(14)))?;
    c.word(0x85100, 0x10b530, true)?;
    c.literal(0x85104, r(0), 0x85110)?;
    c.emit(0x85108, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x8510c, bx(C::Al, r(14)))?;
    c.word(0x85110, 0x10b531, true)?;
    c.literal(0x85114, r(0), 0x85130)?;
    c.emit(0x85118, transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.literal(0x8511c, r(0), 0x85134)?;
    c.emit(0x85120, transfer_word(C::Al, true, r(0), r(0), 36)?)?;
    c.emit(0x85124, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x85128, data_register(C::Eq, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8512c, bx(C::Al, r(14)))?;
    c.word(0x85130, 0x10b530, true)?;
    c.word(0x85134, 0x10b534, true)?;
    c.literal(0x85138, r(0), 0x85154)?;
    c.emit(0x8513c, transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.literal(0x85140, r(0), 0x85158)?;
    c.emit(0x85144, transfer_word(C::Al, true, r(0), r(0), 44)?)?;
    c.emit(0x85148, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x8514c, data_register(C::Eq, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85150, bx(C::Al, r(14)))?;
    c.word(0x85154, 0x10b530, true)?;
    c.word(0x85158, 0x10b534, true)?;
    Ok(())
}

fn copy_identifier(c: &mut Code) -> Result<(), String> {
    c.emit(0x8515c, transfer_multiple(C::Al, false, R::SP, 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85160, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.emit(0x85164, data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85168, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8516c, data_immediate(C::Al, Op::Cmp, r(0), r(2), 14)?)?;
    c.emit(0x85170, branch(C::Cc, 0x85170, 0x85184, false)?)?;
    c.emit(0x85174, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85178, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x8517c, branch(C::Al, 0x8517c, 0x31ec, true)?)?;
    c.emit(0x85180, data_immediate(C::Al, Op::Mov, r(2), r(0), 13)?)?;
    c.literal(0x85184, r(1), 0x85198)?;
    c.emit(0x85188, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x8518c, branch(C::Al, 0x8518c, 0x471c, true)?)?;
    c.emit(0x85190, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85194, transfer_multiple(C::Al, true, R::SP, 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.word(0x85198, 0xf4dfc, true)?;
    Ok(())
}

fn copy_state_mask(c: &mut Code) -> Result<(), String> {
    c.emit(0x8519c, transfer_multiple(C::Al, false, R::SP, 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x851a0, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.emit(0x851a4, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.literal(0x851a8, r(0), 0x851fc)?;
    c.emit(0x851ac, data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x851b0, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x851b4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x851b8, branch(C::Eq, 0x851b8, 0x851e8, false)?)?;
    c.emit(0x851bc, data_immediate(C::Al, Op::Cmp, r(0), r(2), 33)?)?;
    c.emit(0x851c0, branch(C::Cc, 0x851c0, 0x851d4, false)?)?;
    c.emit(0x851c4, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x851c8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x851cc, branch(C::Al, 0x851cc, 0x31ec, true)?)?;
    c.emit(0x851d0, data_immediate(C::Al, Op::Mov, r(2), r(0), 32)?)?;
    c.literal(0x851d4, r(0), 0x85200)?;
    c.emit(0x851d8, data_immediate(C::Al, Op::Add, r(1), r(0), 4)?)?;
    c.emit(0x851dc, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x851e0, branch(C::Al, 0x851e0, 0x471c, true)?)?;
    c.emit(0x851e4, branch(C::Al, 0x851e4, 0x851f4, false)?)?;
    c.emit(0x851e8, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x851ec, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x851f0, branch(C::Al, 0x851f0, 0x31ec, true)?)?;
    c.emit(0x851f4, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x851f8, transfer_multiple(C::Al, true, R::SP, 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.word(0x851fc, 0x10b530, true)?;
    c.word(0x85200, 0x10b534, true)?;
    Ok(())
}

fn copy_indexed_mask(c: &mut Code) -> Result<(), String> {
    c.emit(0x85204, transfer_multiple(C::Al, false, R::SP, 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85208, data_immediate(C::Al, Op::Add, r(11), r(13), 16)?)?;
    c.emit(0x8520c, data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85210, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x85214, data_register(C::Al, Op::Mov, r(5), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85218, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8521c, branch(C::Al, 0x8521c, 0x31ec, true)?)?;
    c.literal(0x85220, r(0), 0x85274)?;
    c.emit(0x85224, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85228, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x8522c, branch(C::Eq, 0x8522c, 0x8526c, false)?)?;
    c.emit(0x85230, multiply_register(C::Al, r(0), r(6), r(5))?)?;
    c.emit(0x85234, data_immediate(C::Al, Op::Cmp, r(0), r(0), 4)?)?;
    c.emit(0x85238, branch(C::Hi, 0x85238, 0x8526c, false)?)?;
    c.emit(0x8523c, data_immediate(C::Al, Op::Add, r(0), r(6), 1)?)?;
    c.emit(0x85240, multiply_register(C::Al, r(1), r(0), r(5))?)?;
    c.emit(0x85244, data_immediate(C::Al, Op::Cmp, r(0), r(1), 4)?)?;
    c.literal(0x85248, r(1), 0x85278)?;
    c.emit(0x8524c, data_immediate(C::Hi, Op::Mov, r(5), r(0), 4)?)?;
    c.emit(0x85250, multiply_register(C::Al, r(0), r(5), r(6))?)?;
    c.emit(0x85254, data_register(C::Al, Op::Mov, r(2), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85258, data_immediate(C::Al, Op::Bic, r(0), r(0), 3)?)?;
    c.emit(0x8525c, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85260, data_immediate(C::Al, Op::Add, r(1), r(0), 40)?)?;
    c.emit(0x85264, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85268, branch(C::Al, 0x85268, 0x471c, true)?)?;
    c.emit(0x8526c, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85270, transfer_multiple(C::Al, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x85274, 0x10b530, true)?;
    c.word(0x85278, 0x10b534, true)?;
    Ok(())
}

fn mask_queries(c: &mut Code) -> Result<(), String> {
    c.literal(0x8527c, r(0), 0x852a4)?;
    c.literal(0x85280, r(1), 0x852a8)?;
    c.emit(0x85284, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85288, transfer_byte(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x8528c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85290, data_immediate(C::Ne, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x85294, data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.emit(0x85298, data_immediate(C::Ne, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x8529c, data_register(C::Al, Op::And, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x852a0, bx(C::Al, r(14)))?;
    c.word(0x852a4, 0x10b530, true)?;
    c.word(0x852a8, 0x10b534, true)?;
    c.literal(0x852ac, r(1), 0x852f0)?;
    c.emit(0x852b0, transfer_byte(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x852b4, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x852b8, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x852bc, bx(C::Eq, r(14)))?;
    c.literal(0x852c0, r(2), 0x852f4)?;
    c.emit(0x852c4, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Asr, 31, false)?)?;
    c.emit(0x852c8, data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsr, 27, false)?)?;
    c.emit(0x852cc, data_immediate(C::Al, Op::And, r(0), r(0), 31)?)?;
    c.emit(0x852d0, data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Asr, 5, false)?)?;
    c.emit(0x852d4, data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x852d8, data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x852dc, transfer_word(C::Al, true, r(1), r(1), 4)?)?;
    c.emit(0x852e0, data_register_shifted_by_register(C::Al, Op::And, r(0), r(1), r(2), Shift::Lsl, r(0), false)?)?;
    c.emit(0x852e4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x852e8, data_immediate(C::Ne, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x852ec, bx(C::Al, r(14)))?;
    c.word(0x852f0, 0x10b530, true)?;
    c.word(0x852f4, 0x10b534, true)?;
    c.emit(0x852f8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x852fc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.emit(0x85300, branch(C::Gt, 0x85300, 0x85330, false)?)?;
    c.literal(0x85304, r(2), 0x85338)?;
    c.emit(0x85308, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Asr, 31, false)?)?;
    c.emit(0x8530c, data_register(C::Al, Op::Add, r(1), r(0), r(1), Shift::Lsr, 27, false)?)?;
    c.emit(0x85310, data_immediate(C::Al, Op::And, r(0), r(0), 31)?)?;
    c.emit(0x85314, data_register(C::Al, Op::Mov, r(1), r(0), r(1), Shift::Asr, 5, false)?)?;
    c.emit(0x85318, data_register(C::Al, Op::Add, r(1), r(2), r(1), Shift::Lsl, 2, false)?)?;
    c.emit(0x8531c, data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x85320, transfer_word(C::Al, true, r(1), r(1), 40)?)?;
    c.emit(0x85324, data_register_shifted_by_register(C::Al, Op::And, r(1), r(1), r(2), Shift::Lsl, r(0), false)?)?;
    c.emit(0x85328, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x8532c, data_immediate(C::Ne, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(0x85330, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85334, bx(C::Al, r(14)))?;
    c.word(0x85338, 0x10b534, true)?;
    Ok(())
}

fn set_mode(c: &mut Code) -> Result<(), String> {
    c.emit(0x8533c, transfer_multiple(C::Al, false, R::SP, 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85340, data_immediate(C::Al, Op::Add, r(11), r(13), 16)?)?;
    c.literal(0x85344, r(7), 0x853ec)?;
    c.emit(0x85348, data_register(C::Al, Op::Mov, r(5), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8534c, data_immediate(C::Al, Op::Mov, r(4), r(0), 1)?)?;
    c.emit(0x85350, transfer_byte(C::Al, true, r(0), r(7), 0)?)?;
    c.emit(0x85354, data_register(C::Al, Op::Cmp, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85358, branch(C::Eq, 0x85358, 0x853d8, false)?)?;
    c.literal(0x8535c, r(0), 0x853f0)?;
    c.emit(0x85360, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85364, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85368, branch(C::Eq, 0x85368, 0x853a8, false)?)?;
    c.literal(0x8536c, r(6), 0x853f4)?;
    c.emit(0x85370, transfer_word(C::Al, true, r(0), r(6), 0)?)?;
    c.emit(0x85374, branch(C::Al, 0x85374, 0x5958, true)?)?;
    c.emit(0x85378, data_immediate(C::Al, Op::Cmp, r(0), r(5), 1)?)?;
    c.emit(0x8537c, branch(C::Ne, 0x8537c, 0x853b0, false)?)?;
    c.emit(0x85380, transfer_word(C::Al, true, r(0), r(7), 36)?)?;
    c.emit(0x85384, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85388, branch(C::Gt, 0x85388, 0x853c0, false)?)?;
    c.emit(0x8538c, transfer_word(C::Al, true, r(0), r(7), 44)?)?;
    c.emit(0x85390, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85394, branch(C::Ne, 0x85394, 0x853c0, false)?)?;
    c.emit(0x85398, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x8539c, data_immediate(C::Al, Op::Mov, r(4), r(0), 2)?)?;
    c.emit(0x853a0, transfer_byte(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(0x853a4, branch(C::Al, 0x853a4, 0x853d0, false)?)?;
    c.emit(0x853a8, data_immediate(C::Al, Op::Mov, r(4), r(0), 130)?)?;
    c.emit(0x853ac, branch(C::Al, 0x853ac, 0x853d8, false)?)?;
    c.emit(0x853b0, transfer_byte(C::Al, false, r(5), r(7), 0)?)?;
    c.emit(0x853b4, branch(C::Al, 0x853b4, 0xd0294, true)?)?;
    c.emit(0x853b8, data_immediate(C::Al, Op::Mov, r(4), r(0), 0)?)?;
    c.emit(0x853bc, branch(C::Al, 0x853bc, 0x853d0, false)?)?;
    c.emit(0x853c0, branch(C::Al, 0x853c0, 0xd0264, true)?)?;
    c.emit(0x853c4, data_immediate(C::Al, Op::Mov, r(4), r(0), 129)?)?;
    c.emit(0x853c8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x853cc, branch(C::Eq, 0x853cc, 0x853e0, false)?)?;
    c.emit(0x853d0, transfer_word(C::Al, true, r(0), r(6), 0)?)?;
    c.emit(0x853d4, branch(C::Al, 0x853d4, 0x59e8, true)?)?;
    c.emit(0x853d8, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x853dc, transfer_multiple(C::Al, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x853e0, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x853e4, transfer_byte(C::Al, false, r(0), r(7), 0)?)?;
    c.emit(0x853e8, branch(C::Al, 0x853e8, 0x853b8, false)?)?;
    c.word(0x853ec, 0x10b534, true)?;
    c.word(0x853f0, 0x10b530, true)?;
    c.word(0x853f4, 0x10b568, true)?;
    Ok(())
}

fn update_state(c: &mut Code) -> Result<(), String> {
    c.emit(0x853f8, transfer_multiple(C::Al, false, R::SP, 0x4bf0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x853fc, data_immediate(C::Al, Op::Add, r(11), r(13), 24)?)?;
    c.literal(0x85400, r(8), 0x8559c)?;
    c.emit(0x85404, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85408, data_register(C::Al, Op::Mov, r(7), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x8540c, data_register(C::Al, Op::Mov, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85410, transfer_byte(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(0x85414, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85418, branch(C::Eq, 0x85418, 0x85474, false)?)?;
    c.emit(0x8541c, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0x85420, branch(C::Eq, 0x85420, 0x8547c, false)?)?;
    c.literal(0x85424, r(9), 0x855a8)?;
    c.emit(0x85428, transfer_word(C::Al, true, r(0), r(9), 0)?)?;
    c.emit(0x8542c, branch(C::Al, 0x8542c, 0x5958, true)?)?;
    c.emit(0x85430, data_immediate(C::Al, Op::Cmp, r(0), r(7), 1)?)?;
    c.emit(0x85434, branch(C::Ne, 0x85434, 0x854b0, false)?)?;
    c.emit(0x85438, transfer_byte(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(0x8543c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85440, branch(C::Eq, 0x85440, 0x854c4, false)?)?;
    c.literal(0x85444, r(0), 0x855a4)?;
    c.emit(0x85448, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x8544c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.emit(0x85450, branch(C::Ne, 0x85450, 0x854c4, false)?)?;
    c.emit(0x85454, branch(C::Al, 0x85454, 0xd0264, true)?)?;
    c.emit(0x85458, data_immediate(C::Al, Op::Mov, r(4), r(0), 128)?)?;
    c.emit(0x8545c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85460, branch(C::Ne, 0x85460, 0x85540, false)?)?;
    c.emit(0x85464, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0x85468, branch(C::Eq, 0x85468, 0x85590, false)?)?;
    c.emit(0x8546c, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x85470, branch(C::Al, 0x85470, 0x854bc, false)?)?;
    c.emit(0x85474, data_immediate(C::Al, Op::Mov, r(4), r(0), 130)?)?;
    c.emit(0x85478, branch(C::Al, 0x85478, 0x85588, false)?)?;
    c.emit(0x8547c, data_immediate(C::Al, Op::Mov, r(4), r(0), 131)?)?;
    c.emit(0x85480, data_immediate(C::Al, Op::Cmp, r(0), r(5), 2)?)?;
    c.emit(0x85484, branch(C::Eq, 0x85484, 0x85588, false)?)?;
    c.literal(0x85488, r(1), 0x855a0)?;
    c.emit(0x8548c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x85490, transfer_byte(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x85494, data_register(C::Al, Op::Cmp, r(0), r(2), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85498, branch(C::Eq, 0x85498, 0x854f0, false)?)?;
    c.emit(0x8549c, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(0x854a0, data_immediate(C::Al, Op::Add, r(1), r(1), 1)?)?;
    c.emit(0x854a4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 3)?)?;
    c.emit(0x854a8, branch(C::Ls, 0x854a8, 0x85490, false)?)?;
    c.emit(0x854ac, branch(C::Al, 0x854ac, 0x85588, false)?)?;
    c.emit(0x854b0, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0x854b4, branch(C::Eq, 0x854b4, 0x854dc, false)?)?;
    c.emit(0x854b8, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x854bc, branch(C::Al, 0x854bc, 0x855ac, true)?)?;
    c.emit(0x854c0, branch(C::Al, 0x854c0, 0x854e8, false)?)?;
    c.emit(0x854c4, branch(C::Al, 0x854c4, 0xd0294, true)?)?;
    c.emit(0x854c8, data_immediate(C::Al, Op::Cmp, r(0), r(6), 0)?)?;
    c.emit(0x854cc, branch(C::Eq, 0x854cc, 0x85528, false)?)?;
    c.emit(0x854d0, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x854d4, branch(C::Al, 0x854d4, 0x855ac, true)?)?;
    c.emit(0x854d8, branch(C::Al, 0x854d8, 0x85534, false)?)?;
    c.emit(0x854dc, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x854e0, data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x854e4, branch(C::Al, 0x854e4, 0x85654, true)?)?;
    c.emit(0x854e8, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x854ec, branch(C::Al, 0x854ec, 0x85540, false)?)?;
    c.literal(0x854f0, r(1), 0x855a4)?;
    c.emit(0x854f4, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Asr, 31, false)?)?;
    c.emit(0x854f8, data_immediate(C::Al, Op::Mov, r(4), r(0), 1)?)?;
    c.emit(0x854fc, data_register(C::Al, Op::Add, r(0), r(5), r(0), Shift::Lsr, 27, false)?)?;
    c.emit(0x85500, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Asr, 5, false)?)?;
    c.emit(0x85504, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x85508, data_immediate(C::Al, Op::And, r(1), r(5), 31)?)?;
    c.emit(0x8550c, transfer_word(C::Al, true, r(0), r(0), 4)?)?;
    c.emit(0x85510, data_register_shifted_by_register(C::Al, Op::And, r(0), r(0), r(4), Shift::Lsl, r(1), false)?)?;
    c.emit(0x85514, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85518, data_immediate(C::Ne, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x8551c, data_register(C::Al, Op::Cmp, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x85520, branch(C::Ne, 0x85520, 0x85424, false)?)?;
    c.emit(0x85524, branch(C::Al, 0x85524, 0x85588, false)?)?;
    c.emit(0x85528, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x8552c, data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(0x85530, branch(C::Al, 0x85530, 0x85654, true)?)?;
    c.emit(0x85534, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85538, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0)?)?;
    c.emit(0x8553c, data_immediate(C::Eq, Op::Mov, r(4), r(0), 2)?)?;
    c.emit(0x85540, transfer_byte(C::Al, true, r(0), r(8), 0)?)?;
    c.emit(0x85544, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85548, transfer_byte(C::Ne, true, r(0), r(8), 0)?)?;
    c.emit(0x8554c, data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85550, branch(C::Eq, 0x85550, 0x8557c, false)?)?;
    c.literal(0x85554, r(0), 0x855a4)?;
    c.emit(0x85558, transfer_byte(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x8555c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.emit(0x85560, branch(C::Ne, 0x85560, 0x8557c, false)?)?;
    c.emit(0x85564, transfer_word(C::Al, true, r(1), r(0), 36)?)?;
    c.emit(0x85568, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x8556c, branch(C::Gt, 0x8556c, 0x85580, false)?)?;
    c.emit(0x85570, transfer_word(C::Al, true, r(0), r(0), 44)?)?;
    c.emit(0x85574, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85578, branch(C::Ne, 0x85578, 0x85580, false)?)?;
    c.emit(0x8557c, branch(C::Al, 0x8557c, 0xd0294, true)?)?;
    c.emit(0x85580, transfer_word(C::Al, true, r(0), r(9), 0)?)?;
    c.emit(0x85584, branch(C::Al, 0x85584, 0x59e8, true)?)?;
    c.emit(0x85588, data_register(C::Al, Op::Mov, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x8558c, transfer_multiple(C::Al, true, R::SP, 0x8bf0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x85590, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85594, data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(0x85598, branch(C::Al, 0x85598, 0x854e4, false)?)?;
    c.word(0x8559c, 0x10b530, true)?;
    c.word(0x855a0, 0x10b52c, true)?;
    c.word(0x855a4, 0x10b534, true)?;
    c.word(0x855a8, 0x10b568, true)?;
    Ok(())
}

fn update_all_masks(c: &mut Code) -> Result<(), String> {
    c.emit(0x855ac, transfer_multiple(C::Al, false, R::SP, 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x855b0, data_immediate(C::Al, Op::Add, r(11), r(13), 28)?)?;
    c.literal(0x855b4, r(1), 0x8564c)?;
    c.literal(0x855b8, r(10), 0x85650)?;
    c.emit(0x855bc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.emit(0x855c0, data_register(C::Al, Op::Mov, r(12), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x855c4, data_immediate(C::Al, Op::Mov, r(9), r(0), 1)?)?;
    c.emit(0x855c8, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(0x855cc, data_immediate(C::Al, Op::Mov, r(14), r(0), 28)?)?;
    c.emit(0x855d0, data_immediate(C::Al, Op::Mov, r(8), r(0), 1)?)?;
    c.emit(0x855d4, data_immediate(C::Ne, Op::Mvn, r(12), r(0), 0)?)?;
    c.emit(0x855d8, transfer_byte_postindexed(C::Al, true, r(6), r(1), 1)?)?;
    c.emit(0x855dc, data_immediate(C::Al, Op::Cmp, r(0), r(6), 2)?)?;
    c.emit(0x855e0, branch(C::Eq, 0x855e0, 0x85638, false)?)?;
    c.emit(0x855e4, data_immediate(C::Al, Op::And, r(5), r(6), 31)?)?;
    c.emit(0x855e8, data_register(C::Al, Op::And, r(6), r(14), r(6), Shift::Lsr, 3, false)?)?;
    c.emit(0x855ec, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x855f0, data_register(C::Al, Op::Add, r(6), r(10), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x855f4, transfer_word_preindexed_writeback(C::Al, true, r(7), r(6), 4)?)?;
    c.emit(0x855f8, data_register_shifted_by_register(C::Al, Op::And, r(2), r(7), r(9), Shift::Lsl, r(5), false)?)?;
    c.emit(0x855fc, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.emit(0x85600, data_immediate(C::Ne, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x85604, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85608, data_immediate(C::Ne, Op::Mov, r(4), r(0), 1)?)?;
    c.emit(0x8560c, data_register(C::Al, Op::Teq, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85610, branch(C::Eq, 0x85610, 0x85638, false)?)?;
    c.emit(0x85614, data_register_shifted_by_register(C::Al, Op::Mov, r(2), r(0), r(9), Shift::Lsl, r(5), false)?)?;
    c.emit(0x85618, data_immediate(C::Al, Op::Cmp, r(0), r(0), 1)?)?;
    c.emit(0x8561c, data_immediate(C::Al, Op::Mov, r(8), r(0), 0)?)?;
    c.emit(0x85620, data_register(C::Al, Op::Bic, r(4), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85624, data_register(C::Eq, Op::Orr, r(4), r(7), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85628, transfer_word(C::Al, false, r(4), r(6), 0)?)?;
    c.emit(0x8562c, transfer_word(C::Al, true, r(2), r(10), 36)?)?;
    c.emit(0x85630, data_register(C::Al, Op::Add, r(2), r(2), r(12), Shift::Lsl, 0, false)?)?;
    c.emit(0x85634, transfer_word(C::Al, false, r(2), r(10), 36)?)?;
    c.emit(0x85638, data_immediate(C::Al, Op::Add, r(3), r(3), 1)?)?;
    c.emit(0x8563c, data_immediate(C::Al, Op::Cmp, r(0), r(3), 4)?)?;
    c.emit(0x85640, data_immediate(C::Eq, Op::And, r(0), r(8), 255)?)?;
    c.emit(0x85644, transfer_multiple(C::Eq, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x85648, branch(C::Al, 0x85648, 0x855d8, false)?)?;
    c.word(0x8564c, 0x10b52c, true)?;
    c.word(0x85650, 0x10b534, true)?;
    Ok(())
}

fn update_one_mask(c: &mut Code) -> Result<(), String> {
    c.emit(0x85654, transfer_multiple(C::Al, false, R::SP, 0x48f0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85658, data_immediate(C::Al, Op::Add, r(11), r(13), 16)?)?;
    c.emit(0x8565c, data_immediate(C::Al, Op::Mov, r(2), r(0), 131)?)?;
    c.emit(0x85660, data_immediate(C::Al, Op::Cmp, r(0), r(0), 2)?)?;
    c.emit(0x85664, branch(C::Eq, 0x85664, 0x856ec, false)?)?;
    c.literal(0x85668, r(12), 0x856f4)?;
    c.emit(0x8566c, data_register(C::Al, Op::Mov, r(3), r(0), r(0), Shift::Asr, 31, false)?)?;
    c.emit(0x85670, data_immediate(C::Al, Op::And, r(4), r(0), 31)?)?;
    c.emit(0x85674, data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(0x85678, data_register(C::Al, Op::Mov, r(6), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8567c, data_register(C::Al, Op::Add, r(3), r(0), r(3), Shift::Lsr, 27, false)?)?;
    c.emit(0x85680, data_register(C::Al, Op::Mov, r(3), r(0), r(3), Shift::Asr, 5, false)?)?;
    c.emit(0x85684, data_register(C::Al, Op::Add, r(3), r(12), r(3), Shift::Lsl, 2, false)?)?;
    c.emit(0x85688, transfer_word_preindexed_writeback(C::Al, true, r(14), r(3), 4)?)?;
    c.emit(0x8568c, data_register_shifted_by_register(C::Al, Op::And, r(5), r(14), r(2), Shift::Lsl, r(4), false)?)?;
    c.emit(0x85690, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0)?)?;
    c.emit(0x85694, data_immediate(C::Ne, Op::Mov, r(5), r(0), 1)?)?;
    c.emit(0x85698, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(0x8569c, data_immediate(C::Ne, Op::Mov, r(6), r(0), 1)?)?;
    c.emit(0x856a0, data_register(C::Al, Op::Teq, r(0), r(6), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x856a4, branch(C::Eq, 0x856a4, 0x856ec, false)?)?;
    c.emit(0x856a8, data_register_shifted_by_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsl, r(4), false)?)?;
    c.emit(0x856ac, data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.emit(0x856b0, branch(C::Ne, 0x856b0, 0x856c8, false)?)?;
    c.emit(0x856b4, data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(0x856b8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 255)?)?;
    c.emit(0x856bc, branch(C::Hi, 0x856bc, 0x856dc, false)?)?;
    c.emit(0x856c0, data_register(C::Al, Op::Orr, r(0), r(14), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x856c4, branch(C::Al, 0x856c4, 0x856d8, false)?)?;
    c.emit(0x856c8, data_immediate(C::Al, Op::Mvn, r(1), r(0), 0)?)?;
    c.emit(0x856cc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 255)?)?;
    c.emit(0x856d0, branch(C::Hi, 0x856d0, 0x856dc, false)?)?;
    c.emit(0x856d4, data_register(C::Al, Op::Bic, r(0), r(14), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x856d8, transfer_word(C::Al, false, r(0), r(3), 0)?)?;
    c.emit(0x856dc, transfer_word(C::Al, true, r(0), r(12), 36)?)?;
    c.emit(0x856e0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x856e4, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x856e8, transfer_word(C::Al, false, r(0), r(12), 36)?)?;
    c.emit(0x856ec, data_register(C::Al, Op::Mov, r(0), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x856f0, transfer_multiple(C::Al, true, R::SP, 0x88f0, BlockMode::IncrementAfter, true)?)?;
    c.word(0x856f4, 0x10b534, true)?;
    Ok(())
}

fn state_wrappers(c: &mut Code) -> Result<(), String> {
    c.emit(0x856f8, data_register(C::Al, Op::Mov, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x856fc, data_register(C::Al, Op::Mov, r(1), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85700, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.emit(0x85704, branch(C::Al, 0x85704, 0x853f8, false)?)?;
    c.emit(0x85708, data_register(C::Al, Op::Mov, r(2), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8570c, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x85710, data_immediate(C::Al, Op::Mov, r(1), r(0), 255)?)?;
    c.emit(0x85714, branch(C::Al, 0x85714, 0x853f8, false)?)?;
    c.emit(0x85718, data_immediate(C::Al, Op::Mov, r(0), r(0), 131)?)?;
    c.emit(0x8571c, bx(C::Al, r(14)))?;
    c.emit(0x85720, data_immediate(C::Al, Op::Mov, r(0), r(0), 131)?)?;
    c.emit(0x85724, bx(C::Al, r(14)))?;
    Ok(())
}

fn initialize(c: &mut Code) -> Result<(), String> {
    c.emit(0x85728, transfer_multiple(C::Al, false, R::SP, 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8572c, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.literal(0x85730, r(4), 0x857bc)?;
    c.emit(0x85734, transfer_byte(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x85738, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x8573c, branch(C::Ne, 0x8573c, 0x85768, false)?)?;
    c.literal(0x85740, r(0), 0x857c0)?;
    c.emit(0x85744, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85748, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x8574c, branch(C::Ne, 0x8574c, 0x85760, false)?)?;
    c.literal(0x85750, r(1), 0x857c0)?;
    c.emit(0x85754, data_immediate(C::Al, Op::Mov, r(0), r(0), 64)?)?;
    c.emit(0x85758, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.emit(0x8575c, branch(C::Al, 0x8575c, 0x58d8, true)?)?;
    c.emit(0x85760, data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x85764, transfer_byte(C::Al, false, r(0), r(4), 0)?)?;
    c.literal(0x85768, r(4), 0x857c0)?;
    c.emit(0x8576c, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x85770, branch(C::Al, 0x85770, 0x5958, true)?)?;
    c.literal(0x85774, r(5), 0x857c4)?;
    c.emit(0x85778, transfer_byte(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(0x8577c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85780, branch(C::Ne, 0x85780, 0x857b0, false)?)?;
    c.literal(0x85784, r(0), 0x857c8)?;
    c.emit(0x85788, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x8578c, data_immediate(C::Al, Op::Mov, r(2), r(0), 44)?)?;
    c.emit(0x85790, transfer_byte_postindexed(C::Al, false, r(1), r(0), 4)?)?;
    c.emit(0x85794, data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(0x85798, branch(C::Al, 0x85798, 0x31ec, true)?)?;
    c.emit(0x8579c, branch(C::Al, 0x8579c, 0x84c48, true)?)?;
    c.emit(0x857a0, branch(C::Al, 0x857a0, 0x86038, true)?)?;
    c.emit(0x857a4, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x857a8, data_immediate(C::Eq, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(0x857ac, transfer_byte(C::Eq, false, r(0), r(5), 0)?)?;
    c.emit(0x857b0, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x857b4, transfer_multiple(C::Al, true, R::SP, 0x4830, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x857b8, branch(C::Al, 0x857b8, 0x59e8, false)?)?;
    c.word(0x857bc, 0x10b564, true)?;
    c.word(0x857c0, 0x10b568, true)?;
    c.word(0x857c4, 0x10b530, true)?;
    c.word(0x857c8, 0x10b534, true)?;
    Ok(())
}

fn initialize_secondary(c: &mut Code) -> Result<(), String> {
    c.emit(0x857cc, transfer_multiple(C::Al, false, R::SP, 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x857d0, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.emit(0x857d4, branch(C::Al, 0x857d4, 0x85728, true)?)?;
    c.literal(0x857d8, r(4), 0x85818)?;
    c.emit(0x857dc, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x857e0, branch(C::Al, 0x857e0, 0x5958, true)?)?;
    c.literal(0x857e4, r(0), 0x8581c)?;
    c.emit(0x857e8, transfer_byte(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x857ec, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x857f0, branch(C::Eq, 0x857f0, 0x8580c, false)?)?;
    c.literal(0x857f4, r(5), 0x85820)?;
    c.emit(0x857f8, transfer_byte(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(0x857fc, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(0x85800, branch(C::Ne, 0x85800, 0x8580c, false)?)?;
    c.emit(0x85804, branch(C::Al, 0x85804, 0x85e64, true)?)?;
    c.emit(0x85808, transfer_byte(C::Al, false, r(0), r(5), 0)?)?;
    c.emit(0x8580c, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x85810, transfer_multiple(C::Al, true, R::SP, 0x4830, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x85814, branch(C::Al, 0x85814, 0x59e8, false)?)?;
    c.word(0x85818, 0x10b568, true)?;
    c.word(0x8581c, 0x10b530, true)?;
    c.word(0x85820, 0x10b531, true)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    small_accessors(&mut c)?;
    copy_identifier(&mut c)?;
    copy_state_mask(&mut c)?;
    copy_indexed_mask(&mut c)?;
    mask_queries(&mut c)?;
    set_mode(&mut c)?;
    update_state(&mut c)?;
    update_all_masks(&mut c)?;
    update_one_mask(&mut c)?;
    state_wrappers(&mut c)?;
    initialize(&mut c)?;
    initialize_secondary(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != END - START || CODE_BYTES + DATA_BYTES != END - START
        || LITERALS.len() * 4 != DATA_BYTES
    {
        return Err("invalid ARM state extent".into());
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
    fn extent_literals_and_partition_guards() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1876);
        assert_eq!((CODE_BYTES, DATA_BYTES), (436 * 4, 33 * 4));
        for (at, value) in LITERALS {
            assert_eq!(word(&bytes, at), value);
        }
        for at in (START..END).step_by(4) {
            let expected = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, v)| v);
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(at, bx(C::Al, R::LR)).is_err(), expected.is_some());
            for (_, value) in LITERALS {
                let mut c = Code { at, bytes: Vec::new() };
                let valid = expected == Some(value);
                assert_eq!(c.word(at, value, true).is_ok(), valid);
                assert_eq!(c.at, at + if valid { 4 } else { 0 });
                assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
            }
            if let Some(value) = expected {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.word(at, value ^ 1, true).is_err());
                assert!(c.bytes.is_empty());
                assert_eq!(c.at, at);
            }
        }
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, bx(C::Al, R::LR)).is_err());
            assert!(c.word(at, 0x10b530, true).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.literal(START, r(0), START).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn entry_rooted_static_partition() {
        let bytes = fragment().unwrap();
        let mut pending = vec![
            0x850d0, 0x850e4, 0x850e8, 0x850ec, 0x850f4, 0x85104,
            0x85114, 0x85138, 0x8515c, 0x8519c, 0x85204, 0x8527c,
            0x852ac, 0x852f8, 0x8533c, 0x853f8, 0x855ac, 0x85654,
            0x856f8, 0x85708, 0x85718, 0x85720, 0x85728, 0x857cc,
        ];
        let mut code = std::collections::BTreeSet::new();
        let mut literals = std::collections::BTreeSet::new();
        while let Some(at) = pending.pop() {
            if !(START..END).contains(&at) || !code.insert(at) {
                continue;
            }
            assert!(!LITERALS.iter().any(|&(slot, _)| slot == at));
            let w = word(&bytes, at);
            let conditional = w >> 28 != 14;
            if w & 0x0e00_0000 == 0x0a00_0000 {
                let displacement = ((w << 8) as i32 >> 6) as isize;
                let target = (at as isize + 8 + displacement) as usize;
                pending.push(target);
                if conditional || w & 0x0100_0000 != 0 {
                    pending.push(at + 4);
                }
            } else if w & 0x0fff_fff0 == 0x012f_ff10
                || (w & 0x0e10_0000 == 0x0810_0000 && w & 0x8000 != 0)
            {
                if conditional {
                    pending.push(at + 4);
                }
            } else {
                pending.push(at + 4);
            }
            if w & 0x0f7f_0000 == 0x051f_0000 {
                let offset = (w & 0xfff) as isize;
                let slot = (at as isize + 8 + if w & 0x0080_0000 != 0 { offset } else { -offset }) as usize;
                assert!(LITERALS.iter().any(|&(address, _)| address == slot));
                literals.insert(slot);
            }
        }
        assert_eq!(code.len() * 4, CODE_BYTES);
        assert_eq!(literals, LITERALS.iter().map(|&(at, _)| at).collect());
        for at in (START..END).step_by(4) {
            assert_ne!(code.contains(&at), literals.contains(&at));
        }
    }

    #[test]
    fn flags_branches_and_complete_final_wrapper() {
        let bytes = fragment().unwrap();
        for (at, condition, target, link) in [
            (0x85170, C::Cc, 0x85184, false),
            (0x853cc, C::Eq, 0x853e0, false),
            (0x85520, C::Ne, 0x85424, false),
            (0x85648, C::Al, 0x855d8, false),
            (0x856d0, C::Hi, 0x856dc, false),
            (0x857d4, C::Al, 0x85728, true),
            (0x857f0, C::Eq, 0x8580c, false),
            (0x85800, C::Ne, 0x8580c, false),
            (0x85804, C::Al, 0x85e64, true),
            (0x85814, C::Al, 0x59e8, false),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        assert_eq!(word(&bytes, 0x852bc), bx(C::Eq, R::LR));
        assert_eq!(word(&bytes, 0x8560c), data_register(C::Al, Op::Teq, r(0), r(4), r(2), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x8554c), data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0).unwrap());
        assert_eq!(word(&bytes, 0x85644), transfer_multiple(C::Eq, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x857cc), transfer_multiple(C::Al, false, R::SP, 0x4830, BlockMode::DecrementBefore, true).unwrap());
        assert_eq!(word(&bytes, 0x85810), transfer_multiple(C::Al, true, R::SP, 0x4830, BlockMode::IncrementAfter, true).unwrap());
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
