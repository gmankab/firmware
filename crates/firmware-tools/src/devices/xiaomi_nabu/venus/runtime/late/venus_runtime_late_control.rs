// SPDX-License-Identifier: GPL-3.0-only
//! Vendor Venus codec controller control paths: register a live session,
//! drive its mode-dependent work loop and report command failures.
//! Literal pools are intentionally distinct from ARM instruction output.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x1ae84;
const END:usize=0x1b164;
const ATTACH_TRACES:[u32;3]=[0x000e_4d26,0x000e_2788,0x000e_4d41];
const CONTROL_LOOP_GLOBALS:[u32;3]=[0x0010_dfb0,0x000e_256c,0x000e_471e];
const CONTROL_DIAGNOSTICS:[u32;5]=[0x0001_af30,0x000e_471e,0x000e_4d6e,
   0x000e_4da9,0x000e_2788];

/// Check a newly attached codec session, publish its resource handle and
/// emit success/failure diagnostics via the shared logger.
fn activate_codec_session(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1ae84: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1ae88: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x1ae8c: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x1ae90: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1ae94: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x4)?)?;
    // 0x1ae98: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1ae9c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1aea0: bl 0x80994 <entry+0x667a8>
    c.bl(0x80994)?;
    // 0x1aea4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1aea8: bne 0x1af00 <entry+0xd14>
    c.b(C::Ne, 0x1af00)?;
    // 0x1aeac: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1aeb0: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 4)?)?;
    // 0x1aeb4: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1aeb8: bl 0x26754 <entry+0xc568>
    c.bl(0x26754)?;
    // 0x1aebc: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 148)?)?;
    // 0x1aec0: mov r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x1aec4: ldr r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 20)?)?;
    // 0x1aec8: mov r1, #3696
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe70)?)?;
    // 0x1aecc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1aed0: str r0, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 0x1aed4: ldr r1, [pc, #0x50]
    c.literal(R::R1, 0x1af2c)?;
    // 0x1aed8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1aedc: ldr r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 12)?)?;
    // 0x1aee0: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1aee4: ldr r1, [pc, #0x3c]
    c.literal(R::R1, 0x1af28)?;
    // 0x1aee8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1aeec: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1aef0: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1aef4: str r5, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 16)?)?;
    // 0x1aef8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1aefc: b 0x1af1c <entry+0xd30>
    c.b(C::Al, 0x1af1c)?;
    // 0x1af00: ldr r1, [pc, #0x1c]
    c.literal(R::R1, 0x1af24)?;
    // 0x1af04: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1af08: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1af0c: ldr r1, [pc, #0x14]
    c.literal(R::R1, 0x1af28)?;
    // 0x1af10: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1af14: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1af18: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1af1c: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x8)?)?;
    // 0x1af20: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for v in ATTACH_TRACES{c.emit(v)?;}
    c.check(0x1af30)
}

/// Poll codec state and run the selected controller command mode until
/// success or a fatal error exits the loop.
fn codec_controller_loop(c:&mut Code)->Result<(),String>{
    c.check(0x1af30)?;
    // 0x1af30: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1af34: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x18)?)?;
    // 0x1af38: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1af3c: ldr r0, [pc, #0x90]
    c.literal(R::R0, 0x1afd4)?;
    // 0x1af40: ldr r1, [pc, #0x90]
    c.literal(R::R1, 0x1afd8)?;
    // 0x1af44: ldr r5, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(4)?, 148)?)?;
    // 0x1af48: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1af4c: bl 0x307c <entry+0xfffffffffffe8e90>
    c.bl(0x307c)?;
    // 0x1af50: ldr r9, [pc, #0x84]
    c.literal(R::new(9)?, 0x1afdc)?;
    // 0x1af54: mov r8, #79
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x4f)?)?;
    // 0x1af58: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1af5c: orr r8, r8, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(8)?, R::new(8)?, 0x100)?)?;
    // 0x1af60: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1af64: bl 0x294c <entry+0xfffffffffffe8760>
    c.bl(0x294c)?;
    // 0x1af68: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1af6c: bl 0x293c <entry+0xfffffffffffe8750>
    c.bl(0x293c)?;
    // 0x1af70: mov r7, r6
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(6)?))?;
    // 0x1af74: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x1af78: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1af7c: beq 0x1af98 <entry+0xdac>
    c.b(C::Eq, 0x1af98)?;
    // 0x1af80: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1af84: bne 0x1afb0 <entry+0xdc4>
    c.b(C::Ne, 0x1afb0)?;
    // 0x1af88: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1af8c: bl 0x1ae84 <entry+0xc98>
    c.bl(0x1ae84)?;
    // 0x1af90: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x1af94: b 0x1afc4 <entry+0xdd8>
    c.b(C::Al, 0x1afc4)?;
    // 0x1af98: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1af9c: bl 0x1a8a8 <entry+0x6bc>
    c.bl(0x1a8a8)?;
    // 0x1afa0: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1afa4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1afa8: str r7, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 16)?)?;
    // 0x1afac: b 0x1af74 <entry+0xd88>
    c.b(C::Al, 0x1af74)?;
    // 0x1afb0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1afb4: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 0x1afb8: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 0x1afbc: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1afc0: mov r6, r7
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x1afc4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1afc8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x1afcc: beq 0x1af74 <entry+0xd88>
    c.b(C::Eq, 0x1af74)?;
    // 0x1afd0: b 0x1af60 <entry+0xd74>
    c.b(C::Al, 0x1af60)?;
    for v in CONTROL_LOOP_GLOBALS{c.emit(v)?;}
    c.check(0x1afe0)
}

