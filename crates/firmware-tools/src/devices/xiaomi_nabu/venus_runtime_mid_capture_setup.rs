// SPDX-License-Identifier: GPL-3.0-only
//! Codec capture/controller resource initialization and small MMIO toggles.
//! Per-buffer allocation, checks and callback command arguments are emitted
//! as checked typed ARM A32 instructions, with separately classified tables,
//! MMIO addresses and diagnostic strings, not copied instruction opcodes.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x29ae4;
const END:usize=0x2a198;

/// Initialize codec controller resource records, verify all attached buffers, and allocate a 8192-byte working block before initializing callback handles.
fn initialize_controller_session(c:&mut Code)->Result<(),String>{
    c.check(0x29ae4)?;
    // 0x29ae4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),BlockMode::DecrementBefore,true)?)?;
    // 0x29ae8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x29aec: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,36)?)?;
    // 0x29af0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x29af4: ldr r0, [pc, #0x618]        @ 0xe78 <entry+0xe78>
    c.literal(R::R0,0x2a114)?;
    // 0x29af8: ldr r1, [pc, #0x618]        @ 0xe7c <entry+0xe7c>
    c.literal(R::R1,0x2a118)?;
    // 0x29afc: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x29b00: bl 0x307c <entry+0xfffffffffffd9de0> @ imm = #-0x26a8c
    c.bl(0x307c)?;
    // 0x29b04: bl 0x2571c <entry+0xffffffffffffc480> @ imm = #-0x43f0
    c.bl(0x2571c)?;
    // 0x29b08: ldr r6, [pc, #0x60c]        @ 0xe80 <entry+0xe80>
    c.literal(R::new(6)?,0x2a11c)?;
    // 0x29b0c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29b10: mov r2, #125
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,125)?)?;
    // 0x29b14: add r5, r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(6)?,3)?)?;
    // 0x29b18: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x29b1c: bl 0x31ec <entry+0xfffffffffffd9f50> @ imm = #-0x26938
    c.bl(0x31ec)?;
    // 0x29b20: mov r0, #49
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,49)?)?;
    // 0x29b24: mov r1, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,61)?)?;
    // 0x29b28: strb r0, [r6]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x29b2c: mov r0, #52
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,52)?)?;
    // 0x29b30: strb r0, [r6, #0x1]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,1)?)?;
    // 0x29b34: mov r0, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,58)?)?;
    // 0x29b38: strb r0, [r6, #0x2]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,2)?)?;
    // 0x29b3c: ldr r0, [pc, #0x5dc]        @ 0xe84 <entry+0xe84>
    c.literal(R::R0,0x2a120)?;
    // 0x29b40: bl 0x84120 <entry+0x5ae84> @ imm = #0x5a5d8
    c.bl(0x84120)?;
    // 0x29b44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29b48: mov r2, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,72)?)?;
    // 0x29b4c: ldreq r1, [pc, #0x5cc]        @ 0xe84 <entry+0xe84>
    c.literal_cond(C::Eq,R::R1,0x2a120)?;
    // 0x29b50: addne r1, r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R1,R::R0,1)?)?;
    // 0x29b54: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x29b58: bl 0x3294 <entry+0xfffffffffffd9ff8> @ imm = #-0x268cc
    c.bl(0x3294)?;
    // 0x29b5c: ldr r0, [pc, #0x5c0]        @ 0xe88 <entry+0xe88>
    c.literal(R::R0,0x2a124)?;
    // 0x29b60: mov r1, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,61)?)?;
    // 0x29b64: bl 0x84120 <entry+0x5ae84> @ imm = #0x5a5b4
    c.bl(0x84120)?;
    // 0x29b68: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29b6c: beq 0x29b80 <entry+0x8e4>   @ imm = #0xc
    c.b(C::Eq,0x29b80)?;
    // 0x29b70: add r2, r6, #75
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,75)?)?;
    // 0x29b74: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x29b78: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x29b7c: b 0x29b88 <entry+0x8ec>   @ imm = #0x4
    c.b(C::Al,0x29b88)?;
    // 0x29b80: ldr r1, [pc, #0x59c]        @ 0xe88 <entry+0xe88>
    c.literal(R::R1,0x2a124)?;
    // 0x29b84: add r0, r6, #75
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,75)?)?;
    // 0x29b88: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x29b8c: bl 0x3294 <entry+0xfffffffffffd9ff8> @ imm = #-0x26900
    c.bl(0x3294)?;
    // 0x29b90: mov r0, #58
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,58)?)?;
    // 0x29b94: mov r1, #61
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,61)?)?;
    // 0x29b98: strb r0, [r6, #0x60]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(6)?,96)?)?;
    // 0x29b9c: ldr r0, [pc, #0x584]        @ 0xe8c <entry+0xe8c>
    c.literal(R::R0,0x2a128)?;
    // 0x29ba0: bl 0x84120 <entry+0x5ae84> @ imm = #0x5a578
    c.bl(0x84120)?;
    // 0x29ba4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29ba8: beq 0x29bbc <entry+0x920>   @ imm = #0xc
    c.b(C::Eq,0x29bbc)?;
    // 0x29bac: add r2, r6, #97
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(6)?,97)?)?;
    // 0x29bb0: add r1, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,1)?)?;
    // 0x29bb4: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x29bb8: b 0x29bc4 <entry+0x928>   @ imm = #0x4
    c.b(C::Al,0x29bc4)?;
    // 0x29bbc: ldr r1, [pc, #0x564]        @ 0xe8c <entry+0xe8c>
    c.literal(R::R1,0x2a128)?;
    // 0x29bc0: add r0, r6, #97
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,97)?)?;
    // 0x29bc4: mov r2, #31
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,31)?)?;
    // 0x29bc8: bl 0x3294 <entry+0xfffffffffffd9ff8> @ imm = #-0x2693c
    c.bl(0x3294)?;
    // 0x29bcc: ldr r7, [pc, #0x558]        @ 0xe90 <entry+0xe90>
    c.literal(R::new(7)?,0x2a12c)?;
    // 0x29bd0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29bd4: str r0, [r7]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,0)?)?;
    // 0x29bd8: b 0x29bf4 <entry+0x958>   @ imm = #0x14
    c.b(C::Al,0x29bf4)?;
    // 0x29bdc: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x29be0: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x29be4: bl 0x2dbc <entry+0xfffffffffffd9b20> @ imm = #-0x26e30
    c.bl(0x2dbc)?;
    // 0x29be8: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x29bec: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x29bf0: bl 0x2e40 <entry+0xfffffffffffd9ba4> @ imm = #-0x26db8
    c.bl(0x2e40)?;
    // 0x29bf4: bl 0x25700 <entry+0xffffffffffffc464> @ imm = #-0x44fc
    c.bl(0x25700)?;
    // 0x29bf8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29bfc: beq 0x29bdc <entry+0x940>   @ imm = #-0x28
    c.b(C::Eq,0x29bdc)?;
    // 0x29c00: bl 0x14dc <entry+0xfffffffffffd8240> @ imm = #-0x2872c
    c.bl(0x14dc)?;
    // 0x29c04: ldr r0, [pc, #0x524]        @ 0xe94 <entry+0xe94>
    c.literal(R::R0,0x2a130)?;
    // 0x29c08: add r2, r4, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,24)?)?;
    // 0x29c0c: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x29c10: str r1, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,136)?)?;
    // 0x29c14: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29c18: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x29c1c: str r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,140)?)?;
    // 0x29c20: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29c24: bl 0x2f45c <entry+0x61c0>  @ imm = #0x5830
    c.bl(0x2f45c)?;
    // 0x29c28: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29c2c: bne 0x29c44 <entry+0x9a8>   @ imm = #0x10
    c.b(C::Ne,0x29c44)?;
    // 0x29c30: ldr r2, [pc, #0x4fc]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29c34: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x29c38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29c3c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x29c40: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28498
    c.bl(0x17b0)?;
    // 0x29c44: add r2, r4, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,28)?)?;
    // 0x29c48: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29c4c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x29c50: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x29c54: bl 0x2f45c <entry+0x61c0>  @ imm = #0x5800
    c.bl(0x2f45c)?;
    // 0x29c58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29c5c: bne 0x29c74 <entry+0x9d8>   @ imm = #0x10
    c.b(C::Ne,0x29c74)?;
    // 0x29c60: ldr r2, [pc, #0x4cc]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29c64: mov r1, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,9)?)?;
    // 0x29c68: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29c6c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x29c70: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x284c8
    c.bl(0x17b0)?;
    // 0x29c74: mov r10, r4
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::new(4)?))?;
    // 0x29c78: mov r6, r4
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(4)?))?;
    // 0x29c7c: str r5, [r10, #0x20]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(5)?,R::new(10)?,32)?)?;
    // 0x29c80: str r5, [r6, #0x24]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al,false,R::new(5)?,R::new(6)?,36)?)?;
    // 0x29c84: ldr r0, [r7, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,48)?)?;
    // 0x29c88: tst r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,2)?)?;
    // 0x29c8c: beq 0x29ce8 <entry+0xa4c>   @ imm = #0x54
    c.b(C::Eq,0x29ce8)?;
    // 0x29c90: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29c94: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29c98: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x29c9c: bl 0x2f45c <entry+0x61c0>  @ imm = #0x57b8
    c.bl(0x2f45c)?;
    // 0x29ca0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29ca4: bne 0x29cbc <entry+0xa20>   @ imm = #0x10
    c.b(C::Ne,0x29cbc)?;
    // 0x29ca8: ldr r2, [pc, #0x484]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29cac: mov r1, #21
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,21)?)?;
    // 0x29cb0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29cb4: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x29cb8: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28510
    c.bl(0x17b0)?;
    // 0x29cbc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29cc0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x29cc4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x29cc8: bl 0x2f45c <entry+0x61c0>  @ imm = #0x578c
    c.bl(0x2f45c)?;
    // 0x29ccc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29cd0: bne 0x29ce8 <entry+0xa4c>   @ imm = #0x10
    c.b(C::Ne,0x29ce8)?;
    // 0x29cd4: ldr r2, [pc, #0x458]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29cd8: mov r1, #27
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,27)?)?;
    // 0x29cdc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29ce0: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1280)?)?;
    // 0x29ce4: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x2853c
    c.bl(0x17b0)?;
    // 0x29ce8: bl 0x2eb30 <entry+0x5894>  @ imm = #0x4e40
    c.bl(0x2eb30)?;
    // 0x29cec: ldr r0, [pc, #0x444]        @ 0xe9c <entry+0xe9c>
    c.literal(R::R0,0x2a138)?;
    // 0x29cf0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x29cf4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29cf8: beq 0x29d00 <entry+0xa64>   @ imm = #0x0
    c.b(C::Eq,0x29d00)?;
    // 0x29cfc: bl 0x257c0 <entry+0xffffffffffffc524> @ imm = #-0x4544
    c.bl(0x257c0)?;
    // 0x29d00: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8192)?)?;
    // 0x29d04: bl 0x2efc <entry+0xfffffffffffd9c60> @ imm = #-0x26e10
    c.bl(0x2efc)?;
    // 0x29d08: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29d0c: str r0, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,116)?)?;
    // 0x29d10: bne 0x29d4c <entry+0xab0>   @ imm = #0x34
    c.b(C::Ne,0x29d4c)?;
    // 0x29d14: ldr r1, [pc, #0x420]        @ 0xea0 <entry+0xea0>
    c.literal(R::R1,0x2a13c)?;
    // 0x29d18: ldr r2, [pc, #0x420]        @ 0xea4 <entry+0xea4>
    c.literal(R::R2,0x2a140)?;
    // 0x29d1c: mov r3, #210
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,210)?)?;
    // 0x29d20: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29d24: orr r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,2048)?)?;
    // 0x29d28: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4f64
    c.bl(0x2ec94)?;
    // 0x29d2c: ldr r1, [pc, #0x410]        @ 0xea8 <entry+0xea8>
    c.literal(R::R1,0x2a144)?;
    // 0x29d30: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29d34: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4f58
    c.bl(0x2ec94)?;
    // 0x29d38: ldr r2, [pc, #0x3f4]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29d3c: mov r1, #211
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,211)?)?;
    // 0x29d40: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29d44: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x29d48: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x285a0
    c.bl(0x17b0)?;
    // 0x29d4c: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x29d50: ldr r5, [pc, #0x3e8]        @ 0xea4 <entry+0xea4>
    c.literal(R::new(5)?,0x2a140)?;
    // 0x29d54: ldr r8, [pc, #0x3e8]        @ 0xea8 <entry+0xea8>
    c.literal(R::new(8)?,0x2a144)?;
    // 0x29d58: mov r7, #247
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,247)?)?;
    // 0x29d5c: orr r7, r7, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2048)?)?;
    // 0x29d60: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // 0x29d64: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29d68: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,65536)?)?;
    // 0x29d6c: add r9, r0, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,69632)?)?;
    // 0x29d70: b 0x29e10 <entry+0xb74>   @ imm = #0x98
    c.b(C::Al,0x29e10)?;
    // 0x29d74: ldr r0, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,120)?)?;
    // 0x29d78: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x29d7c: orr r2, r2, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,65536)?)?;
    // 0x29d80: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // 0x29d84: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x29d88: beq 0x29e4c <entry+0xbb0>   @ imm = #0xbc
    c.b(C::Eq,0x29e4c)?;
    // 0x29d8c: mvn r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
    // 0x29d90: str r9, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,12)?)?;
    // 0x29d94: add r1, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::SP,8)?)?;
    // 0x29d98: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x29d9c: str r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,16)?)?;
    // 0x29da0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x29da4: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,20)?)?;
    // 0x29da8: mov r0, #15
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,15)?)?;
    // 0x29dac: str r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,24)?)?;
    // 0x29db0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29db4: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,28)?)?;
    // 0x29db8: mov r0, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,28)?)?;
    // 0x29dbc: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
    // 0x29dc0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x29dc4: ldr r3, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,128)?)?;
    // 0x29dc8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x29dcc: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x29dd0: bl 0x2a198 <entry+0xefc>   @ imm = #0x3c0
    c.bl(0x2a198)?;
    // 0x29dd4: ldr r1, [pc, #0x36c]        @ 0xeac <entry+0xeac>
    c.literal(R::R1,0x2a148)?;
    // 0x29dd8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29ddc: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x29de0: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x29de4: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4ea8
    c.bl(0x2ec94)?;
    // 0x29de8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29dec: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x29df0: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4e9c
    c.bl(0x2ec94)?;
    // 0x29df4: b 0x29e10 <entry+0xb74>   @ imm = #0x14
    c.b(C::Al,0x29e10)?;
    // 0x29df8: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x29dfc: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x29e00: bl 0x2dbc <entry+0xfffffffffffd9b20> @ imm = #-0x2704c
    c.bl(0x2dbc)?;
    // 0x29e04: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x29e08: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x29e0c: bl 0x2e40 <entry+0xfffffffffffd9ba4> @ imm = #-0x26fd4
    c.bl(0x2e40)?;
    // 0x29e10: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,120)?)?;
    // 0x29e14: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x29e18: bl 0x2d19c <entry+0x3f00>  @ imm = #0x337c
    c.bl(0x2d19c)?;
    // 0x29e1c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x29e20: bne 0x29df8 <entry+0xb5c>   @ imm = #-0x30
    c.b(C::Ne,0x29df8)?;
    // 0x29e24: ldr r6, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,116)?)?;
    // 0x29e28: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x29e2c: bne 0x29d74 <entry+0xad8>   @ imm = #-0xc0
    c.b(C::Ne,0x29d74)?;
    // 0x29e30: ldr r2, [pc, #0x2fc]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29e34: mov r1, #234
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,234)?)?;
    // 0x29e38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29e3c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2048)?)?;
    // 0x29e40: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28698
    c.bl(0x17b0)?;
    // 0x29e44: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x29e48: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x29e4c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,8)?)?;
    // 0x29e50: ldr r1, [pc, #0x2f4]        @ 0xeb0 <entry+0xeb0>
    c.literal(R::R1,0x2a14c)?;
    // 0x29e54: mov r7, #9
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,9)?)?;
    // 0x29e58: mov r3, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,28)?)?;
    // 0x29e5c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x29e60: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,4096)?)?;
    // 0x29e64: str r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,128)?)?;
    // 0x29e68: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x29e6c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x29e70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29e74: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x29e78: orr r1, r1, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,65536)?)?;
    // 0x29e7c: add r1, r1, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,65536)?)?;
    // 0x29e80: str r1, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,4)?)?;
    // 0x29e84: add r1, r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(6)?,8)?)?;
    // 0x29e88: stm r1, {r0, r2, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R1,R::R0.bit()|R::R2.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x29e8c: add r7, r6, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(6)?,28)?)?;
    // 0x29e90: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,4)?)?;
    // 0x29e94: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,128)?)?;
    // 0x29e98: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x29e9c: beq 0x29fa4 <entry+0xd08>   @ imm = #0x100
    c.b(C::Eq,0x29fa4)?;
    // 0x29ea0: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x29ea4: bne 0x2a054 <entry+0xdb8>   @ imm = #0x1a8
    c.b(C::Ne,0x2a054)?;
    // 0x29ea8: mov r0, #40
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,40)?)?;
    // 0x29eac: bl 0x2efc <entry+0xfffffffffffd9c60> @ imm = #-0x26fb8
    c.bl(0x2efc)?;
    // 0x29eb0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29eb4: str r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,48)?)?;
    // 0x29eb8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x29ebc: str r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
    // 0x29ec0: str r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,8)?)?;
    // 0x29ec4: ldr r1, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,48)?)?;
    // 0x29ec8: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,8)?)?;
    // 0x29ecc: bl 0x32550 <entry+0x92b4>  @ imm = #0x867c
    c.bl(0x32550)?;
    // 0x29ed0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x29ed4: bne 0x29eec <entry+0xc50>   @ imm = #0x10
    c.b(C::Ne,0x29eec)?;
    // 0x29ed8: ldr r2, [pc, #0x254]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29edc: mov r1, #47
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,47)?)?;
    // 0x29ee0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29ee4: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2304)?)?;
    // 0x29ee8: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28740
    c.bl(0x17b0)?;
    // 0x29eec: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x29ef0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x29ef4: add r8, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,4)?)?;
    // 0x29ef8: orr r0, r0, #28672
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,28672)?)?;
    // 0x29efc: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,8192)?)?;
    // 0x29f00: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // 0x29f04: str r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // 0x29f08: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x29f0c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x29f10: bl 0x2d3f4 <entry+0x4158>  @ imm = #0x34dc
    c.bl(0x2d3f4)?;
    // 0x29f14: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x29f18: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x29f1c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x29f20: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x29f24: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x29f28: bl 0x2ddfc <entry+0x4b60>  @ imm = #0x3ecc
    c.bl(0x2ddfc)?;
    // 0x29f2c: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x29f30: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x29f34: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x29f38: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x29f3c: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x29f40: sub r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x29f44: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x29f48: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x29f4c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x29f50: bls 0x29f8c <entry+0xcf0>   @ imm = #0x34
    c.b(C::Ls,0x29f8c)?;
    // 0x29f54: ldr r1, [pc, #0x1f4]        @ 0xeb4 <entry+0xeb4>
    c.literal(R::R1,0x2a150)?;
    // 0x29f58: ldr r2, [pc, #0x1e0]        @ 0xea4 <entry+0xea4>
    c.literal(R::R2,0x2a140)?;
    // 0x29f5c: mov r7, #62
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,62)?)?;
    // 0x29f60: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29f64: orr r7, r7, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2304)?)?;
    // 0x29f68: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x29f6c: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4d20
    c.bl(0x2ec94)?;
    // 0x29f70: ldr r1, [pc, #0x1cc]        @ 0xea8 <entry+0xea8>
    c.literal(R::R1,0x2a144)?;
    // 0x29f74: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x29f78: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4d14
    c.bl(0x2ec94)?;
    // 0x29f7c: ldr r2, [pc, #0x1b0]        @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x29f80: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x29f84: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x29f88: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x287e0
    c.bl(0x17b0)?;
    // 0x29f8c: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x29f90: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x29f94: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x29f98: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x29f9c: bl 0x2a198 <entry+0xefc>   @ imm = #0x1f4
    c.bl(0x2a198)?;
    // 0x29fa0: b 0x2a06c <entry+0xdd0>   @ imm = #0xc4
    c.b(C::Al,0x2a06c)?;
    // 0x29fa4: mov r0, #26
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,26)?)?;
    // 0x29fa8: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x29fac: add r8, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::SP,4)?)?;
    // 0x29fb0: orr r0, r0, #28672
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,28672)?)?;
    // 0x29fb4: orr r1, r1, #45056
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,45056)?)?;
    // 0x29fb8: str r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,20)?)?;
    // 0x29fbc: str r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(6)?,24)?)?;
    // 0x29fc0: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x29fc4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29fc8: bl 0x2d3f4 <entry+0x4158>  @ imm = #0x3424
    c.bl(0x2d3f4)?;
    // 0x29fcc: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x29fd0: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x29fd4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x29fd8: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x29fdc: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x29fe0: bl 0x2ddfc <entry+0x4b60>  @ imm = #0x3e14
    c.bl(0x2ddfc)?;
    // 0x29fe4: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x29fe8: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x29fec: str r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,12)?)?;
    // 0x29ff0: ldr r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,4)?)?;
    // 0x29ff4: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,0)?)?;
    // 0x29ff8: sub r0, r0, r7
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x29ffc: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x2a000: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x2a004: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x2a008: bls 0x2a044 <entry+0xda8>   @ imm = #0x34
    c.b(C::Ls,0x2a044)?;
    // 0x2a00c: ldr r1, [pc, #0x13c]        @ 0xeb4 <entry+0xeb4>
    c.literal(R::R1,0x2a150)?;
    // 0x2a010: ldr r2, [pc, #0x128]        @ 0xea4 <entry+0xea4>
    c.literal(R::R2,0x2a140)?;
    // 0x2a014: mov r7, #87
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,87)?)?;
    // 0x2a018: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2a01c: orr r7, r7, #2304
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,2304)?)?;
    // 0x2a020: mov r3, r7
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
    // 0x2a024: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4c68
    c.bl(0x2ec94)?;
    // 0x2a028: ldr r1, [pc, #0x114]        @ 0xea8 <entry+0xea8>
    c.literal(R::R1,0x2a144)?;
    // 0x2a02c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,8)?)?;
    // 0x2a030: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4c5c
    c.bl(0x2ec94)?;
    // 0x2a034: ldr r2, [pc, #0xf8]         @ 0xe98 <entry+0xe98>
    c.literal(R::R2,0x2a134)?;
    // 0x2a038: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2a03c: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x2a040: bl 0x17b0 <entry+0xfffffffffffd8514> @ imm = #-0x28898
    c.bl(0x17b0)?;
    // 0x2a044: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2a048: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x2a04c: bl 0x2a878 <entry+0x15dc>  @ imm = #0x824
    c.bl(0x2a878)?;
    // 0x2a050: b 0x2a06c <entry+0xdd0>   @ imm = #0x14
    c.b(C::Al,0x2a06c)?;
    // 0x2a054: ldr r1, [pc, #0xf8]         @ 0xeb8 <entry+0xeb8>
    c.literal(R::R1,0x2a154)?;
    // 0x2a058: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2a05c: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4c30
    c.bl(0x2ec94)?;
    // 0x2a060: ldr r1, [pc, #0xdc]         @ 0xea8 <entry+0xea8>
    c.literal(R::R1,0x2a144)?;
    // 0x2a064: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x2a068: bl 0x2ec94 <entry+0x59f8>  @ imm = #0x4c24
    c.bl(0x2ec94)?;
    // 0x2a06c: ldr r0, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,116)?)?;
    // 0x2a070: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a074: beq 0x2a084 <entry+0xde8>   @ imm = #0x8
    c.b(C::Eq,0x2a084)?;
    // 0x2a078: bl 0x2f1c <entry+0xfffffffffffd9c80> @ imm = #-0x27164
    c.bl(0x2f1c)?;
    // 0x2a07c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x2a080: str r0, [r4, #0x74]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,116)?)?;
    // 0x2a084: bl 0x30cc <entry+0xfffffffffffd9e30> @ imm = #-0x26fc0
    c.bl(0x30cc)?;
    // 0x2a088: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x2a08c: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x2a090: ldr r0, [r4, #0x70]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,112)?)?;
    // 0x2a094: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2a098: ldr r0, [r0, #0xbc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,188)?)?;
    // 0x2a09c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a0a0: beq 0x2a0c8 <entry+0xe2c>   @ imm = #0x20
    c.b(C::Eq,0x2a0c8)?;
    // 0x2a0a4: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
    // 0x2a0a8: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x2a0ac: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a0b0: beq 0x2a0c8 <entry+0xe2c>   @ imm = #0x10
    c.b(C::Eq,0x2a0c8)?;
    // 0x2a0b4: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // 0x2a0b8: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2a0bc: str r5, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,20)?)?;
    // 0x2a0c0: bl 0x2b474 <entry+0x21d8>  @ imm = #0x13ac
    c.bl(0x2b474)?;
    // 0x2a0c4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x2a0c8: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x2a0cc: str r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,16)?)?;
    // 0x2a0d0: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x2a0d4: str r7, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(4)?,20)?)?;
    // 0x2a0d8: bl 0x2b474 <entry+0x21d8>  @ imm = #0x1394
    c.bl(0x2b474)?;
    // 0x2a0dc: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
    // 0x2a0e0: cmpne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x2a0e4: beq 0x2a090 <entry+0xdf4>   @ imm = #-0x5c
    c.b(C::Eq,0x2a090)?;
    // 0x2a0e8: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
    // 0x2a0ec: bl 0x2f58c <entry+0x62f0>  @ imm = #0x5498
    c.bl(0x2f58c)?;
    // 0x2a0f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x2a0f4: beq 0x2a084 <entry+0xde8>   @ imm = #-0x78
    c.b(C::Eq,0x2a084)?;
    // 0x2a0f8: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x2a0fc: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x2a100: bl 0x2dbc <entry+0xfffffffffffd9b20> @ imm = #-0x2734c
    c.bl(0x2dbc)?;
    // 0x2a104: ldr r0, [r4, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
    // 0x2a108: mov r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2147483648)?)?;
    // 0x2a10c: bl 0x2e40 <entry+0xfffffffffffd9ba4> @ imm = #-0x272d4
    c.bl(0x2e40)?;
    // 0x2a110: b 0x2a084 <entry+0xde8>   @ imm = #-0x94
    c.b(C::Al,0x2a084)?;
    c.check(0x2a114)?;
    // Identified literal/address data, not executable ARM instructions.
    const DATA:[u32;17]=[0x0010dfe0,0x000e2549,0x0010ded8,0x000e24a8,0x000e24da,0x000e24f4,0xe00d2018,0xe00e1024,0x000e884d,0x0010306c,0x000e8881,0x000ea76f,0x000e2788,0x000ea792,0x0010df58,0x000ea7dc,0x000ea814];
    for v in DATA {c.emit(v)?;}
    c.check(0x2a158)
}

