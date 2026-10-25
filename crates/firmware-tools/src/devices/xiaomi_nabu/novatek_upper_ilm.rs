// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-18: upper-ILM routines reconstructed as typed NDS32 operations.
//!
//! Instruction addresses below are firmware FILE offsets. The pinned image
//! maps ILM file 0x200 to runtime 0; runtime address = file offset - 0x200.
//! No reference firmware is read by the emitter. Names describe observed
//! operations, not proven hardware behavior. EX9 indices refer to an external
//! instruction table until that table has its own verified reconstruction.

use super::novatek_upper_isa::{Gpr::*, Instruction, Instruction::*};

pub const RESERVED_START: u32 = 0x10000;
pub const RESERVED_END: u32 = 0x1ac80;
pub const SOURCE_START: u32 = 0x10016;
pub const SOURCE_END: u32 = 0x101b6;
pub const ILM_FILE_BIAS: u32 = 0x200;

pub const CODE: &[(u32, Instruction)] = &[
    // Save the caller context and select an input mode; EX9 entries remain explicit dependencies.
    (0x10016, Push25(R10, 8)),
    (0x10018, Mov55(R6, R0)),
    (0x1001a, LwiGp(R0, -82992)),
    (0x1001e, Swi37Sp(R0, 4)),
    (0x10020, Ex9It(205)),
    (0x10022, Ex9It(203)),
    (0x10024, Mov55(R7, R1)),
    (0x10026, Mov55(R9, R2)),
    (0x10028, Beq(R5, R0, 0x101aa)),
    (0x1002c, Bnez38(R6, 0x10034)),
    (0x1002e, Sbi333(R6, R2, 0)),
    (0x10030, Movi55(R7, 1)),
    (0x10032, J8(0x10050)),
    (0x10034, Bnec(R6, 4, 0x10046)),
    (0x10038, Mov55(R0, R1)),
    (0x1003a, Jal(0xfffc)),
    (0x1003e, Sbi(R7, R9, 0)),
    (0x10042, Movi55(R7, 3)),
    (0x10044, J8(0x10050)),
    (0x10046, Addri36Sp(R0, 4)),
    (0x10048, Lb(R6, R0, R1, 0)),
    (0x1004c, Sbi333(R1, R2, 0)),
    (0x1004e, Movi55(R7, 3)),
    // Initialize all 32 bytes of the channel-code workspace at SRAM 0x2dc58.
    (0x10050, Movi55(R0, 0)),
    (0x10052, Movi(R1, 0x2dc58)),
    (0x10056, Sb(R6, R1, R0, 0)),
    (0x1005a, Addi45(R0, 1)),
    (0x1005c, Movi(R9, 0x2dc58)),
    (0x10060, Bnec(R0, 0x20, 0x10056)),
    (0x10064, Jal(0xff6c)),
    (0x10068, LwiGp(R6, -5340)),
    // Obtain controller pointers, normalize status bytes, and write descriptor fields.
    (0x1006c, Beqz38(R0, 0x1008c)),
    (0x1006e, Lwi(R0, R6, 0x50)),
    (0x10072, Movi(R1, 0x203c0)),
    (0x10076, Srli45(R0, 0x10)),
    (0x10078, Ex9It(68)),
    (0x1007a, Ex9It(77)),
    (0x1007c, LwiGp(R1, 0xe68)),
    (0x10080, Addi45(R1, 5)),
    (0x10082, Zeb33(R1, R1)),
    (0x10084, Sbi333(R1, R0, 4)),
    (0x10086, Sbi333(R1, R0, 5)),
    (0x10088, Sbi333(R1, R0, 6)),
    (0x1008a, Sbi333(R1, R0, 7)),
    (0x1008c, Jal(0xff48)),
    (0x10090, Beqz38(R0, 0x1015c)),
    (0x10092, Addi(R8, R6, 0x50)),
    (0x10096, Lwi450(R0, R8)),
    (0x10098, Movi(R1, 0x20420)),
    (0x1009c, Srli45(R0, 0x10)),
    (0x1009e, Ex9It(68)),
    (0x100a0, Ex9It(77)),
    (0x100a2, Ex9It(458)),
    (0x100a4, LwiGp(R2, -5376)),
    (0x100a8, Addi45(R1, 5)),
    (0x100aa, Zeb33(R1, R1)),
    (0x100ac, Sbi333(R1, R0, 4)),
    (0x100ae, LwiGp(R1, 0xe70)),
    (0x100b2, Movi(R3, 0x39780)),
    (0x100b6, Addi45(R1, 5)),
    (0x100b8, Zeb33(R1, R1)),
    (0x100ba, Sbi333(R1, R0, 6)),
    (0x100bc, Lwi450(R0, R8)),
    (0x100be, Movi(R1, 0x20360)),
    (0x100c2, Srli45(R0, 0x10)),
    (0x100c4, Ex9It(68)),
    (0x100c6, Ex9It(77)),
    (0x100c8, LwiGp(R1, 0x2b8)),
    (0x100cc, Movi55(R4, 8)),
    (0x100ce, Addi45(R1, 5)),
    (0x100d0, Zeb33(R1, R1)),
    (0x100d2, Sbi333(R1, R0, 4)),
    (0x100d4, LwiGp(R1, 0x2c0)),
    (0x100d8, Movi55(R5, 2)),
    (0x100da, Addi45(R1, 5)),
    (0x100dc, Zeb33(R1, R1)),
    (0x100de, Sbi333(R1, R0, 5)),
    (0x100e0, LwiGp(R1, 0x2bc)),
    (0x100e4, Movi55(R16, 9)),
    (0x100e6, Addi45(R1, 5)),
    (0x100e8, Zeb33(R1, R1)),
    (0x100ea, Sbi333(R1, R0, 6)),
    (0x100ec, LwiGp(R1, 0x2c4)),
    (0x100f0, Addi45(R1, 5)),
    (0x100f2, Zeb33(R1, R1)),
    (0x100f4, Sbi333(R1, R0, 7)),
    (0x100f6, Lwi450(R0, R8)),
    (0x100f8, Movi(R1, 0x203c0)),
    (0x100fc, Srli45(R0, 0x10)),
    (0x100fe, Ex9It(68)),
    (0x10100, Ex9It(77)),
    (0x10102, LwiGp(R1, 0xe68)),
    (0x10106, Movi55(R6, 0xb)),
    (0x10108, Addi45(R1, 5)),
    (0x1010a, Zeb33(R1, R1)),
    (0x1010c, Sbi333(R1, R0, 4)),
    (0x1010e, Sbi333(R1, R0, 5)),
    (0x10110, Sbi333(R1, R0, 6)),
    (0x10112, Sbi333(R1, R0, 7)),
    // Map each bounded input channel code to the selected output value.
    (0x10114, Movi55(R0, 0)),
    (0x10116, Lwi(R1, R2, 0x20)),
    (0x1011a, Fexti33(R1, 5)),
    (0x1011c, Slts45(R0, R1)),
    (0x1011e, Beqzs8(0x101a0)),
    (0x10120, Add333(R1, R0, R3)),
    (0x10122, Lbi333(R1, R1, 0)),
    (0x10124, Zeb33(R1, R1)),
    (0x10126, Slti45(R1, 0x14)),
    (0x10128, Beqzs8(0x1013a)),
    (0x1012a, Slti45(R1, 0xc)),
    (0x1012c, Beqzs8(0x1014c)),
    (0x1012e, Beqz38(R1, 0x10152)),
    (0x10130, Bnec(R1, 0xa, 0x10156)),
    (0x10134, Sb(R16, R9, R0, 0)),
    (0x10138, J8(0x10156)),
    (0x1013a, Beqc(R1, 0x14, 0x10146)),
    (0x1013e, Addi(R15, R1, -32)),
    (0x10142, Slt45(R5, R15)),
    (0x10144, Bnezs8(0x10156)),
    (0x10146, Sb(R4, R9, R0, 0)),
    (0x1014a, J8(0x10156)),
    (0x1014c, Sb(R6, R9, R0, 0)),
    (0x10150, J8(0x10156)),
    (0x10152, Sb(R7, R9, R0, 0)),
    (0x10156, Addi45(R0, 1)),
    (0x10158, Zeb33(R0, R0)),
    (0x1015a, J8(0x10116)),
    // Handle the alternate channel selections and update GP-relative status bytes.
    (0x1015c, Ex9It(109)),
    (0x1015e, Beq(R6, R0, 0x10168)),
    (0x10162, Ex9It(152)),
    (0x10164, Bne(R6, R0, 0x101a0)),
    (0x10168, Movi55(R1, 8)),
    (0x1016a, SbiGp(R1, 0x278)),
    (0x1016e, Movi55(R1, 0xa)),
    (0x10170, SbiGp(R1, 0x27a)),
    (0x10174, SbiGp(R1, 0x284)),
    (0x10178, LbiGp(R1, 0x13d2)),
    (0x1017c, Movi(R2, 0x2e1f4)),
    (0x10180, Lb(R2, R2, R1, 0)),
    (0x10184, Movi(R0, 0x2dc58)),
    (0x10188, Bnec(R2, 9, 0x10194)),
    (0x1018c, Movi55(R2, 2)),
    (0x1018e, Sb(R2, R0, R1, 0)),
    (0x10192, J8(0x101a0)),
    (0x10194, Sb(R7, R0, R1, 0)),
    (0x10198, LbiGp(R1, 0x13d3)),
    (0x1019c, Sb(R7, R0, R1, 0)),
    // Publish the workspace through the lower-ILM helper, then restore context and return.
    (0x101a0, Ex9It(80)),
    (0x101a2, Movi(R1, 0x2dc58)),
    (0x101a6, Jal(0xfdee)),
    (0x101aa, Pop25(R10, 8)),
    // Separate leaf helper: store two supplied byte values and return through LP.
    (0x101ac, Movi(R1, 0x38d40)),
    (0x101b0, Sbi333(R0, R1, 0)),
    (0x101b2, Sbi333(R2, R3, 0)),
    (0x101b4, Ret5(R30)),
];

