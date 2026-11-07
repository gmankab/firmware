use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
    branch, bx, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word, transfer_word_postindexed,
};

pub const START: usize = 0x804fc;
pub const END: usize = 0x80c18;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1796;
pub const DATA_BYTES: usize = 24;
const POOLS: [(usize, u32); 6] = [
    (0x8075c, 0xf4bc2), (0x80808, 0xf4bc2),
    (0x808c4, 0x2efc), (0x808c8, 0x2f1c),
    (0x808cc, 0x806d4), (0x80a20, 0xf4bc2),
];

fn r(number: u8) -> R {
    R::new(number).expect("valid ARM register")
}

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
    code_bytes: usize,
    data_bytes: usize,
}

impl TypedA32 {
    fn new() -> Self {
        Self { at: START, bytes: Vec::with_capacity(END - START), code_bytes: 0, data_bytes: 0 }
    }

    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("dispatch allocator address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        let pool = POOLS.iter().find(|(at, _)| *at == self.at).map(|(_, value)| *value);
        if !(START..END).contains(&self.at) || self.at % 4 != 0
            || if data { pool != Some(value) } else { pool.is_some() }
        {
            return Err(format!("invalid dispatch allocator word at {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        if data { self.data_bytes += 4; } else { self.code_bytes += 4; }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn pool(&mut self, slot: usize, value: u32) -> Result<(), String> {
        self.check(slot)?;
        self.word(value, true)
    }

    fn immediate(&mut self, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn mov(&mut self, condition: C, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(condition, r(rd), r(rm)))
    }

    fn transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn literal(&mut self, rd: u8, slot: usize) -> Result<(), String> {
        if !POOLS.iter().any(|(at, _)| *at == slot) {
            return Err("invalid dispatch allocator literal target".into());
        }
        self.transfer(true, rd, 15, slot as i32 - self.at as i32 - 8)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn pop(&mut self, registers: u16) -> Result<(), String> {
        self.emit(transfer_multiple(C::Al, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }

    fn finish(self) -> Result<Vec<u8>, String> {
        self.check(END)?;
        if self.bytes.len() != END - START || self.code_bytes != CODE_BYTES || self.data_bytes != DATA_BYTES {
            return Err("invalid dispatch allocator byte counts".into());
        }
        Ok(self.bytes)
    }
}

fn initialize_dispatch(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x804fc)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(C::Al, 4, 2)?;
    c.immediate(C::Al, Op::Mov, 6, 0, 0x0)?;
    c.immediate(C::Al, Op::Add, 5, 4, 0x10)?;
    c.emit(transfer_multiple(C::Al, false, r(4), r(0).bit() | r(1).bit() | r(3).bit(), BlockMode::IncrementAfter, false)?)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x20)?;
    c.branch(C::Al, 0x2f28, true)?;
    c.immediate(C::Al, Op::Add, 6, 6, 0x1)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(0), r(5), 4)?)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0x2)?;
    c.branch(C::Ne, 0x80514, false)?;
    c.immediate(C::Al, Op::Mov, 5, 0, 0x0)?;
    c.immediate(C::Al, Op::Add, 6, 4, 0x1c)?;
    c.transfer(false, 5, 4, 0x18)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x20)?;
    c.branch(C::Al, 0x2f28, true)?;
    c.immediate(C::Al, Op::Add, 5, 5, 0x1)?;
    c.emit(transfer_word_postindexed(C::Al, false, r(0), r(6), 4)?)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0x2)?;
    c.branch(C::Ne, 0x80538, false)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.transfer(false, 0, 4, 0x24)?;
    c.pop(r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x8055c)
}

fn tail_slot_00(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x8055c)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x0)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80570)
}

fn tail_slot_04(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80570)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x4)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80584)
}

fn tail_slot_08(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80584)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x8)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80598)
}

fn tail_slot_0c(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80598)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0xc)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x805ac)
}

fn tail_slot_10(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x805ac)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x10)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x805c0)
}

fn tail_slot_18(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x805c0)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x18)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x805d4)
}

fn tail_slot_1c(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x805d4)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x1c)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x805e8)
}

fn tail_slot_20(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x805e8)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x20)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x805fc)
}

