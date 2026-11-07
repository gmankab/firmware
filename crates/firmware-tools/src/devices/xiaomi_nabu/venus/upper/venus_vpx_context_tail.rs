use arm32::{
    branch, bx, data_immediate, data_register, mov_register, multiply_register,
    transfer_byte, transfer_multiple, transfer_word, transfer_word_preindexed_writeback,
    transfer_word_register, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0xc6e54;
pub const END: usize = 0xc77a4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 2300;
pub const DATA_BYTES: usize = 84;
pub const FUNCTION_STARTS: [usize; 6] = [START, 0xc7314, 0xc73b4, 0xc7604, 0xc7628, 0xc7788];
pub const CODE_RANGES: [(usize, usize); 6] = [
    (START, 0xc7300), (0xc7314, 0xc74a4), (0xc74b4, 0xc7558),
    (0xc7568, 0xc75f8), (0xc7604, 0xc7774), (0xc7788, END),
];
pub const FUNCTION_NAMES: [&str; 3] = [
    "VenusVPxStoreContextInfo", "VenusVP8BufManagement", "VenusVPXGetConcealmentType",
];

fn r(n: u8) -> R { R::new(n).unwrap() }

const DATA: &[(usize, u32)] = &[
    (0xc7300, 0xfbc08),
    (0xc7304, 0xe2788),
    (0xc7308, 0xfabdc),
    (0xc730c, 0xfbc2f),
    (0xc7310, 0xfbc45),
    (0xc74a4, 0xc74b4),
    (0xc74a8, 0xc74dc),
    (0xc74ac, 0xc7510),
    (0xc74b0, 0xc751c),
    (0xc7558, 0xc7568),
    (0xc755c, 0xc75c4),
    (0xc7560, 0xc7590),
    (0xc7564, 0xc75d0),
    (0xc75f8, 0xfbc5e),
    (0xc75fc, 0xfbc9a),
    (0xc7600, 0xe2788),
    (0xc7774, 0xfbcb0),
    (0xc7778, 0xfbce5),
    (0xc777c, 0xfbd1a),
    (0xc7780, 0xe2788),
    (0xc7784, 0xfbd35),
];

const EDGES: &[(usize, C, usize, bool)] = &[
    (0xc6e74, C::Ne, 0xc6ec0, false),
    (0xc6ec8, C::Ne, 0xc6efc, false),
    (0xc6ef8, C::Al, 0xc7044, false),
    (0xc6f04, C::Ne, 0xc7044, false),
    (0xc6f18, C::Ne, 0xc6f68, false),
    (0xc6f24, C::Ne, 0xc6f58, false),
    (0xc6f34, C::Ne, 0xc6f68, false),
    (0xc6f48, C::Eq, 0xc6f60, false),
    (0xc6f4c, C::Al, 0x7fed0, true),
    (0xc6f54, C::Al, 0xc6f64, false),
    (0xc6f5c, C::Al, 0xc6f68, false),
    (0xc6fbc, C::Eq, 0xc6fcc, false),
    (0xc6fc8, C::Eq, 0xc72e4, false),
    (0xc6fd4, C::Eq, 0xc700c, false),
    (0xc6fdc, C::Ne, 0xc7030, false),
    (0xc6fe8, C::Al, 0x2ec94, true),
    (0xc6ff4, C::Al, 0x2ec94, true),
    (0xc7000, C::Eq, 0xc7030, false),
    (0xc7004, C::Al, 0xc65ac, true),
    (0xc7008, C::Al, 0xc7030, false),
    (0xc7014, C::Al, 0x2ec94, true),
    (0xc7020, C::Al, 0x2ec94, true),
    (0xc702c, C::Ne, 0xc643c, true),
    (0xc7038, C::Eq, 0xc7044, false),
    (0xc7040, C::Al, 0x205c, true),
    (0xc7068, C::Eq, 0xc70a0, false),
    (0xc7074, C::Eq, 0xc7088, false),
    (0xc707c, C::Ne, 0xc7098, false),
    (0xc7080, C::Al, 0xc65ac, true),
    (0xc7084, C::Al, 0xc708c, false),
    (0xc7088, C::Al, 0xc643c, true),
    (0xc7094, C::Eq, 0xc70a0, false),
    (0xc709c, C::Al, 0x205c, true),
    (0xc70c4, C::Al, 0xd8be4, true),
    (0xc70d0, C::Ne, 0xc70e0, false),
    (0xc70dc, C::Eq, 0xc70f8, false),
    (0xc70f4, C::Al, 0xd8be4, true),
    (0xc7154, C::Ne, 0xc717c, false),
    (0xc7164, C::Ne, 0xc717c, false),
    (0xc71e0, C::Al, 0x471c, true),
    (0xc71f4, C::Al, 0x471c, true),
    (0xc7218, C::Al, 0x471c, true),
    (0xc722c, C::Ne, 0xc724c, false),
    (0xc723c, C::Eq, 0xc7280, false),
    (0xc7258, C::Eq, 0xc7274, false),
    (0xc7270, C::Al, 0x31ec, true),
    (0xc7284, C::Eq, 0xc72dc, false),
    (0xc729c, C::Al, 0x2ec94, true),
    (0xc72a8, C::Al, 0x2ec94, true),
    (0xc72d4, C::Al, 0x2a990, true),
    (0xc72d8, C::Al, 0xc7274, false),
    (0xc72e0, C::Al, 0xc724c, false),
    (0xc72ec, C::Eq, 0xc7044, false),
    (0xc72f8, C::Ne, 0xc7044, false),
    (0xc72fc, C::Al, 0xc6fcc, false),
    (0xc732c, C::Eq, 0xc7368, false),
    (0xc7334, C::Al, 0x7ffc8, true),
    (0xc7340, C::Eq, 0xc7354, false),
    (0xc7348, C::Al, 0x7fed0, true),
    (0xc7364, C::Al, 0xc73a8, false),
    (0xc7378, C::Al, 0x7ffc8, true),
    (0xc7384, C::Eq, 0xc7398, false),
    (0xc738c, C::Al, 0x7fed0, true),
    (0xc739c, C::Al, 0xc643c, true),
    (0xc73d4, C::Ne, 0xc740c, false),
    (0xc7420, C::Eq, 0xc7434, false),
    (0xc7430, C::Ne, 0xc7458, false),
    (0xc7444, C::Al, 0x2ec94, true),
    (0xc7454, C::Al, 0x2ec94, false),
    (0xc7468, C::Ne, 0xc748c, false),
    (0xc7480, C::Al, 0x7ffc8, true),
    (0xc7494, C::Hi, 0xc7540, false),
    (0xc74bc, C::Eq, 0xc7510, false),
    (0xc74d0, C::Eq, 0xc7540, false),
    (0xc74d8, C::Al, 0xc7508, false),
    (0xc74e8, C::Eq, 0xc7510, false),
    (0xc7504, C::Eq, 0xc7540, false),
    (0xc7508, C::Al, 0x7ffc8, true),
    (0xc750c, C::Al, 0xc7540, false),
    (0xc7518, C::Al, 0xc7540, false),
    (0xc7534, C::Al, 0x7ffc8, true),
    (0xc7548, C::Hi, 0xc75cc, false),
    (0xc7570, C::Eq, 0xc75c4, false),
    (0xc7584, C::Eq, 0xc75cc, false),
    (0xc758c, C::Al, 0xc75bc, false),
    (0xc759c, C::Eq, 0xc75c4, false),
    (0xc75c0, C::Al, 0x7ffc8, false),
    (0xc75e8, C::Al, 0x7ffc8, true),
    (0xc7618, C::Ne, 0xc7624, false),
    (0xc7620, C::Al, 0x7ffc8, false),
    (0xc7660, C::Al, 0x17b0, true),
    (0xc766c, C::Eq, 0xc76e0, false),
    (0xc767c, C::Eq, 0xc76fc, false),
    (0xc768c, C::Ne, 0xc76fc, false),
    (0xc7698, C::Ne, 0xc76fc, false),
    (0xc76a4, C::Ne, 0xc76fc, false),
    (0xc76c8, C::Al, 0x2ec94, true),
    (0xc76d4, C::Al, 0x2ec94, true),
    (0xc76ec, C::Eq, 0xc76fc, false),
    (0xc76f8, C::Eq, 0xc7700, false),
    (0xc7714, C::Eq, 0xc7734, false),
    (0xc771c, C::Ne, 0xc7744, false),
    (0xc7730, C::Al, 0xc7744, false),
    (0xc775c, C::Al, 0x2ec94, true),
    (0xc7768, C::Al, 0x2ec94, true),
];

const LITERALS: &[(usize, R, usize)] = &[
    (0xc6fe0, R::R1, 0xc7308),
    (0xc6fec, R::R1, 0xc7304),
    (0xc700c, R::R1, 0xc7300),
    (0xc7018, R::R1, 0xc7304),
    (0xc7288, R::R1, 0xc730c),
    (0xc728c, R::R2, 0xc7310),
    (0xc72a0, R::R1, 0xc7304),
    (0xc7434, R::R1, 0xc75f8),
    (0xc7438, R::R2, 0xc75fc),
    (0xc7448, R::R1, 0xc7600),
    (0xc7638, R::R2, 0xc7774),
    (0xc76ac, R::R1, 0xc7778),
    (0xc76b0, R::R2, 0xc777c),
    (0xc76cc, R::R1, 0xc7780),
    (0xc7754, R::R1, 0xc7784),
    (0xc7760, R::R1, 0xc7780),
];

const DISPATCHES: &[(usize, R, R)] = &[
    (0xc74a0, R::R0, R::R1),
    (0xc7554, R::R0, R::R1),
];

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VPX context address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        at % 4 == 0 && CODE_RANGES.iter().any(|&(start, end)| (start..end).contains(&at))
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("VPX context instruction outside CODE".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        if !EDGES.contains(&(self.at, condition, target, link)) {
            return Err("VPX context invalid branch edge".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        if !LITERALS.contains(&(self.at, rd, target)) {
            return Err("VPX context invalid literal load".into());
        }
        self.emit(transfer_word(C::Al, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn dispatch(&mut self, base: R, index: R) -> Result<(), String> {
        if !DISPATCHES.contains(&(self.at, base, index)) {
            return Err("VPX context invalid table dispatch".into());
        }
        self.emit(transfer_word_register(C::Al, true, R::PC, base, index, Shift::Lsl, 0)?)
    }

    fn data(&mut self, value: u32) -> Result<(), String> {
        if !DATA.contains(&(self.at, value)) {
            return Err("VPX context invalid data pin".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn venus_vpx_store_context_info(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc6e54)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.check(0xc6e58)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x18)?)?;
    c.check(0xc6e5c)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x10)?)?;
    c.check(0xc6e60)?;
    c.emit(mov_register(C::Al, r(9), r(0)))?;
    c.check(0xc6e64)?;
    c.emit(transfer_word(C::Al, true, r(5), r(9), 0)?)?;
    c.check(0xc6e68)?;
    c.emit(transfer_word(C::Al, true, r(6), r(9), 676)?)?;
    c.check(0xc6e6c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 16)?)?;
    c.check(0xc6e70)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.check(0xc6e74)?;
    c.branch(C::Ne, 0xc6ec0, false)?;
    c.check(0xc6e78)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 600)?)?;
    c.check(0xc6e7c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1416)?)?;
    c.check(0xc6e80)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 564)?)?;
    c.check(0xc6e84)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1304)?)?;
    c.check(0xc6e88)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 568)?)?;
    c.check(0xc6e8c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1308)?)?;
    c.check(0xc6e90)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 592)?)?;
    c.check(0xc6e94)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1924)?)?;
    c.check(0xc6e98)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 596)?)?;
    c.check(0xc6e9c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1920)?)?;
    c.check(0xc6ea0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 572)?)?;
    c.check(0xc6ea4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1296)?)?;
    c.check(0xc6ea8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 588)?)?;
    c.check(0xc6eac)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(6), 1288)?)?;
    c.check(0xc6eb0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 468)?)?;
    c.check(0xc6eb4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 112)?)?;
    c.check(0xc6eb8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 472)?)?;
    c.check(0xc6ebc)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 108)?)?;
    c.check(0xc6ec0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 576)?)?;
    c.check(0xc6ec4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0xc6ec8)?;
    c.branch(C::Ne, 0xc6efc, false)?;
    c.check(0xc6ecc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc6ed0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0xc6ed4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(9), 576)?)?;
    c.check(0xc6ed8)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 1976)?)?;
    c.check(0xc6edc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 936)?)?;
    c.check(0xc6ee0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0xc6ee4)?;
    c.emit(transfer_word(C::Eq, false, r(0), r(9), 412)?)?;
    c.check(0xc6ee8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1980)?)?;
    c.check(0xc6eec)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 372)?)?;
    c.check(0xc6ef0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 280)?)?;
    c.check(0xc6ef4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1272)?)?;
    c.check(0xc6ef8)?;
    c.branch(C::Al, 0xc7044, false)?;
    c.check(0xc6efc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 452)?)?;
    c.check(0xc6f00)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc6f04)?;
    c.branch(C::Ne, 0xc7044, false)?;
    c.check(0xc6f08)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc6f0c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 1980)?)?;
    c.check(0xc6f10)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1296)?)?;
    c.check(0xc6f14)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0xc6f18)?;
    c.branch(C::Ne, 0xc6f68, false)?;
    c.check(0xc6f1c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 1060)?)?;
    c.check(0xc6f20)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.check(0xc6f24)?;
    c.branch(C::Ne, 0xc6f58, false)?;
    c.check(0xc6f28)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 1355)?)?;
    c.check(0xc6f2c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc6f30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc6f34)?;
    c.branch(C::Ne, 0xc6f68, false)?;
    c.check(0xc6f38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0xc6f3c)?;
    c.emit(transfer_word(C::Al, false, r(7), r(6), 1980)?)?;
    c.check(0xc6f40)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1276)?)?;
    c.check(0xc6f44)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc6f48)?;
    c.branch(C::Eq, 0xc6f60, false)?;
    c.check(0xc6f4c)?;
    c.branch(C::Al, 0x7fed0, true)?;
    c.check(0xc6f50)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1296)?)?;
    c.check(0xc6f54)?;
    c.branch(C::Al, 0xc6f64, false)?;
    c.check(0xc6f58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc6f5c)?;
    c.branch(C::Al, 0xc6f68, false)?;
    c.check(0xc6f60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc6f64)?;
    c.emit(transfer_word(C::Al, false, r(7), r(6), 1276)?)?;
    c.check(0xc6f68)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 548)?)?;
    c.check(0xc6f6c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc6f70)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(6), 1348)?)?;
    c.check(0xc6f74)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0xc6f78)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 1976)?)?;
    c.check(0xc6f7c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 372)?)?;
    c.check(0xc6f80)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 280)?)?;
    c.check(0xc6f84)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 476)?)?;
    c.check(0xc6f88)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 104)?)?;
    c.check(0xc6f8c)?;
    c.emit(transfer_word(C::Al, true, r(2), r(5), 480)?)?;
    c.check(0xc6f90)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 100)?)?;
    c.check(0xc6f94)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 472)?)?;
    c.check(0xc6f98)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 1280)?)?;
    c.check(0xc6f9c)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 464)?)?;
    c.check(0xc6fa0)?;
    c.emit(transfer_word(C::Al, false, r(2), r(6), 1284)?)?;
    c.check(0xc6fa4)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 472)?)?;
    c.check(0xc6fa8)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 464)?)?;
    c.check(0xc6fac)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 884)?)?;
    c.check(0xc6fb0)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(6), 1390)?)?;
    c.check(0xc6fb4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 885)?)?;
    c.check(0xc6fb8)?;
    c.emit(transfer_byte(C::Al, false, r(1), r(6), 1388)?)?;
    c.check(0xc6fbc)?;
    c.branch(C::Eq, 0xc6fcc, false)?;
    c.check(0xc6fc0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 1355)?)?;
    c.check(0xc6fc4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc6fc8)?;
    c.branch(C::Eq, 0xc72e4, false)?;
    c.check(0xc6fcc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 16)?)?;
    c.check(0xc6fd0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1000)?)?;
    c.check(0xc6fd4)?;
    c.branch(C::Eq, 0xc700c, false)?;
    c.check(0xc6fd8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.check(0xc6fdc)?;
    c.branch(C::Ne, 0xc7030, false)?;
    c.check(0xc6fe0)?;
    c.literal(r(1), 0xc7308)?;
    c.check(0xc6fe4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0xc6fe8)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc6fec)?;
    c.literal(r(1), 0xc7304)?;
    c.check(0xc6ff0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0xc6ff4)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc6ff8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1272)?)?;
    c.check(0xc6ffc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7000)?;
    c.branch(C::Eq, 0xc7030, false)?;
    c.check(0xc7004)?;
    c.branch(C::Al, 0xc65ac, true)?;
    c.check(0xc7008)?;
    c.branch(C::Al, 0xc7030, false)?;
    c.check(0xc700c)?;
    c.literal(r(1), 0xc7300)?;
    c.check(0xc7010)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0xc7014)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc7018)?;
    c.literal(r(1), 0xc7304)?;
    c.check(0xc701c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0xc7020)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc7024)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1272)?)?;
    c.check(0xc7028)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc702c)?;
    c.branch(C::Ne, 0xc643c, true)?;
    c.check(0xc7030)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1272)?)?;
    c.check(0xc7034)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7038)?;
    c.branch(C::Eq, 0xc7044, false)?;
    c.check(0xc703c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe40)?)?;
    c.check(0xc7040)?;
    c.branch(C::Al, 0x205c, true)?;
    c.check(0xc7044)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0)?)?;
    c.check(0xc7048)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 2704)?)?;
    c.check(0xc704c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 0)?)?;
    c.check(0xc7050)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 8)?)?;
    c.check(0xc7054)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 4)?)?;
    c.check(0xc7058)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 12)?)?;
    c.check(0xc705c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 8)?)?;
    c.check(0xc7060)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1280)?)?;
    c.check(0xc7064)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7068)?;
    c.branch(C::Eq, 0xc70a0, false)?;
    c.check(0xc706c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 16)?)?;
    c.check(0xc7070)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1000)?)?;
    c.check(0xc7074)?;
    c.branch(C::Eq, 0xc7088, false)?;
    c.check(0xc7078)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x4000)?)?;
    c.check(0xc707c)?;
    c.branch(C::Ne, 0xc7098, false)?;
    c.check(0xc7080)?;
    c.branch(C::Al, 0xc65ac, true)?;
    c.check(0xc7084)?;
    c.branch(C::Al, 0xc708c, false)?;
    c.check(0xc7088)?;
    c.branch(C::Al, 0xc643c, true)?;
    c.check(0xc708c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 1280)?)?;
    c.check(0xc7090)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7094)?;
    c.branch(C::Eq, 0xc70a0, false)?;
    c.check(0xc7098)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe40)?)?;
    c.check(0xc709c)?;
    c.branch(C::Al, 0x205c, true)?;
    c.check(0xc70a0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 484)?)?;
    c.check(0xc70a4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0x7c)?)?;
    c.check(0xc70a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0xc70ac)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 120)?)?;
    c.check(0xc70b0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 488)?)?;
    c.check(0xc70b4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 116)?)?;
    c.check(0xc70b8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 400)?)?;
    c.check(0xc70bc)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 376)?)?;
    c.check(0xc70c0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0xc70c4)?;
    c.branch(C::Al, 0xd8be4, true)?;
    c.check(0xc70c8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 2320)?)?;
    c.check(0xc70cc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc70d0)?;
    c.branch(C::Ne, 0xc70e0, false)?;
    c.check(0xc70d4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 972)?)?;
    c.check(0xc70d8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc70dc)?;
    c.branch(C::Eq, 0xc70f8, false)?;
    c.check(0xc70e0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 412)?)?;
    c.check(0xc70e4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(6), 0xc4)?)?;
    c.check(0xc70e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0xc70ec)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 380)?)?;
    c.check(0xc70f0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0xc70f4)?;
    c.branch(C::Al, 0xd8be4, true)?;
    c.check(0xc70f8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 2308)?)?;
    c.check(0xc70fc)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 524)?)?;
    c.check(0xc7100)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 1092)?)?;
    c.check(0xc7104)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 276)?)?;
    c.check(0xc7108)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 180)?)?;
    c.check(0xc710c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 364)?)?;
    c.check(0xc7110)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 236)?)?;
    c.check(0xc7114)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 368)?)?;
    c.check(0xc7118)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 432)?)?;
    c.check(0xc711c)?;
    c.emit(data_immediate(C::Al, Op::Bic, r(1), r(0), 0xff)?)?;
    c.check(0xc7120)?;
    c.emit(data_immediate(C::Al, Op::And, r(0), r(0), 0xff)?)?;
    c.check(0xc7124)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 12)?)?;
    c.check(0xc7128)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 436)?)?;
    c.check(0xc712c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 16)?)?;
    c.check(0xc7130)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 444)?)?;
    c.check(0xc7134)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 20)?)?;
    c.check(0xc7138)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 440)?)?;
    c.check(0xc713c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 32)?)?;
    c.check(0xc7140)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 36)?)?;
    c.check(0xc7144)?;
    c.emit(transfer_word(C::Al, true, r(8), r(9), 448)?)?;
    c.check(0xc7148)?;
    c.emit(transfer_word(C::Al, false, r(8), r(6), 24)?)?;
    c.check(0xc714c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 16)?)?;
    c.check(0xc7150)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x4000)?)?;
    c.check(0xc7154)?;
    c.branch(C::Ne, 0xc717c, false)?;
    c.check(0xc7158)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 2068)?)?;
    c.check(0xc715c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0xc7160)?;
    c.emit(data_immediate(C::Eq, Op::Cmp, r(0), r(8), 0x1)?)?;
    c.check(0xc7164)?;
    c.branch(C::Ne, 0xc717c, false)?;
    c.check(0xc7168)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 936)?)?;
    c.check(0xc716c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.check(0xc7170)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7174)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(8), r(0), 0x0)?)?;
    c.check(0xc7178)?;
    c.emit(transfer_word(C::Eq, false, r(8), r(6), 24)?)?;
    c.check(0xc717c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 420)?)?;
    c.check(0xc7180)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(9), 0xc8)?)?;
    c.check(0xc7184)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x94)?)?;
    c.check(0xc7188)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 88)?)?;
    c.check(0xc718c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 408)?)?;
    c.check(0xc7190)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 76)?)?;
    c.check(0xc7194)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 384)?)?;
    c.check(0xc7198)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 52)?)?;
    c.check(0xc719c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 388)?)?;
    c.check(0xc71a0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 56)?)?;
    c.check(0xc71a4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 392)?)?;
    c.check(0xc71a8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 60)?)?;
    c.check(0xc71ac)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 412)?)?;
    c.check(0xc71b0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 80)?)?;
    c.check(0xc71b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 416)?)?;
    c.check(0xc71b8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 84)?)?;
    c.check(0xc71bc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 400)?)?;
    c.check(0xc71c0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 68)?)?;
    c.check(0xc71c4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 396)?)?;
    c.check(0xc71c8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 64)?)?;
    c.check(0xc71cc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 404)?)?;
    c.check(0xc71d0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 72)?)?;
    c.check(0xc71d4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 456)?)?;
    c.check(0xc71d8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 392)?)?;
    c.check(0xc71dc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x210)?)?;
    c.check(0xc71e0)?;
    c.branch(C::Al, 0x471c, true)?;
    c.check(0xc71e4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0x1b8)?)?;
    c.check(0xc71e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0xcc)?)?;
    c.check(0xc71ec)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(0), 0x800)?)?;
    c.check(0xc71f0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x2a4)?)?;
    c.check(0xc71f4)?;
    c.branch(C::Al, 0x471c, true)?;
    c.check(0xc71f8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x2f8)?)?;
    c.check(0xc71fc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(9), 0x2c4)?)?;
    c.check(0xc7200)?;
    c.emit(data_immediate(C::Al, Op::Add, r(12), r(0), 0x400)?)?;
    c.check(0xc7204)?;
    c.emit(transfer_multiple(C::Al, true, r(1), r(0).bit() | r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0xc7208)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(9), 0x2dc)?)?;
    c.check(0xc720c)?;
    c.emit(transfer_multiple(C::Al, false, r(12), r(0).bit() | r(2).bit() | r(3).bit() | r(4).bit() | r(5).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0xc7210)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(6), 0x710)?)?;
    c.check(0xc7214)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x70)?)?;
    c.check(0xc7218)?;
    c.branch(C::Al, 0x471c, true)?;
    c.check(0xc721c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 936)?)?;
    c.check(0xc7220)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0xc7224)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0x1)?)?;
    c.check(0xc7228)?;
    c.emit(transfer_word(C::Al, false, r(0), r(6), 2060)?)?;
    c.check(0xc722c)?;
    c.branch(C::Ne, 0xc724c, false)?;
    c.check(0xc7230)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 420)?)?;
    c.check(0xc7234)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 412)?)?;
    c.check(0xc7238)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0xc723c)?;
    c.branch(C::Eq, 0xc7280, false)?;
    c.check(0xc7240)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc7244)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0xc7248)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x2)?)?;
    c.check(0xc724c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.check(0xc7250)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7254)?;
    c.emit(transfer_word(C::Al, false, r(1), r(6), 28)?)?;
    c.check(0xc7258)?;
    c.branch(C::Eq, 0xc7274, false)?;
    c.check(0xc725c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0xc7260)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x174)?)?;
    c.check(0xc7264)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x44)?)?;
    c.check(0xc7268)?;
    c.emit(transfer_word(C::Al, false, r(1), r(9), 444)?)?;
    c.check(0xc726c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0xc7270)?;
    c.branch(C::Al, 0x31ec, true)?;
    c.check(0xc7274)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0xc7278)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x18)?)?;
    c.check(0xc727c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc7280)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc7284)?;
    c.branch(C::Eq, 0xc72dc, false)?;
    c.check(0xc7288)?;
    c.literal(r(1), 0xc730c)?;
    c.check(0xc728c)?;
    c.literal(r(2), 0xc7310)?;
    c.check(0xc7290)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x9b)?)?;
    c.check(0xc7294)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0xc7298)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x100)?)?;
    c.check(0xc729c)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc72a0)?;
    c.literal(r(1), 0xc7304)?;
    c.check(0xc72a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0xc72a8)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc72ac)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0)?)?;
    c.check(0xc72b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x3)?)?;
    c.check(0xc72b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0xc72b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0xc72bc)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x1000)?)?;
    c.check(0xc72c0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.check(0xc72c4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 128)?)?;
    c.check(0xc72c8)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 0)?)?;
    c.check(0xc72cc)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 4)?)?;
    c.check(0xc72d0)?;
    c.emit(transfer_word(C::Al, false, r(5), R::SP, 8)?)?;
    c.check(0xc72d4)?;
    c.branch(C::Al, 0x2a990, true)?;
    c.check(0xc72d8)?;
    c.branch(C::Al, 0xc7274, false)?;
    c.check(0xc72dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3)?)?;
    c.check(0xc72e0)?;
    c.branch(C::Al, 0xc724c, false)?;
    c.check(0xc72e4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 1314)?)?;
    c.check(0xc72e8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc72ec)?;
    c.branch(C::Eq, 0xc7044, false)?;
    c.check(0xc72f0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 1312)?)?;
    c.check(0xc72f4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc72f8)?;
    c.branch(C::Ne, 0xc7044, false)?;
    c.check(0xc72fc)?;
    c.branch(C::Al, 0xc6fcc, false)?;
    c.check(0xc7300)?;
    c.data(0xfbc08)?;
    c.check(0xc7304)?;
    c.data(0xe2788)?;
    c.check(0xc7308)?;
    c.data(0xfabdc)?;
    c.check(0xc730c)?;
    c.data(0xfbc2f)?;
    c.check(0xc7310)?;
    c.data(0xfbc45)?;
    Ok(())
}

