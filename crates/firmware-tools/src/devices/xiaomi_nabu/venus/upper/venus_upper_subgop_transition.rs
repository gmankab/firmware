use arm32::{
    branch, branch_exchange_immediate, data_immediate, data_register, mov_register,
    mov_shifted_register, multiply_accumulate, multiply_long, multiply_register, transfer_multiple,
    transfer_word, BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x6f61c;
pub const CODE_END: usize = 0x6f728;
pub const END: usize = 0x6f738;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = CODE_END - START;
pub const DATA_BYTES: usize = END - CODE_END;
const SWITCH_FORMAT: u32 = 0xf3087;
const NEWLINE: u32 = 0xe2788;
const NUM_P_NUM_B_FORMAT: u32 = 0xf30c5;
const SOURCE_PATH: u32 = 0xf2e4c;
const LOG: u32 = 0x2ec94;
const DIVIDE: u32 = 0x83c4c;
const ASSERT: u32 = 0x17b0;
const WIDE_DIVIDE: u32 = 0x8187c;
const ADAPTATION: i32 = 0x294;
const TRANSITION_ACTIVE: i32 = 0x298;
const NUM_P: i32 = 0x64;
const NUM_B: i32 = 0x68;
const SAVE_REGISTERS: u16 = (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 11);

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn emit(&mut self, at: usize, instruction: u32) -> Result<(), String> {
        if self.at != at || !(START..CODE_END).contains(&at) || at % 4 != 0
            || self.bytes.len() != at - START
        {
            return Err(format!("subGOP instruction address {at:#x}, cursor {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, at: usize, value: u32) -> Result<(), String> {
        let expected = match at {
            0x6f728 => SWITCH_FORMAT,
            0x6f72c => NEWLINE,
            0x6f730 => NUM_P_NUM_B_FORMAT,
            0x6f734 => SOURCE_PATH,
            _ => return Err(format!("invalid subGOP literal address {at:#x}")),
        };
        if self.at != at || self.bytes.len() != at - START || value != expected {
            return Err(format!("subGOP literal boundary or value mismatch at {at:#x}"));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn r(number: u8) -> R {
    R::new(number).expect("valid subGOP register")
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.emit(0x6f61c, transfer_multiple(C::Al, false, R::SP, SAVE_REGISTERS | R::LR.bit(), BlockMode::DecrementBefore, true)?)?;
    c.emit(0x6f620, data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    c.emit(0x6f624, mov_register(C::Al, r(5), R::R2))?;
    c.emit(0x6f628, mov_register(C::Al, r(6), R::R1))?;
    c.emit(0x6f62c, mov_register(C::Al, r(4), R::R0))?;
    c.emit(0x6f630, data_register(C::Al, Op::Cmp, R::R0, r(5), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6f634, branch(C::Cs, 0x6f634, 0x6f65c, false)?)?;
    c.emit(0x6f638, transfer_word(C::Al, true, R::R1, R::PC, 0xe8)?)?;
    c.emit(0x6f63c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 4)?)?;
    c.emit(0x6f640, mov_register(C::Al, R::R2, r(5)))?;
    c.emit(0x6f644, branch(C::Al, 0x6f644, LOG, true)?)?;
    c.emit(0x6f648, transfer_word(C::Al, true, R::R1, R::PC, 0xdc)?)?;
    c.emit(0x6f64c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 4)?)?;
    c.emit(0x6f650, branch(C::Al, 0x6f650, LOG, true)?)?;
    c.emit(0x6f654, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 3)?)?;
    c.emit(0x6f658, branch(C::Al, 0x6f658, 0x6f664, false)?)?;
    c.emit(0x6f65c, transfer_word(C::Al, true, R::R0, r(4), ADAPTATION)?)?;
    c.emit(0x6f660, mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 1)?)?;
    c.emit(0x6f664, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 1)?)?;
    c.emit(0x6f668, data_immediate(C::Al, Op::Sub, r(6), r(6), 1)?)?;
    c.emit(0x6f66c, transfer_word(C::Al, false, R::R0, r(4), ADAPTATION)?)?;
    c.emit(0x6f670, transfer_word(C::Al, false, R::R1, r(4), TRANSITION_ACTIVE)?)?;
    c.emit(0x6f674, mov_register(C::Al, R::R0, r(6)))?;
    c.emit(0x6f678, mov_register(C::Al, R::R1, r(5)))?;
    c.emit(0x6f67c, branch_exchange_immediate(0x6f67c, DIVIDE)?)?;
    c.emit(0x6f680, data_immediate(C::Al, Op::Sub, R::R1, r(5), 1)?)?;
    c.emit(0x6f684, multiply_register(C::Al, r(3), R::R0, R::R1)?)?;
    c.emit(0x6f688, transfer_word(C::Al, true, R::R1, R::PC, 0xa0)?)?;
    c.emit(0x6f68c, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 2)?)?;
    c.emit(0x6f690, data_register(C::Al, Op::Sub, R::R2, r(6), r(3), Shift::Lsl, 0, false)?)?;
    c.emit(0x6f694, transfer_word(C::Al, false, R::R2, r(4), NUM_P)?)?;
    c.emit(0x6f698, transfer_word(C::Al, false, r(3), r(4), NUM_B)?)?;
    c.emit(0x6f69c, branch(C::Al, 0x6f69c, LOG, true)?)?;
    c.emit(0x6f6a0, transfer_word(C::Al, true, R::R1, R::PC, 0x84)?)?;
    c.emit(0x6f6a4, data_immediate(C::Al, Op::Mov, R::R0, R::R0, 2)?)?;
    c.emit(0x6f6a8, branch(C::Al, 0x6f6a8, LOG, true)?)?;
    c.emit(0x6f6ac, transfer_word(C::Al, true, R::R0, r(4), 0x6c)?)?;
    c.emit(0x6f6b0, transfer_word(C::Al, true, R::R2, R::PC, 0x7c)?)?;
    c.emit(0x6f6b4, transfer_word(C::Al, true, r(6), r(4), 0x2a0)?)?;
    c.emit(0x6f6b8, data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x66)?)?;
    c.emit(0x6f6bc, data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x2300)?)?;
    c.emit(0x6f6c0, data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0)?)?;
    c.emit(0x6f6c4, data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 1)?)?;
    c.emit(0x6f6c8, branch(C::Al, 0x6f6c8, ASSERT, true)?)?;
    c.emit(0x6f6cc, transfer_word(C::Al, true, R::R0, r(4), NUM_P)?)?;
    c.emit(0x6f6d0, transfer_word(C::Al, true, r(7), r(4), 0x5c)?)?;
    c.emit(0x6f6d4, transfer_word(C::Al, true, r(5), r(4), 0x6c)?)?;
    c.emit(0x6f6d8, multiply_long(C::Al, false, R::R1, R::R2, R::R0, r(6))?)?;
    c.emit(0x6f6dc, multiply_long(C::Al, false, R::R0, r(3), R::R1, r(7))?)?;
    c.emit(0x6f6e0, multiply_accumulate(C::Al, R::R1, R::R2, r(7), r(3))?)?;
    c.emit(0x6f6e4, mov_register(C::Al, R::R2, r(5)))?;
    c.emit(0x6f6e8, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0)?)?;
    c.emit(0x6f6ec, branch(C::Al, 0x6f6ec, WIDE_DIVIDE, true)?)?;
    c.emit(0x6f6f0, mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(0x6f6f4, data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(0x6f6f8, transfer_word(C::Al, false, R::R0, r(4), 0x2b8)?)?;
    c.emit(0x6f6fc, transfer_word(C::Al, true, R::R0, r(4), NUM_B)?)?;
    c.emit(0x6f700, multiply_long(C::Al, false, R::R1, R::R2, R::R0, r(6))?)?;
    c.emit(0x6f704, multiply_long(C::Al, false, R::R0, r(3), R::R1, r(7))?)?;
    c.emit(0x6f708, multiply_accumulate(C::Al, R::R1, R::R2, r(7), r(3))?)?;
    c.emit(0x6f70c, mov_register(C::Al, R::R2, r(5)))?;
    c.emit(0x6f710, data_immediate(C::Al, Op::Mov, r(3), R::R0, 0)?)?;
    c.emit(0x6f714, branch(C::Al, 0x6f714, WIDE_DIVIDE, true)?)?;
    c.emit(0x6f718, mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2)?)?;
    c.emit(0x6f71c, data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false)?)?;
    c.emit(0x6f720, transfer_word(C::Al, false, R::R0, r(4), 0x2bc)?)?;
    c.emit(0x6f724, transfer_multiple(C::Al, true, R::SP, SAVE_REGISTERS | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.literal(0x6f728, SWITCH_FORMAT)?;
    c.literal(0x6f72c, NEWLINE)?;
    c.literal(0x6f730, NUM_P_NUM_B_FORMAT)?;
    c.literal(0x6f734, SOURCE_PATH)?;
    if c.at != END || c.bytes.len() != END - START {
        return Err("subGOP final extent mismatch".into());
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
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(), reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }

    #[test]
    fn exact_extent_and_checked_boundaries() {
        let bytes = fragment().unwrap();
        assert_eq!((bytes.len(), CODE_BYTES, DATA_BYTES), (284, 268, 16));
        for at in [START - 4, START + 1, CODE_END, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(at, mov_register(C::Al, R::R0, R::R0)).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.emit(START + 4, mov_register(C::Al, R::R0, R::R0)).is_err());
        assert!(c.literal(CODE_END, SWITCH_FORMAT).is_err());
        let mut c = Code { at: CODE_END, bytes: Vec::new() };
        assert!(c.literal(CODE_END, SWITCH_FORMAT).is_err());
        let mut c = Code { at: CODE_END, bytes: bytes[..CODE_BYTES].to_vec() };
        assert!(c.literal(CODE_END, NEWLINE).is_err());
        assert!(c.literal(CODE_END + 4, NEWLINE).is_err());
        c.literal(CODE_END, SWITCH_FORMAT).unwrap();
        assert!(c.literal(CODE_END, SWITCH_FORMAT).is_err());
        c.literal(CODE_END + 4, NEWLINE).unwrap();
        c.literal(CODE_END + 8, NUM_P_NUM_B_FORMAT).unwrap();
        c.literal(CODE_END + 12, SOURCE_PATH).unwrap();
        assert!(c.literal(END, SOURCE_PATH).is_err());
        assert_eq!(c.bytes, bytes);
    }

    #[test]
    fn literal_targets_and_calls() {
        let bytes = fragment().unwrap();
        for (at, target, value) in [
            (0x6f638, 0x6f728, SWITCH_FORMAT),
            (0x6f648, 0x6f72c, NEWLINE),
            (0x6f688, 0x6f730, NUM_P_NUM_B_FORMAT),
            (0x6f6a0, 0x6f72c, NEWLINE),
            (0x6f6b0, 0x6f734, SOURCE_PATH),
        ] {
            let instruction = word(&bytes, at);
            assert_eq!(instruction & 0xffff_0000, 0xe59f_0000);
            assert_eq!(at + 8 + (instruction as usize & 0xfff), target);
            assert_eq!(word(&bytes, target), value);
        }
        for at in [0x6f644, 0x6f650, 0x6f69c, 0x6f6a8] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, LOG, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x6f67c), branch_exchange_immediate(0x6f67c, DIVIDE).unwrap());
        assert_eq!(word(&bytes, 0x6f6c8), branch(C::Al, 0x6f6c8, ASSERT, true).unwrap());
        for at in [0x6f6ec, 0x6f714] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, WIDE_DIVIDE, true).unwrap());
        }
    }

    #[test]
    fn adaptation_partition_and_wide_accounting() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x6f630), data_register(C::Al, Op::Cmp, R::R0, r(5), r(3), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x6f634), branch(C::Cs, 0x6f634, 0x6f65c, false).unwrap());
        assert_eq!(word(&bytes, 0x6f654), data_immediate(C::Al, Op::Mov, R::R0, R::R0, 3).unwrap());
        assert_eq!(word(&bytes, 0x6f660), mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 1).unwrap());
        assert_eq!(word(&bytes, 0x6f668), data_immediate(C::Al, Op::Sub, r(6), r(6), 1).unwrap());
        assert_eq!(word(&bytes, 0x6f670), transfer_word(C::Al, false, R::R1, r(4), TRANSITION_ACTIVE).unwrap());
        assert_eq!(word(&bytes, 0x6f684), multiply_register(C::Al, r(3), R::R0, R::R1).unwrap());
        assert_eq!(word(&bytes, 0x6f690), data_register(C::Al, Op::Sub, R::R2, r(6), r(3), Shift::Lsl, 0, false).unwrap());
        assert_eq!(word(&bytes, 0x6f694), transfer_word(C::Al, false, R::R2, r(4), NUM_P).unwrap());
        assert_eq!(word(&bytes, 0x6f698), transfer_word(C::Al, false, r(3), r(4), NUM_B).unwrap());
        for (at, output) in [(0x6f6d8, 0x2b8), (0x6f700, 0x2bc)] {
            assert_eq!(word(&bytes, at), multiply_long(C::Al, false, R::R1, R::R2, R::R0, r(6)).unwrap());
            assert_eq!(word(&bytes, at + 4), multiply_long(C::Al, false, R::R0, r(3), R::R1, r(7)).unwrap());
            assert_eq!(word(&bytes, at + 8), multiply_accumulate(C::Al, R::R1, R::R2, r(7), r(3)).unwrap());
            assert_eq!(word(&bytes, at + 24), mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 2).unwrap());
            assert_eq!(word(&bytes, at + 28), data_register(C::Al, Op::Orr, R::R0, R::R0, R::R1, Shift::Lsl, 30, false).unwrap());
            assert_eq!(word(&bytes, at + 32), transfer_word(C::Al, false, R::R0, r(4), output).unwrap());
        }
    }
}
