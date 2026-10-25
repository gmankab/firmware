// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus session codec command-controller setup and 23-way class
//! dispatcher. The underlying ARM32 instruction bytes are authored from
//! checked typed operands, NOT raw OEM opcodes. The 23-entry dispatch table
//! and 40-word pool contain firmware-revision-specific data, not executable
//! instruction words. This is reconstruction at machine-byte level only.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;

const START:usize=0x1c9c8;
const END:usize=0x1d910;

/// Sparse 23-way control-flow targets. Original class values are indexed
/// from a codec identifier whose recognized range is 0..=22.
const CLASS_HANDLERS:[u32;23]=[
    0x1ceb8, // class 0
    0x1d0c8, // class 1
    0x1d7fc, // class 2
    0x1d7fc, // class 3
    0x1d188, // class 4
    0x1d7fc, // class 5
    0x1cc24, // class 6
    0x1d7fc, // class 7
    0x1d7fc, // class 8
    0x1d224, // class 9
    0x1d7fc, // class 10
    0x1d2c0, // class 11
    0x1d7fc, // class 12
    0x1d360, // class 13
    0x1d7fc, // class 14
    0x1d7fc, // class 15
    0x1cc24, // class 16
    0x1d7fc, // class 17
    0x1cc24, // class 18
    0x1d7fc, // class 19
    0x1d7fc, // class 20
    0x1d7fc, // class 21
    0x1d42c, // class 22
];

/// Format IDs, addresses of diagnostics, and controller link constants.
const CLASS_DATA:[u32;40]=[
    0x000e53c3, // codec class data 0
    0x04011005, // codec class data 1
    0x04021001, // codec class data 2
    0xfbfeefff, // codec class data 3
    0x0401109f, // codec class data 4
    0x040110a0, // codec class data 5
    0x040110a1, // codec class data 6
    0x040110ff, // codec class data 7
    0x000e5ccb, // codec class data 8
    0x000e2788, // codec class data 9
    0x000e5d1c, // codec class data 10
    0x000e5c89, // codec class data 11
    0x000e5c4e, // codec class data 12
    0x000e5c0f, // codec class data 13
    0x000e5bd1, // codec class data 14
    0x000e5b92, // codec class data 15
    0x000e5b63, // codec class data 16
    0x000e5ae4, // codec class data 17
    0x000e5b21, // codec class data 18
    0x000e5a0f, // codec class data 19
    0x000e5a4b, // codec class data 20
    0x000e56a7, // codec class data 21
    0x000e5aa5, // codec class data 22
    0x000e5982, // codec class data 23
    0x000e59ab, // codec class data 24
    0x000e59db, // codec class data 25
    0x000e58f4, // codec class data 26
    0x000e595e, // codec class data 27
    0x000e5913, // codec class data 28
    0x000e57da, // codec class data 29
    0x000e5816, // codec class data 30
    0x000e570c, // codec class data 31
    0x000e576d, // codec class data 32
    0x000e57a5, // codec class data 33
    0x000e5d5f, // codec class data 34
    0x000e5686, // codec class data 35
    0x000e56c8, // codec class data 36
    0x000e5851, // codec class data 37
    0x000e5890, // codec class data 38
    0x000e58c0, // codec class data 39
];

