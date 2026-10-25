// SPDX-License-Identifier: GPL-3.0-only
//! Independently encoded Qualcomm CDSP Hexagon entry vectors.
//!
//! This module emits the first 17 decoded, four-byte instruction packets
//! beginning at the reference's first executable PT_LOAD, file offset 0x3000.
//! Some privileged V66 system-register opcodes are outside the available LLVM
//! 6 assembler, and are reconstructed with checked architecture bitfields
//! corroborated by the Rizin Hexagon decoder. No OEM instruction bytes are
//! imported as literal arrays.
//!
//! All direct-jump displacements are relative to the instruction's address
//! and counted in **bytes**. The Hexagon immediate stores halfword units.
//!
//! Only the evidenced scalar forms are implemented. This is a partial
//! firmware reconstruction, not a bootable or authenticated image.

use crate::parse_elf32;

pub const START_FILE_OFFSET: usize = 0x3000;
pub const REBUILT_LEN: usize = 9 * 4;
/// Entry jump vectors and the entire following supervisor configuration.
pub const INIT_PREFIX_LEN: usize = 0x44;
pub const ENTRY_PHYS: u32 = 0x96c0_0000;
pub const ENTRY_ELF_VADDR: u32 = 0xfe00_0000;
pub const FIRST_EXECUTABLE_SIZE: u32 = 0x37b34;
pub const HEXAGON_ELF_MACHINE: u16 = 164;
pub const CDSP_ELF_FLAGS: u32 = 0x66;

/// Register operand for the supported subset of Hexagon V6 instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneralRegister(u8);

impl GeneralRegister {
    pub fn new(index: u8) -> Result<Self, String> {
        if index > 31 {
            return Err(format!("Hexagon register out of range: r{index}"));
        }
        Ok(Self(index))
    }

    pub fn number(self) -> u8 { self.0 }
}

/// Independently evidenced scalar instructions, restricted to operand forms
/// whose bit fields have been cross-checked against LLVM or the Hexagon
/// decoder. No duplex-instruction support is implied by these constructors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarPacket {
    /// One four-byte instruction packet, `jump` with a relative byte offset.
    /// This narrow encoding only supports nonnegative, halfword-aligned
    /// displacement up to 0x1ffe. Larger or negative branches require the
    /// full Hexagon immediate/extension encoding and deliberately fail.
    JumpRelative { displacement_bytes: u32 },
    /// One four-byte instruction packet `rN = #unsigned_small`.
    /// Only the 7-bit subset independently checked against the assembler is
    /// accepted; no sign extension is inferred from the original firmware.
    AssignSmallImmediate { dst: GeneralRegister, value: u8 },
    /// Hexagon V66 special-register write (`SSR=Rn` or `SYSCFG=Rn`).
    /// The selectors were independently decoded with Rizin's Hexagon backend.
    WriteSystem { system: SystemRegister, src: GeneralRegister },
    /// Hexagon V66 `Rn = SYSCFG`; only this documented register is supported.
    ReadSyscfg { dst: GeneralRegister },
    /// Hexagon `Rd = setbit(Rs,#index)`, 0 <= index < 32.
    SetBit { dst: GeneralRegister, src: GeneralRegister, position: u8 },
    /// Hexagon `Rd = clrbit(Rs,#index)`.
    ClearBit { dst: GeneralRegister, src: GeneralRegister, position: u8 },
    /// Hexagon `Rd = or(Rs,#imm)` with the confirmed nonnegative immediate field.
    OrSmallImmediate { dst: GeneralRegister, src: GeneralRegister, value: u8 },
    /// Hexagon `Rd = sub(Rs,Rt)`; fields confirmed with distinct operands.
    SubtractRegisters { dst: GeneralRegister, left: GeneralRegister, right: GeneralRegister },
    /// Hexagon `jumpr Rn` (not a return/stack ABI assumption).
    JumpRegister { source: GeneralRegister },
    /// Cache and instruction synchronization (singleton packets).
    Dckill,
    Ickill,
    Isync,
}

/// V66 system register IDs *as used by this exact instruction family*.
/// Unsupported special registers must not be guessed from their numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemRegister {
    Ssr,
    Syscfg,
}
impl SystemRegister {
    fn opcode_selector(self) -> u32 {
        match self { Self::Ssr => 6, Self::Syscfg => 18 }
    }
}

