use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_register, data_register_shifted_by_register, load_halfword,
    load_signed_byte_register, mov_register, mov_shifted_register, push_registers,
    transfer_byte, transfer_halfword_register, transfer_multiple, transfer_word,
    transfer_word_register,
};

pub const START: usize = 0x7c6b8;
pub const END: usize = 0x7d4f8;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 3644;
pub const DATA_BYTES: usize = 4;
pub const FUNCTION_STARTS: [usize; 4] = [START, 0x7ceac, 0x7d328, 0x7d390];
const ASSERTION_LOG: u32 = 0xf4260;

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("HEVC header writer at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..END).contains(&at) && at % 4 == 0 && at != 0x7d324
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) {
            return Err("HEVC header instruction outside code".into());
        }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if at != 0x7d324 || value != ASSERTION_LOG {
            return Err("HEVC header literal mismatch".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        if self.at != 0x7d1e4 || rd != r(2) || target != 0x7d324 {
            return Err("invalid HEVC header literal load".into());
        }
        self.emit(transfer_word(C::Al, true, rd, R::PC, target as i32 - self.at as i32 - 8)?)
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
              (0x7c6f4, C::Eq, 0x7c704, false)
            | (0x7c700, C::Al, 0x7c718, false)
            | (0x7c718, C::Al, 0x79b6c, true)
            | (0x7c730, C::Al, 0x7c2e8, true)
            | (0x7c740, C::Al, 0x798f8, true)
            | (0x7c754, C::Al, 0x798f8, true)
            | (0x7c76c, C::Al, 0x798f8, true)
            | (0x7c778, C::Al, 0x7c51c, true)
            | (0x7c784, C::Al, 0x79db8, true)
            | (0x7c790, C::Al, 0x79db8, true)
            | (0x7c79c, C::Al, 0x79db8, true)
            | (0x7c7a8, C::Al, 0x79db8, true)
            | (0x7c7e4, C::Al, 0x798f8, true)
            | (0x7c7ec, C::Eq, 0x7c838, false)
            | (0x7c7f8, C::Al, 0x79db8, true)
            | (0x7c810, C::Al, 0x79db8, true)
            | (0x7c81c, C::Al, 0x79db8, true)
            | (0x7c834, C::Al, 0x79db8, true)
            | (0x7c848, C::Al, 0x79db8, true)
            | (0x7c854, C::Al, 0x79db8, true)
            | (0x7c860, C::Al, 0x79db8, true)
            | (0x7c870, C::Al, 0x798f8, true)
            | (0x7c87c, C::Ne, 0x7c898, false)
            | (0x7c888, C::Al, 0x79db8, true)
            | (0x7c894, C::Al, 0x7c8ac, false)
            | (0x7c8a0, C::Al, 0x79db8, true)
            | (0x7c8ac, C::Al, 0x79db8, true)
            | (0x7c8b8, C::Al, 0x79db8, true)
            | (0x7c8cc, C::Ls, 0x7c874, false)
            | (0x7c8d8, C::Cc, 0x7c8f4, false)
            | (0x7c8f0, C::Hi, 0x7c8e0, false)
            | (0x7c8f8, C::Al, 0x79db8, true)
            | (0x7c90c, C::Al, 0x79db8, true)
            | (0x7c91c, C::Al, 0x79db8, true)
            | (0x7c930, C::Al, 0x79db8, true)
            | (0x7c940, C::Al, 0x79db8, true)
            | (0x7c950, C::Al, 0x79db8, true)
            | (0x7c964, C::Al, 0x798f8, true)
            | (0x7c974, C::Al, 0x798f8, true)
            | (0x7c994, C::Al, 0x798f8, true)
            | (0x7c9a8, C::Al, 0x798f8, true)
            | (0x7c9b8, C::Eq, 0x7ca1c, false)
            | (0x7c9cc, C::Al, 0x798f8, true)
            | (0x7c9e4, C::Al, 0x798f8, true)
            | (0x7c9f4, C::Al, 0x79db8, true)
            | (0x7ca04, C::Al, 0x79db8, true)
            | (0x7ca18, C::Al, 0x798f8, true)
            | (0x7ca24, C::Al, 0x79db8, true)
            | (0x7ca30, C::Eq, 0x7ca6c, false)
            | (0x7ca58, C::Al, 0x7c0fc, true)
            | (0x7ca68, C::Cc, 0x7ca44, false)
            | (0x7ca74, C::Eq, 0x7ca98, false)
            | (0x7ca84, C::Al, 0x798f8, true)
            | (0x7ca90, C::Al, 0x79db8, true)
            | (0x7ca94, C::Al, 0x7caa8, false)
            | (0x7caa4, C::Al, 0x798f8, true)
            | (0x7cab8, C::Al, 0x798f8, true)
            | (0x7cacc, C::Al, 0x798f8, true)
            | (0x7cad8, C::Eq, 0x7ce2c, false)
            | (0x7cae8, C::Eq, 0x7cb08, false)
            | (0x7cafc, C::Al, 0x35540, true)
            | (0x7cb2c, C::Al, 0x798f8, true)
            | (0x7cb40, C::Al, 0x798f8, true)
            | (0x7cb4c, C::Eq, 0x7cb98, false)
            | (0x7cb58, C::Al, 0x7a08c, true)
            | (0x7cb6c, C::Al, 0x798f8, true)
            | (0x7cb74, C::Ne, 0x7cb98, false)
            | (0x7cb84, C::Al, 0x798f8, true)
            | (0x7cb94, C::Al, 0x798f8, true)
            | (0x7cba8, C::Al, 0x798f8, true)
            | (0x7cbbc, C::Al, 0x798f8, true)
            | (0x7cbc8, C::Eq, 0x7cc7c, false)
            | (0x7cbf0, C::Al, 0x798f8, true)
            | (0x7cc08, C::Al, 0x798f8, true)
            | (0x7cc18, C::Al, 0x798f8, true)
            | (0x7cc24, C::Ne, 0x7cc7c, false)
            | (0x7cc48, C::Al, 0x798f8, true)
            | (0x7cc60, C::Al, 0x798f8, true)
            | (0x7cc78, C::Al, 0x798f8, true)
            | (0x7cc90, C::Al, 0x798f8, true)
            | (0x7cc9c, C::Eq, 0x7ccc0, false)
            | (0x7ccac, C::Al, 0x798f8, true)
            | (0x7ccbc, C::Al, 0x798f8, true)
            | (0x7cccc, C::Al, 0x798f8, true)
            | (0x7ccdc, C::Al, 0x798f8, true)
            | (0x7ccec, C::Al, 0x798f8, true)
            | (0x7ccfc, C::Al, 0x798f8, true)
            | (0x7cd10, C::Al, 0x798f8, true)
            | (0x7cd1c, C::Eq, 0x7cd9c, false)
            | (0x7cd2c, C::Al, 0x798f8, true)
            | (0x7cd3c, C::Al, 0x798f8, true)
            | (0x7cd58, C::Al, 0x798f8, true)
            | (0x7cd74, C::Al, 0x798f8, true)
            | (0x7cd84, C::Al, 0x798f8, true)
            | (0x7cd94, C::Al, 0x798f8, true)
            | (0x7cdac, C::Al, 0x798f8, true)
            | (0x7cdb8, C::Eq, 0x7ce3c, false)
            | (0x7cdc8, C::Al, 0x798f8, true)
            | (0x7cdd8, C::Al, 0x798f8, true)
            | (0x7cde8, C::Al, 0x798f8, true)
            | (0x7cdf4, C::Al, 0x79db8, true)
            | (0x7ce00, C::Al, 0x79db8, true)
            | (0x7ce0c, C::Al, 0x79db8, true)
            | (0x7ce18, C::Al, 0x79db8, true)
            | (0x7ce24, C::Al, 0x79db8, true)
            | (0x7ce28, C::Al, 0x7ce3c, false)
            | (0x7ce38, C::Al, 0x798f8, true)
            | (0x7ce48, C::Al, 0x798f8, true)
            | (0x7ce58, C::Al, 0x798f8, true)
            | (0x7ce64, C::Al, 0x79c18, true)
            | (0x7ce6c, C::Al, 0x79c98, true)
            | (0x7ce80, C::Eq, 0x7cea4, false)
            | (0x7cea0, C::Al, 0x79b34, true)
            | (0x7cedc, C::Eq, 0x7ceec, false)
            | (0x7cee8, C::Al, 0x7cf00, false)
            | (0x7cf00, C::Al, 0x79b6c, true)
            | (0x7cf10, C::Al, 0x7c2e8, true)
            | (0x7cf1c, C::Al, 0x79db8, true)
            | (0x7cf28, C::Al, 0x79db8, true)
            | (0x7cf3c, C::Al, 0x798f8, true)
            | (0x7cf4c, C::Al, 0x798f8, true)
            | (0x7cf5c, C::Al, 0x798f8, true)
            | (0x7cf6c, C::Al, 0x798f8, true)
            | (0x7cf7c, C::Al, 0x798f8, true)
            | (0x7cf90, C::Al, 0x79db8, true)
            | (0x7cf9c, C::Al, 0x79db8, true)
            | (0x7cfb8, C::Al, 0x79df0, true)
            | (0x7cfcc, C::Al, 0x798f8, true)
            | (0x7cfe0, C::Al, 0x798f8, true)
            | (0x7cff0, C::Al, 0x798f8, true)
            | (0x7cffc, C::Eq, 0x7d00c, false)
            | (0x7d008, C::Al, 0x79db8, true)
            | (0x7d014, C::Al, 0x79df0, true)
            | (0x7d020, C::Al, 0x79df0, true)
            | (0x7d030, C::Al, 0x798f8, true)
            | (0x7d040, C::Al, 0x798f8, true)
            | (0x7d050, C::Al, 0x798f8, true)
            | (0x7d064, C::Al, 0x798f8, true)
            | (0x7d074, C::Al, 0x798f8, true)
            | (0x7d084, C::Al, 0x798f8, true)
            | (0x7d090, C::Eq, 0x7d154, false)
            | (0x7d0a0, C::Al, 0x79db8, true)
            | (0x7d0b0, C::Al, 0x79db8, true)
            | (0x7d0c4, C::Al, 0x798f8, true)
            | (0x7d0d0, C::Eq, 0x7d0fc, false)
            | (0x7d0e4, C::Al, 0x79db8, true)
            | (0x7d0f8, C::Cc, 0x7d0d8, false)
            | (0x7d104, C::Eq, 0x7d134, false)
            | (0x7d11c, C::Al, 0x79db8, true)
            | (0x7d130, C::Cc, 0x7d110, false)
            | (0x7d150, C::Al, 0x798f8, true)
            | (0x7d164, C::Al, 0x798f8, true)
            | (0x7d17c, C::Ne, 0x7d1a0, false)
            | (0x7d188, C::Ne, 0x7d1e0, false)
            | (0x7d198, C::Al, 0x798f8, true)
            | (0x7d19c, C::Al, 0x7d1bc, false)
            | (0x7d1ac, C::Al, 0x798f8, true)
            | (0x7d1b8, C::Eq, 0x7d264, false)
            | (0x7d1c8, C::Al, 0x798f8, true)
            | (0x7d1d8, C::Al, 0x798f8, true)
            | (0x7d1dc, C::Al, 0x7d264, false)
            | (0x7d1fc, C::Al, 0x17b0, true)
            | (0x7d20c, C::Al, 0x798f8, true)
            | (0x7d21c, C::Al, 0x798f8, true)
            | (0x7d22c, C::Al, 0x798f8, true)
            | (0x7d24c, C::Al, 0x79df0, true)
            | (0x7d260, C::Al, 0x79df0, true)
            | (0x7d270, C::Al, 0x798f8, true)
            | (0x7d27c, C::Eq, 0x7d28c, false)
            | (0x7d288, C::Al, 0x7d294, false)
            | (0x7d298, C::Al, 0x798f8, true)
            | (0x7d2a4, C::Al, 0x79db8, true)
            | (0x7d2b4, C::Al, 0x798f8, true)
            | (0x7d2c4, C::Al, 0x798f8, true)
            | (0x7d2d4, C::Al, 0x798f8, true)
            | (0x7d2e0, C::Al, 0x79c18, true)
            | (0x7d2e8, C::Al, 0x79c98, true)
            | (0x7d2f8, C::Ne, 0x7d300, false)
            | (0x7d320, C::Al, 0x79b34, false)
            | (0x7d348, C::Al, 0x798c8, true)
            | (0x7d358, C::Al, 0x7c340, true)
            | (0x7d36c, C::Al, 0x7c6b8, true)
            | (0x7d37c, C::Al, 0x7ceac, true)
            | (0x7d3c0, C::Al, 0x79b6c, true)
            | (0x7d3d4, C::Al, 0x7c2e8, true)
            | (0x7d3e4, C::Al, 0x798f8, true)
            | (0x7d3f4, C::Al, 0x798f8, true)
            | (0x7d408, C::Al, 0x798f8, true)
            | (0x7d418, C::Al, 0x798f8, true)
            | (0x7d428, C::Ne, 0x7d3fc, false)
            | (0x7d438, C::Al, 0x798f8, true)
            | (0x7d448, C::Al, 0x798f8, true)
            | (0x7d458, C::Al, 0x798f8, true)
            | (0x7d468, C::Al, 0x798f8, true)
            | (0x7d478, C::Al, 0x798f8, true)
            | (0x7d488, C::Al, 0x798f8, true)
            | (0x7d498, C::Al, 0x798f8, true)
            | (0x7d4a8, C::Al, 0x798f8, true)
            | (0x7d4b8, C::Al, 0x798f8, true)
            | (0x7d4c4, C::Al, 0x79c18, true)
            | (0x7d4cc, C::Al, 0x79c98, true)
            | (0x7d4f4, C::Al, 0x79b34, false)
        );
        if !valid { return Err("invalid HEVC header branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }
}

fn write_sequence_parameter_set(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7c6b8)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7c6bc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.check(0x7c6c0)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x1c)?)?;
    c.check(0x7c6c4)?;
    c.emit(transfer_word(C::Al, false, r(3), r(13), 0xc)?)?;
    c.check(0x7c6c8)?;
    c.emit(mov_register(C::Al, r(9), r(0)))?;
    c.check(0x7c6cc)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.check(0x7c6d0)?;
    c.emit(mov_register(C::Al, r(10), r(2)))?;
    c.check(0x7c6d4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71c)?)?;
    c.check(0x7c6d8)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x71d)?)?;
    c.check(0x7c6dc)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(9), 0x71e)?)?;
    c.check(0x7c6e0)?;
    c.emit(data_register(C::Al, Op::Sub, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7c6e4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.check(0x7c6e8)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x8)?)?;
    c.check(0x7c6ec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x82c)?)?;
    c.check(0x7c6f0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c6f4)?;
    c.branch(C::Eq, 0x7c704, false)?;
    c.check(0x7c6f8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c6fc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c700)?;
    c.branch(C::Al, 0x7c718, false)?;
    c.check(0x7c704)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7c708)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7c70c)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7c710)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(1))?)?;
    c.check(0x7c714)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c718)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7c71c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x21)?)?;
    c.check(0x7c720)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c724)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.check(0x7c728)?;
    c.emit(data_register_shifted_by_register(C::Al, Op::Mov, r(5), r(0), r(5), Shift::Lsr, r(7), false)?)?;
    c.check(0x7c72c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.check(0x7c730)?;
    c.branch(C::Al, 0x7c2e8, true)?;
    c.check(0x7c734)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c738)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c73c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.check(0x7c740)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c744)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71b)?)?;
    c.check(0x7c748)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7c74c)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7c750)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c754)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c758)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x71b)?)?;
    c.check(0x7c75c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c760)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c764)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7c768)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c76c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c770)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.check(0x7c774)?;
    c.emit(mov_register(C::Al, r(1), r(4)))?;
    c.check(0x7c778)?;
    c.branch(C::Al, 0x7c51c, true)?;
    c.check(0x7c77c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c780)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c784)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c788)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c78c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c790)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c794)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x7dc)?)?;
    c.check(0x7c798)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c79c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c7a0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x7e0)?)?;
    c.check(0x7c7a4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c7a8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c7ac)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x50)?)?;
    c.check(0x7c7b0)?;
    c.emit(transfer_word(C::Al, true, r(3), r(9), 0x7e0)?)?;
    c.check(0x7c7b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.check(0x7c7b8)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x7dc)?)?;
    c.check(0x7c7bc)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7c7c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c7c4)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c7c8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c7cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7c7d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c7d4)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7c7d8)?;
    c.emit(data_register(C::Al, Op::Orr, r(7), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7c7dc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c7e0)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7c7e4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c7e8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7c7ec)?;
    c.branch(C::Eq, 0x7c838, false)?;
    c.check(0x7c7f0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c7f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c7f8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c7fc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.check(0x7c800)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x7dc)?)?;
    c.check(0x7c804)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c808)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 1)?)?;
    c.check(0x7c80c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c810)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c814)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c818)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c81c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c820)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x50)?)?;
    c.check(0x7c824)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x7e0)?)?;
    c.check(0x7c828)?;
    c.emit(data_register(C::Al, Op::Sub, r(0), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c82c)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Lsr, 1)?)?;
    c.check(0x7c830)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c834)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c838)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71f)?)?;
    c.check(0x7c83c)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(7), r(0), 0x8)?)?;
    c.check(0x7c840)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c844)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7c848)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c84c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c850)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7c854)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c858)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x720)?)?;
    c.check(0x7c85c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c860)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c864)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c868)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7c86c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c870)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c874)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x1e8)?)?;
    c.check(0x7c878)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xc)?)?;
    c.check(0x7c87c)?;
    c.branch(C::Ne, 0x7c898, false)?;
    c.check(0x7c880)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c884)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c888)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c88c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c890)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c894)?;
    c.branch(C::Al, 0x7c8ac, false)?;
    c.check(0x7c898)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x165)?)?;
    c.check(0x7c89c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c8a0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c8a4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x719)?)?;
    c.check(0x7c8a8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c8ac)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c8b0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c8b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c8b8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c8bc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71b)?)?;
    c.check(0x7c8c0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.check(0x7c8c4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.check(0x7c8c8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(8), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7c8cc)?;
    c.branch(C::Ls, 0x7c874, false)?;
    c.check(0x7c8d0)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x2)?)?;
    c.check(0x7c8d4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x2)?)?;
    c.check(0x7c8d8)?;
    c.branch(C::Cc, 0x7c8f4, false)?;
    c.check(0x7c8dc)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(1), r(0), 0x2)?)?;
    c.check(0x7c8e0)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(5), Shift::Lsr, 1)?)?;
    c.check(0x7c8e4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x3)?)?;
    c.check(0x7c8e8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x1)?)?;
    c.check(0x7c8ec)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0x7c8f0)?;
    c.branch(C::Hi, 0x7c8e0, false)?;
    c.check(0x7c8f4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c8f8)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c8fc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71c)?)?;
    c.check(0x7c900)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x71d)?)?;
    c.check(0x7c904)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7c908)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c90c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c910)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x722)?)?;
    c.check(0x7c914)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x2)?)?;
    c.check(0x7c918)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c91c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c920)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x721)?)?;
    c.check(0x7c924)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x722)?)?;
    c.check(0x7c928)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.check(0x7c92c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c930)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c934)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x723)?)?;
    c.check(0x7c938)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7c93c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c940)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c944)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x724)?)?;
    c.check(0x7c948)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7c94c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c950)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c954)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c958)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7c95c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c960)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7c964)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c968)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x725)?)?;
    c.check(0x7c96c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c970)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c974)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c978)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7c97c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c980)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x14)?)?;
    c.check(0x7c984)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7c988)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c98c)?;
    c.emit(data_immediate(C::Gt, Op::Mov, r(7), r(0), 0x1)?)?;
    c.check(0x7c990)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7c994)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c998)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7c99c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7c9a0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0xd0)?)?;
    c.check(0x7c9a4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c9a8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c9ac)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7c9b0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0xd0)?)?;
    c.check(0x7c9b4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7c9b8)?;
    c.branch(C::Eq, 0x7ca1c, false)?;
    c.check(0x7c9bc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0xb4)?)?;
    c.check(0x7c9c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.check(0x7c9c4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7c9c8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c9cc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c9d0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7c9d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x4)?)?;
    c.check(0x7c9d8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0xb8)?)?;
    c.check(0x7c9dc)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7c9e0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c9e4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7c9e8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x715)?)?;
    c.check(0x7c9ec)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x3)?)?;
    c.check(0x7c9f0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7c9f4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7c9f8)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x715)?)?;
    c.check(0x7c9fc)?;
    c.emit(data_immediate(C::Al, Op::Rsb, r(1), r(0), 0x8)?)?;
    c.check(0x7ca00)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ca04)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ca08)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7ca0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ca10)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x10)?)?;
    c.check(0x7ca14)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ca18)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ca1c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x71a)?)?;
    c.check(0x7ca20)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ca24)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ca28)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71a)?)?;
    c.check(0x7ca2c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ca30)?;
    c.branch(C::Eq, 0x7ca6c, false)?;
    c.check(0x7ca34)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(9), 0x34c)?)?;
    c.check(0x7ca38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7ca3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7ca40)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(0), 0x400)?)?;
    c.check(0x7ca44)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(5), r(7), Shift::Lsl, 2)?)?;
    c.check(0x7ca48)?;
    c.emit(mov_register(C::Al, r(2), r(7)))?;
    c.check(0x7ca4c)?;
    c.emit(mov_register(C::Al, r(3), r(4)))?;
    c.check(0x7ca50)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x0)?)?;
    c.check(0x7ca54)?;
    c.emit(transfer_word(C::Al, false, r(6), r(13), 0x0)?)?;
    c.check(0x7ca58)?;
    c.branch(C::Al, 0x7c0fc, true)?;
    c.check(0x7ca5c)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x71a)?)?;
    c.check(0x7ca60)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7ca64)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7ca68)?;
    c.branch(C::Cc, 0x7ca44, false)?;
    c.check(0x7ca6c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x10c)?)?;
    c.check(0x7ca70)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ca74)?;
    c.branch(C::Eq, 0x7ca98, false)?;
    c.check(0x7ca78)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ca7c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ca80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ca84)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ca88)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ca8c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ca90)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ca94)?;
    c.branch(C::Al, 0x7caa8, false)?;
    c.check(0x7ca98)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ca9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7caa0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7caa4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7caa8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7caac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cab0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x33c)?)?;
    c.check(0x7cab4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cab8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cabc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7cac0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cac4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0xb0)?)?;
    c.check(0x7cac8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cacc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cad0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cad4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7cad8)?;
    c.branch(C::Eq, 0x7ce2c, false)?;
    c.check(0x7cadc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cae0)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x4)?)?;
    c.check(0x7cae4)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x10)?)?;
    c.check(0x7cae8)?;
    c.branch(C::Eq, 0x7cb08, false)?;
    c.check(0x7caec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x834)?)?;
    c.check(0x7caf0)?;
    c.emit(transfer_word(C::Al, true, r(2), r(9), 0x4)?)?;
    c.check(0x7caf4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x14c)?)?;
    c.check(0x7caf8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x6e4)?)?;
    c.check(0x7cafc)?;
    c.branch(C::Al, 0x35540, true)?;
    c.check(0x7cb00)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0x10)?)?;
    c.check(0x7cb04)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cb08)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x1)?)?;
    c.check(0x7cb0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7cb10)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cb14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x1)?)?;
    c.check(0x7cb18)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(9), 0x1a8)?)?;
    c.check(0x7cb1c)?;
    c.emit(transfer_word(C::Ne, false, r(0), r(13), 0x14)?)?;
    c.check(0x7cb20)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(9), 0x1ac)?)?;
    c.check(0x7cb24)?;
    c.emit(transfer_word(C::Ne, false, r(0), r(13), 0x18)?)?;
    c.check(0x7cb28)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cb2c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cb30)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cb34)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cb38)?;
    c.emit(data_immediate(C::Al, Op::And, r(1), r(0), 0x1)?)?;
    c.check(0x7cb3c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cb40)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cb44)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cb48)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x1)?)?;
    c.check(0x7cb4c)?;
    c.branch(C::Eq, 0x7cb98, false)?;
    c.check(0x7cb50)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(13), 0x10)?)?;
    c.check(0x7cb54)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x4)?)?;
    c.check(0x7cb58)?;
    c.branch(C::Al, 0x7a08c, true)?;
    c.check(0x7cb5c)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0x7cb60)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cb64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7cb68)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7cb6c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cb70)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0xff)?)?;
    c.check(0x7cb74)?;
    c.branch(C::Ne, 0x7cb98, false)?;
    c.check(0x7cb78)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x14)?)?;
    c.check(0x7cb7c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cb80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7cb84)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cb88)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x18)?)?;
    c.check(0x7cb8c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cb90)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7cb94)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cb98)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cb9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cba0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cba4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0x7cba8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cbac)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cbb0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cbb4)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(7), r(0), Shift::Lsr, 1, false)?)?;
    c.check(0x7cbb8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cbbc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cbc0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cbc4)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x2)?)?;
    c.check(0x7cbc8)?;
    c.branch(C::Eq, 0x7cc7c, false)?;
    c.check(0x7cbcc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7cbd0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x1c0)?)?;
    c.check(0x7cbd4)?;
    c.emit(transfer_word(C::Al, true, r(6), r(9), 0x1c4)?)?;
    c.check(0x7cbd8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x1)?)?;
    c.check(0x7cbdc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7cbe0)?;
    c.emit(transfer_byte(C::Al, true, r(7), r(0), 0x143)?)?;
    c.check(0x7cbe4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cbe8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7cbec)?;
    c.emit(mov_register(C::Ne, r(1), r(8)))?;
    c.check(0x7cbf0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cbf4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x0)?)?;
    c.check(0x7cbf8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cbfc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cc00)?;
    c.emit(mov_register(C::Eq, r(5), r(6)))?;
    c.check(0x7cc04)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7cc08)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cc0c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(9), 0x1c8)?)?;
    c.check(0x7cc10)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cc14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cc18)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cc1c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x1c8)?)?;
    c.check(0x7cc20)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7cc24)?;
    c.branch(C::Ne, 0x7cc7c, false)?;
    c.check(0x7cc28)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(9), 0x1cc)?)?;
    c.check(0x7cc2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7cc30)?;
    c.emit(transfer_multiple(C::Al, true, r(7), r(1).bit() | r(5).bit() | r(7).bit(), BlockMode::IncrementAfter, false)?)?;
    c.check(0x7cc34)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7cc38)?;
    c.emit(transfer_byte(C::Al, true, r(6), r(0), 0x143)?)?;
    c.check(0x7cc3c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cc40)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7cc44)?;
    c.emit(mov_register(C::Ne, r(1), r(8)))?;
    c.check(0x7cc48)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cc4c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7cc50)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cc54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7cc58)?;
    c.emit(mov_register(C::Ne, r(5), r(8)))?;
    c.check(0x7cc5c)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7cc60)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cc64)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7cc68)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cc6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7cc70)?;
    c.emit(mov_register(C::Ne, r(7), r(8)))?;
    c.check(0x7cc74)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7cc78)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cc7c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cc80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x1)?)?;
    c.check(0x7cc84)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cc88)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(5), r(0), Shift::Lsr, 4, false)?)?;
    c.check(0x7cc8c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cc90)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cc94)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cc98)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x10)?)?;
    c.check(0x7cc9c)?;
    c.branch(C::Eq, 0x7ccc0, false)?;
    c.check(0x7cca0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x19a)?)?;
    c.check(0x7cca4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cca8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7ccac)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ccb0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(9), 0x19a)?)?;
    c.check(0x7ccb4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ccb8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7ccbc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ccc0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ccc4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ccc8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cccc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ccd0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ccd4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ccd8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ccdc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cce0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cce4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cce8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ccec)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ccf0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ccf4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ccf8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ccfc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd00)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cd04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cd08)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(5), r(0), Shift::Lsr, 2, false)?)?;
    c.check(0x7cd0c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd10)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd14)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cd18)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x4)?)?;
    c.check(0x7cd1c)?;
    c.branch(C::Eq, 0x7cd9c, false)?;
    c.check(0x7cd20)?;
    c.emit(load_halfword(C::Al, r(1), r(13), 0x12)?)?;
    c.check(0x7cd24)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd28)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7cd2c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd30)?;
    c.emit(load_halfword(C::Al, r(1), r(13), 0x10)?)?;
    c.check(0x7cd34)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7cd3c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd40)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x834)?)?;
    c.check(0x7cd44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xe6)?)?;
    c.check(0x7cd48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7cd4c)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x600)?)?;
    c.check(0x7cd50)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(0), r(1))?)?;
    c.check(0x7cd54)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd58)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd5c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x834)?)?;
    c.check(0x7cd60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2e4)?)?;
    c.check(0x7cd64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7cd68)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7cd6c)?;
    c.emit(transfer_halfword_register(C::Al, true, r(1), r(0), r(1))?)?;
    c.check(0x7cd70)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd74)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd78)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd7c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cd80)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cd84)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd88)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cd8c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cd90)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cd94)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cd98)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cd9c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7cda0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cda4)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(1), r(0), Shift::Lsr, 3, false)?)?;
    c.check(0x7cda8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cdac)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cdb0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(9), 0x264)?)?;
    c.check(0x7cdb4)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(0), 0x8)?)?;
    c.check(0x7cdb8)?;
    c.branch(C::Eq, 0x7ce3c, false)?;
    c.check(0x7cdbc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cdc0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cdc4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cdc8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cdcc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cdd0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7cdd4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cdd8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cddc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cde0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7cde4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cde8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cdec)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cdf0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cdf4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7cdf8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cdfc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x2)?)?;
    c.check(0x7ce00)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ce04)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce08)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ce0c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ce10)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9)?)?;
    c.check(0x7ce18)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ce1c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce20)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x8)?)?;
    c.check(0x7ce24)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7ce28)?;
    c.branch(C::Al, 0x7ce3c, false)?;
    c.check(0x7ce2c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ce34)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ce38)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ce3c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce40)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ce44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ce48)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ce4c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7ce54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7ce58)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7ce5c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7ce64)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7ce68)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7ce6c)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7ce70)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0xc)?)?;
    c.check(0x7ce74)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ce78)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(9), 0x16d)?)?;
    c.check(0x7ce7c)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7ce80)?;
    c.branch(C::Eq, 0x7cea4, false)?;
    c.check(0x7ce84)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7ce88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7ce8c)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7ce90)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(1))?)?;
    c.check(0x7ce94)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x20)?)?;
    c.check(0x7ce98)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x8)?)?;
    c.check(0x7ce9c)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7cea0)?;
    c.branch(C::Al, 0x79b34, true)?;
    c.check(0x7cea4)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x7cea8)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7ceac)
}

