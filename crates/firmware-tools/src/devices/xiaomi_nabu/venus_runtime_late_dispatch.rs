// SPDX-License-Identifier: GPL-3.0-only
//! Venus late codec stream dispatch with a twenty-way bounded jump table.
//! The observed routine compares packet state, updates descriptor fields,
//! handles per-codec error traces, and reads status values from typed A32.
//! Its branch table and 33-word literal table are separately typed data;
//! neither is treated as decoded A32 instructions or included verbatim from
//! a proprietary executable instruction array.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize = 0x1a8a8;
const END:usize = 0x1ae84;
const MODE_DISPATCH_TARGETS: [u32;20] = [
    // Twenty mode-class branch destinations in this same routine.
    0x0001a950, 0x0001a95c, 0x0001a968, 0x0001a938, 0x0001a968,
    0x0001a9a0, 0x0001a9ac, 0x0001a968, 0x0001a9b8, 0x0001a968,
    0x0001a9c4, 0x0001a968, 0x0001a9d0, 0x0001a938, 0x0001a968,
    0x0001a938, 0x0001a968, 0x0001a968, 0x0001a968, 0x0001a9dc,
];
const RESOURCE_DIAGNOSTICS: [u32;33] = [
    // Controller diagnostics, runtime lookup addresses and two signed offsets.
    0xfbfeeffd, 0x000e4940, 0x000e2788, 0x000e490b, 0x000e48d0,
    0x000e4895, 0x000e485a, 0x000e481f, 0x000e47df, 0x000e47a8,
    0x000e4774, 0x000e497f, 0x000e499e, 0x000e471e, 0x000e49b5,
    0xffffe1c4, 0xffffe1c0, 0x000e49e3, 0x000e4a83, 0x000e4ab0,
    0x000e4add, 0x000e4b0a, 0x000e4b37, 0x000e4b64, 0x000e4b91,
    0x000e4bbe, 0x000e4beb, 0x000e4c18, 0x000e4c45, 0x000e4c72,
    0x000e4c9f, 0x000e4ccc, 0x000e4cf9,
];

/// Begin decoding the codec frame classifier; a twenty-way table selects
/// codec-specific stream start/update cases.
fn emit_dispatch_prologue(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1a8a8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1a8ac: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1a8b0: sub sp, sp, #140
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8c)?)?;
    // 0x1a8b4: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x1a8b8: ldr r0, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 148)?)?;
    // 0x1a8bc: ldr r4, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::R0, 20)?)?;
    // 0x1a8c0: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x0)?)?;
    // 0x1a8c4: beq 0x1a944 <entry+0x758>
    c.b(C::Eq, 0x1a944)?;
    // 0x1a8c8: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 8)?)?;
    // 0x1a8cc: ldr r1, [pc, #0x52c]
    c.literal(R::R1, 0x1ae00)?;
    // 0x1a8d0: add r1, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1a8d4: cmp r1, #19
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x13)?)?;
    // 0x1a8d8: bhi 0x1a968 <entry+0x77c>
    c.b(C::Hi, 0x1a968)?;
    // 0x1a8dc: lsl r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 2)?)?;
    // 0x1a8e0: add r2, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::PC, 0x0)?)?;
    // 0x1a8e4: ldr pc, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R1, R::R2, Shift::Lsl, 0)?)?;
    for addr in MODE_DISPATCH_TARGETS { c.emit(addr)?; }
    c.check(0x1a938)
}