/// Enable a controller MMIO event bit in the stream-control register.
fn enable_codec_controller_event(c:&mut Code)->Result<(),String>{
    c.check(0x2a158)?;
    // 0x2a158: ldr r0, [pc, #0x8]          @ 0xecc <entry+0xecc>
    c.literal(R::R0,0x2a168)?;
    // 0x2a15c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2a160: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2a164: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2a168)?;
    // Identified literal/address data, not executable ARM instructions.
    const DATA:[u32;1]=[0xe00d2018];
    for v in DATA {c.emit(v)?;}
    c.check(0x2a16c)
}

/// Enable a second stream/IRQ line and set the controller status bit.
fn enable_codec_buffer_interrupter(c:&mut Code)->Result<(),String>{
    c.check(0x2a16c)?;
    // 0x2a16c: ldr r0, [pc, #0x14]         @ 0xeec <entry+0xeec>
    c.literal(R::R0,0x2a188)?;
    // 0x2a170: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x2a174: str r1, [r0, #0xec]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,236)?)?;
    // 0x2a178: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x2a17c: orr r1, r1, #-2147483648
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,2147483648)?)?;
    // 0x2a180: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,0)?)?;
    // 0x2a184: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(0x2a188)?;
    // Identified literal/address data, not executable ARM instructions.
    const DATA:[u32;1]=[0xe00d204c];
    for v in DATA {c.emit(v)?;}
    c.check(0x2a18c)
}

