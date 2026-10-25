// SPDX-License-Identifier: GPL-3.0-only
//! Codec frame submission, completion and resource cleanup callbacks.
//! Includes a two-instruction program-register shift (A32 register shift) and
//! separate constant words for input-format and diagnostics. The executable
//! bytes are generated from checked typed ARM operands, not vendor opcodes.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2a878;
const END:usize=0x2aad8;

/// Find active buffer descriptor, bound the stream length to 32 bytes and notify the shared codec policy routine.
fn submit_pending_codec_frame(c:&mut Code)->Result<(),String>{
    c.check(0x2a878)?;
    // 0x2a878: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2a87c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x2a880: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x2a884: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2a888: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2a88c: ldr r2, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,0)?)?;
    // 0x2a890: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2a894: add r6, r5, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(5)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x2a898: ldr r3, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,48)?)?;
    // 0x2a89c: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x2a8a0: beq 0x2a8b0 <entry+0x4b0>   @ imm = #0x8
    c.b(C::Eq,0x2a8b0)?;
    // 0x2a8a4: ldr r4, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(3)?,8)?)?;
    // 0x2a8a8: cmp r0, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2a8ac: beq 0x2a8c0 <entry+0x4c0>   @ imm = #0xc
    c.b(C::Eq,0x2a8c0)?;
    // 0x2a8b0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x2a8b4: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x2a8b8: blo 0x2a894 <entry+0x494>   @ imm = #-0x2c
    c.b(C::Cc,0x2a894)?;
    // 0x2a8bc: b 0x2a8f8 <entry+0x4f8>   @ imm = #0x34
    c.b(C::Al,0x2a8f8)?;
    // 0x2a8c0: ldr r0, [r3, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,36)?)?;
    // 0x2a8c4: add r0, r3, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x2a8c8: ldr r0, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,28)?)?;
    // 0x2a8cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a8d0: beq 0x2a8f8 <entry+0x4f8>   @ imm = #0x20
    c.b(C::Eq,0x2a8f8)?;
    // 0x2a8d4: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x2a8d8: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2a8dc: movhs r2, #32
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x2a8e0: bl 0x471c <entry+0xfffffffffffda31c> @ imm = #-0x261cc
    c.bl(0x471c)?;
    // 0x2a8e4: ldr r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // 0x2a8e8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2a8ec: ldr r1, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,36)?)?;
    // 0x2a8f0: bic r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2a8f4: str r1, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,36)?)?;
    // 0x2a8f8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2a8fc: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2a900: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2a904: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x2a908: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2a90c: b 0x2a198 <entry+0xfffffffffffffd98> @ imm = #-0x77c
    c.b(C::Al,0x2a198)?;
    // 0x2a910: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2a914: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x2a918: sub sp, sp, #104
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,104)?)?;
    // 0x2a91c: mov r8, sp
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::SP))?;
    // 0x2a920: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x2a924: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x2a928: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x2a92c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2a930: mov r2, #80
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,80)?)?;
    // 0x2a934: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x2a938: add r0, r8, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,24)?)?;
    // 0x2a93c: bl 0x31ec <entry+0xfffffffffffd8dec> @ imm = #-0x27758
    c.bl(0x31ec)?;
    // 0x2a940: mov r0, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // 0x2a944: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x2a948: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2a94c: ldr r0, [pc, #0x38]         @ 0x58c <entry+0x58c>
    c.literal(R::R0,0x2a98c)?;
    // 0x2a950: stmib sp, {r0, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(6)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2a954: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    // 0x2a958: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2a95c: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x2a960: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x2a964: ldr r0, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    // 0x2a968: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x2a96c: ldr r0, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,16)?)?;
    // 0x2a970: str r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,96)?)?;
    // 0x2a974: ldr r0, [r11, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,20)?)?;
    // 0x2a978: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2a97c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2a980: bl 0x2a878 <entry+0x478>   @ imm = #-0x110
    c.bl(0x2a878)?;
    // 0x2a984: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x2a988: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2a98c)?;
    const DATA:[u32;1]=[0x00221008];
    for x in DATA {c.emit(x)?;}
    c.check(0x2a990)
}

