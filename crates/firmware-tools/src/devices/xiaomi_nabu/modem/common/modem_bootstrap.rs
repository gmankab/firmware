// SPDX-License-Identifier: GPL-3.0-only
//! Modem reset/trap vectors and early Hexagon system initialization.
//!
//! This is a standalone 68-byte function-level reconstruction of the first
//! executable PT_LOAD. It is **not** a complete modem image and does **not**
//! include the following memory-map table at file offset 0x3044.
//! ELF entry physical address: 0x8e000000; first RX segment ELF virtual
//! address: 0xc0800000; source file offset: 0x3000.
use super::modem_hexagon::{encode_single, CacheCommand, Operation, SpecialReg};

pub const ELF_OFFSET: usize = 0x3000;
pub const PHYSICAL_ADDR: u32 = 0x8e00_0000;
pub const VIRTUAL_ADDR: u32 = 0xc080_0000;
pub const CODE_LENGTH: usize = 0x44;

const RESET_HANDLER: u32 = PHYSICAL_ADDR + 0x001c;
const EXCEPTION_HANDLER: u32 = PHYSICAL_ADDR + 0x111c;
const NEXT_BOOTSTRAP: u32 = PHYSICAL_ADDR + 0x05c4;

/// Typed original operations in code order; each packet has exactly one
/// instruction and ends explicitly. Branches are relative to the current
/// instruction address, not to the ELF file start or next instruction.
pub fn operations() -> Vec<Operation> {
    use Operation::*;
    let mut ops = Vec::with_capacity(CODE_LENGTH / 4);
    ops.push(Jump {offset_bytes: (RESET_HANDLER - PHYSICAL_ADDR) as i32});
    // Seven more vector slots: six trap self-loops and a dispatch into the
    // later bootstrap. Explicitly keep all eight vectors in the emitted code.
    for _ in 1..7 { ops.push(Jump {offset_bytes: 0}); }
    ops.push(Jump {offset_bytes: (EXCEPTION_HANDLER - (PHYSICAL_ADDR + 0x1c)) as i32});
    ops.push(ClearRegister {dst: 0});
    ops.push(WriteSystem {dst: SpecialReg::Ssr, src: 0});
    ops.push(ReadSystem {dst: 0, src: SpecialReg::Syscfg});
    ops.push(SetBit {dst: 0, src: 0, index: 3});
    ops.push(SetBit {dst: 0, src: 0, index: 5});
    ops.push(WriteSystem {dst: SpecialReg::Syscfg, src: 0});
    ops.push(Cache(CacheCommand::DataKill));
    ops.push(Cache(CacheCommand::InstructionKill));
    ops.push(Jump {offset_bytes: (NEXT_BOOTSTRAP - (PHYSICAL_ADDR + 0x40)) as i32});
    assert_eq!(ops.len(), CODE_LENGTH / 4);
    ops
}

pub fn rebuild() -> Result<Vec<u8>, String> {
    let mut code = Vec::with_capacity(CODE_LENGTH);
    for op in operations() {
        code.extend_from_slice(&encode_single(op)?.to_le_bytes());
    }
    if code.len() != CODE_LENGTH {
        return Err("modem bootstrap instruction count is incorrect".into());
    }
    Ok(code)
}

pub fn verify(reference: &[u8]) -> Result<(), String> {
    let src = reference.get(ELF_OFFSET..ELF_OFFSET + CODE_LENGTH)
        .ok_or_else(|| "modem reference is shorter than bootstrap interval".to_owned())?;
    let rebuild = rebuild()?;
    if src != rebuild {
        let i = src.iter().zip(&rebuild).position(|(a,b)| a!=b).unwrap();
        return Err(format!("modem bootstrap mismatch at file 0x{:x}: expected 0x{:02x}, got 0x{:02x}",
            ELF_OFFSET+i,src[i],rebuild[i]));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vectors_are_eight_aligned_slots_followed_by_boot_setup() {
        let ops=operations();
        assert_eq!(ops.len(),17);
        assert!(matches!(ops[0],Operation::Jump {offset_bytes:0x1c}));
        assert!(matches!(ops[7],Operation::Jump {offset_bytes:0x1100}));
        assert!(matches!(ops[16],Operation::Jump {offset_bytes:0x584}));
        assert_eq!(rebuild().unwrap().len(),CODE_LENGTH);
    }
    #[test]
    fn truncated_reference_fails_closed() {
        assert!(verify(&[]).is_err());
        assert!(verify(&vec![0;ELF_OFFSET + CODE_LENGTH - 1]).is_err());
    }
}