fn initialize_class_command(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1c9c8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1c9cc: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1c9d0: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x24)?)?;
    // 0x1c9d4: sub sp, sp, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x800)?)?;
    // 0x1c9d8: mov r6, r1
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R1))?;
    // 0x1c9dc: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1c9e0: mov r8, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x7)?)?;
    // 0x1c9e4: mov r5, #6
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x6)?)?;
    // 0x1c9e8: ldm r6, {r0, r9, r10}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(6)?, R::R0.bit() | R::new(9)?.bit() | R::new(10)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1c9ec: orr r8, r8, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(8)?, R::new(8)?, 0x10000)?)?;
    // 0x1c9f0: orr r5, r5, #131072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x20000)?)?;
    // 0x1c9f4: bic r0, r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x3)?)?;
    // 0x1c9f8: cmp r9, r8
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::new(8)?, Shift::Lsl, 0, false)?)?;
    // 0x1c9fc: add r0, r6, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1ca00: ldr r7, [r0, #-0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R0, -4)?)?;
    // 0x1ca04: bne 0x1caf8 <entry+0x310>  
    c.b(C::Ne, 0x1caf8)?;
    // 0x1ca08: mov r0, r8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x1ca0c: bl 0x23644 <entry+0x6e5c> 
    c.bl(0x23644)?;
    // 0x1ca10: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1ca14: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x1ca18: mov r8, r7
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(7)?))?;
    // 0x1ca1c: bl 0x236c8 <entry+0x6ee0> 
    c.bl(0x236c8)?;
    // 0x1ca20: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x1ca24: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x1ca28: bne 0x1ca64 <entry+0x27c>  
    c.b(C::Ne, 0x1ca64)?;
    // 0x1ca2c: ldr r1, [pc, #0xed0]       
    c.literal(R::R1, 0x1d904)?;
    // 0x1ca30: ldr r2, [pc, #0xe8c]       
    c.literal(R::R2, 0x1d8c4)?;
    // 0x1ca34: mov r3, #206
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xce)?)?;
    // 0x1ca38: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1ca3c: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x200)?)?;
    // 0x1ca40: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ca44: ldr r1, [pc, #0xe48]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1ca48: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1ca4c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ca50: ldr r2, [pc, #0xe18]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1ca54: mov r1, #207
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xcf)?)?;
    // 0x1ca58: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1ca5c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1ca60: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1ca64: ldr r1, [pc, #0xe9c]       
    c.literal(R::R1, 0x1d908)?;
    // 0x1ca68: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ca6c: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1ca70: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ca74: ldr r7, [pc, #0xe18]       
    c.literal(R::new(7)?, 0x1d894)?;
    // 0x1ca78: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ca7c: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1ca80: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ca84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1ca88: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x1ca8c: ldr r1, [pc, #0xe78]       
    c.literal(R::R1, 0x1d90c)?;
    // 0x1ca90: str r5, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 36)?)?;
    // 0x1ca94: str r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::SP, 40)?)?;
    // 0x1ca98: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1ca9c: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1caa0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1caa4: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1caa8: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 0x1caac: mov r0, #24
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x18)?)?;
    // 0x1cab0: str r6, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 48)?)?;
    // 0x1cab4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1cab8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1cabc: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cac0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1cac4: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1cac8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cacc: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1cad0: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x20)?)?;
    // 0x1cad4: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1cad8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1cadc: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1cae0: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1cae4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1cae8: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1caec: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1caf0: str r6, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 20)?)?;
    // 0x1caf4: b 0x1d420 <entry+0xc38>  
    c.b(C::Al, 0x1d420)?;
    // 0x1caf8: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1cafc: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x1cb00: bl 0x23668 <entry+0x6e80> 
    c.bl(0x23668)?;
    // 0x1cb04: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1cb08: cmn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmn, R::R0, R::R2, 0x1)?)?;
    // 0x1cb0c: beq 0x1ccc4 <entry+0x4dc>  
    c.b(C::Eq, 0x1ccc4)?;
    // 0x1cb10: str r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 20)?)?;
    // 0x1cb14: str r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 24)?)?;
    // 0x1cb18: ldr r7, [pc, #0xd50]       
    c.literal(R::new(7)?, 0x1d870)?;
    // 0x1cb1c: mov r1, #78
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4e)?)?;
    // 0x1cb20: mov r6, r2
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R2))?;
    // 0x1cb24: ldr r0, [r4, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R2, Shift::Lsl, 2)?)?;
    // 0x1cb28: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1cb2c: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x1cb30: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1cb34: str r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 28)?)?;
    // 0x1cb38: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 0x1cb3c: mov r5, r6
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(6)?))?;
    // 0x1cb40: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x1cb44: ldr r0, [r4, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::new(6)?, Shift::Lsl, 2)?)?;
    // 0x1cb48: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1cb4c: add r6, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1cb50: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1cb54: mov r1, #592
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x250)?)?;
    // 0x1cb58: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x1cb5c: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1cb60: mov r0, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10000)?)?;
    // 0x1cb64: orr r0, r0, #67108864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x4000000)?)?;
    // 0x1cb68: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1cb6c: ldrhs r1, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Cs, true, R::R1, R::SP, 28)?)?;
    // 0x1cb70: movhs r0, #255
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0xff)?)?;
    // 0x1cb74: strhs r0, [r1, #0x88]
    c.emit(arm32::transfer_word(C::Cs, false, R::R0, R::R1, 136)?)?;
    // 0x1cb78: mov r0, r9
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(9)?))?;
    // 0x1cb7c: bl 0x23644 <entry+0x6e5c> 
    c.bl(0x23644)?;
    // 0x1cb80: ldr r2, [pc, #0xcec]       
    c.literal(R::R2, 0x1d874)?;
    // 0x1cb84: ldr r7, [pc, #0xcec]       
    c.literal(R::new(7)?, 0x1d878)?;
    // 0x1cb88: mov r3, r0
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R0))?;
    // 0x1cb8c: mov r0, #69632
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x11000)?)?;
    // 0x1cb90: orr r0, r0, #67108864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x4000000)?)?;
    // 0x1cb94: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1cb98: ble 0x1cd48 <entry+0x560>  
    c.b(C::Le, 0x1cd48)?;
    // 0x1cb9c: ldr r0, [pc, #0xcdc]       
    c.literal(R::R0, 0x1d880)?;
    // 0x1cba0: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1cba4: bgt 0x1ce1c <entry+0x634>  
    c.b(C::Gt, 0x1ce1c)?;
    // 0x1cba8: ldr r0, [pc, #0xccc]       
    c.literal(R::R0, 0x1d87c)?;
    // 0x1cbac: mov r8, r5
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(5)?))?;
    // 0x1cbb0: add r0, r9, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1cbb4: cmp r0, #22
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x16)?)?;
    // 0x1cbb8: bhi 0x1d7fc <entry+0x1014> 
    c.b(C::Hi, 0x1d7fc)?;
    // 0x1cbbc: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1cbc0: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x0)?)?;
    // 0x1cbc4: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;

    c.check(0x1cbc8)?;
    for target in CLASS_HANDLERS { c.emit(target)?; }
    c.check(0x1cc24)
}

