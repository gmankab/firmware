// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller buffer format-bit translation, controller-cache reset,
//! multi-channel transfer descriptors and IRQ buffer-list registration.
//! Register data and log-address pools follow their respective functions.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x311c0;
const END:usize=0x3192c;
fn configure_register_writer_and_list(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x311c0: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x311c4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x311c8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x311cc: beq 0x311e8 <entry+0x1be8>  @ imm = #0x14
    c.b(C::Eq,0x311e8)?;
    // 0x311d0: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x311d4: beq 0x311f0 <entry+0x1bf0>  @ imm = #0x14
    c.b(C::Eq,0x311f0)?;
    // 0x311d8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x311dc: bne 0x311fc <entry+0x1bfc>  @ imm = #0x18
    c.b(C::Ne,0x311fc)?;
    // 0x311e0: mov r4, #1696
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1696)?)?;
    // 0x311e4: b 0x31238 <entry+0x1c38>  @ imm = #0x4c
    c.b(C::Al,0x31238)?;
    // 0x311e8: mov r4, #1872
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1872)?)?;
    // 0x311ec: b 0x31238 <entry+0x1c38>  @ imm = #0x44
    c.b(C::Al,0x31238)?;
    // 0x311f0: mov r4, #476
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,476)?)?;
    // 0x311f4: orr r4, r4, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,1024)?)?;
    // 0x311f8: b 0x31238 <entry+0x1c38>  @ imm = #0x38
    c.b(C::Al,0x31238)?;
    // 0x311fc: ldr r1, [pc, #0x3c]         @ 0x1c40 <entry+0x1c40>
    c.literal(R::R1,0x31240)?;
    // 0x31200: ldr r2, [pc, #0x3c]         @ 0x1c44 <entry+0x1c44>
    c.literal(R::R2,0x31244)?;
    // 0x31204: mov r3, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,222)?)?;
    // 0x31208: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3120c: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x31210: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2584
    c.bl(0x2ec94)?;
    // 0x31214: ldr r1, [pc, #0x2c]         @ 0x1c48 <entry+0x1c48>
    c.literal(R::R1,0x31248)?;
    // 0x31218: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3121c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2590
    c.bl(0x2ec94)?;
    // 0x31220: ldr r2, [pc, #0x24]         @ 0x1c4c <entry+0x1c4c>
    c.literal(R::R2,0x3124c)?;
    // 0x31224: mov r1, #223
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,223)?)?;
    // 0x31228: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3122c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x31230: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x31234: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2fa8c
    c.bl(0x17b0)?;
    // 0x31238: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x3123c: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x31240)?;
    const CONTROLLER_REGISTER_TABLE_31240:[u32;4]=[0x000eb189,0x000eb1a7,0x000e2788,0x000eafbb];
    for word in CONTROLLER_REGISTER_TABLE_31240 {c.emit(word)?;}
    c.check(0x31250)?;
    // 0x31250: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x31254: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x31258: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x3125c: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x31260: beq 0x31284 <entry+0x1c84>  @ imm = #0x1c
    c.b(C::Eq,0x31284)?;
    // 0x31264: cmp r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // 0x31268: beq 0x31294 <entry+0x1c94>  @ imm = #0x24
    c.b(C::Eq,0x31294)?;
    // 0x3126c: cmp r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,64)?)?;
    // 0x31270: bne 0x312a4 <entry+0x1ca4>  @ imm = #0x2c
    c.b(C::Ne,0x312a4)?;
    // 0x31274: mov r5, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,512)?)?;
    // 0x31278: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3127c: movne r5, #640
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,640)?)?;
    // 0x31280: b 0x312bc <entry+0x1cbc>  @ imm = #0x34
    c.b(C::Al,0x312bc)?;
    // 0x31284: mov r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,128)?)?;
    // 0x31288: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3128c: movne r5, #160
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,160)?)?;
    // 0x31290: b 0x312bc <entry+0x1cbc>  @ imm = #0x24
    c.b(C::Al,0x312bc)?;
    // 0x31294: mov r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,256)?)?;
    // 0x31298: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x3129c: movne r5, #320
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,320)?)?;
    // 0x312a0: b 0x312bc <entry+0x1cbc>  @ imm = #0x14
    c.b(C::Al,0x312bc)?;
    // 0x312a4: ldr r2, [pc, #0x4c]         @ 0x1cf8 <entry+0x1cf8>
    c.literal(R::R2,0x312f8)?;
    // 0x312a8: mov r1, #89
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,89)?)?;
    // 0x312ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x312b0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x312b4: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x312b8: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2fb10
    c.bl(0x17b0)?;
    // 0x312bc: orr r1, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,16)?)?;
    // 0x312c0: mov r0, #748
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,748)?)?;
    // 0x312c4: mov r2, #62
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,62)?)?;
    // 0x312c8: add r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // 0x312cc: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x312d0: add r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,768)?)?;
    // 0x312d4: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x312d8: and r1, r2, r1, lsr #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsr,4,false)?)?;
    // 0x312dc: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x312e0: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x312e4: mov r0, #2800
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2800)?)?;
    // 0x312e8: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x312ec: orr r0, r0, r4, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,15,false)?)?;
    // 0x312f0: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x312f4: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x312f8)?;
    const CONTROLLER_REGISTER_TABLE_312F8:[u32;1]=[0x000eafbb];
    for word in CONTROLLER_REGISTER_TABLE_312F8 {c.emit(word)?;}
    c.check(0x312fc)?;
    // 0x312fc: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x31300: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x31304: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x31308: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x3130c: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x31310: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x31314: and r2, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(6)?,255)?)?;
    // 0x31318: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3131c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x31320: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x31324: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x31328: bl 0x31830 <entry+0x2230>  @ imm = #0x500
    c.bl(0x31830)?;
    // 0x3132c: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x31330: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // 0x31334: cmp r6, #10
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,10)?)?;
    // 0x31338: bne 0x31314 <entry+0x1d14>  @ imm = #-0x2c
    c.b(C::Ne,0x31314)?;
    // 0x3133c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31340: mov r6, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,124)?)?;
    // 0x31344: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x31348: sub r3, r6, #84
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,84)?)?;
    // 0x3134c: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x31350: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x31354: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // 0x31358: beq 0x3134c <entry+0x1d4c>  @ imm = #-0x14
    c.b(C::Eq,0x3134c)?;
    // 0x3135c: and r2, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,255)?)?;
    // 0x31360: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31364: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x31368: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x3136c: bl 0x31830 <entry+0x2230>  @ imm = #0x4bc
    c.bl(0x31830)?;
    // 0x31370: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x31374: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // 0x31378: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x3137c: bne 0x31348 <entry+0x1d48>  @ imm = #-0x3c
    c.b(C::Ne,0x31348)?;
    // 0x31380: mov r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16)?)?;
    // 0x31384: sub r3, r6, #84
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,84)?)?;
    // 0x31388: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3138c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x31390: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x31394: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x31398: bl 0x31830 <entry+0x2230>  @ imm = #0x490
    c.bl(0x31830)?;
    // 0x3139c: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x313a0: sub r3, r6, #68
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,68)?)?;
    // 0x313a4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x313a8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x313ac: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x313b0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x313b4: bl 0x31830 <entry+0x2230>  @ imm = #0x474
    c.bl(0x31830)?;
    // 0x313b8: sub r3, r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,64)?)?;
    // 0x313bc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x313c0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x313c4: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x313c8: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x313cc: bl 0x31830 <entry+0x2230>  @ imm = #0x45c
    c.bl(0x31830)?;
    // 0x313d0: sub r3, r6, #48
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,48)?)?;
    // 0x313d4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x313d8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x313dc: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x313e0: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x313e4: bl 0x31830 <entry+0x2230>  @ imm = #0x444
    c.bl(0x31830)?;
    // 0x313e8: sub r3, r6, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,44)?)?;
    // 0x313ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x313f0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x313f4: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x313f8: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x313fc: bl 0x31830 <entry+0x2230>  @ imm = #0x42c
    c.bl(0x31830)?;
    // 0x31400: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31404: sub r3, r6, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,40)?)?;
    // 0x31408: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x3140c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x31410: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x31414: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31418: bl 0x31830 <entry+0x2230>  @ imm = #0x410
    c.bl(0x31830)?;
    // 0x3141c: sub r4, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,32)?)?;
    // 0x31420: mov r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7)?)?;
    // 0x31424: and r2, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(7)?,255)?)?;
    // 0x31428: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3142c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x31430: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x31434: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x31438: bl 0x31830 <entry+0x2230>  @ imm = #0x3f0
    c.bl(0x31830)?;
    // 0x3143c: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x31440: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // 0x31444: cmp r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,15)?)?;
    // 0x31448: bne 0x31424 <entry+0x1e24>  @ imm = #-0x2c
    c.b(C::Ne,0x31424)?;
    // 0x3144c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31450: mov r8, r6
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // 0x31454: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x31458: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3145c: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,1)?)?;
    // 0x31460: beq 0x31454 <entry+0x1e54>  @ imm = #-0x14
    c.b(C::Eq,0x31454)?;
    // 0x31464: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // 0x31468: cmpne r5, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x3146c: bne 0x31478 <entry+0x1e78>  @ imm = #0x4
    c.b(C::Ne,0x31478)?;
    // 0x31470: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x31474: b 0x3147c <entry+0x1e7c>  @ imm = #0x0
    c.b(C::Al,0x3147c)?;
    // 0x31478: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x3147c: and r2, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,255)?)?;
    // 0x31480: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31484: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x31488: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x3148c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x31490: bl 0x31830 <entry+0x2230>  @ imm = #0x398
    c.bl(0x31830)?;
    // 0x31494: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
    // 0x31498: add r6, r8, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3149c: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x314a0: bne 0x31450 <entry+0x1e50>  @ imm = #-0x58
    c.b(C::Ne,0x31450)?;
    // 0x314a4: mov r4, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,6)?)?;
    // 0x314a8: mov r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,8)?)?;
    // 0x314ac: mov r5, r6
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // 0x314b0: and r2, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(4)?,255)?)?;
    // 0x314b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x314b8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x314bc: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x314c0: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
    // 0x314c4: bl 0x31830 <entry+0x2230>  @ imm = #0x364
    c.bl(0x31830)?;
    // 0x314c8: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x314cc: add r5, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,8)?)?;
    // 0x314d0: cmp r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,8)?)?;
    // 0x314d4: bne 0x314b0 <entry+0x1eb0>  @ imm = #-0x2c
    c.b(C::Ne,0x314b0)?;
    // 0x314d8: add r5, r6, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,16)?)?;
    // 0x314dc: mov r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,8)?)?;
    // 0x314e0: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x314e4: and r2, r4, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(4)?,255)?)?;
    // 0x314e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x314ec: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x314f0: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x314f4: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x314f8: bl 0x31830 <entry+0x2230>  @ imm = #0x330
    c.bl(0x31830)?;
    // 0x314fc: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x31500: add r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // 0x31504: cmp r4, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,14)?)?;
    // 0x31508: bne 0x314e4 <entry+0x1ee4>  @ imm = #-0x2c
    c.b(C::Ne,0x314e4)?;
    // 0x3150c: add r0, r8, r7
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x31510: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x31514: add r6, r0, #520
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,520)?)?;
    // 0x31518: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3151c: sub r3, r6, #480
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,480)?)?;
    // 0x31520: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x31524: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x31528: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x3152c: beq 0x31520 <entry+0x1f20>  @ imm = #-0x14
    c.b(C::Eq,0x31520)?;
    // 0x31530: and r4, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(7)?,255)?)?;
    // 0x31534: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31538: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3153c: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x31540: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x31544: bl 0x31830 <entry+0x2230>  @ imm = #0x2e4
    c.bl(0x31830)?;
    // 0x31548: sub r3, r6, #476
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,476)?)?;
    // 0x3154c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31550: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x31554: mov r2, r4
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
    // 0x31558: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x3155c: bl 0x31830 <entry+0x2230>  @ imm = #0x2cc
    c.bl(0x31830)?;
    // 0x31560: add r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(7)?,1)?)?;
    // 0x31564: add r6, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,8)?)?;
    // 0x31568: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x3156c: bne 0x3151c <entry+0x1f1c>  @ imm = #-0x58
    c.b(C::Ne,0x3151c)?;
    // 0x31570: sub r5, r6, #480
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::new(6)?,480)?)?;
    // 0x31574: sub r4, r6, #472
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,472)?)?;
    // 0x31578: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // 0x3157c: mov r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // 0x31580: and r2, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(7)?,255)?)?;
    // 0x31584: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31588: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3158c: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x31590: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x31594: bl 0x31830 <entry+0x2230>  @ imm = #0x294
    c.bl(0x31830)?;
    // 0x31598: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3159c: add r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // 0x315a0: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x315a4: bne 0x31580 <entry+0x1f80>  @ imm = #-0x2c
    c.b(C::Ne,0x31580)?;
    // 0x315a8: mov r5, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,11)?)?;
    // 0x315ac: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x315b0: and r2, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,255)?)?;
    // 0x315b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x315b8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x315bc: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x315c0: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x315c4: bl 0x31830 <entry+0x2230>  @ imm = #0x264
    c.bl(0x31830)?;
    // 0x315c8: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x315cc: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x315d0: cmp r5, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,13)?)?;
    // 0x315d4: bne 0x315b0 <entry+0x1fb0>  @ imm = #-0x2c
    c.b(C::Ne,0x315b0)?;
    // 0x315d8: sub r4, r6, #456
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,456)?)?;
    // 0x315dc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x315e0: mov r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // 0x315e4: and r5, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(7)?,255)?)?;
    // 0x315e8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x315ec: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x315f0: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x315f4: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x315f8: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x315fc: bl 0x31830 <entry+0x2230>  @ imm = #0x22c
    c.bl(0x31830)?;
    // 0x31600: add r3, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,4)?)?;
    // 0x31604: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31608: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x3160c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x31610: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x31614: bl 0x31830 <entry+0x2230>  @ imm = #0x214
    c.bl(0x31830)?;
    // 0x31618: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3161c: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x31620: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x31624: bne 0x315e4 <entry+0x1fe4>  @ imm = #-0x48
    c.b(C::Ne,0x315e4)?;
    // 0x31628: sub r4, r6, #440
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,440)?)?;
    // 0x3162c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x31630: mov r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // 0x31634: and r5, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(7)?,255)?)?;
    // 0x31638: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3163c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x31640: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x31644: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x31648: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x3164c: bl 0x31830 <entry+0x2230>  @ imm = #0x1dc
    c.bl(0x31830)?;
    // 0x31650: add r3, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,4)?)?;
    // 0x31654: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31658: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x3165c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x31660: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x31664: bl 0x31830 <entry+0x2230>  @ imm = #0x1c4
    c.bl(0x31830)?;
    // 0x31668: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3166c: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x31670: cmp r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,5)?)?;
    // 0x31674: bne 0x31634 <entry+0x2034>  @ imm = #-0x48
    c.b(C::Ne,0x31634)?;
    // 0x31678: sub r4, r6, #400
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,400)?)?;
    // 0x3167c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x31680: mov r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // 0x31684: and r5, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(7)?,255)?)?;
    // 0x31688: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3168c: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x31690: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x31694: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x31698: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x3169c: bl 0x31830 <entry+0x2230>  @ imm = #0x18c
    c.bl(0x31830)?;
    // 0x316a0: add r3, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,4)?)?;
    // 0x316a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x316a8: mov r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x316ac: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x316b0: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x316b4: bl 0x31830 <entry+0x2230>  @ imm = #0x174
    c.bl(0x31830)?;
    // 0x316b8: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x316bc: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x316c0: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x316c4: bne 0x31684 <entry+0x2084>  @ imm = #-0x48
    c.b(C::Ne,0x31684)?;
    // 0x316c8: mov r4, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,32)?)?;
    // 0x316cc: sub r3, r6, #336
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,336)?)?;
    // 0x316d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x316d4: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x316d8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x316dc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x316e0: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x316e4: bl 0x31830 <entry+0x2230>  @ imm = #0x144
    c.bl(0x31830)?;
    // 0x316e8: mov r0, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,180)?)?;
    // 0x316ec: sub r3, r6, #304
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,304)?)?;
    // 0x316f0: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x316f4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x316f8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x316fc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31700: bl 0x31830 <entry+0x2230>  @ imm = #0x128
    c.bl(0x31830)?;
    // 0x31704: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x31708: sub r3, r6, #124
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,124)?)?;
    // 0x3170c: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x31710: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x31714: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x31718: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3171c: bl 0x31830 <entry+0x2230>  @ imm = #0x10c
    c.bl(0x31830)?;
    // 0x31720: sub r3, r6, #92
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::new(6)?,92)?)?;
    // 0x31724: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31728: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x3172c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x31730: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x31734: bl 0x31830 <entry+0x2230>  @ imm = #0xf4
    c.bl(0x31830)?;
    // 0x31738: sub r4, r6, #60
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,60)?)?;
    // 0x3173c: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x31740: and r2, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::new(5)?,255)?)?;
    // 0x31744: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31748: mov r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,7)?)?;
    // 0x3174c: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x31750: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x31754: bl 0x31830 <entry+0x2230>  @ imm = #0xd4
    c.bl(0x31830)?;
    // 0x31758: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x3175c: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,4)?)?;
    // 0x31760: cmp r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,3)?)?;
    // 0x31764: bne 0x31740 <entry+0x2140>  @ imm = #-0x2c
    c.b(C::Ne,0x31740)?;
    // 0x31768: sub r4, r6, #48
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,48)?)?;
    // 0x3176c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x31770: mov r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // 0x31774: and r5, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(7)?,255)?)?;
    // 0x31778: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3177c: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x31780: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x31784: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x31788: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x3178c: bl 0x31830 <entry+0x2230>  @ imm = #0x9c
    c.bl(0x31830)?;
    // 0x31790: add r3, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,4)?)?;
    // 0x31794: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31798: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x3179c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x317a0: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x317a4: bl 0x31830 <entry+0x2230>  @ imm = #0x84
    c.bl(0x31830)?;
    // 0x317a8: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x317ac: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x317b0: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x317b4: bne 0x31774 <entry+0x2174>  @ imm = #-0x48
    c.b(C::Ne,0x31774)?;
    // 0x317b8: sub r4, r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::new(6)?,32)?)?;
    // 0x317bc: mov r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4)?)?;
    // 0x317c0: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x317c4: and r5, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(7)?,255)?)?;
    // 0x317c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x317cc: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x317d0: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x317d4: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x317d8: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x317dc: bl 0x31830 <entry+0x2230>  @ imm = #0x4c
    c.bl(0x31830)?;
    // 0x317e0: add r3, r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,4)?)?;
    // 0x317e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x317e8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x317ec: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x317f0: str r8, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
    // 0x317f4: bl 0x31830 <entry+0x2230>  @ imm = #0x34
    c.bl(0x31830)?;
    // 0x317f8: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x317fc: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x31800: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x31804: bne 0x317c4 <entry+0x21c4>  @ imm = #-0x48
    c.b(C::Ne,0x317c4)?;
    // 0x31808: ldr r2, [pc, #0x1c]         @ 0x222c <entry+0x222c>
    c.literal(R::R2,0x3182c)?;
    // 0x3180c: mov r1, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,44)?)?;
    // 0x31810: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31814: cmp r6, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,768)?)?;
    // 0x31818: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3181c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x31820: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x31824: pop {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31828: b 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x30080
    c.b(C::Al,0x17b0)?;
    c.check(0x3182c)?;
    const CONTROLLER_REGISTER_TABLE_3182C:[u32;1]=[0x000eafbb];
    for word in CONTROLLER_REGISTER_TABLE_3182C {c.emit(word)?;}
    c.check(0x31830)?;
    // 0x31830: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x31834: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x31838: mov r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,255)?)?;
    // 0x3183c: orr r0, r1, r0, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,12,false)?)?;
    // 0x31840: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x31844: orr r5, r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65280)?)?;
    // 0x31848: and r3, r3, r5
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x3184c: ldr r5, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,8)?)?;
    // 0x31850: orr r4, r3, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(3)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x31854: orr r5, r0, r2, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R0,R::R2,Shift::Lsl,4,false)?)?;
    // 0x31858: beq 0x31898 <entry+0x2298>  @ imm = #0x38
    c.b(C::Eq,0x31898)?;
    // 0x3185c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x31860: bne 0x318c8 <entry+0x22c8>  @ imm = #0x60
    c.b(C::Ne,0x318c8)?;
    // 0x31864: ldr r6, [pc, #0xb0]         @ 0x231c <entry+0x231c>
    c.literal(R::new(6)?,0x3191c)?;
    // 0x31868: ldr r2, [pc, #0xa4]         @ 0x2314 <entry+0x2314>
    c.literal(R::R2,0x31914)?;
    // 0x3186c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x31870: add r1, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2)?)?;
    // 0x31874: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x31878: cmp r1, #56
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,56)?)?;
    // 0x3187c: mov r1, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,118)?)?;
    // 0x31880: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31884: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x31888: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x300e0
    c.bl(0x17b0)?;
    // 0x3188c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x31890: ldr r1, [pc, #0x88]         @ 0x2320 <entry+0x2320>
    c.literal(R::R1,0x31920)?;
    // 0x31894: b 0x318f8 <entry+0x22f8>  @ imm = #0x5c
    c.b(C::Al,0x318f8)?;
    // 0x31898: ldr r6, [pc, #0x70]         @ 0x2310 <entry+0x2310>
    c.literal(R::new(6)?,0x31910)?;
    // 0x3189c: ldr r2, [pc, #0x70]         @ 0x2314 <entry+0x2314>
    c.literal(R::R2,0x31914)?;
    // 0x318a0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x318a4: add r1, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2)?)?;
    // 0x318a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x318ac: cmp r1, #108
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,108)?)?;
    // 0x318b0: mov r1, #892
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,892)?)?;
    // 0x318b4: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x318b8: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x30110
    c.bl(0x17b0)?;
    // 0x318bc: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x318c0: ldr r1, [pc, #0x50]         @ 0x2318 <entry+0x2318>
    c.literal(R::R1,0x31918)?;
    // 0x318c4: b 0x318f8 <entry+0x22f8>  @ imm = #0x2c
    c.b(C::Al,0x318f8)?;
    // 0x318c8: ldr r6, [pc, #0x54]         @ 0x2324 <entry+0x2324>
    c.literal(R::new(6)?,0x31924)?;
    // 0x318cc: ldr r2, [pc, #0x40]         @ 0x2314 <entry+0x2314>
    c.literal(R::R2,0x31914)?;
    // 0x318d0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x318d4: add r1, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2)?)?;
    // 0x318d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x318dc: cmp r1, #172
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,172)?)?;
    // 0x318e0: mov r1, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,130)?)?;
    // 0x318e4: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x318e8: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x318ec: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x30144
    c.bl(0x17b0)?;
    // 0x318f0: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x318f4: ldr r1, [pc, #0x2c]         @ 0x2328 <entry+0x2328>
    c.literal(R::R1,0x31928)?;
    // 0x318f8: str r5, [r1, r0, lsl #2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(5)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x318fc: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x31900: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x31904: add r0, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,4)?)?;
    // 0x31908: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,0)?)?;
    // 0x3190c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x31910)?;
    const CONTROLLER_REGISTER_TABLE_31910:[u32;7]=[0x0010420c,0x000eafbb,0x00104210,0x00104128,0x0010412c,0x001043c0,0x001043c4];
    for word in CONTROLLER_REGISTER_TABLE_31910 {c.emit(word)?;}
    c.check(0x3192c)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;configure_register_writer_and_list(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_buffer_command_lists_preserve_literal_banks(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let get=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(get(0x31240),0x000eb189);
        assert_eq!(get(0x312f8),0x000eafbb);
        assert_eq!(get(0x3182c),0x000eafbb);
        assert_eq!(get(0x31910),0x0010420c);
        assert_eq!(get(0x31928),0x001043c4);
    }
}