/// Dispatch buffer errors, perform selected callback and expose an alternate
/// three-instruction resource-pointer accessor at the end of the pool.
fn diagnose_codec_command(c:&mut Code)->Result<(),String>{
    c.check(0x1afe0)?;
    // 0x1afe0: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1afe4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x1afe8: sub sp, sp, #40
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x28)?)?;
    // 0x1afec: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1aff0: ldr r0, [pc, #0x14c]
    c.literal(R::R0, 0x1b144)?;
    // 0x1aff4: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    // 0x1aff8: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 16)?)?;
    // 0x1affc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1b000: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 20)?)?;
    // 0x1b004: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2000)?)?;
    // 0x1b008: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 24)?)?;
    // 0x1b00c: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 28)?)?;
    // 0x1b010: str r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 32)?)?;
    // 0x1b014: str r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 36)?)?;
    // 0x1b018: add r0, sp, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0xc)?)?;
    // 0x1b01c: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 148)?)?;
    // 0x1b020: bl 0x29b4 <entry+0xfffffffffffe87c8>
    c.bl(0x29b4)?;
    // 0x1b024: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1b028: beq 0x1b040 <entry+0xe54>
    c.b(C::Eq, 0x1b040)?;
    // 0x1b02c: ldr r2, [pc, #0x114]
    c.literal(R::R2, 0x1b148)?;
    // 0x1b030: mov r1, #118
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x76)?)?;
    // 0x1b034: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b038: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    // 0x1b03c: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b040: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1b044: ldr r1, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al, true, R::R1, R::R0, 4)?)?;
    // 0x1b048: bl 0x2c44 <entry+0xfffffffffffe8a58>
    c.bl(0x2c44)?;
    // 0x1b04c: ldr r2, [pc, #0xf4]
    c.literal(R::R2, 0x1b148)?;
    // 0x1b050: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1b054: mov r1, #122
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7a)?)?;
    // 0x1b058: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x1b05c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    // 0x1b060: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1b064: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b068: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b06c: bl 0x2efc <entry+0xfffffffffffe8d10>
    c.bl(0x2efc)?;
    // 0x1b070: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 148)?)?;
    // 0x1b074: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1b078: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 12)?)?;
    // 0x1b07c: bne 0x1b09c <entry+0xeb0>
    c.b(C::Ne, 0x1b09c)?;
    // 0x1b080: ldr r2, [pc, #0xc0]
    c.literal(R::R2, 0x1b148)?;
    // 0x1b084: mov r1, #127
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7f)?)?;
    // 0x1b088: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b08c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    // 0x1b090: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b094: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 148)?)?;
    // 0x1b098: ldr r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 12)?)?;
    // 0x1b09c: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x1b0a0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1b0a4: bl 0x2924 <entry+0xfffffffffffe8738>
    c.bl(0x2924)?;
    // 0x1b0a8: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1b0ac: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 0x1b0b0: bl 0x293c <entry+0xfffffffffffe8750>
    c.bl(0x293c)?;
    // 0x1b0b4: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x1b0b8: bl 0x2efc <entry+0xfffffffffffe8d10>
    c.bl(0x2efc)?;
    // 0x1b0bc: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 148)?)?;
    // 0x1b0c0: ldr r2, [pc, #0x80]
    c.literal(R::R2, 0x1b148)?;
    // 0x1b0c4: str r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 8)?)?;
    // 0x1b0c8: mov r1, #134
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x86)?)?;
    // 0x1b0cc: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    // 0x1b0d0: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b0d4: ldr r1, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 148)?)?;
    // 0x1b0d8: mov r3, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20)?)?;
    // 0x1b0dc: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 8)?)?;
    // 0x1b0e0: ldr r1, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 12)?)?;
    // 0x1b0e4: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    // 0x1b0e8: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x1b0ec: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x1b0f0: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x1b0f4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b0f8: mov r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x4)?)?;
    // 0x1b0fc: bl 0x8080c <entry+0x66620>
    c.bl(0x8080c)?;
    // 0x1b100: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1b104: beq 0x1b13c <entry+0xf50>
    c.b(C::Eq, 0x1b13c)?;
    // 0x1b108: ldr r1, [pc, #0x3c]
    c.literal(R::R1, 0x1b14c)?;
    // 0x1b10c: ldr r2, [pc, #0x3c]
    c.literal(R::R2, 0x1b150)?;
    // 0x1b110: mov r3, #143
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8f)?)?;
    // 0x1b114: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b118: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x100)?)?;
    // 0x1b11c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b120: ldr r1, [pc, #0x2c]
    c.literal(R::R1, 0x1b154)?;
    // 0x1b124: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b128: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b12c: ldr r2, [pc, #0x14]
    c.literal(R::R2, 0x1b148)?;
    // 0x1b130: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b134: mov r1, #400
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x190)?)?;
    // 0x1b138: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b13c: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x8)?)?;
    // 0x1b140: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for v in CONTROL_DIAGNOSTICS{c.emit(v)?;}
    c.check(0x1b158)?;
    // 0x1b158: ldr r0, [r0, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 148)?)?;
    // 0x1b15c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x1b160: b 0x2b58 <entry+0xfffffffffffe896c>
    c.b(C::Al, 0x2b58)?;
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    activate_codec_session(c)?;
    codec_controller_loop(c)?;
    diagnose_codec_command(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
  use super::*;
  #[test]fn control_paths_and_literal_pools_have_exact_boundaries(){
    let mut c=Code::new(START);
    extend(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    for (lo,vals) in [(0x1af24,&ATTACH_TRACES[..]),(0x1afd4,&CONTROL_LOOP_GLOBALS[..]),
       (0x1b144,&CONTROL_DIAGNOSTICS[..])] {
      for (index,val) in vals.iter().enumerate(){
        let offset=lo-START+index*4;
        assert_eq!(&c.bytes[offset..offset+4],&val.to_le_bytes());
      }
    }
  }
}
