use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, load_signed_byte_register, mov_register,
    push_registers, transfer_byte, transfer_multiple, transfer_word,
};

pub const START: usize = 0x7a234;
pub const END: usize = 0x7a3ec;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = END - START;
pub const DATA_BYTES: usize = 0;

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("bitstream writer at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 {
            return Err("bitstream writer instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn jump(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = match (self.at, condition, target, link) {
            (0x7a35c, C::Eq, 0x7a36c, false)
            | (0x7a368, C::Eq, 0x7a398, false)
            | (0x7a3c8, C::Ne, 0x7a3d0, false)
            | (0x7a3e8, C::Al, 0x79b34, false) => true,
            (_, C::Al, _, true) => CALLS.contains(&(self.at, target)),
            _ => false,
        };
        if !valid {
            return Err("invalid bitstream writer branch".into());
        }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

const CALLS: &[(usize, usize)] = &[
    (0x7a264, 0x79b6c), (0x7a274, 0x798f8), (0x7a280, 0x79db8),
    (0x7a28c, 0x79db8), (0x7a29c, 0x798f8), (0x7a2ac, 0x798f8),
    (0x7a2b8, 0x79db8), (0x7a2c8, 0x79db8), (0x7a2d8, 0x79db8),
    (0x7a2e8, 0x798f8), (0x7a2f8, 0x798f8), (0x7a304, 0x79df0),
    (0x7a310, 0x79df0), (0x7a31c, 0x79df0), (0x7a32c, 0x798f8),
    (0x7a340, 0x798f8), (0x7a350, 0x798f8), (0x7a378, 0x798f8),
    (0x7a388, 0x798f8), (0x7a394, 0x79df0), (0x7a3a4, 0x798f8),
    (0x7a3b0, 0x79c18), (0x7a3b8, 0x79c98),
];

fn r(n: u8) -> R {
    R::new(n).unwrap()
}

fn saved(returning: bool) -> u16 {
    [4, 5, 6, 7, 8, 9, 11].into_iter().fold(
        if returning { R::PC.bit() } else { R::LR.bit() },
        |mask, n| mask | r(n).bit(),
    )
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(CODE_BYTES) };
    c.emit(push_registers(C::Al, saved(false))?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 0x18)?)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0x11c)?)?;
    c.emit(mov_register(C::Al, r(7), r(3)))?;
    c.emit(mov_register(C::Al, r(6), r(2)))?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(9), r(9), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 0x20)?)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(9))?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.jump(C::Al, 0x79b6c, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x68)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 8)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    for _ in 0..2 {
        c.emit(mov_register(C::Al, r(0), r(4)))?;
        c.emit(mov_register(C::Al, r(1), r(6)))?;
        c.jump(C::Al, 0x79db8, true)?;
    }
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0xa8)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.jump(C::Al, 0x79db8, true)?;
    for offset in [0x2d9, 0x2da] {
        c.emit(transfer_byte(C::Al, true, r(0), r(5), offset)?)?;
        c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 1)?)?;
        c.emit(mov_register(C::Al, r(0), r(4)))?;
        c.jump(C::Al, 0x79db8, true)?;
    }
    for width in [1, 2] {
        c.emit(mov_register(C::Al, r(0), r(4)))?;
        c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
        c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), width)?)?;
        c.jump(C::Al, 0x798f8, true)?;
    }
    for (op, value) in [(Op::Mvn, 5), (Op::Mov, 0), (Op::Mov, 0)] {
        c.emit(mov_register(C::Al, r(0), r(4)))?;
        c.emit(data_immediate(C::Al, op, r(1), r(0), value)?)?;
        c.jump(C::Al, 0x79df0, true)?;
    }
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x34)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x64)?)?;
    c.jump(C::Eq, 0x7a36c, false)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x2ec)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.jump(C::Eq, 0x7a398, false)?;
    c.check(0x7a36c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(5), 0x164)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.jump(C::Al, 0x79df0, true)?;
    c.check(0x7a398)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.jump(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.jump(C::Al, 0x79c18, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.jump(C::Al, 0x79c98, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0)?)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(5), 0x16d)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0)?)?;
    c.jump(C::Ne, 0x7a3d0, false)?;
    c.check(0x7a3cc)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(true), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7a3d0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(9))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.emit(transfer_multiple(C::Al, true, R::SP, saved(false), BlockMode::IncrementAfter, true)?)?;
    c.jump(C::Al, 0x79b34, false)?;
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
    fn code_extent_flags_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(bytes.len(), 440);
        assert_eq!(CODE_BYTES, 440);
        assert_eq!(DATA_BYTES, 0);
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            let class = (instruction >> 25) & 7;
            assert!([0, 1, 2, 4, 5].contains(&class));
            if class == 1 || (class == 0 && instruction & 0x90 != 0x90) {
                assert_eq!(instruction & (1 << 20) != 0,
                    [0x7a358, 0x7a364, 0x7a3bc, 0x7a3c4].contains(&at));
            }
            if class == 2 {
                assert_ne!((instruction >> 16) & 15, 15);
            }
            assert_eq!(instruction >> 28, match at {
                0x7a35c | 0x7a368 => 0,
                0x7a3c0 | 0x7a3c4 | 0x7a3c8 => 1,
                _ => 14,
            });
        }
        let mut edges = CALLS.to_vec();
        edges.extend([(0x7a35c, 0x7a36c), (0x7a368, 0x7a398),
            (0x7a3c8, 0x7a3d0), (0x7a3e8, 0x79b34)]);
        assert_eq!(bytes.chunks_exact(4).filter(|b|
            (u32::from_le_bytes((*b).try_into().unwrap()) >> 25) & 7 == 5).count(), edges.len());
        for (at, target) in edges {
            let condition = match at {
                0x7a35c | 0x7a368 => C::Eq,
                0x7a3c8 => C::Ne,
                _ => C::Al,
            };
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32,
                CALLS.contains(&(at, target))).unwrap());
        }
        for (at, returning) in [(0x7a3cc, true), (0x7a3e4, false)] {
            assert_eq!(word(&bytes, at), transfer_multiple(C::Al, true, R::SP,
                saved(returning), BlockMode::IncrementAfter, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x7a3c0), transfer_byte(C::Ne, true, r(0), r(5), 0x16d).unwrap());
        assert_eq!(word(&bytes, 0x7a3c4), data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0).unwrap());
        assert_eq!(word(&bytes, 0x7a3d4), load_signed_byte_register(C::Al, r(2), r(0), r(9)).unwrap());
        assert_eq!(word(&bytes, 0x7a3dc), data_register(C::Al, Op::Sub,
            r(1), r(0), r(8), Shift::Lsl, 0, false).unwrap());
    }

    #[test]
    fn structural_bounds_and_branch_guards() {
        for at in (START..END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_ok());
            assert_eq!(c.at, at + 4);
            assert_eq!(c.bytes.len(), 4);
        }
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        for (at, condition, target, link) in [
            (START, C::Al, START, false), (START, C::Al, 0x798f8, true),
            (0x7a274, C::Ne, 0x798f8, true), (0x7a274, C::Al, 0x79db8, true),
            (0x7a3c8, C::Ne, 0x7a3cc, false), (0x7a3c8, C::Eq, 0x7a3d0, false),
            (0x7a3e8, C::Al, 0x79b34, true), (END, C::Al, START, false),
        ] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.jump(condition, target, link).is_err());
            assert!(c.bytes.is_empty());
            assert_eq!(c.at, at);
        }
        let c = TypedA32 { at: START, bytes: Vec::new() };
        assert!(c.check(END).is_err());
    }

    #[test]
    fn authenticated_reference_when_available() {
        let Some(path) = std::env::var_os("NABU_VENUS_REFERENCE")
            .or_else(|| std::env::var_os("VENUS_REFERENCE")) else { return };
        let reference = std::fs::read(path).unwrap();
        assert_eq!(reference.len(), 1_159_200);
        assert_eq!(crate::wmfw_sha256::hex_digest(&reference),
            "9d4af65d7ede845e900f1b29ff425b7a8e2947056e695e246e58a2091445a085");
        assert_eq!(fragment().unwrap(),
            reference[START + FILE_MAPPING_DELTA..END + FILE_MAPPING_DELTA]);
    }
}
