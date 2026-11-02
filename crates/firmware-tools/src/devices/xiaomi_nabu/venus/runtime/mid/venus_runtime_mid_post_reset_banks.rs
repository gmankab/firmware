// SPDX-License-Identifier: GPL-3.0-only
//! Venus post-reset MMIO bank synchronizers. Two short per-channel loops
//! copy controller command register pairs from indexed buffer descriptors,
//! followed by another register array. Pointers are literal data pools.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x3192c;
const END:usize=0x31b80;
fn synchronize_device_register_banks(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3192c: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x31930: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x31934: ldr r12, [pc, #0x54]        @ 0x2390 <entry+0x2390>
    c.literal(R::R12,0x31990)?;
    // 0x31938: ldr r1, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,0)?)?;
    // 0x3193c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x31940: popeq {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x31944: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2960)?)?;
    // 0x31948: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3194c: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x31950: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x31954: orr lr, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31958: mov r1, #916
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,916)?)?;
    // 0x3195c: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x31960: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31964: ldr r1, [pc, #0x28]         @ 0x2394 <entry+0x2394>
    c.literal(R::R1,0x31994)?;
    // 0x31968: ldr r4, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,0)?)?;
    // 0x3196c: add r3, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,2)?)?;
    // 0x31970: str r4, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x31974: ldr r4, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,4)?)?;
    // 0x31978: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x3197c: str r4, [lr, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::LR,R::R2,Shift::Lsl,0)?)?;
    // 0x31980: ldr r4, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R12,0)?)?;
    // 0x31984: cmp r3, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x31988: blo 0x31968 <entry+0x2368>  @ imm = #-0x28
    c.b(C::Cc,0x31968)?;
    // 0x3198c: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x31990)?;
    const REGISTER_BANK_POINTERS_31990:[u32;2]=[0x00104128,0x0010412c];
    for word in REGISTER_BANK_POINTERS_31990 {c.emit(word)?;}
    c.check(0x31998)?;
    // 0x31998: push {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x3199c: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x319a0: ldr r12, [pc, #0x54]        @ 0x23fc <entry+0x23fc>
    c.literal(R::R12,0x319fc)?;
    // 0x319a4: ldr r1, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,0)?)?;
    // 0x319a8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x319ac: popeq {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x319b0: mov r1, #904
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,904)?)?;
    // 0x319b4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x319b8: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x319bc: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x319c0: orr lr, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x319c4: mov r1, #908
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,908)?)?;
    // 0x319c8: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x319cc: orr r0, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x319d0: ldr r1, [pc, #0x28]         @ 0x2400 <entry+0x2400>
    c.literal(R::R1,0x31a00)?;
    // 0x319d4: ldr r4, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,0)?)?;
    // 0x319d8: add r3, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,2)?)?;
    // 0x319dc: str r4, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x319e0: ldr r4, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,4)?)?;
    // 0x319e4: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x319e8: str r4, [lr, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(4)?,R::LR,R::R2,Shift::Lsl,0)?)?;
    // 0x319ec: ldr r4, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R12,0)?)?;
    // 0x319f0: cmp r3, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x319f4: blo 0x319d4 <entry+0x23d4>  @ imm = #-0x28
    c.b(C::Cc,0x319d4)?;
    // 0x319f8: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x319fc)?;
    const REGISTER_BANK_POINTERS_319FC:[u32;2]=[0x0010420c,0x00104210];
    for word in REGISTER_BANK_POINTERS_319FC {c.emit(word)?;}
    c.check(0x31a04)?;
    // 0x31a04: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x31a08: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x31a0c: ldr r12, [pc, #0x164]       @ 0x2578 <entry+0x2578>
    c.literal(R::R12,0x31b78)?;
    // 0x31a10: ldr r2, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R12,0)?)?;
    // 0x31a14: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x31a18: beq 0x31a60 <entry+0x2460>  @ imm = #0x40
    c.b(C::Eq,0x31a60)?;
    // 0x31a1c: mov r2, #15232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,15232)?)?;
    // 0x31a20: ldr r4, [pc, #0x154]        @ 0x257c <entry+0x257c>
    c.literal(R::new(4)?,0x31b7c)?;
    // 0x31a24: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x31a28: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // 0x31a2c: orr lr, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::LR,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31a30: mov r2, #900
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,900)?)?;
    // 0x31a34: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x31a38: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31a3c: ldr r6, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,0)?)?;
    // 0x31a40: add r3, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,2)?)?;
    // 0x31a44: str r6, [r2, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::R2,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x31a48: ldr r6, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,4)?)?;
    // 0x31a4c: add r4, r4, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,8)?)?;
    // 0x31a50: str r6, [lr, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::LR,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x31a54: ldr r6, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R12,0)?)?;
    // 0x31a58: cmp r3, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x31a5c: blo 0x31a3c <entry+0x243c>  @ imm = #-0x28
    c.b(C::Cc,0x31a3c)?;
    // 0x31a60: ldr r6, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,76)?)?;
    // 0x31a64: mov r2, #14848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14848)?)?;
    // 0x31a68: orr r3, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31a6c: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x31a70: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31a74: mov r3, #516
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,516)?)?;
    // 0x31a78: ldr r6, [r1, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,80)?)?;
    // 0x31a7c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31a80: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31a84: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31a88: mov r3, #520
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,520)?)?;
    // 0x31a8c: ldr r6, [r1, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,84)?)?;
    // 0x31a90: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31a94: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31a98: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31a9c: mov r3, #524
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,524)?)?;
    // 0x31aa0: ldr r6, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,88)?)?;
    // 0x31aa4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31aa8: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31aac: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31ab0: mov r3, #536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,536)?)?;
    // 0x31ab4: ldr r6, [r1, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,100)?)?;
    // 0x31ab8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31abc: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31ac0: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31ac4: mov r3, #540
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,540)?)?;
    // 0x31ac8: ldr r6, [r1, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,104)?)?;
    // 0x31acc: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31ad0: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31ad4: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31ad8: mov r3, #2592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2592)?)?;
    // 0x31adc: ldr r6, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,108)?)?;
    // 0x31ae0: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x31ae4: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31ae8: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31aec: mov r3, #548
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,548)?)?;
    // 0x31af0: ldr r6, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,112)?)?;
    // 0x31af4: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31af8: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31afc: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31b00: mov r3, #2608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2608)?)?;
    // 0x31b04: ldr r6, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,116)?)?;
    // 0x31b08: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x31b0c: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31b10: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31b14: mov r3, #564
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,564)?)?;
    // 0x31b18: ldr r6, [r1, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,120)?)?;
    // 0x31b1c: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31b20: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31b24: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31b28: mov r3, #14912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14912)?)?;
    // 0x31b2c: ldr r6, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,132)?)?;
    // 0x31b30: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31b34: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31b38: mov r3, #580
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,580)?)?;
    // 0x31b3c: ldr r6, [r1, #0x88]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,136)?)?;
    // 0x31b40: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31b44: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31b48: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31b4c: mov r3, #584
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,584)?)?;
    // 0x31b50: ldr r6, [r1, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,140)?)?;
    // 0x31b54: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31b58: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31b5c: str r6, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x31b60: mov r3, #588
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,588)?)?;
    // 0x31b64: ldr r1, [r1, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,144)?)?;
    // 0x31b68: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x31b6c: orr r0, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x31b70: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x31b74: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x31b78)?;
    const REGISTER_BANK_POINTERS_31B78:[u32;2]=[0x001043c0,0x001043c4];
    for word in REGISTER_BANK_POINTERS_31B78 {c.emit(word)?;}
    c.check(0x31b80)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;synchronize_device_register_banks(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn post_reset_register_banks_are_exactly_bounded(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let get=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(get(0x31990),0x00104128);
        assert_eq!(get(0x31994),0x0010412c);
        assert_eq!(get(0x319fc),0x0010420c);
        assert_eq!(get(0x31b78),0x001043c0);
        assert_eq!(get(0x31b7c),0x001043c4);
    }
}
