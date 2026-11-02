// SPDX-License-Identifier: GPL-3.0-only
//! Codec controller status publication and 20-class mode dispatcher.
//! The controller mode handlers record initialization flags, select device
//! session callbacks and emit per-class diagnostics. ARM encodings are typed;
//! all control-flow tables and diagnostic pools are modeled as data words.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x1b304;
const END:usize=0x1ba40;
const TRACE_PREPARE: [u32;3] = [
    0xe0008d50, 0x000e4e55, 0x000e2788,
];
const TRACE_COMPLETE: [u32;3] = [
    0xe0008d50, 0x000e4e87, 0x000e2788,
];
const PARAMETER_MESSAGES: [u32;5] = [
    0xe0008d54, 0x000e4ebc, 0x000e2788, 0x000e4ee9, 0x000e4f1e,
];
const CLASS_JUMP_TARGETS: [u32;20] = [
    0x0001b6a0, 0x0001b6bc, 0x0001b6f0, 0x0001b634, 0x0001b6f0,
    0x0001b848, 0x0001b728, 0x0001b6f0, 0x0001b6f0, 0x0001b6f0,
    0x0001b75c, 0x0001b6f0, 0x0001b7a8, 0x0001b634, 0x0001b6f0,
    0x0001b634, 0x0001b6f0, 0x0001b6f0, 0x0001b6f0, 0x0001b81c,
];
const CLASS_DIAGNOSTICS: [u32;28] = [
    0x000e4f53, 0x000e2788, 0x000e4f83, 0x000e4fae, 0x000e4fcd,
    0x000e4feb, 0x000e5010, 0x00103e40, 0x000e5034, 0x000e5062,
    0xfbfeeffd, 0x000e5149, 0x000e5129, 0x000e50fa, 0x000e50de,
    0x000e50b8, 0x000e50a1, 0x000e508d, 0x000e516f, 0x000e518c,
    0x000e4dd7, 0x000e51a7, 0x000e51ce, 0x000e51f4, 0x000e5215,
    0x000e5235, 0x000e5262, 0x0401100f,
];

