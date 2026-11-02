// SPDX-License-Identifier: GPL-3.0-only
//! Four controller descriptor retrieval and consumption helpers. They compute
//! packed entry sizes, check input queue capacity, update read and write
//! indices, and synchronize owner callbacks around ring operations.
//! The three repeated diagnostic pointers remain literal data words.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x35bec;
const END:usize=0x35ee4;
const QUEUE_DIAGNOSTIC:u32=0x000e_c0b3;
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x35bec: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35bf0: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35bf4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35bf8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35bfc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35c00: beq 0x35c08 <entry+0x77c>   @ imm = #0x0
    c.b(C::Eq,0x35c08)?;
    // 0x35c04: bl 0x2bcc <entry+0xfffffffffffcd740> @ imm = #-0x33040
    c.bl(0x2bcc)?;
    // 0x35c08: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x35c0c: pop {r4, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35c10: b 0x2f1c <entry+0xfffffffffffcda90> @ imm = #-0x32cfc
    c.b(C::Al,0x2f1c)?;
    // 0x35c14: ldr r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,16)?)?;
    // 0x35c18: ldr r3, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,24)?)?;
    // 0x35c1c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35c20: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x35c24: ldrlo r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Cc,true,R::R2,R::R0,12)?)?;
    // 0x35c28: ldrlo r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Cc,true,R::new(3)?,R::R0,20)?)?;
    // 0x35c2c: ldrlo r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Cc,true,R::R0,R::R0,32)?)?;
    // 0x35c30: mlalo r1, r3, r0, r2
    c.emit(arm32::multiply_accumulate(C::Cc,R::R1,R::new(3)?,R::R0,R::R2)?)?;
    // 0x35c34: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x35c38: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x35c3c: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35c40: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35c44: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35c48: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35c4c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35c50: beq 0x35c58 <entry+0x7cc>   @ imm = #0x0
    c.b(C::Eq,0x35c58)?;
    // 0x35c54: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x33068
    c.bl(0x2bf4)?;
    // 0x35c58: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x35c5c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35c60: blt 0x35c84 <entry+0x7f8>   @ imm = #0x1c
    c.b(C::Lt,0x35c84)?;
    // 0x35c64: add r2, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,12)?)?;
    // 0x35c68: ldm r2, {r0, r1, r2}
    c.emit(arm32::transfer_multiple(C::Al,true,R::R2,R::R0.bit() | R::R1.bit() | R::R2.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x35c6c: ldr r3, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,32)?)?;
    // 0x35c70: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x35c74: movne r1, r3
    c.emit(arm32::mov_register(C::Ne,R::R1,R::new(3)?))?;
    // 0x35c78: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x35c7c: mla r5, r1, r2, r0
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R1,R::R2,R::R0)?)?;
    // 0x35c80: b 0x35c88 <entry+0x7fc>   @ imm = #0x0
    c.b(C::Al,0x35c88)?;
    // 0x35c84: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x35c88: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35c8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35c90: beq 0x35c98 <entry+0x80c>   @ imm = #0x0
    c.b(C::Eq,0x35c98)?;
    // 0x35c94: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x33068
    c.bl(0x2c34)?;
    // 0x35c98: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35c9c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35ca0: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35ca4: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x35ca8: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35cac: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35cb0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35cb4: beq 0x35cbc <entry+0x830>   @ imm = #0x0
    c.b(C::Eq,0x35cbc)?;
    // 0x35cb8: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x330cc
    c.bl(0x2bf4)?;
    // 0x35cbc: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x35cc0: ldr r2, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,24)?)?;
    // 0x35cc4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x35cc8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x35ccc: bhs 0x35d14 <entry+0x888>   @ imm = #0x40
    c.b(C::Cs,0x35d14)?;
    // 0x35cd0: ldr r6, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,32)?)?;
    // 0x35cd4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35cd8: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,1)?)?;
    // 0x35cdc: ldr r8, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,12)?)?;
    // 0x35ce0: ldr r7, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,20)?)?;
    // 0x35ce4: add r3, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(6)?,1)?)?;
    // 0x35ce8: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x35cec: movhs r3, r0
    c.emit(arm32::mov_register(C::Cs,R::new(3)?,R::R0))?;
    // 0x35cf0: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x35cf4: mov r1, #34
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,34)?)?;
    // 0x35cf8: str r3, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(4)?,32)?)?;
    // 0x35cfc: str r2, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(4)?,24)?)?;
    // 0x35d00: ldr r2, [pc, #0x38]         @ 0x8b4 <entry+0x8b4>
    c.literal_cond(C::Al,R::R2,0x35d40)?;
    // 0x35d04: movls r0, #1
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x35d08: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x35d0c: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x34564
    c.bl(0x17b0)?;
    // 0x35d10: mla r5, r7, r6, r8
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(7)?,R::new(6)?,R::new(8)?)?)?;
    // 0x35d14: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x35d18: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35d1c: beq 0x35d28 <entry+0x89c>   @ imm = #0x4
    c.b(C::Eq,0x35d28)?;
    // 0x35d20: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // 0x35d24: bl 0x2d20 <entry+0xfffffffffffcd894> @ imm = #-0x3300c
    c.bl(0x2d20)?;
    // 0x35d28: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35d2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35d30: beq 0x35d38 <entry+0x8ac>   @ imm = #0x0
    c.b(C::Eq,0x35d38)?;
    // 0x35d34: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x33108
    c.bl(0x2c34)?;
    // 0x35d38: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35d3c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x35d40)?;
    c.emit(QUEUE_DIAGNOSTIC)?;
    c.check(0x35d44)?;
    // 0x35d44: ldr r1, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,24)?)?;
    // 0x35d48: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x35d4c: ldrge r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Ge,true,R::R1,R::R0,12)?)?;
    // 0x35d50: ldrge r2, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Ge,true,R::R2,R::R0,20)?)?;
    // 0x35d54: ldrge r3, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Ge,true,R::new(3)?,R::R0,28)?)?;
    // 0x35d58: mlage r0, r2, r3, r1
    c.emit(arm32::multiply_accumulate(C::Ge,R::R0,R::R2,R::new(3)?,R::R1)?)?;
    // 0x35d5c: bxge lr
    c.emit(arm32::bx(C::Ge,R::LR))?;
    // 0x35d60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35d64: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x35d68: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35d6c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x35d70: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35d74: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35d78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35d7c: beq 0x35d84 <entry+0x8f8>   @ imm = #0x0
    c.b(C::Eq,0x35d84)?;
    // 0x35d80: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x33194
    c.bl(0x2bf4)?;
    // 0x35d84: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x35d88: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35d8c: blt 0x35dd4 <entry+0x948>   @ imm = #0x40
    c.b(C::Lt,0x35dd4)?;
    // 0x35d90: ldr r3, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,28)?)?;
    // 0x35d94: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x35d98: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x35d9c: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x35da0: add r5, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(3)?,1)?)?;
    // 0x35da4: mla r6, r2, r3, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R2,R::new(3)?,R::R1)?)?;
    // 0x35da8: ldr r2, [pc, #0x40]         @ 0x964 <entry+0x964>
    c.literal_cond(C::Al,R::R2,0x35df0)?;
    // 0x35dac: str r5, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,28)?)?;
    // 0x35db0: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x35db4: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x35db8: movhs r1, #0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35dbc: strhs r1, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Cs,false,R::R1,R::new(4)?,28)?)?;
    // 0x35dc0: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // 0x35dc4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x35dc8: mov r1, #848
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,848)?)?;
    // 0x35dcc: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x34624
    c.bl(0x17b0)?;
    // 0x35dd0: b 0x35dd8 <entry+0x94c>   @ imm = #0x0
    c.b(C::Al,0x35dd8)?;
    // 0x35dd4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x35dd8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35ddc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35de0: beq 0x35de8 <entry+0x95c>   @ imm = #0x0
    c.b(C::Eq,0x35de8)?;
    // 0x35de4: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x331b8
    c.bl(0x2c34)?;
    // 0x35de8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x35dec: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x35df0)?;
    c.emit(QUEUE_DIAGNOSTIC)?;
    c.check(0x35df4)?;
    // 0x35df4: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35df8: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35dfc: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35e00: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35e04: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35e08: beq 0x35e10 <entry+0x984>   @ imm = #0x0
    c.b(C::Eq,0x35e10)?;
    // 0x35e0c: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x33220
    c.bl(0x2bf4)?;
    // 0x35e10: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x35e14: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x35e18: blt 0x35e84 <entry+0x9f8>   @ imm = #0x64
    c.b(C::Lt,0x35e84)?;
    // 0x35e1c: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x35e20: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x35e24: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x35e28: mla r5, r2, r0, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::R2,R::R0,R::R1)?)?;
    // 0x35e2c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x35e30: beq 0x35e44 <entry+0x9b8>   @ imm = #0xc
    c.b(C::Eq,0x35e44)?;
    // 0x35e34: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35e38: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35e3c: bl 0x31ec <entry+0xfffffffffffcdd60> @ imm = #-0x32c58
    c.bl(0x31ec)?;
    // 0x35e40: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x35e44: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x35e48: ldr r2, [pc, #0x50]         @ 0xa14 <entry+0xa14>
    c.literal_cond(C::Al,R::R2,0x35ea0)?;
    // 0x35e4c: str r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,28)?)?;
    // 0x35e50: ldr r1, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,16)?)?;
    // 0x35e54: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x35e58: movhs r0, #0
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35e5c: strhs r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Cs,false,R::R0,R::new(4)?,28)?)?;
    // 0x35e60: ldr r1, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,24)?)?;
    // 0x35e64: sub r0, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R1,1)?)?;
    // 0x35e68: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x35e6c: mov r1, #876
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,876)?)?;
    // 0x35e70: str r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,24)?)?;
    // 0x35e74: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x35e78: movgt r0, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x35e7c: bl 0x17b0 <entry+0xfffffffffffcc324> @ imm = #-0x346d4
    c.bl(0x17b0)?;
    // 0x35e80: b 0x35e88 <entry+0x9fc>   @ imm = #0x0
    c.b(C::Al,0x35e88)?;
    // 0x35e84: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x35e88: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35e8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35e90: beq 0x35e98 <entry+0xa0c>   @ imm = #0x0
    c.b(C::Eq,0x35e98)?;
    // 0x35e94: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x33268
    c.bl(0x2c34)?;
    // 0x35e98: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x35e9c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x35ea0)?;
    c.emit(QUEUE_DIAGNOSTIC)?;
    c.check(0x35ea4)?;
    // 0x35ea4: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35ea8: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x35eac: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x35eb0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35eb4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35eb8: beq 0x35ed8 <entry+0xa4c>   @ imm = #0x18
    c.b(C::Eq,0x35ed8)?;
    // 0x35ebc: bl 0x2bf4 <entry+0xfffffffffffcd768> @ imm = #-0x332d0
    c.bl(0x2bf4)?;
    // 0x35ec0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x35ec4: ldr r4, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,24)?)?;
    // 0x35ec8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35ecc: beq 0x35edc <entry+0xa50>   @ imm = #0x8
    c.b(C::Eq,0x35edc)?;
    // 0x35ed0: bl 0x2c34 <entry+0xfffffffffffcd7a8> @ imm = #-0x332a4
    c.bl(0x2c34)?;
    // 0x35ed4: b 0x35edc <entry+0xa50>   @ imm = #0x0
    c.b(C::Al,0x35edc)?;
    // 0x35ed8: ldr r4, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(4)?,24)?)?;
    // 0x35edc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x35ee0: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(END)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn ring_descriptor_helpers_are_complete(){
    let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
    for at in [0x35d40,0x35df0,0x35ea0] {
      let i=at-START;
      assert_eq!(u32::from_le_bytes(c.bytes[i..i+4].try_into().unwrap()), QUEUE_DIAGNOSTIC);
    }
 }
}
