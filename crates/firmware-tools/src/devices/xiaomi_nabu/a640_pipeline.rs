// SPDX-License-Identifier: GPL-3.0-only
//! Build the A640 protected-compute dispatch from named GPU operations.
//!
//! The shader address comes from the GPU registers, not a fixed host pointer.
//! This fragment installs shader state, clears unused shader stages, dispatches
//! the protected draw/compute work and programs an RB resolve sequence.
//! It is intentionally tied to this reference firmware's word-address layout.

use qrisc::script::{ImmediateOperation as Imm, RegisterOperation as Reg, BranchCondition, Script};
use qrisc::command_stream::StreamValue::Small;

pub const START_WORD: usize = 0x9e;
pub const END_WORD: usize = 0x110;

// The public Mesa A6xx register database is the source of these addresses.
const ZAP_SHADER_ADDR: u16 = 0x62;
const SP_CS_BASE: u16 = 0xa9b4;
const SP_CS_INSTR_SIZE: u16 = 0xa9bc;
const HLSQ_LOAD_STATE_FRAG_EXT_SRC_ADDR: u16 = 0xb9a1;
const HLSQ_LOAD_STATE_FRAG_CMD: u16 = 0xb9a0;
const HLSQ_LOAD_STATE_GEOM_CMD: u16 = 0xb820;
const HLSQ_LOAD_STATE_GEOM_DATA: u16 = 0xb823;
const HLSQ_LOAD_STATE_FRAG_DATA: u16 = 0xb9a3;
const CP_EVENT_START: u16 = 0xd600;
const SP_EVENT_INITIATOR: u16 = 0xbb02;
const PC_EVENT_INITIATOR: u16 = 0x9842;

fn install_shader_address(p: &mut Script) -> Result<(), String> {
    // The embedded IR3 begins 0x440 bytes into the QRISC executable segment.
    p.control_read(3, ZAP_SHADER_ADDR);
    p.control_read(4, ZAP_SHADER_ADDR + 1);
    p.imm(Imm::Add, 3, 3, 0x440);
    p.register(Reg::AddWithCarry, 4, 4, 0);
    // Round the 0x200-byte shader allocation up to a 128-byte unit count.
    p.mov(5, 0x80);
    p.imm(Imm::ShiftLeft, 5, 5, 2);
    p.imm(Imm::Add, 5, 5, 0x7f);
    p.imm(Imm::ShiftRightUnsigned, 5, 5, 7);
    p.imm(Imm::Or, 29, 0, SP_CS_BASE);
    p.write_data_from(3);
    p.write_data_from(4);
    p.imm(Imm::Or, 29, 0, SP_CS_INSTR_SIZE);
    p.write_data_from(5);
    p.imm(Imm::ShiftLeft, 5, 5, 22);
    p.imm(Imm::Or, 29, 0, HLSQ_LOAD_STATE_FRAG_EXT_SRC_ADDR);
    p.write_data_from(3);
    p.write_data_from(4);
    p.mov_shift(4, 0x36, 16);
    p.imm(Imm::Or, 4, 4, 0);
    p.imm(Imm::Or, 29, 0, HLSQ_LOAD_STATE_FRAG_CMD);
    p.register(Reg::Or, 31, 5, 4);
    // The event helper exists in a separate QRISC callable routine.
    p.call(0x1bf);
    p.nop();
    p.expect_position(0xb5)
}

fn clear_other_shader_stages(p: &mut Script) -> Result<(), String> {
    // Unused vertex, pixel, geometry, hull and domain pipelines are reset.
    for (reg, value) in [
        (0xb800, 0x140), (0xa823, 0x100),
        (0xbb10, 0x140), (0xab04, 0x100),
        (0xb803, 0x100), (0xa894, 0x100),
        (0xb801, 0), (0xa83b, 0),
        (0xb802, 0), (0xa863, 0),
    ] {
        p.write_register_range(reg, &[Small(value)])?;
    }
    // Clear geometry and fragment constant storage with $rem and (rep)mov.
    for (command, descriptor, port) in [
        (HLSQ_LOAD_STATE_GEOM_CMD, 0x4020, HLSQ_LOAD_STATE_GEOM_DATA),
        (HLSQ_LOAD_STATE_FRAG_CMD, 0x4030, HLSQ_LOAD_STATE_FRAG_DATA),
    ] {
        p.imm(Imm::Or, 29, 0, command);
        p.mov_shift(4, descriptor, 16);
        p.imm(Imm::Or, 31, 4, 0x4000);
        p.mov_shift(4, 1, 18);
        p.imm(Imm::Or, 29, 4, port);
        p.mov(28, 0x400);
        p.mov_repeat(31, 0);
    }
    p.expect_position(0xd7)
}

