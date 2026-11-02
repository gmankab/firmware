// SPDX-License-Identifier: GPL-3.0-only
//! Opening control flow of the Venus codec-class state machine. The
//! three bounded indexed jump tables are separately typed data addresses.
//! The parent function continues beyond this fragment at runtime 0x32c9c.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
const START:usize=0x329f8;
const END:usize=0x32c9c;

/// Codec controller state at 0x329f8–0x32b74.
fn initialize_codec_state_class(c:&mut Code)->Result<(),String>{
    c.check(0x329f8)?;
    // 0x329f8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x329fc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x32a00: sub sp, sp, #476
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,476)?)?;
    // 0x32a04: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x32a08: mov r0, #360
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,360)?)?;
    // 0x32a0c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x32a10: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x32a14: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,5120)?)?;
    // 0x32a18: mov r8, r6
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(6)?))?;
    // 0x32a1c: str r1, [r8, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R1,R::new(8)?,R::R0,Shift::Lsl,0)?)?;
    // 0x32a20: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32a24: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x32a28: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x2f9ac
    c.bl(0x3084)?;
    // 0x32a2c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32a30: ldr r9, [pc, #0x9f8]        @ 0x16b0 <entry+0x16b0>
    c.literal(R::new(9)?,0x33430)?;
    // 0x32a34: ldr r10, [pc, #0x9f0]       @ 0x16ac <entry+0x16ac>
    c.literal(R::new(10)?,0x3342c)?;
    // 0x32a38: add r5, sp, #60
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,60)?)?;
    // 0x32a3c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,4096)?)?;
    // 0x32a40: add r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32a44: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,36)?)?;
    // 0x32a48: ldr r0, [pc, #0x9d4]        @ 0x16a4 <entry+0x16a4>
    c.literal(R::R0,0x33424)?;
    // 0x32a4c: add r4, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32a50: ldr r0, [pc, #0x9d0]        @ 0x16a8 <entry+0x16a8>
    c.literal(R::R0,0x33428)?;
    // 0x32a54: str r4, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,48)?)?;
    // 0x32a58: add r0, r8, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32a5c: str r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,56)?)?;
    // 0x32a60: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x32a64: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4096)?)?;
    // 0x32a68: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,52)?)?;
    // 0x32a6c: add r0, r8, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,8)?)?;
    // 0x32a70: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,32)?)?;
    // 0x32a74: add r0, r8, #20
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,20)?)?;
    // 0x32a78: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x32a7c: add r0, r8, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,16)?)?;
    // 0x32a80: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x32a84: add r0, r8, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,24)?)?;
    // 0x32a88: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x32a8c: add r0, r8, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,12)?)?;
    // 0x32a90: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x32a94: add r0, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(8)?,4)?)?;
    // 0x32a98: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,12)?)?;
    // 0x32a9c: sub r0, r8, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::new(8)?,5120)?)?;
    // 0x32aa0: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,40)?)?;
    // 0x32aa4: add r0, sp, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,72)?)?;
    // 0x32aa8: add r0, r0, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,24)?)?;
    // 0x32aac: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,44)?)?;
    // 0x32ab0: ldr r0, [r6, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,320)?)?;
    // 0x32ab4: bl 0x294c <entry+0xfffffffffffd0bcc> @ imm = #-0x30170
    c.bl(0x294c)?;
    // 0x32ab8: ldr r0, [r6, #0x140]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,320)?)?;
    // 0x32abc: bl 0x293c <entry+0xfffffffffffd0bbc> @ imm = #-0x30188
    c.bl(0x293c)?;
    // 0x32ac0: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
    // 0x32ac4: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x32ac8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x32acc: bne 0x32af4 <entry+0xd74>   @ imm = #0x20
    c.b(C::Ne,0x32af4)?;
    // 0x32ad0: b 0x3340c <entry+0x168c>  @ imm = #0x934
    c.b(C::Al,0x3340c)?;
    // 0x32ad4: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x32ad8: bl 0x3489c <entry+0x2b1c>  @ imm = #0x1dbc
    c.bl(0x3489c)?;
    // 0x32adc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32ae0: mov r1, r10
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(10)?))?;
    // 0x32ae4: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3e58
    c.bl(0x2ec94)?;
    // 0x32ae8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32aec: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x32af0: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3e64
    c.bl(0x2ec94)?;
    // 0x32af4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x32af8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x32afc: bl 0x80994 <entry+0x4ec14> @ imm = #0x4de90
    c.bl(0x80994)?;
    // 0x32b00: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x32b04: beq 0x32ad4 <entry+0xd54>   @ imm = #-0x38
    c.b(C::Eq,0x32ad4)?;
    // 0x32b08: b 0x3340c <entry+0x168c>  @ imm = #0x8fc
    c.b(C::Al,0x3340c)?;
    // 0x32b0c: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x32b10: ldr r1, [pc, #0x91c]        @ 0x16b4 <entry+0x16b4>
    c.literal(R::R1,0x33434)?;
    // 0x32b14: ldr r7, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R0,4)?)?;
    // 0x32b18: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32b1c: mov r2, r7
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(7)?))?;
    // 0x32b20: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3e94
    c.bl(0x2ec94)?;
    // 0x32b24: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x32b28: mov r1, r9
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(9)?))?;
    // 0x32b2c: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x3ea0
    c.bl(0x2ec94)?;
    // 0x32b30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32b34: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x32b38: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x32b3c: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x2fac0
    c.bl(0x3084)?;
    // 0x32b40: cmp r7, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1114112)?)?;
    // 0x32b44: ble 0x32b8c <entry+0xe0c>   @ imm = #0x40
    c.b(C::Le,0x32b8c)?;
    // 0x32b48: ldr r0, [pc, #0x8ec]        @ 0x16bc <entry+0x16bc>
    c.literal(R::R0,0x3343c)?;
    // 0x32b4c: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32b50: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,7)?)?;
    // 0x32b54: blo 0x32c08 <entry+0xe88>   @ imm = #0xac
    c.b(C::Cc,0x32c08)?;
    // 0x32b58: mvn r0, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,1114112)?)?;
    // 0x32b5c: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32b60: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x32b64: bhi 0x32f38 <entry+0x11b8>  @ imm = #0x3cc
    c.b(C::Hi,0x32f38)?;
    // 0x32b68: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x32b6c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x32b70: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x32b74)
}

