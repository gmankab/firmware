use arm32::{
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift, branch, bx,
    data_immediate, data_register, load_signed_byte_register, mov_register,
    mov_shifted_register, push_registers, transfer_byte, transfer_multiple,
    transfer_word, transfer_word_register,
};

pub const START: usize = 0x7d4f8;
pub const END: usize = 0x7d834;
pub const FILE_MAPPING_DELTA: usize = 0x4000;
pub const CODE_BYTES: usize = 816;
pub const DATA_BYTES: usize = 12;
pub const FUNCTION_STARTS: [usize; 4] = [START, 0x7d6a4, 0x7d72c, 0x7d7f8];

fn r(n: u8) -> R { R::new(n).unwrap() }

struct TypedA32 {
    at: usize,
    bytes: Vec<u8>,
}

impl TypedA32 {
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at != expected {
            return Err(format!("HEVC HDR metadata at {:#x}, expected {expected:#x}", self.at));
        }
        Ok(())
    }

    fn is_code(at: usize) -> bool {
        (START..END).contains(&at) && !(0x7d698..0x7d6a4).contains(&at) && at % 4 == 0
    }

    fn emit(&mut self, instruction: u32) -> Result<(), String> {
        if !Self::is_code(self.at) { return Err("HDR instruction outside code".into()); }
        self.bytes.extend_from_slice(&instruction.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn branch(&mut self, condition: C, target: usize, link: bool) -> Result<(), String> {
        let valid = matches!((self.at, condition, target, link),
            (0x7d530, C::Al, 0x79b6c, true)
            | (0x7d540, C::Al, 0x7c2e8, true)
            | (0x7d558, C::Al, 0x798f8, true)
            | (0x7d560, C::Cc, 0x7d580, false)
            | (0x7d570, C::Al, 0x798f8, true)
            | (0x7d57c, C::Hi, 0x7d564, false)
            | (0x7d58c, C::Al, 0x798f8, true)
            | (0x7d59c, C::Al, 0x79c18, true)
            | (0x7d5a4, C::Al, 0x79c98, true)
            | (0x7d5b8, C::Eq, 0x7d600, false)
            | (0x7d5f8, C::Al, 0x51d08, true)
            | (0x7d5fc, C::Al, 0x7d624, false)
            | (0x7d614, C::Al, 0x2ec94, true)
            | (0x7d620, C::Al, 0x2ec94, true)
            | (0x7d650, C::Al, 0x798f8, true)
            | (0x7d65c, C::Al, 0x79c18, true)
            | (0x7d664, C::Al, 0x79c98, true)
            | (0x7d694, C::Al, 0x79b34, false)
            | (0x7d6c4, C::Al, 0x79df0, true)
            | (0x7d6d4, C::Eq, 0x7d6fc, false)
            | (0x7d6e8, C::Ne, 0x7d6fc, false)
            | (0x7d6f8, C::Al, 0x798f8, true)
            | (0x7d704, C::Eq, 0x7d714, false)
            | (0x7d710, C::Al, 0x79db8, true)
            | (0x7d71c, C::Al, 0x79c18, true)
            | (0x7d728, C::Al, 0x79c98, false)
            | (0x7d758, C::Al, 0x79b6c, true)
            | (0x7d768, C::Al, 0x798f8, true)
            | (0x7d778, C::Al, 0x798f8, true)
            | (0x7d788, C::Al, 0x798f8, true)
            | (0x7d798, C::Al, 0x798f8, true)
            | (0x7d7a8, C::Al, 0x798f8, true)
            | (0x7d7b8, C::Al, 0x798f8, true)
            | (0x7d7c4, C::Al, 0x79c18, true)
            | (0x7d7cc, C::Al, 0x79c98, true)
            | (0x7d7f4, C::Al, 0x79b34, false)
        );
        if !valid { return Err("invalid HDR branch".into()); }
        self.emit(branch(condition, self.at as u32, target as u32, link)?)
    }

    fn literal(&mut self, condition: C, rd: R, target: usize) -> Result<(), String> {
        if !matches!((self.at, condition, rd.number(), target),
            (0x7d600, C::Al, 1, 0x7d698)
            | (0x7d604, C::Al, 2, 0x7d69c)
            | (0x7d618, C::Al, 1, 0x7d6a0)) {
            return Err("invalid HDR literal load".into());
        }
        self.emit(transfer_word(condition, true, rd, R::PC, (target - self.at - 8) as i32)?)
    }

    fn pool(&mut self, value: u32) -> Result<(), String> {
        if !matches!((self.at, value), (0x7d698, 0xf3562)
            | (0x7d69c, 0xf429b) | (0x7d6a0, 0xe2788)) {
            return Err("invalid HDR literal pool".into());
        }
        self.bytes.extend_from_slice(&value.to_le_bytes());
        self.at += 4;
        Ok(())
    }

    fn restore(&mut self, condition: C, registers: u16) -> Result<(), String> {
        let valid = match (self.at, condition) {
            (0x7d674, C::Eq) => registers == 0x8ff0,
            (0x7d690, C::Al) => registers == 0x4ff0,
            (0x7d724, C::Al) => registers == 0x4830,
            (0x7d7d8, C::Eq) => registers == 0x8bf0,
            (0x7d7f0, C::Al) => registers == 0x4bf0,
            _ => false,
        };
        if !valid { return Err("invalid HDR restore".into()); }
        self.emit(transfer_multiple(condition, true, R::SP, registers, BlockMode::IncrementAfter, true)?)
    }
}

fn write_hdr10plus(c: &mut TypedA32) -> Result<(), String> {
    c.check(START)?;
    c.emit(push_registers(C::Al, 0x4ff0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 28)?)?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, R::SP, 28)?)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(7), r(0), 0x11c)?)?;
    c.emit(mov_register(C::Al, r(10), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(8), r(4), 0x20)?)?;
    c.emit(transfer_byte(C::Al, false, r(0), r(4), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(7), r(7), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0xab8)?)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(7))?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x79b6c, true)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 0x27)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(mov_register(C::Al, r(2), r(4)))?;
    c.branch(C::Al, 0x7c2e8, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x258)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 4)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 8)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(0), 0)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0x100)?)?;
    c.branch(C::Cc, 0x7d580, false)?;
    c.check(0x7d564)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 8)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(data_immediate(C::Al, Op::Sub, r(5), r(5), 0xff)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(5), 0xff)?)?;
    c.branch(C::Hi, 0x7d564, false)?;
    c.check(0x7d580)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(mov_register(C::Al, r(1), r(5)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 8)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(9), r(0), 0)?)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x79c98, true)?;
    c.emit(transfer_word(C::Al, true, r(12), r(6), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(10), 0x258)?)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(2), 0)?)?;
    c.branch(C::Eq, 0x7d600, false)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0x1800)?)?;
    c.emit(mov_register(C::Al, R::LR, r(7)))?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(12), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(0), r(0), 4)?)?;
    c.emit(transfer_word_register(C::Al, true, r(3), r(6), r(1), Shift::Lsl, 0)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(4), 0x20)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(6), 4)?)?;
    c.emit(transfer_word(C::Al, true, r(7), r(6), 0x284)?)?;
    c.emit(transfer_word(C::Al, false, r(9), R::SP, 0)?)?;
    c.emit(transfer_multiple(C::Al, false, R::SP, r(3).bit() | r(5).bit() | r(7).bit(), BlockMode::IncrementBefore, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 4)?)?;
    c.emit(transfer_word(C::Al, false, r(9), R::SP, 16)?)?;
    c.emit(mov_register(C::Al, r(7), R::LR))?;
    c.emit(transfer_word(C::Al, false, r(9), R::SP, 20)?)?;
    c.emit(data_immediate(C::Eq, Op::Mov, r(3), r(0), 5)?)?;
    c.branch(C::Al, 0x51d08, true)?;
    c.branch(C::Al, 0x7d624, false)?;
    c.check(0x7d600)?;
    c.literal(C::Al, r(1), 0x7d698)?;
    c.literal(C::Al, r(2), 0x7d69c)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0x26)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 8)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(3), r(3), 0x500)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.literal(C::Al, r(1), 0x7d6a0)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 8)?)?;
    c.branch(C::Al, 0x2ec94, true)?;
    c.check(0x7d624)?;
    c.emit(transfer_word(C::Al, true, r(1), r(10), 0x258)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(1), 0)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 0, false)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x20)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x24)?)?;
    c.emit(data_register(C::Al, Op::Add, r(0), r(0), r(1), Shift::Lsl, 3, false)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(4), 0x24)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x79c98, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x3ed)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(data_immediate(C::Eq, Op::Sub, R::SP, r(11), 28)?)?;
    c.restore(C::Eq, 0x8ff0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0xab8)?)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(7))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(8), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r(0), r(8)))?;
    c.emit(data_immediate(C::Al, Op::Sub, R::SP, r(11), 28)?)?;
    c.restore(C::Al, 0x4ff0)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(0x7d698)?;
    c.pool(0xf3562)?;
    c.pool(0xf429b)?;
    c.pool(0xe2788)?;
    c.check(0x7d6a4)
}

