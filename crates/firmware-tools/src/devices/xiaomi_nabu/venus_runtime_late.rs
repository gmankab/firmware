// SPDX-License-Identifier: GPL-3.0-only
//! Independent, byte-verified late Venus A32 routines for the Xiaomi Pad 5.
//!
//! This region is explicitly DISJOINT from the early boot/runtime and Venus
//! service reconstruction. It is neither a complete image nor flashable.
//! Every instruction is encoded from ARM operands; no OEM instruction words
//! or authentication data are embedded. Address constants in literal pools
//! are reported separately, not counted as executable instructions.
use arm32::{branch, transfer_word, BlockMode, Condition as C,
            DataOp as Op, Register as R, Shift};

/// First function boundary found after reservation start 0x18000.
/// 0x18000 is in the middle of the function beginning at 0x17fd0; its
/// jump table includes entries at 0x18010–0x1802b and must not be split.
pub const FILE_BASE: usize = 0x4000;
pub const START: usize = 0x183e0;
pub const END: usize = 0x1ae84;
pub const MAX_RESERVED_END: usize = 0x24000;

// Isolated builder: unlike the boot-prefix helper, the A32 address is tracked
// separately from output length, and all gaps are rejected.
struct Code { start: usize, bytes: Vec<u8> }
impl Code {
    fn new(start: usize) -> Self { Self { start, bytes: Vec::new() } }
    fn at(&self) -> usize { self.start + self.bytes.len() }
    fn check(&self, expected: usize) -> Result<(), String> {
        if self.at() != expected {
            return Err(format!("late Venus ARM layout 0x{:x} != expected 0x{expected:x}", self.at()));
        }
        Ok(())
    }
    fn emit(&mut self, word: u32) -> Result<(), String> {
        if self.at() < 0x18000 || self.at() >= MAX_RESERVED_END || self.at() % 4 != 0 {
            return Err(format!("late Venus emitted beyond exclusive reservation 0x{:x}", self.at()));
        }
        self.bytes.extend_from_slice(&word.to_le_bytes());
        Ok(())
    }
    fn b(&mut self, condition: C, target: usize) -> Result<(), String> {
        self.emit(branch(condition, self.at() as u32, target as u32, false)?)
    }
    fn bl(&mut self, target: usize) -> Result<(), String> {
        self.emit(branch(C::Al, self.at() as u32, target as u32, true)?)
    }
    fn literal(&mut self, rd: R, target: usize) -> Result<(), String> {
        let diff = target as isize - self.at() as isize - 8;
        let signed = i32::try_from(diff).map_err(|_| "late Venus literal displacement overflow")?;
        self.emit(transfer_word(C::Al, true, rd, R::PC, signed)?)
    }
}

const CODEC_RUNTIME_DIAGNOSTIC: u32 = 0x000e_3d16;

#[path = "venus_runtime_late_stream.rs"]
mod stream;
#[path = "venus_runtime_late_more.rs"]
mod more;
#[path = "venus_runtime_late_structure.rs"]
mod structure;
#[path = "venus_runtime_late_packets.rs"]
mod packets;
#[path = "venus_runtime_late_coefficients.rs"]
mod coefficients;
#[path = "venus_runtime_late_dispatch.rs"]
mod dispatch;

