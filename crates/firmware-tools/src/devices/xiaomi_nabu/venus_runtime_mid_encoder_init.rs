// SPDX-License-Identifier: GPL-3.0-only
//! Build controller surface descriptors and optional encoder resource slots.
//! This routine validates codec operating flags and writes several packed
//! hardware command blocks. All A32 instructions are represented by checked
//! operand encodings, while the seven words after POP {...,pc} are data.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2ddfc;
const END:usize=0x2ea50;
fn initialize_encoder_resource_slots(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2ddfc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2de00: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2de04: sub sp, sp, #52
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,52)?)?;
    // 0x2de08: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2de0c: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x2de10: ldr r1, [pc, #0xc20]        @ 0x1438 <entry+0x1438>
    c.literal(R::R1,0x2ea38)?;
    // 0x2de14: ldr r8, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,0)?)?;
    // 0x2de18: ldr r0, [pc, #0xc14]        @ 0x1434 <entry+0x1434>
    c.literal(R::R0,0x2ea34)?;
    // 0x2de1c: ldr r7, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,0)?)?;
    // 0x2de20: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2de24: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x2de28: bl 0x2ec94 <entry+0x1694>  @ imm = #0xe64
    c.bl(0x2ec94)?;
    // 0x2de2c: ldr r1, [pc, #0xc08]        @ 0x143c <entry+0x143c>
    c.literal(R::R1,0x2ea3c)?;
    // 0x2de30: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2de34: bl 0x2ec94 <entry+0x1694>  @ imm = #0xe58
    c.bl(0x2ec94)?;
    // 0x2de38: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2de3c: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2de40: mov r9, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,8192)?)?;
    // 0x2de44: and r5, r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(7)?,8192)?)?;
    // 0x2de48: str r7, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,12)?)?;
    // 0x2de4c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2de50: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2de54: and r0, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(7)?,2)?)?;
    // 0x2de58: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x2de5c: eor r0, r9, r0, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::new(9)?,R::R0,Shift::Lsl,12,false)?)?;
    // 0x2de60: str r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-32)?)?;
    // 0x2de64: and r0, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(7)?,1)?)?;
    // 0x2de68: mov r7, #10688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10688)?)?;
    // 0x2de6c: str r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-36)?)?;
    // 0x2de70: orr r7, r7, #1933312
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,1933312)?)?;
    // 0x2de74: bl 0x2f8ac <entry+0x22ac>  @ imm = #0x1a30
    c.bl(0x2f8ac)?;
    // 0x2de78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2de7c: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,4)?)?;
    // 0x2de80: beq 0x2dea4 <entry+0x8a4>   @ imm = #0x1c
    c.b(C::Eq,0x2dea4)?;
    // 0x2de84: mov r0, #1248
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1248)?)?;
    // 0x2de88: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2de8c: mov r10, #36864
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,36864)?)?;
    // 0x2de90: mov r9, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,4096)?)?;
    // 0x2de94: orr r0, r0, #970752
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,970752)?)?;
    // 0x2de98: movne r7, r0
    c.emit(arm32::mov_register(C::Ne,R::new(7)?,R::R0))?;
    // 0x2de9c: mov r0, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,480)?)?;
    // 0x2dea0: b 0x2decc <entry+0x8cc>   @ imm = #0x24
    c.b(C::Al,0x2decc)?;
    // 0x2dea4: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x2dea8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2deac: orr r0, r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
    // 0x2deb0: movne r9, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(9)?,R::R0,4096)?)?;
    // 0x2deb4: movne r0, #135
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,135)?)?;
    // 0x2deb8: lsr r1, r9, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(9)?,Shift::Lsr,4)?)?;
    // 0x2debc: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2dec0: smulbb r10, r1, r0
    c.emit(super::encode_smulbb(C::Al,R::new(10)?,R::R1,R::R0)?)?;
    // 0x2dec4: mov r0, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,480)?)?;
    // 0x2dec8: movne r0, #120
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,120)?)?;
    // 0x2decc: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2ded0: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x2ded4: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x2ded8: mov r4, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,14)?)?;
    // 0x2dedc: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x2dee0: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x2dee4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2dee8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2deec: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x2def0: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x2def4: str r4, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,0)?)?;
    // 0x2def8: mov r4, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,128)?)?;
    // 0x2defc: movne r4, #256
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,256)?)?;
    // 0x2df00: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2df04: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2df08: orr r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x2df0c: stmib r8, {r0, r1, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(8)?,R::R0.bit()|R::R1.bit()|R::new(5)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2df10: add r0, r8, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,20)?)?;
    // 0x2df14: str r1, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,16)?)?;
    // 0x2df18: stm r0, {r1, r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2df1c: bl 0x2f8d0 <entry+0x22d0>  @ imm = #0x19ac
    c.bl(0x2f8d0)?;
    // 0x2df20: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2df24: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2df28: mov r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8192)?)?;
    // 0x2df2c: mov r2, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,32)?)?;
    // 0x2df30: movne r0, #512
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,512)?)?;
    // 0x2df34: str r0, [r8, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,32)?)?;
    // 0x2df38: str r1, [r8, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,36)?)?;
    // 0x2df3c: str r2, [r8, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,40)?)?;
    // 0x2df40: mov r2, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,256)?)?;
    // 0x2df44: str r2, [r8, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,44)?)?;
    // 0x2df48: ldr r3, [r8, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,28)?)?;
    // 0x2df4c: str r0, [r8, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,48)?)?;
    // 0x2df50: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2df54: str r1, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,52)?)?;
    // 0x2df58: str r0, [r8, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,56)?)?;
    // 0x2df5c: str r2, [r8, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,60)?)?;
    // 0x2df60: add r0, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,2)?)?;
    // 0x2df64: str r0, [r8, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,28)?)?;
    // 0x2df68: ldr r5, [pc, #0xad0]        @ 0x1440 <entry+0x1440>
    c.literal(R::new(5)?,0x2ea40)?;
    // 0x2df6c: ldr r1, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-36)?)?;
    // 0x2df70: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2df74: mov r1, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,166)?)?;
    // 0x2df78: orr r1, r1, #31488
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,31488)?)?;
    // 0x2df7c: movne r9, #3808
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(9)?,R::R0,3808)?)?;
    // 0x2df80: moveq r5, r7
    c.emit(arm32::mov_register(C::Eq,R::new(5)?,R::new(7)?))?;
    // 0x2df84: moveq r1, r10
    c.emit(arm32::mov_register(C::Eq,R::R1,R::new(10)?))?;
    // 0x2df88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2df8c: str r9, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,24)?)?;
    // 0x2df90: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x2df94: bne 0x2dfac <entry+0x9ac>   @ imm = #0x10
    c.b(C::Ne,0x2dfac)?;
    // 0x2df98: ldr r2, [pc, #0xaa4]        @ 0x1444 <entry+0x1444>
    c.literal(R::R2,0x2ea44)?;
    // 0x2df9c: mov r1, #82
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,82)?)?;
    // 0x2dfa0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2dfa4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x2dfa8: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2c800
    c.bl(0x17b0)?;
    // 0x2dfac: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2dfb0: str r6, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R11,-36)?)?;
    // 0x2dfb4: bl 0x2f808 <entry+0x2208>  @ imm = #0x184c
    c.bl(0x2f808)?;
    // 0x2dfb8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2dfbc: beq 0x2e0c0 <entry+0xac0>   @ imm = #0xfc
    c.b(C::Eq,0x2e0c0)?;
    // 0x2dfc0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2dfc4: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x2dfc8: mov r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // 0x2dfcc: mov r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16)?)?;
    // 0x2dfd0: mov r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4096)?)?;
    // 0x2dfd4: mov r7, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,128)?)?;
    // 0x2dfd8: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x2dfdc: add lr, r8, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(8)?,108)?)?;
    // 0x2dfe0: add r9, r8, #164
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(8)?,164)?)?;
    // 0x2dfe4: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,32768)?)?;
    // 0x2dfe8: orr r5, r5, #49152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,49152)?)?;
    // 0x2dfec: str r0, [r8, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,64)?)?;
    // 0x2dff0: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x2dff4: ldr r12, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(8)?,20)?)?;
    // 0x2dff8: str r6, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,72)?)?;
    // 0x2dffc: str r0, [r8, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,76)?)?;
    // 0x2e000: str r1, [r8, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,80)?)?;
    // 0x2e004: str r2, [r8, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,84)?)?;
    // 0x2e008: str r7, [r8, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,88)?)?;
    // 0x2e00c: str r0, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,92)?)?;
    // 0x2e010: str r3, [r8, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,96)?)?;
    // 0x2e014: str r2, [r8, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,100)?)?;
    // 0x2e018: str r6, [r8, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,104)?)?;
    // 0x2e01c: stm lr, {r0, r1, r2, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::LR,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e020: mov lr, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,4)?)?;
    // 0x2e024: str r0, [r8, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,124)?)?;
    // 0x2e028: str r3, [r8, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,128)?)?;
    // 0x2e02c: str r2, [r8, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,132)?)?;
    // 0x2e030: str lr, [r8, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,68)?)?;
    // 0x2e034: str r5, [r8, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,136)?)?;
    // 0x2e038: add r5, r8, #148
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,148)?)?;
    // 0x2e03c: str r6, [r8, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,144)?)?;
    // 0x2e040: stm r5, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e044: mov r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,256)?)?;
    // 0x2e048: str r5, [r8, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,160)?)?;
    // 0x2e04c: stm r9, {r0, r1, r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(9)?,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e050: add r6, r8, #180
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,180)?)?;
    // 0x2e054: stm r6, {r0, r1, r2, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R0.bit()|R::R1.bit()|R::R2.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e058: add r6, r8, #196
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(8)?,196)?)?;
    // 0x2e05c: stm r6, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e060: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x2e064: str lr, [r8, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,140)?)?;
    // 0x2e068: orr r2, r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16384)?)?;
    // 0x2e06c: str r2, [r8, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,208)?)?;
    // 0x2e070: add r2, r8, #220
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,220)?)?;
    // 0x2e074: str r7, [r8, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,216)?)?;
    // 0x2e078: stm r2, {r0, r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(3)?.bit()|R::new(5)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e07c: add r2, r8, #236
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,236)?)?;
    // 0x2e080: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2e084: stm r2, {r0, r1, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::R1.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e088: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x2e08c: str r7, [r8, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,212)?)?;
    // 0x2e090: str r2, [r8, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,248)?)?;
    // 0x2e094: add r2, r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R12,4)?)?;
    // 0x2e098: add r12, r8, #260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(8)?,260)?)?;
    // 0x2e09c: str r2, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,20)?)?;
    // 0x2e0a0: str r4, [r8, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,256)?)?;
    // 0x2e0a4: stm r12, {r0, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::new(3)?.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e0a8: str r4, [r8, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,272)?)?;
    // 0x2e0ac: add r3, r8, #276
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,276)?)?;
    // 0x2e0b0: add r4, r8, #288
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,288)?)?;
    // 0x2e0b4: stm r3, {r0, r1, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e0b8: str r7, [r8, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,252)?)?;
    // 0x2e0bc: b 0x2e0c8 <entry+0xac8>   @ imm = #0x4
    c.b(C::Al,0x2e0c8)?;
    // 0x2e0c0: ldr r2, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,20)?)?;
    // 0x2e0c4: add r4, r8, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,64)?)?;
    // 0x2e0c8: mov r5, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,14)?)?;
    // 0x2e0cc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2e0d0: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x2e0d4: bne 0x2e0ec <entry+0xaec>   @ imm = #0x10
    c.b(C::Ne,0x2e0ec)?;
    // 0x2e0d8: ldr r2, [pc, #0x964]        @ 0x1444 <entry+0x1444>
    c.literal(R::R2,0x2ea44)?;
    // 0x2e0dc: mov r1, #115
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,115)?)?;
    // 0x2e0e0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2e0e4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x2e0e8: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2c940
    c.bl(0x17b0)?;
    // 0x2e0ec: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // 0x2e0f0: ldr r10, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-36)?)?;
    // 0x2e0f4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2e0f8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2e0fc: beq 0x2e108 <entry+0xb08>   @ imm = #0x4
    c.b(C::Eq,0x2e108)?;
    // 0x2e100: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x2e104: b 0x2e160 <entry+0xb60>   @ imm = #0x54
    c.b(C::Al,0x2e160)?;
    // 0x2e108: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // 0x2e10c: mov r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1024)?)?;
    // 0x2e110: mov r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8192)?)?;
    // 0x2e114: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x2e118: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // 0x2e11c: mov r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,6)?)?;
    // 0x2e120: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x2e124: orr r1, r1, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,536870912)?)?;
    // 0x2e128: stmib r4, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(4)?,R::R2.bit()|R::new(3)?.bit()|R::new(7)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2e12c: str r3, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,16)?)?;
    // 0x2e130: str r3, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,20)?)?;
    // 0x2e134: str r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,24)?)?;
    // 0x2e138: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // 0x2e13c: tst r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,12288)?)?;
    // 0x2e140: beq 0x2e14c <entry+0xb4c>   @ imm = #0x4
    c.b(C::Eq,0x2e14c)?;
    // 0x2e144: add r4, r4, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,28)?)?;
    // 0x2e148: b 0x2e160 <entry+0xb60>   @ imm = #0x10
    c.b(C::Al,0x2e160)?;
    // 0x2e14c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x2e150: str r2, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,28)?)?;
    // 0x2e154: str r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // 0x2e158: str r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,16)?)?;
    // 0x2e15c: add r4, r4, #36
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,36)?)?;
    // 0x2e160: mov r12, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,6)?)?;
    // 0x2e164: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2e168: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x2e16c: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x2e170: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,0)?)?;
    // 0x2e174: mov r2, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,65536)?)?;
    // 0x2e178: add r3, r4, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,28)?)?;
    // 0x2e17c: mov lr, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,5)?)?;
    // 0x2e180: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x2e184: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4096)?)?;
    // 0x2e188: str r7, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,4)?)?;
    // 0x2e18c: str r8, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,8)?)?;
    // 0x2e190: str r12, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,12)?)?;
    // 0x2e194: str r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,20)?)?;
    // 0x2e198: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x2e19c: str r2, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,24)?)?;
    // 0x2e1a0: stm r3, {r1, r2, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R1.bit()|R::R2.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e1a4: str r2, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,40)?)?;
    // 0x2e1a8: str r7, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,44)?)?;
    // 0x2e1ac: str r2, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,48)?)?;
    // 0x2e1b0: str r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,52)?)?;
    // 0x2e1b4: str r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,56)?)?;
    // 0x2e1b8: str lr, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,16)?)?;
    // 0x2e1bc: ldr r3, [pc, #0x884]        @ 0x1448 <entry+0x1448>
    c.literal(R::new(3)?,0x2ea48)?;
    // 0x2e1c0: beq 0x2e1d4 <entry+0xbd4>   @ imm = #0xc
    c.b(C::Eq,0x2e1d4)?;
    // 0x2e1c4: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // 0x2e1c8: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x2e1cc: add r12, r4, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(4)?,60)?)?;
    // 0x2e1d0: b 0x2e23c <entry+0xc3c>   @ imm = #0x64
    c.b(C::Al,0x2e23c)?;
    // 0x2e1d4: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2e1d8: add r1, r4, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,64)?)?;
    // 0x2e1dc: str r5, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,60)?)?;
    // 0x2e1e0: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // 0x2e1e4: stm r1, {r0, r8, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(8)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e1e8: mov r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32768)?)?;
    // 0x2e1ec: add r1, r4, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,92)?)?;
    // 0x2e1f0: str r8, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,76)?)?;
    // 0x2e1f4: str r8, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,80)?)?;
    // 0x2e1f8: str r8, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,84)?)?;
    // 0x2e1fc: str r5, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,88)?)?;
    // 0x2e200: stm r1, {r0, r8, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(8)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e204: str r8, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,108)?)?;
    // 0x2e208: str r8, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,112)?)?;
    // 0x2e20c: str r7, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,116)?)?;
    // 0x2e210: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2e214: mov r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x2e218: str r8, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,120)?)?;
    // 0x2e21c: add r12, r4, #140
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(4)?,140)?)?;
    // 0x2e220: str r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,124)?)?;
    // 0x2e224: str r8, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,128)?)?;
    // 0x2e228: str r7, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,132)?)?;
    // 0x2e22c: str r8, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,136)?)?;
    // 0x2e230: str r0, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,104)?)?;
    // 0x2e234: str r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,12)?)?;
    // 0x2e238: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x2e23c: str r5, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R12,0)?)?;
    // 0x2e240: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x2e244: mov r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7)?)?;
    // 0x2e248: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x2e24c: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x2e250: ldr r4, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R11,-32)?)?;
    // 0x2e254: orr r0, r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(4)?,2)?)?;
    // 0x2e258: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x2e25c: stmib r12, {r0, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::new(8)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2e260: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x2e264: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2e268: str r0, [r12, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,12)?)?;
    // 0x2e26c: str lr, [r12, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R12,20)?)?;
    // 0x2e270: str r8, [r12, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,24)?)?;
    // 0x2e274: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2e278: str r0, [r12, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,28)?)?;
    // 0x2e27c: str r8, [r12, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,32)?)?;
    // 0x2e280: str r1, [r12, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,36)?)?;
    // 0x2e284: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x2e288: add r1, r12, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R12,40)?)?;
    // 0x2e28c: stm r1, {r0, r2, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e290: str r7, [r12, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,52)?)?;
    // 0x2e294: str r0, [r12, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,56)?)?;
    // 0x2e298: str r2, [r12, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,60)?)?;
    // 0x2e29c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2e2a0: mov r2, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10)?)?;
    // 0x2e2a4: mov r7, #32000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,32000)?)?;
    // 0x2e2a8: str r8, [r12, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,64)?)?;
    // 0x2e2ac: mov r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,17)?)?;
    // 0x2e2b0: str r0, [r12, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,68)?)?;
    // 0x2e2b4: str r8, [r12, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,72)?)?;
    // 0x2e2b8: str r3, [r12, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,76)?)?;
    // 0x2e2bc: str r8, [r12, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,80)?)?;
    // 0x2e2c0: str r2, [r12, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,84)?)?;
    // 0x2e2c4: str r7, [r12, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,88)?)?;
    // 0x2e2c8: str r3, [r12, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R12,92)?)?;
    // 0x2e2cc: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x2e2d0: add r2, r12, #100
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R12,100)?)?;
    // 0x2e2d4: str r8, [r12, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,96)?)?;
    // 0x2e2d8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2e2dc: mov r7, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,40)?)?;
    // 0x2e2e0: stm r2, {r3, r9, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::new(3)?.bit()|R::new(9)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e2e4: mov lr, r5
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(5)?))?;
    // 0x2e2e8: mov r5, #288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,288)?)?;
    // 0x2e2ec: str r8, [r12, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,112)?)?;
    // 0x2e2f0: str r1, [r12, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,116)?)?;
    // 0x2e2f4: str r9, [r12, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R12,120)?)?;
    // 0x2e2f8: str r0, [r12, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,124)?)?;
    // 0x2e2fc: mov r1, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,39)?)?;
    // 0x2e300: mov r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,512)?)?;
    // 0x2e304: mov r0, #34560
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,34560)?)?;
    // 0x2e308: str r8, [r12, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,128)?)?;
    // 0x2e30c: orr r5, r5, #499712
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,499712)?)?;
    // 0x2e310: str r1, [r12, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,132)?)?;
    // 0x2e314: str r2, [r12, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,136)?)?;
    // 0x2e318: mov r1, #43
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,43)?)?;
    // 0x2e31c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2e320: str r5, [r12, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R12,140)?)?;
    // 0x2e324: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2e328: str r8, [r12, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,144)?)?;
    // 0x2e32c: str r7, [r12, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,148)?)?;
    // 0x2e330: str r8, [r12, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,152)?)?;
    // 0x2e334: str r0, [r12, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,156)?)?;
    // 0x2e338: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x2e33c: str r8, [r12, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,160)?)?;
    // 0x2e340: str r1, [r12, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,164)?)?;
    // 0x2e344: str r8, [r12, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,168)?)?;
    // 0x2e348: str r8, [r12, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,172)?)?;
    // 0x2e34c: str r8, [r12, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,176)?)?;
    // 0x2e350: mov r7, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,7)?)?;
    // 0x2e354: movne r2, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,2)?)?;
    // 0x2e358: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(10)?,0)?)?;
    // 0x2e35c: str r0, [r12, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,180)?)?;
    // 0x2e360: orr r5, r5, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,16777216)?)?;
    // 0x2e364: mov r0, #35
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,35)?)?;
    // 0x2e368: str r8, [r12, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,184)?)?;
    // 0x2e36c: str r6, [r12, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,188)?)?;
    // 0x2e370: str r8, [r12, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,192)?)?;
    // 0x2e374: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x2e378: str r0, [r12, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,196)?)?;
    // 0x2e37c: add r1, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,4)?)?;
    // 0x2e380: mov r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,12)?)?;
    // 0x2e384: str r5, [r12, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R12,200)?)?;
    // 0x2e388: str r1, [r12, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,204)?)?;
    // 0x2e38c: str r8, [r12, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,208)?)?;
    // 0x2e390: str r0, [r12, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,16)?)?;
    // 0x2e394: str lr, [r12, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R12,212)?)?;
    // 0x2e398: str r6, [r12, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,216)?)?;
    // 0x2e39c: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x2e3a0: add r6, r12, #240
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R12,240)?)?;
    // 0x2e3a4: str r8, [r12, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,220)?)?;
    // 0x2e3a8: str r7, [r12, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,224)?)?;
    // 0x2e3ac: str r8, [r12, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,228)?)?;
    // 0x2e3b0: str r0, [r12, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,232)?)?;
    // 0x2e3b4: str r9, [r12, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R12,236)?)?;
    // 0x2e3b8: stm r6, {r2, r8, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(6)?,R::R2.bit()|R::new(8)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e3bc: str r4, [r12, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,252)?)?;
    // 0x2e3c0: str r8, [r12, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,256)?)?;
    // 0x2e3c4: str r7, [r12, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,260)?)?;
    // 0x2e3c8: str r8, [r12, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,264)?)?;
    // 0x2e3cc: str r0, [r12, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,268)?)?;
    // 0x2e3d0: str r9, [r12, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R12,272)?)?;
    // 0x2e3d4: str r8, [r12, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,276)?)?;
    // 0x2e3d8: str r8, [r12, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R12,280)?)?;
    // 0x2e3dc: beq 0x2e3f0 <entry+0xdf0>   @ imm = #0xc
    c.b(C::Eq,0x2e3f0)?;
    // 0x2e3e0: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2e3e4: add r4, r12, #284
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R12,284)?)?;
    // 0x2e3e8: add r0, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,6)?)?;
    // 0x2e3ec: b 0x2e500 <entry+0xf00>   @ imm = #0x10c
    c.b(C::Al,0x2e500)?;
    // 0x2e3f0: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2e3f4: str lr, [r12, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R12,284)?)?;
    // 0x2e3f8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2e3fc: mov r4, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,6)?)?;
    // 0x2e400: mov r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,7)?)?;
    // 0x2e404: add r8, r12, #380
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R12,380)?)?;
    // 0x2e408: str r0, [r12, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,288)?)?;
    // 0x2e40c: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x2e410: str r2, [r12, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,292)?)?;
    // 0x2e414: str r7, [r12, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R12,296)?)?;
    // 0x2e418: str r0, [r12, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,304)?)?;
    // 0x2e41c: str r2, [r12, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,308)?)?;
    // 0x2e420: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // 0x2e424: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x2e428: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x2e42c: mov r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16)?)?;
    // 0x2e430: moveq r0, #240
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,240)?)?;
    // 0x2e434: str r0, [r12, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,312)?)?;
    // 0x2e438: str r2, [r12, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,316)?)?;
    // 0x2e43c: str r4, [r12, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,320)?)?;
    // 0x2e440: mov r4, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,8192)?)?;
    // 0x2e444: mov r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,65536)?)?;
    // 0x2e448: str r4, [r12, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,324)?)?;
    // 0x2e44c: str r0, [r12, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,328)?)?;
    // 0x2e450: str r2, [r12, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,332)?)?;
    // 0x2e454: str r6, [r12, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,336)?)?;
    // 0x2e458: str r4, [r12, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,340)?)?;
    // 0x2e45c: str r0, [r12, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,344)?)?;
    // 0x2e460: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2e464: str r2, [r12, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,348)?)?;
    // 0x2e468: mov r6, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,10)?)?;
    // 0x2e46c: mov r4, #32000
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,32000)?)?;
    // 0x2e470: str r0, [r12, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,352)?)?;
    // 0x2e474: str r2, [r12, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,356)?)?;
    // 0x2e478: ldr r0, [pc, #0x5cc]        @ 0x144c <entry+0x144c>
    c.literal(R::R0,0x2ea4c)?;
    // 0x2e47c: str r0, [r12, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,360)?)?;
    // 0x2e480: str r2, [r12, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,364)?)?;
    // 0x2e484: str r6, [r12, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,368)?)?;
    // 0x2e488: str r4, [r12, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,372)?)?;
    // 0x2e48c: str r0, [r12, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,376)?)?;
    // 0x2e490: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2e494: stm r8, {r2, r3, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(8)?,R::R2.bit()|R::new(3)?.bit()|R::new(9)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e498: mov r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x2e49c: add r8, r12, #432
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R12,432)?)?;
    // 0x2e4a0: str r0, [r12, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,392)?)?;
    // 0x2e4a4: mov r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,17)?)?;
    // 0x2e4a8: str r2, [r12, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,396)?)?;
    // 0x2e4ac: str r0, [r12, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,400)?)?;
    // 0x2e4b0: mov r0, #43
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,43)?)?;
    // 0x2e4b4: str r9, [r12, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R12,404)?)?;
    // 0x2e4b8: str r4, [r12, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R12,408)?)?;
    // 0x2e4bc: str r2, [r12, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,412)?)?;
    // 0x2e4c0: str r0, [r12, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,416)?)?;
    // 0x2e4c4: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x2e4c8: str r2, [r12, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,420)?)?;
    // 0x2e4cc: str r2, [r12, #0x1a8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,424)?)?;
    // 0x2e4d0: str r2, [r12, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,428)?)?;
    // 0x2e4d4: stm r8, {r0, r2, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(8)?,R::R0.bit()|R::R2.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e4d8: mov r0, #35
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,35)?)?;
    // 0x2e4dc: str r2, [r12, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,444)?)?;
    // 0x2e4e0: add r4, r12, #464
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R12,464)?)?;
    // 0x2e4e4: str r0, [r12, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,448)?)?;
    // 0x2e4e8: str r5, [r12, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R12,452)?)?;
    // 0x2e4ec: str r1, [r12, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R12,456)?)?;
    // 0x2e4f0: str r2, [r12, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R12,460)?)?;
    // 0x2e4f4: str r6, [r12, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R12,300)?)?;
    // 0x2e4f8: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2e4fc: add r0, r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
    // 0x2e500: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2e504: mov r8, lr
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::LR))?;
    // 0x2e508: ldr r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,32)?)?;
    // 0x2e50c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2e510: mov r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,32)?)?;
    // 0x2e514: mov r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,2)?)?;
    // 0x2e518: str r8, [r4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,0)?)?;
    // 0x2e51c: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x2e520: orr r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x2e524: str r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x2e528: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x2e52c: str r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,8)?)?;
    // 0x2e530: str r7, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,12)?)?;
    // 0x2e534: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,20)?)?;
    // 0x2e538: str r3, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,24)?)?;
    // 0x2e53c: str r3, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,28)?)?;
    // 0x2e540: str r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,32)?)?;
    // 0x2e544: str r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,16)?)?;
    // 0x2e548: str r8, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,36)?)?;
    // 0x2e54c: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // 0x2e550: str r2, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,40)?)?;
    // 0x2e554: mov r2, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,96)?)?;
    // 0x2e558: str r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,44)?)?;
    // 0x2e55c: str r7, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,48)?)?;
    // 0x2e560: str r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,56)?)?;
    // 0x2e564: str r5, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,60)?)?;
    // 0x2e568: str r5, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,64)?)?;
    // 0x2e56c: str r1, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,68)?)?;
    // 0x2e570: str r1, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,52)?)?;
    // 0x2e574: str r8, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,72)?)?;
    // 0x2e578: str r10, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,76)?)?;
    // 0x2e57c: str r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,80)?)?;
    // 0x2e580: str r7, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,84)?)?;
    // 0x2e584: str r1, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,92)?)?;
    // 0x2e588: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2e58c: mov r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,16)?)?;
    // 0x2e590: str r2, [r4, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,96)?)?;
    // 0x2e594: ldr lr, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,24)?)?;
    // 0x2e598: str lr, [r4, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,100)?)?;
    // 0x2e59c: str r3, [r4, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,104)?)?;
    // 0x2e5a0: str r10, [r4, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,108)?)?;
    // 0x2e5a4: str r2, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,112)?)?;
    // 0x2e5a8: str lr, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,116)?)?;
    // 0x2e5ac: str r3, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,120)?)?;
    // 0x2e5b0: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x2e5b4: str r0, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,124)?)?;
    // 0x2e5b8: str r1, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,128)?)?;
    // 0x2e5bc: str r6, [r4, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,132)?)?;
    // 0x2e5c0: str r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,136)?)?;
    // 0x2e5c4: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x2e5c8: mov r6, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,33)?)?;
    // 0x2e5cc: str r3, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,140)?)?;
    // 0x2e5d0: str r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,144)?)?;
    // 0x2e5d4: mov r3, #51
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,51)?)?;
    // 0x2e5d8: ldr r12, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,20)?)?;
    // 0x2e5dc: str r12, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(4)?,148)?)?;
    // 0x2e5e0: str r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,152)?)?;
    // 0x2e5e4: str r0, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,156)?)?;
    // 0x2e5e8: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x2e5ec: str r9, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,160)?)?;
    // 0x2e5f0: str r0, [r4, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,164)?)?;
    // 0x2e5f4: add r0, r4, #184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,184)?)?;
    // 0x2e5f8: str r1, [r4, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,168)?)?;
    // 0x2e5fc: str r5, [r4, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,172)?)?;
    // 0x2e600: str r9, [r4, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,176)?)?;
    // 0x2e604: str r3, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,180)?)?;
    // 0x2e608: add r5, r4, #260
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,260)?)?;
    // 0x2e60c: stm r0, {r1, r6, r9}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(6)?.bit()|R::new(9)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e610: mov r0, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,34)?)?;
    // 0x2e614: str r3, [r4, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,196)?)?;
    // 0x2e618: str r1, [r4, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,200)?)?;
    // 0x2e61c: mov r6, lr
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::LR))?;
    // 0x2e620: mov lr, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,7)?)?;
    // 0x2e624: str r0, [r4, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,204)?)?;
    // 0x2e628: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2e62c: str r9, [r4, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(4)?,208)?)?;
    // 0x2e630: str r3, [r4, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,212)?)?;
    // 0x2e634: str r1, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,216)?)?;
    // 0x2e638: orr lr, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,4096)?)?;
    // 0x2e63c: add r3, r4, #224
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,224)?)?;
    // 0x2e640: add r9, r4, #292
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(4)?,292)?)?;
    // 0x2e644: str r0, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,88)?)?;
    // 0x2e648: str r8, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,220)?)?;
    // 0x2e64c: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2e650: stm r3, {r0, r1, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e654: mov r0, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,36)?)?;
    // 0x2e658: add r3, r4, #240
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,240)?)?;
    // 0x2e65c: stm r3, {r0, r2, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R2.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e660: mov r3, #37
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,37)?)?;
    // 0x2e664: str r3, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,256)?)?;
    // 0x2e668: str r10, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,236)?)?;
    // 0x2e66c: stm r5, {r2, r6, r7, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R2.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e670: mov r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4096)?)?;
    // 0x2e674: str r5, [r4, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,276)?)?;
    // 0x2e678: str r1, [r4, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,280)?)?;
    // 0x2e67c: str lr, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(4)?,284)?)?;
    // 0x2e680: stm r9, {r0, r2, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(9)?,R::R0.bit()|R::R2.bit()|R::new(5)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e684: add r0, r4, #312
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,312)?)?;
    // 0x2e688: str r3, [r4, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,308)?)?;
    // 0x2e68c: stm r0, {r2, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(5)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e690: str r10, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,288)?)?;
    // 0x2e694: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,40)?)?;
    // 0x2e698: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2e69c: beq 0x2e6b0 <entry+0x10b0>  @ imm = #0xc
    c.b(C::Eq,0x2e6b0)?;
    // 0x2e6a0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2e6a4: add r1, r4, #324
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,324)?)?;
    // 0x2e6a8: add r10, r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R0,10)?)?;
    // 0x2e6ac: b 0x2e794 <entry+0x1194>  @ imm = #0xe0
    c.b(C::Al,0x2e794)?;
    // 0x2e6b0: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x2e6b4: add r3, r4, #328
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,328)?)?;
    // 0x2e6b8: str r8, [r4, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(4)?,324)?)?;
    // 0x2e6bc: stm r3, {r0, r1, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e6c0: add r0, r4, #344
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,344)?)?;
    // 0x2e6c4: stm r0, {r1, r2, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::R2.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e6c8: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2e6cc: str r1, [r4, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,356)?)?;
    // 0x2e6d0: str r10, [r4, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(4)?,360)?)?;
    // 0x2e6d4: str r2, [r4, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,364)?)?;
    // 0x2e6d8: str r6, [r4, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(4)?,368)?)?;
    // 0x2e6dc: str r1, [r4, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,372)?)?;
    // 0x2e6e0: add r2, r4, #392
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,392)?)?;
    // 0x2e6e4: str r0, [r4, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,376)?)?;
    // 0x2e6e8: str r1, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,380)?)?;
    // 0x2e6ec: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x2e6f0: str r0, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,384)?)?;
    // 0x2e6f4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2e6f8: str r1, [r4, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,388)?)?;
    // 0x2e6fc: stm r2, {r0, r1, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::R1.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e700: str r1, [r4, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,404)?)?;
    // 0x2e704: str r0, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,340)?)?;
    // 0x2e708: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x2e70c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2e710: beq 0x2e740 <entry+0x1140>  @ imm = #0x28
    c.b(C::Eq,0x2e740)?;
    // 0x2e714: mov r0, #19
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,19)?)?;
    // 0x2e718: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2e71c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x2e720: add r3, r4, #408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(4)?,408)?)?;
    // 0x2e724: stm r3, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::R1.bit()|R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e728: str r2, [r4, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,420)?)?;
    // 0x2e72c: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x2e730: add r1, r4, #424
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,424)?)?;
    // 0x2e734: mov r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8)?)?;
    // 0x2e738: str r0, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,340)?)?;
    // 0x2e73c: b 0x2e748 <entry+0x1148>  @ imm = #0x4
    c.b(C::Al,0x2e748)?;
    // 0x2e740: add r1, r4, #408
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,408)?)?;
    // 0x2e744: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x2e748: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x2e74c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2e750: mov r7, #63
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,63)?)?;
    // 0x2e754: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2e758: mov r5, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,33)?)?;
    // 0x2e75c: add r12, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::R1,32)?)?;
    // 0x2e760: stm r1, {r0, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(3)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e764: mov r0, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,34)?)?;
    // 0x2e768: str r6, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,12)?)?;
    // 0x2e76c: str r5, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,16)?)?;
    // 0x2e770: str r3, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,20)?)?;
    // 0x2e774: str r7, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,24)?)?;
    // 0x2e778: str r6, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,28)?)?;
    // 0x2e77c: stm r12, {r0, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::new(3)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e780: str r6, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,44)?)?;
    // 0x2e784: str r2, [r4, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,340)?)?;
    // 0x2e788: add r1, r1, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,48)?)?;
    // 0x2e78c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2e790: add r10, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::R0,12)?)?;
    // 0x2e794: ldr r0, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-36)?)?;
    // 0x2e798: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2e79c: bne 0x2e8e8 <entry+0x12e8>  @ imm = #0x144
    c.b(C::Ne,0x2e8e8)?;
    // 0x2e7a0: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // 0x2e7a4: mov r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,4096)?)?;
    // 0x2e7a8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x2e7ac: mov r5, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,96)?)?;
    // 0x2e7b0: mov r6, #4064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4064)?)?;
    // 0x2e7b4: mov r0, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1920)?)?;
    // 0x2e7b8: mov r12, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,3)?)?;
    // 0x2e7bc: add r4, r1, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R1,44)?)?;
    // 0x2e7c0: mov r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,4)?)?;
    // 0x2e7c4: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2e7c8: add r10, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(10)?,R::new(10)?,4)?)?;
    // 0x2e7cc: str r8, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,4)?)?;
    // 0x2e7d0: str r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,8)?)?;
    // 0x2e7d4: str lr, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,12)?)?;
    // 0x2e7d8: str r3, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,20)?)?;
    // 0x2e7dc: str r5, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,24)?)?;
    // 0x2e7e0: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x2e7e4: mov lr, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x2e7e8: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // 0x2e7ec: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2e7f0: moveq r0, #4096
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2e7f4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2e7f8: moveq r6, #36864
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,36864)?)?;
    // 0x2e7fc: str r0, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,28)?)?;
    // 0x2e800: str r3, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,32)?)?;
    // 0x2e804: str lr, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,36)?)?;
    // 0x2e808: str r5, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,40)?)?;
    // 0x2e80c: stm r4, {r0, r3, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(4)?,R::R0.bit()|R::new(3)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e810: mov r4, #14464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,14464)?)?;
    // 0x2e814: str r3, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,56)?)?;
    // 0x2e818: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2e81c: mov r0, #127
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,127)?)?;
    // 0x2e820: add r2, r1, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R1,80)?)?;
    // 0x2e824: str r6, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,60)?)?;
    // 0x2e828: mov r6, #53760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,53760)?)?;
    // 0x2e82c: orr r4, r4, #475136
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,475136)?)?;
    // 0x2e830: str r3, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,64)?)?;
    // 0x2e834: str r9, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::R1,68)?)?;
    // 0x2e838: str r3, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,72)?)?;
    // 0x2e83c: orr r6, r6, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,983040)?)?;
    // 0x2e840: moveq r4, r6
    c.emit(arm32::mov_register(C::Eq,R::new(4)?,R::new(6)?))?;
    // 0x2e844: mov r6, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,32)?)?;
    // 0x2e848: str r4, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,76)?)?;
    // 0x2e84c: mov r4, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,33)?)?;
    // 0x2e850: stm r2, {r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e854: str r0, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,92)?)?;
    // 0x2e858: str r3, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,96)?)?;
    // 0x2e85c: mov r6, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,60)?)?;
    // 0x2e860: str r4, [r1, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,100)?)?;
    // 0x2e864: str r7, [r1, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,104)?)?;
    // 0x2e868: str r0, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,108)?)?;
    // 0x2e86c: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x2e870: str r3, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,112)?)?;
    // 0x2e874: mov r7, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,5)?)?;
    // 0x2e878: str r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,16)?)?;
    // 0x2e87c: mov r0, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,14)?)?;
    // 0x2e880: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2e884: str r0, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,116)?)?;
    // 0x2e888: mov r0, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32768)?)?;
    // 0x2e88c: str r0, [r1, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,120)?)?;
    // 0x2e890: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x2e894: str r3, [r1, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,124)?)?;
    // 0x2e898: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2e89c: str r0, [r1, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,128)?)?;
    // 0x2e8a0: add r0, r1, #136
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R1,136)?)?;
    // 0x2e8a4: stm r0, {r3, r5, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(3)?.bit()|R::new(5)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e8a8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2e8ac: str r0, [r1, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,148)?)?;
    // 0x2e8b0: str lr, [r1, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,152)?)?;
    // 0x2e8b4: mov lr, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,7)?)?;
    // 0x2e8b8: str r5, [r1, #0x9c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,156)?)?;
    // 0x2e8bc: str r8, [r1, #0xa0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,160)?)?;
    // 0x2e8c0: mov r8, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,14)?)?;
    // 0x2e8c4: str r0, [r1, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,164)?)?;
    // 0x2e8c8: str r7, [r1, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,168)?)?;
    // 0x2e8cc: str r3, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,172)?)?;
    // 0x2e8d0: str r6, [r1, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,176)?)?;
    // 0x2e8d4: str r3, [r1, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,180)?)?;
    // 0x2e8d8: str r12, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,132)?)?;
    // 0x2e8dc: add r1, r1, #184
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,184)?)?;
    // 0x2e8e0: orr lr, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,4096)?)?;
    // 0x2e8e4: orr r8, r8, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::new(8)?,4096)?)?;
    // 0x2e8e8: ldr r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-32)?)?;
    // 0x2e8ec: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2e8f0: mov r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x2e8f4: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x2e8f8: mov r6, #17
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,17)?)?;
    // 0x2e8fc: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // 0x2e900: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8192)?)?;
    // 0x2e904: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x2e908: stmib r1, {r0, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(3)?.bit()|R::new(7)?.bit(),BlockMode::IncrementBefore,false)?)?;
    // 0x2e90c: mov r0, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,47)?)?;
    // 0x2e910: str r6, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,16)?)?;
    // 0x2e914: str r8, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,20)?)?;
    // 0x2e918: mov r7, #90
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,90)?)?;
    // 0x2e91c: orr r5, r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R2,2)?)?;
    // 0x2e920: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,4096)?)?;
    // 0x2e924: str r5, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,24)?)?;
    // 0x2e928: str r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,28)?)?;
    // 0x2e92c: str lr, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R1,32)?)?;
    // 0x2e930: str r0, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,40)?)?;
    // 0x2e934: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2e938: str r3, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,44)?)?;
    // 0x2e93c: str r0, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,48)?)?;
    // 0x2e940: str r7, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,52)?)?;
    // 0x2e944: mov r7, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,48)?)?;
    // 0x2e948: str r7, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,56)?)?;
    // 0x2e94c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2e950: str r7, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,60)?)?;
    // 0x2e954: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2e958: str r7, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,64)?)?;
    // 0x2e95c: str r3, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,68)?)?;
    // 0x2e960: str r7, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,36)?)?;
    // 0x2e964: ldr r2, [r11, #-0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-36)?)?;
    // 0x2e968: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2e96c: beq 0x2e97c <entry+0x137c>  @ imm = #0x8
    c.b(C::Eq,0x2e97c)?;
    // 0x2e970: add r0, r10, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,4)?)?;
    // 0x2e974: add r1, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,72)?)?;
    // 0x2e978: b 0x2ea24 <entry+0x1424>  @ imm = #0xa4
    c.b(C::Al,0x2ea24)?;
    // 0x2e97c: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x2e980: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x2e984: mov r12, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,32768)?)?;
    // 0x2e988: str r8, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,72)?)?;
    // 0x2e98c: mov r5, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,8192)?)?;
    // 0x2e990: mov r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16)?)?;
    // 0x2e994: mov r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4096)?)?;
    // 0x2e998: mov lr, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,32)?)?;
    // 0x2e99c: add r9, r1, #120
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R1,120)?)?;
    // 0x2e9a0: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x2e9a4: orr r6, r6, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,32768)?)?;
    // 0x2e9a8: str r12, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,76)?)?;
    // 0x2e9ac: str r3, [r1, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,80)?)?;
    // 0x2e9b0: str r2, [r1, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,84)?)?;
    // 0x2e9b4: str r3, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,88)?)?;
    // 0x2e9b8: str r6, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,96)?)?;
    // 0x2e9bc: mov r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // 0x2e9c0: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x2e9c4: str r3, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,92)?)?;
    // 0x2e9c8: str r6, [r1, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,104)?)?;
    // 0x2e9cc: str r5, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,108)?)?;
    // 0x2e9d0: str r4, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,112)?)?;
    // 0x2e9d4: str r7, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,116)?)?;
    // 0x2e9d8: stm r9, {r2, r5, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(9)?,R::R2.bit()|R::new(5)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e9dc: str r7, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,132)?)?;
    // 0x2e9e0: str r6, [r1, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,136)?)?;
    // 0x2e9e4: str r5, [r1, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,140)?)?;
    // 0x2e9e8: str r4, [r1, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,144)?)?;
    // 0x2e9ec: add r4, r1, #152
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R1,152)?)?;
    // 0x2e9f0: str r7, [r1, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,148)?)?;
    // 0x2e9f4: stm r4, {r2, r5, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(4)?,R::R2.bit()|R::new(5)?.bit()|R::LR.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2e9f8: str r7, [r1, #0xa4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,164)?)?;
    // 0x2e9fc: str r0, [r1, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,100)?)?;
    // 0x2ea00: mov r0, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,22)?)?;
    // 0x2ea04: str r8, [r1, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,168)?)?;
    // 0x2ea08: str r12, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::R1,172)?)?;
    // 0x2ea0c: str r3, [r1, #0xb0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,176)?)?;
    // 0x2ea10: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2ea14: str r0, [r1, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,180)?)?;
    // 0x2ea18: str r3, [r1, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,184)?)?;
    // 0x2ea1c: add r0, r10, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,8)?)?;
    // 0x2ea20: add r1, r1, #188
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,188)?)?;
    // 0x2ea24: ldr r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,28)?)?;
    // 0x2ea28: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x2ea2c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2ea30: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2ea34)?;
    const CODEC_INIT_REFERENCES_2EA34:[u32;7]=[0xe00e0008,0x000eabd4,0x000e2788,0x000e7d74,0x000eabf5,0x09896800,0x02625a00];
    for word in CODEC_INIT_REFERENCES_2EA34 {c.emit(word)?;}
    c.check(0x2ea50)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;initialize_encoder_resource_slots(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_resource_initialization_has_structured_literals(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let data=|at:usize|u32::from_le_bytes(c.bytes[at-START..at-START+4].try_into().unwrap());
        assert_eq!(data(0x2ea34),0xe00e0008);
        assert_eq!(data(0x2ea38),0x000eabd4);
        assert_eq!(data(0x2ea4c),0x02625a00);
        assert_eq!(data(0x2ea48),0x09896800);
    }
}
