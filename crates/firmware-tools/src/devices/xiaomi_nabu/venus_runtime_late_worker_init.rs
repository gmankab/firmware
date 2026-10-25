// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus codec session worker loop with six-way mode dispatch. A single
//! functions with literal-address pools separately identified from ARM A32.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x21604;
const END:usize=0x21888;
// Non-executable diagnostic-pointer pool [0x21718,0x21730).
const CONTROL_POOL_0: [u32;6] = [
    0x00021730, // runtime 0x21718
    0x0002175c, // runtime 0x2171c
    0x0002176c, // runtime 0x21720
    0x000217ac, // runtime 0x21724
    0x00021814, // runtime 0x21728
    0x00021848, // runtime 0x2172c
];
// Non-executable diagnostic-pointer pool [0x21878,0x21888).
const CONTROL_POOL_1: [u32;4] = [
    0x0010ceac, // runtime 0x21878
    0x000e53c3, // runtime 0x2187c
    0x0010dfd0, // runtime 0x21880
    0x000e255e, // runtime 0x21884
];
fn initialize_and_check(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x21604: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x21608: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x2160c: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x14)?)?;
    // 0x21610: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x21614: ldr r0, [pc, #0x25c]
    c.literal(R::R0, 0x21878)?;
    // 0x21618: mov r1, #716
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2cc)?)?;
    // 0x2161c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x21620: mov r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x0)?)?;
    // 0x21624: ldr r5, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(4)?, 144)?)?;
    // 0x21628: add r10, r4, #180
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::new(4)?, 0xb4)?)?;
    // 0x2162c: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x21630: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x21634: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x21638: str r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 128)?)?;
    // 0x2163c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x21640: bl 0x7e544 <entry+0x5d148>
    c.bl(0x7e544)?;
    // 0x21644: ldr r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 180)?)?;
    // 0x21648: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x2164c: bne 0x21664 <entry+0x268>
    c.b(C::Ne, 0x21664)?;
    // 0x21650: ldr r2, [pc, #0x224]
    c.literal(R::R2, 0x2187c)?;
    // 0x21654: mov r1, #221
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xdd)?)?;
    // 0x21658: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x2165c: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 0x21660: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21664: mov r6, #716
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x2cc)?)?;
    // 0x21668: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0x2166c: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1c00)?)?;
    // 0x21670: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x21674: bl 0x2efc <entry+0xfffffffffffe1b00>
    c.bl(0x2efc)?;
    // 0x21678: ldr r2, [pc, #0x1fc]
    c.literal(R::R2, 0x2187c)?;
    // 0x2167c: mov r7, r0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    // 0x21680: mov r1, #227
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe3)?)?;
    // 0x21684: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x0)?)?;
    // 0x21688: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 0x2168c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x21690: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21694: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 0x21698: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x10)?)?;
    // 0x2169c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x216a0: mov r3, r7
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 0x216a4: stm sp, {r1, r8}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R1.bit() | R::new(8)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x216a8: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x216ac: str r8, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::SP, 8)?)?;
    // 0x216b0: bl 0x7edc8 <entry+0x5d9cc>
    c.bl(0x7edc8)?;
    // 0x216b4: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(9)?, 0x1)?)?;
    // 0x216b8: cmp r9, #8
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0x8)?)?;
    // 0x216bc: bne 0x21670 <entry+0x274>
    c.b(C::Ne, 0x21670)?;
    // 0x216c0: ldr r0, [pc, #0x1b8]
    c.literal(R::R0, 0x21880)?;
    // 0x216c4: ldr r1, [pc, #0x1b8]
    c.literal(R::R1, 0x21884)?;
    // 0x216c8: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x216cc: bl 0x307c <entry+0xfffffffffffe1c80>
    c.bl(0x307c)?;
    // 0x216d0: mov r9, #708
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x2c4)?)?;
    // 0x216d4: mov r8, #600
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x258)?)?;
    // 0x216d8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x216dc: mov r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x0)?)?;
    // 0x216e0: orr r9, r9, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(9)?, R::new(9)?, 0x1c00)?)?;
    // 0x216e4: orr r8, r8, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(8)?, R::new(8)?, 0x1c00)?)?;
    // 0x216e8: str r6, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::new(5)?, 20)?)?;
    // 0x216ec: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x216f0: bl 0x294c <entry+0xfffffffffffe1550>
    c.bl(0x294c)?;
    // 0x216f4: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x216f8: bl 0x293c <entry+0xfffffffffffe1540>
    c.bl(0x293c)?;
    // 0x216fc: mov r7, r10
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(10)?))?;
    // 0x21700: ldr r0, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 20)?)?;
    // 0x21704: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x5)?)?;
    // 0x21708: bhi 0x21740 <entry+0x344>
    c.b(C::Hi, 0x21740)?;
    // 0x2170c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x21710: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x0)?)?;
    // 0x21714: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.check(0x21718)?;
    for value in CONTROL_POOL_0 {c.emit(value)?;}
    c.check(0x21730)?;
    // 0x21730: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21734: bl 0x214f8 <entry+0xfc>
    c.bl(0x214f8)?;
    // 0x21738: mov r10, r0
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R0))?;
    // 0x2173c: b 0x21868 <entry+0x46c>
    c.b(C::Al, 0x21868)?;
    // 0x21740: ldr r2, [pc, #0x134]
    c.literal(R::R2, 0x2187c)?;
    // 0x21744: mov r1, #94
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5e)?)?;
    // 0x21748: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x2174c: orr r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xc00)?)?;
    // 0x21750: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x21754: mov r10, r7
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(7)?))?;
    // 0x21758: b 0x21868 <entry+0x46c>
    c.b(C::Al, 0x21868)?;
    // 0x2175c: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21760: bl 0x213fc <entry>
    c.bl(0x213fc)?;
    // 0x21764: mov r10, r0
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R0))?;
    // 0x21768: b 0x21868 <entry+0x46c>
    c.b(C::Al, 0x21868)?;
    // 0x2176c: bl 0x26754 <entry+0x5358>
    c.bl(0x26754)?;
    // 0x21770: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21774: mov r2, #632
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x278)?)?;
    // 0x21778: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x2177c: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 24)?)?;
    // 0x21780: str r0, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::R2, Shift::Lsl, 0)?)?;
    // 0x21784: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21788: bl 0x1e704 <entry+0xffffffffffffd308>
    c.bl(0x1e704)?;
    // 0x2178c: bl 0x26754 <entry+0x5358>
    c.bl(0x26754)?;
    // 0x21790: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21794: mov r2, #636
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x27c)?)?;
    // 0x21798: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x2179c: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x217a0: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 24)?)?;
    // 0x217a4: str r0, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::R2, Shift::Lsl, 0)?)?;
    // 0x217a8: b 0x21700 <entry+0x304>
    c.b(C::Al, 0x21700)?;
    // 0x217ac: bl 0x26754 <entry+0x5358>
    c.bl(0x26754)?;
    // 0x217b0: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x217b4: mov r2, #596
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x254)?)?;
    // 0x217b8: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x217bc: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 24)?)?;
    // 0x217c0: str r0, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::R2, Shift::Lsl, 0)?)?;
    // 0x217c4: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x217c8: bl 0x21c78 <entry+0x87c>
    c.bl(0x21c78)?;
    // 0x217cc: ldr r2, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 144)?)?;
    // 0x217d0: mov r3, #7872
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1ec0)?)?;
    // 0x217d4: ldr r1, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R2, 24)?)?;
    // 0x217d8: str r0, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x217dc: ldr r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 16)?)?;
    // 0x217e0: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x7)?)?;
    // 0x217e4: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 0x217e8: bne 0x21800 <entry+0x404>
    c.b(C::Ne, 0x21800)?;
    // 0x217ec: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x217f0: bl 0x21b1c <entry+0x720>
    c.bl(0x21b1c)?;
    // 0x217f4: ldr r2, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 144)?)?;
    // 0x217f8: ldr r1, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R2, 24)?)?;
    // 0x217fc: str r0, [r1, r9]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::new(9)?, Shift::Lsl, 0)?)?;
    // 0x21800: ldr r1, [r2, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R2, 24)?)?;
    // 0x21804: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21808: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x2180c: bl 0x1b164 <entry+0xffffffffffff9d68>
    c.bl(0x1b164)?;
    // 0x21810: b 0x21858 <entry+0x45c>
    c.b(C::Al, 0x21858)?;
    // 0x21814: bl 0x26754 <entry+0x5358>
    c.bl(0x26754)?;
    // 0x21818: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x2181c: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 24)?)?;
    // 0x21820: str r0, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::new(8)?, Shift::Lsl, 0)?)?;
    // 0x21824: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21828: bl 0x21b1c <entry+0x720>
    c.bl(0x21b1c)?;
    // 0x2182c: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21830: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x21834: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 24)?)?;
    // 0x21838: str r0, [r1, r9]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::new(9)?, Shift::Lsl, 0)?)?;
    // 0x2183c: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21840: bl 0xd790 <entry+0xfffffffffffec394>
    c.bl(0xd790)?;
    // 0x21844: b 0x21858 <entry+0x45c>
    c.b(C::Al, 0x21858)?;
    // 0x21848: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x2184c: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 24)?)?;
    // 0x21850: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x21854: bl 0x1a7fc <entry+0xffffffffffff9400>
    c.bl(0x1a7fc)?;
    // 0x21858: mov r10, r0
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R0))?;
    // 0x2185c: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x21860: str r6, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 20)?)?;
    // 0x21864: str r6, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 24)?)?;
    // 0x21868: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x2186c: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 0x0)?)?;
    // 0x21870: beq 0x21700 <entry+0x304>
    c.b(C::Eq, 0x21700)?;
    // 0x21874: b 0x216ec <entry+0x2f0>
    c.b(C::Al, 0x216ec)?;
    c.check(0x21878)?;
    for value in CONTROL_POOL_1 {c.emit(value)?;}
    c.check(0x21888)?;
    c.check(END)
}
pub(super) fn extend(c:&mut Code)->Result<(),String>{initialize_and_check(c)?;c.check(END)}
#[cfg(test)]mod tests{use super::*;#[test]fn controller_entry_exact(){let mut c=Code::new(START);extend(&mut c).unwrap();assert_eq!(c.bytes.len(),END-START);}}
