// SPDX-License-Identifier: GPL-3.0-only
//! Upper-ILM boundary continuation of a routine that begins in Agent 17's
//! lower-ILM ownership. The first MOVI instruction straddles the reservation
//! boundary: only its two *high-owner* bytes are emitted by Agent 18.
//!
//! This code is an executable NDS32 instruction reconstruction, not copied
//! OEM payload, and preserves the calling function's original POP25 return.
use super::novatek_upper_isa::{Gpr::*, Instruction, Instruction::*};

pub const FILE_START: u32 = 0x10000;
pub const FILE_END: u32 = 0x10016;
pub const SOURCE_INSTRUCTION_START: u32 = 0xfffe;
pub const OWNED_BYTES: usize = (FILE_END-FILE_START) as usize;
pub const INSTRUCTION_COUNT: usize = 7;

/// Instruction PCs are *firmware file offsets*: subtract 0x200 for runtime.
/// The MOVI starts at file 0xfffe and its two initial bytes belong to Agent 17.
const CODE: &[(u32, Instruction)] = &[
    (0xfffe, Movi(R6, 0x20104)),
    (0x10002, AddSlli(R6, R6, R0, 1)),
    (0x10006, Lhsi(R0, R6, 0)),
    (0x1000a, Jal(0xffaa)),
    (0x1000e, Lhi333(R0, R6, 0)),
    (0x10010, ShiGp(R0, -5494)),
    (0x10014, Pop25(R6, 0)),
];

pub fn emit() -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(OWNED_BYTES);
    let mut expected = SOURCE_INSTRUCTION_START;
    for &(file, instruction) in CODE {
        if file != expected {
            return Err(format!("cross-boundary instruction gap at file {expected:#x}"));
        }
        let bytes = instruction.encode(file)?;
        if file < FILE_START {
            if file + bytes.len() as u32 != FILE_START + 2 || bytes.len() != 4 {
                return Err("expected exactly two upper-owned MOVI bytes".into());
            }
            result.extend_from_slice(&bytes[2..]);
        } else {
            result.extend_from_slice(&bytes);
        }
        expected = file + bytes.len() as u32;
    }
    if expected != FILE_END || result.len() != OWNED_BYTES || CODE.len() != INSTRUCTION_COUNT {
        return Err("upper-ILM ownership seam is misaligned or missing bytes".into());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundary_is_crossed_by_movi_but_never_claims_lower_bytes() {
        assert_eq!(CODE[0].0,0xfffe);
        let movi=CODE[0].1.encode(CODE[0].0).unwrap();
        assert_eq!(movi[0],0x44);
        assert_eq!(movi[1],0x62);
        assert_eq!(movi[2..],emit().unwrap()[..2]);
        assert_eq!(emit().unwrap().len(),22);
    }
    #[test]
    fn checked_branch_resolves_to_a_lower_ilm_routine() {
        let (file,instruction)=CODE[3];
        assert_eq!(file,0x1000a);
        assert_eq!(instruction.encode(file).unwrap(),[0x49,0xff,0xff,0xd0]);
        assert_eq!(CODE.last().unwrap().0,0x10014);
    }
}
