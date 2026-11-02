// SPDX-License-Identifier: GPL-3.0-only
//! Early ARM A32 Venus boot vector and status-polling routines.
//!
//! Source is a typed ARM32 implementation informed by a private reference
//! image; it does not contain an extracted vendor instruction array. This is
//! only the boot prefix, not a loadable or authenticated Venus firmware image.

use arm32::{branch, data_immediate, mov_register, push_registers, transfer_word,
            msr_cpsr_control, mcr, mov_shifted_register, data_register,
            data_immediate_with_flags, transfer_word_register, mrc, bx, Shift,
            transfer_multiple, msr_status_register, mrs_status_register,
            msr_cpsr_control_register, BlockMode,
            Condition as C, DataOp, Register as R};

pub const FILE_OFFSET: usize = 0x4000;
pub const PREFIX_SIZE: usize = 0x434;
pub const FIRST_PAGE_SIZE: usize = 0x1000;

/// Entry points and boot data inferred from the ROM's exception-vector table.
/// These target addresses are references, not bytes of vendor object code.
const VECTOR_TARGETS: [u32; 8] = [
    0x0000_0088, 0x0000_0050, 0x0000_0080, 0x0000_0060,
    0x0000_0070, 0x0000_0060, 0x000e_1f9c, 0x0000_02a4,
];
/// Fixed platform addresses and boot boundaries stored after vector literals.
const INITIALIZATION_LITERALS: [u32; 4] = [
    0x0011_37a8, 0x003b_c000, 0x004c_f7a8, 0x0002_0000,
];

fn emit(words: &mut Vec<u8>, expected_byte_offset: usize, instruction: u32) -> Result<(), String> {
    if words.len() != expected_byte_offset {
        return Err(format!("Venus boot layout mismatch at 0x{expected_byte_offset:x}"));
    }
    words.extend_from_slice(&instruction.to_le_bytes());
    Ok(())
}

fn stmia(code: &mut Vec<u8>, at: usize, base: R, regs: u16) -> Result<(), String> {
    emit(code, at, transfer_multiple(C::Al, false, base,
        regs, BlockMode::IncrementAfter, true)?)
}

fn pop(code: &mut Vec<u8>, at: usize, regs: u16) -> Result<(), String> {
    emit(code, at, transfer_multiple(C::Al, true, R::SP,
        regs, BlockMode::IncrementAfter, true)?)
}

/// Both exception-save entries reuse this register/status serialization.
fn save_core_exception_state(code: &mut Vec<u8>, start: usize) -> Result<(), String> {
    stmia(code, start, R::R12, R::R11.bit())?;
    stmia(code, start + 4, R::R12, 0x07ff)?;
    pop(code, start + 8, R::R0.bit() | R::R1.bit())?;
    stmia(code, start + 12, R::R12, R::R0.bit() | R::R1.bit())?;
    emit(code, start + 16, mov_register(C::Al, R::R0, R::SP))?;
    stmia(code, start + 20, R::R12, R::R0.bit())?;
    stmia(code, start + 24, R::R12, R::LR.bit())?;
    emit(code, start + 28, mov_register(C::Al, R::R0, R::PC))?;
    stmia(code, start + 32, R::R12, R::R0.bit())?;
    emit(code, start + 36, mrs_status_register(C::Al, false, R::R0))?;
    stmia(code, start + 40, R::R12, R::R0.bit())?;
    emit(code, start + 44, mov_register(C::Al, R::R1, R::R0))?;
    emit(code, start + 48, mrs_status_register(C::Al, true, R::R0))?;
    stmia(code, start + 52, R::R12, R::R0.bit())?;
    emit(code, start + 56, data_immediate(C::Al, DataOp::Tst, R::R0, R::R0, 15)?)?;
    emit(code, start + 60, data_immediate(C::Eq, DataOp::Orr, R::R0, R::R0, 15)?)?;
    emit(code, start + 64, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 32)?)?;
    Ok(())
}