/// Normalize stride/crop dimensions to hardware layout constraints, clamp
/// bounds exposed by firmware state, and report four alignment invariants.
/// The remaining branch selects an indexed codec buffer address and updates
/// resource structures. ARM runtime `[0x183e0, 0x18578)`.
fn emit_scale_and_validate(c: &mut Code) -> Result<(), String> {
    c.check(START)?;
    // 0x183e0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x183e4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x183e8: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x183ec: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x183f0: mvn r2, #-268435455
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0xf0000001)?)?;
    // 0x183f4: ldr r6, [pc, #0x178]
    c.literal(R::new(6)?, 0x18574)?;
    // 0x183f8: ldr r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 88)?)?;
    // 0x183fc: ldr r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 92)?)?;
    // 0x18400: add r7, r4, #308
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(4)?, 0x134)?)?;
    // 0x18404: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1f)?)?;
    // 0x18408: add r1, r1, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1f)?)?;
    // 0x1840c: and r0, r2, r0, lsr #4
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R2, R::R0, Shift::Lsr, 4, false)?)?;
    // 0x18410: and r1, r2, r1, lsr #4
    c.emit(arm32::data_register(C::Al, Op::And, R::R1, R::R2, R::R1, Shift::Lsr, 4, false)?)?;
    // 0x18414: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0x18418: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R1, R::R1, Shift::Lsl, 1, false)?)?;
    // 0x1841c: lsl r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 4)?)?;
    // 0x18420: lsl r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 4)?)?;
    // 0x18424: str r0, [r4, #0x964]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2404)?)?;
    // 0x18428: str r1, [r4, #0x968]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2408)?)?;
    // 0x1842c: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 2648)?)?;
    // 0x18430: mov r1, r0
    c.emit(arm32::mov_register(C::Al, R::R1, R::R0))?;
    // 0x18434: ldr r2, [r1, #0x5c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, true, R::R2, R::R1, 92)?)?;
    // 0x18438: str r2, [r4, #0x930]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2352)?)?;
    // 0x1843c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x18440: ldr r3, [r0, #0x60]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, true, R::new(3)?, R::R0, 96)?)?;
    // 0x18444: addne r1, r4, #2352
    c.emit(arm32::data_immediate(C::Ne, Op::Add, R::R1, R::new(4)?, 0x930)?)?;
    // 0x18448: str r3, [r4, #0x934]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 2356)?)?;
    // 0x1844c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x0)?)?;
    // 0x18450: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 0)?)?;
    // 0x18454: addne r0, r7, #2048
    c.emit(arm32::data_immediate(C::Ne, Op::Add, R::R0, R::new(7)?, 0x800)?)?;
    // 0x18458: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1845c: str r1, [r4, #0x930]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2352)?)?;
    // 0x18460: cmp r1, #128
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x80)?)?;
    // 0x18464: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x18468: movge r1, #128
    c.emit(arm32::data_immediate(C::Ge, Op::Mov, R::R1, R::R0, 0x80)?)?;
    // 0x1846c: str r1, [r4, #0x930]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2352)?)?;
    // 0x18470: mov r1, #58
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3a)?)?;
    // 0x18474: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x18478: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x40)?)?;
    // 0x1847c: movge r0, #64
    c.emit(arm32::data_immediate(C::Ge, Op::Mov, R::R0, R::R0, 0x40)?)?;
    // 0x18480: tst r2, #31
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R2, 0x1f)?)?;
    // 0x18484: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x18488: str r0, [r4, #0x934]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2356)?)?;
    // 0x1848c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x18490: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x18494: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x18498: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 2648)?)?;
    // 0x1849c: mov r1, #59
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3b)?)?;
    // 0x184a0: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x184a4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x184a8: ldr r0, [r0, #0x60]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 96)?)?;
    // 0x184ac: tst r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0xf)?)?;
    // 0x184b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x184b4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x184b8: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x184bc: ldr r0, [r4, #0x930]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 2352)?)?;
    // 0x184c0: mov r1, #60
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3c)?)?;
    // 0x184c4: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x184c8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x184cc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0x1f)?)?;
    // 0x184d0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x184d4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x184d8: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x184dc: ldr r0, [r4, #0x934]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 2356)?)?;
    // 0x184e0: mov r1, #61
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3d)?)?;
    // 0x184e4: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x184e8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x184ec: tst r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R0, 0xf)?)?;
    // 0x184f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x184f4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x184f8: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x184fc: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 68)?)?;
    // 0x18500: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x7)?)?;
    // 0x18504: bne 0x18514 <later+0x134>
    c.b(C::Ne, 0x18514)?;
    // 0x18508: str r7, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 2444)?)?;
    // 0x1850c: ldr r2, [r4, #0x984]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2436)?)?;
    // 0x18510: b 0x18564 <later+0x184>
    c.b(C::Al, 0x18564)?;
    // 0x18514: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 12)?)?;
    // 0x18518: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1851c: ldr r1, [r5, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(5)?, R::R1, Shift::Lsl, 2)?)?;
    // 0x18520: str r2, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2444)?)?;
    // 0x18524: sub r2, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R2, R::R0, 0x4)?)?;
    // 0x18528: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x2)?)?;
    // 0x1852c: blo 0x18548 <later+0x168>
    c.b(C::Cc, 0x18548)?;
    // 0x18530: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x6)?)?;
    // 0x18534: bne 0x1856c <later+0x18c>
    c.b(C::Ne, 0x1856c)?;
    // 0x18538: ldr r3, [r4, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 728)?)?;
    // 0x1853c: mov r2, #96
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x60)?)?;
    // 0x18540: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x1)?)?;
    // 0x18544: bhi 0x18564 <later+0x184>
    c.b(C::Hi, 0x18564)?;
    // 0x18548: rsb r0, r0, #6
    c.emit(arm32::data_immediate(C::Al, Op::Rsb, R::R0, R::R0, 0x6)?)?;
    // 0x1854c: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x18550: mov r1, #400
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x190)?)?;
    // 0x18554: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x18558: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1855c: add r0, r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x7)?)?;
    // 0x18560: bic r2, r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R0, 0x7)?)?;
    // 0x18564: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 2648)?)?;
    // 0x18568: str r2, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 40)?)?;
    // 0x1856c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x18570: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x18574)?;
    c.emit(CODEC_RUNTIME_DIAGNOSTIC)?;
    c.check(0x18578)
}

