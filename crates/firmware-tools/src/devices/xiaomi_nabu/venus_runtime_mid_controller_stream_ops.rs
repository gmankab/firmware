// SPDX-License-Identifier: GPL-3.0-only
//! Controller codec stream callback handling: guard packet format, select
//! callback tables, release stale packets and initialize the next response.
//! Literal pools are modeled as separate resource and diagnostic data words.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x347a0;
const END:usize=0x34ca0;
const EXTENSION_VALIDATION_LOGS:[u32;5]=[0x00121002,0x000ebf55,0x000ec09b,0x000e2788,0x000ebe82];
const STREAM_COPY_DIAGNOSTICS:[u32;6]=[0x000ebf96,0x000e2788,0x00121004,0x000ebf55,0x000ebf7f,0x000ebe82];
const STREAM_STATUS_DIAGNOSTICS:[u32;5]=[0x00121004,0x000ebf55,0x000ebfca,0x000e2788,0x000ebe82];
const STREAM_CALLBACK_DIAGNOSTICS:[u32;5]=[0x000ebfe9,0x000e2788,0xffffeb60,0x000ebffd,0x000ebe82];

fn validate_extended_controller(c:&mut Code)->Result<(),String>{
 c.check(0x347a0)?;
    // 0x347a0: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x347a4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x347a8: sub sp, sp, #512
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,512)?)?;
    // 0x347ac: ldr r3, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,88)?)?;
    // 0x347b0: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x347b4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x347b8: str r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,4)?)?;
    // 0x347bc: mov r3, #76
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,76)?)?;
    // 0x347c0: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,0)?)?;
    // 0x347c4: mov r3, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,68)?)?;
    // 0x347c8: str r3, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,8)?)?;
    // 0x347cc: ldr r3, [pc, #0xb4]         @ 0x2b08 <entry+0x2b08>
    c.literal(R::new(3)?,0x34888)?;
    // 0x347d0: str r3, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,12)?)?;
    // 0x347d4: ldr r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,8)?)?;
    // 0x347d8: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x347dc: str r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,20)?)?;
    // 0x347e0: str lr, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,24)?)?;
    // 0x347e4: str lr, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,28)?)?;
    // 0x347e8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x347ec: ldr r12, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R1,64)?)?;
    // 0x347f0: str r12, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,32)?)?;
    // 0x347f4: ldr r3, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,92)?)?;
    // 0x347f8: str r3, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,40)?)?;
    // 0x347fc: ldr r4, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,96)?)?;
    // 0x34800: str r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // 0x34804: str r3, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,48)?)?;
    // 0x34808: mov r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,255)?)?;
    // 0x3480c: str r4, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,52)?)?;
    // 0x34810: orr r3, r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,65280)?)?;
    // 0x34814: str r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,72)?)?;
    // 0x34818: str r12, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,68)?)?;
    // 0x3481c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x34820: ldrne r12, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Ne,true,R::R12,R::R1,48)?)?;
    // 0x34824: ldrne lr, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Ne,true,R::LR,R::R1,52)?)?;
    // 0x34828: ldrne r2, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::R1,60)?)?;
    // 0x3482c: mov r1, sp
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // 0x34830: str lr, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::SP,56)?)?;
    // 0x34834: str r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,60)?)?;
    // 0x34838: str r12, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,64)?)?;
    // 0x3483c: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,56)?)?;
    // 0x34840: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x34844: bl 0x80928 <entry+0x4eba8> @ imm = #0x4c0dc
    c.bl(0x80928)?;
    // 0x34848: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3484c: beq 0x34880 <entry+0x2b00>  @ imm = #0x2c
    c.b(C::Eq,0x34880)?;
    // 0x34850: ldr r1, [pc, #0x34]         @ 0x2b0c <entry+0x2b0c>
    c.literal(R::R1,0x3488c)?;
    // 0x34854: ldr r2, [pc, #0x34]         @ 0x2b10 <entry+0x2b10>
    c.literal(R::R2,0x34890)?;
    // 0x34858: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3485c: mov r3, #112
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,112)?)?;
    // 0x34860: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5bd4
    c.bl(0x2ec94)?;
    // 0x34864: ldr r1, [pc, #0x28]         @ 0x2b14 <entry+0x2b14>
    c.literal(R::R1,0x34894)?;
    // 0x34868: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3486c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5be0
    c.bl(0x2ec94)?;
    // 0x34870: ldr r2, [pc, #0x20]         @ 0x2b18 <entry+0x2b18>
    c.literal(R::R2,0x34898)?;
    // 0x34874: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34878: mov r1, #113
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,113)?)?;
    // 0x3487c: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x330d4
    c.bl(0x17b0)?;
    // 0x34880: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x34884: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x34888)?;
 for word in EXTENSION_VALIDATION_LOGS { c.emit(word)?; }
 c.check(0x3489c)
}

