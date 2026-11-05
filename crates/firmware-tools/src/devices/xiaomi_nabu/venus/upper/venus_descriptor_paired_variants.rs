use arm32::{
    branch, data_immediate, data_register, mov_register, push_registers,
    transfer_multiple, transfer_word, transfer_word_register_preindexed_writeback,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x104d8;
pub const END: usize = 0x10ee4;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const LITERALS: [(usize, u32); 4] = [
    (0x109c4, 0xe3dfc), (0x109c8, 0xe2788),
    (0x10edc, 0xe3dfc), (0x10ee0, 0xe2788),
];

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, at: usize) -> Result<(), String> {
        if self.at != at {
            return Err(format!("descriptor-paired cursor {:#x} != {at:#x}", self.at));
        }
        Ok(())
    }

    fn code(at: usize) -> bool {
        at % 4 == 0 && ((START..0x109c4).contains(&at) || (0x109cc..0x10edc).contains(&at))
    }

    fn word(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::code(self.at) {
            return Err("descriptor-paired instruction outside code".into());
        }
        self.word(instruction);
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !LITERALS.contains(&(at, value)) {
            return Err("descriptor-paired literal mismatch".into());
        }
        self.word(value);
        Ok(())
    }

    fn literal(&mut self, dst: R, at: usize) -> Result<(), String> {
        let pool = if self.at < 0x109c4 { 0x109c4 } else { 0x10edc };
        if ![pool, pool + 4].contains(&at) {
            return Err("descriptor-paired literal target outside function pool".into());
        }
        self.emit(transfer_word(C::Al, true, dst, R::PC, at as i32 - self.at as i32 - 8)?)
    }

    fn jump(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let same_function = if self.at < 0x109c4 {
            (START..0x109c4).contains(&target)
        } else {
            (0x109cc..0x10edc).contains(&target)
        };
        if (link && (condition != C::Al || target != 0x2ec94))
            || (!link && (!same_function || !Self::code(target)))
        {
            return Err("descriptor-paired invalid branch target".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn r(n: u8) -> R { R::new(n).unwrap() }

fn saved(returning: bool) -> u16 {
    (4..=11).fold(if returning { R::PC.bit() } else { R::LR.bit() }, |mask, n| mask | r(n).bit())
}

fn imm(c: &mut Code, op: Op, dst: R, src: R, value: u32) -> Result<(), String> {
    c.emit(data_immediate(C::Al, op, dst, src, value)?)
}

fn mov(c: &mut Code, dst: R, src: R) -> Result<(), String> {
    c.emit(mov_register(C::Al, dst, src))
}

fn add(c: &mut Code, dst: R, left: R, right: R) -> Result<(), String> {
    c.emit(data_register(C::Al, Op::Add, dst, left, right, Shift::Lsl, 0, false)?)
}

fn load(c: &mut Code, dst: R, base: R, offset: i32) -> Result<(), String> {
    c.emit(transfer_word(C::Al, true, dst, base, offset)?)
}

fn store(c: &mut Code, src: R, base: R, offset: i32) -> Result<(), String> {
    c.emit(transfer_word(C::Al, false, src, base, offset)?)
}

fn context_pointer(c: &mut Code, dst: R, base: R, offset: R) -> Result<(), String> {
    c.emit(transfer_word_register_preindexed_writeback(C::Al, true, dst, base, offset, Shift::Lsl, 0)?)
}

fn argument_entry(c: &mut Code) -> Result<(), String> {
    c.check(0x104d8)?;
    c.emit(push_registers(C::Al, saved(false))?)?;
    imm(c, Op::Add, R::R11, R::SP, 0x1c)?;
    imm(c, Op::Sub, R::SP, R::SP, 0x4)?;
    mov(c, r(4), r(0))?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(0), r(4), 0x1c)?;
    load(c, r(0), r(4), 0xa58)?;
    load(c, r(0), r(0), 0x330)?;
    store(c, r(1), R::SP, 0x0)?;
    imm(c, Op::Cmp, R::R0, r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    c.jump(C::Ne, 0x10628, false)?;
    c.check(0x1050c)
}

fn argument_extended_first(c: &mut Code) -> Result<(), String> {
    c.check(0x1050c)?;
    mov(c, r(7), r(4))?;
    load(c, r(2), r(4), 0xd80)?;
    imm(c, Op::Mov, r(6), R::R0, 0x20c)?;
    load(c, R::LR, r(4), 0xd78)?;
    imm(c, Op::Mov, r(12), R::R0, 0x22c)?;
    load(c, r(8), r(4), 0xd84)?;
    load(c, r(9), r(4), 0xd7c)?;
    mov(c, r(5), r(4))?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    context_pointer(c, r(0), r(7), r(0))?;
    imm(c, Op::Orr, r(6), r(6), 0x9400)?;
    imm(c, Op::Orr, r(12), r(12), 0x1c00)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    add(c, r(2), r(1), r(2))?;
    store(c, r(6), r(0), 0x0)?;
    add(c, r(1), r(2), R::LR)?;
    mov(c, r(2), r(9))?;
    context_pointer(c, r(6), r(5), r(12))?;
    add(c, r(8), r(1), r(8))?;
    imm(c, Op::Mov, r(1), R::R0, 0x28c)?;
    imm(c, Op::Mov, r(12), R::R0, 0x12)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(12), r(6), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    store(c, r(8), r(6), 0x4)?;
    store(c, r(1), r(0), 0x8)?;
    imm(c, Op::Add, r(1), r(0), 0x10)?;
    store(c, r(12), r(6), 0x8)?;
    store(c, r(1), r(7), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x288)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(1), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(6), 0x10)?;
    c.literal(r(1), 0x109c4)?;
    store(c, r(0), r(5), 0x0)?;
    imm(c, Op::Add, r(0), r(9), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(10), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(10), r(6), 0xc)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(6), 0x109c8)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(6))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(7), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x12)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(5), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(8), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(7), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(9))?;
    store(c, r(0), r(5), 0x0)?;
    store(c, r(10), r(1), 0xc)?;
    c.jump(C::Al, 0x10730, false)?;
    c.check(0x10628)
}