fn tail_slot_24(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x805fc)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x24)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80610)
}

fn tail_slot_28(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80610)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x28)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80624)
}

fn tail_slot_14(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80624)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x14)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80638)
}

fn tail_slot_2c(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80638)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x2c)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x8064c)
}

fn tail_slot_30(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x8064c)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x30)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80660)
}

fn tail_slot_34(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80660)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x34)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80674)
}

fn tail_slot_38(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80674)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x38)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x80688)
}

fn tail_slot_3c(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80688)?;
    c.transfer(true, 1, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 2, 0, 0x3c)?;
    c.mov(C::Al, 0, 1)?;
    c.emit(bx(C::Al, r(2)))?;
    c.check(0x8069c)
}

fn tail_slot_40(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x8069c)?;
    c.emit(push_registers(C::Al, r(11).bit() | r(14).bit())?)?;
    c.mov(C::Al, 11, 13)?;
    c.transfer(true, 12, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 14, 0, 0x40)?;
    c.mov(C::Al, 0, 12)?;
    c.mov(C::Al, 12, 14)?;
    c.pop(r(11).bit() | r(14).bit())?;
    c.emit(bx(C::Al, r(12)))?;
    c.check(0x806c0)
}

fn tail_slot_48(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x806c0)?;
    c.transfer(true, 2, 0, 0x0)?;
    c.transfer(true, 0, 0, 0x4)?;
    c.transfer(true, 3, 0, 0x48)?;
    c.mov(C::Al, 0, 2)?;
    c.emit(bx(C::Al, r(3)))?;
    c.check(0x806d4)
}

fn copy_fixed_and_append(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x806d4)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 5, 0)?;
    c.mov(C::Al, 4, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.branch(C::Eq, 0x80720, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0x0)?;
    c.branch(C::Eq, 0x80734, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, 0x0)?;
    c.branch(C::Eq, 0x80748, false)?;
    c.transfer(true, 3, 5, 0x14)?;
    c.transfer(true, 0, 4, 0x0)?;
    c.mov(C::Al, 1, 2)?;
    c.mov(C::Al, 2, 3)?;
    c.branch(C::Al, 0x471c, true)?;
    c.immediate(C::Al, Op::Add, 0, 5, 0x18)?;
    c.mov(C::Al, 1, 4)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x80d78, false)?;
    c.check(0x80720)?;
    c.literal(2, 0x8075c)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x21)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x80734)?;
    c.literal(2, 0x8075c)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x28)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x80748)?;
    c.literal(2, 0x8075c)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x2f)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x17b0, false)?;
    c.pool(0x8075c, 0xf4bc2)?;
    c.check(0x80760)
}

fn copy_sized_and_append(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80760)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x10)?;
    c.mov(C::Al, 5, 0)?;
    c.mov(C::Al, 6, 2)?;
    c.mov(C::Al, 4, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.branch(C::Eq, 0x807cc, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0x0)?;
    c.branch(C::Eq, 0x807e0, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0x0)?;
    c.branch(C::Eq, 0x807f4, false)?;
    c.transfer(true, 1, 5, 0x14)?;
    c.transfer(true, 2, 6, 0x0)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(1), Shift::Lsl, 0, false)?)?;
    c.literal(2, 0x80808)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x62)?;
    c.immediate(C::Ls, Op::Mov, 0, 0, 0x1)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.transfer(true, 2, 6, 0x0)?;
    c.transfer(true, 0, 4, 0x0)?;
    c.mov(C::Al, 1, 6)?;
    c.branch(C::Al, 0x471c, true)?;
    c.immediate(C::Al, Op::Add, 0, 5, 0x18)?;
    c.mov(C::Al, 1, 4)?;
    c.pop(r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x80d78, false)?;
    c.check(0x807cc)?;
    c.literal(2, 0x80808)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x4d)?;
    c.pop(r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x807e0)?;
    c.literal(2, 0x80808)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x54)?;
    c.pop(r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x807f4)?;
    c.literal(2, 0x80808)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x5b)?;
    c.pop(r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x17b0, false)?;
    c.pool(0x80808, 0xf4bc2)?;
    c.check(0x8080c)
}

