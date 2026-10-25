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

/// A second, standalone 20-byte function clears the SYSCFG low bit,
/// synchronizes the instruction pipeline and returns to the caller. It is
/// disjoint from the earlier 44-byte reset routine by 20 unreconstructed
/// executable bytes; no bytes are silently filled.
pub const DISABLE_OFFSET: usize = 0x3d80;
pub const DISABLE_PHYSICAL_ADDRESS: u32 = 0x96c0_0d80;
pub const DISABLE_SIZE: usize = 5 * 4;
pub fn build_syscfg_disable() -> Result<Vec<u8>, String> {
    let r15 = Gpr::new(15)?;
    let lr = Gpr::new(31)?;
    let program = [
        Hexagon::ReadSyscfg { dst: r15 },
        Hexagon::ClearBit { dst: r15, src: r15, position: 0 },
        Hexagon::WriteSystem { system: SystemRegister::Syscfg, src: r15 },
        Hexagon::Isync,
        Hexagon::JumpRegister { source: lr },
    ];
    let mut output = Vec::with_capacity(DISABLE_SIZE);
    for instruction in program { output.extend_from_slice(&instruction.encode()?.to_le_bytes()); }
    if output.len() != DISABLE_SIZE { return Err("CDSP SYSCFG disable size mismatch".into()); }
    Ok(output)
}

pub fn verify_syscfg_disable(reference: &[u8]) -> Result<(), String> {
    cdsp_boot::validate_reference(reference)?;
    let generated=build_syscfg_disable()?;
    let found=reference.get(DISABLE_OFFSET..DISABLE_OFFSET+DISABLE_SIZE)
        .ok_or("CDSP reference missing syscfg-disable function")?;
    if let Some((i,(a,b)))=found.iter().zip(&generated).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP SYSCFG disable byte differs at file {:#x}: {a:02x} != {b:02x}",DISABLE_OFFSET+i));
    }
    Ok(())
}

/// A Hexagon relocator trampoline, occupying the previously missing space
/// between the SYSCFG reset and SYSCFG disable functions. LLVM separately
/// assembles the 3-instruction first packet (one immext pair plus addition)
/// followed by singleton addition and register-jump packets.
pub const RELOCATOR_OFFSET: usize = 0x3d6c;
pub const RELOCATOR_SIZE: usize = 5 * 4;
pub fn build_relocator_trampoline() -> Result<Vec<u8>,String> {
    use cdsp_boot::encode_extended_assignment;
    let fp=Gpr::new(30)?;
    let lr=Gpr::new(31)?;
    let relocate=Gpr::new(25)?;
    let mut data=Vec::with_capacity(RELOCATOR_SIZE);
    for word in encode_extended_assignment(fp,-0x1fff280,false) {
        data.extend_from_slice(&word.to_le_bytes());
    }
    for packet in [
        Hexagon::AddRegisters { dst:lr, left:lr, right:relocate },
        Hexagon::AddRegisters { dst:fp, left:fp, right:relocate },
        Hexagon::JumpRegister { source:fp },
    ] { data.extend_from_slice(&packet.encode()?.to_le_bytes()); }
    if data.len()!=RELOCATOR_SIZE { return Err("CDSP relocator is not 20 bytes".into()); }
    Ok(data)
}

pub fn verify_relocator_trampoline(reference: &[u8]) -> Result<(),String> {
    cdsp_boot::validate_reference(reference)?;
    let built=build_relocator_trampoline()?;
    let found=reference.get(RELOCATOR_OFFSET..RELOCATOR_OFFSET+RELOCATOR_SIZE)
        .ok_or("CDSP reference missing relocator")?;
    if let Some((i,(a,b)))=found.iter().zip(&built).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP relocator difference at {:#x}: {a:02x} != {b:02x}",RELOCATOR_OFFSET+i));
    }
    Ok(())
}

/// The reset, relocator, and disable functions now form one CONTIGUOUS
/// 84-byte Hexagon runtime interval, entirely generated from typed source.
pub const SUPERVISOR_SPAN_SIZE: usize = FUNCTION_SIZE + RELOCATOR_SIZE + DISABLE_SIZE;
pub fn build_supervisor_span() -> Result<Vec<u8>,String> {
    if FILE_OFFSET+FUNCTION_SIZE!=RELOCATOR_OFFSET ||
       RELOCATOR_OFFSET+RELOCATOR_SIZE!=DISABLE_OFFSET {
        return Err("CDSP privileged fragment adjacency violation".into());
    }
    let mut result=build_syscfg_reset()?;
    result.extend(build_relocator_trampoline()?);
    result.extend(build_syscfg_disable()?);
    if result.len()!=SUPERVISOR_SPAN_SIZE { return Err("CDSP supervisor length mismatch".into()); }
    Ok(result)
}

pub fn verify_supervisor_span(reference: &[u8]) -> Result<(),String> {
    cdsp_boot::validate_reference(reference)?;
    let actual=build_supervisor_span()?;
    let found=reference.get(FILE_OFFSET..FILE_OFFSET+SUPERVISOR_SPAN_SIZE)
        .ok_or("CDSP reference lacks supervisor interval")?;
    if let Some((i,(a,b)))=found.iter().zip(&actual).enumerate().find(|(_,(a,b))|a!=b) {
        return Err(format!("CDSP supervisor span mismatch at {:#x}: reference {a:02x}, generated {b:02x}", FILE_OFFSET+i));
    }
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn relocator_uses_long_immediate_nonfinal_packet_marker() {
        let b=build_relocator_trampoline().unwrap();
        assert_eq!(b.len(),20);
        let words:Vec<u32>=b.chunks_exact(4).map(|c|u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(words[0]&0xc000,0x4000);
        assert_eq!(words[1]&0xc000,0x4000);
        assert_eq!(words[2]&0xc000,0xc000);
        assert_eq!(words[3]&0xc000,0xc000);
        assert_eq!(words[4]&0xc000,0xc000);
    }
    #[test]
    fn supervisor_segment_is_exactly_adjacent() {
        let data=build_supervisor_span().unwrap();
        assert_eq!(data.len(),84);
        assert_eq!(&data[..FUNCTION_SIZE],build_syscfg_reset().unwrap());
        assert_eq!(&data[FUNCTION_SIZE..FUNCTION_SIZE+RELOCATOR_SIZE],build_relocator_trampoline().unwrap());
        assert_eq!(&data[FUNCTION_SIZE+RELOCATOR_SIZE..],build_syscfg_disable().unwrap());
    }
    #[test] fn syscfg_disable_has_correct_size_and_singleton_packet_markers() {
        let result=build_syscfg_disable().unwrap();
        assert_eq!(result.len(), DISABLE_SIZE);
        assert_eq!(DISABLE_PHYSICAL_ADDRESS,0x96c0_0000+0xd80);
        for chunk in result.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(chunk.try_into().unwrap())&0xc000,0xc000);
        }
    }
    #[test] fn syscfg_reset_fragment_has_exact_function_boundaries() {
        let code=build_syscfg_reset().unwrap();
        assert_eq!(code.len(),FUNCTION_SIZE);
        assert_eq!(PHYSICAL_ADDRESS,0x96c0_0000+0xd40);
        for instruction in code.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(instruction.try_into().unwrap()) & 0xc000,0xc000);
        }
    }
}