fn recycle_vp8_reference(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc7314)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.check(0xc7318)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x8)?)?;
    c.check(0xc731c)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0xc7320)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0xc7324)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 2045)?)?;
    c.check(0xc7328)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc732c)?;
    c.branch(C::Eq, 0xc7368, false)?;
    c.check(0xc7330)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 1276)?)?;
    c.check(0xc7334)?;
    c.branch(C::Al, 0x7ffc8, true)?;
    c.check(0xc7338)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 468)?)?;
    c.check(0xc733c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7340)?;
    c.branch(C::Eq, 0xc7354, false)?;
    c.check(0xc7344)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 460)?)?;
    c.check(0xc7348)?;
    c.branch(C::Al, 0x7fed0, true)?;
    c.check(0xc734c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc7350)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 460)?)?;
    c.check(0xc7354)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 1276)?)?;
    c.check(0xc7358)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 460)?)?;
    c.check(0xc735c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(5), 0xf8)?)?;
    c.check(0xc7360)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(0), 0x400)?)?;
    c.check(0xc7364)?;
    c.branch(C::Al, 0xc73a8, false)?;
    c.check(0xc7368)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 1296)?)?;
    c.check(0xc736c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7370)?;
    c.emit(transfer_multiple(C::Ne, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc7374)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 1276)?)?;
    c.check(0xc7378)?;
    c.branch(C::Al, 0x7ffc8, true)?;
    c.check(0xc737c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 468)?)?;
    c.check(0xc7380)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7384)?;
    c.branch(C::Eq, 0xc7398, false)?;
    c.check(0xc7388)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 460)?)?;
    c.check(0xc738c)?;
    c.branch(C::Al, 0x7fed0, true)?;
    c.check(0xc7390)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc7394)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 460)?)?;
    c.check(0xc7398)?;
    c.emit(transfer_word_preindexed_writeback(C::Al, true, r(0), r(5), 1272)?)?;
    c.check(0xc739c)?;
    c.branch(C::Al, 0xc643c, true)?;
    c.check(0xc73a0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.check(0xc73a4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 460)?)?;
    c.check(0xc73a8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.check(0xc73ac)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 468)?)?;
    c.check(0xc73b0)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn venus_vp8_buf_management(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc73b4)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.check(0xc73b8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x10)?)?;
    c.check(0xc73bc)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0xc73c0)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.check(0xc73c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc73c8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1984)?)?;
    c.check(0xc73cc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 24)?)?;
    c.check(0xc73d0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0xc73d4)?;
    c.branch(C::Ne, 0xc740c, false)?;
    c.check(0xc73d8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 608)?)?;
    c.check(0xc73dc)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 1440)?)?;
    c.check(0xc73e0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 612)?)?;
    c.check(0xc73e4)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 1444)?)?;
    c.check(0xc73e8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 616)?)?;
    c.check(0xc73ec)?;
    c.emit(transfer_word(C::Al, false, r(1), r(4), 1448)?)?;
    c.check(0xc73f0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 640)?)?;
    c.check(0xc73f4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 608)?)?;
    c.check(0xc73f8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 644)?)?;
    c.check(0xc73fc)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 612)?)?;
    c.check(0xc7400)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 648)?)?;
    c.check(0xc7404)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 616)?)?;
    c.check(0xc7408)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc740c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1440)?)?;
    c.check(0xc7410)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1444)?)?;
    c.check(0xc7414)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1448)?)?;
    c.check(0xc7418)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1976)?)?;
    c.check(0xc741c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0xc7420)?;
    c.branch(C::Eq, 0xc7434, false)?;
    c.check(0xc7424)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.check(0xc7428)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 1092)?)?;
    c.check(0xc742c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0xc7430)?;
    c.branch(C::Ne, 0xc7458, false)?;
    c.check(0xc7434)?;
    c.literal(r(1), 0xc75f8)?;
    c.check(0xc7438)?;
    c.literal(r(2), 0xc75fc)?;
    c.check(0xc743c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0xc7440)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x21c)?)?;
    c.check(0xc7444)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc7448)?;
    c.literal(r(1), 0xc7600)?;
    c.check(0xc744c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0xc7450)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc7454)?;
    c.branch(C::Al, 0x2ec94, false)?;
    c.check(0xc7458)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 2048)?)?;
    c.check(0xc745c)?;
    c.emit(transfer_word(C::Al, true, r(6), r(5), 608)?)?;
    c.check(0xc7460)?;
    c.emit(transfer_word(C::Al, true, r(7), r(5), 640)?)?;
    c.check(0xc7464)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.check(0xc7468)?;
    c.branch(C::Ne, 0xc748c, false)?;
    c.check(0xc746c)?;
    c.emit(transfer_word(C::Al, false, r(6), r(4), 1440)?)?;
    c.check(0xc7470)?;
    c.emit(transfer_word(C::Al, false, r(4), r(5), 640)?)?;
    c.check(0xc7474)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1268)?)?;
    c.check(0xc7478)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 608)?)?;
    c.check(0xc747c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1268)?)?;
    c.check(0xc7480)?;
    c.branch(C::Al, 0x7ffc8, true)?;
    c.check(0xc7484)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc7488)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1984)?)?;
    c.check(0xc748c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 2056)?)?;
    c.check(0xc7490)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.check(0xc7494)?;
    c.branch(C::Hi, 0xc7540, false)?;
    c.check(0xc7498)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.check(0xc749c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), R::PC, 0x0)?)?;
    c.check(0xc74a0)?;
    c.dispatch(r(0), r(1))?;
    c.check(0xc74a4)?;
    c.data(0xc74b4)?;
    c.check(0xc74a8)?;
    c.data(0xc74dc)?;
    c.check(0xc74ac)?;
    c.data(0xc7510)?;
    c.check(0xc74b0)?;
    c.data(0xc751c)?;
    c.check(0xc74b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 616)?)?;
    c.check(0xc74b8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.check(0xc74bc)?;
    c.branch(C::Eq, 0xc7510, false)?;
    c.check(0xc74c0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0xc74c4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1448)?)?;
    c.check(0xc74c8)?;
    c.emit(transfer_word(C::Al, false, r(7), r(5), 648)?)?;
    c.check(0xc74cc)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 616)?)?;
    c.check(0xc74d0)?;
    c.branch(C::Eq, 0xc7540, false)?;
    c.check(0xc74d4)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0xc74d8)?;
    c.branch(C::Al, 0xc7508, false)?;
    c.check(0xc74dc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 612)?)?;
    c.check(0xc74e0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 616)?)?;
    c.check(0xc74e4)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0xc74e8)?;
    c.branch(C::Eq, 0xc7510, false)?;
    c.check(0xc74ec)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1448)?)?;
    c.check(0xc74f0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 644)?)?;
    c.check(0xc74f4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 648)?)?;
    c.check(0xc74f8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 612)?)?;
    c.check(0xc74fc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc7500)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 616)?)?;
    c.check(0xc7504)?;
    c.branch(C::Eq, 0xc7540, false)?;
    c.check(0xc7508)?;
    c.branch(C::Al, 0x7ffc8, true)?;
    c.check(0xc750c)?;
    c.branch(C::Al, 0xc7540, false)?;
    c.check(0xc7510)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc7514)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1448)?)?;
    c.check(0xc7518)?;
    c.branch(C::Al, 0xc7540, false)?;
    c.check(0xc751c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 616)?)?;
    c.check(0xc7520)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1448)?)?;
    c.check(0xc7524)?;
    c.emit(transfer_word(C::Al, false, r(4), r(5), 648)?)?;
    c.check(0xc7528)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1268)?)?;
    c.check(0xc752c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 616)?)?;
    c.check(0xc7530)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1268)?)?;
    c.check(0xc7534)?;
    c.branch(C::Al, 0x7ffc8, true)?;
    c.check(0xc7538)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc753c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1984)?)?;
    c.check(0xc7540)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 2052)?)?;
    c.check(0xc7544)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x3)?)?;
    c.check(0xc7548)?;
    c.branch(C::Hi, 0xc75cc, false)?;
    c.check(0xc754c)?;
    c.emit(data_register(C::Al, Op::Mov, r(0), r(0), r(0), Shift::Lsl, 2, false)?)?;
    c.check(0xc7550)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), R::PC, 0x0)?)?;
    c.check(0xc7554)?;
    c.dispatch(r(0), r(1))?;
    c.check(0xc7558)?;
    c.data(0xc7568)?;
    c.check(0xc755c)?;
    c.data(0xc75c4)?;
    c.check(0xc7560)?;
    c.data(0xc7590)?;
    c.check(0xc7564)?;
    c.data(0xc75d0)?;
    c.check(0xc7568)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 612)?)?;
    c.check(0xc756c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.check(0xc7570)?;
    c.branch(C::Eq, 0xc75c4, false)?;
    c.check(0xc7574)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0xc7578)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1444)?)?;
    c.check(0xc757c)?;
    c.emit(transfer_word(C::Al, false, r(7), r(5), 644)?)?;
    c.check(0xc7580)?;
    c.emit(transfer_word(C::Al, false, r(6), r(5), 612)?)?;
    c.check(0xc7584)?;
    c.branch(C::Eq, 0xc75cc, false)?;
    c.check(0xc7588)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0xc758c)?;
    c.branch(C::Al, 0xc75bc, false)?;
    c.check(0xc7590)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 612)?)?;
    c.check(0xc7594)?;
    c.emit(transfer_word(C::Al, true, r(1), r(5), 616)?)?;
    c.check(0xc7598)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0xc759c)?;
    c.branch(C::Eq, 0xc75c4, false)?;
    c.check(0xc75a0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1444)?)?;
    c.check(0xc75a4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 648)?)?;
    c.check(0xc75a8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 644)?)?;
    c.check(0xc75ac)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 616)?)?;
    c.check(0xc75b0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc75b4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 612)?)?;
    c.check(0xc75b8)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc75bc)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc75c0)?;
    c.branch(C::Al, 0x7ffc8, false)?;
    c.check(0xc75c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc75c8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1444)?)?;
    c.check(0xc75cc)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc75d0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 612)?)?;
    c.check(0xc75d4)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1444)?)?;
    c.check(0xc75d8)?;
    c.emit(transfer_word(C::Al, false, r(4), r(5), 644)?)?;
    c.check(0xc75dc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1268)?)?;
    c.check(0xc75e0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(5), 612)?)?;
    c.check(0xc75e4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1268)?)?;
    c.check(0xc75e8)?;
    c.branch(C::Al, 0x7ffc8, true)?;
    c.check(0xc75ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc75f0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 1984)?)?;
    c.check(0xc75f4)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc75f8)?;
    c.data(0xfbc5e)?;
    c.check(0xc75fc)?;
    c.data(0xfbc9a)?;
    c.check(0xc7600)?;
    c.data(0xe2788)?;
    Ok(())
}

