// SPDX-License-Identifier: GPL-3.0-only
//! Typed QRISC operations for configuring a protected A6xx shader launch.
//!
//! This is an address-dependent, partial routine. Calls target absolute word
//! indices in the reference layout. Relinking a program requires adjusting
//! those targets and recomputing all affected branch/call destinations.
//! No machine-instruction words are copied into this implementation.

use qrisc::{codec::QriscWord, command_stream::{CommandStream, StreamValue}, program::{assemble, Item}};

/// Zero-based word offset of this fragment within the A640 QRISC stream.
pub const START_WORD: usize = 84;

// Public A6xx command processor registers (Mesa Freedreno register database).
const WFI_PEND_INCR: u16 = 0x30;
const REG_WRITE_ADDR: u16 = 0x24;
const REG_WRITE: u16 = 0x25;
const REG_READ_DWORDS: u16 = 0x26;
const REG_READ_ADDR: u16 = 0x27;
const ZAP_SHADER_ADDR_LOW: u16 = 0x62;
const ZAP_SHADER_ADDR_HIGH: u16 = 0x63;
const TRUSTED_BASE: u16 = 0xf800;
const TRUST_CONTROL: u16 = 0xf400;

/// Build the A6xx protected-register setup, address alignment and status
/// update operations. This has no entry point and is NOT a standalone firmware.
/// The two absolute call destinations are specific to the reference layout.
pub fn protected_shader_register_setup() -> Result<Vec<u8>, String> {
    use QriscWord::*;
    let instructions = [
        // Shared command routines are elsewhere in the QRISC program.
        Item::Instr(Call { target: 474 }),
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: 4 }),
        Item::Instr(Call { target: 474 }),
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: 0x31 }),
        Item::Instr(ControlWrite { base: 0, source: 0, reg: WFI_PEND_INCR }),
        Item::Instr(MovImmediate { repeat: false, dst: 29, shift: 0, imm: 0x1c }),
        Item::Instr(MovImmediate { repeat: false, dst: 31, shift: 0, imm: 1 }),
        Item::Instr(Call { target: 487 }),
        Item::Instr(MovImmediate { repeat: false, dst: 29, shift: 24, imm: 0x81 }),
        // Read a trusted-base register triplet through the command interface.
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: TRUSTED_BASE }),
        Item::Instr(MovImmediate { repeat: false, dst: 4, shift: 0, imm: 3 }),
        Item::Instr(ControlWrite { base: 0, source: 4, reg: REG_READ_DWORDS }),
        Item::Instr(ControlWrite { base: 0, source: 3, reg: REG_READ_ADDR }),
        Item::Instr(RegisterAlu { src1: 0, src2: 30, dst: 11, function: 6 }),
        Item::Instr(RegisterAlu { src1: 0, src2: 30, dst: 13, function: 6 }),
        Item::Instr(RegisterAlu { src1: 0, src2: 30, dst: 14, function: 6 }),
        // Form the shader's GPU-visible memory address and aligned size.
        Item::Instr(ControlRead { base: 0, dest: 4, reg: ZAP_SHADER_ADDR_LOW }),
        Item::Instr(ControlRead { base: 0, dest: 5, reg: ZAP_SHADER_ADDR_HIGH }),
        Item::Instr(ImmediateAlu { opcode: 0x12, src: 15, dst: 15, imm: 2 }),
        Item::Instr(ImmediateAlu { opcode: 0x02, src: 15, dst: 15, imm: 0xfff }),
        Item::Instr(ImmediateAlu { opcode: 0x14, src: 15, dst: 15, imm: 12 }),
        Item::Instr(ImmediateAlu { opcode: 0x12, src: 15, dst: 15, imm: 12 }),
        Item::Instr(ControlWrite { base: 0, source: 3, reg: REG_WRITE_ADDR }),
        Item::Instr(ControlWrite { base: 0, source: 4, reg: REG_WRITE }),
        Item::Instr(ControlWrite { base: 0, source: 5, reg: REG_WRITE }),
        Item::Instr(ControlWrite { base: 0, source: 15, reg: REG_WRITE }),
        // Enable the trusted control interface for the pending command.
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: TRUST_CONTROL }),
        Item::Instr(MovImmediate { repeat: false, dst: 4, shift: 0, imm: 1 }),
        Item::Instr(ControlWrite { base: 0, source: 4, reg: REG_READ_DWORDS }),
        Item::Instr(ControlWrite { base: 0, source: 3, reg: REG_READ_ADDR }),
        Item::Instr(ImmediateAlu { opcode: 0x0c, src: 30, dst: 4, imm: 2 }),
        Item::Instr(ControlWrite { base: 0, source: 3, reg: REG_WRITE_ADDR }),
        Item::Instr(ControlWrite { base: 0, source: 4, reg: REG_WRITE }),
    ];
    let bytes = assemble(&instructions)?;
    debug_assert_eq!(bytes.len(), 33 * 4);
    Ok(bytes)
}

