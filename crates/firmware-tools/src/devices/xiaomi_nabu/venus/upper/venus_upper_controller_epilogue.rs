use arm32::{
    branch, branch_exchange_immediate, compiler_trap, data_immediate,
    data_immediate_with_flags, data_register, mov_register, mov_shifted_register,
    multiply_accumulate, transfer_byte, transfer_byte_register, transfer_halfword_register,
    transfer_multiple, transfer_word, transfer_word_register, BlockMode,
    Condition as C, DataOp as Op, Register as R, Shift,
};

pub const START: usize = 0x697a0;
pub const END: usize = 0x69b80;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
const QUALITY_TABLE: u32 = 0xf2da8;
const RATE_TABLE: u32 = 0xf2d96;
const QUALITY_THRESHOLD_TABLE: u32 = 0xf2dec;
const QUALITY_INIT_FORMAT: u32 = 0xf3197;
const UPDATE_INIT_I_FRAME_FORMAT: u32 = 0xf31bc;
const RATE_CONTROL_SOURCE: u32 = 0xf2e4c;
const PSNR_FORMAT: u32 = 0xf2e89;
const NEWLINE: u32 = 0xe2788;
const ALTERNATE_RATE_TABLE: u32 = 0xf1c38;
const ALTERNATE_QUALITY_TABLE: u32 = 0xf1c48;
const ALTERNATE_THRESHOLD_TABLE: u32 = 0xf1c7c;
const QUALITY_LOOKUP_TABLE: u32 = 0x10e084;
const QUALITY_LIMIT_TABLE: u32 = 0x10e304;
const RATE_SCALE_TABLE: u32 = 0x10b200;
const DATA_RANGES: [(usize, usize); 3] = [
    (0x69a10, 0x69a20),
    (0x69b0c, 0x69b10),
    (0x69b48, END),
];

fn r(number: u8) -> R {
    R::new(number).unwrap()
}

fn is_data(at: usize) -> bool {
    DATA_RANGES.iter().any(|&(start, end)| (start..end).contains(&at))
}

struct Code {
    at: usize,
    bytes: Vec<u8>,
}