fn drain_codec_stream_outputs(c:&mut Code)->Result<(),String>{
 c.check(0x3489c)?;
    // 0x3489c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x348a0: ldr r1, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,64)?)?;
    // 0x348a4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x348a8: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x348ac: bx r1
    c.emit(arm32::bx(C::Al,R::R1))?;
    // 0x348b0: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x348b4: add r1, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x348b8: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x348bc: ldr r0, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,360)?)?;
    // 0x348c0: ldr r1, [r0, #0x48]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,72)?)?;
    // 0x348c4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x348c8: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x348cc: bx r1
    c.emit(arm32::bx(C::Al,R::R1))?;
    // 0x348d0: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x348d4: add r1, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x348d8: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x348dc: ldr r0, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,360)?)?;
    // 0x348e0: ldr r1, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,80)?)?;
    // 0x348e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x348e8: bxeq lr
    c.emit(arm32::bx(C::Eq,R::LR))?;
    // 0x348ec: bx r1
    c.emit(arm32::bx(C::Al,R::R1))?;
    // 0x348f0: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x348f4: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x348f8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x348fc: bl 0x5a710 <entry+0x28990> @ imm = #0x25e0c
    c.bl(0x5a710)?;
    // 0x34900: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x34904: bl 0x5a968 <entry+0x28be8> @ imm = #0x2605c
    c.bl(0x5a968)?;
    // 0x34908: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3490c: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x34910: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34914: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x34918: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // 0x3491c: add r1, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x34920: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x34924: ldr r4, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R0,360)?)?;
    // 0x34928: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3492c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x34930: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x34934: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,48)?)?;
    // 0x34938: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3493c: beq 0x34950 <entry+0x2bd0>  @ imm = #0xc
    c.b(C::Eq,0x34950)?;
    // 0x34940: ldr r0, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    // 0x34944: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34948: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3494c: strne r1, [r0, #0xc8]
    c.emit(arm32::transfer_word(C::Ne,false,R::R1,R::R0,200)?)?;
    // 0x34950: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x34954: bl 0x5a710 <entry+0x28990> @ imm = #0x25db4
    c.bl(0x5a710)?;
    // 0x34958: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x3495c: bl 0x5aa8c <entry+0x28d0c> @ imm = #0x26128
    c.bl(0x5aa8c)?;
    // 0x34960: mov r0, #5184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5184)?)?;
    // 0x34964: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x34968: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x3496c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x34970: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x34974: mov r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68)?)?;
    // 0x34978: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x3497c: ldr r0, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34980: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x34984: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x34988: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3498c: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34990: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x34994: sub sp, sp, #624
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,624)?)?;
    // 0x34998: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x3499c: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x349a0: add r1, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x349a4: add r1, r4, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(4)?,R::R1,Shift::Lsl,2,false)?)?;
    // 0x349a8: ldr r5, [r1, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,360)?)?;
    // 0x349ac: ldr r1, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,44)?)?;
    // 0x349b0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x349b4: beq 0x349c4 <entry+0x2c44>  @ imm = #0x8
    c.b(C::Eq,0x349c4)?;
    // 0x349b8: ldrb r2, [r1, #0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R1,6)?)?;
    // 0x349bc: tst r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,6)?)?;
    // 0x349c0: beq 0x34a48 <entry+0x2cc8>  @ imm = #0x80
    c.b(C::Eq,0x34a48)?;
    // 0x349c4: mov r1, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,360)?)?;
    // 0x349c8: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
    // 0x349cc: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(4)?,R::R1,Shift::Lsl,0)?)?;
    // 0x349d0: str r1, [sp, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,116)?)?;
    // 0x349d4: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x349d8: str r1, [sp, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,112)?)?;
    // 0x349dc: ldr r1, [pc, #0x130]        @ 0x2d94 <entry+0x2d94>
    c.literal(R::R1,0x34b14)?;
    // 0x349e0: str r1, [sp, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,124)?)?;
    // 0x349e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x349e8: str r1, [sp, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,132)?)?;
    // 0x349ec: str r0, [sp, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,128)?)?;
    // 0x349f0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x349f4: add r1, sp, #112
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,112)?)?;
    // 0x349f8: str r0, [sp, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,120)?)?;
    // 0x349fc: add r0, r4, #200
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,200)?)?;
    // 0x34a00: bl 0x80928 <entry+0x4eba8> @ imm = #0x4bf20
    c.bl(0x80928)?;
    // 0x34a04: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34a08: beq 0x34adc <entry+0x2d5c>  @ imm = #0xcc
    c.b(C::Eq,0x34adc)?;
    // 0x34a0c: ldr r1, [pc, #0x104]        @ 0x2d98 <entry+0x2d98>
    c.literal(R::R1,0x34b18)?;
    // 0x34a10: ldr r2, [pc, #0x104]        @ 0x2d9c <entry+0x2d9c>
    c.literal(R::R2,0x34b1c)?;
    // 0x34a14: mov r3, #245
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,245)?)?;
    // 0x34a18: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34a1c: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x34a20: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5d94
    c.bl(0x2ec94)?;
    // 0x34a24: ldr r1, [pc, #0xe4]         @ 0x2d90 <entry+0x2d90>
    c.literal(R::R1,0x34b10)?;
    // 0x34a28: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34a2c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5da0
    c.bl(0x2ec94)?;
    // 0x34a30: ldr r2, [pc, #0xe8]         @ 0x2da0 <entry+0x2da0>
    c.literal(R::R2,0x34b20)?;
    // 0x34a34: mov r1, #246
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,246)?)?;
    // 0x34a38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34a3c: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x34a40: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x33298
    c.bl(0x17b0)?;
    // 0x34a44: b 0x34adc <entry+0x2d5c>  @ imm = #0x90
    c.b(C::Al,0x34adc)?;
    // 0x34a48: ldr r1, [r1, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,412)?)?;
    // 0x34a4c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x34a50: beq 0x349c4 <entry+0x2c44>  @ imm = #-0x94
    c.b(C::Eq,0x349c4)?;
    // 0x34a54: add r0, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,4)?)?;
    // 0x34a58: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x34a5c: mov r2, #108
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,108)?)?;
    // 0x34a60: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x3187c
    c.bl(0x31ec)?;
    // 0x34a64: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x34a68: str r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,64)?)?;
    // 0x34a6c: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x34a70: ldrb r0, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,6)?)?;
    // 0x34a74: tst r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,1)?)?;
    // 0x34a78: bne 0x34aac <entry+0x2d2c>  @ imm = #0x2c
    c.b(C::Ne,0x34aac)?;
    // 0x34a7c: ldr r3, [r5, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,96)?)?;
    // 0x34a80: ldr r2, [r5, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,112)?)?;
    // 0x34a84: ldr r1, [pc, #0x80]         @ 0x2d8c <entry+0x2d8c>
    c.literal(R::R1,0x34b0c)?;
    // 0x34a88: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34a8c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5e00
    c.bl(0x2ec94)?;
    // 0x34a90: ldr r1, [pc, #0x78]         @ 0x2d90 <entry+0x2d90>
    c.literal(R::R1,0x34b10)?;
    // 0x34a94: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34a98: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5e0c
    c.bl(0x2ec94)?;
    // 0x34a9c: add r1, r5, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,92)?)?;
    // 0x34aa0: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,4)?)?;
    // 0x34aa4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34aa8: bl 0x39490 <entry+0x7710>  @ imm = #0x49e0
    c.bl(0x39490)?;
    // 0x34aac: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34ab0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x34ab4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x34ab8: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34abc: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34ac0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x34ac4: add r0, r5, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1632)?)?;
    // 0x34ac8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x34acc: bl 0x35670 <entry+0x38f0>  @ imm = #0xb9c
    c.bl(0x35670)?;
    // 0x34ad0: ldr r1, [r5, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,64)?)?;
    // 0x34ad4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34ad8: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // 0x34adc: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x34ae0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34ae4: ldrbne r1, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::R0,6)?)?;
    // 0x34ae8: orrne r1, r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Orr,R::R1,R::R1,2)?)?;
    // 0x34aec: strbne r1, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::R0,6)?)?;
    // 0x34af0: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34af4: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34af8: ldr r0, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34afc: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x34b00: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x34b04: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x34b08: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x34b0c)?;
 for word in STREAM_COPY_DIAGNOSTICS { c.emit(word)?; }
 c.check(0x34b24)
}