/// Adapt an input descriptor to the next helper routine using a tail call.
fn prepare_codec_descriptor_helper(c:&mut Code)->Result<(),String>{
    c.check(0x2a18c)?;
    // 0x2a18c: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x2a190: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x2a194: b 0x2a198 <entry+0xefc>   @ imm = #-0x4
    c.b(C::Al,0x2a198)?;
    c.check(0x2a198)?;
    c.check(0x2a198)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    initialize_controller_session(c)?;
    enable_codec_controller_event(c)?;
    enable_codec_buffer_interrupter(c)?;
    prepare_codec_descriptor_helper(c)?;
    c.check(END)
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn controller_configuration_has_precise_constant_pools(){
        let mut c=Code::new(START);emit(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        let word=|addr:usize|u32::from_le_bytes(c.bytes[addr-START..addr-START+4].try_into().unwrap());
        assert_eq!(word(0x2a114),0x0010_dfe0);
        assert_eq!(word(0x2a12c),0xe00d_2018);
        assert_eq!(word(0x2a158),arm32::transfer_word(C::Al,true,R::R0,R::PC,8).unwrap());
        assert_eq!(word(0x2a168),0xe00d_2018);
        assert_eq!(word(0x2a188),0xe00d_204c);
        assert_eq!(word(0x2a194),arm32::branch(C::Al,0x2a194,0x2a198,false).unwrap());
    }
}