pub fn prefix() -> Result<Vec<u8>, String> {
    let mut code = Vec::with_capacity(PREFIX_SIZE);
    for index in 0..8 {
        // The unused vector position holds a deliberate trap marker instead
        // of a PC-relative load. Keep it a named data literal.
        let word = if index == 5 { 0xdead_beef } else {
            transfer_word(C::Al, true, R::PC, R::PC, 24)?
        };
        emit(&mut code, index * 4, word)?;
    }
    for value in VECTOR_TARGETS { code.extend_from_slice(&value.to_le_bytes()); }
    for value in INITIALIZATION_LITERALS { code.extend_from_slice(&value.to_le_bytes()); }

    // Exception handlers pass an exception kind in R11 to the shared handler.
    for (start, reason) in [(0x50, 4), (0x60, 12), (0x70, 16)] {
        emit(&mut code, start, mov_register(C::Al, R::R0, R::R0))?;
        emit(&mut code, start + 4,
            push_registers(C::Al, R::R11.bit() | R::R12.bit())?)?;
        emit(&mut code, start + 8,
            data_immediate(C::Al, DataOp::Mov, R::R11, R::R0, reason)?)?;
        emit(&mut code, start + 12, branch(C::Al, (start + 12) as u32, 0x308, false)?)?;
    }
    emit(&mut code, 0x80, mov_register(C::Al, R::R0, R::R0))?;
    emit(&mut code, 0x84, branch(C::Al, 0x84, 0x80, false)?)?;

    // The 8-byte polling prelude contains data, not ARM instructions.
    emit(&mut code, 0x88, 0x0100_0202)?;
    emit(&mut code, 0x8c, 0xe00d_2048)?;
    emit(&mut code, 0x90, transfer_word(C::Al, true, R::R0, R::PC, -12)?)?;
    emit(&mut code, 0x94, transfer_word(C::Al, true, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x98, data_immediate(C::Al, DataOp::And, R::R1, R::R1, 1)?)?;
    emit(&mut code, 0x9c, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R1, 0)?)?;
    emit(&mut code, 0xa0, branch(C::Eq, 0xa0, 0x94, false)?)?;
    emit(&mut code, 0xa4, transfer_word(C::Al, true, R::R0, R::PC, 0x354)?)?;
    emit(&mut code, 0xa8, transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0xac, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0xb0, branch(C::Ne, 0xb0, 0x1f0, false)?)?;
    emit(&mut code, 0xb4, transfer_word(C::Al, true, R::R0, R::PC, 0x348)?)?;
    emit(&mut code, 0xb8, transfer_word(C::Al, true, R::R1, R::PC, -0x38)?)?;
    emit(&mut code, 0xbc, transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0xc0, transfer_word(C::Al, true, R::R0, R::PC, 0x340)?)?;
    // Configure a distinct stack for each exception mode. The positions and
    // stack displacements are represented as architecture-level instructions.
    for (at, cpsr, stack_delta) in [
        (0xc4, 0xd1, 0x000), (0xd0, 0xd2, 0x080), (0xdc, 0xd7, 0x180),
        (0xe8, 0xdb, 0x200), (0xf4, 0xdf, 0x280), (0x100, 0xd3, 0x680),
    ] {
        emit(&mut code, at, msr_cpsr_control(C::Al, cpsr)?)?;
        emit(&mut code, at + 4, data_immediate(C::Al, DataOp::Sub, R::R1, R::R0, stack_delta)?)?;
        emit(&mut code, at + 8, mov_register(C::Al, R::SP, R::R1))?;
    }
    emit(&mut code, 0x10c, branch(C::Al, 0x10c, 0x1000, true)?)?;
    emit(&mut code, 0x110, transfer_word(C::Al, true, R::R0, R::PC, 0x2f4)?)?;
    emit(&mut code, 0x114, mcr(C::Al, 15, 0, R::R0, 2, 0, 0)?)?;
    // MMIO addresses are literal data used by the subsequent bootstrap.
    for value in [0xe00e_102c_u32, 0xe00e_1030, 0xe00e_1034, 0xe00e_1024] {
        code.extend_from_slice(&value.to_le_bytes());
    }
    // Read four early memory-controller boundaries and convert their
    // physical addresses to page numbers by shifting 20 bits.
    for (i, literal_offset) in [12, 28, 44, 60].into_iter().enumerate() {
        let start = 0x128 + 12 * i;
        let target = R::new(4 + i as u8)?;
        emit(&mut code, start, transfer_word(C::Al, true, R::R1, R::PC, -literal_offset)?)?;
        emit(&mut code, start + 4, transfer_word(C::Al, true, target, R::R1, 0)?)?;
        emit(&mut code, start + 8, mov_shifted_register(C::Al, target, target, Shift::Lsr, 20)?)?;
    }
    emit(&mut code, 0x158, transfer_word(C::Al, true, R::R1, R::PC, 0x2b0)?)?;
    // Early section-table walk: compute a set of page-attribute flags,
    // write table entries, and switch writable/cachable policies at four
    // boundaries. All branches use symbolic byte offsets.
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    emit(&mut code, 0x15c, data_immediate(C::Al, DataOp::Mov, R::R2, R::R0, 18)?)?;
    emit(&mut code, 0x160, data_immediate(C::Al, DataOp::Orr, r8, R::R2, 3552)?)?;
    emit(&mut code, 0x164, data_immediate(C::Al, DataOp::Bic, r9, R::R2, 3)?)?;
    emit(&mut code, 0x168, mov_register(C::Al, R::R2, r8))?;
    emit(&mut code, 0x16c, data_register(C::Al, DataOp::Orr, r3, R::R2, R::R1, Shift::Lsl, 20, false)?)?;
    emit(&mut code, 0x170, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R1, 3584)?)?;
    emit(&mut code, 0x174, data_immediate(C::Lt, DataOp::Orr, r3, r3, 12)?)?;
    emit(&mut code, 0x178, transfer_word_register(C::Al, false, r3, R::R0, R::R1, Shift::Lsl, 2)?)?;
    emit(&mut code, 0x17c, data_immediate_with_flags(C::Al, DataOp::Sub, R::R1, R::R1, 1, true)?)?;
    emit(&mut code, 0x180, branch(C::Mi, 0x180, 0x1b8, false)?)?;
    for (at, bound, replacement, branch_at) in [
        (0x184, r4, None, 0x188),
        (0x18c, r5, Some(r9), 0x194),
        (0x198, r6, Some(r8), 0x1a0),
        (0x1a4, r7, Some(r9), 0x1ac),
    ] {
        emit(&mut code, at, data_register(C::Al, DataOp::Cmp, R::R0, R::R1, bound, Shift::Lsl, 0, false)?)?;
        if let Some(src) = replacement {
            emit(&mut code, at + 4, mov_register(C::Pl, R::R2, src))?;
        }
        emit(&mut code, branch_at, branch(C::Pl, branch_at as u32, 0x16c, false)?)?;
    }
    emit(&mut code, 0x1b0, mov_register(C::Al, R::R2, r8))?;
    emit(&mut code, 0x1b4, branch(C::Al, 0x1b4, 0x16c, false)?)?;
    emit(&mut code, 0x1b8, data_immediate(C::Al, DataOp::Bic, r3, r3, 3072)?)?;
    emit(&mut code, 0x1bc, transfer_word(C::Al, false, r3, R::R0, 0)?)?;
    // Enable the initial access-control and memory-management settings,
    // then transfer control to the address loaded from the boot literal pool.
    emit(&mut code, 0x1c0, branch(C::Al, 0x1c0, 0x1888, true)?)?;
    emit(&mut code, 0x1c4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0x4000_0000)?)?;
    emit(&mut code, 0x1c8, mcr(C::Al, 15, 0, R::R0, 3, 0, 0)?)?;
    emit(&mut code, 0x1cc, transfer_word(C::Al, true, R::R12, R::PC, 0x240)?)?;
    emit(&mut code, 0x1d0, mrc(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    emit(&mut code, 0x1d4, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 4096)?)?;
    emit(&mut code, 0x1d8, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 4)?)?;
    emit(&mut code, 0x1dc, data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x1e0, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 256)?)?;
    emit(&mut code, 0x1e4, data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 512)?)?;
    emit(&mut code, 0x1e8, mcr(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    emit(&mut code, 0x1ec, bx(C::Al, R::R12))?;
    // Restore banked exception-mode stack registers, SPSR and MMU control
    // values from the saved-processor-state record.
    let saved_mode_registers = R::R0.bit() | R::SP.bit() | R::LR.bit();
    emit(&mut code, 0x1f0, transfer_word(C::Al, true, R::R0, R::PC, 0x220)?)?;
    emit(&mut code, 0x1f4, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 1)?)?;
    emit(&mut code, 0x1f8, transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x1fc, msr_cpsr_control(C::Al, 0xd1)?)?;
    emit(&mut code, 0x200, transfer_word(C::Al, true, R::R12, R::PC, 0x214)?)?;
    emit(&mut code, 0x204, data_immediate(C::Al, DataOp::Add, R::R12, R::R12, 124)?)?;
    emit(&mut code, 0x208, transfer_multiple(C::Al, true, R::R12,
        saved_mode_registers, BlockMode::DecrementAfter, true)?)?;
    emit(&mut code, 0x20c, msr_status_register(C::Al, true, R::R0))?;
    for (at, mode) in [(0x210, 0xdf), (0x224, 0xd2), (0x230, 0xd7), (0x23c, 0xdb)] {
        emit(&mut code, at, msr_cpsr_control(C::Al, mode)?)?;
        if at == 0x210 {
            emit(&mut code, 0x214, transfer_word(C::Al, true, R::R12, R::PC, 0x204)?)?;
            emit(&mut code, 0x218, data_immediate(C::Al, DataOp::Add, R::R12, R::R12, 112)?)?;
            emit(&mut code, 0x21c, transfer_multiple(C::Al, true, R::R12,
                saved_mode_registers, BlockMode::DecrementAfter, true)?)?;
            emit(&mut code, 0x220, msr_status_register(C::Al, true, R::R0))?;
        } else {
            emit(&mut code, at + 4, transfer_multiple(C::Al, true, R::R12,
                saved_mode_registers, BlockMode::DecrementAfter, true)?)?;
            emit(&mut code, at + 8, msr_status_register(C::Al, true, R::R0))?;
        }
    }
    emit(&mut code, 0x248, msr_cpsr_control(C::Al, 0xd3)?)?;
    emit(&mut code, 0x24c, transfer_word(C::Al, true, R::R0, R::PC, 0x1d0)?)?;
    emit(&mut code, 0x250, mcr(C::Al, 15, 0, R::R0, 2, 0, 0)?)?;
    emit(&mut code, 0x254, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0x4000_0000)?)?;
    emit(&mut code, 0x258, mcr(C::Al, 15, 0, R::R0, 3, 0, 0)?)?;
    emit(&mut code, 0x25c, mrc(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    emit(&mut code, 0x260, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 4096)?)?;
    emit(&mut code, 0x264, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 4)?)?;
    emit(&mut code, 0x268, data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x26c, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 256)?)?;
    emit(&mut code, 0x270, data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 512)?)?;
    emit(&mut code, 0x274, mcr(C::Al, 15, 0, R::R0, 1, 0, 0)?)?;
    for at in [0x278, 0x27c, 0x280] {
        emit(&mut code, at, mov_register(C::Al, R::R0, R::R0))?;
    }
    emit(&mut code, 0x284, transfer_multiple(C::Al, true, R::R12,
        R::R1.bit() | R::R2.bit(), BlockMode::DecrementAfter, true)?)?;
    emit(&mut code, 0x288, msr_status_register(C::Al, true, R::R1))?;
    emit(&mut code, 0x28c, msr_status_register(C::Al, false, R::R2))?;
    emit(&mut code, 0x290, transfer_multiple(C::Al, true, R::R12,
        R::SP.bit() | R::LR.bit(), BlockMode::DecrementAfter, true)?)?;
    emit(&mut code, 0x294, data_immediate(C::Al, DataOp::Sub, R::R12, R::R12, 4)?)?;
    emit(&mut code, 0x298, transfer_multiple(C::Al, true, R::R12,
        0x0fff, BlockMode::DecrementAfter, true)?)?;
    emit(&mut code, 0x29c, transfer_word(C::Al, true, R::R12, R::R12, 0x34)?)?;
    emit(&mut code, 0x2a0, bx(C::Al, R::LR))?;
    // FIQ/other exception recording and deliberately nonreturning diagnostics.
    emit(&mut code, 0x2a4, mov_register(C::Al, R::R0, R::R0))?;
    emit(&mut code, 0x2a8, push_registers(C::Al, R::R11.bit() | R::R12.bit())?)?;
    emit(&mut code, 0x2ac, data_immediate(C::Al, DataOp::Mov, R::R11, R::R0, 20)?)?;
    emit(&mut code, 0x2b0, transfer_word(C::Al, true, R::R12, R::PC, 0x170)?)?;
    save_core_exception_state(&mut code, 0x2b4)?;
    emit(&mut code, 0x2f8, branch(C::Al, 0x2f8, 0x1504, true)?)?;
    emit(&mut code, 0x2fc, branch(C::Al, 0x2fc, 0x1398, true)?)?;
    emit(&mut code, 0x300, mov_register(C::Al, R::R0, R::R0))?;
    emit(&mut code, 0x304, branch(C::Al, 0x304, 0x300, false)?)?;

    emit(&mut code, 0x308, transfer_word(C::Al, true, R::R12, R::PC, 0x11c)?)?;
    save_core_exception_state(&mut code, 0x30c)?;
    emit(&mut code, 0x350, msr_cpsr_control_register(C::Al, R::R0))?;
    emit(&mut code, 0x354, mov_register(C::Al, R::R2, R::SP))?;
    emit(&mut code, 0x358, mov_register(C::Al, R::new(3)?, R::LR))?;
    emit(&mut code, 0x35c, msr_cpsr_control_register(C::Al, R::R1))?;
    stmia(&mut code, 0x360, R::R12, R::R2.bit() | R::new(3)?.bit())?;
    emit(&mut code, 0x364, data_immediate(C::Al, DataOp::Sub, R::R0, R::LR, 8)?)?;
    stmia(&mut code, 0x368, R::R12, R::R0.bit())?;
    emit(&mut code, 0x36c, mrc(C::Al, 15, 0, R::R0, 5, 0, 0)?)?;
    emit(&mut code, 0x370, mrc(C::Al, 15, 0, R::R1, 5, 0, 1)?)?;
    emit(&mut code, 0x374, mrc(C::Al, 15, 0, R::R2, 6, 0, 0)?)?;
    stmia(&mut code, 0x378, R::R12, R::R0.bit() | R::R1.bit() | R::R2.bit())?;
    emit(&mut code, 0x37c, branch(C::Al, 0x37c, 0x1504, true)?)?;
    emit(&mut code, 0x380, branch(C::Al, 0x380, 0x1398, true)?)?;
    emit(&mut code, 0x384, mov_register(C::Al, R::R0, R::R0))?;
    emit(&mut code, 0x388, branch(C::Al, 0x388, 0x384, false)?)?;

    // Save a complete mode snapshot for diagnostic and restart services.
    emit(&mut code, 0x38c, push_registers(C::Al, R::R12.bit() | R::LR.bit())?)?;
    emit(&mut code, 0x390, transfer_word(C::Al, true, R::R12, R::PC, 0x98)?)?;
    stmia(&mut code, 0x394, R::R12, 0x0fff)?;
    pop(&mut code, 0x398, R::R0.bit() | R::R1.bit())?;
    stmia(&mut code, 0x39c, R::R12, R::R0.bit())?;
    emit(&mut code, 0x3a0, mov_register(C::Al, R::R0, R::SP))?;
    stmia(&mut code, 0x3a4, R::R12, R::R0.bit() | R::R1.bit())?;
    emit(&mut code, 0x3a8, mrs_status_register(C::Al, true, R::R0))?;
    stmia(&mut code, 0x3ac, R::R12, R::R0.bit())?;
    emit(&mut code, 0x3b0, mrs_status_register(C::Al, false, R::R0))?;
    stmia(&mut code, 0x3b4, R::R12, R::R0.bit())?;
    for (at, mode) in [(0x3b8, 0xdb), (0x3c4, 0xd7), (0x3d0, 0xd2), (0x3dc, 0xdf)] {
        emit(&mut code, at, msr_cpsr_control(C::Al, mode)?)?;
        emit(&mut code, at + 4, mrs_status_register(C::Al, true, R::R1))?;
        stmia(&mut code, at + 8, R::R12,
            R::R1.bit() | R::SP.bit() | R::LR.bit())?;
    }
    emit(&mut code, 0x3e8, mov_register(C::Al, R::R2, R::R12))?;
    emit(&mut code, 0x3ec, msr_cpsr_control(C::Al, 0xd1)?)?;
    emit(&mut code, 0x3f0, mrs_status_register(C::Al, true, R::R1))?;
    stmia(&mut code, 0x3f4, R::R2, R::R1.bit() | R::SP.bit() | R::LR.bit())?;
    emit(&mut code, 0x3f8, msr_cpsr_control_register(C::Al, R::R0))?;
    emit(&mut code, 0x3fc, bx(C::Al, R::LR))?;

    // Final boot literal pool contains platform addresses, table bounds and
    // pointers. These are data words, not an embedded OEM instruction stream.
    const PLATFORM_LITERALS: [u32; 13] = [
        0x0010_3074, 0x004e_ffac, 0x004e_ffa8, 0x004f_0000,
        0x0000_0fff, 0x0000_30f0, 0x0010_3078, 0x0010_cf08,
        0x0010_cf08, 0x004f_0000, 0x0010_ce48, 0x0010_ce48,
        0x0010_cf08,
    ];
    for value in PLATFORM_LITERALS {
        code.extend_from_slice(&value.to_le_bytes());
    }
    if code.len() != PREFIX_SIZE { return Err("Venus boot prefix size mismatch".into()); }
    Ok(code)
}

/// Produce the first complete 4 KiB Venus code-and-data boot page.
/// The remaining bytes are documented zero-fill and not missing code.
pub fn first_page() -> Result<Vec<u8>, String> {
    let mut page = prefix()?;
    page.resize(FIRST_PAGE_SIZE, 0);
    Ok(page)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_vector_table_and_polling_layout() {
        let output = prefix().unwrap();
        assert_eq!(output.len(), PREFIX_SIZE);
        let page = first_page().unwrap();
        assert_eq!(page.len(), FIRST_PAGE_SIZE);
        assert!(page[PREFIX_SIZE..].iter().all(|&byte| byte == 0));
        assert_eq!(&output[0x14..0x18], &0xdead_beefu32.to_le_bytes());
        assert_eq!(&output[0x88..0x8c], &0x0100_0202u32.to_le_bytes());
        assert_eq!(&output[0x50..0x54], &0xe1a0_0000u32.to_le_bytes());
        assert_eq!(&output[0xa0..0xa4], &0x0aff_fffbu32.to_le_bytes());
    }
}