fn publish_stream_status(c:&mut Code)->Result<(),String>{
 c.check(0x34b24)?;
    // 0x34b24: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34b28: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x34b2c: sub sp, sp, #512
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,512)?)?;
    // 0x34b30: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x34b34: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x34b38: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x34b3c: ldr r1, [pc, #0x6c]         @ 0x2e30 <entry+0x2e30>
    c.literal(R::R1,0x34bb0)?;
    // 0x34b40: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x34b44: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x34b48: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,20)?)?;
    // 0x34b4c: ldr r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,16)?)?;
    // 0x34b50: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,16)?)?;
    // 0x34b54: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x34b58: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x34b5c: mov r1, sp
    c.emit(arm32::mov_register(C::Al,R::R1,R::SP))?;
    // 0x34b60: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,56)?)?;
    // 0x34b64: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,12)?)?;
    // 0x34b68: bl 0x80928 <entry+0x4eba8> @ imm = #0x4bdb8
    c.bl(0x80928)?;
    // 0x34b6c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34b70: beq 0x34ba8 <entry+0x2e28>  @ imm = #0x30
    c.b(C::Eq,0x34ba8)?;
    // 0x34b74: ldr r1, [pc, #0x38]         @ 0x2e34 <entry+0x2e34>
    c.literal(R::R1,0x34bb4)?;
    // 0x34b78: ldr r2, [pc, #0x38]         @ 0x2e38 <entry+0x2e38>
    c.literal(R::R2,0x34bb8)?;
    // 0x34b7c: mov r3, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,39)?)?;
    // 0x34b80: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34b84: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x34b88: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5efc
    c.bl(0x2ec94)?;
    // 0x34b8c: ldr r1, [pc, #0x28]         @ 0x2e3c <entry+0x2e3c>
    c.literal(R::R1,0x34bbc)?;
    // 0x34b90: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x34b94: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5f08
    c.bl(0x2ec94)?;
    // 0x34b98: ldr r2, [pc, #0x20]         @ 0x2e40 <entry+0x2e40>
    c.literal(R::R2,0x34bc0)?;
    // 0x34b9c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34ba0: mov r1, #552
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,552)?)?;
    // 0x34ba4: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x333fc
    c.bl(0x17b0)?;
    // 0x34ba8: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,8)?)?;
    // 0x34bac: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x34bb0)?;
 for word in STREAM_STATUS_DIAGNOSTICS { c.emit(word)?; }
 c.check(0x34bc4)
}