fn argument_simple_first(c: &mut Code) -> Result<(), String> {
    c.check(0x10628)?;
    mov(c, r(5), r(4))?;
    imm(c, Op::Mov, r(2), R::R0, 0x20c)?;
    load(c, r(3), r(4), 0xd80)?;
    load(c, r(10), r(4), 0xd78)?;
    mov(c, r(7), r(4))?;
    imm(c, Op::Mov, r(6), R::R0, 0x208)?;
    imm(c, Op::Mov, r(12), R::R0, 0x12)?;
    context_pointer(c, r(0), r(5), r(0))?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    imm(c, Op::Orr, r(6), r(6), 0x9400)?;
    add(c, r(8), r(1), r(3))?;
    imm(c, Op::Mov, r(1), R::R0, 0x28c)?;
    store(c, r(2), r(0), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x22c)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1c00)?;
    context_pointer(c, r(2), r(7), r(2))?;
    store(c, r(12), r(2), 0x0)?;
    store(c, r(6), r(0), 0x4)?;
    store(c, r(8), r(2), 0x4)?;
    store(c, r(1), r(0), 0x8)?;
    imm(c, Op::Add, r(1), r(0), 0x10)?;
    store(c, r(12), r(2), 0x8)?;
    store(c, r(1), r(5), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x288)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(1), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(2), 0x10)?;
    c.literal(r(1), 0x109c4)?;
    store(c, r(0), r(7), 0x0)?;
    imm(c, Op::Add, r(0), r(10), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(9), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(9), r(2), 0xc)?;
    mov(c, r(2), r(10))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(6), 0x109c8)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(6))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(5), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x12)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(7), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(8), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(5), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(10))?;
    store(c, r(0), r(7), 0x0)?;
    store(c, r(9), r(1), 0xc)?;
    c.check(0x10730)
}

fn argument_second_dispatch(c: &mut Code) -> Result<(), String> {
    c.check(0x10730)?;
    c.literal(r(1), 0x109c4)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(6))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(4), 0xd70)?;
    imm(c, Op::Cmp, R::R0, r(0), 0x1)?;
    c.jump(C::Ne, 0x109b8, false)?;
    load(c, r(0), r(4), 0xa58)?;
    load(c, r(6), r(4), 0xd80)?;
    load(c, r(0), r(0), 0x330)?;
    imm(c, Op::Cmp, R::R0, r(0), 0x1)?;
    c.jump(C::Ne, 0x10898, false)?;
    c.check(0x10768)
}

