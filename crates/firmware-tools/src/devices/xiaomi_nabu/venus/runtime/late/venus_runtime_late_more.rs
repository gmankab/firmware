// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus ARM32 codec context initialization and parameter-record writer.
//! Both functions are addressed from the firmware's original function table.
//! They follow the full command-controller routine at 0x196ac and use a
//! separate two-word literal pool; no OEM instruction words are embedded.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START: usize=0x196ac;
const END: usize=0x19bf4;
const RUNTIME_DIAGNOSTIC: u32=0x000e_3d16;
const CODEC_CAPABILITY_MASK: u32=0x0003_fffe;

/// Clear and initialize the paired codec stream submission headers, invoking
/// startup services before marking the corresponding hardware descriptors.
fn emit_reset_command_pair(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x196ac: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x196b0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x196b4: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x196b8: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x196bc: ldr r6, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(5)?, 132)?)?;
    // 0x196c0: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x196c4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x196c8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x196cc: bl 0xe3f8 <entry+0xffffffffffff4d4c>
    c.bl(0xe3f8)?;
    // 0x196d0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x196d4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x196d8: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x196dc: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x196e0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x196e4: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x196e8: mov r0, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    // 0x196ec: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x196f0: mov r2, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x108)?)?;
    // 0x196f4: mvn r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0x1)?)?;
    // 0x196f8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x196fc: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x19700: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x19704: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 0x19708: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x1970c: mov r2, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x22c)?)?;
    // 0x19710: add r5, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0xc)?)?;
    // 0x19714: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x19718: ldr r2, [r4, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(4)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1971c: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0x19720: mov r3, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xd4)?)?;
    // 0x19724: orr r3, r3, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9400)?)?;
    // 0x19728: str r3, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 4)?)?;
    // 0x1972c: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x19730: str r3, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 4)?)?;
    // 0x19734: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 0)?)?;
    // 0x19738: mov r5, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x8c00)?)?;
    // 0x1973c: str r5, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 8)?)?;
    // 0x19740: add r5, r2, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R2, 0xc)?)?;
    // 0x19744: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(4)?, 0)?)?;
    // 0x19748: str r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 8)?)?;
    // 0x1974c: beq 0x19780 <entry+0xd4>
    c.b(C::Eq, 0x19780)?;
    // 0x19750: mov r6, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x8d00)?)?;
    // 0x19754: str r6, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 12)?)?;
    // 0x19758: str r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 12)?)?;
    // 0x1975c: add r3, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x14)?)?;
    // 0x19760: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 0)?)?;
    // 0x19764: mov r1, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x104)?)?;
    // 0x19768: orr r1, r1, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    // 0x1976c: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x19770: add r0, r2, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x14)?)?;
    // 0x19774: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x19778: mov r0, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3f)?)?;
    // 0x1977c: str r0, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R2, 16)?)?;
    // 0x19780: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x19784: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x19788)
}

