// SPDX-License-Identifier: GPL-3.0-only
//! Codec input format classification and bounded controller channel defaults.
//! Public ARM A32 operands are typed and encoded, with a distinct pointer/
//! threshold literal pool. No original executable instruction words or OEM
//! authentication payload bytes are embedded in this module.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};

const START:usize=0x37180;
const END:usize=0x37be8;
/// Controller thresholds, runtime diagnostic pointers and related literal
/// operands. Some values happen to resemble A32 opcodes, but are data.
const INPUT_MODE_LITERALS:[u32;10]=[
    0xffff_5555, 0x001f_e001, 0x007e_9001, 0x000e_c0b3,
    0x000e_c144, 0x000e_2788, 0x000e_c1b3, 0x000e_c180,
    0x000e_c1e2, 0x000e_c228,
];
/// A32 register-offset signed halfword LDRSH preindexed. The P,U,W,L and
/// halfword-sign extension bitfields are built from architecture fields,
/// rather than transcribed OEM instruction words. Disallows unpredictable
/// PC and base-destination aliasing during writeback.
fn encode_signed_halfword_register_writeback(c:C,dst:R,base:R,offset:R,wb:bool)->Result<u32,String>{
    if [dst,base,offset].contains(&R::PC) || (wb && dst==base) {
        return Err("unsafe signed-halfword indexed transfer".into());
    }
    Ok(((c as u32)<<28) | 0x0190_00f0 | (if wb {1<<21} else {0})
       |(base.number()<<16)|(dst.number()<<12)|offset.number())
}

