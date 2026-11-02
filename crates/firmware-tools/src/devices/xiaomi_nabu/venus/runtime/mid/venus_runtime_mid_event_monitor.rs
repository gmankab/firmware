// SPDX-License-Identifier: GPL-3.0-only
//! Event-monitor loop for codec controller streams. It validates resource
//! state, chooses a diagnostic format, packs small serialized controller
//! records, and updates a checked 64-bit sequence counter. After the loop's
//! branch are sixteen non-executable diagnostic/device-address words.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2ec94;
const END:usize=0x2efc0;
fn monitor_stream_and_event_state(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2ec94: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x2ec98: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2ec9c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2eca0: sub sp, sp, #348
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,348)?)?;
    // 0x2eca4: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2eca8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ecac: str r3, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,12)?)?;
    // 0x2ecb0: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x2ecb4: str r2, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,8)?)?;
    // 0x2ecb8: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x2ecbc: bl 0x30bc <entry+0xfffffffffffd5abc> @ imm = #-0x2bc08
    c.bl(0x30bc)?;
    // 0x2ecc0: mvn r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0)?)?;
    // 0x2ecc4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ecc8: beq 0x2ece0 <entry+0x16e0>  @ imm = #0x10
    c.b(C::Eq,0x2ece0)?;
    // 0x2eccc: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2ecd0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2ecd4: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2ecd8: add sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::SP,R::SP,8)?)?;
    // 0x2ecdc: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x2ece0: ldr r10, [pc, #0x290]       @ 0x1978 <entry+0x1978>
    c.literal(R::new(10)?,0x2ef78)?;
    // 0x2ece4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2ece8: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,0)?)?;
    // 0x2ecec: tst r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,1)?)?;
    // 0x2ecf0: bne 0x2ed0c <entry+0x170c>  @ imm = #0x14
    c.b(C::Ne,0x2ed0c)?;
    // 0x2ecf4: ldr r1, [pc, #0x280]        @ 0x197c <entry+0x197c>
    c.literal(R::R1,0x2ef7c)?;
    // 0x2ecf8: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x2ecfc: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2ed00: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2ed04: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2ed08: and r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
    // 0x2ed0c: ldr r1, [pc, #0x26c]        @ 0x1980 <entry+0x1980>
    c.literal(R::R1,0x2ef80)?;
    // 0x2ed10: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2ed14: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2ed18: bne 0x2eccc <entry+0x16cc>  @ imm = #-0x54
    c.b(C::Ne,0x2eccc)?;
    // 0x2ed1c: ands r0, r1, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::new(5)?,Shift::Lsl,0,true)?)?;
    // 0x2ed20: beq 0x2eccc <entry+0x16cc>  @ imm = #-0x5c
    c.b(C::Eq,0x2eccc)?;
    // 0x2ed24: add r9, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::SP,16)?)?;
    // 0x2ed28: cmp r5, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,7)?)?;
    // 0x2ed2c: add r0, r9, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,24)?)?;
    // 0x2ed30: ble 0x2ed54 <entry+0x1754>  @ imm = #0x1c
    c.b(C::Le,0x2ed54)?;
    // 0x2ed34: cmp r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,8)?)?;
    // 0x2ed38: beq 0x2ed6c <entry+0x176c>  @ imm = #0x2c
    c.b(C::Eq,0x2ed6c)?;
    // 0x2ed3c: cmp r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,16)?)?;
    // 0x2ed40: beq 0x2ed74 <entry+0x1774>  @ imm = #0x2c
    c.b(C::Eq,0x2ed74)?;
    // 0x2ed44: cmp r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,32)?)?;
    // 0x2ed48: bne 0x2ed84 <entry+0x1784>  @ imm = #0x34
    c.b(C::Ne,0x2ed84)?;
    // 0x2ed4c: ldr r1, [pc, #0x234]        @ 0x1988 <entry+0x1988>
    c.literal(R::R1,0x2ef88)?;
    // 0x2ed50: b 0x2ed88 <entry+0x1788>  @ imm = #0x30
    c.b(C::Al,0x2ed88)?;
    // 0x2ed54: cmp r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,2)?)?;
    // 0x2ed58: beq 0x2ed7c <entry+0x177c>  @ imm = #0x1c
    c.b(C::Eq,0x2ed7c)?;
    // 0x2ed5c: cmp r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,4)?)?;
    // 0x2ed60: bne 0x2ed84 <entry+0x1784>  @ imm = #0x1c
    c.b(C::Ne,0x2ed84)?;
    // 0x2ed64: ldr r1, [pc, #0x218]        @ 0x1984 <entry+0x1984>
    c.literal(R::R1,0x2ef84)?;
    // 0x2ed68: b 0x2ed88 <entry+0x1788>  @ imm = #0x18
    c.b(C::Al,0x2ed88)?;
    // 0x2ed6c: ldr r1, [pc, #0x21c]        @ 0x1990 <entry+0x1990>
    c.literal(R::R1,0x2ef90)?;
    // 0x2ed70: b 0x2ed88 <entry+0x1788>  @ imm = #0x10
    c.b(C::Al,0x2ed88)?;
    // 0x2ed74: ldr r1, [pc, #0x210]        @ 0x198c <entry+0x198c>
    c.literal(R::R1,0x2ef8c)?;
    // 0x2ed78: b 0x2ed88 <entry+0x1788>  @ imm = #0x8
    c.b(C::Al,0x2ed88)?;
    // 0x2ed7c: ldr r1, [pc, #0x210]        @ 0x1994 <entry+0x1994>
    c.literal(R::R1,0x2ef94)?;
    // 0x2ed80: b 0x2ed88 <entry+0x1788>  @ imm = #0x0
    c.b(C::Al,0x2ed88)?;
    // 0x2ed84: ldr r1, [pc, #0x20c]        @ 0x1998 <entry+0x1998>
    c.literal(R::R1,0x2ef98)?;
    // 0x2ed88: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x2ed8c: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x2ed90: bl 0x471c <entry+0xfffffffffffd711c> @ imm = #-0x2a67c
    c.bl(0x471c)?;
    // 0x2ed94: mov r0, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,58)?)?;
    // 0x2ed98: strb r0, [sp, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,47)?)?;
    // 0x2ed9c: bl 0x3080 <entry+0xfffffffffffd5a80> @ imm = #-0x2bd24
    c.bl(0x3080)?;
    // 0x2eda0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2eda4: beq 0x2edb0 <entry+0x17b0>  @ imm = #0x4
    c.b(C::Eq,0x2edb0)?;
    // 0x2eda8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2edac: b 0x2edcc <entry+0x17cc>  @ imm = #0x18
    c.b(C::Al,0x2edcc)?;
    // 0x2edb0: ldr r1, [pc, #0x1e4]        @ 0x199c <entry+0x199c>
    c.literal(R::R1,0x2ef9c)?;
    // 0x2edb4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2edb8: str r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-48)?)?;
    // 0x2edbc: str r0, [r11, #-0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-44)?)?;
    // 0x2edc0: str r0, [r11, #-0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-40)?)?;
    // 0x2edc4: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x2edc8: sub r0, r11, #48
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R11,48)?)?;
    // 0x2edcc: ldrb r2, [r1, #0x5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,5)?)?;
    // 0x2edd0: ldrb r3, [r1, #0x4]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,4)?)?;
    // 0x2edd4: orr r2, r3, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x2edd8: strh r2, [sp, #52]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::SP,52)?)?;
    // 0x2eddc: ldrb r2, [r1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x2ede0: ldrb r3, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,1)?)?;
    // 0x2ede4: ldrb r4, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R1,2)?)?;
    // 0x2ede8: ldrb r1, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,3)?)?;
    // 0x2edec: orr r1, r4, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(4)?,R::R1,Shift::Lsl,8,false)?)?;
    // 0x2edf0: orr r2, r2, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x2edf4: orr r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,16,false)?)?;
    // 0x2edf8: str r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,48)?)?;
    // 0x2edfc: mov r1, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,58)?)?;
    // 0x2ee00: ldr r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // 0x2ee04: strb r1, [sp, #0x36]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::SP,54)?)?;
    // 0x2ee08: ldr r1, [pc, #0x190]        @ 0x19a0 <entry+0x19a0>
    c.literal(R::R1,0x2efa0)?;
    // 0x2ee0c: cmp r2, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,14)?)?;
    // 0x2ee10: movhi r2, #0
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2ee14: ldr r1, [r1, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R1,R::R2,Shift::Lsl,2)?)?;
    // 0x2ee18: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2ee1c: ldrb r3, [r1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // 0x2ee20: ldrb r4, [r1, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(4)?,R::R1,1)?)?;
    // 0x2ee24: ldrb r7, [r1, #0x2]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(7)?,R::R1,2)?)?;
    // 0x2ee28: ldrb r1, [r1, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,3)?)?;
    // 0x2ee2c: orr r6, r7, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::new(7)?,R::R1,Shift::Lsl,8,false)?)?;
    // 0x2ee30: orr r3, r3, r4, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(4)?,Shift::Lsl,8,false)?)?;
    // 0x2ee34: orr r3, r3, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x2ee38: add r6, r9, #39
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(9)?,39)?)?;
    // 0x2ee3c: strb r3, [r6], #2
    c.emit(arm32::transfer_byte_postindexed(C::Al,false,R::new(3)?,R::new(6)?,2)?)?;
    // 0x2ee40: strb r1, [r6, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(6)?,1)?)?;
    // 0x2ee44: add r1, r9, #43
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(9)?,43)?)?;
    // 0x2ee48: strb r7, [r6]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // 0x2ee4c: strb r4, [sp, #0x38]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(4)?,R::SP,56)?)?;
    // 0x2ee50: beq 0x2ee74 <entry+0x1874>  @ imm = #0x1c
    c.b(C::Eq,0x2ee74)?;
    // 0x2ee54: ldr r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,8)?)?;
    // 0x2ee58: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x2ee5c: ldr r2, [pc, #0x140]        @ 0x19a4 <entry+0x19a4>
    c.literal(R::R2,0x2efa4)?;
    // 0x2ee60: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2ee64: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x2ee68: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x2ee6c: bl 0x31c0 <entry+0xfffffffffffd5bc0> @ imm = #-0x2bcb4
    c.bl(0x31c0)?;
    // 0x2ee70: b 0x2ee84 <entry+0x1884>  @ imm = #0xc
    c.b(C::Al,0x2ee84)?;
    // 0x2ee74: ldr r2, [pc, #0x12c]        @ 0x19a8 <entry+0x19a8>
    c.literal(R::R2,0x2efa8)?;
    // 0x2ee78: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x2ee7c: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x2ee80: bl 0x31c0 <entry+0xfffffffffffd5bc0> @ imm = #-0x2bcc8
    c.bl(0x31c0)?;
    // 0x2ee84: add r3, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R11,8)?)?;
    // 0x2ee88: add r0, r9, #56
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,56)?)?;
    // 0x2ee8c: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x2ee90: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x2ee94: str r3, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-48)?)?;
    // 0x2ee98: bl 0x3fd4 <entry+0xfffffffffffd69d4> @ imm = #-0x2aecc
    c.bl(0x3fd4)?;
    // 0x2ee9c: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x2eea0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2eea4: strb r0, [sp, #0x145]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::SP,325)?)?;
    // 0x2eea8: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x2eeac: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x2eeb0: movlt r7, #256
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x2eeb4: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x2eeb8: beq 0x2ef50 <entry+0x1950>  @ imm = #0x90
    c.b(C::Eq,0x2ef50)?;
    // 0x2eebc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2eec0: orr r0, r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // 0x2eec4: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2eec8: str r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,24)?)?;
    // 0x2eecc: ldr r5, [pc, #0xd8]         @ 0x19ac <entry+0x19ac>
    c.literal(R::new(5)?,0x2efac)?;
    // 0x2eed0: add r0, r7, #33
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,33)?)?;
    // 0x2eed4: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2eed8: add r0, r7, #61
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,61)?)?;
    // 0x2eedc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2eee0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2eee4: bl 0x2bf4 <entry+0xfffffffffffd55f4> @ imm = #-0x2c2f8
    c.bl(0x2bf4)?;
    // 0x2eee8: ldr r0, [pc, #0xc0]         @ 0x19b0 <entry+0x19b0>
    c.literal(R::R0,0x2efb0)?;
    // 0x2eeec: ldr r6, [pc, #0xc0]         @ 0x19b4 <entry+0x19b4>
    c.literal(R::new(6)?,0x2efb4)?;
    // 0x2eef0: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,16)?)?;
    // 0x2eef4: ldrd r2, r3, [r0]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x2eef8: str r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,32)?)?;
    // 0x2eefc: str r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,36)?)?;
    // 0x2ef00: adds r2, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R2,R::R2,1,true)?)?;
    // 0x2ef04: adc r3, r3, #0
    c.emit(super::encode_adc_zero(C::Al,R::new(3)?,R::new(3)?)?)?;
    // 0x2ef08: strd r2, r3, [r0]
    c.emit(arm32::transfer_doubleword(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x2ef0c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2ef10: add r2, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,12)?)?;
    // 0x2ef14: bl 0x2f1e4 <entry+0x1be4>  @ imm = #0x2c8
    c.bl(0x2f1e4)?;
    // 0x2ef18: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ef1c: bne 0x2ef28 <entry+0x1928>  @ imm = #0x4
    c.b(C::Ne,0x2ef28)?;
    // 0x2ef20: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2ef24: bl 0x2f394 <entry+0x1d94>  @ imm = #0x468
    c.bl(0x2f394)?;
    // 0x2ef28: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,0)?)?;
    // 0x2ef2c: bl 0x2c34 <entry+0xfffffffffffd5634> @ imm = #-0x2c300
    c.bl(0x2c34)?;
    // 0x2ef30: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2ef34: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ef38: bne 0x2ef4c <entry+0x194c>  @ imm = #0xc
    c.b(C::Ne,0x2ef4c)?;
    // 0x2ef3c: ldr r0, [pc, #0x3c]         @ 0x1980 <entry+0x1980>
    c.literal(R::R0,0x2ef80)?;
    // 0x2ef40: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2ef44: tst r0, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,8388608)?)?;
    // 0x2ef48: beq 0x2ef50 <entry+0x1950>  @ imm = #0x0
    c.b(C::Eq,0x2ef50)?;
    // 0x2ef4c: bl 0x2a158 <entry+0xffffffffffffcb58> @ imm = #-0x4dfc
    c.bl(0x2a158)?;
    // 0x2ef50: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x2ef54: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x2ef58: ldrne r0, [pc, #0x1c]         @ 0x197c <entry+0x197c>
    c.literal_cond(C::Ne,R::R0,0x2ef7c)?;
    // 0x2ef5c: ldrne r0, [r0]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::R0,0)?)?;
    // 0x2ef60: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ef64: beq 0x2eccc <entry+0x16cc>  @ imm = #-0x2a0
    c.b(C::Eq,0x2eccc)?;
    // 0x2ef68: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x2ef6c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2ef70: bl 0x258e4 <entry+0xffffffffffff82e4> @ imm = #-0x9694
    c.bl(0x258e4)?;
    // 0x2ef74: b 0x2eccc <entry+0x16cc>  @ imm = #-0x2b0
    c.b(C::Al,0x2eccc)?;
    c.check(0x2ef78)?;
    const EVENT_MONITOR_DATA_2EF78:[u32;16]=[0x00104108,0x0010306c,0x00104104,0x000eac28,0x000eac40,0x000eac50,0x000eac48,0x000eac30,0x000eac38,0x000e2557,0x000eac58,0x000eacd1,0x000eacc3,0x0010410c,0x0010e070,0x00104110];
    for word in EVENT_MONITOR_DATA_2EF78 {c.emit(word)?;}
    c.check(0x2efb8)?;
    // 0x2efb8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x2efbc: b 0x2efc0 <entry+0x19c0>  @ imm = #-0x4
    c.b(C::Al,0x2efc0)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;monitor_stream_and_event_state(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_stream_monitor_pool_is_checked(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2ef78),0x00104108);
        assert_eq!(data(0x2ef80),0x00104104);
        assert_eq!(data(0x2efb4),0x00104110);
    }
}
