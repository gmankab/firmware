// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus stream session allocation, shutdown and parameter helpers. Three
//! functions with literal-address pools separately identified from ARM A32.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x21888;
const END:usize=0x21c78;
// Non-executable diagnostic-pointer pool [0x21a7c,0x21a98).
const CONTROL_POOL_0: [u32;7] = [
    0x00021604, // runtime 0x21a7c
    0x000e53c3, // runtime 0x21a80
    0x00080760, // runtime 0x21a84
    0x000e4d6e, // runtime 0x21a88
    0x000e66d1, // runtime 0x21a8c
    0x000e2788, // runtime 0x21a90
    0x000e66e7, // runtime 0x21a94
];
// Non-executable diagnostic-pointer pool [0x21b14,0x21b1c).
const CONTROL_POOL_1: [u32;2] = [
    0x000e6a24, // runtime 0x21b14
    0x000e2788, // runtime 0x21b18
];
// Non-executable diagnostic-pointer pool [0x21c74,0x21c78).
const CONTROL_POOL_2: [u32;1] = [
    0x000e6a43, // runtime 0x21c74
];
fn initialize_and_check(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x21888: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x2188c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x21890: sub sp, sp, #40
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x28)?)?;
    // 0x21894: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x21898: ldr r0, [pc, #0x1dc]
    c.literal(R::R0, 0x21a7c)?;
    // 0x2189c: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    // 0x218a0: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 16)?)?;
    // 0x218a4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x218a8: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 20)?)?;
    // 0x218ac: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2000)?)?;
    // 0x218b0: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 24)?)?;
    // 0x218b4: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 28)?)?;
    // 0x218b8: str r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 32)?)?;
    // 0x218bc: str r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 36)?)?;
    // 0x218c0: add r0, sp, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0xc)?)?;
    // 0x218c4: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x218c8: bl 0x29b4 <entry+0xfffffffffffe15b8>
    c.bl(0x29b4)?;
    // 0x218cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x218d0: beq 0x218e8 <entry+0x4ec>
    c.b(C::Eq, 0x218e8)?;
    // 0x218d4: ldr r2, [pc, #0x1a4]
    c.literal(R::R2, 0x21a80)?;
    // 0x218d8: mov r1, #133
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x85)?)?;
    // 0x218dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x218e0: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x218e4: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x218e8: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x218ec: ldr r1, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al, true, R::R1, R::R0, 4)?)?;
    // 0x218f0: bl 0x2c44 <entry+0xfffffffffffe1848>
    c.bl(0x2c44)?;
    // 0x218f4: ldr r2, [pc, #0x184]
    c.literal(R::R2, 0x21a80)?;
    // 0x218f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x218fc: mov r1, #137
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x89)?)?;
    // 0x21900: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x21904: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21908: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x2190c: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21910: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x21914: bl 0x2efc <entry+0xfffffffffffe1b00>
    c.bl(0x2efc)?;
    // 0x21918: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x2191c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x21920: str r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 16)?)?;
    // 0x21924: bne 0x21944 <entry+0x548>
    c.b(C::Ne, 0x21944)?;
    // 0x21928: ldr r2, [pc, #0x150]
    c.literal(R::R2, 0x21a80)?;
    // 0x2192c: mov r1, #142
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8e)?)?;
    // 0x21930: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x21934: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21938: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x2193c: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21940: ldr r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 16)?)?;
    // 0x21944: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x21948: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x2194c: bl 0x2924 <entry+0xfffffffffffe1528>
    c.bl(0x2924)?;
    // 0x21950: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x21954: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 0x21958: bl 0x293c <entry+0xfffffffffffe1540>
    c.bl(0x293c)?;
    // 0x2195c: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x21960: bl 0x2efc <entry+0xfffffffffffe1b00>
    c.bl(0x2efc)?;
    // 0x21964: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21968: ldr r2, [pc, #0x110]
    c.literal(R::R2, 0x21a80)?;
    // 0x2196c: str r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 8)?)?;
    // 0x21970: mov r1, #149
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x95)?)?;
    // 0x21974: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21978: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x2197c: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21980: mov r3, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20)?)?;
    // 0x21984: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 8)?)?;
    // 0x21988: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 16)?)?;
    // 0x2198c: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    // 0x21990: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x21994: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x21998: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x2199c: ldr r1, [pc, #0xe0]
    c.literal(R::R1, 0x21a84)?;
    // 0x219a0: mov r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1000)?)?;
    // 0x219a4: bl 0x8080c <entry+0x5f410>
    c.bl(0x8080c)?;
    // 0x219a8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x219ac: beq 0x219e8 <entry+0x5ec>
    c.b(C::Eq, 0x219e8)?;
    // 0x219b0: ldr r1, [pc, #0xd0]
    c.literal(R::R1, 0x21a88)?;
    // 0x219b4: ldr r2, [pc, #0xd0]
    c.literal(R::R2, 0x21a8c)?;
    // 0x219b8: mov r3, #158
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x9e)?)?;
    // 0x219bc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x219c0: orr r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0xc00)?)?;
    // 0x219c4: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x219c8: ldr r1, [pc, #0xc0]
    c.literal(R::R1, 0x21a90)?;
    // 0x219cc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x219d0: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x219d4: ldr r2, [pc, #0xa4]
    c.literal(R::R2, 0x21a80)?;
    // 0x219d8: mov r1, #159
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x9f)?)?;
    // 0x219dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x219e0: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x219e4: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x219e8: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x219ec: bl 0x2efc <entry+0xfffffffffffe1b00>
    c.bl(0x2efc)?;
    // 0x219f0: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x219f4: ldr r2, [pc, #0x84]
    c.literal(R::R2, 0x21a80)?;
    // 0x219f8: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 12)?)?;
    // 0x219fc: mov r1, #163
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xa3)?)?;
    // 0x21a00: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21a04: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21a08: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21a0c: mov r3, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20)?)?;
    // 0x21a10: ldr r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 12)?)?;
    // 0x21a14: ldr r1, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 16)?)?;
    // 0x21a18: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    // 0x21a1c: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x21a20: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x21a24: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x21a28: ldr r1, [pc, #0x54]
    c.literal(R::R1, 0x21a84)?;
    // 0x21a2c: mov r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1000)?)?;
    // 0x21a30: bl 0x8080c <entry+0x5f410>
    c.bl(0x8080c)?;
    // 0x21a34: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x21a38: beq 0x21a74 <entry+0x678>
    c.b(C::Eq, 0x21a74)?;
    // 0x21a3c: ldr r1, [pc, #0x50]
    c.literal(R::R1, 0x21a94)?;
    // 0x21a40: ldr r2, [pc, #0x44]
    c.literal(R::R2, 0x21a8c)?;
    // 0x21a44: mov r3, #172
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xac)?)?;
    // 0x21a48: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x21a4c: orr r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0xc00)?)?;
    // 0x21a50: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21a54: ldr r1, [pc, #0x34]
    c.literal(R::R1, 0x21a90)?;
    // 0x21a58: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x21a5c: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21a60: ldr r2, [pc, #0x18]
    c.literal(R::R2, 0x21a80)?;
    // 0x21a64: mov r1, #173
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xad)?)?;
    // 0x21a68: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x21a6c: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21a70: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21a74: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x8)?)?;
    // 0x21a78: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x21a7c)?;
    for value in CONTROL_POOL_0 {c.emit(value)?;}
    c.check(0x21a98)?;
    // 0x21a98: ldr r0, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 144)?)?;
    // 0x21a9c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x21aa0: b 0x2b58 <entry+0xfffffffffffe175c>
    c.b(C::Al, 0x2b58)?;
    // 0x21aa4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x21aa8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x21aac: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x21ab0: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x21ab4: ldr r0, [r5, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 0x21ab8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x21abc: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x21ac0: mov r6, #904
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x388)?)?;
    // 0x21ac4: orr r6, r6, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1800)?)?;
    // 0x21ac8: ldr r1, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R0, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x21acc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x21ad0: beq 0x21b04 <entry+0x708>
    c.b(C::Eq, 0x21b04)?;
    // 0x21ad4: mov r0, r1
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    // 0x21ad8: bl 0x7ecc0 <entry+0x5d8c4>
    c.bl(0x7ecc0)?;
    // 0x21adc: ldr r0, [r5, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 0x21ae0: add r0, r0, r6
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 0x21ae4: bl 0x7e7d4 <entry+0x5d3d8>
    c.bl(0x7e7d4)?;
    // 0x21ae8: ldr r1, [pc, #0x24]
    c.literal(R::R1, 0x21b14)?;
    // 0x21aec: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x21af0: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21af4: ldr r1, [pc, #0x1c]
    c.literal(R::R1, 0x21b18)?;
    // 0x21af8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x21afc: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21b00: ldr r0, [r5, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 0x21b04: bl 0x2f1c <entry+0xfffffffffffe1b20>
    c.bl(0x2f1c)?;
    // 0x21b08: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x21b0c: str r0, [r5, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(5)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 0x21b10: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x21b14)?;
    for value in CONTROL_POOL_1 {c.emit(value)?;}
    c.check(0x21b1c)?;
    // 0x21b1c: push {r11, lr}
    c.emit(arm32::push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    // 0x21b20: mov r11, sp
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    // 0x21b24: ldr r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 16)?)?;
    // 0x21b28: cmp r2, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x7)?)?;
    // 0x21b2c: beq 0x21c04 <entry+0x808>
    c.b(C::Eq, 0x21c04)?;
    // 0x21b30: cmp r2, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x6)?)?;
    // 0x21b34: beq 0x21c2c <entry+0x830>
    c.b(C::Eq, 0x21c2c)?;
    // 0x21b38: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x2)?)?;
    // 0x21b3c: bne 0x21c58 <entry+0x85c>
    c.b(C::Ne, 0x21c58)?;
    // 0x21b40: ldr r2, [r1, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 728)?)?;
    // 0x21b44: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x21b48: beq 0x21c04 <entry+0x808>
    c.b(C::Eq, 0x21c04)?;
    // 0x21b4c: ldr r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 12)?)?;
    // 0x21b50: mov r3, #348
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x15c)?)?;
    // 0x21b54: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x21b58: ldr r0, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R2, Shift::Lsl, 2)?)?;
    // 0x21b5c: mov r2, #336
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x150)?)?;
    // 0x21b60: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x21b64: ldr r2, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R2, Shift::Lsl, 0)?)?;
    // 0x21b68: ldr r3, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R0, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x21b6c: add r2, r2, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xf)?)?;
    // 0x21b70: add r3, r3, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0xf)?)?;
    // 0x21b74: lsr r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsr, 4)?)?;
    // 0x21b78: lsr r3, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsr, 4)?)?;
    // 0x21b7c: mul r12, r3, r2
    c.emit(arm32::multiply_register(C::Al, R::R12, R::new(3)?, R::R2)?)?;
    // 0x21b80: mov r2, #332
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x14c)?)?;
    // 0x21b84: mov r3, #344
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x158)?)?;
    // 0x21b88: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x21b8c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x21b90: ldr r2, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R2, Shift::Lsl, 0)?)?;
    // 0x21b94: ldr r3, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R0, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x21b98: add r2, r2, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xf)?)?;
    // 0x21b9c: add r3, r3, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0xf)?)?;
    // 0x21ba0: lsr r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsr, 4)?)?;
    // 0x21ba4: lsr r3, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsr, 4)?)?;
    // 0x21ba8: mla lr, r3, r2, r12
    c.emit(arm32::multiply_accumulate(C::Al, R::LR, R::new(3)?, R::R2, R::R12)?)?;
    // 0x21bac: mov r2, #328
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x148)?)?;
    // 0x21bb0: mov r3, #340
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x154)?)?;
    // 0x21bb4: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x21bb8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x21bbc: ldr r2, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R2, Shift::Lsl, 0)?)?;
    // 0x21bc0: ldr r0, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x21bc4: add r2, r2, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xf)?)?;
    // 0x21bc8: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 0x21bcc: lsr r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsr, 4)?)?;
    // 0x21bd0: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 0x21bd4: mla r3, r0, r2, lr
    c.emit(arm32::multiply_accumulate(C::Al, R::new(3)?, R::R0, R::R2, R::LR)?)?;
    // 0x21bd8: ldr r0, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 88)?)?;
    // 0x21bdc: ldr r1, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 92)?)?;
    // 0x21be0: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 0x21be4: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xf)?)?;
    // 0x21be8: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 0x21bec: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    // 0x21bf0: mla r2, r1, r0, r3
    c.emit(arm32::multiply_accumulate(C::Al, R::R2, R::R1, R::R0, R::new(3)?)?)?;
    // 0x21bf4: mov r1, #222
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xde)?)?;
    // 0x21bf8: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x21bfc: mul r0, r2, r1
    c.emit(arm32::multiply_register(C::Al, R::R0, R::R2, R::R1)?)?;
    // 0x21c00: b 0x21c6c <entry+0x870>
    c.b(C::Al, 0x21c6c)?;
    // 0x21c04: ldr r0, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 88)?)?;
    // 0x21c08: ldr r1, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 92)?)?;
    // 0x21c0c: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xf)?)?;
    // 0x21c10: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 0x21c14: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    // 0x21c18: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 0x21c1c: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al, R::R2, R::R0, R::R1)?)?;
    // 0x21c20: mov r1, #360
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x168)?)?;
    // 0x21c24: mul r0, r2, r1
    c.emit(arm32::multiply_register(C::Al, R::R0, R::R2, R::R1)?)?;
    // 0x21c28: b 0x21c6c <entry+0x870>
    c.b(C::Al, 0x21c6c)?;
    // 0x21c2c: ldr r0, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 88)?)?;
    // 0x21c30: ldr r1, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 92)?)?;
    // 0x21c34: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xf)?)?;
    // 0x21c38: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 0x21c3c: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    // 0x21c40: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 0x21c44: mul r2, r0, r1
    c.emit(arm32::multiply_register(C::Al, R::R2, R::R0, R::R1)?)?;
    // 0x21c48: mov r1, #728
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2d8)?)?;
    // 0x21c4c: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21c50: mul r0, r2, r1
    c.emit(arm32::multiply_register(C::Al, R::R0, R::R2, R::R1)?)?;
    // 0x21c54: b 0x21c6c <entry+0x870>
    c.b(C::Al, 0x21c6c)?;
    // 0x21c58: ldr r2, [pc, #0x14]
    c.literal(R::R2, 0x21c74)?;
    // 0x21c5c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x21c60: mov r1, #95
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5f)?)?;
    // 0x21c64: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21c68: mvn r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0x1)?)?;
    // 0x21c6c: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x21c70: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x21c74)?;
    for value in CONTROL_POOL_2 {c.emit(value)?;}
    c.check(0x21c78)?;
    c.check(END)
}
pub(super) fn extend(c:&mut Code)->Result<(),String>{initialize_and_check(c)?;c.check(END)}
#[cfg(test)]mod tests{use super::*;#[test]fn controller_entry_exact(){let mut c=Code::new(START);extend(&mut c).unwrap();assert_eq!(c.bytes.len(),END-START);}}