fn write_picture_parameter_set(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7ceac)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7ceb0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7ceb4)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7ceb8)?;
    c.emit(mov_register(C::Al, r(4), r(1)))?;
    c.check(0x7cebc)?;
    c.emit(mov_register(C::Al, r(9), r(2)))?;
    c.check(0x7cec0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x716)?)?;
    c.check(0x7cec4)?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 0x20)?)?;
    c.check(0x7cec8)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(6), r(0), Shift::Lsl, 2, false)?)?;
    c.check(0x7cecc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x74c)?)?;
    c.check(0x7ced0)?;
    c.emit(transfer_byte(C::Al, true, r(5), r(0), 0x3)?)?;
    c.check(0x7ced4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x82c)?)?;
    c.check(0x7ced8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7cedc)?;
    c.branch(C::Eq, 0x7ceec, false)?;
    c.check(0x7cee0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cee4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cee8)?;
    c.branch(C::Al, 0x7cf00, false)?;
    c.check(0x7ceec)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7cef0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7cef4)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7cef8)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(1))?)?;
    c.check(0x7cefc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf00)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7cf04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x22)?)?;
    c.check(0x7cf08)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cf0c)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.check(0x7cf10)?;
    c.branch(C::Al, 0x7c2e8, true)?;
    c.check(0x7cf14)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf18)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cf1c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7cf20)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf24)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cf28)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7cf2c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7cf30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cf34)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0xa8)?)?;
    c.check(0x7cf38)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf3c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cf40)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cf48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cf4c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cf50)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf54)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cf58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7cf5c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cf60)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf64)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7cf68)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cf6c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cf70)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x726)?)?;
    c.check(0x7cf74)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf78)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cf7c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cf80)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0x7cf84)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf88)?;
    c.emit(data_immediate(C::Ne, Op::Sub, r(5), r(5), 0x1)?)?;
    c.check(0x7cf8c)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7cf90)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7cf94)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cf98)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7cf9c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7cfa0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x15c)?)?;
    c.check(0x7cfa4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x2fa)?)?;
    c.check(0x7cfa8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7cfac)?;
    c.emit(data_immediate(C::Eq, Op::Sub, r(1), r(0), 0x1a)?)?;
    c.check(0x7cfb0)?;
    c.emit(data_immediate(C::Ne, Op::Sub, r(1), r(0), 0x26)?)?;
    c.check(0x7cfb4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cfb8)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7cfbc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7cfc0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cfc4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x34)?)?;
    c.check(0x7cfc8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cfcc)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cfd0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7cfd4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cfd8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x60)?)?;
    c.check(0x7cfdc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cfe0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cfe4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x727)?)?;
    c.check(0x7cfe8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7cfec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7cff0)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7cff4)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x727)?)?;
    c.check(0x7cff8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7cffc)?;
    c.branch(C::Eq, 0x7d00c, false)?;
    c.check(0x7d000)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d004)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d008)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d00c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d010)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d014)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7d018)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d01c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d020)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7d024)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d028)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d02c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d030)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d034)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d038)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d03c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d040)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d044)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d048)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d04c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d050)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d054)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d058)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d05c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0xac)?)?;
    c.check(0x7d060)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d064)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d068)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x14c)?)?;
    c.check(0x7d06c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d070)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d074)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d078)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d07c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d080)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d084)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d088)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x14c)?)?;
    c.check(0x7d08c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d090)?;
    c.branch(C::Eq, 0x7d154, false)?;
    c.check(0x7d094)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x130)?)?;
    c.check(0x7d098)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7d09c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d0a0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d0a4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x134)?)?;
    c.check(0x7d0a8)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(1), r(0), 0x1)?)?;
    c.check(0x7d0ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d0b0)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d0b4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d0b8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d0bc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d0c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0x7d0c4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d0c8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x130)?)?;
    c.check(0x7d0cc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7d0d0)?;
    c.branch(C::Eq, 0x7d0fc, false)?;
    c.check(0x7d0d4)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(7), r(0), 0x0)?)?;
    c.check(0x7d0d8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x120)?)?;
    c.check(0x7d0dc)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(7), r(0), Shift::Lsr, 5, false)?)?;
    c.check(0x7d0e0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d0e4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d0e8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x130)?)?;
    c.check(0x7d0ec)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.check(0x7d0f0)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.check(0x7d0f4)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7d0f8)?;
    c.branch(C::Cc, 0x7d0d8, false)?;
    c.check(0x7d0fc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x134)?)?;
    c.check(0x7d100)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x1)?)?;
    c.check(0x7d104)?;
    c.branch(C::Eq, 0x7d134, false)?;
    c.check(0x7d108)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0x7d10c)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(7), r(0), 0x0)?)?;
    c.check(0x7d110)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x128)?)?;
    c.check(0x7d114)?;
    c.emit(data_register(C::Al, Op::Add, r(1), r(7), r(0), Shift::Lsr, 5, false)?)?;
    c.check(0x7d118)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d11c)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d120)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x134)?)?;
    c.check(0x7d124)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.check(0x7d128)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(0), r(0), 0x1)?)?;
    c.check(0x7d12c)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(5), r(0), Shift::Lsl, 0, false)?)?;
    c.check(0x7d130)?;
    c.branch(C::Cc, 0x7d110, false)?;
    c.check(0x7d134)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d138)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d13c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x30)?)?;
    c.check(0x7d140)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d144)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d148)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d14c)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d150)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d154)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d158)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d15c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x2c)?)?;
    c.check(0x7d160)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d164)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d168)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d16c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(0), 0x51c)?)?;
    c.check(0x7d170)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x12)?)?;
    c.check(0x7d174)?;
    c.emit(transfer_word(C::Ne, true, r(1), r(6), 0x82c)?)?;
    c.check(0x7d178)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(1), 0x0)?)?;
    c.check(0x7d17c)?;
    c.branch(C::Ne, 0x7d1a0, false)?;
    c.check(0x7d180)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x2c4)?)?;
    c.check(0x7d184)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x1)?)?;
    c.check(0x7d188)?;
    c.branch(C::Ne, 0x7d1e0, false)?;
    c.check(0x7d18c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d190)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d194)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d198)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d19c)?;
    c.branch(C::Al, 0x7d1bc, false)?;
    c.check(0x7d1a0)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x718)?)?;
    c.check(0x7d1a4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d1a8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d1ac)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d1b0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x718)?)?;
    c.check(0x7d1b4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d1b8)?;
    c.branch(C::Eq, 0x7d264, false)?;
    c.check(0x7d1bc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d1c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d1c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d1c8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d1cc)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d1d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d1d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d1d8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d1dc)?;
    c.branch(C::Al, 0x7d264, false)?;
    c.check(0x7d1e0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x38)?)?;
    c.check(0x7d1e4)?;
    c.literal(r(2), 0x7d324)?;
    c.check(0x7d1e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x51)?)?;
    c.check(0x7d1ec)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7d1f0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d1f4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7d1f8)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7d1fc)?;
    c.branch(C::Al, 0x17b0, true)?;
    c.check(0x7d200)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d204)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d208)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d20c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d210)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d214)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d218)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d21c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d220)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d224)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d228)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d22c)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d230)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x2cc)?)?;
    c.check(0x7d234)?;
    c.emit(transfer_word(C::Al, true, r(5), r(6), 0x2c8)?)?;
    c.check(0x7d238)?;
    c.emit(data_immediate(C::Al, Op::Mvn, r(7), r(0), 0xf)?)?;
    c.check(0x7d23c)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(0), Shift::Asr, 1)?)?;
    c.check(0x7d240)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x7)?)?;
    c.check(0x7d244)?;
    c.emit(data_register(C::Gt, Op::Add, r(1), r(7), r(0), Shift::Asr, 1, false)?)?;
    c.check(0x7d248)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d24c)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7d250)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(5), Shift::Asr, 1)?)?;
    c.check(0x7d254)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d258)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0x7)?)?;
    c.check(0x7d25c)?;
    c.emit(data_register(C::Gt, Op::Add, r(1), r(7), r(5), Shift::Asr, 1, false)?)?;
    c.check(0x7d260)?;
    c.branch(C::Al, 0x79df0, true)?;
    c.check(0x7d264)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d268)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d26c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d270)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d274)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x10c)?)?;
    c.check(0x7d278)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d27c)?;
    c.branch(C::Eq, 0x7d28c, false)?;
    c.check(0x7d280)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d284)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d288)?;
    c.branch(C::Al, 0x7d294, false)?;
    c.check(0x7d28c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x728)?)?;
    c.check(0x7d290)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d294)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d298)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d29c)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(6), 0x729)?)?;
    c.check(0x7d2a0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d2a4)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d2a8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d2ac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d2b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d2b4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d2b8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d2bc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d2c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d2c4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d2c8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d2cc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d2d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d2d4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d2d8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d2dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d2e0)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7d2e4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d2e8)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7d2ec)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(9), 0x0)?)?;
    c.check(0x7d2f0)?;
    c.emit(transfer_byte(C::Ne, true, r(0), r(6), 0x16d)?)?;
    c.check(0x7d2f4)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d2f8)?;
    c.branch(C::Ne, 0x7d300, false)?;
    c.check(0x7d2fc)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7d300)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d304)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x11c)?)?;
    c.check(0x7d308)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x400)?)?;
    c.check(0x7d30c)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(1))?)?;
    c.check(0x7d310)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.check(0x7d314)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.check(0x7d318)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.check(0x7d31c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7d320)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(0x7d324)?;
    c.data(0x7d324, ASSERTION_LOG)?;
    c.check(0x7d328)
}