fn argument_extended_second(c: &mut Code) -> Result<(), String> {
    c.check(0x10768)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    mov(c, r(5), r(4))?;
    imm(c, Op::Mov, r(2), R::R0, 0x20c)?;
    load(c, r(1), r(4), 0xd78)?;
    load(c, r(7), r(4), 0xd84)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Mov, r(12), R::R0, 0x13)?;
    c.literal(r(8), 0x109c4)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    context_pointer(c, r(0), r(5), r(0))?;
    store(c, r(2), r(0), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x22c)?;
    imm(c, Op::Orr, r(2), r(2), 0x1c00)?;
    context_pointer(c, r(2), r(4), r(2))?;
    store(c, r(12), r(2), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    load(c, r(3), R::SP, 0x0)?;
    add(c, r(6), r(3), r(6))?;
    add(c, r(9), r(6), r(1))?;
    imm(c, Op::Mov, r(1), R::R0, 0x28c)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(9), r(2), 0x4)?;
    store(c, r(1), r(0), 0x8)?;
    imm(c, Op::Add, r(1), r(0), 0x10)?;
    store(c, r(12), r(2), 0x8)?;
    store(c, r(1), r(5), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x288)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(1), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(2), 0x10)?;
    mov(c, r(1), r(8))?;
    store(c, r(0), r(4), 0x0)?;
    imm(c, Op::Add, r(0), r(7), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(10), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(10), r(2), 0xc)?;
    mov(c, r(2), r(7))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(6), 0x109c8)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(6))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(5), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x13)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(4), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(9), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(5), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(7))?;
    store(c, r(0), r(4), 0x0)?;
    store(c, r(10), r(1), 0xc)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(8))?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(6))?;
    c.jump(C::Al, 0x109b4, false)?;
    c.check(0x10898)
}

fn argument_simple_second(c: &mut Code) -> Result<(), String> {
    c.check(0x10898)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    mov(c, r(5), r(4))?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Mov, r(2), R::R0, 0x13)?;
    c.literal(r(9), 0x109c4)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    context_pointer(c, r(0), r(5), r(0))?;
    store(c, r(1), r(0), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x22c)?;
    imm(c, Op::Orr, r(1), r(1), 0x1c00)?;
    context_pointer(c, r(1), r(4), r(1))?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    load(c, r(10), R::SP, 0x0)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(10), r(1), 0x4)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(5), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(6))?;
    store(c, r(0), r(4), 0x0)?;
    imm(c, Op::Add, r(0), r(6), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(8), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(8), r(1), 0xc)?;
    mov(c, r(1), r(9))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(7), 0x109c8)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(7))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(5), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x13)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(4), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(10), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(5), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(6))?;
    store(c, r(0), r(4), 0x0)?;
    store(c, r(8), r(1), 0xc)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(9))?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(7))?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    imm(c, Op::Sub, R::SP, R::R11, 0x1c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    c.data(0x109c4, 0xe3dfc)?;
    c.data(0x109c8, 0xe2788)?;
    c.check(0x109cc)
}

fn context_entry(c: &mut Code) -> Result<(), String> {
    c.check(0x109cc)?;
    c.emit(push_registers(C::Al, saved(false))?)?;
    imm(c, Op::Add, R::R11, R::SP, 0x1c)?;
    imm(c, Op::Sub, R::SP, R::SP, 0x4)?;
    mov(c, r(4), r(1))?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(0), r(4), 0x1c)?;
    load(c, r(0), r(4), 0xa58)?;
    load(c, r(0), r(0), 0x330)?;
    imm(c, Op::Cmp, R::R0, r(0), 0x1)?;
    c.jump(C::Ne, 0x10b30, false)?;
    c.check(0x109f4)
}