/// Report entry to codec mode state zero with its event arguments.
fn emit_mode_started(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1b304: push {r11, lr}
    c.emit(arm32::push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    // 0x1b308: mov r11, sp
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    // 0x1b30c: ldr r0, [pc, #0x20]
    c.literal(R::R0, 0x1b334)?;
    // 0x1b310: ldr r1, [pc, #0x20]
    c.literal(R::R1, 0x1b338)?;
    // 0x1b314: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R0, 0)?)?;
    // 0x1b318: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1b31c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b320: ldr r1, [pc, #0x14]
    c.literal(R::R1, 0x1b33c)?;
    // 0x1b324: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1b328: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b32c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b330: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for v in TRACE_PREPARE{c.emit(v)?;}
    c.check(0x1b340)
}

/// Report transition to codec command mode two.
fn emit_mode_completed(c:&mut Code)->Result<(),String>{
    c.check(0x1b340)?;
    // 0x1b340: push {r11, lr}
    c.emit(arm32::push_registers(C::Al, R::R11.bit() | R::LR.bit())?)?;
    // 0x1b344: mov r11, sp
    c.emit(arm32::mov_register(C::Al, R::R11, R::SP))?;
    // 0x1b348: ldr r0, [pc, #0x20]
    c.literal(R::R0, 0x1b370)?;
    // 0x1b34c: ldr r1, [pc, #0x20]
    c.literal(R::R1, 0x1b374)?;
    // 0x1b350: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R0, 0)?)?;
    // 0x1b354: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b358: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b35c: ldr r1, [pc, #0x14]
    c.literal(R::R1, 0x1b378)?;
    // 0x1b360: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b364: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b368: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b36c: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for v in TRACE_COMPLETE{c.emit(v)?;}
    c.check(0x1b37c)
}

/// Load platform session settings and publish status/feature flags.
fn emit_session_feature_data(c:&mut Code)->Result<(),String>{
    c.check(0x1b37c)?;
    // 0x1b37c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1b380: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x1b384: ldr r6, [pc, #0x94]
    c.literal(R::new(6)?, 0x1b420)?;
    // 0x1b388: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x1b38c: ldr r1, [pc, #0x90]
    c.literal(R::R1, 0x1b424)?;
    // 0x1b390: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 0)?)?;
    // 0x1b394: str r2, [r4, #0xa90]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2704)?)?;
    // 0x1b398: ldr r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 4)?)?;
    // 0x1b39c: str r0, [r4, #0xa94]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2708)?)?;
    // 0x1b3a0: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1b3a4: str r0, [r4, #0xa98]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2712)?)?;
    // 0x1b3a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b3ac: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b3b0: ldr r5, [pc, #0x70]
    c.literal(R::new(5)?, 0x1b428)?;
    // 0x1b3b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b3b8: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1b3bc: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b3c0: ldr r2, [r4, #0xa94]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2708)?)?;
    // 0x1b3c4: ldr r1, [pc, #0x60]
    c.literal(R::R1, 0x1b42c)?;
    // 0x1b3c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b3cc: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b3d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b3d4: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1b3d8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b3dc: ldr r2, [r4, #0xa98]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2712)?)?;
    // 0x1b3e0: ldr r1, [pc, #0x48]
    c.literal(R::R1, 0x1b430)?;
    // 0x1b3e4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b3e8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b3ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b3f0: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1b3f4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b3f8: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 12)?)?;
    // 0x1b3fc: str r0, [r4, #0xaa0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2720)?)?;
    // 0x1b400: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 16)?)?;
    // 0x1b404: str r0, [r4, #0xaa4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2724)?)?;
    // 0x1b408: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 20)?)?;
    // 0x1b40c: str r0, [r4, #0xaa8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2728)?)?;
    // 0x1b410: ldr r0, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 24)?)?;
    // 0x1b414: str r0, [r4, #0xaac]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2732)?)?;
    // 0x1b418: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b41c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for v in PARAMETER_MESSAGES{c.emit(v)?;}
    c.check(0x1b434)
}

/// Mode dispatcher prologue selects one of 20 controller format cases.
fn emit_class_dispatch_prefix(c:&mut Code)->Result<(),String>{
    c.check(0x1b434)?;
    // 0x1b434: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1b438: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x18)?)?;
    // 0x1b43c: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x1b440: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1b444: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b448: ldr r5, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R0, 28)?)?;
    // 0x1b44c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x0)?)?;
    // 0x1b450: beq 0x1b674 <entry+0x374>
    c.b(C::Eq, 0x1b674)?;
    // 0x1b454: ldr r8, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(5)?, 16)?)?;
    // 0x1b458: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(8)?, 0x2)?)?;
    // 0x1b45c: ldreq r0, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Eq, true, R::R0, R::new(5)?, 728)?)?;
    // 0x1b460: cmpeq r0, #2
    c.emit(arm32::data_immediate(C::Eq, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 0x1b464: bne 0x1b4ac <entry+0x1ac>
    c.b(C::Ne, 0x1b4ac)?;
    // 0x1b468: ldr r1, [pc, #0x560]
    c.literal(R::R1, 0x1b9d0)?;
    // 0x1b46c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b470: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b474: ldr r1, [pc, #0x558]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b478: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b47c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b480: bl 0x25984 <entry+0xa684>
    c.bl(0x25984)?;
    // 0x1b484: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b488: ldr r1, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 28)?)?;
    // 0x1b48c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x1b490: beq 0x1b690 <entry+0x390>
    c.b(C::Eq, 0x1b690)?;
    // 0x1b494: ldr r1, [pc, #0x53c]
    c.literal(R::R1, 0x1b9d8)?;
    // 0x1b498: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b49c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b4a0: ldr r1, [pc, #0x52c]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b4a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b4a8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b4ac: ldr r1, [pc, #0x528]
    c.literal(R::R1, 0x1b9dc)?;
    // 0x1b4b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b4b4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b4b8: ldr r1, [pc, #0x514]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b4bc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b4c0: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b4c4: bl 0x25954 <entry+0xa654>
    c.bl(0x25954)?;
    // 0x1b4c8: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b4cc: ldr r1, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 28)?)?;
    // 0x1b4d0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x1b4d4: beq 0x1b680 <entry+0x380>
    c.b(C::Eq, 0x1b680)?;
    // 0x1b4d8: ldr r1, [pc, #0x500]
    c.literal(R::R1, 0x1b9e0)?;
    // 0x1b4dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b4e0: mov r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x1)?)?;
    // 0x1b4e4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b4e8: ldr r1, [pc, #0x4e4]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b4ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b4f0: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b4f4: cmp r8, #5
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(8)?, 0x5)?)?;
    // 0x1b4f8: bhi 0x1b508 <entry+0x208>
    c.b(C::Hi, 0x1b508)?;
    // 0x1b4fc: mov r0, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x31)?)?;
    // 0x1b500: tst r0, r6, lsl r8
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Tst, R::R0, R::R0, R::new(6)?, Shift::Lsl, R::new(8)?, false)?)?;
    // 0x1b504: bne 0x1b540 <entry+0x240>
    c.b(C::Ne, 0x1b540)?;
    // 0x1b508: ldr r1, [pc, #0x4d4]
    c.literal(R::R1, 0x1b9e4)?;
    // 0x1b50c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b510: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b514: ldr r6, [pc, #0x4b8]
    c.literal(R::new(6)?, 0x1b9d4)?;
    // 0x1b518: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b51c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b520: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b524: bl 0x259b4 <entry+0xa6b4>
    c.bl(0x259b4)?;
    // 0x1b528: ldr r1, [pc, #0x4b8]
    c.literal(R::R1, 0x1b9e8)?;
    // 0x1b52c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b530: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b534: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b538: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b53c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b540: ldr r1, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 728)?)?;
    // 0x1b544: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x1b548: ldr r7, [pc, #0x49c]
    c.literal(R::new(7)?, 0x1b9ec)?;
    // 0x1b54c: ldr r2, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 12)?)?;
    // 0x1b550: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 0)?)?;
    // 0x1b554: ldr r1, [pc, #0x494]
    c.literal(R::R1, 0x1b9f0)?;
    // 0x1b558: add r3, r7, r0, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::new(7)?, R::R0, Shift::Lsl, 4, false)?)?;
    // 0x1b55c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b560: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b564: ldr r6, [pc, #0x468]
    c.literal(R::new(6)?, 0x1b9d4)?;
    // 0x1b568: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b56c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b570: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b574: bl 0x26764 <entry+0xb464>
    c.bl(0x26764)?;
    // 0x1b578: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1b57c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1b580: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1b584: bl 0x22fac <entry+0x7cac>
    c.bl(0x22fac)?;
    // 0x1b588: ldr r1, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 728)?)?;
    // 0x1b58c: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x1b590: ldr r2, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 12)?)?;
    // 0x1b594: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 0)?)?;
    // 0x1b598: ldr r1, [pc, #0x454]
    c.literal(R::R1, 0x1b9f4)?;
    // 0x1b59c: add r3, r7, r0, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::new(7)?, R::R0, Shift::Lsl, 4, false)?)?;
    // 0x1b5a0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b5a4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b5a8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b5ac: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b5b0: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b5b4: bl 0x26754 <entry+0xb454>
    c.bl(0x26754)?;
    // 0x1b5b8: mov r1, #644
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x284)?)?;
    // 0x1b5bc: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x1b5c0: str r0, [r5, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(5)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b5c4: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 8)?)?;
    // 0x1b5c8: ldr r1, [pc, #0x428]
    c.literal(R::R1, 0x1b9f8)?;
    // 0x1b5cc: add r0, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1b5d0: cmp r0, #19
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x13)?)?;
    // 0x1b5d4: bhi 0x1b6f0 <entry+0x3f0>
    c.b(C::Hi, 0x1b6f0)?;
    // 0x1b5d8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b5dc: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x0)?)?;
    // 0x1b5e0: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;
    for v in CLASS_JUMP_TARGETS{c.emit(v)?;}
    c.check(0x1b634)
}

