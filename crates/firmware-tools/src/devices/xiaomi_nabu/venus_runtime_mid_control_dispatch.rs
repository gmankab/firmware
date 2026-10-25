// SPDX-License-Identifier: GPL-3.0-only
//! Codec command callback dispatcher. A nine-way runtime message dispatch
//! handles buffer synchronization, codec clock state and control notifications.
//! Constant tables are distinguished from executable instructions; reference
//! opcode words are NOT embedded in this Rust implementation.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x2aad8;
const END:usize=0x2b13c;

/// Parse input message kind, select handler through a nine-target branch
/// table, then publish controller state and release the saved stack frame.
fn handle_controller_control_messages(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2aad8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2aadc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2aae0: sub sp, sp, #52
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,52)?)?;
    // 0x2aae4: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x2aae8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x2aaec: ldr r2, [pc, #0x5c4]        @ 0xcb8 <entry+0xcb8>
    c.literal(R::R2,0x2b0b8)?;
    // 0x2aaf0: mov r1, #205
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,205)?)?;
    // 0x2aaf4: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x2aaf8: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x2aafc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ab00: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2ab04: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x2935c
    c.bl(0x17b0)?;
    // 0x2ab08: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2ab0c: ldr r9, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,8)?)?;
    // 0x2ab10: ldr r10, [pc, #0x5b0]       @ 0xcc8 <entry+0xcc8>
    c.literal(R::new(10)?,0x2b0c8)?;
    // 0x2ab14: ldr r6, [pc, #0x5b0]        @ 0xccc <entry+0xccc>
    c.literal(R::new(6)?,0x2b0cc)?;
    // 0x2ab18: ldr r8, [pc, #0x5a0]        @ 0xcc0 <entry+0xcc0>
    c.literal(R::new(8)?,0x2b0c0)?;
    // 0x2ab1c: add r7, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,12)?)?;
    // 0x2ab20: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,12)?)?;
    // 0x2ab24: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,16)?)?;
    // 0x2ab28: add r0, r4, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2ab2c: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x2ab30: mov r4, r7
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(7)?))?;
    // 0x2ab34: ldr r0, [r4], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x2ab38: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x2ab3c: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2ab40: bhi 0x2abb4 <entry+0x7b4>   @ imm = #0x6c
    c.b(C::Hi,0x2abb4)?;
    // 0x2ab44: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x2ab48: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x2ab4c: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x2ab50)?;
    const CONTROL_MESSAGE_ROUTES:[u32;9]=[
        0x0002ab74,0x0002abb4,0x0002abb4,0x0002ac80,0x0002ac40,
        0x0002abb4,0x0002abb4,0x0002accc,0x0002ada8,
    ];
    for v in CONTROL_MESSAGE_ROUTES { c.emit(v)?; }
    c.check(0x2ab74)?;
    // 0x2ab74: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x2ab78: add r4, r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(7)?,12)?)?;
    // 0x2ab7c: cmp r0, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x2ab80: blo 0x2b020 <entry+0xc20>   @ imm = #0x498
    c.b(C::Cc,0x2b020)?;
    // 0x2ab84: ldr r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // 0x2ab88: ldr r1, [pc, #0x56c]        @ 0xcfc <entry+0xcfc>
    c.literal(R::R1,0x2b0fc)?;
    // 0x2ab8c: ldr r2, [pc, #0x56c]        @ 0xd00 <entry+0xd00>
    c.literal(R::R2,0x2b100)?;
    // 0x2ab90: mov r8, r5
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(5)?))?;
    // 0x2ab94: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x2ab98: tst r0, #268435456
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,268435456)?)?;
    // 0x2ab9c: ldr r1, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,8)?)?;
    // 0x2aba0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x2aba4: bne 0x2adbc <entry+0x9bc>   @ imm = #0x210
    c.b(C::Ne,0x2adbc)?;
    // 0x2aba8: ldr r0, [pc, #0x554]        @ 0xd04 <entry+0xd04>
    c.literal(R::R0,0x2b104)?;
    // 0x2abac: bl 0x25c24 <entry+0xffffffffffffb824> @ imm = #-0x4f90
    c.bl(0x25c24)?;
    // 0x2abb0: b 0x2adc0 <entry+0x9c0>   @ imm = #0x208
    c.b(C::Al,0x2adc0)?;
    // 0x2abb4: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2abb8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2abbc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2abc0: orr r1, r1, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,135168)?)?;
    // 0x2abc4: str r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,28)?)?;
    // 0x2abc8: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x2abcc: str r1, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,32)?)?;
    // 0x2abd0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2abd4: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,36)?)?;
    // 0x2abd8: mov r1, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,14)?)?;
    // 0x2abdc: str r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,40)?)?;
    // 0x2abe0: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x2abe4: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2abe8: add r1, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,24)?)?;
    // 0x2abec: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2abf0: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2abf4: ldr r3, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,128)?)?;
    // 0x2abf8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2abfc: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2ac00: bl 0x2a198 <entry+0xfffffffffffffd98> @ imm = #-0xa70
    c.bl(0x2a198)?;
    // 0x2ac04: ldr r1, [pc, #0x52c]        @ 0xd38 <entry+0xd38>
    c.literal(R::R1,0x2b138)?;
    // 0x2ac08: mov r3, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,134)?)?;
    // 0x2ac0c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ac10: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2ac14: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2ac18: bl 0x2ec94 <entry+0x4894>  @ imm = #0x4074
    c.bl(0x2ec94)?;
    // 0x2ac1c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ac20: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ac24: bl 0x2ec94 <entry+0x4894>  @ imm = #0x4068
    c.bl(0x2ec94)?;
    // 0x2ac28: ldr r2, [pc, #0x488]        @ 0xcb8 <entry+0xcb8>
    c.literal(R::R2,0x2b0b8)?;
    // 0x2ac2c: mov r1, #135
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,135)?)?;
    // 0x2ac30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ac34: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x2ac38: bl 0x17b0 <entry+0xfffffffffffd73b0> @ imm = #-0x29490
    c.bl(0x17b0)?;
    // 0x2ac3c: b 0x2b010 <entry+0xc10>   @ imm = #0x3cc
    c.b(C::Al,0x2b010)?;
    // 0x2ac40: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x2ac44: add r7, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,8)?)?;
    // 0x2ac48: cmp r0, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2ac4c: blo 0x2b028 <entry+0xc28>   @ imm = #0x3d4
    c.b(C::Cc,0x2b028)?;
    // 0x2ac50: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2ac54: ldr r1, [pc, #0x460]        @ 0xcbc <entry+0xcbc>
    c.literal(R::R1,0x2b0bc)?;
    // 0x2ac58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ac5c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x2ac60: beq 0x2b014 <entry+0xc14>   @ imm = #0x3ac
    c.b(C::Eq,0x2b014)?;
    // 0x2ac64: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x2ac68: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ac6c: bne 0x2b014 <entry+0xc14>   @ imm = #0x3a0
    c.b(C::Ne,0x2b014)?;
    // 0x2ac70: bl 0x26764 <entry+0xffffffffffffc364> @ imm = #-0x4514
    c.bl(0x26764)?;
    // 0x2ac74: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2ac78: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x2ac7c: b 0x2b014 <entry+0xc14>   @ imm = #0x390
    c.b(C::Al,0x2b014)?;
    // 0x2ac80: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,20)?)?;
    // 0x2ac84: add r7, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,8)?)?;
    // 0x2ac88: cmp r0, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2ac8c: blo 0x2b030 <entry+0xc30>   @ imm = #0x39c
    c.b(C::Cc,0x2b030)?;
    // 0x2ac90: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x2ac94: ldr r1, [pc, #0x428]        @ 0xcc4 <entry+0xcc4>
    c.literal(R::R1,0x2b0c4)?;
    // 0x2ac98: mov r3, #93
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,93)?)?;
    // 0x2ac9c: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2aca0: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2aca4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2aca8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2acac: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3fe0
    c.bl(0x2ec94)?;
    // 0x2acb0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2acb4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2acb8: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3fd4
    c.bl(0x2ec94)?;
    // 0x2acbc: ldr r1, [pc, #0x40c]        @ 0xcd0 <entry+0xcd0>
    c.literal(R::R1,0x2b0d0)?;
    // 0x2acc0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2acc4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x2acc8: b 0x2b014 <entry+0xc14>   @ imm = #0x344
    c.b(C::Al,0x2b014)?;
    // 0x2accc: ldr r0, [pc, #0x404]        @ 0xcd8 <entry+0xcd8>
    c.literal(R::R0,0x2b0d8)?;
    // 0x2acd0: mov r10, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,1)?)?;
    // 0x2acd4: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x2acd8: mov r8, r6
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // 0x2acdc: ldr r5, [pc, #0x3fc]        @ 0xce0 <entry+0xce0>
    c.literal(R::new(5)?,0x2b0e0)?;
    // 0x2ace0: str r10, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R0,0)?)?;
    // 0x2ace4: ldr r0, [pc, #0x3f0]        @ 0xcdc <entry+0xcdc>
    c.literal(R::R0,0x2b0dc)?;
    // 0x2ace8: ldm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2acec: stm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2acf0: ldm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2acf4: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2acf8: stm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2acfc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2ad00: ldm r0!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2ad04: stm r1!, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x2ad08: ldm r0, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R0,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2ad0c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ad10: stm r1, {r2, r3, r6, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R2.bit()|R::new(3)?.bit()|R::new(6)?.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2ad14: ldr r1, [pc, #0x3c8]        @ 0xce4 <entry+0xce4>
    c.literal(R::R1,0x2b0e4)?;
    // 0x2ad18: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3f74
    c.bl(0x2ec94)?;
    // 0x2ad1c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ad20: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x2ad24: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3f68
    c.bl(0x2ec94)?;
    // 0x2ad28: add r7, r5, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(5)?,12)?)?;
    // 0x2ad2c: ldm r5, {r2, r3}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R2.bit()|R::new(3)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2ad30: ldm r7, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(7)?,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2ad34: stm sp, {r0, r1, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2ad38: ldr r1, [pc, #0x3a8]        @ 0xce8 <entry+0xce8>
    c.literal(R::R1,0x2b0e8)?;
    // 0x2ad3c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ad40: mov r7, r5
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(5)?))?;
    // 0x2ad44: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3f48
    c.bl(0x2ec94)?;
    // 0x2ad48: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ad4c: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x2ad50: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3f3c
    c.bl(0x2ec94)?;
    // 0x2ad54: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2ad58: and r1, r10, r0, lsr #2
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(10)?,R::R0,Shift::Lsr,2,false)?)?;
    // 0x2ad5c: and r3, r10, r0, lsr #1
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(10)?,R::R0,Shift::Lsr,1,false)?)?;
    // 0x2ad60: and r2, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,1)?)?;
    // 0x2ad64: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ad68: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x2ad6c: ldr r1, [pc, #0x378]        @ 0xcec <entry+0xcec>
    c.literal(R::R1,0x2b0ec)?;
    // 0x2ad70: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3f1c
    c.bl(0x2ec94)?;
    // 0x2ad74: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ad78: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x2ad7c: mov r6, r8
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(8)?))?;
    // 0x2ad80: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3f0c
    c.bl(0x2ec94)?;
    // 0x2ad84: ldr r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,16)?)?;
    // 0x2ad88: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x2ad8c: beq 0x2af1c <entry+0xb1c>   @ imm = #0x188
    c.b(C::Eq,0x2af1c)?;
    // 0x2ad90: ldr r10, [pc, #0x330]       @ 0xcc8 <entry+0xcc8>
    c.literal(R::new(10)?,0x2b0c8)?;
    // 0x2ad94: cmp r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,32)?)?;
    // 0x2ad98: bne 0x2af30 <entry+0xb30>   @ imm = #0x190
    c.b(C::Ne,0x2af30)?;
    // 0x2ad9c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ada0: str r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,16)?)?;
    // 0x2ada4: b 0x2af64 <entry+0xb64>   @ imm = #0x1b8
    c.b(C::Al,0x2af64)?;
    // 0x2ada8: ldr r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,4)?)?;
    // 0x2adac: ldr r1, [pc, #0x320]        @ 0xcd4 <entry+0xcd4>
    c.literal(R::R1,0x2b0d4)?;
    // 0x2adb0: add r7, r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,8)?)?;
    // 0x2adb4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x2adb8: b 0x2b014 <entry+0xc14>   @ imm = #0x254
    c.b(C::Al,0x2b014)?;
    // 0x2adbc: bl 0x25c48 <entry+0xffffffffffffb848> @ imm = #-0x517c
    c.bl(0x25c48)?;
    // 0x2adc0: ldr r0, [pc, #0x340]        @ 0xd08 <entry+0xd08>
    c.literal(R::R0,0x2b108)?;
    // 0x2adc4: ldr r1, [pc, #0x340]        @ 0xd0c <entry+0xd0c>
    c.literal(R::R1,0x2b10c)?;
    // 0x2adc8: mov r3, #241
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,241)?)?;
    // 0x2adcc: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2add0: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x2add4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2add8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2addc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3eb0
    c.bl(0x2ec94)?;
    // 0x2ade0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ade4: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ade8: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3ea4
    c.bl(0x2ec94)?;
    // 0x2adec: ldr r5, [pc, #0x31c]        @ 0xd10 <entry+0xd10>
    c.literal(R::new(5)?,0x2b110)?;
    // 0x2adf0: ldr r2, [pc, #0x31c]        @ 0xd14 <entry+0xd14>
    c.literal(R::R2,0x2b114)?;
    // 0x2adf4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2adf8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2adfc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e90
    c.bl(0x2ec94)?;
    // 0x2ae00: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ae04: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ae08: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e84
    c.bl(0x2ec94)?;
    // 0x2ae0c: ldr r2, [pc, #0x304]        @ 0xd18 <entry+0xd18>
    c.literal(R::R2,0x2b118)?;
    // 0x2ae10: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ae14: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x2ae18: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e74
    c.bl(0x2ec94)?;
    // 0x2ae1c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ae20: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ae24: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e68
    c.bl(0x2ec94)?;
    // 0x2ae28: ldr r0, [pc, #0x2f0]        @ 0xd20 <entry+0xd20>
    c.literal(R::R0,0x2b120)?;
    // 0x2ae2c: ldr r1, [pc, #0x2f0]        @ 0xd24 <entry+0xd24>
    c.literal(R::R1,0x2b124)?;
    // 0x2ae30: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2ae34: mov r3, #1012
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1012)?)?;
    // 0x2ae38: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2ae3c: ldr r0, [pc, #0x2d8]        @ 0xd1c <entry+0xd1c>
    c.literal(R::R0,0x2b11c)?;
    // 0x2ae40: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2ae44: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ae48: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e44
    c.bl(0x2ec94)?;
    // 0x2ae4c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2ae50: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ae54: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e38
    c.bl(0x2ec94)?;
    // 0x2ae58: mov r5, r8
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(8)?))?;
    // 0x2ae5c: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,32)?)?;
    // 0x2ae60: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ae64: bne 0x2ae80 <entry+0xa80>   @ imm = #0x14
    c.b(C::Ne,0x2ae80)?;
    // 0x2ae68: ldr r1, [pc, #0x2b8]        @ 0xd28 <entry+0xd28>
    c.literal(R::R1,0x2b128)?;
    // 0x2ae6c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ae70: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e1c
    c.bl(0x2ec94)?;
    // 0x2ae74: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2ae78: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2ae7c: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3e10
    c.bl(0x2ec94)?;
    // 0x2ae80: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2ae84: ldr r8, [pc, #0x234]        @ 0xcc0 <entry+0xcc0>
    c.literal(R::new(8)?,0x2b0c0)?;
    // 0x2ae88: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x2ae8c: bne 0x2ae9c <entry+0xa9c>   @ imm = #0x8
    c.b(C::Ne,0x2ae9c)?;
    // 0x2ae90: ldr r1, [pc, #0x298]        @ 0xd30 <entry+0xd30>
    c.literal(R::R1,0x2b130)?;
    // 0x2ae94: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2ae98: b 0x2aed4 <entry+0xad4>   @ imm = #0x34
    c.b(C::Al,0x2aed4)?;
    // 0x2ae9c: ldr r1, [pc, #0x288]        @ 0xd2c <entry+0xd2c>
    c.literal(R::R1,0x2b12c)?;
    // 0x2aea0: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x2aea4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2aea8: beq 0x2aec8 <entry+0xac8>   @ imm = #0x18
    c.b(C::Eq,0x2aec8)?;
    // 0x2aeac: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2aeb0: bl 0x85708 <entry+0x5b308> @ imm = #0x5a850
    c.bl(0x85708)?;
    // 0x2aeb4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2aeb8: bl 0x85720 <entry+0x5b320> @ imm = #0x5a860
    c.bl(0x85720)?;
    // 0x2aebc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2aec0: bl 0x8533c <entry+0x5af3c> @ imm = #0x5a474
    c.bl(0x8533c)?;
    // 0x2aec4: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2aec8: ldr r2, [pc, #0x260]        @ 0xd30 <entry+0xd30>
    c.literal(R::R2,0x2b130)?;
    // 0x2aecc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2aed0: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R2,0)?)?;
    // 0x2aed4: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x2aed8: bne 0x2b010 <entry+0xc10>   @ imm = #0x130
    c.b(C::Ne,0x2b010)?;
    // 0x2aedc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2aee0: beq 0x2b010 <entry+0xc10>   @ imm = #0x128
    c.b(C::Eq,0x2b010)?;
    // 0x2aee4: ldr r0, [pc, #0x240]        @ 0xd2c <entry+0xd2c>
    c.literal(R::R0,0x2b12c)?;
    // 0x2aee8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2aeec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2aef0: beq 0x2af0c <entry+0xb0c>   @ imm = #0x14
    c.b(C::Eq,0x2af0c)?;
    // 0x2aef4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2aef8: bl 0x85708 <entry+0x5b308> @ imm = #0x5a808
    c.bl(0x85708)?;
    // 0x2aefc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2af00: bl 0x85720 <entry+0x5b320> @ imm = #0x5a818
    c.bl(0x85720)?;
    // 0x2af04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2af08: bl 0x8533c <entry+0x5af3c> @ imm = #0x5a42c
    c.bl(0x8533c)?;
    // 0x2af0c: ldr r1, [pc, #0x21c]        @ 0xd30 <entry+0xd30>
    c.literal(R::R1,0x2b130)?;
    // 0x2af10: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2af14: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x2af18: b 0x2b010 <entry+0xc10>   @ imm = #0xf0
    c.b(C::Al,0x2b010)?;
    // 0x2af1c: ldr r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // 0x2af20: ldr r10, [pc, #0x1a0]       @ 0xcc8 <entry+0xcc8>
    c.literal(R::new(10)?,0x2b0c8)?;
    // 0x2af24: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2af28: str r0, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,16)?)?;
    // 0x2af2c: b 0x2af68 <entry+0xb68>   @ imm = #0x34
    c.b(C::Al,0x2af68)?;
    // 0x2af30: ldr r1, [pc, #0x1b8]        @ 0xcf0 <entry+0xcf0>
    c.literal(R::R1,0x2b0f0)?;
    // 0x2af34: mov r3, #46
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,46)?)?;
    // 0x2af38: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2af3c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2af40: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2af44: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2af48: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3d44
    c.bl(0x2ec94)?;
    // 0x2af4c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2af50: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2af54: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3d38
    c.bl(0x2ec94)?;
    // 0x2af58: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2af5c: bic r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,2)?)?;
    // 0x2af60: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x2af64: ldr r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,16)?)?;
    // 0x2af68: ldr r0, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,12)?)?;
    // 0x2af6c: ldr r8, [pc, #0x14c]        @ 0xcc0 <entry+0xcc0>
    c.literal(R::new(8)?,0x2b0c0)?;
    // 0x2af70: cmp r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
    // 0x2af74: beq 0x2af8c <entry+0xb8c>   @ imm = #0x10
    c.b(C::Eq,0x2af8c)?;
    // 0x2af78: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4)?)?;
    // 0x2af7c: bne 0x2af98 <entry+0xb98>   @ imm = #0x14
    c.b(C::Ne,0x2af98)?;
    // 0x2af80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2af84: str r0, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,12)?)?;
    // 0x2af88: b 0x2afcc <entry+0xbcc>   @ imm = #0x3c
    c.b(C::Al,0x2afcc)?;
    // 0x2af8c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2af90: str r0, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,12)?)?;
    // 0x2af94: b 0x2afcc <entry+0xbcc>   @ imm = #0x30
    c.b(C::Al,0x2afcc)?;
    // 0x2af98: ldr r1, [pc, #0x154]        @ 0xcf4 <entry+0xcf4>
    c.literal(R::R1,0x2b0f4)?;
    // 0x2af9c: mov r3, #65
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65)?)?;
    // 0x2afa0: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2afa4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2afa8: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2afac: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2afb0: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3cdc
    c.bl(0x2ec94)?;
    // 0x2afb4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2afb8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2afbc: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3cd0
    c.bl(0x2ec94)?;
    // 0x2afc0: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2afc4: bic r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,1)?)?;
    // 0x2afc8: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x2afcc: ldr r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,20)?)?;
    // 0x2afd0: sub r1, r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,13)?)?;
    // 0x2afd4: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,7)?)?;
    // 0x2afd8: blo 0x2b010 <entry+0xc10>   @ imm = #0x30
    c.b(C::Cc,0x2b010)?;
    // 0x2afdc: ldr r1, [pc, #0x114]        @ 0xcf8 <entry+0xcf8>
    c.literal(R::R1,0x2b0f8)?;
    // 0x2afe0: mov r3, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,72)?)?;
    // 0x2afe4: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2afe8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2afec: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x2aff0: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2aff4: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3c98
    c.bl(0x2ec94)?;
    // 0x2aff8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2affc: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2b000: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3c8c
    c.bl(0x2ec94)?;
    // 0x2b004: ldr r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,8)?)?;
    // 0x2b008: bic r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,4)?)?;
    // 0x2b00c: str r0, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,8)?)?;
    // 0x2b010: mov r7, r4
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::new(4)?))?;
    // 0x2b014: subs r9, r9, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(9)?,R::new(9)?,1,true)?)?;
    // 0x2b018: bne 0x2ab30 <entry+0x730>   @ imm = #-0x4f0
    c.b(C::Ne,0x2ab30)?;
    // 0x2b01c: b 0x2b0b0 <entry+0xcb0>   @ imm = #0x8c
    c.b(C::Al,0x2b0b0)?;
    // 0x2b020: mov r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,1)?)?;
    // 0x2b024: b 0x2b034 <entry+0xc34>   @ imm = #0x8
    c.b(C::Al,0x2b034)?;
    // 0x2b028: mov r4, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x2b02c: b 0x2b034 <entry+0xc34>   @ imm = #0x0
    c.b(C::Al,0x2b034)?;
    // 0x2b030: mov r4, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,4)?)?;
    // 0x2b034: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,12)?)?;
    // 0x2b038: ldr r1, [pc, #0xf4]         @ 0xd34 <entry+0xd34>
    c.literal(R::R1,0x2b134)?;
    // 0x2b03c: ldr r2, [pc, #0x84]         @ 0xcc8 <entry+0xcc8>
    c.literal(R::R2,0x2b0c8)?;
    // 0x2b040: mov r3, #142
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,142)?)?;
    // 0x2b044: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2b048: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x2b04c: stm sp, {r0, r4}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(4)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x2b050: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b054: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3c38
    c.bl(0x2ec94)?;
    // 0x2b058: ldr r1, [pc, #0x6c]         @ 0xccc <entry+0xccc>
    c.literal(R::R1,0x2b0cc)?;
    // 0x2b05c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2b060: bl 0x2ec94 <entry+0x4894>  @ imm = #0x3c2c
    c.bl(0x2ec94)?;
    // 0x2b064: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b068: add r1, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,24)?)?;
    // 0x2b06c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2b070: orr r0, r0, #135168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,135168)?)?;
    // 0x2b074: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x2b078: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x2b07c: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x2b080: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x2b084: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x2b088: mov r0, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,13)?)?;
    // 0x2b08c: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x2b090: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x2b094: str r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,44)?)?;
    // 0x2b098: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2b09c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x2b0a0: ldr r3, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,128)?)?;
    // 0x2b0a4: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x2b0a8: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2b0ac: bl 0x2a198 <entry+0xfffffffffffffd98> @ imm = #-0xf1c
    c.bl(0x2a198)?;
    // 0x2b0b0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x2b0b4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x2b0b8)?;
    const CODEC_STATUS_AND_MMIO_POINTERS:[u32;33]=[
        0x000e884d,0x001040e0,0x001040d8,0x000e8b0c,0x000e895f,
        0x000e2788,0x001040e8,0x00103070,0x001040e4,0x0010df7c,
        0x0010df5c,0x000e89ce,0x000e541f,0x000e5471,0x000e89eb,
        0x000e8a47,0x000e8aa3,0x00104104,0x00104108,0x02bde780,
        0x000e24a8,0x000e8954,0x000e897e,0x000e24da,0x000e24f4,
        0x001040f8,0x001040ec,0x000e8981,0x000e899b,0x0010306c,
        0x001040dc,0x000e8b94,0x000e8b4b,
    ];
    for v in CODEC_STATUS_AND_MMIO_POINTERS { c.emit(v)?; }
    c.check(0x2b13c)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    handle_controller_control_messages(c)?;
    c.check(END)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn controller_dispatch_and_pools_have_exact_layout(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|vma:usize|u32::from_le_bytes(c.bytes[vma-START..vma-START+4].try_into().unwrap());
        assert_eq!(word(0x2ab50),0x0002ab74);
        assert_eq!(word(0x2ab70),0x0002ada8);
        assert_eq!(word(0x2b0b8),0x000e884d);
        assert_eq!(word(0x2b138),0x000e8b4b);
    }
}
