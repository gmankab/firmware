// SPDX-License-Identifier: GPL-3.0-only
//! Controller initialization and dispatch helpers, including setup fallback,
//! bounded descriptor lookup and buffer status checks. Original code is
//! re-encoded from typed A32 operands; interleaved device pointers stay data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2d19c;
const END:usize=0x2d3f4;
fn setup_and_search_controllers(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2d19c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2d1a0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x2d1a4: sub sp, sp, #40
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,40)?)?;
    // 0x2d1a8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2d1ac: add r2, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,8)?)?;
    // 0x2d1b0: ldr r6, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(5)?,16)?)?;
    // 0x2d1b4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x2d1b8: bl 0x2efb8 <entry+0x33b8>  @ imm = #0x1df8
    c.bl(0x2efb8)?;
    // 0x2d1bc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x2d1c0: cmn r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmn,R::R0,R::new(4)?,1)?)?;
    // 0x2d1c4: bne 0x2d23c <entry+0x163c>  @ imm = #0x70
    c.b(C::Ne,0x2d23c)?;
    // 0x2d1c8: ldr r1, [pc, #0xd0]         @ 0x16a0 <entry+0x16a0>
    c.literal(R::R1,0x2d2a0)?;
    // 0x2d1cc: ldr r2, [pc, #0xd0]         @ 0x16a4 <entry+0x16a4>
    c.literal(R::R2,0x2d2a4)?;
    // 0x2d1d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2d1d4: mov r3, #488
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,488)?)?;
    // 0x2d1d8: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x2d1dc: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1ab0
    c.bl(0x2ec94)?;
    // 0x2d1e0: ldr r1, [pc, #0xc0]         @ 0x16a8 <entry+0x16a8>
    c.literal(R::R1,0x2d2a8)?;
    // 0x2d1e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2d1e8: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1aa4
    c.bl(0x2ec94)?;
    // 0x2d1ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d1f0: add r1, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,12)?)?;
    // 0x2d1f4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2d1f8: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // 0x2d1fc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2d200: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x2d204: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2d208: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d20c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2d210: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x2d214: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2d218: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d21c: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2d220: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2d224: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x2d228: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2d22c: ldr r3, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,128)?)?;
    // 0x2d230: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2d234: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2d238: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x30a8
    c.bl(0x2a198)?;
    // 0x2d23c: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2d240: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2d244: bne 0x2d294 <entry+0x1694>  @ imm = #0x48
    c.b(C::Ne,0x2d294)?;
    // 0x2d248: ldr r1, [pc, #0x5c]         @ 0x16ac <entry+0x16ac>
    c.literal(R::R1,0x2d2ac)?;
    // 0x2d24c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d250: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2d254: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1a38
    c.bl(0x2ec94)?;
    // 0x2d258: ldr r1, [pc, #0x48]         @ 0x16a8 <entry+0x16a8>
    c.literal(R::R1,0x2d2a8)?;
    // 0x2d25c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d260: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1a2c
    c.bl(0x2ec94)?;
    // 0x2d264: ldr r1, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,20)?)?;
    // 0x2d268: ldr r0, [pc, #0x40]         @ 0x16b0 <entry+0x16b0>
    c.literal(R::R0,0x2d2b0)?;
    // 0x2d26c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x2d270: beq 0x2d284 <entry+0x1684>  @ imm = #0xc
    c.b(C::Eq,0x2d284)?;
    // 0x2d274: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2d278: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2d27c: streq r1, [r0, #-0x34]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,-52)?)?;
    // 0x2d280: b 0x2d294 <entry+0x1694>  @ imm = #0xc
    c.b(C::Al,0x2d294)?;
    // 0x2d284: str r6, [r0, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,236)?)?;
    // 0x2d288: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2d28c: orr r1, r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2147483648)?)?;
    // 0x2d290: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2d294: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2d298: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x2d29c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2d2a0)?;
    const CONTROLLER_STATE_DATA_2D2A0:[u32;5]=[0x000e98c5,0x000e990c,0x000e2788,0x000e992b,0xe00d204c];
    for word in CONTROLLER_STATE_DATA_2D2A0 {c.emit(word)?;}
    c.check(0x2d2b4)?;
    // 0x2d2b4: push {r4, r5, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2d2b8: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x2d2bc: sub sp, sp, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,32)?)?;
    // 0x2d2c0: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x2d2c4: mvn r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,0)?)?;
    // 0x2d2c8: add r3, r0, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,48)?)?;
    // 0x2d2cc: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x2d2d0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2d2d4: ldr r5, [r3, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(3)?,R::R2,Shift::Lsl,2)?)?;
    // 0x2d2d8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2d2dc: beq 0x2d2ec <entry+0x16ec>  @ imm = #0x8
    c.b(C::Eq,0x2d2ec)?;
    // 0x2d2e0: ldr r5, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,8)?)?;
    // 0x2d2e4: cmp r5, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2d2e8: beq 0x2d36c <entry+0x176c>  @ imm = #0x7c
    c.b(C::Eq,0x2d36c)?;
    // 0x2d2ec: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x2d2f0: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x2d2f4: blo 0x2d2d4 <entry+0x16d4>  @ imm = #-0x28
    c.b(C::Cc,0x2d2d4)?;
    // 0x2d2f8: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2d2fc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x2d300: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2d304: orr r1, r1, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,135168)?)?;
    // 0x2d308: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x2d30c: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x2d310: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x2d314: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x2d318: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x2d31c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x2d320: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x2d324: mov r1, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,28)?)?;
    // 0x2d328: str r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,24)?)?;
    // 0x2d32c: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x2d330: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,4)?)?;
    // 0x2d334: ldr r3, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,128)?)?;
    // 0x2d338: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2d33c: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2d340: bl 0x2a198 <entry+0xffffffffffffe598> @ imm = #-0x31b0
    c.bl(0x2a198)?;
    // 0x2d344: ldr r1, [pc, #0x34]         @ 0x1780 <entry+0x1780>
    c.literal(R::R1,0x2d380)?;
    // 0x2d348: ldr r2, [pc, #0x34]         @ 0x1784 <entry+0x1784>
    c.literal(R::R2,0x2d384)?;
    // 0x2d34c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2d350: mov r3, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,180)?)?;
    // 0x2d354: str r4, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,0)?)?;
    // 0x2d358: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1934
    c.bl(0x2ec94)?;
    // 0x2d35c: ldr r1, [pc, #0x24]         @ 0x1788 <entry+0x1788>
    c.literal(R::R1,0x2d388)?;
    // 0x2d360: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2d364: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1928
    c.bl(0x2ec94)?;
    // 0x2d368: b 0x2d374 <entry+0x1774>  @ imm = #0x4
    c.b(C::Al,0x2d374)?;
    // 0x2d36c: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x2d370: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2d374: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2d378: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x2d37c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2d380)?;
    const CONTROLLER_STATE_DATA_2D380:[u32;3]=[0x000e9955,0x000e9997,0x000e2788];
    for word in CONTROLLER_STATE_DATA_2D380 {c.emit(word)?;}
    c.check(0x2d38c)?;
    // 0x2d38c: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2d390: b 0x2b58 <entry+0xfffffffffffd6f58> @ imm = #-0x2a840
    c.b(C::Al,0x2b58)?;
    // 0x2d394: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x2d398: ldr r0, [pc, #0x10]         @ 0x17b0 <entry+0x17b0>
    c.literal(R::R0,0x2d3b0)?;
    // 0x2d39c: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2d3a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d3a4: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2d3a8: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d3ac: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2d3b0)?;
    const CONTROLLER_STATE_DATA_2D3B0:[u32;1]=[0x0010df9c];
    for word in CONTROLLER_STATE_DATA_2D3B0 {c.emit(word)?;}
    c.check(0x2d3b4)?;
    // 0x2d3b4: mov r2, r0
    c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
    // 0x2d3b8: ldr r0, [pc, #0x30]         @ 0x17f0 <entry+0x17f0>
    c.literal(R::R0,0x2d3f0)?;
    // 0x2d3bc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x2d3c0: ldr r3, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,0)?)?;
    // 0x2d3c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d3c8: bne 0x2d3dc <entry+0x17dc>  @ imm = #0xc
    c.b(C::Ne,0x2d3dc)?;
    // 0x2d3cc: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x2d3d0: andeq r0, r3, #2
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::new(3)?,2)?)?;
    // 0x2d3d4: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x2d3d8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x2d3dc: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x2d3e0: bxne lr
    c.emit(arm32::bx(C::Ne,R::LR))?;
    // 0x2d3e4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x2d3e8: andeq r0, r3, #4
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::new(3)?,4)?)?;
    // 0x2d3ec: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2d3f0)?;
    const CONTROLLER_STATE_DATA_2D3F0:[u32;1]=[0xe00e0008];
    for word in CONTROLLER_STATE_DATA_2D3F0 {c.emit(word)?;}
    c.check(0x2d3f4)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;setup_and_search_controllers(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_resource_setup_lit_words_match(){
       let mut c=Code::new(START);emit(&mut c).unwrap();
       assert_eq!(c.bytes.len(),END-START);
       let word=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
       assert_eq!(word(0x2d2a0),0x000e98c5);
       assert_eq!(word(0x2d2b0),0xe00d204c);
       assert_eq!(word(0x2d380),0x000e9955);
       assert_eq!(word(0x2d3b0),0x0010df9c);
       assert_eq!(word(0x2d3f0),0xe00e0008);
    }
}
