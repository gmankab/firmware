// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus codec geometry/controller configuration and diagnostic dispatch.
//! This is a byte-level A32 reconstruction from operands, conditions and
//! control flow, NOT an independently verified source-level implementation.
//! The 12-entry dispatch table and 19 diagnostic/configuration words are data.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;

const START: usize = 0x1c0d0;
const END: usize = 0x1c614;

/// Branches into explicit handlers for supported codec input geometries.
/// Exactly 12 entries; compiler/emitter must never decode these as A32.
const GEOMETRY_HANDLER_TARGETS: [u32;12] = [
    0x1c4c4, // geometry mode 0
    0x1c48c, // geometry mode 1
    0x1c4d4, // geometry mode 2
    0x1c578, // geometry mode 3
    0x1c4b8, // geometry mode 4
    0x1c550, // geometry mode 5
    0x1c564, // geometry mode 6
    0x1c570, // geometry mode 7
    0x1c48c, // geometry mode 8
    0x1c578, // geometry mode 9
    0x1c48c, // geometry mode 10
    0x1c4b8, // geometry mode 11
];

/// Runtime pointers, diagnostic literals and one non-pointer magic word.
/// These are independent word-valued data, not instruction opcodes.
const CODEC_GEOMETRY_LITERALS: [u32;19] = [
    0x0010df5c, // data entry 0
    0x001040e4, // data entry 1
    0x000e53ee, // data entry 2
    0x000e2788, // data entry 3
    0x0010df7c, // data entry 4
    0x000e541f, // data entry 5
    0x000e5471, // data entry 6
    0x000e68a4, // data entry 7
    0x000e6900, // data entry 8
    0x000e695c, // data entry 9
    0x000e69b8, // data entry 10
    0xabaa8a49, // data entry 11
    0x000e671c, // data entry 12
    0x000e6728, // data entry 13
    0x000e6a14, // data entry 14
    0x000e6734, // data entry 15
    0x000e6790, // data entry 16
    0x000e67ec, // data entry 17
    0x000e6848, // data entry 18
];