fn dispatch_codec_class(c:&mut Code)->Result<(),String>{
    c.check(0x1cc24)?;
    // 0x1cc24: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 28)?)?;
    // 0x1cc28: ldr r2, [pc, #0xc40]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1cc2c: mov r1, #35
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x23)?)?;
    // 0x1cc30: mov r7, r3
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(3)?))?;
    // 0x1cc34: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 0x1cc38: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1cc3c: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1cc40: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1cc44: ldr r1, [pc, #0xc50]       
    c.literal(R::R1, 0x1d89c)?;
    // 0x1cc48: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1cc4c: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1cc50: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cc54: ldr r1, [pc, #0xc38]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1cc58: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1cc5c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cc60: add r0, r4, #612
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x264)?)?;
    // 0x1cc64: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1cc68: mov r2, #676
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2a4)?)?;
    // 0x1cc6c: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 0x1cc70: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1cc74: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 0x1cc78: add r0, r4, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x1cc7c: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1cc80: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1cc84: add r4, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1cc88: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1cc8c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1cc90: str r0, [r4, #0xb50]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R0, R::new(4)?, 2896)?)?;
    // 0x1cc94: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1cc98: bl 0x23cf0 <entry+0x7508> 
    c.bl(0x23cf0)?;
    // 0x1cc9c: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1cca0: add r1, r6, #664
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x298)?)?;
    // 0x1cca4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1cca8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1ccac: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1ccb0: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1ccb4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1ccb8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1ccbc: mov r1, r9
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0x1ccc0: b 0x1d400 <entry+0xc18>  
    c.b(C::Al, 0x1d400)?;
    // 0x1ccc4: ldr r6, [pc, #0xbf8]       
    c.literal(R::new(6)?, 0x1d8c4)?;
    // 0x1ccc8: ldr r1, [pc, #0xc2c]       
    c.literal(R::R1, 0x1d8fc)?;
    // 0x1cccc: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1ccd0: mov r3, #576
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x240)?)?;
    // 0x1ccd4: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1ccd8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ccdc: mov r5, r7
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(7)?))?;
    // 0x1cce0: ldr r7, [pc, #0xbac]       
    c.literal(R::new(7)?, 0x1d894)?;
    // 0x1cce4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cce8: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1ccec: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ccf0: ldr r1, [pc, #0xc08]       
    c.literal(R::R1, 0x1d900)?;
    // 0x1ccf4: mov r3, #65
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x41)?)?;
    // 0x1ccf8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1ccfc: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1cd00: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::SP, 0)?)?;
    // 0x1cd04: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x200)?)?;
    // 0x1cd08: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cd0c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cd10: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1cd14: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cd18: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1cd1c: mov r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4)?)?;
    // 0x1cd20: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x1cd24: mov r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2)?)?;
    // 0x1cd28: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::SP, 0)?)?;
    // 0x1cd2c: str r5, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 4)?)?;
    // 0x1cd30: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1cd34: bl 0x23ca8 <entry+0x74c0> 
    c.bl(0x23ca8)?;
    // 0x1cd38: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1cd3c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1cd40: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1cd44: b 0x1d420 <entry+0xc38>  
    c.b(C::Al, 0x1d420)?;
    // 0x1cd48: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 28)?)?;
    // 0x1cd4c: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 24)?)?;
    // 0x1cd50: mov r5, #6
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x6)?)?;
    // 0x1cd54: cmp r9, r8
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::new(8)?, Shift::Lsl, 0, false)?)?;
    // 0x1cd58: orr r5, r5, #131072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x20000)?)?;
    // 0x1cd5c: beq 0x1ca10 <entry+0x228>  
    c.b(C::Eq, 0x1ca10)?;
    // 0x1cd60: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cd64: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x10000)?)?;
    // 0x1cd68: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1cd6c: beq 0x1d504 <entry+0xd1c>  
    c.b(C::Eq, 0x1d504)?;
    // 0x1cd70: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1cd74: orr r0, r0, #2162688
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x210000)?)?;
    // 0x1cd78: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1cd7c: bne 0x1d7fc <entry+0x1014> 
    c.b(C::Ne, 0x1d7fc)?;
    // 0x1cd80: ldr r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 20)?)?;
    // 0x1cd84: ldr r1, [pc, #0xb4c]       
    c.literal(R::R1, 0x1d8d8)?;
    // 0x1cd88: ldr r2, [pc, #0xb34]       
    c.literal(R::R2, 0x1d8c4)?;
    // 0x1cd8c: mov r3, #764
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x2fc)?)?;
    // 0x1cd90: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 8)?)?;
    // 0x1cd94: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 0)?)?;
    // 0x1cd98: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cd9c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cda0: ldr r1, [pc, #0xaec]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1cda4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cda8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cdac: ldr r1, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 8)?)?;
    // 0x1cdb0: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 28)?)?;
    // 0x1cdb4: ldr r6, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 24)?)?;
    // 0x1cdb8: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1cdbc: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1cdc0: bl 0x1c614 <entry+0xfffffffffffffe2c>
    c.bl(0x1c614)?;
    // 0x1cdc4: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1cdc8: bne 0x1d828 <entry+0x1040> 
    c.b(C::Ne, 0x1d828)?;
    // 0x1cdcc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x1cdd0: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x20)?)?;
    // 0x1cdd4: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0x1cdd8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1cddc: orr r0, r0, #2228224
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x220000)?)?;
    // 0x1cde0: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 36)?)?;
    // 0x1cde4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1cde8: str r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::SP, 40)?)?;
    // 0x1cdec: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 0x1cdf0: mov r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10)?)?;
    // 0x1cdf4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1cdf8: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1cdfc: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1ce00: ldr r1, [pc, #0xad8]       
    c.literal(R::R1, 0x1d8e0)?;
    // 0x1ce04: ldr r2, [pc, #0xab8]       
    c.literal(R::R2, 0x1d8c4)?;
    // 0x1ce08: mov r3, #15
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xf)?)?;
    // 0x1ce0c: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::SP, 0)?)?;
    // 0x1ce10: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1ce14: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x300)?)?;
    // 0x1ce18: b 0x1d838 <entry+0x1050> 
    c.b(C::Al, 0x1d838)?;
    // 0x1ce1c: ldr r0, [pc, #0xa60]       
    c.literal(R::R0, 0x1d884)?;
    // 0x1ce20: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 0x1ce24: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1ce28: beq 0x1d5b4 <entry+0xdcc>  
    c.b(C::Eq, 0x1d5b4)?;
    // 0x1ce2c: ldr r0, [pc, #0xa54]       
    c.literal(R::R0, 0x1d888)?;
    // 0x1ce30: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1ce34: beq 0x1d724 <entry+0xf3c>  
    c.b(C::Eq, 0x1d724)?;
    // 0x1ce38: ldr r0, [pc, #0xa4c]       
    c.literal(R::R0, 0x1d88c)?;
    // 0x1ce3c: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1ce40: bne 0x1d7fc <entry+0x1014> 
    c.b(C::Ne, 0x1d7fc)?;
    // 0x1ce44: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 28)?)?;
    // 0x1ce48: ldr r2, [pc, #0xa20]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1ce4c: mov r1, #155
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x9b)?)?;
    // 0x1ce50: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x300)?)?;
    // 0x1ce54: ldr r0, [r4, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1ce58: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1ce5c: ldr r1, [pc, #0xa4c]       
    c.literal(R::R1, 0x1d8b0)?;
    // 0x1ce60: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ce64: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1ce68: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ce6c: ldr r1, [pc, #0xa20]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1ce70: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ce74: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1ce78: ldr r0, [r4, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1ce7c: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 20)?)?;
    // 0x1ce80: mov r2, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x400)?)?;
    // 0x1ce84: add r0, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x4)?)?;
    // 0x1ce88: add r1, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xc)?)?;
    // 0x1ce8c: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xc00)?)?;
    // 0x1ce90: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1ce94: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1ce98: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1ce9c: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1cea0: ldr r0, [r4, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1cea4: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 0x1cea8: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1ceac: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1ceb0: str r2, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R2, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1ceb4: b 0x1d854 <entry+0x106c> 
    c.b(C::Al, 0x1d854)?;
    // 0x1ceb8: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1cebc: ldr r2, [pc, #0x9ac]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1cec0: mov r1, #612
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x264)?)?;
    // 0x1cec4: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 16)?)?;
    // 0x1cec8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1cecc: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1ced0: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1ced4: ldr r9, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 28)?)?;
    // 0x1ced8: ldr r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 32)?)?;
    // 0x1cedc: ldr r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 24)?)?;
    // 0x1cee0: ldr r0, [r9, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(9)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1cee4: add r2, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R1, R::R1, Shift::Lsl, 1, false)?)?;
    // 0x1cee8: add r2, r0, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R0, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x1ceec: ldr r5, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R2, 8)?)?;
    // 0x1cef0: mov r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xff)?)?;
    // 0x1cef4: orr r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf00)?)?;
    // 0x1cef8: tst r5, r2
    c.emit(arm32::data_register(C::Al, Op::Tst, R::R0, R::new(5)?, R::R2, Shift::Lsl, 0, false)?)?;
    // 0x1cefc: beq 0x1cf4c <entry+0x764>  
    c.b(C::Eq, 0x1cf4c)?;
    // 0x1cf00: ldr r1, [pc, #0x9e4]       
    c.literal(R::R1, 0x1d8ec)?;
    // 0x1cf04: ldr r2, [pc, #0x9b8]       
    c.literal(R::R2, 0x1d8c4)?;
    // 0x1cf08: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cf0c: mov r3, #620
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x26c)?)?;
    // 0x1cf10: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 0)?)?;
    // 0x1cf14: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cf18: ldr r1, [pc, #0x974]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1cf1c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cf20: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cf24: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 8)?)?;
    // 0x1cf28: ldr r0, [r9, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 128)?)?;
    // 0x1cf2c: mov r3, #22
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x16)?)?;
    // 0x1cf30: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1cf34: stm sp, {r2, r4}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R2.bit() | R::new(4)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1cf38: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1cf3c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2)?)?;
    // 0x1cf40: bl 0x23ca8 <entry+0x74c0> 
    c.bl(0x23ca8)?;
    // 0x1cf44: ldr r0, [r9, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(9)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1cf48: ldr r1, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 32)?)?;
    // 0x1cf4c: add r1, r1, r1, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R1, R::R1, Shift::Lsl, 1, false)?)?;
    // 0x1cf50: str r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 16)?)?;
    // 0x1cf54: ldr r2, [pc, #0x914]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1cf58: mov r10, r8
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(8)?))?;
    // 0x1cf5c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1cf60: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 2, false)?)?;
    // 0x1cf64: mov r1, #117
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x75)?)?;
    // 0x1cf68: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 0x1cf6c: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1cf70: lsr r0, r0, #17
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 17)?)?;
    // 0x1cf74: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 0x1cf78: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1cf7c: movhi r0, #1
    c.emit(arm32::data_immediate(C::Hi, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1cf80: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1cf84: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1cf88: ldr r1, [pc, #0x960]       
    c.literal(R::R1, 0x1d8f0)?;
    // 0x1cf8c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1cf90: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cf94: ldr r1, [pc, #0x8f8]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1cf98: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1cf9c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1cfa0: ldr r2, [r9, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::new(9)?, R::new(10)?, Shift::Lsl, 2)?)?;
    // 0x1cfa4: mov r8, #904
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x388)?)?;
    // 0x1cfa8: orr r8, r8, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(8)?, R::new(8)?, 0x1800)?)?;
    // 0x1cfac: ldr r0, [r2, r8]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R2, R::new(8)?, Shift::Lsl, 0)?)?;
    // 0x1cfb0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1cfb4: bne 0x1cffc <entry+0x814>  
    c.b(C::Ne, 0x1cffc)?;
    // 0x1cfb8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1cfbc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1cfc0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1cfc4: bl 0x7e544 <entry+0x61d5c>
    c.bl(0x7e544)?;
    // 0x1cfc8: ldr r0, [r9, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(9)?, R::new(10)?, Shift::Lsl, 2)?)?;
    // 0x1cfcc: ldr r0, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::new(8)?, Shift::Lsl, 0)?)?;
    // 0x1cfd0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1cfd4: bne 0x1cffc <entry+0x814>  
    c.b(C::Ne, 0x1cffc)?;
    // 0x1cfd8: ldr r2, [pc, #0x890]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1cfdc: mov r1, #127
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7f)?)?;
    // 0x1cfe0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1cfe4: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1cfe8: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1cfec: ldr r0, [r9, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(9)?, R::new(10)?, Shift::Lsl, 2)?)?;
    // 0x1cff0: ldr r0, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::new(8)?, Shift::Lsl, 0)?)?;
    // 0x1cff4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1cff8: beq 0x1d85c <entry+0x1074> 
    c.b(C::Eq, 0x1d85c)?;
    // 0x1cffc: mov r1, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x10000)?)?;
    // 0x1d000: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1d004: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 0x1d008: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 0)?)?;
    // 0x1d00c: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 4)?)?;
    // 0x1d010: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 8)?)?;
    // 0x1d014: bl 0x7edc8 <entry+0x625e0>
    c.bl(0x7edc8)?;
    // 0x1d018: add r5, r5, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::new(5)?, 0x10000)?)?;
    // 0x1d01c: mov r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x1)?)?;
    // 0x1d020: ldr r0, [r9, r10, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(9)?, R::new(10)?, Shift::Lsl, 2)?)?;
    // 0x1d024: mov r1, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x10000)?)?;
    // 0x1d028: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1d02c: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 0x1d030: ldr r0, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::new(8)?, Shift::Lsl, 0)?)?;
    // 0x1d034: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 0)?)?;
    // 0x1d038: str r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 4)?)?;
    // 0x1d03c: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 8)?)?;
    // 0x1d040: bl 0x7edc8 <entry+0x625e0>
    c.bl(0x7edc8)?;
    // 0x1d044: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x1)?)?;
    // 0x1d048: add r5, r5, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::new(5)?, 0x10000)?)?;
    // 0x1d04c: cmp r4, #8
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x8)?)?;
    // 0x1d050: bne 0x1d020 <entry+0x838>  
    c.b(C::Ne, 0x1d020)?;
    // 0x1d054: ldr r0, [r9, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 144)?)?;
    // 0x1d058: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1d05c: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1d060: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    // 0x1d064: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1d068: add r5, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x20)?)?;
    // 0x1d06c: add r1, r5, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(5)?, 0x10)?)?;
    // 0x1d070: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 36)?)?;
    // 0x1d074: add r0, r6, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x14)?)?;
    // 0x1d078: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d07c: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 40)?)?;
    // 0x1d080: str r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 44)?)?;
    // 0x1d084: ldm r0, {r3, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::new(3)?.bit() | R::new(4)?.bit() | R::new(6)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d088: mov r0, #28
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1c)?)?;
    // 0x1d08c: stm r1, {r3, r4, r6}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::new(3)?.bit() | R::new(4)?.bit() | R::new(6)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d090: ldr r1, [pc, #0x85c]       
    c.literal(R::R1, 0x1d8f4)?;
    // 0x1d094: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1d098: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d09c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d0a0: ldr r1, [pc, #0x7ec]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d0a4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d0a8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d0ac: ldr r0, [r9, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 128)?)?;
    // 0x1d0b0: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x1d0b4: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1d0b8: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1d0bc: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1d0c0: ldr r0, [r9, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 144)?)?;
    // 0x1d0c4: b 0x1d5ac <entry+0xdc4>  
    c.b(C::Al, 0x1d5ac)?;
    // 0x1d0c8: ldr r5, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 20)?)?;
    // 0x1d0cc: ldr r2, [pc, #0x79c]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1d0d0: mov r1, #170
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xaa)?)?;
    // 0x1d0d4: orr r1, r1, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x200)?)?;
    // 0x1d0d8: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 0x1d0dc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1d0e0: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d0e4: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d0e8: ldr r2, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 8)?)?;
    // 0x1d0ec: ldr r1, [pc, #0x7f0]       
    c.literal(R::R1, 0x1d8e4)?;
    // 0x1d0f0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d0f4: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d0f8: ldr r8, [pc, #0x794]       
    c.literal(R::new(8)?, 0x1d894)?;
    // 0x1d0fc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d100: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 0x1d104: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d108: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 28)?)?;
    // 0x1d10c: mov r6, r7
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x1d110: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d114: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1d118: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1d11c: add r0, r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x1)?)?;
    // 0x1d120: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0x1d124: add r6, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::SP, 0x20)?)?;
    // 0x1d128: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 36)?)?;
    // 0x1d12c: add r0, r5, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x14)?)?;
    // 0x1d130: add r1, r6, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x10)?)?;
    // 0x1d134: ldr r2, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 8)?)?;
    // 0x1d138: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 40)?)?;
    // 0x1d13c: str r9, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::SP, 44)?)?;
    // 0x1d140: ldm r0, {r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::new(3)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d144: mov r0, #28
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1c)?)?;
    // 0x1d148: stm r1, {r3, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::new(3)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d14c: ldr r1, [pc, #0x794]       
    c.literal(R::R1, 0x1d8e8)?;
    // 0x1d150: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1d154: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d158: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d15c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d160: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 0x1d164: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d168: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1d16c: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x1d170: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d174: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1d178: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1d17c: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d180: str r9, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::R0, 20)?)?;
    // 0x1d184: b 0x1d420 <entry+0xc38>  
    c.b(C::Al, 0x1d420)?;
    // 0x1d188: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 28)?)?;
    // 0x1d18c: str r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 16)?)?;
    // 0x1d190: ldr r2, [pc, #0x6d8]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1d194: mov r1, #181
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb5)?)?;
    // 0x1d198: mov r7, r3
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(3)?))?;
    // 0x1d19c: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x300)?)?;
    // 0x1d1a0: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1d1a4: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d1a8: ldr r1, [pc, #0x6fc]       
    c.literal(R::R1, 0x1d8ac)?;
    // 0x1d1ac: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d1b0: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1d1b4: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d1b8: ldr r1, [pc, #0x6d4]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d1bc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d1c0: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d1c4: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x1d1c8: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d1cc: mov r2, #116
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x74)?)?;
    // 0x1d1d0: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d1d4: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1d1d8: mov r0, #852
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x354)?)?;
    // 0x1d1dc: add r4, r4, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::new(4)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x1d1e0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0x1d1e4: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1d1e8: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1800)?)?;
    // 0x1d1ec: str r1, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, false, R::R1, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1d1f0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d1f4: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d1f8: bl 0x23cf0 <entry+0x7508> 
    c.bl(0x23cf0)?;
    // 0x1d1fc: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1d200: add r1, r6, #104
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x68)?)?;
    // 0x1d204: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1d208: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d20c: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1d210: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d214: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d218: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d21c: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 16)?)?;
    // 0x1d220: b 0x1d400 <entry+0xc18>  
    c.b(C::Al, 0x1d400)?;
    // 0x1d224: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 28)?)?;
    // 0x1d228: mov r7, r2
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x1d22c: ldr r2, [pc, #0x63c]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1d230: mov r1, #1004
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3ec)?)?;
    // 0x1d234: mov r9, r3
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(3)?))?;
    // 0x1d238: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1d23c: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d240: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d244: ldr r1, [pc, #0x658]       
    c.literal(R::R1, 0x1d8a4)?;
    // 0x1d248: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d24c: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d250: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d254: ldr r1, [pc, #0x638]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d258: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d25c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d260: add r0, r4, #224
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0xe0)?)?;
    // 0x1d264: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d268: mov r2, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5c)?)?;
    // 0x1d26c: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 0x1d270: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1d274: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 0x1d278: add r0, r4, r9, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(9)?, Shift::Lsl, 2, false)?)?;
    // 0x1d27c: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1d280: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1d284: add r4, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1d288: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d28c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d290: str r0, [r4, #0xb50]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R0, R::new(4)?, 2896)?)?;
    // 0x1d294: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d298: bl 0x23cf0 <entry+0x7508> 
    c.bl(0x23cf0)?;
    // 0x1d29c: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1d2a0: add r1, r6, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x50)?)?;
    // 0x1d2a4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1d2a8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d2ac: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1d2b0: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d2b4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d2b8: add r1, r7, #5
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(7)?, 0x5)?)?;
    // 0x1d2bc: b 0x1d3fc <entry+0xc14>  
    c.b(C::Al, 0x1d3fc)?;
    // 0x1d2c0: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 28)?)?;
    // 0x1d2c4: mov r7, r2
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x1d2c8: ldr r2, [pc, #0x5a0]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1d2cc: mov r1, #209
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xd1)?)?;
    // 0x1d2d0: mov r9, r3
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(3)?))?;
    // 0x1d2d4: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x300)?)?;
    // 0x1d2d8: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1d2dc: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d2e0: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d2e4: ldr r1, [pc, #0x5bc]       
    c.literal(R::R1, 0x1d8a8)?;
    // 0x1d2e8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d2ec: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d2f0: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d2f4: ldr r1, [pc, #0x598]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d2f8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d2fc: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d300: add r0, r4, #120
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x78)?)?;
    // 0x1d304: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d308: mov r2, #104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x68)?)?;
    // 0x1d30c: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 0x1d310: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1d314: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 0x1d318: add r0, r4, r9, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(9)?, Shift::Lsl, 2, false)?)?;
    // 0x1d31c: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1d320: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1d324: add r4, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1d328: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d32c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d330: str r0, [r4, #0xb50]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R0, R::new(4)?, 2896)?)?;
    // 0x1d334: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d338: bl 0x23cf0 <entry+0x7508> 
    c.bl(0x23cf0)?;
    // 0x1d33c: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1d340: add r1, r6, #92
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x5c)?)?;
    // 0x1d344: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1d348: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d34c: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1d350: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d354: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d358: add r1, r7, #7
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(7)?, 0x7)?)?;
    // 0x1d35c: b 0x1d3fc <entry+0xc14>  
    c.b(C::Al, 0x1d3fc)?;
    // 0x1d360: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 28)?)?;
    // 0x1d364: mov r7, r2
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x1d368: ldr r2, [pc, #0x500]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1d36c: mov r1, #6
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x6)?)?;
    // 0x1d370: mov r9, r3
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(3)?))?;
    // 0x1d374: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 0x1d378: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1d37c: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d380: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d384: ldr r1, [pc, #0x514]       
    c.literal(R::R1, 0x1d8a0)?;
    // 0x1d388: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d38c: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d390: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d394: ldr r1, [pc, #0x4f8]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d398: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d39c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d3a0: add r0, r4, #316
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x13c)?)?;
    // 0x1d3a4: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d3a8: mov r2, #296
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x128)?)?;
    // 0x1d3ac: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 0x1d3b0: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1d3b4: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 0x1d3b8: add r0, r4, r9, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(9)?, Shift::Lsl, 2, false)?)?;
    // 0x1d3bc: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1d3c0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1d3c4: add r4, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1d3c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d3cc: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d3d0: str r0, [r4, #0xb50]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R0, R::new(4)?, 2896)?)?;
    // 0x1d3d4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d3d8: bl 0x23cf0 <entry+0x7508> 
    c.bl(0x23cf0)?;
    // 0x1d3dc: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1d3e0: add r1, r6, #284
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x11c)?)?;
    // 0x1d3e4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1d3e8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d3ec: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1d3f0: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d3f4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d3f8: add r1, r7, #9
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(7)?, 0x9)?)?;
    // 0x1d3fc: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d400: ldr r3, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 24)?)?;
    // 0x1d404: bl 0x1c93c <entry+0x154>  
    c.bl(0x1c93c)?;
    // 0x1d408: ldr r0, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 144)?)?;
    // 0x1d40c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1d410: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1d414: ldr r0, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 144)?)?;
    // 0x1d418: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1d41c: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1d420: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d424: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1d428: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1d42c: ldr r5, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 28)?)?;
    // 0x1d430: mov r7, r2
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R2))?;
    // 0x1d434: ldr r2, [pc, #0x434]       
    c.literal(R::R2, 0x1d870)?;
    // 0x1d438: mov r1, #1088
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x440)?)?;
    // 0x1d43c: mov r9, r3
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(3)?))?;
    // 0x1d440: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1d444: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d448: ldr r0, [r5, r8, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::new(8)?, Shift::Lsl, 2)?)?;
    // 0x1d44c: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 0x1d450: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1d454: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1d458: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1d45c: bne 0x1d478 <entry+0xc90>  
    c.b(C::Ne, 0x1d478)?;
    // 0x1d460: ldr r1, [pc, #0x428]       
    c.literal(R::R1, 0x1d890)?;
    // 0x1d464: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d468: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d46c: ldr r1, [pc, #0x420]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d470: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d474: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d478: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d47c: ldr r1, [pc, #0x414]       
    c.literal(R::R1, 0x1d898)?;
    // 0x1d480: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d484: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d488: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d48c: ldr r1, [pc, #0x400]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d490: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d494: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d498: add r0, r4, #264
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x108)?)?;
    // 0x1d49c: mov r2, #588
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x24c)?)?;
    // 0x1d4a0: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d4a4: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1400)?)?;
    // 0x1d4a8: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x400)?)?;
    // 0x1d4ac: bl 0x471c <entry+0xfffffffffffe7f34>
    c.bl(0x471c)?;
    // 0x1d4b0: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 0x1d4b4: add r0, r4, r9, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(9)?, Shift::Lsl, 2, false)?)?;
    // 0x1d4b8: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1d4bc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1d4c0: add r4, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1d4c4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d4c8: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d4cc: str r0, [r4, #0xb50]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R0, R::new(4)?, 2896)?)?;
    // 0x1d4d0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d4d4: bl 0x23cf0 <entry+0x7508> 
    c.bl(0x23cf0)?;
    // 0x1d4d8: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x1d4dc: ldr r3, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 24)?)?;
    // 0x1d4e0: add r1, r6, #1600
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x640)?)?;
    // 0x1d4e4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1d4e8: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d4ec: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1d4f0: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d4f4: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d4f8: add r1, r7, #18
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(7)?, 0x12)?)?;
    // 0x1d4fc: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1d500: b 0x1d404 <entry+0xc1c>  
    c.b(C::Al, 0x1d404)?;
    // 0x1d504: ldr r1, [pc, #0x3c0]       
    c.literal(R::R1, 0x1d8cc)?;
    // 0x1d508: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d50c: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1d510: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d514: ldr r6, [pc, #0x378]       
    c.literal(R::new(6)?, 0x1d894)?;
    // 0x1d518: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d51c: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d520: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d524: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1d528: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0x1d52c: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x1d530: bl 0x1c614 <entry+0xfffffffffffffe2c>
    c.bl(0x1c614)?;
    // 0x1d534: ldr r1, [pc, #0x394]       
    c.literal(R::R1, 0x1d8d0)?;
    // 0x1d538: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d53c: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1d540: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d544: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d548: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d54c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d550: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x1)?)?;
    // 0x1d554: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x1d558: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1d55c: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x20)?)?;
    // 0x1d560: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1d564: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 36)?)?;
    // 0x1d568: mov r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10)?)?;
    // 0x1d56c: str r10, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::SP, 40)?)?;
    // 0x1d570: str r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 44)?)?;
    // 0x1d574: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1d578: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1d57c: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1d580: ldr r1, [pc, #0x34c]       
    c.literal(R::R1, 0x1d8d4)?;
    // 0x1d584: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d588: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0x1d58c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d590: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d594: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d598: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d59c: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d5a0: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1d5a4: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1d5a8: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d5ac: str r7, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 20)?)?;
    // 0x1d5b0: b 0x1d420 <entry+0xc38>  
    c.b(C::Al, 0x1d420)?;
    // 0x1d5b4: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d5b8: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 28)?)?;
    // 0x1d5bc: ldr r1, [pc, #0x2f8]       
    c.literal(R::R1, 0x1d8bc)?;
    // 0x1d5c0: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d5c4: ldr r5, [r0, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(5)?, R::R0, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1d5c8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d5cc: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d5d0: ldr r1, [pc, #0x2bc]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d5d4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d5d8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d5dc: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 20)?)?;
    // 0x1d5e0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1d5e4: beq 0x1d6bc <entry+0xed4>  
    c.b(C::Eq, 0x1d6bc)?;
    // 0x1d5e8: ldr r7, [pc, #0x280]       
    c.literal(R::new(7)?, 0x1d870)?;
    // 0x1d5ec: add r10, r6, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::new(6)?, 0x18)?)?;
    // 0x1d5f0: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0x1d5f4: add r8, r5, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::new(5)?, 0x4)?)?;
    // 0x1d5f8: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 0x1d5fc: mov r1, #82
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x52)?)?;
    // 0x1d600: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x1d604: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x300)?)?;
    // 0x1d608: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0x1d60c: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x1d610: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 0x1d614: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1d618: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d61c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d620: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d624: ldm r10, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(10)?, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d628: add r2, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0x1d62c: add r2, r8, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::new(8)?, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x1d630: str r1, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 4)?)?;
    // 0x1d634: mov r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xff)?)?;
    // 0x1d638: orr r2, r2, #3840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0xf00)?)?;
    // 0x1d63c: tst r1, r2
    c.emit(arm32::data_register(C::Al, Op::Tst, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    // 0x1d640: beq 0x1d698 <entry+0xeb0>  
    c.b(C::Eq, 0x1d698)?;
    // 0x1d644: stm sp, {r1, r9}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R1.bit() | R::new(9)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d648: ldr r1, [pc, #0x270]       
    c.literal(R::R1, 0x1d8c0)?;
    // 0x1d64c: ldr r2, [pc, #0x270]       
    c.literal(R::R2, 0x1d8c4)?;
    // 0x1d650: mov r3, #86
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x56)?)?;
    // 0x1d654: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1d658: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x300)?)?;
    // 0x1d65c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d660: ldr r1, [pc, #0x22c]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d664: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1d668: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d66c: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 28)?)?;
    // 0x1d670: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x1d674: ldr r1, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 8)?)?;
    // 0x1d678: mov r3, #22
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x16)?)?;
    // 0x1d67c: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1d680: ldr r0, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 128)?)?;
    // 0x1d684: str r9, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::SP, 0)?)?;
    // 0x1d688: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 4)?)?;
    // 0x1d68c: mov r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2)?)?;
    // 0x1d690: bl 0x23ca8 <entry+0x74c0> 
    c.bl(0x23ca8)?;
    // 0x1d694: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 0x1d698: ldr r1, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 8)?)?;
    // 0x1d69c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0x1d6a0: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(9)?, 0x1)?)?;
    // 0x1d6a4: add r10, r10, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::new(10)?, 0xc)?)?;
    // 0x1d6a8: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x1d6ac: str r1, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 12)?)?;
    // 0x1d6b0: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 20)?)?;
    // 0x1d6b4: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1d6b8: blo 0x1d5f8 <entry+0xe10>  
    c.b(C::Cc, 0x1d5f8)?;
    // 0x1d6bc: add r0, r4, #159
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(4)?, 0x9f)?)?;
    // 0x1d6c0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1d6c4: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 28)?)?;
    // 0x1d6c8: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x1d6cc: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x20)?)?;
    // 0x1d6d0: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 36)?)?;
    // 0x1d6d4: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d6d8: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 40)?)?;
    // 0x1d6dc: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 12)?)?;
    // 0x1d6e0: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 0x1d6e4: mov r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x14)?)?;
    // 0x1d6e8: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1d6ec: str r5, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 48)?)?;
    // 0x1d6f0: ldr r3, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(6)?, 16)?)?;
    // 0x1d6f4: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1d6f8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x1)?)?;
    // 0x1d6fc: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1d700: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1d704: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d708: ldr r1, [pc, #0x1b8]       
    c.literal(R::R1, 0x1d8c8)?;
    // 0x1d70c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d710: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d714: ldr r1, [pc, #0x178]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d718: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d71c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d720: b 0x1d7e4 <entry+0xffc>  
    c.b(C::Al, 0x1d7e4)?;
    // 0x1d724: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1d728: mov r8, r4
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(4)?))?;
    // 0x1d72c: ldr r4, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 28)?)?;
    // 0x1d730: ldr r1, [pc, #0x17c]       
    c.literal(R::R1, 0x1d8b4)?;
    // 0x1d734: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d738: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d73c: ldr r5, [r4, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(5)?, R::new(4)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1d740: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d744: ldr r1, [pc, #0x148]       
    c.literal(R::R1, 0x1d894)?;
    // 0x1d748: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d74c: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d750: ldr r0, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 20)?)?;
    // 0x1d754: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1d758: beq 0x1d788 <entry+0xfa0>  
    c.b(C::Eq, 0x1d788)?;
    // 0x1d75c: add r1, r6, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x18)?)?;
    // 0x1d760: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1d764: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1d768: ldr r7, [r1], #12
    c.emit(arm32::transfer_word_postindexed(C::Al, true, R::new(7)?, R::R1, 12)?)?;
    // 0x1d76c: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x1)?)?;
    // 0x1d770: cmp r3, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(3)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1d774: add r7, r7, r7, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(7)?, R::new(7)?, R::new(7)?, Shift::Lsl, 1, false)?)?;
    // 0x1d778: add r7, r5, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(7)?, R::new(5)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x1d77c: str r2, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(7)?, 8)?)?;
    // 0x1d780: str r2, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(7)?, 12)?)?;
    // 0x1d784: blo 0x1d768 <entry+0xf80>  
    c.b(C::Cc, 0x1d768)?;
    // 0x1d788: add r0, r8, #160
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0xa0)?)?;
    // 0x1d78c: ldr r1, [pc, #0x124]       
    c.literal(R::R1, 0x1d8b8)?;
    // 0x1d790: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1d794: str r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 36)?)?;
    // 0x1d798: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 0x1d79c: str r2, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 40)?)?;
    // 0x1d7a0: ldr r0, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 12)?)?;
    // 0x1d7a4: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 0x1d7a8: mov r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x14)?)?;
    // 0x1d7ac: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1d7b0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d7b4: str r5, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 48)?)?;
    // 0x1d7b8: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d7bc: ldr r1, [pc, #0xd0]        
    c.literal(R::R1, 0x1d894)?;
    // 0x1d7c0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d7c4: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d7c8: ldr r3, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(6)?, 16)?)?;
    // 0x1d7cc: ldr r0, [r4, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 128)?)?;
    // 0x1d7d0: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x1d7d4: add r1, sp, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x20)?)?;
    // 0x1d7d8: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x1)?)?;
    // 0x1d7dc: movne r3, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1d7e0: bl 0x2a198 <entry+0xd9b0> 
    c.bl(0x2a198)?;
    // 0x1d7e4: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d7e8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1d7ec: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1d7f0: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d7f4: str r5, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 20)?)?;
    // 0x1d7f8: b 0x1d420 <entry+0xc38>  
    c.b(C::Al, 0x1d420)?;
    // 0x1d7fc: ldr r1, [pc, #0xf4]        
    c.literal(R::R1, 0x1d8f8)?;
    // 0x1d800: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d804: mov r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x1)?)?;
    // 0x1d808: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d80c: ldr r1, [pc, #0x80]        
    c.literal(R::R1, 0x1d894)?;
    // 0x1d810: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1d814: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d818: ldr r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 28)?)?;
    // 0x1d81c: ldr r0, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 144)?)?;
    // 0x1d820: str r4, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::R0, 20)?)?;
    // 0x1d824: b 0x1d420 <entry+0xc38>  
    c.b(C::Al, 0x1d420)?;
    // 0x1d828: ldr r1, [pc, #0xac]        
    c.literal(R::R1, 0x1d8dc)?;
    // 0x1d82c: ldr r2, [pc, #0x90]        
    c.literal(R::R2, 0x1d8c4)?;
    // 0x1d830: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1d834: mov r3, #788
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x314)?)?;
    // 0x1d838: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d83c: ldr r1, [pc, #0x50]        
    c.literal(R::R1, 0x1d894)?;
    // 0x1d840: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1d844: bl 0x2ec94 <entry+0x124ac>
    c.bl(0x2ec94)?;
    // 0x1d848: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d84c: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 0x1d850: bl 0x80af0 <entry+0x64308>
    c.bl(0x80af0)?;
    // 0x1d854: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x1d858: b 0x1d418 <entry+0xc30>  
    c.b(C::Al, 0x1d418)?;
    // 0x1d85c: ldr r2, [pc, #0xc]         
    c.literal(R::R2, 0x1d870)?;
    // 0x1d860: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1d864: mov r1, #652
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x28c)?)?;
    // 0x1d868: bl 0x17b0 <entry+0xfffffffffffe4fc8>
    c.bl(0x17b0)?;
    // 0x1d86c: b 0x1d054 <entry+0x86c>  
    c.b(C::Al, 0x1d054)?;

    c.check(0x1d870)?;
    for word in CLASS_DATA { c.emit(word)?; }
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    initialize_class_command(c)?;
    dispatch_codec_class(c)?;
    c.check(END)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn class_commands_have_two_data_regions(){
        let mut c=Code::new(START);extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        for (i,word) in CLASS_HANDLERS.iter().enumerate(){
            let offset=0x1cbc8-START+4*i;
            assert_eq!(&c.bytes[offset..offset+4],&word.to_le_bytes());
        }
        for (i,word) in CLASS_DATA.iter().enumerate(){
            let offset=0x1d870-START+4*i;
            assert_eq!(&c.bytes[offset..offset+4],&word.to_le_bytes());
        }
    }
}
