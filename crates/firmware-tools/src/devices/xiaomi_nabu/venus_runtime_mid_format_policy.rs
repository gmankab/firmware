// SPDX-License-Identifier: GPL-3.0-only
//! Controller codec format dispatch and diagnostics.
//!
//! The code first classifies the requested codec/format ID using three
//! distinct switch tables and constructs controller resource descriptors.
//! The 68-word diagnostic/address table is stored as structured runtime data
//! (including externally linked log-address pointers); these values are
//! **not** A32 instructions and are never fetched by the typed emitter.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2a198;
const END:usize=0x2a878;

const PIXEL_FORMAT_DISPATCH:[u32;17]=[
    0x0002a2f8, 0x0002a4c0, 0x0002a4c8, 0x0002a4d0, 0x0002a4d8,
    0x0002a4e0, 0x0002a4e8, 0x0002a4f0, 0x0002a4f8, 0x0002a500,
    0x0002a508, 0x0002a510, 0x0002a518, 0x0002a520, 0x0002a528,
    0x0002a530, 0x0002a538,
];

const STREAM_SIZE_DISPATCH:[u32;14]=[
    0x0002a354, 0x0002a548, 0x0002a550, 0x0002a558, 0x0002a560,
    0x0002a568, 0x0002a570, 0x0002a578, 0x0002a580, 0x0002a588,
    0x0002a590, 0x0002a4b0, 0x0002a598, 0x0002a5a0,
];

const ALT_FORMAT_DISPATCH:[u32;6]=[
    0x0002a390, 0x0002a5a8, 0x0002a5b0, 0x0002a5b8, 0x0002a5c0,
    0x0002a5c8,
];

const CONTROLLER_DEBUG_POINTERS:[u32;68]=[
    0x000e884d, 0x0401ffff, 0xffedf000, 0xffddf000, 0xfbfdf000,
    0x0402109f, 0x040210a0, 0x040210a1, 0x000e9881, 0x000e2788,
    0x000e985e, 0x000e9830, 0x000e9802, 0x000e97d4, 0x000e98a0,
    0x000e97af, 0x000e977f, 0x000e974d, 0x000e9724, 0x000e96f8,
    0x000e96cb, 0x000e96a5, 0x000e967e, 0x000e9656, 0x000e9631,
    0x000e960b, 0x000e95dc, 0x000e95b2, 0x000e9592, 0x000e956e,
    0x000e954e, 0x000e9521, 0x000e94ed, 0x000e94c0, 0x000e9493,
    0x000e9466, 0x000e9442, 0x000e941c, 0x000e93fc, 0x000e93c6,
    0x000e9392, 0x000e936d, 0x000e933d, 0x000e9316, 0x000e92f3,
    0x000e92c7, 0x000e9299, 0x000e926f, 0x000e924a, 0x000e9227,
    0x000e9204, 0x000e91e1, 0x000e91be, 0x000e9198, 0x000e9175,
    0x000e9153, 0x000e9124, 0x000e90f9, 0x000e90d8, 0x000e90a5,
    0x000e9071, 0x000e9049, 0x000e9029, 0x000e98b7, 0xe00d204c,
    0x000e88c4, 0x000e88f8, 0x00221006,
];