fn initialize_allocator(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x8080c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x18)?;
    c.immediate(C::Al, Op::Sub, 13, 13, 0x18)?;
    c.mov(C::Al, 6, 0)?;
    c.literal(0, 0x808c4)?;
    c.mov(C::Al, 5, 1)?;
    c.transfer(true, 1, 11, 0xc)?;
    c.mov(C::Al, 7, 3)?;
    c.mov(C::Al, 8, 2)?;
    c.immediate(C::Al, Op::Add, 4, 6, 0x2c)?;
    c.transfer(false, 0, 13, 0x8)?;
    c.literal(0, 0x808c8)?;
    c.transfer(false, 0, 13, 0x10)?;
    c.transfer(true, 0, 11, 0x8)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.transfer(false, 0, 6, 0x30)?;
    c.mov(C::Eq, 1, 0)?;
    c.mov(C::Al, 0, 4)?;
    c.transfer(false, 1, 6, 0x34)?;
    c.branch(C::Al, 0x2b8c, true)?;
    c.mov(C::Al, 1, 0)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0x0)?;
    c.branch(C::Ne, 0x808bc, false)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.branch(C::Eq, 0x80888, false)?;
    c.immediate(C::Al, Op::Add, 1, 13, 0x8)?;
    c.mov(C::Al, 0, 6)?;
    c.mov(C::Al, 2, 5)?;
    c.transfer(false, 7, 13, 0x0)?;
    c.branch(C::Al, 0x80898, false)?;
    c.literal(2, 0x808cc)?;
    c.transfer(false, 7, 13, 0x0)?;
    c.immediate(C::Al, Op::Add, 1, 13, 0x8)?;
    c.mov(C::Al, 0, 6)?;
    c.mov(C::Al, 3, 8)?;
    c.branch(C::Al, 0x81440, true)?;
    c.mov(C::Al, 1, 0)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x1)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0x1)?;
    c.branch(C::Eq, 0x808bc, false)?;
    c.transfer(true, 0, 4, 0x0)?;
    c.branch(C::Al, 0x2bcc, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Sub, 13, 11, 0x18)?;
    c.pop(r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit())?;
    c.pool(0x808c4, 0x2efc)?;
    c.pool(0x808c8, 0x2f1c)?;
    c.pool(0x808cc, 0x806d4)?;
    c.check(0x808d0)
}

fn destroy_allocator(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x808d0)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bcc, true)?;
    c.mov(C::Al, 0, 4)?;
    c.pop(r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x816cc, false)?;
    c.check(0x808f0)
}

fn query_available(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x808f0)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81438, true)?;
    c.mov(C::Al, 5, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.immediate(C::Ne, Op::Mov, 5, 0, 0x1)?;
    c.mov(C::Al, 0, 5)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80928)
}

fn append_and_notify(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80928)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.mov(C::Al, 5, 1)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81438, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x80960, false)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 5, 0, 0x0)?;
    c.branch(C::Al, 0x8098c, false)?;
    c.mov(C::Al, 0, 4)?;
    c.mov(C::Al, 1, 5)?;
    c.branch(C::Al, 0x81530, true)?;
    c.mov(C::Al, 5, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.transfer(true, 0, 4, 0x30)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x8098c, false)?;
    c.transfer(true, 1, 4, 0x34)?;
    c.branch(C::Al, 0x2d20, true)?;
    c.mov(C::Al, 0, 5)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80994)
}

fn copy_fixed_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80994)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.mov(C::Al, 5, 1)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81430, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x809cc, false)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0x0)?;
    c.branch(C::Al, 0x80a18, false)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815ec, true)?;
    c.mov(C::Al, 1, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0x0)?;
    c.branch(C::Eq, 0x80a04, false)?;
    c.transfer(true, 2, 4, 0x14)?;
    c.mov(C::Al, 0, 5)?;
    c.branch(C::Al, 0x471c, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815a4, true)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0x1)?;
    c.branch(C::Al, 0x80a18, false)?;
    c.check(0x80a04)?;
    c.literal(2, 0x80a20)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x11c)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0x0)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.mov(C::Al, 0, 4)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.pool(0x80a20, 0xf4bc2)?;
    c.check(0x80a24)
}

