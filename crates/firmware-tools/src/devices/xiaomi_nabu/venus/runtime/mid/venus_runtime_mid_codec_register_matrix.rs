// SPDX-License-Identifier: GPL-3.0-only
//! Compute controller codec register stride, plane allocation sizes, and
//! cache-aligned frame geometry from input options. Multiplications and
//! conditional data paths use checked public A32 operand constructors.
//! Twelve trailing words are original controller constants/string pointers,
//! not executable code; their values are explicitly distinguished.
use super::Code;
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
const START:usize=0x3e304;
const END:usize=0x3ead8;
const GEOMETRY_MASKS_AND_MESSAGES:[u32;12]=[
    0x0fffffc0,     0xffff7ffe,     0xaaaaaaab,     0x55555556,
    0xfffff000,     0x000ee0d7,     0x000ee08e,     0x000e2788,
    0x000ee028,     0x000ee0a6,     0x000e6c19,     0x000ee05c,
];

fn calculate_codec_register_matrix(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3e304: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3e308: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x3e30c: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x3e310: mov r10, r3
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(3)?))?;
    // 0x3e314: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x3e318: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x3e31c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x3e320: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3e324: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x3e328: add r0, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,4)?)?;
    // 0x3e32c: bl 0x31ec <entry+0xfffffffffffc961c> @ imm = #-0x3b148
    c.bl(0x31ec)?;
    // 0x3e330: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    // 0x3e334: str r5, [r10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,0)?)?;
    // 0x3e338: ldr r0, [r0, #0x834]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,2100)?)?;
    // 0x3e33c: ldr r1, [r0, #0x734]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,1844)?)?;
    // 0x3e340: ldr r7, [r0, #0x73c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,1852)?)?;
    // 0x3e344: ldr r8, [r0, #0x744]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,1860)?)?;
    // 0x3e348: ldr r4, [r0, #0x74c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,1868)?)?;
    // 0x3e34c: ldr r2, [pc, #0x754]        @ 0x4ed8 <entry+0x4ed8>
    c.literal_cond(C::Al,R::R2,0x3eaa8)?;
    // 0x3e350: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3e354: orr r0, r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16384)?)?;
    // 0x3e358: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e35c: bgt 0x3e3b0 <entry+0x47e0>  @ imm = #0x4c
    c.b(C::Gt,0x3e3b0)?;
    // 0x3e360: sub r0, r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,2)?)?;
    // 0x3e364: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x3e368: blo 0x3e440 <entry+0x4870>  @ imm = #0xd0
    c.b(C::Cc,0x3e440)?;
    // 0x3e36c: sub r0, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(5)?,4)?)?;
    // 0x3e370: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x3e374: bhs 0x3e46c <entry+0x489c>  @ imm = #0xf0
    c.b(C::Cs,0x3e46c)?;
    // 0x3e378: ldr r1, [pc, #0x74c]        @ 0x4efc <entry+0x4efc>
    c.literal_cond(C::Al,R::R1,0x3eacc)?;
    // 0x3e37c: ldr r2, [pc, #0x73c]        @ 0x4ef0 <entry+0x4ef0>
    c.literal_cond(C::Al,R::R2,0x3eac0)?;
    // 0x3e380: mov r3, #179
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,179)?)?;
    // 0x3e384: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3e388: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x3e38c: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xf700
    c.bl(0x2ec94)?;
    // 0x3e390: ldr r1, [pc, #0x72c]        @ 0x4ef4 <entry+0x4ef4>
    c.literal_cond(C::Al,R::R1,0x3eac4)?;
    // 0x3e394: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3e398: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xf70c
    c.bl(0x2ec94)?;
    // 0x3e39c: ldr r2, [pc, #0x724]        @ 0x4ef8 <entry+0x4ef8>
    c.literal_cond(C::Al,R::R2,0x3eac8)?;
    // 0x3e3a0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3e3a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3e3a8: mov r1, #436
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,436)?)?;
    // 0x3e3ac: b 0x3e548 <entry+0x4978>  @ imm = #0x194
    c.b(C::Al,0x3e548)?;
    // 0x3e3b0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3e3b4: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32768)?)?;
    // 0x3e3b8: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e3bc: ble 0x3e3fc <entry+0x482c>  @ imm = #0x38
    c.b(C::Le,0x3e3fc)?;
    // 0x3e3c0: ldr r0, [pc, #0x6e4]        @ 0x4edc <entry+0x4edc>
    c.literal_cond(C::Al,R::R0,0x3eaac)?;
    // 0x3e3c4: str r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,32)?)?;
    // 0x3e3c8: add r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e3cc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x3e3d0: bhs 0x3e4a4 <entry+0x48d4>  @ imm = #0xcc
    c.b(C::Cs,0x3e4a4)?;
    // 0x3e3d4: str r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,28)?)?;
    // 0x3e3d8: sub r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3e3dc: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x3e3e0: add r5, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e3e4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e3e8: beq 0x3e560 <entry+0x4990>  @ imm = #0x170
    c.b(C::Eq,0x3e560)?;
    // 0x3e3ec: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e3f0: bl 0x81850 <entry+0x47c80> @ imm = #0x43458
    c.bl(0x81850)?;
    // 0x3e3f4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e3f8: b 0x3e568 <entry+0x4998>  @ imm = #0x168
    c.b(C::Al,0x3e568)?;
    // 0x3e3fc: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x3e400: orr r2, r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16384)?)?;
    // 0x3e404: cmp r5, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3e408: beq 0x3e50c <entry+0x493c>  @ imm = #0xfc
    c.b(C::Eq,0x3e50c)?;
    // 0x3e40c: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e410: bne 0x3e534 <entry+0x4964>  @ imm = #0x11c
    c.b(C::Ne,0x3e534)?;
    // 0x3e414: ldr r1, [pc, #0x6a0]        @ 0x4eec <entry+0x4eec>
    c.literal_cond(C::Al,R::R1,0x3eabc)?;
    // 0x3e418: ldr r2, [pc, #0x6a0]        @ 0x4ef0 <entry+0x4ef0>
    c.literal_cond(C::Al,R::R2,0x3eac0)?;
    // 0x3e41c: mov r3, #185
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,185)?)?;
    // 0x3e420: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3e424: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x3e428: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xf79c
    c.bl(0x2ec94)?;
    // 0x3e42c: ldr r1, [pc, #0x690]        @ 0x4ef4 <entry+0x4ef4>
    c.literal_cond(C::Al,R::R1,0x3eac4)?;
    // 0x3e430: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3e434: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xf7a8
    c.bl(0x2ec94)?;
    // 0x3e438: mov r1, #186
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,186)?)?;
    // 0x3e43c: b 0x3e498 <entry+0x48c8>  @ imm = #0x54
    c.b(C::Al,0x3e498)?;
    // 0x3e440: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x3e444: mov r6, r9
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x3e448: sub r9, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R0,1)?)?;
    // 0x3e44c: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x3e450: add r5, r9, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(9)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e454: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e458: beq 0x3e73c <entry+0x4b6c>  @ imm = #0x2dc
    c.b(C::Eq,0x3e73c)?;
    // 0x3e45c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e460: bl 0x81850 <entry+0x47c80> @ imm = #0x433e8
    c.bl(0x81850)?;
    // 0x3e464: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e468: b 0x3e744 <entry+0x4b74>  @ imm = #0x2d4
    c.b(C::Al,0x3e744)?;
    // 0x3e46c: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x3e470: bne 0x3e534 <entry+0x4964>  @ imm = #0xbc
    c.b(C::Ne,0x3e534)?;
    // 0x3e474: ldr r1, [pc, #0x658]        @ 0x4f04 <entry+0x4f04>
    c.literal_cond(C::Al,R::R1,0x3ead4)?;
    // 0x3e478: ldr r2, [pc, #0x640]        @ 0x4ef0 <entry+0x4ef0>
    c.literal_cond(C::Al,R::R2,0x3eac0)?;
    // 0x3e47c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3e480: mov r3, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,388)?)?;
    // 0x3e484: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xf7f8
    c.bl(0x2ec94)?;
    // 0x3e488: ldr r1, [pc, #0x634]        @ 0x4ef4 <entry+0x4ef4>
    c.literal_cond(C::Al,R::R1,0x3eac4)?;
    // 0x3e48c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3e490: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xf804
    c.bl(0x2ec94)?;
    // 0x3e494: mov r1, #133
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,133)?)?;
    // 0x3e498: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3e49c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x3e4a0: b 0x3e540 <entry+0x4970>  @ imm = #0x98
    c.b(C::Al,0x3e540)?;
    // 0x3e4a4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3e4a8: orr r0, r0, #49152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,49152)?)?;
    // 0x3e4ac: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e4b0: bne 0x3e534 <entry+0x4964>  @ imm = #0x7c
    c.b(C::Ne,0x3e534)?;
    // 0x3e4b4: ldr r2, [pc, #0x5f4]        @ 0x4ee0 <entry+0x4ee0>
    c.literal_cond(C::Al,R::R2,0x3eab0)?;
    // 0x3e4b8: add r0, r6, #191
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,191)?)?;
    // 0x3e4bc: umull r3, r5, r0, r2
    c.emit(super::encode_long_multiply(C::Al,false,R::new(3)?,R::new(5)?,R::R0,R::R2)?)?;
    // 0x3e4c0: lsr r0, r5, #7
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(5)?,Shift::Lsr,7)?)?;
    // 0x3e4c4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x3e4c8: lsl r0, r0, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,6)?)?;
    // 0x3e4cc: str r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,40)?)?;
    // 0x3e4d0: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x3e4d4: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x3e4d8: ldr r2, [pc, #0x5d4]        @ 0x4ee4 <entry+0x4ee4>
    c.literal_cond(C::Al,R::R2,0x3eab4)?;
    // 0x3e4dc: smull r3, r5, r0, r2
    c.emit(super::encode_long_multiply(C::Al,true,R::new(3)?,R::new(5)?,R::R0,R::R2)?)?;
    // 0x3e4e0: add r0, r5, r5, lsr #31
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::new(5)?,Shift::Lsr,31,false)?)?;
    // 0x3e4e4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3e4e8: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x3e4ec: add r5, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e4f0: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x3e4f4: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e4f8: beq 0x3e82c <entry+0x4c5c>  @ imm = #0x32c
    c.b(C::Eq,0x3e82c)?;
    // 0x3e4fc: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e500: bl 0x81850 <entry+0x47c80> @ imm = #0x43348
    c.bl(0x81850)?;
    // 0x3e504: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e508: b 0x3e834 <entry+0x4c64>  @ imm = #0x324
    c.b(C::Al,0x3e834)?;
    // 0x3e50c: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x3e510: add r6, r0, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R0,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x3e514: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x3e518: add r5, r6, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e51c: tst r0, r1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e520: beq 0x3e868 <entry+0x4c98>  @ imm = #0x340
    c.b(C::Eq,0x3e868)?;
    // 0x3e524: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e528: bl 0x81850 <entry+0x47c80> @ imm = #0x43320
    c.bl(0x81850)?;
    // 0x3e52c: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e530: b 0x3e870 <entry+0x4ca0>  @ imm = #0x338
    c.b(C::Al,0x3e870)?;
    // 0x3e534: mov r1, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,118)?)?;
    // 0x3e538: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3e53c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3e540: ldr r2, [pc, #0x580]        @ 0x4ef8 <entry+0x4ef8>
    c.literal_cond(C::Al,R::R2,0x3eac8)?;
    // 0x3e544: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3e548: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3cda0
    c.bl(0x17b0)?;
    // 0x3e54c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3e550: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3e554: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3e558: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3e55c: b 0x3ea90 <entry+0x4ec0>  @ imm = #0x52c
    c.b(C::Al,0x3ea90)?;
    // 0x3e560: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x3e564: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e568: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e56c: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x3e570: str r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,40)?)?;
    // 0x3e574: sub r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,1)?)?;
    // 0x3e578: ands r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,true)?)?;
    // 0x3e57c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x3e580: add r0, r9, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e584: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e588: beq 0x3e5a0 <entry+0x49d0>  @ imm = #0x10
    c.b(C::Eq,0x3e5a0)?;
    // 0x3e58c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e590: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3e594: bl 0x81850 <entry+0x47c80> @ imm = #0x432b4
    c.bl(0x81850)?;
    // 0x3e598: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e59c: b 0x3e5a8 <entry+0x49d8>  @ imm = #0x4
    c.b(C::Al,0x3e5a8)?;
    // 0x3e5a0: rsb r0, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(7)?,0)?)?;
    // 0x3e5a4: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e5a8: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e5ac: add r5, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e5b0: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x3e5b4: str r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,44)?)?;
    // 0x3e5b8: sub r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,1)?)?;
    // 0x3e5bc: tst r0, r8
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e5c0: beq 0x3e5d8 <entry+0x4a08>  @ imm = #0x10
    c.b(C::Eq,0x3e5d8)?;
    // 0x3e5c4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e5c8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x3e5cc: bl 0x81850 <entry+0x47c80> @ imm = #0x4327c
    c.bl(0x81850)?;
    // 0x3e5d0: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e5d4: b 0x3e5e0 <entry+0x4a10>  @ imm = #0x4
    c.b(C::Al,0x3e5e0)?;
    // 0x3e5d8: rsb r0, r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(8)?,0)?)?;
    // 0x3e5dc: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e5e0: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e5e4: ldr r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,28)?)?;
    // 0x3e5e8: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x3e5ec: str r0, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,56)?)?;
    // 0x3e5f0: sub r0, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,1)?)?;
    // 0x3e5f4: ands r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,true)?)?;
    // 0x3e5f8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x3e5fc: add r0, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1)?)?;
    // 0x3e600: add r0, r4, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsr,1,false)?)?;
    // 0x3e604: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e608: beq 0x3e620 <entry+0x4a50>  @ imm = #0x10
    c.b(C::Eq,0x3e620)?;
    // 0x3e60c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e610: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3e614: bl 0x81850 <entry+0x47c80> @ imm = #0x43234
    c.bl(0x81850)?;
    // 0x3e618: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e61c: b 0x3e628 <entry+0x4a58>  @ imm = #0x4
    c.b(C::Al,0x3e628)?;
    // 0x3e620: rsb r0, r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(4)?,0)?)?;
    // 0x3e624: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e628: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e62c: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x3e630: str r0, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,60)?)?;
    // 0x3e634: add r0, r6, #31
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,31)?)?;
    // 0x3e638: mov r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // 0x3e63c: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // 0x3e640: add r0, r1, r0, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,5,false)?)?;
    // 0x3e644: and r1, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3e648: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x3e64c: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x3e650: str r1, [r10, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,48)?)?;
    // 0x3e654: add r0, r9, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,7)?)?;
    // 0x3e658: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x3e65c: add r8, r1, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::R1,R::R0,Shift::Lsr,3,false)?)?;
    // 0x3e660: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x3e664: add r5, r8, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e668: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3e66c: beq 0x3e688 <entry+0x4ab8>  @ imm = #0x14
    c.b(C::Eq,0x3e688)?;
    // 0x3e670: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e674: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3e678: bl 0x81850 <entry+0x47c80> @ imm = #0x431d0
    c.bl(0x81850)?;
    // 0x3e67c: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // 0x3e680: sub r7, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e684: b 0x3e690 <entry+0x4ac0>  @ imm = #0x4
    c.b(C::Al,0x3e690)?;
    // 0x3e688: rsb r0, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(7)?,0)?)?;
    // 0x3e68c: and r7, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e690: str r7, [r10, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(10)?,52)?)?;
    // 0x3e694: mov r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,63)?)?;
    // 0x3e698: add r5, r8, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(8)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3e69c: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x3e6a0: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,15)?)?;
    // 0x3e6a4: add r0, r1, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsr,4,false)?)?;
    // 0x3e6a8: add r1, r2, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,268435456)?)?;
    // 0x3e6ac: and r9, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(9)?,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e6b0: str r9, [r10, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(10)?,64)?)?;
    // 0x3e6b4: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x3e6b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3e6bc: beq 0x3e6d4 <entry+0x4b04>  @ imm = #0x10
    c.b(C::Eq,0x3e6d4)?;
    // 0x3e6c0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e6c4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3e6c8: bl 0x81850 <entry+0x47c80> @ imm = #0x43180
    c.bl(0x81850)?;
    // 0x3e6cc: sub r3, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(3)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e6d0: b 0x3e6dc <entry+0x4b0c>  @ imm = #0x4
    c.b(C::Al,0x3e6dc)?;
    // 0x3e6d4: rsb r0, r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(4)?,0)?)?;
    // 0x3e6d8: and r3, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e6dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3e6e0: str r3, [r10, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,68)?)?;
    // 0x3e6e4: mov r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,255)?)?;
    // 0x3e6e8: str r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,4)?)?;
    // 0x3e6ec: str r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,8)?)?;
    // 0x3e6f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3e6f4: orr r12, r12, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,3840)?)?;
    // 0x3e6f8: str r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,28)?)?;
    // 0x3e6fc: mla r5, r9, r3, r12
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(9)?,R::new(3)?,R::R12)?)?;
    // 0x3e700: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x3e704: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // 0x3e708: ldr r6, [pc, #0x3a8]        @ 0x4ee8 <entry+0x4ee8>
    c.literal_cond(C::Al,R::new(6)?,0x3eab8)?;
    // 0x3e70c: ldr r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,12)?)?;
    // 0x3e710: mla r0, r2, r1, r12
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R2,R::R1,R::R12)?)?;
    // 0x3e714: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // 0x3e718: and r5, r5, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3e71c: and r1, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3e720: mla r0, r7, r2, r12
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(7)?,R::R2,R::R12)?)?;
    // 0x3e724: ldr r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,16)?)?;
    // 0x3e728: and r2, r0, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3e72c: add r0, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e730: mla r3, r4, r7, r12
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(4)?,R::new(7)?,R::R12)?)?;
    // 0x3e734: and r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x3e738: b 0x3ea90 <entry+0x4ec0>  @ imm = #0x350
    c.b(C::Al,0x3ea90)?;
    // 0x3e73c: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x3e740: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e744: str r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,40)?)?;
    // 0x3e748: add r0, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e74c: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e750: sub r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,1)?)?;
    // 0x3e754: tst r0, r7
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e758: beq 0x3e770 <entry+0x4ba0>  @ imm = #0x10
    c.b(C::Eq,0x3e770)?;
    // 0x3e75c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e760: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3e764: bl 0x81850 <entry+0x47c80> @ imm = #0x430e4
    c.bl(0x81850)?;
    // 0x3e768: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e76c: b 0x3e778 <entry+0x4ba8>  @ imm = #0x4
    c.b(C::Al,0x3e778)?;
    // 0x3e770: rsb r0, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(7)?,0)?)?;
    // 0x3e774: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e778: str r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,44)?)?;
    // 0x3e77c: sub r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,1)?)?;
    // 0x3e780: add r5, r9, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(9)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e784: tst r0, r8
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e788: beq 0x3e7a0 <entry+0x4bd0>  @ imm = #0x10
    c.b(C::Eq,0x3e7a0)?;
    // 0x3e78c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e790: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x3e794: bl 0x81850 <entry+0x47c80> @ imm = #0x430b4
    c.bl(0x81850)?;
    // 0x3e798: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e79c: b 0x3e7a8 <entry+0x4bd8>  @ imm = #0x4
    c.b(C::Al,0x3e7a8)?;
    // 0x3e7a0: rsb r0, r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(8)?,0)?)?;
    // 0x3e7a4: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e7a8: mov r8, r6
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // 0x3e7ac: str r0, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,56)?)?;
    // 0x3e7b0: add r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,1)?)?;
    // 0x3e7b4: add r0, r4, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsr,1,false)?)?;
    // 0x3e7b8: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e7bc: sub r0, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,1)?)?;
    // 0x3e7c0: tst r0, r4
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3e7c4: beq 0x3e7dc <entry+0x4c0c>  @ imm = #0x10
    c.b(C::Eq,0x3e7dc)?;
    // 0x3e7c8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e7cc: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3e7d0: bl 0x81850 <entry+0x47c80> @ imm = #0x43078
    c.bl(0x81850)?;
    // 0x3e7d4: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e7d8: b 0x3e7e4 <entry+0x4c14>  @ imm = #0x4
    c.b(C::Al,0x3e7e4)?;
    // 0x3e7dc: rsb r0, r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(4)?,0)?)?;
    // 0x3e7e0: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e7e4: ldr r1, [pc, #0x2e4]        @ 0x4f00 <entry+0x4f00>
    c.literal_cond(C::Al,R::R1,0x3ead0)?;
    // 0x3e7e8: str r0, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,60)?)?;
    // 0x3e7ec: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3e7f0: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xfb64
    c.bl(0x2ec94)?;
    // 0x3e7f4: ldr r1, [pc, #0x2c8]        @ 0x4ef4 <entry+0x4ef4>
    c.literal_cond(C::Al,R::R1,0x3eac4)?;
    // 0x3e7f8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3e7fc: bl 0x2ec94 <entry+0xffffffffffff50c4> @ imm = #-0xfb70
    c.bl(0x2ec94)?;
    // 0x3e800: ldr r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,40)?)?;
    // 0x3e804: ldr r2, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,44)?)?;
    // 0x3e808: ldr r1, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,56)?)?;
    // 0x3e80c: ldr r7, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,60)?)?;
    // 0x3e810: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3e814: str r5, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,4)?)?;
    // 0x3e818: str r5, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,8)?)?;
    // 0x3e81c: str r5, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,28)?)?;
    // 0x3e820: mul r3, r7, r1
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::new(7)?,R::R1)?)?;
    // 0x3e824: mul r1, r2, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R2,R::R0)?)?;
    // 0x3e828: b 0x3ea88 <entry+0x4eb8>  @ imm = #0x258
    c.b(C::Al,0x3ea88)?;
    // 0x3e82c: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x3e830: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e834: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e838: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x3e83c: str r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,40)?)?;
    // 0x3e840: add r0, r9, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e844: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e848: sub r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,1)?)?;
    // 0x3e84c: tst r0, r7
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e850: beq 0x3e8a4 <entry+0x4cd4>  @ imm = #0x4c
    c.b(C::Eq,0x3e8a4)?;
    // 0x3e854: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e858: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3e85c: bl 0x81850 <entry+0x47c80> @ imm = #0x42fec
    c.bl(0x81850)?;
    // 0x3e860: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e864: b 0x3e8ac <entry+0x4cdc>  @ imm = #0x40
    c.b(C::Al,0x3e8ac)?;
    // 0x3e868: rsb r0, r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R1,0)?)?;
    // 0x3e86c: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e870: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e874: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x3e878: str r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,40)?)?;
    // 0x3e87c: add r0, r9, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e880: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e884: sub r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(7)?,1)?)?;
    // 0x3e888: tst r0, r7
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3e88c: beq 0x3e8ec <entry+0x4d1c>  @ imm = #0x58
    c.b(C::Eq,0x3e8ec)?;
    // 0x3e890: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e894: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3e898: bl 0x81850 <entry+0x47c80> @ imm = #0x42fb0
    c.bl(0x81850)?;
    // 0x3e89c: sub r7, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(7)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e8a0: b 0x3e8f4 <entry+0x4d24>  @ imm = #0x4c
    c.b(C::Al,0x3e8f4)?;
    // 0x3e8a4: rsb r0, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(7)?,0)?)?;
    // 0x3e8a8: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e8ac: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x3e8b0: mov r7, r6
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(6)?))?;
    // 0x3e8b4: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,24)?)?;
    // 0x3e8b8: str r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,44)?)?;
    // 0x3e8bc: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x3e8c0: str r0, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,56)?)?;
    // 0x3e8c4: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x3e8c8: add r5, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e8cc: sub r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,1)?)?;
    // 0x3e8d0: tst r0, r8
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e8d4: beq 0x3e91c <entry+0x4d4c>  @ imm = #0x40
    c.b(C::Eq,0x3e91c)?;
    // 0x3e8d8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e8dc: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x3e8e0: bl 0x81850 <entry+0x47c80> @ imm = #0x42f68
    c.bl(0x81850)?;
    // 0x3e8e4: sub r8, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(8)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e8e8: b 0x3e924 <entry+0x4d54>  @ imm = #0x34
    c.b(C::Al,0x3e924)?;
    // 0x3e8ec: rsb r0, r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(7)?,0)?)?;
    // 0x3e8f0: and r7, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(7)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e8f4: sub r0, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,1)?)?;
    // 0x3e8f8: add r5, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e8fc: str r7, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(10)?,44)?)?;
    // 0x3e900: tst r0, r8
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3e904: beq 0x3e960 <entry+0x4d90>  @ imm = #0x54
    c.b(C::Eq,0x3e960)?;
    // 0x3e908: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e90c: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x3e910: bl 0x81850 <entry+0x47c80> @ imm = #0x42f38
    c.bl(0x81850)?;
    // 0x3e914: sub r6, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e918: b 0x3e968 <entry+0x4d98>  @ imm = #0x48
    c.b(C::Al,0x3e968)?;
    // 0x3e91c: rsb r0, r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(8)?,0)?)?;
    // 0x3e920: and r8, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(8)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e924: add r0, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1)?)?;
    // 0x3e928: str r8, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(10)?,56)?)?;
    // 0x3e92c: mov r6, r9
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x3e930: add r1, r4, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R0,Shift::Lsr,1,false)?)?;
    // 0x3e934: lsr r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,1)?)?;
    // 0x3e938: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x3e93c: sub r0, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,1)?)?;
    // 0x3e940: sub r5, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R1,1)?)?;
    // 0x3e944: tst r0, r4
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3e948: beq 0x3e998 <entry+0x4dc8>  @ imm = #0x48
    c.b(C::Eq,0x3e998)?;
    // 0x3e94c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e950: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3e954: bl 0x81850 <entry+0x47c80> @ imm = #0x42ef4
    c.bl(0x81850)?;
    // 0x3e958: sub r4, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(4)?,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e95c: b 0x3e9a0 <entry+0x4dd0>  @ imm = #0x3c
    c.b(C::Al,0x3e9a0)?;
    // 0x3e960: rsb r0, r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(8)?,0)?)?;
    // 0x3e964: and r6, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e968: add r0, r9, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,1)?)?;
    // 0x3e96c: str r6, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(10)?,56)?)?;
    // 0x3e970: add r0, r4, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsr,1,false)?)?;
    // 0x3e974: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x3e978: sub r0, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(4)?,1)?)?;
    // 0x3e97c: tst r0, r4
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3e980: beq 0x3ea5c <entry+0x4e8c>  @ imm = #0xd4
    c.b(C::Eq,0x3ea5c)?;
    // 0x3e984: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3e988: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3e98c: bl 0x81850 <entry+0x47c80> @ imm = #0x42ebc
    c.bl(0x81850)?;
    // 0x3e990: sub r0, r5, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e994: b 0x3ea64 <entry+0x4e94>  @ imm = #0xc8
    c.b(C::Al,0x3ea64)?;
    // 0x3e998: rsb r0, r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(4)?,0)?)?;
    // 0x3e99c: and r4, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::new(4)?,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e9a0: str r4, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(10)?,60)?)?;
    // 0x3e9a4: add r0, r7, #47
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,47)?)?;
    // 0x3e9a8: mov lr, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,63)?)?;
    // 0x3e9ac: mov r12, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,15)?)?;
    // 0x3e9b0: ldr r1, [pc, #0xf8]         @ 0x4ee0 <entry+0x4ee0>
    c.literal_cond(C::Al,R::R1,0x3eab0)?;
    // 0x3e9b4: ldr r9, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,32)?)?;
    // 0x3e9b8: umull r2, r5, r0, r1
    c.emit(super::encode_long_multiply(C::Al,false,R::R2,R::new(5)?,R::R0,R::R1)?)?;
    // 0x3e9bc: add r2, lr, r5, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::LR,R::new(5)?,Shift::Lsr,5,false)?)?;
    // 0x3e9c0: add r5, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3)?)?;
    // 0x3e9c4: add r6, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(7)?,1)?)?;
    // 0x3e9c8: mov r7, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,23)?)?;
    // 0x3e9cc: add r7, r7, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsr,1,false)?)?;
    // 0x3e9d0: add r5, r12, r5, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R12,R::new(5)?,Shift::Lsr,2,false)?)?;
    // 0x3e9d4: and r2, r2, r9
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x3e9d8: umull r6, r0, r7, r1
    c.emit(super::encode_long_multiply(C::Al,false,R::new(6)?,R::R0,R::new(7)?,R::R1)?)?;
    // 0x3e9dc: add r1, r9, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,268435456)?)?;
    // 0x3e9e0: bic r3, r5, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(5)?,2147483663)?)?;
    // 0x3e9e4: str r2, [r10, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,48)?)?;
    // 0x3e9e8: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x3e9ec: add r0, lr, r0, lsr #4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::R0,Shift::Lsr,4,false)?)?;
    // 0x3e9f0: str r3, [r10, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,52)?)?;
    // 0x3e9f4: orr r5, r5, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,3840)?)?;
    // 0x3e9f8: and lr, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::LR,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3e9fc: str lr, [r10, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(10)?,64)?)?;
    // 0x3ea00: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x3ea04: add r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3)?)?;
    // 0x3ea08: add r0, r12, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R12,R::R0,Shift::Lsr,2,false)?)?;
    // 0x3ea0c: bic r12, r0, #-2147483633
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R12,R::R0,2147483663)?)?;
    // 0x3ea10: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3ea14: str r12, [r10, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(10)?,68)?)?;
    // 0x3ea18: str r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,4)?)?;
    // 0x3ea1c: str r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,8)?)?;
    // 0x3ea20: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3ea24: str r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,28)?)?;
    // 0x3ea28: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x3ea2c: ldr r6, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,24)?)?;
    // 0x3ea30: ldr r7, [pc, #0x80]         @ 0x4ee8 <entry+0x4ee8>
    c.literal_cond(C::Al,R::new(7)?,0x3eab8)?;
    // 0x3ea34: mla r0, r6, r1, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(6)?,R::R1,R::new(5)?)?)?;
    // 0x3ea38: mla r6, lr, r12, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::LR,R::R12,R::new(5)?)?)?;
    // 0x3ea3c: and r1, r0, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3ea40: mla r0, r3, r2, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(3)?,R::R2,R::new(5)?)?)?;
    // 0x3ea44: mla r3, r4, r8, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::new(4)?,R::new(8)?,R::new(5)?)?)?;
    // 0x3ea48: and r5, r6, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3ea4c: and r2, r0, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3ea50: and r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3ea54: add r0, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3ea58: b 0x3ea90 <entry+0x4ec0>  @ imm = #0x30
    c.b(C::Al,0x3ea90)?;
    // 0x3ea5c: rsb r0, r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(4)?,0)?)?;
    // 0x3ea60: and r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3ea64: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3ea68: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3ea6c: str r0, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,60)?)?;
    // 0x3ea70: mul r3, r6, r0
    c.emit(arm32::multiply_register(C::Al,R::new(3)?,R::new(6)?,R::R0)?)?;
    // 0x3ea74: str r5, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,4)?)?;
    // 0x3ea78: str r5, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,8)?)?;
    // 0x3ea7c: str r1, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,28)?)?;
    // 0x3ea80: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x3ea84: mul r1, r7, r0
    c.emit(arm32::multiply_register(C::Al,R::R1,R::new(7)?,R::R0)?)?;
    // 0x3ea88: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3ea8c: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x3ea90: str r1, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,12)?)?;
    // 0x3ea94: str r3, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,16)?)?;
    // 0x3ea98: str r2, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,20)?)?;
    // 0x3ea9c: str r5, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,24)?)?;
    // 0x3eaa0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x3eaa4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;

    c.check(0x3eaa8)?;
    for word in GEOMETRY_MASKS_AND_MESSAGES {c.emit(word)?;}
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{calculate_codec_register_matrix(c)}

#[cfg(test)] mod tests {
 use super::*;
 #[test]fn preserves_codec_register_constants_and_real_return(){
  let mut c=Code::new(START);emit(&mut c).unwrap();
  assert_eq!(c.at(),END);
  assert_eq!(c.bytes.len(),END-START);
  for (i,word) in GEOMETRY_MASKS_AND_MESSAGES.iter().enumerate() {
    let o=0x3eaa8-START+4*i;
    assert_eq!(u32::from_le_bytes(c.bytes[o..o+4].try_into().unwrap()),*word);
  }
 }
}