/// Emit the complete checked interval without any original-image input.
pub fn emit() -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity((SOURCE_END - SOURCE_START) as usize);
    let mut pc = SOURCE_START;
    for &(at, instruction) in CODE {
        if at != pc {
            return Err(format!("source boundary mismatch: expected file {pc:#x}, found {at:#x}"));
        }
        let encoded = instruction.encode(at)?;
        if encoded.len() != instruction.width() {
            return Err(format!("instruction width disagreement at {at:#x}"));
        }
        out.extend_from_slice(&encoded);
        pc = pc.checked_add(encoded.len() as u32).ok_or("instruction offset overflow")?;
    }
    if pc != SOURCE_END || pc > RESERVED_END {
        return Err(format!("source ends at {pc:#x}, expected {SOURCE_END:#x}"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_starts_at_the_next_complete_function_and_stays_in_reservation() {
        assert_eq!(SOURCE_START - RESERVED_START, 22);
        assert!(SOURCE_END <= RESERVED_END);
        assert_eq!(emit().unwrap().len(), 416);
        assert_eq!(SOURCE_START - ILM_FILE_BIAS, 0xfe16);
    }

    #[test]
    fn branch_targets_do_not_split_local_instructions() {
        for &(_, op) in CODE {
            let target = match op {
                Beq(_, _, t) | Bne(_, _, t) | Beqc(_, _, t) | Bnec(_, _, t)
                | Beqz38(_, t) | Bnez38(_, t) | Beqzs8(t) | Bnezs8(t) | J8(t) | Jal(t) => Some(t),
                _ => None,
            };
            if let Some(t) = target {
                if (SOURCE_START..SOURCE_END).contains(&t) {
                    assert!(CODE.iter().any(|(pc, _)| *pc == t), "target {t:#x} splits an instruction");
                }
            }
        }
    }
}