/// Execute the mode-specific controller paths and publish diagnostics.
fn emit_class_dispatch_cases(c:&mut Code)->Result<(),String>{
    c.check(0x1b634)?;
    // 0x1b634: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b638: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b63c: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b640: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b644: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b648: ldr r1, [pc, #0x3bc]
    c.literal(R::R1, 0x1ba0c)?;
    // 0x1b64c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b650: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b654: ldr r1, [pc, #0x378]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b658: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b65c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b660: bl 0x1b340 <entry+0x40>
    c.bl(0x1b340)?;
    // 0x1b664: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b668: str r0, [r5, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 2668)?)?;
    // 0x1b66c: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b670: b 0x1b858 <entry+0x558>
    c.b(C::Al, 0x1b858)?;
    // 0x1b674: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b678: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 24)?)?;
    // 0x1b67c: b 0x1b9c4 <entry+0x6c4>
    c.b(C::Al, 0x1b9c4)?;
    // 0x1b680: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b684: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 24)?)?;
    // 0x1b688: bl 0x25974 <entry+0xa674>
    c.bl(0x25974)?;
    // 0x1b68c: b 0x1b9c4 <entry+0x6c4>
    c.b(C::Al, 0x1b9c4)?;
    // 0x1b690: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b694: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 24)?)?;
    // 0x1b698: bl 0x259a4 <entry+0xa6a4>
    c.bl(0x259a4)?;
    // 0x1b69c: b 0x1b9c4 <entry+0x6c4>
    c.b(C::Al, 0x1b9c4)?;
    // 0x1b6a0: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b6a4: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b6a8: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b6ac: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b6b0: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b6b4: ldr r1, [pc, #0x358]
    c.literal(R::R1, 0x1ba14)?;
    // 0x1b6b8: b 0x1b834 <entry+0x534>
    c.b(C::Al, 0x1b834)?;
    // 0x1b6bc: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b6c0: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b6c4: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b6c8: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b6cc: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b6d0: ldr r1, [pc, #0x330]
    c.literal(R::R1, 0x1ba08)?;
    // 0x1b6d4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b6d8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b6dc: ldr r1, [pc, #0x2f0]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b6e0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b6e4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b6e8: bl 0x1b304 <entry+0x4>
    c.bl(0x1b304)?;
    // 0x1b6ec: b 0x1b848 <entry+0x548>
    c.b(C::Al, 0x1b848)?;
    // 0x1b6f0: ldr r1, [pc, #0x320]
    c.literal(R::R1, 0x1ba18)?;
    // 0x1b6f4: ldr r2, [pc, #0x320]
    c.literal(R::R2, 0x1ba1c)?;
    // 0x1b6f8: mov r3, #59
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x3b)?)?;
    // 0x1b6fc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b700: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x100)?)?;
    // 0x1b704: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b708: ldr r1, [pc, #0x2c4]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b70c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b710: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b714: ldr r2, [pc, #0x304]
    c.literal(R::R2, 0x1ba20)?;
    // 0x1b718: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b71c: mov r1, #316
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x13c)?)?;
    // 0x1b720: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1b724: b 0x1b860 <entry+0x560>
    c.b(C::Al, 0x1b860)?;
    // 0x1b728: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b72c: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b730: mov r6, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x2)?)?;
    // 0x1b734: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b738: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b73c: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b740: ldr r1, [pc, #0x2b4]
    c.literal(R::R1, 0x1b9fc)?;
    // 0x1b744: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b748: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b74c: ldr r1, [pc, #0x280]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b750: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b754: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b758: b 0x1b79c <entry+0x49c>
    c.b(C::Al, 0x1b79c)?;
    // 0x1b75c: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b760: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b764: ldr r2, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 728)?)?;
    // 0x1b768: mov r6, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x2)?)?;
    // 0x1b76c: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b770: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b774: ldr r3, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b778: ldr r1, [pc, #0x284]
    c.literal(R::R1, 0x1ba04)?;
    // 0x1b77c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b780: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b784: ldr r1, [pc, #0x248]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b788: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b78c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b790: ldr r0, [r5, #0x900]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 2304)?)?;
    // 0x1b794: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1b798: moveq r6, #3
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(6)?, R::R0, 0x3)?)?;
    // 0x1b79c: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b7a0: str r6, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 24)?)?;
    // 0x1b7a4: b 0x1b860 <entry+0x560>
    c.b(C::Al, 0x1b860)?;
    // 0x1b7a8: mov r0, #684
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2ac)?)?;
    // 0x1b7ac: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1b7b0: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1b7b4: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1b7b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1b7bc: bne 0x1b7d4 <entry+0x4d4>
    c.b(C::Ne, 0x1b7d4)?;
    // 0x1b7c0: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b7c4: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b7c8: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b7cc: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b7d0: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b7d4: ldr r1, [pc, #0x234]
    c.literal(R::R1, 0x1ba10)?;
    // 0x1b7d8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b7dc: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b7e0: ldr r1, [pc, #0x1ec]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b7e4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b7e8: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b7ec: mov r0, #696
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2b8)?)?;
    // 0x1b7f0: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1b7f4: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1b7f8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1b7fc: movne r1, r5
    c.emit(arm32::mov_register(C::Ne, R::R1, R::new(5)?))?;
    // 0x1b800: blne 0x1b37c <entry+0x7c>
    c.emit(arm32::branch(C::Ne,c.at() as u32,0x1b37c,true)?)?;
    // 0x1b804: mov r0, #680
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2a8)?)?;
    // 0x1b808: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1b80c: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1b810: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1b814: bne 0x1b860 <entry+0x560>
    c.b(C::Ne, 0x1b860)?;
    // 0x1b818: b 0x1b848 <entry+0x548>
    c.b(C::Al, 0x1b848)?;
    // 0x1b81c: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b820: mov r1, #908
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38c)?)?;
    // 0x1b824: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1b828: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b82c: ldr r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1b830: ldr r1, [pc, #0x1c8]
    c.literal(R::R1, 0x1ba00)?;
    // 0x1b834: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b838: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b83c: ldr r1, [pc, #0x190]
    c.literal(R::R1, 0x1b9d4)?;
    // 0x1b840: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1b844: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b848: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b84c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1b850: ldr r1, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 28)?)?;
    // 0x1b854: str r2, [r1, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 2668)?)?;
    // 0x1b858: mov r1, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3)?)?;
    // 0x1b85c: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 24)?)?;
    // 0x1b860: bl 0x2693c <entry+0xb63c>
    c.bl(0x2693c)?;
    // 0x1b864: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b868: ldr r0, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 24)?)?;
    // 0x1b86c: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x3)?)?;
    // 0x1b870: bne 0x1b8b4 <entry+0x5b4>
    c.b(C::Ne, 0x1b8b4)?;
    // 0x1b874: mov r0, #520
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x208)?)?;
    // 0x1b878: mov r2, #904
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x388)?)?;
    // 0x1b87c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1b880: orr r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1800)?)?;
    // 0x1b884: ldr r1, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1b888: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0x1b88c: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x1b890: ldr r0, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R2, Shift::Lsl, 0)?)?;
    // 0x1b894: bl 0x7fd1c <entry+0x64a1c>
    c.bl(0x7fd1c)?;
    // 0x1b898: ldr r2, [pc, #0x180]
    c.literal(R::R2, 0x1ba20)?;
    // 0x1b89c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b8a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1b8a4: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1b8a8: mov r0, r1
    c.emit(arm32::mov_register(C::Al, R::R0, R::R1))?;
    // 0x1b8ac: mov r1, #332
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x14c)?)?;
    // 0x1b8b0: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1b8b4: cmp r8, #5
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(8)?, 0x5)?)?;
    // 0x1b8b8: bhi 0x1b8cc <entry+0x5cc>
    c.b(C::Hi, 0x1b8cc)?;
    // 0x1b8bc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b8c0: mov r1, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x31)?)?;
    // 0x1b8c4: tst r1, r0, lsl r8
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Tst, R::R0, R::R1, R::R0, Shift::Lsl, R::new(8)?, false)?)?;
    // 0x1b8c8: bne 0x1b904 <entry+0x604>
    c.b(C::Ne, 0x1b904)?;
    // 0x1b8cc: ldr r1, [pc, #0x150]
    c.literal(R::R1, 0x1ba24)?;
    // 0x1b8d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b8d4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b8d8: ldr r6, [pc, #0xf4]
    c.literal(R::new(6)?, 0x1b9d4)?;
    // 0x1b8dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b8e0: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b8e4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b8e8: bl 0x259c4 <entry+0xa6c4>
    c.bl(0x259c4)?;
    // 0x1b8ec: ldr r1, [pc, #0x134]
    c.literal(R::R1, 0x1ba28)?;
    // 0x1b8f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b8f4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b8f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b8fc: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b900: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b904: ldr r1, [pc, #0x120]
    c.literal(R::R1, 0x1ba2c)?;
    // 0x1b908: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b90c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b910: ldr r6, [pc, #0xbc]
    c.literal(R::new(6)?, 0x1b9d4)?;
    // 0x1b914: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b918: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b91c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b920: bl 0x25974 <entry+0xa674>
    c.bl(0x25974)?;
    // 0x1b924: ldr r1, [pc, #0x104]
    c.literal(R::R1, 0x1ba30)?;
    // 0x1b928: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b92c: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b930: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b934: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b938: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b93c: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x1b940: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 0x1b944: ldreq r0, [r5, #0x2d8]
    c.emit(arm32::transfer_word(C::Eq, true, R::R0, R::new(5)?, 728)?)?;
    // 0x1b948: cmpeq r0, #2
    c.emit(arm32::data_immediate(C::Eq, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 0x1b94c: bne 0x1b988 <entry+0x688>
    c.b(C::Ne, 0x1b988)?;
    // 0x1b950: ldr r1, [pc, #0xdc]
    c.literal(R::R1, 0x1ba34)?;
    // 0x1b954: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b958: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b95c: ldr r6, [pc, #0x70]
    c.literal(R::new(6)?, 0x1b9d4)?;
    // 0x1b960: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b964: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b968: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b96c: bl 0x259a4 <entry+0xa6a4>
    c.bl(0x259a4)?;
    // 0x1b970: ldr r1, [pc, #0xc0]
    c.literal(R::R1, 0x1ba38)?;
    // 0x1b974: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b978: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b97c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b980: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1b984: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1b988: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 8)?)?;
    // 0x1b98c: ldr r1, [pc, #0xa8]
    c.literal(R::R1, 0x1ba3c)?;
    // 0x1b990: cmp r0, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1b994: bne 0x1b9c4 <entry+0x6c4>
    c.b(C::Ne, 0x1b9c4)?;
    // 0x1b998: mov r0, #680
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2a8)?)?;
    // 0x1b99c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1b9a0: ldr r1, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1b9a4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x1b9a8: beq 0x1b9c4 <entry+0x6c4>
    c.b(C::Eq, 0x1b9c4)?;
    // 0x1b9ac: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1b9b0: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // 0x1b9b4: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b9b8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b9bc: str r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 24)?)?;
    // 0x1b9c0: str r1, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 28)?)?;
    // 0x1b9c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b9c8: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x18)?)?;
    // 0x1b9cc: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for v in CLASS_DIAGNOSTICS{c.emit(v)?;}
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_mode_started(c)?;
    emit_mode_completed(c)?;
    emit_session_feature_data(c)?;
    emit_class_dispatch_prefix(c)?;
    emit_class_dispatch_cases(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn controller_mode_and_literal_tables_are_complete(){
    let mut c=Code::new(START);extend(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    for (a,vals) in [(0x1b334,&TRACE_PREPARE[..]),(0x1b370,&TRACE_COMPLETE[..]),
      (0x1b420,&PARAMETER_MESSAGES[..]),(0x1b5e4,&CLASS_JUMP_TARGETS[..]),
      (0x1b9d0,&CLASS_DIAGNOSTICS[..])] {
       for (i,v) in vals.iter().enumerate(){
         let offset=a-START+i*4;
         assert_eq!(&c.bytes[offset..offset+4],&v.to_le_bytes());
       }
    }
 }
}
