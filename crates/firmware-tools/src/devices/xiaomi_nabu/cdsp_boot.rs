// SPDX-License-Identifier: GPL-3.0-only
//! Independently encoded Qualcomm CDSP Hexagon entry vectors.
//!
//! This module emits *only* the nine decoded, four-byte instruction packets
//! beginning at the reference's first executable PT_LOAD, file offset 0x3000.
//! The next opcode in the OEM image is not understood by the available LLVM 6
//! backend; neither that opcode nor any OEM bytes are embedded here.
//!
//! All direct-jump displacements are relative to the instruction's address
//! and counted in **bytes**. The Hexagon immediate stores halfword units.
//!
//! Only the evidenced scalar forms are implemented. This is a partial
//! firmware reconstruction, not a bootable or authenticated image.

use crate::parse_elf32;

pub const START_FILE_OFFSET: usize = 0x3000;
pub const REBUILT_LEN: usize = 9 * 4;
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

/// Two proved scalar instructions: direct packet-relative jump and small
/// signed-independent (nonnegative) integer assignment. No packet bundling
/// or duplex-instruction support is implied by these constructors.
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
    fn rejects_wrong_reference_header() {
        assert!(validate_reference(&vec![0; 36]).is_err());
    }
}