/// Append a 2-word or 3-word codec command to either hardware command ring,
/// depending on whether the stream context is present. This directly models
/// independent setup and published producer cursors. ARM runtime
/// `[0x18578, 0x18628)`.
fn emit_publish_codec_command(c: &mut Code) -> Result<(), String> {
    c.check(0x18578)?;
    // 0x18578: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1857c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x18580: ldr r12, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::R11, 8)?)?;
    // 0x18584: add r4, r2, #556
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R2, 0x22c)?)?;
    // 0x18588: add r1, r2, #568
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0x238)?)?;
    // 0x1858c: add r4, r4, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x1c00)?)?;
    // 0x18590: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R12, 0x0)?)?;
    // 0x18594: addne r4, r1, #7168
    c.emit(arm32::data_immediate(C::Ne, Op::Add, R::new(4)?, R::R1, 0x1c00)?)?;
    // 0x18598: add r1, r2, #564
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0x234)?)?;
    // 0x1859c: add r2, r2, #552
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x228)?)?;
    // 0x185a0: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R12, 0x0)?)?;
    // 0x185a4: add r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x1c00)?)?;
    // 0x185a8: addne r2, r1, #7168
    c.emit(arm32::data_immediate(C::Ne, Op::Add, R::R2, R::R1, 0x1c00)?)?;
    // 0x185ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x185b0: popeq {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq, true, R::SP, R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x185b4: ldr r0, [r2]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R2, 0)?)?;
    // 0x185b8: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R12, 0x0)?)?;
    // 0x185bc: add r1, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x4)?)?;
    // 0x185c0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 0)?)?;
    // 0x185c4: beq 0x185ec <later+0x20c>
    c.b(C::Eq, 0x185ec)?;
    // 0x185c8: mov r1, #1120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x460)?)?;
    // 0x185cc: orr r12, r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R12, R::new(3)?, 0x1)?)?;
    // 0x185d0: mov lr, #100
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::LR, R::R0, 0x64)?)?;
    // 0x185d4: orr r1, r1, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8000)?)?;
    // 0x185d8: orr lr, lr, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::LR, R::LR, 0x8400)?)?;
    // 0x185dc: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x185e0: ldr r3, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 0)?)?;
    // 0x185e4: str r12, [r3]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::new(3)?, 0)?)?;
    // 0x185e8: b 0x1860c <later+0x22c>
    c.b(C::Al, 0x1860c)?;
    // 0x185ec: mov r1, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8d00)?)?;
    // 0x185f0: mov lr, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::LR, R::R0, 0x104)?)?;
    // 0x185f4: mov r12, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x3f)?)?;
    // 0x185f8: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x185fc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x18600: orr lr, lr, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::LR, R::LR, 0x8c00)?)?;
    // 0x18604: ldr r3, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 0)?)?;
    // 0x18608: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(3)?, 0)?)?;
    // 0x1860c: add r1, r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x8)?)?;
    // 0x18610: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 0)?)?;
    // 0x18614: str lr, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R0, 4)?)?;
    // 0x18618: add r0, r3, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(3)?, 0x8)?)?;
    // 0x1861c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x18620: str r12, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::new(3)?, 4)?)?;
    // 0x18624: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x18628)
}

