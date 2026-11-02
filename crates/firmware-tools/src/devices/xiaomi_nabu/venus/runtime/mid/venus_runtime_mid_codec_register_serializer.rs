// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec-controller command register serialization. The large writer
//! maps hundreds of model/format fields to independent command registers and
//! buffers, with retry/diagnostic callback paths. The tail of its function is
//! explicit non-instruction data for controller layout and log messages.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x3027c;
const END:usize=0x311c0;
fn serialize_controller_registers(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3027c: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x30280: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x30284: sub sp, sp, #44
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,44)?)?;
    // 0x30288: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x3028c: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x30290: str r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,24)?)?;
    // 0x30294: mov r7, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,3)?)?;
    // 0x30298: mov r5, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4096)?)?;
    // 0x3029c: mov r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // 0x302a0: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x302a4: orr r1, r1, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15360)?)?;
    // 0x302a8: and r5, r5, r3, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(5)?,R::new(5)?,R::new(3)?,Shift::Lsl,12,false)?)?;
    // 0x302ac: mov r3, #60
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,60)?)?;
    // 0x302b0: orr r2, r1, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R1,R::R0,Shift::Lsl,15,false)?)?;
    // 0x302b4: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x302b8: orr r3, r3, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,14336)?)?;
    // 0x302bc: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x302c0: mov r2, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12)?)?;
    // 0x302c4: mov r7, #129
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,129)?)?;
    // 0x302c8: orr r3, r3, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x302cc: orr r2, r2, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,15360)?)?;
    // 0x302d0: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x302d4: str r7, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x302d8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x302dc: str r7, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R2,0)?)?;
    // 0x302e0: mov r2, #788
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,788)?)?;
    // 0x302e4: ldr r7, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,4)?)?;
    // 0x302e8: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x302ec: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x302f0: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x302f4: mov r2, #340
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,340)?)?;
    // 0x302f8: mov r7, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,15)?)?;
    // 0x302fc: orr r2, r2, #15360
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,15360)?)?;
    // 0x30300: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30304: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30308: mov r2, #2256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2256)?)?;
    // 0x3030c: ldr r7, [r4, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,996)?)?;
    // 0x30310: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x30314: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30318: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x3031c: mov r2, #232
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,232)?)?;
    // 0x30320: ldr r7, [r4, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,1000)?)?;
    // 0x30324: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x30328: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x3032c: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30330: mov r2, #236
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,236)?)?;
    // 0x30334: ldr r7, [r4, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,944)?)?;
    // 0x30338: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x3033c: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30340: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30344: ldr r2, [pc, #0xe44]        @ 0x1b90 <entry+0x1b90>
    c.literal(R::R2,0x31190)?;
    // 0x30348: ldr r7, [r4, #0x398]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,920)?)?;
    // 0x3034c: ldr r6, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,0)?)?;
    // 0x30350: bic r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x30354: orr r7, r7, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x30358: lsl r5, r0, #15
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R0,Shift::Lsl,15)?)?;
    // 0x3035c: and r6, r12, r6, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::R12,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x30360: orr r7, r6, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x30364: str r7, [r4, #0x398]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,920)?)?;
    // 0x30368: mov r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,14336)?)?;
    // 0x3036c: ldr r6, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,0)?)?;
    // 0x30370: orr r7, r7, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30374: str r6, [r7, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30378: mov r7, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,28)?)?;
    // 0x3037c: ldr r6, [r4, #0x398]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,920)?)?;
    // 0x30380: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x30384: orr r7, r7, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30388: str r6, [r7, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(6)?,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x3038c: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x30390: ldr r7, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,924)?)?;
    // 0x30394: and r2, r12, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R12,R::R2,Shift::Lsl,1,false)?)?;
    // 0x30398: orr r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3039c: mov r7, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,24)?)?;
    // 0x303a0: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x303a4: str r2, [r4, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,924)?)?;
    // 0x303a8: orr r7, r7, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::new(7)?,R::R0,Shift::Lsl,15,false)?)?;
    // 0x303ac: str r2, [r7, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x303b0: mov r2, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,56)?)?;
    // 0x303b4: ldr r7, [r4, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,928)?)?;
    // 0x303b8: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x303bc: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x303c0: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x303c4: mov r2, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,104)?)?;
    // 0x303c8: ldr r7, [r4, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,932)?)?;
    // 0x303cc: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x303d0: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x303d4: str r7, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(7)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x303d8: add r2, r4, #988
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,988)?)?;
    // 0x303dc: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x303e0: addne r2, r4, #992
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::new(4)?,992)?)?;
    // 0x303e4: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,0)?)?;
    // 0x303e8: str r2, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x303ec: mov r2, #14592
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14592)?)?;
    // 0x303f0: ldr r3, [r4, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,940)?)?;
    // 0x303f4: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x303f8: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x303fc: mov r2, #628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,628)?)?;
    // 0x30400: ldr r3, [r4, #0x13c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,316)?)?;
    // 0x30404: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x30408: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x3040c: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30410: mov r2, #14976
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,14976)?)?;
    // 0x30414: ldr r3, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,396)?)?;
    // 0x30418: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x3041c: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30420: mov r2, #644
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,644)?)?;
    // 0x30424: ldr r3, [r4, #0x10c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,268)?)?;
    // 0x30428: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x3042c: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30430: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30434: mov r2, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,648)?)?;
    // 0x30438: ldr r3, [r4, #0xec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,236)?)?;
    // 0x3043c: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x30440: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30444: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30448: mov r2, #652
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,652)?)?;
    // 0x3044c: ldr r3, [r4, #0xcc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,204)?)?;
    // 0x30450: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x30454: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30458: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x3045c: mov r2, #2704
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2704)?)?;
    // 0x30460: ldr r3, [r4, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,172)?)?;
    // 0x30464: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x30468: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x3046c: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30470: mov r2, #660
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,660)?)?;
    // 0x30474: ldr r3, [r4, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,428)?)?;
    // 0x30478: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x3047c: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30480: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30484: mov r2, #664
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,664)?)?;
    // 0x30488: ldr r3, [r4, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,460)?)?;
    // 0x3048c: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x30490: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x30494: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30498: mov r2, #668
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,668)?)?;
    // 0x3049c: ldr r3, [r4, #0x20c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,524)?)?;
    // 0x304a0: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x304a4: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x304a8: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x304ac: mov r2, #2720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2720)?)?;
    // 0x304b0: ldr r3, [r4, #0x22c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,556)?)?;
    // 0x304b4: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x304b8: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x304bc: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x304c0: mov r2, #676
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,676)?)?;
    // 0x304c4: ldr r3, [r4, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,620)?)?;
    // 0x304c8: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x304cc: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x304d0: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x304d4: mov r2, #680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,680)?)?;
    // 0x304d8: ldr r3, [r4, #0x28c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,652)?)?;
    // 0x304dc: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x304e0: orr r2, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x304e4: str r3, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x304e8: mov r2, #420
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,420)?)?;
    // 0x304ec: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x304f0: ldr r3, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,980)?)?;
    // 0x304f4: orr r6, r2, r0, lsl #15
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(6)?,R::R2,R::R0,Shift::Lsl,15,false)?)?;
    // 0x304f8: ldr r2, [r4, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,964)?)?;
    // 0x304fc: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x30500: and r3, r7, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x30504: and r2, r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,3)?)?;
    // 0x30508: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x3050c: str r2, [r6, r1]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30510: ldr r2, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,12)?)?;
    // 0x30514: ldr r0, [r4, #0x3c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,964)?)?;
    // 0x30518: str r2, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-32)?)?;
    // 0x3051c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x30520: ldr r2, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,8)?)?;
    // 0x30524: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,4)?)?;
    // 0x30528: ldrne r0, [r4, #0x3d4]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,980)?)?;
    // 0x3052c: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x30530: bne 0x30594 <entry+0xf94>   @ imm = #0x5c
    c.b(C::Ne,0x30594)?;
    // 0x30534: orr r0, r5, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,852)?)?;
    // 0x30538: orr r2, r5, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,2896)?)?;
    // 0x3053c: orr r7, r5, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(5)?,972)?)?;
    // 0x30540: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30544: orr r2, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,12288)?)?;
    // 0x30548: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x3054c: str r12, [r0, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::R0,R::R1,Shift::Lsl,0)?)?;
    // 0x30550: ldr r3, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,300)?)?;
    // 0x30554: str r3, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30558: orr r3, r5, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(5)?,3024)?)?;
    // 0x3055c: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x30560: str r12, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R12,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30564: ldr r6, [r4, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,304)?)?;
    // 0x30568: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // 0x3056c: str r6, [r7, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30570: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x30574: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x30578: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x3057c: ldr r0, [r4, #0x12c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,300)?)?;
    // 0x30580: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30584: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x30588: ldr r0, [r4, #0x130]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,304)?)?;
    // 0x3058c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30590: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30594: orr r0, r5, #424
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,424)?)?;
    // 0x30598: ldr r1, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,972)?)?;
    // 0x3059c: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x305a0: orr r3, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::R0,14336)?)?;
    // 0x305a4: ldr r0, [r4, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,956)?)?;
    // 0x305a8: str r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,16)?)?;
    // 0x305ac: and r1, r2, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // 0x305b0: and r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,3)?)?;
    // 0x305b4: orr r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x305b8: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // 0x305bc: str r1, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x305c0: ldr r1, [r4, #0x3bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,956)?)?;
    // 0x305c4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x305c8: ldrne r1, [r4, #0x3cc]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,972)?)?;
    // 0x305cc: cmpne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x305d0: bne 0x30638 <entry+0x1038>  @ imm = #0x60
    c.b(C::Ne,0x30638)?;
    // 0x305d4: orr r1, r5, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,852)?)?;
    // 0x305d8: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x305dc: orr r3, r5, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(5)?,2896)?)?;
    // 0x305e0: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x305e4: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x305e8: str r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x305ec: ldr r7, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,380)?)?;
    // 0x305f0: str r7, [r3, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x305f4: orr r7, r5, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(5)?,3024)?)?;
    // 0x305f8: orr r7, r7, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,12288)?)?;
    // 0x305fc: str r2, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x30600: orr r2, r5, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,972)?)?;
    // 0x30604: ldr r6, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,384)?)?;
    // 0x30608: orr r2, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,14336)?)?;
    // 0x3060c: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // 0x30610: str r6, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // 0x30614: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x30618: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x3061c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30620: ldr r1, [r4, #0x17c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,380)?)?;
    // 0x30624: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x30628: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x3062c: ldr r0, [r4, #0x180]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,384)?)?;
    // 0x30630: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30634: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30638: ldr r0, [r4, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,960)?)?;
    // 0x3063c: mov r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,768)?)?;
    // 0x30640: mov r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,12288)?)?;
    // 0x30644: ldr r1, [r4, #0x3d0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,976)?)?;
    // 0x30648: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // 0x3064c: and r3, r3, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R0,Shift::Lsl,8,false)?)?;
    // 0x30650: and r7, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R0,3)?)?;
    // 0x30654: and r2, r2, r0, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R0,Shift::Lsl,12,false)?)?;
    // 0x30658: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x3065c: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x30660: mov r3, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,196608)?)?;
    // 0x30664: and r0, r3, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(3)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x30668: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3066c: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x30670: and r3, r2, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::R2,R::R1,Shift::Lsl,2,false)?)?;
    // 0x30674: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x30678: mov r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1024)?)?;
    // 0x3067c: and r3, r3, r1, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,10,false)?)?;
    // 0x30680: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x30684: mov r3, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16384)?)?;
    // 0x30688: and r3, r3, r1, lsl #14
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,14,false)?)?;
    // 0x3068c: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x30690: mov r3, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,262144)?)?;
    // 0x30694: and r1, r3, r1, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,18,false)?)?;
    // 0x30698: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3069c: orr r1, r5, #428
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,428)?)?;
    // 0x306a0: orr r1, r1, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
    // 0x306a4: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x306a8: str r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x306ac: ldr r0, [r4, #0x3c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,960)?)?;
    // 0x306b0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x306b4: ldrne r0, [r4, #0x3d0]
    c.emit(arm32::transfer_word(C::Ne,true,R::R0,R::new(4)?,976)?)?;
    // 0x306b8: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x306bc: bne 0x307ec <entry+0x11ec>  @ imm = #0x128
    c.b(C::Ne,0x307ec)?;
    // 0x306c0: orr r0, r5, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,852)?)?;
    // 0x306c4: orr r1, r5, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::new(5)?,2896)?)?;
    // 0x306c8: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x306cc: orr r1, r1, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,12288)?)?;
    // 0x306d0: str r2, [r0, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x306d4: ldr r3, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,252)?)?;
    // 0x306d8: str r3, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(3)?,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x306dc: orr r3, r5, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(5)?,3024)?)?;
    // 0x306e0: orr r3, r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,12288)?)?;
    // 0x306e4: str r2, [r3, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x306e8: orr r2, r5, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,972)?)?;
    // 0x306ec: orr r7, r2, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::R2,14336)?)?;
    // 0x306f0: ldr r2, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,256)?)?;
    // 0x306f4: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x306f8: str r2, [r7, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x306fc: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x30700: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x30704: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x30708: ldr r6, [r4, #0xfc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,252)?)?;
    // 0x3070c: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x30710: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30714: ldr r2, [r4, #0x100]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,256)?)?;
    // 0x30718: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x3071c: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x30720: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x30724: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x30728: ldr r6, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,220)?)?;
    // 0x3072c: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x30730: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30734: ldr r2, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,224)?)?;
    // 0x30738: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x3073c: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x30740: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x30744: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x30748: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x3074c: ldr r6, [r4, #0xdc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,220)?)?;
    // 0x30750: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x30754: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30758: ldr r2, [r4, #0xe0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,224)?)?;
    // 0x3075c: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30760: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x30764: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x30768: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x3076c: ldr r6, [r4, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,188)?)?;
    // 0x30770: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x30774: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30778: ldr r2, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,192)?)?;
    // 0x3077c: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30780: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x30784: mov r2, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,52)?)?;
    // 0x30788: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x3078c: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x30790: ldr r6, [r4, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,188)?)?;
    // 0x30794: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x30798: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x3079c: ldr r2, [r4, #0xc0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,192)?)?;
    // 0x307a0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x307a4: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x307a8: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x307ac: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x307b0: ldr r6, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,156)?)?;
    // 0x307b4: str r6, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,0)?)?;
    // 0x307b8: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x307bc: ldr r2, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,160)?)?;
    // 0x307c0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x307c4: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(7)?,0)?)?;
    // 0x307c8: mov r2, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,68)?)?;
    // 0x307cc: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x307d0: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x307d4: ldr r0, [r4, #0x9c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,156)?)?;
    // 0x307d8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x307dc: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x307e0: ldr r0, [r4, #0xa0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,160)?)?;
    // 0x307e4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x307e8: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x307ec: orr r0, r5, #852
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,852)?)?;
    // 0x307f0: mov r1, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3758096384)?)?;
    // 0x307f4: orr r2, r5, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,2896)?)?;
    // 0x307f8: mov r10, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,18)?)?;
    // 0x307fc: orr r12, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R0,14336)?)?;
    // 0x30800: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x30804: orr r8, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::R2,12288)?)?;
    // 0x30808: orr r10, r10, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,65536)?)?;
    // 0x3080c: mov r3, r12
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R12))?;
    // 0x30810: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x30814: str r0, [r3, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(3)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30818: ldr r2, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,284)?)?;
    // 0x3081c: str r2, [r7, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30820: orr r2, r5, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::new(5)?,3024)?)?;
    // 0x30824: orr r9, r2, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::R2,12288)?)?;
    // 0x30828: mov r6, r9
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(9)?))?;
    // 0x3082c: str r0, [r6, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30830: orr r0, r5, #972
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(5)?,972)?)?;
    // 0x30834: str r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,28)?)?;
    // 0x30838: orr lr, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::R0,14336)?)?;
    // 0x3083c: ldr r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // 0x30840: mov r2, lr
    c.emit(arm32::mov_register(C::Al,R::R2,R::LR))?;
    // 0x30844: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30848: str r0, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x3084c: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x30850: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30854: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // 0x30858: ldr r3, [r4, #0x11c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,284)?)?;
    // 0x3085c: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,0)?)?;
    // 0x30860: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30864: ldr r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // 0x30868: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x3086c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30870: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x30874: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x30878: blo 0x308d0 <entry+0x12d0>  @ imm = #0x50
    c.b(C::Cc,0x308d0)?;
    // 0x3087c: add r0, r10, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,2)?)?;
    // 0x30880: mov r2, r12
    c.emit(arm32::mov_register(C::Al,R::R2,R::R12))?;
    // 0x30884: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // 0x30888: mov r5, lr
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // 0x3088c: str r0, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x30890: ldr r7, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,292)?)?;
    // 0x30894: str r7, [r6, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30898: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x3089c: str r0, [r7, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x308a0: ldr r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // 0x308a4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x308a8: str r0, [r5, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,R::R1,Shift::Lsl,0)?)?;
    // 0x308ac: mov r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,20)?)?;
    // 0x308b0: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x308b4: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x308b8: ldr r2, [r4, #0x124]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,292)?)?;
    // 0x308bc: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x308c0: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x308c4: ldr r0, [r4, #0x120]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,288)?)?;
    // 0x308c8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x308cc: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x308d0: mov r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,18)?)?;
    // 0x308d4: mov r2, r12
    c.emit(arm32::mov_register(C::Al,R::R2,R::R12))?;
    // 0x308d8: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // 0x308dc: mov r5, lr
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // 0x308e0: str r0, [r2, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::R1,Shift::Lsl,0)?)?;
    // 0x308e4: ldr r7, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,348)?)?;
    // 0x308e8: str r7, [r6, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(6)?,R::R1,Shift::Lsl,0)?)?;
    // 0x308ec: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x308f0: str r0, [r7, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::R1,Shift::Lsl,0)?)?;
    // 0x308f4: ldr r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,352)?)?;
    // 0x308f8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x308fc: str r0, [r5, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,R::R1,Shift::Lsl,0)?)?;
    // 0x30900: mov r0, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,18)?)?;
    // 0x30904: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30908: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x3090c: ldr r1, [r4, #0x15c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,348)?)?;
    // 0x30910: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,0)?)?;
    // 0x30914: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30918: ldr r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,352)?)?;
    // 0x3091c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30920: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x30924: mov r5, r12
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R12))?;
    // 0x30928: mov r12, r9
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(9)?))?;
    // 0x3092c: mov r9, lr
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::LR))?;
    // 0x30930: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x30934: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x30938: blo 0x3098c <entry+0x138c>  @ imm = #0x4c
    c.b(C::Cc,0x3098c)?;
    // 0x3093c: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // 0x30940: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x30944: mov r7, r8
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(8)?))?;
    // 0x30948: mov r3, r9
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(9)?))?;
    // 0x3094c: str r10, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x30950: ldr r2, [r4, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,356)?)?;
    // 0x30954: str r2, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x30958: mov r2, r12
    c.emit(arm32::mov_register(C::Al,R::R2,R::R12))?;
    // 0x3095c: str r10, [r2, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(10)?,R::R2,R::R0,Shift::Lsl,0)?)?;
    // 0x30960: ldr r6, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,352)?)?;
    // 0x30964: bic r6, r6, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(6)?,R::new(6)?,255)?)?;
    // 0x30968: str r6, [r3, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x3096c: add r0, r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(10)?,4096)?)?;
    // 0x30970: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30974: ldr r1, [r4, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,356)?)?;
    // 0x30978: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,0)?)?;
    // 0x3097c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30980: ldr r0, [r4, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,352)?)?;
    // 0x30984: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30988: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // 0x3098c: ldr r1, [r4, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,952)?)?;
    // 0x30990: mov r3, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,48)?)?;
    // 0x30994: mov r2, #768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,768)?)?;
    // 0x30998: ldr r0, [r4, #0x3c8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,968)?)?;
    // 0x3099c: and r3, r3, r1, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,4,false)?)?;
    // 0x309a0: and r7, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(7)?,R::R1,3)?)?;
    // 0x309a4: and r2, r2, r1, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R1,Shift::Lsl,8,false)?)?;
    // 0x309a8: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x309ac: orr r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x309b0: mov r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3072)?)?;
    // 0x309b4: and r3, r3, r1, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,10,false)?)?;
    // 0x309b8: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x309bc: mov r3, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12288)?)?;
    // 0x309c0: and r3, r3, r1, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,12,false)?)?;
    // 0x309c4: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x309c8: mov r3, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,196608)?)?;
    // 0x309cc: and r3, r3, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,16,false)?)?;
    // 0x309d0: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x309d4: mov r3, #3145728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3145728)?)?;
    // 0x309d8: and r3, r3, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R1,Shift::Lsl,20,false)?)?;
    // 0x309dc: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x309e0: mov r3, #12582912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,12582912)?)?;
    // 0x309e4: and r1, r3, r1, lsl #22
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,22,false)?)?;
    // 0x309e8: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x309ec: mov r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
    // 0x309f0: and r2, r2, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R0,Shift::Lsl,2,false)?)?;
    // 0x309f4: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x309f8: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // 0x309fc: and r2, r2, r0, lsl #6
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R0,Shift::Lsl,6,false)?)?;
    // 0x30a00: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x30a04: mov r2, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16384)?)?;
    // 0x30a08: and r2, r2, r0, lsl #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::R0,Shift::Lsl,14,false)?)?;
    // 0x30a0c: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x30a10: mov r2, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,262144)?)?;
    // 0x30a14: and r0, r2, r0, lsl #18
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R0,Shift::Lsl,18,false)?)?;
    // 0x30a18: orr r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x30a1c: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x30a20: orr r0, r0, #2480
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2480)?)?;
    // 0x30a24: orr r2, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,12288)?)?;
    // 0x30a28: mov r0, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3758096384)?)?;
    // 0x30a2c: str r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,8)?)?;
    // 0x30a30: str r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R2,R::R0,Shift::Lsl,0)?)?;
    // 0x30a34: ldr r1, [r4, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,952)?)?;
    // 0x30a38: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x30a3c: ldrne r1, [r4, #0x3c8]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,968)?)?;
    // 0x30a40: cmpne r1, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x30a44: bne 0x30c78 <entry+0x1678>  @ imm = #0x22c
    c.b(C::Ne,0x30c78)?;
    // 0x30a48: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x30a4c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x30a50: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x30a54: mov r6, r12
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // 0x30a58: str r2, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::R1,R::R0,Shift::Lsl,0)?)?;
    // 0x30a5c: ldr r7, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,412)?)?;
    // 0x30a60: str r7, [r3, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(7)?,R::new(3)?,R::R0,Shift::Lsl,0)?)?;
    // 0x30a64: str r2, [r6, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(6)?,R::R0,Shift::Lsl,0)?)?;
    // 0x30a68: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x30a6c: ldr r2, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,416)?)?;
    // 0x30a70: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30a74: str r2, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R0,Shift::Lsl,0)?)?;
    // 0x30a78: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x30a7c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30a80: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30a84: ldr r2, [r4, #0x19c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,412)?)?;
    // 0x30a88: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30a8c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30a90: ldr r0, [r4, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,416)?)?;
    // 0x30a94: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30a98: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30a9c: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x30aa0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30aa4: ldr r2, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,444)?)?;
    // 0x30aa8: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30aac: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30ab0: ldr r0, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,448)?)?;
    // 0x30ab4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30ab8: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30abc: mov r0, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,21)?)?;
    // 0x30ac0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30ac4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30ac8: ldr r2, [r4, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,444)?)?;
    // 0x30acc: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30ad0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30ad4: ldr r0, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,448)?)?;
    // 0x30ad8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30adc: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30ae0: mov r0, #69
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,69)?)?;
    // 0x30ae4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30ae8: ldr r2, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,508)?)?;
    // 0x30aec: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30af0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30af4: ldr r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,512)?)?;
    // 0x30af8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30afc: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30b00: mov r0, #69
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,69)?)?;
    // 0x30b04: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30b08: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30b0c: ldr r2, [r4, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,508)?)?;
    // 0x30b10: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30b14: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30b18: ldr r0, [r4, #0x200]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,512)?)?;
    // 0x30b1c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30b20: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30b24: mov r0, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,85)?)?;
    // 0x30b28: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30b2c: ldr r2, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,540)?)?;
    // 0x30b30: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30b34: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30b38: ldr r0, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,544)?)?;
    // 0x30b3c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30b40: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30b44: mov r0, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,85)?)?;
    // 0x30b48: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30b4c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30b50: ldr r2, [r4, #0x21c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,540)?)?;
    // 0x30b54: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30b58: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30b5c: ldr r0, [r4, #0x220]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,544)?)?;
    // 0x30b60: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30b64: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30b68: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x30b6c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30b70: ldr r2, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,604)?)?;
    // 0x30b74: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30b78: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30b7c: ldr r0, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,608)?)?;
    // 0x30b80: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30b84: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30b88: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x30b8c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30b90: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30b94: ldr r2, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,604)?)?;
    // 0x30b98: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30b9c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30ba0: ldr r0, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,608)?)?;
    // 0x30ba4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30ba8: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30bac: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x30bb0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30bb4: ldr r2, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,636)?)?;
    // 0x30bb8: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30bbc: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30bc0: ldr r0, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,640)?)?;
    // 0x30bc4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30bc8: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30bcc: mov r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,24)?)?;
    // 0x30bd0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30bd4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30bd8: ldr r2, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,636)?)?;
    // 0x30bdc: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30be0: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30be4: ldr r0, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,640)?)?;
    // 0x30be8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30bec: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30bf0: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x30bf4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30bf8: ldr r2, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,604)?)?;
    // 0x30bfc: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30c00: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30c04: ldr r0, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,608)?)?;
    // 0x30c08: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30c0c: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30c10: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x30c14: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30c18: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30c1c: ldr r2, [r4, #0x25c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,604)?)?;
    // 0x30c20: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30c24: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30c28: ldr r0, [r4, #0x260]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,608)?)?;
    // 0x30c2c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30c30: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30c34: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x30c38: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30c3c: ldr r2, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,636)?)?;
    // 0x30c40: str r2, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,0)?)?;
    // 0x30c44: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30c48: ldr r0, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,640)?)?;
    // 0x30c4c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30c50: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30c54: mov r0, #56
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,56)?)?;
    // 0x30c58: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30c5c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30c60: ldr r1, [r4, #0x27c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,636)?)?;
    // 0x30c64: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x30c68: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x30c6c: ldr r0, [r4, #0x280]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,640)?)?;
    // 0x30c70: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30c74: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30c78: mov r6, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3758096384)?)?;
    // 0x30c7c: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // 0x30c80: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x30c84: mov r3, r8
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
    // 0x30c88: mov r7, r9
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(9)?))?;
    // 0x30c8c: str r0, [r1, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x30c90: ldr r2, [r4, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,732)?)?;
    // 0x30c94: str r2, [r3, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(3)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x30c98: mov r2, r12
    c.emit(arm32::mov_register(C::Al,R::R2,R::R12))?;
    // 0x30c9c: str r0, [r2, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R2,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x30ca0: ldr r0, [r4, #0x2e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,736)?)?;
    // 0x30ca4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30ca8: str r0, [r7, r6]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(7)?,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x30cac: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // 0x30cb0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30cb4: ldr r6, [r4, #0x2ec]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,748)?)?;
    // 0x30cb8: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // 0x30cbc: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30cc0: ldr r0, [r4, #0x2f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,752)?)?;
    // 0x30cc4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30cc8: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30ccc: mov r0, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,72)?)?;
    // 0x30cd0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30cd4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30cd8: ldr r6, [r4, #0x2dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,732)?)?;
    // 0x30cdc: str r6, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(3)?,0)?)?;
    // 0x30ce0: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30ce4: ldr r0, [r4, #0x2e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,736)?)?;
    // 0x30ce8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30cec: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30cf0: mov r0, #88
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,88)?)?;
    // 0x30cf4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30cf8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x30cfc: ldr r1, [r4, #0x2ec]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,748)?)?;
    // 0x30d00: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x30d04: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x30d08: ldr r0, [r4, #0x2f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,752)?)?;
    // 0x30d0c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30d10: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x30d14: ldr r0, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
    // 0x30d18: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x30d1c: beq 0x30e74 <entry+0x1874>  @ imm = #0x150
    c.b(C::Eq,0x30e74)?;
    // 0x30d20: mov r7, #37
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,37)?)?;
    // 0x30d24: add r1, r4, #476
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,476)?)?;
    // 0x30d28: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x30d2c: str r12, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,36)?)?;
    // 0x30d30: str r8, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,32)?)?;
    // 0x30d34: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x30d38: sub r2, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(7)?,4096)?)?;
    // 0x30d3c: mov lr, r5
    c.emit(arm32::mov_register(C::Al,R::LR,R::new(5)?))?;
    // 0x30d40: mov r10, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,3758096384)?)?;
    // 0x30d44: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x30d48: mov r8, r5
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(5)?))?;
    // 0x30d4c: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x30d50: str r2, [lr, r10]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::LR,R::new(10)?,Shift::Lsl,0)?)?;
    // 0x30d54: ldr r6, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R1,0)?)?;
    // 0x30d58: str r6, [r0, r10]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::new(6)?,R::R0,R::new(10)?,Shift::Lsl,0)?)?;
    // 0x30d5c: mov r6, r12
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R12))?;
    // 0x30d60: mov r12, r9
    c.emit(arm32::mov_register(C::Al,R::R12,R::new(9)?))?;
    // 0x30d64: str r2, [r6, r10]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(6)?,R::new(10)?,Shift::Lsl,0)?)?;
    // 0x30d68: ldr r2, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,480)?)?;
    // 0x30d6c: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30d70: str r2, [r9, r10]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(9)?,R::new(10)?,Shift::Lsl,0)?)?;
    // 0x30d74: str r7, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::LR,0)?)?;
    // 0x30d78: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x30d7c: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
    // 0x30d80: str r7, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(6)?,0)?)?;
    // 0x30d84: ldr r2, [r4, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,480)?)?;
    // 0x30d88: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30d8c: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x30d90: sub r2, r7, #4080
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(7)?,4080)?)?;
    // 0x30d94: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x30d98: ldr r10, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R1,16)?)?;
    // 0x30d9c: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x30da0: ldr r10, [r11, #-0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::R11,-32)?)?;
    // 0x30da4: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x30da8: ldr r2, [r4, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,496)?)?;
    // 0x30dac: cmp r3, r10
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(10)?,Shift::Lsl,0,false)?)?;
    // 0x30db0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30db4: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x30db8: add r2, r7, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,16)?)?;
    // 0x30dbc: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x30dc0: ldr r5, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,16)?)?;
    // 0x30dc4: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x30dc8: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x30dcc: ldr r2, [r4, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,496)?)?;
    // 0x30dd0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30dd4: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x30dd8: sub r2, r7, #4032
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(7)?,4032)?)?;
    // 0x30ddc: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x30de0: ldr r5, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,96)?)?;
    // 0x30de4: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x30de8: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x30dec: ldr r2, [r4, #0x240]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,576)?)?;
    // 0x30df0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30df4: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x30df8: add r2, r7, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,64)?)?;
    // 0x30dfc: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x30e00: ldr r5, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,96)?)?;
    // 0x30e04: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x30e08: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x30e0c: ldr r2, [r4, #0x240]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,576)?)?;
    // 0x30e10: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30e14: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x30e18: sub r2, r7, #4016
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(7)?,4016)?)?;
    // 0x30e1c: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x30e20: ldr r5, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,112)?)?;
    // 0x30e24: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x30e28: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x30e2c: ldr r2, [r4, #0x250]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,592)?)?;
    // 0x30e30: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R2,255)?)?;
    // 0x30e34: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,0)?)?;
    // 0x30e38: add r2, r7, #80
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(7)?,80)?)?;
    // 0x30e3c: add r7, r7, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,65536)?)?;
    // 0x30e40: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::LR,0)?)?;
    // 0x30e44: ldr r5, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,112)?)?;
    // 0x30e48: add r1, r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,8)?)?;
    // 0x30e4c: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x30e50: str r2, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(6)?,0)?)?;
    // 0x30e54: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x30e58: ldr r0, [r4, #0x250]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,592)?)?;
    // 0x30e5c: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30e60: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x30e64: mov r9, r12
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R12))?;
    // 0x30e68: ldr r8, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,32)?)?;
    // 0x30e6c: ldr r12, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::SP,36)?)?;
    // 0x30e70: bne 0x30d38 <entry+0x1738>  @ imm = #-0x140
    c.b(C::Ne,0x30d38)?;
    // 0x30e74: mov r0, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // 0x30e78: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x30e7c: str r0, [r5, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(5)?,R::R2,Shift::Lsl,0)?)?;
    // 0x30e80: ldr r1, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,668)?)?;
    // 0x30e84: str r1, [r8, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(8)?,R::R2,Shift::Lsl,0)?)?;
    // 0x30e88: str r0, [r12, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::R12,R::R2,Shift::Lsl,0)?)?;
    // 0x30e8c: ldr r0, [r4, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,672)?)?;
    // 0x30e90: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30e94: str r0, [r9, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R0,R::new(9)?,R::R2,Shift::Lsl,0)?)?;
    // 0x30e98: mov r0, #104
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,104)?)?;
    // 0x30e9c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30ea0: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x30ea4: ldr r1, [r4, #0x29c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,668)?)?;
    // 0x30ea8: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x30eac: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x30eb0: ldr r0, [r4, #0x2a0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,672)?)?;
    // 0x30eb4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30eb8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x30ebc: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // 0x30ec0: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x30ec4: ldr r1, [r4, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,684)?)?;
    // 0x30ec8: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x30ecc: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x30ed0: ldr r0, [r4, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,688)?)?;
    // 0x30ed4: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30ed8: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x30edc: mov r0, #120
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,120)?)?;
    // 0x30ee0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x30ee4: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,0)?)?;
    // 0x30ee8: ldr r1, [r4, #0x2ac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,684)?)?;
    // 0x30eec: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x30ef0: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x30ef4: ldr r0, [r4, #0x2b0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,688)?)?;
    // 0x30ef8: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,255)?)?;
    // 0x30efc: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x30f00: ldr r7, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,28)?)?;
    // 0x30f04: ldr r1, [r4, #0x324]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,804)?)?;
    // 0x30f08: orr r0, r7, #724
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,724)?)?;
    // 0x30f0c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f10: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f14: orr r0, r7, #2784
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2784)?)?;
    // 0x30f18: ldr r1, [r4, #0x338]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,824)?)?;
    // 0x30f1c: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x30f20: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f24: orr r0, r7, #740
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,740)?)?;
    // 0x30f28: ldr r1, [r4, #0x318]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,792)?)?;
    // 0x30f2c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f30: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f34: orr r0, r7, #744
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,744)?)?;
    // 0x30f38: ldr r1, [r4, #0x310]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,784)?)?;
    // 0x30f3c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f40: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f44: orr r0, r7, #748
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,748)?)?;
    // 0x30f48: ldr r1, [r4, #0x308]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,776)?)?;
    // 0x30f4c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f50: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f54: orr r0, r7, #2800
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2800)?)?;
    // 0x30f58: ldr r1, [r4, #0x300]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,768)?)?;
    // 0x30f5c: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x30f60: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f64: orr r0, r7, #756
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,756)?)?;
    // 0x30f68: ldr r1, [r4, #0x340]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,832)?)?;
    // 0x30f6c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f70: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f74: orr r0, r7, #760
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,760)?)?;
    // 0x30f78: ldr r1, [r4, #0x348]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,840)?)?;
    // 0x30f7c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f80: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f84: orr r0, r7, #764
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,764)?)?;
    // 0x30f88: ldr r1, [r4, #0x358]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,856)?)?;
    // 0x30f8c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30f90: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30f94: orr r0, r7, #15104
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,15104)?)?;
    // 0x30f98: ldr r1, [r4, #0x360]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,864)?)?;
    // 0x30f9c: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30fa0: orr r0, r7, #772
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,772)?)?;
    // 0x30fa4: ldr r1, [r4, #0x370]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,880)?)?;
    // 0x30fa8: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30fac: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30fb0: orr r0, r7, #776
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,776)?)?;
    // 0x30fb4: ldr r1, [r4, #0x370]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,880)?)?;
    // 0x30fb8: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x30fbc: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x30fc0: orr r0, r7, #436
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,436)?)?;
    // 0x30fc4: orr r8, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(8)?,R::R0,14336)?)?;
    // 0x30fc8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x30fcc: str r0, [r8, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(8)?,R::R2,Shift::Lsl,0)?)?;
    // 0x30fd0: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,24)?)?;
    // 0x30fd4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x30fd8: beq 0x310c0 <entry+0x1ac0>  @ imm = #0xe0
    c.b(C::Eq,0x310c0)?;
    // 0x30fdc: ldr r9, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,4)?)?;
    // 0x30fe0: ldr r5, [r4, #0x3d8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,984)?)?;
    // 0x30fe4: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x30fe8: bl 0x311c0 <entry+0x1bc0>  @ imm = #0x1d0
    c.bl(0x311c0)?;
    // 0x30fec: subs r6, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::R0,R::new(5)?,Shift::Lsl,0,true)?)?;
    // 0x30ff0: beq 0x3109c <entry+0x1a9c>  @ imm = #0xa4
    c.b(C::Eq,0x3109c)?;
    // 0x30ff4: cmp r6, #199
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,199)?)?;
    // 0x30ff8: bhi 0x31024 <entry+0x1a24>  @ imm = #0x24
    c.b(C::Hi,0x31024)?;
    // 0x30ffc: ldr r1, [pc, #0x190]        @ 0x1b94 <entry+0x1b94>
    c.literal(R::R1,0x31194)?;
    // 0x31000: ldr r2, [pc, #0x190]        @ 0x1b98 <entry+0x1b98>
    c.literal(R::R2,0x31198)?;
    // 0x31004: mov r3, #29
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,29)?)?;
    // 0x31008: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3100c: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,0)?)?;
    // 0x31010: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x31014: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2388
    c.bl(0x2ec94)?;
    // 0x31018: ldr r1, [pc, #0x17c]        @ 0x1b9c <entry+0x1b9c>
    c.literal(R::R1,0x3119c)?;
    // 0x3101c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x31020: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2394
    c.bl(0x2ec94)?;
    // 0x31024: lsr r0, r6, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(6)?,Shift::Lsr,2)?)?;
    // 0x31028: orr r0, r5, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x3102c: add r5, r5, r6, lsr #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::new(5)?,R::new(6)?,Shift::Lsr,2,false)?)?;
    // 0x31030: str r0, [r4, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,700)?)?;
    // 0x31034: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x31038: bl 0x311c0 <entry+0x1bc0>  @ imm = #0x180
    c.bl(0x311c0)?;
    // 0x3103c: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x31040: sub r0, r6, r5
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::new(6)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x31044: orr r0, r5, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x31048: str r0, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,716)?)?;
    // 0x3104c: mov r0, r9
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(9)?))?;
    // 0x31050: bl 0x311c0 <entry+0x1bc0>  @ imm = #0x168
    c.bl(0x311c0)?;
    // 0x31054: ldr r2, [pc, #0x144]        @ 0x1ba0 <entry+0x1ba0>
    c.literal(R::R2,0x311a0)?;
    // 0x31058: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x3105c: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x31060: mov r1, #39
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,39)?)?;
    // 0x31064: movls r3, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x31068: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x3106c: mov r0, r3
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
    // 0x31070: bl 0x17b0 <entry+0xfffffffffffd21b0> @ imm = #-0x2f8c8
    c.bl(0x17b0)?;
    // 0x31074: ldr r1, [r4, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,700)?)?;
    // 0x31078: orr r0, r7, #684
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,684)?)?;
    // 0x3107c: mov r2, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3758096384)?)?;
    // 0x31080: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x31084: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x31088: orr r0, r7, #2736
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2736)?)?;
    // 0x3108c: ldr r1, [r4, #0x2cc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,716)?)?;
    // 0x31090: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x31094: str r1, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::R0,R::R2,Shift::Lsl,0)?)?;
    // 0x31098: b 0x310c0 <entry+0x1ac0>  @ imm = #0x20
    c.b(C::Al,0x310c0)?;
    // 0x3109c: ldr r1, [pc, #0x100]        @ 0x1ba4 <entry+0x1ba4>
    c.literal(R::R1,0x311a4)?;
    // 0x310a0: ldr r2, [pc, #0xf0]         @ 0x1b98 <entry+0x1b98>
    c.literal(R::R2,0x31198)?;
    // 0x310a4: mov r3, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,46)?)?;
    // 0x310a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x310ac: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x310b0: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2424
    c.bl(0x2ec94)?;
    // 0x310b4: ldr r1, [pc, #0xe0]         @ 0x1b9c <entry+0x1b9c>
    c.literal(R::R1,0x3119c)?;
    // 0x310b8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x310bc: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2430
    c.bl(0x2ec94)?;
    // 0x310c0: orr r0, r7, #2464
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::new(7)?,2464)?)?;
    // 0x310c4: mov r5, #-536870912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3758096384)?)?;
    // 0x310c8: ldr r1, [pc, #0xd8]         @ 0x1ba8 <entry+0x1ba8>
    c.literal(R::R1,0x311a8)?;
    // 0x310cc: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x310d0: ldr r2, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x310d4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x310d8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x244c
    c.bl(0x2ec94)?;
    // 0x310dc: ldr r4, [pc, #0xb8]         @ 0x1b9c <entry+0x1b9c>
    c.literal(R::new(4)?,0x3119c)?;
    // 0x310e0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x310e4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x310e8: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x245c
    c.bl(0x2ec94)?;
    // 0x310ec: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x310f0: ldr r1, [pc, #0xb4]         @ 0x1bac <entry+0x1bac>
    c.literal(R::R1,0x311ac)?;
    // 0x310f4: ldr r2, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x310f8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x310fc: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2470
    c.bl(0x2ec94)?;
    // 0x31100: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31104: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x31108: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x247c
    c.bl(0x2ec94)?;
    // 0x3110c: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x31110: ldr r1, [pc, #0x98]         @ 0x1bb0 <entry+0x1bb0>
    c.literal(R::R1,0x311b0)?;
    // 0x31114: ldr r2, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x31118: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3111c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2490
    c.bl(0x2ec94)?;
    // 0x31120: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31124: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x31128: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x249c
    c.bl(0x2ec94)?;
    // 0x3112c: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x31130: ldr r1, [pc, #0x7c]         @ 0x1bb4 <entry+0x1bb4>
    c.literal(R::R1,0x311b4)?;
    // 0x31134: ldr r2, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x31138: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3113c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x24b0
    c.bl(0x2ec94)?;
    // 0x31140: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31144: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x31148: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x24bc
    c.bl(0x2ec94)?;
    // 0x3114c: ldr r2, [r8, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::new(8)?,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x31150: ldr r1, [pc, #0x60]         @ 0x1bb8 <entry+0x1bb8>
    c.literal(R::R1,0x311b8)?;
    // 0x31154: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31158: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x24cc
    c.bl(0x2ec94)?;
    // 0x3115c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31160: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x31164: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x24d8
    c.bl(0x2ec94)?;
    // 0x31168: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x3116c: ldr r1, [pc, #0x48]         @ 0x1bbc <entry+0x1bbc>
    c.literal(R::R1,0x311bc)?;
    // 0x31170: ldr r2, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R2,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x31174: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31178: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x24ec
    c.bl(0x2ec94)?;
    // 0x3117c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x31180: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x31184: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x31188: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3118c: b 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x2500
    c.b(C::Al,0x2ec94)?;
    c.check(0x31190)?;
    const SERIALIZER_LAYOUT_AND_LOG_WORDS:[u32;12]=[0x00103070,0x000eb05b,0x000eb0a0,0x000e2788,0x000eafbb,0x000eb0ab,0x000eb0e0,0x000eb0fc,0x000eb117,0x000eb132,0x000eb14d,0x000eb16c];
    for word in SERIALIZER_LAYOUT_AND_LOG_WORDS {c.emit(word)?;}
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;serialize_controller_registers(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_register_serializer_retains_layout_and_diagnostics(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let tail=&c.bytes[c.bytes.len()-48..];
        let get=|i:usize|u32::from_le_bytes(tail[4*i..4*i+4].try_into().unwrap());
        assert_eq!(get(0),0x00103070);
        assert_eq!(get(1),0x000eb05b);
        assert_eq!(get(3),0x000e2788);
        assert_eq!(get(11),0x000eb16c);
    }
}
