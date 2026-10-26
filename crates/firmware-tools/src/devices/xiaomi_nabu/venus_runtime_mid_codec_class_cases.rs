// SPDX-License-Identifier: GPL-3.0-only
//! Continuation of Venus codec-class controller event dispatch. The 7-way
//! branch table and 25-word controller/diagnostic constant pool are typed
//! data, not copied proprietary instruction words. Code uses ARM A32 encoders.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x32c9c;
const END:usize=0x33488;

/// Controller codec cases at runtime [0x32c9c,0x32d18).
fn select_frame_buffer_action(c:&mut Code)->Result<(),String>{
    c.check(0x32c9c)?;
    // 0x32c9c: mov r1, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,36)?)?;
    // 0x32ca0: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x32ca4: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,72)?)?;
    // 0x32ca8: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,0)?)?;
    // 0x32cac: str r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,76)?)?;
    // 0x32cb0: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x32cb4: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(8)?,0)?)?;
    // 0x32cb8: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // 0x32cbc: str r1, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,84)?)?;
    // 0x32cc0: ldr r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,12)?)?;
    // 0x32cc4: str r1, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,92)?)?;
    // 0x32cc8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x32ccc: str r1, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,80)?)?;
    // 0x32cd0: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x32cd4: str r1, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,88)?)?;
    // 0x32cd8: ldr r2, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,16)?)?;
    // 0x32cdc: str r2, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,96)?)?;
    // 0x32ce0: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // 0x32ce4: str r1, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,100)?)?;
    // 0x32ce8: ldr r1, [pc, #0x790]        @ 0x1700 <entry+0x1700>
    c.literal(R::R1,0x33480)?;
    // 0x32cec: ldr r0, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,24)?)?;
    // 0x32cf0: str r0, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,104)?)?;
    // 0x32cf4: b 0x33210 <entry+0x1490>  @ imm = #0x514
    c.b(C::Al,0x33210)?;
    // 0x32cf8: ldr r0, [pc, #0x73c]        @ 0x16bc <entry+0x16bc>
    c.literal(R::R0,0x3343c)?;
    // 0x32cfc: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // 0x32d00: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32d04: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x32d08: bhi 0x32f38 <entry+0x11b8>  @ imm = #0x228
    c.b(C::Hi,0x32f38)?;
    // 0x32d0c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x32d10: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x32d14: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x32d18)
}

