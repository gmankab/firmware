// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec descriptor write and queue advancement for streamed planes.
//! Encoded from explicit ARMv7 A32 operand forms; the diagnostic words at
//! 0x3e2c4 and 0x3e300 are non-executable linker address data.
use super::Code;
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
const START:usize=0x3dfec;
const END:usize=0x3e304;
const INVALID_INPUT_DIAGNOSTIC:u32=0x000e_e028;

/// Publish codec descriptor words, adjust the queued record count and
/// return using the original saved register list.
fn publish_buffer_records(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x3dfec: ldr r12, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,0)?)?;
    // 0x3dff0: tst r12, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R12,31)?)?;
    // 0x3dff4: bne 0x3e010 <entry+0x4440>  @ imm = #0x14
    c.b(C::Ne,0x3e010)?;
    // 0x3dff8: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x3dffc: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x3e000: ldr lr, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R12,0)?)?;
    // 0x3e004: and lr, lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::LR,R::LR,1)?)?;
    // 0x3e008: str lr, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R12,0)?)?;
    // 0x3e00c: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x3e010: stm r12, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::R12,R::R0.bit()|R::R1.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3e014: ldr r0, [r3]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(3)?,0)?)?;
    // 0x3e018: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x3e01c: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,0)?)?;
    // 0x3e020: add r0, r12, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R12,8)?)?;
    // 0x3e024: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R2,0)?)?;
    // 0x3e028: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x3e02c: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x3e030: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x3e034: ldr r8, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,12)?)?;
    // 0x3e038: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x3e03c: ldr r2, [pc, #0x280]        @ 0x46f4 <entry+0x46f4>
    c.literal_cond(C::Al,R::R2,0x3e2c4)?;
    // 0x3e040: mov r7, r0
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R0))?;
    // 0x3e044: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3e048: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x3e04c: mov r1, #184
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,184)?)?;
    // 0x3e050: mov r5, r3
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(3)?))?;
    // 0x3e054: tst r8, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(8)?,255)?)?;
    // 0x3e058: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x3e05c: bl 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3c8b4
    c.bl(0x17b0)?;
    // 0x3e060: orr r0, r4, r6, lsl #4
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,4,false)?)?;
    // 0x3e064: ldr r6, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R11,8)?)?;
    // 0x3e068: ldr r12, [r11, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,28)?)?;
    // 0x3e06c: ldr r1, [r11, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,24)?)?;
    // 0x3e070: orr r0, r0, r5, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(5)?,Shift::Lsl,8,false)?)?;
    // 0x3e074: orr r7, r0, r7, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(7)?,R::R0,R::new(7)?,Shift::Lsl,16,false)?)?;
    // 0x3e078: ldr r0, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,16)?)?;
    // 0x3e07c: orr r3, r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(7)?,4096)?)?;
    // 0x3e080: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x3e084: orrne r7, r7, r6, lsl #12
    c.emit(arm32::data_register(C::Ne,Op::Orr,R::new(7)?,R::new(7)?,R::new(6)?,Shift::Lsl,12,false)?)?;
    // 0x3e088: cmp r4, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,1)?)?;
    // 0x3e08c: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,255)?)?;
    // 0x3e090: bic r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R2,R::R0,255)?)?;
    // 0x3e094: beq 0x3e148 <entry+0x4578>  @ imm = #0xac
    c.b(C::Eq,0x3e148)?;
    // 0x3e098: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x3e09c: bne 0x3e1c4 <entry+0x45f4>  @ imm = #0x120
    c.b(C::Ne,0x3e1c4)?;
    // 0x3e0a0: ldr r3, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // 0x3e0a4: tst r3, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
    // 0x3e0a8: ldreq r0, [r3]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(3)?,0)?)?;
    // 0x3e0ac: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x3e0b0: streq r0, [r3]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(3)?,0)?)?;
    // 0x3e0b4: mov r0, #868
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,868)?)?;
    // 0x3e0b8: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x3e0bc: stm r3, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R0.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3e0c0: add r0, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,8)?)?;
    // 0x3e0c4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e0c8: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e0cc: ldreq r6, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R0,0)?)?;
    // 0x3e0d0: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3e0d4: streq r6, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R0,0)?)?;
    // 0x3e0d8: mov r0, #2912
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2912)?)?;
    // 0x3e0dc: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x3e0e0: str r0, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,8)?)?;
    // 0x3e0e4: add r0, r3, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,16)?)?;
    // 0x3e0e8: str r8, [r3, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(3)?,12)?)?;
    // 0x3e0ec: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e0f0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e0f4: ldreq r6, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(6)?,R::R0,0)?)?;
    // 0x3e0f8: andeq r6, r6, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(6)?,R::new(6)?,1)?)?;
    // 0x3e0fc: streq r6, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(6)?,R::R0,0)?)?;
    // 0x3e100: mov r0, #3040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3040)?)?;
    // 0x3e104: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x3e108: str r0, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,16)?)?;
    // 0x3e10c: add r0, r3, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,24)?)?;
    // 0x3e110: str r7, [r3, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(3)?,20)?)?;
    // 0x3e114: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e118: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e11c: ldreq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(7)?,R::R0,0)?)?;
    // 0x3e120: andeq r7, r7, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(7)?,R::new(7)?,1)?)?;
    // 0x3e124: streq r7, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(7)?,R::R0,0)?)?;
    // 0x3e128: mov r0, #988
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,988)?)?;
    // 0x3e12c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x3e130: str r0, [r3, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(3)?,24)?)?;
    // 0x3e134: add r0, r3, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(3)?,32)?)?;
    // 0x3e138: str r2, [r3, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(3)?,28)?)?;
    // 0x3e13c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e140: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3e144: b 0x3e2b4 <entry+0x46e4>  @ imm = #0x168
    c.b(C::Al,0x3e2b4)?;
    // 0x3e148: ldr r5, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,0)?)?;
    // 0x3e14c: mov lr, #984
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,984)?)?;
    // 0x3e150: orr lr, lr, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,14336)?)?;
    // 0x3e154: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x3e158: ldreq r0, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,0)?)?;
    // 0x3e15c: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x3e160: streq r0, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,0)?)?;
    // 0x3e164: mov r0, #860
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,860)?)?;
    // 0x3e168: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x3e16c: stm r5, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R0.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3e170: add r0, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,8)?)?;
    // 0x3e174: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e178: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e17c: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x3e180: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3e184: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x3e188: mov r0, #856
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,856)?)?;
    // 0x3e18c: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x3e190: str r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    // 0x3e194: add r0, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,16)?)?;
    // 0x3e198: str r8, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,12)?)?;
    // 0x3e19c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e1a0: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e1a4: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x3e1a8: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3e1ac: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x3e1b0: add r0, r5, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,24)?)?;
    // 0x3e1b4: str lr, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,16)?)?;
    // 0x3e1b8: str r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,20)?)?;
    // 0x3e1bc: mov r7, #980
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,980)?)?;
    // 0x3e1c0: b 0x3e23c <entry+0x466c>  @ imm = #0x74
    c.b(C::Al,0x3e23c)?;
    // 0x3e1c4: ldr r5, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R1,0)?)?;
    // 0x3e1c8: mov lr, #3024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,3024)?)?;
    // 0x3e1cc: orr lr, lr, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::LR,R::LR,12288)?)?;
    // 0x3e1d0: tst r5, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(5)?,31)?)?;
    // 0x3e1d4: ldreq r0, [r5]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(5)?,0)?)?;
    // 0x3e1d8: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x3e1dc: streq r0, [r5]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(5)?,0)?)?;
    // 0x3e1e0: mov r0, #852
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,852)?)?;
    // 0x3e1e4: orr r0, r0, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,14336)?)?;
    // 0x3e1e8: stm r5, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al,false,R::new(5)?,R::R0.bit()|R::new(7)?.bit(),BlockMode::IncrementAfter,false)?)?;
    // 0x3e1ec: add r0, r5, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,8)?)?;
    // 0x3e1f0: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e1f4: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e1f8: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x3e1fc: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3e200: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x3e204: mov r0, #2896
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2896)?)?;
    // 0x3e208: orr r0, r0, #12288
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,12288)?)?;
    // 0x3e20c: str r0, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(5)?,8)?)?;
    // 0x3e210: add r0, r5, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,16)?)?;
    // 0x3e214: str r8, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::new(5)?,12)?)?;
    // 0x3e218: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e21c: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e220: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x3e224: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3e228: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x3e22c: add r0, r5, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,24)?)?;
    // 0x3e230: str lr, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,16)?)?;
    // 0x3e234: str r7, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,20)?)?;
    // 0x3e238: mov r7, #972
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,972)?)?;
    // 0x3e23c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e240: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e244: ldreq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::R0,0)?)?;
    // 0x3e248: andeq r4, r4, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
    // 0x3e24c: streq r4, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::R0,0)?)?;
    // 0x3e250: orr r7, r7, #14336
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(7)?,R::new(7)?,14336)?)?;
    // 0x3e254: add r4, r5, #32
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(5)?,32)?)?;
    // 0x3e258: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x3e25c: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x3e260: str r7, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,24)?)?;
    // 0x3e264: str r2, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,28)?)?;
    // 0x3e268: str r4, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R1,0)?)?;
    // 0x3e26c: bne 0x3e2b4 <entry+0x46e4>  @ imm = #0x40
    c.b(C::Ne,0x3e2b4)?;
    // 0x3e270: tst r4, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,31)?)?;
    // 0x3e274: ldreq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,0)?)?;
    // 0x3e278: andeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::R0,R::R0,1)?)?;
    // 0x3e27c: streq r0, [r4]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,0)?)?;
    // 0x3e280: add r0, r5, #40
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,40)?)?;
    // 0x3e284: str lr, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::new(5)?,32)?)?;
    // 0x3e288: str r3, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(5)?,36)?)?;
    // 0x3e28c: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e290: tst r0, #31
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
    // 0x3e294: ldreq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,true,R::new(3)?,R::R0,0)?)?;
    // 0x3e298: andeq r3, r3, #1
    c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(3)?,R::new(3)?,1)?)?;
    // 0x3e29c: streq r3, [r0]
    c.emit(arm32::transfer_word(C::Eq,false,R::new(3)?,R::R0,0)?)?;
    // 0x3e2a0: add r0, r5, #48
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,48)?)?;
    // 0x3e2a4: str r7, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::new(5)?,40)?)?;
    // 0x3e2a8: str r2, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::new(5)?,44)?)?;
    // 0x3e2ac: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e2b0: mov r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,6)?)?;
    // 0x3e2b4: ldr r1, [r12]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R12,0)?)?;
    // 0x3e2b8: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x3e2bc: str r0, [r12]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R12,0)?)?;
    // 0x3e2c0: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;

    c.check(0x3e2c4)?;
    c.emit(INVALID_INPUT_DIAGNOSTIC)?;
    c.check(0x3e2c8)
}