fn write_picture_metadata(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7d6a4)?;
    c.emit(push_registers(C::Al, 0x4830)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 8)?)?;
    c.emit(mov_register(C::Al, r(5), r(0)))?;
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 1)?)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x2fa)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(1), r(0), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x79df0, true)?;
    c.emit(transfer_word(C::Al, true, r(0), r(5), 0x838)?)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x2c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.branch(C::Eq, 0x7d6fc, false)?;
    c.emit(transfer_word(C::Al, true, r(1), r(0), 0x38)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(1), 0)?)?;
    c.emit(transfer_word(C::Ne, true, r(0), r(0), 0x14)?)?;
    c.emit(data_immediate(C::Ne, Op::Cmp, r(0), r(0), 1)?)?;
    c.branch(C::Ne, 0x7d6fc, false)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.check(0x7d6fc)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(5), 0x14c)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.branch(C::Eq, 0x7d714, false)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.branch(C::Al, 0x79db8, true)?;
    c.check(0x7d714)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.restore(C::Al, 0x4830)?;
    c.branch(C::Al, 0x79c98, false)?;
    c.check(0x7d72c)
}

fn write_access_unit_metadata(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7d72c)?;
    c.emit(push_registers(C::Al, 0x4bf0)?)?;
    c.emit(data_immediate(C::Al, Op::Add, r(11), R::SP, 24)?)?;
    c.emit(mov_register(C::Al, r(6), r(0)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(8), r(0), 0x11c)?)?;
    c.emit(mov_register(C::Al, r(4), r(2)))?;
    c.emit(mov_register(C::Al, r(7), r(1)))?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(8), r(8), 0x400)?)?;
    c.emit(transfer_word(C::Al, true, r(5), r(4), 0x20)?)?;
    c.emit(load_signed_byte_register(C::Al, r(1), r(0), r(8))?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x79b6c, true)?;
    for (value, bits) in [(0, 1), (0x23, 6), (0, 6), (1, 3)] {
        c.emit(mov_register(C::Al, r(0), r(4)))?;
        c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), value)?)?;
        c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), bits)?)?;
        c.branch(C::Al, 0x798f8, true)?;
    }
    c.check(0x7d79c)?;
    c.emit(transfer_word(C::Al, true, r(1), r(7), 0x20)?)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 3)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 1)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(2), r(0), 1)?)?;
    c.branch(C::Al, 0x798f8, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.emit(data_immediate(C::Al, Op::Mov, r(1), r(0), 0)?)?;
    c.branch(C::Al, 0x79c18, true)?;
    c.emit(mov_register(C::Al, r(0), r(4)))?;
    c.branch(C::Al, 0x79c98, true)?;
    c.emit(transfer_byte(C::Al, true, r(0), r(6), 0x16d)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.restore(C::Eq, 0x8bf0)?;
    c.emit(transfer_word(C::Al, true, r(0), r(6), 0x838)?)?;
    c.emit(load_signed_byte_register(C::Al, r(2), r(0), r(8))?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(4), 0x20)?)?;
    c.emit(data_register(C::Al, Op::Sub, r(1), r(0), r(5), Shift::Lsl, 0, false)?)?;
    c.emit(mov_register(C::Al, r(0), r(5)))?;
    c.restore(C::Al, 0x4bf0)?;
    c.branch(C::Al, 0x79b34, false)?;
    c.check(0x7d7f8)
}