/// Codec controller state at 0x32b8c–0x32ba8.
fn validate_codec_frame_mode(c:&mut Code)->Result<(),String>{
    c.check(0x32b8c)?;
    // 0x32b8c: ldr r0, [pc, #0x8a4]        @ 0x16b8 <entry+0x16b8>
    c.literal(R::R0,0x33438)?;
    // 0x32b90: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32b94: cmp r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,6)?)?;
    // 0x32b98: bhi 0x32bf8 <entry+0xe78>   @ imm = #0x58
    c.b(C::Hi,0x32bf8)?;
    // 0x32b9c: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x32ba0: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x32ba4: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x32ba8)
}

/// Codec controller state at 0x32bc4–0x32c84.
fn prepare_codec_frame_command(c:&mut Code)->Result<(),String>{
    c.check(0x32bc4)?;
    // 0x32bc4: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,68)?)?;
    // 0x32bc8: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,12)?)?;
    // 0x32bcc: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x32bd0: bne 0x3328c <entry+0x150c>  @ imm = #0x6b4
    c.b(C::Ne,0x3328c)?;
    // 0x32bd4: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x32bd8: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x32bdc: ldr r1, [r0, #0x10]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,true,R::R1,R::R0,16)?)?;
    // 0x32be0: add r5, r2, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(5)?,R::R2,R::R1,Shift::Lsl,3,false)?)?;
    // 0x32be4: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,0)?)?;
    // 0x32be8: cmp r1, r5
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x32bec: bne 0x33328 <entry+0x15a8>  @ imm = #0x734
    c.b(C::Ne,0x33328)?;
    // 0x32bf0: bl 0x2f96c <entry+0xffffffffffffdbec> @ imm = #-0x328c
    c.bl(0x2f96c)?;
    // 0x32bf4: b 0x3339c <entry+0x161c>  @ imm = #0x7a0
    c.b(C::Al,0x3339c)?;
    // 0x32bf8: mvn r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,69632)?)?;
    // 0x32bfc: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32c00: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x32c04: bhs 0x32f38 <entry+0x11b8>  @ imm = #0x32c
    c.b(C::Cs,0x32f38)?;
    // 0x32c08: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,68)?)?;
    // 0x32c0c: mov r4, r8
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(8)?))?;
    // 0x32c10: ldr r2, [pc, #0x834]        @ 0x16cc <entry+0x16cc>
    c.literal(R::R2,0x3344c)?;
    // 0x32c14: mov r1, #103
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,103)?)?;
    // 0x32c18: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x32c1c: ldr r8, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R0,8)?)?;
    // 0x32c20: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x32c24: cmp r8, #256
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,256)?)?;
    // 0x32c28: movlo r0, #1
    c.emit(arm32::data_immediate(C::Cc,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x32c2c: bl 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x31484
    c.bl(0x17b0)?;
    // 0x32c30: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,40)?)?;
    // 0x32c34: add r0, r8, r8, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::new(8)?,Shift::Lsl,2,false)?)?;
    // 0x32c38: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x32c3c: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,2,false)?)?;
    // 0x32c40: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x32c44: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x32c48: bl 0x3084 <entry+0xfffffffffffd1304> @ imm = #-0x2fbcc
    c.bl(0x3084)?;
    // 0x32c4c: mov r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,69632)?)?;
    // 0x32c50: orr r0, r0, #1048576
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1048576)?)?;
    // 0x32c54: cmp r7, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32c58: bgt 0x32cf8 <entry+0xf78>   @ imm = #0x98
    c.b(C::Gt,0x32cf8)?;
    // 0x32c5c: cmp r7, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,1114112)?)?;
    // 0x32c60: mov r8, r4
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::new(4)?))?;
    // 0x32c64: ble 0x32d64 <entry+0xfe4>   @ imm = #0xf8
    c.b(C::Le,0x32d64)?;
    // 0x32c68: mvn r0, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,1114112)?)?;
    // 0x32c6c: add r0, r7, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(7)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x32c70: cmp r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,5)?)?;
    // 0x32c74: bhi 0x32f38 <entry+0x11b8>  @ imm = #0x2bc
    c.b(C::Hi,0x32f38)?;
    // 0x32c78: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,2)?)?;
    // 0x32c7c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::PC,0)?)?;
    // 0x32c80: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R0,R::R1,Shift::Lsl,0)?)?;
    c.check(0x32c84)
}

