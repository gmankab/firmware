// SPDX-License-Identifier: GPL-3.0-only
//! Agent 20-03 checkpoint 101: codec stream/output queue command submission.
//! Controller channel selection, packed records, and guarded ring-buffer
//! updates. Instruction words are emitted through checked A32 operand forms,
//! never copied from the original proprietary MBN. This routine returns at
//! 0x3f818, with two explicit delayed/fallback branch instructions to 0x3f834.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x3f34c;
const END:usize=0x3f834;

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3f34c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3f350: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x3f354: sub sp, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // 0x3f358: mov r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,0)?)?;
    // 0x3f35c: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x3f360: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x3f364: str r10, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R11,-32)?)?;
    // 0x3f368: ldr r0, [r9, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2104)?)?;
    // 0x3f36c: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x3f370: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x3f374: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3f378: beq 0x3f5e4 <entry+0x298>   @ imm = #0x264
    c.b(C::Eq,0x3f5e4)?;
    // 0x3f37c: ldrb r2, [r9, #0xa48]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,2632)?)?;
    // 0x3f380: add r0, r9, #588
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,588)?)?;
    // 0x3f384: sub r8, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,32)?)?;
    // 0x3f388: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f38c: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2048)?)?;
    // 0x3f390: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3f394: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x3f398: addeq r0, r1, #272
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R1,272)?)?;
    // 0x3f39c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3f3a0: str r0, [r9, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1784)?)?;
    // 0x3f3a4: ldr r1, [r9, #0x86c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2156)?)?;
    // 0x3f3a8: add r7, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3f3ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f3b0: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x3f3b4: str r7, [r9, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(9)?,1776)?)?;
    // 0x3f3b8: ldr r4, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(9)?,2160)?)?;
    // 0x3f3bc: ldr r5, [r9, #0x874]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(9)?,2164)?)?;
    // 0x3f3c0: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f3c4: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f3c8: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x3f3cc: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x3f3d0: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x3f3d4: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x13b0
    c.bl(0x3e02c)?;
    // 0x3f3d8: ldr r0, [r9, #0x6f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1776)?)?;
    // 0x3f3dc: ldr r1, [r9, #0x870]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2160)?)?;
    // 0x3f3e0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3f3e4: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f3e8: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f3ec: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f3f0: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,0)?)?;
    // 0x3f3f4: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x3f3f8: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f3fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f400: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x3f404: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x13e0
    c.bl(0x3e02c)?;
    // 0x3f408: ldr r0, [r9, #0x878]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2168)?)?;
    // 0x3f40c: add r7, r7, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3f410: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f414: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f418: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x3f41c: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x3f420: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x3f424: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f428: add r4, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3f42c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x3f430: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x3f434: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f438: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f43c: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x1418
    c.bl(0x3e02c)?;
    // 0x3f440: ldr r0, [r9, #0x878]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2168)?)?;
    // 0x3f444: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f448: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f44c: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x3f450: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x3f454: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x3f458: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x3f45c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f460: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f464: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f468: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x1444
    c.bl(0x3e02c)?;
    // 0x3f46c: ldrb r0, [r9, #0x139]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,313)?)?;
    // 0x3f470: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3f474: beq 0x3f524 <entry+0x1d8>   @ imm = #0xa8
    c.b(C::Eq,0x3f524)?;
    // 0x3f478: ldr r0, [r9, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1784)?)?;
    // 0x3f47c: ldr r1, [r9, #0x86c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2156)?)?;
    // 0x3f480: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x3f484: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f488: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f48c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x3f490: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f494: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x3f498: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f49c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f4a0: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x3f4a4: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x1480
    c.bl(0x3e02c)?;
    // 0x3f4a8: ldr r0, [r9, #0x6f8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,1784)?)?;
    // 0x3f4ac: ldr r1, [r9, #0x86c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2156)?)?;
    // 0x3f4b0: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f4b4: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f4b8: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x3f4bc: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x3f4c0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f4c4: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f4c8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f4cc: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x3f4d0: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x14ac
    c.bl(0x3e02c)?;
    // 0x3f4d4: ldr r0, [r9, #0x874]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2164)?)?;
    // 0x3f4d8: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f4dc: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f4e0: stm sp, {r4, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3f4e4: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x3f4e8: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x3f4ec: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f4f0: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f4f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f4f8: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x14d4
    c.bl(0x3e02c)?;
    // 0x3f4fc: ldr r0, [r9, #0x874]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2164)?)?;
    // 0x3f500: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f504: str r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,20)?)?;
    // 0x3f508: stm sp, {r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(5)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3f50c: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x3f510: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x3f514: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f518: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3f51c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f520: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x14fc
    c.bl(0x3e02c)?;
    // 0x3f524: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x3f528: mvn r2, #1600
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1600)?)?;
    // 0x3f52c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3f530: ldreq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
    // 0x3f534: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x3f538: streq r1, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
    // 0x3f53c: mov r1, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,96)?)?;
    // 0x3f540: stm r0, {r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3f544: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x3f548: add r3, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,1)?)?;
    // 0x3f54c: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // 0x3f550: add r3, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,8)?)?;
    // 0x3f554: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x3f558: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x3f55c: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x3f560: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3f564: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x3f568: mov r3, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,100)?)?;
    // 0x3f56c: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,8)?)?;
    // 0x3f570: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,12)?)?;
    // 0x3f574: add r2, r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,2)?)?;
    // 0x3f578: str r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-32)?)?;
    // 0x3f57c: add r2, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,16)?)?;
    // 0x3f580: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x3f584: tst r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
    // 0x3f588: ldreq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R2,0)?)?;
    // 0x3f58c: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x3f590: streq r3, [r2]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R2,0)?)?;
    // 0x3f594: mov r3, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,104)?)?;
    // 0x3f598: mvn r2, #1600
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1600)?)?;
    // 0x3f59c: str r3, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,16)?)?;
    // 0x3f5a0: add r3, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,3)?)?;
    // 0x3f5a4: str r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,20)?)?;
    // 0x3f5a8: add r1, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
    // 0x3f5ac: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // 0x3f5b0: add r3, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,24)?)?;
    // 0x3f5b4: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x3f5b8: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x3f5bc: ldreq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::new(3)?,0)?)?;
    // 0x3f5c0: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3f5c4: streq r7, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::new(3)?,0)?)?;
    // 0x3f5c8: mov r3, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,108)?)?;
    // 0x3f5cc: str r3, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,24)?)?;
    // 0x3f5d0: str r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,28)?)?;
    // 0x3f5d4: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x3f5d8: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x3f5dc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x3f5e0: b 0x3f7c8 <entry+0x47c>   @ imm = #0x1e0
    c.b(C::Al,0x3f7c8)?;
    // 0x3f5e4: ldrb r0, [r9, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,350)?)?;
    // 0x3f5e8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3f5ec: bne 0x3f608 <entry+0x2bc>   @ imm = #0x14
    c.b(C::Ne,0x3f608)?;
    // 0x3f5f0: ldr r1, [r1, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,296)?)?;
    // 0x3f5f4: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // 0x3f5f8: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x3f5fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3f600: bne 0x3f640 <entry+0x2f4>   @ imm = #0x38
    c.b(C::Ne,0x3f640)?;
    // 0x3f604: b 0x3f7c8 <entry+0x47c>   @ imm = #0x1bc
    c.b(C::Al,0x3f7c8)?;
    // 0x3f608: ldr r0, [r9, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,300)?)?;
    // 0x3f60c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3f610: bne 0x3f620 <entry+0x2d4>   @ imm = #0x8
    c.b(C::Ne,0x3f620)?;
    // 0x3f614: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // 0x3f618: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3f61c: bls 0x3f628 <entry+0x2dc>   @ imm = #0x4
    c.b(C::Ls,0x3f628)?;
    // 0x3f620: add r2, r1, #296
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,296)?)?;
    // 0x3f624: b 0x3f62c <entry+0x2e0>   @ imm = #0x0
    c.b(C::Al,0x3f62c)?;
    // 0x3f628: add r2, r1, #272
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,272)?)?;
    // 0x3f62c: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // 0x3f630: ldr r1, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,0)?)?;
    // 0x3f634: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3f638: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x3f63c: bls 0x3f81c <entry+0x4d0>   @ imm = #0x1d8
    c.b(C::Ls,0x3f81c)?;
    // 0x3f640: mov r8, #356
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,356)?)?;
    // 0x3f644: mov r10, #2400
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,2400)?)?;
    // 0x3f648: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x3f64c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3f650: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3f654: orr r8, r8, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,14336)?)?;
    // 0x3f658: orr r10, r10, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,12288)?)?;
    // 0x3f65c: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x3f660: ldr r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,32)?)?;
    // 0x3f664: sub r2, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,32)?)?;
    // 0x3f668: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f66c: ldr r0, [r9, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,964)?)?;
    // 0x3f670: ldr r1, [r9, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,2064)?)?;
    // 0x3f674: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f678: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x3f67c: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x3f680: mla r2, r0, r7, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::R2,R::R0,R::new(7)?,R::new(5)?)?)?;
    // 0x3f684: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x3f688: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x3f68c: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x3f690: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3f694: mov r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,11)?)?;
    // 0x3f698: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x1674
    c.bl(0x3e02c)?;
    // 0x3f69c: ldr r1, [r9, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,964)?)?;
    // 0x3f6a0: ldr r0, [r9, #0x810]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,2064)?)?;
    // 0x3f6a4: ldr r2, [r9, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(9)?,968)?)?;
    // 0x3f6a8: sub r3, r11, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R11,32)?)?;
    // 0x3f6ac: str r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,16)?)?;
    // 0x3f6b0: str r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,20)?)?;
    // 0x3f6b4: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x3f6b8: mla r3, r1, r7, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R1,R::new(7)?,R::new(5)?)?)?;
    // 0x3f6bc: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3f6c0: add r0, r3, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3f6c4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f6c8: stmib sp, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R2.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x3f6cc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x3f6d0: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x3f6d4: bl 0x3e02c <entry+0xffffffffffffece0> @ imm = #-0x16b0
    c.bl(0x3e02c)?;
    // 0x3f6d8: ldr r0, [r9, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,304)?)?;
    // 0x3f6dc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3f6e0: beq 0x3f7b0 <entry+0x464>   @ imm = #0xc8
    c.b(C::Eq,0x3f7b0)?;
    // 0x3f6e4: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x3f6e8: mov r12, r6
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(6)?))?;
    // 0x3f6ec: lsl r1, r7, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(7)?,Shift::Lsl,16)?)?;
    // 0x3f6f0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3f6f4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3f6f8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3f6fc: orr r7, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3f700: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x3f704: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x3f708: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x3f70c: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x3f710: str r8, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,0)?)?;
    // 0x3f714: str r7, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,4)?)?;
    // 0x3f718: orr r7, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,16)?)?;
    // 0x3f71c: ldr r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
    // 0x3f720: add r5, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(3)?,1)?)?;
    // 0x3f724: str r5, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
    // 0x3f728: add r5, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,8)?)?;
    // 0x3f72c: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x3f730: ldreq r6, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(5)?,0)?)?;
    // 0x3f734: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3f738: streq r6, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(5)?,0)?)?;
    // 0x3f73c: add r5, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,16)?)?;
    // 0x3f740: add r6, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(3)?,2)?)?;
    // 0x3f744: str r10, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,8)?)?;
    // 0x3f748: str r4, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,12)?)?;
    // 0x3f74c: str r6, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-32)?)?;
    // 0x3f750: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x3f754: ldreq r6, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(5)?,0)?)?;
    // 0x3f758: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3f75c: streq r6, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(5)?,0)?)?;
    // 0x3f760: str r8, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R0,16)?)?;
    // 0x3f764: str r7, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,20)?)?;
    // 0x3f768: add r7, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(3)?,3)?)?;
    // 0x3f76c: add r3, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,4)?)?;
    // 0x3f770: str r7, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
    // 0x3f774: add r7, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,24)?)?;
    // 0x3f778: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x3f77c: ldreq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::new(7)?,0)?)?;
    // 0x3f780: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3f784: streq r6, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::new(7)?,0)?)?;
    // 0x3f788: str r10, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,24)?)?;
    // 0x3f78c: str r4, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,28)?)?;
    // 0x3f790: str r3, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
    // 0x3f794: and r3, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R2,255)?)?;
    // 0x3f798: add r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
    // 0x3f79c: ldr r7, [r9, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,304)?)?;
    // 0x3f7a0: cmp r7, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3f7a4: bhi 0x3f6f8 <entry+0x3ac>   @ imm = #-0xb4
    c.b(C::Hi,0x3f6f8)?;
    // 0x3f7a8: mov r6, r12
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // 0x3f7ac: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x3f7b0: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x3f7b4: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,24)?)?;
    // 0x3f7b8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x3f7bc: and r7, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,255)?)?;
    // 0x3f7c0: cmp r0, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3f7c4: bhi 0x3f65c <entry+0x310>   @ imm = #-0x170
    c.b(C::Hi,0x3f65c)?;
    // 0x3f7c8: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x3f7cc: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x3f7d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3f7d4: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x3f7d8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3f7dc: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x3f7e0: moveq r0, #256
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x3f7e4: tst r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,31)?)?;
    // 0x3f7e8: ldreq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R1,0)?)?;
    // 0x3f7ec: andeq r2, r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
    // 0x3f7f0: streq r2, [r1]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R1,0)?)?;
    // 0x3f7f4: mov r2, #2256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2256)?)?;
    // 0x3f7f8: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x3f7fc: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x3f800: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x3f804: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,8)?)?;
    // 0x3f808: ldr r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
    // 0x3f80c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x3f810: add r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,1)?)?;
    // 0x3f814: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x3f818: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3f81c: ldr r1, [r9, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(9)?,300)?)?;
    // 0x3f820: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3f824: ldrne r1, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(9)?,4)?)?;
    // 0x3f828: cmpne r1, #8192
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x3f82c: beq 0x3f7c8 <entry+0x47c>   @ imm = #-0x6c
    c.b(C::Eq,0x3f7c8)?;
    // 0x3f830: b 0x3f5fc <entry+0x2b0>   @ imm = #-0x23c
    c.b(C::Al,0x3f5fc)?;
    c.check(END)
}
