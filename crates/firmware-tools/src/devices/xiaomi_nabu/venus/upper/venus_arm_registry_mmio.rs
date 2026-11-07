use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    branch_exchange_with_link, bx, data_immediate, data_immediate_with_flags,
    data_register, multiply_accumulate, multiply_register, transfer_multiple,
    transfer_word, transfer_word_preindexed_writeback, transfer_word_register_offset,
};

pub const START: usize = 0x84574;
pub const END: usize = 0x84c54;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1620;
pub const DATA_BYTES: usize = 140;

const LITERALS: [(usize, u32); 35] = [
    (0x848e4, 0x10cfa4),
    (0x848fc, 0x10cfa4),
    (0x84900, 0xc5acce55),
    (0x84918, 0x10cfa4),
    (0x8491c, 0x3a5331aa),
    (0x84944, 0x10cfa4),
    (0x8496c, 0x10cfa4),
    (0x84970, 0xfffc0003),
    (0x84990, 0x10cfa4),
    (0x849b0, 0x10cfa4),
    (0x849d0, 0x10cfa4),
    (0x849ec, 0x10cfa4),
    (0x84a04, 0x10cfa4),
    (0x84a08, 0x3020100),
    (0x84a28, 0x10cfa4),
    (0x84a48, 0x10cfa4),
    (0x84a7c, 0x10cfa4),
    (0x84a80, 0xfffc01ff),
    (0x84aa0, 0x10cfa4),
    (0x84ac0, 0x10cfa4),
    (0x84af4, 0x10cfa4),
    (0x84af8, 0xf803ffff),
    (0x84b10, 0x10cfa4),
    (0x84b28, 0x10cfa4),
    (0x84b48, 0x10cfa4),
    (0x84b68, 0x10cfa4),
    (0x84b9c, 0x10cfa4),
    (0x84bb8, 0x10cfa4),
    (0x84bd4, 0x10cfa4),
    (0x84bf4, 0x10cfa4),
    (0x84c08, 0x10cfa4),
    (0x84c1c, 0x10cfa4),
    (0x84c30, 0x10cfa4),
    (0x84c44, 0x10cfa4),
    (0x84c50, 0x10cfb4),
];

const LITERAL_REFERENCES: [(usize, usize); 35] = [
    (0x848d8, 0x848e4),
    (0x848e8, 0x848fc),
    (0x848f0, 0x84900),
    (0x84904, 0x84918),
    (0x8490c, 0x8491c),
    (0x84920, 0x84944),
    (0x84948, 0x8496c),
    (0x84954, 0x84970),
    (0x84974, 0x84990),
    (0x84994, 0x849b0),
    (0x849b4, 0x849d0),
    (0x849d4, 0x849ec),
    (0x849f0, 0x84a04),
    (0x849f8, 0x84a08),
    (0x84a0c, 0x84a28),
    (0x84a2c, 0x84a48),
    (0x84a58, 0x84a7c),
    (0x84a64, 0x84a80),
    (0x84a84, 0x84aa0),
    (0x84aa4, 0x84ac0),
    (0x84ad0, 0x84af4),
    (0x84adc, 0x84af8),
    (0x84afc, 0x84b10),
    (0x84b14, 0x84b28),
    (0x84b2c, 0x84b48),
    (0x84b4c, 0x84b68),
    (0x84b6c, 0x84b9c),
    (0x84ba0, 0x84bb8),
    (0x84bbc, 0x84bd4),
    (0x84bd8, 0x84bf4),
    (0x84bf8, 0x84c08),
    (0x84c0c, 0x84c1c),
    (0x84c20, 0x84c30),
    (0x84c34, 0x84c44),
    (0x84c48, 0x84c50),
];

