// SPDX-License-Identifier: GPL-3.0-only
//! Codec format-option validation and geometry configuration.
//! The source derives codec mode capabilities, masks resource-format flags,
//! and publishes a controller parameter layout. ARM instructions are typed;
//! separate diagnostic data constants are never interpreted as executable A32.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x1bc34;
const END:usize=0x1c0d0;
const CODEC_TRACE_ADDRESSES:[u32;7]=[0x0001baf4, 0x000e4dd7, 0x000e52f3, 0x000e532d, 0x000e2788, 0x000e5348, 0x000e5385];
const CONTROL_DIAGNOSTIC:u32=0x000e53c3;

/// Encode error traces around platform packet validation. The last three
/// executable instructions form a separate resource-pointer accessor.
fn emit_codec_parameter_diagnostics(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1bc34: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1bc38: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x1bc3c: sub sp, sp, #40
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x28)?)?;
    // 0x1bc40: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1bc44: ldr r0, [pc, #0x254]
    c.literal(R::R0, 0x1bea0)?;
    // 0x1bc48: str r2, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 12)?)?;
    // 0x1bc4c: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 16)?)?;
    // 0x1bc50: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1bc54: str r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 20)?)?;
    // 0x1bc58: mov r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2000)?)?;
    // 0x1bc5c: str r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 24)?)?;
    // 0x1bc60: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 28)?)?;
    // 0x1bc64: str r5, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 32)?)?;
    // 0x1bc68: str r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 36)?)?;
    // 0x1bc6c: add r0, sp, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0xc)?)?;
    // 0x1bc70: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bc74: bl 0x29b4 <entry+0xfffffffffffe76b4>
    c.bl(0x29b4)?;
    // 0x1bc78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1bc7c: beq 0x1bc90 <entry+0x990>
    c.b(C::Eq, 0x1bc90)?;
    // 0x1bc80: ldr r2, [pc, #0x21c]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1bc84: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1bc88: mov r1, #536
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x218)?)?;
    // 0x1bc8c: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bc90: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1bc94: ldr r1, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al, true, R::R1, R::R0, 4)?)?;
    // 0x1bc98: bl 0x2c44 <entry+0xfffffffffffe7944>
    c.bl(0x2c44)?;
    // 0x1bc9c: ldr r2, [pc, #0x200]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1bca0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1bca4: mov r1, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x21c)?)?;
    // 0x1bca8: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x1bcac: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1bcb0: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bcb4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1bcb8: bl 0x2efc <entry+0xfffffffffffe7bfc>
    c.bl(0x2efc)?;
    // 0x1bcbc: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bcc0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1bcc4: str r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 20)?)?;
    // 0x1bcc8: bne 0x1bce8 <entry+0x9e8>
    c.b(C::Ne, 0x1bce8)?;
    // 0x1bccc: ldr r2, [pc, #0x1d0]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1bcd0: mov r1, #33
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x21)?)?;
    // 0x1bcd4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1bcd8: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1bcdc: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bce0: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bce4: ldr r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 20)?)?;
    // 0x1bce8: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x1bcec: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1bcf0: bl 0x2924 <entry+0xfffffffffffe7624>
    c.bl(0x2924)?;
    // 0x1bcf4: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1bcf8: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 20)?)?;
    // 0x1bcfc: bl 0x293c <entry+0xfffffffffffe763c>
    c.bl(0x293c)?;
    // 0x1bd00: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x1bd04: bl 0x2efc <entry+0xfffffffffffe7bfc>
    c.bl(0x2efc)?;
    // 0x1bd08: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bd0c: ldr r2, [pc, #0x190]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1bd10: str r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 8)?)?;
    // 0x1bd14: mov r1, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x228)?)?;
    // 0x1bd18: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bd1c: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bd20: mov r3, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20)?)?;
    // 0x1bd24: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 8)?)?;
    // 0x1bd28: ldr r1, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 20)?)?;
    // 0x1bd2c: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    // 0x1bd30: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x1bd34: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x1bd38: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x1bd3c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1bd40: mov r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x4)?)?;
    // 0x1bd44: bl 0x8080c <entry+0x6550c>
    c.bl(0x8080c)?;
    // 0x1bd48: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1bd4c: beq 0x1bd88 <entry+0xa88>
    c.b(C::Eq, 0x1bd88)?;
    // 0x1bd50: ldr r1, [pc, #0x150]
    c.literal(R::R1, 0x1bea8)?;
    // 0x1bd54: ldr r2, [pc, #0x150]
    c.literal(R::R2, 0x1beac)?;
    // 0x1bd58: mov r3, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x31)?)?;
    // 0x1bd5c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1bd60: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x200)?)?;
    // 0x1bd64: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1bd68: ldr r1, [pc, #0x140]
    c.literal(R::R1, 0x1beb0)?;
    // 0x1bd6c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1bd70: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1bd74: ldr r2, [pc, #0x128]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1bd78: mov r1, #50
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x32)?)?;
    // 0x1bd7c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1bd80: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1bd84: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bd88: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x1bd8c: bl 0x2efc <entry+0xfffffffffffe7bfc>
    c.bl(0x2efc)?;
    // 0x1bd90: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bd94: ldr r2, [pc, #0x108]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1bd98: str r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 12)?)?;
    // 0x1bd9c: mov r1, #54
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x36)?)?;
    // 0x1bda0: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1bda4: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bda8: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bdac: mov r3, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20)?)?;
    // 0x1bdb0: ldr r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 12)?)?;
    // 0x1bdb4: ldr r1, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 20)?)?;
    // 0x1bdb8: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    // 0x1bdbc: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x1bdc0: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x1bdc4: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x1bdc8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1bdcc: mov r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x4)?)?;
    // 0x1bdd0: bl 0x8080c <entry+0x6550c>
    c.bl(0x8080c)?;
    // 0x1bdd4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1bdd8: beq 0x1be10 <entry+0xb10>
    c.b(C::Eq, 0x1be10)?;
    // 0x1bddc: ldr r1, [pc, #0xd0]
    c.literal(R::R1, 0x1beb4)?;
    // 0x1bde0: ldr r2, [pc, #0xc4]
    c.literal(R::R2, 0x1beac)?;
    // 0x1bde4: mov r3, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x3f)?)?;
    // 0x1bde8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1bdec: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x200)?)?;
    // 0x1bdf0: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1bdf4: ldr r1, [pc, #0xb4]
    c.literal(R::R1, 0x1beb0)?;
    // 0x1bdf8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1bdfc: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1be00: ldr r2, [pc, #0x9c]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1be04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1be08: mov r1, #576
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x240)?)?;
    // 0x1be0c: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1be10: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x1be14: bl 0x2efc <entry+0xfffffffffffe7bfc>
    c.bl(0x2efc)?;
    // 0x1be18: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1be1c: ldr r2, [pc, #0x80]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1be20: str r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 16)?)?;
    // 0x1be24: mov r1, #580
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x244)?)?;
    // 0x1be28: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1be2c: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1be30: mov r3, #32
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x20)?)?;
    // 0x1be34: ldr r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 16)?)?;
    // 0x1be38: ldr r1, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 20)?)?;
    // 0x1be3c: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 0)?)?;
    // 0x1be40: ldr r1, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 4)?)?;
    // 0x1be44: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x1be48: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x1be4c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1be50: mov r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x4)?)?;
    // 0x1be54: bl 0x8080c <entry+0x6550c>
    c.bl(0x8080c)?;
    // 0x1be58: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1be5c: beq 0x1be98 <entry+0xb98>
    c.b(C::Eq, 0x1be98)?;
    // 0x1be60: ldr r1, [pc, #0x50]
    c.literal(R::R1, 0x1beb8)?;
    // 0x1be64: ldr r2, [pc, #0x40]
    c.literal(R::R2, 0x1beac)?;
    // 0x1be68: mov r3, #77
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4d)?)?;
    // 0x1be6c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1be70: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x200)?)?;
    // 0x1be74: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1be78: ldr r1, [pc, #0x30]
    c.literal(R::R1, 0x1beb0)?;
    // 0x1be7c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1be80: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1be84: ldr r2, [pc, #0x18]
    c.literal(R::R2, 0x1bea4)?;
    // 0x1be88: mov r1, #78
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4e)?)?;
    // 0x1be8c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1be90: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1be94: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1be98: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x8)?)?;
    // 0x1be9c: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for value in CODEC_TRACE_ADDRESSES { c.emit(value)?; }
    c.check(0x1bebc)
}