fn prepare_stream_callback_buffer(c:&mut Code)->Result<(),String>{
 c.check(0x34bc4)?;
    // 0x34bc4: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34bc8: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x34bcc: sub sp, sp, #512
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,512)?)?;
    // 0x34bd0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x34bd4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x34bd8: ldr r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,8)?)?;
    // 0x34bdc: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x34be0: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x34be4: ldr r6, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,360)?)?;
    // 0x34be8: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34bec: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34bf0: ldr r0, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34bf4: ldr r8, [r6, #0x81c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(6)?,2076)?)?;
    // 0x34bf8: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x34bfc: str r1, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,0)?)?;
    // 0x34c00: ldr r1, [pc, #0x84]         @ 0x2f0c <entry+0x2f0c>
    c.literal(R::R1,0x34c8c)?;
    // 0x34c04: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x34c08: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x34c0c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x34c10: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34c14: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5f88
    c.bl(0x2ec94)?;
    // 0x34c18: ldr r1, [pc, #0x70]         @ 0x2f10 <entry+0x2f10>
    c.literal(R::R1,0x34c90)?;
    // 0x34c1c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34c20: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5f94
    c.bl(0x2ec94)?;
    // 0x34c24: mov r7, sp
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::SP))?;
    // 0x34c28: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x34c2c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x34c30: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x34c34: add r2, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,8)?)?;
    // 0x34c38: bl 0x593a8 <entry+0x27628> @ imm = #0x24768
    c.bl(0x593a8)?;
    // 0x34c3c: ldr r0, [pc, #0x50]         @ 0x2f14 <entry+0x2f14>
    c.literal(R::R0,0x34c94)?;
    // 0x34c40: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x34c44: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x34c48: bl 0x80928 <entry+0x4eba8> @ imm = #0x4bcd8
    c.bl(0x80928)?;
    // 0x34c4c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34c50: beq 0x34c80 <entry+0x2f00>  @ imm = #0x28
    c.b(C::Eq,0x34c80)?;
    // 0x34c54: ldr r1, [pc, #0x3c]         @ 0x2f18 <entry+0x2f18>
    c.literal(R::R1,0x34c98)?;
    // 0x34c58: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34c5c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5fd0
    c.bl(0x2ec94)?;
    // 0x34c60: ldr r1, [pc, #0x28]         @ 0x2f10 <entry+0x2f10>
    c.literal(R::R1,0x34c90)?;
    // 0x34c64: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34c68: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5fdc
    c.bl(0x2ec94)?;
    // 0x34c6c: ldr r2, [pc, #0x28]         @ 0x2f1c <entry+0x2f1c>
    c.literal(R::R2,0x34c9c)?;
    // 0x34c70: mov r1, #67
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,67)?)?;
    // 0x34c74: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34c78: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x34c7c: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x334d4
    c.bl(0x17b0)?;
    // 0x34c80: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x34c84: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x34c88: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x34c8c)?;
 for word in STREAM_CALLBACK_DIAGNOSTICS { c.emit(word)?; }
 c.check(0x34ca0)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 validate_extended_controller(c)?;
 drain_codec_stream_outputs(c)?;
 publish_stream_status(c)?;
 prepare_stream_callback_buffer(c)?;
 c.check(END)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn stream_callbacks_keep_all_four_literal_pools(){
  assert_eq!(END-START,299*4+21*4);
  let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
  assert_eq!(EXTENSION_VALIDATION_LOGS.len(),5);
  assert_eq!(STREAM_COPY_DIAGNOSTICS.len(),6);
  assert_eq!(STREAM_STATUS_DIAGNOSTICS.len(),5);
  assert_eq!(STREAM_CALLBACK_DIAGNOSTICS.len(),5);
 }
}
