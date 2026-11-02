// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec controller resource allocator with conditional rollback,
//! linked buffer initialization, and status-word validation. The diagnostic
//! literal pool is emitted as typed data separate from A32 opcodes.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x32550;
const END:usize=0x329f8;
const RESOURCE_STATUS_LITERALS:[u32;14]=[
    0x000329f8,  // runtime 0x329c0
    0x00033488,  // runtime 0x329c4
    0x00033abc,  // runtime 0x329c8
    0x00033b60,  // runtime 0x329cc
    0x0010ceac,  // runtime 0x329d0
    0x000eb784,  // runtime 0x329d4
    0x000eb7b0,  // runtime 0x329d8
    0x000e2788,  // runtime 0x329dc
    0x000eb7df,  // runtime 0x329e0
    0x000eb80b,  // runtime 0x329e4
    0x00080760,  // runtime 0x329e8
    0x000eb845,  // runtime 0x329ec
    0x000eb87f,  // runtime 0x329f0
    0x000eb753,  // runtime 0x329f4
];

/// Establish four codec buffers, attach callbacks and initialize the
/// corresponding controller MMIO resource slots. Failures unwind exactly
/// through the original release branches without fabricated code padding.
fn allocate_codec_resources(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x32550: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x32554: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x32558: sub sp, sp, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,20)?)?;
    // 0x3255c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x32560: mov r0, #372
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,372)?)?;
    // 0x32564: mov r8, r2
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R2))?;
    // 0x32568: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x3256c: orr r0, r0, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,9216)?)?;
    // 0x32570: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f67c
    c.bl(0x2efc)?;
    // 0x32574: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x32578: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x3257c: beq 0x32974 <entry+0xbf4>   @ imm = #0x3f0
    c.b(C::Eq,0x32974)?;
    // 0x32580: mov r2, #372
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,372)?)?;
    // 0x32584: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x32588: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3258c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x32590: orr r2, r2, #9216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,9216)?)?;
    // 0x32594: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x2f3b0
    c.bl(0x31ec)?;
    // 0x32598: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3259c: str r4, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,0)?)?;
    // 0x325a0: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f6ac
    c.bl(0x2efc)?;
    // 0x325a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x325a8: str r0, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,320)?)?;
    // 0x325ac: beq 0x329ac <entry+0xc2c>   @ imm = #0x3f8
    c.b(C::Eq,0x329ac)?;
    // 0x325b0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x325b4: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f6c0
    c.bl(0x2efc)?;
    // 0x325b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x325bc: str r0, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,324)?)?;
    // 0x325c0: beq 0x329a4 <entry+0xc24>   @ imm = #0x3dc
    c.b(C::Eq,0x329a4)?;
    // 0x325c4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x325c8: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f6d4
    c.bl(0x2efc)?;
    // 0x325cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x325d0: str r0, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,312)?)?;
    // 0x325d4: beq 0x3299c <entry+0xc1c>   @ imm = #0x3c0
    c.b(C::Eq,0x3299c)?;
    // 0x325d8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x325dc: strb r1, [r0, #0x5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,5)?)?;
    // 0x325e0: strb r1, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3)?)?;
    // 0x325e4: strb r1, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2)?)?;
    // 0x325e8: strb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // 0x325ec: strb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x325f0: strb r1, [r0, #0x4]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x325f4: strb r1, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3)?)?;
    // 0x325f8: strb r1, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2)?)?;
    // 0x325fc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x32600: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x2f70c
    c.bl(0x2efc)?;
    // 0x32604: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x32608: str r0, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,316)?)?;
    // 0x3260c: beq 0x32994 <entry+0xc14>   @ imm = #0x380
    c.b(C::Eq,0x32994)?;
    // 0x32610: ldr r2, [pc, #0x3a8]        @ 0xc40 <entry+0xc40>
    c.literal(R::R2,0x329c0)?;
    // 0x32614: add r7, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,8)?)?;
    // 0x32618: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3261c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x32620: bl 0x30d0 <entry+0xfffffffffffd1350> @ imm = #-0x2f558
    c.bl(0x30d0)?;
    // 0x32624: ldr r2, [pc, #0x398]        @ 0xc44 <entry+0xc44>
    c.literal(R::R2,0x329c4)?;
    // 0x32628: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x3262c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x32630: bl 0x30d0 <entry+0xfffffffffffd1350> @ imm = #-0x2f568
    c.bl(0x30d0)?;
    // 0x32634: ldr r2, [pc, #0x38c]        @ 0xc48 <entry+0xc48>
    c.literal(R::R2,0x329c8)?;
    // 0x32638: add r0, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,16)?)?;
    // 0x3263c: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x32640: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x32644: bl 0x30d0 <entry+0xfffffffffffd1350> @ imm = #-0x2f57c
    c.bl(0x30d0)?;
    // 0x32648: ldr r2, [pc, #0x37c]        @ 0xc4c <entry+0xc4c>
    c.literal(R::R2,0x329cc)?;
    // 0x3264c: add r9, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,24)?)?;
    // 0x32650: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x32654: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x32658: bl 0x30d0 <entry+0xfffffffffffd1350> @ imm = #-0x2f590
    c.bl(0x30d0)?;
    // 0x3265c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x32660: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x32664: bl 0x26d0 <entry+0xfffffffffffd0950> @ imm = #-0x2ff9c
    c.bl(0x26d0)?;
    // 0x32668: ldr r10, [pc, #0x360]       @ 0xc50 <entry+0xc50>
    c.literal(R::new(10)?,0x329d0)?;
    // 0x3266c: ldr r3, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,320)?)?;
    // 0x32670: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x32674: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x32678: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x3267c: ldr r5, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,20)?)?;
    // 0x32680: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x32684: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,12)?)?;
    // 0x32688: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x3268c: ldr r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,16)?)?;
    // 0x32690: bl 0x2550 <entry+0xfffffffffffd07d0> @ imm = #-0x30148
    c.bl(0x2550)?;
    // 0x32694: ldr r7, [pc, #0x338]        @ 0xc54 <entry+0xc54>
    c.literal(R::new(7)?,0x329d4)?;
    // 0x32698: mov r1, #114
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,114)?)?;
    // 0x3269c: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x326a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x326a4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x326a8: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x326ac: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x326b0: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x30f08
    c.bl(0x17b0)?;
    // 0x326b4: ldr r3, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,324)?)?;
    // 0x326b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x326bc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x326c0: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x326c4: bl 0x2550 <entry+0xfffffffffffd07d0> @ imm = #-0x3017c
    c.bl(0x2550)?;
    // 0x326c8: mov r1, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,122)?)?;
    // 0x326cc: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x326d0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x326d4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x326d8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x326dc: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x326e0: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x30f38
    c.bl(0x17b0)?;
    // 0x326e4: ldr r3, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,312)?)?;
    // 0x326e8: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x326ec: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x326f0: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x326f4: bl 0x2550 <entry+0xfffffffffffd07d0> @ imm = #-0x301ac
    c.bl(0x2550)?;
    // 0x326f8: mov r1, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,130)?)?;
    // 0x326fc: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x32700: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32704: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x32708: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3270c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x32710: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x30f68
    c.bl(0x17b0)?;
    // 0x32714: ldr r3, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,316)?)?;
    // 0x32718: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x3271c: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,16)?)?;
    // 0x32720: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x32724: bl 0x2550 <entry+0xfffffffffffd07d0> @ imm = #-0x301dc
    c.bl(0x2550)?;
    // 0x32728: mov r1, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,138)?)?;
    // 0x3272c: cmn r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::R0,1)?)?;
    // 0x32730: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32734: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x32738: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3273c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x32740: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x30f98
    c.bl(0x17b0)?;
    // 0x32744: add r0, r10, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,68)?)?;
    // 0x32748: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x3274c: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x32750: str r0, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,336)?)?;
    // 0x32754: add r0, r10, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,44)?)?;
    // 0x32758: str r0, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,340)?)?;
    // 0x3275c: add r0, r10, #52
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,52)?)?;
    // 0x32760: str r0, [r4, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,344)?)?;
    // 0x32764: add r0, r10, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,28)?)?;
    // 0x32768: str r0, [r4, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,328)?)?;
    // 0x3276c: add r0, r10, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,76)?)?;
    // 0x32770: str r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,348)?)?;
    // 0x32774: ldr r0, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,316)?)?;
    // 0x32778: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x3277c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x32780: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x32784: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x32788: add r0, r4, #144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,144)?)?;
    // 0x3278c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x32790: bl 0x8080c <entry+0x4ea8c> @ imm = #0x4e074
    c.bl(0x8080c)?;
    // 0x32794: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x32798: beq 0x327c8 <entry+0xa48>   @ imm = #0x28
    c.b(C::Eq,0x327c8)?;
    // 0x3279c: ldr r1, [pc, #0x234]        @ 0xc58 <entry+0xc58>
    c.literal(R::R1,0x329d8)?;
    // 0x327a0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x327a4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3b18
    c.bl(0x2ec94)?;
    // 0x327a8: ldr r1, [pc, #0x22c]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x329dc)?;
    // 0x327ac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x327b0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3b24
    c.bl(0x2ec94)?;
    // 0x327b4: ldr r2, [pc, #0x218]        @ 0xc54 <entry+0xc54>
    c.literal(R::R2,0x329d4)?;
    // 0x327b8: mov r1, #169
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,169)?)?;
    // 0x327bc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x327c0: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x327c4: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x3101c
    c.bl(0x17b0)?;
    // 0x327c8: ldr r0, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,312)?)?;
    // 0x327cc: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x327d0: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x327d4: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x327d8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x327dc: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x327e0: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x327e4: add r0, r4, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,88)?)?;
    // 0x327e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x327ec: bl 0x8080c <entry+0x4ea8c> @ imm = #0x4e018
    c.bl(0x8080c)?;
    // 0x327f0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x327f4: beq 0x32824 <entry+0xaa4>   @ imm = #0x28
    c.b(C::Eq,0x32824)?;
    // 0x327f8: ldr r1, [pc, #0x1e0]        @ 0xc60 <entry+0xc60>
    c.literal(R::R1,0x329e0)?;
    // 0x327fc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32800: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3b74
    c.bl(0x2ec94)?;
    // 0x32804: ldr r1, [pc, #0x1d0]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x329dc)?;
    // 0x32808: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3280c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3b80
    c.bl(0x2ec94)?;
    // 0x32810: ldr r2, [pc, #0x1bc]        @ 0xc54 <entry+0xc54>
    c.literal(R::R2,0x329d4)?;
    // 0x32814: mov r1, #181
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,181)?)?;
    // 0x32818: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3281c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x32820: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x31078
    c.bl(0x17b0)?;
    // 0x32824: mov r0, #1392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1392)?)?;
    // 0x32828: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x3282c: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x32830: str r8, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(8)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x32834: beq 0x32894 <entry+0xb14>   @ imm = #0x58
    c.b(C::Eq,0x32894)?;
    // 0x32838: ldr r0, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,320)?)?;
    // 0x3283c: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x32840: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x32844: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x32848: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x3284c: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x32850: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x32854: add r0, r4, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,256)?)?;
    // 0x32858: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3285c: bl 0x8080c <entry+0x4ea8c> @ imm = #0x4dfa8
    c.bl(0x8080c)?;
    // 0x32860: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x32864: beq 0x32894 <entry+0xb14>   @ imm = #0x28
    c.b(C::Eq,0x32894)?;
    // 0x32868: ldr r1, [pc, #0x174]        @ 0xc64 <entry+0xc64>
    c.literal(R::R1,0x329e4)?;
    // 0x3286c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32870: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3be4
    c.bl(0x2ec94)?;
    // 0x32874: ldr r1, [pc, #0x160]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x329dc)?;
    // 0x32878: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3287c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3bf0
    c.bl(0x2ec94)?;
    // 0x32880: ldr r2, [pc, #0x14c]        @ 0xc54 <entry+0xc54>
    c.literal(R::R2,0x329d4)?;
    // 0x32884: mov r1, #452
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,452)?)?;
    // 0x32888: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3288c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x32890: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x310e8
    c.bl(0x17b0)?;
    // 0x32894: ldr r0, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,324)?)?;
    // 0x32898: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // 0x3289c: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x328a0: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x328a4: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x328a8: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x328ac: ldr r1, [pc, #0x134]        @ 0xc68 <entry+0xc68>
    c.literal(R::R1,0x329e8)?;
    // 0x328b0: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x328b4: add r0, r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,200)?)?;
    // 0x328b8: bl 0x8080c <entry+0x4ea8c> @ imm = #0x4df4c
    c.bl(0x8080c)?;
    // 0x328bc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x328c0: beq 0x328f0 <entry+0xb70>   @ imm = #0x28
    c.b(C::Eq,0x328f0)?;
    // 0x328c4: ldr r1, [pc, #0x120]        @ 0xc6c <entry+0xc6c>
    c.literal(R::R1,0x329ec)?;
    // 0x328c8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x328cc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3c40
    c.bl(0x2ec94)?;
    // 0x328d0: ldr r1, [pc, #0x104]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x329dc)?;
    // 0x328d4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x328d8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3c4c
    c.bl(0x2ec94)?;
    // 0x328dc: ldr r2, [pc, #0xf0]         @ 0xc54 <entry+0xc54>
    c.literal(R::R2,0x329d4)?;
    // 0x328e0: mov r1, #209
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,209)?)?;
    // 0x328e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x328e8: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x328ec: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x31144
    c.bl(0x17b0)?;
    // 0x328f0: ldr r0, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,320)?)?;
    // 0x328f4: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // 0x328f8: mov r3, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,128)?)?;
    // 0x328fc: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x32900: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x32904: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x32908: ldr r1, [pc, #0xd8]         @ 0xc68 <entry+0xc68>
    c.literal(R::R1,0x329e8)?;
    // 0x3290c: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x32910: add r0, r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,32)?)?;
    // 0x32914: bl 0x8080c <entry+0x4ea8c> @ imm = #0x4def0
    c.bl(0x8080c)?;
    // 0x32918: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3291c: beq 0x3294c <entry+0xbcc>   @ imm = #0x28
    c.b(C::Eq,0x3294c)?;
    // 0x32920: ldr r1, [pc, #0xc8]         @ 0xc70 <entry+0xc70>
    c.literal(R::R1,0x329f0)?;
    // 0x32924: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32928: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3c9c
    c.bl(0x2ec94)?;
    // 0x3292c: ldr r1, [pc, #0xa8]         @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x329dc)?;
    // 0x32930: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32934: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3ca8
    c.bl(0x2ec94)?;
    // 0x32938: ldr r2, [pc, #0x94]         @ 0xc54 <entry+0xc54>
    c.literal(R::R2,0x329d4)?;
    // 0x3293c: mov r1, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,221)?)?;
    // 0x32940: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32944: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x32948: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x311a0
    c.bl(0x17b0)?;
    // 0x3294c: ldr r0, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,320)?)?;
    // 0x32950: bl 0x292c <entry+0xfffffffffffd0bac> @ imm = #-0x3002c
    c.bl(0x292c)?;
    // 0x32954: ldr r0, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,324)?)?;
    // 0x32958: bl 0x292c <entry+0xfffffffffffd0bac> @ imm = #-0x30034
    c.bl(0x292c)?;
    // 0x3295c: ldr r0, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,316)?)?;
    // 0x32960: bl 0x292c <entry+0xfffffffffffd0bac> @ imm = #-0x3003c
    c.bl(0x292c)?;
    // 0x32964: ldr r0, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,312)?)?;
    // 0x32968: bl 0x292c <entry+0xfffffffffffd0bac> @ imm = #-0x30044
    c.bl(0x292c)?;
    // 0x3296c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x32970: b 0x329b4 <entry+0xc34>   @ imm = #0x3c
    c.b(C::Al,0x329b4)?;
    // 0x32974: ldr r1, [pc, #0x78]         @ 0xc74 <entry+0xc74>
    c.literal(R::R1,0x329f4)?;
    // 0x32978: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3297c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3cf0
    c.bl(0x2ec94)?;
    // 0x32980: ldr r1, [pc, #0x54]         @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x329dc)?;
    // 0x32984: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32988: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3cfc
    c.bl(0x2ec94)?;
    // 0x3298c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x32990: b 0x329b4 <entry+0xc34>   @ imm = #0x1c
    c.b(C::Al,0x329b4)?;
    // 0x32994: ldr r0, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,312)?)?;
    // 0x32998: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2fa84
    c.bl(0x2f1c)?;
    // 0x3299c: ldr r0, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,324)?)?;
    // 0x329a0: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2fa8c
    c.bl(0x2f1c)?;
    // 0x329a4: ldr r0, [r4, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,320)?)?;
    // 0x329a8: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2fa94
    c.bl(0x2f1c)?;
    // 0x329ac: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x329b0: bl 0x2f1c <entry+0xfffffffffffd119c> @ imm = #-0x2fa9c
    c.bl(0x2f1c)?;
    // 0x329b4: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x329b8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x329bc: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x329c0)?;
    for word in RESOURCE_STATUS_LITERALS {c.emit(word)?;}
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    allocate_codec_resources(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_buffer_resources_and_literals_are_bounded(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let read=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(read(0x329c0),0x0003_29f8);
        assert_eq!(read(0x329d0),0x0010_ceac);
        assert_eq!(read(0x329f4),RESOURCE_STATUS_LITERALS[13]);
    }
}
