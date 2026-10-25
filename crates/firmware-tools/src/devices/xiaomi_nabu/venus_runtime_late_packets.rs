// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus ARM32 codec session command queue setup.
//! Branches distinguish a short command sequence from an expanded command
//! sequence; both publish command indices into controller-owned queues.
//! Original A32 instructions are emitted from checked typed operands and
//! the trailing codec-format code is represented as non-executable data.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize = 0x1a1ec;
const END:usize = 0x1a4d4;
const FORMAT_IDENTIFIER:u32=0x0401_1009;

/// Validate session state and format, then submit hardware commands.
fn encode_codec_session_commands(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1a1ec: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1a1f0: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x18)?)?;
    // 0x1a1f4: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1a1f8: mov r0, #684
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2ac)?)?;
    // 0x1a1fc: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x1a200: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1a204: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a208: ldr r8, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(5)?, 132)?)?;
    // 0x1a20c: ldr r2, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a210: ldr r0, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 144)?)?;
    // 0x1a214: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 32)?)?;
    // 0x1a218: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1a21c: bne 0x1a258 <entry+0x6c>
    c.b(C::Ne, 0x1a258)?;
    // 0x1a220: ldr r2, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 12)?)?;
    // 0x1a224: mov r3, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x21c)?)?;
    // 0x1a228: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1a22c: ldr r2, [r5, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::new(5)?, R::R2, Shift::Lsl, 2)?)?;
    // 0x1a230: ldr r3, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x1a234: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x0)?)?;
    // 0x1a238: bne 0x1a258 <entry+0x6c>
    c.b(C::Ne, 0x1a258)?;
    // 0x1a23c: mov r3, #496
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1f0)?)?;
    // 0x1a240: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1a244: ldr r2, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x1a248: mvn r3, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0x8000)?)?;
    // 0x1a24c: add r2, r2, r3
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x1a250: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x1)?)?;
    // 0x1a254: movls r1, #1
    c.emit(arm32::data_immediate(C::Ls, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1a258: ldr r2, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 68)?)?;
    // 0x1a25c: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x4)?)?;
    // 0x1a260: blo 0x1a3ac <entry+0x1c0>
    c.b(C::Cc, 0x1a3ac)?;
    // 0x1a264: mov r2, #680
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2a8)?)?;
    // 0x1a268: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x1a26c: ldr r2, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::new(4)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1a270: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1a274: bne 0x1a288 <entry+0x9c>
    c.b(C::Ne, 0x1a288)?;
    // 0x1a278: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 0x1a27c: ldrne r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Ne, true, R::R0, R::R0, 16)?)?;
    // 0x1a280: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1a284: bne 0x1a3ac <entry+0x1c0>
    c.b(C::Ne, 0x1a3ac)?;
    // 0x1a288: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a28c: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a290: bl 0x183e0 <entry+0xffffffffffffe1f4>
    c.bl(0x183e0)?;
    // 0x1a294: mov r0, #564
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x234)?)?;
    // 0x1a298: mov r9, r4
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(4)?))?;
    // 0x1a29c: mov r1, #112
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x70)?)?;
    // 0x1a2a0: mov r2, #568
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x238)?)?;
    // 0x1a2a4: mov r6, r4
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(4)?))?;
    // 0x1a2a8: mov r3, #255
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xff)?)?;
    // 0x1a2ac: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1a2b0: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a2b4: orr r1, r1, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8000)?)?;
    // 0x1a2b8: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x1a2bc: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0xff00)?)?;
    // 0x1a2c0: ldr r0, [r9, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a2c4: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1a2c8: ldr r2, [r6, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(6)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1a2cc: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0x1a2d0: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x8)?)?;
    // 0x1a2d4: str r3, [r9]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(9)?, 0)?)?;
    // 0x1a2d8: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 4)?)?;
    // 0x1a2dc: add r0, r2, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x8)?)?;
    // 0x1a2e0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a2e4: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 0)?)?;
    // 0x1a2e8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a2ec: str r7, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R2, 4)?)?;
    // 0x1a2f0: bl 0x163e8 <entry+0xffffffffffffc1fc>
    c.bl(0x163e8)?;
    // 0x1a2f4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a2f8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a2fc: bl 0x1698c <entry+0xffffffffffffc7a0>
    c.bl(0x1698c)?;
    // 0x1a300: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a304: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a308: bl 0x174e4 <entry+0xffffffffffffd2f8>
    c.bl(0x174e4)?;
    // 0x1a30c: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a310: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a314: bl 0x17fd0 <entry+0xffffffffffffdde4>
    c.bl(0x17fd0)?;
    // 0x1a318: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 0)?)?;
    // 0x1a31c: mov r1, #1232
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4d0)?)?;
    // 0x1a320: orr r1, r1, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x9000)?)?;
    // 0x1a324: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1a328: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 8)?)?;
    // 0x1a32c: ldr r2, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 68)?)?;
    // 0x1a330: ldr r3, [pc, #0x198]
    c.literal(R::new(3)?, 0x1a4d0)?;
    // 0x1a334: cmp r1, r3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x1a338: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1a33c: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x1a340: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x3)?)?;
    // 0x1a344: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 0)?)?;
    // 0x1a348: lsl r3, r7, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(7)?, Shift::Lsl, 4)?)?;
    // 0x1a34c: orrhi r3, r1, r7, lsl #4
    c.emit(arm32::data_register(C::Hi, Op::Orr, R::new(3)?, R::R1, R::new(7)?, Shift::Lsl, 4, false)?)?;
    // 0x1a350: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(8)?, 0x0)?)?;
    // 0x1a354: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0x1a358: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x8)?)?;
    // 0x1a35c: str r3, [r9]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(9)?, 0)?)?;
    // 0x1a360: mov r3, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8400)?)?;
    // 0x1a364: str r3, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 4)?)?;
    // 0x1a368: add r3, r2, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R2, 0x8)?)?;
    // 0x1a36c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(6)?, 0)?)?;
    // 0x1a370: str r1, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 4)?)?;
    // 0x1a374: beq 0x1a4c8 <entry+0x2dc>
    c.b(C::Eq, 0x1a4c8)?;
    // 0x1a378: mov r3, #1120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x460)?)?;
    // 0x1a37c: orr r3, r3, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8000)?)?;
    // 0x1a380: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 8)?)?;
    // 0x1a384: add r3, r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x10)?)?;
    // 0x1a388: str r1, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 8)?)?;
    // 0x1a38c: str r3, [r9]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(9)?, 0)?)?;
    // 0x1a390: mov r3, #100
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x64)?)?;
    // 0x1a394: orr r3, r3, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8400)?)?;
    // 0x1a398: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 12)?)?;
    // 0x1a39c: add r0, r2, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x10)?)?;
    // 0x1a3a0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 0)?)?;
    // 0x1a3a4: str r1, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 12)?)?;
    // 0x1a3a8: b 0x1a4c8 <entry+0x2dc>
    c.b(C::Al, 0x1a4c8)?;
    // 0x1a3ac: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a3b0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a3b4: bl 0x183e0 <entry+0xffffffffffffe1f4>
    c.bl(0x183e0)?;
    // 0x1a3b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a3bc: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a3c0: bl 0x15ea8 <entry+0xffffffffffffbcbc>
    c.bl(0x15ea8)?;
    // 0x1a3c4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a3c8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a3cc: bl 0xe3f8 <entry+0xffffffffffff420c>
    c.bl(0xe3f8)?;
    // 0x1a3d0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a3d4: bl 0x12cc4 <entry+0xffffffffffff8ad8>
    c.bl(0x12cc4)?;
    // 0x1a3d8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a3dc: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a3e0: bl 0x13b14 <entry+0xffffffffffff9928>
    c.bl(0x13b14)?;
    // 0x1a3e4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a3e8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a3ec: bl 0x12e5c <entry+0xffffffffffff8c70>
    c.bl(0x12e5c)?;
    // 0x1a3f0: mov r0, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    // 0x1a3f4: mov r5, r4
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(4)?))?;
    // 0x1a3f8: ldr r2, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2648)?)?;
    // 0x1a3fc: mov r6, r4
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(4)?))?;
    // 0x1a400: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a404: ldr r0, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a408: ldr r2, [r2, #0x364]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R2, 868)?)?;
    // 0x1a40c: add r1, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x4)?)?;
    // 0x1a410: str r1, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(5)?, 0)?)?;
    // 0x1a414: mov r1, #44
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2c)?)?;
    // 0x1a418: orr r1, r1, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    // 0x1a41c: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1a420: mov r0, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x22c)?)?;
    // 0x1a424: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a428: ldr r0, [r6, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(6)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a42c: add r1, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x4)?)?;
    // 0x1a430: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 0)?)?;
    // 0x1a434: lsl r1, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R2, Shift::Lsl, 2)?)?;
    // 0x1a438: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1a43c: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a440: bl 0x12cc4 <entry+0xffffffffffff8ad8>
    c.bl(0x12cc4)?;
    // 0x1a444: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 0)?)?;
    // 0x1a448: mov r0, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x108)?)?;
    // 0x1a44c: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0x1)?)?;
    // 0x1a450: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(8)?, 0x0)?)?;
    // 0x1a454: orr r0, r0, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8c00)?)?;
    // 0x1a458: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    // 0x1a45c: add r3, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R1, 0xc)?)?;
    // 0x1a460: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 0)?)?;
    // 0x1a464: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x1a468: mov r2, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xd4)?)?;
    // 0x1a46c: orr r2, r2, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9400)?)?;
    // 0x1a470: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x1a474: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1a478: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 4)?)?;
    // 0x1a47c: str r3, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(5)?, 0)?)?;
    // 0x1a480: mov r3, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8c00)?)?;
    // 0x1a484: str r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 8)?)?;
    // 0x1a488: add r3, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xc)?)?;
    // 0x1a48c: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(6)?, 0)?)?;
    // 0x1a490: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 8)?)?;
    // 0x1a494: beq 0x1a4c8 <entry+0x2dc>
    c.b(C::Eq, 0x1a4c8)?;
    // 0x1a498: mov r3, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8d00)?)?;
    // 0x1a49c: str r3, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 12)?)?;
    // 0x1a4a0: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 12)?)?;
    // 0x1a4a4: add r2, r1, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x14)?)?;
    // 0x1a4a8: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(5)?, 0)?)?;
    // 0x1a4ac: mov r2, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x104)?)?;
    // 0x1a4b0: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x1a4b4: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 16)?)?;
    // 0x1a4b8: add r1, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x14)?)?;
    // 0x1a4bc: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 0)?)?;
    // 0x1a4c0: mov r1, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3f)?)?;
    // 0x1a4c4: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x1a4c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1a4cc: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(FORMAT_IDENTIFIER)?;
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    encode_codec_session_commands(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn full_session_command_queue_layout(){
        let mut c=Code::new(START);
        extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[c.bytes.len()-4..],&FORMAT_IDENTIFIER.to_le_bytes());
    }
}