fn context_extended_first(c: &mut Code) -> Result<(), String> {
    c.check(0x109f4)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    mov(c, r(6), r(4))?;
    load(c, r(2), r(4), 0x414)?;
    load(c, r(5), r(4), 0x3e4)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    load(c, R::LR, r(4), 0x40c)?;
    imm(c, Op::Mov, r(12), R::R0, 0x22c)?;
    load(c, r(9), r(4), 0x418)?;
    load(c, r(8), r(4), 0x410)?;
    mov(c, r(7), r(4))?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(12), r(12), 0x1c00)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    context_pointer(c, r(0), r(6), r(0))?;
    add(c, r(2), r(5), r(2))?;
    add(c, r(2), r(2), R::LR)?;
    store(c, r(1), r(0), 0x0)?;
    add(c, r(10), r(2), r(9))?;
    imm(c, Op::Mov, r(2), R::R0, 0x28c)?;
    context_pointer(c, r(1), r(7), r(12))?;
    imm(c, Op::Mov, r(12), R::R0, 0x12)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(12), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    store(c, r(10), r(1), 0x4)?;
    store(c, r(2), r(0), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(12), r(1), 0x8)?;
    store(c, r(2), r(6), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(8))?;
    store(c, r(0), r(7), 0x0)?;
    imm(c, Op::Add, r(0), r(8), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(5), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(5), r(1), 0xc)?;
    c.literal(r(1), 0x10edc)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(9), 0x10ee0)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(9))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(6), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x12)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(7), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(10), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(6), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(8))?;
    store(c, r(0), r(7), 0x0)?;
    store(c, r(5), r(1), 0xc)?;
    c.literal(r(1), 0x10edc)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(9))?;
    c.jump(C::Al, 0x10c58, false)?;
    c.check(0x10b30)
}

fn context_simple_first(c: &mut Code) -> Result<(), String> {
    c.check(0x10b30)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    mov(c, r(6), r(4))?;
    imm(c, Op::Mov, r(3), R::R0, 0x20c)?;
    load(c, r(1), r(4), 0x3e4)?;
    load(c, r(2), r(4), 0x414)?;
    load(c, r(9), r(4), 0x40c)?;
    mov(c, r(7), r(4))?;
    imm(c, Op::Mov, r(5), R::R0, 0x208)?;
    imm(c, Op::Mov, r(12), R::R0, 0x12)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    imm(c, Op::Orr, r(5), r(5), 0x9400)?;
    context_pointer(c, r(0), r(6), r(0))?;
    add(c, r(10), r(1), r(2))?;
    imm(c, Op::Mov, r(1), R::R0, 0x28c)?;
    mov(c, r(2), r(9))?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(3), r(0), 0x0)?;
    imm(c, Op::Mov, r(3), R::R0, 0x22c)?;
    imm(c, Op::Orr, r(3), r(3), 0x1c00)?;
    context_pointer(c, r(3), r(7), r(3))?;
    store(c, r(12), r(3), 0x0)?;
    store(c, r(5), r(0), 0x4)?;
    store(c, r(10), r(3), 0x4)?;
    store(c, r(1), r(0), 0x8)?;
    imm(c, Op::Add, r(1), r(0), 0x10)?;
    store(c, r(12), r(3), 0x8)?;
    store(c, r(1), r(6), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x288)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(1), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(3), 0x10)?;
    c.literal(r(1), 0x10edc)?;
    store(c, r(0), r(7), 0x0)?;
    imm(c, Op::Add, r(0), r(9), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(8), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(8), r(3), 0xc)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(5), 0x10ee0)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(5))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(6), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x12)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(7), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(10), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(6), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(9))?;
    store(c, r(0), r(7), 0x0)?;
    store(c, r(8), r(1), 0xc)?;
    c.literal(r(1), 0x10edc)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(5))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.check(0x10c5c)
}

fn context_second_dispatch(c: &mut Code) -> Result<(), String> {
    c.check(0x10c5c)?;
    load(c, r(0), r(4), 0x404)?;
    imm(c, Op::Cmp, R::R0, r(0), 0x1)?;
    c.jump(C::Ne, 0x10ed0, false)?;
    load(c, r(0), r(4), 0xa58)?;
    load(c, r(0), r(0), 0x330)?;
    imm(c, Op::Cmp, R::R0, r(0), 0x1)?;
    c.jump(C::Ne, 0x10dac, false)?;
    c.check(0x10c78)
}

