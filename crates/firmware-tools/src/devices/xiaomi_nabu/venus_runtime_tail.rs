// SPDX-License-Identifier: GPL-3.0-only
//! Venus ARM32 runtime continuation: fault reports and mapping-state checks.
//! Every instruction uses the checked ARM A32 encoder; literal-pool entries
//! are addresses/data, not verbatim copied machine instructions.
//! PC-relative addresses refer to the start of the executable ELF segment.
use arm32::{
    branch, data_immediate, data_register, mov_register, mov_shifted_register,
    push_registers, transfer_multiple, transfer_word, transfer_word_register,
    BlockMode, Condition as C, DataOp as Op, Register as R, Shift,
};

const RUNTIME_START_VADDR: usize = 0x1000;

pub(super) fn put(bytes: &mut Vec<u8>, at: usize, word: u32) -> Result<(), String> {
    let local = at.checked_sub(RUNTIME_START_VADDR)
        .ok_or_else(|| "Venus ARM32 address precedes runtime".to_string())?;
    if bytes.len() != local {
        return Err(format!("Venus continuation gap at 0x{at:04x}: got 0x{:04x}", bytes.len() + RUNTIME_START_VADDR));
    }
    bytes.extend_from_slice(&word.to_le_bytes());
    Ok(())
}

pub(super) fn call(at: usize, target: usize) -> Result<u32, String> {
    branch(C::Al, at as u32, target as u32, true)
}
pub(super) fn jump(condition: C, at: usize, target: usize) -> Result<u32, String> {
    branch(condition, at as u32, target as u32, false)
}
pub(super) fn mov(rd: R, value: u32) -> Result<u32, String> {
    data_immediate(C::Al, Op::Mov, rd, R::R0, value)
}
pub(super) fn add(rd: R, rn: R, value: u32) -> Result<u32, String> {
    data_immediate(C::Al, Op::Add, rd, rn, value)
}
pub(super) fn sub(rd: R, rn: R, value: u32) -> Result<u32, String> {
    data_immediate(C::Al, Op::Sub, rd, rn, value)
}
pub(super) fn cmp(reg: R, value: u32) -> Result<u32, String> {
    data_immediate(C::Al, Op::Cmp, R::R0, reg, value)
}
pub(super) fn load(dst: R, base: R, offset: i32) -> Result<u32, String> {
    transfer_word(C::Al, true, dst, base, offset)
}
pub(super) fn store(src: R, base: R, offset: i32) -> Result<u32, String> {
    transfer_word(C::Al, false, src, base, offset)
}

