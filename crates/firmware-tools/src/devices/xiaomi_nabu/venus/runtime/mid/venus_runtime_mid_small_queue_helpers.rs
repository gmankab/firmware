// SPDX-License-Identifier: GPL-3.0-only
//! Five small typed ARMv7 routines for byte/word queue emission, bounded
//! codec descriptor records, power-of-two geometry and controller bus modes.
//! Only three known 32-bit diagnostic pointer words are non-executable.
use super::Code;
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
const START:usize=0x3ead8;
const END:usize=0x3ee98;
const QUEUE_DIAGNOSTIC:u32=0x000e_e028;

fn stream_byte_queue(c:&mut Code)->Result<(),String>{
    c.check(0x3ead8)?;
    // 0x3ead8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3eadc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x3eae0: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x3eae4: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x3eae8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x3eaec: ldr r9, [pc, #0xf0]         @ 0x5014 <entry+0x5014>
    c.literal_cond(C::Al,R::new(9)?,0x3ebe4)?;
    // 0x3eaf0: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // 0x3eaf4: mov r1, #135
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,135)?)?;
    // 0x3eaf8: mov r7, r3
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(3)?))?;
    // 0x3eafc: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x3eb00: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x3eb04: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3eb08: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x3eb0c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3eb10: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3eb14: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3eb18: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d370
    c.bl(0x17b0)?;
    // 0x3eb1c: ldr r4, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,36)?)?;
    // 0x3eb20: mov r1, #137
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,137)?)?;
    // 0x3eb24: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x3eb28: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3eb2c: tst r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,7)?)?;
    // 0x3eb30: moveq r8, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x3eb34: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x3eb38: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d390
    c.bl(0x17b0)?;
    // 0x3eb3c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3eb40: add r0, r0, r4, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(4)?,Shift::Lsr,3,false)?)?;
    // 0x3eb44: ands r0, r0, #15
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,15,true)?)?;
    // 0x3eb48: rsbsne r0, r0, #16
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::Rsb,R::R0,R::R0,16,true)?)?;
    // 0x3eb4c: bne 0x3eb58 <entry+0x4f88>  @ imm = #0x4
    c.b(C::Ne,0x3eb58)?;
    // 0x3eb50: ldr r4, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,36)?)?;
    // 0x3eb54: b 0x3eb8c <entry+0x4fbc>  @ imm = #0x30
    c.b(C::Al,0x3eb8c)?;
    // 0x3eb58: ldr r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,32)?)?;
    // 0x3eb5c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x3eb60: mvnne r7, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mvn,R::new(7)?,R::R0,0)?)?;
    // 0x3eb64: strb r7, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(7)?,R::R1,0)?)?;
    // 0x3eb68: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3eb6c: ldr r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,32)?)?;
    // 0x3eb70: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x3eb74: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x3eb78: str r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,32)?)?;
    // 0x3eb7c: ldr r2, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,36)?)?;
    // 0x3eb80: add r4, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R2,8)?)?;
    // 0x3eb84: str r4, [r6, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,36)?)?;
    // 0x3eb88: bne 0x3eb64 <entry+0x4f94>  @ imm = #-0x2c
    c.b(C::Ne,0x3eb64)?;
    // 0x3eb8c: ldr r2, [pc, #0x50]         @ 0x5014 <entry+0x5014>
    c.literal_cond(C::Al,R::R2,0x3ebe4)?;
    // 0x3eb90: add r0, r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,32)?)?;
    // 0x3eb94: mov r1, #149
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,149)?)?;
    // 0x3eb98: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3eb9c: tst r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,127)?)?;
    // 0x3eba0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3eba4: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3eba8: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3ebac: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d404
    c.bl(0x17b0)?;
    // 0x3ebb0: cmp r7, r4, lsr #5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsr,5,false)?)?;
    // 0x3ebb4: beq 0x3ebdc <entry+0x500c>  @ imm = #0x20
    c.b(C::Eq,0x3ebdc)?;
    // 0x3ebb8: ldr r6, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,28)?)?;
    // 0x3ebbc: lsr r4, r4, #5
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(4)?,Shift::Lsr,5)?)?;
    // 0x3ebc0: ldr r2, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::new(6)?,4)?)?;
    // 0x3ebc4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3ebc8: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x3ebcc: bl 0x3e2c8 <entry+0x46f8>  @ imm = #-0x90c
    c.bl(0x3e2c8)?;
    // 0x3ebd0: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3ebd4: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3ebd8: bne 0x3ebc0 <entry+0x4ff0>  @ imm = #-0x20
    c.b(C::Ne,0x3ebc0)?;
    // 0x3ebdc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x3ebe0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3ebe4)?;
    c.emit(QUEUE_DIAGNOSTIC)?;
    c.check(0x3ebe8)
}

