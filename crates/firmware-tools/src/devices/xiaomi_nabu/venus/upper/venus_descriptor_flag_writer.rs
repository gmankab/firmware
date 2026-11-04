use arm32::{
    branch, data_immediate, data_register, mov_register, push_registers, transfer_multiple,
    transfer_word, transfer_word_register_preindexed_writeback, BlockMode, Condition as C,
    DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xffbc;
pub const END: usize = 0x104d8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 0x514;
pub const DATA_BYTES: usize = 8;

const LITERALS: [(usize, u32); 2] = [(0x104d0, 0xe3dfc), (0x104d4, 0xe2788)];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("descriptor writer address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        let expected = LITERALS.iter().find(|&&(at, _)| at == self.at).map(|&(_, v)| v);
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || if data { expected != Some(value) } else { expected.is_some() }
        {
            return Err(format!("invalid descriptor writer word at {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        self.word(instruction, false)
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        self.word(value, true)
    }

    fn immediate(&mut self, at: usize, op: Op, dst: u8, src: u8, value: u32) -> Result<(), String> {
        self.emit(at, data_immediate(C::Al, op, r(dst), r(src), value)?)
    }

    fn transfer(&mut self, at: usize, load: bool, value: u8, base: u8, offset: i32) -> Result<(), String> {
        self.emit(at, transfer_word(C::Al, load, r(value), r(base), offset)?)
    }

    fn load_literal(&mut self, at: usize, dst: u8, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| address == slot) {
            return Err(format!("invalid descriptor writer literal slot {slot:#x}"));
        }
        self.transfer(at, true, dst, 15, slot as i32 - at as i32 - 8)
    }
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(0xffbc, push_registers(C::Al, 0x4ff0)?)?;
    c.immediate(0xffc0, Op::Add, 11, 13, 0x1c)?;
    c.immediate(0xffc4, Op::Sub, 13, 13, 0x14)?;
    c.emit(0xffc8, mov_register(C::Al, r(10), r(0)))?;
    c.emit(0xffcc, mov_register(C::Al, r(6), r(2)))?;
    c.immediate(0xffd0, Op::Mov, 2, 0, 0x1)?;
    c.transfer(0xffd4, true, 0, 10, 0xa58)?;
    c.emit(0xffd8, mov_register(C::Al, r(4), r(10)))?;
    c.transfer(0xffdc, true, 3, 0, 0x334)?;
    c.transfer(0xffe0, false, 2, 10, 0x1c)?;
    c.immediate(0xffe4, Op::Mov, 2, 0, 0x10)?;
    c.transfer(0xffe8, true, 0, 0, 0x330)?;
    c.transfer(0xffec, false, 1, 13, 0xc)?;
    c.emit(0xfff0, data_register(C::Al, Op::Bic, r(12), r(2), r(3), Shift::Lsl, 3, false)?)?;
    c.immediate(0xfff4, Op::Cmp, 0, 0, 0x1)?;
    c.immediate(0xfff8, Op::Mov, 0, 0, 0x228)?;
    c.transfer(0xfffc, false, 12, 13, 0x10)?;
    c.immediate(0x10000, Op::Orr, 0, 0, 0x1c00)?;
    c.emit(0x10004, branch(C::Ne, 0x10004, 0x1012c, false)?)?;
    c.transfer(0x10008, true, 2, 6, 0x10)?;
    c.emit(0x1000c, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.transfer(0x10010, true, 14, 6, 0xc)?;
    c.immediate(0x10014, Op::Mov, 5, 0, 0x20c)?;
    c.emit(0x10018, mov_register(C::Al, r(8), r(10)))?;
    c.immediate(0x1001c, Op::Mov, 3, 0, 0x208)?;
    c.immediate(0x10020, Op::Orr, 7, 12, 0x2)?;
    c.immediate(0x10024, Op::Orr, 5, 5, 0x9400)?;
    c.immediate(0x10028, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x1002c, false, 2, 13, 0x8)?;
    c.transfer(0x10030, true, 2, 6, 0x14)?;
    c.transfer(0x10034, true, 9, 6, 0x18)?;
    c.transfer(0x10038, false, 5, 0, 0x0)?;
    c.immediate(0x1003c, Op::Mov, 5, 0, 0x22c)?;
    c.immediate(0x10040, Op::Orr, 5, 5, 0x1c00)?;
    c.emit(0x10044, transfer_word_register_preindexed_writeback(C::Al, true, r(5), r(8), r(5), Shift::Lsl, 0)?)?;
    c.emit(0x10048, data_register(C::Al, Op::Add, r(2), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x1004c, data_register(C::Al, Op::Add, r(1), r(2), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(0x10050, data_register(C::Al, Op::Add, r(9), r(1), r(9), Shift::Lsl, 0, false)?)?;
    c.immediate(0x10054, Op::Mov, 1, 0, 0x28c)?;
    c.transfer(0x10058, false, 7, 5, 0x0)?;
    c.transfer(0x1005c, false, 3, 0, 0x4)?;
    c.immediate(0x10060, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10064, false, 9, 5, 0x4)?;
    c.transfer(0x10068, false, 1, 0, 0x8)?;
    c.immediate(0x1006c, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x10070, false, 7, 5, 0x8)?;
    c.transfer(0x10074, false, 1, 4, 0x0)?;
    c.immediate(0x10078, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x1007c, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10080, false, 1, 0, 0xc)?;
    c.immediate(0x10084, Op::Add, 0, 5, 0x10)?;
    c.load_literal(0x10088, 1, 0x104d0)?;
    c.transfer(0x1008c, false, 0, 8, 0x0)?;
    c.transfer(0x10090, true, 2, 13, 0x8)?;
    c.immediate(0x10094, Op::Add, 0, 2, 0xff)?;
    c.immediate(0x10098, Op::Bic, 0, 0, 0xff)?;
    c.immediate(0x1009c, Op::Orr, 7, 0, 0x1)?;
    c.immediate(0x100a0, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x100a4, false, 7, 5, 0xc)?;
    c.emit(0x100a8, mov_register(C::Al, r(5), r(12)))?;
    c.emit(0x100ac, branch(C::Al, 0x100ac, 0x2ec94, true)?)?;
    c.load_literal(0x100b0, 1, 0x104d4)?;
    c.immediate(0x100b4, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x100b8, branch(C::Al, 0x100b8, 0x2ec94, true)?)?;
    c.transfer(0x100bc, true, 0, 4, 0x0)?;
    c.immediate(0x100c0, Op::Mov, 1, 0, 0x20c)?;
    c.immediate(0x100c4, Op::Mov, 3, 0, 0x208)?;
    c.immediate(0x100c8, Op::Orr, 1, 1, 0x9400)?;
    c.immediate(0x100cc, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x100d0, false, 1, 0, 0x0)?;
    c.immediate(0x100d4, Op::Orr, 1, 5, 0x2)?;
    c.transfer(0x100d8, true, 2, 8, 0x0)?;
    c.immediate(0x100dc, Op::Orr, 1, 1, 0x1000)?;
    c.transfer(0x100e0, false, 1, 2, 0x0)?;
    c.transfer(0x100e4, false, 3, 0, 0x4)?;
    c.immediate(0x100e8, Op::Mov, 3, 0, 0x28c)?;
    c.transfer(0x100ec, false, 9, 2, 0x4)?;
    c.immediate(0x100f0, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x100f4, false, 3, 0, 0x8)?;
    c.transfer(0x100f8, false, 1, 2, 0x8)?;
    c.immediate(0x100fc, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x10100, false, 1, 4, 0x0)?;
    c.immediate(0x10104, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x10108, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x1010c, false, 1, 0, 0xc)?;
    c.immediate(0x10110, Op::Add, 0, 2, 0x10)?;
    c.load_literal(0x10114, 1, 0x104d0)?;
    c.transfer(0x10118, false, 0, 8, 0x0)?;
    c.transfer(0x1011c, false, 7, 2, 0xc)?;
    c.immediate(0x10120, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x10124, true, 2, 13, 0x8)?;
    c.emit(0x10128, branch(C::Al, 0x10128, 0x10244, false)?)?;
    c.emit(0x1012c, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.immediate(0x10130, Op::Mov, 3, 0, 0x20c)?;
    c.transfer(0x10134, true, 14, 6, 0x14)?;
    c.transfer(0x10138, true, 2, 6, 0xc)?;
    c.emit(0x1013c, mov_register(C::Al, r(9), r(10)))?;
    c.immediate(0x10140, Op::Mov, 7, 0, 0x208)?;
    c.immediate(0x10144, Op::Orr, 3, 3, 0x9400)?;
    c.immediate(0x10148, Op::Orr, 7, 7, 0x9400)?;
    c.transfer(0x1014c, false, 3, 0, 0x0)?;
    c.immediate(0x10150, Op::Mov, 3, 0, 0x22c)?;
    c.emit(0x10154, data_register(C::Al, Op::Add, r(1), r(1), r(14), Shift::Lsl, 0, false)?)?;
    c.immediate(0x10158, Op::Orr, 3, 3, 0x1c00)?;
    c.emit(0x1015c, transfer_word_register_preindexed_writeback(C::Al, true, r(5), r(9), r(3), Shift::Lsl, 0)?)?;
    c.immediate(0x10160, Op::Orr, 3, 12, 0x2)?;
    c.transfer(0x10164, false, 3, 5, 0x0)?;
    c.transfer(0x10168, false, 7, 0, 0x4)?;
    c.transfer(0x1016c, false, 1, 13, 0x8)?;
    c.transfer(0x10170, false, 1, 5, 0x4)?;
    c.immediate(0x10174, Op::Mov, 1, 0, 0x28c)?;
    c.emit(0x10178, mov_register(C::Al, r(7), r(12)))?;
    c.immediate(0x1017c, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10180, false, 1, 0, 0x8)?;
    c.immediate(0x10184, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x10188, false, 3, 5, 0x8)?;
    c.transfer(0x1018c, false, 1, 4, 0x0)?;
    c.immediate(0x10190, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x10194, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10198, false, 1, 0, 0xc)?;
    c.immediate(0x1019c, Op::Add, 0, 5, 0x10)?;
    c.transfer(0x101a0, false, 0, 9, 0x0)?;
    c.immediate(0x101a4, Op::Add, 0, 2, 0xff)?;
    c.transfer(0x101a8, false, 2, 13, 0x4)?;
    c.immediate(0x101ac, Op::Bic, 0, 0, 0xff)?;
    c.immediate(0x101b0, Op::Orr, 8, 0, 0x1)?;
    c.immediate(0x101b4, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x101b8, false, 8, 5, 0xc)?;
    c.load_literal(0x101bc, 5, 0x104d0)?;
    c.emit(0x101c0, mov_register(C::Al, r(1), r(5)))?;
    c.emit(0x101c4, branch(C::Al, 0x101c4, 0x2ec94, true)?)?;
    c.load_literal(0x101c8, 1, 0x104d4)?;
    c.immediate(0x101cc, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x101d0, branch(C::Al, 0x101d0, 0x2ec94, true)?)?;
    c.transfer(0x101d4, true, 0, 4, 0x0)?;
    c.immediate(0x101d8, Op::Mov, 1, 0, 0x20c)?;
    c.immediate(0x101dc, Op::Mov, 3, 0, 0x208)?;
    c.immediate(0x101e0, Op::Orr, 1, 1, 0x9400)?;
    c.immediate(0x101e4, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x101e8, false, 1, 0, 0x0)?;
    c.immediate(0x101ec, Op::Orr, 1, 7, 0x2)?;
    c.transfer(0x101f0, true, 2, 9, 0x0)?;
    c.immediate(0x101f4, Op::Orr, 1, 1, 0x1000)?;
    c.transfer(0x101f8, false, 1, 2, 0x0)?;
    c.transfer(0x101fc, false, 3, 0, 0x4)?;
    c.transfer(0x10200, true, 3, 13, 0x8)?;
    c.transfer(0x10204, false, 3, 2, 0x4)?;
    c.immediate(0x10208, Op::Mov, 3, 0, 0x28c)?;
    c.immediate(0x1020c, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x10210, false, 3, 0, 0x8)?;
    c.transfer(0x10214, false, 1, 2, 0x8)?;
    c.immediate(0x10218, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x1021c, false, 1, 4, 0x0)?;
    c.immediate(0x10220, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x10224, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10228, false, 1, 0, 0xc)?;
    c.immediate(0x1022c, Op::Add, 0, 2, 0x10)?;
    c.emit(0x10230, mov_register(C::Al, r(1), r(5)))?;
    c.transfer(0x10234, false, 0, 9, 0x0)?;
    c.transfer(0x10238, false, 8, 2, 0xc)?;
    c.immediate(0x1023c, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x10240, true, 2, 13, 0x4)?;
    c.emit(0x10244, branch(C::Al, 0x10244, 0x2ec94, true)?)?;
    c.load_literal(0x10248, 1, 0x104d4)?;
    c.immediate(0x1024c, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x10250, branch(C::Al, 0x10250, 0x2ec94, true)?)?;
    c.transfer(0x10254, true, 0, 6, 0x4)?;
    c.immediate(0x10258, Op::Cmp, 0, 0, 0x1)?;
    c.emit(0x1025c, branch(C::Ne, 0x1025c, 0x104c4, false)?)?;
    c.transfer(0x10260, true, 0, 10, 0xa58)?;
    c.transfer(0x10264, true, 7, 6, 0x14)?;
    c.transfer(0x10268, true, 0, 0, 0x330)?;
    c.immediate(0x1026c, Op::Cmp, 0, 0, 0x1)?;
    c.emit(0x10270, branch(C::Ne, 0x10270, 0x103a4, false)?)?;
    c.immediate(0x10274, Op::Mov, 0, 0, 0x228)?;
    c.emit(0x10278, mov_register(C::Al, r(4), r(10)))?;
    c.immediate(0x1027c, Op::Mov, 2, 0, 0x20c)?;
    c.transfer(0x10280, true, 1, 6, 0xc)?;
    c.transfer(0x10284, true, 6, 6, 0x18)?;
    c.immediate(0x10288, Op::Mov, 5, 0, 0x208)?;
    c.immediate(0x1028c, Op::Orr, 0, 0, 0x1c00)?;
    c.immediate(0x10290, Op::Orr, 2, 2, 0x9400)?;
    c.immediate(0x10294, Op::Orr, 5, 5, 0x9400)?;
    c.emit(0x10298, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.transfer(0x1029c, false, 2, 0, 0x0)?;
    c.immediate(0x102a0, Op::Mov, 2, 0, 0x22c)?;
    c.immediate(0x102a4, Op::Orr, 2, 2, 0x1c00)?;
    c.transfer(0x102a8, true, 8, 13, 0x10)?;
    c.emit(0x102ac, transfer_word_register_preindexed_writeback(C::Al, true, r(2), r(10), r(2), Shift::Lsl, 0)?)?;
    c.immediate(0x102b0, Op::Orr, 3, 8, 0x3)?;
    c.transfer(0x102b4, false, 3, 2, 0x0)?;
    c.transfer(0x102b8, false, 5, 0, 0x4)?;
    c.transfer(0x102bc, true, 5, 13, 0xc)?;
    c.emit(0x102c0, data_register(C::Al, Op::Add, r(7), r(5), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x102c4, data_register(C::Al, Op::Add, r(9), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.immediate(0x102c8, Op::Mov, 1, 0, 0x28c)?;
    c.immediate(0x102cc, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x102d0, false, 9, 2, 0x4)?;
    c.transfer(0x102d4, false, 1, 0, 0x8)?;
    c.immediate(0x102d8, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x102dc, false, 3, 2, 0x8)?;
    c.transfer(0x102e0, false, 1, 4, 0x0)?;
    c.immediate(0x102e4, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x102e8, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x102ec, false, 1, 0, 0xc)?;
    c.immediate(0x102f0, Op::Add, 0, 2, 0x10)?;
    c.load_literal(0x102f4, 1, 0x104d0)?;
    c.transfer(0x102f8, false, 0, 10, 0x0)?;
    c.immediate(0x102fc, Op::Add, 0, 6, 0xff)?;
    c.immediate(0x10300, Op::Bic, 0, 0, 0xff)?;
    c.immediate(0x10304, Op::Orr, 5, 0, 0x1)?;
    c.immediate(0x10308, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x1030c, false, 5, 2, 0xc)?;
    c.emit(0x10310, mov_register(C::Al, r(2), r(6)))?;
    c.emit(0x10314, branch(C::Al, 0x10314, 0x2ec94, true)?)?;
    c.load_literal(0x10318, 7, 0x104d4)?;
    c.immediate(0x1031c, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x10320, mov_register(C::Al, r(1), r(7)))?;
    c.emit(0x10324, branch(C::Al, 0x10324, 0x2ec94, true)?)?;
    c.transfer(0x10328, true, 0, 4, 0x0)?;
    c.immediate(0x1032c, Op::Mov, 1, 0, 0x20c)?;
    c.immediate(0x10330, Op::Mov, 3, 0, 0x208)?;
    c.immediate(0x10334, Op::Orr, 1, 1, 0x9400)?;
    c.immediate(0x10338, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x1033c, false, 1, 0, 0x0)?;
    c.immediate(0x10340, Op::Orr, 1, 8, 0x3)?;
    c.transfer(0x10344, true, 2, 10, 0x0)?;
    c.immediate(0x10348, Op::Orr, 1, 1, 0x1000)?;
    c.transfer(0x1034c, false, 1, 2, 0x0)?;
    c.transfer(0x10350, false, 3, 0, 0x4)?;
    c.immediate(0x10354, Op::Mov, 3, 0, 0x28c)?;
    c.transfer(0x10358, false, 9, 2, 0x4)?;
    c.immediate(0x1035c, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x10360, false, 3, 0, 0x8)?;
    c.transfer(0x10364, false, 1, 2, 0x8)?;
    c.immediate(0x10368, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x1036c, false, 1, 4, 0x0)?;
    c.immediate(0x10370, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x10374, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10378, false, 1, 0, 0xc)?;
    c.load_literal(0x1037c, 1, 0x104d0)?;
    c.immediate(0x10380, Op::Add, 0, 2, 0x10)?;
    c.transfer(0x10384, false, 0, 10, 0x0)?;
    c.transfer(0x10388, false, 5, 2, 0xc)?;
    c.immediate(0x1038c, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x10390, mov_register(C::Al, r(2), r(6)))?;
    c.emit(0x10394, branch(C::Al, 0x10394, 0x2ec94, true)?)?;
    c.immediate(0x10398, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x1039c, mov_register(C::Al, r(1), r(7)))?;
    c.emit(0x103a0, branch(C::Al, 0x103a0, 0x104c0, false)?)?;
    c.immediate(0x103a4, Op::Mov, 0, 0, 0x228)?;
    c.emit(0x103a8, mov_register(C::Al, r(4), r(10)))?;
    c.immediate(0x103ac, Op::Mov, 1, 0, 0x20c)?;
    c.immediate(0x103b0, Op::Mov, 3, 0, 0x208)?;
    c.immediate(0x103b4, Op::Orr, 0, 0, 0x1c00)?;
    c.immediate(0x103b8, Op::Orr, 1, 1, 0x9400)?;
    c.immediate(0x103bc, Op::Orr, 3, 3, 0x9400)?;
    c.emit(0x103c0, transfer_word_register_preindexed_writeback(C::Al, true, r(0), r(4), r(0), Shift::Lsl, 0)?)?;
    c.transfer(0x103c4, false, 1, 0, 0x0)?;
    c.immediate(0x103c8, Op::Mov, 1, 0, 0x22c)?;
    c.immediate(0x103cc, Op::Orr, 1, 1, 0x1c00)?;
    c.transfer(0x103d0, true, 9, 13, 0x10)?;
    c.emit(0x103d4, transfer_word_register_preindexed_writeback(C::Al, true, r(1), r(10), r(1), Shift::Lsl, 0)?)?;
    c.immediate(0x103d8, Op::Orr, 2, 9, 0x3)?;
    c.transfer(0x103dc, false, 2, 1, 0x0)?;
    c.transfer(0x103e0, false, 3, 0, 0x4)?;
    c.immediate(0x103e4, Op::Mov, 3, 0, 0x28c)?;
    c.transfer(0x103e8, true, 8, 13, 0xc)?;
    c.immediate(0x103ec, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x103f0, false, 8, 1, 0x4)?;
    c.transfer(0x103f4, false, 3, 0, 0x8)?;
    c.transfer(0x103f8, false, 2, 1, 0x8)?;
    c.immediate(0x103fc, Op::Add, 2, 0, 0x10)?;
    c.transfer(0x10400, false, 2, 4, 0x0)?;
    c.immediate(0x10404, Op::Mov, 2, 0, 0x288)?;
    c.immediate(0x10408, Op::Orr, 2, 2, 0x9400)?;
    c.transfer(0x1040c, false, 2, 0, 0xc)?;
    c.immediate(0x10410, Op::Add, 0, 1, 0x10)?;
    c.emit(0x10414, mov_register(C::Al, r(2), r(7)))?;
    c.transfer(0x10418, false, 0, 10, 0x0)?;
    c.immediate(0x1041c, Op::Add, 0, 7, 0xff)?;
    c.immediate(0x10420, Op::Bic, 0, 0, 0xff)?;
    c.immediate(0x10424, Op::Orr, 5, 0, 0x1)?;
    c.immediate(0x10428, Op::Mov, 0, 0, 0x1)?;
    c.transfer(0x1042c, false, 5, 1, 0xc)?;
    c.load_literal(0x10430, 1, 0x104d0)?;
    c.emit(0x10434, branch(C::Al, 0x10434, 0x2ec94, true)?)?;
    c.load_literal(0x10438, 6, 0x104d4)?;
    c.immediate(0x1043c, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x10440, mov_register(C::Al, r(1), r(6)))?;
    c.emit(0x10444, branch(C::Al, 0x10444, 0x2ec94, true)?)?;
    c.transfer(0x10448, true, 0, 4, 0x0)?;
    c.immediate(0x1044c, Op::Mov, 1, 0, 0x20c)?;
    c.immediate(0x10450, Op::Mov, 3, 0, 0x208)?;
    c.immediate(0x10454, Op::Orr, 1, 1, 0x9400)?;
    c.immediate(0x10458, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x1045c, false, 1, 0, 0x0)?;
    c.immediate(0x10460, Op::Orr, 1, 9, 0x3)?;
    c.transfer(0x10464, true, 2, 10, 0x0)?;
    c.immediate(0x10468, Op::Orr, 1, 1, 0x1000)?;
    c.transfer(0x1046c, false, 1, 2, 0x0)?;
    c.transfer(0x10470, false, 3, 0, 0x4)?;
    c.immediate(0x10474, Op::Mov, 3, 0, 0x28c)?;
    c.transfer(0x10478, false, 8, 2, 0x4)?;
    c.immediate(0x1047c, Op::Orr, 3, 3, 0x9400)?;
    c.transfer(0x10480, false, 3, 0, 0x8)?;
    c.transfer(0x10484, false, 1, 2, 0x8)?;
    c.immediate(0x10488, Op::Add, 1, 0, 0x10)?;
    c.transfer(0x1048c, false, 1, 4, 0x0)?;
    c.immediate(0x10490, Op::Mov, 1, 0, 0x288)?;
    c.immediate(0x10494, Op::Orr, 1, 1, 0x9400)?;
    c.transfer(0x10498, false, 1, 0, 0xc)?;
    c.load_literal(0x1049c, 1, 0x104d0)?;
    c.immediate(0x104a0, Op::Add, 0, 2, 0x10)?;
    c.transfer(0x104a4, false, 0, 10, 0x0)?;
    c.transfer(0x104a8, false, 5, 2, 0xc)?;
    c.immediate(0x104ac, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x104b0, mov_register(C::Al, r(2), r(7)))?;
    c.emit(0x104b4, branch(C::Al, 0x104b4, 0x2ec94, true)?)?;
    c.immediate(0x104b8, Op::Mov, 0, 0, 0x1)?;
    c.emit(0x104bc, mov_register(C::Al, r(1), r(6)))?;
    c.emit(0x104c0, branch(C::Al, 0x104c0, 0x2ec94, true)?)?;
    c.immediate(0x104c4, Op::Mov, 0, 0, 0x1)?;
    c.immediate(0x104c8, Op::Sub, 13, 11, 0x1c)?;
    c.emit(0x104cc, transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    for (at, value) in LITERALS {
        c.literal(at, value)?;
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

    #[test]
    fn structure_and_operand_order() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), END - START);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(word(&bytes, START), push_registers(C::Al, 0x4ff0).unwrap());
        assert_eq!(word(&bytes, 0x104cc), transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0xfff0), data_register(C::Al, Op::Bic, r(12), r(2), r(3), Shift::Lsl, 3, false).unwrap());
        for (at, dst, base, index) in [
            (0x1000c, 0, 4, 0), (0x10044, 5, 8, 5),
            (0x1012c, 0, 4, 0), (0x1015c, 5, 9, 3),
            (0x10298, 0, 4, 0), (0x102ac, 2, 10, 2),
            (0x103c0, 0, 4, 0), (0x103d4, 1, 10, 1),
        ] {
            assert_eq!(word(&bytes, at), transfer_word_register_preindexed_writeback(C::Al, true, r(dst), r(base), r(index), Shift::Lsl, 0).unwrap());
        }
        for (at, dst, value) in [
            (0x10000, 0, 0x1c00), (0x10040, 5, 0x1c00),
            (0x10158, 3, 0x1c00), (0x1028c, 0, 0x1c00),
            (0x102a4, 2, 0x1c00), (0x103b4, 0, 0x1c00),
            (0x103cc, 1, 0x1c00),
        ] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Orr, r(dst), r(dst), value).unwrap());
        }
        let mut calls = 0;
        for at in (START..0x104d0).step_by(4) {
            let instruction = word(&bytes, at);
            if instruction & 0x0e000000 == 0x0a000000 {
                let displacement = (((instruction & 0xffffff) << 8) as i32 >> 6) as u32;
                let target = (at as u32).wrapping_add(8).wrapping_add(displacement);
                if instruction & 0x01000000 != 0 {
                    calls += 1;
                    assert_eq!(target, 0x2ec94);
                } else {
                    assert!((START as u32..0x104d0).contains(&target));
                }
            }
            if instruction & 0x0fff8000 == 0x08bd8000 {
                assert_eq!(at, 0x104cc);
            }
            if instruction & 0x0f7f0000 == 0x051f0000 {
                let slot = at + 8 + (instruction & 0xfff) as usize;
                assert!(LITERALS.iter().any(|&(address, _)| address == slot));
            }
        }
        assert_eq!(calls, 13);
        for (at, dst, base) in [
            (0x1006c, 1, 0), (0x10084, 0, 5),
            (0x100fc, 1, 0), (0x10110, 0, 2),
            (0x10184, 1, 0), (0x1019c, 0, 5),
            (0x10218, 1, 0), (0x1022c, 0, 2),
            (0x102d8, 1, 0), (0x102f0, 0, 2),
            (0x10368, 1, 0), (0x10380, 0, 2),
            (0x103fc, 2, 0), (0x10410, 0, 1),
            (0x10488, 1, 0), (0x104a0, 0, 2),
        ] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Add, r(dst), r(base), 0x10).unwrap());
        }
        for (at, value) in LITERALS {
            assert_eq!(word(&bytes, at), value);
        }
    }

    #[test]
    fn exact_data_guards() {
        for at in (START..END).step_by(4) {
            let expected = LITERALS.iter().find(|&&(slot, _)| slot == at).map(|&(_, value)| value);
            let mut c = Code { at, bytes: Vec::new() };
            assert_eq!(c.emit(at, mov_register(C::Al, r(0), r(0))).is_err(), expected.is_some());
            for (_, value) in LITERALS {
                let mut c = Code { at, bytes: Vec::new() };
                let ok = expected == Some(value);
                assert_eq!(c.literal(at, value).is_ok(), ok);
                assert_eq!(c.at, at + if ok { 4 } else { 0 });
                assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
            }
            if let Some(value) = expected {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.literal(at, value ^ 1).is_err());
                assert_eq!(c.at, at);
                assert!(c.bytes.is_empty());
            }
        }
        for at in [START - 4, START + 1, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.literal(at, 0xe3dfc).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, mov_register(C::Al, r(0), r(0))).is_err());
        assert!(c.load_literal(START, 0, 0x104d1).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