fn configure_codec_geometry(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1c0d0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1c0d4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1c0d8: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x1c)?)?;
    // 0x1c0dc: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1c0e0: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1c0e4: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x1c0e8: mov r0, #320
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x140)?)?;
    // 0x1c0ec: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 24)?)?;
    // 0x1c0f0: mov r5, r2
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R2))?;
    // 0x1c0f4: str r6, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::new(4)?, 32)?)?;
    // 0x1c0f8: str r7, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 36)?)?;
    // 0x1c0fc: str r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 40)?)?;
    // 0x1c100: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1c104: str r6, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::new(4)?, 44)?)?;
    // 0x1c108: str r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 52)?)?;
    // 0x1c10c: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 56)?)?;
    // 0x1c110: bl 0x2f8ac <entry+0x145ac>
    c.bl(0x2f8ac)?;
    // 0x1c114: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c118: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c11c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c120: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x2)?)?;
    // 0x1c124: str r0, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 60)?)?;
    // 0x1c128: bne 0x1c13c <entry+0xe3c>  
    c.b(C::Ne, 0x1c13c)?;
    // 0x1c12c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10)?)?;
    // 0x1c130: str r7, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 72)?)?;
    // 0x1c134: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 88)?)?;
    // 0x1c138: b 0x1c178 <entry+0xe78>  
    c.b(C::Al, 0x1c178)?;
    // 0x1c13c: str r6, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::new(4)?, 72)?)?;
    // 0x1c140: bl 0x2f8d0 <entry+0x145d0>
    c.bl(0x2f8d0)?;
    // 0x1c144: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c148: beq 0x1c164 <entry+0xe64>  
    c.b(C::Eq, 0x1c164)?;
    // 0x1c14c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c150: str r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 72)?)?;
    // 0x1c154: str r0, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 60)?)?;
    // 0x1c158: mov r0, #14
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xe)?)?;
    // 0x1c15c: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 88)?)?;
    // 0x1c160: b 0x1c178 <entry+0xe78>  
    c.b(C::Al, 0x1c178)?;
    // 0x1c164: bl 0x2f8ac <entry+0x145ac>
    c.bl(0x2f8ac)?;
    // 0x1c168: mov r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf)?)?;
    // 0x1c16c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c170: movne r1, #14
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 0xe)?)?;
    // 0x1c174: str r1, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 88)?)?;
    // 0x1c178: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c17c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1c180: str r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 64)?)?;
    // 0x1c184: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 68)?)?;
    // 0x1c188: str r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 76)?)?;
    // 0x1c18c: str r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 80)?)?;
    // 0x1c190: str r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 84)?)?;
    // 0x1c194: ldr r0, [pc, #0x42c]       
    c.literal(R::R0, 0x1c5c8)?;
    // 0x1c198: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 0x1c19c: tst r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R1, 0x4)?)?;
    // 0x1c1a0: ldrne r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Ne, true, R::R2, R::R0, 20)?)?;
    // 0x1c1a4: strne r2, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Ne, false, R::R2, R::new(4)?, 88)?)?;
    // 0x1c1a8: tst r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R1, 0x2)?)?;
    // 0x1c1ac: ldrne r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Ne, true, R::R2, R::R0, 16)?)?;
    // 0x1c1b0: strne r2, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Ne, false, R::R2, R::new(4)?, 68)?)?;
    // 0x1c1b4: tst r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R1, 0x1)?)?;
    // 0x1c1b8: ldrne r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Ne, true, R::R0, R::R0, 12)?)?;
    // 0x1c1bc: strne r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 72)?)?;
    // 0x1c1c0: ldr r0, [pc, #0x404]       
    c.literal(R::R0, 0x1c5cc)?;
    // 0x1c1c4: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x1c1c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c1cc: beq 0x1c248 <entry+0xf48>  
    c.b(C::Eq, 0x1c248)?;
    // 0x1c1d0: ldr r1, [pc, #0x3f8]       
    c.literal(R::R1, 0x1c5d0)?;
    // 0x1c1d4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c1d8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c1dc: ldr r5, [pc, #0x3f0]       
    c.literal(R::new(5)?, 0x1c5d4)?;
    // 0x1c1e0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c1e4: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1c1e8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c1ec: ldr r6, [pc, #0x3e4]       
    c.literal(R::new(6)?, 0x1c5d8)?;
    // 0x1c1f0: add r7, r6, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(6)?, 0xc)?)?;
    // 0x1c1f4: ldm r6, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(6)?, R::R2.bit() | R::new(3)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c1f8: ldm r7, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(7)?, R::R0.bit() | R::R1.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c1fc: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c200: ldr r1, [pc, #0x3d4]       
    c.literal(R::R1, 0x1c5dc)?;
    // 0x1c204: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c208: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c20c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c210: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1c214: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c218: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1c21c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1c220: and r2, r1, r0, lsr #2
    c.emit(arm32::data_register(C::Al, Op::And, R::R2, R::R1, R::R0, Shift::Lsr, 2, false)?)?;
    // 0x1c224: and r3, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::new(3)?, R::R1, R::R0, Shift::Lsr, 1, false)?)?;
    // 0x1c228: ldr r1, [pc, #0x3b0]       
    c.literal(R::R1, 0x1c5e0)?;
    // 0x1c22c: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x1c230: and r2, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::And, R::R2, R::R0, 0x1)?)?;
    // 0x1c234: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c238: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c23c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c240: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1c244: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1c248: mov r0, #96
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x60)?)?;
    // 0x1c24c: mov r1, #48
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x30)?)?;
    // 0x1c250: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x1c254: add r3, r4, #92
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(4)?, 0x5c)?)?;
    // 0x1c258: mov r2, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x3)?)?;
    // 0x1c25c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0x1c260: mov r8, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x8)?)?;
    // 0x1c264: stm r3, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::new(3)?, R::R0.bit() | R::R1.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c268: str r2, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 104)?)?;
    // 0x1c26c: add r0, r4, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x80)?)?;
    // 0x1c270: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1c274: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c278: str r9, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 112)?)?;
    // 0x1c27c: str r9, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 116)?)?;
    // 0x1c280: str r8, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::new(4)?, 120)?)?;
    // 0x1c284: str r8, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::new(4)?, 124)?)?;
    // 0x1c288: bl 0x31ec <entry+0xfffffffffffe7eec>
    c.bl(0x31ec)?;
    // 0x1c28c: ldr r1, [pc, #0x350]       
    c.literal(R::R1, 0x1c5e4)?;
    // 0x1c290: add r6, r4, #220
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::new(4)?, 0xdc)?)?;
    // 0x1c294: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c298: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x1c29c: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c2a0: ldr r1, [pc, #0x340]       
    c.literal(R::R1, 0x1c5e8)?;
    // 0x1c2a4: add r10, r4, #404
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::new(4)?, 0x194)?)?;
    // 0x1c2a8: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c2ac: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1c2b0: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c2b4: ldr r1, [pc, #0x330]       
    c.literal(R::R1, 0x1c5ec)?;
    // 0x1c2b8: add r0, r4, #496
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x1f0)?)?;
    // 0x1c2bc: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c2c0: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 16)?)?;
    // 0x1c2c4: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c2c8: ldr r1, [pc, #0x320]       
    c.literal(R::R1, 0x1c5f0)?;
    // 0x1c2cc: add r0, r4, #588
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x24c)?)?;
    // 0x1c2d0: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c2d4: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 20)?)?;
    // 0x1c2d8: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c2dc: mov r0, #500
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1f4)?)?;
    // 0x1c2e0: str r9, [r4, #0x2a8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 680)?)?;
    // 0x1c2e4: mov r1, #100
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x64)?)?;
    // 0x1c2e8: mov r2, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5)?)?;
    // 0x1c2ec: mov r3, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x7)?)?;
    // 0x1c2f0: mov r5, #36
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x24)?)?;
    // 0x1c2f4: str r0, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 720)?)?;
    // 0x1c2f8: mov r0, #50
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x32)?)?;
    // 0x1c2fc: str r9, [r4, #0x2d4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 724)?)?;
    // 0x1c300: str r0, [r4, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 728)?)?;
    // 0x1c304: str r0, [r4, #0x2dc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 732)?)?;
    // 0x1c308: str r9, [r4, #0x2e0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 736)?)?;
    // 0x1c30c: str r7, [r4, #0x2e4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 740)?)?;
    // 0x1c310: ldr r0, [pc, #0x2dc]       
    c.literal(R::R0, 0x1c5f4)?;
    // 0x1c314: str r0, [r4, #0x2e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 744)?)?;
    // 0x1c318: mov r0, #162
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xa2)?)?;
    // 0x1c31c: orr r0, r0, #74752
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x12400)?)?;
    // 0x1c320: str r0, [r4, #0x2ec]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 748)?)?;
    // 0x1c324: mov r0, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x20)?)?;
    // 0x1c328: str r7, [r4, #0x2f0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 752)?)?;
    // 0x1c32c: str r7, [r4, #0x2f4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 756)?)?;
    // 0x1c330: str r0, [r4, #0x2f8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 760)?)?;
    // 0x1c334: mov r0, #18
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x12)?)?;
    // 0x1c338: str r9, [r4, #0x2fc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 764)?)?;
    // 0x1c33c: str r9, [r4, #0x304]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 772)?)?;
    // 0x1c340: str r9, [r4, #0x308]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 776)?)?;
    // 0x1c344: str r0, [r4, #0x30c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 780)?)?;
    // 0x1c348: str r0, [r4, #0x310]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 784)?)?;
    // 0x1c34c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1c350: str r8, [r4, #0x314]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::new(4)?, 788)?)?;
    // 0x1c354: str r8, [r4, #0x318]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::new(4)?, 792)?)?;
    // 0x1c358: str r7, [r4, #0x31c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 796)?)?;
    // 0x1c35c: str r9, [r4, #0x320]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 800)?)?;
    // 0x1c360: str r9, [r4, #0x324]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 804)?)?;
    // 0x1c364: str r9, [r4, #0x328]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 808)?)?;
    // 0x1c368: str r7, [r4, #0x32c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 812)?)?;
    // 0x1c36c: str r9, [r4, #0x330]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 816)?)?;
    // 0x1c370: str r9, [r4, #0x334]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 820)?)?;
    // 0x1c374: str r0, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 824)?)?;
    // 0x1c378: str r0, [r4, #0x33c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 828)?)?;
    // 0x1c37c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1c380: str r9, [r4, #0x34c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 844)?)?;
    // 0x1c384: str r9, [r4, #0x348]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 840)?)?;
    // 0x1c388: str r9, [r4, #0x344]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 836)?)?;
    // 0x1c38c: str r9, [r4, #0x340]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 832)?)?;
    // 0x1c390: str r7, [r4, #0x350]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 848)?)?;
    // 0x1c394: str r7, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 852)?)?;
    // 0x1c398: str r0, [r4, #0x35c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 860)?)?;
    // 0x1c39c: str r9, [r4, #0x358]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 856)?)?;
    // 0x1c3a0: str r1, [r4, #0x360]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 864)?)?;
    // 0x1c3a4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x100)?)?;
    // 0x1c3a8: str r1, [r4, #0x364]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 868)?)?;
    // 0x1c3ac: str r9, [r4, #0x368]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 872)?)?;
    // 0x1c3b0: str r2, [r4, #0x36c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 876)?)?;
    // 0x1c3b4: mov r2, #424
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1a8)?)?;
    // 0x1c3b8: str r3, [r4, #0x370]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 880)?)?;
    // 0x1c3bc: str r3, [r4, #0x374]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 884)?)?;
    // 0x1c3c0: mov r1, #10
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xa)?)?;
    // 0x1c3c4: add r3, r4, #912
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(4)?, 0x390)?)?;
    // 0x1c3c8: str r9, [r4, #0x378]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 888)?)?;
    // 0x1c3cc: str r5, [r4, #0x37c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(4)?, 892)?)?;
    // 0x1c3d0: str r9, [r4, #0x380]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 896)?)?;
    // 0x1c3d4: str r9, [r4, #0x384]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 900)?)?;
    // 0x1c3d8: str r9, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::new(4)?, 904)?)?;
    // 0x1c3dc: str r0, [r4, #0x38c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 908)?)?;
    // 0x1c3e0: orr r2, r2, #24576
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x6000)?)?;
    // 0x1c3e4: stm r3, {r0, r1, r2, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::new(3)?, R::R0.bit() | R::R1.bit() | R::R2.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c3e8: ldr r1, [pc, #0x208]       
    c.literal(R::R1, 0x1c5f8)?;
    // 0x1c3ec: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 0x1c3f0: str r0, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 928)?)?;
    // 0x1c3f4: add r0, r4, #932
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x3a4)?)?;
    // 0x1c3f8: mov r2, r1
    c.emit(arm32::mov_register(C::Al, R::R2, R::R1))?;
    // 0x1c3fc: ldm r2, {r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R2, R::new(3)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c400: stm r0, {r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::new(3)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c404: add r0, r4, #944
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x3b0)?)?;
    // 0x1c408: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c40c: ldr r1, [pc, #0x1e8]       
    c.literal(R::R1, 0x1c5fc)?;
    // 0x1c410: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c414: add r0, r4, #956
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x3bc)?)?;
    // 0x1c418: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c41c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1c420: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c424: add r0, r4, #968
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x3c8)?)?;
    // 0x1c428: mov r2, #20
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x14)?)?;
    // 0x1c42c: bl 0x31ec <entry+0xfffffffffffe7eec>
    c.bl(0x31ec)?;
    // 0x1c430: ldr r1, [pc, #0x1c8]       
    c.literal(R::R1, 0x1c600)?;
    // 0x1c434: add r0, r4, #988
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x3dc)?)?;
    // 0x1c438: ldm r1, {r2, r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c43c: stm r0, {r2, r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c440: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x1c444: ldr r0, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R2, 16)?)?;
    // 0x1c448: cmp r0, #11
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0xb)?)?;
    // 0x1c44c: bhi 0x1c578 <entry+0x1278> 
    c.b(C::Hi, 0x1c578)?;
    // 0x1c450: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1c454: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x0)?)?;
    // 0x1c458: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;

    c.check(0x1c45c)?;
    for destination in GEOMETRY_HANDLER_TARGETS { c.emit(destination)?; }
    c.check(0x1c48c)
}