fn copy_sized_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80a24)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.mov(C::Al, 5, 1)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81430, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x80a5c, false)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.check(0x80a54)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815ec, true)?;
    c.mov(C::Al, 1, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0x0)?;
    c.branch(C::Eq, 0x80a54, false)?;
    c.transfer(true, 2, 1, 0x0)?;
    c.mov(C::Al, 0, 5)?;
    c.branch(C::Al, 0x471c, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815a4, true)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x1)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80a94)
}

fn borrow_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80a94)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.mov(C::Al, 5, 1)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81430, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x80acc, false)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.check(0x80ac4)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815ec, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x80ac4, false)?;
    c.transfer(false, 0, 5, 0x0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x1)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80af0)
}

fn advance_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80af0)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815a4, true)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x1)?;
    c.pop(r(4).bit() | r(7).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80b1c)
}

fn query_empty(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80b1c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81430, true)?;
    c.mov(C::Al, 5, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 5, 0x0)?;
    c.immediate(C::Ne, Op::Mov, 5, 0, 0x1)?;
    c.mov(C::Al, 0, 5)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80b54)
}

fn drain_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80b54)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.branch(C::Al, 0x80b74, false)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x815a4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81430, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x80b6c, false)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.pop(r(4).bit() | r(7).bit() | r(11).bit() | r(14).bit())?;
    c.branch(C::Al, 0x2c34, false)?;
    c.check(0x80b90)
}

fn query_count(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80b90)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.immediate(C::Al, Op::Add, 0, 4, 0x18)?;
    c.branch(C::Al, 0x813d8, true)?;
    c.mov(C::Al, 5, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.mov(C::Al, 0, 5)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80bc0)
}