fn context_extended_second(c: &mut Code) -> Result<(), String> {
    c.check(0x10c78)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    mov(c, r(6), r(4))?;
    imm(c, Op::Mov, r(7), R::R0, 0x20c)?;
    load(c, r(1), r(4), 0x3e4)?;
    load(c, r(3), r(4), 0x414)?;
    load(c, r(2), r(4), 0x40c)?;
    load(c, r(8), r(4), 0x418)?;
    imm(c, Op::Mov, r(5), R::R0, 0x208)?;
    imm(c, Op::Mov, r(12), R::R0, 0x13)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    imm(c, Op::Orr, r(7), r(7), 0x9400)?;
    imm(c, Op::Orr, r(5), r(5), 0x9400)?;
    context_pointer(c, r(0), r(6), r(0))?;
    add(c, r(1), r(1), r(3))?;
    add(c, r(9), r(1), r(2))?;
    imm(c, Op::Mov, r(1), R::R0, 0x28c)?;
    mov(c, r(2), r(8))?;
    store(c, r(7), r(0), 0x0)?;
    imm(c, Op::Mov, r(7), R::R0, 0x22c)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(7), r(7), 0x1c00)?;
    context_pointer(c, r(7), r(4), r(7))?;
    store(c, r(12), r(7), 0x0)?;
    store(c, r(5), r(0), 0x4)?;
    store(c, r(9), r(7), 0x4)?;
    store(c, r(1), r(0), 0x8)?;
    imm(c, Op::Add, r(1), r(0), 0x10)?;
    store(c, r(12), r(7), 0x8)?;
    c.literal(r(5), 0x10edc)?;
    store(c, r(1), r(6), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x288)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    store(c, r(1), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(7), 0x10)?;
    store(c, r(0), r(4), 0x0)?;
    imm(c, Op::Add, r(0), r(8), 0xff)?;
    mov(c, r(1), r(5))?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(10), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(10), r(7), 0xc)?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(7), 0x10ee0)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(7))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(6), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x13)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(4), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(9), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(6), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(8))?;
    store(c, r(0), r(4), 0x0)?;
    store(c, r(10), r(1), 0xc)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(5))?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(7))?;
    c.jump(C::Al, 0x10ecc, false)?;
    c.check(0x10dac)
}