/// Move to the next aligned queue slot, reporting capacity violations.
fn advance_queue_pointer(c:&mut Code)->Result<(),String>{
    c.check(0x3e2c8)?;
    // 0x3e2c8: ldr r3, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R1,0)?)?;
    // 0x3e2cc: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x3e2d0: bic r3, r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::new(3)?,R::new(3)?,3)?)?;
    // 0x3e2d4: str r2, [r0, r3]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R2,R::R0,R::new(3)?,Shift::Lsl,0)?)?;
    // 0x3e2d8: ldr r0, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,0)?)?;
    // 0x3e2dc: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,4)?)?;
    // 0x3e2e0: cmp r0, #20480
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20480)?)?;
    // 0x3e2e4: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R1,0)?)?;
    // 0x3e2e8: bxls lr
    c.emit(arm32::bx(C::Ls,R::LR))?;
    // 0x3e2ec: ldr r2, [pc, #0xc]          @ 0x4730 <entry+0x4730>
    c.literal_cond(C::Al,R::R2,0x3e300)?;
    // 0x3e2f0: mov r1, #107
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,107)?)?;
    // 0x3e2f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x3e2f8: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x3e2fc: b 0x17b0 <entry+0xfffffffffffc7be0> @ imm = #-0x3cb54
    c.b(C::Al,0x17b0)?;

    c.check(0x3e300)?;
    c.emit(INVALID_INPUT_DIAGNOSTIC)?;
    c.check(END)
}

pub(super) fn emit(c:&mut Code)->Result<(),String>{
    publish_buffer_records(c)?;
    advance_queue_pointer(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
 use super::*;
 #[test] fn descriptor_and_queue_helpers_end_at_real_boundaries(){
  let mut c=Code::new(START);emit(&mut c).unwrap();
  assert_eq!(c.at(),END);
  assert_eq!(c.bytes.len(),END-START);
  for a in [0x3e2c4,0x3e300]{
   let i=a-START;
   assert_eq!(u32::from_le_bytes(c.bytes[i..i+4].try_into().unwrap()),INVALID_INPUT_DIAGNOSTIC);
  }
 }
}
