// SPDX-License-Identifier: GPL-3.0-only
//! Dispatch codec frame class for 18 controller message types, initialize
//! frame-plane flags and zero/initialize the per-stream descriptor matrix.
//! Branch targets and message constants are data, never decoded as opcodes.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x3580c;
const END:usize=0x35bec;
const FRAME_CLASS_HANDLERS:[u32;18]=[
0x00035888, 0x00035878, 0x000358a0, 0x00035878, 0x00035878, 0x000358bc, 0x000358bc, 0x000358bc, 0x00035904, 0x00035878, 0x00035878, 0x000358d4, 0x00035888, 0x00035878, 0x00035878, 0x000358f8, 0x00035894, 0x00035894];
const FRAME_INVALID_CLASS_DIAGNOSTIC:u32=0x000e_c0b3;
const DEFAULT_CODEC_FORMAT:u32=0x0101_0200;

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3580c: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35810: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35814: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35818: ldrb r0, [r4, #0x1e8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,488)?)?;
    // 0x3581c: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x35820: bhi 0x358bc <entry+0x430>   @ imm = #0x94
    c.b(C::Hi,0x358bc)?;
    // 0x35824: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x35828: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x3582c: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x35830)?;
    for handler in FRAME_CLASS_HANDLERS {c.emit(handler)?;}
    c.check(0x35878)?;
    // 0x35878: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x3587c: str r0, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,904)?)?;
    // 0x35880: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x35884: b 0x35910 <entry+0x484>   @ imm = #0x84
    c.b(C::Al,0x35910)?;
    // 0x35888: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3588c: str r0, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,904)?)?;
    // 0x35890: b 0x35910 <entry+0x484>   @ imm = #0x78
    c.b(C::Al,0x35910)?;
    // 0x35894: ldr r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,284)?)?;
    // 0x35898: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x3589c: b 0x358b0 <entry+0x424>   @ imm = #0xc
    c.b(C::Al,0x358b0)?;
    // 0x358a0: ldr r0, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,284)?)?;
    // 0x358a4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x358a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x358ac: movne r1, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x358b0: str r1, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,904)?)?;
    // 0x358b4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x358b8: b 0x35910 <entry+0x484>   @ imm = #0x50
    c.b(C::Al,0x35910)?;
    // 0x358bc: ldr r2, [pc, #0x168]        @ 0x5a0 <entry+0x5a0>
    c.literal_cond(C::Al,R::R2,0x35a2c)?;
    // 0x358c0: mov r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    // 0x358c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x358c8: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x358cc: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x34124
    c.bl(0x17b0)?;
    // 0x358d0: b 0x35880 <entry+0x3f4>   @ imm = #-0x58
    c.b(C::Al,0x35880)?;
    // 0x358d4: ldr r0, [r4, #0x82c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2092)?)?;
    // 0x358d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x358dc: beq 0x35a24 <entry+0x598>   @ imm = #0x140
    c.b(C::Eq,0x35a24)?;
    // 0x358e0: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x358e4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x358e8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x358ec: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x358f0: str r1, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,904)?)?;
    // 0x358f4: b 0x35880 <entry+0x3f4>   @ imm = #-0x7c
    c.b(C::Al,0x35880)?;
    // 0x358f8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x358fc: str r0, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,904)?)?;
    // 0x35900: b 0x35910 <entry+0x484>   @ imm = #0x8
    c.b(C::Al,0x35910)?;
    // 0x35904: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x35908: str r0, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,904)?)?;
    // 0x3590c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x35910: ldr r1, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x35914: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x35918: bge 0x35934 <entry+0x4a8>   @ imm = #0x14
    c.b(C::Ge,0x35934)?;
    // 0x3591c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x35920: beq 0x35950 <entry+0x4c4>   @ imm = #0x28
    c.b(C::Eq,0x35950)?;
    // 0x35924: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x35928: bne 0x35a08 <entry+0x57c>   @ imm = #0xd8
    c.b(C::Ne,0x35a08)?;
    // 0x3592c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x35930: b 0x35944 <entry+0x4b8>   @ imm = #0xc
    c.b(C::Al,0x35944)?;
    // 0x35934: beq 0x35950 <entry+0x4c4>   @ imm = #0x14
    c.b(C::Eq,0x35950)?;
    // 0x35938: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x3593c: bne 0x35a08 <entry+0x57c>   @ imm = #0xc4
    c.b(C::Ne,0x35a08)?;
    // 0x35940: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x35944: str r0, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,904)?)?;
    // 0x35948: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3594c: b 0x35a08 <entry+0x57c>   @ imm = #0xb4
    c.b(C::Al,0x35a08)?;
    // 0x35950: ldrb r2, [r4, #0x1e8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,488)?)?;
    // 0x35954: cmp r2, #11
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,11)?)?;
    // 0x35958: bne 0x35988 <entry+0x4fc>   @ imm = #0x28
    c.b(C::Ne,0x35988)?;
    // 0x3595c: ldr r2, [r4, #0x254]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,596)?)?;
    // 0x35960: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x35964: bne 0x359b0 <entry+0x524>   @ imm = #0x44
    c.b(C::Ne,0x359b0)?;
    // 0x35968: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x3596c: beq 0x359bc <entry+0x530>   @ imm = #0x48
    c.b(C::Eq,0x359bc)?;
    // 0x35970: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x35974: bne 0x35a1c <entry+0x590>   @ imm = #0xa0
    c.b(C::Ne,0x35a1c)?;
    // 0x35978: ldrb r1, [r4, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,737)?)?;
    // 0x3597c: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x35980: lsr r1, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,1)?)?;
    // 0x35984: b 0x359e8 <entry+0x55c>   @ imm = #0x5c
    c.b(C::Al,0x359e8)?;
    // 0x35988: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x3598c: beq 0x359a0 <entry+0x514>   @ imm = #0xc
    c.b(C::Eq,0x359a0)?;
    // 0x35990: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x35994: ldreq r1, [r4, #0x82c]
    c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::new(4)?,2092)?)?;
    // 0x35998: cmpeq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3599c: bne 0x35a08 <entry+0x57c>   @ imm = #0x64
    c.b(C::Ne,0x35a08)?;
    // 0x359a0: ldr r1, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,904)?)?;
    // 0x359a4: add r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // 0x359a8: strb r1, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,357)?)?;
    // 0x359ac: b 0x35a08 <entry+0x57c>   @ imm = #0x54
    c.b(C::Al,0x35a08)?;
    // 0x359b0: ldr r1, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,904)?)?;
    // 0x359b4: add r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,255)?)?;
    // 0x359b8: b 0x359e8 <entry+0x55c>   @ imm = #0x28
    c.b(C::Al,0x359e8)?;
    // 0x359bc: ldrb r2, [r4, #0x2e1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,737)?)?;
    // 0x359c0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x359c4: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x359c8: addhi r1, r2, #255
    c.emit(arm32::data_immediate(C::Hi,Op::Add,R::R1,R::R2,255)?)?;
    // 0x359cc: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4)?)?;
    // 0x359d0: strb r1, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,357)?)?;
    // 0x359d4: beq 0x359e0 <entry+0x554>   @ imm = #0x4
    c.b(C::Eq,0x359e0)?;
    // 0x359d8: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,5)?)?;
    // 0x359dc: blo 0x359ec <entry+0x560>   @ imm = #0x8
    c.b(C::Cc,0x359ec)?;
    // 0x359e0: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x359e4: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x359e8: strb r1, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,357)?)?;
    // 0x359ec: ldr r2, [r4, #0x110]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,272)?)?;
    // 0x359f0: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x359f4: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x359f8: strb r1, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,357)?)?;
    // 0x359fc: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x35a00: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x35a04: str r1, [r4, #0x388]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,904)?)?;
    // 0x35a08: ldrb r1, [r4, #0x15f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,351)?)?;
    // 0x35a0c: ldrb r2, [r4, #0x160]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,352)?)?;
    // 0x35a10: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x35a14: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x35a18: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35a1c: ldrb r1, [r4, #0x165]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,357)?)?;
    // 0x35a20: b 0x359ec <entry+0x560>   @ imm = #-0x3c
    c.b(C::Al,0x359ec)?;
    // 0x35a24: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x35a28: b 0x3587c <entry+0x3f0>   @ imm = #-0x1b4
    c.b(C::Al,0x3587c)?;
    c.check(0x35a2c)?;
    c.emit(FRAME_INVALID_CLASS_DIAGNOSTIC)?;
    c.check(0x35a30)?;
    // 0x35a30: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35a34: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35a38: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35a3c: mov r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,63)?)?;
    // 0x35a40: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35a44: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x35a48: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x35a4c: str r0, [r4, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,296)?)?;
    // 0x35a50: mov r0, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,983040)?)?;
    // 0x35a54: str r0, [r4, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,332)?)?;
    // 0x35a58: add r0, r4, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,300)?)?;
    // 0x35a5c: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x32878
    c.bl(0x31ec)?;
    // 0x35a60: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x35a64: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35a68: mov r2, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,40)?)?;
    // 0x35a6c: str r0, [r4, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,336)?)?;
    // 0x35a70: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x35a74: str r5, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,340)?)?;
    // 0x35a78: str r5, [r4, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,344)?)?;
    // 0x35a7c: str r5, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,420)?)?;
    // 0x35a80: str r5, [r4, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,452)?)?;
    // 0x35a84: str r0, [r4, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,436)?)?;
    // 0x35a88: add r0, r4, #352
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,352)?)?;
    // 0x35a8c: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x328a8
    c.bl(0x31ec)?;
    // 0x35a90: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x35a94: str r0, [r4, #0xcb8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,3256)?)?;
    // 0x35a98: ldr r0, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,280)?)?;
    // 0x35a9c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35aa0: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x35aa4: streq r0, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,348)?)?;
    // 0x35aa8: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35aac: mov r12, #252706816
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,252706816)?)?;
    // 0x35ab0: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x35ab4: orr r12, r12, #1879048192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,1879048192)?)?;
    // 0x35ab8: str r12, [r0, #0x588]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R0,1416)?)?;
    // 0x35abc: add r1, r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,2)?)?;
    // 0x35ac0: ldr r2, [pc, #0x64]         @ 0x6a0 <entry+0x6a0>
    c.literal_cond(C::Al,R::R2,0x35b2c)?;
    // 0x35ac4: str r2, [r0, #0x580]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1408)?)?;
    // 0x35ac8: str r3, [r0, #0x584]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1412)?)?;
    // 0x35acc: str r1, [r0, #0x5a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1440)?)?;
    // 0x35ad0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x35ad4: str r2, [r0, #0x598]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1432)?)?;
    // 0x35ad8: str r1, [r0, #0x59c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1436)?)?;
    // 0x35adc: str r3, [r0, #0x594]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1428)?)?;
    // 0x35ae0: str r2, [r0, #0x58c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1420)?)?;
    // 0x35ae4: str r1, [r0, #0x590]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1424)?)?;
    // 0x35ae8: add r1, r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,4)?)?;
    // 0x35aec: str r1, [r0, #0x5ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1452)?)?;
    // 0x35af0: add r1, r12, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,5)?)?;
    // 0x35af4: str r2, [r0, #0x5a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1444)?)?;
    // 0x35af8: str r3, [r0, #0x5a8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1448)?)?;
    // 0x35afc: str r1, [r0, #0x5b8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1464)?)?;
    // 0x35b00: add r1, r12, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,8)?)?;
    // 0x35b04: str r2, [r0, #0x5b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1456)?)?;
    // 0x35b08: str r3, [r0, #0x5b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1460)?)?;
    // 0x35b0c: str r1, [r0, #0x5c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1476)?)?;
    // 0x35b10: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x35b14: str r2, [r0, #0x5bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1468)?)?;
    // 0x35b18: str r3, [r0, #0x5c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1472)?)?;
    // 0x35b1c: str r2, [r0, #0x5d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1492)?)?;
    // 0x35b20: str r3, [r0, #0x5d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1496)?)?;
    // 0x35b24: str r1, [r0, #0x5dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,1500)?)?;
    // 0x35b28: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x35b2c)?;
    c.emit(DEFAULT_CODEC_FORMAT)?;
    c.check(0x35b30)?;
    // 0x35b30: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35b34: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x35b38: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35b3c: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,8)?)?;
    // 0x35b40: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x35b44: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x35b48: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x35b4c: moveq r0, r3
    c.emit(arm32::mov_register(C::Eq,R::R0,R::new(3)?))?;
    // 0x35b50: str r3, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,4)?)?;
    // 0x35b54: str r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,8)?)?;
    // 0x35b58: ldr r0, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,12)?)?;
    // 0x35b5c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35b60: beq 0x35b7c <entry+0x6f0>   @ imm = #0x14
    c.b(C::Eq,0x35b7c)?;
    // 0x35b64: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x35b68: bl 0x2b8c <entry+0xfffffffffffcd700> @ imm = #-0x32fe4
    c.bl(0x2b8c)?;
    // 0x35b6c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x35b70: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35b74: bne 0x35be0 <entry+0x754>   @ imm = #0x64
    c.b(C::Ne,0x35be0)?;
    // 0x35b78: b 0x35b84 <entry+0x6f8>   @ imm = #0x4
    c.b(C::Al,0x35b84)?;
    // 0x35b7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35b80: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x35b84: mul r0, r5, r6
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(5)?,R::new(6)?)?)?;
    // 0x35b88: str r5, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,16)?)?;
    // 0x35b8c: str r6, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,20)?)?;
    // 0x35b90: bl 0x2efc <entry+0xfffffffffffcda70> @ imm = #-0x32c9c
    c.bl(0x2efc)?;
    // 0x35b94: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35b98: str r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,12)?)?;
    // 0x35b9c: beq 0x35bc8 <entry+0x73c>   @ imm = #0x24
    c.b(C::Eq,0x35bc8)?;
    // 0x35ba0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35ba4: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x35ba8: strb r1, [r0, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,3)?)?;
    // 0x35bac: strb r1, [r0, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,2)?)?;
    // 0x35bb0: strb r1, [r0, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1)?)?;
    // 0x35bb4: strb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x35bb8: str r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,24)?)?;
    // 0x35bbc: str r1, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,28)?)?;
    // 0x35bc0: str r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // 0x35bc4: b 0x35be0 <entry+0x754>   @ imm = #0x14
    c.b(C::Al,0x35be0)?;
    // 0x35bc8: ldr r2, [pc, #0x18]         @ 0x75c <entry+0x75c>
    c.literal_cond(C::Al,R::R2,0x35be8)?;
    // 0x35bcc: mov r1, #211
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,211)?)?;
    // 0x35bd0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35bd4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x35bd8: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x35bdc: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x34434
    c.bl(0x17b0)?;
    // 0x35be0: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x35be4: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x35be8)?;
    c.emit(FRAME_INVALID_CLASS_DIAGNOSTIC)?;
    c.check(END)
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn frame_dispatch_is_18_entry_table(){
   let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
   let word=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
   for (i,target) in FRAME_CLASS_HANDLERS.iter().enumerate(){assert_eq!(word(0x35830+i*4),*target);}
   assert_eq!(word(0x35a2c),FRAME_INVALID_CLASS_DIAGNOSTIC);
   assert_eq!(word(0x35b2c),DEFAULT_CODEC_FORMAT);
   assert_eq!(word(0x35be8),FRAME_INVALID_CLASS_DIAGNOSTIC);
 }
}