fn stream_word_queue(c:&mut Code)->Result<(),String>{
    c.check(0x3ebe8)?;
    // 0x3ebe8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3ebec: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x3ebf0: ldr r6, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,8)?)?;
    // 0x3ebf4: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x3ebf8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x3ebfc: add r0, r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,7)?)?;
    // 0x3ec00: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x3ec04: add r1, r1, r0, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsr,3,false)?)?;
    // 0x3ec08: lsr r8, r0, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::new(8)?,R::R0,Shift::Lsr,3)?)?;
    // 0x3ec0c: tst r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,15)?)?;
    // 0x3ec10: beq 0x3ec3c <entry+0x506c>  @ imm = #0x24
    c.b(C::Eq,0x3ec3c)?;
    // 0x3ec14: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x3ec18: mov r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15)?)?;
    // 0x3ec1c: add r0, r6, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x3ec20: bic r4, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Bic,R::new(4)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3ec24: mvnne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mvn,R::new(3)?,R::R0,0)?)?;
    // 0x3ec28: add r2, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,1)?)?;
    // 0x3ec2c: mov r1, r3
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(3)?))?;
    // 0x3ec30: bl 0x31ec <entry+0xfffffffffffc961c> @ imm = #-0x3ba4c
    c.bl(0x31ec)?;
    // 0x3ec34: add r0, r8, r4
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3ec38: add r8, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,1)?)?;
    // 0x3ec3c: ldr r2, [pc, #0x48]         @ 0x50bc <entry+0x50bc>
    c.literal_cond(C::Al,R::R2,0x3ec8c)?;
    // 0x3ec40: add r0, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,4)?)?;
    // 0x3ec44: mov r1, #174
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,174)?)?;
    // 0x3ec48: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3ec4c: tst r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,15)?)?;
    // 0x3ec50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3ec54: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3ec58: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3ec5c: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d4b4
    c.bl(0x17b0)?;
    // 0x3ec60: cmp r7, r8, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(8)?,Shift::Lsr,2,false)?)?;
    // 0x3ec64: popeq {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3ec68: lsr r4, r8, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::new(8)?,Shift::Lsr,2)?)?;
    // 0x3ec6c: ldr r2, [r6], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::new(6)?,4)?)?;
    // 0x3ec70: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3ec74: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x3ec78: bl 0x3e2c8 <entry+0x46f8>  @ imm = #-0x9b8
    c.bl(0x3e2c8)?;
    // 0x3ec7c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3ec80: cmp r7, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x3ec84: bne 0x3ec6c <entry+0x509c>  @ imm = #-0x20
    c.b(C::Ne,0x3ec6c)?;
    // 0x3ec88: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3ec8c)?;
    c.emit(QUEUE_DIAGNOSTIC)?;
    c.check(0x3ec90)
}