/// Controller codec cases at runtime [0x32d34,0x33424).
fn complete_class_command_dispatch(c:&mut Code)->Result<(),String>{
    c.check(0x32d34)?;
    // 0x32d34: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x32d38: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x32d3c: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x32d40: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x32d44: ldr r2, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,360)?)?;
    // 0x32d48: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x32d4c: beq 0x3339c <entry+0x161c>  @ imm = #0x648
    c.b(C::Eq,0x3339c)?;
    // 0x32d50: ldr r0, [r0, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,364)?)?;
    // 0x32d54: ldr r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // 0x32d58: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x32d5c: blx r2
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // 0x32d60: b 0x3339c <entry+0x161c>  @ imm = #0x634
    c.b(C::Al,0x3339c)?;
    // 0x32d64: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32d68: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x32d6c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32d70: beq 0x33234 <entry+0x14b4>  @ imm = #0x4bc
    c.b(C::Eq,0x33234)?;
    // 0x32d74: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x32d78: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x32d7c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32d80: beq 0x33260 <entry+0x14e0>  @ imm = #0x4d8
    c.b(C::Eq,0x33260)?;
    // 0x32d84: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x32d88: orr r0, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,69632)?)?;
    // 0x32d8c: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32d90: bne 0x32f38 <entry+0x11b8>  @ imm = #0x1a0
    c.b(C::Ne,0x32f38)?;
    // 0x32d94: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x32d98: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x32d9c: bl 0x34bc4 <entry+0x2e44>  @ imm = #0x1e20
    c.bl(0x34bc4)?;
    // 0x32da0: b 0x3339c <entry+0x161c>  @ imm = #0x5f4
    c.b(C::Al,0x3339c)?;
    // 0x32da4: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,36)?)?;
    // 0x32da8: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x32dac: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x32db0: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,8)?)?;
    // 0x32db4: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x32db8: b 0x3339c <entry+0x161c>  @ imm = #0x5dc
    c.b(C::Al,0x3339c)?;
    // 0x32dbc: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x32dc0: ldr r2, [pc, #0x684]        @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x32dc4: mov r1, #356
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,356)?)?;
    // 0x32dc8: ldr r0, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,28)?)?;
    // 0x32dcc: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x31624
    c.bl(0x17b0)?;
    // 0x32dd0: mov r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,44)?)?;
    // 0x32dd4: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x32dd8: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x32ddc: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x32de0: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x32de4: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x32de8: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x32dec: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x32df0: ldr r2, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,12)?)?;
    // 0x32df4: str r2, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,92)?)?;
    // 0x32df8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x32dfc: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x32e00: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x32e04: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x32e08: ldr r5, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,16)?)?;
    // 0x32e0c: str r5, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,96)?)?;
    // 0x32e10: ldr r1, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,20)?)?;
    // 0x32e14: str r1, [sp, #0x64]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,100)?)?;
    // 0x32e18: ldr r3, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,24)?)?;
    // 0x32e1c: str r3, [sp, #0x68]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,104)?)?;
    // 0x32e20: ldr r1, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,28)?)?;
    // 0x32e24: str r1, [sp, #0x6c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,108)?)?;
    // 0x32e28: ldr r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // 0x32e2c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x32e30: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32e34: str r1, [sp, #0x70]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,112)?)?;
    // 0x32e38: ldr r1, [pc, #0x63c]        @ 0x16fc <entry+0x16fc>
    c.literal(R::R1,0x3347c)?;
    // 0x32e3c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x41b0
    c.bl(0x2ec94)?;
    // 0x32e40: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32e44: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x32e48: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x41bc
    c.bl(0x2ec94)?;
    // 0x32e4c: ldr r1, [pc, #0x62c]        @ 0x1700 <entry+0x1700>
    c.literal(R::R1,0x33480)?;
    // 0x32e50: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32e54: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x32e58: b 0x33214 <entry+0x1494>  @ imm = #0x3b4
    c.b(C::Al,0x33214)?;
    // 0x32e5c: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x32e60: ldr r2, [pc, #0x5e4]        @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x32e64: mov r1, #33
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,33)?)?;
    // 0x32e68: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x32e6c: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x32e70: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x316c8
    c.bl(0x17b0)?;
    // 0x32e74: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x32e78: ldr r1, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,12)?)?;
    // 0x32e7c: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,4)?)?;
    // 0x32e80: b 0x3339c <entry+0x161c>  @ imm = #0x514
    c.b(C::Al,0x3339c)?;
    // 0x32e84: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x32e88: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x32e8c: cmp r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,64)?)?;
    // 0x32e90: bne 0x3312c <entry+0x13ac>  @ imm = #0x294
    c.b(C::Ne,0x3312c)?;
    // 0x32e94: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x32e98: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,16)?)?;
    // 0x32e9c: ldr r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // 0x32ea0: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32ea4: bl 0x2038 <entry+0xfffffffffffd02b8> @ imm = #-0x30e74
    c.bl(0x2038)?;
    // 0x32ea8: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x32eac: ldr r1, [pc, #0x5c0]        @ 0x16f4 <entry+0x16f4>
    c.literal(R::R1,0x33474)?;
    // 0x32eb0: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x32eb4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x32eb8: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x32ebc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32ec0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4234
    c.bl(0x2ec94)?;
    // 0x32ec4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32ec8: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x32ecc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4240
    c.bl(0x2ec94)?;
    // 0x32ed0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32ed4: orr r0, r0, #1179648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1179648)?)?;
    // 0x32ed8: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x32edc: mov r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,32)?)?;
    // 0x32ee0: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x32ee4: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x32ee8: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x32eec: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32ef0: ldr r1, [r4, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,28)?)?;
    // 0x32ef4: ldr r2, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,32)?)?;
    // 0x32ef8: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x32efc: bhi 0x32f30 <entry+0x11b0>  @ imm = #0x2c
    c.b(C::Hi,0x32f30)?;
    // 0x32f00: add r5, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(4)?,12)?)?;
    // 0x32f04: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,8)?)?;
    // 0x32f08: ldm r5, {r0, r3, r5}
    c.emit(arm32::transfer_multiple(C::Al,true,R::new(5)?,R::R0.bit()|R::new(3)?.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x32f0c: add r0, r5, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32f10: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x32f14: bl 0x471c <entry+0xfffffffffffd299c> @ imm = #-0x2e800
    c.bl(0x471c)?;
    // 0x32f18: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x32f1c: ldr r2, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,20)?)?;
    // 0x32f20: ldr r1, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,32)?)?;
    // 0x32f24: add r0, r2, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32f28: bl 0x2014 <entry+0xfffffffffffd0294> @ imm = #-0x30f1c
    c.bl(0x2014)?;
    // 0x32f2c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32f30: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x32f34: b 0x33224 <entry+0x14a4>  @ imm = #0x2e8
    c.b(C::Al,0x33224)?;
    // 0x32f38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32f3c: mov r1, #1004
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1004)?)?;
    // 0x32f40: b 0x33394 <entry+0x1614>  @ imm = #0x44c
    c.b(C::Al,0x33394)?;
    // 0x32f44: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x32f48: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x32f4c: cmp r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,16)?)?;
    // 0x32f50: bne 0x332bc <entry+0x153c>  @ imm = #0x364
    c.b(C::Ne,0x332bc)?;
    // 0x32f54: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x32f58: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x32f5c: bne 0x33364 <entry+0x15e4>  @ imm = #0x400
    c.b(C::Ne,0x33364)?;
    // 0x32f60: bl 0x2fa34 <entry+0xffffffffffffdcb4> @ imm = #-0x3534
    c.bl(0x2fa34)?;
    // 0x32f64: b 0x3339c <entry+0x161c>  @ imm = #0x430
    c.b(C::Al,0x3339c)?;
    // 0x32f68: ldr r0, [r6, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,352)?)?;
    // 0x32f6c: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x32f70: bl 0x2aad8 <entry+0xffffffffffff8d58> @ imm = #-0x84a0
    c.bl(0x2aad8)?;
    // 0x32f74: b 0x3339c <entry+0x161c>  @ imm = #0x420
    c.b(C::Al,0x3339c)?;
    // 0x32f78: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x32f7c: ldr r2, [pc, #0x4c8]        @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x32f80: mov r1, #145
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,145)?)?;
    // 0x32f84: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x32f88: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x32f8c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x32f90: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32f94: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x317ec
    c.bl(0x17b0)?;
    // 0x32f98: ldr r5, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,8)?)?;
    // 0x32f9c: add r0, r4, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,12)?)?;
    // 0x32fa0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x32fa4: ldr r1, [r4], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R1,R::new(4)?,4)?)?;
    // 0x32fa8: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,6)?)?;
    // 0x32fac: bne 0x33028 <entry+0x12a8>  @ imm = #0x74
    c.b(C::Ne,0x33028)?;
    // 0x32fb0: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,0)?)?;
    // 0x32fb4: ldr r1, [pc, #0x4a4]        @ 0x16e0 <entry+0x16e0>
    c.literal(R::R1,0x33460)?;
    // 0x32fb8: str r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,76)?)?;
    // 0x32fbc: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x32fc0: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x32fc4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32fc8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x433c
    c.bl(0x2ec94)?;
    // 0x32fcc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32fd0: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x32fd4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4348
    c.bl(0x2ec94)?;
    // 0x32fd8: mov r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,7)?)?;
    // 0x32fdc: ldr r1, [pc, #0x480]        @ 0x16e4 <entry+0x16e4>
    c.literal(R::R1,0x33464)?;
    // 0x32fe0: mov r2, #128
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,128)?)?;
    // 0x32fe4: orr r0, r0, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,131072)?)?;
    // 0x32fe8: add r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3)?)?;
    // 0x32fec: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x32ff0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32ff4: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x32ff8: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x32ffc: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x33000: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x33004: bl 0x471c <entry+0xfffffffffffd299c> @ imm = #-0x2e8f0
    c.bl(0x471c)?;
    // 0x33008: mov r0, #144
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,144)?)?;
    // 0x3300c: add r1, sp, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,72)?)?;
    // 0x33010: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x33014: mov r0, #152
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,152)?)?;
    // 0x33018: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x3301c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x33020: bl 0x33d38 <entry+0x1fb8>  @ imm = #0xd10
    c.bl(0x33d38)?;
    // 0x33024: b 0x33068 <entry+0x12e8>  @ imm = #0x3c
    c.b(C::Al,0x33068)?;
    // 0x33028: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,4)?)?;
    // 0x3302c: ldr r1, [pc, #0x424]        @ 0x16d8 <entry+0x16d8>
    c.literal(R::R1,0x33458)?;
    // 0x33030: ldr r2, [pc, #0x424]        @ 0x16dc <entry+0x16dc>
    c.literal(R::R2,0x3345c)?;
    // 0x33034: mov r3, #177
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,177)?)?;
    // 0x33038: orr r3, r3, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,256)?)?;
    // 0x3303c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x33040: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x33044: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x43b8
    c.bl(0x2ec94)?;
    // 0x33048: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x3304c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33050: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x43c4
    c.bl(0x2ec94)?;
    // 0x33054: ldr r2, [pc, #0x3f0]        @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x33058: mov r1, #178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,178)?)?;
    // 0x3305c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33060: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x33064: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x318bc
    c.bl(0x17b0)?;
    // 0x33068: subs r5, r5, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::new(5)?,R::new(5)?,1,true)?)?;
    // 0x3306c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x33070: bne 0x32fa4 <entry+0x1224>  @ imm = #-0xd4
    c.b(C::Ne,0x32fa4)?;
    // 0x33074: b 0x3339c <entry+0x161c>  @ imm = #0x320
    c.b(C::Al,0x3339c)?;
    // 0x33078: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x3307c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x33080: bl 0x34088 <entry+0x2308>  @ imm = #0x1000
    c.bl(0x34088)?;
    // 0x33084: b 0x3339c <entry+0x161c>  @ imm = #0x310
    c.b(C::Al,0x3339c)?;
    // 0x33088: ldr r3, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,20)?)?;
    // 0x3308c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x33090: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x33094: mov r2, #7
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
    // 0x33098: orr r2, r2, #131072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,131072)?)?;
    // 0x3309c: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(3)?,0)?)?;
    // 0x330a0: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
    // 0x330a4: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x330a8: mov r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,16)?)?;
    // 0x330ac: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x330b0: ldr r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,28)?)?;
    // 0x330b4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x330b8: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x330bc: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,16)?)?;
    // 0x330c0: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x330c4: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x330c8: mov r2, #24
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
    // 0x330cc: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x330d0: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,0)?)?;
    // 0x330d4: ldr r5, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(8)?,0)?)?;
    // 0x330d8: ldr r2, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,32)?)?;
    // 0x330dc: str r5, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R2,0)?)?;
    // 0x330e0: add r2, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(5)?,1)?)?;
    // 0x330e4: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(8)?,0)?)?;
    // 0x330e8: ldr r0, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,360)?)?;
    // 0x330ec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x330f0: moveq r0, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x330f4: orreq r0, r0, #4096
    c.emit(arm32::data_immediate(C::Eq,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x330f8: streq r0, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(3)?,0)?)?;
    // 0x330fc: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x33100: bl 0x33d38 <entry+0x1fb8>  @ imm = #0xc30
    c.bl(0x33d38)?;
    // 0x33104: b 0x3339c <entry+0x161c>  @ imm = #0x290
    c.b(C::Al,0x3339c)?;
    // 0x33108: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x3310c: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,8)?)?;
    // 0x33110: ldr r2, [pc, #0x350]        @ 0x16e8 <entry+0x16e8>
    c.literal(R::R2,0x33468)?;
    // 0x33114: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x33118: bne 0x3331c <entry+0x159c>  @ imm = #0x1fc
    c.b(C::Ne,0x3331c)?;
    // 0x3311c: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x33120: ldr r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,36)?)?;
    // 0x33124: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x33128: b 0x3339c <entry+0x161c>  @ imm = #0x26c
    c.b(C::Al,0x3339c)?;
    // 0x3312c: mov r1, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,6)?)?;
    // 0x33130: ldr r0, [r6, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,352)?)?;
    // 0x33134: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x33138: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // 0x3313c: orr r1, r1, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1114112)?)?;
    // 0x33140: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x33144: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x33148: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x3314c: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x33150: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x33154: bl 0x2a990 <entry+0xffffffffffff8c10> @ imm = #-0x87cc
    c.bl(0x2a990)?;
    // 0x33158: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x3315c: ldr r1, [pc, #0x308]        @ 0x16ec <entry+0x16ec>
    c.literal(R::R1,0x3346c)?;
    // 0x33160: ldr r2, [pc, #0x308]        @ 0x16f0 <entry+0x16f0>
    c.literal(R::R2,0x33470)?;
    // 0x33164: mov r3, #166
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,166)?)?;
    // 0x33168: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x3316c: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x33170: mov r0, #64
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,64)?)?;
    // 0x33174: b 0x33300 <entry+0x1580>  @ imm = #0x184
    c.b(C::Al,0x33300)?;
    // 0x33178: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x3317c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x33180: bl 0x34ca0 <entry+0x2f20>  @ imm = #0x1b18
    c.bl(0x34ca0)?;
    // 0x33184: b 0x3339c <entry+0x161c>  @ imm = #0x210
    c.b(C::Al,0x3339c)?;
    // 0x33188: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x3318c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x33190: bl 0x3498c <entry+0x2c0c>  @ imm = #0x17f4
    c.bl(0x3498c)?;
    // 0x33194: b 0x3339c <entry+0x161c>  @ imm = #0x200
    c.b(C::Al,0x3339c)?;
    // 0x33198: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x3319c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x331a0: bl 0x34910 <entry+0x2b90>  @ imm = #0x1768
    c.bl(0x34910)?;
    // 0x331a4: b 0x3339c <entry+0x161c>  @ imm = #0x1f0
    c.b(C::Al,0x3339c)?;
    // 0x331a8: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x331ac: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x331b0: bl 0x343bc <entry+0x263c>  @ imm = #0x1204
    c.bl(0x343bc)?;
    // 0x331b4: b 0x3339c <entry+0x161c>  @ imm = #0x1e0
    c.b(C::Al,0x3339c)?;
    // 0x331b8: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x331bc: ldr r2, [pc, #0x288]        @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x331c0: mov r1, #552
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,552)?)?;
    // 0x331c4: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x331c8: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x31a20
    c.bl(0x17b0)?;
    // 0x331cc: mov r0, #68
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,68)?)?;
    // 0x331d0: ldr r1, [pc, #0x2a0]        @ 0x16f8 <entry+0x16f8>
    c.literal(R::R1,0x33478)?;
    // 0x331d4: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,72)?)?;
    // 0x331d8: ldr r2, [r8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(8)?,0)?)?;
    // 0x331dc: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R2,1)?)?;
    // 0x331e0: str r2, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::SP,76)?)?;
    // 0x331e4: str r0, [r8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(8)?,0)?)?;
    // 0x331e8: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x331ec: str r0, [sp, #0x54]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,84)?)?;
    // 0x331f0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x331f4: str r0, [sp, #0x50]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,80)?)?;
    // 0x331f8: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x331fc: str r0, [sp, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,92)?)?;
    // 0x33200: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x33204: str r0, [sp, #0x60]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,96)?)?;
    // 0x33208: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,8)?)?;
    // 0x3320c: str r0, [sp, #0x58]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,88)?)?;
    // 0x33210: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33214: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4588
    c.bl(0x2ec94)?;
    // 0x33218: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3321c: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33220: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4594
    c.bl(0x2ec94)?;
    // 0x33224: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x33228: add r1, sp, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,72)?)?;
    // 0x3322c: bl 0x33d38 <entry+0x1fb8>  @ imm = #0xb04
    c.bl(0x33d38)?;
    // 0x33230: b 0x3339c <entry+0x161c>  @ imm = #0x164
    c.b(C::Al,0x3339c)?;
    // 0x33234: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x33238: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x3323c: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33240: add r2, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33244: ldr r0, [r2, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,360)?)?;
    // 0x33248: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3324c: beq 0x3339c <entry+0x161c>  @ imm = #0x148
    c.b(C::Eq,0x3339c)?;
    // 0x33250: ldr r2, [r2, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,364)?)?;
    // 0x33254: ldr r2, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,12)?)?;
    // 0x33258: blx r2
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // 0x3325c: b 0x3339c <entry+0x161c>  @ imm = #0x138
    c.b(C::Al,0x3339c)?;
    // 0x33260: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,68)?)?;
    // 0x33264: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x33268: add r0, r0, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R0,Shift::Lsl,2,false)?)?;
    // 0x3326c: add r2, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(6)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x33270: ldr r0, [r2, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R2,360)?)?;
    // 0x33274: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33278: beq 0x3339c <entry+0x161c>  @ imm = #0x11c
    c.b(C::Eq,0x3339c)?;
    // 0x3327c: ldr r2, [r2, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,364)?)?;
    // 0x33280: ldr r2, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R2,8)?)?;
    // 0x33284: blx r2
    c.emit(arm32::branch_exchange_with_link(C::Al,R::R2)?)?;
    // 0x33288: b 0x3339c <entry+0x161c>  @ imm = #0x10c
    c.b(C::Al,0x3339c)?;
    // 0x3328c: ldr r1, [pc, #0x1b4]        @ 0x16c8 <entry+0x16c8>
    c.literal(R::R1,0x33448)?;
    // 0x33290: ldr r2, [pc, #0x1b8]        @ 0x16d0 <entry+0x16d0>
    c.literal(R::R2,0x33450)?;
    // 0x33294: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x33298: mov r3, #484
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,484)?)?;
    // 0x3329c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4610
    c.bl(0x2ec94)?;
    // 0x332a0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x332a4: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x332a8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x461c
    c.bl(0x2ec94)?;
    // 0x332ac: mov r1, #229
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,229)?)?;
    // 0x332b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x332b4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x332b8: b 0x33394 <entry+0x1614>  @ imm = #0xd4
    c.b(C::Al,0x33394)?;
    // 0x332bc: mov r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,4)?)?;
    // 0x332c0: ldr r0, [r6, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,352)?)?;
    // 0x332c4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x332c8: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // 0x332cc: orr r1, r1, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65536)?)?;
    // 0x332d0: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x332d4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x332d8: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x332dc: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x332e0: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x332e4: bl 0x2a990 <entry+0xffffffffffff8c10> @ imm = #-0x895c
    c.bl(0x2a990)?;
    // 0x332e8: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x332ec: ldr r1, [pc, #0x14c]        @ 0x16c0 <entry+0x16c0>
    c.literal(R::R1,0x33440)?;
    // 0x332f0: ldr r2, [pc, #0x14c]        @ 0x16c4 <entry+0x16c4>
    c.literal(R::R2,0x33444)?;
    // 0x332f4: mov r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,512)?)?;
    // 0x332f8: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
    // 0x332fc: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x33300: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
    // 0x33304: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x33308: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x467c
    c.bl(0x2ec94)?;
    // 0x3330c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x33310: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33314: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4688
    c.bl(0x2ec94)?;
    // 0x33318: b 0x3339c <entry+0x161c>  @ imm = #0x7c
    c.b(C::Al,0x3339c)?;
    // 0x3331c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33320: mov r1, #249
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,249)?)?;
    // 0x33324: b 0x33394 <entry+0x1614>  @ imm = #0x68
    c.b(C::Al,0x33394)?;
    // 0x33328: ldr r0, [r6, #0x160]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,352)?)?;
    // 0x3332c: str r1, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,0)?)?;
    // 0x33330: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x33334: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x33338: mov r3, #13
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,13)?)?;
    // 0x3333c: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
    // 0x33340: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,8)?)?;
    // 0x33344: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
    // 0x33348: bl 0x2a990 <entry+0xffffffffffff8c10> @ imm = #-0x89c0
    c.bl(0x2a990)?;
    // 0x3334c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x33350: ldr r1, [pc, #0xfc]         @ 0x16d4 <entry+0x16d4>
    c.literal(R::R1,0x33454)?;
    // 0x33354: ldr r2, [pc, #0xf4]         @ 0x16d0 <entry+0x16d0>
    c.literal(R::R2,0x33450)?;
    // 0x33358: mov r3, #476
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,476)?)?;
    // 0x3335c: stm sp, {r0, r5}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::new(5)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x33360: b 0x33304 <entry+0x1584>  @ imm = #-0x64
    c.b(C::Al,0x33304)?;
    // 0x33364: ldr r1, [pc, #0xdc]         @ 0x16c8 <entry+0x16c8>
    c.literal(R::R1,0x33448)?;
    // 0x33368: ldr r2, [pc, #0xd4]         @ 0x16c4 <entry+0x16c4>
    c.literal(R::R2,0x33444)?;
    // 0x3336c: mov r3, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,14)?)?;
    // 0x33370: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x33374: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x33378: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x46ec
    c.bl(0x2ec94)?;
    // 0x3337c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x33380: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33384: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x46f8
    c.bl(0x2ec94)?;
    // 0x33388: mov r1, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15)?)?;
    // 0x3338c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33390: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,512)?)?;
    // 0x33394: ldr r2, [pc, #0xb0]         @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x33398: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x31bf0
    c.bl(0x17b0)?;
    // 0x3339c: ldr r1, [pc, #0xe0]         @ 0x1704 <entry+0x1704>
    c.literal(R::R1,0x33484)?;
    // 0x333a0: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x333a4: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x333a8: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x471c
    c.bl(0x2ec94)?;
    // 0x333ac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x333b0: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x333b4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4728
    c.bl(0x2ec94)?;
    // 0x333b8: ldr r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,56)?)?;
    // 0x333bc: bl 0x80af0 <entry+0x4ed70> @ imm = #0x4d72c
    c.bl(0x80af0)?;
    // 0x333c0: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x333c4: ldr r4, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,48)?)?;
    // 0x333c8: add r5, sp, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,60)?)?;
    // 0x333cc: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x333d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x333d4: beq 0x3340c <entry+0x168c>  @ imm = #0x30
    c.b(C::Eq,0x3340c)?;
    // 0x333d8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x333dc: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x333e0: bl 0x80994 <entry+0x4ec14> @ imm = #0x4d5ac
    c.bl(0x80994)?;
    // 0x333e4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x333e8: bne 0x3340c <entry+0x168c>  @ imm = #0x1c
    c.b(C::Ne,0x3340c)?;
    // 0x333ec: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x333f0: bl 0x3489c <entry+0x2b1c>  @ imm = #0x14a4
    c.bl(0x3489c)?;
    // 0x333f4: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x333f8: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x333fc: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x4770
    c.bl(0x2ec94)?;
    // 0x33400: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x33404: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x33408: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x477c
    c.bl(0x2ec94)?;
    // 0x3340c: ldr r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,56)?)?;
    // 0x33410: add r1, sp, #68
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,68)?)?;
    // 0x33414: bl 0x80a94 <entry+0x4ed14> @ imm = #0x4d678
    c.bl(0x80a94)?;
    // 0x33418: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x3341c: beq 0x32b0c <entry+0xd8c>   @ imm = #-0x918
    c.b(C::Eq,0x32b0c)?;
    // 0x33420: b 0x32ab0 <entry+0xd30>   @ imm = #-0x978
    c.b(C::Al,0x32ab0)?;
    c.check(0x33424)
}

