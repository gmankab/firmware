// SPDX-License-Identifier: GPL-3.0-only
//! Reconstruct Venus codec image/plane dimension calculations and command
//! word layouts. Explicit original constants are named data words; all
//! executable opcodes are encoded with validated ARM A32 operand builders.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x2fcbc;
const END:usize=0x3027c;
fn write_image_plane_parameters(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x2fcbc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x2fcc0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x2fcc4: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x2fcc8: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x2fccc: mov r4, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,5)?)?;
    // 0x2fcd0: mov r9, r1
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R1))?;
    // 0x2fcd4: mov r8, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,5)?)?;
    // 0x2fcd8: ldrb r0, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,47)?)?;
    // 0x2fcdc: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x2fce0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fce4: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x2fce8: movne r4, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,3)?)?;
    // 0x2fcec: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x2fcf0: ldrb r5, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,46)?)?;
    // 0x2fcf4: ldrh r0, [r10, #16]
    c.emit(arm32::load_halfword(C::Al,R::R0,R::new(10)?,16)?)?;
    // 0x2fcf8: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2fcfc: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x2fd00: beq 0x2fd14 <entry+0x714>   @ imm = #0xc
    c.b(C::Eq,0x2fd14)?;
    // 0x2fd04: ldrb r0, [r10, #0x30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,48)?)?;
    // 0x2fd08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fd0c: movne r8, #3
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(8)?,R::R0,3)?)?;
    // 0x2fd10: b 0x2fd18 <entry+0x718>   @ imm = #0x0
    c.b(C::Al,0x2fd18)?;
    // 0x2fd14: mov r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x2fd18: ldr r0, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,32)?)?;
    // 0x2fd1c: sub r1, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R0,4)?)?;
    // 0x2fd20: cmp r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,3)?)?;
    // 0x2fd24: bhs 0x2fd4c <entry+0x74c>   @ imm = #0x20
    c.b(C::Cs,0x2fd4c)?;
    // 0x2fd28: ldr r0, [pc, #0x530]        @ 0xc60 <entry+0xc60>
    c.literal(R::R0,0x30260)?;
    // 0x2fd2c: ldr r2, [pc, #0x534]        @ 0xc68 <entry+0xc68>
    c.literal(R::R2,0x30268)?;
    // 0x2fd30: ldr r3, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(3)?,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x2fd34: ldr r0, [pc, #0x528]        @ 0xc64 <entry+0xc64>
    c.literal(R::R0,0x30264)?;
    // 0x2fd38: ldr r6, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(6)?,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x2fd3c: ldr r2, [pc, #0x528]        @ 0xc6c <entry+0xc6c>
    c.literal(R::R2,0x3026c)?;
    // 0x2fd40: ldr r0, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::R1,Shift::Lsl,2)?)?;
    // 0x2fd44: ldr r1, [r2, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::R2,R::R1,Shift::Lsl,2)?)?;
    // 0x2fd48: b 0x2fd84 <entry+0x784>   @ imm = #0x34
    c.b(C::Al,0x2fd84)?;
    // 0x2fd4c: ldr r1, [pc, #0x500]        @ 0xc54 <entry+0xc54>
    c.literal(R::R1,0x30254)?;
    // 0x2fd50: ldr r2, [pc, #0x500]        @ 0xc58 <entry+0xc58>
    c.literal(R::R2,0x30258)?;
    // 0x2fd54: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x2fd58: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2fd5c: mov r3, #222
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,222)?)?;
    // 0x2fd60: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x10d4
    c.bl(0x2ec94)?;
    // 0x2fd64: ldr r1, [pc, #0x4f0]        @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x3025c)?;
    // 0x2fd68: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2fd6c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x10e0
    c.bl(0x2ec94)?;
    // 0x2fd70: ldrb r5, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(10)?,46)?)?;
    // 0x2fd74: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2fd78: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2fd7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2fd80: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2fd84: str r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,28)?)?;
    // 0x2fd88: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x2fd8c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x2fd90: bne 0x2fdb4 <entry+0x7b4>   @ imm = #0x1c
    c.b(C::Ne,0x2fdb4)?;
    // 0x2fd94: ldrb r2, [r10, #0x30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,48)?)?;
    // 0x2fd98: mov r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
    // 0x2fd9c: sub r0, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,12)?)?;
    // 0x2fda0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x2fda4: sub r2, r3, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(3)?,12)?)?;
    // 0x2fda8: movne r7, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x2fdac: lsl r3, r2, r7
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(3)?,R::R0,R::R2,Shift::Lsl,R::new(7)?,false)?)?;
    // 0x2fdb0: lsl r0, r0, r7
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsl,R::new(7)?,false)?)?;
    // 0x2fdb4: bic r0, r0, r0, asr #31
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R0,R::R0,R::R0,Shift::Asr,31,false)?)?;
    // 0x2fdb8: mul r2, r6, r4
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(6)?,R::new(4)?)?)?;
    // 0x2fdbc: mul r6, r1, r4
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::R1,R::new(4)?)?)?;
    // 0x2fdc0: bic r1, r3, r3, asr #31
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R1,R::new(3)?,R::new(3)?,Shift::Asr,31,false)?)?;
    // 0x2fdc4: add r7, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(8)?,1)?)?;
    // 0x2fdc8: lsr r4, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(4)?,R::R0,Shift::Lsr,2)?)?;
    // 0x2fdcc: ldrb r0, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,47)?)?;
    // 0x2fdd0: lsr r1, r1, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Lsr,2)?)?;
    // 0x2fdd4: mul lr, r1, r7
    c.emit(arm32::multiply_register(C::Al,R::LR,R::R1,R::new(7)?)?)?;
    // 0x2fdd8: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x2fddc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2fde0: ldrb r1, [r10, #0x30]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,48)?)?;
    // 0x2fde4: str r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,20)?)?;
    // 0x2fde8: addne r0, r2, r2, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R2,R::R2,Shift::Lsl,1,false)?)?;
    // 0x2fdec: lsrne r2, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::R2,R::R0,Shift::Lsr,1)?)?;
    // 0x2fdf0: addne r0, r6, r6, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::new(6)?,R::new(6)?,Shift::Lsl,1,false)?)?;
    // 0x2fdf4: lsrne r6, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::new(6)?,R::R0,Shift::Lsr,1)?)?;
    // 0x2fdf8: mul r0, r4, r7
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(4)?,R::new(7)?)?)?;
    // 0x2fdfc: str r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,12)?)?;
    // 0x2fe00: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2fe04: ldr r12, [r10, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::new(10)?,168)?)?;
    // 0x2fe08: addne r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,1,false)?)?;
    // 0x2fe0c: addne r1, lr, lr, lsl #1
    c.emit(arm32::data_register(C::Ne,Op::Add,R::R1,R::LR,R::LR,Shift::Lsl,1,false)?)?;
    // 0x2fe10: str r8, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,32)?)?;
    // 0x2fe14: lsrne r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::R0,R::R0,Shift::Lsr,1)?)?;
    // 0x2fe18: lsrne lr, r1, #1
    c.emit(arm32::mov_shifted_register(C::Ne,R::LR,R::R1,Shift::Lsr,1)?)?;
    // 0x2fe1c: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x2fe20: beq 0x2fe54 <entry+0x854>   @ imm = #0x2c
    c.b(C::Eq,0x2fe54)?;
    // 0x2fe24: ldr r1, [r10, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,172)?)?;
    // 0x2fe28: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2fe2c: mov r1, r0
    c.emit(arm32::mov_register(C::Al,R::R1,R::R0))?;
    // 0x2fe30: movne r1, r6
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(6)?))?;
    // 0x2fe34: lsl r3, r1, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::R1,Shift::Lsl,16)?)?;
    // 0x2fe38: str r3, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,76)?)?;
    // 0x2fe3c: str r3, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,116)?)?;
    // 0x2fe40: ldr r3, [r10, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,172)?)?;
    // 0x2fe44: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x2fe48: mov r3, lr
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::LR))?;
    // 0x2fe4c: movne r3, r2
    c.emit(arm32::mov_register(C::Ne,R::new(3)?,R::R2))?;
    // 0x2fe50: b 0x2fe68 <entry+0x868>   @ imm = #0x10
    c.b(C::Al,0x2fe68)?;
    // 0x2fe54: lsl r1, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(6)?,Shift::Lsl,16)?)?;
    // 0x2fe58: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x2fe5c: str r1, [r10, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,76)?)?;
    // 0x2fe60: str r1, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,116)?)?;
    // 0x2fe64: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2fe68: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,4)?)?;
    // 0x2fe6c: add r8, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x2fe70: add r5, r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,1)?)?;
    // 0x2fe74: orr r4, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x2fe78: str r4, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(10)?,80)?)?;
    // 0x2fe7c: str r4, [r10, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::new(10)?,120)?)?;
    // 0x2fe80: ldrb r1, [r10, #0x2c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,44)?)?;
    // 0x2fe84: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2fe88: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x2fe8c: beq 0x2feb0 <entry+0x8b0>   @ imm = #0x1c
    c.b(C::Eq,0x2feb0)?;
    // 0x2fe90: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x2fe94: bne 0x2feb8 <entry+0x8b8>   @ imm = #0x1c
    c.b(C::Ne,0x2feb8)?;
    // 0x2fe98: orr r1, r8, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(8)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x2fe9c: add r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2fea0: orr r3, r0, lr, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::LR,Shift::Lsl,16,false)?)?;
    // 0x2fea4: str r1, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,84)?)?;
    // 0x2fea8: str r3, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,88)?)?;
    // 0x2feac: b 0x2feec <entry+0x8ec>   @ imm = #0x38
    c.b(C::Al,0x2feec)?;
    // 0x2feb0: cmp r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,0)?)?;
    // 0x2feb4: beq 0x2fef0 <entry+0x8f0>   @ imm = #0x34
    c.b(C::Eq,0x2fef0)?;
    // 0x2feb8: ldr r1, [r10, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,172)?)?;
    // 0x2febc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x2fec0: moveq r0, r6
    c.emit(arm32::mov_register(C::Eq,R::R0,R::new(6)?))?;
    // 0x2fec4: orr r1, r8, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::new(8)?,R::R0,Shift::Lsl,16,false)?)?;
    // 0x2fec8: add r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x2fecc: str r1, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,84)?)?;
    // 0x2fed0: ldr r3, [r10, #0xac]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(10)?,172)?)?;
    // 0x2fed4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x2fed8: moveq lr, r2
    c.emit(arm32::mov_register(C::Eq,R::LR,R::R2))?;
    // 0x2fedc: orr r3, r0, lr, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R0,R::LR,Shift::Lsl,16,false)?)?;
    // 0x2fee0: str r3, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,88)?)?;
    // 0x2fee4: str r1, [r10, #0x7c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,124)?)?;
    // 0x2fee8: str r3, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,128)?)?;
    // 0x2feec: add r8, lr, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(8)?,R::LR,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2fef0: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,8)?)?;
    // 0x2fef4: mul r12, r5, r0
    c.emit(arm32::multiply_register(C::Al,R::R12,R::new(5)?,R::R0)?)?;
    // 0x2fef8: ldr r0, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,40)?)?;
    // 0x2fefc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2ff00: beq 0x2ff50 <entry+0x950>   @ imm = #0x48
    c.b(C::Eq,0x2ff50)?;
    // 0x2ff04: add r1, r8, r6
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(8)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x2ff08: orr r0, r8, r6, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(8)?,R::new(6)?,Shift::Lsl,16,false)?)?;
    // 0x2ff0c: orr r3, r1, r2, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R2,Shift::Lsl,16,false)?)?;
    // 0x2ff10: add r2, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x2ff14: str r0, [r10, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,100)?)?;
    // 0x2ff18: add r7, r2, #64
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R2,64)?)?;
    // 0x2ff1c: orr r1, r2, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R2,4194304)?)?;
    // 0x2ff20: add r0, r2, #768
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,768)?)?;
    // 0x2ff24: str r3, [r10, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,104)?)?;
    // 0x2ff28: add lr, r2, #896
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::R2,896)?)?;
    // 0x2ff2c: add r8, r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R2,128)?)?;
    // 0x2ff30: str r1, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,108)?)?;
    // 0x2ff34: orr r7, r7, #4194304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4194304)?)?;
    // 0x2ff38: add r1, r2, #384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R2,384)?)?;
    // 0x2ff3c: add r2, r2, #512
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,512)?)?;
    // 0x2ff40: orr r0, r0, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,8388608)?)?;
    // 0x2ff44: str r7, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(10)?,112)?)?;
    // 0x2ff48: orr r2, r2, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,16777216)?)?;
    // 0x2ff4c: b 0x2ff70 <entry+0x970>   @ imm = #0x1c
    c.b(C::Al,0x2ff70)?;
    // 0x2ff50: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2ff54: add lr, r8, #384
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::LR,R::new(8)?,384)?)?;
    // 0x2ff58: add r1, r8, #256
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(8)?,256)?)?;
    // 0x2ff5c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2ff60: str r0, [r10, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,100)?)?;
    // 0x2ff64: str r0, [r10, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,104)?)?;
    // 0x2ff68: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,108)?)?;
    // 0x2ff6c: str r0, [r10, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,112)?)?;
    // 0x2ff70: orr r3, r8, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(8)?,16777216)?)?;
    // 0x2ff74: orr r1, r1, #8388608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8388608)?)?;
    // 0x2ff78: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2ff7c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2ff80: str r3, [r10, #0x84]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(10)?,132)?)?;
    // 0x2ff84: str r1, [r10, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,136)?)?;
    // 0x2ff88: str r2, [r10, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(10)?,140)?)?;
    // 0x2ff8c: str r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,144)?)?;
    // 0x2ff90: add r0, r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,12)?)?;
    // 0x2ff94: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2ff98: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x2ff9c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x2ffa0: ldr r5, [pc, #0x2c8]        @ 0xc70 <entry+0xc70>
    c.literal(R::new(5)?,0x30270)?;
    // 0x2ffa4: orr r4, r2, r7
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x2ffa8: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x2ffac: add r2, r2, #67108864
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,67108864)?)?;
    // 0x2ffb0: add r7, r7, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,4)?)?;
    // 0x2ffb4: orr r4, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x2ffb8: add r6, r6, #262144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,262144)?)?;
    // 0x2ffbc: cmp r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,32)?)?;
    // 0x2ffc0: orr r4, r4, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(4)?,R::new(4)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x2ffc4: add r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1024)?)?;
    // 0x2ffc8: orr r5, r4, r5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::new(4)?,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x2ffcc: str r5, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(5)?,R::R0,4)?)?;
    // 0x2ffd0: bne 0x2ffa0 <entry+0x9a0>   @ imm = #-0x38
    c.b(C::Ne,0x2ffa0)?;
    // 0x2ffd4: ldr r4, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,16)?)?;
    // 0x2ffd8: cmp r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,1)?)?;
    // 0x2ffdc: bne 0x2fff4 <entry+0x9f4>   @ imm = #0x10
    c.b(C::Ne,0x2fff4)?;
    // 0x2ffe0: ldrb r1, [r10, #0x2d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,45)?)?;
    // 0x2ffe4: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,8)?)?;
    // 0x2ffe8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x2ffec: movne r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x2fff0: mul r12, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R12,R::R1,R::R0)?)?;
    // 0x2fff4: ldr r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,4)?)?;
    // 0x2fff8: mov r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1048576)?)?;
    // 0x2fffc: ldr r2, [r10, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,36)?)?;
    // 0x30000: mov r3, #49152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,49152)?)?;
    // 0x30004: mov r7, #124
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,124)?)?;
    // 0x30008: and r0, r0, r1, lsl #20
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,20,false)?)?;
    // 0x3000c: ldr r1, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,32)?)?;
    // 0x30010: and r1, r3, r1, lsl #14
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,14,false)?)?;
    // 0x30014: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x30018: and r1, r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R2,15)?)?;
    // 0x3001c: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x30020: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x30024: ldrb r3, [r10, #0x47]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,71)?)?;
    // 0x30028: ldrb r2, [r10, #0x45]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,69)?)?;
    // 0x3002c: ldrb r1, [r10, #0x44]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,68)?)?;
    // 0x30030: ldrb r0, [r10, #0x43]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,67)?)?;
    // 0x30034: and r3, r7, r3, lsl #2
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,2,false)?)?;
    // 0x30038: mov r7, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,128)?)?;
    // 0x3003c: and r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R0,1)?)?;
    // 0x30040: and r2, r7, r2, lsl #7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,7,false)?)?;
    // 0x30044: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x30048: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x3004c: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x30050: orr r7, r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,256)?)?;
    // 0x30054: and r1, r3, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,1,false)?)?;
    // 0x30058: orr r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x3005c: mov r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1024)?)?;
    // 0x30060: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x30064: ldrb r1, [r10, #0x49]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,73)?)?;
    // 0x30068: and r1, r2, r1, lsl #10
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,10,false)?)?;
    // 0x3006c: mov r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2048)?)?;
    // 0x30070: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x30074: ldrb r1, [r10, #0x48]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,72)?)?;
    // 0x30078: and r1, r2, r1, lsl #11
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,11,false)?)?;
    // 0x3007c: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x30080: mov r1, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,65536)?)?;
    // 0x30084: orr r0, r0, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,512)?)?;
    // 0x30088: str r0, [r9, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,4)?)?;
    // 0x3008c: ldr r2, [r10, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,40)?)?;
    // 0x30090: ldrb r0, [r10, #0x35]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,53)?)?;
    // 0x30094: orr r2, r4, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::new(4)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x30098: and r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x3009c: and r2, r2, r7
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x300a0: mov r7, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,256)?)?;
    // 0x300a4: orr r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x300a8: ldrb r2, [r10, #0x37]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,55)?)?;
    // 0x300ac: and r2, r3, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(3)?,R::R2,Shift::Lsl,1,false)?)?;
    // 0x300b0: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x300b4: str r0, [r9, #0x398]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,920)?)?;
    // 0x300b8: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x300bc: ldr r2, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,20)?)?;
    // 0x300c0: ldrb r3, [r10, #0x36]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,54)?)?;
    // 0x300c4: and r0, r0, r2, lsl #4
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,4,false)?)?;
    // 0x300c8: ldrb r2, [r10, #0x34]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(10)?,52)?)?;
    // 0x300cc: and r1, r1, r3, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R1,R::new(3)?,Shift::Lsl,16,false)?)?;
    // 0x300d0: mov r3, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,16711680)?)?;
    // 0x300d4: orr r3, r3, #-16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,4278190080)?)?;
    // 0x300d8: and r2, r7, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::new(7)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x300dc: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x300e0: ldr r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,12)?)?;
    // 0x300e4: and r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R2,1)?)?;
    // 0x300e8: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x300ec: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x300f0: str r0, [r9, #0x39c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,924)?)?;
    // 0x300f4: ldr r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,20)?)?;
    // 0x300f8: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x300fc: orr r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x30100: str r0, [r9, #0x3a0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,928)?)?;
    // 0x30104: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,28)?)?;
    // 0x30108: ldrh r1, [r10, #24]
    c.emit(arm32::load_halfword(C::Al,R::R1,R::new(10)?,24)?)?;
    // 0x3010c: orr r0, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x30110: str r0, [r9, #0x3a4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,932)?)?;
    // 0x30114: mov r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
    // 0x30118: ldr r1, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,12)?)?;
    // 0x3011c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65280)?)?;
    // 0x30120: and r0, r12, r0
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R12,R::R0,Shift::Lsl,0,false)?)?;
    // 0x30124: orr r2, r0, r1, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R2,R::R0,R::R1,Shift::Lsl,16,false)?)?;
    // 0x30128: and r1, r3, r1, lsl #15
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,15,false)?)?;
    // 0x3012c: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x30130: str r2, [r9, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(9)?,988)?)?;
    // 0x30134: str r0, [r9, #0x3e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,992)?)?;
    // 0x30138: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,28)?)?;
    // 0x3013c: str r0, [r9, #0x3e8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,1000)?)?;
    // 0x30140: ldrb r0, [r10, #0x2e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,46)?)?;
    // 0x30144: ldr r1, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,32)?)?;
    // 0x30148: ldr r4, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,32)?)?;
    // 0x3014c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x30150: beq 0x30190 <entry+0xb90>   @ imm = #0x38
    c.b(C::Eq,0x30190)?;
    // 0x30154: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x30158: beq 0x301c0 <entry+0xbc0>   @ imm = #0x60
    c.b(C::Eq,0x301c0)?;
    // 0x3015c: mov r0, #655360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,655360)?)?;
    // 0x30160: mov r5, lr
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // 0x30164: cmp r1, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,5)?)?;
    // 0x30168: orr r0, r0, #134217728
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,134217728)?)?;
    // 0x3016c: beq 0x301dc <entry+0xbdc>   @ imm = #0x68
    c.b(C::Eq,0x301dc)?;
    // 0x30170: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
    // 0x30174: bne 0x301f4 <entry+0xbf4>   @ imm = #0x78
    c.b(C::Ne,0x301f4)?;
    // 0x30178: ldrb r1, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,47)?)?;
    // 0x3017c: mov r2, #34078720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,34078720)?)?;
    // 0x30180: orr r2, r2, #67108864
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,67108864)?)?;
    // 0x30184: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x30188: movne r0, r2
    c.emit(arm32::mov_register(C::Ne,R::R0,R::R2))?;
    // 0x3018c: b 0x30220 <entry+0xc20>   @ imm = #0x8c
    c.b(C::Al,0x30220)?;
    // 0x30190: sub r0, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,4)?)?;
    // 0x30194: mov r5, lr
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // 0x30198: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x3019c: bhs 0x301ac <entry+0xbac>   @ imm = #0x8
    c.b(C::Cs,0x301ac)?;
    // 0x301a0: ldr r1, [pc, #0xcc]         @ 0xc74 <entry+0xc74>
    c.literal(R::R1,0x30274)?;
    // 0x301a4: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x301a8: b 0x30220 <entry+0xc20>   @ imm = #0x70
    c.b(C::Al,0x30220)?;
    // 0x301ac: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x301b0: ldr r1, [pc, #0x9c]         @ 0xc54 <entry+0xc54>
    c.literal(R::R1,0x30254)?;
    // 0x301b4: ldr r2, [pc, #0x9c]         @ 0xc58 <entry+0xc58>
    c.literal(R::R2,0x30258)?;
    // 0x301b8: mov r3, #221
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,221)?)?;
    // 0x301bc: b 0x30204 <entry+0xc04>   @ imm = #0x40
    c.b(C::Al,0x30204)?;
    // 0x301c0: ldrb r1, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,47)?)?;
    // 0x301c4: mov r0, #1310720
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1310720)?)?;
    // 0x301c8: mov r5, lr
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::LR))?;
    // 0x301cc: orr r0, r0, #201326592
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,201326592)?)?;
    // 0x301d0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x301d4: addeq r0, r0, #33554432
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R0,33554432)?)?;
    // 0x301d8: b 0x30220 <entry+0xc20>   @ imm = #0x40
    c.b(C::Al,0x30220)?;
    // 0x301dc: ldrb r1, [r10, #0x2f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,47)?)?;
    // 0x301e0: add r2, r0, #33816576
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,33816576)?)?;
    // 0x301e4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x301e8: addne r2, r0, #131072
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R0,131072)?)?;
    // 0x301ec: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x301f0: b 0x30220 <entry+0xc20>   @ imm = #0x28
    c.b(C::Al,0x30220)?;
    // 0x301f4: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x301f8: ldr r1, [pc, #0x54]         @ 0xc54 <entry+0xc54>
    c.literal(R::R1,0x30254)?;
    // 0x301fc: ldr r2, [pc, #0x54]         @ 0xc58 <entry+0xc58>
    c.literal(R::R2,0x30258)?;
    // 0x30200: mov r3, #250
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,250)?)?;
    // 0x30204: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x30208: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x3020c: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x1580
    c.bl(0x2ec94)?;
    // 0x30210: ldr r1, [pc, #0x44]         @ 0xc5c <entry+0xc5c>
    c.literal(R::R1,0x3025c)?;
    // 0x30214: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x30218: bl 0x2ec94 <entry+0xfffffffffffff694> @ imm = #-0x158c
    c.bl(0x2ec94)?;
    // 0x3021c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x30220: orr r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x30224: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x30228: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x3022c: str r0, [r9, #0x3b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,944)?)?;
    // 0x30230: mov r0, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
    // 0x30234: str r0, [r9, #0x3e4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,996)?)?;
    // 0x30238: ldr r0, [pc, #0x38]         @ 0xc78 <entry+0xc78>
    c.literal(R::R0,0x30278)?;
    // 0x3023c: str r0, [r9, #0x3ac]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,940)?)?;
    // 0x30240: add r0, r9, #948
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(9)?,948)?)?;
    // 0x30244: bl 0x31ec <entry+0xfffffffffffd3bec> @ imm = #-0x2d060
    c.bl(0x31ec)?;
    // 0x30248: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x3024c: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x30250: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x30254)?;
    const CODEC_PLANE_PARAMETERS:[u32;10]=[0x000eb02f,0x000eb04e,0x000e2788,0x000eb1e0,0x000eb1d4,0x000eb1c8,0x000eb1bc,0x03020100,0x000eb1ec,0x00010201];
    for value in CODEC_PLANE_PARAMETERS {c.emit(value)?;}
    c.check(END)
}
pub(super) fn emit(c:&mut Code)->Result<(),String>{c.check(START)?;write_image_plane_parameters(c)?;c.check(END)}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_plane_parameters_are_structured_literals(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let tail=&c.bytes[c.bytes.len()-40..];
        let val=|index:usize|u32::from_le_bytes(tail[4*index..4*index+4].try_into().unwrap());
        assert_eq!(val(0),0x000eb02f);
        assert_eq!(val(7),0x03020100);
        assert_eq!(val(9),0x00010201);
    }
}
