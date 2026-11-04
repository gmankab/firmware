use arm32::{
    branch, data_immediate, data_register, data_register_shifted_by_register, load_halfword,
    load_signed_byte, load_signed_byte_postindexed, load_signed_halfword,
    signed_multiply_accumulate_bottom_halves, store_halfword, transfer_byte,
    transfer_byte_postindexed, transfer_byte_register, transfer_doubleword, transfer_multiple,
    transfer_word, transfer_word_postindexed, transfer_word_preindexed_writeback,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x70128;
pub const END: usize = 0x70468;
const PACKED160_TABLE_POINTERS: u32 = 0x000f2e30;
const PACKED160_WIDTHS: u32 = 0x000f2d5b;
const PACKED160_SHIFTS: u32 = 0x000f2d55;
const PACKED128_TABLE_ZERO: u32 = 0x000f27d6;
const PACKED128_TABLE_NONZERO: u32 = 0x000f29d6;

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn instruction(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if at != self.at || at % 4 != 0
            || !((START..0x703f0).contains(&at) || (0x703fc..0x70460).contains(&at))
        {
            return Err(format!("invalid history/packed instruction address 0x{at:x}"));
        }
        self.bytes.extend(instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn checked_data(&mut self, at: usize, value: u32) -> Result<(), String> {
        let expected = match at {
            0x703f0 => PACKED160_TABLE_POINTERS,
            0x703f4 => PACKED160_WIDTHS,
            0x703f8 => PACKED160_SHIFTS,
            0x70460 => PACKED128_TABLE_ZERO,
            0x70464 => PACKED128_TABLE_NONZERO,
            _ => return Err(format!("invalid history/packed data address 0x{at:x}")),
        };
        if at != self.at || value != expected {
            return Err(format!("invalid history/packed literal at 0x{at:x}"));
        }
        self.bytes.extend(value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("constant A32 register")
}

fn history_commit(c: &mut Code) -> Result<(), String> {
    c.instruction(0x70128, transfer_multiple(C::Al, false, r(13), 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x7012c, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70130, data_immediate(C::Al, Op::Mov, r(3), r(0), 0xb30)?)?;
    c.instruction(0x70134, data_immediate(C::Al, Op::Add, r(12), r(0), 0x278)?)?;
    c.instruction(0x70138, data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.instruction(0x7013c, transfer_word_register(C::Al, true, r(14), r(0), r(3), Shift::Lsl, 0)?)?;
    c.instruction(0x70140, transfer_word(C::Al, true, r(0), r(0), 12)?)?;
    c.instruction(0x70144, data_immediate(C::Al, Op::Mov, r(3), r(0), 0xd8)?)?;
    c.instruction(0x70148, signed_multiply_accumulate_bottom_halves(C::Al, r(2), r(2), r(3), r(14))?)?;
    c.instruction(0x7014c, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.instruction(0x70150, data_immediate(C::Al, Op::Add, r(2), r(2), 0xf8)?)?;
    c.instruction(0x70154, data_immediate(C::Eq, Op::Add, r(2), r(12), 0x400)?)?;
    c.instruction(0x70158, data_immediate(C::Al, Op::Cmp, r(0), r(1), 2)?)?;
    c.instruction(0x7015c, branch(C::Eq, 0x7015c, 0x70190, false)?)?;
    c.instruction(0x70160, data_immediate(C::Al, Op::Cmp, r(0), r(1), 1)?)?;
    c.instruction(0x70164, branch(C::Eq, 0x70164, 0x701f0, false)?)?;
    c.instruction(0x70168, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.instruction(0x7016c, transfer_multiple(C::Ne, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x70170, transfer_word(C::Al, true, r(0), r(2), 128)?)?;
    c.instruction(0x70174, transfer_word(C::Al, false, r(0), r(2), 132)?)?;
    c.instruction(0x70178, transfer_byte(C::Al, true, r(0), r(2), 152)?)?;
    c.instruction(0x7017c, transfer_byte(C::Al, false, r(0), r(2), 153)?)?;
    c.instruction(0x70180, transfer_doubleword(C::Al, true, r(0), r(2), 56)?)?;
    c.instruction(0x70184, transfer_doubleword(C::Al, false, r(0), r(2), 64)?)?;
    c.instruction(0x70188, transfer_word_preindexed_writeback(C::Al, true, r(0), r(2), 104)?)?;
    c.instruction(0x7018c, branch(C::Al, 0x7018c, 0x70258, false)?)?;
    c.instruction(0x70190, transfer_word(C::Al, true, r(0), r(2), 144)?)?;
    c.instruction(0x70194, transfer_word(C::Al, false, r(0), r(2), 148)?)?;
    c.instruction(0x70198, transfer_byte(C::Al, true, r(0), r(2), 156)?)?;
    c.instruction(0x7019c, transfer_byte(C::Al, false, r(0), r(2), 157)?)?;
    c.instruction(0x701a0, transfer_doubleword(C::Al, true, r(0), r(2), 88)?)?;
    c.instruction(0x701a4, transfer_doubleword(C::Al, false, r(0), r(2), 96)?)?;
    c.instruction(0x701a8, transfer_word(C::Al, true, r(0), r(2), 120)?)?;
    c.instruction(0x701ac, transfer_word(C::Al, false, r(0), r(2), 124)?)?;
    c.instruction(0x701b0, load_signed_halfword(C::Al, r(0), r(2), 184)?)?;
    c.instruction(0x701b4, load_signed_halfword(C::Al, r(1), r(2), 188)?)?;
    c.instruction(0x701b8, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x701bc, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x701c0, store_halfword(C::Al, r(0), r(2), 188)?)?;
    c.instruction(0x701c4, load_signed_halfword(C::Al, r(0), r(2), 186)?)?;
    c.instruction(0x701c8, load_signed_halfword(C::Al, r(1), r(2), 190)?)?;
    c.instruction(0x701cc, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x701d0, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x701d4, store_halfword(C::Al, r(0), r(2), 190)?)?;
    c.instruction(0x701d8, transfer_doubleword(C::Al, true, r(0), r(2), 200)?)?;
    c.instruction(0x701dc, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x701e0, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x701e4, transfer_word(C::Al, false, r(0), r(2), 204)?)?;
    c.instruction(0x701e8, transfer_word_preindexed_writeback(C::Al, true, r(0), r(2), 192)?)?;
    c.instruction(0x701ec, branch(C::Al, 0x701ec, 0x7024c, false)?)?;
    c.instruction(0x701f0, transfer_word(C::Al, true, r(0), r(2), 136)?)?;
    c.instruction(0x701f4, transfer_word(C::Al, false, r(0), r(2), 140)?)?;
    c.instruction(0x701f8, transfer_byte(C::Al, true, r(0), r(2), 154)?)?;
    c.instruction(0x701fc, transfer_byte(C::Al, false, r(0), r(2), 155)?)?;
    c.instruction(0x70200, transfer_doubleword(C::Al, true, r(0), r(2), 72)?)?;
    c.instruction(0x70204, transfer_doubleword(C::Al, false, r(0), r(2), 80)?)?;
    c.instruction(0x70208, transfer_word(C::Al, true, r(0), r(2), 112)?)?;
    c.instruction(0x7020c, transfer_word(C::Al, false, r(0), r(2), 116)?)?;
    c.instruction(0x70210, load_signed_halfword(C::Al, r(0), r(2), 158)?)?;
    c.instruction(0x70214, load_signed_halfword(C::Al, r(1), r(2), 162)?)?;
    c.instruction(0x70218, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x7021c, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x70220, store_halfword(C::Al, r(0), r(2), 162)?)?;
    c.instruction(0x70224, load_signed_halfword(C::Al, r(0), r(2), 160)?)?;
    c.instruction(0x70228, load_signed_halfword(C::Al, r(1), r(2), 164)?)?;
    c.instruction(0x7022c, data_register(C::Al, Op::Add, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70230, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x70234, store_halfword(C::Al, r(0), r(2), 164)?)?;
    c.instruction(0x70238, transfer_doubleword(C::Al, true, r(0), r(2), 176)?)?;
    c.instruction(0x7023c, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70240, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x70244, transfer_word(C::Al, false, r(0), r(2), 180)?)?;
    c.instruction(0x70248, transfer_word_preindexed_writeback(C::Al, true, r(0), r(2), 168)?)?;
    c.instruction(0x7024c, transfer_word(C::Al, true, r(1), r(2), 4)?)?;
    c.instruction(0x70250, data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70254, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false)?)?;
    c.instruction(0x70258, transfer_word(C::Al, false, r(0), r(2), 4)?)?;
    c.instruction(0x7025c, transfer_multiple(C::Al, true, r(13), 0x8800, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn packed160_words(c: &mut Code) -> Result<(), String> {
    c.instruction(0x70260, transfer_multiple(C::Al, false, r(13), 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x70264, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.instruction(0x70268, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x34)?)?;
    c.instruction(0x7026c, transfer_word(C::Al, true, r(2), r(0), 2688)?)?;
    c.instruction(0x70270, data_immediate(C::Al, Op::Mov, r(7), r(0), 0)?)?;
    c.instruction(0x70274, data_immediate(C::Al, Op::Mov, r(12), r(0), 0xaa)?)?;
    c.instruction(0x70278, data_immediate(C::Al, Op::Add, r(1), r(1), 0x280)?)?;
    c.instruction(0x7027c, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.instruction(0x70280, data_immediate(C::Al, Op::Orr, r(12), r(12), 0x100)?)?;
    c.instruction(0x70284, load_signed_byte(C::Al, r(3), r(2), 0)?)?;
    c.instruction(0x70288, transfer_word(C::Al, false, r(1), r(13), 0)?)?;
    c.instruction(0x7028c, data_immediate(C::Al, Op::Add, r(1), r(2), 0x26)?)?;
    c.instruction(0x70290, data_immediate(C::Al, Op::Cmp, r(0), r(3), 3)?)?;
    c.instruction(0x70294, transfer_word(C::Ls, true, r(7), r(15), 340)?)?;
    c.instruction(0x70298, transfer_word_register(C::Ls, true, r(7), r(7), r(3), Shift::Lsl, 2)?)?;
    c.instruction(0x7029c, data_immediate(C::Al, Op::Add, r(3), r(13), 0x1c)?)?;
    c.instruction(0x702a0, data_register(C::Al, Op::Mov, r(2), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x702a4, load_halfword(C::Al, r(5), r(1), -24)?)?;
    c.instruction(0x702a8, load_halfword(C::Al, r(4), r(1), 0)?)?;
    c.instruction(0x702ac, data_register(C::Al, Op::Mov, r(6), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.instruction(0x702b0, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.instruction(0x702b4, data_immediate(C::Al, Op::Add, r(1), r(1), 2)?)?;
    c.instruction(0x702b8, data_register(C::Al, Op::Sub, r(4), r(4), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x702bc, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x9f)?)?;
    c.instruction(0x702c0, data_register(C::Le, Op::Add, r(6), r(2), r(5), Shift::Lsl, 0, false)?)?;
    c.instruction(0x702c4, data_register(C::Al, Op::Add, r(2), r(2), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x702c8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 6)?)?;
    c.instruction(0x702cc, transfer_word_postindexed(C::Al, false, r(6), r(3), 4)?)?;
    c.instruction(0x702d0, branch(C::Ne, 0x702d0, 0x702a4, false)?)?;
    c.instruction(0x702d4, data_immediate(C::Al, Op::Add, r(0), r(13), 0x1c)?)?;
    c.instruction(0x702d8, transfer_word(C::Al, true, r(3), r(13), 28)?)?;
    c.instruction(0x702dc, transfer_word(C::Al, true, r(4), r(13), 0)?)?;
    c.instruction(0x702e0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0)?)?;
    c.instruction(0x702e4, data_immediate(C::Al, Op::Mov, r(5), r(0), 0)?)?;
    c.instruction(0x702e8, data_immediate(C::Al, Op::Add, r(0), r(0), 4)?)?;
    c.instruction(0x702ec, transfer_word(C::Al, false, r(0), r(13), 12)?)?;
    c.instruction(0x702f0, transfer_word(C::Al, true, r(0), r(15), 252)?)?;
    c.instruction(0x702f4, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.instruction(0x702f8, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.instruction(0x702fc, transfer_word(C::Al, true, r(0), r(15), 244)?)?;
    c.instruction(0x70300, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.instruction(0x70304, transfer_word(C::Al, false, r(0), r(13), 4)?)?;
    c.instruction(0x70308, data_register(C::Al, Op::Mov, r(0), r(0), r(5), Shift::Asr, 31, false)?)?;
    c.instruction(0x7030c, data_immediate(C::Al, Op::Mov, r(7), r(0), 1)?)?;
    c.instruction(0x70310, transfer_word(C::Al, true, r(9), r(13), 4)?)?;
    c.instruction(0x70314, transfer_word(C::Al, true, r(8), r(13), 8)?)?;
    c.instruction(0x70318, transfer_word(C::Al, true, r(14), r(13), 12)?)?;
    c.instruction(0x7031c, data_register(C::Al, Op::Add, r(0), r(5), r(0), Shift::Lsr, 27, false)?)?;
    c.instruction(0x70320, data_immediate(C::Al, Op::Bic, r(1), r(0), 0x1f)?)?;
    c.instruction(0x70324, data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Asr, 5, false)?)?;
    c.instruction(0x70328, data_register(C::Al, Op::Sub, r(1), r(5), r(1), Shift::Lsl, 0, false)?)?;
    c.instruction(0x7032c, transfer_word(C::Al, false, r(0), r(13), 20)?)?;
    c.instruction(0x70330, data_register_shifted_by_register(C::Al, Op::Mov, r(6), r(0), r(7), Shift::Lsl, r(1), false)?)?;
    c.instruction(0x70334, transfer_word(C::Al, false, r(6), r(13), 16)?)?;
    c.instruction(0x70338, load_signed_byte_postindexed(C::Al, r(12), r(3), 1)?)?;
    c.instruction(0x7033c, data_register(C::Al, Op::Mov, r(0), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70340, transfer_word(C::Al, false, r(3), r(13), 24)?)?;
    c.instruction(0x70344, transfer_word(C::Al, true, r(3), r(14), 0)?)?;
    c.instruction(0x70348, transfer_byte(C::Al, true, r(6), r(8), 0)?)?;
    c.instruction(0x7034c, transfer_byte_register(C::Al, true, r(3), r(3), r(5), Shift::Lsl, 0)?)?;
    c.instruction(0x70350, data_immediate(C::Al, Op::Cmp, r(0), r(6), 7)?)?;
    c.instruction(0x70354, data_register(C::Al, Op::Sub, r(10), r(3), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70358, branch(C::Eq, 0x70358, 0x7036c, false)?)?;
    c.instruction(0x7035c, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.instruction(0x70360, data_immediate(C::Al, Op::Cmp, r(0), r(6), 6)?)?;
    c.instruction(0x70364, data_immediate(C::Eq, Op::And, r(3), r(10), 0x3f)?)?;
    c.instruction(0x70368, branch(C::Al, 0x70368, 0x70370, false)?)?;
    c.instruction(0x7036c, data_immediate(C::Al, Op::And, r(3), r(10), 0x7f)?)?;
    c.instruction(0x70370, transfer_byte(C::Al, true, r(6), r(9), 0)?)?;
    c.instruction(0x70374, data_immediate(C::Al, Op::Cmp, r(0), r(7), 5)?)?;
    c.instruction(0x70378, data_register_shifted_by_register(C::Al, Op::Orr, r(0), r(0), r(3), Shift::Lsl, r(6), false)?)?;
    c.instruction(0x7037c, branch(C::Eq, 0x7037c, 0x703a0, false)?)?;
    c.instruction(0x70380, data_immediate(C::Al, Op::Add, r(7), r(7), 1)?)?;
    c.instruction(0x70384, data_immediate(C::Al, Op::Add, r(9), r(9), 1)?)?;
    c.instruction(0x70388, data_immediate(C::Al, Op::Add, r(8), r(8), 1)?)?;
    c.instruction(0x7038c, data_immediate(C::Al, Op::Add, r(14), r(14), 4)?)?;
    c.instruction(0x70390, transfer_word(C::Al, false, r(2), r(4), 0)?)?;
    c.instruction(0x70394, data_immediate(C::Al, Op::Cmp, r(0), r(7), 6)?)?;
    c.instruction(0x70398, branch(C::Ne, 0x70398, 0x70344, false)?)?;
    c.instruction(0x7039c, branch(C::Al, 0x7039c, 0x703d0, false)?)?;
    c.instruction(0x703a0, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.instruction(0x703a4, transfer_word(C::Eq, true, r(1), r(13), 0)?)?;
    c.instruction(0x703a8, transfer_word(C::Eq, true, r(3), r(13), 20)?)?;
    c.instruction(0x703ac, transfer_word_register(C::Eq, false, r(2), r(1), r(3), Shift::Lsl, 2)?)?;
    c.instruction(0x703b0, data_immediate(C::Al, Op::Tst, r(0), r(10), 0x40)?)?;
    c.instruction(0x703b4, branch(C::Eq, 0x703b4, 0x703d0, false)?)?;
    c.instruction(0x703b8, transfer_word(C::Al, true, r(3), r(13), 0)?)?;
    c.instruction(0x703bc, transfer_word(C::Al, true, r(7), r(13), 20)?)?;
    c.instruction(0x703c0, transfer_word(C::Al, true, r(6), r(13), 16)?)?;
    c.instruction(0x703c4, transfer_word_register(C::Al, true, r(1), r(3), r(7), Shift::Lsl, 2)?)?;
    c.instruction(0x703c8, data_register(C::Al, Op::Orr, r(1), r(1), r(6), Shift::Lsl, 0, false)?)?;
    c.instruction(0x703cc, transfer_word_register(C::Al, false, r(1), r(3), r(7), Shift::Lsl, 2)?)?;
    c.instruction(0x703d0, transfer_word(C::Al, false, r(0), r(4), -640)?)?;
    c.instruction(0x703d4, data_immediate(C::Al, Op::Add, r(5), r(5), 1)?)?;
    c.instruction(0x703d8, data_immediate(C::Al, Op::Add, r(4), r(4), 4)?)?;
    c.instruction(0x703dc, transfer_word(C::Al, true, r(3), r(13), 24)?)?;
    c.instruction(0x703e0, data_immediate(C::Al, Op::Cmp, r(0), r(5), 0xa0)?)?;
    c.instruction(0x703e4, data_immediate(C::Eq, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.instruction(0x703e8, transfer_multiple(C::Eq, true, r(13), 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x703ec, branch(C::Al, 0x703ec, 0x70308, false)?)?;
    c.checked_data(0x703f0, PACKED160_TABLE_POINTERS)?;
    c.checked_data(0x703f4, PACKED160_WIDTHS)?;
    c.checked_data(0x703f8, PACKED160_SHIFTS)?;
    Ok(())
}

fn packed128_words(c: &mut Code) -> Result<(), String> {
    c.instruction(0x703fc, transfer_multiple(C::Al, false, r(13), 0x4830, BlockMode::DecrementBefore, true)?)?;
    c.instruction(0x70400, data_immediate(C::Al, Op::Add, r(11), r(13), 8)?)?;
    c.instruction(0x70404, transfer_word(C::Al, true, r(0), r(0), 728)?)?;
    c.instruction(0x70408, transfer_word(C::Al, true, r(2), r(15), 80)?)?;
    c.instruction(0x7040c, transfer_word(C::Al, true, r(12), r(15), 80)?)?;
    c.instruction(0x70410, data_immediate(C::Al, Op::Mov, r(14), r(0), 0)?)?;
    c.instruction(0x70414, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.instruction(0x70418, data_register(C::Eq, Op::Mov, r(12), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.instruction(0x7041c, data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.instruction(0x70420, data_register(C::Al, Op::Mov, r(2), r(0), r(12), Shift::Lsl, 0, false)?)?;
    c.instruction(0x70424, data_immediate(C::Al, Op::Mov, r(4), r(0), 0)?)?;
    c.instruction(0x70428, data_immediate(C::Al, Op::Mov, r(0), r(0), 0)?)?;
    c.instruction(0x7042c, transfer_byte_postindexed(C::Al, true, r(5), r(2), 1)?)?;
    c.instruction(0x70430, data_immediate(C::Al, Op::Add, r(4), r(4), 1)?)?;
    c.instruction(0x70434, data_immediate(C::Al, Op::Cmp, r(0), r(4), 4)?)?;
    c.instruction(0x70438, data_immediate(C::Al, Op::And, r(5), r(5), 0x3f)?)?;
    c.instruction(0x7043c, data_register_shifted_by_register(C::Al, Op::Orr, r(0), r(0), r(5), Shift::Lsl, r(3), false)?)?;
    c.instruction(0x70440, data_immediate(C::Al, Op::Add, r(3), r(3), 6)?)?;
    c.instruction(0x70444, branch(C::Ne, 0x70444, 0x7042c, false)?)?;
    c.instruction(0x70448, data_immediate(C::Al, Op::Add, r(14), r(14), 1)?)?;
    c.instruction(0x7044c, transfer_word_postindexed(C::Al, false, r(0), r(1), 4)?)?;
    c.instruction(0x70450, data_immediate(C::Al, Op::Add, r(12), r(12), 4)?)?;
    c.instruction(0x70454, data_immediate(C::Al, Op::Cmp, r(0), r(14), 0x80)?)?;
    c.instruction(0x70458, transfer_multiple(C::Eq, true, r(13), 0x8830, BlockMode::IncrementAfter, true)?)?;
    c.instruction(0x7045c, branch(C::Al, 0x7045c, 0x7041c, false)?)?;
    c.checked_data(0x70460, PACKED128_TABLE_ZERO)?;
    c.checked_data(0x70464, PACKED128_TABLE_NONZERO)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    history_commit(&mut c)?;
    packed160_words(&mut c)?;
    packed128_words(&mut c)?;
    if c.at != END || c.bytes.len() != END - START {
        return Err("incomplete history/packed fragment".into());
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
    fn exact_extent_and_checked_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 832);
        assert_eq!((0x703f0 - START) + (0x70460 - 0x703fc), 812);
        let literals = [
            (0x703f0, PACKED160_TABLE_POINTERS), (0x703f4, PACKED160_WIDTHS),
            (0x703f8, PACKED160_SHIFTS), (0x70460, PACKED128_TABLE_ZERO),
            (0x70464, PACKED128_TABLE_NONZERO),
        ];
        assert_eq!(literals.len() * 4, 20);
        for (at, value) in literals {
            assert_eq!(word(&bytes, at), value);
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.instruction(at, 0).is_err());
            assert!(c.checked_data(at, value ^ 1).is_err());
            assert!(c.checked_data(at + 4, value).is_err());
            c.checked_data(at, value).unwrap();
            assert!(c.checked_data(at, value).is_err());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.checked_data(START, PACKED160_TABLE_POINTERS).is_err());
        assert!(c.instruction(START + 4, 0).is_err());
        let mut c = Code { at: END, bytes: Vec::new() };
        assert!(c.instruction(END, 0).is_err());
    }

    #[test]
    fn history_classes_and_strided_copies() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x70148), signed_multiply_accumulate_bottom_halves(C::Al, r(2), r(2), r(3), r(14)).unwrap());
        for (at, target) in [(0x7015c, 0x70190), (0x70164, 0x701f0)] {
            assert_eq!(word(&bytes, at), branch(C::Eq, at as u32, target, false).unwrap());
        }
        for (at, source) in [(0x70180, 56), (0x70200, 72), (0x701a0, 88)] {
            assert_eq!(word(&bytes, at), transfer_doubleword(C::Al, true, r(0), r(2), source).unwrap());
            assert_eq!(word(&bytes, at + 4), transfer_doubleword(C::Al, false, r(0), r(2), source + 8).unwrap());
        }
        assert_eq!(word(&bytes, 0x70254), data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsr, 1, false).unwrap());
    }

    #[test]
    fn packed_streams_bitmap_and_selector() {
        let bytes = fragment().unwrap();
        for (at, target) in [(0x70294, 0x703f0), (0x702f0, 0x703f4), (0x702fc, 0x703f8), (0x70408, 0x70460), (0x7040c, 0x70464)] {
            let dest = match at { 0x70294 => r(7), 0x70408 => r(2), 0x7040c => r(12), _ => r(0) };
            let cond = if at == 0x70294 { C::Ls } else { C::Al };
            assert_eq!(word(&bytes, at), transfer_word(cond, true, dest, R::PC, (target - at - 8) as i32).unwrap());
        }
        for (at, reg, limit) in [(0x702c8, 0, 6), (0x703e0, 5, 160), (0x70434, 4, 4), (0x70454, 14, 128)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Cmp, r(0), r(reg), limit).unwrap());
        }
        assert_eq!(word(&bytes, 0x70278), data_immediate(C::Al, Op::Add, r(1), r(1), 0x280).unwrap());
        assert_eq!(word(&bytes, 0x703ac), transfer_word_register(C::Eq, false, r(2), r(1), r(3), Shift::Lsl, 2).unwrap());
        assert_eq!(word(&bytes, 0x703cc), transfer_word_register(C::Al, false, r(1), r(3), r(7), Shift::Lsl, 2).unwrap());
        assert_eq!(word(&bytes, 0x70404), transfer_word(C::Al, true, r(0), r(0), 0x2d8).unwrap());
        assert_eq!(word(&bytes, 0x70438), data_immediate(C::Al, Op::And, r(5), r(5), 0x3f).unwrap());
    }

    #[test]
    fn optional_pinned_reference() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return; };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }
}
