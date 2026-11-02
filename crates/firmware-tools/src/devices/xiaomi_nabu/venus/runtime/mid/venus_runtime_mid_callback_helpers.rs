// SPDX-License-Identifier: GPL-3.0-only
//! Venus stream descriptor callbacks: drain two request queues, invoke
//! optional completion callbacks, validate codec packet-class words, and
//! report failures through the platform's diagnostic API.
//!
//! Each executable instruction is encoded from named typed A32 operands;
//! controller-class IDs and diagnostic strings in the literal pools are
//! represented separately, never disassembled as executable instructions.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x33abc;
const END:usize=0x33e10;
const CODEC_CALLBACK_TYPES:[u32;6]=[0x00111002,0x00111001,0x000eb8ce,0x000e2788,0x000eb8ab,0x000eb8f1];
const STREAM_PRIMARY_ERRORS:[u32;2]=[0x000eb90b,0x000e2788];
const STREAM_SECONDARY_ERRORS:[u32;2]=[0x000eb925,0x000e2788];
const STREAM_ALTERNATE_ERRORS:[u32;2]=[0x000eb941,0x000e2788];
const IRQ_STATE_ERRORS:[u32;2]=[0x000eb941,0x000e2788];
const STREAM_EXTENDED_ERRORS:[u32;2]=[0x000eb960,0x000e2788];