/// Complete contiguous fragment containing fifteen functions and seventy-eight data words.
/// All other code in reserved `[0x18000,0x24000)` is **unimplemented**.
pub fn fragment() -> Result<Vec<u8>, String> {
    let mut c = Code::new(START);
    emit_scale_and_validate(&mut c)?;
    emit_publish_codec_command(&mut c)?;
    stream::extend(&mut c)?;
    more::extend(&mut c)?;
    structure::extend(&mut c)?;
    packets::extend(&mut c)?;
    coefficients::extend(&mut c)?;
    dispatch::extend(&mut c)?;
    c.check(END)?;
    Ok(c.bytes)
}

/// Strict opt-in comparison with a caller-supplied private reference image.
/// Rejects missing reference bytes and returns the FIRST differing file offset.
/// No original firmware bytes are ever included in a commit or output artifact.
pub fn verify_reference(reference: &[u8]) -> Result<usize, String> {
    let expected = fragment()?;
    let file_start = FILE_BASE + START;
    let file_end = FILE_BASE + END;
    let slice = reference.get(file_start..file_end)
        .ok_or_else(|| format!("missing Venus reference range 0x{file_start:x}..0x{file_end:x}"))?;
    if let Some(index) = expected.iter().zip(slice).position(|(a,b)| a != b) {
        return Err(format!("Venus late fragment mismatch at file 0x{:x}, runtime 0x{:x}: generated {:02x}, reference {:02x}",
            file_start + index, START + index, expected[index], slice[index]));
    }
    Ok(expected.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fifteen_late_functions_have_no_gaps() {
        let buf = fragment().unwrap();
        assert_eq!(buf.len(), END - START);
        assert_eq!(START, 0x183e0);
        assert_eq!(END, 0x1ae84);
        assert_eq!(&buf[0x18574 - START..0x18578 - START], &CODEC_RUNTIME_DIAGNOSTIC.to_le_bytes());
        assert_eq!(&buf[0x19690 - START..0x19694 - START], &CODEC_RUNTIME_DIAGNOSTIC.to_le_bytes());
        assert_eq!(&buf[0x196a0 - START..0x196a4 - START], &0xffff_fb80u32.to_le_bytes());
    }
    #[test] fn too_short_reference_fails_closed() {
        assert!(verify_reference(&[]).unwrap_err().contains("missing Venus reference"));
    }
    #[test] fn first_difference_is_reported_with_both_address_spaces() {
        let mut reference = vec![0; FILE_BASE + END];
        reference[FILE_BASE + START..FILE_BASE + END].copy_from_slice(&fragment().unwrap());
        assert_eq!(verify_reference(&reference).unwrap(), END - START);
        reference[FILE_BASE + START + 8] ^= 0x01;
        let msg = verify_reference(&reference).unwrap_err();
        assert!(msg.contains("file 0x1c3e8"), "{msg}");
        assert!(msg.contains("runtime 0x183e8"), "{msg}");
    }
}