const FUNCTIONS: [(usize, fn(&mut Code) -> Result<(), String>); 35] = [
    (0x84574, packet_registry_lookup),
    (0x8468c, registry_initialize),
    (0x846f8, registry_lookup_or_insert),
    (0x847e4, reference_acquire),
    (0x84804, packet_reference_acquire),
    (0x84838, reference_release),
    (0x848d8, mmio_848d8),
    (0x848e8, mmio_848e8),
    (0x84904, mmio_84904),
    (0x84920, mmio_84920),
    (0x84948, mmio_84948),
    (0x84974, mmio_84974),
    (0x84994, mmio_84994),
    (0x849b4, mmio_849b4),
    (0x849d4, mmio_849d4),
    (0x849f0, mmio_849f0),
    (0x84a0c, mmio_84a0c),
    (0x84a2c, mmio_84a2c),
    (0x84a4c, mmio_84a4c),
    (0x84a84, mmio_84a84),
    (0x84aa4, mmio_84aa4),
    (0x84ac4, mmio_84ac4),
    (0x84afc, mmio_84afc),
    (0x84b14, mmio_84b14),
    (0x84b2c, mmio_84b2c),
    (0x84b4c, mmio_84b4c),
    (0x84b6c, mmio_84b6c),
    (0x84ba0, mmio_84ba0),
    (0x84bbc, mmio_84bbc),
    (0x84bd8, mmio_84bd8),
    (0x84bf8, mmio_84bf8),
    (0x84c0c, mmio_84c0c),
    (0x84c20, mmio_84c20),
    (0x84c34, mmio_84c34),
    (0x84c48, final_tail_wrapper),
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
            return Err(format!("invalid registry {} at {at:#x}", if data { "data" } else { "code" }));
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
            return Err(format!("invalid registry literal reference {at:#x} -> {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, R::new(rd)?, R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn packet_registry_lookup(c: &mut Code) -> Result<(), String> {
    c.emit(0x84574, transfer_multiple(C::Al, false, R::SP, 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x84578, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x8457c, data_immediate(C::Al, Op::Sub, r(13), r(13), 0xc)?)?;
    c.emit(0x84580, transfer_word(C::Al, false, r(2), r(13), 4)?)?;
    c.emit(0x84584, data_register(C::Al, Op::Mov, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84588, data_register(C::Al, Op::Mov, r(6), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8458c, transfer_word(C::Al, true, r(4), r(7), 8)?)?;
    c.emit(0x84590, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x84594, branch(C::Al, 0x84594, 0x8468c, true)?)?;
    c.emit(0x84598, transfer_word(C::Al, true, r(0), r(7), 8)?)?;
    c.emit(0x8459c, transfer_word(C::Al, false, r(7), r(13), 0)?)?;
    c.emit(0x845a0, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x845a4, transfer_word(C::Al, false, r(0), r(13), 8)?)?;
    c.emit(0x845a8, branch(C::Eq, 0x845a8, 0x84620, false)?)?;
    c.emit(0x845ac, transfer_word(C::Al, true, r(8), r(7), 12)?)?;
    c.emit(0x845b0, data_immediate(C::Al, Op::Add, r(5), r(7), 0x44)?)?;
    c.emit(0x845b4, data_immediate(C::Al, Op::Mov, r(9), r(0), 0x0)?)?;
    c.emit(0x845b8, transfer_word(C::Al, true, r(7), r(5), 0)?)?;
    c.emit(0x845bc, data_register(C::Al, Op::Mov, r(10), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x845c0, data_register(C::Al, Op::Mov, r(4), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x845c4, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(0x845c8, branch(C::Eq, 0x845c8, 0x8460c, false)?)?;
    c.emit(0x845cc, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x845d0, branch(C::Al, 0x845d0, 0x3388, true)?)?;
    c.emit(0x845d4, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x845d8, data_register(C::Al, Op::Mov, r(0), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x845dc, branch(C::Al, 0x845dc, 0x3388, true)?)?;
    c.emit(0x845e0, data_register(C::Al, Op::Cmp, r(0), r(4), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x845e4, branch(C::Ne, 0x845e4, 0x84608, false)?)?;
    c.emit(0x845e8, data_register(C::Al, Op::Mov, r(0), r(0), r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x845ec, data_register(C::Al, Op::Mov, r(1), r(0), r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x845f0, data_register(C::Al, Op::Mov, r(2), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x845f4, branch(C::Al, 0x845f4, 0x84210, true)?)?;
    c.emit(0x845f8, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x845fc, data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x84600, branch(C::Ne, 0x84600, 0x8460c, false)?)?;
    c.emit(0x84604, branch(C::Al, 0x84604, 0x84654, false)?)?;
    c.emit(0x84608, data_register(C::Al, Op::Mov, r(4), r(0), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x8460c, transfer_word(C::Al, true, r(1), r(13), 8)?)?;
    c.emit(0x84610, data_immediate(C::Al, Op::Add, r(9), r(9), 0x1)?)?;
    c.emit(0x84614, data_register(C::Al, Op::Add, r(5), r(5), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x84618, data_register(C::Al, Op::Cmp, r(0), r(9), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x8461c, branch(C::Cc, 0x8461c, 0x845b8, false)?)?;
    c.emit(0x84620, transfer_word(C::Al, true, r(2), r(13), 8)?)?;
    c.emit(0x84624, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x84628, data_register(C::Al, Op::Cmp, r(0), r(4), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x8462c, branch(C::Eq, 0x8462c, 0x84658, false)?)?;
    c.emit(0x84630, transfer_word(C::Al, true, r(3), r(13), 0)?)?;
    c.emit(0x84634, transfer_word(C::Al, true, r(2), r(3), 12)?)?;
    c.emit(0x84638, multiply_accumulate(C::Al, r(1), r(2), r(4), r(3))?)?;
    c.emit(0x8463c, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x84640, transfer_word_preindexed_writeback(C::Al, false, r(2), r(1), 24)?)?;
    c.emit(0x84644, transfer_multiple(C::Al, false, r(1), 0x14, BlockMode::IncrementBefore, false)?)?;
    c.emit(0x84648, transfer_word(C::Al, false, r(3), r(1), 12)?)?;
    c.emit(0x8464c, transfer_word(C::Al, false, r(6), r(1), 44)?)?;
    c.emit(0x84650, branch(C::Al, 0x84650, 0x84658, false)?)?;
    c.emit(0x84654, data_immediate(C::Al, Op::Sub, r(1), r(5), 0x2c)?)?;
    c.emit(0x84658, transfer_word(C::Al, true, r(2), r(13), 4)?)?;
    c.emit(0x8465c, transfer_word(C::Al, true, r(0), r(2), 12)?)?;
    c.emit(0x84660, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x84664, branch(C::Ne, 0x84664, 0x84678, false)?)?;
    c.emit(0x84668, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8466c, transfer_word(C::Al, false, r(0), r(2), 0)?)?;
    c.emit(0x84670, transfer_word(C::Al, false, r(0), r(2), 4)?)?;
    c.emit(0x84674, transfer_word(C::Al, false, r(1), r(2), 12)?)?;
    c.emit(0x84678, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x8467c, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x84680, data_immediate(C::Eq, Op::Mvn, r(0), r(0), 0x0)?)?;
    c.emit(0x84684, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x84688, transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn registry_initialize(c: &mut Code) -> Result<(), String> {
    c.emit(0x8468c, transfer_word(C::Al, true, r(1), r(0), 16)?)?;
    c.emit(0x84690, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x84694, bx(C::Ne, r(14)))?;
    c.emit(0x84698, transfer_multiple(C::Al, false, R::SP, 0x4800, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8469c, data_register(C::Al, Op::Mov, r(11), r(0), r(13), Shift::Lsl, 0, false)?)?;
    c.emit(0x846a0, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.emit(0x846a4, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(0x846a8, transfer_word(C::Al, false, r(2), r(0), 16)?)?;
    c.emit(0x846ac, transfer_word(C::Al, false, r(12), r(0), 20)?)?;
    c.emit(0x846b0, transfer_word(C::Al, true, r(2), r(0), 8)?)?;
    c.emit(0x846b4, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x846b8, branch(C::Eq, 0x846b8, 0x846e0, false)?)?;
    c.emit(0x846bc, data_immediate(C::Al, Op::Add, r(14), r(0), 0x18)?)?;
    c.emit(0x846c0, data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.emit(0x846c4, transfer_word(C::Al, true, r(1), r(0), 12)?)?;
    c.emit(0x846c8, multiply_accumulate(C::Al, r(2), r(1), r(3), r(14))?)?;
    c.emit(0x846cc, data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.emit(0x846d0, transfer_word(C::Al, false, r(12), r(2), 4)?)?;
    c.emit(0x846d4, transfer_word(C::Al, true, r(1), r(0), 8)?)?;
    c.emit(0x846d8, data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x846dc, branch(C::Cc, 0x846dc, 0x846c4, false)?)?;
    c.emit(0x846e0, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x846e4, data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.emit(0x846e8, transfer_multiple(C::Al, true, R::SP, 0x4800, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x846ec, branch(C::Eq, 0x846ec, 0x846f4, false)?)?;
    c.emit(0x846f0, bx(C::Al, r(1)))?;
    c.emit(0x846f4, bx(C::Al, r(14)))?;
    Ok(())
}

fn registry_lookup_or_insert(c: &mut Code) -> Result<(), String> {
    c.emit(0x846f8, transfer_multiple(C::Al, false, R::SP, 0x4ff0, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x846fc, data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.emit(0x84700, data_immediate(C::Al, Op::Sub, r(13), r(13), 0x4)?)?;
    c.emit(0x84704, data_register(C::Al, Op::Mov, r(9), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84708, data_register(C::Al, Op::Mov, r(10), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8470c, data_register(C::Al, Op::Mov, r(8), r(0), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84710, data_register(C::Al, Op::Mov, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x84714, branch(C::Al, 0x84714, 0x8468c, true)?)?;
    c.emit(0x84718, transfer_word(C::Al, true, r(2), r(9), 8)?)?;
    c.emit(0x8471c, data_immediate(C::Al, Op::Mvn, r(0), r(0), 0x0)?)?;
    c.emit(0x84720, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x84724, branch(C::Eq, 0x84724, 0x847dc, false)?)?;
    c.emit(0x84728, transfer_word(C::Al, true, r(14), r(9), 12)?)?;
    c.emit(0x8472c, data_immediate(C::Al, Op::Sub, r(3), r(2), 0x1)?)?;
    c.emit(0x84730, data_immediate(C::Al, Op::Mvn, r(7), r(0), 0x1b)?)?;
    c.emit(0x84734, data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.emit(0x84738, data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.emit(0x8473c, multiply_register(C::Al, r(1), r(14), r(3))?)?;
    c.emit(0x84740, data_register(C::Al, Op::Sub, r(1), r(7), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84744, data_immediate(C::Al, Op::Mov, r(7), r(0), 0x2)?)?;
    c.emit(0x84748, data_register(C::Al, Op::Sub, r(5), r(1), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x8474c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x84750, transfer_word_register_offset(C::Al, true, r(4), r(12), r(5), Shift::Lsl, 0, true)?)?;
    c.emit(0x84754, data_register(C::Al, Op::Cmp, r(0), r(4), r(10), Shift::Lsl, 0, false)?)?;
    c.emit(0x84758, branch(C::Eq, 0x84758, 0x847ac, false)?)?;
    c.emit(0x8475c, data_immediate(C::Al, Op::Cmp, r(0), r(4), 0x0)?)?;
    c.emit(0x84760, data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.emit(0x84764, data_register(C::Al, Op::Add, r(5), r(5), r(14), Shift::Lsl, 0, false)?)?;
    c.emit(0x84768, data_register(C::Eq, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x8476c, data_immediate(C::Eq, Op::Mov, r(7), r(0), 0x1)?)?;
    c.emit(0x84770, data_immediate(C::Al, Op::Sub, r(3), r(3), 0x1)?)?;
    c.emit(0x84774, data_register(C::Al, Op::Cmp, r(0), r(6), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x84778, branch(C::Cc, 0x84778, 0x84750, false)?)?;
    c.emit(0x8477c, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.emit(0x84780, branch(C::Eq, 0x84780, 0x847b0, false)?)?;
    c.emit(0x84784, data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x1)?)?;
    c.emit(0x84788, branch(C::Ne, 0x84788, 0x847dc, false)?)?;
    c.emit(0x8478c, transfer_word(C::Al, true, r(0), r(9), 12)?)?;
    c.emit(0x84790, multiply_accumulate(C::Al, r(2), r(0), r(1), r(9))?)?;
    c.emit(0x84794, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x84798, transfer_word(C::Al, false, r(0), r(2), 24)?)?;
    c.emit(0x8479c, transfer_word(C::Al, false, r(10), r(2), 28)?)?;
    c.emit(0x847a0, transfer_word(C::Al, false, r(1), r(2), 32)?)?;
    c.emit(0x847a4, transfer_word(C::Al, false, r(9), r(2), 36)?)?;
    c.emit(0x847a8, branch(C::Al, 0x847a8, 0x847b0, false)?)?;
    c.emit(0x847ac, data_register(C::Al, Op::Mov, r(1), r(0), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x847b0, transfer_word(C::Al, true, r(2), r(8), 12)?)?;
    c.emit(0x847b4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x847b8, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x847bc, branch(C::Ne, 0x847bc, 0x847dc, false)?)?;
    c.emit(0x847c0, transfer_word(C::Al, true, r(2), r(9), 12)?)?;
    c.emit(0x847c4, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x847c8, transfer_word(C::Al, false, r(0), r(8), 0)?)?;
    c.emit(0x847cc, transfer_word(C::Al, false, r(0), r(8), 4)?)?;
    c.emit(0x847d0, multiply_accumulate(C::Al, r(3), r(2), r(1), r(9))?)?;
    c.emit(0x847d4, data_immediate(C::Al, Op::Add, r(1), r(3), 0x18)?)?;
    c.emit(0x847d8, transfer_word(C::Al, false, r(1), r(8), 12)?)?;
    c.emit(0x847dc, data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.emit(0x847e0, transfer_multiple(C::Al, true, R::SP, 0x8ff0, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn reference_acquire(c: &mut Code) -> Result<(), String> {
    c.emit(0x847e4, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x847e8, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x847ec, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x847f0, transfer_word(C::Al, true, r(0), r(0), 12)?)?;
    c.emit(0x847f4, transfer_word(C::Al, true, r(1), r(0), 20)?)?;
    c.emit(0x847f8, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x847fc, transfer_word(C::Al, false, r(1), r(0), 20)?)?;
    c.emit(0x84800, bx(C::Al, r(14)))?;
    Ok(())
}

fn packet_reference_acquire(c: &mut Code) -> Result<(), String> {
    c.emit(0x84804, transfer_word(C::Al, true, r(1), r(0), 12)?)?;
    c.emit(0x84808, transfer_word(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x8480c, data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(0x84810, transfer_word(C::Al, false, r(2), r(1), 0)?)?;
    c.emit(0x84814, transfer_word(C::Al, true, r(1), r(1), 12)?)?;
    c.emit(0x84818, transfer_word(C::Al, true, r(2), r(1), 20)?)?;
    c.emit(0x8481c, data_immediate(C::Al, Op::Add, r(2), r(2), 0x1)?)?;
    c.emit(0x84820, transfer_word(C::Al, false, r(2), r(1), 20)?)?;
    c.emit(0x84824, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84828, data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.emit(0x8482c, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x84830, data_register(C::Al, Op::Mov, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x84834, bx(C::Al, r(14)))?;
    Ok(())
}

fn reference_release(c: &mut Code) -> Result<(), String> {
    c.emit(0x84838, transfer_multiple(C::Al, false, R::SP, 0x4890, BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8483c, data_immediate(C::Al, Op::Add, r(11), r(13), 0x8)?)?;
    c.emit(0x84840, data_register(C::Al, Op::Mov, r(4), r(0), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84844, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x84848, data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.emit(0x8484c, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x84850, transfer_multiple(C::Eq, true, R::SP, 0x8890, BlockMode::IncrementAfter, true)?)?;
    c.emit(0x84854, transfer_word(C::Al, true, r(1), r(4), 12)?)?;
    c.emit(0x84858, transfer_word(C::Al, true, r(0), r(1), 12)?)?;
    c.emit(0x8485c, transfer_word(C::Al, true, r(2), r(0), 20)?)?;
    c.emit(0x84860, data_immediate_with_flags(C::Al, Op::Sub, r(2), r(2), 0x1, true)?)?;
    c.emit(0x84864, transfer_word(C::Al, false, r(2), r(0), 20)?)?;
    c.emit(0x84868, branch(C::Ne, 0x84868, 0x84898, false)?)?;
    c.emit(0x8486c, transfer_word(C::Al, true, r(2), r(0), 16)?)?;
    c.emit(0x84870, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x84874, branch(C::Eq, 0x84874, 0x84898, false)?)?;
    c.emit(0x84878, data_immediate(C::Al, Op::Mov, r(2), r(0), 0x0)?)?;
    c.emit(0x8487c, transfer_word(C::Al, false, r(2), r(0), 16)?)?;
    c.emit(0x84880, transfer_word(C::Al, false, r(2), r(0), 20)?)?;
    c.emit(0x84884, transfer_word(C::Al, true, r(2), r(0), 4)?)?;
    c.emit(0x84888, data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.emit(0x8488c, branch(C::Eq, 0x8488c, 0x84898, false)?)?;
    c.emit(0x84890, branch_exchange_with_link(C::Al, r(2))?)?;
    c.emit(0x84894, transfer_word(C::Al, true, r(1), r(4), 12)?)?;
    c.emit(0x84898, transfer_word(C::Al, true, r(0), r(1), 0)?)?;
    c.emit(0x8489c, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 0x1, true)?)?;
    c.emit(0x848a0, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x848a4, branch(C::Ne, 0x848a4, 0x848c0, false)?)?;
    c.emit(0x848a8, data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x848ac, transfer_word(C::Al, false, r(0), r(1), 44)?)?;
    c.emit(0x848b0, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x848b4, transfer_word(C::Al, false, r(0), r(1), 4)?)?;
    c.emit(0x848b8, transfer_word(C::Al, false, r(0), r(1), 8)?)?;
    c.emit(0x848bc, transfer_word(C::Al, false, r(0), r(1), 12)?)?;
    c.emit(0x848c0, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x848c4, data_immediate_with_flags(C::Al, Op::Sub, r(0), r(0), 0x1, true)?)?;
    c.emit(0x848c8, transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0x848cc, data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x0)?)?;
    c.emit(0x848d0, transfer_word(C::Eq, false, r(0), r(4), 12)?)?;
    c.emit(0x848d4, transfer_multiple(C::Al, true, R::SP, 0x8890, BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn mmio_848d8(c: &mut Code) -> Result<(), String> {
    c.literal(0x848d8, 1, 0x848e4)?;
    c.emit(0x848dc, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x848e0, bx(C::Al, r(14)))?;
    c.word(0x848e4, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_848e8(c: &mut Code) -> Result<(), String> {
    c.literal(0x848e8, 0, 0x848fc)?;
    c.emit(0x848ec, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.literal(0x848f0, 1, 0x84900)?;
    c.emit(0x848f4, transfer_word(C::Al, false, r(1), r(0), 4016)?)?;
    c.emit(0x848f8, bx(C::Al, r(14)))?;
    c.word(0x848fc, 0x10cfa4, true)?;
    c.word(0x84900, 0xc5acce55, true)?;
    Ok(())
}

fn mmio_84904(c: &mut Code) -> Result<(), String> {
    c.literal(0x84904, 0, 0x84918)?;
    c.emit(0x84908, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.literal(0x8490c, 1, 0x8491c)?;
    c.emit(0x84910, transfer_word(C::Al, false, r(1), r(0), 4016)?)?;
    c.emit(0x84914, bx(C::Al, r(14)))?;
    c.word(0x84918, 0x10cfa4, true)?;
    c.word(0x8491c, 0x3a5331aa, true)?;
    Ok(())
}

fn mmio_84920(c: &mut Code) -> Result<(), String> {
    c.literal(0x84920, 1, 0x84944)?;
    c.emit(0x84924, data_immediate(C::Al, Op::And, r(0), r(0), 0x3)?)?;
    c.emit(0x84928, transfer_word(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x8492c, transfer_word(C::Al, true, r(2), r(2), 48)?)?;
    c.emit(0x84930, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84934, data_immediate(C::Al, Op::Bic, r(2), r(2), 0x3)?)?;
    c.emit(0x84938, data_register(C::Al, Op::Orr, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8493c, transfer_word(C::Al, false, r(0), r(1), 48)?)?;
    c.emit(0x84940, bx(C::Al, r(14)))?;
    c.word(0x84944, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84948(c: &mut Code) -> Result<(), String> {
    c.literal(0x84948, 1, 0x8496c)?;
    c.emit(0x8494c, transfer_word(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x84950, transfer_word(C::Al, true, r(2), r(2), 52)?)?;
    c.literal(0x84954, 3, 0x84970)?;
    c.emit(0x84958, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x8495c, data_register(C::Al, Op::And, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x84960, data_register(C::Al, Op::Orr, r(0), r(2), r(0), Shift::Lsl, 2, false)?)?;
    c.emit(0x84964, transfer_word(C::Al, false, r(0), r(1), 52)?)?;
    c.emit(0x84968, bx(C::Al, r(14)))?;
    c.word(0x8496c, 0x10cfa4, true)?;
    c.word(0x84970, 0xfffc0003, true)?;
    Ok(())
}

fn mmio_84974(c: &mut Code) -> Result<(), String> {
    c.literal(0x84974, 0, 0x84990)?;
    c.emit(0x84978, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x8497c, transfer_word(C::Al, true, r(1), r(1), 52)?)?;
    c.emit(0x84980, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84984, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x2)?)?;
    c.emit(0x84988, transfer_word(C::Al, false, r(1), r(0), 52)?)?;
    c.emit(0x8498c, bx(C::Al, r(14)))?;
    c.word(0x84990, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84994(c: &mut Code) -> Result<(), String> {
    c.literal(0x84994, 0, 0x849b0)?;
    c.emit(0x84998, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x8499c, transfer_word(C::Al, true, r(1), r(1), 52)?)?;
    c.emit(0x849a0, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x849a4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1)?)?;
    c.emit(0x849a8, transfer_word(C::Al, false, r(1), r(0), 52)?)?;
    c.emit(0x849ac, bx(C::Al, r(14)))?;
    c.word(0x849b0, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_849b4(c: &mut Code) -> Result<(), String> {
    c.literal(0x849b4, 0, 0x849d0)?;
    c.emit(0x849b8, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x849bc, transfer_word(C::Al, true, r(1), r(1), 48)?)?;
    c.emit(0x849c0, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x849c4, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x4)?)?;
    c.emit(0x849c8, transfer_word(C::Al, false, r(1), r(0), 48)?)?;
    c.emit(0x849cc, bx(C::Al, r(14)))?;
    c.word(0x849d0, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_849d4(c: &mut Code) -> Result<(), String> {
    c.literal(0x849d4, 0, 0x849ec)?;
    c.emit(0x849d8, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.emit(0x849dc, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x849e0, transfer_word(C::Al, true, r(0), r(0), 48)?)?;
    c.emit(0x849e4, data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsr, 2, false)?)?;
    c.emit(0x849e8, bx(C::Al, r(14)))?;
    c.word(0x849ec, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_849f0(c: &mut Code) -> Result<(), String> {
    c.literal(0x849f0, 0, 0x84a04)?;
    c.emit(0x849f4, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.literal(0x849f8, 1, 0x84a08)?;
    c.emit(0x849fc, transfer_word(C::Al, false, r(1), r(0), 20)?)?;
    c.emit(0x84a00, bx(C::Al, r(14)))?;
    c.word(0x84a04, 0x10cfa4, true)?;
    c.word(0x84a08, 0x3020100, true)?;
    Ok(())
}

fn mmio_84a0c(c: &mut Code) -> Result<(), String> {
    c.literal(0x84a0c, 0, 0x84a28)?;
    c.emit(0x84a10, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84a14, transfer_word(C::Al, true, r(1), r(1), 16)?)?;
    c.emit(0x84a18, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84a1c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x2)?)?;
    c.emit(0x84a20, transfer_word(C::Al, false, r(1), r(0), 16)?)?;
    c.emit(0x84a24, bx(C::Al, r(14)))?;
    c.word(0x84a28, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84a2c(c: &mut Code) -> Result<(), String> {
    c.literal(0x84a2c, 0, 0x84a48)?;
    c.emit(0x84a30, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84a34, transfer_word(C::Al, true, r(1), r(1), 16)?)?;
    c.emit(0x84a38, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84a3c, data_immediate(C::Al, Op::Bic, r(1), r(1), 0x2)?)?;
    c.emit(0x84a40, transfer_word(C::Al, false, r(1), r(0), 16)?)?;
    c.emit(0x84a44, bx(C::Al, r(14)))?;
    c.word(0x84a48, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84a4c(c: &mut Code) -> Result<(), String> {
    c.emit(0x84a4c, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xfe00)?)?;
    c.emit(0x84a50, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x30000)?)?;
    c.emit(0x84a54, data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 9, false)?)?;
    c.literal(0x84a58, 1, 0x84a7c)?;
    c.emit(0x84a5c, transfer_word(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x84a60, transfer_word(C::Al, true, r(2), r(2), 24)?)?;
    c.literal(0x84a64, 3, 0x84a80)?;
    c.emit(0x84a68, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84a6c, data_register(C::Al, Op::And, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x84a70, data_register(C::Al, Op::Orr, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84a74, transfer_word(C::Al, false, r(0), r(1), 24)?)?;
    c.emit(0x84a78, bx(C::Al, r(14)))?;
    c.word(0x84a7c, 0x10cfa4, true)?;
    c.word(0x84a80, 0xfffc01ff, true)?;
    Ok(())
}

fn mmio_84a84(c: &mut Code) -> Result<(), String> {
    c.literal(0x84a84, 0, 0x84aa0)?;
    c.emit(0x84a88, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84a8c, transfer_word(C::Al, true, r(1), r(1), 16)?)?;
    c.emit(0x84a90, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84a94, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x4)?)?;
    c.emit(0x84a98, transfer_word(C::Al, false, r(1), r(0), 16)?)?;
    c.emit(0x84a9c, bx(C::Al, r(14)))?;
    c.word(0x84aa0, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84aa4(c: &mut Code) -> Result<(), String> {
    c.literal(0x84aa4, 0, 0x84ac0)?;
    c.emit(0x84aa8, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84aac, transfer_word(C::Al, true, r(1), r(1), 16)?)?;
    c.emit(0x84ab0, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84ab4, data_immediate(C::Al, Op::Bic, r(1), r(1), 0x4)?)?;
    c.emit(0x84ab8, transfer_word(C::Al, false, r(1), r(0), 16)?)?;
    c.emit(0x84abc, bx(C::Al, r(14)))?;
    c.word(0x84ac0, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84ac4(c: &mut Code) -> Result<(), String> {
    c.emit(0x84ac4, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x3fc0000)?)?;
    c.emit(0x84ac8, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x4000000)?)?;
    c.emit(0x84acc, data_register(C::Al, Op::And, r(0), r(1), r(0), Shift::Lsl, 18, false)?)?;
    c.literal(0x84ad0, 1, 0x84af4)?;
    c.emit(0x84ad4, transfer_word(C::Al, true, r(2), r(1), 0)?)?;
    c.emit(0x84ad8, transfer_word(C::Al, true, r(2), r(2), 24)?)?;
    c.literal(0x84adc, 3, 0x84af8)?;
    c.emit(0x84ae0, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84ae4, data_register(C::Al, Op::And, r(2), r(2), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x84ae8, data_register(C::Al, Op::Orr, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x84aec, transfer_word(C::Al, false, r(0), r(1), 24)?)?;
    c.emit(0x84af0, bx(C::Al, r(14)))?;
    c.word(0x84af4, 0x10cfa4, true)?;
    c.word(0x84af8, 0xf803ffff, true)?;
    Ok(())
}

fn mmio_84afc(c: &mut Code) -> Result<(), String> {
    c.literal(0x84afc, 0, 0x84b10)?;
    c.emit(0x84b00, data_immediate(C::Al, Op::Mov, r(1), r(0), 0xf)?)?;
    c.emit(0x84b04, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84b08, transfer_word(C::Al, false, r(1), r(0), 16)?)?;
    c.emit(0x84b0c, bx(C::Al, r(14)))?;
    c.word(0x84b10, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84b14(c: &mut Code) -> Result<(), String> {
    c.literal(0x84b14, 0, 0x84b28)?;
    c.emit(0x84b18, data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.emit(0x84b1c, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84b20, transfer_word(C::Al, false, r(1), r(0), 16)?)?;
    c.emit(0x84b24, bx(C::Al, r(14)))?;
    c.word(0x84b28, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84b2c(c: &mut Code) -> Result<(), String> {
    c.literal(0x84b2c, 0, 0x84b48)?;
    c.emit(0x84b30, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84b34, transfer_word(C::Al, true, r(1), r(1), 56)?)?;
    c.emit(0x84b38, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84b3c, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x2)?)?;
    c.emit(0x84b40, transfer_word(C::Al, false, r(1), r(0), 56)?)?;
    c.emit(0x84b44, bx(C::Al, r(14)))?;
    c.word(0x84b48, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84b4c(c: &mut Code) -> Result<(), String> {
    c.literal(0x84b4c, 0, 0x84b68)?;
    c.emit(0x84b50, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84b54, transfer_word(C::Al, true, r(1), r(1), 56)?)?;
    c.emit(0x84b58, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84b5c, data_immediate(C::Al, Op::Bic, r(1), r(1), 0x2)?)?;
    c.emit(0x84b60, transfer_word(C::Al, false, r(1), r(0), 56)?)?;
    c.emit(0x84b64, bx(C::Al, r(14)))?;
    c.word(0x84b68, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84b6c(c: &mut Code) -> Result<(), String> {
    c.literal(0x84b6c, 0, 0x84b9c)?;
    c.emit(0x84b70, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84b74, transfer_word(C::Al, true, r(1), r(1), 56)?)?;
    c.emit(0x84b78, transfer_word(C::Al, true, r(2), r(0), 0)?)?;
    c.emit(0x84b7c, data_immediate(C::Al, Op::Bic, r(1), r(1), 0x1)?)?;
    c.emit(0x84b80, transfer_word(C::Al, false, r(1), r(2), 56)?)?;
    c.emit(0x84b84, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84b88, transfer_word(C::Al, true, r(1), r(1), 56)?)?;
    c.emit(0x84b8c, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84b90, data_immediate(C::Al, Op::Orr, r(1), r(1), 0x1)?)?;
    c.emit(0x84b94, transfer_word(C::Al, false, r(1), r(0), 56)?)?;
    c.emit(0x84b98, bx(C::Al, r(14)))?;
    c.word(0x84b9c, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84ba0(c: &mut Code) -> Result<(), String> {
    c.literal(0x84ba0, 0, 0x84bb8)?;
    c.emit(0x84ba4, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84ba8, transfer_word(C::Al, true, r(1), r(1), 72)?)?;
    c.emit(0x84bac, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84bb0, transfer_word(C::Al, true, r(0), r(0), 68)?)?;
    c.emit(0x84bb4, bx(C::Al, r(14)))?;
    c.word(0x84bb8, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84bbc(c: &mut Code) -> Result<(), String> {
    c.literal(0x84bbc, 2, 0x84bd4)?;
    c.emit(0x84bc0, transfer_word(C::Al, true, r(3), r(2), 0)?)?;
    c.emit(0x84bc4, transfer_word(C::Al, false, r(1), r(3), 80)?)?;
    c.emit(0x84bc8, transfer_word(C::Al, true, r(1), r(2), 0)?)?;
    c.emit(0x84bcc, transfer_word(C::Al, false, r(0), r(1), 76)?)?;
    c.emit(0x84bd0, bx(C::Al, r(14)))?;
    c.word(0x84bd4, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84bd8(c: &mut Code) -> Result<(), String> {
    c.literal(0x84bd8, 0, 0x84bf4)?;
    c.emit(0x84bdc, transfer_word(C::Al, true, r(1), r(0), 0)?)?;
    c.emit(0x84be0, transfer_word(C::Al, true, r(1), r(1), 64)?)?;
    c.emit(0x84be4, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x84be8, transfer_word(C::Al, true, r(0), r(0), 60)?)?;
    c.emit(0x84bec, data_immediate(C::Al, Op::Bic, r(1), r(1), 0xff000000)?)?;
    c.emit(0x84bf0, bx(C::Al, r(14)))?;
    c.word(0x84bf4, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84bf8(c: &mut Code) -> Result<(), String> {
    c.literal(0x84bf8, 1, 0x84c08)?;
    c.emit(0x84bfc, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84c00, transfer_word(C::Al, false, r(0), r(1), 32)?)?;
    c.emit(0x84c04, bx(C::Al, r(14)))?;
    c.word(0x84c08, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84c0c(c: &mut Code) -> Result<(), String> {
    c.literal(0x84c0c, 1, 0x84c1c)?;
    c.emit(0x84c10, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84c14, transfer_word(C::Al, false, r(0), r(1), 36)?)?;
    c.emit(0x84c18, bx(C::Al, r(14)))?;
    c.word(0x84c1c, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84c20(c: &mut Code) -> Result<(), String> {
    c.literal(0x84c20, 1, 0x84c30)?;
    c.emit(0x84c24, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84c28, transfer_word(C::Al, false, r(0), r(1), 40)?)?;
    c.emit(0x84c2c, bx(C::Al, r(14)))?;
    c.word(0x84c30, 0x10cfa4, true)?;
    Ok(())
}

fn mmio_84c34(c: &mut Code) -> Result<(), String> {
    c.literal(0x84c34, 1, 0x84c44)?;
    c.emit(0x84c38, transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(0x84c3c, transfer_word(C::Al, false, r(0), r(1), 44)?)?;
    c.emit(0x84c40, bx(C::Al, r(14)))?;
    c.word(0x84c44, 0x10cfa4, true)?;
    Ok(())
}

fn final_tail_wrapper(c: &mut Code) -> Result<(), String> {
    c.literal(0x84c48, 0, 0x84c50)?;
    c.emit(0x84c4c, branch(C::Al, 0x84c4c, 0x85824, false)?)?;
    c.word(0x84c50, 0x10cfb4, true)?;
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
        return Err("registry code/data partition mismatch".into());
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
        assert_eq!((START, END, FILE_MAPPING_DELTA), (0x84574, 0x84c54, 0x4000));
        assert_eq!((CODE_BYTES, DATA_BYTES, bytes.len()), (1620, 140, 1760));
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
            "dd5bb07df8ccbd145140bc29b7291bad034a6b264a5282a2db894b1e355ca028");
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
    fn direct_branches_and_external_tail_dependency() {
        let bytes = fragment().unwrap();
        let expected = [
            (0x84594, 0x8468c, true, 0xe),
            (0x845a8, 0x84620, false, 0x0),
            (0x845c8, 0x8460c, false, 0x0),
            (0x845d0, 0x3388, true, 0xe),
            (0x845dc, 0x3388, true, 0xe),
            (0x845e4, 0x84608, false, 0x1),
            (0x845f4, 0x84210, true, 0xe),
            (0x84600, 0x8460c, false, 0x1),
            (0x84604, 0x84654, false, 0xe),
            (0x8461c, 0x845b8, false, 0x3),
            (0x8462c, 0x84658, false, 0x0),
            (0x84650, 0x84658, false, 0xe),
            (0x84664, 0x84678, false, 0x1),
            (0x846b8, 0x846e0, false, 0x0),
            (0x846dc, 0x846c4, false, 0x3),
            (0x846ec, 0x846f4, false, 0x0),
            (0x84714, 0x8468c, true, 0xe),
            (0x84724, 0x847dc, false, 0x0),
            (0x84758, 0x847ac, false, 0x0),
            (0x84778, 0x84750, false, 0x3),
            (0x84780, 0x847b0, false, 0x0),
            (0x84788, 0x847dc, false, 0x1),
            (0x847a8, 0x847b0, false, 0xe),
            (0x847bc, 0x847dc, false, 0x1),
            (0x84868, 0x84898, false, 0x1),
            (0x84874, 0x84898, false, 0x0),
            (0x8488c, 0x84898, false, 0x0),
            (0x848a4, 0x848c0, false, 0x1),
            (0x84c4c, 0x85824, false, 0xe),
        ];
        let mut count = 0;
        for at in (START..END).step_by(4) {
            if LITERALS.iter().any(|&(slot, _)| slot == at) { continue; }
            let instruction = word(&bytes, at);
            if instruction & 0x0e00_0000 != 0x0a00_0000 { continue; }
            count += 1;
            let target = (at as i64 + 8 + (((instruction << 8) as i32 >> 8) as i64 * 4)) as usize;
            let link = instruction & (1 << 24) != 0;
            let condition = instruction >> 28;
            assert!(expected.contains(&(at, target, link, condition)));
            if (START..END).contains(&target) {
                assert!(!LITERALS.iter().any(|&(slot, _)| slot == target));
            }
        }
        assert_eq!(count, expected.len());
        assert_eq!(word(&bytes, 0x846f0), bx(C::Al, r(1)));
        assert_eq!(word(&bytes, 0x84890), branch_exchange_with_link(C::Al, r(2)).unwrap());
        assert_eq!(word(&bytes, 0x84c48), transfer_word(C::Al, true, r(0), R::PC, 0).unwrap());
        assert_eq!(word(&bytes, 0x84c4c), branch(C::Al, 0x84c4c, 0x85824, false).unwrap());
        assert_eq!(word(&bytes, 0x84c50), 0x10cfb4);
        assert!(0x85824 >= END);
    }

    #[test]
    fn flag_updates_predication_and_writeback() {
        let bytes = fragment().unwrap();
        for at in [0x84860, 0x8489c, 0x848c4] {
            assert_eq!((word(&bytes, at) >> 20) & 1, 1);
        }
        for at in [0x84578, 0x84610, 0x846cc, 0x84760, 0x84770, 0x847e8, 0x8480c] {
            assert_eq!((word(&bytes, at) >> 20) & 1, 0);
        }
        assert_eq!(word(&bytes, 0x84680) >> 28, 0);
        assert_eq!(word(&bytes, 0x84694) >> 28, 1);
        assert_eq!(word(&bytes, 0x84850) >> 28, 0);
        assert_eq!(word(&bytes, 0x848d0) >> 28, 0);
        assert_eq!(word(&bytes, 0x84640), transfer_word_preindexed_writeback(C::Al, false, r(2), r(1), 0x18).unwrap());
        assert_eq!(word(&bytes, 0x84644), transfer_multiple(C::Al, false, r(1), 0x14, BlockMode::IncrementBefore, false).unwrap());
        assert_eq!(word(&bytes, 0x84750), transfer_word_register_offset(C::Al, true, r(4), r(12), r(5), Shift::Lsl, 0, true).unwrap());
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
        for (at, slot, rd) in [(START, 0x848e4, 0), (0x848d8, 0x848fc, 1),
            (0x848d8, 0x848e5, 1), (0x848d8, END, 1), (0x848e4, 0x848e4, 1),
            (END, 0x848e4, 1), (0x848d8, 0x848e4, 16)] {
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
            "dd5bb07df8ccbd145140bc29b7291bad034a6b264a5282a2db894b1e355ca028");
    }
}