fn select_controller_geometry(c:&mut Code)->Result<(),String>{
    c.check(0x1c48c)?;
    // 0x1c48c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 0x1c490: str r0, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 928)?)?;
    // 0x1c494: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1c498: str r0, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 824)?)?;
    // 0x1c49c: str r0, [r4, #0x33c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 828)?)?;
    // 0x1c4a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c4a4: str r0, [r4, #0x350]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 848)?)?;
    // 0x1c4a8: str r0, [r4, #0x354]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 852)?)?;
    // 0x1c4ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c4b0: str r0, [r4, #0x35c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 860)?)?;
    // 0x1c4b4: b 0x1c578 <entry+0x1278> 
    c.b(C::Al, 0x1c578)?;
    // 0x1c4b8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 0x1c4bc: str r0, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 928)?)?;
    // 0x1c4c0: b 0x1c578 <entry+0x1278> 
    c.b(C::Al, 0x1c578)?;
    // 0x1c4c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c4c8: str r0, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 824)?)?;
    // 0x1c4cc: str r0, [r4, #0x33c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 828)?)?;
    // 0x1c4d0: b 0x1c578 <entry+0x1278> 
    c.b(C::Al, 0x1c578)?;
    // 0x1c4d4: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 0x1c4d8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2)?)?;
    // 0x1c4dc: str r0, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 928)?)?;
    // 0x1c4e0: mov r0, #6
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x6)?)?;
    // 0x1c4e4: str r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 980)?)?;
    // 0x1c4e8: str r0, [r4, #0x3d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 984)?)?;
    // 0x1c4ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c4f0: str r0, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 824)?)?;
    // 0x1c4f4: mov r0, #256
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x100)?)?;
    // 0x1c4f8: str r1, [r4, #0x33c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 828)?)?;
    // 0x1c4fc: str r0, [r4, #0x364]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 868)?)?;
    // 0x1c500: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c504: str r0, [r4, #0x2a8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 680)?)?;
    // 0x1c508: str r0, [r4, #0x2d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 724)?)?;
    // 0x1c50c: ldr r0, [r2, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R2, 728)?)?;
    // 0x1c510: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1c514: beq 0x1c580 <entry+0x1280> 
    c.b(C::Eq, 0x1c580)?;
    // 0x1c518: ldr r1, [pc, #0xc4]        
    c.literal(R::R1, 0x1c5e4)?;
    // 0x1c51c: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x1c520: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c524: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c528: ldr r1, [pc, #0xb8]        
    c.literal(R::R1, 0x1c5e8)?;
    // 0x1c52c: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1c530: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c534: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c538: ldr r1, [pc, #0xac]        
    c.literal(R::R1, 0x1c5ec)?;
    // 0x1c53c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    // 0x1c540: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c544: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c548: ldr r1, [pc, #0xa0]        
    c.literal(R::R1, 0x1c5f0)?;
    // 0x1c54c: b 0x1c5b4 <entry+0x12b4> 
    c.b(C::Al, 0x1c5b4)?;
    // 0x1c550: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1c554: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1c558: str r0, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 824)?)?;
    // 0x1c55c: str r1, [r4, #0x33c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 828)?)?;
    // 0x1c560: b 0x1c578 <entry+0x1278> 
    c.b(C::Al, 0x1c578)?;
    // 0x1c564: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1c568: str r0, [r4, #0x300]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 768)?)?;
    // 0x1c56c: b 0x1c574 <entry+0x1274> 
    c.b(C::Al, 0x1c574)?;
    // 0x1c570: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1c574: str r0, [r4, #0x304]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 772)?)?;
    // 0x1c578: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1c57c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1c580: ldr r1, [pc, #0x7c]        
    c.literal(R::R1, 0x1c604)?;
    // 0x1c584: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x1c588: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c58c: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c590: ldr r1, [pc, #0x70]        
    c.literal(R::R1, 0x1c608)?;
    // 0x1c594: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1c598: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c59c: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c5a0: ldr r1, [pc, #0x64]        
    c.literal(R::R1, 0x1c60c)?;
    // 0x1c5a4: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    // 0x1c5a8: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c5ac: bl 0x471c <entry+0xfffffffffffe941c>
    c.bl(0x471c)?;
    // 0x1c5b0: ldr r1, [pc, #0x58]        
    c.literal(R::R1, 0x1c610)?;
    // 0x1c5b4: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 20)?)?;
    // 0x1c5b8: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1c5bc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1c5c0: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1c5c4: b 0x471c <entry+0xfffffffffffe941c>
    c.b(C::Al, 0x471c)?;

    c.check(0x1c5c8)?;
    for word in CODEC_GEOMETRY_LITERALS { c.emit(word)?; }
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    configure_codec_geometry(c)?;
    select_controller_geometry(c)?;
    c.check(END)
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn geometry_data_words_are_separate() {
        let mut c=Code::new(START);
        extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        for (i,word) in GEOMETRY_HANDLER_TARGETS.iter().enumerate() {
            let offset=0x1c45c-START+i*4;
            assert_eq!(&c.bytes[offset..offset+4],&word.to_le_bytes());
        }
        for (i,word) in CODEC_GEOMETRY_LITERALS.iter().enumerate() {
            let offset=0x1c5c8-START+i*4;
            assert_eq!(&c.bytes[offset..offset+4],&word.to_le_bytes());
        }
    }
}
