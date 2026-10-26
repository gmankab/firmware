// SPDX-License-Identifier: GPL-3.0-only
//! Publish and commit controller stream fields, check input frame state and
//! call the optional frame-event callback. The trailing controller constants
//! are literal data and must not be confused with executable ARM instructions.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x34ca0;
const END:usize=0x34fac;
const CONTROLLER_UPDATE_LITERALS:[u32;7]=[
  0x00121002,0x000ebf55,0x000ec01f,0x000e2788,
  0x000ebe82,0x000ec034,0x000ec05d,
];
pub(super) fn emit(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x34ca0: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34ca4: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x34ca8: sub sp, sp, #772
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,772)?)?;
    // 0x34cac: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x34cb0: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x34cb4: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x34cb8: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x34cbc: ldr r5, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,360)?)?;
    // 0x34cc0: mov r9, r5
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(5)?))?;
    // 0x34cc4: ldr r0, [r9], #92
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(9)?,92)?)?;
    // 0x34cc8: ldr r10, [pc, #0x2c0]       @ 0x3210 <entry+0x3210>
    c.literal(R::new(10)?,0x34f90)?;
    // 0x34ccc: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x34cd0: bne 0x34edc <entry+0x315c>  @ imm = #0x204
    c.b(C::Ne,0x34edc)?;
    // 0x34cd4: mov r0, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1232)?)?;
    // 0x34cd8: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,4)?)?;
    // 0x34cdc: mov r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,9)?)?;
    // 0x34ce0: mov r4, r5
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // 0x34ce4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x34ce8: str r1, [r4], r0
    c.emit(arm32::transfer_word_postindexed_register(C::Al,false,R::R1,R::new(4)?,R::R0)?)?;
    // 0x34cec: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34cf0: bl 0x54ccc <entry+0x22f4c> @ imm = #0x1ffd4
    c.bl(0x54ccc)?;
    // 0x34cf4: sub r7, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,56)?)?;
    // 0x34cf8: sub r6, r11, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,60)?)?;
    // 0x34cfc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x34d00: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x34d04: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x34d08: bl 0x356e8 <entry+0x3968>  @ imm = #0x9d8
    c.bl(0x356e8)?;
    // 0x34d0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34d10: bne 0x34cfc <entry+0x2f7c>  @ imm = #-0x1c
    c.b(C::Ne,0x34cfc)?;
    // 0x34d14: add r0, r5, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1632)?)?;
    // 0x34d18: sub r1, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,56)?)?;
    // 0x34d1c: sub r2, r11, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,60)?)?;
    // 0x34d20: add r7, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,4096)?)?;
    // 0x34d24: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x34d28: bl 0x356e8 <entry+0x3968>  @ imm = #0x9b8
    c.bl(0x356e8)?;
    // 0x34d2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34d30: beq 0x34e3c <entry+0x30bc>  @ imm = #0x104
    c.b(C::Eq,0x34e3c)?;
    // 0x34d34: add r0, sp, #228
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,228)?)?;
    // 0x34d38: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x34d3c: sub r8, r11, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,60)?)?;
    // 0x34d40: add r6, r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,40)?)?;
    // 0x34d44: ldr r0, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-60)?)?;
    // 0x34d48: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x34d4c: add r2, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,8)?)?;
    // 0x34d50: str r0, [sp, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,232)?)?;
    // 0x34d54: mov r0, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,76)?)?;
    // 0x34d58: str r0, [sp, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,228)?)?;
    // 0x34d5c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x34d60: bl 0x39970 <entry+0x7bf0>  @ imm = #0x4c08
    c.bl(0x39970)?;
    // 0x34d64: mov r1, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,68)?)?;
    // 0x34d68: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34d6c: str r1, [sp, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,236)?)?;
    // 0x34d70: str r10, [sp, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,240)?)?;
    // 0x34d74: ldr r1, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    // 0x34d78: str r1, [sp, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,244)?)?;
    // 0x34d7c: ldr r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,60)?)?;
    // 0x34d80: str r4, [sp, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,248)?)?;
    // 0x34d84: str r4, [sp, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,252)?)?;
    // 0x34d88: str r4, [sp, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,256)?)?;
    // 0x34d8c: str r4, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,0)?)?;
    // 0x34d90: str r4, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,4)?)?;
    // 0x34d94: str r4, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,8)?)?;
    // 0x34d98: str r4, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,12)?)?;
    // 0x34d9c: movne r1, r4
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(4)?))?;
    // 0x34da0: str r1, [sp, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,284)?)?;
    // 0x34da4: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x34da8: movne r1, r4
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(4)?))?;
    // 0x34dac: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34db0: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x34db4: str r1, [sp, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,288)?)?;
    // 0x34db8: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,56)?)?;
    // 0x34dbc: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x34dc0: movne r1, r4
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(4)?))?;
    // 0x34dc4: str r1, [sp, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,292)?)?;
    // 0x34dc8: str r0, [sp, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,300)?)?;
    // 0x34dcc: ldr r0, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-52)?)?;
    // 0x34dd0: add r1, sp, #228
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,228)?)?;
    // 0x34dd4: str r0, [sp, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,260)?)?;
    // 0x34dd8: str r0, [sp, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,296)?)?;
    // 0x34ddc: ldr r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,56)?)?;
    // 0x34de0: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x34de4: bl 0x80928 <entry+0x4eba8> @ imm = #0x4bb3c
    c.bl(0x80928)?;
    // 0x34de8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34dec: beq 0x34e24 <entry+0x30a4>  @ imm = #0x30
    c.b(C::Eq,0x34e24)?;
    // 0x34df0: ldr r1, [pc, #0x19c]        @ 0x3214 <entry+0x3214>
    c.literal(R::R1,0x34f94)?;
    // 0x34df4: ldr r2, [pc, #0x19c]        @ 0x3218 <entry+0x3218>
    c.literal(R::R2,0x34f98)?;
    // 0x34df8: mov r3, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,127)?)?;
    // 0x34dfc: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34e00: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x34e04: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x6178
    c.bl(0x2ec94)?;
    // 0x34e08: ldr r1, [pc, #0x18c]        @ 0x321c <entry+0x321c>
    c.literal(R::R1,0x34f9c)?;
    // 0x34e0c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34e10: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x6184
    c.bl(0x2ec94)?;
    // 0x34e14: ldr r2, [pc, #0x184]        @ 0x3220 <entry+0x3220>
    c.literal(R::R2,0x34fa0)?;
    // 0x34e18: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34e1c: mov r1, #640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,640)?)?;
    // 0x34e20: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x33678
    c.bl(0x17b0)?;
    // 0x34e24: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x34e28: sub r1, r11, #56
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,56)?)?;
    // 0x34e2c: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x34e30: bl 0x356e8 <entry+0x3968>  @ imm = #0x8b0
    c.bl(0x356e8)?;
    // 0x34e34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34e38: bne 0x34d44 <entry+0x2fc4>  @ imm = #-0xfc
    c.b(C::Ne,0x34d44)?;
    // 0x34e3c: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,4)?)?;
    // 0x34e40: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34e44: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34e48: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34e4c: str r0, [r5, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,284)?)?;
    // 0x34e50: ldr r1, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,16)?)?;
    // 0x34e54: str r1, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-44)?)?;
    // 0x34e58: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x34e5c: add r0, r4, #144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,144)?)?;
    // 0x34e60: sub r1, r11, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,44)?)?;
    // 0x34e64: bl 0x80928 <entry+0x4eba8> @ imm = #0x4babc
    c.bl(0x80928)?;
    // 0x34e68: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34e6c: beq 0x34e94 <entry+0x3114>  @ imm = #0x20
    c.b(C::Eq,0x34e94)?;
    // 0x34e70: ldr r1, [pc, #0x12c]        @ 0x3224 <entry+0x3224>
    c.literal(R::R1,0x34fa4)?;
    // 0x34e74: ldr r2, [pc, #0x11c]        @ 0x3218 <entry+0x3218>
    c.literal(R::R2,0x34f98)?;
    // 0x34e78: mov r3, #139
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,139)?)?;
    // 0x34e7c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34e80: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x34e84: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x61f8
    c.bl(0x2ec94)?;
    // 0x34e88: ldr r1, [pc, #0x10c]        @ 0x321c <entry+0x321c>
    c.literal(R::R1,0x34f9c)?;
    // 0x34e8c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34e90: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x6204
    c.bl(0x2ec94)?;
    // 0x34e94: ldrb r0, [r5, #0x3de]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,990)?)?;
    // 0x34e98: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34e9c: bne 0x34f5c <entry+0x31dc>  @ imm = #0xb8
    c.b(C::Ne,0x34f5c)?;
    // 0x34ea0: add r0, r4, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,88)?)?;
    // 0x34ea4: sub r1, r11, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,44)?)?;
    // 0x34ea8: bl 0x80928 <entry+0x4eba8> @ imm = #0x4ba78
    c.bl(0x80928)?;
    // 0x34eac: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34eb0: beq 0x34f5c <entry+0x31dc>  @ imm = #0xa4
    c.b(C::Eq,0x34f5c)?;
    // 0x34eb4: ldr r1, [pc, #0xec]         @ 0x3228 <entry+0x3228>
    c.literal(R::R1,0x34fa8)?;
    // 0x34eb8: ldr r2, [pc, #0xd8]         @ 0x3218 <entry+0x3218>
    c.literal(R::R2,0x34f98)?;
    // 0x34ebc: mov r3, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,145)?)?;
    // 0x34ec0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34ec4: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x34ec8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x623c
    c.bl(0x2ec94)?;
    // 0x34ecc: ldr r1, [pc, #0xc8]         @ 0x321c <entry+0x321c>
    c.literal(R::R1,0x34f9c)?;
    // 0x34ed0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34ed4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x6248
    c.bl(0x2ec94)?;
    // 0x34ed8: b 0x34f5c <entry+0x31dc>  @ imm = #0x7c
    c.b(C::Al,0x34f5c)?;
    // 0x34edc: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34ee0: add r1, sp, #228
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,228)?)?;
    // 0x34ee4: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34ee8: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34eec: str r0, [sp, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,232)?)?;
    // 0x34ef0: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x34ef4: str r0, [sp, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,228)?)?;
    // 0x34ef8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34efc: str r0, [sp, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,248)?)?;
    // 0x34f00: add r0, r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,1)?)?;
    // 0x34f04: str r0, [sp, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,240)?)?;
    // 0x34f08: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x34f0c: str r0, [sp, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,244)?)?;
    // 0x34f10: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34f14: str r0, [sp, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,236)?)?;
    // 0x34f18: add r0, r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,200)?)?;
    // 0x34f1c: bl 0x80928 <entry+0x4eba8> @ imm = #0x4ba04
    c.bl(0x80928)?;
    // 0x34f20: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34f24: beq 0x34f5c <entry+0x31dc>  @ imm = #0x30
    c.b(C::Eq,0x34f5c)?;
    // 0x34f28: ldr r1, [pc, #0x64]         @ 0x3214 <entry+0x3214>
    c.literal(R::R1,0x34f94)?;
    // 0x34f2c: ldr r2, [pc, #0x64]         @ 0x3218 <entry+0x3218>
    c.literal(R::R2,0x34f98)?;
    // 0x34f30: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34f34: mov r3, #676
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,676)?)?;
    // 0x34f38: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x62ac
    c.bl(0x2ec94)?;
    // 0x34f3c: ldr r1, [pc, #0x58]         @ 0x321c <entry+0x321c>
    c.literal(R::R1,0x34f9c)?;
    // 0x34f40: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34f44: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x62b8
    c.bl(0x2ec94)?;
    // 0x34f48: ldr r2, [pc, #0x50]         @ 0x3220 <entry+0x3220>
    c.literal(R::R2,0x34fa0)?;
    // 0x34f4c: mov r1, #165
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,165)?)?;
    // 0x34f50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34f54: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x34f58: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x337b0
    c.bl(0x17b0)?;
    // 0x34f5c: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34f60: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34f64: ldr r0, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34f68: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x34f6c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x34f70: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x34f74: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34f78: ldrbne r1, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::R0,6)?)?;
    // 0x34f7c: orrne r1, r1, #4
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::R1,4)?)?;
    // 0x34f80: strbne r1, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::R0,6)?)?;
    // 0x34f84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x34f88: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x34f8c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x34f90)?;
 for word in CONTROLLER_UPDATE_LITERALS {c.emit(word)?;}
 c.check(END)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn codec_output_commit_literal_table_is_exact(){
  assert_eq!(END-START,188*4+7*4);
  let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
  assert_eq!(CONTROLLER_UPDATE_LITERALS[4],0x000ebe82);
 }
}
