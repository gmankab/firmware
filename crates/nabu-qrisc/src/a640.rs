// SPDX-License-Identifier: GPL-3.0-only
//! Small, human-authored QRISC routines using documented Mesa Freedreno A6xx
//! GPU register names and the label-aware Rust instruction generator.
//!
//! The chosen behavior and instruction ordering were guided by analysis of
//! the original firmware; this is NOT a clean-room copyright determination.
//! It is a first step toward independently compiled behavior, not complete
//! A640 firmware, and these snippets cannot be flashed or executed alone.
use crate::{codec::{firmware_header, QriscWord}, program::{assemble, Item}};

// Public register numbers, from Mesa Freedreno's adreno_common.xml and
// adreno_control_regs.xml for the A6xx GPU command processor.
const CP_STATE_DEBUG_INDEX: u16 = 0x01ec;
const PREEMPT_ENABLE: u16 = 0x0071;
const WFI_PEND_CTR: u16 = 0x0038;

/// Generate the QRISC firmware metadata word and an entry prologue that saves
/// and temporarily disables command processor preemption. No OEM instruction
/// words are embedded in this source code.
pub fn preemption_guard(family_id: u16, revision: u16) -> Result<Vec<u8>, String> {
    let instructions = [
        Item::Instr(firmware_header(family_id, revision).map_err(str::to_string)?),
        Item::Instr(QriscWord::MovImmediate {
            repeat: false, dst: 15, shift: 0, imm: CP_STATE_DEBUG_INDEX,
        }),
        Item::Instr(QriscWord::ControlRead { base: 0, dest: 9, reg: PREEMPT_ENABLE }),
        Item::Instr(QriscWord::ControlWrite { base: 0, source: 0, reg: PREEMPT_ENABLE }),
    ];
    assemble(&instructions)
}

/// Read the GPU write-for-idle counter until it is zero, read again, and
/// return. This is a QRISC *code generator*, not a simulator or live driver.
pub fn wait_for_idle() -> Result<Vec<u8>, String> {
    use QriscWord::*;
    let items = [
        Item::Instr(ControlRead { base: 0, dest: 5, reg: WFI_PEND_CTR }),
        Item::Label("retry"),
        Item::BranchTo { opcode: 0x30, src: 5, condition: 0, label: "retry" },
        Item::Instr(ControlRead { base: 0, dest: 5, reg: WFI_PEND_CTR }),
        Item::Instr(Return { ignored_bits: 0 }),
        Item::Instr(Nop { metadata: 0 }),
    ];
    assemble(&items)
}


// Register addresses documented by Freedreno's A6xx register database.
const REG_WRITE_ADDR: u16 = 0x0024;
const REG_WRITE: u16 = 0x0025;
const TRUSTED_BASE: u16 = 0xf800;
// These registers' numeric addresses are known, but their complete security
// semantics are NOT established by this project.
const SECURITY_STATUS: u16 = 0x0083;
const SECURITY_POLL: u16 = 0x006d;
const SECURITY_STATE: u16 = 0x0080;