impl Code {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("epilogue address {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn word(&mut self, value: u32, data: bool) -> Result<(), String> {
        if !(START..END).contains(&self.at) || self.at % 4 != 0 || is_data(self.at) != data {
            return Err(format!("epilogue output kind or address {:#x}", self.at));
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        self.word(instruction, false)
    }

    fn pointer(&mut self, value: u32) -> Result<(), String> {
        self.word(value, true)
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

    fn word_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_word(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn byte_transfer(&mut self, load: bool, rd: u8, rn: u8, offset: i32) -> Result<(), String> {
        self.emit(transfer_byte(C::Al, load, r(rd), r(rn), offset)?)
    }

    fn branch(&mut self, condition: C, target: usize) -> Result<(), String> {
        self.emit(branch(condition, self.at as u32, target as u32, false)?)
    }

    fn call(&mut self, target: usize) -> Result<(), String> {
        self.emit(branch(C::Al, self.at as u32, target as u32, true)?)
    }
}

fn clamp_record_bounds(c: &mut Code, source: i32, upper: i32, lower: i32, bounds: i32) -> Result<(), String> {
    c.byte_transfer(true, 0, 8, source)?;
    c.byte_transfer(false, 0, 8, bounds)?;
    if source == 0x28 {
        c.byte_transfer(false, 0, 8, 0xfa0)?;
    } else {
        c.emit(transfer_byte_register(C::Al, false, r(0), r(8), r(1), Shift::Lsl, 0)?)?;
    }
    c.register(Op::Add, 1, 10, 0)?;
    c.register(Op::Sub, 0, 0, 6)?;
    c.byte_transfer(true, 2, 8, upper)?;
    c.register(Op::Cmp, 0, 1, 2)?;
    c.mov(C::Hi, 1, 2)?;
    c.byte_transfer(false, 1, 8, bounds + 4)?;
    c.byte_transfer(true, 2, 8, lower)?;
    c.register(Op::Cmp, 0, 0, 2)?;
    c.mov(C::Lt, 0, 2)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x14)?;
    c.byte_transfer(false, 0, 8, bounds + 3)?;
    c.byte_transfer(false, 1, 8, bounds + 7)?;
    c.byte_transfer(false, 0, 8, bounds + 5)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code { at: START, bytes: Vec::with_capacity(END - START) };
    c.immediate(C::Al, Op::Mov, 0, 0, 0x3d)?;
    c.immediate(C::Al, Op::Orr, 0, 0, 0x600)?;
    c.word_transfer(false, 0, 8, 0xb0c)?;
    c.byte_transfer(false, 9, 8, 0xa85)?;
    c.byte_transfer(false, 7, 8, 0xd10)?;
    c.word_transfer(true, 0, 8, 0)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x1000)?;
    c.branch(C::Ne, 0x6981c)?;
    c.immediate(C::Al, Op::Sub, 0, 11, 0x1d)?;
    c.immediate(C::Al, Op::And, 1, 4, 0xff)?;
    c.immediate(C::Al, Op::Add, 3, 13, 0x40)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.word_transfer(false, 0, 13, 0)?;
    c.immediate(C::Al, Op::And, 0, 5, 0xff)?;
    c.call(0x69c3c)?;
    c.byte_transfer(true, 6, 11, -0x1d)?;
    c.byte_transfer(true, 7, 13, 0x40)?;
    c.byte_transfer(true, 3, 8, 0xa84)?;
    c.immediate(C::Al, Op::Add, 0, 8, 0x92)?;
    c.immediate(C::Al, Op::Add, 5, 0, 0xa00)?;
    c.mov(C::Al, 0, 5)?;
    c.mov(C::Al, 1, 7)?;
    c.mov(C::Al, 2, 6)?;
    c.call(0x69cd8)?;
    c.byte_transfer(true, 3, 8, 0xa84)?;
    c.mov(C::Al, 0, 5)?;
    c.mov(C::Al, 1, 7)?;
    c.mov(C::Al, 2, 6)?;
    c.call(0x69cd8)?;
    c.byte_transfer(true, 10, 8, 0x2c)?;
    c.byte_transfer(true, 6, 8, 0x2d)?;
    c.check(0x6981c)?;
    c.word_transfer(true, 4, 13, 0x34)?;
    c.byte_transfer(false, 9, 8, 0xd14)?;
    clamp_record_bounds(&mut c, 0x28, 0xd20, 0xd1f, 0xd16)?;
    c.immediate(C::Al, Op::Add, 0, 8, 0x184)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Add, 0, 0, 0xc00)?;
    c.call(0x31ec)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x3d)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x230)?;
    c.immediate(C::Al, Op::Orr, 0, 0, 0x600)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x1000)?;
    c.mov(C::Al, 5, 0)?;
    c.word_transfer(false, 5, 8, 0xd9c)?;
    c.byte_transfer(false, 9, 8, 0xd15)?;
    c.byte_transfer(false, 9, 8, 0xfa4)?;
    clamp_record_bounds(&mut c, 0x29, 0xfb0, 0xfaf, 0xfa6)?;
    c.immediate(C::Al, Op::Add, 0, 8, 0x14)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Add, 0, 0, 0x1000)?;
    c.call(0x31ec)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x2c)?;
    c.immediate(C::Al, Op::Orr, 0, 0, 0x1000)?;
    c.emit(transfer_word_register(C::Al, false, r(5), r(8), r(0), Shift::Lsl, 0)?)?;
    c.byte_transfer(false, 9, 8, 0xfa5)?;
    c.word_transfer(true, 0, 8, 0xc)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x699a8)?;
    c.word_transfer(true, 0, 13, 4)?;
    c.word_transfer(true, 2, 13, 0x24)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(8), r(0), 0xff, true)?)?;
    c.branch(C::Eq, 0x699ac)?;
    c.immediate(C::Al, Op::Add, 0, 4, 0x32c)?;
    c.immediate(C::Al, Op::Mov, 6, 0, 0x3b4)?;
    c.immediate(C::Al, Op::Mov, 10, 0, 0x9c)?;
    c.immediate(C::Al, Op::Add, 7, 0, 0x800)?;
    c.immediate(C::Al, Op::Add, 0, 4, 0x208)?;
    c.immediate(C::Al, Op::Orr, 6, 6, 0x400)?;
    c.immediate(C::Al, Op::Orr, 10, 10, 0x800)?;
    c.immediate(C::Al, Op::Add, 5, 0, 0x400)?;
    c.immediate(C::Al, Op::Add, 0, 4, 0x20c)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 0)?;
    c.immediate(C::Al, Op::Add, 9, 0, 0x400)?;
    c.check(0x6993c)?;
    c.word_transfer(true, 1, 13, 0x28)?;
    c.mov(C::Al, 0, 5)?;
    c.mov(C::Al, 2, 6)?;
    c.call(0x471c)?;
    c.immediate(C::Al, Op::Cmp, 0, 4, 0)?;
    c.mov(C::Al, 0, 9)?;
    c.branch(C::Eq, 0x69984)?;
    c.word_transfer(true, 0, 13, 0x3c)?;
    c.word_transfer(true, 0, 0, 0xc)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 2)?;
    c.branch(C::Eq, 0x69980)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 1)?;
    c.branch(C::Ne, 0x69988)?;
    c.word_transfer(true, 1, 13, 0x34)?;
    c.emit(multiply_accumulate(C::Al, r(0), r(4), r(6), r(1))?)?;
    c.register(Op::Add, 0, 0, 10)?;
    c.branch(C::Al, 0x69984)?;
    c.check(0x69980)?;
    c.mov(C::Al, 0, 7)?;
    c.word_transfer(false, 0, 5, 0)?;
    c.immediate(C::Al, Op::Add, 4, 4, 1)?;
    c.register(Op::Add, 7, 7, 6)?;
    c.register(Op::Add, 5, 5, 6)?;
    c.register(Op::Cmp, 0, 4, 8)?;
    c.branch(C::Ne, 0x6993c)?;
    c.word_transfer(true, 0, 13, 0x3c)?;
    c.byte_transfer(true, 10, 0, 0x2c)?;
    c.byte_transfer(true, 6, 0, 0x2d)?;
    c.check(0x699a8)?;
    c.word_transfer(true, 2, 13, 0x24)?;
    c.word_transfer(true, 7, 13, 0x3c)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 2)?;
    c.byte_transfer(false, 0, 7, 0x781)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.byte_transfer(false, 0, 7, 0x782)?;
    c.byte_transfer(false, 4, 7, 0x783)?;
    c.word_transfer(true, 0, 7, 0xa80)?;
    c.byte_transfer(true, 1, 0, 7)?;
    c.byte_transfer(false, 1, 7, 0x784)?;
    c.byte_transfer(true, 0, 0, 9)?;
    c.byte_transfer(false, 0, 7, 0x785)?;
    c.byte_transfer(false, 10, 7, 0x786)?;
    c.byte_transfer(false, 6, 7, 0x787)?;
    c.byte_transfer(true, 0, 7, 0x2ad)?;
    c.byte_transfer(false, 0, 7, 0x788)?;
    c.byte_transfer(true, 0, 7, 0x30)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0)?;
    c.branch(C::Eq, 0x69a20)?;
    c.word_transfer(true, 0, 7, 0x38)?;
    c.word_transfer(true, 3, 7, 0x3c)?;
    c.immediate(C::Al, Op::Add, 0, 0, 0x3fc)?;
    c.immediate(C::Al, Op::Add, 1, 0, 0xfc00)?;
    c.immediate(C::Al, Op::Sub, 0, 3, 4)?;
    c.branch(C::Al, 0x69a48)?;
    c.check(0x69a10)?;
    c.pointer(QUALITY_TABLE)?;
    c.pointer(RATE_TABLE)?;
    c.pointer(QUALITY_THRESHOLD_TABLE)?;
    c.pointer(QUALITY_INIT_FORMAT)?;
    c.check(0x69a20)?;
    c.word_transfer(true, 0, 7, 0)?;
    c.word_transfer(true, 5, 2, 0x22c)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 3)?;
    c.immediate(C::Al, Op::Mov, 4, 0, 3)?;
    c.immediate(C::Al, Op::Cmp, 0, 0, 0x2000)?;
    c.word_transfer(true, 0, 2, 0x230)?;
    c.immediate(C::Eq, Op::Mov, 1, 0, 6)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsl, 1)?)?;
    c.emit(branch_exchange_immediate(c.at as u32, 0x83c4c)?)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(5), Shift::Lsr, 1)?)?;
    c.check(0x69a48)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0x39c)?;
    c.immediate(C::Al, Op::Orr, 2, 2, 0x400)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(2))?)?;
    c.word_transfer(false, 0, 7, 0x7a4)?;
    c.immediate(C::Al, Op::Add, 0, 7, 0x89)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.immediate(C::Al, Op::Mov, 2, 0, 0)?;
    c.word_transfer(false, 4, 7, 0x7a8)?;
    c.immediate(C::Al, Op::Add, 0, 0, 0x700)?;
    c.check(0x69a6c)?;
    c.immediate(C::Al, Op::Add, 2, 2, 1)?;
    c.byte_transfer(false, 1, 0, 0)?;
    c.byte_transfer(false, 1, 0, 9)?;
    c.immediate(C::Al, Op::Add, 0, 0, 1)?;
    c.immediate(C::Al, Op::Cmp, 0, 2, 9)?;
    c.branch(C::Ne, 0x69a6c)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 1)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0)?;
    c.word_transfer(false, 0, 7, 0x7a0)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0x33c)?;
    c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(0))?)?;
    for (low, high) in [
        (0x3a, 0x300), (0x3e, 0x300), (0x24, 0x400), (0x22, 0x400),
        (0x26, 0x400), (0x10c, 0x400), (0xa, 0x500), (0xe, 0x500),
    ] {
        c.immediate(C::Al, Op::Mov, 0, 0, low)?;
        c.immediate(C::Al, Op::Orr, 0, 0, high)?;
        c.emit(transfer_halfword_register(C::Al, false, r(1), r(7), r(0))?)?;
    }
    c.check(0x69af8)?;
    c.immediate(C::Al, Op::Sub, 13, 11, 0x1c)?;
    c.emit(transfer_multiple(C::Al, true, r(13),
        r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit()
            | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(),
        BlockMode::IncrementAfter, true)?)?;
    c.check(0x69b00)?;
    c.word_transfer(false, 1, 4, 0x2e4)?;
    c.mov(C::Al, 0, 1)?;
    c.branch(C::Al, 0x692a0)?;
    c.check(0x69b0c)?;
    c.pointer(UPDATE_INIT_I_FRAME_FORMAT)?;
    c.check(0x69b10)?;
    c.word_transfer(true, 0, 8, 0x64)?;
    c.word_transfer(true, 1, 13, 0x34)?;
    c.register(Op::Sub, 0, 0, 3)?;
    c.word_transfer(false, 0, 1, 8)?;
    c.branch(C::Al, 0x67ee4)?;
    c.check(0x69b24)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(0), r(10), Shift::Lsl, 2)?)?;
    c.immediate(C::Al, Op::Sub, 2, 0, 0x5a)?;
    c.branch(C::Al, 0x6843c)?;
    c.check(0x69b30)?;
    c.word_transfer(true, 2, 15, 0x10)?;
    c.immediate(C::Al, Op::Mov, 1, 0, 0x37)?;
    c.immediate(C::Al, Op::Mov, 0, 0, 0)?;
    c.immediate(C::Al, Op::Orr, 1, 1, 0x400)?;
    c.call(0x17b0)?;
    c.check(0x69b44)?;
    c.emit(compiler_trap())?;
    c.check(0x69b48)?;
    c.pointer(RATE_CONTROL_SOURCE)?;
    c.pointer(PSNR_FORMAT)?;
    c.pointer(NEWLINE)?;
    c.pointer(QUALITY_TABLE)?;
    c.pointer(RATE_TABLE)?;
    c.pointer(QUALITY_THRESHOLD_TABLE)?;
    c.pointer(QUALITY_INIT_FORMAT)?;
    c.pointer(UPDATE_INIT_I_FRAME_FORMAT)?;
    c.pointer(ALTERNATE_RATE_TABLE)?;
    c.pointer(ALTERNATE_QUALITY_TABLE)?;
    c.pointer(ALTERNATE_THRESHOLD_TABLE)?;
    c.pointer(QUALITY_LOOKUP_TABLE)?;
    c.pointer(QUALITY_LIMIT_TABLE)?;
    c.pointer(RATE_SCALE_TABLE)?;
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
    fn extent_and_embedded_pointers() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 992);
        assert_eq!(bytes.len(), 992);
        let data_bytes: usize = DATA_RANGES.iter().map(|(start, end)| end - start).sum();
        assert_eq!(data_bytes, 76);
        assert_eq!(bytes.len() - data_bytes, 916);
        for (at, pointer) in [
            (0x69a10, QUALITY_TABLE), (0x69a14, RATE_TABLE),
            (0x69a18, QUALITY_THRESHOLD_TABLE), (0x69a1c, QUALITY_INIT_FORMAT),
            (0x69b0c, UPDATE_INIT_I_FRAME_FORMAT),
            (0x69b48, RATE_CONTROL_SOURCE), (0x69b4c, PSNR_FORMAT), (0x69b50, NEWLINE),
            (0x69b54, QUALITY_TABLE), (0x69b58, RATE_TABLE),
            (0x69b5c, QUALITY_THRESHOLD_TABLE), (0x69b60, QUALITY_INIT_FORMAT),
            (0x69b64, UPDATE_INIT_I_FRAME_FORMAT), (0x69b68, ALTERNATE_RATE_TABLE),
            (0x69b6c, ALTERNATE_QUALITY_TABLE), (0x69b70, ALTERNATE_THRESHOLD_TABLE),
            (0x69b74, QUALITY_LOOKUP_TABLE), (0x69b78, QUALITY_LIMIT_TABLE),
            (0x69b7c, RATE_SCALE_TABLE),
        ] {
            assert_eq!(word(&bytes, at), pointer);
        }
    }

    #[test]
    fn output_kind_and_bounds_are_checked() {
        for (start, end) in DATA_RANGES {
            for at in (start..end).step_by(4) {
                let mut c = Code { at, bytes: Vec::new() };
                assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
                assert!(c.bytes.is_empty());
                c.pointer(QUALITY_TABLE).unwrap();
                assert_eq!(c.at, at + 4);
            }
        }
        for at in [START - 4, START + 1, END] {
            let mut c = Code { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.pointer(QUALITY_TABLE).is_err());
        }
        let mut c = Code { at: START, bytes: Vec::new() };
        assert!(c.pointer(QUALITY_TABLE).is_err());
        assert!(c.check(END).is_err());
    }

    #[test]
    fn ands_sets_flags() {
        let bytes = fragment().unwrap();
        let opcode = word(&bytes, 0x69908);
        assert_eq!(opcode, data_immediate_with_flags(C::Al, Op::And, r(8), r(0), 0xff, true).unwrap());
        assert_ne!(opcode & (1 << 20), 0);
    }

    #[test]
    fn branch_edges_and_cold_blocks() {
        let bytes = fragment().unwrap();
        for (at, condition, target) in [
            (0x697bc, C::Ne, 0x6981c), (0x698fc, C::Eq, 0x699a8),
            (0x6990c, C::Eq, 0x699ac), (0x69954, C::Eq, 0x69984),
            (0x69964, C::Eq, 0x69980), (0x6996c, C::Ne, 0x69988),
            (0x6997c, C::Al, 0x69984), (0x69998, C::Ne, 0x6993c),
            (0x699f4, C::Eq, 0x69a20), (0x69a0c, C::Al, 0x69a48),
            (0x69a80, C::Ne, 0x69a6c), (0x69b08, C::Al, 0x692a0),
            (0x69b20, C::Al, 0x67ee4), (0x69b2c, C::Al, 0x6843c),
        ] {
            assert!(!is_data(target));
            assert_eq!(word(&bytes, at), branch(condition, at as u32, target as u32, false).unwrap());
        }
        assert_eq!(word(&bytes, 0x69b00), transfer_word(C::Al, false, r(1), r(4), 0x2e4).unwrap());
        assert_eq!(word(&bytes, 0x69b10), transfer_word(C::Al, true, r(0), r(8), 0x64).unwrap());
        assert_eq!(word(&bytes, 0x69b1c), transfer_word(C::Al, false, r(0), r(1), 8).unwrap());
        assert_eq!(word(&bytes, 0x69b24), transfer_word_register(C::Al, true, r(0), r(0), r(10), Shift::Lsl, 2).unwrap());
        assert_eq!(word(&bytes, 0x69b28), data_immediate(C::Al, Op::Sub, r(2), r(0), 0x5a).unwrap());
    }

    #[test]
    fn return_assertion_and_calls() {
        let bytes = fragment().unwrap();
        assert_eq!(word(&bytes, 0x69af8), data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c).unwrap());
        assert_eq!(word(&bytes, 0x69afc), transfer_multiple(C::Al, true, r(13),
            r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit()
                | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(),
            BlockMode::IncrementAfter, true).unwrap());
        assert_eq!(word(&bytes, 0x69b30), transfer_word(C::Al, true, r(2), r(15), 0x10).unwrap());
        assert_eq!(0x69b30 + 8 + 0x10, 0x69b48);
        assert_eq!(word(&bytes, 0x69b34), data_immediate(C::Al, Op::Mov, r(1), r(0), 0x37).unwrap());
        assert_eq!(word(&bytes, 0x69b3c), data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400).unwrap());
        assert_eq!(0x37 | 0x400, 0x437);
        assert_eq!(word(&bytes, 0x69b44), compiler_trap());
        for (at, target) in [
            (0x697d8, 0x69c3c), (0x697fc, 0x69cd8), (0x69810, 0x69cd8),
            (0x69870, 0x31ec), (0x698e0, 0x31ec), (0x69948, 0x471c),
            (0x69b40, 0x17b0),
        ] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, target, true).unwrap());
        }
        assert_eq!(word(&bytes, 0x69a40), branch_exchange_immediate(0x69a40, 0x83c4c).unwrap());
    }
}
