// SPDX-License-Identifier: GPL-3.0-only
//! Independently authored, single-function Hexagon privileged state update.
//!
//! Physical Hexagon address `0x96c00d40` maps to CDSP MBN file offset
//! `0x3d40`, within the first executable PT_LOAD. This 44-byte function is
//! **disjoint** from the separately rebuilt 68-byte entry prefix. The 0xd40
//! offset means "bytes from physical entry", not a fake contiguous extension.
use super::cdsp_boot::{self, GeneralRegister as Gpr, ScalarPacket as Hexagon, SystemRegister};

pub const FILE_OFFSET: usize = 0x3d40;
pub const PHYSICAL_ADDRESS: u32 = 0x96c0_0d40;
pub const FUNCTION_SIZE: usize = 11 * 4;

/// Restore configured status-register flags and then jump to the saved link
/// register after subtracting the relocation base. This describes the emitted
/// instruction syntax; execution on the real controller is not yet tested.
pub fn build_syscfg_reset() -> Result<Vec<u8>, String> {
    let r15 = Gpr::new(15)?;
    let lr = Gpr::new(31)?;
    let r25 = Gpr::new(25)?;
    let mut code = Vec::with_capacity(FUNCTION_SIZE);
    let source = [
        Hexagon::ReadSyscfg { dst: r15 },
        Hexagon::ClearBit { dst: r15, src: r15, position: 1 },
        Hexagon::WriteSystem { system: SystemRegister::Syscfg, src: r15 },
        Hexagon::Isync,
        Hexagon::Ickill,
        Hexagon::Isync,
        Hexagon::OrSmallImmediate { dst: r15, src: r15, value: 3 },
        Hexagon::WriteSystem { system: SystemRegister::Syscfg, src: r15 },
        Hexagon::Isync,
        Hexagon::SubtractRegisters { dst: lr, left: lr, right: r25 },
        Hexagon::JumpRegister { source: lr },
    ];
    for instruction in source {
        code.extend_from_slice(&instruction.encode()?.to_le_bytes());
    }
    if code.len() != FUNCTION_SIZE {
        return Err("Hexagon privileged reset function length is not 44".into());
    }
    Ok(code)
}

pub fn verify_syscfg_reset(reference: &[u8]) -> Result<(), String> {
    cdsp_boot::validate_reference(reference)?;
    let generated = build_syscfg_reset()?;
    let target = reference.get(FILE_OFFSET..FILE_OFFSET+FUNCTION_SIZE)
        .ok_or("CDSP reference missing syscfg reset interval")?;
    if let Some((at, (a, b))) = target.iter().zip(&generated).enumerate().find(|(_, (a, b))| a != b) {
        return Err(format!("Hexagon privileged reset byte mismatch at file {:#x}: reference {a:02x}, built {b:02x}", FILE_OFFSET+at));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn syscfg_reset_fragment_has_exact_function_boundaries() {
        let code=build_syscfg_reset().unwrap();
        assert_eq!(code.len(),FUNCTION_SIZE);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+0xd40);
        for instruction in code.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(instruction.try_into().unwrap()) & 0xc000,0xc000);
        }
    }
}
