// SPDX-License-Identifier: GPL-3.0-only
//! Reconstructed ARM A32 entry stubs in the Venus runtime region.
//! These operations model public ARM encodings and named entry points; they
//! are insufficient to produce a complete signed or working Venus image.
use arm32::{branch, bx, data_immediate, mov_register, push_registers,
            transfer_multiple, transfer_word, transfer_word_register,
            mov_shifted_register, data_register, Shift, BlockMode, Condition as C,
            DataOp, Register as R};

pub const FILE_OFFSET: usize = 0x5000;
pub const PREFIX_SIZE: usize = 0x1afc;

fn emit(dst: &mut Vec<u8>, at: usize, instruction: u32) -> Result<(), String> {
    if dst.len() != at { return Err(format!("Venus runtime layout mismatch at 0x{at:x}")); }
    dst.extend_from_slice(&instruction.to_le_bytes());
    Ok(())
}


/// Serialize the hardware diagnostics record and forward it to the shared
/// firmware logging services. Both observed event handlers share the same
/// control flow but differ in the record subtype field.
fn record_hardware_event(code: &mut Vec<u8>, start: usize,
                         record_type: u32) -> Result<(), String> {
    let r4 = R::new(4)?;
    let r7 = R::new(7)?;
    let mut at = start;
    macro_rules! op {
        ($ins:expr) => {{ emit(code, at, $ins)?; at += 4; }};
    }
    op!(push_registers(C::Al, r4.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?);
    op!(data_immediate(C::Al, DataOp::Add, R::R11, R::SP, 8)?);
    op!(data_immediate(C::Al, DataOp::Sub, R::SP, R::SP, 32)?);
    op!(transfer_word(C::Al, true, R::R0, R::PC,
        if record_type == 17 { 0x88 } else { 0x84 })?);
    op!(transfer_word(C::Al, true, r4, R::R0, 0)?);
    op!(data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 1)?);
    op!(data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 135168)?);
    op!(transfer_word(C::Al, false, R::R0, R::SP, 8)?);
    op!(data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 255)?);
    op!(transfer_word(C::Al, false, R::R0, R::SP, 12)?);
    op!(data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 1)?);
    op!(transfer_word(C::Al, false, R::R0, R::SP, 16)?);
    if record_type != 1 {
        op!(data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, record_type)?);
    }
    op!(transfer_word(C::Al, false, R::R0, R::SP, 20)?);
    op!(transfer_word(C::Al, true, R::R0, R::PC, 0x60)?);
    op!(transfer_word(C::Al, true, R::R0, R::R0, 0)?);
    op!(transfer_word(C::Al, false, R::R0, R::SP, 24)?);
    op!(data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 28)?);
    op!(transfer_word(C::Al, false, R::R0, R::SP, 4)?);
    op!(data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?);
    op!(transfer_word(C::Al, false, R::R0, R::SP, 28)?);
    op!(branch(C::Al, at as u32, 0x10a0, true)?);
    op!(branch(C::Al, at as u32, 0x24734, true)?);
    op!(transfer_word(C::Al, true, R::R0, r4, 28)?);
    op!(data_immediate(C::Al, DataOp::Add, R::R1, R::SP, 4)?);
    op!(mov_register(C::Al, R::R2, R::SP));
    op!(branch(C::Al, at as u32, 0x2e1e4, true)?);
    op!(transfer_word(C::Al, true, R::R0, r4, 36)?);
    op!(data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?);
    op!(branch(C::Eq, at as u32, (at + 16) as u32, false)?);
    op!(data_immediate(C::Al, DataOp::Add, R::R1, R::SP, 4)?);
    op!(mov_register(C::Al, R::R2, R::SP));
    op!(branch(C::Al, at as u32, 0x2e1e4, true)?);
    op!(transfer_word(C::Al, true, R::R0, r4, 0x80)?);
    op!(data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 1)?);
    op!(branch(C::Ne, at as u32, (at + 8) as u32, false)?);
    op!(branch(C::Al, at as u32, 0x29158, true)?);
    op!(data_immediate(C::Al, DataOp::Sub, R::SP, R::R11, 8)?);
    op!(transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r7.bit() | R::R11.bit() | R::PC.bit(),
        BlockMode::IncrementAfter, true)?);
    // These are pointers to a runtime diagnostics object and interrupt-state
    // location. They are data operands, not copied vendor instructions.
    op!(0x0010_ceac);
    op!(0x0010_307c);
    if at != if record_type == 17 { 0x4dc } else { 0x438 } {
        return Err("Venus event-handler layout mismatch".into());
    }
    Ok(())
}