pub(super) fn extend(bytes: &mut Vec<u8>) -> Result<(), String> {
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;

    // Fault-record logger: preserve the three argument registers and caller's
    // return address, construct the error record and enter diagnostics.
    put(bytes, 0x17b0, push_registers(C::Al, r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    put(bytes, 0x17b4, add(R::R11, R::SP, 16)?)?;
    put(bytes, 0x17b8, sub(R::SP, R::SP, 16)?)?;
    put(bytes, 0x17bc, cmp(R::R0, 0)?)?;
    put(bytes, 0x17c0, mov_register(C::Al, r4, R::LR))?;
    put(bytes, 0x17c4, mov_register(C::Al, r6, R::R2))?;
    put(bytes, 0x17c8, mov_register(C::Al, r5, R::R1))?;
    put(bytes, 0x17cc, data_immediate(C::Ne, Op::Sub, R::SP, R::R11, 16)?)?;
    put(bytes, 0x17d0, transfer_multiple(C::Ne, true, R::SP,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?)?;
    put(bytes, 0x17d4, call(0x17d4, 0x1734)?)?;
    for (at, dst) in [(0x17d8, R::R0), (0x17dc, R::R2), (0x17e0, r3)] {
        put(bytes, at, load(dst, R::PC, (0x1834_i32 + ((at - 0x17d8) as i32 / 4)*4) - at as i32 - 8)?)?;
    }
    put(bytes, 0x17e4, mov(R::R1, 1024)?)?;
    put(bytes, 0x17e8, store(r6, R::SP, 0)?)?;
    put(bytes, 0x17ec, store(r5, R::SP, 4)?)?;
    put(bytes, 0x17f0, store(r4, R::SP, 8)?)?;
    put(bytes, 0x17f4, call(0x17f4, 0x31c0)?)?;
    put(bytes, 0x17f8, call(0x17f8, 0x1fcc)?)?;
    put(bytes, 0x17fc, load(R::R1, R::PC, 0x3c)?)?;
    put(bytes, 0x1800, load(R::R2, R::PC, 0x3c)?)?;
    put(bytes, 0x1804, mov(r3, 245)?)?;
    put(bytes, 0x1808, mov(R::R0, 8)?)?;
    put(bytes, 0x180c, store(r6, R::SP, 0)?)?;
    put(bytes, 0x1810, store(r5, R::SP, 4)?)?;
    put(bytes, 0x1814, store(r4, R::SP, 8)?)?;
    put(bytes, 0x1818, data_immediate(C::Al, Op::Orr, r3, r3, 256)?)?;
    put(bytes, 0x181c, call(0x181c, 0x2ec94)?)?;
    put(bytes, 0x1820, load(R::R1, R::PC, 0x20)?)?;
    put(bytes, 0x1824, mov(R::R0, 8)?)?;
    put(bytes, 0x1828, call(0x1828, 0x2ec94)?)?;
    put(bytes, 0x182c, call(0x182c, 0x1398)?)?;
    put(bytes, 0x1830, jump(C::Al, 0x1830, 0x1830)?)?;
    for (at, diagnostic_label) in [
        (0x1834, 0x0010_3084), (0x1838, 0x000e_278a),
        (0x183c, 0x000e_24a8), (0x1840, 0x000e_27a2),
        (0x1844, 0x000e_27c2), (0x1848, 0x000e_2788),
    ] { put(bytes, at, diagnostic_label)?; }

    // Nonreturning simplified diagnostic path.
    put(bytes, 0x184c, push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    put(bytes, 0x1850, mov_register(C::Al, R::R11, R::SP))?;
    put(bytes, 0x1854, sub(R::SP, R::SP, 8)?)?;
    put(bytes, 0x1858, mov(R::R0, 0)?)?;
    put(bytes, 0x185c, load(R::R2, R::PC, 0x20)?)?;
    put(bytes, 0x1860, add(r3, R::SP, 4)?)?;
    put(bytes, 0x1864, mov(R::R1, 1024)?)?;
    put(bytes, 0x1868, store(R::R0, R::SP, 4)?)?;
    put(bytes, 0x186c, load(R::R0, R::PC, 0x0c)?)?;
    put(bytes, 0x1870, call(0x1870, 0x31c0)?)?;
    put(bytes, 0x1874, call(0x1874, 0x1fcc)?)?;
    put(bytes, 0x1878, call(0x1878, 0x1398)?)?;
    put(bytes, 0x187c, jump(C::Al, 0x187c, 0x187c)?)?;
    put(bytes, 0x1880, 0x0010_3084)?;
    put(bytes, 0x1884, 0x000e_27ce)?;

    // ARM32 domain/mapping-state guard. Explicit branches retain their A32
    // addresses, and two table values at the end remain typed data operands.
    put(bytes, 0x1888, push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    put(bytes, 0x188c, add(R::R11, R::SP, 8)?)?;
    put(bytes, 0x1890, load(R::R1, R::PC, 0xf4)?)?;
    for (at, dst, offset) in [(0x1894,R::R12,0x2c),(0x1898,r3,0x30),(0x189c,R::R2,0x30)] {
        put(bytes, at, load(dst, R::R1, offset)?)?;
    }
    put(bytes, 0x18a0, load(R::LR, R::PC, 0xe8)?)?;
    let tst = |rn, rm| data_register(C::Al, Op::Tst, R::R0, rn, rm, Shift::Lsl, 0, false);
    put(bytes, 0x18a4, tst(R::R2,R::LR)?)?;
    put(bytes, 0x18a8, jump(C::Eq,0x18a8,0x18c4)?)?;
    put(bytes, 0x18ac, load(R::R0,R::R1,0x14)?)?;
    put(bytes, 0x18b0, data_immediate(C::Al,Op::Orr,R::R0,R::R0,4)?)?;
    put(bytes, 0x18b4, store(R::R0,R::R1,0x14)?)?;
    put(bytes, 0x18b8, mov(R::R0,1)?)?;
    put(bytes, 0x18bc, store(R::R0,R::R1,-0x20)?)?;
    put(bytes, 0x18c0, jump(C::Al,0x18c0,0x18c0)?)?;
    put(bytes, 0x18c4, load(R::R2,R::R1,0x30)?)?;
    put(bytes, 0x18c8, cmp(R::R2,0)?)?;
    put(bytes, 0x18cc, jump(C::Eq,0x18cc,0x18ac)?)?;
    put(bytes, 0x18d0, load(R::R2,R::R1,0x2c)?)?;
    put(bytes, 0x18d4, tst(R::R2,R::LR)?)?;
    put(bytes, 0x18d8, jump(C::Ne,0x18d8,0x18ac)?)?;
    put(bytes, 0x18dc, mov_shifted_register(C::Al,R::R2,r3,Shift::Lsr,20)?)?;
    put(bytes, 0x18e0, load(r3,R::R1,0x2c)?)?;
    put(bytes, 0x18e4, cmp(r3,0)?)?;
    put(bytes, 0x18e8, jump(C::Eq,0x18e8,0x18ac)?)?;
    put(bytes, 0x18ec, cmp(R::R2,0)?)?;
    put(bytes, 0x18f0, jump(C::Eq,0x18f0,0x1914)?)?;
    put(bytes, 0x18f4, data_register(C::Al,Op::Add,r4,R::R2,R::R12,Shift::Lsr,20,false)?)?;
    put(bytes, 0x18f8, mov_shifted_register(C::Al,R::R2,R::R12,Shift::Lsr,20)?)?;
    put(bytes, 0x18fc, transfer_word_register(C::Al,true,r3,R::R0,R::R2,Shift::Lsl,2)?)?;
    put(bytes, 0x1900, data_immediate(C::Al,Op::Bic,r3,r3,12)?)?;
    put(bytes, 0x1904, transfer_word_register(C::Al,false,r3,R::R0,R::R2,Shift::Lsl,2)?)?;
    put(bytes, 0x1908, add(R::R2,R::R2,1)?)?;
    put(bytes, 0x190c, data_register(C::Al,Op::Cmp,R::R0,R::R2,r4,Shift::Lsl,0,false)?)?;
    put(bytes, 0x1910, jump(C::Cc,0x1910,0x18fc)?)?;
    put(bytes, 0x1914, load(R::R2,R::R1,0x10)?)?;
    put(bytes, 0x1918, data_immediate(C::Al,Op::Tst,R::R0,R::R2,2)?)?;
    put(bytes, 0x191c, jump(C::Ne,0x191c,0x1924)?)?;
    let return_regs = r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit();
    put(bytes, 0x1920, transfer_multiple(C::Al,true,R::SP,return_regs,BlockMode::IncrementAfter,true)?)?;
    put(bytes, 0x1924, load(R::R12,R::R1,0)?)?;
    put(bytes, 0x1928, load(r3,R::R1,4)?)?;
    put(bytes, 0x192c, load(R::R2,R::R1,4)?)?;
    put(bytes, 0x1930, tst(R::R2,R::LR)?)?;
    put(bytes, 0x1934, jump(C::Ne,0x1934,0x18ac)?)?;
    put(bytes, 0x1938, load(R::R2,R::R1,4)?)?;
    put(bytes, 0x193c, cmp(R::R2,0)?)?;
    put(bytes, 0x1940, jump(C::Eq,0x1940,0x18ac)?)?;
    put(bytes, 0x1944, load(R::R2,R::R1,0)?)?;
    put(bytes, 0x1948, tst(R::R2,R::LR)?)?;
    put(bytes, 0x194c, jump(C::Ne,0x194c,0x18ac)?)?;
    put(bytes, 0x1950, mov_shifted_register(C::Al,R::R2,r3,Shift::Lsr,20)?)?;
    put(bytes, 0x1954, load(r3,R::R1,0)?)?;
    put(bytes, 0x1958, cmp(r3,0)?)?;
    put(bytes, 0x195c, jump(C::Eq,0x195c,0x18ac)?)?;
    put(bytes, 0x1960, cmp(R::R2,0)?)?;
    put(bytes, 0x1964, transfer_multiple(C::Eq,true,R::SP,return_regs,BlockMode::IncrementAfter,true)?)?;
    put(bytes, 0x1968, data_register(C::Al,Op::Add,R::R1,R::R2,R::R12,Shift::Lsr,20,false)?)?;
    put(bytes, 0x196c, mov_shifted_register(C::Al,R::R2,R::R12,Shift::Lsr,20)?)?;
    put(bytes, 0x1970, transfer_word_register(C::Al,true,r3,R::R0,R::R2,Shift::Lsl,2)?)?;
    put(bytes, 0x1974, data_immediate(C::Al,Op::Bic,r3,r3,12)?)?;
    put(bytes, 0x1978, transfer_word_register(C::Al,false,r3,R::R0,R::R2,Shift::Lsl,2)?)?;
    put(bytes, 0x197c, add(R::R2,R::R2,1)?)?;
    put(bytes, 0x1980, data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    put(bytes, 0x1984, jump(C::Cc,0x1984,0x1970)?)?;
    put(bytes, 0x1988, jump(C::Al,0x1988,0x1920)?)?;
    put(bytes, 0x198c, 0xe00d_2038)?; // Platform domain-state address
    put(bytes, 0x1990, 0x000f_ffff)?; // Address-mask constant
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_noncontiguous_extensions() {
        assert!(extend(&mut Vec::new()).is_err());
    }
    #[test]
    fn second_runtime_extension_is_contiguous() {
        let mut output = vec![0; 0x7b0];
        extend(&mut output).unwrap();
        assert_eq!(output.len(), 0x994);
    }
}