/// Classify the input stream, normalize codec size/channel thresholds, and
/// dispatch table-guided defaults. Branches inside the routine are preserved
/// and the function returns at 0x379c0; reachable tail cases follow.
fn select_input_configuration(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x37180: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x37184: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x37188: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,28)?)?;
    // 0x3718c: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x37190: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x37194: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x37198: mov r3, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,15)?)?;
    // 0x3719c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x371a0: ldr r8, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,4)?)?;
    // 0x371a4: ldr lr, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(10)?,76)?)?;
    // 0x371a8: cmp r8, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,8192)?)?;
    // 0x371ac: moveq r1, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x371b0: moveq r3, #31
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,31)?)?;
    // 0x371b4: cmp r8, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,8192)?)?;
    // 0x371b8: ldrbne r0, [r10, #0x6c3]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(10)?,1731)?)?;
    // 0x371bc: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x371c0: str r0, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,300)?)?;
    // 0x371c4: ldr r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,40)?)?;
    // 0x371c8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x371cc: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x371d0: strbeq r0, [r10, #0x13a]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(10)?,314)?)?;
    // 0x371d4: ldrb r0, [r10, #0x68c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1676)?)?;
    // 0x371d8: cmp r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,9)?)?;
    // 0x371dc: bne 0x371f8 <entry+0x1d6c>  @ imm = #0x14
    c.b(C::Ne,0x371f8)?;
    // 0x371e0: ldr r0, [pc, #0x93c]        @ 0x2698 <entry+0x2698>
    c.literal_cond(C::Al,R::R0,0x37b24)?;
    // 0x371e4: str r0, [r10, #0x678]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1656)?)?;
    // 0x371e8: str r0, [r10, #0x67c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1660)?)?;
    // 0x371ec: str r0, [r10, #0x680]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1664)?)?;
    // 0x371f0: str r0, [r10, #0x684]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1668)?)?;
    // 0x371f4: str r0, [r10, #0x688]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,1672)?)?;
    // 0x371f8: ldrb r2, [r10, #0x60a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,1546)?)?;
    // 0x371fc: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x37200: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37204: strbeq r0, [r10, #0x17c]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R0,R::new(10)?,380)?)?;
    // 0x37208: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x3720c: add r0, r3, lr
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(3)?,R::LR,Shift::Lsl,0,false)?)?;
    // 0x37210: moveq r2, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x37214: lsr r0, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::R1,false)?)?;
    // 0x37218: strb r2, [r10, #0x60a]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,1546)?)?;
    // 0x3721c: lsl r0, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsl,R::R1,false)?)?;
    // 0x37220: ldr r5, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,80)?)?;
    // 0x37224: ldr r3, [pc, #0x8fc]        @ 0x269c <entry+0x269c>
    c.literal_cond(C::Al,R::new(3)?,0x37b28)?;
    // 0x37228: mul r2, r5, lr
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(5)?,R::LR)?)?;
    // 0x3722c: cmp r2, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x37230: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x37234: movhs r2, #0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x37238: strbhs r2, [r10, #0x60a]
    c.emit(arm32::transfer_byte(C::Cs,false,R::R2,R::new(10)?,1546)?)?;
    // 0x3723c: strbhs r2, [r10, #0x609]
    c.emit(arm32::transfer_byte(C::Cs,false,R::R2,R::new(10)?,1545)?)?;
    // 0x37240: ldr r2, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,108)?)?;
    // 0x37244: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x37248: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x3724c: streq r2, [r10, #0x314]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(10)?,788)?)?;
    // 0x37250: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x37254: ldr r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,288)?)?;
    // 0x37258: cmp r1, lr
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::LR,Shift::Lsl,0,false)?)?;
    // 0x3725c: moveq r1, r0
    c.emit(arm32::mov_register(C::Eq,R::R1,R::R0))?;
    // 0x37260: streq r0, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(10)?,288)?)?;
    // 0x37264: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x37268: str r5, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(10)?,296)?)?;
    // 0x3726c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x37270: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37274: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37278: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x3727c: strb r0, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,332)?)?;
    // 0x37280: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37284: strb r0, [r10, #0x169]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,361)?)?;
    // 0x37288: strb r0, [r10, #0x14b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,331)?)?;
    // 0x3728c: strb r2, [r10, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,349)?)?;
    // 0x37290: str r2, [r10, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,376)?)?;
    // 0x37294: strb r3, [r10, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,357)?)?;
    // 0x37298: ldr r3, [r10, #0x320]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,800)?)?;
    // 0x3729c: strb r3, [r10, #0xa9]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,169)?)?;
    // 0x372a0: ldr r3, [r10, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,52)?)?;
    // 0x372a4: ldr r5, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(10)?,56)?)?;
    // 0x372a8: mul r6, r3, r5
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(3)?,R::new(5)?)?)?;
    // 0x372ac: ldr r3, [pc, #0x878]        @ 0x26a0 <entry+0x26a0>
    c.literal_cond(C::Al,R::new(3)?,0x37b2c)?;
    // 0x372b0: cmp r6, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x372b4: blo 0x372d0 <entry+0x1e44>  @ imm = #0x14
    c.b(C::Cc,0x372d0)?;
    // 0x372b8: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x372bc: str r3, [r10, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,372)?)?;
    // 0x372c0: str r2, [r10, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,284)?)?;
    // 0x372c4: strb r2, [r10, #0x162]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,354)?)?;
    // 0x372c8: strb r2, [r10, #0x108]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,264)?)?;
    // 0x372cc: strb r0, [r10, #0x165]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,357)?)?;
    // 0x372d0: ldrb r0, [r10, #0x13b]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,315)?)?;
    // 0x372d4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x372d8: bne 0x372f8 <entry+0x1e6c>  @ imm = #0x18
    c.b(C::Ne,0x372f8)?;
    // 0x372dc: ldrb r0, [r10, #0x13a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,314)?)?;
    // 0x372e0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x372e4: ldrbne r0, [r10, #0x139]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R0,R::new(10)?,313)?)?;
    // 0x372e8: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x372ec: bne 0x372f8 <entry+0x1e6c>  @ imm = #0x4
    c.b(C::Ne,0x372f8)?;
    // 0x372f0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x372f4: strb r0, [r10, #0x13b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,315)?)?;
    // 0x372f8: cmp r8, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,8192)?)?;
    // 0x372fc: beq 0x37310 <entry+0x1e84>  @ imm = #0xc
    c.b(C::Eq,0x37310)?;
    // 0x37300: cmp r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,4096)?)?;
    // 0x37304: beq 0x3731c <entry+0x1e90>  @ imm = #0x10
    c.b(C::Eq,0x3731c)?;
    // 0x37308: cmp r8, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,2)?)?;
    // 0x3730c: bne 0x37324 <entry+0x1e98>  @ imm = #0x10
    c.b(C::Ne,0x37324)?;
    // 0x37310: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37314: strb r0, [r10, #0xb8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,184)?)?;
    // 0x37318: b 0x37324 <entry+0x1e98>  @ imm = #0x4
    c.b(C::Al,0x37324)?;
    // 0x3731c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37320: str r0, [r10, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,284)?)?;
    // 0x37324: ldrb r0, [r10, #0x258]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,600)?)?;
    // 0x37328: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x3732c: bhi 0x37344 <entry+0x1eb8>  @ imm = #0x10
    c.b(C::Hi,0x37344)?;
    // 0x37330: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37334: mov r3, #194
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,194)?)?;
    // 0x37338: tst r3, r2, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(3)?,R::R2,Shift::Lsl,R::R0,false)?)?;
    // 0x3733c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37340: strbne r0, [r10, #0x17c]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R0,R::new(10)?,380)?)?;
    // 0x37344: mov r5, r10
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // 0x37348: mov r7, r10
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(10)?))?;
    // 0x3734c: mov r3, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,64)?)?;
    // 0x37350: mov r4, r10
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // 0x37354: ldr r12, [r5, #0x190]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R12,R::new(5)?,400)?)?;
    // 0x37358: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x3735c: ldreq r12, [r10, #0x188]
    c.emit(arm32::transfer_word(C::Eq,true,R::R12,R::new(10)?,392)?)?;
    // 0x37360: streq r12, [r10, #0x190]
    c.emit(arm32::transfer_word(C::Eq,false,R::R12,R::new(10)?,400)?)?;
    // 0x37364: mov r0, r12
    c.emit(arm32::mov_register(C::Al,R::R0,R::R12))?;
    // 0x37368: ldr r9, [r7, #0x194]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(9)?,R::new(7)?,404)?)?;
    // 0x3736c: ldr r2, [r7, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,-8)?)?;
    // 0x37370: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x37374: streq r2, [r7]
    c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::new(7)?,0)?)?;
    // 0x37378: moveq r9, r2
    c.emit(arm32::mov_register(C::Eq,R::new(9)?,R::R2))?;
    // 0x3737c: cmp r12, #192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,192)?)?;
    // 0x37380: movge r0, #192
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,192)?)?;
    // 0x37384: cmp r9, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,64)?)?;
    // 0x37388: movge r9, r3
    c.emit(arm32::mov_register(C::Ge,R::new(9)?,R::new(3)?))?;
    // 0x3738c: str r0, [r10, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,400)?)?;
    // 0x37390: str r9, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(10)?,404)?)?;
    // 0x37394: ldr r6, [r4, #0x188]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::new(6)?,R::new(4)?,392)?)?;
    // 0x37398: cmp r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,64)?)?;
    // 0x3739c: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x373a0: movlt r3, r6
    c.emit(arm32::mov_register(C::Lt,R::new(3)?,R::new(6)?))?;
    // 0x373a4: cmp r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,32)?)?;
    // 0x373a8: mov r6, r10
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(10)?))?;
    // 0x373ac: str r3, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,0)?)?;
    // 0x373b0: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x373b4: movge r2, r3
    c.emit(arm32::mov_register(C::Ge,R::R2,R::new(3)?))?;
    // 0x373b8: str r2, [r6, #0x18c]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::R2,R::new(6)?,396)?)?;
    // 0x373bc: ldrb r2, [r6, #0x50c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(6)?,1292)?)?;
    // 0x373c0: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x373c4: bne 0x373dc <entry+0x1f50>  @ imm = #0x10
    c.b(C::Ne,0x373dc)?;
    // 0x373c8: cmp r0, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,128)?)?;
    // 0x373cc: movge r12, #128
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R12,R::R0,128)?)?;
    // 0x373d0: mov r4, r12
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R12))?;
    // 0x373d4: str r12, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,0)?)?;
    // 0x373d8: b 0x373e0 <entry+0x1f54>  @ imm = #0x0
    c.b(C::Al,0x373e0)?;
    // 0x373dc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x373e0: sub r0, r9, #33
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(9)?,33)?)?;
    // 0x373e4: cmp r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,15)?)?;
    // 0x373e8: blo 0x373fc <entry+0x1f70>  @ imm = #0xc
    c.b(C::Cc,0x373fc)?;
    // 0x373ec: sub r0, r9, #17
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(9)?,17)?)?;
    // 0x373f0: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x373f4: cmp r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,14)?)?;
    // 0x373f8: bhi 0x37404 <entry+0x1f78>  @ imm = #0x4
    c.b(C::Hi,0x37404)?;
    // 0x373fc: mov r9, r3
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::new(3)?))?;
    // 0x37400: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // 0x37404: cmp r8, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,8192)?)?;
    // 0x37408: bne 0x3743c <entry+0x1fb0>  @ imm = #0x2c
    c.b(C::Ne,0x3743c)?;
    // 0x3740c: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x37410: cmp r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,32)?)?;
    // 0x37414: str r2, [r10, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,396)?)?;
    // 0x37418: movgt r2, r9
    c.emit(arm32::mov_register(C::Gt,R::R2,R::new(9)?))?;
    // 0x3741c: str r2, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,404)?)?;
    // 0x37420: ldr r0, [r10, #0x174]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,372)?)?;
    // 0x37424: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x37428: beq 0x37438 <entry+0x1fac>  @ imm = #0x8
    c.b(C::Eq,0x37438)?;
    // 0x3742c: ldrb r0, [r10, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,707)?)?;
    // 0x37430: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37434: beq 0x379c4 <entry+0x2538>  @ imm = #0x588
    c.b(C::Eq,0x379c4)?;
    // 0x37438: mov r9, r2
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // 0x3743c: ldr r0, [r10, #0x84]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,132)?)?;
    // 0x37440: str r8, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,16)?)?;
    // 0x37444: str r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,12)?)?;
    // 0x37448: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x3744c: blo 0x3745c <entry+0x1fd0>  @ imm = #0x8
    c.b(C::Cc,0x3745c)?;
    // 0x37450: ldrb r0, [r10, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,707)?)?;
    // 0x37454: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37458: beq 0x378f0 <entry+0x2464>  @ imm = #0x490
    c.b(C::Eq,0x378f0)?;
    // 0x3745c: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // 0x37460: mov r8, r5
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(5)?))?;
    // 0x37464: add r0, lr, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::LR,R::R1,Shift::Lsl,0,false)?)?;
    // 0x37468: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x3746c: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4c7d8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x37470: str r0, [r10, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,304)?)?;
    // 0x37474: add r0, r8, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x37478: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x3747c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x37480: blx 0x83c4c <entry+0x4e7c0> @ imm = #0x4c7c4
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x83c4c)?)?;
    // 0x37484: str r0, [r10, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,308)?)?;
    // 0x37488: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x3748c: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x37490: ldr r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,8)?)?;
    // 0x37494: moveq r7, r6
    c.emit(arm32::mov_register(C::Eq,R::new(7)?,R::new(6)?))?;
    // 0x37498: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x3749c: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x374a0: ldr r9, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(7)?,0)?)?;
    // 0x374a4: mov r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16)?)?;
    // 0x374a8: moveq r0, r1
    c.emit(arm32::mov_register(C::Eq,R::R0,R::R1))?;
    // 0x374ac: ldr r8, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,0)?)?;
    // 0x374b0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x374b4: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x374b8: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x374bc: moveq r7, #32
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(7)?,R::R0,32)?)?;
    // 0x374c0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x374c4: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b718
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x374c8: mov r1, #776
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,776)?)?;
    // 0x374cc: mov r5, r10
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // 0x374d0: ldrsh r6, [r5, r1]!
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::new(6)?,R::new(5)?,R::R1,true)?)?;
    // 0x374d4: cmp r0, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x374d8: bgt 0x374f4 <entry+0x2068>  @ imm = #0x14
    c.b(C::Gt,0x374f4)?;
    // 0x374dc: sub r0, r8, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,16)?)?;
    // 0x374e0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x374e4: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b6f8
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x374e8: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x374ec: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x374f0: movlt r6, #0
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x374f4: rsb r0, r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(8)?,0)?)?;
    // 0x374f8: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x374fc: strh r6, [r5]
    c.emit(arm32::store_halfword(C::Al,R::new(6)?,R::new(5)?,0)?)?;
    // 0x37500: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b6dc
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x37504: lsl r1, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(6)?,Shift::Lsl,16)?)?;
    // 0x37508: asr r1, r1, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,16)?)?;
    // 0x3750c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x37510: bgt 0x3752c <entry+0x20a0>  @ imm = #0x14
    c.b(C::Gt,0x3752c)?;
    // 0x37514: sub r0, r8, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,16)?)?;
    // 0x37518: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3751c: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b6c0
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x37520: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37524: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37528: rsbgt r1, r0, #0
    c.emit(arm32::data_immediate(C::Gt,Op::Rsb,R::R1,R::R0,0)?)?;
    // 0x3752c: mov r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
    // 0x37530: strh r1, [r5]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(5)?,0)?)?;
    // 0x37534: mov r6, r10
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(10)?))?;
    // 0x37538: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3753c: orr r0, r0, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
    // 0x37540: ldrsh r5, [r6, r0]!
    c.emit(encode_signed_halfword_register_writeback(C::Al,R::new(5)?,R::new(6)?,R::R0,true)?)?;
    // 0x37544: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x37548: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b694
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x3754c: cmp r0, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x37550: bgt 0x3756c <entry+0x20e0>  @ imm = #0x14
    c.b(C::Gt,0x3756c)?;
    // 0x37554: sub r0, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(9)?,16)?)?;
    // 0x37558: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x3755c: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b680
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x37560: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x37564: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x37568: movlt r5, #0
    c.emit(arm32::data_immediate(C::Lt,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x3756c: rsb r0, r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::new(9)?,0)?)?;
    // 0x37570: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x37574: strh r5, [r6]
    c.emit(arm32::store_halfword(C::Al,R::new(5)?,R::new(6)?,0)?)?;
    // 0x37578: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b664
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x3757c: lsl r1, r5, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(5)?,Shift::Lsl,16)?)?;
    // 0x37580: asr r1, r1, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,16)?)?;
    // 0x37584: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x37588: bgt 0x375a4 <entry+0x2118>  @ imm = #0x14
    c.b(C::Gt,0x375a4)?;
    // 0x3758c: sub r0, r9, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(9)?,16)?)?;
    // 0x37590: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x37594: blx 0x82be4 <entry+0x4d758> @ imm = #0x4b648
    c.emit(arm32::branch_exchange_immediate(c.at() as u32,0x82be4)?)?;
    // 0x37598: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x3759c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x375a0: rsbgt r1, r0, #0
    c.emit(arm32::data_immediate(C::Gt,Op::Rsb,R::R1,R::R0,0)?)?;
    // 0x375a4: strh r1, [r6]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::new(6)?,0)?)?;
    // 0x375a8: ldrb r0, [r10, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,345)?)?;
    // 0x375ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x375b0: beq 0x375c4 <entry+0x2138>  @ imm = #0xc
    c.b(C::Eq,0x375c4)?;
    // 0x375b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x375b8: strb r0, [r10, #0x60b]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,1547)?)?;
    // 0x375bc: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // 0x375c0: b 0x375d8 <entry+0x214c>  @ imm = #0x10
    c.b(C::Al,0x375d8)?;
    // 0x375c4: ldrb r1, [r10, #0x2c3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,707)?)?;
    // 0x375c8: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // 0x375cc: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x375d0: tst r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,255)?)?;
    // 0x375d4: beq 0x375e0 <entry+0x2154>  @ imm = #0x4
    c.b(C::Eq,0x375e0)?;
    // 0x375d8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x375dc: strb r0, [r10, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,350)?)?;
    // 0x375e0: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x375e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x375e8: moveq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x375ec: streq r0, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(10)?,300)?)?;
    // 0x375f0: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x375f4: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x375f8: cmpne r0, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x375fc: bne 0x37628 <entry+0x219c>  @ imm = #0x24
    c.b(C::Ne,0x37628)?;
    // 0x37600: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x37604: beq 0x37628 <entry+0x219c>  @ imm = #0x1c
    c.b(C::Eq,0x37628)?;
    // 0x37608: ldr r0, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,300)?)?;
    // 0x3760c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37610: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37614: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x37618: movne r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x3761c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x37620: moveq r1, r0
    c.emit(arm32::mov_register(C::Eq,R::R1,R::R0))?;
    // 0x37624: strb r1, [r10, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,350)?)?;
    // 0x37628: ldr r0, [r10, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,556)?)?;
    // 0x3762c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37630: bne 0x37678 <entry+0x21ec>  @ imm = #0x40
    c.b(C::Ne,0x37678)?;
    // 0x37634: ldr r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,560)?)?;
    // 0x37638: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3763c: bne 0x37678 <entry+0x21ec>  @ imm = #0x34
    c.b(C::Ne,0x37678)?;
    // 0x37640: ldr r0, [r10, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,564)?)?;
    // 0x37644: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37648: bne 0x37678 <entry+0x21ec>  @ imm = #0x28
    c.b(C::Ne,0x37678)?;
    // 0x3764c: ldr r0, [r10, #0x238]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,568)?)?;
    // 0x37650: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37654: bne 0x37678 <entry+0x21ec>  @ imm = #0x1c
    c.b(C::Ne,0x37678)?;
    // 0x37658: ldr r1, [pc, #0x4d4]        @ 0x26a8 <entry+0x26a8>
    c.literal_cond(C::Al,R::R1,0x37b34)?;
    // 0x3765c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37660: str r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,560)?)?;
    // 0x37664: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37668: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x89dc
    c.bl(0x2ec94)?;
    // 0x3766c: ldr r1, [pc, #0x4c4]        @ 0x26ac <entry+0x26ac>
    c.literal_cond(C::Al,R::R1,0x37b38)?;
    // 0x37670: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37674: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x89e8
    c.bl(0x2ec94)?;
    // 0x37678: ldrb r0, [r10, #0x660]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,1632)?)?;
    // 0x3767c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37680: beq 0x376bc <entry+0x2230>  @ imm = #0x34
    c.b(C::Eq,0x376bc)?;
    // 0x37684: ldr r0, [r10, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,556)?)?;
    // 0x37688: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3768c: ldrne r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(10)?,560)?)?;
    // 0x37690: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37694: beq 0x376e8 <entry+0x225c>  @ imm = #0x4c
    c.b(C::Eq,0x376e8)?;
    // 0x37698: ldr r0, [r10, #0x234]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,564)?)?;
    // 0x3769c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x376a0: ldrne r0, [r10, #0x238]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(10)?,568)?)?;
    // 0x376a4: cmpne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x376a8: beq 0x376e8 <entry+0x225c>  @ imm = #0x38
    c.b(C::Eq,0x376e8)?;
    // 0x376ac: ldr r1, [pc, #0x488]        @ 0x26b0 <entry+0x26b0>
    c.literal_cond(C::Al,R::R1,0x37b3c)?;
    // 0x376b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x376b4: str r0, [r10, #0x238]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,568)?)?;
    // 0x376b8: b 0x376d4 <entry+0x2248>  @ imm = #0x14
    c.b(C::Al,0x376d4)?;
    // 0x376bc: ldr r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,560)?)?;
    // 0x376c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x376c4: bne 0x376e8 <entry+0x225c>  @ imm = #0x1c
    c.b(C::Ne,0x376e8)?;
    // 0x376c8: ldr r1, [pc, #0x470]        @ 0x26b4 <entry+0x26b4>
    c.literal_cond(C::Al,R::R1,0x37b40)?;
    // 0x376cc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x376d0: str r0, [r10, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,560)?)?;
    // 0x376d4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x376d8: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x8a4c
    c.bl(0x2ec94)?;
    // 0x376dc: ldr r1, [pc, #0x454]        @ 0x26ac <entry+0x26ac>
    c.literal_cond(C::Al,R::R1,0x37b38)?;
    // 0x376e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x376e4: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x8a58
    c.bl(0x2ec94)?;
    // 0x376e8: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x376ec: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x376f0: bne 0x37710 <entry+0x2284>  @ imm = #0x18
    c.b(C::Ne,0x37710)?;
    // 0x376f4: ldr r1, [r10, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,540)?)?;
    // 0x376f8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x376fc: bne 0x37734 <entry+0x22a8>  @ imm = #0x30
    c.b(C::Ne,0x37734)?;
    // 0x37700: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37704: moveq r1, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37708: streq r1, [r10, #0x1a8]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(10)?,424)?)?;
    // 0x3770c: b 0x37734 <entry+0x22a8>  @ imm = #0x20
    c.b(C::Al,0x37734)?;
    // 0x37710: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37714: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x37718: str r1, [r10, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,556)?)?;
    // 0x3771c: bne 0x37734 <entry+0x22a8>  @ imm = #0x10
    c.b(C::Ne,0x37734)?;
    // 0x37720: str r1, [r10, #0x234]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,564)?)?;
    // 0x37724: ldrb r1, [r10, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,572)?)?;
    // 0x37728: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x3772c: movne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37730: strbne r1, [r10, #0x608]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(10)?,1544)?)?;
    // 0x37734: ldr r1, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,80)?)?;
    // 0x37738: cmp r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,128)?)?;
    // 0x3773c: bhi 0x37750 <entry+0x22c4>  @ imm = #0xc
    c.b(C::Hi,0x37750)?;
    // 0x37740: ldr r1, [r5]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,0)?)?;
    // 0x37744: cmp r1, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,33)?)?;
    // 0x37748: movge r1, #32
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x3774c: strge r1, [r5]
    c.emit(arm32::transfer_word(C::Ge,false,R::R1,R::new(5)?,0)?)?;
    // 0x37750: ldr r2, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,76)?)?;
    // 0x37754: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x37758: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x3775c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x37760: moveq r0, #31
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,31)?)?;
    // 0x37764: moveq r1, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,5)?)?;
    // 0x37768: add r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3776c: lsr r6, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::R0,Shift::Lsr,R::R1,false)?)?;
    // 0x37770: cmp r6, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,12)?)?;
    // 0x37774: bhi 0x37790 <entry+0x2304>  @ imm = #0x14
    c.b(C::Hi,0x37790)?;
    // 0x37778: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x3777c: mov r5, r10
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // 0x37780: strb r0, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,584)?)?;
    // 0x37784: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37788: strb r0, [r5, #0x626]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,1574)?)?;
    // 0x3778c: b 0x377c8 <entry+0x233c>  @ imm = #0x34
    c.b(C::Al,0x377c8)?;
    // 0x37790: mov r5, r10
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(10)?))?;
    // 0x37794: ldrb r0, [r5, #0x626]!
    c.emit(arm32::transfer_byte_preindexed_writeback(C::Al,true,R::R0,R::new(5)?,1574)?)?;
    // 0x37798: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3779c: bne 0x377c8 <entry+0x233c>  @ imm = #0x24
    c.b(C::Ne,0x377c8)?;
    // 0x377a0: ldrb r0, [r10, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,350)?)?;
    // 0x377a4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x377a8: bne 0x377c8 <entry+0x233c>  @ imm = #0x18
    c.b(C::Ne,0x377c8)?;
    // 0x377ac: ldrb r0, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,584)?)?;
    // 0x377b0: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x377b4: bls 0x377e4 <entry+0x2358>  @ imm = #0x28
    c.b(C::Ls,0x377e4)?;
    // 0x377b8: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x377bc: blo 0x377ec <entry+0x2360>  @ imm = #0x28
    c.b(C::Cc,0x377ec)?;
    // 0x377c0: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x377c4: b 0x377e8 <entry+0x235c>  @ imm = #0x1c
    c.b(C::Al,0x377e8)?;
    // 0x377c8: ldrb r0, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,584)?)?;
    // 0x377cc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x377d0: bhi 0x377dc <entry+0x2350>  @ imm = #0x4
    c.b(C::Hi,0x377dc)?;
    // 0x377d4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x377d8: b 0x377e8 <entry+0x235c>  @ imm = #0x8
    c.b(C::Al,0x377e8)?;
    // 0x377dc: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x377e0: blo 0x377ec <entry+0x2360>  @ imm = #0x4
    c.b(C::Cc,0x377ec)?;
    // 0x377e4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x377e8: strb r0, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,584)?)?;
    // 0x377ec: ldr r2, [pc, #0x33c]        @ 0x26a4 <entry+0x26a4>
    c.literal_cond(C::Al,R::R2,0x37b30)?;
    // 0x377f0: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // 0x377f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x377f8: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x377fc: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37800: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x37804: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x3605c
    c.bl(0x17b0)?;
    // 0x37808: ldrb r0, [r10, #0x15d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,349)?)?;
    // 0x3780c: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R2,R::R0,1)?)?;
    // 0x37810: add r2, r2, r6, lsr #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(6)?,Shift::Lsr,1,false)?)?;
    // 0x37814: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37818: moveq r1, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x3781c: strbeq r1, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Eq,false,R::R1,R::new(10)?,584)?)?;
    // 0x37820: ldrbne r1, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::new(10)?,584)?)?;
    // 0x37824: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x37828: bhs 0x3785c <entry+0x23d0>  @ imm = #0x2c
    c.b(C::Cs,0x3785c)?;
    // 0x3782c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x37830: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x37834: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37838: bne 0x3784c <entry+0x23c0>  @ imm = #0xc
    c.b(C::Ne,0x3784c)?;
    // 0x3783c: strb r3, [r10, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,false,R::new(3)?,R::new(10)?,572)?)?;
    // 0x37840: ldr r7, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(10)?,108)?)?;
    // 0x37844: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x37848: streq r3, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::new(10)?,108)?)?;
    // 0x3784c: and r7, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,255)?)?;
    // 0x37850: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x37854: blo 0x37830 <entry+0x23a4>  @ imm = #-0x2c
    c.b(C::Cc,0x37830)?;
    // 0x37858: strb r1, [r10, #0x248]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,584)?)?;
    // 0x3785c: ldrb r2, [r5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(5)?,0)?)?;
    // 0x37860: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x37864: beq 0x37878 <entry+0x23ec>  @ imm = #0xc
    c.b(C::Eq,0x37878)?;
    // 0x37868: and r1, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,255)?)?;
    // 0x3786c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x37870: bls 0x37890 <entry+0x2404>  @ imm = #0x18
    c.b(C::Ls,0x37890)?;
    // 0x37874: b 0x378a4 <entry+0x2418>  @ imm = #0x28
    c.b(C::Al,0x378a4)?;
    // 0x37878: and r2, r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R1,255)?)?;
    // 0x3787c: cmp r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,3)?)?;
    // 0x37880: bhi 0x37868 <entry+0x23dc>  @ imm = #-0x20
    c.b(C::Hi,0x37868)?;
    // 0x37884: ldrb r2, [r10, #0x15e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,350)?)?;
    // 0x37888: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x3788c: bne 0x37868 <entry+0x23dc>  @ imm = #-0x2c
    c.b(C::Ne,0x37868)?;
    // 0x37890: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x37894: strb r1, [r10, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(10)?,572)?)?;
    // 0x37898: ldr r2, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,108)?)?;
    // 0x3789c: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x378a0: streq r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::new(10)?,108)?)?;
    // 0x378a4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x378a8: bne 0x37984 <entry+0x24f8>  @ imm = #0xd4
    c.b(C::Ne,0x37984)?;
    // 0x378ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x378b0: str r0, [r10, #0x318]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,792)?)?;
    // 0x378b4: str r0, [r10, #0x31c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,796)?)?;
    // 0x378b8: ldr r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,108)?)?;
    // 0x378bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x378c0: beq 0x37984 <entry+0x24f8>  @ imm = #0xbc
    c.b(C::Eq,0x37984)?;
    // 0x378c4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x378c8: bne 0x37958 <entry+0x24cc>  @ imm = #0x88
    c.b(C::Ne,0x37958)?;
    // 0x378cc: ldr r1, [pc, #0x270]        @ 0x26b8 <entry+0x26b8>
    c.literal_cond(C::Al,R::R1,0x37b44)?;
    // 0x378d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x378d4: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x8c48
    c.bl(0x2ec94)?;
    // 0x378d8: ldr r1, [pc, #0x258]        @ 0x26ac <entry+0x26ac>
    c.literal_cond(C::Al,R::R1,0x37b38)?;
    // 0x378dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x378e0: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x8c54
    c.bl(0x2ec94)?;
    // 0x378e4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x378e8: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,108)?)?;
    // 0x378ec: b 0x37984 <entry+0x24f8>  @ imm = #0x90
    c.b(C::Al,0x37984)?;
    // 0x378f0: cmp lr, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1920)?)?;
    // 0x378f4: bls 0x379fc <entry+0x2570>  @ imm = #0x100
    c.b(C::Ls,0x379fc)?;
    // 0x378f8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x378fc: str r0, [r10, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,284)?)?;
    // 0x37900: strb r0, [r10, #0x162]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,354)?)?;
    // 0x37904: ldr r0, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,0)?)?;
    // 0x37908: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3790c: beq 0x37ac0 <entry+0x2634>  @ imm = #0x1ac
    c.b(C::Eq,0x37ac0)?;
    // 0x37910: ldr r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,20)?)?;
    // 0x37914: cmp lr, #3840
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,3840)?)?;
    // 0x37918: bls 0x3794c <entry+0x24c0>  @ imm = #0x2c
    c.b(C::Ls,0x3794c)?;
    // 0x3791c: ldr r2, [pc, #0x20c]        @ 0x26a4 <entry+0x26a4>
    c.literal_cond(C::Al,R::R2,0x37b30)?;
    // 0x37920: mov r1, #53
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,53)?)?;
    // 0x37924: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37928: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x3792c: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x36184
    c.bl(0x17b0)?;
    // 0x37930: ldr r4, [r10, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(10)?,400)?)?;
    // 0x37934: ldr r9, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(10)?,404)?)?;
    // 0x37938: ldr lr, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::new(10)?,76)?)?;
    // 0x3793c: ldr r8, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(10)?,80)?)?;
    // 0x37940: ldr r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,288)?)?;
    // 0x37944: ldr r0, [r10, #0x128]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,296)?)?;
    // 0x37948: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x3794c: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x37950: movhs r1, #1920
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,1920)?)?;
    // 0x37954: b 0x37b08 <entry+0x267c>  @ imm = #0x1ac
    c.b(C::Al,0x37b08)?;
    // 0x37958: ldrb r0, [r10, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,572)?)?;
    // 0x3795c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x37960: bne 0x37984 <entry+0x24f8>  @ imm = #0x1c
    c.b(C::Ne,0x37984)?;
    // 0x37964: ldr r1, [pc, #0x1dc]        @ 0x26bc <entry+0x26bc>
    c.literal_cond(C::Al,R::R1,0x37b48)?;
    // 0x37968: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3796c: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x8ce0
    c.bl(0x2ec94)?;
    // 0x37970: ldr r1, [pc, #0x1c0]        @ 0x26ac <entry+0x26ac>
    c.literal_cond(C::Al,R::R1,0x37b38)?;
    // 0x37974: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37978: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x8cec
    c.bl(0x2ec94)?;
    // 0x3797c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37980: strb r0, [r10, #0x23c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,572)?)?;
    // 0x37984: ldr r0, [r10, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,300)?)?;
    // 0x37988: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3798c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x37990: strb r0, [r10, #0x170]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,368)?)?;
    // 0x37994: ldrb r0, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x37998: strb r0, [r10, #0x14d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,333)?)?;
    // 0x3799c: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x379a0: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x379a4: beq 0x379b4 <entry+0x2528>  @ imm = #0x8
    c.b(C::Eq,0x379b4)?;
    // 0x379a8: ldrb r0, [r10, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,345)?)?;
    // 0x379ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x379b0: beq 0x379bc <entry+0x2530>  @ imm = #0x4
    c.b(C::Eq,0x379bc)?;
    // 0x379b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x379b8: strb r0, [r10, #0x170]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,368)?)?;
    // 0x379bc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x379c0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x379c4: cmp lr, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1920)?)?;
    // 0x379c8: bls 0x37a48 <entry+0x25bc>  @ imm = #0x78
    c.b(C::Ls,0x37a48)?;
    // 0x379cc: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x379d0: cmp r9, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,33)?)?;
    // 0x379d4: str r0, [r10, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,372)?)?;
    // 0x379d8: blt 0x37ad4 <entry+0x2648>  @ imm = #0xf4
    c.b(C::Lt,0x37ad4)?;
    // 0x379dc: cmp r1, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,960)?)?;
    // 0x379e0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x379e4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x379e8: movhs r1, #960
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,960)?)?;
    // 0x379ec: str r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,288)?)?;
    // 0x379f0: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x379f4: strb r2, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x379f8: b 0x3743c <entry+0x1fb0>  @ imm = #-0x5c4
    c.b(C::Al,0x3743c)?;
    // 0x379fc: cmp lr, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,960)?)?;
    // 0x37a00: bls 0x37a78 <entry+0x25ec>  @ imm = #0x70
    c.b(C::Ls,0x37a78)?;
    // 0x37a04: ldr r0, [r10, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,284)?)?;
    // 0x37a08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37a0c: bne 0x37a98 <entry+0x260c>  @ imm = #0x84
    c.b(C::Ne,0x37a98)?;
    // 0x37a10: ldrb r0, [r10, #0x162]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,354)?)?;
    // 0x37a14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37a18: bne 0x37a98 <entry+0x260c>  @ imm = #0x78
    c.b(C::Ne,0x37a98)?;
    // 0x37a1c: cmp r9, #48
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,48)?)?;
    // 0x37a20: bne 0x3745c <entry+0x1fd0>  @ imm = #-0x5cc
    c.b(C::Ne,0x3745c)?;
    // 0x37a24: cmp r1, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,960)?)?;
    // 0x37a28: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37a2c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37a30: mov r9, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,48)?)?;
    // 0x37a34: movhs r1, #960
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,960)?)?;
    // 0x37a38: str r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,288)?)?;
    // 0x37a3c: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x37a40: strb r2, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x37a44: b 0x3745c <entry+0x1fd0>  @ imm = #-0x5f0
    c.b(C::Al,0x3745c)?;
    // 0x37a48: cmp lr, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,768)?)?;
    // 0x37a4c: bls 0x37438 <entry+0x1fac>  @ imm = #-0x61c
    c.b(C::Ls,0x37438)?;
    // 0x37a50: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x37a54: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37a58: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37a5c: mov r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,32)?)?;
    // 0x37a60: movhs r1, #768
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,768)?)?;
    // 0x37a64: str r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,288)?)?;
    // 0x37a68: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x37a6c: strb r2, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x37a70: str r9, [r10, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(10)?,396)?)?;
    // 0x37a74: b 0x37af4 <entry+0x2668>  @ imm = #0x78
    c.b(C::Al,0x37af4)?;
    // 0x37a78: cmp lr, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,768)?)?;
    // 0x37a7c: bls 0x3745c <entry+0x1fd0>  @ imm = #-0x628
    c.b(C::Ls,0x3745c)?;
    // 0x37a80: ldr r0, [r10, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,284)?)?;
    // 0x37a84: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37a88: bne 0x37a98 <entry+0x260c>  @ imm = #0x8
    c.b(C::Ne,0x37a98)?;
    // 0x37a8c: ldrb r0, [r10, #0x162]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,354)?)?;
    // 0x37a90: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x37a94: beq 0x3745c <entry+0x1fd0>  @ imm = #-0x640
    c.b(C::Eq,0x3745c)?;
    // 0x37a98: cmp r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,768)?)?;
    // 0x37a9c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37aa0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37aa4: movhs r1, #768
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,768)?)?;
    // 0x37aa8: str r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,288)?)?;
    // 0x37aac: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x37ab0: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x37ab4: strb r2, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x37ab8: str r0, [r10, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,396)?)?;
    // 0x37abc: b 0x3745c <entry+0x1fd0>  @ imm = #-0x668
    c.b(C::Al,0x3745c)?;
    // 0x37ac0: cmp r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,32)?)?;
    // 0x37ac4: bgt 0x37afc <entry+0x2670>  @ imm = #0x30
    c.b(C::Gt,0x37afc)?;
    // 0x37ac8: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x37acc: movhs r1, #1920
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,1920)?)?;
    // 0x37ad0: b 0x37b04 <entry+0x2678>  @ imm = #0x2c
    c.b(C::Al,0x37b04)?;
    // 0x37ad4: cmp r1, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1920)?)?;
    // 0x37ad8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37adc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37ae0: mov r9, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,32)?)?;
    // 0x37ae4: movhs r1, #1920
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,1920)?)?;
    // 0x37ae8: str r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,288)?)?;
    // 0x37aec: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x37af0: strb r2, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x37af4: str r9, [r10, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(10)?,404)?)?;
    // 0x37af8: b 0x3743c <entry+0x1fb0>  @ imm = #-0x6c4
    c.b(C::Al,0x3743c)?;
    // 0x37afc: cmp r1, #960
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,960)?)?;
    // 0x37b00: movhs r1, #960
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,960)?)?;
    // 0x37b04: ldr r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,20)?)?;
    // 0x37b08: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x37b0c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x37b10: str r1, [r10, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,288)?)?;
    // 0x37b14: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x37b18: strb r2, [r10, #0x14c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::new(10)?,332)?)?;
    // 0x37b1c: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,20)?)?;
    // 0x37b20: b 0x37464 <entry+0x1fd8>  @ imm = #-0x6c4
    c.b(C::Al,0x37464)?;
    c.check(0x37b24)?;
    for word in INPUT_MODE_LITERALS {c.emit(word)?;}
    c.check(0x37b4c)?;
    Ok(())
}