fn stream_descriptor_access(c:&mut Code)->Result<(),String>{
    c.check(0x3ec90)?;
    // 0x3ec90: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3ec94: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x3ec98: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x3ec9c: ldr r9, [pc, #0xd0]         @ 0x51a4 <entry+0x51a4>
    c.literal_cond(C::Al,R::new(9)?,0x3ed74)?;
    // 0x3eca0: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x3eca4: mov r1, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,187)?)?;
    // 0x3eca8: mov r6, r3
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(3)?))?;
    // 0x3ecac: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x3ecb0: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x3ecb4: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3ecb8: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x3ecbc: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3ecc0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3ecc4: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3ecc8: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d520
    c.bl(0x17b0)?;
    // 0x3eccc: ldr r4, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,36)?)?;
    // 0x3ecd0: mov r1, #189
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,189)?)?;
    // 0x3ecd4: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x3ecd8: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3ecdc: tst r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,7)?)?;
    // 0x3ece0: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x3ece4: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x3ece8: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d540
    c.bl(0x17b0)?;
    // 0x3ecec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3ecf0: add r0, r0, r4, lsr #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(4)?,Shift::Lsr,3,false)?)?;
    // 0x3ecf4: ands r0, r0, #15
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::R0,R::R0,15,true)?)?;
    // 0x3ecf8: rsbsne r0, r0, #16
    c.emit(arm32::data_immediate_with_flags(C::Ne,Op::Rsb,R::R0,R::R0,16,true)?)?;
    // 0x3ecfc: bne 0x3ed08 <entry+0x5138>  @ imm = #0x4
    c.b(C::Ne,0x3ed08)?;
    // 0x3ed00: ldr r4, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,36)?)?;
    // 0x3ed04: b 0x3ed3c <entry+0x516c>  @ imm = #0x30
    c.b(C::Al,0x3ed3c)?;
    // 0x3ed08: ldr r1, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,32)?)?;
    // 0x3ed0c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x3ed10: mvnne r6, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mvn,R::new(6)?,R::R0,0)?)?;
    // 0x3ed14: strb r6, [r1]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x3ed18: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3ed1c: ldr r1, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,32)?)?;
    // 0x3ed20: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x3ed24: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x3ed28: str r1, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,32)?)?;
    // 0x3ed2c: ldr r2, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,36)?)?;
    // 0x3ed30: add r4, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R2,8)?)?;
    // 0x3ed34: str r4, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,36)?)?;
    // 0x3ed38: bne 0x3ed14 <entry+0x5144>  @ imm = #-0x2c
    c.b(C::Ne,0x3ed14)?;
    // 0x3ed3c: ldr r2, [pc, #0x30]         @ 0x51a4 <entry+0x51a4>
    c.literal_cond(C::Al,R::R2,0x3ed74)?;
    // 0x3ed40: add r1, r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,32)?)?;
    // 0x3ed44: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3ed48: tst r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,127)?)?;
    // 0x3ed4c: mov r1, #201
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,201)?)?;
    // 0x3ed50: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3ed54: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x3ed58: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3d5b0
    c.bl(0x17b0)?;
    // 0x3ed5c: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x3ed60: mvn r0, #-536870909
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,3758096387)?)?;
    // 0x3ed64: and r0, r0, r4, lsr #3
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::new(4)?,Shift::Lsr,3,false)?)?;
    // 0x3ed68: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3ed6c: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x3ed70: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3ed74)?;
    c.emit(QUEUE_DIAGNOSTIC)?;
    c.check(0x3ed78)
}

fn power_of_two_dimension(c:&mut Code)->Result<(),String>{
    c.check(0x3ed78)?;
    // 0x3ed78: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3ed7c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x3ed80: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x3ed84: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x3ed88: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3ed8c: blx 0x83c4c <entry+0x4a07c> @ imm = #0x44eb8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x3ed90: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x3ed94: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3ed98: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x3ed9c: blx 0x83c4c <entry+0x4a07c> @ imm = #0x44ea8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x3eda0: mul r1, r0, r6
    c.emit(arm32::multiply_register(C::Al,R::R1,R::R0,R::new(6)?)?)?;
    // 0x3eda4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3eda8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x3edac: beq 0x3edd0 <entry+0x5200>  @ imm = #0x1c
    c.b(C::Eq,0x3edd0)?;
    // 0x3edb0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3edb4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3edb8: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x3edbc: lsr r6, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(3)?,Shift::Lsr,1)?)?;
    // 0x3edc0: cmp r2, r3, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsr,1,false)?)?;
    // 0x3edc4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3edc8: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x3edcc: bne 0x3edbc <entry+0x51ec>  @ imm = #-0x18
    c.b(C::Ne,0x3edbc)?;
    // 0x3edd0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3edd4: cmp r1, r2, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,R::R0,false)?)?;
    // 0x3edd8: addhi r0, r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3eddc: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3ede0)?;
    c.check(0x3ede0)
}