impl ScalarPacket {
    pub fn encode(self) -> Result<u32, String> {
        match self {
            Self::JumpRelative { displacement_bytes } => {
                if displacement_bytes > 0x1ffe || displacement_bytes & 1 != 0 {
                    return Err(format!(
                        "Hexagon jump displacement outside confirmed encoding: {displacement_bytes:#x}"
                    ));
                }
                // 0x5800 selects Hexagon direct jump; bits 15:14 == 3 mark
                // the last instruction of a singleton packet, not a gap.
                Ok(0x5800_c000 | (displacement_bytes / 2))
            }
            Self::AssignSmallImmediate { dst, value } => {
                if value > 127 {
                    return Err("Hexagon small immediate must be <= 127".into());
                }
                Ok(0x7800_c000 | (u32::from(value) << 5) | u32::from(dst.number()))
            }
            Self::WriteSystem { system, src } => {
                // `SSR=R0` and `SYSCFG=R0` use instruction family 0x67.
                // Source general-register number occupies bits 20:16;
                // destination system-register selector occupies bits 4:0.
                Ok(0x6700_c000 | (u32::from(src.number()) << 16)
                    | system.opcode_selector())
            }
            Self::ReadSyscfg { dst } => {
                // The observed SYSCFG selector is 0x12 in bits 20:16,
                // destination register in bits 4:0. Rizin V66 confirms it.
                Ok(0x6e80_c000 | (SystemRegister::Syscfg.opcode_selector() << 16)
                    | u32::from(dst.number()))
            }
            Self::SetBit { dst, src, position } => {
                if position >= 32 {
                    return Err(format!("Hexagon setbit position out of range: {position}"));
                }
                // Confirmed against LLVM 6's independent Hexagon assembler
                // with distinct source/destination registers and bit indices.
                Ok(0x8cc0_c000 | (u32::from(src.number()) << 16)
                    | (u32::from(position) << 8) | u32::from(dst.number()))
            }
            Self::ClearBit { dst, src, position } => {
                if position >= 32 { return Err("Hexagon clrbit position >= 32".into()); }
                Ok(0x8cc0_c020 | (u32::from(src.number()) << 16)
                    | (u32::from(position) << 8) | u32::from(dst.number()))
            }
            Self::OrSmallImmediate { dst, src, value } => {
                if value > 127 { return Err("Hexagon OR immediate > 127".into()); }
                Ok(0x7680_c000 | (u32::from(src.number()) << 16)
                    | (u32::from(value) << 5) | u32::from(dst.number()))
            }
            Self::SubtractRegisters { dst, left, right } => {
                // Hexagon subtraction register fields are **not** laid out
                // like the ARM32 data-processing fields. Independent LLVM
                // samples include r31 = sub(r31,r25), r1 = sub(r2,r3).
                Ok(0xf320_c000 | (u32::from(right.number()) << 16)
                    | (u32::from(left.number()) << 8) | u32::from(dst.number()))
            }
            Self::JumpRegister { source } =>
                Ok(0x5280_c000 | (u32::from(source.number()) << 16)),
            Self::Dckill => Ok(0xa200_c000),
            Self::Ickill => Ok(0x56c0_d000),
            Self::Isync => Ok(0x57c0_c002),
        }
    }
}

/// The reference opens with a selector branch, six guarded self-branches,
/// a relocation-free jump into the entry dispatch, then initializes `r0`.
/// The target at runtime physical address `0x96c00cac` remains outside this
/// generated fragment and must be reconstructed separately.
fn entry_packets() -> Result<[ScalarPacket; 9], String> {
    use ScalarPacket::*;
    let r0 = GeneralRegister::new(0)?;
    Ok([
        JumpRelative { displacement_bytes: 0x1c },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0 },
        JumpRelative { displacement_bytes: 0xc90 },
        AssignSmallImmediate { dst: r0, value: 0 },
    ])
}

pub fn build_entry() -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity(REBUILT_LEN);
    for packet in entry_packets()? {
        output.extend_from_slice(&packet.encode()?.to_le_bytes());
    }
    if output.len() != REBUILT_LEN {
        return Err("CDSP entry emitter produced an incorrect length".into());
    }
    Ok(output)
}

