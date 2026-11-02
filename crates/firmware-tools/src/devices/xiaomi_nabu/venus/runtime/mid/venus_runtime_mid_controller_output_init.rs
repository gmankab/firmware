// SPDX-License-Identifier: GPL-3.0-only
//! Typed A32 construction of a codec output controller descriptor matrix.
//! Initializes queue, channel, and packed plane format fields, branching into
//! common codec command helpers. This is an incomplete function: code starts
//! at 0x3f834 and continues beyond 0x3fc00, with NO invented epilogue.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x3f834;
const END:usize=0x3fc00;

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3f834: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3f838: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x3f83c: sub sp, sp, #188
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,188)?)?;
    // 0x3f840: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x3f844: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3f848: str r1, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
    // 0x3f84c: ldr r1, [r0, #0x2b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,696)?)?;
    // 0x3f850: ldr lr, [r0, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R0,692)?)?;
    // 0x3f854: ldr r10, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R0,716)?)?;
    // 0x3f858: ldr r8, [r0, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,720)?)?;
    // 0x3f85c: str r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,104)?)?;
    // 0x3f860: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x3f864: mov r12, r1
    c.emit(arm32::mov_register(C::Al,R::R12,R::R1))?;
    // 0x3f868: ldrb r2, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,336)?)?;
    // 0x3f86c: ands r2, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R2,R::R2,1,true)?)?;
    // 0x3f870: movne r12, lr
    c.emit(arm32::mov_register(C::Ne,R::R12,R::LR))?;
    // 0x3f874: movne lr, r1
    c.emit(arm32::mov_register(C::Ne,R::LR,R::R1))?;
    // 0x3f878: lsl r1, r10, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(10)?,Shift::Lsl,3)?)?;
    // 0x3f87c: cmp r1, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // 0x3f880: str r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,96)?)?;
    // 0x3f884: str lr, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,76)?)?;
    // 0x3f888: lslhs r1, r8, #3
    c.emit(arm32::mov_shifted_register(C::Cs,R::R1,R::new(8)?,Shift::Lsl,3)?)?;
    // 0x3f88c: cmphs r1, r12
    c.emit(arm32::data_register(C::Cs,Op::Cmp,R::R0,R::R1,R::R12,Shift::Lsl,0,false)?)?;
    // 0x3f890: bhs 0x3f8b8 <entry+0x56c>   @ imm = #0x20
    c.b(C::Cs,0x3f8b8)?;
    // 0x3f894: ldr r2, [pc, #0xd20]        @ 0x1270 <entry+0x1270>
    c.literal_cond(C::Al,R::R2,0x405bc)?;
    // 0x3f898: mov r1, #79
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,79)?)?;
    // 0x3f89c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3f8a0: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5632)?)?;
    // 0x3f8a4: bl 0x17b0 <entry+0xfffffffffffc2464> @ imm = #-0x3e0fc
    c.bl(0x17b0)?;
    // 0x3f8a8: ldr r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x3f8ac: ldr lr, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,76)?)?;
    // 0x3f8b0: ldr r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,96)?)?;
    // 0x3f8b4: ldr r0, [r0, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2744)?)?;
    // 0x3f8b8: ldrb r2, [r0, #0x145]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,325)?)?;
    // 0x3f8bc: ldrb r3, [r0, #0x13e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R0,318)?)?;
    // 0x3f8c0: cmp lr, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x3f8c4: ldrb r7, [r0, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R0,323)?)?;
    // 0x3f8c8: ldrb r1, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,339)?)?;
    // 0x3f8cc: str r8, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,72)?)?;
    // 0x3f8d0: and r6, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::R2,2)?)?;
    // 0x3f8d4: and r4, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::R2,1)?)?;
    // 0x3f8d8: movne r4, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x3f8dc: lsr r5, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x3f8e0: cmp r12, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3f8e4: str r6, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,60)?)?;
    // 0x3f8e8: movne r5, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x3f8ec: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3f8f0: str r4, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,48)?)?;
    // 0x3f8f4: str r5, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,52)?)?;
    // 0x3f8f8: orr r6, r4, r5, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(5)?,Shift::Lsl,1,false)?)?;
    // 0x3f8fc: mov r5, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,24)?)?;
    // 0x3f900: and r3, r5, r3, lsl #3
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,3,false)?)?;
    // 0x3f904: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x3f908: and r7, r5, r7, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::new(7)?,Shift::Lsl,1,false)?)?;
    // 0x3f90c: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3f910: orr r3, r7, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3f914: orreq r7, r3, #96
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::new(7)?,R::new(3)?,96)?)?;
    // 0x3f918: cmp r12, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3f91c: orr r3, r7, r6, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(7)?,R::new(6)?,Shift::Lsl,7,false)?)?;
    // 0x3f920: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3f924: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x3f928: orr r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1)?)?;
    // 0x3f92c: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x3f930: str r7, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,56)?)?;
    // 0x3f934: blo 0x3f970 <entry+0x624>   @ imm = #0x34
    c.b(C::Cc,0x3f970)?;
    // 0x3f938: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x3f93c: bic r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x3f940: and r4, r7, r2, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(7)?,R::R2,Shift::Lsr,4,false)?)?;
    // 0x3f944: and r7, r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R2,32)?)?;
    // 0x3f948: orr r6, r4, r7, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(4)?,R::new(7)?,Shift::Lsr,4,false)?)?;
    // 0x3f94c: str r4, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,80)?)?;
    // 0x3f950: lsr r4, r7, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(7)?,Shift::Lsr,5)?)?;
    // 0x3f954: and r7, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R2,8)?)?;
    // 0x3f958: and r2, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,4)?)?;
    // 0x3f95c: orr r3, r3, r6, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,9,false)?)?;
    // 0x3f960: str r4, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,84)?)?;
    // 0x3f964: lsr r8, r7, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::new(7)?,Shift::Lsr,3)?)?;
    // 0x3f968: lsr r5, r2, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R2,Shift::Lsr,2)?)?;
    // 0x3f96c: b 0x3f9b4 <entry+0x668>   @ imm = #0x40
    c.b(C::Al,0x3f9b4)?;
    // 0x3f970: rsb r2, r8, r8, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R2,R::new(8)?,R::new(8)?,Shift::Lsl,3,false)?)?;
    // 0x3f974: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x3f978: cmp r12, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3f97c: rsb r2, r10, r10, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R2,R::new(10)?,R::new(10)?,Shift::Lsl,3,false)?)?;
    // 0x3f980: movhi r4, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x3f984: cmp lr, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3f988: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3f98c: movhi r2, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3f990: cmp r12, r8, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(8)?,Shift::Lsl,2,false)?)?;
    // 0x3f994: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x3f998: str r4, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,84)?)?;
    // 0x3f99c: str r2, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,80)?)?;
    // 0x3f9a0: orr r2, r2, r4, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(4)?,Shift::Lsl,1,false)?)?;
    // 0x3f9a4: movhi r8, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x3f9a8: cmp lr, r10, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::LR,R::new(10)?,Shift::Lsl,2,false)?)?;
    // 0x3f9ac: orr r3, r3, r2, lsl #9
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,9,false)?)?;
    // 0x3f9b0: movhi r5, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x3f9b4: ldrb r0, [r0, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,336)?)?;
    // 0x3f9b8: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // 0x3f9bc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3f9c0: and r0, r2, r0, lsl #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,11,false)?)?;
    // 0x3f9c4: lsr r2, lr, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::LR,Shift::Lsr,8)?)?;
    // 0x3f9c8: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x3f9cc: bic r2, r3, #30720
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::new(3)?,30720)?)?;
    // 0x3f9d0: movhi r7, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x3f9d4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3f9d8: orr r2, r2, r7, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,12,false)?)?;
    // 0x3f9dc: ldr r7, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(9)?,0)?)?;
    // 0x3f9e0: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3f9e4: orrne r0, r0, #16384
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x3f9e8: tst r7, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,31)?)?;
    // 0x3f9ec: ldreq r1, [r7]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(7)?,0)?)?;
    // 0x3f9f0: andeq r1, r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
    // 0x3f9f4: streq r1, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(7)?,0)?)?;
    // 0x3f9f8: mov r1, #2064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2064)?)?;
    // 0x3f9fc: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x3fa00: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // 0x3fa04: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x3fa08: str r9, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,100)?)?;
    // 0x3fa0c: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x3fa10: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,68)?)?;
    // 0x3fa14: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3fa18: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fa1c: add r0, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,8)?)?;
    // 0x3fa20: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x3fa24: ldr r2, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,104)?)?;
    // 0x3fa28: ldrb r1, [r2, #0x3db]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,987)?)?;
    // 0x3fa2c: ldr r2, [r2, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,2744)?)?;
    // 0x3fa30: ldr r3, [r2, #0x520]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,1312)?)?;
    // 0x3fa34: cmp r3, #25
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,25)?)?;
    // 0x3fa38: beq 0x3fa44 <entry+0x6f8>   @ imm = #0x4
    c.b(C::Eq,0x3fa44)?;
    // 0x3fa3c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3fa40: beq 0x405a4 <entry+0x1258>  @ imm = #0xb5c
    c.b(C::Eq,0x405a4)?;
    // 0x3fa44: and r3, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,1)?)?;
    // 0x3fa48: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x3fa4c: ldr r6, [pc, #0xfc4]        @ 0x16cc <entry+0x16cc>
    c.literal_cond(C::Al,R::new(6)?,0x40a18)?;
    // 0x3fa50: ldr r1, [r1, #0x30c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,780)?)?;
    // 0x3fa54: and r1, r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
    // 0x3fa58: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x3fa5c: addlo r6, r6, r1
    c.emit(arm32::data_register(C::Cc,Op::Add,R::new(6)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3fa60: ldrb r1, [r6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x3fa64: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x3fa68: and r1, r6, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(6)?,R::R1,Shift::Lsl,1,false)?)?;
    // 0x3fa6c: orr r4, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3fa70: ldrb r3, [r2, #0x143]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,323)?)?;
    // 0x3fa74: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x3fa78: beq 0x3fa84 <entry+0x738>   @ imm = #0x4
    c.b(C::Eq,0x3fa84)?;
    // 0x3fa7c: orr r4, r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,16)?)?;
    // 0x3fa80: b 0x3faa4 <entry+0x758>   @ imm = #0x1c
    c.b(C::Al,0x3faa4)?;
    // 0x3fa84: ldr r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,72)?)?;
    // 0x3fa88: cmp r12, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x3fa8c: lslls r3, r10, #1
    c.emit(arm32::mov_shifted_register(C::Ls,R::new(3)?,R::new(10)?,Shift::Lsl,1)?)?;
    // 0x3fa90: cmpls lr, r3
    c.emit(arm32::data_register(C::Ls,Op::Cmp,R::R0,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3fa94: bhi 0x3fa7c <entry+0x730>   @ imm = #-0x20
    c.b(C::Hi,0x3fa7c)?;
    // 0x3fa98: ldrb r2, [r2, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,336)?)?;
    // 0x3fa9c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3faa0: orrne r4, r4, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::new(4)?,R::new(4)?,16)?)?;
    // 0x3faa4: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,24)?)?;
    // 0x3faa8: ldr r4, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,104)?)?;
    // 0x3faac: mov r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,255)?)?;
    // 0x3fab0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3fab4: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x3fab8: orr r6, r6, #16128
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,16128)?)?;
    // 0x3fabc: ldr r2, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,716)?)?;
    // 0x3fac0: ldr r3, [r4, #0x2d0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,720)?)?;
    // 0x3fac4: and r2, r2, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3fac8: mov r6, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16711680)?)?;
    // 0x3facc: orr r6, r6, #1056964608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,1056964608)?)?;
    // 0x3fad0: and r3, r6, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(6)?,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x3fad4: add r6, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,16)?)?;
    // 0x3fad8: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3fadc: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x3fae0: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x3fae4: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x3fae8: mov r0, #10432
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10432)?)?;
    // 0x3faec: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x3faf0: str r2, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,12)?)?;
    // 0x3faf4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3faf8: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x3fafc: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x3fb00: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x3fb04: ldr r0, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,100)?)?;
    // 0x3fb08: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,0)?)?;
    // 0x3fb0c: ldr r0, [r4, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,644)?)?;
    // 0x3fb10: str r5, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
    // 0x3fb14: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x3fb18: lsl r0, r5, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsl,6)?)?;
    // 0x3fb1c: orr r3, r0, r8, lsl #7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::new(8)?,Shift::Lsl,7,false)?)?;
    // 0x3fb20: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x3fb24: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3fb28: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x3fb2c: ldrb r5, [r0, #0x151]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::R0,337)?)?;
    // 0x3fb30: and r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x3fb34: orr r3, r3, r5, lsl #25
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,25,false)?)?;
    // 0x3fb38: ldrb r5, [r4, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(4)?,988)?)?;
    // 0x3fb3c: and r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(5)?,1)?)?;
    // 0x3fb40: orr r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3fb44: orr r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3fb48: orr r1, r1, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,2,false)?)?;
    // 0x3fb4c: bne 0x3fb5c <entry+0x810>   @ imm = #0x8
    c.b(C::Ne,0x3fb5c)?;
    // 0x3fb50: ldr r2, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,92)?)?;
    // 0x3fb54: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x3fb58: bne 0x3fb64 <entry+0x818>   @ imm = #0x4
    c.b(C::Ne,0x3fb64)?;
    // 0x3fb5c: orr r5, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,32)?)?;
    // 0x3fb60: b 0x3fb68 <entry+0x81c>   @ imm = #0x0
    c.b(C::Al,0x3fb68)?;
    // 0x3fb64: orr r5, r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R1,16)?)?;
    // 0x3fb68: ldrb r2, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R0,339)?)?;
    // 0x3fb6c: mov r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,255)?)?;
    // 0x3fb70: mov r3, #1044480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1044480)?)?;
    // 0x3fb74: orr r4, r4, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4261412864)?)?;
    // 0x3fb78: orr r3, r3, #32505856
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,32505856)?)?;
    // 0x3fb7c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3fb80: beq 0x3fbc4 <entry+0x878>   @ imm = #0x3c
    c.b(C::Eq,0x3fbc4)?;
    // 0x3fb84: ldrb r0, [r0, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,340)?)?;
    // 0x3fb88: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3fb8c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3fb90: orrhi r0, r1, #48
    c.emit(arm32::data_immediate(C::Hi,Op::Orr,R::R0,R::R1,48)?)?;
    // 0x3fb94: ldr r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
    // 0x3fb98: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3fb9c: ldr r1, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,104)?)?;
    // 0x3fba0: movne r5, r0
    c.emit(arm32::mov_register(C::Ne,R::new(5)?,R::R0))?;
    // 0x3fba4: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x3fba8: movne r5, r0
    c.emit(arm32::mov_register(C::Ne,R::new(5)?,R::R0))?;
    // 0x3fbac: and r0, r5, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3fbb0: ldr r1, [r1, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,716)?)?;
    // 0x3fbb4: and r1, r3, r1, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,12,false)?)?;
    // 0x3fbb8: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3fbbc: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x3fbc0: b 0x3fc18 <entry+0x8cc>   @ imm = #0x50
    c.b(C::Al,0x3fc18)?;
    // 0x3fbc4: ldr r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,104)?)?;
    // 0x3fbc8: ldr r4, [r0, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,928)?)?;
    // 0x3fbcc: ldr r9, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::R0,716)?)?;
    // 0x3fbd0: add r0, r10, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3fbd4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3fbd8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3fbdc: blx 0x83c4c <entry+0x44900> @ imm = #0x44068
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x3fbe0: mov r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,255)?)?;
    // 0x3fbe4: mov r2, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3840)?)?;
    // 0x3fbe8: ldr lr, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,76)?)?;
    // 0x3fbec: ldr r12, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,96)?)?;
    // 0x3fbf0: orr r1, r1, #-33554432
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4261412864)?)?;
    // 0x3fbf4: and r0, r2, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,8,false)?)?;
    // 0x3fbf8: and r1, r5, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3fbfc: cmp r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,256)?)?;
    c.check(END)
}
