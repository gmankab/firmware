// SPDX-License-Identifier: GPL-3.0-only
//! Codec register-and-command table initialization, including conditional
//! size validation, signed halfword multiplication and register block stores.
//! ARM32 instructions are emitted through typed public encoders. The ten
//! trailing words are device data after the function's final POP {..,pc}.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2d3f4;
const END:usize=0x2ddfc;
fn initialize_command_register_tables(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2d3f4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2d3f8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2d3fc: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x2d400: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2d404: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2d408: ldr r1, [pc, #0x9c8]        @ 0x21d8 <entry+0x21d8>
    c.literal(R::R1,0x2ddd8)?;
    // 0x2d40c: ldr r8, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,0)?)?;
    // 0x2d410: ldr r0, [pc, #0x9bc]        @ 0x21d4 <entry+0x21d4>
    c.literal(R::R0,0x2ddd4)?;
    // 0x2d414: ldr r6, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R0,0)?)?;
    // 0x2d418: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d41c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x2d420: bl 0x2ec94 <entry+0x3094>  @ imm = #0x186c
    c.bl(0x2ec94)?;
    // 0x2d424: ldr r1, [pc, #0x9b0]        @ 0x21dc <entry+0x21dc>
    c.literal(R::R1,0x2dddc)?;
    // 0x2d428: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d42c: bl 0x2ec94 <entry+0x3094>  @ imm = #0x1860
    c.bl(0x2ec94)?;
    // 0x2d430: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2d434: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x2d438: mov r7, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,24)?)?;
    // 0x2d43c: str r4, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,32)?)?;
    // 0x2d440: and r4, r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(4)?,R::new(6)?,2)?)?;
    // 0x2d444: and r5, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::new(6)?,1)?)?;
    // 0x2d448: mov r9, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,8192)?)?;
    // 0x2d44c: and r6, r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(6)?,R::new(6)?,8192)?)?;
    // 0x2d450: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d454: movne r7, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2d458: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2d45c: bl 0x2f8ac <entry+0x3cac>  @ imm = #0x2448
    c.bl(0x2f8ac)?;
    // 0x2d460: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d464: beq 0x2d4a4 <entry+0x18a4>  @ imm = #0x38
    c.b(C::Eq,0x2d4a4)?;
    // 0x2d468: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2d46c: mov r2, #5760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5760)?)?;
    // 0x2d470: mov r0, #180
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,180)?)?;
    // 0x2d474: movne r2, #4096
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,4096)?)?;
    // 0x2d478: movne r0, #144
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,144)?)?;
    // 0x2d47c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x2d480: lsr r1, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R2,Shift::Lsr,4)?)?;
    // 0x2d484: smulbb r0, r1, r0
    c.emit(super::encode_smulbb(C::Al,R::R0,R::R1,R::R0)?)?;
    // 0x2d488: ldr r1, [pc, #0x950]        @ 0x21e0 <entry+0x21e0>
    c.literal(R::R1,0x2dde0)?;
    // 0x2d48c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x2d490: mov r1, #10688
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10688)?)?;
    // 0x2d494: rsb r0, r0, r0, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R0,R::R0,R::R0,Shift::Lsl,4,false)?)?;
    // 0x2d498: orr r1, r1, #1933312
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1933312)?)?;
    // 0x2d49c: lslne r1, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::R1,R::R0,Shift::Lsl,1)?)?;
    // 0x2d4a0: b 0x2d4bc <entry+0x18bc>  @ imm = #0x14
    c.b(C::Al,0x2d4bc)?;
    // 0x2d4a4: ldr r0, [pc, #0x934]        @ 0x21e0 <entry+0x21e0>
    c.literal(R::R0,0x2dde0)?;
    // 0x2d4a8: mov r1, #138240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,138240)?)?;
    // 0x2d4ac: mov r2, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,8192)?)?;
    // 0x2d4b0: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2d4b4: mov r1, #4992
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4992)?)?;
    // 0x2d4b8: orr r1, r1, #3883008
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,3883008)?)?;
    // 0x2d4bc: eor r0, r9, r4, lsl #12
    c.emit(arm32::data_register(C::Al,Op::Eor,R::R0,R::new(9)?,R::new(4)?,Shift::Lsl,12,false)?)?;
    // 0x2d4c0: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2d4c4: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::SP,20)?)?;
    // 0x2d4c8: str r4, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,8)?)?;
    // 0x2d4cc: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2d4d0: beq 0x2d4ec <entry+0x18ec>  @ imm = #0x14
    c.b(C::Eq,0x2d4ec)?;
    // 0x2d4d4: ldr r0, [pc, #0x904]        @ 0x21e0 <entry+0x21e0>
    c.literal(R::R0,0x2dde0)?;
    // 0x2d4d8: mov r1, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,166)?)?;
    // 0x2d4dc: mov r2, #3808
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3808)?)?;
    // 0x2d4e0: orr r1, r1, #31488
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,31488)?)?;
    // 0x2d4e4: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2d4e8: ldr r1, [pc, #0x8f4]        @ 0x21e4 <entry+0x21e4>
    c.literal(R::R1,0x2dde4)?;
    // 0x2d4ec: mov r9, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,14)?)?;
    // 0x2d4f0: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x2d4f4: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,16)?)?;
    // 0x2d4f8: mov r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // 0x2d4fc: mov r5, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
    // 0x2d500: mov r1, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,64)?)?;
    // 0x2d504: mov r6, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,8192)?)?;
    // 0x2d508: mov r4, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,16)?)?;
    // 0x2d50c: mov lr, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,128)?)?;
    // 0x2d510: mov r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,32)?)?;
    // 0x2d514: orr r9, r9, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(9)?,R::new(9)?,4096)?)?;
    // 0x2d518: orr r12, r12, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,32768)?)?;
    // 0x2d51c: str r9, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,0)?)?;
    // 0x2d520: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x2d524: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x2d528: orr r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2d52c: mov r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4)?)?;
    // 0x2d530: orr r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x2d534: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x2d538: orr r2, r0, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,20480)?)?;
    // 0x2d53c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2d540: str r2, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,4)?)?;
    // 0x2d544: str r5, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,8)?)?;
    // 0x2d548: str r7, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,12)?)?;
    // 0x2d54c: str r5, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,16)?)?;
    // 0x2d550: str r12, [r8, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(8)?,24)?)?;
    // 0x2d554: str r0, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,20)?)?;
    // 0x2d558: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2d55c: str r1, [r8, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,32)?)?;
    // 0x2d560: str r6, [r8, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,36)?)?;
    // 0x2d564: str r4, [r8, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,40)?)?;
    // 0x2d568: str r0, [r8, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,44)?)?;
    // 0x2d56c: str lr, [r8, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,48)?)?;
    // 0x2d570: add lr, r8, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(8)?,60)?)?;
    // 0x2d574: str r6, [r8, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,52)?)?;
    // 0x2d578: str r3, [r8, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,56)?)?;
    // 0x2d57c: stm lr, {r0, r1, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::LR,R::R0.bit()|R::R1.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d580: str r4, [r8, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,72)?)?;
    // 0x2d584: mov r4, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,128)?)?;
    // 0x2d588: add lr, r8, #76
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(8)?,76)?)?;
    // 0x2d58c: stm lr, {r0, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::LR,R::R0.bit()|R::new(4)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d590: mov lr, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,4)?)?;
    // 0x2d594: str r3, [r8, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,88)?)?;
    // 0x2d598: str r0, [r8, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,92)?)?;
    // 0x2d59c: str lr, [r8, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,28)?)?;
    // 0x2d5a0: str r9, [r8, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,96)?)?;
    // 0x2d5a4: str r2, [r8, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,100)?)?;
    // 0x2d5a8: mov r2, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
    // 0x2d5ac: str r5, [r8, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,104)?)?;
    // 0x2d5b0: str r7, [r8, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,108)?)?;
    // 0x2d5b4: mov r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,16)?)?;
    // 0x2d5b8: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2d5bc: mov r9, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,128)?)?;
    // 0x2d5c0: str r2, [r8, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,112)?)?;
    // 0x2d5c4: add r2, r8, #140
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,140)?)?;
    // 0x2d5c8: str r12, [r8, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(8)?,120)?)?;
    // 0x2d5cc: str r1, [r8, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,128)?)?;
    // 0x2d5d0: str r6, [r8, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,132)?)?;
    // 0x2d5d4: str r5, [r8, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,136)?)?;
    // 0x2d5d8: stm r2, {r0, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(4)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d5dc: add r2, r8, #156
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(8)?,156)?)?;
    // 0x2d5e0: str r3, [r8, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,152)?)?;
    // 0x2d5e4: stm r2, {r0, r1, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::R1.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d5e8: add r1, r8, #172
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,172)?)?;
    // 0x2d5ec: str r5, [r8, #0xa8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,168)?)?;
    // 0x2d5f0: stm r1, {r0, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::new(4)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d5f4: str r3, [r8, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,184)?)?;
    // 0x2d5f8: str r0, [r8, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,188)?)?;
    // 0x2d5fc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d600: str lr, [r8, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,124)?)?;
    // 0x2d604: str r7, [r8, #0xc0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,192)?)?;
    // 0x2d608: str r0, [r8, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,196)?)?;
    // 0x2d60c: str r7, [r8, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,116)?)?;
    // 0x2d610: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x2d614: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d618: movne r9, #256
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(9)?,R::R0,256)?)?;
    // 0x2d61c: bl 0x2f8d0 <entry+0x22d0>  @ imm = #0x22ac
    c.bl(0x2f8d0)?;
    // 0x2d620: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d624: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x2d628: mov r10, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,256)?)?;
    // 0x2d62c: movne r9, #512
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(9)?,R::R0,512)?)?;
    // 0x2d630: str r9, [r8, #0xc8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,200)?)?;
    // 0x2d634: str r6, [r8, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,204)?)?;
    // 0x2d638: str r1, [r8, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,208)?)?;
    // 0x2d63c: str r10, [r8, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,212)?)?;
    // 0x2d640: ldr r0, [r8, #0xc4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,196)?)?;
    // 0x2d644: str r9, [r8, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,216)?)?;
    // 0x2d648: str r6, [r8, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,220)?)?;
    // 0x2d64c: str r5, [r8, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,224)?)?;
    // 0x2d650: str r10, [r8, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,228)?)?;
    // 0x2d654: adds r0, r0, #2
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R0,2,true)?)?;
    // 0x2d658: str r0, [r8, #0xc4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,196)?)?;
    // 0x2d65c: bne 0x2d674 <entry+0x74>    @ imm = #0x10
    c.b(C::Ne,0x2d674)?;
    // 0x2d660: ldr r2, [pc, #0x780]        @ 0x7e8 <entry+0x7e8>
    c.literal(R::R2,0x2dde8)?;
    // 0x2d664: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d668: mov r1, #640
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,640)?)?;
    // 0x2d66c: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2bec4
    c.bl(0x17b0)?;
    // 0x2d670: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x2d674: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2d678: str r0, [r8, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,232)?)?;
    // 0x2d67c: ldr r0, [r8, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,116)?)?;
    // 0x2d680: adds r0, r0, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R0,1,true)?)?;
    // 0x2d684: str r0, [r8, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,116)?)?;
    // 0x2d688: str r4, [r8, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,240)?)?;
    // 0x2d68c: str r6, [r8, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,244)?)?;
    // 0x2d690: str r1, [r8, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,248)?)?;
    // 0x2d694: str r10, [r8, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,252)?)?;
    // 0x2d698: str r4, [r8, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,256)?)?;
    // 0x2d69c: str r6, [r8, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,260)?)?;
    // 0x2d6a0: str r5, [r8, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,264)?)?;
    // 0x2d6a4: str r10, [r8, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,268)?)?;
    // 0x2d6a8: str r7, [r8, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,236)?)?;
    // 0x2d6ac: bne 0x2d6c0 <entry+0xc0>    @ imm = #0xc
    c.b(C::Ne,0x2d6c0)?;
    // 0x2d6b0: ldr r2, [pc, #0x730]        @ 0x7e8 <entry+0x7e8>
    c.literal(R::R2,0x2dde8)?;
    // 0x2d6b4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d6b8: mov r1, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,648)?)?;
    // 0x2d6bc: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2bf14
    c.bl(0x17b0)?;
    // 0x2d6c0: mov lr, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,14)?)?;
    // 0x2d6c4: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x2d6c8: mov r12, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,2)?)?;
    // 0x2d6cc: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x2d6d0: mov r6, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,64)?)?;
    // 0x2d6d4: mov r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,8192)?)?;
    // 0x2d6d8: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x2d6dc: mov r4, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,256)?)?;
    // 0x2d6e0: mov r9, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,4)?)?;
    // 0x2d6e4: orr lr, lr, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,4096)?)?;
    // 0x2d6e8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
    // 0x2d6ec: orr r12, r12, #49152
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,49152)?)?;
    // 0x2d6f0: str lr, [r8, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,272)?)?;
    // 0x2d6f4: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x2d6f8: mov r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // 0x2d6fc: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x2d700: orr r5, r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::R0,16384)?)?;
    // 0x2d704: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2d708: str r5, [r8, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,276)?)?;
    // 0x2d70c: str r0, [r8, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,280)?)?;
    // 0x2d710: str r7, [r8, #0x11c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,284)?)?;
    // 0x2d714: str r0, [r8, #0x120]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,288)?)?;
    // 0x2d718: str r12, [r8, #0x128]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(8)?,296)?)?;
    // 0x2d71c: str r10, [r8, #0x124]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,292)?)?;
    // 0x2d720: str r6, [r8, #0x130]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,304)?)?;
    // 0x2d724: str r3, [r8, #0x134]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,308)?)?;
    // 0x2d728: str r2, [r8, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,312)?)?;
    // 0x2d72c: str r1, [r8, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,316)?)?;
    // 0x2d730: str r4, [r8, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,320)?)?;
    // 0x2d734: str r3, [r8, #0x144]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,324)?)?;
    // 0x2d738: str r2, [r8, #0x148]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,328)?)?;
    // 0x2d73c: str r1, [r8, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,332)?)?;
    // 0x2d740: str r6, [r8, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,336)?)?;
    // 0x2d744: str r3, [r8, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,340)?)?;
    // 0x2d748: str r2, [r8, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,344)?)?;
    // 0x2d74c: str r1, [r8, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,348)?)?;
    // 0x2d750: str r4, [r8, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,352)?)?;
    // 0x2d754: str r3, [r8, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,356)?)?;
    // 0x2d758: str r2, [r8, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,360)?)?;
    // 0x2d75c: str r1, [r8, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,364)?)?;
    // 0x2d760: str r9, [r8, #0x12c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,300)?)?;
    // 0x2d764: str lr, [r8, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,368)?)?;
    // 0x2d768: str r5, [r8, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,372)?)?;
    // 0x2d76c: mov r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3)?)?;
    // 0x2d770: str r0, [r8, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,376)?)?;
    // 0x2d774: str r7, [r8, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,380)?)?;
    // 0x2d778: mov r7, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,32)?)?;
    // 0x2d77c: str r5, [r8, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,384)?)?;
    // 0x2d780: mov r5, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3)?)?;
    // 0x2d784: orr r5, r5, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,16384)?)?;
    // 0x2d788: str r5, [r8, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,392)?)?;
    // 0x2d78c: mov r5, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,128)?)?;
    // 0x2d790: str r5, [r8, #0x190]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,400)?)?;
    // 0x2d794: str r3, [r8, #0x194]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,404)?)?;
    // 0x2d798: str r7, [r8, #0x198]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(8)?,408)?)?;
    // 0x2d79c: str r4, [r8, #0x19c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,412)?)?;
    // 0x2d7a0: str r5, [r8, #0x1a0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(8)?,416)?)?;
    // 0x2d7a4: add r5, r8, #424
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,424)?)?;
    // 0x2d7a8: str r3, [r8, #0x1a4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,420)?)?;
    // 0x2d7ac: str r0, [r8, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,396)?)?;
    // 0x2d7b0: stm r5, {r2, r4, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R2.bit()|R::new(4)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d7b4: str r0, [r8, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,388)?)?;
    // 0x2d7b8: str r6, [r8, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,440)?)?;
    // 0x2d7bc: str r3, [r8, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,444)?)?;
    // 0x2d7c0: str r2, [r8, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,448)?)?;
    // 0x2d7c4: str r1, [r8, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,452)?)?;
    // 0x2d7c8: str r4, [r8, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,456)?)?;
    // 0x2d7cc: str r3, [r8, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,460)?)?;
    // 0x2d7d0: str r2, [r8, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,464)?)?;
    // 0x2d7d4: str r1, [r8, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,468)?)?;
    // 0x2d7d8: str r6, [r8, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(8)?,472)?)?;
    // 0x2d7dc: str r3, [r8, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,476)?)?;
    // 0x2d7e0: str r2, [r8, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,480)?)?;
    // 0x2d7e4: str r1, [r8, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,484)?)?;
    // 0x2d7e8: str r4, [r8, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(8)?,488)?)?;
    // 0x2d7ec: str r3, [r8, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(8)?,492)?)?;
    // 0x2d7f0: str r2, [r8, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,496)?)?;
    // 0x2d7f4: str r1, [r8, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,500)?)?;
    // 0x2d7f8: str r9, [r8, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,436)?)?;
    // 0x2d7fc: ldr r3, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,32)?)?;
    // 0x2d800: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x2d804: beq 0x2d814 <entry+0x214>   @ imm = #0x8
    c.b(C::Eq,0x2d814)?;
    // 0x2d808: add r5, r8, #504
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,504)?)?;
    // 0x2d80c: mov r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,12)?)?;
    // 0x2d810: b 0x2d848 <entry+0x248>   @ imm = #0x30
    c.b(C::Al,0x2d848)?;
    // 0x2d814: str lr, [r8, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(8)?,504)?)?;
    // 0x2d818: str r2, [r8, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,508)?)?;
    // 0x2d81c: mov r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
    // 0x2d820: add r5, r8, #540
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(8)?,540)?)?;
    // 0x2d824: mov r7, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,14)?)?;
    // 0x2d828: str r0, [r8, #0x200]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,512)?)?;
    // 0x2d82c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,4096)?)?;
    // 0x2d830: str r2, [r8, #0x204]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,516)?)?;
    // 0x2d834: str r10, [r8, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(8)?,524)?)?;
    // 0x2d838: str r9, [r8, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,528)?)?;
    // 0x2d83c: str r0, [r8, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,532)?)?;
    // 0x2d840: str r9, [r8, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(8)?,536)?)?;
    // 0x2d844: str r0, [r8, #0x208]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,520)?)?;
    // 0x2d848: mov r6, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,6)?)?;
    // 0x2d84c: mov r8, lr
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::LR))?;
    // 0x2d850: str r8, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,0)?)?;
    // 0x2d854: str r1, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,4)?)?;
    // 0x2d858: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x2d85c: mov r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
    // 0x2d860: str r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    // 0x2d864: str r6, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,12)?)?;
    // 0x2d868: str r10, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,20)?)?;
    // 0x2d86c: str r10, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,24)?)?;
    // 0x2d870: str r10, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,28)?)?;
    // 0x2d874: str r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,32)?)?;
    // 0x2d878: str r10, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,36)?)?;
    // 0x2d87c: str r9, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,40)?)?;
    // 0x2d880: str r10, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,44)?)?;
    // 0x2d884: str r1, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,48)?)?;
    // 0x2d888: mov r1, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16384)?)?;
    // 0x2d88c: str r9, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,16)?)?;
    // 0x2d890: str r8, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,52)?)?;
    // 0x2d894: str r1, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,56)?)?;
    // 0x2d898: str r0, [r5, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,60)?)?;
    // 0x2d89c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d8a0: str r6, [r5, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,64)?)?;
    // 0x2d8a4: str r0, [r5, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,68)?)?;
    // 0x2d8a8: bl 0x2f8ac <entry+0x22ac>  @ imm = #0x1ffc
    c.bl(0x2f8ac)?;
    // 0x2d8ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d8b0: str r10, [r5, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,72)?)?;
    // 0x2d8b4: moveq r4, #2048
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(4)?,R::R0,2048)?)?;
    // 0x2d8b8: str r4, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,76)?)?;
    // 0x2d8bc: ldr r0, [r5, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,68)?)?;
    // 0x2d8c0: str r9, [r5, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,80)?)?;
    // 0x2d8c4: str r4, [r5, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,84)?)?;
    // 0x2d8c8: adds r0, r0, #2
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R0,2,true)?)?;
    // 0x2d8cc: str r0, [r5, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,68)?)?;
    // 0x2d8d0: bne 0x2d8e8 <entry+0x2e8>   @ imm = #0x10
    c.b(C::Ne,0x2d8e8)?;
    // 0x2d8d4: ldr r2, [pc, #0x50c]        @ 0x7e8 <entry+0x7e8>
    c.literal(R::R2,0x2dde8)?;
    // 0x2d8d8: mov r1, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,254)?)?;
    // 0x2d8dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d8e0: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x2d8e4: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2c13c
    c.bl(0x17b0)?;
    // 0x2d8e8: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2d8ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d8f0: bne 0x2d96c <entry+0x36c>   @ imm = #0x74
    c.b(C::Ne,0x2d96c)?;
    // 0x2d8f4: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x2d8f8: mov r4, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,2)?)?;
    // 0x2d8fc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2d900: add r2, r5, #92
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,92)?)?;
    // 0x2d904: str r8, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,88)?)?;
    // 0x2d908: stm r2, {r0, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(4)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2d90c: str r1, [r5, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,104)?)?;
    // 0x2d910: bl 0x2f8ac <entry+0x22ac>  @ imm = #0x1f94
    c.bl(0x2f8ac)?;
    // 0x2d914: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2d918: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d91c: str r1, [r5, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,108)?)?;
    // 0x2d920: mov r1, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,128)?)?;
    // 0x2d924: orr r1, r1, #536870912
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,536870912)?)?;
    // 0x2d928: addeq r1, r1, #896
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R1,R::R1,896)?)?;
    // 0x2d92c: str r1, [r5, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,112)?)?;
    // 0x2d930: ldr r0, [r5, #0x68]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,104)?)?;
    // 0x2d934: str r4, [r5, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,116)?)?;
    // 0x2d938: str r1, [r5, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,120)?)?;
    // 0x2d93c: adds r0, r0, #2
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R0,2,true)?)?;
    // 0x2d940: str r0, [r5, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,104)?)?;
    // 0x2d944: bne 0x2d95c <entry+0x35c>   @ imm = #0x10
    c.b(C::Ne,0x2d95c)?;
    // 0x2d948: ldr r2, [pc, #0x498]        @ 0x7e8 <entry+0x7e8>
    c.literal(R::R2,0x2dde8)?;
    // 0x2d94c: mov r1, #25
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,25)?)?;
    // 0x2d950: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d954: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x2d958: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2c1b0
    c.bl(0x17b0)?;
    // 0x2d95c: add r5, r5, #124
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,124)?)?;
    // 0x2d960: add r7, r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,2)?)?;
    // 0x2d964: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x2d968: b 0x2d974 <entry+0x374>   @ imm = #0x4
    c.b(C::Al,0x2d974)?;
    // 0x2d96c: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,8)?)?;
    // 0x2d970: add r5, r5, #88
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,88)?)?;
    // 0x2d974: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2d978: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d97c: str r8, [r5]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,0)?)?;
    // 0x2d980: str r7, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,4)?)?;
    // 0x2d984: str r7, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,8)?)?;
    // 0x2d988: str r6, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,12)?)?;
    // 0x2d98c: str r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // 0x2d990: bl 0x2f8ac <entry+0x22ac>  @ imm = #0x1f14
    c.bl(0x2f8ac)?;
    // 0x2d994: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x2d998: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2d99c: mov r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,64)?)?;
    // 0x2d9a0: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x2d9a4: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2d9a8: str r1, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,20)?)?;
    // 0x2d9ac: mov r1, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32768)?)?;
    // 0x2d9b0: moveq r1, #131072
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,131072)?)?;
    // 0x2d9b4: str r1, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,24)?)?;
    // 0x2d9b8: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,16)?)?;
    // 0x2d9bc: str r2, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,28)?)?;
    // 0x2d9c0: str r1, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,32)?)?;
    // 0x2d9c4: str r7, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,36)?)?;
    // 0x2d9c8: str r1, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,40)?)?;
    // 0x2d9cc: str r6, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,44)?)?;
    // 0x2d9d0: str r1, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,48)?)?;
    // 0x2d9d4: str r4, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,52)?)?;
    // 0x2d9d8: str r1, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,56)?)?;
    // 0x2d9dc: adds r0, r0, #5
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R0,5,true)?)?;
    // 0x2d9e0: str r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,16)?)?;
    // 0x2d9e4: bne 0x2d9fc <entry+0x3fc>   @ imm = #0x10
    c.b(C::Ne,0x2d9fc)?;
    // 0x2d9e8: ldr r2, [pc, #0x3f8]        @ 0x7e8 <entry+0x7e8>
    c.literal(R::R2,0x2dde8)?;
    // 0x2d9ec: mov r1, #57
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,57)?)?;
    // 0x2d9f0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2d9f4: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x2d9f8: bl 0x17b0 <entry+0xfffffffffffd41b0> @ imm = #-0x2c250
    c.bl(0x17b0)?;
    // 0x2d9fc: str r8, [r5, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,60)?)?;
    // 0x2da00: mov r12, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,7)?)?;
    // 0x2da04: mov r1, #480
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,480)?)?;
    // 0x2da08: add r3, r5, #104
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,104)?)?;
    // 0x2da0c: add lr, r5, #192
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(5)?,192)?)?;
    // 0x2da10: mov r10, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,44)?)?;
    // 0x2da14: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x2da18: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R12,R::R12,4096)?)?;
    // 0x2da1c: orr r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,2)?)?;
    // 0x2da20: orr r2, r0, #16384
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R0,16384)?)?;
    // 0x2da24: str r2, [r5, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,64)?)?;
    // 0x2da28: str r7, [r5, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,68)?)?;
    // 0x2da2c: str r12, [r5, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,72)?)?;
    // 0x2da30: str r6, [r5, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,80)?)?;
    // 0x2da34: str r4, [r5, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,84)?)?;
    // 0x2da38: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2da3c: str r0, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,88)?)?;
    // 0x2da40: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x2da44: str r4, [r5, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,92)?)?;
    // 0x2da48: str r0, [r5, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,96)?)?;
    // 0x2da4c: str r4, [r5, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,100)?)?;
    // 0x2da50: str r7, [r5, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,76)?)?;
    // 0x2da54: stm r3, {r1, r4, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R1.bit()|R::new(4)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2da58: mov r1, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4096)?)?;
    // 0x2da5c: add r3, r5, #116
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(5)?,116)?)?;
    // 0x2da60: stm r3, {r1, r7, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R1.bit()|R::new(7)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2da64: str r6, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,132)?)?;
    // 0x2da68: str r4, [r5, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,136)?)?;
    // 0x2da6c: mov r3, #14464
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14464)?)?;
    // 0x2da70: mov r6, #53760
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,53760)?)?;
    // 0x2da74: ldr r9, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,20)?)?;
    // 0x2da78: orr r3, r3, #475136
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,475136)?)?;
    // 0x2da7c: orr r6, r6, #983040
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,983040)?)?;
    // 0x2da80: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x2da84: movne r6, r3
    c.emit(arm32::mov_register(C::Ne,R::new(6)?,R::new(3)?))?;
    // 0x2da88: mov r3, #240
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,240)?)?;
    // 0x2da8c: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x2da90: str r6, [r5, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,140)?)?;
    // 0x2da94: str r4, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,144)?)?;
    // 0x2da98: str r0, [r5, #0x94]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,148)?)?;
    // 0x2da9c: movne r3, #120
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,120)?)?;
    // 0x2daa0: add r0, r5, #156
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,156)?)?;
    // 0x2daa4: str r4, [r5, #0x98]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,152)?)?;
    // 0x2daa8: str r7, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,128)?)?;
    // 0x2daac: mov r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,7)?)?;
    // 0x2dab0: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x2dab4: stm r0, {r3, r4, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::new(3)?.bit()|R::new(4)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dab8: add r0, r5, #168
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,168)?)?;
    // 0x2dabc: mov r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x2dac0: stm r0, {r2, r7, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(7)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dac4: mov r2, #96
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,96)?)?;
    // 0x2dac8: str r4, [r5, #0xb8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,184)?)?;
    // 0x2dacc: str r2, [r5, #0xbc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,188)?)?;
    // 0x2dad0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,16)?)?;
    // 0x2dad4: stm lr, {r0, r4, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::LR,R::R0.bit()|R::new(4)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dad8: str r2, [r5, #0xcc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,204)?)?;
    // 0x2dadc: str r0, [r5, #0xd0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,208)?)?;
    // 0x2dae0: str r4, [r5, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,212)?)?;
    // 0x2dae4: str r3, [r5, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,216)?)?;
    // 0x2dae8: ldr r3, [pc, #0x2f0]        @ 0x7e0 <entry+0x7e0>
    c.literal(R::new(3)?,0x2dde0)?;
    // 0x2daec: str r4, [r5, #0xdc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,220)?)?;
    // 0x2daf0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2daf4: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,0)?)?;
    // 0x2daf8: str r3, [r5, #0xe0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,224)?)?;
    // 0x2dafc: str r4, [r5, #0xe4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,228)?)?;
    // 0x2db00: str r0, [r5, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,232)?)?;
    // 0x2db04: str r4, [r5, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,236)?)?;
    // 0x2db08: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x2db0c: ldr r3, [pc, #0x2d8]        @ 0x7ec <entry+0x7ec>
    c.literal(R::new(3)?,0x2ddec)?;
    // 0x2db10: str r3, [r5, #0xf0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,240)?)?;
    // 0x2db14: str r4, [r5, #0xf4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,244)?)?;
    // 0x2db18: str r10, [r5, #0xf8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,248)?)?;
    // 0x2db1c: str r4, [r5, #0xfc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,252)?)?;
    // 0x2db20: mov r3, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65536)?)?;
    // 0x2db24: ldr lr, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::SP,24)?)?;
    // 0x2db28: str lr, [r5, #0x100]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,256)?)?;
    // 0x2db2c: str r4, [r5, #0x104]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,260)?)?;
    // 0x2db30: str r0, [r5, #0x108]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,264)?)?;
    // 0x2db34: add r0, r5, #284
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,284)?)?;
    // 0x2db38: str r1, [r5, #0x10c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,268)?)?;
    // 0x2db3c: str r3, [r5, #0x110]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,272)?)?;
    // 0x2db40: str r4, [r5, #0x114]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,276)?)?;
    // 0x2db44: str r6, [r5, #0x118]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,280)?)?;
    // 0x2db48: str r6, [r5, #0xb4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,180)?)?;
    // 0x2db4c: stm r0, {r1, r3, r4, r8}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(8)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2db50: add r0, r5, #300
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,300)?)?;
    // 0x2db54: stm r0, {r1, r7, r12}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(7)?.bit()|R::R12.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2db58: mov r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4096)?)?;
    // 0x2db5c: add r12, r5, #324
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R12,R::new(5)?,324)?)?;
    // 0x2db60: str r4, [r5, #0x13c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,316)?)?;
    // 0x2db64: str r2, [r5, #0x140]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,320)?)?;
    // 0x2db68: movne r0, #1920
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1920)?)?;
    // 0x2db6c: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x2db70: mov r9, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,8)?)?;
    // 0x2db74: stm r12, {r0, r4, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::new(4)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2db78: mov r7, #4064
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,4064)?)?;
    // 0x2db7c: str r2, [r5, #0x150]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,336)?)?;
    // 0x2db80: str r0, [r5, #0x154]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,340)?)?;
    // 0x2db84: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2db88: str r4, [r5, #0x158]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,344)?)?;
    // 0x2db8c: mov r12, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,6)?)?;
    // 0x2db90: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x2db94: str r0, [r5, #0x15c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,348)?)?;
    // 0x2db98: str r4, [r5, #0x160]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,352)?)?;
    // 0x2db9c: mov r0, r7
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(7)?))?;
    // 0x2dba0: moveq r0, #36864
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,36864)?)?;
    // 0x2dba4: str r0, [r5, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,356)?)?;
    // 0x2dba8: str r4, [r5, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,360)?)?;
    // 0x2dbac: str r9, [r5, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,364)?)?;
    // 0x2dbb0: str r4, [r5, #0x170]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,368)?)?;
    // 0x2dbb4: ldr r0, [pc, #0x234]        @ 0x7f0 <entry+0x7f0>
    c.literal(R::R0,0x2ddf0)?;
    // 0x2dbb8: str r0, [r5, #0x174]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,372)?)?;
    // 0x2dbbc: add r0, r5, #400
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,400)?)?;
    // 0x2dbc0: str r4, [r5, #0x178]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,376)?)?;
    // 0x2dbc4: str r10, [r5, #0x17c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,380)?)?;
    // 0x2dbc8: str r4, [r5, #0x180]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,384)?)?;
    // 0x2dbcc: str lr, [r5, #0x184]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,388)?)?;
    // 0x2dbd0: str r4, [r5, #0x188]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,392)?)?;
    // 0x2dbd4: str r12, [r5, #0x18c]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,396)?)?;
    // 0x2dbd8: stm r0, {r1, r3, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dbdc: add r0, r5, #416
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,416)?)?;
    // 0x2dbe0: stm r0, {r1, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(3)?.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dbe4: str r6, [r5, #0x138]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(5)?,312)?)?;
    // 0x2dbe8: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x2dbec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2dbf0: beq 0x2dc10 <entry+0x610>   @ imm = #0x18
    c.b(C::Eq,0x2dc10)?;
    // 0x2dbf4: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2dbf8: add r1, r5, #428
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,428)?)?;
    // 0x2dbfc: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // 0x2dc00: mov r4, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,7)?)?;
    // 0x2dc04: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,4096)?)?;
    // 0x2dc08: add r2, r0, #10
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,10)?)?;
    // 0x2dc0c: b 0x2dcf8 <entry+0x6f8>   @ imm = #0xe4
    c.b(C::Al,0x2dcf8)?;
    // 0x2dc10: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x2dc14: mov r10, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,7)?)?;
    // 0x2dc18: str r8, [r5, #0x1ac]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,428)?)?;
    // 0x2dc1c: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x2dc20: mov lr, #1920
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1920)?)?;
    // 0x2dc24: str r0, [r5, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,432)?)?;
    // 0x2dc28: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2dc2c: orr r10, r10, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,4096)?)?;
    // 0x2dc30: str r0, [r5, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,436)?)?;
    // 0x2dc34: mov r0, #15424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15424)?)?;
    // 0x2dc38: str r10, [r5, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::new(5)?,440)?)?;
    // 0x2dc3c: str r4, [r5, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,448)?)?;
    // 0x2dc40: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2dc44: orr r0, r0, #229376
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,229376)?)?;
    // 0x2dc48: str r4, [r5, #0x1c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,452)?)?;
    // 0x2dc4c: str r0, [r5, #0x1c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,456)?)?;
    // 0x2dc50: mov r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,5)?)?;
    // 0x2dc54: str r4, [r5, #0x1cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,460)?)?;
    // 0x2dc58: str r0, [r5, #0x1d0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,464)?)?;
    // 0x2dc5c: mov r0, #30
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,30)?)?;
    // 0x2dc60: str r4, [r5, #0x1d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,468)?)?;
    // 0x2dc64: str r0, [r5, #0x1d8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,472)?)?;
    // 0x2dc68: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2dc6c: str r4, [r5, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,476)?)?;
    // 0x2dc70: str r4, [r5, #0x1e0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,480)?)?;
    // 0x2dc74: str r2, [r5, #0x1e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,484)?)?;
    // 0x2dc78: str lr, [r5, #0x1e8]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,488)?)?;
    // 0x2dc7c: str r4, [r5, #0x1ec]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,492)?)?;
    // 0x2dc80: str r0, [r5, #0x1f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,496)?)?;
    // 0x2dc84: str r2, [r5, #0x1f4]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,500)?)?;
    // 0x2dc88: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2dc8c: add r2, r5, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,512)?)?;
    // 0x2dc90: str lr, [r5, #0x1f8]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,504)?)?;
    // 0x2dc94: str r4, [r5, #0x1fc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,508)?)?;
    // 0x2dc98: stm r2, {r0, r4, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R0.bit()|R::new(4)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dc9c: str r4, [r5, #0x20c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,524)?)?;
    // 0x2dca0: str r9, [r5, #0x210]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::new(5)?,528)?)?;
    // 0x2dca4: str r4, [r5, #0x214]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,532)?)?;
    // 0x2dca8: ldr r0, [pc, #0x144]        @ 0x7f4 <entry+0x7f4>
    c.literal(R::R0,0x2ddf4)?;
    // 0x2dcac: str r0, [r5, #0x218]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,536)?)?;
    // 0x2dcb0: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x2dcb4: str r4, [r5, #0x21c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,540)?)?;
    // 0x2dcb8: str r0, [r5, #0x220]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,544)?)?;
    // 0x2dcbc: add r0, r5, #564
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,564)?)?;
    // 0x2dcc0: str r4, [r5, #0x224]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,548)?)?;
    // 0x2dcc4: str r4, [r5, #0x228]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,552)?)?;
    // 0x2dcc8: str r4, [r5, #0x22c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(5)?,556)?)?;
    // 0x2dccc: str r12, [r5, #0x230]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::new(5)?,560)?)?;
    // 0x2dcd0: stm r0, {r1, r3, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(3)?.bit()|R::new(4)?.bit()|R::new(6)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dcd4: add r0, r5, #580
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,580)?)?;
    // 0x2dcd8: stm r0, {r1, r3, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit()|R::new(3)?.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2dcdc: mov r0, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
    // 0x2dce0: add r1, r5, #592
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,592)?)?;
    // 0x2dce4: mov r4, r10
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(10)?))?;
    // 0x2dce8: str r0, [r5, #0x1bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,444)?)?;
    // 0x2dcec: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2dcf0: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // 0x2dcf4: add r2, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,12)?)?;
    // 0x2dcf8: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x2dcfc: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2dd00: mov r0, #43
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,43)?)?;
    // 0x2dd04: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x2dd08: orr r5, r5, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,20480)?)?;
    // 0x2dd0c: str r8, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,0)?)?;
    // 0x2dd10: orr r3, r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,8192)?)?;
    // 0x2dd14: str r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,4)?)?;
    // 0x2dd18: str r7, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,8)?)?;
    // 0x2dd1c: str r4, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,12)?)?;
    // 0x2dd20: str r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,20)?)?;
    // 0x2dd24: str r6, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,24)?)?;
    // 0x2dd28: str r6, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,28)?)?;
    // 0x2dd2c: str r6, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,32)?)?;
    // 0x2dd30: str r6, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,16)?)?;
    // 0x2dd34: str r8, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,36)?)?;
    // 0x2dd38: str r5, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R1,40)?)?;
    // 0x2dd3c: str r7, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,44)?)?;
    // 0x2dd40: str r4, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,48)?)?;
    // 0x2dd44: str r0, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,56)?)?;
    // 0x2dd48: str r6, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,60)?)?;
    // 0x2dd4c: str r6, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,64)?)?;
    // 0x2dd50: str r6, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,68)?)?;
    // 0x2dd54: str r6, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R1,52)?)?;
    // 0x2dd58: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,32)?)?;
    // 0x2dd5c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2dd60: beq 0x2ddbc <entry+0x7bc>   @ imm = #0x54
    c.b(C::Eq,0x2ddbc)?;
    // 0x2dd64: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2dd68: str r8, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::R1,72)?)?;
    // 0x2dd6c: str r3, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R1,76)?)?;
    // 0x2dd70: str r7, [r1, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R1,80)?)?;
    // 0x2dd74: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2dd78: str r0, [r1, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,84)?)?;
    // 0x2dd7c: ldr r0, [pc, #0x74]         @ 0x7f8 <entry+0x7f8>
    c.literal(R::R0,0x2ddf8)?;
    // 0x2dd80: str r0, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,92)?)?;
    // 0x2dd84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2dd88: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2dd8c: str r0, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,96)?)?;
    // 0x2dd90: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x2dd94: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2dd98: str r0, [r1, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,100)?)?;
    // 0x2dd9c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x2dda0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x2dda4: str r0, [r1, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,104)?)?;
    // 0x2dda8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ddac: str r0, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,88)?)?;
    // 0x2ddb0: add r0, r2, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,6)?)?;
    // 0x2ddb4: add r1, r1, #108
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,108)?)?;
    // 0x2ddb8: b 0x2ddc4 <entry+0x7c4>   @ imm = #0x4
    c.b(C::Al,0x2ddc4)?;
    // 0x2ddbc: add r0, r2, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,4)?)?;
    // 0x2ddc0: add r1, r1, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,72)?)?;
    // 0x2ddc4: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,4)?)?;
    // 0x2ddc8: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x2ddcc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2ddd0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2ddd4)?;
    const CONTROLLER_CONFIG_DATA_2DDD4:[u32;10]=[0xe00e0008,0x000eabd4,0x000e2788,0x0010df9c,0x000e7d74,0x000eabf5,0x0d1cef00,0x05f5e100,0x02625a00,0x00201001];
    for word in CONTROLLER_CONFIG_DATA_2DDD4 {c.emit(word)?;}
    c.check(0x2ddfc)?;
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;initialize_command_register_tables(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn full_command_register_table_has_exact_span(){
        let mut c=Code::new(START); emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(u32::from_le_bytes(c.bytes[0x2ddd4-START..0x2ddd8-START].try_into().unwrap()),0xe00e0008);
    }
}
