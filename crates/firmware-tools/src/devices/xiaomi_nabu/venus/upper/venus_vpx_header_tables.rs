use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch,
    data_immediate, data_immediate_with_flags, data_register,
    load_signed_halfword_postindexed, load_signed_halfword_register, mov_register,
    mov_shifted_register, push_registers, transfer_byte, transfer_byte_postindexed,
    transfer_byte_preindexed_writeback, transfer_byte_register,
    transfer_halfword_register, transfer_multiple, transfer_word, transfer_word_register,
};

pub const START: usize = 0x7d834;
pub const END: usize = 0x7df7c;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 1848;
pub const DATA_BYTES: usize = 16;
pub const FUNCTION_STARTS: [usize; 2] = [START, 0x7dc70];
pub const CODE_END: usize = 0x7df6c;
pub const EXTERNAL_TABLES: [(usize, usize); 2] = [(0xf430b, 1056), (0xf472b, 38)];

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("VPX header tables at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..CODE_END).contains(&at) && at % 4 == 0
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) { return Err("VPX instruction outside code".into()); }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7d850, C::Ne, 0x7d924, false)
            | (0x7d910, C::Al, 0x7a048, true)
            | (0x7d920, C::Al, 0x7a048, true)
            | (0x7d930, C::Al, 0x7a048, true)
            | (0x7d93c, C::Eq, 0x7dab4, false)
            | (0x7d94c, C::Al, 0x7a048, true)
            | (0x7d95c, C::Al, 0x7a048, true)
            | (0x7d970, C::Al, 0x7a048, true)
            | (0x7d984, C::Al, 0x7a048, true)
            | (0x7d9a4, C::Al, 0x7a048, true)
            | (0x7d9b4, C::Al, 0x7a048, true)
            | (0x7d9c0, C::Ne, 0x7d978, false)
            | (0x7d9d0, C::Eq, 0x7da38, false)
            | (0x7d9ec, C::Al, 0x7a048, true)
            | (0x7da10, C::Al, 0x7a048, true)
            | (0x7da20, C::Al, 0x7a048, true)
            | (0x7da30, C::Ne, 0x7d9e0, false)
            | (0x7da34, C::Al, 0x7da58, false)
            | (0x7da48, C::Al, 0x7a048, true)
            | (0x7da54, C::Ne, 0x7da3c, false)
            | (0x7da70, C::Ne, 0x7da84, false)
            | (0x7da80, C::Al, 0x7daa4, false)
            | (0x7da90, C::Al, 0x7a048, true)
            | (0x7daa4, C::Al, 0x7a048, true)
            | (0x7dab0, C::Ne, 0x7da64, false)
            | (0x7dac0, C::Al, 0x79e74, true)
            | (0x7dad0, C::Al, 0x7a048, true)
            | (0x7dae0, C::Al, 0x7a048, true)
            | (0x7daf0, C::Al, 0x7a048, true)
            | (0x7db00, C::Al, 0x7a048, true)
            | (0x7db10, C::Al, 0x7a048, true)
            | (0x7db20, C::Al, 0x79e74, true)
            | (0x7db30, C::Al, 0x79e74, true)
            | (0x7db40, C::Al, 0x79e74, true)
            | (0x7db50, C::Al, 0x79e74, true)
            | (0x7db60, C::Al, 0x79e74, true)
            | (0x7db6c, C::Eq, 0x7dc30, false)
            | (0x7db84, C::Al, 0x7a048, true)
            | (0x7db98, C::Al, 0x7a048, true)
            | (0x7dba0, C::Ne, 0x7dbc0, false)
            | (0x7dbbc, C::Al, 0x7a048, true)
            | (0x7dbc4, C::Ne, 0x7dbe4, false)
            | (0x7dbe0, C::Al, 0x7a048, true)
            | (0x7dbf4, C::Al, 0x7a048, true)
            | (0x7dc04, C::Al, 0x7a048, true)
            | (0x7dc10, C::Eq, 0x7dc48, false)
            | (0x7dc20, C::Eq, 0x7dc48, false)
            | (0x7dc2c, C::Al, 0x7dc50, false)
            | (0x7dc44, C::Al, 0x7dc60, false)
            | (0x7dc54, C::Al, 0x7a048, true)
            | (0x7dc64, C::Al, 0x7a048, true)
            | (0x7dc78, C::Ne, 0x7dcfc, false)
            | (0x7dc98, C::Eq, 0x7dd10, false)
            | (0x7dca8, C::Eq, 0x7dd10, false)
            | (0x7dccc, C::Ne, 0x7df60, false)
            | (0x7dcd4, C::Eq, 0x7dce4, false)
            | (0x7dce0, C::Ne, 0x7df60, false)
            | (0x7dce8, C::Eq, 0x7df5c, false)
            | (0x7dcf8, C::Al, 0x7df60, false)
            | (0x7dd0c, C::Al, 0x17b0, false)
            | (0x7dd18, C::Ne, 0x7df4c, false)
            | (0x7dd74, C::Ne, 0x7dd64, false)
            | (0x7dd9c, C::Ne, 0x7dd58, false)
            | (0x7ddb0, C::Ne, 0x7dd4c, false)
            | (0x7ddc8, C::Ne, 0x7dd3c, false)
            | (0x7de64, C::Ne, 0x7de54, false)
            | (0x7dea0, C::Ne, 0x7de90, false)
            | (0x7decc, C::Ne, 0x7de44, false)
            | (0x7df34, C::Al, 0x205c, true)
            | (0x7df58, C::Al, 0x7df64, false)
        );
        if !valid { return Err("invalid VPX branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn restore(&mut self, condition: C, registers: u16) -> Result<(), String> {
        let valid = match (self.at, condition) {
            (0x7dc6c, C::Al) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit()),
            (0x7df68, C::Al) => registers == (r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit()),
            _ => false,
        };
        if !valid { return Err("invalid VPX restore".into()); }
        self.emit(transfer_multiple(condition, true, R::SP, registers,
            BlockMode::IncrementAfter, true)?)
    }

    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        let valid = match self.at {
            0x7dcfc => rd == r(2) && target == 0x7df6c,
            0x7dd20 => rd == r(1) && target == 0x7df70,
            0x7ddd4 => rd == r(4) && target == 0x7df74,
            0x7dedc => rd == r(1) && target == 0x7df78,
            _ => false,
        };
        if !valid { return Err("invalid VPX literal load".into()); }
        self.emit(transfer_word(C::Al, true, rd, R::PC, (target - self.at - 8) as i32)?)
    }

    fn signed_halfword_postindexed(&mut self, condition: C, rd: R, rn: R, offset: i32) -> Result<(), String> {
        if self.at != 0x7d98c || condition != C::Al || rd != r(1) || rn != r(6) || offset != 2 {
            return Err("invalid VPX signed halfword postindex".into());
        }
        self.emit(load_signed_halfword_postindexed(condition, rd, rn, offset)?)
    }

    fn data(&mut self, at: usize, value: u32) -> Result<(), String> {
        self.check(at)?;
        if !matches!((at, value), (0x7df6c, 0xf4751) | (0x7df70, 0xf430b)
            | (0x7df74, 0xf472b) | (0x7df78, 0x80808080)) {
            return Err("invalid VPX pool word".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }
}

fn write_frame_header(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7d834)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7d838)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x18)?)?;
    c.check(0x7d83c)?;
    c.emit(mov_register(C::Al, r(8), r(1)))?;
    c.check(0x7d840)?;
    c.emit(mov_register(C::Al, r(4), r(0)))?;
    c.check(0x7d844)?;
    c.emit(mov_register(C::Al, r(9), r(2)))?;
    c.check(0x7d848)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x20)?)?;
    c.check(0x7d84c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d850)?;
    c.branch(C::Ne, 0x7d924, false)?;
    c.check(0x7d854)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.check(0x7d858)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x14)?)?;
    c.check(0x7d85c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x1)?)?;
    c.check(0x7d860)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0xc)?)?;
    c.check(0x7d864)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x9d)?)?;
    c.check(0x7d868)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0)?)?;
    c.check(0x7d86c)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.check(0x7d870)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x14)?)?;
    c.check(0x7d874)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x1)?)?;
    c.check(0x7d878)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0xc)?)?;
    c.check(0x7d87c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d880)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0)?)?;
    c.check(0x7d884)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.check(0x7d888)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x14)?)?;
    c.check(0x7d88c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(2), r(0), 0x1)?)?;
    c.check(0x7d890)?;
    c.emit(transfer_word(C::Al, false, r(2), r(4), 0xc)?)?;
    c.check(0x7d894)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2a)?)?;
    c.check(0x7d898)?;
    c.emit(transfer_byte_register(C::Al, false, r(2), r(1), r(0), Shift::Lsl, 0)?)?;
    c.check(0x7d89c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xc)?)?;
    c.check(0x7d8a0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.check(0x7d8a4)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x14)?)?;
    c.check(0x7d8a8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7d8ac)?;
    c.emit(transfer_word(C::Al, false, r(3), r(4), 0xc)?)?;
    c.check(0x7d8b0)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7d8b4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x4c)?)?;
    c.check(0x7d8b8)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xc)?)?;
    c.check(0x7d8bc)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x14)?)?;
    c.check(0x7d8c0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7d8c4)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 8)?)?;
    c.check(0x7d8c8)?;
    c.emit(transfer_word(C::Al, false, r(3), r(4), 0xc)?)?;
    c.check(0x7d8cc)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7d8d0)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xc)?)?;
    c.check(0x7d8d4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x50)?)?;
    c.check(0x7d8d8)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x14)?)?;
    c.check(0x7d8dc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7d8e0)?;
    c.emit(transfer_word(C::Al, false, r(3), r(4), 0xc)?)?;
    c.check(0x7d8e4)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7d8e8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x50)?)?;
    c.check(0x7d8ec)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0xc)?)?;
    c.check(0x7d8f0)?;
    c.emit(transfer_word(C::Al, true, r(2), r(4), 0x14)?)?;
    c.check(0x7d8f4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(1), 0x1)?)?;
    c.check(0x7d8f8)?;
    c.emit(mov_shifted_register(C::Al, r(0), r(0), Shift::Lsr, 8)?)?;
    c.check(0x7d8fc)?;
    c.emit(transfer_word(C::Al, false, r(3), r(4), 0xc)?)?;
    c.check(0x7d900)?;
    c.emit(transfer_byte_register(C::Al, false, r(0), r(2), r(1), Shift::Lsl, 0)?)?;
    c.check(0x7d904)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d908)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d90c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d910)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d914)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d918)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d91c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d920)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d924)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x1d2)?)?;
    c.check(0x7d928)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d92c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d930)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d934)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x1d2)?)?;
    c.check(0x7d938)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d93c)?;
    c.branch(C::Eq, 0x7dab4, false)?;
    c.check(0x7d940)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d944)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d948)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d94c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d950)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d954)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d958)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d95c)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d960)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d964)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7d968)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d96c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7d970)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d974)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(8), 0xe)?)?;
    c.check(0x7d978)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d97c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d980)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d984)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d988)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0xc)?)?;
    c.check(0x7d98c)?;
    c.signed_halfword_postindexed(C::Al, r(1), r(6), 2)?;
    c.check(0x7d990)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7)?)?;
    c.check(0x7d994)?;
    c.emit(data_register(C::Al, Op::Sub, r(5), r(1), r(0), Shift::Lsl, 0, true)?)?;
    c.check(0x7d998)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d99c)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7d9a0)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r(1), r(5), 0x0)?)?;
    c.check(0x7d9a4)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d9a8)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(5), Shift::Lsr, 31)?)?;
    c.check(0x7d9ac)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d9b0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d9b4)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d9b8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7d9bc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x4)?)?;
    c.check(0x7d9c0)?;
    c.branch(C::Ne, 0x7d978, false)?;
    c.check(0x7d9c4)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7d9c8)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x4c8)?)?;
    c.check(0x7d9cc)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7d9d0)?;
    c.branch(C::Eq, 0x7da38, false)?;
    c.check(0x7d9d4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0xcc)?)?;
    c.check(0x7d9d8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x0)?)?;
    c.check(0x7d9dc)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(6), r(6), 0x400)?)?;
    c.check(0x7d9e0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7d9e4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7d9e8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7d9ec)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7d9f0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7d9f4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x1cf)?)?;
    c.check(0x7d9f8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7d9fc)?;
    c.emit(transfer_word_register(C::Al, true, r(0), r(0), r(6), Shift::Lsl, 0)?)?;
    c.check(0x7da00)?;
    c.emit(data_register(C::Al, Op::Sub, r(5), r(0), r(1), Shift::Lsl, 0, true)?)?;
    c.check(0x7da04)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7da08)?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.check(0x7da0c)?;
    c.emit(data_immediate(C::Mi, Op::Rsb, r(1), r(5), 0x0)?)?;
    c.check(0x7da10)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7da14)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(5), Shift::Lsr, 31)?)?;
    c.check(0x7da18)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7da1c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7da20)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7da24)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x1)?)?;
    c.check(0x7da28)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x4)?)?;
    c.check(0x7da2c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(7), 0x4)?)?;
    c.check(0x7da30)?;
    c.branch(C::Ne, 0x7d9e0, false)?;
    c.check(0x7da34)?;
    c.branch(C::Al, 0x7da58, false)?;
    c.check(0x7da38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7da3c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7da40)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7da44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7da48)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7da4c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7da50)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x4)?)?;
    c.check(0x7da54)?;
    c.branch(C::Ne, 0x7da3c, false)?;
    c.check(0x7da58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0xe2)?)?;
    c.check(0x7da5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7da60)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x400)?)?;
    c.check(0x7da64)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7da68)?;
    c.emit(transfer_halfword_register(C::Al, true, r(0), r(0), r(7))?)?;
    c.check(0x7da6c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0xff)?)?;
    c.check(0x7da70)?;
    c.branch(C::Ne, 0x7da84, false)?;
    c.check(0x7da74)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7da78)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7da7c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7da80)?;
    c.branch(C::Al, 0x7daa4, false)?;
    c.check(0x7da84)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7da88)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7da8c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7da90)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7da94)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7da98)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x8)?)?;
    c.check(0x7da9c)?;
    c.emit(load_signed_halfword_register(C::Al, r(1), r(0), r(7))?)?;
    c.check(0x7daa0)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7daa4)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7daa8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7daac)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x3)?)?;
    c.check(0x7dab0)?;
    c.branch(C::Ne, 0x7da64, false)?;
    c.check(0x7dab4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x1ce)?)?;
    c.check(0x7dab8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dabc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7dac0)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7dac4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x1cf)?)?;
    c.check(0x7dac8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dacc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x6)?)?;
    c.check(0x7dad0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dad4)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0x1d0)?)?;
    c.check(0x7dad8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dadc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x3)?)?;
    c.check(0x7dae0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dae4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dae8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7daec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7daf0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7daf4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7daf8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7dafc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7db00)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7db04)?;
    c.emit(transfer_byte(C::Al, true, r(1), r(8), 0xc)?)?;
    c.check(0x7db08)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x7)?)?;
    c.check(0x7db10)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7db14)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db18)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7db1c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7db20)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7db24)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db28)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7db2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7db30)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7db34)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db38)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7db3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7db40)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7db44)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db48)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7db4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7db50)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7db54)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db58)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7db5c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x80)?)?;
    c.check(0x7db60)?;
    c.branch(C::Al, 0x79e74, true)?;
    c.check(0x7db64)?;
    c.emit(transfer_word(C::Al, true, r(0), r(8), 0x20)?)?;
    c.check(0x7db68)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7db6c)?;
    c.branch(C::Eq, 0x7dc30, false)?;
    c.check(0x7db70)?;
    c.emit(transfer_word(C::Al, true, r(7), r(8), 0x3b4)?)?;
    c.check(0x7db74)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db78)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7db7c)?;
    c.emit(data_immediate(C::Al, Op::And, r(5), r(7), 0x2)?)?;
    c.check(0x7db80)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(5), Shift::Lsr, 1)?)?;
    c.check(0x7db84)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7db88)?;
    c.emit(data_immediate(C::Al, Op::And, r(6), r(7), 0x4)?)?;
    c.check(0x7db8c)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7db90)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7db94)?;
    c.emit(mov_shifted_register(C::Al, r(1), r(6), Shift::Lsr, 2)?)?;
    c.check(0x7db98)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7db9c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x0)?)?;
    c.check(0x7dba0)?;
    c.branch(C::Ne, 0x7dbc0, false)?;
    c.check(0x7dba4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.check(0x7dba8)?;
    c.emit(data_immediate(C::Al, Op::Tst, r(0), r(7), 0x10)?)?;
    c.check(0x7dbac)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7dbb0)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(0), r(7), Shift::Lsr, 4, false)?)?;
    c.check(0x7dbb4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dbb8)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7dbbc)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dbc0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x0)?)?;
    c.check(0x7dbc4)?;
    c.branch(C::Ne, 0x7dbe4, false)?;
    c.check(0x7dbc8)?;
    c.emit(data_immediate_with_flags(C::Al, Op::And, r(0), r(7), 0x80, true)?)?;
    c.check(0x7dbcc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x2)?)?;
    c.check(0x7dbd0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x2)?)?;
    c.check(0x7dbd4)?;
    c.emit(data_register(C::Al, Op::And, r(1), r(0), r(7), Shift::Lsr, 7, false)?)?;
    c.check(0x7dbd8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dbdc)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7dbe0)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dbe4)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dbe8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7dbec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7dbf0)?;
    c.emit(data_immediate(C::Al, Op::And, r(7), r(7), 0x1)?)?;
    c.check(0x7dbf4)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dbf8)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dbfc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7dc00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7dc04)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dc08)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x3b1)?)?;
    c.check(0x7dc0c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7dc10)?;
    c.branch(C::Eq, 0x7dc48, false)?;
    c.check(0x7dc14)?;
    c.emit(transfer_word(C::Al, true, r(0), r(9), 0x838)?)?;
    c.check(0x7dc18)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(0), 0x4e0)?)?;
    c.check(0x7dc1c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7dc20)?;
    c.branch(C::Eq, 0x7dc48, false)?;
    c.check(0x7dc24)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dc28)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7dc2c)?;
    c.branch(C::Al, 0x7dc50, false)?;
    c.check(0x7dc30)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(8), 0x3b1)?)?;
    c.check(0x7dc34)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0x0)?)?;
    c.check(0x7dc38)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dc3c)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7dc40)?;
    c.emit(data_immediate(C::Ne, Op::Mov, r(1), r(0), 0x1)?)?;
    c.check(0x7dc44)?;
    c.branch(C::Al, 0x7dc60, false)?;
    c.check(0x7dc48)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dc4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x0)?)?;
    c.check(0x7dc50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7dc54)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dc58)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.check(0x7dc5c)?;
    c.emit(mov_register(C::Al, r(1), r(7)))?;
    c.check(0x7dc60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 0x1)?)?;
    c.check(0x7dc64)?;
    c.branch(C::Al, 0x7a048, true)?;
    c.check(0x7dc68)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0xc)?)?;
    c.check(0x7dc6c)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x7dc70)
}