fn drain_active_callback_queues(c:&mut Code)->Result<(),String>{
    c.check(0x33abc)?;
    // 0x33abc: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x33ac0: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x33ac4: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x33ac8: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x33acc: add r9, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::SP,4)?)?;
    // 0x33ad0: add r4, r8, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,360)?)?;
    // 0x33ad4: add r5, r8, #144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,144)?)?;
    // 0x33ad8: ldr r0, [r8, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,316)?)?;
    // 0x33adc: bl 0x294c <entry+0xfffffffffffd0bcc> @ imm = #-0x31198
    c.bl(0x294c)?;
    // 0x33ae0: ldr r0, [r8, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,316)?)?;
    // 0x33ae4: bl 0x293c <entry+0xfffffffffffd0bbc> @ imm = #-0x311b0
    c.bl(0x293c)?;
    // 0x33ae8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x33aec: bl 0x80b90 <entry+0x4ee10> @ imm = #0x4d09c
    c.bl(0x80b90)?;
    // 0x33af0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x33af4: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x33af8: beq 0x33ad8 <entry+0x1d58>  @ imm = #-0x28
    c.b(C::Eq,0x33ad8)?;
    // 0x33afc: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x33b00: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x33b04: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33b08: bl 0x80994 <entry+0x4ec14> @ imm = #0x4ce84
    c.bl(0x80994)?;
    // 0x33b0c: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x33b10: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x33b14: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33b18: ldr r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // 0x33b1c: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x33b20: beq 0x33b50 <entry+0x1dd0>  @ imm = #0x28
    c.b(C::Eq,0x33b50)?;
    // 0x33b24: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x33b28: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // 0x33b2c: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x30ab0
    c.bl(0x3084)?;
    // 0x33b30: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x33b34: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33b38: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33b3c: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33b40: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x33b44: ldr r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,20)?)?;
    // 0x33b48: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x33b4c: blx r2
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // 0x33b50: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x33b54: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x33b58: bne 0x33b00 <entry+0x1d80>  @ imm = #-0x60
    c.b(C::Ne,0x33b00)?;
    // 0x33b5c: b 0x33ad8 <entry+0x1d58>  @ imm = #-0x8c
    c.b(C::Al,0x33ad8)?;
    // 0x33b60: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x33b64: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x33b68: sub sp, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,16)?)?;
    // 0x33b6c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x33b70: add r6, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::SP,4)?)?;
    // 0x33b74: add r7, r4, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,360)?)?;
    // 0x33b78: add r5, r4, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,88)?)?;
    // 0x33b7c: ldr r0, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,312)?)?;
    // 0x33b80: bl 0x294c <entry+0xfffffffffffd0bcc> @ imm = #-0x3123c
    c.bl(0x294c)?;
    // 0x33b84: ldr r0, [r4, #0x138]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,312)?)?;
    // 0x33b88: bl 0x293c <entry+0xfffffffffffd0bbc> @ imm = #-0x31254
    c.bl(0x293c)?;
    // 0x33b8c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x33b90: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x33b94: bl 0x80994 <entry+0x4ec14> @ imm = #0x4cdf8
    c.bl(0x80994)?;
    // 0x33b98: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x33b9c: bne 0x33b7c <entry+0x1dfc>  @ imm = #-0x28
    c.b(C::Ne,0x33b7c)?;
    // 0x33ba0: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x33ba4: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x33ba8: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // 0x33bac: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33bb0: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x33bb4: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x30b38
    c.bl(0x3084)?;
    // 0x33bb8: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x33bbc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x33bc0: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33bc4: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33bc8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x33bcc: ldr r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,16)?)?;
    // 0x33bd0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x33bd4: blx r2
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // 0x33bd8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x33bdc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x33be0: bl 0x80994 <entry+0x4ec14> @ imm = #0x4cdac
    c.bl(0x80994)?;
    // 0x33be4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x33be8: beq 0x33ba0 <entry+0x1e20>  @ imm = #-0x50
    c.b(C::Eq,0x33ba0)?;
    // 0x33bec: b 0x33b7c <entry+0x1dfc>  @ imm = #-0x78
    c.b(C::Al,0x33b7c)?;
    // 0x33bf0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x33bf4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x33bf8: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x33bfc: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x33c00: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x33c04: ldr r1, [pc, #0x84]         @ 0x1f10 <entry+0x1f10>
    c.literal(R::R1,0x33c90)?;
    // 0x33c08: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x33c0c: beq 0x33c2c <entry+0x1eac>  @ imm = #0x18
    c.b(C::Eq,0x33c2c)?;
    // 0x33c10: ldr r1, [pc, #0x7c]         @ 0x1f14 <entry+0x1f14>
    c.literal(R::R1,0x33c94)?;
    // 0x33c14: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x33c18: bne 0x33c54 <entry+0x1ed4>  @ imm = #0x34
    c.b(C::Ne,0x33c54)?;
    // 0x33c1c: ldr r6, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,8)?)?;
    // 0x33c20: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xd4d4
    c.bl(0x26754)?;
    // 0x33c24: ldr r1, [pc, #0x74]         @ 0x1f20 <entry+0x1f20>
    c.literal(R::R1,0x33ca0)?;
    // 0x33c28: b 0x33c38 <entry+0x1eb8>  @ imm = #0x8
    c.b(C::Al,0x33c38)?;
    // 0x33c2c: ldr r6, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,8)?)?;
    // 0x33c30: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xd4e4
    c.bl(0x26754)?;
    // 0x33c34: ldr r1, [pc, #0x5c]         @ 0x1f18 <entry+0x1f18>
    c.literal(R::R1,0x33c98)?;
    // 0x33c38: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x33c3c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x33c40: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x33c44: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4fb8
    c.bl(0x2ec94)?;
    // 0x33c48: ldr r1, [pc, #0x4c]         @ 0x1f1c <entry+0x1f1c>
    c.literal(R::R1,0x33c9c)?;
    // 0x33c4c: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x33c50: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4fc4
    c.bl(0x2ec94)?;
    // 0x33c54: add r0, r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,32)?)?;
    // 0x33c58: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x33c5c: bl 0x80928 <entry+0x4eba8> @ imm = #0x4ccc4
    c.bl(0x80928)?;
    // 0x33c60: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x33c64: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33c68: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x33c6c: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33c70: ldr r1, [pc, #0x2c]         @ 0x1f24 <entry+0x1f24>
    c.literal(R::R1,0x33ca4)?;
    // 0x33c74: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33c78: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4fec
    c.bl(0x2ec94)?;
    // 0x33c7c: ldr r1, [pc, #0x18]         @ 0x1f1c <entry+0x1f1c>
    c.literal(R::R1,0x33c9c)?;
    // 0x33c80: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33c84: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4ff8
    c.bl(0x2ec94)?;
    // 0x33c88: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33c8c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x33c90)?;
    for word in CODEC_CALLBACK_TYPES { c.emit(word)?; }
    c.check(0x33ca8)
}

fn validate_primary_stream(c:&mut Code)->Result<(),String>{
    c.check(0x33ca8)?;
    // 0x33ca8: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x33cac: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x33cb0: add r0, r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,88)?)?;
    // 0x33cb4: bl 0x80928 <entry+0x4eba8> @ imm = #0x4cc6c
    c.bl(0x80928)?;
    // 0x33cb8: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x33cbc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33cc0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x33cc4: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33cc8: ldr r1, [pc, #0x18]         @ 0x1f68 <entry+0x1f68>
    c.literal(R::R1,0x33ce8)?;
    // 0x33ccc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33cd0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5044
    c.bl(0x2ec94)?;
    // 0x33cd4: ldr r1, [pc, #0x10]         @ 0x1f6c <entry+0x1f6c>
    c.literal(R::R1,0x33cec)?;
    // 0x33cd8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33cdc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5050
    c.bl(0x2ec94)?;
    // 0x33ce0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33ce4: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x33ce8)?;
    for word in STREAM_PRIMARY_ERRORS { c.emit(word)?; }
    c.check(0x33cf0)
}

