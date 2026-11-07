use arm32::{
    branch, bx, data_immediate, data_register,
    transfer_byte_postindexed, transfer_byte, transfer_multiple, transfer_word,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x85824;
pub const END: usize = 0x85de0;
pub const CODE_BYTES: usize = 1448;
pub const DATA_BYTES: usize = 20;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const LITERALS: [(usize, u32); 5] = [
    (0x85b58, 0x10cff0),
    (0x85b74, 0x10cff0),
    (0x85b9c, 0x10cff0),
    (0x85d94, 0x10cff0),
    (0x85ddc, 0x10cff0),
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
            return Err(format!("ARM list management at {:#x}, expected {at:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        self.check(at)?;
        if !(START..END).contains(&at) || at % 4 != 0
            || LITERALS.iter().any(|&(slot, _)| at == slot)
        {
            return Err(format!("ARM list instruction outside code: {at:#x}"));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        self.code_bytes += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !LITERALS.contains(&(at, value)) {
            return Err(format!("ARM list invalid literal at {at:#x}: {value:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        self.data_bytes += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, rd: R, slot: usize) -> Result<(), String> {
        if !LITERALS.iter().any(|&(address, _)| slot == address) {
            return Err(format!("ARM list unknown literal: {slot:#x}"));
        }
        self.emit(at, transfer_word(C::Al, true, rd, R::PC, slot as i32 - at as i32 - 8)?)
    }
}

fn initialize_list(c: &mut Code) -> Result<(), String> {
    c.emit(0x85824, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85828, data_immediate(C::Ne, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x8582c, transfer_word(C::Ne, false, r(1), r(0), 0)?)?;
    c.emit(0x85830, transfer_word(C::Ne, false, r(1), r(0), 4)?)?;
    c.emit(0x85834, bx(C::Al, R::LR))?;
    Ok(())
}

fn allocate_and_prepend(c: &mut Code) -> Result<(), String> {
    c.emit(0x85838, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x8583c, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x10)?)?;
    c.emit(0x85840, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    c.emit(0x85844, data_register(C::Al, Op::Mov, r(6), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85848, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8584c, data_register(C::Al, Op::Mov, r(1), R::R0, R::SP, Shift::Lsl, 0, false)?)?;
    c.emit(0x85850, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x10)?)?;
    c.emit(0x85854, branch(C::Al, 0x85854, 0x57d4, true)?)?;
    c.emit(0x85858, data_immediate(C::Al, Op::Mov, r(7), R::R0, 0)?)?;
    c.emit(0x8585c, transfer_word(C::Al, true, r(5), R::SP, 0)?)?;
    c.emit(0x85860, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85864, data_register(C::Ne, Op::Mov, r(5), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x85868, data_immediate(C::Al, Op::Cmp, R::R0, r(5), 0)?)?;
    c.emit(0x8586c, branch(C::Eq, 0x8586c, 0x858f8, false)?)?;
    c.emit(0x85870, data_immediate(C::Al, Op::Mov, r(7), R::R0, 0)?)?;
    c.emit(0x85874, data_immediate(C::Al, Op::Add, r(1), R::SP, 4)?)?;
    c.emit(0x85878, data_register(C::Al, Op::Mov, r(0), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x8587c, transfer_word(C::Al, false, r(7), r(5), 0)?)?;
    c.emit(0x85880, transfer_word(C::Al, false, r(7), r(5), 4)?)?;
    c.emit(0x85884, transfer_word(C::Al, false, r(7), r(5), 8)?)?;
    c.emit(0x85888, transfer_word(C::Al, false, r(7), r(5), 0xc)?)?;
    c.emit(0x8588c, branch(C::Al, 0x8588c, 0x57d4, true)?)?;
    c.emit(0x85890, data_register(C::Al, Op::Mov, r(1), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85894, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85898, data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0)?)?;
    c.emit(0x8589c, data_register(C::Ne, Op::Mov, r(0), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x858a0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x858a4, transfer_word(C::Al, false, r(0), r(5), 0)?)?;
    c.emit(0x858a8, branch(C::Eq, 0x858a8, 0x858f0, false)?)?;
    c.emit(0x858ac, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x858b0, data_register(C::Al, Op::Mov, r(2), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x858b4, branch(C::Al, 0x858b4, 0x31ec, true)?)?;
    c.emit(0x858b8, transfer_word(C::Al, false, r(6), r(5), 4)?)?;
    c.emit(0x858bc, transfer_word(C::Al, false, r(7), r(5), 8)?)?;
    c.emit(0x858c0, data_register(C::Al, Op::Mov, r(7), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x858c4, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x858c8, transfer_word(C::Al, false, r(0), r(5), 0xc)?)?;
    c.emit(0x858cc, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x858d0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x858d4, transfer_word(C::Ne, true, r(0), r(4), 0)?)?;
    c.emit(0x858d8, transfer_word(C::Ne, false, r(5), r(0), 8)?)?;
    c.emit(0x858dc, transfer_word(C::Al, false, r(5), r(4), 0)?)?;
    c.emit(0x858e0, transfer_word(C::Al, true, r(0), r(4), 4)?)?;
    c.emit(0x858e4, data_immediate(C::Al, Op::Add, r(0), r(0), 1)?)?;
    c.emit(0x858e8, transfer_word(C::Al, false, r(0), r(4), 4)?)?;
    c.emit(0x858ec, branch(C::Al, 0x858ec, 0x858f8, false)?)?;
    c.emit(0x858f0, data_register(C::Al, Op::Mov, r(0), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x858f4, branch(C::Al, 0x858f4, 0x5838, true)?)?;
    c.emit(0x858f8, data_register(C::Al, Op::Mov, r(0), R::R0, r(7), Shift::Lsl, 0, false)?)?;
    c.emit(0x858fc, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x10)?)?;
    c.emit(0x85900, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn destroy_node(c: &mut Code) -> Result<(), String> {
    c.emit(0x85904, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85908, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x8590c, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    c.emit(0x85910, transfer_word(C::Al, false, r(0), R::SP, 4)?)?;
    c.emit(0x85914, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85918, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x8591c, branch(C::Eq, 0x8591c, 0x85978, false)?)?;
    c.emit(0x85920, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85924, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85928, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x8592c, branch(C::Eq, 0x8592c, 0x85954, false)?)?;
    c.emit(0x85930, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85934, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85938, transfer_word(C::Al, true, r(1), R::SP, 4)?)?;
    c.emit(0x8593c, transfer_word(C::Al, true, r(2), r(1), 4)?)?;
    c.emit(0x85940, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x85944, branch(C::Al, 0x85944, 0x31ec, true)?)?;
    c.emit(0x85948, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x8594c, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85950, branch(C::Al, 0x85950, 0x5838, true)?)?;
    c.emit(0x85954, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85958, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0)?)?;
    c.emit(0x8595c, transfer_word(C::Al, false, r(4), r(0), 0)?)?;
    c.emit(0x85960, transfer_word(C::Al, false, r(4), r(0), 4)?)?;
    c.emit(0x85964, transfer_word(C::Al, false, r(4), r(0), 8)?)?;
    c.emit(0x85968, transfer_word(C::Al, false, r(4), r(0), 0xc)?)?;
    c.emit(0x8596c, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85970, branch(C::Al, 0x85970, 0x5838, true)?)?;
    c.emit(0x85974, transfer_word(C::Al, false, r(4), R::SP, 4)?)?;
    c.emit(0x85978, data_immediate(C::Al, Op::Sub, R::SP, r(11), 8)?)?;
    c.emit(0x8597c, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn clear_list(c: &mut Code) -> Result<(), String> {
    c.emit(0x85980, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85984, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x85988, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x8598c, data_immediate(C::Al, Op::Cmp, R::R0, r(4), 0)?)?;
    c.emit(0x85990, transfer_multiple(C::Eq, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(0x85994, branch(C::Al, 0x85994, 0x859ac, false)?)?;
    c.emit(0x85998, transfer_word(C::Al, true, r(0), r(0), 0xc)?)?;
    c.emit(0x8599c, transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0x859a0, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x859a4, transfer_word(C::Al, true, r(0), r(0), 8)?)?;
    c.emit(0x859a8, branch(C::Al, 0x859a8, 0x85904, true)?)?;
    c.emit(0x859ac, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x859b0, transfer_word(C::Al, true, r(1), r(0), 0xc)?)?;
    c.emit(0x859b4, transfer_word(C::Al, true, r(0), r(4), 0)?)?;
    c.emit(0x859b8, data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0)?)?;
    c.emit(0x859bc, branch(C::Ne, 0x859bc, 0x85998, false)?)?;
    c.emit(0x859c0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x859c4, transfer_word(C::Ne, true, r(0), r(4), 0)?)?;
    c.emit(0x859c8, branch(C::Ne, 0x859c8, 0x85904, true)?)?;
    c.emit(0x859cc, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0)?)?;
    c.emit(0x859d0, transfer_word(C::Al, false, r(0), r(4), 0)?)?;
    c.emit(0x859d4, transfer_word(C::Al, false, r(0), r(4), 4)?)?;
    c.emit(0x859d8, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn unlink_and_destroy(c: &mut Code) -> Result<(), String> {
    c.emit(0x859dc, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x859e0, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x859e4, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    c.emit(0x859e8, transfer_word(C::Al, false, r(1), R::SP, 4)?)?;
    c.emit(0x859ec, data_register(C::Al, Op::Mov, r(5), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x859f0, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0)?)?;
    c.emit(0x859f4, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x859f8, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x859fc, branch(C::Eq, 0x859fc, 0x85aac, false)?)?;
    c.emit(0x85a00, transfer_word(C::Al, true, r(0), r(5), 0)?)?;
    c.emit(0x85a04, branch(C::Al, 0x85a04, 0x85a18, false)?)?;
    c.emit(0x85a08, transfer_word(C::Al, true, r(1), R::SP, 0)?)?;
    c.emit(0x85a0c, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85a10, branch(C::Eq, 0x85a10, 0x85a34, false)?)?;
    c.emit(0x85a14, transfer_word(C::Al, true, r(0), r(1), 0xc)?)?;
    c.emit(0x85a18, transfer_word(C::Al, false, r(0), R::SP, 0)?)?;
    c.emit(0x85a1c, transfer_word(C::Al, true, r(1), R::SP, 0)?)?;
    c.emit(0x85a20, transfer_word(C::Al, true, r(2), R::SP, 4)?)?;
    c.emit(0x85a24, transfer_word(C::Al, true, r(0), R::SP, 0)?)?;
    c.emit(0x85a28, data_register(C::Al, Op::Cmp, R::R0, r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85a2c, branch(C::Ne, 0x85a2c, 0x85a08, false)?)?;
    c.emit(0x85a30, branch(C::Al, 0x85a30, 0x85a38, false)?)?;
    c.emit(0x85a34, data_register(C::Al, Op::Mov, r(0), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85a38, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85a3c, branch(C::Eq, 0x85a3c, 0x85a68, false)?)?;
    c.emit(0x85a40, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85a44, transfer_word(C::Al, true, r(1), r(0), 8)?)?;
    c.emit(0x85a48, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85a4c, transfer_word(C::Al, true, r(0), r(0), 0xc)?)?;
    c.emit(0x85a50, data_immediate(C::Al, Op::Cmp, R::R0, r(1), 0)?)?;
    c.emit(0x85a54, branch(C::Eq, 0x85a54, 0x85a70, false)?)?;
    c.emit(0x85a58, transfer_word(C::Al, true, r(1), R::SP, 4)?)?;
    c.emit(0x85a5c, transfer_word(C::Al, true, r(1), r(1), 8)?)?;
    c.emit(0x85a60, data_immediate(C::Al, Op::Add, r(1), r(1), 0xc)?)?;
    c.emit(0x85a64, branch(C::Al, 0x85a64, 0x85a74, false)?)?;
    c.emit(0x85a68, data_immediate(C::Al, Op::Mov, r(4), R::R0, 1)?)?;
    c.emit(0x85a6c, branch(C::Al, 0x85a6c, 0x85aac, false)?)?;
    c.emit(0x85a70, data_register(C::Al, Op::Mov, r(1), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85a74, transfer_word(C::Al, false, r(0), r(1), 0)?)?;
    c.emit(0x85a78, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85a7c, transfer_word(C::Al, true, r(0), r(0), 0xc)?)?;
    c.emit(0x85a80, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85a84, transfer_word(C::Ne, true, r(0), R::SP, 4)?)?;
    c.emit(0x85a88, transfer_word(C::Ne, true, r(0), r(0), 8)?)?;
    c.emit(0x85a8c, transfer_word(C::Ne, true, r(1), R::SP, 4)?)?;
    c.emit(0x85a90, transfer_word(C::Ne, true, r(1), r(1), 0xc)?)?;
    c.emit(0x85a94, transfer_word(C::Ne, false, r(0), r(1), 8)?)?;
    c.emit(0x85a98, transfer_word(C::Al, true, r(0), R::SP, 4)?)?;
    c.emit(0x85a9c, branch(C::Al, 0x85a9c, 0x85904, true)?)?;
    c.emit(0x85aa0, transfer_word(C::Al, true, r(0), r(5), 4)?)?;
    c.emit(0x85aa4, data_immediate(C::Al, Op::Sub, r(0), r(0), 1)?)?;
    c.emit(0x85aa8, transfer_word(C::Al, false, r(0), r(5), 4)?)?;
    c.emit(0x85aac, data_register(C::Al, Op::Mov, r(0), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85ab0, data_immediate(C::Al, Op::Sub, R::SP, r(11), 8)?)?;
    c.emit(0x85ab4, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    Ok(())
}

fn write_output_header(c: &mut Code) -> Result<(), String> {
    c.emit(0x85ab8, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85abc, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x85ac0, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 8)?)?;
    c.emit(0x85ac4, data_register(C::Al, Op::Mov, r(4), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.literal(0x85ac8, r(0), 0x85b58)?;
    c.emit(0x85acc, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85ad0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85ad4, branch(C::Eq, 0x85ad4, 0x85b50, false)?)?;
    c.emit(0x85ad8, data_register(C::Al, Op::Mvn, r(0), R::R0, r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85adc, data_register(C::Al, Op::Mov, r(2), R::R0, r(2), Shift::Lsl, 0x10, false)?)?;
    c.emit(0x85ae0, data_immediate(C::Al, Op::Orr, r(0), r(0), 8)?)?;
    c.emit(0x85ae4, data_register(C::Al, Op::Orr, r(2), r(2), r(3), Shift::Lsl, 24, false)?)?;
    c.emit(0x85ae8, data_immediate(C::Al, Op::And, r(0), r(0), 0x88)?)?;
    c.emit(0x85aec, data_immediate(C::Al, Op::Orr, r(2), r(2), 0x130)?)?;
    c.emit(0x85af0, data_register(C::Al, Op::Orr, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85af4, transfer_word(C::Al, false, r(2), r(0), 0)?)?;
    c.emit(0x85af8, transfer_word(C::Al, true, r(0), r(11), 8)?)?;
    c.emit(0x85afc, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0xff)?)?;
    c.emit(0x85b00, branch(C::Hi, 0x85b00, 0x85b1c, false)?)?;
    c.emit(0x85b04, transfer_byte(C::Al, false, r(0), R::SP, 7)?)?;
    c.emit(0x85b08, data_immediate(C::Al, Op::And, r(1), r(1), 0xe7)?)?;
    c.emit(0x85b0c, data_immediate(C::Al, Op::Add, r(3), R::SP, 7)?)?;
    c.emit(0x85b10, data_register(C::Al, Op::Mov, r(0), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b14, data_immediate(C::Al, Op::Mov, r(2), R::R0, 1)?)?;
    c.emit(0x85b18, branch(C::Al, 0x85b18, 0x85b4c, false)?)?;
    c.emit(0x85b1c, data_immediate(C::Al, Op::And, r(5), r(1), 0xe7)?)?;
    c.emit(0x85b20, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0)?)?;
    c.emit(0x85b24, data_immediate(C::Al, Op::Add, r(3), R::SP, 7)?)?;
    c.emit(0x85b28, data_immediate(C::Al, Op::Mov, r(2), R::R0, 1)?)?;
    c.emit(0x85b2c, transfer_byte(C::Al, false, r(0), R::SP, 7)?)?;
    c.emit(0x85b30, data_register(C::Al, Op::Mov, r(0), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b34, data_register(C::Al, Op::Mov, r(1), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b38, branch(C::Al, 0x85b38, 0x8607c, true)?)?;
    c.emit(0x85b3c, data_immediate(C::Al, Op::Add, r(3), r(11), 8)?)?;
    c.emit(0x85b40, data_register(C::Al, Op::Mov, r(0), R::R0, r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b44, data_register(C::Al, Op::Mov, r(1), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b48, data_immediate(C::Al, Op::Mov, r(2), R::R0, 4)?)?;
    c.emit(0x85b4c, branch(C::Al, 0x85b4c, 0x8607c, true)?)?;
    c.emit(0x85b50, data_immediate(C::Al, Op::Sub, R::SP, r(11), 8)?)?;
    c.emit(0x85b54, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x85b58, 0x10cff0)?;
    Ok(())
}

fn write_output_tail(c: &mut Code) -> Result<(), String> {
    c.literal(0x85b5c, r(12), 0x85b74)?;
    c.emit(0x85b60, transfer_word(C::Al, true, r(12), r(12), 0)?)?;
    c.emit(0x85b64, data_immediate(C::Al, Op::Cmp, R::R0, r(12), 0)?)?;
    c.emit(0x85b68, bx(C::Eq, R::LR))?;
    c.emit(0x85b6c, data_immediate(C::Al, Op::And, r(1), r(1), 0xe7)?)?;
    c.emit(0x85b70, branch(C::Al, 0x85b70, 0x8607c, false)?)?;
    c.data(0x85b74, 0x10cff0)?;
    Ok(())
}

fn reset_output(c: &mut Code) -> Result<(), String> {
    c.literal(0x85b78, r(2), 0x85b9c)?;
    c.emit(0x85b7c, transfer_word(C::Al, true, r(2), r(2), 0)?)?;
    c.emit(0x85b80, data_immediate(C::Al, Op::Cmp, R::R0, r(2), 0)?)?;
    c.emit(0x85b84, data_immediate(C::Ne, Op::Mov, r(2), R::R0, 0xe0)?)?;
    c.emit(0x85b88, data_register(C::Ne, Op::Bic, r(1), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b8c, data_register(C::Ne, Op::Orr, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(0x85b90, data_immediate(C::Ne, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x85b94, transfer_word(C::Ne, false, r(1), r(0), 0)?)?;
    c.emit(0x85b98, bx(C::Al, R::LR))?;
    c.data(0x85b9c, 0x10cff0)?;
    Ok(())
}

fn write_output_record(c: &mut Code) -> Result<(), String> {
    c.emit(0x85ba0, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85ba4, data_immediate(C::Al, Op::Add, r(11), R::SP, 0x1c)?)?;
    c.emit(0x85ba8, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x4c)?)?;
    c.emit(0x85bac, data_register(C::Al, Op::Mov, r(9), R::R0, r(0), Shift::Lsl, 0, false)?)?;
    c.literal(0x85bb0, r(0), 0x85d94)?;
    c.emit(0x85bb4, data_register(C::Al, Op::Mov, r(10), R::R0, r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x85bb8, data_register(C::Al, Op::Mov, r(7), R::R0, r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85bbc, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85bc0, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85bc4, branch(C::Eq, 0x85bc4, 0x85d8c, false)?)?;
    c.emit(0x85bc8, transfer_word(C::Al, false, r(1), R::SP, 4)?)?;
    c.emit(0x85bcc, data_immediate(C::Al, Op::Add, r(8), r(11), 8)?)?;
    c.emit(0x85bd0, data_immediate(C::Al, Op::Add, r(5), R::SP, 0xc)?)?;
    c.emit(0x85bd4, data_immediate(C::Al, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x85bd8, data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x30)?)?;
    c.emit(0x85bdc, transfer_multiple(C::Al, true, r(8), r(4).bit() | r(6).bit() | r(8).bit(), BlockMode::IncrementAfter, false)?)?;
    c.emit(0x85be0, data_register(C::Al, Op::Mov, r(0), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85be4, branch(C::Al, 0x85be4, 0x31ec, true)?)?;
    c.emit(0x85be8, data_register(C::Al, Op::Mov, r(0), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85bec, data_register(C::Al, Op::Mov, r(1), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x85bf0, branch(C::Al, 0x85bf0, 0x86010, true)?)?;
    c.emit(0x85bf4, transfer_word(C::Al, true, r(0), R::SP, 0x48)?)?;
    c.emit(0x85bf8, data_immediate(C::Al, Op::Mvn, r(3), R::R0, 7)?)?;
    c.emit(0x85bfc, data_register(C::Al, Op::Cmp, R::R0, r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c00, data_register(C::Al, Op::Mov, r(0), R::R0, r(7), Shift::Lsl, 0x10, false)?)?;
    c.emit(0x85c04, data_register(C::Al, Op::Orr, r(0), r(0), r(10), Shift::Lsl, 24, false)?)?;
    c.emit(0x85c08, transfer_word(C::Cs, false, r(4), R::SP, 0x48)?)?;
    c.emit(0x85c0c, data_immediate(C::Al, Op::Orr, r(1), r(0), 0x130)?)?;
    c.emit(0x85c10, transfer_word(C::Al, true, r(0), R::SP, 0x3c)?)?;
    c.emit(0x85c14, transfer_word(C::Al, false, r(1), r(0), 0)?)?;
    c.emit(0x85c18, transfer_word(C::Al, true, r(1), R::SP, 0x48)?)?;
    c.emit(0x85c1c, data_register(C::Al, Op::Sub, r(2), r(4), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c20, data_register(C::Al, Op::Add, r(1), r(8), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c24, data_register(C::Al, Op::Sub, r(1), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c28, data_immediate(C::Al, Op::Cmp, R::R0, r(2), 1)?)?;
    c.emit(0x85c2c, data_immediate(C::Al, Op::And, r(1), r(1), 0xf)?)?;
    c.emit(0x85c30, branch(C::Lt, 0x85c30, 0x85c48, false)?)?;
    c.literal(0x85c34, r(10), 0x85d94)?;
    c.emit(0x85c38, data_register(C::Al, Op::Cmp, R::R0, r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c3c, data_register(C::Cs, Op::Sub, r(1), r(1), r(2), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c40, transfer_word(C::Cs, false, r(4), R::SP, 0x48)?)?;
    c.emit(0x85c44, branch(C::Al, 0x85c44, 0x85c4c, false)?)?;
    c.literal(0x85c48, r(10), 0x85d94)?;
    c.emit(0x85c4c, data_register(C::Al, Op::Add, r(2), r(8), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c50, transfer_word(C::Al, false, r(4), R::SP, 8)?)?;
    c.emit(0x85c54, data_immediate(C::Al, Op::Cmp, R::R0, r(8), 0)?)?;
    c.emit(0x85c58, data_immediate(C::Al, Op::Bic, r(2), r(2), 0xff000000)?)?;
    c.emit(0x85c5c, data_register(C::Al, Op::Orr, r(2), r(2), r(1), Shift::Lsl, 24, false)?)?;
    c.emit(0x85c60, transfer_word(C::Al, false, r(2), r(0), 4)?)?;
    c.emit(0x85c64, data_register(C::Al, Op::Add, r(2), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c68, data_immediate(C::Al, Op::Add, r(5), r(2), 8)?)?;
    c.emit(0x85c6c, branch(C::Eq, 0x85c6c, 0x85c98, false)?)?;
    c.emit(0x85c70, transfer_word(C::Al, true, r(2), r(11), 0x14)?)?;
    c.emit(0x85c74, data_register(C::Al, Op::Add, r(1), r(1), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c78, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0)?)?;
    c.emit(0x85c7c, data_immediate(C::Al, Op::Add, r(1), r(1), 8)?)?;
    c.emit(0x85c80, transfer_byte_postindexed(C::Al, true, r(7), r(2), 1)?)?;
    c.emit(0x85c84, data_immediate(C::Al, Op::Add, r(3), r(3), 1)?)?;
    c.emit(0x85c88, data_register(C::Al, Op::Cmp, R::R0, r(3), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c8c, transfer_byte_postindexed(C::Al, false, r(7), r(5), 1)?)?;
    c.emit(0x85c90, branch(C::Ne, 0x85c90, 0x85c80, false)?)?;
    c.emit(0x85c94, data_register(C::Al, Op::Add, r(5), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85c98, transfer_word(C::Al, true, r(0), R::SP, 0x48)?)?;
    c.emit(0x85c9c, transfer_word(C::Al, true, r(8), R::SP, 4)?)?;
    c.emit(0x85ca0, data_immediate(C::Al, Op::Mov, r(7), R::R0, 0)?)?;
    c.emit(0x85ca4, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0)?)?;
    c.emit(0x85ca8, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85cac, branch(C::Eq, 0x85cac, 0x85cd8, false)?)?;
    c.emit(0x85cb0, data_register(C::Al, Op::Mov, r(0), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85cb4, transfer_byte_postindexed(C::Al, true, r(1), r(6), 1)?)?;
    c.emit(0x85cb8, data_immediate(C::Al, Op::Add, r(4), r(4), 1)?)?;
    c.emit(0x85cbc, transfer_byte_postindexed(C::Al, false, r(1), r(0), 1)?)?;
    c.emit(0x85cc0, transfer_word(C::Al, true, r(1), R::SP, 0x48)?)?;
    c.emit(0x85cc4, data_register(C::Al, Op::Cmp, R::R0, r(4), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(0x85cc8, branch(C::Cc, 0x85cc8, 0x85cb4, false)?)?;
    c.emit(0x85ccc, transfer_word(C::Al, true, r(0), r(11), 8)?)?;
    c.emit(0x85cd0, data_register(C::Al, Op::Sub, r(0), r(0), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85cd4, transfer_word(C::Al, false, r(0), R::SP, 8)?)?;
    c.emit(0x85cd8, transfer_word(C::Al, true, r(1), R::SP, 0x3c)?)?;
    c.emit(0x85cdc, data_immediate(C::Al, Op::Mov, r(0), R::R0, 0x80)?)?;
    c.emit(0x85ce0, data_register(C::Al, Op::Bic, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x85ce4, data_register(C::Al, Op::Orr, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x85ce8, branch(C::Al, 0x85ce8, 0x85e54, true)?)?;
    c.emit(0x85cec, transfer_word(C::Al, true, r(0), R::SP, 0x3c)?)?;
    c.emit(0x85cf0, data_register(C::Al, Op::Add, r(4), r(5), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85cf4, data_immediate(C::Al, Op::Add, r(3), r(0), 0x10)?)?;
    c.emit(0x85cf8, transfer_word(C::Al, true, r(0), r(10), 0)?)?;
    c.emit(0x85cfc, transfer_word(C::Al, false, r(3), R::SP, 0x3c)?)?;
    c.emit(0x85d00, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85d04, data_immediate(C::Eq, Op::Mov, r(7), R::R0, 1)?)?;
    c.emit(0x85d08, data_register(C::Al, Op::Cmp, R::R0, r(3), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d0c, branch(C::Cs, 0x85d0c, 0x85d50, false)?)?;
    c.emit(0x85d10, data_immediate(C::Al, Op::And, r(5), r(8), 0xe7)?)?;
    c.emit(0x85d14, data_immediate(C::Al, Op::Tst, R::R0, r(7), 1)?)?;
    c.emit(0x85d18, branch(C::Ne, 0x85d18, 0x85d34, false)?)?;
    c.emit(0x85d1c, data_register(C::Al, Op::Mov, r(0), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d20, data_register(C::Al, Op::Mov, r(1), R::R0, r(5), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d24, data_immediate(C::Al, Op::Mov, r(2), R::R0, 0x10)?)?;
    c.emit(0x85d28, branch(C::Al, 0x85d28, 0x8607c, true)?)?;
    c.emit(0x85d2c, transfer_word(C::Al, true, r(0), r(10), 0)?)?;
    c.emit(0x85d30, transfer_word(C::Al, true, r(3), R::SP, 0x3c)?)?;
    c.emit(0x85d34, data_immediate(C::Al, Op::Add, r(3), r(3), 0x10)?)?;
    c.emit(0x85d38, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85d3c, data_immediate(C::Al, Op::Mov, r(7), R::R0, 0)?)?;
    c.emit(0x85d40, data_immediate(C::Eq, Op::Mov, r(7), R::R0, 1)?)?;
    c.emit(0x85d44, data_register(C::Al, Op::Cmp, R::R0, r(3), r(4), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d48, transfer_word(C::Al, false, r(3), R::SP, 0x3c)?)?;
    c.emit(0x85d4c, branch(C::Cc, 0x85d4c, 0x85d14, false)?)?;
    c.emit(0x85d50, data_immediate(C::Al, Op::Cmp, R::R0, r(7), 0)?)?;
    c.emit(0x85d54, branch(C::Ne, 0x85d54, 0x85d8c, false)?)?;
    c.emit(0x85d58, transfer_word(C::Al, true, r(2), R::SP, 8)?)?;
    c.emit(0x85d5c, data_immediate(C::Al, Op::And, r(1), r(8), 0xe7)?)?;
    c.emit(0x85d60, data_register(C::Al, Op::Mov, r(0), R::R0, r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d64, data_register(C::Al, Op::Mov, r(3), R::R0, r(6), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d68, branch(C::Al, 0x85d68, 0x8607c, true)?)?;
    c.literal(0x85d6c, r(0), 0x85d94)?;
    c.emit(0x85d70, transfer_word(C::Al, true, r(0), r(0), 0)?)?;
    c.emit(0x85d74, data_immediate(C::Al, Op::Cmp, R::R0, r(0), 0)?)?;
    c.emit(0x85d78, data_immediate(C::Ne, Op::Mov, r(0), R::R0, 0xe0)?)?;
    c.emit(0x85d7c, data_immediate(C::Ne, Op::Mov, r(1), R::R0, 0)?)?;
    c.emit(0x85d80, data_register(C::Ne, Op::Bic, r(0), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d84, data_register(C::Ne, Op::Orr, r(0), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.emit(0x85d88, transfer_word(C::Ne, false, r(1), r(0), 0)?)?;
    c.emit(0x85d8c, data_immediate(C::Al, Op::Sub, R::SP, r(11), 0x1c)?)?;
    c.emit(0x85d90, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x85d94, 0x10cff0)?;
    Ok(())
}

fn write_output_wrapper(c: &mut Code) -> Result<(), String> {
    c.literal(0x85d98, r(12), 0x85ddc)?;
    c.emit(0x85d9c, transfer_word(C::Al, true, r(12), r(12), 0)?)?;
    c.emit(0x85da0, data_immediate(C::Al, Op::Cmp, R::R0, r(12), 0)?)?;
    c.emit(0x85da4, bx(C::Eq, R::LR))?;
    c.emit(0x85da8, transfer_multiple(C::Al, false, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x85dac, data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(0x85db0, data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x10)?)?;
    c.emit(0x85db4, transfer_word(C::Al, true, R::LR, r(11), 8)?)?;
    c.emit(0x85db8, transfer_word(C::Al, true, r(12), r(11), 0xc)?)?;
    c.emit(0x85dbc, data_immediate(C::Al, Op::Mov, r(4), R::R0, 0)?)?;
    c.emit(0x85dc0, transfer_word(C::Al, false, R::LR, R::SP, 0)?)?;
    c.emit(0x85dc4, transfer_word(C::Al, false, r(12), R::SP, 4)?)?;
    c.emit(0x85dc8, transfer_word(C::Al, false, r(4), R::SP, 8)?)?;
    c.emit(0x85dcc, transfer_word(C::Al, false, r(4), R::SP, 0xc)?)?;
    c.emit(0x85dd0, branch(C::Al, 0x85dd0, 0x85ba0, true)?)?;
    c.emit(0x85dd4, data_immediate(C::Al, Op::Sub, R::SP, r(11), 8)?)?;
    c.emit(0x85dd8, transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(7).bit() | r(11).bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.data(0x85ddc, 0x10cff0)?;
    Ok(())
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    initialize_list(&mut c)?;
    allocate_and_prepend(&mut c)?;
    destroy_node(&mut c)?;
    clear_list(&mut c)?;
    unlink_and_destroy(&mut c)?;
    write_output_header(&mut c)?;
    write_output_tail(&mut c)?;
    reset_output(&mut c)?;
    write_output_record(&mut c)?;
    write_output_wrapper(&mut c)?;
    c.check(END)?;
    if c.code_bytes != CODE_BYTES || c.data_bytes != DATA_BYTES
        || c.bytes.len() != END - START || c.bytes.len() != CODE_BYTES + DATA_BYTES
    {
        return Err("ARM list management partition mismatch".into());
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
        (at as isize + 8 + ((instruction << 8) as i32 >> 6) as isize) as usize
    }

    #[test]
    fn partitions_and_guards() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 1468);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(LITERALS.len() * 4, DATA_BYTES);
        assert_eq!((START + FILE_MAPPING_DELTA, END + FILE_MAPPING_DELTA), (0x89824, 0x89de0));
        let mut c = Code::new(START);
        for (emit, end, code, data) in [
            (initialize_list as fn(&mut Code) -> Result<(), String>, 0x85838, 20, 0),
            (allocate_and_prepend, 0x85904, 204, 0),
            (destroy_node, 0x85980, 124, 0),
            (clear_list, 0x859dc, 92, 0),
            (unlink_and_destroy, 0x85ab8, 220, 0),
            (write_output_header, 0x85b5c, 160, 4),
            (write_output_tail, 0x85b78, 24, 4),
            (reset_output, 0x85ba0, 36, 4),
            (write_output_record, 0x85d98, 500, 4),
            (write_output_wrapper, END, 68, 4),
        ] {
            let before = (c.code_bytes, c.data_bytes);
            emit(&mut c).unwrap();
            c.check(end).unwrap();
            assert_eq!((c.code_bytes - before.0, c.data_bytes - before.1), (code, data));
        }
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
            assert!(c.data(at, 0x10cff0).is_err());
            assert!(c.bytes.is_empty());
        }
        let mut c = Code::new(START);
        assert!(c.emit(START + 4, bx(C::Al, R::LR)).is_err());
        assert!(c.data(START, 0x10cff0).is_err());
        assert!(c.literal(START, R::R0, START).is_err());
        assert!(c.bytes.is_empty());
        assert_eq!(c.at, START);
    }

    #[test]
    fn literals_flags_and_branches() {
        let bytes = fragment().unwrap();
        for (at, slot) in [
            (0x85ac8, 0x85b58), (0x85b5c, 0x85b74), (0x85b78, 0x85b9c),
            (0x85bb0, 0x85d94), (0x85c34, 0x85d94), (0x85c48, 0x85d94),
            (0x85d6c, 0x85d94), (0x85d98, 0x85ddc),
        ] {
            let w = word(&bytes, at);
            assert_eq!((w >> 16) & 15, 15);
            assert_eq!(at + 8 + (w & 0xfff) as usize, slot);
            assert_eq!(w & (1 << 20), 1 << 20);
        }
        for at in (START..END).step_by(4) {
            if LITERALS.iter().any(|&(slot, _)| at == slot) { continue; }
            let w = word(&bytes, at);
            if (w >> 26) & 3 == 0 {
                let op = (w >> 21) & 15;
                assert_eq!(w & (1 << 20) != 0, op == Op::Cmp as u32 || op == Op::Tst as u32);
            }
            if (w >> 25) & 7 == 5 {
                let destination = target(&bytes, at);
                assert!(!LITERALS.iter().any(|&(slot, _)| destination == slot));
                assert!(destination % 4 == 0);
                if w & (1 << 24) != 0 {
                    assert!([0x31ec, 0x57d4, 0x5838, 0x85904, 0x85ba0, 0x85e54, 0x86010, 0x8607c].contains(&destination));
                } else {
                    assert!((START..END).contains(&destination) || destination == 0x8607c);
                }
            }
        }
        for (at, destination, condition) in [
            (0x8586c, 0x858f8, C::Eq), (0x858a8, 0x858f0, C::Eq),
            (0x859bc, 0x85998, C::Ne), (0x859c8, 0x85904, C::Ne),
            (0x85a2c, 0x85a08, C::Ne), (0x85b00, 0x85b1c, C::Hi),
            (0x85b70, 0x8607c, C::Al), (0x85c30, 0x85c48, C::Lt),
            (0x85c90, 0x85c80, C::Ne), (0x85cc8, 0x85cb4, C::Cc),
            (0x85d0c, 0x85d50, C::Cs), (0x85d4c, 0x85d14, C::Cc),
            (0x85dd0, 0x85ba0, C::Al),
        ] {
            assert_eq!(target(&bytes, at), destination);
            assert_eq!(word(&bytes, at) >> 28, condition as u32);
        }
        for at in [0x85828, 0x8582c, 0x85830, 0x85a84, 0x85a88, 0x85a8c, 0x85a90, 0x85a94] {
            assert_eq!(word(&bytes, at) >> 28, C::Ne as u32);
        }
        assert_eq!(word(&bytes, 0x85b68), bx(C::Eq, R::LR));
        assert_eq!(word(&bytes, 0x85da4), bx(C::Eq, R::LR));
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
