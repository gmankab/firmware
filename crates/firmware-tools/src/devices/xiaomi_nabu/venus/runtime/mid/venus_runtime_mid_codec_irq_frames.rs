// SPDX-License-Identifier: GPL-3.0-only
//! Attach and restore codec frame state, synchronize input fields with
//! controller MMIO banks, and allocate the per-controller descriptor object.
//! Named diagnostic/field-offset pools are not executable ARM instructions.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x34088;
const END:usize=0x344c0;
const FRAME_SYNC_DIAGNOSTICS:[u32;5]=[
  0x000ebe11,0x000ebe3b,0x000e2788,0x000ebe4c,0x000eb784,
];
const ALLOCATOR_DIAGNOSTICS:[u32;2]=[0x00104674,0x000ebe82];
/// Read and update codec MMIO frame-state fields according to input mode.
fn apply_frame_status(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x34088: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3408c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x34090: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x34094: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x34098: mov r0, #376
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,376)?)?;
    // 0x3409c: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x340a0: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x340a4: mov r2, #380
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,380)?)?;
    // 0x340a8: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x340ac: mov r7, #1392
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1392)?)?;
    // 0x340b0: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x340b4: orr r1, r1, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,131072)?)?;
    // 0x340b8: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x340bc: mov r5, r4
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(4)?))?;
    // 0x340c0: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x340c4: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x340c8: mov r0, #372
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,372)?)?;
    // 0x340cc: mov r1, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,24)?)?;
    // 0x340d0: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x340d4: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x340d8: mov r0, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,388)?)?;
    // 0x340dc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x340e0: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x340e4: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x340e8: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,8)?)?;
    // 0x340ec: str r0, [r4, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(4)?,R::R2,Shift::Lsl,0)?)?;
    // 0x340f0: mov r2, #364
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,364)?)?;
    // 0x340f4: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x340f8: str r3, [r5, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::new(5)?,R::R2,Shift::Lsl,0)?)?;
    // 0x340fc: mov r2, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,360)?)?;
    // 0x34100: mov r3, r4
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(4)?))?;
    // 0x34104: orr r2, r2, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,5120)?)?;
    // 0x34108: ldr r2, [r3, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,true,R::R2,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x3410c: str r2, [r4, r7]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(4)?,R::new(7)?,Shift::Lsl,0)?)?;
    // 0x34110: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x34114: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x34118: add r2, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3411c: add r2, r4, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x34120: ldr r3, [r2, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R2,360)?)?;
    // 0x34124: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x34128: beq 0x34164 <entry+0x23e4>  @ imm = #0x34
    c.b(C::Eq,0x34164)?;
    // 0x3412c: ldr r1, [pc, #0x274]        @ 0x2628 <entry+0x2628>
    c.literal(R::R1,0x343a8)?;
    // 0x34130: ldr r2, [pc, #0x274]        @ 0x262c <entry+0x262c>
    c.literal(R::R2,0x343ac)?;
    // 0x34134: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x34138: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3413c: mov r3, #83
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,83)?)?;
    // 0x34140: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x54b4
    c.bl(0x2ec94)?;
    // 0x34144: ldr r1, [pc, #0x264]        @ 0x2630 <entry+0x2630>
    c.literal(R::R1,0x343b0)?;
    // 0x34148: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3414c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x54c0
    c.bl(0x2ec94)?;
    // 0x34150: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x34154: mov r0, #5504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5504)?)?;
    // 0x34158: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x3415c: str r1, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34160: b 0x34394 <entry+0x2614>  @ imm = #0x22c
    c.b(C::Al,0x34394)?;
    // 0x34164: str r1, [r2, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,368)?)?;
    // 0x34168: ldr r1, [pc, #0x244]        @ 0x2634 <entry+0x2634>
    c.literal(R::R1,0x343b4)?;
    // 0x3416c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x34170: ldr r2, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,12)?)?;
    // 0x34174: ldr r3, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,16)?)?;
    // 0x34178: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x54ec
    c.bl(0x2ec94)?;
    // 0x3417c: ldr r1, [pc, #0x22c]        @ 0x2630 <entry+0x2630>
    c.literal(R::R1,0x343b0)?;
    // 0x34180: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x34184: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x54f8
    c.bl(0x2ec94)?;
    // 0x34188: ldr r3, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,12)?)?;
    // 0x3418c: tst r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,2)?)?;
    // 0x34190: bne 0x341c4 <entry+0x2444>  @ imm = #0x2c
    c.b(C::Ne,0x341c4)?;
    // 0x34194: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x34198: bne 0x341f8 <entry+0x2478>  @ imm = #0x58
    c.b(C::Ne,0x341f8)?;
    // 0x3419c: ldr r2, [pc, #0x214]        @ 0x2638 <entry+0x2638>
    c.literal(R::R2,0x343b8)?;
    // 0x341a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x341a4: mov r1, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,166)?)?;
    // 0x341a8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x341ac: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x32a04
    c.bl(0x17b0)?;
    // 0x341b0: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // 0x341b4: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x341b8: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x341bc: ldr r0, [r0, #0x170]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,368)?)?;
    // 0x341c0: b 0x34384 <entry+0x2604>  @ imm = #0x1bc
    c.b(C::Al,0x34384)?;
    // 0x341c4: ldr r2, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // 0x341c8: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x341cc: bge 0x3421c <entry+0x249c>  @ imm = #0x48
    c.b(C::Ge,0x3421c)?;
    // 0x341d0: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x341d4: beq 0x3429c <entry+0x251c>  @ imm = #0xc0
    c.b(C::Eq,0x3429c)?;
    // 0x341d8: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x341dc: beq 0x34228 <entry+0x24a8>  @ imm = #0x44
    c.b(C::Eq,0x34228)?;
    // 0x341e0: ldr r2, [pc, #0x1d0]        @ 0x2638 <entry+0x2638>
    c.literal(R::R2,0x343b8)?;
    // 0x341e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x341e8: mov r1, #131
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,131)?)?;
    // 0x341ec: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x341f0: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x32a48
    c.bl(0x17b0)?;
    // 0x341f4: b 0x342d4 <entry+0x2554>  @ imm = #0xd8
    c.b(C::Al,0x342d4)?;
    // 0x341f8: ldr r2, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // 0x341fc: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x34200: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x34204: bge 0x34240 <entry+0x24c0>  @ imm = #0x34
    c.b(C::Ge,0x34240)?;
    // 0x34208: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x3420c: beq 0x3424c <entry+0x24cc>  @ imm = #0x38
    c.b(C::Eq,0x3424c)?;
    // 0x34210: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x34214: beq 0x3424c <entry+0x24cc>  @ imm = #0x30
    c.b(C::Eq,0x3424c)?;
    // 0x34218: b 0x34268 <entry+0x24e8>  @ imm = #0x48
    c.b(C::Al,0x34268)?;
    // 0x3421c: beq 0x342b8 <entry+0x2538>  @ imm = #0x94
    c.b(C::Eq,0x342b8)?;
    // 0x34220: cmp r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16384)?)?;
    // 0x34224: bne 0x341e0 <entry+0x2460>  @ imm = #-0x4c
    c.b(C::Ne,0x341e0)?;
    // 0x34228: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // 0x3422c: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x34230: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x34234: add r0, r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,360)?)?;
    // 0x34238: bl 0xc1514 <entry+0x8f794> @ imm = #0x8d2d4
    c.bl(0xc1514)?;
    // 0x3423c: b 0x342d0 <entry+0x2550>  @ imm = #0x8c
    c.b(C::Al,0x342d0)?;
    // 0x34240: cmp r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16384)?)?;
    // 0x34244: cmpne r2, #8192
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x34248: bne 0x34268 <entry+0x24e8>  @ imm = #0x18
    c.b(C::Ne,0x34268)?;
    // 0x3424c: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // 0x34250: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x34254: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x34258: add r0, r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,360)?)?;
    // 0x3425c: bl 0x3441c <entry+0x269c>  @ imm = #0x1b8
    c.bl(0x3441c)?;
    // 0x34260: ldr r2, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,16)?)?;
    // 0x34264: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x34268: cmp r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8192)?)?;
    // 0x3426c: bge 0x34288 <entry+0x2508>  @ imm = #0x14
    c.b(C::Ge,0x34288)?;
    // 0x34270: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x34274: beq 0x34358 <entry+0x25d8>  @ imm = #0xdc
    c.b(C::Eq,0x34358)?;
    // 0x34278: cmp r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,4096)?)?;
    // 0x3427c: bne 0x34368 <entry+0x25e8>  @ imm = #0xe4
    c.b(C::Ne,0x34368)?;
    // 0x34280: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x34284: b 0x34374 <entry+0x25f4>  @ imm = #0xe8
    c.b(C::Al,0x34374)?;
    // 0x34288: beq 0x34360 <entry+0x25e0>  @ imm = #0xd0
    c.b(C::Eq,0x34360)?;
    // 0x3428c: cmp r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16384)?)?;
    // 0x34290: bne 0x34368 <entry+0x25e8>  @ imm = #0xd0
    c.b(C::Ne,0x34368)?;
    // 0x34294: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x34298: b 0x34374 <entry+0x25f4>  @ imm = #0xd4
    c.b(C::Al,0x34374)?;
    // 0x3429c: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // 0x342a0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x342a4: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x342a8: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x342ac: add r0, r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,360)?)?;
    // 0x342b0: bl 0x86c2c <entry+0x54eac> @ imm = #0x52974
    c.bl(0x86c2c)?;
    // 0x342b4: b 0x342d0 <entry+0x2550>  @ imm = #0x14
    c.b(C::Al,0x342d0)?;
    // 0x342b8: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // 0x342bc: mov r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8192)?)?;
    // 0x342c0: add r0, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x342c4: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x342c8: add r0, r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,360)?)?;
    // 0x342cc: bl 0xa34b0 <entry+0x71730> @ imm = #0x6f1dc
    c.bl(0xa34b0)?;
    // 0x342d0: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x342d4: ldr r1, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,16)?)?;
    // 0x342d8: cmp r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,255)?)?;
    // 0x342dc: ble 0x34300 <entry+0x2580>  @ imm = #0x1c
    c.b(C::Le,0x34300)?;
    // 0x342e0: cmp r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4096)?)?;
    // 0x342e4: bge 0x3431c <entry+0x259c>  @ imm = #0x30
    c.b(C::Ge,0x3431c)?;
    // 0x342e8: cmp r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,256)?)?;
    // 0x342ec: beq 0x34330 <entry+0x25b0>  @ imm = #0x3c
    c.b(C::Eq,0x34330)?;
    // 0x342f0: cmp r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,512)?)?;
    // 0x342f4: bne 0x34348 <entry+0x25c8>  @ imm = #0x4c
    c.b(C::Ne,0x34348)?;
    // 0x342f8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x342fc: b 0x34374 <entry+0x25f4>  @ imm = #0x70
    c.b(C::Al,0x34374)?;
    // 0x34300: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x34304: beq 0x34338 <entry+0x25b8>  @ imm = #0x2c
    c.b(C::Eq,0x34338)?;
    // 0x34308: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x3430c: cmpne r1, #16
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x34310: bne 0x34348 <entry+0x25c8>  @ imm = #0x30
    c.b(C::Ne,0x34348)?;
    // 0x34314: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x34318: b 0x34374 <entry+0x25f4>  @ imm = #0x54
    c.b(C::Al,0x34374)?;
    // 0x3431c: beq 0x34340 <entry+0x25c0>  @ imm = #0x1c
    c.b(C::Eq,0x34340)?;
    // 0x34320: cmp r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16384)?)?;
    // 0x34324: bne 0x34348 <entry+0x25c8>  @ imm = #0x1c
    c.b(C::Ne,0x34348)?;
    // 0x34328: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x3432c: b 0x34374 <entry+0x25f4>  @ imm = #0x40
    c.b(C::Al,0x34374)?;
    // 0x34330: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x34334: b 0x34374 <entry+0x25f4>  @ imm = #0x38
    c.b(C::Al,0x34374)?;
    // 0x34338: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x3433c: b 0x34374 <entry+0x25f4>  @ imm = #0x30
    c.b(C::Al,0x34374)?;
    // 0x34340: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x34344: b 0x34374 <entry+0x25f4>  @ imm = #0x28
    c.b(C::Al,0x34374)?;
    // 0x34348: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3434c: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x34350: moveq r0, #8
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x34354: b 0x34374 <entry+0x25f4>  @ imm = #0x18
    c.b(C::Al,0x34374)?;
    // 0x34358: mov r0, #11
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,11)?)?;
    // 0x3435c: b 0x34374 <entry+0x25f4>  @ imm = #0x10
    c.b(C::Al,0x34374)?;
    // 0x34360: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x34364: b 0x34374 <entry+0x25f4>  @ imm = #0x8
    c.b(C::Al,0x34374)?;
    // 0x34368: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3436c: cmp r2, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32768)?)?;
    // 0x34370: moveq r0, #12
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x34374: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,8)?)?;
    // 0x34378: add r2, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x3437c: add r2, r4, r2, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(4)?,R::R2,Shift::Lsl,2,false)?)?;
    // 0x34380: str r0, [r2, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,368)?)?;
    // 0x34384: mov r2, r1
    c.emit(arm32::mov_register(C::Al,R::R2,R::R1))?;
    // 0x34388: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x3130c
    c.bl(0x3084)?;
    // 0x3438c: mov r0, #5504
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5504)?)?;
    // 0x34390: str r7, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::new(4)?,R::R0,Shift::Lsl,0)?)?;
    // 0x34394: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x34398: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3439c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x343a0: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x343a4: b 0x33d38 <entry+0x1fb8>  @ imm = #-0x674
    c.b(C::Al,0x33d38)?;
 c.check(0x343a8)?;
 for word in FRAME_SYNC_DIAGNOSTICS {c.emit(word)?;}
 c.check(0x343bc)
}
/// Reset a codec channel and wake the backing streaming planes.
fn reset_codec_channel(c:&mut Code)->Result<(),String>{
 c.check(0x343bc)?;
    // 0x343bc: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x343c0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x343c4: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // 0x343c8: add r1, r1, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R1,Shift::Lsl,2,false)?)?;
    // 0x343cc: add r1, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x343d0: ldr r4, [r1, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R1,360)?)?;
    // 0x343d4: str r0, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,276)?)?;
    // 0x343d8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x343dc: bl 0x54cfc <entry+0x22f7c> @ imm = #0x20918
    c.bl(0x54cfc)?;
    // 0x343e0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x343e4: bl 0x59638 <entry+0x278b8> @ imm = #0x2524c
    c.bl(0x59638)?;
    // 0x343e8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x343ec: bl 0x59604 <entry+0x27884> @ imm = #0x25210
    c.bl(0x59604)?;
    // 0x343f0: add r0, r4, #1632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1632)?)?;
    // 0x343f4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x343f8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x343fc: bl 0x3564c <entry+0x38cc>  @ imm = #0x1248
    c.bl(0x3564c)?;
    // 0x34400: add r0, r4, #1232
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,1232)?)?;
    // 0x34404: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x34408: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x3440c: bl 0x3564c <entry+0x38cc>  @ imm = #0x1238
    c.bl(0x3564c)?;
    // 0x34410: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x34414: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
    // 0x34418: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x3441c)
}
/// Allocate a controller descriptor and attach initialized fields.
fn allocate_stream_descriptor(c:&mut Code)->Result<(),String>{
 c.check(0x3441c)?;
    // 0x3441c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x34420: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x34424: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x34428: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3442c: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x34430: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x34434: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,6144)?)?;
    // 0x34438: bl 0x2efc <entry+0xfffffffffffd117c> @ imm = #-0x31544
    c.bl(0x2efc)?;
    // 0x3443c: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x34440: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x34444: beq 0x3449c <entry+0x271c>  @ imm = #0x50
    c.b(C::Eq,0x3449c)?;
    // 0x34448: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x3444c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x34450: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x34454: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x34458: orr r2, r2, #6144
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,6144)?)?;
    // 0x3445c: bl 0x31ec <entry+0xfffffffffffd146c> @ imm = #-0x31278
    c.bl(0x31ec)?;
    // 0x34460: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x34464: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x34468: str r0, [r7, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,280)?)?;
    // 0x3446c: str r0, [r7, #0x570]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,1392)?)?;
    // 0x34470: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x34474: str r0, [r7, #0xcb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,3252)?)?;
    // 0x34478: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x3447c: bl 0x58c38 <entry+0x26eb8> @ imm = #0x247b4
    c.bl(0x58c38)?;
    // 0x34480: ldr r0, [pc, #0x30]         @ 0x2738 <entry+0x2738>
    c.literal(R::R0,0x344b8)?;
    // 0x34484: str r5, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,16)?)?;
    // 0x34488: str r5, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(7)?,20)?)?;
    // 0x3448c: str r8, [r7, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(7)?,40)?)?;
    // 0x34490: str r7, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,0)?)?;
    // 0x34494: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x34498: b 0x344b0 <entry+0x2730>  @ imm = #0x10
    c.b(C::Al,0x344b0)?;
    // 0x3449c: ldr r2, [pc, #0x18]         @ 0x273c <entry+0x273c>
    c.literal(R::R2,0x344bc)?;
    // 0x344a0: mov r1, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,31)?)?;
    // 0x344a4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x344a8: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x344ac: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x32d04
    c.bl(0x17b0)?;
    // 0x344b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x344b4: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
 c.check(0x344b8)?;
 for word in ALLOCATOR_DIAGNOSTICS {c.emit(word)?;}
 c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 apply_frame_status(c)?;
 reset_codec_channel(c)?;
 allocate_stream_descriptor(c)?;
 c.check(END)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn frame_status_and_allocator_have_exact_boundaries(){
  assert_eq!(END-START,263*4+7*4);
  let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
  assert_eq!(FRAME_SYNC_DIAGNOSTICS[2],0x000e2788);
 }
}