fn validate_secondary_stream(c:&mut Code)->Result<(),String>{
    c.check(0x33cf0)?;
    // 0x33cf0: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x33cf4: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x33cf8: add r0, r0, #144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,144)?)?;
    // 0x33cfc: bl 0x80928 <entry+0x4eba8> @ imm = #0x4cc24
    c.bl(0x80928)?;
    // 0x33d00: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x33d04: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33d08: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x33d0c: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33d10: ldr r1, [pc, #0x18]         @ 0x1fb0 <entry+0x1fb0>
    c.literal(R::R1,0x33d30)?;
    // 0x33d14: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33d18: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x508c
    c.bl(0x2ec94)?;
    // 0x33d1c: ldr r1, [pc, #0x10]         @ 0x1fb4 <entry+0x1fb4>
    c.literal(R::R1,0x33d34)?;
    // 0x33d20: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33d24: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5098
    c.bl(0x2ec94)?;
    // 0x33d28: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33d2c: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x33d30)?;
    for word in STREAM_SECONDARY_ERRORS { c.emit(word)?; }
    c.check(0x33d38)
}

fn validate_alternate_stream(c:&mut Code)->Result<(),String>{
    c.check(0x33d38)?;
    // 0x33d38: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x33d3c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x33d40: add r0, r0, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,200)?)?;
    // 0x33d44: bl 0x80928 <entry+0x4eba8> @ imm = #0x4cbdc
    c.bl(0x80928)?;
    // 0x33d48: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x33d4c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33d50: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x33d54: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33d58: ldr r1, [pc, #0x18]         @ 0x1ff8 <entry+0x1ff8>
    c.literal(R::R1,0x33d78)?;
    // 0x33d5c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33d60: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x50d4
    c.bl(0x2ec94)?;
    // 0x33d64: ldr r1, [pc, #0x10]         @ 0x1ffc <entry+0x1ffc>
    c.literal(R::R1,0x33d7c)?;
    // 0x33d68: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33d6c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x50e0
    c.bl(0x2ec94)?;
    // 0x33d70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33d74: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x33d78)?;
    for word in STREAM_ALTERNATE_ERRORS { c.emit(word)?; }
    c.check(0x33d80)
}

fn validate_irq_controller_state(c:&mut Code)->Result<(),String>{
    c.check(0x33d80)?;
    // 0x33d80: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x33d84: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x33d88: add r0, r0, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,200)?)?;
    // 0x33d8c: bl 0x80bc0 <entry+0x4ee40> @ imm = #0x4ce2c
    c.bl(0x80bc0)?;
    // 0x33d90: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x33d94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33d98: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x33d9c: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33da0: ldr r1, [pc, #0x18]         @ 0x2040 <entry+0x2040>
    c.literal(R::R1,0x33dc0)?;
    // 0x33da4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33da8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x511c
    c.bl(0x2ec94)?;
    // 0x33dac: ldr r1, [pc, #0x10]         @ 0x2044 <entry+0x2044>
    c.literal(R::R1,0x33dc4)?;
    // 0x33db0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33db4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5128
    c.bl(0x2ec94)?;
    // 0x33db8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33dbc: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x33dc0)?;
    for word in IRQ_STATE_ERRORS { c.emit(word)?; }
    c.check(0x33dc8)
}

fn validate_extended_stream(c:&mut Code)->Result<(),String>{
    c.check(0x33dc8)?;
    // 0x33dc8: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x33dcc: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x33dd0: add r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
    // 0x33dd4: bl 0x80928 <entry+0x4eba8> @ imm = #0x4cb4c
    c.bl(0x80928)?;
    // 0x33dd8: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x33ddc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x33de0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x33de4: popeq {r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33de8: ldr r1, [pc, #0x18]         @ 0x2088 <entry+0x2088>
    c.literal(R::R1,0x33e08)?;
    // 0x33dec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33df0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5164
    c.bl(0x2ec94)?;
    // 0x33df4: ldr r1, [pc, #0x10]         @ 0x208c <entry+0x208c>
    c.literal(R::R1,0x33e0c)?;
    // 0x33df8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33dfc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5170
    c.bl(0x2ec94)?;
    // 0x33e00: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33e04: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x33e08)?;
    for word in STREAM_EXTENDED_ERRORS { c.emit(word)?; }
    c.check(0x33e10)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    drain_active_callback_queues(c)?;
    validate_primary_stream(c)?;
    validate_secondary_stream(c)?;
    validate_alternate_stream(c)?;
    validate_irq_controller_state(c)?;
    validate_extended_stream(c)?;
    c.check(END)
}
