use arm32::{
    branch, data_immediate, data_immediate_with_flags, data_register, mov_register,
    mov_shifted_register, multiply_accumulate, multiply_register, transfer_byte,
    transfer_byte_register, transfer_doubleword, transfer_word, transfer_word_register,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x68920;
pub const END: usize = 0x68d44;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const DIAGNOSTIC_POINTER_ADDRESS: usize = 0x68a6c;
pub const DIAGNOSTIC_POINTER: u32 = 0x000f2e4c;

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
            return Err(format!("type selection address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, word: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err(format!("type selection output address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if self.at == DIAGNOSTIC_POINTER_ADDRESS {
            return Err("diagnostic pointer is data, not an instruction".into());
        }
        self.word(instruction)
    }

    fn diagnostic_pointer_data(&mut self) -> Result<(), String> {
        self.check(DIAGNOSTIC_POINTER_ADDRESS)?;
        self.word(DIAGNOSTIC_POINTER)
    }

    fn branch(&mut self, condition: C, target: usize) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, false)?)
    }

    fn call(&mut self, target: usize) -> Result<(), String> {
        self.emit(branch(C::Al, self.at as u32, target as u32, true)?)
    }

    fn word_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn byte_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn immediate(&mut self, condition: C, op: Op, rd: u8, rn: u8, value: u32) -> Result<(), String> {
        self.emit(data_immediate(condition, op, r(rd), r(rn), value)?)
    }

    fn register(&mut self, op: Op, rd: u8, rn: u8, rm: u8) -> Result<(), String> {
        self.emit(data_register(C::Al, op, r(rd), r(rn), r(rm), Shift::Lsl, 0, false)?)
    }

    fn mov(&mut self, condition: C, rd: u8, rm: u8) -> Result<(), String> {
        self.emit(mov_register(condition, r(rd), r(rm)))
    }

    fn literal(&mut self, rd: u8, target: usize) -> Result<(), String> {
        self.word_transfer(true, rd, 15, target as i32 - self.at as i32 - 8)
    }
}

fn rate_quality_arm(c: &mut Code, start: usize, search: usize, zero_rate: usize, log: usize) -> Result<(), String> {
    c.check(start)?;
    c.word_transfer(true, 4, 13, 60)?;
    c.mov(C::Al, 0, 4)?;
    c.call(0x728cc)?;
    c.mov(C::Al, 5, 0)?;
    c.byte_transfer(false, 5, 4, 39)?;
    c.byte_transfer(true, 0, 4, 137)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, search)?;
    c.word_transfer(true, 1, 4, 0x634)?;
    c.word_transfer(true, 2, 4, 0x94)?;
    c.word_transfer(true, 0, 4, 0x98)?;
    c.immediate(C::Al, Op::Mov, 6, 0, 0x200)?;
    c.word_transfer(true, 3, 4, 0x64)?;
    c.emit(multiply_register(C::Al, r(7), r(2), r(1))?)?;
    c.word_transfer(true, 1, 4, 0x640)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(6), r(1), Shift::Lsl, 4, false)?)?;
    c.register(Op::Sub, 1, 1, 7)?;
    c.immediate(C::Al, Op::Mov, 7, 0, 255)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(7), r(1), Shift::Lsr, 10, false)?)?;
    c.register(Op::Cmp, 0, 1, 0)?;
    c.mov(C::Cc, 0, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 3, 0)?;
    c.branch(C::Eq, zero_rate)?;
    c.mov(C::Al, 5, 0)?;
    c.branch(C::Al, log)
}

fn zero_rate_quality(c: &mut Code, start: usize) -> Result<(), String> {
    c.check(start)?;
    c.word_transfer(true, 1, 4, 104)?;
    c.register(Op::Cmp, 0, 0, 5)?;
    c.mov(C::Hi, 5, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 1, 0)?;
    c.mov(C::Ne, 5, 0)
}

fn log_separator(c: &mut Code, separator: u8) -> Result<(), String> {
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.mov(C::Al, 1, separator)?;
    c.call(0x2ec94)
}

fn log_quality_tail(c: &mut Code, separator: u8, old_quality_format: usize) -> Result<(), String> {
    c.immediate(C::Al, Op::And, 2, 5, 255)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.call(0x2ec94)?;
    log_separator(c, separator)?;
    c.byte_transfer(true, 2, 4, 39)?;
    c.literal(1, old_quality_format)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.call(0x2ec94)?;
    log_separator(c, separator)?;
    c.byte_transfer(false, 5, 4, 39)
}

fn budget_minus_one(c: &mut Code) -> Result<(), String> {
    c.emit(transfer_doubleword(C::Al, true, r(2), r(10), 0)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sub, r(1), r(2), 1, true)?)?;
    c.immediate(C::Al, Op::Sbc, 2, 3, 0)
}