pub fn prefix() -> Result<Vec<u8>, String> {
    let mut code = Vec::with_capacity(PREFIX_SIZE);
    let frame = R::R11.bit() | R::LR.bit();
    emit(&mut code, 0x00, push_registers(C::Al, frame)?)?;
    emit(&mut code, 0x04, mov_register(C::Al, R::R11, R::SP))?;
    emit(&mut code, 0x08, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x0c, branch(C::Al, 0x0c, 0x247b0, true)?)?;
    emit(&mut code, 0x10, transfer_word(C::Al, true, R::R1, R::PC, 4)?)?;
    emit(&mut code, 0x14, transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    emit(&mut code, 0x18, transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // Literal pool: device-specific status-address pointer, not code.
    emit(&mut code, 0x1c, 0x0010_3068)?;

    emit(&mut code, 0x20, push_registers(C::Al, frame)?)?;
    emit(&mut code, 0x24, mov_register(C::Al, R::R11, R::SP))?;
    emit(&mut code, 0x28, branch(C::Al, 0x28, 0xec0, true)?)?;
    emit(&mut code, 0x2c, branch(C::Al, 0x2c, 0xf34, true)?)?;
    emit(&mut code, 0x30, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x34, transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    // Small control-service stubs return status values without touching DSP
    // firmware metadata. Their return branches are modeled explicitly.
    for (start, value) in [
        (0x38, 256), (0x40, 0), (0x48, 0), (0x50, 0),
    ] {
        emit(&mut code, start, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, value)?)?;
        emit(&mut code, start + 4, bx(C::Al, R::LR))?;
    }
    emit(&mut code, 0x58, bx(C::Al, R::LR))?;
    emit(&mut code, 0x5c, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x60, bx(C::Al, R::LR))?;
    emit(&mut code, 0x64, data_immediate(C::Al, DataOp::Mvn, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x68, bx(C::Al, R::LR))?;
    for start in [0x6c, 0x74, 0x7c] {
        emit(&mut code, start, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
        emit(&mut code, start + 4, bx(C::Al, R::LR))?;
    }
    emit(&mut code, 0x84, branch(C::Al, 0x84, 0x84, false)?)?;
    emit(&mut code, 0x88, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x8c, bx(C::Al, R::LR))?;

    emit(&mut code, 0x90, push_registers(C::Al, frame)?)?;
    emit(&mut code, 0x94, mov_register(C::Al, R::R11, R::SP))?;
    emit(&mut code, 0x98, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x9c, branch(C::Al, 0x9c, 0x10e8, true)?)?;
    emit(&mut code, 0xa0, transfer_word(C::Al, true, R::R0, R::PC, 0x10)?)?;
    emit(&mut code, 0xa4, transfer_word(C::Al, true, R::R2, R::PC, 0x10)?)?;
    emit(&mut code, 0xa8, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0xac, branch(C::Al, 0xac, 0x21ec, true)?)?;
    emit(&mut code, 0xb0, transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    emit(&mut code, 0xb4, branch(C::Al, 0xb4, 0x20, false)?)?;
    // Runtime object and control-parameter literal pool.
    emit(&mut code, 0xb8, 0x0010_ce48)?;
    emit(&mut code, 0xbc, 0x0000_295c)?;

    // Handle diagnostic/health faults using two MMIO paths. Read source
    // fields, normalize status bits and call shared low-level event services.
    let r3 = R::new(3)?;
    let r4 = R::new(4)?;
    let r5 = R::new(5)?;
    emit(&mut code, 0xc0, push_registers(C::Al,
        r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit())?)?;
    emit(&mut code, 0xc4, data_immediate(C::Al, DataOp::Add, R::R11, R::SP, 8)?)?;
    emit(&mut code, 0xc8, data_immediate(C::Al, DataOp::Sub, R::SP, R::SP, 16)?)?;
    emit(&mut code, 0xcc, data_immediate(C::Al, DataOp::Tst, R::R0, R::R1, 128)?)?;
    emit(&mut code, 0xd0, branch(C::Eq, 0xd0, 0x13c, false)?)?;
    emit(&mut code, 0xd4, transfer_word(C::Al, true, R::R2, R::PC, 0x114)?)?;
    emit(&mut code, 0xd8, data_immediate(C::Al, DataOp::Mov, r3, R::R0, 13568)?)?;
    emit(&mut code, 0xdc, transfer_word_register(C::Al, true, r3, R::R2, r3, Shift::Lsl, 0)?)?;
    emit(&mut code, 0xe0, data_immediate(C::Al, DataOp::Cmp, R::R0, r3, 0)?)?;
    emit(&mut code, 0xe4, branch(C::Eq, 0xe4, 0x13c, false)?)?;
    emit(&mut code, 0xe8, transfer_word(C::Al, true, r4, R::PC, 0x104)?)?;
    emit(&mut code, 0xec, transfer_word(C::Al, true, r3, r4, 0x18)?)?;
    emit(&mut code, 0xf0, transfer_word(C::Al, true, R::R12, r4, 0x20)?)?;
    emit(&mut code, 0xf4, transfer_word(C::Al, true, R::R1, R::PC, 0xfc)?)?;
    emit(&mut code, 0xf8, transfer_word(C::Al, true, R::LR, R::R1, 0)?)?;
    emit(&mut code, 0xfc, transfer_word(C::Al, true, R::R1, R::PC, 0xf8)?)?;
    emit(&mut code, 0x100, transfer_word(C::Al, true, r5, R::R1, 0)?)?;
    emit(&mut code, 0x104, transfer_word(C::Al, true, R::R2, R::R2, 0)?)?;
    emit(&mut code, 0x108, mov_shifted_register(C::Al, R::R1, R::LR, Shift::Lsr, 1)?)?;
    emit(&mut code, 0x10c, transfer_word(C::Al, false, R::R12, R::SP, 0)?)?;
    emit(&mut code, 0x110, transfer_multiple(C::Al, false, R::SP,
        R::R1.bit() | r5.bit(), BlockMode::IncrementBefore, false)?)?;
    emit(&mut code, 0x114, transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    emit(&mut code, 0x118, transfer_word(C::Al, true, R::R2, R::PC, 0xe0)?)?;
    emit(&mut code, 0x11c, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 100)?)?;
    emit(&mut code, 0x120, branch(C::Al, 0x120, 0x21c0, true)?)?;
    emit(&mut code, 0x124, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x128, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x12c, branch(C::Al, 0x12c, 0x262e4, true)?)?;
    emit(&mut code, 0x130, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x134, branch(C::Al, 0x134, 0x264ac, true)?)?;
    emit(&mut code, 0x138, branch(C::Al, 0x138, 0x1b0, false)?)?;
    emit(&mut code, 0x13c, data_immediate(C::Al, DataOp::Tst, R::R0, R::R1, 256)?)?;
    emit(&mut code, 0x140, branch(C::Eq, 0x140, 0x1c0, false)?)?;
    emit(&mut code, 0x144, transfer_word(C::Al, true, r4, R::PC, 0xb8)?)?;
    emit(&mut code, 0x148, transfer_word(C::Al, true, R::R2, r4, 0)?)?;
    emit(&mut code, 0x14c, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R2, 0)?)?;
    emit(&mut code, 0x150, branch(C::Eq, 0x150, 0x1c0, false)?)?;
    emit(&mut code, 0x154, transfer_word(C::Al, true, r3, r4, 0x20)?)?;
    emit(&mut code, 0x158, transfer_word(C::Al, true, R::R12, r4, 0x28)?)?;
    emit(&mut code, 0x15c, transfer_word(C::Al, true, R::R2, R::PC, 0xa4)?)?;
    emit(&mut code, 0x160, data_immediate(C::Al, DataOp::Mov, R::LR, R::R0, 1)?)?;
    emit(&mut code, 0x164, transfer_word(C::Al, true, r5, R::R2, 0)?)?;
    emit(&mut code, 0x168, transfer_word(C::Al, true, R::R1, R::R2, 0)?)?;
    emit(&mut code, 0x16c, transfer_word(C::Al, true, R::R2, R::R2, 0)?)?;
    emit(&mut code, 0x170, transfer_word(C::Al, false, R::R12, R::SP, 0)?)?;
    emit(&mut code, 0x174, data_register(C::Al, DataOp::And, r5, R::LR, r5, Shift::Lsr, 2, false)?)?;
    emit(&mut code, 0x178, data_immediate(C::Al, DataOp::And, R::R2, R::R2, 1)?)?;
    emit(&mut code, 0x17c, data_register(C::Al, DataOp::And, R::R1, R::LR, R::R1, Shift::Lsr, 1, false)?)?;
    emit(&mut code, 0x180, transfer_word(C::Al, false, r5, R::SP, 4)?)?;
    emit(&mut code, 0x184, transfer_word(C::Al, false, R::R1, R::SP, 8)?)?;
    emit(&mut code, 0x188, transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    emit(&mut code, 0x18c, transfer_word(C::Al, true, R::R2, R::PC, 0x78)?)?;
    emit(&mut code, 0x190, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 100)?)?;
    emit(&mut code, 0x194, branch(C::Al, 0x194, 0x21c0, true)?)?;
    emit(&mut code, 0x198, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x19c, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x1a0, branch(C::Al, 0x1a0, 0x262e4, true)?)?;
    emit(&mut code, 0x1a4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x1a8, branch(C::Al, 0x1a8, 0x264ac, true)?)?;
    emit(&mut code, 0x1ac, data_immediate(C::Al, DataOp::Add, r4, r4, 8)?)?;
    emit(&mut code, 0x1b0, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x1b4, transfer_word(C::Al, false, R::R0, r4, 0)?)?;
    emit(&mut code, 0x1b8, data_immediate(C::Al, DataOp::Sub, R::SP, R::R11, 8)?)?;
    emit(&mut code, 0x1bc, transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    emit(&mut code, 0x1c0, data_immediate(C::Al, DataOp::Tst, R::R0, R::R1, 2)?)?;
    emit(&mut code, 0x1c4, branch(C::Eq, 0x1c4, 0x1d4, false)?)?;
    emit(&mut code, 0x1c8, transfer_word(C::Al, true, R::R2, R::PC, 0x40)?)?;
    emit(&mut code, 0x1cc, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 100)?)?;
    emit(&mut code, 0x1d0, branch(C::Al, 0x1d0, 0x21c0, true)?)?;
    emit(&mut code, 0x1d4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x1d8, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x1dc, branch(C::Al, 0x1dc, 0x262e4, true)?)?;
    emit(&mut code, 0x1e0, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x1e4, data_immediate(C::Al, DataOp::Sub, R::SP, R::R11, 8)?)?;
    emit(&mut code, 0x1e8, transfer_multiple(C::Al, true, R::SP,
        r4.bit() | r5.bit() | R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    emit(&mut code, 0x1ec, branch(C::Al, 0x1ec, 0x264ac, false)?)?;
    // Named MMIO pointers and firmware service identifiers used by event logging.
    const EVENT_LITERALS: [u32; 9] = [
        0xe000_1010, 0xe000_4518, 0xe000_0824,
        0xe000_040c, 0x000e_257a, 0xe000_c510,
        0xe000_8060, 0x000e_25be, 0x000e_25ff,
    ];
    for value in EVENT_LITERALS { code.extend_from_slice(&value.to_le_bytes()); }

    // Early runtime's event loop. PC-relative literals name MMIO/status
    // locations; branch targets are expressed as addresses, never raw opcodes.
    let r6 = R::new(6)?;
    let r7 = R::new(7)?;
    let r8 = R::new(8)?;
    let r9 = R::new(9)?;
    let r10 = R::new(10)?;
    emit(&mut code, 0x214, push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | r8.bit() |
        r9.bit() | r10.bit() | R::R11.bit() | R::LR.bit())?)?;
    emit(&mut code, 0x218, data_immediate(C::Al, DataOp::Add, R::R11, R::SP, 28)?)?;
    emit(&mut code, 0x21c, data_immediate(C::Al, DataOp::Sub, R::SP, R::SP, 4)?)?;
    emit(&mut code, 0x220, transfer_word(C::Al, true, r4, R::PC, 0x154)?)?;
    emit(&mut code, 0x224, transfer_word(C::Al, true, r5, R::PC, 0x154)?)?;
    emit(&mut code, 0x228, transfer_word(C::Al, true, r10, R::PC, 0x15c)?)?;
    emit(&mut code, 0x22c, data_immediate(C::Al, DataOp::Mov, r6, R::R0, 0)?)?;
    emit(&mut code, 0x230, transfer_word(C::Al, true, R::R0, r4, 0)?)?;
    emit(&mut code, 0x234, data_immediate(C::Al, DataOp::Tst, R::R0, R::R0, 1 << 24)?)?;
    emit(&mut code, 0x238, branch(C::Ne, 0x238, 0x230, false)?)?;
    emit(&mut code, 0x23c, transfer_word(C::Al, true, R::R0, r5, 0)?)?;
    emit(&mut code, 0x240, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x244, branch(C::Ne, 0x244, 0x250, false)?)?;
    emit(&mut code, 0x248, arm32::mcr(C::Al, 15, 0, r6, 7, 0, 4)?)?;
    emit(&mut code, 0x24c, branch(C::Al, 0x24c, 0x230, false)?)?;
    emit(&mut code, 0x250, branch(C::Al, 0x250, 0x7e98, true)?)?;
    emit(&mut code, 0x254, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x258, branch(C::Al, 0x258, 0x26180, true)?)?;
    emit(&mut code, 0x25c, transfer_word(C::Al, true, r9, R::PC, 0x120)?)?;
    emit(&mut code, 0x260, transfer_word(C::Al, true, r7, R::PC, 0x120)?)?;
    emit(&mut code, 0x264, data_immediate(C::Al, DataOp::Mov, r8, R::R0, 3)?)?;
    emit(&mut code, 0x268, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x26c, branch(C::Eq, 0x26c, 0x2b0, false)?)?;
    emit(&mut code, 0x270, branch(C::Al, 0x270, 0x25ac4, true)?)?;
    emit(&mut code, 0x274, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x278, branch(C::Eq, 0x278, 0x304, false)?)?;
    emit(&mut code, 0x27c, data_immediate(C::Al, DataOp::Cmp, R::R0, r8, 3)?)?;
    emit(&mut code, 0x280, branch(C::Ne, 0x280, 0x328, false)?)?;
    emit(&mut code, 0x284, transfer_word(C::Al, true, R::R0, r10, 0)?)?;
    emit(&mut code, 0x288, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x28c, branch(C::Ne, 0x28c, 0x294, false)?)?;
    emit(&mut code, 0x290, branch(C::Al, 0x290, 0x24c48, true)?)?;
    emit(&mut code, 0x294, transfer_word(C::Al, true, R::R0, r7, 0)?)?;
    emit(&mut code, 0x298, data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 1 << 30)?)?;
    emit(&mut code, 0x29c, transfer_word(C::Al, false, R::R0, r7, 0)?)?;
    emit(&mut code, 0x2a0, branch(C::Al, 0x2a0, 0x20cc, true)?)?;
    emit(&mut code, 0x2a4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 32)?)?;
    emit(&mut code, 0x2a8, transfer_word(C::Al, false, R::R0, r9, 0)?)?;
    emit(&mut code, 0x2ac, branch(C::Al, 0x2ac, 0x328, false)?)?;
    emit(&mut code, 0x2b0, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2b4, data_immediate(C::Al, DataOp::Mov, r8, R::R0, 0)?)?;
    emit(&mut code, 0x2b8, branch(C::Al, 0x2b8, 0x25474, true)?)?;
    emit(&mut code, 0x2bc, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2c0, branch(C::Eq, 0x2c0, 0x270, false)?)?;
    emit(&mut code, 0x2c4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2c8, branch(C::Al, 0x2c8, 0x25488, true)?)?;
    emit(&mut code, 0x2cc, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2d0, branch(C::Al, 0x2d0, 0x25438, true)?)?;
    emit(&mut code, 0x2d4, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2d8, branch(C::Eq, 0x2d8, 0x36c, false)?)?;
    emit(&mut code, 0x2dc, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2e0, branch(C::Al, 0x2e0, 0x2544c, true)?)?;
    emit(&mut code, 0x2e4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2e8, branch(C::Al, 0x2e8, 0x254b0, true)?)?;
    emit(&mut code, 0x2ec, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2f0, branch(C::Eq, 0x2f0, 0x374, false)?)?;
    emit(&mut code, 0x2f4, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x2f8, branch(C::Al, 0x2f8, 0x254c4, true)?)?;
    emit(&mut code, 0x2fc, data_immediate(C::Al, DataOp::Mov, r8, R::R0, 3)?)?;
    emit(&mut code, 0x300, branch(C::Al, 0x300, 0x270, false)?)?;
    emit(&mut code, 0x304, branch(C::Al, 0x304, 0x24964, true)?)?;
    emit(&mut code, 0x308, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x30c, branch(C::Eq, 0x30c, 0x328, false)?)?;
    emit(&mut code, 0x310, branch(C::Al, 0x310, 0x24974, true)?)?;
    emit(&mut code, 0x314, branch(C::Al, 0x314, 0x24994, true)?)?;
    emit(&mut code, 0x318, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x31c, branch(C::Eq, 0x31c, 0x328, false)?)?;
    emit(&mut code, 0x320, branch(C::Al, 0x320, 0x249a4, true)?)?;
    emit(&mut code, 0x324, branch(C::Al, 0x324, 0x27c, false)?)?;
    emit(&mut code, 0x328, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 2)?)?;
    emit(&mut code, 0x32c, arm32::mcr(C::Al, 15, 0, r6, 7, 0, 4)?)?;
    emit(&mut code, 0x330, transfer_word(C::Al, false, R::R0, r9, 0)?)?;
    emit(&mut code, 0x334, transfer_word(C::Al, true, R::R0, r7, 0)?)?;
    emit(&mut code, 0x338, data_immediate(C::Al, DataOp::Bic, R::R0, R::R0, 1 << 30)?)?;
    emit(&mut code, 0x33c, transfer_word(C::Al, false, R::R0, r7, 0)?)?;
    emit(&mut code, 0x340, transfer_word(C::Al, true, R::R0, r10, 0)?)?;
    emit(&mut code, 0x344, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x348, branch(C::Ne, 0x348, 0x364, false)?)?;
    emit(&mut code, 0x34c, transfer_word(C::Al, true, R::R0, R::PC, 0x3c)?)?;
    emit(&mut code, 0x350, transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x354, data_immediate(C::Al, DataOp::Tst, R::R0, R::R0, 3)?)?;
    emit(&mut code, 0x358, branch(C::Ne, 0x358, 0x364, false)?)?;
    emit(&mut code, 0x35c, transfer_word(C::Al, true, R::R0, R::PC, 0x30)?)?;
    emit(&mut code, 0x360, branch(C::Al, 0x360, 0x24c24, true)?)?;
    emit(&mut code, 0x364, branch(C::Al, 0x364, 0xe1248, true)?)?;
    emit(&mut code, 0x368, branch(C::Al, 0x368, 0x230, false)?)?;
    emit(&mut code, 0x36c, data_immediate(C::Al, DataOp::Mov, r8, R::R0, 1)?)?;
    emit(&mut code, 0x370, branch(C::Al, 0x370, 0x270, false)?)?;
    emit(&mut code, 0x374, data_immediate(C::Al, DataOp::Mov, r8, R::R0, 2)?)?;
    emit(&mut code, 0x378, branch(C::Al, 0x378, 0x270, false)?)?;
    // Seven decoded pointers/configuration constants, not executable words.
    const LOOP_LITERALS: [u32; 7] = [
        0x0010_4104, 0x0010_40e8, 0xe00e_0050, 0xe00d_204c,
        0x0010_4088, 0xe00c_1008, 0x02bd_e780,
    ];
    for value in LOOP_LITERALS { code.extend_from_slice(&value.to_le_bytes()); }

    record_hardware_event(&mut code, 0x398, 1)?;
    record_hardware_event(&mut code, 0x438, 17)?;
    emit(&mut code, 0x4dc, push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    emit(&mut code, 0x4e0, mov_register(C::Al, R::R11, R::SP))?;
    emit(&mut code, 0x4e4, transfer_word(C::Al, true, R::R0, R::PC, 16)?)?;
    emit(&mut code, 0x4e8, transfer_word(C::Al, true, R::R2, R::PC, 16)?)?;
    emit(&mut code, 0x4ec, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 1024)?)?;
    emit(&mut code, 0x4f0, branch(C::Al, 0x4f0, 0x21c0, true)?)?;
    emit(&mut code, 0x4f4, transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    emit(&mut code, 0x4f8, branch(C::Al, 0x4f8, 0xfcc, false)?)?;
    // Runtime configuration pointer and diagnostics service selector.
    emit(&mut code, 0x4fc, 0x0010_3084)?;
    emit(&mut code, 0x500, 0x000e_2610)?;

    // Dispatch firmware service notifications through a compact computed
    // branch table. Entries are named runtime addresses rather than raw ARM
    // machine words; the PC-relative load is encoded separately.
    emit(&mut code, 0x504, push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    emit(&mut code, 0x508, mov_register(C::Al, R::R11, R::SP))?;
    emit(&mut code, 0x50c, data_immediate(C::Al, DataOp::Sub, R::SP, R::SP, 8)?)?;
    emit(&mut code, 0x510, transfer_word(C::Al, true, R::R0, R::PC, 0xa8)?)?;
    emit(&mut code, 0x514, transfer_word(C::Al, true, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x518, data_immediate(C::Al, DataOp::Sub, R::R1, R::R1, 4)?)?;
    emit(&mut code, 0x51c, mov_shifted_register(C::Al, R::R1, R::R1, Shift::Ror, 2)?)?;
    emit(&mut code, 0x520, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R1, 4)?)?;
    emit(&mut code, 0x524, branch(C::Hi, 0x524, 0x558, false)?)?;
    emit(&mut code, 0x528, mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 2)?)?;
    emit(&mut code, 0x52c, data_immediate(C::Al, DataOp::Add, R::R2, R::PC, 0)?)?;
    emit(&mut code, 0x530, transfer_word_register(C::Al, true, R::PC,
        R::R1, R::R2, Shift::Lsl, 0)?)?;
    const RUNTIME_VADDR: u32 = 0x1000;
    for (at, offset) in [(0x534,0x548), (0x538,0x558), (0x53c,0x570),
                         (0x540,0x584), (0x544,0x594)] {
        emit(&mut code, at, RUNTIME_VADDR + offset)?;
    }
    let r3 = R::new(3)?;
    emit(&mut code, 0x548, transfer_word(C::Al, true, r3, R::R0, 0x54)?)?;
    emit(&mut code, 0x54c, transfer_word(C::Al, true, R::R0, R::R0, 0x60)?)?;
    emit(&mut code, 0x550, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 4)?)?;
    emit(&mut code, 0x554, branch(C::Al, 0x554, 0x5a0, false)?)?;
    emit(&mut code, 0x558, transfer_word(C::Al, true, r3, R::R0, 0x54)?)?;
    emit(&mut code, 0x55c, transfer_word(C::Al, true, R::R0, R::R0, 0x60)?)?;
    emit(&mut code, 0x560, transfer_word(C::Al, true, R::R2, R::PC, 0x64)?)?;
    emit(&mut code, 0x564, transfer_word(C::Al, false, R::R0, R::SP, 0)?)?;
    emit(&mut code, 0x568, transfer_word(C::Al, true, R::R0, R::PC, 0x54)?)?;
    emit(&mut code, 0x56c, branch(C::Al, 0x56c, 0x5ac, false)?)?;
    emit(&mut code, 0x570, transfer_word(C::Al, true, r3, R::R0, 0x54)?)?;
    emit(&mut code, 0x574, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 12)?)?;
    emit(&mut code, 0x578, transfer_word(C::Al, false, r3, R::SP, 0)?)?;
    emit(&mut code, 0x57c, transfer_word(C::Al, false, R::R0, R::SP, 4)?)?;
    emit(&mut code, 0x580, branch(C::Al, 0x580, 0x5a4, false)?)?;
    emit(&mut code, 0x584, transfer_word(C::Al, true, r3, R::R0, 0x54)?)?;
    emit(&mut code, 0x588, transfer_word(C::Al, true, R::R0, R::R0, 0x60)?)?;
    emit(&mut code, 0x58c, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 16)?)?;
    emit(&mut code, 0x590, branch(C::Al, 0x590, 0x5a0, false)?)?;
    emit(&mut code, 0x594, transfer_word(C::Al, true, r3, R::R0, 0x54)?)?;
    emit(&mut code, 0x598, transfer_word(C::Al, true, R::R0, R::R0, 0x60)?)?;
    emit(&mut code, 0x59c, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 20)?)?;
    emit(&mut code, 0x5a0, transfer_multiple(C::Al, false, R::SP,
        R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    emit(&mut code, 0x5a4, transfer_word(C::Al, true, R::R0, R::PC, 24)?)?;
    emit(&mut code, 0x5a8, transfer_word(C::Al, true, R::R2, R::PC, 24)?)?;
    emit(&mut code, 0x5ac, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 1024)?)?;
    emit(&mut code, 0x5b0, branch(C::Al, 0x5b0, 0x21c0, true)?)?;
    emit(&mut code, 0x5b4, mov_register(C::Al, R::SP, R::R11))?;
    emit(&mut code, 0x5b8, transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    emit(&mut code, 0x5bc, branch(C::Al, 0x5bc, 0xfcc, false)?)?;
    // Fixed address/diagnostic-text IDs resolved by the handler's literal pool.
    for (at, value) in [(0x5c0, 0x0010_ce48), (0x5c4, 0x0010_3084),
                        (0x5c8, 0x000e_262c), (0x5cc, 0x000e_2666)] {
        emit(&mut code, at, value)?;
    }

    // Diagnostic snapshot dispatcher. The computed jump table stores
    // relocated runtime addresses rather than instruction encodings.
    emit(&mut code, 0x5d0, push_registers(C::Al,
        r4.bit() | r5.bit() | r6.bit() | r7.bit() | R::R11.bit() | R::LR.bit())?)?;
    emit(&mut code, 0x5d4, data_immediate(C::Al, DataOp::Add, R::R11, R::SP, 16)?)?;
    emit(&mut code, 0x5d8, data_immediate(C::Al, DataOp::Sub, R::SP, R::SP, 120)?)?;
    emit(&mut code, 0x5dc, data_immediate(C::Al, DataOp::Add, r5, R::SP, 20)?)?;
    emit(&mut code, 0x5e0, mov_register(C::Al, r4, R::R1))?;
    emit(&mut code, 0x5e4, mov_register(C::Al, r6, R::R0))?;
    emit(&mut code, 0x5e8, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 0)?)?;
    emit(&mut code, 0x5ec, data_immediate(C::Al, DataOp::Mov, R::R2, R::R0, 100)?)?;
    emit(&mut code, 0x5f0, mov_register(C::Al, R::R0, r5))?;
    emit(&mut code, 0x5f4, branch(C::Al, 0x5f4, 0x21ec, true)?)?;
    emit(&mut code, 0x5f8, data_immediate(C::Al, DataOp::Cmp, R::R0, r6, 3)?)?;
    emit(&mut code, 0x5fc, branch(C::Hi, 0x5fc, 0x6d8, false)?)?;
    emit(&mut code, 0x600, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 32768)?)?;
    emit(&mut code, 0x604, mov_shifted_register(C::Al, R::R1, r6, Shift::Lsl, 2)?)?;
    emit(&mut code, 0x608, data_immediate(C::Al, DataOp::Add, R::R2, R::PC, 4)?)?;
    emit(&mut code, 0x60c, data_immediate(C::Al, DataOp::Orr, R::R0, R::R0, 0xe000_0000)?)?;
    emit(&mut code, 0x610, transfer_word_register(C::Al, true, R::PC,
        R::R1, R::R2, Shift::Lsl, 0)?)?;
    const RUNTIME_VADDR_CASES: u32 = 0x1000;
    for (at, destination) in [(0x614,0x624), (0x618,0x660),
                              (0x61c,0x690), (0x620,0x6b0)] {
        emit(&mut code, at, RUNTIME_VADDR_CASES + destination)?;
    }
    emit(&mut code, 0x624, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 52)?)?;
    emit(&mut code, 0x628, data_immediate(C::Al, DataOp::Mov, R::R2, R::R0, 1024)?)?;
    emit(&mut code, 0x62c, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 0xe000_0000)?)?;
    emit(&mut code, 0x630, data_immediate(C::Al, DataOp::Mov, r3, R::R0, 4096)?)?;
    emit(&mut code, 0x634, data_register(C::Al, DataOp::Orr, R::R0,
        R::R0, r4, Shift::Lsl, 15, false)?)?;
    emit(&mut code, 0x638, data_register(C::Al, DataOp::Orr, R::R2,
        R::R2, r4, Shift::Lsl, 15, false)?)?;
    emit(&mut code, 0x63c, data_register(C::Al, DataOp::Orr, r3,
        r3, r4, Shift::Lsl, 15, false)?)?;
    emit(&mut code, 0x640, transfer_word_register(C::Al, true, R::R0,
        R::R0, R::R1, Shift::Lsl, 0)?)?;
    emit(&mut code, 0x644, transfer_word_register(C::Al, true, R::R2,
        R::R2, R::R1, Shift::Lsl, 0)?)?;
    emit(&mut code, 0x648, transfer_word_register(C::Al, true, R::R1,
        r3, R::R1, Shift::Lsl, 0)?)?;
    emit(&mut code, 0x64c, transfer_multiple(C::Al, false, R::SP,
        R::R0.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    emit(&mut code, 0x650, transfer_word(C::Al, true, R::R0, R::PC, 0xc0)?)?;
    emit(&mut code, 0x654, transfer_word(C::Al, true, R::R2, R::PC, 0xd4)?)?;
    emit(&mut code, 0x658, transfer_word(C::Al, false, R::R1, R::SP, 8)?)?;
    emit(&mut code, 0x65c, branch(C::Al, 0x65c, 0x6cc, false)?)?;
    emit(&mut code, 0x660, mov_register(C::Al, R::R0, r5))?;
    emit(&mut code, 0x664, mov_register(C::Al, R::R1, r4))?;
    emit(&mut code, 0x668, branch(C::Al, 0x668, 0xc0, true)?)?;
    emit(&mut code, 0x66c, transfer_word(C::Al, true, R::R0, R::PC, 0xb0)?)?;
    emit(&mut code, 0x670, transfer_word(C::Al, true, R::R2, R::PC, 0xb4)?)?;
    emit(&mut code, 0x674, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 1024)?)?;
    emit(&mut code, 0x678, transfer_word(C::Al, true, r3, R::R0, 0)?)?;
    emit(&mut code, 0x67c, transfer_word(C::Al, true, R::R0, R::PC, 0xa4)?)?;
    emit(&mut code, 0x680, transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x684, transfer_multiple(C::Al, false, R::SP,
        R::R0.bit() | r5.bit(), BlockMode::IncrementAfter, false)?)?;
    emit(&mut code, 0x688, transfer_word(C::Al, true, R::R0, R::PC, 0x88)?)?;
    emit(&mut code, 0x68c, branch(C::Al, 0x68c, 0x6d4, false)?)?;
    emit(&mut code, 0x690, transfer_word(C::Al, true, R::R1, R::R0, 0x34)?)?;
    emit(&mut code, 0x694, transfer_word(C::Al, true, R::R2, R::R0, 0)?)?;
    emit(&mut code, 0x698, transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    emit(&mut code, 0x69c, transfer_multiple(C::Al, false, R::SP,
        R::R1.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    emit(&mut code, 0x6a0, transfer_word(C::Al, false, R::R0, R::SP, 8)?)?;
    emit(&mut code, 0x6a4, transfer_word(C::Al, true, R::R0, R::PC, 0x6c)?)?;
    emit(&mut code, 0x6a8, transfer_word(C::Al, true, R::R2, R::PC, 0x70)?)?;
    emit(&mut code, 0x6ac, branch(C::Al, 0x6ac, 0x6cc, false)?)?;
    emit(&mut code, 0x6b0, transfer_word(C::Al, true, R::R1, R::R0, 0x34)?)?;
    emit(&mut code, 0x6b4, transfer_word(C::Al, true, R::R2, R::R0, 0)?)?;
    emit(&mut code, 0x6b8, transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    emit(&mut code, 0x6bc, transfer_multiple(C::Al, false, R::SP,
        R::R1.bit() | R::R2.bit(), BlockMode::IncrementAfter, false)?)?;
    emit(&mut code, 0x6c0, transfer_word(C::Al, false, R::R0, R::SP, 8)?)?;
    emit(&mut code, 0x6c4, transfer_word(C::Al, true, R::R0, R::PC, 0x4c)?)?;
    emit(&mut code, 0x6c8, transfer_word(C::Al, true, R::R2, R::PC, 0x4c)?)?;
    emit(&mut code, 0x6cc, data_immediate(C::Al, DataOp::Mov, R::R1, R::R0, 1024)?)?;
    emit(&mut code, 0x6d0, mov_register(C::Al, r3, r4))?;
    emit(&mut code, 0x6d4, branch(C::Al, 0x6d4, 0x21c0, true)?)?;
    emit(&mut code, 0x6d8, branch(C::Al, 0x6d8, 0xfcc, true)?)?;
    emit(&mut code, 0x6dc, branch(C::Al, 0x6dc, 0x2e8ac, true)?)?;
    emit(&mut code, 0x6e0, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x6e4, branch(C::Ne, 0x6e4, 0x6f4, false)?)?;
    emit(&mut code, 0x6e8, branch(C::Al, 0x6e8, 0x2e808, true)?)?;
    emit(&mut code, 0x6ec, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x6f0, branch(C::Eq, 0x6f0, 0x710, false)?)?;
    emit(&mut code, 0x6f4, branch(C::Al, 0x6f4, 0x398, true)?)?;
    emit(&mut code, 0x6f8, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x6fc, transfer_word(C::Al, false, R::R0, R::SP, 16)?)?;
    emit(&mut code, 0x700, transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    emit(&mut code, 0x704, data_immediate(C::Al, DataOp::Add, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x708, transfer_word(C::Al, false, R::R0, R::SP, 16)?)?;
    emit(&mut code, 0x70c, branch(C::Al, 0x70c, 0x700, false)?)?;
    emit(&mut code, 0x710, branch(C::Al, 0x710, 0x438, true)?)?;
    emit(&mut code, 0x714, branch(C::Al, 0x714, 0x6f8, false)?)?;
    const DIAGNOSTIC_SERVICE_TABLE: [u32; 7] = [
        0x0010_3084, 0x000e_271f, 0x000e_26f1, 0x0010_307c,
        0x0010_3080, 0x000e_26cf, 0x000e_26a6,
    ];
    for value in DIAGNOSTIC_SERVICE_TABLE { code.extend_from_slice(&value.to_le_bytes()); }

    // Runtime service initialization with recoverable branch paths. The
    // trailing table comprises service-label addresses rather than opcodes.
    emit(&mut code, 0x734, push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    emit(&mut code, 0x738, mov_register(C::Al, R::R11, R::SP))?;
    emit(&mut code, 0x73c, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x740, branch(C::Al, 0x740, 0x26180, true)?)?;
    emit(&mut code, 0x744, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x748, branch(C::Ne, 0x748, 0x764, false)?)?;
    emit(&mut code, 0x74c, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x750, branch(C::Al, 0x750, 0x25460, true)?)?;
    emit(&mut code, 0x754, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x758, branch(C::Al, 0x758, 0x25424, true)?)?;
    emit(&mut code, 0x75c, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x760, branch(C::Al, 0x760, 0x2549c, true)?)?;
    emit(&mut code, 0x764, branch(C::Al, 0x764, 0x25ac4, true)?)?;
    emit(&mut code, 0x768, data_immediate(C::Al, DataOp::Cmp, R::R0, R::R0, 0)?)?;
    emit(&mut code, 0x76c, branch(C::Ne, 0x76c, 0x778, false)?)?;
    emit(&mut code, 0x770, branch(C::Al, 0x770, 0x24954, true)?)?;
    emit(&mut code, 0x774, branch(C::Al, 0x774, 0x24984, true)?)?;
    emit(&mut code, 0x778, transfer_word(C::Al, true, R::R1, R::PC, 36)?)?;
    emit(&mut code, 0x77c, transfer_word(C::Al, true, R::R2, R::PC, 36)?)?;
    emit(&mut code, 0x780, data_immediate(C::Al, DataOp::Mov, r3, R::R0, 218)?)?;
    emit(&mut code, 0x784, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 8)?)?;
    emit(&mut code, 0x788, data_immediate(C::Al, DataOp::Orr, r3, r3, 256)?)?;
    emit(&mut code, 0x78c, branch(C::Al, 0x78c, 0x2dc94, true)?)?;
    emit(&mut code, 0x790, transfer_word(C::Al, true, R::R1, R::PC, 20)?)?;
    emit(&mut code, 0x794, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 8)?)?;
    emit(&mut code, 0x798, branch(C::Al, 0x798, 0x2dc94, true)?)?;
    emit(&mut code, 0x79c, data_immediate(C::Al, DataOp::Mov, R::R0, R::R0, 1)?)?;
    emit(&mut code, 0x7a0, transfer_multiple(C::Al, true, R::SP,
        R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for (at, service_address) in [
        (0x7a4, 0x000e_2750), (0x7a8, 0x000e_277a), (0x7ac, 0x000e_2788),
    ] { emit(&mut code, at, service_address)?; }
    super::venus_runtime_tail::extend(&mut code)?;
    super::venus_runtime_init::extend(&mut code)?;
    super::venus_runtime_services::extend(&mut code)?;
    super::venus_runtime_services_next::extend(&mut code)?;
    super::venus_runtime_dispatch::extend(&mut code)?;
    super::venus_runtime_queue::extend(&mut code)?;
    super::venus_runtime_allocator::extend(&mut code)?;
    super::venus_runtime_mask::extend(&mut code)?;
    super::venus_runtime_wrappers::extend(&mut code)?;
    super::venus_runtime_setup::extend(&mut code)?;
    if code.len() != PREFIX_SIZE { return Err("Venus runtime prefix size mismatch".into()); }
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_arm32_runtime_entrypoints() {
        let code = prefix().unwrap();
        assert_eq!(code.len(), PREFIX_SIZE);
        assert_eq!(&code[0x1c..0x20], &0x0010_3068u32.to_le_bytes());
        assert_eq!(&code[0x64..0x68], &0xe3e0_0000u32.to_le_bytes());
        assert_eq!(&code[0x84..0x88], &0xeaff_fffeu32.to_le_bytes());
    }
}