/// Select a format-dependent register set, pack 5-bit field groups into a
/// controller word and publish a command pointer to the runtime submit queue.
fn emit_codec_parameter_word(c:&mut Code)->Result<(),String>{
    c.check(0x19788)?;
    // 0x19788: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1978c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x19790: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x4)?)?;
    // 0x19794: mov r8, r1
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::R1))?;
    // 0x19798: ldr r1, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 16)?)?;
    // 0x1979c: sub r3, r1, #9
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::new(3)?, R::R1, 0x9)?)?;
    // 0x197a0: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x2)?)?;
    // 0x197a4: blo 0x19824 <entry+0x178>
    c.b(C::Cc, 0x19824)?;
    // 0x197a8: cmp r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x4)?)?;
    // 0x197ac: beq 0x19888 <entry+0x1dc>
    c.b(C::Eq, 0x19888)?;
    // 0x197b0: cmp r1, #11
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0xb)?)?;
    // 0x197b4: bne 0x198ec <entry+0x240>
    c.b(C::Ne, 0x198ec)?;
    // 0x197b8: ldr r1, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 12)?)?;
    // 0x197bc: mov r7, #836
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x344)?)?;
    // 0x197c0: mov r5, #776
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x308)?)?;
    // 0x197c4: orr r7, r7, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1800)?)?;
    // 0x197c8: orr r5, r5, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1800)?)?;
    // 0x197cc: ldr r3, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R0, R::R1, Shift::Lsl, 2)?)?;
    // 0x197d0: ldr r0, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 144)?)?;
    // 0x197d4: mov r1, #2800
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xaf0)?)?;
    // 0x197d8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x197dc: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 32)?)?;
    // 0x197e0: add r6, r3, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::new(3)?, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x197e4: ldr r12, [r3, r7]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R12, R::new(3)?, R::new(7)?, Shift::Lsl, 0)?)?;
    // 0x197e8: ldr r9, [r3, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(9)?, R::new(3)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x197ec: mov r3, #2864
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xb30)?)?;
    // 0x197f0: mov r7, #756
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x2f4)?)?;
    // 0x197f4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x197f8: orr r7, r7, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1800)?)?;
    // 0x197fc: ldr r10, [r6, r5]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(10)?, R::new(6)?, R::new(5)?, Shift::Lsl, 0)?)?;
    // 0x19800: ldr lr, [r6, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::LR, R::new(6)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x19804: mov r3, #796
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x31c)?)?;
    // 0x19808: orr r3, r3, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1800)?)?;
    // 0x1980c: add r2, r0, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R0, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x19810: ldr r3, [r6, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::new(6)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x19814: ldr r6, [r6, r7]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::new(6)?, R::new(7)?, Shift::Lsl, 0)?)?;
    // 0x19818: ldrb r0, [r0, #0x9c]
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, R::R0, 156)?)?;
    // 0x1981c: ldr r7, [r2, #0xa0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R2, 160)?)?;
    // 0x19820: b 0x19920 <entry+0x274>
    c.b(C::Al, 0x19920)?;
    // 0x19824: ldr r1, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 12)?)?;
    // 0x19828: mov r3, #248
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xf8)?)?;
    // 0x1982c: mov r5, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0xec)?)?;
    // 0x19830: mov r6, #232
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0xe8)?)?;
    // 0x19834: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1400)?)?;
    // 0x19838: orr r5, r5, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1400)?)?;
    // 0x1983c: orr r6, r6, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1400)?)?;
    // 0x19840: ldr r2, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 2)?)?;
    // 0x19844: ldr r0, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 144)?)?;
    // 0x19848: mov r1, #228
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe4)?)?;
    // 0x1984c: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x19850: ldr r12, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R12, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x19854: mov r3, #244
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xf4)?)?;
    // 0x19858: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 32)?)?;
    // 0x1985c: ldr r9, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(9)?, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 0x19860: ldr r10, [r2, r5]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(10)?, R::R2, R::new(5)?, Shift::Lsl, 0)?)?;
    // 0x19864: ldr r6, [r2, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::R2, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x19868: orr r3, r3, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1400)?)?;
    // 0x1986c: ldr lr, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::LR, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x19870: mov r3, #1264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4f0)?)?;
    // 0x19874: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x19878: ldr r7, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R0, 92)?)?;
    // 0x1987c: ldr r3, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x19880: ldrb r0, [r0, #0x68]
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, R::R0, 104)?)?;
    // 0x19884: b 0x19920 <entry+0x274>
    c.b(C::Al, 0x19920)?;
    // 0x19888: ldr r1, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 12)?)?;
    // 0x1988c: mov r3, #100
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x64)?)?;
    // 0x19890: mov r5, #88
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x58)?)?;
    // 0x19894: mov r6, #84
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x54)?)?;
    // 0x19898: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1989c: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1000)?)?;
    // 0x198a0: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x198a4: ldr r2, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R1, Shift::Lsl, 2)?)?;
    // 0x198a8: ldr r0, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 144)?)?;
    // 0x198ac: mov r1, #104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x68)?)?;
    // 0x198b0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x198b4: ldr r12, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R12, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x198b8: mov r3, #96
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x60)?)?;
    // 0x198bc: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 32)?)?;
    // 0x198c0: ldr r9, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(9)?, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 0x198c4: ldr r10, [r2, r5]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(10)?, R::R2, R::new(5)?, Shift::Lsl, 0)?)?;
    // 0x198c8: ldr r6, [r2, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::R2, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x198cc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x198d0: ldr lr, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::LR, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x198d4: mov r3, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x5c)?)?;
    // 0x198d8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x198dc: ldr r7, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R0, 28)?)?;
    // 0x198e0: ldr r3, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x198e4: ldrb r0, [r0, #0x18]
    c.emit(arm32::transfer_byte(C::Al, true, R::R0, R::R0, 24)?)?;
    // 0x198e8: b 0x19920 <entry+0x274>
    c.b(C::Al, 0x19920)?;
    // 0x198ec: ldr r2, [pc, #0xe0]
    c.literal(R::R2, 0x199d4)?;
    // 0x198f0: mov r1, #74
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4a)?)?;
    // 0x198f4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x198f8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x198fc: orr r1, r1, #5888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1700)?)?;
    // 0x19900: bl 0x17b0 <entry+0xfffffffffffe8104>
    c.bl(0x17b0)?;
    // 0x19904: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0x19908: mov r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x0)?)?;
    // 0x1990c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x19910: mov lr, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::LR, R::R0, 0x0)?)?;
    // 0x19914: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x19918: mov r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x0)?)?;
    // 0x1991c: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x19920: mov r2, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x228)?)?;
    // 0x19924: mov r4, r8
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(8)?))?;
    // 0x19928: mov r1, #40
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x28)?)?;
    // 0x1992c: mov r5, #100663296
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x6000000)?)?;
    // 0x19930: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x19934: and r5, r5, r9, lsl #25
    c.emit(arm32::data_register(C::Al, Op::And, R::new(5)?, R::new(5)?, R::new(9)?, Shift::Lsl, 25, false)?)?;
    // 0x19938: orr r1, r1, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    // 0x1993c: ldr r2, [r4, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(4)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x19940: orr r5, r5, r7, lsl #27
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(5)?, R::new(5)?, R::new(7)?, Shift::Lsl, 27, false)?)?;
    // 0x19944: orr r0, r5, r0
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x19948: mov r5, r8
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(8)?))?;
    // 0x1994c: str r1, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 0)?)?;
    // 0x19950: mov r1, #32505856
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1f00000)?)?;
    // 0x19954: and r3, r1, r3, lsl #20
    c.emit(arm32::data_register(C::Al, Op::And, R::new(3)?, R::R1, R::new(3)?, Shift::Lsl, 20, false)?)?;
    // 0x19958: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R0, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x1995c: mov r3, #1015808
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xf8000)?)?;
    // 0x19960: and r3, r3, r10, lsl #15
    c.emit(arm32::data_register(C::Al, Op::And, R::new(3)?, R::new(3)?, R::new(10)?, Shift::Lsl, 15, false)?)?;
    // 0x19964: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R0, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x19968: mov r3, #31744
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x7c00)?)?;
    // 0x1996c: and r3, r3, r6, lsl #10
    c.emit(arm32::data_register(C::Al, Op::And, R::new(3)?, R::new(3)?, R::new(6)?, Shift::Lsl, 10, false)?)?;
    // 0x19970: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R0, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x19974: mov r3, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x22c)?)?;
    // 0x19978: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1c00)?)?;
    // 0x1997c: ldr r3, [r5, r3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::new(3)?, R::new(5)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x19980: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 0)?)?;
    // 0x19984: add r0, r2, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x8)?)?;
    // 0x19988: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1998c: mov r0, #3104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xc20)?)?;
    // 0x19990: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8000)?)?;
    // 0x19994: str r0, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R2, 4)?)?;
    // 0x19998: add r0, r3, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(3)?, 0x8)?)?;
    // 0x1999c: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 0)?)?;
    // 0x199a0: and r0, r1, lr, lsl #20
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R1, R::LR, Shift::Lsl, 20, false)?)?;
    // 0x199a4: ldr r1, [pc, #0x2c]
    c.literal(R::R1, 0x199d8)?;
    // 0x199a8: and r1, r1, r12, lsl #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R1, R::R1, R::R12, Shift::Lsl, 1, false)?)?;
    // 0x199ac: orr r0, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x199b0: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 0x199b4: orr r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1)?)?;
    // 0x199b8: str r0, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 4)?)?;
    // 0x199bc: bl 0x12cc4 <entry+0xffffffffffff9618>
    c.bl(0x12cc4)?;
    // 0x199c0: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 0x199c4: bl 0x15b4c <entry+0xffffffffffffc4a0>
    c.bl(0x15b4c)?;
    // 0x199c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x199cc: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x199d0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x199d4)?;
    for word in [RUNTIME_DIAGNOSTIC,CODEC_CAPABILITY_MASK]{c.emit(word)?;}
    c.check(0x199dc)
}