/// Dispatch format/codec ID and initialize controller stream parameters.
fn dispatch_codec_policy(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2a198: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2a19c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2a1a0: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x2a1a4: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x2a1a8: mov r4, r3
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(3)?))?;
    // 0x2a1ac: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2a1b0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2a1b4: ldr r6, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,4)?)?;
    // 0x2a1b8: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,12)?)?;
    // 0x2a1bc: beq 0x2a1ec <entry+0xf50>   @ imm = #0x28
    c.b(C::Eq,0x2a1ec)?;
    // 0x2a1c0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x2a1c4: bne 0x2a278 <entry+0xfdc>   @ imm = #0xac
    c.b(C::Ne,0x2a278)?;
    // 0x2a1c8: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,36)?)?;
    // 0x2a1cc: ldr r2, [pc, #0x594]        @ 0x14cc <entry+0x14cc>
    c.literal(R::R2,0x2a768)?;
    // 0x2a1d0: ldr r7, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,44)?)?;
    // 0x2a1d4: mov r1, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25)?)?;
    // 0x2a1d8: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x2a1dc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2a1e0: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28a38
    c.bl(0x17b0)?;
    // 0x2a1e4: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x2a1e8: b 0x2a20c <entry+0xf70>   @ imm = #0x1c
    c.b(C::Al,0x2a20c)?;
    // 0x2a1ec: ldr r0, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,28)?)?;
    // 0x2a1f0: ldr r2, [pc, #0x570]        @ 0x14cc <entry+0x14cc>
    c.literal(R::R2,0x2a768)?;
    // 0x2a1f4: ldr r7, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,40)?)?;
    // 0x2a1f8: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // 0x2a1fc: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x2a200: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2a204: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28a5c
    c.bl(0x17b0)?;
    // 0x2a208: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2a20c: ldr r5, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::new(5)?,R::R0,Shift::Lsl,0)?)?;
    // 0x2a210: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2a214: bl 0x2bf4 <entry+0xfffffffffffd9958> @ imm = #-0x27628
    c.bl(0x2bf4)?;
    // 0x2a218: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x2a21c: ldr r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // 0x2a220: ldr r0, [pc, #0x544]        @ 0x14d0 <entry+0x14d0>
    c.literal(R::R0,0x2a76c)?;
    // 0x2a224: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x2a228: str r6, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,4)?)?;
    // 0x2a22c: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x2a230: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a234: bgt 0x2a28c <entry+0xff0>   @ imm = #0x50
    c.b(C::Gt,0x2a28c)?;
    // 0x2a238: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a23c: orr r0, r0, #2228224
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2228224)?)?;
    // 0x2a240: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a244: bgt 0x2a300 <entry+0x1064>  @ imm = #0xb4
    c.b(C::Gt,0x2a300)?;
    // 0x2a248: cmp r2, #1179648
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1179648)?)?;
    // 0x2a24c: bgt 0x2a35c <entry+0x10c0>  @ imm = #0x108
    c.b(C::Gt,0x2a35c)?;
    // 0x2a250: cmp r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,135168)?)?;
    // 0x2a254: bgt 0x2a398 <entry+0x10fc>  @ imm = #0x13c
    c.b(C::Gt,0x2a398)?;
    // 0x2a258: cmp r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,256)?)?;
    // 0x2a25c: beq 0x2a430 <entry+0x1194>  @ imm = #0x1cc
    c.b(C::Eq,0x2a430)?;
    // 0x2a260: cmp r2, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,131072)?)?;
    // 0x2a264: beq 0x2a438 <entry+0x119c>  @ imm = #0x1cc
    c.b(C::Eq,0x2a438)?;
    // 0x2a268: cmp r2, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,135168)?)?;
    // 0x2a26c: bne 0x2a4b0 <entry+0x1214>  @ imm = #0x23c
    c.b(C::Ne,0x2a4b0)?;
    // 0x2a270: ldr r1, [pc, #0x5e4]        @ 0x15c0 <entry+0x15c0>
    c.literal(R::R1,0x2a85c)?;
    // 0x2a274: b 0x2a60c <entry+0x1370>  @ imm = #0x390
    c.b(C::Al,0x2a60c)?;
    // 0x2a278: ldr r2, [pc, #0x4e8]        @ 0x14cc <entry+0x14cc>
    c.literal(R::R2,0x2a768)?;
    // 0x2a27c: mov r1, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,30)?)?;
    // 0x2a280: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2a284: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x2a288: b 0x2a754 <entry+0x14b8>  @ imm = #0x4c4
    c.b(C::Al,0x2a754)?;
    // 0x2a28c: ldr r0, [pc, #0x4e8]        @ 0x14e0 <entry+0x14e0>
    c.literal(R::R0,0x2a77c)?;
    // 0x2a290: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a294: bgt 0x2a3c8 <entry+0x112c>  @ imm = #0x12c
    c.b(C::Gt,0x2a3c8)?;
    // 0x2a298: ldr r0, [pc, #0x4d8]        @ 0x14dc <entry+0x14dc>
    c.literal(R::R0,0x2a778)?;
    // 0x2a29c: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a2a0: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x2a2a4: bhi 0x2a440 <entry+0x11a4>  @ imm = #0x194
    c.b(C::Hi,0x2a440)?;
    // 0x2a2a8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2a2ac: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2a2b0: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2a2b4)?;
    // Named handler/diagnostic pointers, not executable code.
    for data in PIXEL_FORMAT_DISPATCH {c.emit(data)?;}
    c.check(0x2a2f8)?;
    c.check(0x2a2f8)?;
    // 0x2a2f8: ldr r1, [pc, #0x50c]        @ 0x1570 <entry+0x1570>
    c.literal(R::R1,0x2a80c)?;
    // 0x2a2fc: b 0x2a60c <entry+0x1370>  @ imm = #0x308
    c.b(C::Al,0x2a60c)?;
    // 0x2a300: ldr r0, [pc, #0x46c]        @ 0x14d8 <entry+0x14d8>
    c.literal(R::R0,0x2a774)?;
    // 0x2a304: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a308: cmp r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,13)?)?;
    // 0x2a30c: bhi 0x2a3e8 <entry+0x114c>  @ imm = #0xd4
    c.b(C::Hi,0x2a3e8)?;
    // 0x2a310: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2a314: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2a318: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2a31c)?;
    // Named handler/diagnostic pointers, not executable code.
    for data in STREAM_SIZE_DISPATCH {c.emit(data)?;}
    c.check(0x2a354)?;
    c.check(0x2a354)?;
    // 0x2a354: ldr r1, [pc, #0x480]        @ 0x1540 <entry+0x1540>
    c.literal(R::R1,0x2a7dc)?;
    // 0x2a358: b 0x2a60c <entry+0x1370>  @ imm = #0x2ac
    c.b(C::Al,0x2a60c)?;
    // 0x2a35c: ldr r0, [pc, #0x40c]        @ 0x14d4 <entry+0x14d4>
    c.literal(R::R0,0x2a770)?;
    // 0x2a360: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a364: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x2a368: bhi 0x2a410 <entry+0x1174>  @ imm = #0xa0
    c.b(C::Hi,0x2a410)?;
    // 0x2a36c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2a370: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2a374: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2a378)?;
    // Named handler/diagnostic pointers, not executable code.
    for data in ALT_FORMAT_DISPATCH {c.emit(data)?;}
    c.check(0x2a390)?;
    c.check(0x2a390)?;
    // 0x2a390: ldr r1, [pc, #0x460]        @ 0x155c <entry+0x155c>
    c.literal(R::R1,0x2a7f8)?;
    // 0x2a394: b 0x2a60c <entry+0x1370>  @ imm = #0x270
    c.b(C::Al,0x2a60c)?;
    // 0x2a398: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a39c: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // 0x2a3a0: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a3a4: beq 0x2a458 <entry+0x11bc>  @ imm = #0xac
    c.b(C::Eq,0x2a458)?;
    // 0x2a3a8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2a3ac: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // 0x2a3b0: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a3b4: beq 0x2a4a8 <entry+0x120c>  @ imm = #0xec
    c.b(C::Eq,0x2a4a8)?;
    // 0x2a3b8: cmp r2, #1179648
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1179648)?)?;
    // 0x2a3bc: bne 0x2a4b0 <entry+0x1214>  @ imm = #0xec
    c.b(C::Ne,0x2a4b0)?;
    // 0x2a3c0: ldr r1, [pc, #0x438]        @ 0x1564 <entry+0x1564>
    c.literal(R::R1,0x2a800)?;
    // 0x2a3c4: b 0x2a60c <entry+0x1370>  @ imm = #0x240
    c.b(C::Al,0x2a60c)?;
    // 0x2a3c8: ldr r0, [pc, #0x3b0]        @ 0x14e4 <entry+0x14e4>
    c.literal(R::R0,0x2a780)?;
    // 0x2a3cc: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a3d0: beq 0x2a540 <entry+0x12a4>  @ imm = #0x168
    c.b(C::Eq,0x2a540)?;
    // 0x2a3d4: ldr r0, [pc, #0x3a8]        @ 0x14e8 <entry+0x14e8>
    c.literal(R::R0,0x2a784)?;
    // 0x2a3d8: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a3dc: bne 0x2a4b0 <entry+0x1214>  @ imm = #0xcc
    c.b(C::Ne,0x2a4b0)?;
    // 0x2a3e0: ldr r1, [pc, #0x41c]        @ 0x1568 <entry+0x1568>
    c.literal(R::R1,0x2a804)?;
    // 0x2a3e4: b 0x2a60c <entry+0x1370>  @ imm = #0x220
    c.b(C::Al,0x2a60c)?;
    // 0x2a3e8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2a3ec: orr r0, r0, #2228224
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2228224)?)?;
    // 0x2a3f0: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a3f4: beq 0x2a5d8 <entry+0x133c>  @ imm = #0x1dc
    c.b(C::Eq,0x2a5d8)?;
    // 0x2a3f8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2a3fc: orr r0, r0, #2228224
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2228224)?)?;
    // 0x2a400: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a404: bne 0x2a4b0 <entry+0xb0>    @ imm = #0xa4
    c.b(C::Ne,0x2a4b0)?;
    // 0x2a408: ldr r1, [pc, #0x3c4]        @ 0x3d4 <entry+0x3d4>
    c.literal(R::R1,0x2a7d4)?;
    // 0x2a40c: b 0x2a60c <entry+0x20c>   @ imm = #0x1f8
    c.b(C::Al,0x2a60c)?;
    // 0x2a410: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a414: orr r0, r0, #1179648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1179648)?)?;
    // 0x2a418: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a41c: beq 0x2a5d0 <entry+0x1d0>   @ imm = #0x1ac
    c.b(C::Eq,0x2a5d0)?;
    // 0x2a420: cmp r2, #2228224
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2228224)?)?;
    // 0x2a424: bne 0x2a4b0 <entry+0xb0>    @ imm = #0x84
    c.b(C::Ne,0x2a4b0)?;
    // 0x2a428: ldr r1, [pc, #0x3b0]        @ 0x3e0 <entry+0x3e0>
    c.literal(R::R1,0x2a7e0)?;
    // 0x2a42c: b 0x2a60c <entry+0x20c>   @ imm = #0x1d8
    c.b(C::Al,0x2a60c)?;
    // 0x2a430: ldr r1, [pc, #0x41c]        @ 0x454 <entry+0x454>
    c.literal(R::R1,0x2a854)?;
    // 0x2a434: b 0x2a60c <entry+0x20c>   @ imm = #0x1d0
    c.b(C::Al,0x2a60c)?;
    // 0x2a438: ldr r1, [pc, #0x420]        @ 0x460 <entry+0x460>
    c.literal(R::R1,0x2a860)?;
    // 0x2a43c: b 0x2a60c <entry+0x20c>   @ imm = #0x1c8
    c.b(C::Al,0x2a60c)?;
    // 0x2a440: mov r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,131072)?)?;
    // 0x2a444: orr r0, r0, #67108864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,67108864)?)?;
    // 0x2a448: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a44c: bne 0x2a4b0 <entry+0xb0>    @ imm = #0x5c
    c.b(C::Ne,0x2a4b0)?;
    // 0x2a450: ldr r1, [pc, #0x3f8]        @ 0x450 <entry+0x450>
    c.literal(R::R1,0x2a850)?;
    // 0x2a454: b 0x2a60c <entry+0x20c>   @ imm = #0x1b0
    c.b(C::Al,0x2a60c)?;
    // 0x2a458: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x2a45c: ldr r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,12)?)?;
    // 0x2a460: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2a464: orr r0, r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16777216)?)?;
    // 0x2a468: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a46c: ble 0x2a5e0 <entry+0x1e0>   @ imm = #0x16c
    c.b(C::Le,0x2a5e0)?;
    // 0x2a470: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2a474: orr r0, r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16777216)?)?;
    // 0x2a478: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a47c: beq 0x2a5f8 <entry+0x1f8>   @ imm = #0x174
    c.b(C::Eq,0x2a5f8)?;
    // 0x2a480: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2a484: orr r0, r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16777216)?)?;
    // 0x2a488: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a48c: beq 0x2a600 <entry+0x200>   @ imm = #0x16c
    c.b(C::Eq,0x2a600)?;
    // 0x2a490: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x2a494: orr r0, r0, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,16777216)?)?;
    // 0x2a498: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a49c: bne 0x2a760 <entry+0x360>   @ imm = #0x2bc
    c.b(C::Ne,0x2a760)?;
    // 0x2a4a0: ldr r1, [pc, #0x2f4]        @ 0x39c <entry+0x39c>
    c.literal(R::R1,0x2a79c)?;
    // 0x2a4a4: b 0x2a60c <entry+0x20c>   @ imm = #0x160
    c.b(C::Al,0x2a60c)?;
    // 0x2a4a8: ldr r1, [pc, #0x3a8]        @ 0x458 <entry+0x458>
    c.literal(R::R1,0x2a858)?;
    // 0x2a4ac: b 0x2a60c <entry+0x20c>   @ imm = #0x158
    c.b(C::Al,0x2a60c)?;
    // 0x2a4b0: ldr r1, [pc, #0x3ac]        @ 0x464 <entry+0x464>
    c.literal(R::R1,0x2a864)?;
    // 0x2a4b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a4b8: bl 0x2ec94 <entry+0x4894>  @ imm = #0x47d4
    c.bl(0x2ec94)?;
    // 0x2a4bc: b 0x2a614 <entry+0x214>   @ imm = #0x150
    c.b(C::Al,0x2a614)?;
    // 0x2a4c0: ldr r1, [pc, #0x384]        @ 0x44c <entry+0x44c>
    c.literal(R::R1,0x2a84c)?;
    // 0x2a4c4: b 0x2a60c <entry+0x20c>   @ imm = #0x140
    c.b(C::Al,0x2a60c)?;
    // 0x2a4c8: ldr r1, [pc, #0x378]        @ 0x448 <entry+0x448>
    c.literal(R::R1,0x2a848)?;
    // 0x2a4cc: b 0x2a60c <entry+0x20c>   @ imm = #0x138
    c.b(C::Al,0x2a60c)?;
    // 0x2a4d0: ldr r1, [pc, #0x36c]        @ 0x444 <entry+0x444>
    c.literal(R::R1,0x2a844)?;
    // 0x2a4d4: b 0x2a60c <entry+0x20c>   @ imm = #0x130
    c.b(C::Al,0x2a60c)?;
    // 0x2a4d8: ldr r1, [pc, #0x360]        @ 0x440 <entry+0x440>
    c.literal(R::R1,0x2a840)?;
    // 0x2a4dc: b 0x2a60c <entry+0x20c>   @ imm = #0x128
    c.b(C::Al,0x2a60c)?;
    // 0x2a4e0: ldr r1, [pc, #0x354]        @ 0x43c <entry+0x43c>
    c.literal(R::R1,0x2a83c)?;
    // 0x2a4e4: b 0x2a60c <entry+0x20c>   @ imm = #0x120
    c.b(C::Al,0x2a60c)?;
    // 0x2a4e8: ldr r1, [pc, #0x348]        @ 0x438 <entry+0x438>
    c.literal(R::R1,0x2a838)?;
    // 0x2a4ec: b 0x2a60c <entry+0x20c>   @ imm = #0x118
    c.b(C::Al,0x2a60c)?;
    // 0x2a4f0: ldr r1, [pc, #0x33c]        @ 0x434 <entry+0x434>
    c.literal(R::R1,0x2a834)?;
    // 0x2a4f4: b 0x2a60c <entry+0x20c>   @ imm = #0x110
    c.b(C::Al,0x2a60c)?;
    // 0x2a4f8: ldr r1, [pc, #0x330]        @ 0x430 <entry+0x430>
    c.literal(R::R1,0x2a830)?;
    // 0x2a4fc: b 0x2a60c <entry+0x20c>   @ imm = #0x108
    c.b(C::Al,0x2a60c)?;
    // 0x2a500: ldr r1, [pc, #0x324]        @ 0x42c <entry+0x42c>
    c.literal(R::R1,0x2a82c)?;
    // 0x2a504: b 0x2a60c <entry+0x20c>   @ imm = #0x100
    c.b(C::Al,0x2a60c)?;
    // 0x2a508: ldr r1, [pc, #0x318]        @ 0x428 <entry+0x428>
    c.literal(R::R1,0x2a828)?;
    // 0x2a50c: b 0x2a60c <entry+0x20c>   @ imm = #0xf8
    c.b(C::Al,0x2a60c)?;
    // 0x2a510: ldr r1, [pc, #0x30c]        @ 0x424 <entry+0x424>
    c.literal(R::R1,0x2a824)?;
    // 0x2a514: b 0x2a60c <entry+0x20c>   @ imm = #0xf0
    c.b(C::Al,0x2a60c)?;
    // 0x2a518: ldr r1, [pc, #0x300]        @ 0x420 <entry+0x420>
    c.literal(R::R1,0x2a820)?;
    // 0x2a51c: b 0x2a60c <entry+0x20c>   @ imm = #0xe8
    c.b(C::Al,0x2a60c)?;
    // 0x2a520: ldr r1, [pc, #0x2f4]        @ 0x41c <entry+0x41c>
    c.literal(R::R1,0x2a81c)?;
    // 0x2a524: b 0x2a60c <entry+0x20c>   @ imm = #0xe0
    c.b(C::Al,0x2a60c)?;
    // 0x2a528: ldr r1, [pc, #0x2e8]        @ 0x418 <entry+0x418>
    c.literal(R::R1,0x2a818)?;
    // 0x2a52c: b 0x2a60c <entry+0x20c>   @ imm = #0xd8
    c.b(C::Al,0x2a60c)?;
    // 0x2a530: ldr r1, [pc, #0x2dc]        @ 0x414 <entry+0x414>
    c.literal(R::R1,0x2a814)?;
    // 0x2a534: b 0x2a60c <entry+0x20c>   @ imm = #0xd0
    c.b(C::Al,0x2a60c)?;
    // 0x2a538: ldr r1, [pc, #0x2d0]        @ 0x410 <entry+0x410>
    c.literal(R::R1,0x2a810)?;
    // 0x2a53c: b 0x2a60c <entry+0x20c>   @ imm = #0xc8
    c.b(C::Al,0x2a60c)?;
    // 0x2a540: ldr r1, [pc, #0x2c0]        @ 0x408 <entry+0x408>
    c.literal(R::R1,0x2a808)?;
    // 0x2a544: b 0x2a60c <entry+0x20c>   @ imm = #0xc0
    c.b(C::Al,0x2a60c)?;
    // 0x2a548: ldr r1, [pc, #0x280]        @ 0x3d0 <entry+0x3d0>
    c.literal(R::R1,0x2a7d0)?;
    // 0x2a54c: b 0x2a60c <entry+0x20c>   @ imm = #0xb8
    c.b(C::Al,0x2a60c)?;
    // 0x2a550: ldr r1, [pc, #0x274]        @ 0x3cc <entry+0x3cc>
    c.literal(R::R1,0x2a7cc)?;
    // 0x2a554: b 0x2a60c <entry+0x20c>   @ imm = #0xb0
    c.b(C::Al,0x2a60c)?;
    // 0x2a558: ldr r1, [pc, #0x268]        @ 0x3c8 <entry+0x3c8>
    c.literal(R::R1,0x2a7c8)?;
    // 0x2a55c: b 0x2a60c <entry+0x20c>   @ imm = #0xa8
    c.b(C::Al,0x2a60c)?;
    // 0x2a560: ldr r1, [pc, #0x25c]        @ 0x3c4 <entry+0x3c4>
    c.literal(R::R1,0x2a7c4)?;
    // 0x2a564: b 0x2a60c <entry+0x20c>   @ imm = #0xa0
    c.b(C::Al,0x2a60c)?;
    // 0x2a568: ldr r1, [pc, #0x250]        @ 0x3c0 <entry+0x3c0>
    c.literal(R::R1,0x2a7c0)?;
    // 0x2a56c: b 0x2a60c <entry+0x20c>   @ imm = #0x98
    c.b(C::Al,0x2a60c)?;
    // 0x2a570: ldr r1, [pc, #0x244]        @ 0x3bc <entry+0x3bc>
    c.literal(R::R1,0x2a7bc)?;
    // 0x2a574: b 0x2a60c <entry+0x20c>   @ imm = #0x90
    c.b(C::Al,0x2a60c)?;
    // 0x2a578: ldr r1, [pc, #0x238]        @ 0x3b8 <entry+0x3b8>
    c.literal(R::R1,0x2a7b8)?;
    // 0x2a57c: b 0x2a60c <entry+0x20c>   @ imm = #0x88
    c.b(C::Al,0x2a60c)?;
    // 0x2a580: ldr r1, [pc, #0x22c]        @ 0x3b4 <entry+0x3b4>
    c.literal(R::R1,0x2a7b4)?;
    // 0x2a584: b 0x2a60c <entry+0x20c>   @ imm = #0x80
    c.b(C::Al,0x2a60c)?;
    // 0x2a588: ldr r1, [pc, #0x220]        @ 0x3b0 <entry+0x3b0>
    c.literal(R::R1,0x2a7b0)?;
    // 0x2a58c: b 0x2a60c <entry+0x20c>   @ imm = #0x78
    c.b(C::Al,0x2a60c)?;
    // 0x2a590: ldr r1, [pc, #0x214]        @ 0x3ac <entry+0x3ac>
    c.literal(R::R1,0x2a7ac)?;
    // 0x2a594: b 0x2a60c <entry+0x20c>   @ imm = #0x70
    c.b(C::Al,0x2a60c)?;
    // 0x2a598: ldr r1, [pc, #0x208]        @ 0x3a8 <entry+0x3a8>
    c.literal(R::R1,0x2a7a8)?;
    // 0x2a59c: b 0x2a60c <entry+0x20c>   @ imm = #0x68
    c.b(C::Al,0x2a60c)?;
    // 0x2a5a0: ldr r1, [pc, #0x1fc]        @ 0x3a4 <entry+0x3a4>
    c.literal(R::R1,0x2a7a4)?;
    // 0x2a5a4: b 0x2a60c <entry+0x20c>   @ imm = #0x60
    c.b(C::Al,0x2a60c)?;
    // 0x2a5a8: ldr r1, [pc, #0x244]        @ 0x3f4 <entry+0x3f4>
    c.literal(R::R1,0x2a7f4)?;
    // 0x2a5ac: b 0x2a60c <entry+0x20c>   @ imm = #0x58
    c.b(C::Al,0x2a60c)?;
    // 0x2a5b0: ldr r1, [pc, #0x238]        @ 0x3f0 <entry+0x3f0>
    c.literal(R::R1,0x2a7f0)?;
    // 0x2a5b4: b 0x2a60c <entry+0x20c>   @ imm = #0x50
    c.b(C::Al,0x2a60c)?;
    // 0x2a5b8: ldr r1, [pc, #0x22c]        @ 0x3ec <entry+0x3ec>
    c.literal(R::R1,0x2a7ec)?;
    // 0x2a5bc: b 0x2a60c <entry+0x20c>   @ imm = #0x48
    c.b(C::Al,0x2a60c)?;
    // 0x2a5c0: ldr r1, [pc, #0x220]        @ 0x3e8 <entry+0x3e8>
    c.literal(R::R1,0x2a7e8)?;
    // 0x2a5c4: b 0x2a60c <entry+0x20c>   @ imm = #0x40
    c.b(C::Al,0x2a60c)?;
    // 0x2a5c8: ldr r1, [pc, #0x214]        @ 0x3e4 <entry+0x3e4>
    c.literal(R::R1,0x2a7e4)?;
    // 0x2a5cc: b 0x2a60c <entry+0x20c>   @ imm = #0x38
    c.b(C::Al,0x2a60c)?;
    // 0x2a5d0: ldr r1, [pc, #0x224]        @ 0x3fc <entry+0x3fc>
    c.literal(R::R1,0x2a7fc)?;
    // 0x2a5d4: b 0x2a60c <entry+0x20c>   @ imm = #0x30
    c.b(C::Al,0x2a60c)?;
    // 0x2a5d8: ldr r1, [pc, #0x1f8]        @ 0x3d8 <entry+0x3d8>
    c.literal(R::R1,0x2a7d8)?;
    // 0x2a5dc: b 0x2a60c <entry+0x20c>   @ imm = #0x28
    c.b(C::Al,0x2a60c)?;
    // 0x2a5e0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x2a5e4: beq 0x2a608 <entry+0x208>   @ imm = #0x1c
    c.b(C::Eq,0x2a608)?;
    // 0x2a5e8: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x2a5ec: bne 0x2a760 <entry+0x360>   @ imm = #0x16c
    c.b(C::Ne,0x2a760)?;
    // 0x2a5f0: ldr r1, [pc, #0x198]        @ 0x390 <entry+0x390>
    c.literal(R::R1,0x2a790)?;
    // 0x2a5f4: b 0x2a60c <entry+0x20c>   @ imm = #0x10
    c.b(C::Al,0x2a60c)?;
    // 0x2a5f8: ldr r1, [pc, #0x198]        @ 0x398 <entry+0x398>
    c.literal(R::R1,0x2a798)?;
    // 0x2a5fc: b 0x2a60c <entry+0x20c>   @ imm = #0x8
    c.b(C::Al,0x2a60c)?;
    // 0x2a600: ldr r1, [pc, #0x18c]        @ 0x394 <entry+0x394>
    c.literal(R::R1,0x2a794)?;
    // 0x2a604: b 0x2a60c <entry+0x20c>   @ imm = #0x0
    c.b(C::Al,0x2a60c)?;
    // 0x2a608: ldr r1, [pc, #0x178]        @ 0x388 <entry+0x388>
    c.literal(R::R1,0x2a788)?;
    // 0x2a60c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a610: bl 0x2ec94 <entry+0x4894>  @ imm = #0x467c
    c.bl(0x2ec94)?;
    // 0x2a614: ldr r1, [pc, #0x170]        @ 0x38c <entry+0x38c>
    c.literal(R::R1,0x2a78c)?;
    // 0x2a618: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a61c: bl 0x2ec94 <entry+0x4894>  @ imm = #0x4670
    c.bl(0x2ec94)?;
    // 0x2a620: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2a624: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x2a628: add r2, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,24)?)?;
    // 0x2a62c: bl 0x2f1e4 <entry+0x4de4>  @ imm = #0x4bb0
    c.bl(0x2f1e4)?;
    // 0x2a630: ldr r6, [pc, #0x230]        @ 0x468 <entry+0x468>
    c.literal(R::new(6)?,0x2a868)?;
    // 0x2a634: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a638: bne 0x2a6d8 <entry+0x2d8>   @ imm = #0x98
    c.b(C::Ne,0x2a6d8)?;
    // 0x2a63c: ldr r7, [pc, #0x228]        @ 0x46c <entry+0x46c>
    c.literal(R::new(7)?,0x2a86c)?;
    // 0x2a640: ldr r8, [pc, #0x228]        @ 0x470 <entry+0x470>
    c.literal(R::new(8)?,0x2a870)?;
    // 0x2a644: ldr r9, [pc, #0x140]        @ 0x38c <entry+0x38c>
    c.literal(R::new(9)?,0x2a78c)?;
    // 0x2a648: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2a64c: add r10, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::SP,24)?)?;
    // 0x2a650: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2a654: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2a658: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x2a65c: mov r3, #324
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,324)?)?;
    // 0x2a660: bl 0x2ec94 <entry+0x4894>  @ imm = #0x462c
    c.bl(0x2ec94)?;
    // 0x2a664: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2a668: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x2a66c: bl 0x2ec94 <entry+0x4894>  @ imm = #0x4620
    c.bl(0x2ec94)?;
    // 0x2a670: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2a674: bl 0x2e84 <entry+0xfffffffffffd8a84> @ imm = #-0x277f8
    c.bl(0x2e84)?;
    // 0x2a678: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2a67c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a680: beq 0x2a69c <entry+0x29c>   @ imm = #0x14
    c.b(C::Eq,0x2a69c)?;
    // 0x2a684: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2a688: streq r4, [r6, #0xec]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(6)?,236)?)?;
    // 0x2a68c: ldreq r0, [r6]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(6)?,0)?)?;
    // 0x2a690: orreq r0, r0, #-2147483648
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // 0x2a694: streq r0, [r6]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(6)?,0)?)?;
    // 0x2a698: b 0x2a6a0 <entry+0x2a0>   @ imm = #0x0
    c.b(C::Al,0x2a6a0)?;
    // 0x2a69c: str r4, [r6, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(6)?,-52)?)?;
    // 0x2a6a0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2a6a4: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // 0x2a6a8: bl 0x2e40 <entry+0xfffffffffffd8a40> @ imm = #-0x27870
    c.bl(0x2e40)?;
    // 0x2a6ac: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2a6b0: mov r1, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16777216)?)?;
    // 0x2a6b4: bl 0x2dbc <entry+0xfffffffffffd89bc> @ imm = #-0x27900
    c.bl(0x2dbc)?;
    // 0x2a6b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2a6bc: bl 0x2ec8 <entry+0xfffffffffffd8ac8> @ imm = #-0x277fc
    c.bl(0x2ec8)?;
    // 0x2a6c0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2a6c4: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,20)?)?;
    // 0x2a6c8: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2a6cc: bl 0x2f1e4 <entry+0x4de4>  @ imm = #0x4b10
    c.bl(0x2f1e4)?;
    // 0x2a6d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a6d4: beq 0x2a650 <entry+0x250>   @ imm = #-0x8c
    c.b(C::Eq,0x2a650)?;
    // 0x2a6d8: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x2a6dc: bl 0x2c34 <entry+0xfffffffffffd8834> @ imm = #-0x27ab0
    c.bl(0x2c34)?;
    // 0x2a6e0: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,8)?)?;
    // 0x2a6e4: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x2a6e8: ldr r1, [pc, #0x184]        @ 0x474 <entry+0x474>
    c.literal(R::R1,0x2a874)?;
    // 0x2a6ec: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x2a6f0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x2a6f4: movne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2a6f8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2a6fc: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2a700: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2a704: andne r0, r2, r0
    c.emit(arm32::data_register(C::Ne,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a708: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2a70c: bne 0x2a758 <entry+0x358>   @ imm = #0x44
    c.b(C::Ne,0x2a758)?;
    // 0x2a710: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2a714: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a718: beq 0x2a73c <entry+0x33c>   @ imm = #0x1c
    c.b(C::Eq,0x2a73c)?;
    // 0x2a71c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2a720: bne 0x2a748 <entry+0x348>   @ imm = #0x20
    c.b(C::Ne,0x2a748)?;
    // 0x2a724: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a728: str r0, [r6, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,236)?)?;
    // 0x2a72c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,0)?)?;
    // 0x2a730: orr r0, r0, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2147483648)?)?;
    // 0x2a734: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x2a738: b 0x2a758 <entry+0x358>   @ imm = #0x18
    c.b(C::Al,0x2a758)?;
    // 0x2a73c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2a740: str r0, [r6, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,-52)?)?;
    // 0x2a744: b 0x2a758 <entry+0x358>   @ imm = #0xc
    c.b(C::Al,0x2a758)?;
    // 0x2a748: ldr r2, [pc, #0x18]         @ 0x368 <entry+0x368>
    c.literal(R::R2,0x2a768)?;
    // 0x2a74c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2a750: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // 0x2a754: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x28fac
    c.bl(0x17b0)?;
    // 0x2a758: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2a75c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2a760: ldr r1, [pc, #0x38]         @ 0x3a0 <entry+0x3a0>
    c.literal(R::R1,0x2a7a0)?;
    // 0x2a764: b 0x2a4b4 <entry+0xb4>    @ imm = #-0x2b8
    c.b(C::Al,0x2a4b4)?;
    c.check(0x2a768)?;
    // Named handler/diagnostic pointers, not executable code.
    for data in CONTROLLER_DEBUG_POINTERS {c.emit(data)?;}
    c.check(0x2a878)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{dispatch_codec_policy(c)}
#[cfg(test)]mod tests{
    use super::*;
    #[test] fn triple_switch_and_diagnostic_pool_boundaries(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(word(0x2a2b4),PIXEL_FORMAT_DISPATCH[0]);
        assert_eq!(word(0x2a2f4),PIXEL_FORMAT_DISPATCH[16]);
        assert_eq!(word(0x2a31c),STREAM_SIZE_DISPATCH[0]);
        assert_eq!(word(0x2a350),STREAM_SIZE_DISPATCH[13]);
        assert_eq!(word(0x2a378),ALT_FORMAT_DISPATCH[0]);
        assert_eq!(word(0x2a38c),ALT_FORMAT_DISPATCH[5]);
        assert_eq!(word(0x2a768),CONTROLLER_DEBUG_POINTERS[0]);
        assert_eq!(word(0x2a874),CONTROLLER_DEBUG_POINTERS[67]);
    }
}