fn initialize_metadata(c: &mut TypedA32) -> Result<(), String> {
    c.check(0x7d7f8)?;
    c.emit(transfer_word(C::Al, true, r(2), r(0), 0x3bc)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(3), r(0), 0)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(12), r(0), 1)?)?;
    for offset in [0, 4, 8] {
        c.emit(transfer_word(C::Al, false, r(3), r(2), offset)?)?;
    }
    c.emit(transfer_byte(C::Al, true, r(1), r(1), 0xb8)?)?;
    c.emit(transfer_word(C::Al, true, r(0), r(0), 0x20)?)?;
    c.emit(mov_shifted_register(C::Al, r(3), r(1), Shift::Lsl, 1)?)?;
    c.emit(data_immediate(C::Al, Op::Cmp, r(0), r(0), 0)?)?;
    c.emit(data_register(C::Ne, Op::Orr, r(3), r(12), r(1), Shift::Lsl, 1, false)?)?;
    c.emit(data_immediate(C::Al, Op::Orr, r(0), r(3), 16)?)?;
    c.emit(transfer_word(C::Al, false, r(0), r(2), 12)?)?;
    c.emit(data_immediate(C::Al, Op::Mov, r(0), r(0), 15)?)?;
    c.emit(bx(C::Al, R::LR))?;
    c.check(END)
}

pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = TypedA32 { at: START, bytes: Vec::with_capacity(END - START) };
    write_hdr10plus(&mut c)?;
    write_picture_metadata(&mut c)?;
    write_access_unit_metadata(&mut c)?;
    initialize_metadata(&mut c)?;
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
    fn structure_and_actual_counts() {
        let bytes = fragment().unwrap();
        assert_eq!(END - START, 828);
        assert_eq!(bytes.len(), CODE_BYTES + DATA_BYTES);
        assert_eq!((CODE_BYTES, DATA_BYTES), (816, 12));
        assert_eq!(FUNCTION_STARTS, [0x7d4f8, 0x7d6a4, 0x7d72c, 0x7d7f8]);
        let mut counts = (0, 0, 0, 0, 0);
        for at in (START..END).step_by(4) {
            if !TypedA32::is_code(at) { counts.1 += 4; continue; }
            counts.0 += 4;
            let instruction = word(&bytes, at);
            if (instruction >> 25) & 7 != 5 { continue; }
            let target = (at as isize + 8 + (((instruction << 8) as i32 >> 6) as isize)) as usize;
            if instruction & (1 << 24) != 0 {
                counts.2 += 1;
                assert!([0x79b6c, 0x7c2e8, 0x798f8, 0x79c18, 0x79c98,
                    0x51d08, 0x2ec94, 0x79df0, 0x79db8].contains(&target));
            } else if (START..END).contains(&target) {
                counts.3 += 1;
                assert!(TypedA32::is_code(target));
            } else {
                counts.4 += 1;
                assert!([(0x7d694, 0x79b34), (0x7d728, 0x79c98),
                    (0x7d7f4, 0x79b34)].contains(&(at, target)));
            }
        }
        assert_eq!(counts, (816, 12, 26, 7, 3));
        for (at, value) in [(0x7d698, 0xf3562), (0x7d69c, 0xf429b), (0x7d6a0, 0xe2788)] {
            assert_eq!(word(&bytes, at), value);
        }
        assert_eq!(word(&bytes, 0x7d5b8), branch(C::Eq, 0x7d5b8, 0x7d600, false).unwrap());
        for (at, rd, target) in [(0x7d600, r(1), 0x7d698),
            (0x7d604, r(2), 0x7d69c), (0x7d618, r(1), 0x7d6a0)] {
            assert_eq!(word(&bytes, at), transfer_word(C::Al, true, rd, R::PC, (target - at - 8) as i32).unwrap());
        }
        assert_eq!(word(&bytes, 0x7d614), branch(C::Al, 0x7d614, 0x2ec94, true).unwrap());
        assert_eq!(word(&bytes, 0x7d620), branch(C::Al, 0x7d620, 0x2ec94, true).unwrap());
        assert_eq!(word(&bytes, 0x7d5fc), branch(C::Al, 0x7d5fc, 0x7d624, false).unwrap());
        assert_eq!(word(&bytes, END - 4), bx(C::Al, R::LR));
    }

    #[test]
    fn exact_guards() {
        let bytes = fragment().unwrap();
        let conditions = [C::Eq, C::Ne, C::Cs, C::Cc, C::Mi, C::Pl,
            C::Hi, C::Ls, C::Ge, C::Lt, C::Gt, C::Le, C::Al];
        for at in (START..END).step_by(4) {
            let instruction = word(&bytes, at);
            let code = TypedA32::is_code(at);
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert_eq!(c.emit(mov_register(C::Al, r(0), r(0))).is_ok(), code);
            let target = (at as isize + 8 + (((instruction << 8) as i32 >> 6) as isize)) as usize;
            for condition in conditions {
                for link in [false, true] {
                    for candidate in [target, target.wrapping_add(4), START + 1, END] {
                        let mut c = TypedA32 { at, bytes: Vec::new() };
                        let expected = code && (instruction >> 25) & 7 == 5
                            && candidate == target && condition as u32 == instruction >> 28
                            && link == (instruction & (1 << 24) != 0);
                        assert_eq!(c.branch(condition, candidate, link).is_ok(), expected);
                        assert_eq!(c.at, at + if expected { 4 } else { 0 });
                        assert_eq!(c.bytes.len(), if expected { 4 } else { 0 });
                    }
                }
                for rd in [r(0), r(1), r(2), R::PC] {
                    for target in [0x7d698, 0x7d69c, 0x7d6a0, 0x7d6a4] {
                        let expected = matches!((at, condition, rd.number(), target),
                            (0x7d600, C::Al, 1, 0x7d698) | (0x7d604, C::Al, 2, 0x7d69c)
                            | (0x7d618, C::Al, 1, 0x7d6a0));
                        let mut c = TypedA32 { at, bytes: Vec::new() };
                        assert_eq!(c.literal(condition, rd, target).is_ok(), expected);
                        assert_eq!(c.bytes.len(), if expected { 4 } else { 0 });
                    }
                }
                for registers in [0x8ff0, 0x4ff0, 0x4830, 0x8bf0, 0x4bf0, 0] {
                    let expected = matches!((at, condition, registers),
                        (0x7d674, C::Eq, 0x8ff0) | (0x7d690, C::Al, 0x4ff0)
                        | (0x7d724, C::Al, 0x4830) | (0x7d7d8, C::Eq, 0x8bf0)
                        | (0x7d7f0, C::Al, 0x4bf0));
                    let mut c = TypedA32 { at, bytes: Vec::new() };
                    assert_eq!(c.restore(condition, registers).is_ok(), expected);
                    assert_eq!(c.bytes.len(), if expected { 4 } else { 0 });
                }
            }
            for value in [0xf3562, 0xf429b, 0xe2788, 0] {
                let expected = matches!((at, value), (0x7d698, 0xf3562)
                    | (0x7d69c, 0xf429b) | (0x7d6a0, 0xe2788));
                let mut c = TypedA32 { at, bytes: Vec::new() };
                assert_eq!(c.pool(value).is_ok(), expected);
                assert_eq!(c.bytes.len(), if expected { 4 } else { 0 });
            }
        }
        for at in [START - 4, START + 1, 0x7d699, END - 1, END, END + 4] {
            let mut c = TypedA32 { at, bytes: Vec::new() };
            assert!(!TypedA32::is_code(at));
            assert!(c.emit(bx(C::Al, R::LR)).is_err());
            assert!(c.branch(C::Al, 0x79b34, false).is_err());
            assert!(c.literal(C::Al, r(1), 0x7d698).is_err());
            assert!(c.pool(0xf3562).is_err());
            assert!(c.restore(C::Al, 0x4ff0).is_err());
            assert_eq!(c.at, at);
            assert!(c.bytes.is_empty());
        }
        assert!(TypedA32 { at: START, bytes: Vec::new() }.check(END).is_err());
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