/// Extend the verified nine-packet entry with eight system-init packets.
/// These privilege-state and cache-control operations were independently
/// *decoded* with the Rizin Hexagon V6 backend; LLVM 6 cannot assemble the
/// special-register forms, so the emitted bytes are not claimed to have a
/// complete second-toolchain assembler round-trip. Nothing after 0x3044
/// (including the reference's invalid/reserved 0x3044 word) is filled in.
pub fn build_init_prefix() -> Result<Vec<u8>, String> {
    use ScalarPacket::*;
    let r0 = GeneralRegister::new(0)?;
    let mut output = build_entry()?;
    let privileged = [
        WriteSystem { system: SystemRegister::Ssr, src: r0 },
        ReadSyscfg { dst: r0 },
        SetBit { dst: r0, src: r0, position: 3 },
        SetBit { dst: r0, src: r0, position: 5 },
        WriteSystem { system: SystemRegister::Syscfg, src: r0 },
        Dckill,
        Ickill,
        JumpRelative { displacement_bytes: 0x104 },
    ];
    for packet in privileged {
        output.extend_from_slice(&packet.encode()?.to_le_bytes());
    }
    if output.len() != INIT_PREFIX_LEN {
        return Err("CDSP initialization emitter produced an incorrect length".into());
    }
    Ok(output)
}

pub fn verify_init_prefix(image: &[u8]) -> Result<(), String> {
    validate_reference(image)?;
    let generated = build_init_prefix()?;
    let expected = &image[START_FILE_OFFSET..START_FILE_OFFSET + INIT_PREFIX_LEN];
    if let Some((i, (original, built))) = expected.iter().zip(&generated)
        .enumerate().find(|(_, (a, b))| a != b)
    {
        return Err(format!("CDSP initialization differs at file {:#x}: expected {original:02x}, built {built:02x}",
            START_FILE_OFFSET + i));
    }
    Ok(())
}

/// Check that the user's private reference is the expected CDSP ELF32 image,
/// without confusing ELF vaddr, physical load address, and file offset.
/// This intentionally doesn't purport to authenticate the vendor image.
pub fn validate_reference(image: &[u8]) -> Result<(), String> {
    let elf = parse_elf32(image)?;
    if elf.machine != HEXAGON_ELF_MACHINE {
        return Err(format!("expected Hexagon ELF machine 164, found {}", elf.machine));
    }
    if elf.entry != ENTRY_PHYS {
        return Err(format!("unexpected CDSP physical entrypoint: {:#x}", elf.entry));
    }
    if image.get(36..40) != Some(&CDSP_ELF_FLAGS.to_le_bytes()) {
        return Err("unexpected CDSP ELF e_flags".into());
    }
    let segment = elf.program_headers.iter().find(|ph| {
        ph.kind == 1 && ph.offset as usize == START_FILE_OFFSET &&
            ph.file_size == FIRST_EXECUTABLE_SIZE
    }).ok_or("expected CDSP first executable PT_LOAD not found")?;
    if segment.vaddr != ENTRY_ELF_VADDR || segment.paddr != ENTRY_PHYS ||
        segment.flags & 1 == 0 || segment.file_size > segment.mem_size
    {
        return Err("CDSP first PT_LOAD mapping or permissions differ".into());
    }
    let end = START_FILE_OFFSET.checked_add(REBUILT_LEN)
        .ok_or("CDSP entry offset overflow")?;
    if image.get(START_FILE_OFFSET..end).is_none() {
        return Err("truncated CDSP entry instructions".into());
    }
    Ok(())
}