/// Beginning of the typed compute-shader register configuration block.
/// These program the GPU's command stream rather than load a complete shader.
pub const CONFIG_START_WORD: usize = START_WORD + 33;

/// Emit a bounded A6xx compute configuration using named register write
/// operations, including a contiguous ndrange descriptor. Register values
/// and ordering are hardware-specific; no instruction words are embedded.
pub fn compute_register_configuration() -> Result<Vec<u8>, String> {
    use StreamValue::{Small, Wide};
    let mut writes = CommandStream::new();
    // Control state for the protected compute workload.
    writes.write_range(0xbb08, &[Wide { upper: 0x000f, lower: 0xffff }])?;
    writes.write_range(0xbb11, &[Small(0)])?;
    writes.write_range(0xab00, &[Small(0)])?;
    writes.write_range(0xb987, &[Small(0x101)])?;
    writes.write_range(0xa9bb, &[Wide { upper: 0x20, lower: 0x100 }])?;
    // The ndrange descriptor is an implicit incrementing register stream.
    writes.write_range(0xb990, &[
        Small(0x5fd), Small(0x300), Small(0), Small(1),
        Small(0), Small(1), Small(0),
        Wide { upper: 0xdc, lower: 0xd8d4 }, Small(0xfc),
    ])?;
    writes.write_range(0xb999, &[Small(2), Small(1), Small(1)])?;
    writes.write_range(0xa9bc, &[Small(4)])?;
    writes.write_range(0xa9b3, &[Small(0)])?;
    writes.write_range(0xa9b0, &[Wide { upper: 0x42, lower: 0x1800 }, Small(0x1f)])?;
    writes.write_range(0x001c, &[Small(1)])?;
    writes.write_range(0xb309, &[Small(0x4c)])?;

    let mut emitted = assemble(&[
        Item::Instr(QriscWord::MovImmediate { repeat: false, dst: 0, shift: 0, imm: 0x11 }),
        Item::Instr(QriscWord::MovImmediate { repeat: false, dst: 0, shift: 0, imm: 0x6401 }),
    ])?;
    emitted.extend_from_slice(&writes.encode().map_err(str::to_string)?);
    debug_assert_eq!(emitted.len(), 41 * 4);
    Ok(emitted)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compute_stream_encodes_a_sequence_of_named_gpu_register_writes() {
        let instructions = compute_register_configuration().unwrap();
        assert_eq!(CONFIG_START_WORD, 117);
        assert_eq!(instructions.len(), 41 * 4);
        let first_cp_address = u32::from_le_bytes(instructions[8..12].try_into().unwrap());
        assert_eq!(QriscWord::decode(first_cp_address), Some(QriscWord::MovImmediate {
            repeat: false, dst: 29, shift: 0, imm: 0xbb08,
        }));
    }

    #[test]
    fn builds_layout_anchored_fragment_from_typed_instructions() {
        let program = protected_shader_register_setup().unwrap();
        assert_eq!(program.len(), 33 * 4);
        let word = |index: usize| u32::from_le_bytes(program[index*4..][..4].try_into().unwrap());
        assert_eq!(QriscWord::decode(word(0)), Some(QriscWord::Call { target: 474 }));
        assert_eq!(QriscWord::decode(word(7)), Some(QriscWord::Call { target: 487 }));
        assert_eq!(QriscWord::decode(word(17)), Some(QriscWord::ControlRead {
            base: 0, dest: 5, reg: ZAP_SHADER_ADDR_HIGH,
        }));
    }
}