fn initialize_or_recycle_tables(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7dc70)?;
    c.emit(transfer_word(C::Al, true, r(2), r(1), 0x4)?)?;
    c.check(0x7dc74)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1000)?)?;
    c.check(0x7dc78)?;
    c.branch(C::Ne, 0x7dcfc, false)?;
    c.check(0x7dc7c)?;
    c.emit(push_registers(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(14).bit())?)?;
    c.check(0x7dc80)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), r(13), 0x1c)?)?;
    c.check(0x7dc84)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(13), 0x14)?)?;
    c.check(0x7dc88)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(0), 0x0)?)?;
    c.check(0x7dc8c)?;
    c.emit(transfer_word(C::Al, true, r(8), r(1), 0x5b0)?)?;
    c.check(0x7dc90)?;
    c.emit(transfer_word(C::Al, true, r(5), r(1), 0x5a4)?)?;
    c.check(0x7dc94)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7dc98)?;
    c.branch(C::Eq, 0x7dd10, false)?;
    c.check(0x7dc9c)?;
    c.emit(transfer_word(C::Al, true, r(2), r(1), 0x838)?)?;
    c.check(0x7dca0)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(2), 0x4e0)?)?;
    c.check(0x7dca4)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7dca8)?;
    c.branch(C::Eq, 0x7dd10, false)?;
    c.check(0x7dcac)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0x14)?)?;
    c.check(0x7dcb0)?;
    c.emit(transfer_word(C::Al, true, r(3), r(0), 0x18)?)?;
    c.check(0x7dcb4)?;
    c.emit(transfer_word(C::Al, true, r(4), r(1), 0x5a8)?)?;
    c.check(0x7dcb8)?;
    c.emit(transfer_word(C::Al, true, r(6), r(1), 0x5ac)?)?;
    c.check(0x7dcbc)?;
    c.emit(transfer_word(C::Al, false, r(2), r(0), 0x18)?)?;
    c.check(0x7dcc0)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(3), r(8), Shift::Lsl, 0, false)?)?;
    c.check(0x7dcc4)?;
    c.emit(transfer_word(C::Al, true, r(7), r(0), 0x1c)?)?;
    c.check(0x7dcc8)?;
    c.emit(transfer_word(C::Al, false, r(7), r(0), 0x14)?)?;
    c.check(0x7dccc)?;
    c.branch(C::Ne, 0x7df60, false)?;
    c.check(0x7dcd0)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(5), Shift::Lsl, 0, false)?)?;
    c.check(0x7dcd4)?;
    c.branch(C::Eq, 0x7dce4, false)?;
    c.check(0x7dcd8)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(5), Shift::Lsl, 0, false)?)?;
    c.check(0x7dcdc)?;
    c.emit(mov_register(C::Al, r(3), r(5)))?;
    c.check(0x7dce0)?;
    c.branch(C::Ne, 0x7df60, false)?;
    c.check(0x7dce4)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(7), r(4), Shift::Lsl, 0, false)?)?;
    c.check(0x7dce8)?;
    c.branch(C::Eq, 0x7df5c, false)?;
    c.check(0x7dcec)?;
    c.emit(data_register(C::Al, Op::Cmp, r(0), r(2), r(4), Shift::Lsl, 0, false)?)?;
    c.check(0x7dcf0)?;
    c.emit(mov_register(C::Al, r(3), r(4)))?;
    c.check(0x7dcf4)?;
    c.emit(mov_register(C::Eq, r(3), r(6)))?;
    c.check(0x7dcf8)?;
    c.branch(C::Al, 0x7df60, false)?;
    c.check(0x7dcfc)?;
    c.literal(r(2), 0x7df6c)?;
    c.check(0x7dd00)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x9e)?)?;
    c.check(0x7dd04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7dd08)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(1), r(1), 0x200)?)?;
    c.check(0x7dd0c)?;
    c.branch(C::Al, 0x17b0, false)?;
    c.check(0x7dd10)?;
    c.emit(transfer_byte(C::Al, true, r(2), r(1), 0x5d4)?)?;
    c.check(0x7dd14)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x0)?)?;
    c.check(0x7dd18)?;
    c.branch(C::Ne, 0x7df4c, false)?;
    c.check(0x7dd1c)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x0)?)?;
    c.check(0x7dd20)?;
    c.literal(r(1), 0x7df70)?;
    c.check(0x7dd24)?;
    c.emit(transfer_word(C::Al, false, r(5), r(13), 0x8)?)?;
    c.check(0x7dd28)?;
    c.emit(transfer_word(C::Al, false, r(0), r(13), 0xc)?)?;
    c.check(0x7dd2c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7dd30)?;
    c.emit(mov_register(C::Al, r(14), r(8)))?;
    c.check(0x7dd34)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.check(0x7dd38)?;
    c.emit(transfer_word(C::Al, false, r(8), r(13), 0x4)?)?;
    c.check(0x7dd3c)?;
    c.emit(mov_register(C::Al, r(7), r(1)))?;
    c.check(0x7dd40)?;
    c.emit(mov_register(C::Al, r(4), r(14)))?;
    c.check(0x7dd44)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x0)?)?;
    c.check(0x7dd48)?;
    c.emit(transfer_word(C::Al, false, r(1), r(13), 0x10)?)?;
    c.check(0x7dd4c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7dd50)?;
    c.emit(mov_register(C::Al, r(9), r(7)))?;
    c.check(0x7dd54)?;
    c.emit(mov_register(C::Al, r(10), r(4)))?;
    c.check(0x7dd58)?;
    c.emit(mov_register(C::Al, r(1), r(9)))?;
    c.check(0x7dd5c)?;
    c.emit(mov_register(C::Al, r(2), r(10)))?;
    c.check(0x7dd60)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x0)?)?;
    c.check(0x7dd64)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(12), r(1), 0x1)?)?;
    c.check(0x7dd68)?;
    c.emit(data_immediate(C::Al, Op::Add, r(8), r(8), 0x1)?)?;
    c.check(0x7dd6c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(8), 0xb)?)?;
    c.check(0x7dd70)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(12), r(2), 0x1)?)?;
    c.check(0x7dd74)?;
    c.branch(C::Ne, 0x7dd64, false)?;
    c.check(0x7dd78)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(0), r(10), 0xb)?)?;
    c.check(0x7dd7c)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7dd80)?;
    c.emit(data_immediate(C::Al, Op::Add, r(9), r(9), 0xb)?)?;
    c.check(0x7dd84)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 0x3)?)?;
    c.check(0x7dd88)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 0x2)?)?;
    c.check(0x7dd8c)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 0x1)?)?;
    c.check(0x7dd90)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(10), 0x4)?)?;
    c.check(0x7dd94)?;
    c.emit(data_immediate(C::Al, Op::Add, r(10), r(10), 0x5)?)?;
    c.check(0x7dd98)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x3)?)?;
    c.check(0x7dd9c)?;
    c.branch(C::Ne, 0x7dd58, false)?;
    c.check(0x7dda0)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.check(0x7dda4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(7), 0x21)?)?;
    c.check(0x7dda8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x30)?)?;
    c.check(0x7ddac)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x8)?)?;
    c.check(0x7ddb0)?;
    c.branch(C::Ne, 0x7dd4c, false)?;
    c.check(0x7ddb4)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x10)?)?;
    c.check(0x7ddb8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.check(0x7ddbc)?;
    c.emit(data_immediate(C::Al, Op::Add, r(14), r(14), 0x180)?)?;
    c.check(0x7ddc0)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x4)?)?;
    c.check(0x7ddc4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(1), r(1), 0x108)?)?;
    c.check(0x7ddc8)?;
    c.branch(C::Ne, 0x7dd3c, false)?;
    c.check(0x7ddcc)?;
    c.emit(transfer_word(C::Al, true, r(8), r(13), 0x4)?)?;
    c.check(0x7ddd0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xff)?)?;
    c.check(0x7ddd4)?;
    c.literal(r(4), 0x7df74)?;
    c.check(0x7ddd8)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 0x0)?)?;
    c.check(0x7dddc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x0)?)?;
    c.check(0x7dde0)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x601)?)?;
    c.check(0x7dde4)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x600)?)?;
    c.check(0x7dde8)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x602)?)?;
    c.check(0x7ddec)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xa2)?)?;
    c.check(0x7ddf0)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x603)?)?;
    c.check(0x7ddf4)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x65)?)?;
    c.check(0x7ddf8)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x604)?)?;
    c.check(0x7ddfc)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0xcc)?)?;
    c.check(0x7de00)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x605)?)?;
    c.check(0x7de04)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x70)?)?;
    c.check(0x7de08)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x606)?)?;
    c.check(0x7de0c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x56)?)?;
    c.check(0x7de10)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x607)?)?;
    c.check(0x7de14)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x8c)?)?;
    c.check(0x7de18)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x608)?)?;
    c.check(0x7de1c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x25)?)?;
    c.check(0x7de20)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x609)?)?;
    c.check(0x7de24)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.check(0x7de28)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(12), r(0), 0x60a)?)?;
    c.check(0x7de2c)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x5)?)?;
    c.check(0x7de30)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x4)?)?;
    c.check(0x7de34)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x3)?)?;
    c.check(0x7de38)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x2)?)?;
    c.check(0x7de3c)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x1)?)?;
    c.check(0x7de40)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x6)?)?;
    c.check(0x7de44)?;
    c.emit(data_immediate(C::Al, Op::Add, r(7), r(4), 0x9)?)?;
    c.check(0x7de48)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.check(0x7de4c)?;
    c.emit(mov_register(C::Al, r(1), r(0)))?;
    c.check(0x7de50)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(6), r(0), 0x0)?)?;
    c.check(0x7de54)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(5), r(2), 0x1)?)?;
    c.check(0x7de58)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(6), 0x1)?)?;
    c.check(0x7de5c)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(6), 0x9)?)?;
    c.check(0x7de60)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(5), r(1), 0x1)?)?;
    c.check(0x7de64)?;
    c.branch(C::Ne, 0x7de54, false)?;
    c.check(0x7de68)?;
    c.emit(mov_register(C::Al, r(1), r(0)))?;
    c.check(0x7de6c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(5), r(0), 0x9)?)?;
    c.check(0x7de70)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(12), r(1), 0x9)?)?;
    c.check(0x7de74)?;
    c.emit(data_immediate(C::Al, Op::Add, r(6), r(1), 0x7)?)?;
    c.check(0x7de78)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(1), 0x5)?)?;
    c.check(0x7de7c)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(1), 0x4)?)?;
    c.check(0x7de80)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(1), 0x3)?)?;
    c.check(0x7de84)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(1), 0x2)?)?;
    c.check(0x7de88)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(1), 0x1)?)?;
    c.check(0x7de8c)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(1), 0x6)?)?;
    c.check(0x7de90)?;
    c.emit(transfer_byte_postindexed(C::Al, true, r(1), r(7), 0x1)?)?;
    c.check(0x7de94)?;
    c.emit(data_immediate(C::Al, Op::Add, r(5), r(5), 0x1)?)?;
    c.check(0x7de98)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x13)?)?;
    c.check(0x7de9c)?;
    c.emit(transfer_byte_postindexed(C::Al, false, r(1), r(6), 0x1)?)?;
    c.check(0x7dea0)?;
    c.branch(C::Ne, 0x7de90, false)?;
    c.check(0x7dea4)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(12), r(0), 0x1a)?)?;
    c.check(0x7dea8)?;
    c.emit(data_immediate(C::Al, Op::Add, r(3), r(3), 0x1)?)?;
    c.check(0x7deac)?;
    c.emit(data_immediate(C::Al, Op::Add, r(4), r(4), 0x13)?)?;
    c.check(0x7deb0)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x5)?)?;
    c.check(0x7deb4)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x4)?)?;
    c.check(0x7deb8)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x3)?)?;
    c.check(0x7debc)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x2)?)?;
    c.check(0x7dec0)?;
    c.emit(transfer_byte(C::Al, false, r(12), r(0), 0x1)?)?;
    c.check(0x7dec4)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 0x6)?)?;
    c.check(0x7dec8)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(3), 0x2)?)?;
    c.check(0x7decc)?;
    c.branch(C::Ne, 0x7de44, false)?;
    c.check(0x7ded0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x80)?)?;
    c.check(0x7ded4)?;
    c.emit(mov_register(C::Al, r(2), r(8)))?;
    c.check(0x7ded8)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x651)?)?;
    c.check(0x7dedc)?;
    c.literal(r(1), 0x7df78)?;
    c.check(0x7dee0)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(1), r(2), 0x650)?)?;
    c.check(0x7dee4)?;
    c.emit(mov_register(C::Al, r(1), r(8)))?;
    c.check(0x7dee8)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(2), 0x3)?)?;
    c.check(0x7deec)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(2), 0x2)?)?;
    c.check(0x7def0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x0)?)?;
    c.check(0x7def4)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x65d)?)?;
    c.check(0x7def8)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x659)?)?;
    c.check(0x7defc)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(8), 0x655)?)?;
    c.check(0x7df00)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(0), r(1), 0x65c)?)?;
    c.check(0x7df04)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x3)?)?;
    c.check(0x7df08)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x2)?)?;
    c.check(0x7df0c)?;
    c.emit(mov_register(C::Al, r(1), r(8)))?;
    c.check(0x7df10)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(0), r(1), 0x658)?)?;
    c.check(0x7df14)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x3)?)?;
    c.check(0x7df18)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x2)?)?;
    c.check(0x7df1c)?;
    c.emit(mov_register(C::Al, r(1), r(8)))?;
    c.check(0x7df20)?;
    c.emit(transfer_byte_preindexed_writeback(C::Al, false, r(0), r(1), 0x654)?)?;
    c.check(0x7df24)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x3)?)?;
    c.check(0x7df28)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x2)?)?;
    c.check(0x7df2c)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.check(0x7df30)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x660)?)?;
    c.check(0x7df34)?;
    c.branch(C::Al, 0x205c, true)?;
    c.check(0x7df38)?;
    c.emit(transfer_word(C::Al, true, r(1), r(13), 0x0)?)?;
    c.check(0x7df3c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x1)?)?;
    c.check(0x7df40)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(1), 0x5d4)?)?;
    c.check(0x7df44)?;
    c.emit(transfer_word(C::Al, true, r(0), r(13), 0xc)?)?;
    c.check(0x7df48)?;
    c.emit(transfer_word(C::Al, true, r(5), r(13), 0x8)?)?;
    c.check(0x7df4c)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0x14)?)?;
    c.check(0x7df50)?;
    c.emit(transfer_word(C::Al, false, r(8), r(0), 0x18)?)?;
    c.check(0x7df54)?;
    c.emit(transfer_word(C::Al, false, r(5), r(0), 0x1c)?)?;
    c.check(0x7df58)?;
    c.branch(C::Al, 0x7df64, false)?;
    c.check(0x7df5c)?;
    c.emit(mov_register(C::Al, r(3), r(6)))?;
    c.check(0x7df60)?;
    c.emit(transfer_word(C::Al, false, r(3), r(0), 0x1c)?)?;
    c.check(0x7df64)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(13), r(11), 0x1c)?)?;
    c.check(0x7df68)?;
    c.restore(C::Al, r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit() | r(9).bit() | r(10).bit() | r(11).bit() | r(15).bit())?;
    c.check(0x7df6c)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    write_frame_header(&mut c)?;
    initialize_or_recycle_tables(&mut c)?;
    c.data(0x7df6c, 0xf4751)?;
    c.data(0x7df70, 0xf430b)?;
    c.data(0x7df74, 0xf472b)?;
    c.data(0x7df78, 0x80808080)?;
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
        assert_eq!((START, END, CODE_END), (0x7d834, 0x7df7c, 0x7df6c));
        assert_eq!(bytes.len(), 1864);
        assert_eq!((CODE_BYTES, DATA_BYTES), (1848, 16));
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!(FUNCTION_STARTS, [START, 0x7dc70]);
        assert_eq!((START..END).step_by(4).filter(|&at| TypedA32::is_code(at)).count(), 462);
        assert_eq!(EXTERNAL_TABLES, [(0xf430b, 0xf472b - 0xf430b), (0xf472b, 0xf4751 - 0xf472b)]);
        let mut c = TypedA32 { at: START, bytes: Vec::new() };
        write_frame_header(&mut c).unwrap();
        c.check(0x7dc70).unwrap();
        initialize_or_recycle_tables(&mut c).unwrap();
        c.check(CODE_END).unwrap();
        assert_eq!(c.bytes, bytes[..CODE_BYTES]);
        let mut branches = 0;
        let mut calls = 0;
        for at in (START..CODE_END).step_by(4) {
            let instruction = word(&bytes, at);
            if (instruction >> 25) & 7 == 5 {
                branches += 1;
                calls += usize::from(instruction & (1 << 24) != 0);
            }
        }
        assert_eq!((branches, calls), (70, 35));
        assert_eq!(word(&bytes, 0x7dc70), transfer_word(C::Al, true, r(2), r(1), 4).unwrap());
        assert_eq!(word(&bytes, 0x7dc74), data_immediate(C::Al, Op::Cmp, r(0), r(2), 0x1000).unwrap());
        assert_eq!(word(&bytes, 0x7dc78), branch(C::Ne, 0x7dc78, 0x7dcfc, false).unwrap());
        assert_eq!(word(&bytes, 0x7dc7c), push_registers(C::Al,
            r(4).bit() | r(5).bit() | r(6).bit() | r(7).bit() | r(8).bit()
            | r(9).bit() | r(10).bit() | r(11).bit() | R::LR.bit()).unwrap());
        assert_eq!(word(&bytes, 0x7dd0c), branch(C::Al, 0x7dd0c, 0x17b0, false).unwrap());
        for at in (0x7dcfc..0x7dd10).step_by(4) {
            assert_ne!(word(&bytes, at) & 0x0fff0000, 0x08bd0000);
        }
        for (at, value) in [(0x7d864, 0x9d), (0x7d87c, 1), (0x7d894, 0x2a)] {
            assert_eq!(word(&bytes, at), data_immediate(C::Al, Op::Mov, r(2), r(0), value).unwrap());
        }
    }

    #[test]
    fn pool_and_literal_guards() {
        let bytes = fragment().unwrap();
        let pool = [(0x7df6c, 0xf4751), (0x7df70, 0xf430b),
            (0x7df74, 0xf472b), (0x7df78, 0x80808080)];
        let loads = [(0x7dcfc, r(2), 0x7df6c), (0x7dd20, r(1), 0x7df70),
            (0x7ddd4, r(4), 0x7df74), (0x7dedc, r(1), 0x7df78)];
        for at in (START..END).step_by(4) {
            for (site, rd, target) in loads {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.literal(rd, target).is_ok(), at == site);
                if at == site { assert_eq!(c.bytes, word(&bytes, at).to_le_bytes()); }
                for (bad_rd, bad_target) in [(r(0), target), (rd, target + 4), (rd, usize::MAX)] {
                    let mut c = TypedA32 { at, bytes: Vec::new() };
                    assert!(c.literal(bad_rd, bad_target).is_err());
                    assert!(c.bytes.is_empty());
                }
            }
            for (site, value) in pool {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.data(site, value).is_ok(), at == site);
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.data(site, value ^ 1).is_err());
                assert!(c.bytes.is_empty());
            }
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.signed_halfword_postindexed(C::Al, r(1), r(6), 2).is_ok(), at == 0x7d98c);
            for (condition, rd, rn, offset) in [(C::Ne, r(1), r(6), 2),
                (C::Al, r(0), r(6), 2), (C::Al, r(1), r(5), 2), (C::Al, r(1), r(6), 4)] {
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.signed_halfword_postindexed(condition, rd, rn, offset).is_err());
                assert!(c.bytes.is_empty());
            }
        }
        for (at, value) in pool {
            assert_eq!(word(&bytes, at), value);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
        }
    }

    #[test]
    fn exact_guards() {
        let bytes = fragment().unwrap();
        let conditions = [C::Eq, C::Ne, C::Cs, C::Cc, C::Mi, C::Pl, C::Hi, C::Ls,
            C::Ge, C::Lt, C::Gt, C::Le, C::Al];
        let mut accepted_branches = 0;
        let mut accepted_restores = 0;
        for at in (START..CODE_END).step_by(4) {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            c.emit(mov_register(C::Al, r(0), r(0))).unwrap();
            assert_eq!(c.bytes.len(), 4);
            let instruction = word(&bytes, at);
            let is_branch = (instruction >> 25) & 7 == 5;
            let displacement = ((instruction << 8) as i32 >> 6) as isize;
            let target = (at as isize + 8 + displacement) as usize;
            for condition in conditions {
                for link in [false, true] {
                    let expected = is_branch && instruction >> 28 == condition as u32
                        && (instruction & (1 << 24) != 0) == link;
                    let mut c = TypedA32 { at, bytes: Vec::new() };
                    assert_eq!(c.branch(condition, target, link).is_ok(), expected);
                    if expected {
                        accepted_branches += 1;
                        assert_eq!(c.bytes, instruction.to_le_bytes());
                    } else {
                        assert!(c.bytes.is_empty());
                    }
                    for invalid in [target.wrapping_add(4), START + 1, END, usize::MAX] {
                        let mut c = TypedA32 { at, bytes: Vec::new() };
                        assert!(c.branch(condition, invalid, link).is_err());
                        assert!(c.bytes.is_empty());
                    }
                }
                let registers = instruction as u16;
                let expected = [0x7dc6c, 0x7df68].contains(&at)
                    && instruction >> 28 == condition as u32;
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.restore(condition, registers).is_ok(), expected);
                if expected {
                    accepted_restores += 1;
                    assert_eq!(c.bytes, instruction.to_le_bytes());
                } else {
                    assert!(c.bytes.is_empty());
                }
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert!(c.restore(condition, registers ^ r(0).bit()).is_err());
                assert!(c.bytes.is_empty());
            }
        }
        assert_eq!((accepted_branches, accepted_restores), (70, 2));
        for at in [START - 4, START + 1, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(c.emit(mov_register(C::Al, r(0), r(0))).is_err());
            assert!(c.branch(C::Al, 0x7a048, true).is_err());
            assert!(c.restore(C::Al, R::PC.bit()).is_err());
            assert!(c.check(START).is_err());
            assert!(c.bytes.is_empty());
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