const FRAME_BUFFER_BRANCHES:[u32;7]=[
    0x00032d34,  // runtime 0x32d18
    0x00032d34,  // runtime 0x32d1c
    0x00033178,  // runtime 0x32d20
    0x00033188,  // runtime 0x32d24
    0x00033198,  // runtime 0x32d28
    0x000331a8,  // runtime 0x32d2c
    0x000331b8,  // runtime 0x32d30
];

const CONTROLLER_CLASS_CONSTANTS:[u32;25]=[
    0xffffeb98,  // runtime 0x33424
    0xffffeab8,  // runtime 0x33428
    0x000eb9c2,  // runtime 0x3342c
    0x000e2788,  // runtime 0x33430
    0x000eb9f2,  // runtime 0x33434
    0xfffefffd,  // runtime 0x33438
    0xffeeefff,  // runtime 0x3343c
    0x000e8f9b,  // runtime 0x33440
    0x000ebb1b,  // runtime 0x33444
    0x000ebaee,  // runtime 0x33448
    0x000eb784,  // runtime 0x3344c
    0x000ebadc,  // runtime 0x33450
    0x000e8ee1,  // runtime 0x33454
    0x000ea48e,  // runtime 0x33458
    0x000eba5f,  // runtime 0x3345c
    0x000eba2e,  // runtime 0x33460
    0x0010ded8,  // runtime 0x33464
    0x02000101,  // runtime 0x33468
    0x000ebb31,  // runtime 0x3346c
    0x000ebb82,  // runtime 0x33470
    0x000ebb91,  // runtime 0x33474
    0x000ebbb1,  // runtime 0x33478
    0x000eba8e,  // runtime 0x3347c
    0x000eba71,  // runtime 0x33480
    0x000eba0b,  // runtime 0x33484
];

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    select_frame_buffer_action(c)?;
    c.check(0x32d18)?;
    for data in FRAME_BUFFER_BRANCHES {c.emit(data)?;}
    c.check(0x32d34)?;
    complete_class_command_dispatch(c)?;
    c.check(0x33424)?;
    for data in CONTROLLER_CLASS_CONSTANTS {c.emit(data)?;}
    c.check(0x33488)?;
    c.check(END)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_class_dispatch_finishes_with_data_pool(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let at=|a:usize|u32::from_le_bytes(c.bytes[a-START..a-START+4].try_into().unwrap());
        assert_eq!(at(0x32d18),0x32d34);
        assert_eq!(at(0x32d30),0x331b8);
        assert_eq!(at(0x33424),0xffffeb98);
        assert_eq!(at(0x33464),0x0010ded8);
        assert_eq!(at(0x33484),CONTROLLER_CLASS_CONSTANTS[24]);
    }
}