/// Generate the first 30 A640 QRISC control words as *typed operations*.
///
/// The logic is reconstructed from observed branching and public A6xx ISA
/// and register descriptions: preserve preemption, dispatch on command mode,
/// optionally initialize a trusted-address range, and begin polling security
/// status. This is deliberately only a prefix: execution continues beyond the
/// last word, and these bytes are NOT a flashable firmware image.
///
/// Instruction ordering is informed by studying OEM firmware. Byte equality
/// does NOT establish independent copyright provenance or correct hardware
/// semantics. No original 32-bit instruction words are present in this source.
pub fn security_setup_prefix(family_id: u16, revision: u16) -> Result<Vec<u8>, String> {
    use QriscWord::*;
    let p = vec![
        Item::Instr(firmware_header(family_id, revision).map_err(str::to_string)?),
        Item::Instr(MovImmediate { repeat: false, dst: 15, shift: 0, imm: CP_STATE_DEBUG_INDEX }),
        Item::Instr(ControlRead { base: 0, dest: 9, reg: PREEMPT_ENABLE }),
        Item::Instr(ControlWrite { base: 0, source: 0, reg: PREEMPT_ENABLE }),
        // Command mode is supplied in QRISC register 2.
        Item::BranchTo { opcode: 0x31, src: 2, condition: 0, label: "security_status" },
        Item::BranchTo { opcode: 0x31, src: 2, condition: 1, label: "security_status" },
        Item::BranchTo { opcode: 0x31, src: 2, condition: 3, label: "program_trusted_base" },
        Item::BranchTo { opcode: 0x31, src: 2, condition: 2, label: "finish_dispatch" },
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: 0x20 }),
        Item::BranchTo { opcode: 0x32, src: 0, condition: 0, label: "finish_dispatch" },
        Item::Instr(RegisterAlu { src1: 0, src2: 0, dst: 3, function: 6 }),
        Item::Label("program_trusted_base"),
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: TRUSTED_BASE }),
        Item::Instr(ControlWrite { base: 0, source: 3, reg: REG_WRITE_ADDR }),
        Item::Instr(ControlWrite { base: 0, source: 4, reg: REG_WRITE }),
        Item::Instr(ControlWrite { base: 0, source: 5, reg: REG_WRITE }),
        Item::Instr(ControlWrite { base: 0, source: 6, reg: REG_WRITE }),
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: 1 }),
        Item::Label("finish_dispatch"),
        // Undocumented bit 24 is retained as a typed return field. The
        // significance of this bit on real A640 hardware is still unknown.
        Item::Instr(Return { ignored_bits: 1 << 24 }),
        Item::Instr(ControlWrite { base: 0, source: 9, reg: PREEMPT_ENABLE }),
        Item::Label("security_status"),
        Item::Instr(ControlRead { base: 0, dest: 8, reg: SECURITY_STATUS }),
        Item::Instr(MovImmediate { repeat: false, dst: 3, shift: 0, imm: 4 }),
        Item::Instr(ControlWrite { base: 0, source: 3, reg: SECURITY_STATUS }),
        Item::Label("poll_status"),
        Item::Instr(ControlRead { base: 0, dest: 3, reg: SECURITY_POLL }),
        // QRISC immediate ALU encodings for rotate, shift left and unsigned
        // shift right (per the public Mesa ISA definitions).
        Item::Instr(ImmediateAlu { opcode: 0x18, src: 3, dst: 3, imm: 1 }),
        Item::Instr(ImmediateAlu { opcode: 0x12, src: 3, dst: 3, imm: 7 }),
        Item::Instr(ImmediateAlu { opcode: 0x14, src: 3, dst: 3, imm: 7 }),
        Item::BranchTo { opcode: 0x31, src: 3, condition: 0, label: "read_security_state" },
        Item::BranchTo { opcode: 0x30, src: 3, condition: 0, label: "poll_status" },
        Item::Instr(RegisterAlu { src1: 0, src2: 30, dst: 0, function: 6 }),
        Item::Label("read_security_state"),
        Item::Instr(ControlRead { base: 0, dest: 10, reg: SECURITY_STATE }),
    ];
    let bytes = assemble(&p)?;
    debug_assert_eq!(bytes.len(), 30 * 4);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assembled_routines_have_expected_lengths_and_family_header() {
        let prefix = preemption_guard(0x6ee, 1).unwrap();
        assert_eq!(prefix.len(), 16);
        assert_eq!(u32::from_le_bytes(prefix[..4].try_into().unwrap()),
            firmware_header(0x6ee, 1).unwrap().encode().unwrap());
        assert_eq!(wait_for_idle().unwrap().len(), 20);
    }
    #[test]
    fn security_setup_is_label_resolved_and_has_guard_prefix() {
        let bytes = security_setup_prefix(0x6ff, 0x020).unwrap();
        assert_eq!(bytes.len(), 120);
        assert!(bytes.starts_with(&preemption_guard(0x6ff, 0x020).unwrap()));
        let word = |i: usize| u32::from_le_bytes(bytes[i*4..i*4+4].try_into().unwrap());
        assert_eq!(QriscWord::decode(word(4)), Some(QriscWord::Branch {
            opcode: 0x31, src: 2, condition: 0, delta: 15,
        }));
        assert_eq!(QriscWord::decode(word(6)), Some(QriscWord::Branch {
            opcode: 0x31, src: 2, condition: 3, delta: 5,
        }));
        assert_eq!(QriscWord::decode(word(27)), Some(QriscWord::Branch {
            opcode: 0x30, src: 3, condition: 0, delta: -5,
        }));
        assert_eq!(QriscWord::decode(word(17)), Some(QriscWord::Return {
            ignored_bits: 1 << 24,
        }));
    }
    #[test]
    fn invalid_firmware_id_is_rejected() {
        assert!(preemption_guard(0x1000, 0).is_err());
    }
}