pub fn verify_reference(image: &[u8]) -> Result<(), String> {
    validate_reference(image)?;
    let output = build_entry()?;
    let reference = &image[START_FILE_OFFSET..START_FILE_OFFSET + REBUILT_LEN];
    if let Some((position, (expected, found))) = reference.iter().zip(&output)
        .enumerate().find(|(_, (expected, found))| expected != found)
    {
        return Err(format!(
            "CDSP entry differs at file offset {:#x}: reference {expected:02x} generated {found:02x}",
            START_FILE_OFFSET + position
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encodes_direct_jumps_and_checks_bounds() {
        assert_eq!(ScalarPacket::JumpRelative { displacement_bytes: 0 }.encode().unwrap(), 0x5800_c000);
        assert_eq!(ScalarPacket::JumpRelative { displacement_bytes: 0x1c }.encode().unwrap(), 0x5800_c00e);
        assert_eq!(ScalarPacket::JumpRelative { displacement_bytes: 0xc90 }.encode().unwrap(), 0x5800_c648);
        assert!(ScalarPacket::JumpRelative { displacement_bytes: 1 }.encode().is_err());
        assert!(ScalarPacket::JumpRelative { displacement_bytes: 0x2000 }.encode().is_err());
    }

    #[test]
    fn assigns_small_immediates_to_checked_registers() {
        assert!(GeneralRegister::new(32).is_err());
        let r0 = GeneralRegister::new(0).unwrap();
        let r3 = GeneralRegister::new(3).unwrap();
        assert_eq!(ScalarPacket::AssignSmallImmediate { dst: r0, value: 0 }.encode().unwrap(), 0x7800_c000);
        assert_eq!(ScalarPacket::AssignSmallImmediate { dst: r3, value: 12 }.encode().unwrap(), 0x7800_c183);
        assert!(ScalarPacket::AssignSmallImmediate { dst: r3, value: 128 }.encode().is_err());
    }

    #[test]
    fn entry_has_nine_singleton_packets_not_filler() {
        let b = build_entry().unwrap();
        assert_eq!(b.len(), REBUILT_LEN);
        for word in b.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(word.try_into().unwrap()) & 0xc000, 0xc000);
        }
        assert_eq!(entry_packets().unwrap().len(), 9);
    }
    #[test]
    fn privileged_system_operations_use_checked_hexagon_fields() {
        use ScalarPacket::*;
        let r0 = GeneralRegister::new(0).unwrap();
        let r15 = GeneralRegister::new(15).unwrap();
        assert_eq!(WriteSystem { system: SystemRegister::Ssr, src: r0 }.encode().unwrap(), 0x6700_c006);
        assert_eq!(WriteSystem { system: SystemRegister::Syscfg, src: r0 }.encode().unwrap(), 0x6700_c012);
        assert_eq!(WriteSystem { system: SystemRegister::Syscfg, src: r15 }.encode().unwrap(), 0x670f_c012);
        assert_eq!(ReadSyscfg { dst: r0 }.encode().unwrap(), 0x6e92_c000);
        assert_eq!(Dckill.encode().unwrap(), 0xa200_c000);
        assert_eq!(Ickill.encode().unwrap(), 0x56c0_d000);
    }

    #[test]
    fn bit_instruction_is_encoded_independently_of_registers() {
        use ScalarPacket::SetBit;
        let r0 = GeneralRegister::new(0).unwrap();
        let r1 = GeneralRegister::new(1).unwrap();
        let r5 = GeneralRegister::new(5).unwrap();
        let r7 = GeneralRegister::new(7).unwrap();
        assert_eq!(SetBit { dst: r0, src: r0, position: 3 }.encode().unwrap(), 0x8cc0_c300);
        assert_eq!(SetBit { dst: r5, src: r7, position: 2 }.encode().unwrap(), 0x8cc7_c205);
        assert_eq!(SetBit { dst: r1, src: r1, position: 3 }.encode().unwrap(), 0x8cc1_c301);
        assert!(SetBit { dst: r0, src: r0, position: 32 }.encode().is_err());
    }

    #[test]
    fn hexagon_conditional_bit_and_arithmetic_fields_match_independent_llvm() {
        use ScalarPacket::*;
        let reg = |n| GeneralRegister::new(n).unwrap();
        assert_eq!(ClearBit { dst: reg(15), src: reg(15), position: 1 }.encode().unwrap(), 0x8ccf_c12f);
        assert_eq!(OrSmallImmediate { dst: reg(15), src: reg(15), value: 3 }.encode().unwrap(), 0x768f_c06f);
        assert_eq!(OrSmallImmediate { dst: reg(1), src: reg(2), value: 3 }.encode().unwrap(), 0x7682_c061);
        assert_eq!(SubtractRegisters { dst: reg(31), left: reg(31), right: reg(25) }.encode().unwrap(), 0xf339_df1f);
        assert_eq!(SubtractRegisters { dst: reg(1), left: reg(2), right: reg(3) }.encode().unwrap(), 0xf323_c201);
        assert_eq!(JumpRegister { source: reg(31) }.encode().unwrap(), 0x529f_c000);
        assert_eq!(Isync.encode().unwrap(), 0x57c0_c002);
        assert!(ClearBit { dst: reg(15), src: reg(15), position: 32 }.encode().is_err());
        assert!(OrSmallImmediate { dst: reg(15), src: reg(15), value: 128 }.encode().is_err());
    }

    #[test]
    fn privileged_initialization_has_exact_packet_coverage() {
        let b = build_init_prefix().unwrap();
        assert_eq!(b.len(), INIT_PREFIX_LEN);
        assert_eq!(&b[..REBUILT_LEN], build_entry().unwrap().as_slice());
        for word in b.chunks_exact(4) {
            assert_eq!(u32::from_le_bytes(word.try_into().unwrap()) & 0xc000, 0xc000);
        }
        assert_eq!(u32::from_le_bytes(b[64..68].try_into().unwrap()), 0x5800_c082);
    }

    #[test]
    fn rejects_wrong_reference_header() {
        assert!(validate_reference(&vec![0; 36]).is_err());
    }
}
