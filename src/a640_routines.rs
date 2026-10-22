// SPDX-License-Identifier: GPL-3.0-only
//! Small, human-authored QRISC routines using documented Mesa Freedreno A6xx
//! GPU register names and the label-aware Rust instruction generator.
//!
//! The chosen behavior and instruction ordering were guided by analysis of
//! the original firmware; this is NOT a clean-room copyright determination.
//! It is a first step toward independently compiled behavior, not complete
//! A640 firmware, and these snippets cannot be flashed or executed alone.
use crate::{qrisc_codec::{firmware_header, QriscWord}, qrisc_program::{assemble, Item}};

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
    fn invalid_firmware_id_is_rejected() {
        assert!(preemption_guard(0x1000, 0).is_err());
    }
}