fn cap_budget(c: &mut Code) -> Result<(), String> {
    c.emit(multiply_register(C::Al, r(0), r(3), r(7))?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(1), r(0), Shift::Lsl, 0, true)?)?;
    c.emit(data_immediate_with_flags(C::Al, Op::Sbc, r(1), r(2), 0, true)?)?;
    c.immediate(C::Cs, Op::Mov, 1, 0, 0)?;
    c.emit(transfer_doubleword(C::Cs, false, r(0), r(10), 0)?)
}

fn table_search(c: &mut Code, start: usize, last: u32, quality_table: usize, rate_table: usize, threshold_table: usize, selected_quality_table: usize, budget_rate_table: usize) -> Result<(), String> {
    c.check(start)?;
    c.word_transfer(true, 0, 4, 96)?;
    c.literal(2, quality_table)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 2)?)?;
    c.word_transfer(false, 0, 4, 76)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.word_transfer(true, 1, 4, 8)?;
    c.byte_transfer(true, 3, 2, 0)?;
    c.register(Op::Cmp, 0, 1, 3)?;
    c.branch(C::Eq, start + 0x44)?;
    c.immediate(C::Al, Op::Add, 3, 0, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, last)?;
    c.immediate(C::Al, Op::Add, 2, 2, 1)?;
    c.mov(C::Al, 0, 3)?;
    c.branch(C::Lt, start + 0x18)?;
    c.word_transfer(true, 0, 13, 60)?;
    c.word_transfer(true, 1, 0, 72)?;
    c.branch(C::Al, start + 0x64)?;
    c.literal(1, rate_table)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 1000)?;
    c.emit(transfer_word_register(C::Al, true, r(1), r(1), r(0), Shift::Lsl, 2)?)?;
    c.emit(multiply_register(C::Al, r(3), r(1), r(2))?)?;
    c.word_transfer(true, 1, 13, 60)?;
    c.word_transfer(true, 1, 1, 72)?;
    c.register(Op::Cmp, 0, 1, 3)?;
    c.branch(C::Ls, start + 0xb0)?;
    c.literal(3, threshold_table)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.word_transfer(true, 0, 3, 0)?;
    c.emit(multiply_accumulate(C::Al, r(7), r(0), r(8), r(1))?)?;
    c.immediate(C::Al, Op::Cmp, 0, 7, 0)?;
    c.branch(C::Lt, start + 0x9c)?;
    c.immediate(C::Al, Op::Add, 0, 2, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, last)?;
    c.immediate(C::Al, Op::Add, 3, 3, 4)?;
    c.mov(C::Al, 2, 0)?;
    c.branch(C::Lt, start + 0x6c)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, last + 1)?;
    c.immediate(C::Eq, Op::Sub, 0, 0, 1)?;
    c.branch(C::Al, start + 0xa0)?;
    c.mov(C::Al, 0, 2)?;
    c.literal(1, selected_quality_table)?;
    c.word_transfer(true, 2, 13, 60)?;
    c.emit(transfer_byte_register(C::Al, true, r(1), r(1), r(0), Shift::Lsl, 0)?)?;
    c.word_transfer(false, 1, 2, 8)?;
    budget_minus_one(c)?;
    c.literal(3, budget_rate_table)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    rate_quality_arm(&mut c, 0x68920, 0x68b2c, 0x68ac0, 0x68ad4)?;
    rate_quality_arm(&mut c, 0x68984, 0x68c68, 0x68bf0, 0x68c04)?;
    c.check(0x689e8)?;
    c.word_transfer(true, 4, 13, 60)?;
    c.byte_transfer(true, 0, 4, 38)?;
    c.immediate(C::Al, Op::Tst, 0, 0, 1)?;
    c.branch(C::Ne, 0x68a08)?;
    c.mov(C::Al, 0, 4)?;
    c.call(0x728cc)?;
    c.byte_transfer(false, 0, 4, 39)?;
    c.branch(C::Al, 0x68a24)?;
    c.byte_transfer(true, 0, 4, 39)?;
    c.word_transfer(true, 1, 13, 32)?;
    c.register(Op::Cmp, 0, 0, 1)?;
    c.branch(C::Cc, 0x689f8)?;
    c.word_transfer(true, 1, 13, 24)?;
    c.register(Op::Cmp, 0, 0, 1)?;
    c.branch(C::Hi, 0x689f8)?;
    c.word_transfer(true, 0, 4, 96)?;
    c.literal(3, 0x69a10)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 2)?)?;
    c.word_transfer(false, 0, 4, 76)?;
    c.word_transfer(true, 1, 4, 72)?;
    c.word_transfer(true, 0, 3, 0)?;
    c.emit(multiply_accumulate(C::Al, r(7), r(0), r(8), r(1))?)?;
    c.immediate(C::Al, Op::Cmp, 0, 7, 0)?;
    c.branch(C::Lt, 0x68a70)?;
    c.immediate(C::Al, Op::Add, 0, 2, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, 15)?;
    c.immediate(C::Al, Op::Add, 3, 3, 4)?;
    c.mov(C::Al, 2, 0)?;
    c.branch(C::Lt, 0x68a3c)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 16)?;
    c.immediate(C::Eq, Op::Sub, 0, 0, 1)?;
    c.branch(C::Al, 0x68a74)?;
    c.diagnostic_pointer_data()?;
    c.mov(C::Al, 0, 2)?;
    c.literal(1, 0x69a14)?;
    c.word_transfer(true, 4, 13, 60)?;
    c.immediate(C::Al, Op::Mov, 7, 0, 1000)?;
    c.emit(transfer_byte_register(C::Al, true, r(1), r(1), r(0), Shift::Lsl, 0)?)?;
    c.word_transfer(false, 1, 4, 8)?;
    budget_minus_one(&mut c)?;
    c.literal(3, 0x69a18)?;
    c.emit(transfer_word_register(C::Al, true, r(3), r(3), r(0), Shift::Lsl, 2)?)?;
    cap_budget(&mut c)?;
    c.mov(C::Al, 0, 4)?;
    c.call(0x710c0)?;
    c.byte_transfer(false, 0, 4, 39)?;
    c.branch(C::Al, END)?;
    zero_rate_quality(&mut c, 0x68ac0)?;
    c.literal(1, 0x69284)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.call(0x2ec94)?;
    c.literal(6, 0x69288)?;
    log_separator(&mut c, 6)?;
    c.literal(1, 0x69a1c)?;
    log_quality_tail(&mut c, 6, 0x69b0c)?;
    table_search(&mut c, 0x68b2c, 16, 0x69a14, 0x69b54, 0x69b54, 0x69b58, 0x69b5c)?;
    c.branch(C::Al, 0x68d28)?;
    zero_rate_quality(&mut c, 0x68bf0)?;
    c.literal(1, 0x69b4c)?;
    c.byte_transfer(true, 6, 4, 126)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.call(0x2ec94)?;
    c.literal(7, 0x69b50)?;
    log_separator(&mut c, 7)?;
    c.literal(1, 0x69b60)?;
    c.immediate(C::Al, Op::Cmp, 0, 6, 0)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Ne, Op::Add, 5, 5, 12)?;
    c.immediate(C::Al, Op::And, 2, 5, 255)?;
    c.call(0x2ec94)?;
    log_separator(&mut c, 7)?;
    c.byte_transfer(true, 2, 4, 39)?;
    c.literal(1, 0x69b64)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.call(0x2ec94)?;
    log_separator(&mut c, 7)?;
    c.byte_transfer(false, 5, 4, 39)?;
    table_search(&mut c, 0x68c68, 12, 0x69b68, 0x69b6c, 0x69b6c, 0x69b68, 0x69b70)?;
    c.check(0x68d28)?;
    c.emit(transfer_word_register(C::Al, true, r(3), r(3), r(0), Shift::Lsl, 2)?)?;
    c.immediate(C::Al, Op::Mov, 7, 0, 1000)?;
    cap_budget(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], address: usize) -> u32 {
        let offset = address - START;
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        let identity = crate::firmware_manifest_identity::canonical_image("venus.mbn").unwrap();
        assert_eq!(reference.len(), identity.size);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference), identity.sha256);
        assert_eq!(fragment().unwrap(), reference[START + 0x4000..END + 0x4000]);
    }

    #[test]
    fn exact_bounds_and_diagnostic_data() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 1060);
        assert_eq!(END - START, 1060);
        assert_eq!(word(&bytes, DIAGNOSTIC_POINTER_ADDRESS), DIAGNOSTIC_POINTER);
        assert_eq!(DIAGNOSTIC_POINTER_ADDRESS + 4, 0x68a70);
    }

    #[test]
    fn data_slot_rejects_instruction_emission() {
        let mut c = Code { at: DIAGNOSTIC_POINTER_ADDRESS, bytes: Vec::new() };
        assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
        c.diagnostic_pointer_data().unwrap();
        assert_eq!(c.at, 0x68a70);
        assert_eq!(c.bytes, DIAGNOSTIC_POINTER.to_le_bytes());
        c.at = END;
        assert!(c.word(0).is_err());
    }

    #[test]
    fn branch_edges_skip_data_and_converge() {
        let bytes = fragment().unwrap();
        for (at, condition, target) in [
            (0x6893c, C::Ne, 0x68b2c),
            (0x689a0, C::Ne, 0x68c68),
            (0x68a48, C::Lt, 0x68a70),
            (0x68a68, C::Al, 0x68a74),
            (0x68abc, C::Al, END),
            (0x68bec, C::Al, 0x68d28),
        ] {
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, false).unwrap());
        }
        for at in [0x68aac, 0x68d40] {
            assert_eq!(word(&bytes, at), transfer_doubleword(C::Cs, false, r(0), r(10), 0).unwrap());
        }
    }
}