fn context_simple_second(c: &mut Code) -> Result<(), String> {
    c.check(0x10dac)?;
    imm(c, Op::Mov, r(0), R::R0, 0x228)?;
    mov(c, r(6), r(4))?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    load(c, r(7), r(4), 0x3e4)?;
    load(c, r(5), r(4), 0x414)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Mov, r(2), R::R0, 0x13)?;
    c.literal(r(8), 0x10edc)?;
    imm(c, Op::Orr, r(0), r(0), 0x1c00)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    context_pointer(c, r(0), r(6), r(0))?;
    store(c, r(1), r(0), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x22c)?;
    imm(c, Op::Orr, r(1), r(1), 0x1c00)?;
    context_pointer(c, r(1), r(4), r(1))?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(7), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(6), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(5))?;
    store(c, r(0), r(4), 0x0)?;
    imm(c, Op::Add, r(0), r(5), 0xff)?;
    imm(c, Op::Bic, r(0), r(0), 0xff)?;
    imm(c, Op::Orr, r(10), r(0), 0x1)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    store(c, r(10), r(1), 0xc)?;
    mov(c, r(1), r(8))?;
    c.jump(C::Al, 0x2ec94, true)?;
    c.literal(r(9), 0x10ee0)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(9))?;
    c.jump(C::Al, 0x2ec94, true)?;
    load(c, r(0), r(6), 0x0)?;
    imm(c, Op::Mov, r(1), R::R0, 0x20c)?;
    imm(c, Op::Mov, r(2), R::R0, 0x13)?;
    imm(c, Op::Mov, r(3), R::R0, 0x208)?;
    imm(c, Op::Orr, r(1), r(1), 0x9400)?;
    imm(c, Op::Orr, r(2), r(2), 0x1000)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(1), r(0), 0x0)?;
    load(c, r(1), r(4), 0x0)?;
    store(c, r(2), r(1), 0x0)?;
    store(c, r(3), r(0), 0x4)?;
    imm(c, Op::Mov, r(3), R::R0, 0x28c)?;
    store(c, r(7), r(1), 0x4)?;
    imm(c, Op::Orr, r(3), r(3), 0x9400)?;
    store(c, r(3), r(0), 0x8)?;
    store(c, r(2), r(1), 0x8)?;
    imm(c, Op::Add, r(2), r(0), 0x10)?;
    store(c, r(2), r(6), 0x0)?;
    imm(c, Op::Mov, r(2), R::R0, 0x288)?;
    imm(c, Op::Orr, r(2), r(2), 0x9400)?;
    store(c, r(2), r(0), 0xc)?;
    imm(c, Op::Add, r(0), r(1), 0x10)?;
    mov(c, r(2), r(5))?;
    store(c, r(0), r(4), 0x0)?;
    store(c, r(10), r(1), 0xc)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(8))?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    mov(c, r(1), r(9))?;
    c.jump(C::Al, 0x2ec94, true)?;
    imm(c, Op::Mov, r(0), R::R0, 0x1)?;
    imm(c, Op::Sub, R::SP, R::R11, 0x1c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    c.data(0x10edc, 0xe3dfc)?;
    c.data(0x10ee0, 0xe2788)?;
    c.check(0x10ee4)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    argument_entry(&mut c)?;
    argument_extended_first(&mut c)?;
    argument_simple_first(&mut c)?;
    argument_second_dispatch(&mut c)?;
    argument_extended_second(&mut c)?;
    argument_simple_second(&mut c)?;
    context_entry(&mut c)?;
    context_extended_first(&mut c)?;
    context_simple_first(&mut c)?;
    context_second_dispatch(&mut c)?;
    context_extended_second(&mut c)?;
    context_simple_second(&mut c)?;
    c.check(END)?;
    if c.bytes.len() != END - START {
        return Err("descriptor-paired length mismatch".into());
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
    fn structure() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 2572);
        for (at, value) in LITERALS { assert_eq!(word(&bytes, at), value); }
        for at in [START, 0x109cc] {
            assert_eq!(word(&bytes, at), push_registers(C::Al, saved(false)).unwrap());
        }
        for at in [0x109c0, 0x10ed8] {
            assert_eq!(word(&bytes, at), transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true).unwrap());
        }
        for (at, dst, base, offset) in [
            (0x10530, r(0), r(7), r(0)),
            (0x10550, r(6), r(5), r(12)),
            (0x10644, r(0), r(5), r(0)),
            (0x10668, r(2), r(7), r(2)),
            (0x10794, r(0), r(5), r(0)),
            (0x107a4, r(2), r(4), r(2)),
            (0x108bc, r(0), r(5), r(0)),
            (0x108cc, r(1), r(4), r(1)),
            (0x10a30, r(0), r(6), r(0)),
            (0x10a48, r(1), r(7), r(12)),
            (0x10b60, r(0), r(6), r(0)),
            (0x10b80, r(3), r(7), r(3)),
            (0x10ca8, r(0), r(6), r(0)),
            (0x10ccc, r(7), r(4), r(7)),
            (0x10dd8, r(0), r(6), r(0)),
            (0x10de8, r(1), r(4), r(1)),
        ] {
            assert_eq!(word(&bytes, at), transfer_word_register_preindexed_writeback(C::Al, true, dst, base, offset, Shift::Lsl, 0).unwrap());
        }
        for (at, target, dst) in [
            (0x10590, 0x109c4, r(1)),
            (0x105b0, 0x109c8, r(6)),
            (0x10698, 0x109c4, r(1)),
            (0x106bc, 0x109c8, r(6)),
            (0x10730, 0x109c4, r(1)),
            (0x10784, 0x109c4, r(8)),
            (0x1080c, 0x109c8, r(6)),
            (0x108ac, 0x109c4, r(9)),
            (0x1092c, 0x109c8, r(7)),
            (0x10a9c, 0x10edc, r(1)),
            (0x10aa4, 0x10ee0, r(9)),
            (0x10b18, 0x10edc, r(1)),
            (0x10bb0, 0x10edc, r(1)),
            (0x10bd0, 0x10ee0, r(5)),
            (0x10c44, 0x10edc, r(1)),
            (0x10ce8, 0x10edc, r(5)),
            (0x10d20, 0x10ee0, r(7)),
            (0x10dc8, 0x10edc, r(8)),
            (0x10e44, 0x10ee0, r(9)),
        ] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, dst, R::PC, target as i32 - at as i32 - 8).unwrap());
        }
        for (at, target, condition, link) in [
            (0x10508, 0x10628, C::Ne, false),
            (0x105ac, 0x2ec94, C::Al, true),
            (0x105bc, 0x2ec94, C::Al, true),
            (0x10624, 0x10730, C::Al, false),
            (0x106b8, 0x2ec94, C::Al, true),
            (0x106c8, 0x2ec94, C::Al, true),
            (0x10738, 0x2ec94, C::Al, true),
            (0x10744, 0x2ec94, C::Al, true),
            (0x10750, 0x109b8, C::Ne, false),
            (0x10764, 0x10898, C::Ne, false),
            (0x10808, 0x2ec94, C::Al, true),
            (0x10818, 0x2ec94, C::Al, true),
            (0x10888, 0x2ec94, C::Al, true),
            (0x10894, 0x109b4, C::Al, false),
            (0x10928, 0x2ec94, C::Al, true),
            (0x10938, 0x2ec94, C::Al, true),
            (0x109a8, 0x2ec94, C::Al, true),
            (0x109b4, 0x2ec94, C::Al, true),
            (0x109f0, 0x10b30, C::Ne, false),
            (0x10aa0, 0x2ec94, C::Al, true),
            (0x10ab0, 0x2ec94, C::Al, true),
            (0x10b20, 0x2ec94, C::Al, true),
            (0x10b2c, 0x10c58, C::Al, false),
            (0x10bcc, 0x2ec94, C::Al, true),
            (0x10bdc, 0x2ec94, C::Al, true),
            (0x10c4c, 0x2ec94, C::Al, true),
            (0x10c58, 0x2ec94, C::Al, true),
            (0x10c64, 0x10ed0, C::Ne, false),
            (0x10c74, 0x10dac, C::Ne, false),
            (0x10d1c, 0x2ec94, C::Al, true),
            (0x10d2c, 0x2ec94, C::Al, true),
            (0x10d9c, 0x2ec94, C::Al, true),
            (0x10da8, 0x10ecc, C::Al, false),
            (0x10e40, 0x2ec94, C::Al, true),
            (0x10e50, 0x2ec94, C::Al, true),
            (0x10ec0, 0x2ec94, C::Al, true),
            (0x10ecc, 0x2ec94, C::Al, true),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target, link).unwrap());
        }
        for at in (START..END).step_by(4).filter(|&at| Code::code(at)) {
            let instruction = word(&bytes, at);
            if instruction & 0x0c00_0000 == 0 {
                let op = (instruction >> 21) & 15;
                assert_eq!(instruction & (1 << 20) != 0, op == Op::Cmp as u32);
            }
        }
    }

    #[test]
    fn exact_guards() {
        for (at, value) in LITERALS {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.data(at, value ^ 1).is_err());
            assert!(c.data(at + 4, value).is_err());
            assert!(c.bytes.is_empty());
            c.data(at, value).unwrap();
            assert_eq!(c.at, at + 4);
        }
        for at in [START - 4, START + 1, 0x109c5, END, END + 4] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.data(at, 0xe3dfc).is_err());
            assert!(c.bytes.is_empty());
        }
        for (at, pool, other_pool) in [(START, 0x109c4, 0x10edc), (0x109cc, 0x10edc, 0x109c4)] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.data(at, 0xe3dfc).is_err());
            assert!(c.literal(r(1), other_pool).is_err());
            assert!(c.literal(r(1), pool + 8).is_err());
            for target in [pool, other_pool, END, at + 1, 0x2ec94] {
                assert!(c.jump(C::Al, target, false).is_err());
            }
            assert!(c.jump(C::Al, at, true).is_err());
            assert!(c.jump(C::Ne, 0x2ec94, true).is_err());
            assert!(c.bytes.is_empty());
            c.literal(r(1), pool).unwrap();
        }
    }

    #[test]
    fn authenticated_original_byte_equality() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let original = std::fs::read(path).unwrap();
        assert_eq!(original.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&original),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), original[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