const FRAME_TYPE_DISPATCH:[u32;6]=[
    0x00032c9c, // 0x32b74 branch destination
    0x00032da4, // 0x32b78 branch destination
    0x00032dbc, // 0x32b7c branch destination
    0x00032e5c, // 0x32b80 branch destination
    0x00032f38, // 0x32b84 branch destination
    0x00032e84, // 0x32b88 branch destination
];

const FORMAT_CLASS_DISPATCH:[u32;7]=[
    0x00032bc4, // 0x32ba8 branch destination
    0x00032f44, // 0x32bac branch destination
    0x00032f68, // 0x32bb0 branch destination
    0x00032f78, // 0x32bb4 branch destination
    0x00033078, // 0x32bb8 branch destination
    0x00033088, // 0x32bbc branch destination
    0x00033108, // 0x32bc0 branch destination
];

const FRAME_COMPARE_DISPATCH:[u32;6]=[
    0x00032c9c, // 0x32c84 branch destination
    0x00032da4, // 0x32c88 branch destination
    0x00032dbc, // 0x32c8c branch destination
    0x00032e5c, // 0x32c90 branch destination
    0x00032f38, // 0x32c94 branch destination
    0x00032e84, // 0x32c98 branch destination
];

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    initialize_codec_state_class(c)?;
    c.check(0x32b74)?;
    for destination in FRAME_TYPE_DISPATCH{c.emit(destination)?;}
    c.check(0x32b8c)?;
    validate_codec_frame_mode(c)?;
    c.check(0x32ba8)?;
    for destination in FORMAT_CLASS_DISPATCH{c.emit(destination)?;}
    c.check(0x32bc4)?;
    prepare_codec_frame_command(c)?;
    c.check(0x32c84)?;
    for destination in FRAME_COMPARE_DISPATCH{c.emit(destination)?;}
    c.check(0x32c9c)?;
    c.check(END)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn codec_switch_prefix_has_three_explicit_tables(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let at=|v:usize|u32::from_le_bytes(c.bytes[v-START..v-START+4].try_into().unwrap());
        assert_eq!(at(0x32b74),0x32c9c);
        assert_eq!(at(0x32ba8),0x32bc4);
        assert_eq!(at(0x32c84),FRAME_COMPARE_DISPATCH[0]);
        assert_eq!(at(0x32c98),FRAME_COMPARE_DISPATCH[5]);
    }
}