fn retain_managed_reference(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc7604)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 24)?)?;
    c.check(0xc7608)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc760c)?;
    c.emit(bx(C::Ne, R::LR))?;
    c.check(0xc7610)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 1984)?)?;
    c.check(0xc7614)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0xc7618)?;
    c.branch(C::Ne, 0xc7624, false)?;
    c.check(0xc761c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 284)?)?;
    c.check(0xc7620)?;
    c.branch(C::Al, 0x7ffc8, false)?;
    c.check(0xc7624)?;
    c.emit(bx(C::Al, R::LR))?;
    Ok(())
}

fn venus_vpx_get_concealment_type(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc7628)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.check(0xc762c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x8)?)?;
    c.check(0xc7630)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.check(0xc7634)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0xc7638)?;
    c.literal(r(2), 0xc7774)?;
    c.check(0xc763c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.check(0xc7640)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0xc7644)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0xc7648)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc764c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0xc7650)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0xc7654)?;
    c.emit(data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0xc7658)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xd6)?)?;
    c.check(0xc765c)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.check(0xc7660)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0xc7664)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 1296)?)?;
    c.check(0xc7668)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0xc766c)?;
    c.branch(C::Eq, 0xc76e0, false)?;
    c.check(0xc7670)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 276)?)?;
    c.check(0xc7674)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc7678)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0xc767c)?;
    c.branch(C::Eq, 0xc76fc, false)?;
    c.check(0xc7680)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 2028)?)?;
    c.check(0xc7684)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc7688)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc768c)?;
    c.branch(C::Ne, 0xc76fc, false)?;
    c.check(0xc7690)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 2032)?)?;
    c.check(0xc7694)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc7698)?;
    c.branch(C::Ne, 0xc76fc, false)?;
    c.check(0xc769c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 2036)?)?;
    c.check(0xc76a0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc76a4)?;
    c.branch(C::Ne, 0xc76fc, false)?;
    c.check(0xc76a8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 60)?)?;
    c.check(0xc76ac)?;
    c.literal(r(1), 0xc7778)?;
    c.check(0xc76b0)?;
    c.literal(r(2), 0xc777c)?;
    c.check(0xc76b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0xf1)?)?;
    c.check(0xc76b8)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x200)?)?;
    c.check(0xc76bc)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(0), 0x8)?)?;
    c.check(0xc76c0)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 60)?)?;
    c.check(0xc76c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0xc76c8)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc76cc)?;
    c.literal(r(1), 0xc7780)?;
    c.check(0xc76d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8)?)?;
    c.check(0xc76d4)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc76d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0xc76dc)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc76e0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 388)?)?;
    c.check(0xc76e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0xc76e8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0xc76ec)?;
    c.branch(C::Eq, 0xc76fc, false)?;
    c.check(0xc76f0)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(4), 1312)?)?;
    c.check(0xc76f4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0xc76f8)?;
    c.branch(C::Eq, 0xc7700, false)?;
    c.check(0xc76fc)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc7700)?;
    c.emit(transfer_word(C::Al, true, r(5), r(5), 0)?)?;
    c.check(0xc7704)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 108)?)?;
    c.check(0xc7708)?;
    c.emit(transfer_word(C::Al, true, r(3), r(4), 112)?)?;
    c.check(0xc770c)?;
    c.emit(transfer_word(C::Al, true, r(5), r(5), 16)?)?;
    c.check(0xc7710)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x1000)?)?;
    c.check(0xc7714)?;
    c.branch(C::Eq, 0xc7734, false)?;
    c.check(0xc7718)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x4000)?)?;
    c.check(0xc771c)?;
    c.branch(C::Ne, 0xc7744, false)?;
    c.check(0xc7720)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0x3f)?)?;
    c.check(0xc7724)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x3f)?)?;
    c.check(0xc7728)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsr, 6, false)?)?;
    c.check(0xc772c)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(3), Shift::Lsr, 6, false)?)?;
    c.check(0xc7730)?;
    c.branch(C::Al, 0xc7744, false)?;
    c.check(0xc7734)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(2), 0xf)?)?;
    c.check(0xc7738)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0xf)?)?;
    c.check(0xc773c)?;
    c.emit(data_register(C::Al, Op::Mov, r(2), r(0), r(2), Shift::Lsr, 4, false)?)?;
    c.check(0xc7740)?;
    c.emit(data_register(C::Al, Op::Mov, r(3), r(0), r(3), Shift::Lsr, 4, false)?)?;
    c.check(0xc7744)?;
    c.emit(multiply_register(C::Al, r(5), r(3), r(2))?)?;
    c.check(0xc7748)?;
    c.emit(data_register(C::Al, Op::Add, r(2), r(5), r(5), Shift::Lsl, 1, false)?)?;
    c.check(0xc774c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsr, 2, false)?)?;
    c.check(0xc7750)?;
    c.emit(transfer_multiple(C::Cc, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc7754)?;
    c.literal(r(1), 0xc7784)?;
    c.check(0xc7758)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0xc775c)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc7760)?;
    c.literal(r(1), 0xc7780)?;
    c.check(0xc7764)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x4)?)?;
    c.check(0xc7768)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0xc776c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.check(0xc7770)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0xc7774)?;
    c.data(0xfbcb0)?;
    c.check(0xc7778)?;
    c.data(0xfbce5)?;
    c.check(0xc777c)?;
    c.data(0xfbd1a)?;
    c.check(0xc7780)?;
    c.data(0xe2788)?;
    c.check(0xc7784)?;
    c.data(0xfbd35)?;
    Ok(())
}

