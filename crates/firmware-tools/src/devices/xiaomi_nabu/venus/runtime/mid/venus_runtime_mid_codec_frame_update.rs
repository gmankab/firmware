// SPDX-License-Identifier: GPL-3.0-only
//! Apply codec frame update to controller state, including active channel
//! buffer metadata and per-plane format flags. Data pools are modeled as
//! structured codec diagnostics; A32 instructions are checked typed encodings.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x344c0;
const END:usize=0x347a0;
const FRAME_UPDATE_DIAGNOSTICS:[u32;7]=[
  0x000ebf01,0x000e2788,0x000ebee3,0x000ebec1,
  0x000ebf23,0x000ebf40,0x000ebe82,
];
pub(super) fn emit(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x344c0: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x344c4: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x344c8: sub sp, sp, #176
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,176)?)?;
    // 0x344cc: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x344d0: mov r9, r0
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R0))?;
    // 0x344d4: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x344d8: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x344dc: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x344e0: ldr r5, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R0,360)?)?;
    // 0x344e4: mov r7, r5
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(5)?))?;
    // 0x344e8: ldr r0, [r7, #0x5c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R0,R::new(7)?,92)?)?;
    // 0x344ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x344f0: bne 0x34538 <entry+0x27b8>  @ imm = #0x40
    c.b(C::Ne,0x34538)?;
    // 0x344f4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x344f8: str r9, [r5, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,276)?)?;
    // 0x344fc: bl 0x54cfc <entry+0x22f7c> @ imm = #0x207f8
    c.bl(0x54cfc)?;
    // 0x34500: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34504: bl 0x59638 <entry+0x278b8> @ imm = #0x2512c
    c.bl(0x59638)?;
    // 0x34508: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3450c: bl 0x59604 <entry+0x27884> @ imm = #0x250f0
    c.bl(0x59604)?;
    // 0x34510: add r0, r5, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1632)?)?;
    // 0x34514: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x34518: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x3451c: bl 0x3564c <entry+0x38cc>  @ imm = #0x1128
    c.bl(0x3564c)?;
    // 0x34520: add r0, r5, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1232)?)?;
    // 0x34524: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x34528: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x3452c: bl 0x3564c <entry+0x38cc>  @ imm = #0x1118
    c.bl(0x3564c)?;
    // 0x34530: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x34534: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x34538: ldr r1, [r6, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,44)?)?;
    // 0x3453c: ldr r2, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,52)?)?;
    // 0x34540: ldr r0, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,48)?)?;
    // 0x34544: ldr r4, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,60)?)?;
    // 0x34548: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x3454c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4096)?)?;
    // 0x34550: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x34554: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x34558: movhi r3, #0
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3455c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x34560: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34564: moveq r3, r2
    c.emit(arm32::mov_register(C::Eq,R::new(3)?,R::R2))?;
    // 0x34568: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x3456c: beq 0x34584 <entry+0x2804>  @ imm = #0x10
    c.b(C::Eq,0x34584)?;
    // 0x34570: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x34574: beq 0x345a0 <entry+0x2820>  @ imm = #0x24
    c.b(C::Eq,0x345a0)?;
    // 0x34578: ldr r1, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,28)?)?;
    // 0x3457c: mov r2, r3
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(3)?))?;
    // 0x34580: b 0x345e4 <entry+0x2864>  @ imm = #0x5c
    c.b(C::Al,0x345e4)?;
    // 0x34584: ldr r1, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,28)?)?;
    // 0x34588: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3458c: beq 0x345e0 <entry+0x2860>  @ imm = #0x4c
    c.b(C::Eq,0x345e0)?;
    // 0x34590: ldr r4, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(6)?,32)?)?;
    // 0x34594: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x34598: bne 0x34570 <entry+0x27f0>  @ imm = #-0x30
    c.b(C::Ne,0x34570)?;
    // 0x3459c: b 0x345e4 <entry+0x2864>  @ imm = #0x40
    c.b(C::Al,0x345e4)?;
    // 0x345a0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x345a4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x345a8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x345ac: bl 0x39490 <entry+0x7710>  @ imm = #0x4edc
    c.bl(0x39490)?;
    // 0x345b0: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x345b4: ldr r0, [r5, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,644)?)?;
    // 0x345b8: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x345bc: beq 0x34638 <entry+0x28b8>  @ imm = #0x74
    c.b(C::Eq,0x34638)?;
    // 0x345c0: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x345c4: beq 0x34648 <entry+0x28c8>  @ imm = #0x7c
    c.b(C::Eq,0x34648)?;
    // 0x345c8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x345cc: bne 0x34674 <entry+0x28f4>  @ imm = #0xa0
    c.b(C::Ne,0x34674)?;
    // 0x345d0: ldr r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,20)?)?;
    // 0x345d4: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xde88
    c.bl(0x26754)?;
    // 0x345d8: ldr r1, [pc, #0x1b0]        @ 0x2a10 <entry+0x2a10>
    c.literal(R::R1,0x34790)?;
    // 0x345dc: b 0x34654 <entry+0x28d4>  @ imm = #0x70
    c.b(C::Al,0x34654)?;
    // 0x345e0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x345e4: ldr r3, [r5, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,276)?)?;
    // 0x345e8: mov r7, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,360)?)?;
    // 0x345ec: orr r7, r7, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,5120)?)?;
    // 0x345f0: ldr r3, [r3, r7]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x345f4: str r3, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,96)?)?;
    // 0x345f8: ldr r3, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,8)?)?;
    // 0x345fc: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x34600: ldr r3, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // 0x34604: str r3, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,100)?)?;
    // 0x34608: ldr r3, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,16)?)?;
    // 0x3460c: str r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,104)?)?;
    // 0x34610: str r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,60)?)?;
    // 0x34614: ldr r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,32)?)?;
    // 0x34618: str r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,68)?)?;
    // 0x3461c: ldr r1, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,56)?)?;
    // 0x34620: str r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,56)?)?;
    // 0x34624: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x34628: add r1, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
    // 0x3462c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34630: bl 0x347a0 <entry+0x2a20>  @ imm = #0x168
    c.bl(0x347a0)?;
    // 0x34634: b 0x34754 <entry+0x29d4>  @ imm = #0x118
    c.b(C::Al,0x34754)?;
    // 0x34638: ldr r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,20)?)?;
    // 0x3463c: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xdef0
    c.bl(0x26754)?;
    // 0x34640: ldr r1, [pc, #0x13c]        @ 0x2a04 <entry+0x2a04>
    c.literal(R::R1,0x34784)?;
    // 0x34644: b 0x34654 <entry+0x28d4>  @ imm = #0x8
    c.b(C::Al,0x34654)?;
    // 0x34648: ldr r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,20)?)?;
    // 0x3464c: bl 0x26754 <entry+0xffffffffffff49d4> @ imm = #-0xdf00
    c.bl(0x26754)?;
    // 0x34650: ldr r1, [pc, #0x134]        @ 0x2a0c <entry+0x2a0c>
    c.literal(R::R1,0x3478c)?;
    // 0x34654: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x34658: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x3465c: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x34660: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x59d4
    c.bl(0x2ec94)?;
    // 0x34664: ldr r1, [pc, #0x11c]        @ 0x2a08 <entry+0x2a08>
    c.literal(R::R1,0x34788)?;
    // 0x34668: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x3466c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x59e0
    c.bl(0x2ec94)?;
    // 0x34670: b 0x346a8 <entry+0x2928>  @ imm = #0x30
    c.b(C::Al,0x346a8)?;
    // 0x34674: ldr r1, [pc, #0x118]        @ 0x2a14 <entry+0x2a14>
    c.literal(R::R1,0x34794)?;
    // 0x34678: ldr r2, [pc, #0x118]        @ 0x2a18 <entry+0x2a18>
    c.literal(R::R2,0x34798)?;
    // 0x3467c: mov r3, #95
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,95)?)?;
    // 0x34680: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34684: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x34688: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x59fc
    c.bl(0x2ec94)?;
    // 0x3468c: ldr r1, [pc, #0xf4]         @ 0x2a08 <entry+0x2a08>
    c.literal(R::R1,0x34788)?;
    // 0x34690: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34694: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5a08
    c.bl(0x2ec94)?;
    // 0x34698: ldr r2, [pc, #0xfc]         @ 0x2a1c <entry+0x2a1c>
    c.literal(R::R2,0x3479c)?;
    // 0x3469c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x346a0: mov r1, #352
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,352)?)?;
    // 0x346a4: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x32efc
    c.bl(0x17b0)?;
    // 0x346a8: ldr r0, [r6, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,28)?)?;
    // 0x346ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x346b0: bne 0x346d0 <entry+0x2950>  @ imm = #0x18
    c.b(C::Ne,0x346d0)?;
    // 0x346b4: ldr r0, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,60)?)?;
    // 0x346b8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x346bc: bne 0x346d0 <entry+0x2950>  @ imm = #0xc
    c.b(C::Ne,0x346d0)?;
    // 0x346c0: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x346c4: ldrb r1, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,6)?)?;
    // 0x346c8: orr r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1)?)?;
    // 0x346cc: b 0x346d8 <entry+0x2958>  @ imm = #0x4
    c.b(C::Al,0x346d8)?;
    // 0x346d0: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,44)?)?;
    // 0x346d4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x346d8: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x346dc: strb r1, [r0, #0x6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,6)?)?;
    // 0x346e0: beq 0x34718 <entry+0x2998>  @ imm = #0x30
    c.b(C::Eq,0x34718)?;
    // 0x346e4: ldr r0, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,36)?)?;
    // 0x346e8: ldr r1, [r5, #0x64]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,100)?)?;
    // 0x346ec: ldr r0, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,92)?)?;
    // 0x346f0: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x346f4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x346f8: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x346fc: bhs 0x34718 <entry+0x2998>  @ imm = #0x14
    c.b(C::Cs,0x34718)?;
    // 0x34700: ldr r0, [r5, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,1236)?)?;
    // 0x34704: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x34708: beq 0x34718 <entry+0x2998>  @ imm = #0x8
    c.b(C::Eq,0x34718)?;
    // 0x3470c: ldrb r0, [r5, #0x399]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,921)?)?;
    // 0x34710: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34714: beq 0x34770 <entry+0x29f0>  @ imm = #0x54
    c.b(C::Eq,0x34770)?;
    // 0x34718: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x3471c: ldr r3, [r6, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,48)?)?;
    // 0x34720: ldr r2, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,52)?)?;
    // 0x34724: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x34728: ldr r0, [r9, r0]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x3472c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x34730: add r0, r5, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1632)?)?;
    // 0x34734: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x34738: bl 0x35670 <entry+0x38f0>  @ imm = #0xf30
    c.bl(0x35670)?;
    // 0x3473c: ldr r0, [r5, #0x114]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,276)?)?;
    // 0x34740: ldr r0, [r0, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,316)?)?;
    // 0x34744: bl 0x292c <entry+0xfffffffffffd0bac> @ imm = #-0x31e20
    c.bl(0x292c)?;
    // 0x34748: ldr r1, [r5, #0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,64)?)?;
    // 0x3474c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34750: blx r1
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R1)?)?;
    // 0x34754: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x34758: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x3475c: ldr r0, [r9, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R0,R::new(9)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34760: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x34764: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x34768: sub sp, r11, #24
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,24)?)?;
    // 0x3476c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x34770: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x34774: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x34778: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x3477c: bl 0x347a0 <entry+0x2a20>  @ imm = #0x1c
    c.bl(0x347a0)?;
    // 0x34780: b 0x34754 <entry+0x29d4>  @ imm = #-0x34
    c.b(C::Al,0x34754)?;
 c.check(0x34784)?;
 for word in FRAME_UPDATE_DIAGNOSTICS {c.emit(word)?;}
 c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn frame_update_preserves_literal_locations(){
  assert_eq!(END-START,177*4+7*4);
  let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
  assert_eq!(FRAME_UPDATE_DIAGNOSTICS[1],0x000e2788);
 }
}