/// Iterate the requested codec-class list, emit a queue command per live
/// item, and preserve format-level accounting and diagnostic logging.
fn emit_codec_class_list(c:&mut Code)->Result<(),String>{
    c.check(0x199dc)?;
    // 0x199dc: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x199e0: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x199e4: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0xc)?)?;
    // 0x199e8: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x199ec: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x199f0: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x199f4: str r2, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 76)?)?;
    // 0x199f8: str r2, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 68)?)?;
    // 0x199fc: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 12)?)?;
    // 0x19a00: ldr r6, [r5, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::new(5)?, R::R0, Shift::Lsl, 2)?)?;
    // 0x19a04: str r2, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 72)?)?;
    // 0x19a08: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 56)?)?;
    // 0x19a0c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x19a10: beq 0x19bdc <entry+0x530>
    c.b(C::Eq, 0x19bdc)?;
    // 0x19a14: add r1, r4, #516
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x204)?)?;
    // 0x19a18: mov r10, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x1)?)?;
    // 0x19a1c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x19a20: add r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1c00)?)?;
    // 0x19a24: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 8)?)?;
    // 0x19a28: add r1, r4, #556
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x22c)?)?;
    // 0x19a2c: add r8, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::R1, 0x1c00)?)?;
    // 0x19a30: add r1, r4, #552
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x228)?)?;
    // 0x19a34: add r9, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::R1, 0x1c00)?)?;
    // 0x19a38: ldr r1, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 60)?)?;
    // 0x19a3c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x19a40: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x19a44: str r7, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 76)?)?;
    // 0x19a48: beq 0x19bc8 <entry+0x51c>
    c.b(C::Eq, 0x19bc8)?;
    // 0x19a4c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x19a50: b 0x19a60 <entry+0x3b4>
    c.b(C::Al, 0x19a60)?;
    // 0x19a54: ldr r1, [pc, #0x194]
    c.literal(R::R1, 0x19bf0)?;
    // 0x19a58: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 0)?)?;
    // 0x19a5c: b 0x19b84 <entry+0x4d8>
    c.b(C::Al, 0x19b84)?;
    // 0x19a60: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x1)?)?;
    // 0x19a64: cmp r3, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(3)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x19a68: bne 0x19a84 <entry+0x3d8>
    c.b(C::Ne, 0x19a84)?;
    // 0x19a6c: mov r1, #732
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2dc)?)?;
    // 0x19a70: add r0, r6, r3, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::new(3)?, Shift::Lsl, 2, false)?)?;
    // 0x19a74: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x19a78: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x19a7c: orrs r0, r0, r7
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R0, R::new(7)?, Shift::Lsl, 0, true)?)?;
    // 0x19a80: beq 0x19a54 <entry+0x3a8>
    c.b(C::Eq, 0x19a54)?;
    // 0x19a84: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19a88: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19a8c: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x19a90: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19a94: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19a98: bl 0xe3f8 <entry+0xffffffffffff4d4c>
    c.bl(0xe3f8)?;
    // 0x19a9c: ldr r2, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 72)?)?;
    // 0x19aa0: mov r1, #732
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2dc)?)?;
    // 0x19aa4: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x19aa8: add r0, r6, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x19aac: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x19ab0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x19ab4: beq 0x19ad0 <entry+0x424>
    c.b(C::Eq, 0x19ad0)?;
    // 0x19ab8: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 0x19abc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x19ac0: bne 0x19ad0 <entry+0x424>
    c.b(C::Ne, 0x19ad0)?;
    // 0x19ac4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ac8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19acc: bl 0x19788 <entry+0xdc>
    c.bl(0x19788)?;
    // 0x19ad0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ad4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ad8: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x19adc: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ae0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ae4: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x19ae8: ldr r1, [r9]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 0)?)?;
    // 0x19aec: mov r0, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x108)?)?;
    // 0x19af0: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0x1)?)?;
    // 0x19af4: orr r0, r0, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8c00)?)?;
    // 0x19af8: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    // 0x19afc: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 0)?)?;
    // 0x19b00: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x19b04: mov r2, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xd4)?)?;
    // 0x19b08: orr r2, r2, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9400)?)?;
    // 0x19b0c: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x19b10: add r2, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0xc)?)?;
    // 0x19b14: str r10, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 4)?)?;
    // 0x19b18: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(9)?, 0)?)?;
    // 0x19b1c: mov r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x8c00)?)?;
    // 0x19b20: str r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 8)?)?;
    // 0x19b24: add r2, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0xc)?)?;
    // 0x19b28: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(8)?, 0)?)?;
    // 0x19b2c: str r10, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 8)?)?;
    // 0x19b30: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 8)?)?;
    // 0x19b34: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R2, 0)?)?;
    // 0x19b38: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x19b3c: beq 0x19b70 <entry+0x4c4>
    c.b(C::Eq, 0x19b70)?;
    // 0x19b40: mov r2, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x8d00)?)?;
    // 0x19b44: str r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 12)?)?;
    // 0x19b48: add r2, r1, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x14)?)?;
    // 0x19b4c: str r10, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 12)?)?;
    // 0x19b50: str r2, [r9]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(9)?, 0)?)?;
    // 0x19b54: mov r2, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x104)?)?;
    // 0x19b58: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x19b5c: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 16)?)?;
    // 0x19b60: add r1, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x14)?)?;
    // 0x19b64: str r1, [r8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(8)?, 0)?)?;
    // 0x19b68: mov r1, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3f)?)?;
    // 0x19b6c: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x19b70: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 0x19b74: ldr r2, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 68)?)?;
    // 0x19b78: ldr r3, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 72)?)?;
    // 0x19b7c: ldr r1, [pc, #0x64]
    c.literal(R::R1, 0x19be8)?;
    // 0x19b80: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 0)?)?;
    // 0x19b84: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x19b88: bl 0x2ec94 <entry+0x155e8>
    c.bl(0x2ec94)?;
    // 0x19b8c: ldr r1, [pc, #0x58]
    c.literal(R::R1, 0x19bec)?;
    // 0x19b90: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x19b94: bl 0x2ec94 <entry+0x155e8>
    c.bl(0x2ec94)?;
    // 0x19b98: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 68)?)?;
    // 0x19b9c: add r2, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0x1)?)?;
    // 0x19ba0: str r2, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 68)?)?;
    // 0x19ba4: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 0x19ba8: add r7, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0x1)?)?;
    // 0x19bac: str r7, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 76)?)?;
    // 0x19bb0: ldr r1, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 60)?)?;
    // 0x19bb4: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 56)?)?;
    // 0x19bb8: ldr r3, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 72)?)?;
    // 0x19bbc: cmp r7, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x19bc0: blo 0x19a60 <entry+0x3b4>
    c.b(C::Cc, 0x19a60)?;
    // 0x19bc4: b 0x19bcc <entry+0x520>
    c.b(C::Al, 0x19bcc)?;
    // 0x19bc8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x19bcc: add r3, r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x1)?)?;
    // 0x19bd0: cmp r3, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(3)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x19bd4: str r3, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 72)?)?;
    // 0x19bd8: blo 0x19a3c <entry+0x390>
    c.b(C::Cc, 0x19a3c)?;
    // 0x19bdc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x19be0: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x19be4: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x19be8)?;
    // Named debug-message words, not executable ARM instructions.
    for value in [0x000e_4382_u32,0x000e_2788,0x000e_434e] { c.emit(value)?; }
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_reset_command_pair(c)?;
    emit_codec_parameter_word(c)?;
    emit_codec_class_list(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn two_more_routines_have_exact_layout(){
        let mut c=Code::new(START);extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(), END-START);
        assert_eq!(&c.bytes[0x199d8-START..0x199dc-START],&CODEC_CAPABILITY_MASK.to_le_bytes());
        assert_eq!(&c.bytes[c.bytes.len()-4..],&0x000e_434eu32.to_le_bytes());
    }
}