/// Dispose of frame buffer resources and deliver callback/error diagnostics to the controller.
fn release_codec_capture_resources(c:&mut Code)->Result<(),String>{
    c.check(0x2a990)?;
    // 0x2a990: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2a994: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2a998: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x2a99c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2a9a0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a9a4: add r12, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::SP,8)?)?;
    // 0x2a9a8: ldr r5, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,16)?)?;
    // 0x2a9ac: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // 0x2a9b0: stm r12, {r0, r1, r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2a9b4: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    // 0x2a9b8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2a9bc: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2a9c0: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2a9c4: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2a9c8: beq 0x2a9e8 <entry+0x5e8>   @ imm = #0x18
    c.b(C::Eq,0x2a9e8)?;
    // 0x2a9cc: ldr r1, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,12)?)?;
    // 0x2a9d0: add r0, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,4)?)?;
    // 0x2a9d4: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x2a9d8: add r0, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,24)?)?;
    // 0x2a9dc: bl 0x471c <entry+0xfffffffffffda31c> @ imm = #-0x262c8
    c.bl(0x471c)?;
    // 0x2a9e0: add r0, r5, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,24)?)?;
    // 0x2a9e4: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2a9e8: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x2a9ec: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // 0x2a9f0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2a9f4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2a9f8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2a9fc: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2aa00: bl 0x2a198 <entry+0xfffffffffffffd98> @ imm = #-0x870
    c.bl(0x2a198)?;
    // 0x2aa04: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x2aa08: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2aa0c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x2aa10: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x2aa14: lsl r2, r3, r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(3)?,Shift::Lsl,R::R2,false)?)?;
    // 0x2aa18: stm r1, {r0, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2aa1c: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x2aa20: b 0x293c <entry+0xfffffffffffd853c> @ imm = #-0x280ec
    c.b(C::Al,0x293c)?;
    // 0x2aa24: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2aa28: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2aa2c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2aa30: ldr r0, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,120)?)?;
    // 0x2aa34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2aa38: beq 0x2aa48 <entry+0x648>   @ imm = #0x8
    c.b(C::Eq,0x2aa48)?;
    // 0x2aa3c: bl 0x2f1c <entry+0xfffffffffffd8b1c> @ imm = #-0x27b28
    c.bl(0x2f1c)?;
    // 0x2aa40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2aa44: str r0, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,120)?)?;
    // 0x2aa48: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2aa4c: bl 0x2b18 <entry+0xfffffffffffd8718> @ imm = #-0x27f3c
    c.bl(0x2b18)?;
    // 0x2aa50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2aa54: beq 0x2aa90 <entry+0x690>   @ imm = #0x34
    c.b(C::Eq,0x2aa90)?;
    // 0x2aa58: ldr r1, [pc, #0x64]         @ 0x6c4 <entry+0x6c4>
    c.literal(R::R1,0x2aac4)?;
    // 0x2aa5c: ldr r2, [pc, #0x64]         @ 0x6c8 <entry+0x6c8>
    c.literal(R::R2,0x2aac8)?;
    // 0x2aa60: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // 0x2aa64: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2aa68: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x2aa6c: bl 0x2ec94 <entry+0x4894>  @ imm = #0x4220
    c.bl(0x2ec94)?;
    // 0x2aa70: ldr r1, [pc, #0x54]         @ 0x6cc <entry+0x6cc>
    c.literal(R::R1,0x2aacc)?;
    // 0x2aa74: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2aa78: bl 0x2ec94 <entry+0x4894>  @ imm = #0x4214
    c.bl(0x2ec94)?;
    // 0x2aa7c: ldr r2, [pc, #0x4c]         @ 0x6d0 <entry+0x6d0>
    c.literal(R::R2,0x2aad0)?;
    // 0x2aa80: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x2aa84: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2aa88: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2aa8c: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x292e4
    c.bl(0x17b0)?;
    // 0x2aa90: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,40)?)?;
    // 0x2aa94: bl 0x2bcc <entry+0xfffffffffffd87cc> @ imm = #-0x27ed0
    c.bl(0x2bcc)?;
    // 0x2aa98: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x2aa9c: bl 0x2bcc <entry+0xfffffffffffd87cc> @ imm = #-0x27ed8
    c.bl(0x2bcc)?;
    // 0x2aaa0: add r0, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,4)?)?;
    // 0x2aaa4: bl 0x2ce0 <entry+0xfffffffffffd88e0> @ imm = #-0x27dcc
    c.bl(0x2ce0)?;
    // 0x2aaa8: bl 0x31db8 <entry+0x79b8>  @ imm = #0x7308
    c.bl(0x31db8)?;
    // 0x2aaac: ldr r0, [pc, #0x20]         @ 0x6d4 <entry+0x6d4>
    c.literal(R::R0,0x2aad4)?;
    // 0x2aab0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2aab4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2aab8: popeq {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2aabc: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2aac0: b 0x25888 <entry+0xffffffffffffb488> @ imm = #-0x5240
    c.b(C::Al,0x25888)?;
    c.check(0x2aac4)?;
    const DATA:[u32;5]=[0x000e8915,0x000e893c,0x000e2788,0x000e884d,0x0010306c];
    for x in DATA {c.emit(x)?;}
    c.check(0x2aad8)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    submit_pending_codec_frame(c)?;
    release_codec_capture_resources(c)?;
    c.check(END)
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn submit_and_release_have_precise_data_fields(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(word(0x2a98c),0x0022_1008);
        assert_eq!(word(0x2aacc),0x000e_2788);
        assert_eq!(word(0x2aad4),0x0010_306c);
    }
}