/// Compact controller mode defaults for two descriptor classes, which both
/// terminate with the original BX LR (not a synthetic function return).
fn assign_fixed_mode_defaults(c:&mut Code)->Result<(),String>{
    c.check(0x37b4c)?;
    // 0x37b4c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x37b50: mov r2, #192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,192)?)?;
    // 0x37b54: strb r1, [r0, #0x159]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,345)?)?;
    // 0x37b58: str r2, [r0, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,392)?)?;
    // 0x37b5c: str r2, [r0, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,400)?)?;
    // 0x37b60: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x37b64: strb r2, [r0, #0x418]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1048)?)?;
    // 0x37b68: strb r1, [r0, #0x419]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1049)?)?;
    // 0x37b6c: strb r2, [r0, #0x42f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1071)?)?;
    // 0x37b70: strb r1, [r0, #0x430]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1072)?)?;
    // 0x37b74: strb r2, [r0, #0x446]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1094)?)?;
    // 0x37b78: strb r1, [r0, #0x447]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1095)?)?;
    // 0x37b7c: strb r2, [r0, #0x45d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1117)?)?;
    // 0x37b80: strb r1, [r0, #0x45e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1118)?)?;
    // 0x37b84: strb r2, [r0, #0x474]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1140)?)?;
    // 0x37b88: strb r1, [r0, #0x475]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1141)?)?;
    // 0x37b8c: str r2, [r0, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,496)?)?;
    // 0x37b90: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x37b94: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x37b98: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x37b9c: strb r1, [r0, #0x159]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,345)?)?;
    // 0x37ba0: mov r1, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,48)?)?;
    // 0x37ba4: str r1, [r0, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,392)?)?;
    // 0x37ba8: mov r1, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,96)?)?;
    // 0x37bac: str r1, [r0, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,400)?)?;
    // 0x37bb0: mov r1, #23
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,23)?)?;
    // 0x37bb4: strb r1, [r0, #0x418]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1048)?)?;
    // 0x37bb8: strb r2, [r0, #0x419]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1049)?)?;
    // 0x37bbc: strb r1, [r0, #0x42f]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1071)?)?;
    // 0x37bc0: strb r2, [r0, #0x430]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1072)?)?;
    // 0x37bc4: strb r1, [r0, #0x446]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1094)?)?;
    // 0x37bc8: strb r2, [r0, #0x447]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1095)?)?;
    // 0x37bcc: strb r1, [r0, #0x45d]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1117)?)?;
    // 0x37bd0: strb r2, [r0, #0x45e]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1118)?)?;
    // 0x37bd4: strb r1, [r0, #0x474]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1140)?)?;
    // 0x37bd8: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x37bdc: strb r2, [r0, #0x475]
    c.emit(arm32::transfer_byte(C::Al,false,R::R2,R::R0,1141)?)?;
    // 0x37be0: str r1, [r0, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,496)?)?;
    // 0x37be4: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    select_input_configuration(c)?;
    assign_fixed_mode_defaults(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn input_configuration_exact_typed_layout(){
        let mut code=Code::new(START);
        emit(&mut code).unwrap();
        assert_eq!(code.bytes.len(),END-START);
        assert_eq!(&code.bytes[0x37b24-START..0x37b4c-START],
            &INPUT_MODE_LITERALS.iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>());
    }
    #[test]fn signed_halfword_indexed_writeback_bits_and_invalid_registers(){
        let r5=R::new(5).unwrap();let r6=R::new(6).unwrap();
        assert_eq!(encode_signed_halfword_register_writeback(C::Al,r6,r5,R::R1,true).unwrap(),0xe1b5_60f1);
        assert_eq!(encode_signed_halfword_register_writeback(C::Al,r5,r6,R::R0,true).unwrap(),0xe1b6_50f0);
        assert!(encode_signed_halfword_register_writeback(C::Al,r5,r5,R::R0,true).is_err());
        assert!(encode_signed_halfword_register_writeback(C::Al,R::PC,r5,R::R1,false).is_err());
    }
}