fn run_protected_compute(p: &mut Script) -> Result<(), String> {
    const REG_WRITE_ADDR: u16 = 0x24;
    const REG_WRITE: u16 = 0x25;
    // Draw-call control sends a three-DWORD indirect register transaction.
    p.mov(3, 0x4080);
    p.mov(4, 0);
    p.mov(5, 3);
    p.mov(12, 0x9e0b);
    p.control_write(12, REG_WRITE_ADDR);
    p.imm(Imm::Or, 3, 3, 0x4000);
    p.control_write(3, REG_WRITE);
    p.control_write(4, REG_WRITE);
    p.control_write(5, REG_WRITE);
    // Synchronize event start across CP, SP and PC event initiators.
    p.mov_shift(29, 0xe7, 24);
    p.data(1);
    p.data_address(CP_EVENT_START);
    for initiator in [CP_EVENT_START, 0xbb00, 0x9840] {
        if initiator != CP_EVENT_START { p.data_address(initiator); }
        p.masked_data(6, 0xff);
    }
    p.call(0x1cb);
    p.nop();
    p.expect_position(0xea)
}

fn resolve_protected_result(p: &mut Script) -> Result<(), String> {
    p.write_register_range(0x88d4, &[Small(0)])?;        // resolve window origin
    p.write_register_range(0x88d7, &[Small(0x4100)])?;   // system buffer format
    p.write_register_range(0x88d8, &[Small(0), Small(0)])?;
    p.write_register_range(0x88d6, &[Small(0)])?;
    p.write_register_range(0x88d0, &[Small(0), Small(0)])?;
    p.mov_shift(4, 0xff, 16);
    p.imm(Imm::Or, 31, 4, 0xff);
    p.data(0x1008);
    p.mov_shift(4, 1, 18);
    p.imm(Imm::Or, 29, 4, 0x88df); // resolve clear-color bank
    for _ in 0..4 { p.data(0); }
    p.write_register_range(0x88e3, &[Small(0xf2)])?;    // resolve operation
    p.mov_shift(29, 0xe7, 24);
    p.imm(Imm::Or, 31, 0, 1);
    p.data_address(CP_EVENT_START);
    p.masked_data(6, 0xff);
    p.imm(Imm::And, 3, 6, 0xff);
    p.imm(Imm::ShiftLeft, 3, 3, 16);
    for initiator in [SP_EVENT_INITIATOR, PC_EVENT_INITIATOR] {
        p.data_address(initiator);
        p.imm(Imm::Or, 31, 3, 0x1e);
    }
    p.call(0x1cb);
    p.nop();
    // Transfer control past the embedded 64-slot shader program.
    p.branch(BranchCondition::Always, 0x190)?;
    p.nop();
    p.nop();
    p.expect_position(END_WORD)
}

/// Emit exactly words `0x9e..0x110` (inclusive start, exclusive end).
/// The following bytes are an IR3 shader embedded in the QRISC image.
pub fn protected_compute_and_resolve() -> Result<Vec<u8>, String> {
    let mut script = Script::new(START_WORD);
    install_shader_address(&mut script)?;
    clear_other_shader_stages(&mut script)?;
    run_protected_compute(&mut script)?;
    resolve_protected_result(&mut script)?;
    script.finish(END_WORD)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qrisc::codec::QriscWord;
    #[test]
    fn emits_named_shader_address_setup_and_jump_over_ir3() {
        let image = protected_compute_and_resolve().unwrap();
        assert_eq!(image.len(), (END_WORD - START_WORD) * 4);
        let last_jump = u32::from_le_bytes(image[(0x10d-START_WORD)*4..(0x10e-START_WORD)*4].try_into().unwrap());
        assert_eq!(QriscWord::decode(last_jump), Some(QriscWord::Branch {
            opcode: 0x32, src: 0, condition: 0, delta: 0x190 - 0x10d,
        }));
    }
}
