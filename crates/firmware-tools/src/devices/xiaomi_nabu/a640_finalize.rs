// SPDX-License-Identifier: GPL-3.0-only
//! QRISC A640 post-shader security-state commit and event helper routines.
//!
//! These routines use the A6xx command processor's event interface and
//! preserve the target firmware's absolute word-address control flow. All
//! emitted words come from typed ISA fields, not original instruction bytes.
//! They have **not** been tested on hardware or proven safe as replacements.

use qrisc::script::{BranchCondition as Branch, ImmediateOperation as Imm,
    RegisterOperation as Reg, Script};

pub const START_WORD: usize = 0x190;
pub const END_WORD: usize = 0x1e7;

const WAIT_FOR_IDLE_FN: u32 = 0x1e7;
const EVENT_START_FN: u32 = 0x1da;
const EVENT_END_FN: u32 = 0x1cb;

const WFI_PEND_INCR: u16 = 0x30;
const REG_WRITE_ADDR: u16 = 0x24;
const REG_WRITE: u16 = 0x25;
const REG_READ_DWORDS: u16 = 0x26;
const REG_READ_ADDR: u16 = 0x27;
const TRUSTED_BASE: u16 = 0xf800;
const TRUST_CONTROL: u16 = 0xf400;
const CP_EVENT_START: u16 = 0xd600;
const CP_EVENT_END: u16 = 0xd601;
const SP_EVENT_INITIATOR: u16 = 0xbb02;
const PC_EVENT_INITIATOR: u16 = 0x9842;

/// Increment outstanding work, dispatch a CP idle packet, and await the
/// shared idle subroutine. A matching decrement command follows the call.
fn wait_for_command_processor(p: &mut Script) {
    p.control_write(0, WFI_PEND_INCR);
    p.data_address(0x1c);
    p.data(1);
    p.call(WAIT_FOR_IDLE_FN);
    p.mov_shift(29, 0x81, 24);
}

fn restore_trusted_state(p: &mut Script) -> Result<(), String> {
    wait_for_command_processor(p);
    p.mov(3, TRUSTED_BASE);
    p.control_write(3, REG_WRITE_ADDR);
    for source in [11, 13, 14] { p.control_write(source, REG_WRITE); }
    p.call(EVENT_START_FN);
    p.mov(3, 0x31);
    wait_for_command_processor(p);
    p.mov(3, TRUST_CONTROL);
    p.control_write(3, REG_WRITE_ADDR);
    p.control_write(0, REG_WRITE);
    p.mov(4, 1);
    p.control_write(4, REG_READ_DWORDS);
    p.control_write(3, REG_READ_ADDR);
    // A failed secure-mode transition traps in the earlier halt loop.
    p.branch(Branch::NotEqualImmediate { register: 30, value: 0 }, 0x3e)?;
    p.expect_position(0x1a8)
}

fn inspect_and_finalize_state(p: &mut Script) -> Result<(), String> {
    p.mov(5, 2);
    p.control_write(5, 0x8b);
    p.control_read(3, 0x8b);
    p.imm(Imm::ShiftRightUnsigned, 3, 3, 25);
    p.imm(Imm::And, 3, 3, 1);
    p.register(Reg::Compare, 4, 3, 7);
    p.control_write(0, 0x8b);
    p.branch(Branch::BitSet { register: 4, bit: 0 }, 0x1bb)?;
    p.nop();
    // The conditional event notification is sent only when state changed.
    p.mov_shift(29, 0xe7, 24);
    p.imm(Imm::Or, 31, 0, 1);
    p.data_address(CP_EVENT_START);
    p.call(EVENT_END_FN);
    p.masked_data(6, 0xff);
    wait_for_command_processor(p);
    p.expect_position(0x1bb)?;
    p.control_write(8, 0x83); // restore previous secure state
    p.mov(3, 1);
    p.ret(true);
    p.control_write(9, 0x71); // restore preemption enable
    p.expect_position(0x1bf)
}

/// Initiate a GPU compute kernel event, synchronizing SP and PC engines.
fn kernel_event(p: &mut Script) -> Result<(), String> {
    p.mov_shift(29, 0xe7, 24);
    p.data(3);
    p.data_address(CP_EVENT_START);
    for address in [0xbb01, 0x9841] {
        p.masked_data(6, 0xff);
        p.data_address(address);
    }
    p.masked_data(6, 0xff);
    p.call(EVENT_END_FN);
    p.nop();
    p.ret(false);
    p.nop();
    p.expect_position(0x1cb)
}

/// Emit end-of-event markers and advance the event identifier, conditionally
/// toggling its high bit when the GPU event counter wraps its lower byte.
fn end_event(p: &mut Script) -> Result<(), String> {
    p.imm(Imm::And, 5, 6, 0xff);
    p.imm(Imm::ShiftLeft, 5, 5, 16);
    for address in [SP_EVENT_INITIATOR, PC_EVENT_INITIATOR] {
        p.data_address(address);
        p.imm(Imm::Or, 31, 5, 5);
    }
    p.data_address(0x9880); // PC marker
    p.imm(Imm::Or, 31, 0, 0x800);
    p.data_address(CP_EVENT_END);
    p.masked_data(6, 0xff);
    p.branch(Branch::BitClear { register: 6, bit: 8 }, 0x1d8)?;
    p.imm(Imm::Add, 6, 6, 1);
    p.imm(Imm::Xor, 6, 6, 0x100);
    p.ret(false);
    p.nop();
    p.expect_position(0x1da)
}

/// Write the event ID and command-specific flags to the SP and PC event ports.
fn begin_event(p: &mut Script) -> Result<(), String> {
    p.mov_shift(29, 0xe7, 24);
    p.data(1);
    p.data_address(CP_EVENT_START);
    p.masked_data(6, 0xff);
    p.imm(Imm::And, 4, 6, 0xff);
    p.imm(Imm::ShiftLeft, 4, 4, 16);
    p.data_address(SP_EVENT_INITIATOR);
    p.register(Reg::Or, 31, 4, 3);
    p.data_address(PC_EVENT_INITIATOR);
    p.call(EVENT_END_FN);
    p.register(Reg::Or, 31, 4, 3);
    p.ret(false);
    p.nop();
    p.expect_position(END_WORD)
}

/// Executable QRISC words `0x190..0x1e7`; the already-authored wait-for-idle
/// routine covers `0x1e7..0x1ec` in a separate reusable fragment.
pub fn finish_protected_compute() -> Result<Vec<u8>, String> {
    let mut p = Script::new(START_WORD);
    restore_trusted_state(&mut p)?;
    inspect_and_finalize_state(&mut p)?;
    kernel_event(&mut p)?;
    end_event(&mut p)?;
    begin_event(&mut p)?;
    p.finish(END_WORD)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emitted_routines_have_expected_targets_and_no_implicit_padding() {
        let bytes = finish_protected_compute().unwrap();
        assert_eq!(bytes.len(), (END_WORD - START_WORD) * 4);
        let word = |index: usize| u32::from_le_bytes(
            bytes[(index-START_WORD)*4..(index-START_WORD+1)*4].try_into().unwrap()
        );
        assert_eq!(qrisc::codec::QriscWord::decode(word(0x193)),
            Some(qrisc::codec::QriscWord::Call { target: WAIT_FOR_IDLE_FN }));
        assert_eq!(qrisc::codec::QriscWord::decode(word(0x1a7)),
            Some(qrisc::codec::QriscWord::Branch {
                opcode: 0x30, src: 30, condition: 0, delta: 0x3e - 0x1a7,
            }));
    }
}