fn write_parameter_sets(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7d328)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7d32c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x10)?)?;
    c.check(0x7d330)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x30)?)?;
    c.check(0x7d334)?;
    c.emit(mov_register(C::Al, r(6), r(13)))?;
    c.check(0x7d338)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.check(0x7d33c)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.check(0x7d340)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d344)?;
    c.emit(mov_register(C::Al, r(0), r(6)))?;
    c.check(0x7d348)?;
    c.branch(C::Al, 0x798c8, true)?;
    c.check(0x7d34c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d350)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7d354)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d358)?;
    c.branch(C::Al, 0x7c340, true)?;
    c.check(0x7d35c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d360)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7d364)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.check(0x7d368)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x1)?)?;
    c.check(0x7d36c)?;
    c.branch(C::Al, 0x7c6b8, true)?;
    c.check(0x7d370)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d374)?;
    c.emit(mov_register(C::Al, r(1), r(6)))?;
    c.check(0x7d378)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d37c)?;
    c.branch(C::Al, 0x7ceac, true)?;
    c.check(0x7d380)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0x24)?)?;
    c.check(0x7d384)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 3)?)?;
    c.check(0x7d388)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x10)?)?;
    c.check(0x7d38c)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7d390)
}

fn write_mastering_display(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7d390)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7d394)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7d398)?;
    c.emit(mov_register(C::Al, r(5), r(1)))?;
    c.check(0x7d39c)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.check(0x7d3a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7d3a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.check(0x7d3a8)?;
    c.emit(transfer_word(C::Al, true, r(9), r(5), 0x20)?)?;
    c.check(0x7d3ac)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(5), 0xc)?)?;
    c.check(0x7d3b0)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.check(0x7d3b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d3b8)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(8))?)?;
    c.check(0x7d3bc)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d3c0)?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.check(0x7d3c4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x27)?)?;
    c.check(0x7d3c8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d3cc)?;
    c.emit(mov_register(C::Al, r(2), r(5)))?;
    c.check(0x7d3d0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7d3d4)?;
    c.branch(C::Al, 0x7c2e8, true)?;
    c.check(0x7d3d8)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d3dc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x89)?)?;
    c.check(0x7d3e0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7d3e4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d3e8)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d3ec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x18)?)?;
    c.check(0x7d3f0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7d3f4)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d3f8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(6), 0x9e0)?)?;
    c.check(0x7d3fc)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), -0xc)?)?;
    c.check(0x7d400)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d404)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7d408)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d40c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x0)?)?;
    c.check(0x7d410)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d414)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7d418)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d41c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7d420)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x4)?)?;
    c.check(0x7d424)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x3)?)?;
    c.check(0x7d428)?;
    c.branch(C::Ne, 0x7d3fc, false)?;
    c.check(0x7d42c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x9ec)?)?;
    c.check(0x7d430)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d434)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7d438)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d43c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x9f0)?)?;
    c.check(0x7d440)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d444)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7d448)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d44c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x9f4)?)?;
    c.check(0x7d450)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d454)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x20)?)?;
    c.check(0x7d458)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d45c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x9f8)?)?;
    c.check(0x7d460)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d464)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x20)?)?;
    c.check(0x7d468)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d46c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d470)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x90)?)?;
    c.check(0x7d474)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7d478)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d47c)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d480)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x4)?)?;
    c.check(0x7d484)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7d488)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d48c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0x9fc)?)?;
    c.check(0x7d490)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d494)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7d498)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d49c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(6), 0xa00)?)?;
    c.check(0x7d4a0)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d4a4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x10)?)?;
    c.check(0x7d4a8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d4ac)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d4b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d4b4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d4b8)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d4bc)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d4c0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d4c4)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.check(0x7d4c8)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.check(0x7d4cc)?;
    c.branch(C::Al, 0x79c98, true)?;
    c.check(0x7d4d0)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x16d)?)?;
    c.check(0x7d4d4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d4d8)?;
    c.emit(transfer_multiple(C::Eq, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7d4dc)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.check(0x7d4e0)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(8))?)?;
    c.check(0x7d4e4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x20)?)?;
    c.check(0x7d4e8)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(9), Shift::Lsl, 0, false)?)?;
    c.check(0x7d4ec)?;
    c.emit(mov_register(C::Al, r(0), r(9)))?;
    c.check(0x7d4f0)?;
    c.emit(transfer_multiple(C::Al, true, R::SP, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x7d4f4)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(END)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    write_sequence_parameter_set(&mut c)?;
    write_picture_parameter_set(&mut c)?;
    write_parameter_sets(&mut c)?;
    write_mastering_display(&mut c)?;
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
    fn structure_and_control_flow() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 3648);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(FUNCTION_STARTS, [0x7c6b8, 0x7ceac, 0x7d328, 0x7d390]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 911);
        assert_eq!(CODE_BYTES, 911 * 4);
        assert_eq!(DATA_BYTES, 4);
        assert_eq!(word(&bytes, 0x7d324), ASSERTION_LOG);
        assert_eq!(word(&bytes, 0x7d1e4), transfer_word(C::Al, true, r(2), R::PC, 0x138).unwrap());
        assert_eq!(word(&bytes, 0x7d1fc), branch(C::Al, 0x7d1fc, 0x17b0, true).unwrap());
        assert_eq!(word(&bytes, 0x7d200), mov_register(C::Al, r(0), r(4)));
        for at in [0x7d320, 0x7d4f4] {
            assert_eq!(word(&bytes, at), branch(C::Al, at as u32, 0x79b34, false).unwrap());
        }
        for (at, condition, last) in [(0x7d2fc, C::Al, R::PC), (0x7d31c, C::Al, R::LR),
            (0x7d4d8, C::Eq, R::PC), (0x7d4f0, C::Al, R::LR)] {
            let mask = [4, 5, 6, 7, 8, 9, 11].into_iter().fold(last.bit(), |mask, n| mask | r(n).bit());
            assert_eq!(word(&bytes, at), transfer_multiple(condition, true, R::SP, mask,
                BlockMode::IncrementAfter, true).unwrap());
        }
        for value in [0u32, 1, 14, 16, 0x8000_0000, u32::MAX] {
            let half = (value as i32) >> 1;
            let adjusted = if half > 7 { half.wrapping_sub(16) } else { half };
            assert_eq!(adjusted as u32, if half > 7 { (half as u32).wrapping_add(!15) } else { half as u32 });
            assert_eq!(u32::MAX.wrapping_add(value >> 5), (value >> 5).wrapping_sub(1));
        }
    }

    #[test]
    fn exact_code_data_and_literal_guards() {
        for at in (START..END).step_by(4).chain([START - 4, START + 1, 0x7d325, END, END + 4]) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(0).is_ok(), TypedA32::is_code(at));
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.data(at, ASSERTION_LOG).is_ok(), at == 0x7d324);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.data(at, 0).is_err());
            assert!(c.bytes.is_empty());
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.literal(r(2), 0x7d324).is_ok(), at == 0x7d1e4);
        }
        let mut c = TypedA32 { at: 0x7d1e4, bytes: Vec::new() };
        assert!(c.literal(r(1), 0x7d324).is_err());
        assert!(c.literal(r(2), 0x7d328).is_err());
        assert!(c.check(END).is_err());
        assert!(c.data(0x7d324, ASSERTION_LOG).is_err());
        assert!(c.bytes.is_empty());
    }

    #[test]
    fn exact_branch_guards() {
        let bytes = fragment().unwrap();
        let edges = [
            (0x7c6f4, C::Eq, 0x7c704, false),
            (0x7c700, C::Al, 0x7c718, false),
            (0x7c718, C::Al, 0x79b6c, true),
            (0x7c730, C::Al, 0x7c2e8, true),
            (0x7c740, C::Al, 0x798f8, true),
            (0x7c754, C::Al, 0x798f8, true),
            (0x7c76c, C::Al, 0x798f8, true),
            (0x7c778, C::Al, 0x7c51c, true),
            (0x7c784, C::Al, 0x79db8, true),
            (0x7c790, C::Al, 0x79db8, true),
            (0x7c79c, C::Al, 0x79db8, true),
            (0x7c7a8, C::Al, 0x79db8, true),
            (0x7c7e4, C::Al, 0x798f8, true),
            (0x7c7ec, C::Eq, 0x7c838, false),
            (0x7c7f8, C::Al, 0x79db8, true),
            (0x7c810, C::Al, 0x79db8, true),
            (0x7c81c, C::Al, 0x79db8, true),
            (0x7c834, C::Al, 0x79db8, true),
            (0x7c848, C::Al, 0x79db8, true),
            (0x7c854, C::Al, 0x79db8, true),
            (0x7c860, C::Al, 0x79db8, true),
            (0x7c870, C::Al, 0x798f8, true),
            (0x7c87c, C::Ne, 0x7c898, false),
            (0x7c888, C::Al, 0x79db8, true),
            (0x7c894, C::Al, 0x7c8ac, false),
            (0x7c8a0, C::Al, 0x79db8, true),
            (0x7c8ac, C::Al, 0x79db8, true),
            (0x7c8b8, C::Al, 0x79db8, true),
            (0x7c8cc, C::Ls, 0x7c874, false),
            (0x7c8d8, C::Cc, 0x7c8f4, false),
            (0x7c8f0, C::Hi, 0x7c8e0, false),
            (0x7c8f8, C::Al, 0x79db8, true),
            (0x7c90c, C::Al, 0x79db8, true),
            (0x7c91c, C::Al, 0x79db8, true),
            (0x7c930, C::Al, 0x79db8, true),
            (0x7c940, C::Al, 0x79db8, true),
            (0x7c950, C::Al, 0x79db8, true),
            (0x7c964, C::Al, 0x798f8, true),
            (0x7c974, C::Al, 0x798f8, true),
            (0x7c994, C::Al, 0x798f8, true),
            (0x7c9a8, C::Al, 0x798f8, true),
            (0x7c9b8, C::Eq, 0x7ca1c, false),
            (0x7c9cc, C::Al, 0x798f8, true),
            (0x7c9e4, C::Al, 0x798f8, true),
            (0x7c9f4, C::Al, 0x79db8, true),
            (0x7ca04, C::Al, 0x79db8, true),
            (0x7ca18, C::Al, 0x798f8, true),
            (0x7ca24, C::Al, 0x79db8, true),
            (0x7ca30, C::Eq, 0x7ca6c, false),
            (0x7ca58, C::Al, 0x7c0fc, true),
            (0x7ca68, C::Cc, 0x7ca44, false),
            (0x7ca74, C::Eq, 0x7ca98, false),
            (0x7ca84, C::Al, 0x798f8, true),
            (0x7ca90, C::Al, 0x79db8, true),
            (0x7ca94, C::Al, 0x7caa8, false),
            (0x7caa4, C::Al, 0x798f8, true),
            (0x7cab8, C::Al, 0x798f8, true),
            (0x7cacc, C::Al, 0x798f8, true),
            (0x7cad8, C::Eq, 0x7ce2c, false),
            (0x7cae8, C::Eq, 0x7cb08, false),
            (0x7cafc, C::Al, 0x35540, true),
            (0x7cb2c, C::Al, 0x798f8, true),
            (0x7cb40, C::Al, 0x798f8, true),
            (0x7cb4c, C::Eq, 0x7cb98, false),
            (0x7cb58, C::Al, 0x7a08c, true),
            (0x7cb6c, C::Al, 0x798f8, true),
            (0x7cb74, C::Ne, 0x7cb98, false),
            (0x7cb84, C::Al, 0x798f8, true),
            (0x7cb94, C::Al, 0x798f8, true),
            (0x7cba8, C::Al, 0x798f8, true),
            (0x7cbbc, C::Al, 0x798f8, true),
            (0x7cbc8, C::Eq, 0x7cc7c, false),
            (0x7cbf0, C::Al, 0x798f8, true),
            (0x7cc08, C::Al, 0x798f8, true),
            (0x7cc18, C::Al, 0x798f8, true),
            (0x7cc24, C::Ne, 0x7cc7c, false),
            (0x7cc48, C::Al, 0x798f8, true),
            (0x7cc60, C::Al, 0x798f8, true),
            (0x7cc78, C::Al, 0x798f8, true),
            (0x7cc90, C::Al, 0x798f8, true),
            (0x7cc9c, C::Eq, 0x7ccc0, false),
            (0x7ccac, C::Al, 0x798f8, true),
            (0x7ccbc, C::Al, 0x798f8, true),
            (0x7cccc, C::Al, 0x798f8, true),
            (0x7ccdc, C::Al, 0x798f8, true),
            (0x7ccec, C::Al, 0x798f8, true),
            (0x7ccfc, C::Al, 0x798f8, true),
            (0x7cd10, C::Al, 0x798f8, true),
            (0x7cd1c, C::Eq, 0x7cd9c, false),
            (0x7cd2c, C::Al, 0x798f8, true),
            (0x7cd3c, C::Al, 0x798f8, true),
            (0x7cd58, C::Al, 0x798f8, true),
            (0x7cd74, C::Al, 0x798f8, true),
            (0x7cd84, C::Al, 0x798f8, true),
            (0x7cd94, C::Al, 0x798f8, true),
            (0x7cdac, C::Al, 0x798f8, true),
            (0x7cdb8, C::Eq, 0x7ce3c, false),
            (0x7cdc8, C::Al, 0x798f8, true),
            (0x7cdd8, C::Al, 0x798f8, true),
            (0x7cde8, C::Al, 0x798f8, true),
            (0x7cdf4, C::Al, 0x79db8, true),
            (0x7ce00, C::Al, 0x79db8, true),
            (0x7ce0c, C::Al, 0x79db8, true),
            (0x7ce18, C::Al, 0x79db8, true),
            (0x7ce24, C::Al, 0x79db8, true),
            (0x7ce28, C::Al, 0x7ce3c, false),
            (0x7ce38, C::Al, 0x798f8, true),
            (0x7ce48, C::Al, 0x798f8, true),
            (0x7ce58, C::Al, 0x798f8, true),
            (0x7ce64, C::Al, 0x79c18, true),
            (0x7ce6c, C::Al, 0x79c98, true),
            (0x7ce80, C::Eq, 0x7cea4, false),
            (0x7cea0, C::Al, 0x79b34, true),
            (0x7cedc, C::Eq, 0x7ceec, false),
            (0x7cee8, C::Al, 0x7cf00, false),
            (0x7cf00, C::Al, 0x79b6c, true),
            (0x7cf10, C::Al, 0x7c2e8, true),
            (0x7cf1c, C::Al, 0x79db8, true),
            (0x7cf28, C::Al, 0x79db8, true),
            (0x7cf3c, C::Al, 0x798f8, true),
            (0x7cf4c, C::Al, 0x798f8, true),
            (0x7cf5c, C::Al, 0x798f8, true),
            (0x7cf6c, C::Al, 0x798f8, true),
            (0x7cf7c, C::Al, 0x798f8, true),
            (0x7cf90, C::Al, 0x79db8, true),
            (0x7cf9c, C::Al, 0x79db8, true),
            (0x7cfb8, C::Al, 0x79df0, true),
            (0x7cfcc, C::Al, 0x798f8, true),
            (0x7cfe0, C::Al, 0x798f8, true),
            (0x7cff0, C::Al, 0x798f8, true),
            (0x7cffc, C::Eq, 0x7d00c, false),
            (0x7d008, C::Al, 0x79db8, true),
            (0x7d014, C::Al, 0x79df0, true),
            (0x7d020, C::Al, 0x79df0, true),
            (0x7d030, C::Al, 0x798f8, true),
            (0x7d040, C::Al, 0x798f8, true),
            (0x7d050, C::Al, 0x798f8, true),
            (0x7d064, C::Al, 0x798f8, true),
            (0x7d074, C::Al, 0x798f8, true),
            (0x7d084, C::Al, 0x798f8, true),
            (0x7d090, C::Eq, 0x7d154, false),
            (0x7d0a0, C::Al, 0x79db8, true),
            (0x7d0b0, C::Al, 0x79db8, true),
            (0x7d0c4, C::Al, 0x798f8, true),
            (0x7d0d0, C::Eq, 0x7d0fc, false),
            (0x7d0e4, C::Al, 0x79db8, true),
            (0x7d0f8, C::Cc, 0x7d0d8, false),
            (0x7d104, C::Eq, 0x7d134, false),
            (0x7d11c, C::Al, 0x79db8, true),
            (0x7d130, C::Cc, 0x7d110, false),
            (0x7d150, C::Al, 0x798f8, true),
            (0x7d164, C::Al, 0x798f8, true),
            (0x7d17c, C::Ne, 0x7d1a0, false),
            (0x7d188, C::Ne, 0x7d1e0, false),
            (0x7d198, C::Al, 0x798f8, true),
            (0x7d19c, C::Al, 0x7d1bc, false),
            (0x7d1ac, C::Al, 0x798f8, true),
            (0x7d1b8, C::Eq, 0x7d264, false),
            (0x7d1c8, C::Al, 0x798f8, true),
            (0x7d1d8, C::Al, 0x798f8, true),
            (0x7d1dc, C::Al, 0x7d264, false),
            (0x7d1fc, C::Al, 0x17b0, true),
            (0x7d20c, C::Al, 0x798f8, true),
            (0x7d21c, C::Al, 0x798f8, true),
            (0x7d22c, C::Al, 0x798f8, true),
            (0x7d24c, C::Al, 0x79df0, true),
            (0x7d260, C::Al, 0x79df0, true),
            (0x7d270, C::Al, 0x798f8, true),
            (0x7d27c, C::Eq, 0x7d28c, false),
            (0x7d288, C::Al, 0x7d294, false),
            (0x7d298, C::Al, 0x798f8, true),
            (0x7d2a4, C::Al, 0x79db8, true),
            (0x7d2b4, C::Al, 0x798f8, true),
            (0x7d2c4, C::Al, 0x798f8, true),
            (0x7d2d4, C::Al, 0x798f8, true),
            (0x7d2e0, C::Al, 0x79c18, true),
            (0x7d2e8, C::Al, 0x79c98, true),
            (0x7d2f8, C::Ne, 0x7d300, false),
            (0x7d320, C::Al, 0x79b34, false),
            (0x7d348, C::Al, 0x798c8, true),
            (0x7d358, C::Al, 0x7c340, true),
            (0x7d36c, C::Al, 0x7c6b8, true),
            (0x7d37c, C::Al, 0x7ceac, true),
            (0x7d3c0, C::Al, 0x79b6c, true),
            (0x7d3d4, C::Al, 0x7c2e8, true),
            (0x7d3e4, C::Al, 0x798f8, true),
            (0x7d3f4, C::Al, 0x798f8, true),
            (0x7d408, C::Al, 0x798f8, true),
            (0x7d418, C::Al, 0x798f8, true),
            (0x7d428, C::Ne, 0x7d3fc, false),
            (0x7d438, C::Al, 0x798f8, true),
            (0x7d448, C::Al, 0x798f8, true),
            (0x7d458, C::Al, 0x798f8, true),
            (0x7d468, C::Al, 0x798f8, true),
            (0x7d478, C::Al, 0x798f8, true),
            (0x7d488, C::Al, 0x798f8, true),
            (0x7d498, C::Al, 0x798f8, true),
            (0x7d4a8, C::Al, 0x798f8, true),
            (0x7d4b8, C::Al, 0x798f8, true),
            (0x7d4c4, C::Al, 0x79c18, true),
            (0x7d4cc, C::Al, 0x79c98, true),
            (0x7d4f4, C::Al, 0x79b34, false),
        ];
        assert_eq!(edges.len(), 202);
        for &(at, condition, target, link) in &edges {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            c.branch(condition, target, link).unwrap();
            assert_eq!(word(&bytes, at), u32::from_le_bytes(c.bytes.try_into().unwrap()));
            for (source, cc, dest, linked) in [
                (at, condition, target, !link),
                (at, if condition == C::Al { C::Eq } else { C::Al }, target, link),
                (at, condition, target + 4, link),
                (at + 1, condition, target, link),
                (0x7d324, condition, target, link),
                (END, condition, target, link),
            ] {
                let mut c = TypedA32 { at: source, bytes: Vec::new() };
                assert!(c.branch(cc, dest, linked).is_err());
                assert!(c.bytes.is_empty());
            }
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
    }
}