/// Common codec processing, trace dispatch and entry of selected format.
fn emit_dispatch_cases(c:&mut Code)->Result<(),String>{
    c.check(0x1a938)?;
    // 0x1a938: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a93c: ldr r1, [pc, #0x4dc]
    c.literal(R::R1, 0x1ae20)?;
    // 0x1a940: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a944: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1a948: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x1a94c: b 0x1adf4 <entry+0xc08>
    c.b(C::Al, 0x1adf4)?;
    // 0x1a950: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a954: ldr r1, [pc, #0x4cc]
    c.literal(R::R1, 0x1ae28)?;
    // 0x1a958: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a95c: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a960: ldr r1, [pc, #0x4b4]
    c.literal(R::R1, 0x1ae1c)?;
    // 0x1a964: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a968: ldr r1, [pc, #0x4bc]
    c.literal(R::R1, 0x1ae2c)?;
    // 0x1a96c: ldr r2, [pc, #0x4bc]
    c.literal(R::R2, 0x1ae30)?;
    // 0x1a970: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 0)?)?;
    // 0x1a974: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1a978: mov r3, #192
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xc0)?)?;
    // 0x1a97c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1a980: ldr r1, [pc, #0x480]
    c.literal(R::R1, 0x1ae08)?;
    // 0x1a984: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1a988: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1a98c: ldr r2, [pc, #0x4a0]
    c.literal(R::R2, 0x1ae34)?;
    // 0x1a990: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1a994: mov r1, #193
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xc1)?)?;
    // 0x1a998: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1a99c: b 0x1aa44 <entry+0x858>
    c.b(C::Al, 0x1aa44)?;
    // 0x1a9a0: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a9a4: ldr r1, [pc, #0x460]
    c.literal(R::R1, 0x1ae0c)?;
    // 0x1a9a8: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a9ac: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a9b0: ldr r1, [pc, #0x458]
    c.literal(R::R1, 0x1ae10)?;
    // 0x1a9b4: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a9b8: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a9bc: ldr r1, [pc, #0x450]
    c.literal(R::R1, 0x1ae14)?;
    // 0x1a9c0: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a9c4: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a9c8: ldr r1, [pc, #0x448]
    c.literal(R::R1, 0x1ae18)?;
    // 0x1a9cc: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a9d0: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a9d4: ldr r1, [pc, #0x448]
    c.literal(R::R1, 0x1ae24)?;
    // 0x1a9d8: b 0x1a9e4 <entry+0x7f8>
    c.b(C::Al, 0x1a9e4)?;
    // 0x1a9dc: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x1a9e0: ldr r1, [pc, #0x41c]
    c.literal(R::R1, 0x1ae04)?;
    // 0x1a9e4: add r5, r4, #2656
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::new(4)?, 0xa60)?)?;
    // 0x1a9e8: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a9ec: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a9f0: str r0, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2668)?)?;
    // 0x1a9f4: mov r0, #696
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2b8)?)?;
    // 0x1a9f8: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a9fc: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1aa00: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1aa04: movne r0, #2
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1aa08: orrne r0, r0, #8192
    c.emit(arm32::data_immediate(C::Ne, Op::Orr, R::R0, R::R0, 0x2000)?)?;
    // 0x1aa0c: strne r0, [r4, #0xa6c]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 2668)?)?;
    // 0x1aa10: ldr r2, [r4, #0xa68]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2664)?)?;
    // 0x1aa14: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1aa18: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1aa1c: ldr r1, [pc, #0x3e4]
    c.literal(R::R1, 0x1ae08)?;
    // 0x1aa20: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1aa24: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1aa28: ldr r1, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 148)?)?;
    // 0x1aa2c: ldr r0, [r6, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 128)?)?;
    // 0x1aa30: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1aa34: ldr r1, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 20)?)?;
    // 0x1aa38: ldr r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 4)?)?;
    // 0x1aa3c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1aa40: bl 0x2a198 <entry+0xffac>
    c.bl(0x2a198)?;
    // 0x1aa44: ldr r0, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 148)?)?;
    // 0x1aa48: ldr r1, [pc, #0x3e8]
    c.literal(R::R1, 0x1ae38)?;
    // 0x1aa4c: ldr r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R0, 20)?)?;
    // 0x1aa50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1aa54: ldr r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 12)?)?;
    // 0x1aa58: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1aa5c: ldr r1, [pc, #0x3a4]
    c.literal(R::R1, 0x1ae08)?;
    // 0x1aa60: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1aa64: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1aa68: ldr r1, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 148)?)?;
    // 0x1aa6c: ldr r0, [r6, #0xb4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 180)?)?;
    // 0x1aa70: ldr r1, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 20)?)?;
    // 0x1aa74: bl 0x7fd1c <entry+0x65b30>
    c.bl(0x7fd1c)?;
    // 0x1aa78: ldr r0, [r6, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 148)?)?;
    // 0x1aa7c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1aa80: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1aa84: bl 0x26754 <entry+0xc568>
    c.bl(0x26754)?;
    // 0x1aa88: mov r1, #628
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x274)?)?;
    // 0x1aa8c: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1aa90: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1aa94: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x1aa98: str r5, [r0, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, false, R::new(5)?, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1aa9c: mov r1, #588
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x24c)?)?;
    // 0x1aaa0: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -36)?)?;
    // 0x1aaa4: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x1aaa8: ldr r0, [r4, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(4)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x1aaac: str r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -52)?)?;
    // 0x1aab0: ldr r2, [pc, #0x384]
    c.literal(R::R2, 0x1ae3c)?;
    // 0x1aab4: ldr r0, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1aab8: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -44)?)?;
    // 0x1aabc: ldr r0, [pc, #0x37c]
    c.literal(R::R0, 0x1ae40)?;
    // 0x1aac0: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -32)?)?;
    // 0x1aac4: ldr r3, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1aac8: mov r0, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38c)?)?;
    // 0x1aacc: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1800)?)?;
    // 0x1aad0: ldr r3, [r6, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::new(6)?, R::new(3)?, Shift::Lsl, 2)?)?;
    // 0x1aad4: ldr r0, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(3)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1aad8: str r0, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -48)?)?;
    // 0x1aadc: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 44)?)?;
    // 0x1aae0: str r0, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -60)?)?;
    // 0x1aae4: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 48)?)?;
    // 0x1aae8: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -56)?)?;
    // 0x1aaec: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 52)?)?;
    // 0x1aaf0: str r0, [r11, #-0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -80)?)?;
    // 0x1aaf4: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 56)?)?;
    // 0x1aaf8: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 84)?)?;
    // 0x1aafc: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 68)?)?;
    // 0x1ab00: ldr r7, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(4)?, 60)?)?;
    // 0x1ab04: ldr r8, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(4)?, 64)?)?;
    // 0x1ab08: str r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -64)?)?;
    // 0x1ab0c: ldr r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 72)?)?;
    // 0x1ab10: str r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -68)?)?;
    // 0x1ab14: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 0x1ab18: str r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -72)?)?;
    // 0x1ab1c: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 80)?)?;
    // 0x1ab20: str r0, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R11, -76)?)?;
    // 0x1ab24: ldmib r4, {r10, r12}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(4)?, R::new(10)?.bit() | R::R12.bit(), BlockMode::IncrementBefore, false)?)?;
    // 0x1ab28: ldr r3, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 12)?)?;
    // 0x1ab2c: ldr r9, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::new(4)?, 16)?)?;
    // 0x1ab30: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 20)?)?;
    // 0x1ab34: str r6, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R11, -40)?)?;
    // 0x1ab38: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 32)?)?;
    // 0x1ab3c: ldr r1, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 36)?)?;
    // 0x1ab40: ldr r6, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(4)?, 24)?)?;
    // 0x1ab44: ldr lr, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::new(4)?, 28)?)?;
    // 0x1ab48: str r7, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 64)?)?;
    // 0x1ab4c: add r7, sp, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::SP, 0x48)?)?;
    // 0x1ab50: str r8, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::SP, 68)?)?;
    // 0x1ab54: stm r7, {r0, r1, r5}
    c.emit(arm32::transfer_multiple(C::Al, false, R::new(7)?, R::R0.bit() | R::R1.bit() | R::new(5)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1ab58: str r12, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::SP, 32)?)?;
    // 0x1ab5c: str r3, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::SP, 36)?)?;
    // 0x1ab60: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 40)?)?;
    // 0x1ab64: add r1, sp, #48
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x30)?)?;
    // 0x1ab68: ldr r0, [r11, #-0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -80)?)?;
    // 0x1ab6c: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 0x1ab70: ldr r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 84)?)?;
    // 0x1ab74: stm r1, {r0, r9, lr}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R0.bit() | R::new(9)?.bit() | R::LR.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1ab78: str r6, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 60)?)?;
    // 0x1ab7c: ldr r1, [pc, #0x2c0]
    c.literal(R::R1, 0x1ae44)?;
    // 0x1ab80: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -52)?)?;
    // 0x1ab84: stm sp, {r0, r10}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::new(10)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1ab88: ldr r0, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -60)?)?;
    // 0x1ab8c: ldr r2, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R11, -44)?)?;
    // 0x1ab90: ldr r3, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R11, -48)?)?;
    // 0x1ab94: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 8)?)?;
    // 0x1ab98: ldr r0, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -64)?)?;
    // 0x1ab9c: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 12)?)?;
    // 0x1aba0: ldr r0, [r11, #-0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -68)?)?;
    // 0x1aba4: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 16)?)?;
    // 0x1aba8: ldr r0, [r11, #-0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -72)?)?;
    // 0x1abac: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 20)?)?;
    // 0x1abb0: ldr r0, [r11, #-0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -76)?)?;
    // 0x1abb4: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 24)?)?;
    // 0x1abb8: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -56)?)?;
    // 0x1abbc: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 28)?)?;
    // 0x1abc0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x20)?)?;
    // 0x1abc4: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1abc8: ldr r5, [pc, #0x238]
    c.literal(R::new(5)?, 0x1ae08)?;
    // 0x1abcc: mov r0, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x20)?)?;
    // 0x1abd0: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1abd4: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1abd8: ldr r2, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 0)?)?;
    // 0x1abdc: ldr r1, [pc, #0x264]
    c.literal(R::R1, 0x1ae48)?;
    // 0x1abe0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1abe4: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1abe8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1abec: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1abf0: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1abf4: ldr r2, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 4)?)?;
    // 0x1abf8: ldr r1, [pc, #0x24c]
    c.literal(R::R1, 0x1ae4c)?;
    // 0x1abfc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac00: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac04: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac08: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ac0c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac10: ldr r2, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 44)?)?;
    // 0x1ac14: ldr r1, [pc, #0x234]
    c.literal(R::R1, 0x1ae50)?;
    // 0x1ac18: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac1c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac20: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac24: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ac28: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac2c: ldr r2, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 48)?)?;
    // 0x1ac30: ldr r1, [pc, #0x21c]
    c.literal(R::R1, 0x1ae54)?;
    // 0x1ac34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac38: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac3c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac40: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ac44: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac48: ldr r2, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 8)?)?;
    // 0x1ac4c: ldr r1, [pc, #0x204]
    c.literal(R::R1, 0x1ae58)?;
    // 0x1ac50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac54: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac58: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac5c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ac60: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac64: ldr r2, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 12)?)?;
    // 0x1ac68: ldr r1, [pc, #0x1ec]
    c.literal(R::R1, 0x1ae5c)?;
    // 0x1ac6c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac70: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac74: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac78: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ac7c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac80: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 20)?)?;
    // 0x1ac84: ldr r1, [pc, #0x1d4]
    c.literal(R::R1, 0x1ae60)?;
    // 0x1ac88: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac8c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac90: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ac94: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ac98: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ac9c: ldr r6, [pc, #0x1c0]
    c.literal(R::new(6)?, 0x1ae64)?;
    // 0x1aca0: ldr r2, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 52)?)?;
    // 0x1aca4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1aca8: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1acac: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1acb0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1acb4: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1acb8: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1acbc: ldr r7, [pc, #0x1a4]
    c.literal(R::new(7)?, 0x1ae68)?;
    // 0x1acc0: ldr r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 56)?)?;
    // 0x1acc4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1acc8: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1accc: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1acd0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1acd4: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1acd8: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1acdc: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 16)?)?;
    // 0x1ace0: ldr r1, [pc, #0x184]
    c.literal(R::R1, 0x1ae6c)?;
    // 0x1ace4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ace8: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1acec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1acf0: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1acf4: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1acf8: ldr r2, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 28)?)?;
    // 0x1acfc: ldr r1, [pc, #0x16c]
    c.literal(R::R1, 0x1ae70)?;
    // 0x1ad00: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad04: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad08: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad0c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ad10: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad14: ldr r2, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 24)?)?;
    // 0x1ad18: ldr r1, [pc, #0x154]
    c.literal(R::R1, 0x1ae74)?;
    // 0x1ad1c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad20: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad24: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad28: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ad2c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad30: ldr r2, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 60)?)?;
    // 0x1ad34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad38: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1ad3c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad40: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad44: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ad48: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad4c: ldr r2, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 64)?)?;
    // 0x1ad50: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad54: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1ad58: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad5c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad60: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ad64: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad68: ldr r2, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 32)?)?;
    // 0x1ad6c: ldr r1, [pc, #0x104]
    c.literal(R::R1, 0x1ae78)?;
    // 0x1ad70: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad74: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad78: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad7c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ad80: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad84: ldr r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 36)?)?;
    // 0x1ad88: ldr r1, [pc, #0xec]
    c.literal(R::R1, 0x1ae7c)?;
    // 0x1ad8c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad90: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ad94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ad98: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1ad9c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1ada0: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -36)?)?;
    // 0x1ada4: ldr r1, [pc, #0xd4]
    c.literal(R::R1, 0x1ae80)?;
    // 0x1ada8: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R0, 0)?)?;
    // 0x1adac: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1adb0: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1adb4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1adb8: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1adbc: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1adc0: ldr r2, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R11, -40)?)?;
    // 0x1adc4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1adc8: ldr r0, [r2, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R2, 148)?)?;
    // 0x1adcc: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x1add0: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1add4: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1add8: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, -32)?)?;
    // 0x1addc: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1ade0: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1ade4: ldr r0, [r2, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R2, R::R0, Shift::Lsl, 2)?)?;
    // 0x1ade8: ldr r1, [r0, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1adec: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1)?)?;
    // 0x1adf0: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1adf4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1adf8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1adfc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for addr in RESOURCE_DIAGNOSTICS { c.emit(addr)?; }
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_dispatch_prologue(c)?;
    emit_dispatch_cases(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn dispatch_literals_are_kept_out_of_instruction_stream(){
    let mut c=Code::new(START);extend(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    for (i,v) in MODE_DISPATCH_TARGETS.iter().enumerate(){
       let offset=0x1a8e8-START+i*4;
       assert_eq!(&c.bytes[offset..offset+4],&v.to_le_bytes());
       assert!((0x1a8a8..0x1ae00).contains(&(*v as usize)));
    }
    for (i,v) in RESOURCE_DIAGNOSTICS.iter().enumerate(){
       let offset=0x1ae00-START+i*4;
       assert_eq!(&c.bytes[offset..offset+4],&v.to_le_bytes());
    }
 }
}