fn copy_reference_availability(c: &mut TypedA32) -> Result<(), String> {
    c.check(0xc7788)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 640)?)?;
    c.check(0xc778c)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 2028)?)?;
    c.check(0xc7790)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 644)?)?;
    c.check(0xc7794)?;
    c.emit(transfer_word(C::Al, false, r(2), r(1), 2032)?)?;
    c.check(0xc7798)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 648)?)?;
    c.check(0xc779c)?;
    c.emit(transfer_word(C::Al, false, r(0), r(1), 2036)?)?;
    c.check(0xc77a0)?;
    c.emit(bx(C::Al, R::LR))?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    venus_vpx_store_context_info(&mut c)?;
    recycle_vp8_reference(&mut c)?;
    venus_vp8_buf_management(&mut c)?;
    retain_managed_reference(&mut c)?;
    venus_vpx_get_concealment_type(&mut c)?;
    copy_reference_availability(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != CODE_BYTES + DATA_BYTES {
        return Err("VPX context size mismatch".into());
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
    fn structure_and_pins() {
        let bytes = fragment().unwrap();
        assert_eq!((START, END, END - START), (0xc6e54, 0xc77a4, 2384));
        assert_eq!((CODE_BYTES, DATA_BYTES), (575 * 4, 21 * 4));
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(FUNCTION_STARTS, [START, 0xc7314, 0xc73b4, 0xc7604, 0xc7628, 0xc7788]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 575);
        assert_eq!(EDGES.len(), 105);
        assert_eq!(LITERALS.len(), 16);
        for &(at, value) in DATA {
            assert!(!TypedA32::is_code(at));
            assert_eq!(word(&bytes, at), value);
        }
        for (at, targets) in [
            (0xc74a4, [0xc74b4, 0xc74dc, 0xc7510, 0xc751c]),
            (0xc7558, [0xc7568, 0xc75c4, 0xc7590, 0xc75d0]),
        ] {
            for (i, target) in targets.into_iter().enumerate() {
                assert_eq!(word(&bytes, at + i * 4), target as u32);
                assert!(TypedA32::is_code(target));
            }
        }
        for &(at, condition, target, link) in EDGES {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        assert_eq!(word(&bytes, 0xc7744), multiply_register(C::Al, r(5), r(3), r(2)).unwrap());
        assert_eq!(word(&bytes, 0xc7748), data_register(C::Al, Op::Add, r(2), r(5), r(5), Shift::Lsl, 1, false).unwrap());
        assert_eq!(word(&bytes, 0xc774c), data_register(C::Al, Op::Cmp, r(0), r(1), r(2), Shift::Lsr, 2, false).unwrap());
        assert_eq!(word(&bytes, 0xc7750), transfer_multiple(C::Cc, true, R::SP,
            r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true).unwrap());
        for at in [0xc7280, 0xc72e4, 0xc7374, 0xc740c, 0xc75d0, 0xc76e0, 0xc7700] {
            assert!(TypedA32::is_code(at));
        }
    }

    #[test]
    fn exact_code_and_data_guards() {
        for at in (START - 4..END + 8).chain([0, usize::MAX]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            let valid = TypedA32::is_code(at);
            assert_eq!(c.emit(bx(C::Al, R::LR)).is_ok(), valid);
            assert_eq!(c.bytes.len(), if valid { 4 } else { 0 });
            assert_eq!(c.at, if valid { at + 4 } else { at });
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(0).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        for &(at, value) in DATA {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(value ^ 1).is_err());
            assert!(c.bytes.is_empty());
            c.data(value).unwrap();
            assert_eq!(c.bytes, value.to_le_bytes());
        }
        assert!(TypedA32 { at: START, bytes: Vec::new() }.check(END).is_err());
    }

    #[test]
    fn exact_control_flow_guards() {
        for &(at, condition, target, link) in EDGES {
            for (source, cc, dest, linked) in [
                (at, condition, target, !link),
                (at, if condition == C::Al { C::Eq } else { C::Al }, target, link),
                (at, condition, target + 4, link),
                (at + 1, condition, target, link),
                (END, condition, target, link),
            ] {
                let mut c = TypedA32 { at: source, bytes: Vec::new() };
                assert!(c.branch(cc, dest, linked).is_err());
                assert!(c.bytes.is_empty());
                assert_eq!(c.at, source);
            }
        }
        for &(at, rd, target) in LITERALS {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.literal(R::PC, target).is_err());
            assert!(c.literal(rd, target + 4).is_err());
            assert!(c.bytes.is_empty());
            c.literal(rd, target).unwrap();
        }
        for &(at, base, index) in DISPATCHES {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.dispatch(index, base).is_err());
            assert!(c.bytes.is_empty());
            c.dispatch(base, index).unwrap();
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
        for (at, name) in [0xfbc45, 0xfbc9a, 0xfbd1a].into_iter().zip(FUNCTION_NAMES) {
            let offset = at + FILE_MAPPING_DELTA;
            assert_eq!(&reference[offset..offset + name.len()], name.as_bytes());
            assert_eq!(reference[offset + name.len()], 0);
        }
    }
}