fn append_locked(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x80bc0)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(11).bit() | r(14).bit())?)?;
    c.immediate(C::Al, Op::Add, 11, 13, 0x8)?;
    c.mov(C::Al, 4, 0)?;
    c.mov(C::Al, 5, 1)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2bf4, true)?;
    c.mov(C::Al, 0, 4)?;
    c.branch(C::Al, 0x81438, true)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x0)?;
    c.branch(C::Eq, 0x80bf8, false)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x0)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.mov(C::Al, 0, 4)?;
    c.mov(C::Al, 1, 5)?;
    c.branch(C::Al, 0x81530, true)?;
    c.mov(C::Al, 5, 0)?;
    c.transfer(true, 0, 4, 0x2c)?;
    c.branch(C::Al, 0x2c34, true)?;
    c.mov(C::Al, 0, 5)?;
    c.pop(r(4).bit() | r(5).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x80c18)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32::new();
    initialize_dispatch(&mut c)?;
    tail_slot_00(&mut c)?;
    tail_slot_04(&mut c)?;
    tail_slot_08(&mut c)?;
    tail_slot_0c(&mut c)?;
    tail_slot_10(&mut c)?;
    tail_slot_18(&mut c)?;
    tail_slot_1c(&mut c)?;
    tail_slot_20(&mut c)?;
    tail_slot_24(&mut c)?;
    tail_slot_28(&mut c)?;
    tail_slot_14(&mut c)?;
    tail_slot_2c(&mut c)?;
    tail_slot_30(&mut c)?;
    tail_slot_34(&mut c)?;
    tail_slot_38(&mut c)?;
    tail_slot_3c(&mut c)?;
    tail_slot_40(&mut c)?;
    tail_slot_48(&mut c)?;
    copy_fixed_and_append(&mut c)?;
    copy_sized_and_append(&mut c)?;
    initialize_allocator(&mut c)?;
    destroy_allocator(&mut c)?;
    query_available(&mut c)?;
    append_and_notify(&mut c)?;
    copy_fixed_locked(&mut c)?;
    copy_sized_locked(&mut c)?;
    borrow_locked(&mut c)?;
    advance_locked(&mut c)?;
    query_empty(&mut c)?;
    drain_locked(&mut c)?;
    query_count(&mut c)?;
    append_locked(&mut c)?;
    c.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at - START..at - START + 4].try_into().unwrap())
    }

    #[test]
    fn extents_pools_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1820);
        assert_eq!(END - START, CODE_BYTES + DATA_BYTES);
        assert_eq!(CODE_BYTES, 449 * 4);
        assert_eq!(DATA_BYTES, POOLS.len() * 4);
        for (slot, value) in POOLS {
            assert_eq!(word(&bytes, slot), value);
        }
        for (at, condition, target, link) in [
            (0x80518, C::Al, 0x2f28, true),
            (0x80528, C::Ne, 0x80514, false),
            (0x8053c, C::Al, 0x2f28, true),
            (0x8054c, C::Ne, 0x80538, false),
            (0x806e8, C::Eq, 0x80720, false),
            (0x806f0, C::Eq, 0x80734, false),
            (0x806f8, C::Eq, 0x80748, false),
            (0x8070c, C::Al, 0x471c, true),
            (0x8071c, C::Al, 0x80d78, false),
            (0x80730, C::Al, 0x17b0, false),
            (0x80744, C::Al, 0x17b0, false),
            (0x80758, C::Al, 0x17b0, false),
            (0x80778, C::Eq, 0x807cc, false),
            (0x80780, C::Eq, 0x807e0, false),
            (0x80788, C::Eq, 0x807f4, false),
            (0x807a8, C::Al, 0x17b0, true),
            (0x807b8, C::Al, 0x471c, true),
            (0x807c8, C::Al, 0x80d78, false),
            (0x807dc, C::Al, 0x17b0, false),
            (0x807f0, C::Al, 0x17b0, false),
            (0x80804, C::Al, 0x17b0, false),
            (0x80858, C::Al, 0x2b8c, true),
            (0x80868, C::Ne, 0x808bc, false),
            (0x80870, C::Eq, 0x80888, false),
            (0x80884, C::Al, 0x80898, false),
            (0x8089c, C::Al, 0x81440, true),
            (0x808ac, C::Eq, 0x808bc, false),
            (0x808b4, C::Al, 0x2bcc, true),
            (0x808e0, C::Al, 0x2bcc, true),
            (0x808ec, C::Al, 0x816cc, false),
            (0x80900, C::Al, 0x2bf4, true),
            (0x80908, C::Al, 0x81438, true),
            (0x80914, C::Al, 0x2c34, true),
            (0x8093c, C::Al, 0x2bf4, true),
            (0x80944, C::Al, 0x81438, true),
            (0x8094c, C::Eq, 0x80960, false),
            (0x80954, C::Al, 0x2c34, true),
            (0x8095c, C::Al, 0x8098c, false),
            (0x80968, C::Al, 0x81530, true),
            (0x80974, C::Al, 0x2c34, true),
            (0x80980, C::Eq, 0x8098c, false),
            (0x80988, C::Al, 0x2d20, true),
            (0x809a8, C::Al, 0x2bf4, true),
            (0x809b0, C::Al, 0x81430, true),
            (0x809b8, C::Eq, 0x809cc, false),
            (0x809c0, C::Al, 0x2c34, true),
            (0x809c8, C::Al, 0x80a18, false),
            (0x809d0, C::Al, 0x815ec, true),
            (0x809dc, C::Eq, 0x80a04, false),
            (0x809e8, C::Al, 0x471c, true),
            (0x809f0, C::Al, 0x815a4, true),
            (0x809f8, C::Al, 0x2c34, true),
            (0x80a00, C::Al, 0x80a18, false),
            (0x80a14, C::Al, 0x17b0, true),
            (0x80a38, C::Al, 0x2bf4, true),
            (0x80a40, C::Al, 0x81430, true),
            (0x80a48, C::Eq, 0x80a5c, false),
            (0x80a50, C::Al, 0x2c34, true),
            (0x80a60, C::Al, 0x815ec, true),
            (0x80a6c, C::Eq, 0x80a54, false),
            (0x80a78, C::Al, 0x471c, true),
            (0x80a80, C::Al, 0x815a4, true),
            (0x80a88, C::Al, 0x2c34, true),
            (0x80aa8, C::Al, 0x2bf4, true),
            (0x80ab0, C::Al, 0x81430, true),
            (0x80ab8, C::Eq, 0x80acc, false),
            (0x80ac0, C::Al, 0x2c34, true),
            (0x80ad0, C::Al, 0x815ec, true),
            (0x80ad8, C::Eq, 0x80ac4, false),
            (0x80ae4, C::Al, 0x2c34, true),
            (0x80b00, C::Al, 0x2bf4, true),
            (0x80b08, C::Al, 0x815a4, true),
            (0x80b10, C::Al, 0x2c34, true),
            (0x80b2c, C::Al, 0x2bf4, true),
            (0x80b34, C::Al, 0x81430, true),
            (0x80b40, C::Al, 0x2c34, true),
            (0x80b64, C::Al, 0x2bf4, true),
            (0x80b68, C::Al, 0x80b74, false),
            (0x80b70, C::Al, 0x815a4, true),
            (0x80b78, C::Al, 0x81430, true),
            (0x80b80, C::Eq, 0x80b6c, false),
            (0x80b8c, C::Al, 0x2c34, false),
            (0x80ba0, C::Al, 0x2bf4, true),
            (0x80ba8, C::Al, 0x813d8, true),
            (0x80bb4, C::Al, 0x2c34, true),
            (0x80bd4, C::Al, 0x2bf4, true),
            (0x80bdc, C::Al, 0x81438, true),
            (0x80be4, C::Eq, 0x80bf8, false),
            (0x80bec, C::Al, 0x2c34, true),
            (0x80c00, C::Al, 0x81530, true),
            (0x80c0c, C::Al, 0x2c34, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, link).unwrap());
        }
        for (entry, slot) in [
            (0x8055c, 0), (0x80570, 4), (0x80584, 8), (0x80598, 12),
            (0x805ac, 16), (0x805c0, 24), (0x805d4, 28), (0x805e8, 32),
            (0x805fc, 36), (0x80610, 40), (0x80624, 20), (0x80638, 44),
            (0x8064c, 48), (0x80660, 52), (0x80674, 56), (0x806c0, 72),
        ] {
            assert_eq!(word(&bytes, entry + 8), transfer_word(C::Al, true, r(3), r(0), slot).unwrap());
            assert_eq!(word(&bytes, entry + 16), bx(C::Al, r(3)));
        }
        assert_eq!(word(&bytes, 0x80690), transfer_word(C::Al, true, r(2), r(0), 0x3c).unwrap());
        assert_eq!(word(&bytes, 0x80698), bx(C::Al, r(2)));
        assert_eq!(word(&bytes, 0x806ac), transfer_word(C::Al, true, r(14), r(0), 0x40).unwrap());
        assert_eq!(word(&bytes, 0x806bc), bx(C::Al, r(12)));
    }

    #[test]
    fn guards_reject_without_mutation() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, END, END + 4]) {
            let valid = (START..END).contains(&at) && at % 4 == 0;
            let pool = POOLS.iter().any(|(slot, _)| *slot == at);
            let mut c = TypedA32::new();
            c.at = at;
            let ok = valid && !pool;
            assert_eq!(c.emit(mov_register(C::Al, r(0), r(0))).is_ok(), ok);
            assert_eq!(c.at, at + if ok { 4 } else { 0 });
            assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
            assert_eq!(c.code_bytes, if ok { 4 } else { 0 });
            assert_eq!(c.data_bytes, 0);
            for (slot, value) in POOLS {
                for candidate in [value, value ^ 1] {
                    let mut c = TypedA32::new();
                    c.at = at;
                    let ok = valid && at == slot && candidate == value;
                    assert_eq!(c.pool(slot, candidate).is_ok(), ok);
                    assert_eq!(c.at, at + if ok { 4 } else { 0 });
                    assert_eq!(c.bytes.len(), if ok { 4 } else { 0 });
                    assert_eq!(c.code_bytes, 0);
                    assert_eq!(c.data_bytes, if ok { 4 } else { 0 });
                }
            }
        }
        let mut c = TypedA32::new();
        assert!(c.literal(0, START).is_err());
        assert!(c.pool(START, 0xf4bc2).is_err());
        assert!(c.check(START + 4).is_err());
        assert_eq!((c.at, c.bytes.len(), c.code_bytes, c.data_bytes), (START, 0, 0, 0));
        assert!(c.finish().is_err());
        let mut c = TypedA32::new();
        c.at = END;
        assert!(c.finish().is_err());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let path = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE"));
        let Some(path) = path else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