fn controller_bus_selector(c:&mut Code)->Result<(),String>{
    c.check(0x3ede0)?;
    // 0x3ede0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3ede4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x3ede8: mov lr, #132
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,132)?)?;
    // 0x3edec: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3edf0: orr lr, lr, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,3758096384)?)?;
    // 0x3edf4: beq 0x3ee4c <entry+0x527c>  @ imm = #0x50
    c.b(C::Eq,0x3ee4c)?;
    // 0x3edf8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x3edfc: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x3ee00: beq 0x3ee5c <entry+0x528c>  @ imm = #0x54
    c.b(C::Eq,0x3ee5c)?;
    // 0x3ee04: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x3ee08: bne 0x3ee90 <entry+0x52c0>  @ imm = #0x80
    c.b(C::Ne,0x3ee90)?;
    // 0x3ee0c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x3ee10: blt 0x3ee44 <entry+0x5274>  @ imm = #0x2c
    c.b(C::Lt,0x3ee44)?;
    // 0x3ee14: ldr r2, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,8)?)?;
    // 0x3ee18: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x3ee1c: mov r4, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,3758096384)?)?;
    // 0x3ee20: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3ee24: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x3ee28: ldr r6, [r0, r4]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::R0,R::new(4)?,Shift::Lsl,0)?)?;
    // 0x3ee2c: tst r6, r2
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3ee30: bne 0x3ee90 <entry+0x52c0>  @ imm = #0x58
    c.b(C::Ne,0x3ee90)?;
    // 0x3ee34: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x3ee38: and r12, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R12,R::new(5)?,255)?)?;
    // 0x3ee3c: cmp r12, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3ee40: blt 0x3ee24 <entry+0x5254>  @ imm = #-0x24
    c.b(C::Lt,0x3ee24)?;
    // 0x3ee44: mov r12, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,10)?)?;
    // 0x3ee48: b 0x3ee90 <entry+0x52c0>  @ imm = #0x40
    c.b(C::Al,0x3ee90)?;
    // 0x3ee4c: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x3ee50: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x3ee54: ldr r12, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R12,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x3ee58: b 0x3ee90 <entry+0x52c0>  @ imm = #0x30
    c.b(C::Al,0x3ee90)?;
    // 0x3ee5c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x3ee60: blt 0x3ee90 <entry+0x52c0>  @ imm = #0x28
    c.b(C::Lt,0x3ee90)?;
    // 0x3ee64: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3ee68: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x3ee6c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x3ee70: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x3ee74: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::LR,0)?)?;
    // 0x3ee78: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3ee7c: ldr r3, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x3ee80: add r12, r3, r12
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::new(3)?,R::R12,Shift::Lsl,0,false)?)?;
    // 0x3ee84: and r3, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::new(4)?,255)?)?;
    // 0x3ee88: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3ee8c: blt 0x3ee74 <entry+0x52a4>  @ imm = #-0x20
    c.b(C::Lt,0x3ee74)?;
    // 0x3ee90: mov r0, r12
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    // 0x3ee94: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x3ee98)?;
    c.check(0x3ee98)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    stream_byte_queue(c)?;
    stream_word_queue(c)?;
    stream_descriptor_access(c)?;
    power_of_two_dimension(c)?;
    controller_bus_selector(c)?;
    c.check(END)
}

#[cfg(test)] mod tests {
 use super::*;
 #[test] fn checked_bounded_queue_and_bus_helpers(){
  let mut c=Code::new(START);emit(&mut c).unwrap();
  assert_eq!(c.at(),END);
  for addr in [0x3ebe4,0x3ec8c,0x3ed74]{
   let i=addr-START;
   assert_eq!(u32::from_le_bytes(c.bytes[i..i+4].try_into().unwrap()),QUEUE_DIAGNOSTIC);
  }
 }
}