/// Clamp plane format flags to supported device capabilities and publish
/// the resulting parameter block to the active codec runtime context.
fn emit_codec_parameter_layout(c:&mut Code)->Result<(),String>{
    c.check(0x1bebc)?;
    // 0x1bebc: ldr r0, [r0, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 152)?)?;
    // 0x1bec0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x1bec4: b 0x2b58 <entry+0xfffffffffffe7858>
    c.b(C::Al, 0x2b58)?;
    // 0x1bec8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1becc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1bed0: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x4)?)?;
    // 0x1bed4: mov r6, r2
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R2))?;
    // 0x1bed8: ldr r2, [pc, #0x1ec]
    c.literal(R::R2, 0x1c0cc)?;
    // 0x1bedc: mov r10, r1
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R1))?;
    // 0x1bee0: mov r1, #36
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x24)?)?;
    // 0x1bee4: mov r7, r3
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(3)?))?;
    // 0x1bee8: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1beec: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bef0: ldr r0, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 2648)?)?;
    // 0x1bef4: ldr lr, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::R11, 12)?)?;
    // 0x1bef8: ldr r12, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::R11, 8)?)?;
    // 0x1befc: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 0x0)?)?;
    // 0x1bf00: beq 0x1bf4c <entry+0xc4c>
    c.b(C::Eq, 0x1bf4c)?;
    // 0x1bf04: add r2, r6, r6, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::new(6)?, R::new(6)?, Shift::Lsl, 2, false)?)?;
    // 0x1bf08: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1bf0c: str r1, [r4, #0x908]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2312)?)?;
    // 0x1bf10: str r1, [r4, #0x90c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2316)?)?;
    // 0x1bf14: str r1, [r4, #0x914]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2324)?)?;
    // 0x1bf18: str r1, [r4, #0x918]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2328)?)?;
    // 0x1bf1c: add r1, r12, r12, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R12, R::R12, Shift::Lsl, 1, false)?)?;
    // 0x1bf20: lsl r2, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 1)?)?;
    // 0x1bf24: cmp r2, r1, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 3, false)?)?;
    // 0x1bf28: add r2, r7, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::new(7)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x1bf2c: ldrhs r1, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Cs, true, R::R1, R::R0, 832)?)?;
    // 0x1bf30: lsl r2, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 1)?)?;
    // 0x1bf34: orrhs r1, r1, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Orr, R::R1, R::R1, 0x1)?)?;
    // 0x1bf38: strhs r1, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Cs, false, R::R1, R::R0, 832)?)?;
    // 0x1bf3c: add r1, lr, lr, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::LR, R::LR, Shift::Lsl, 1, false)?)?;
    // 0x1bf40: cmp r2, r1, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R1, Shift::Lsl, 3, false)?)?;
    // 0x1bf44: bhs 0x1bfcc <entry+0xccc>
    c.b(C::Cs, 0x1bfcc)?;
    // 0x1bf48: b 0x1bfd8 <entry+0xcd8>
    c.b(C::Al, 0x1bfd8)?;
    // 0x1bf4c: add r1, r6, r6, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(6)?, R::new(6)?, Shift::Lsl, 2, false)?)?;
    // 0x1bf50: add r9, r12, r12, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(9)?, R::R12, R::R12, Shift::Lsl, 1, false)?)?;
    // 0x1bf54: lsl r8, r1, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::new(8)?, R::R1, Shift::Lsl, 1)?)?;
    // 0x1bf58: cmp r8, r9, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(8)?, R::new(9)?, Shift::Lsl, 3, false)?)?;
    // 0x1bf5c: bls 0x1bf78 <entry+0xc78>
    c.b(C::Ls, 0x1bf78)?;
    // 0x1bf60: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1bf64: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1bf68: str r3, [r4, #0x908]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 2312)?)?;
    // 0x1bf6c: str r2, [r4, #0x90c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2316)?)?;
    // 0x1bf70: str r2, [r4, #0x914]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2324)?)?;
    // 0x1bf74: b 0x1bf9c <entry+0xc9c>
    c.b(C::Al, 0x1bf9c)?;
    // 0x1bf78: add r3, r7, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::new(7)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x1bf7c: add r2, lr, lr, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::LR, R::LR, Shift::Lsl, 1, false)?)?;
    // 0x1bf80: lsl r3, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 1)?)?;
    // 0x1bf84: cmp r3, r2, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(3)?, R::R2, Shift::Lsl, 3, false)?)?;
    // 0x1bf88: bls 0x1c0b0 <entry+0xdb0>
    c.b(C::Ls, 0x1c0b0)?;
    // 0x1bf8c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1bf90: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1bf94: str r3, [r4, #0x908]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 2312)?)?;
    // 0x1bf98: str r2, [r4, #0x90c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2316)?)?;
    // 0x1bf9c: add r2, r7, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::new(7)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x1bfa0: add r3, lr, lr, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::LR, R::LR, Shift::Lsl, 1, false)?)?;
    // 0x1bfa4: lsl r2, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 1)?)?;
    // 0x1bfa8: cmp r2, r3, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(3)?, Shift::Lsl, 3, false)?)?;
    // 0x1bfac: movhi r1, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1bfb0: strhi r1, [r4, #0x918]
    c.emit(arm32::transfer_word(C::Hi, false, R::R1, R::new(4)?, 2328)?)?;
    // 0x1bfb4: cmp r8, r9, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(8)?, R::new(9)?, Shift::Lsl, 4, false)?)?;
    // 0x1bfb8: ldrhi r1, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Hi, true, R::R1, R::R0, 832)?)?;
    // 0x1bfbc: orrhi r1, r1, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Orr, R::R1, R::R1, 0x1)?)?;
    // 0x1bfc0: strhi r1, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Hi, false, R::R1, R::R0, 832)?)?;
    // 0x1bfc4: cmp r2, r3, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(3)?, Shift::Lsl, 4, false)?)?;
    // 0x1bfc8: bls 0x1bfd8 <entry+0xcd8>
    c.b(C::Ls, 0x1bfd8)?;
    // 0x1bfcc: ldr r1, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 832)?)?;
    // 0x1bfd0: orr r1, r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x2)?)?;
    // 0x1bfd4: str r1, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 832)?)?;
    // 0x1bfd8: add r2, r12, r12, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R12, R::R12, Shift::Lsl, 2, false)?)?;
    // 0x1bfdc: cmp r12, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R12, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 0x1bfe0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1bfe4: add r5, lr, lr, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::LR, R::LR, Shift::Lsl, 2, false)?)?;
    // 0x1bfe8: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1bfec: lsl r2, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 1)?)?;
    // 0x1bff0: movhi r1, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1bff4: lsl r5, r5, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 1)?)?;
    // 0x1bff8: cmp r2, r6, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::new(6)?, Shift::Lsl, 3, false)?)?;
    // 0x1bffc: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1c000: movlo r2, #1
    c.emit(arm32::data_immediate(C::Cc, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c004: cmp lr, r7
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::LR, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0x1c008: orr r1, r1, r2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R1, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    // 0x1c00c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1c010: movhi r2, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c014: cmp r5, r7, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(5)?, R::new(7)?, Shift::Lsl, 3, false)?)?;
    // 0x1c018: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1c01c: str r1, [r0, #0x3d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 980)?)?;
    // 0x1c020: movlo r5, #1
    c.emit(arm32::data_immediate(C::Cc, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x1c024: orr r2, r2, r5
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 0x1c028: str r2, [r0, #0x3d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 984)?)?;
    // 0x1c02c: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 16)?)?;
    // 0x1c030: cmp r2, #5
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x5)?)?;
    // 0x1c034: bhi 0x1c060 <entry+0xd60>
    c.b(C::Hi, 0x1c060)?;
    // 0x1c038: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1c03c: mov r5, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x31)?)?;
    // 0x1c040: tst r5, r1, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Tst, R::R0, R::new(5)?, R::R1, Shift::Lsl, R::R2, false)?)?;
    // 0x1c044: beq 0x1c060 <entry+0xd60>
    c.b(C::Eq, 0x1c060)?;
    // 0x1c048: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1c04c: cmp r7, lr
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::LR, Shift::Lsl, 0, false)?)?;
    // 0x1c050: movhi r2, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c054: cmp r6, r12
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(6)?, R::R12, Shift::Lsl, 0, false)?)?;
    // 0x1c058: lsl r3, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::R2, Shift::Lsl, 1)?)?;
    // 0x1c05c: orrhi r3, r1, r2, lsl #1
    c.emit(arm32::data_register(C::Hi, Op::Orr, R::new(3)?, R::R1, R::R2, Shift::Lsl, 1, false)?)?;
    // 0x1c060: str r3, [r0, #0x324]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 804)?)?;
    // 0x1c064: lsl r1, r10, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::new(10)?, Shift::Lsl, 1)?)?;
    // 0x1c068: ldr r2, [r4, #0x914]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2324)?)?;
    // 0x1c06c: ldr r7, [r4, #0x918]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(4)?, 2328)?)?;
    // 0x1c070: orr r1, r1, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R1, R::R1, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x1c074: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c078: orr r1, r1, r7, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R1, R::R1, R::new(7)?, Shift::Lsl, 3, false)?)?;
    // 0x1c07c: and r7, r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::And, R::new(7)?, R::new(3)?, 0x1)?)?;
    // 0x1c080: orr r1, r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1)?)?;
    // 0x1c084: str r1, [r0, #0x334]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 820)?)?;
    // 0x1c088: ldr r0, [r0, #0x340]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 832)?)?;
    // 0x1c08c: and r1, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::And, R::R1, R::R0, 0x1)?)?;
    // 0x1c090: and r0, r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R2, R::R0, Shift::Lsr, 1, false)?)?;
    // 0x1c094: and r2, r2, r3, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R2, R::R2, R::new(3)?, Shift::Lsr, 1, false)?)?;
    // 0x1c098: str r1, [r4, #0x91c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2332)?)?;
    // 0x1c09c: str r0, [r4, #0x920]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 2336)?)?;
    // 0x1c0a0: str r7, [r4, #0x924]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 2340)?)?;
    // 0x1c0a4: str r2, [r4, #0x928]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2344)?)?;
    // 0x1c0a8: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1c0ac: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1c0b0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1c0b4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1c0b8: str r2, [r4, #0x908]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 2312)?)?;
    // 0x1c0bc: str r1, [r4, #0x90c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2316)?)?;
    // 0x1c0c0: str r1, [r4, #0x914]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2324)?)?;
    // 0x1c0c4: str r1, [r4, #0x918]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 2328)?)?;
    // 0x1c0c8: b 0x1bfd8 <entry+0xcd8>
    c.b(C::Al, 0x1bfd8)?;
    c.emit(CONTROL_DIAGNOSTIC)?;
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_codec_parameter_diagnostics(c)?;
    emit_codec_parameter_layout(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn literal_addresses_and_codec_layout_are_exact(){
    let mut c=Code::new(START);extend(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    for (i,v) in CODEC_TRACE_ADDRESSES.iter().enumerate(){
      let offset=0x1bea0-START+i*4;
      assert_eq!(&c.bytes[offset..offset+4],&v.to_le_bytes());
    }
    assert_eq!(&c.bytes[0x1c0cc-START..0x1c0d0-START],&CONTROL_DIAGNOSTIC.to_le_bytes());
 }
}
