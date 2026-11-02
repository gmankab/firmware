// SPDX-License-Identifier: GPL-3.0-only
//! Venus codec stream-format command handler, with a 20-way format table.
//! Reconstructed ARM32 opcode semantics from typed register operations.
//! Handler addresses and the pool of diagnostic/configuration data are not
//! A32 instructions. Exact byte-level verification is independent of runtime
//! functional correctness or the OEM firmware signing chain.
use arm32::{BlockMode, Condition as C, DataOp as Op, Register as R, Shift};
use super::Code;
const START:usize=0x1d910;
const END:usize=0x1e5c8;
const FORMAT_HANDLERS:[u32;20]=[
    0x1e018, // format 0
    0x1dc7c, // format 1
    0x1dc64, // format 2
    0x1db54, // format 3
    0x1dc64, // format 4
    0x1dd70, // format 5
    0x1e110, // format 6
    0x1dc64, // format 7
    0x1e1c0, // format 8
    0x1dc64, // format 9
    0x1e250, // format 10
    0x1dc64, // format 11
    0x1de6c, // format 12
    0x1db54, // format 13
    0x1dc64, // format 14
    0x1db54, // format 15
    0x1dc64, // format 16
    0x1dc64, // format 17
    0x1dc64, // format 18
    0x1df10, // format 19
];
const FORMAT_DATA:[u32;27]=[
    0x000e5d8c, // device configuration 0
    0x000e2788, // device configuration 1
    0x000e53c3, // device configuration 2
    0x000e5686, // device configuration 3
    0x000e5dca, // device configuration 4
    0x000e5de4, // device configuration 5
    0x00103070, // device configuration 6
    0xfbfeeffd, // device configuration 7
    0x04021003, // device configuration 8
    0xfffefff9, // device configuration 9
    0xfbfeef60, // device configuration 10
    0x000e60cb, // device configuration 11
    0x000e6062, // device configuration 12
    0x000e608b, // device configuration 13
    0x000e601c, // device configuration 14
    0x000e5fd6, // device configuration 15
    0x000e5f90, // device configuration 16
    0x000e5f49, // device configuration 17
    0x000e5f07, // device configuration 18
    0x000e5ebc, // device configuration 19
    0x04011012, // device configuration 20
    0x04011010, // device configuration 21
    0x04011006, // device configuration 22
    0x000e5e44, // device configuration 23
    0x000e5e82, // device configuration 24
    0x000e6115, // device configuration 25
    0x000e5e21, // device configuration 26
];

/// Validate selected codec format and existing resource; dispatch to
/// compatible controller-format initialization, including pointer checks.
fn start_stream_format(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1d910: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1d914: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1d918: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x1c)?)?;
    // 0x1d91c: mov r9, r1
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R1))?;
    // 0x1d920: mov r10, r0
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R0))?;
    // 0x1d924: ldr r1, [pc, #0xc30]       
    c.literal(R::R1, 0x1e55c)?;
    // 0x1d928: ldm r9, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(9)?, R::R0.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1d92c: ldr r6, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 8)?)?;
    // 0x1d930: bic r0, r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x3)?)?;
    // 0x1d934: mov r3, r7
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 0x1d938: add r0, r9, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1d93c: ldr r4, [r0, #-0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::R0, -4)?)?;
    // 0x1d940: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d944: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 0)?)?;
    // 0x1d948: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 0x1d94c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1d950: ldr r1, [pc, #0xc08]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1d954: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1d958: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1d95c: mov r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x7)?)?;
    // 0x1d960: orr r0, r0, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x10000)?)?;
    // 0x1d964: cmp r7, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1d968: bne 0x1d97c <entry+0x94>   
    c.b(C::Ne, 0x1d97c)?;
    // 0x1d96c: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1d970: mvn r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1d974: ldr r6, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::R0, 24)?)?;
    // 0x1d978: b 0x1d9c8 <entry+0xe0>   
    c.b(C::Al, 0x1d9c8)?;
    // 0x1d97c: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1d980: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1d984: bl 0x23668 <entry+0x5d80> 
    c.bl(0x23668)?;
    // 0x1d988: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1d98c: cmn r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmn, R::R0, R::new(5)?, 0x1)?)?;
    // 0x1d990: beq 0x1dbc4 <entry+0x2dc>  
    c.b(C::Eq, 0x1dbc4)?;
    // 0x1d994: ldr r0, [r10, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1d998: ldr r2, [pc, #0xbc4]       
    c.literal(R::R2, 0x1e564)?;
    // 0x1d99c: mov r1, #141
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8d)?)?;
    // 0x1d9a0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 0x1d9a4: bl 0x17b0 <entry+0xfffffffffffe3ec8>
    c.bl(0x17b0)?;
    // 0x1d9a8: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1d9ac: ldr r6, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::R0, 24)?)?;
    // 0x1d9b0: mov r0, #65536
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10000)?)?;
    // 0x1d9b4: orr r0, r0, #67108864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x4000000)?)?;
    // 0x1d9b8: cmp r7, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1d9bc: movhs r0, #255
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0xff)?)?;
    // 0x1d9c0: strhs r0, [r10, #0x88]
    c.emit(arm32::transfer_word(C::Cs, false, R::R0, R::new(10)?, 136)?)?;
    // 0x1d9c4: strhs r7, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Cs, false, R::new(7)?, R::new(6)?, 8)?)?;
    // 0x1d9c8: str r4, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::new(6)?, 4)?)?;
    // 0x1d9cc: ldr r0, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 8)?)?;
    // 0x1d9d0: bl 0x23644 <entry+0x5d5c> 
    c.bl(0x23644)?;
    // 0x1d9d4: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1d9d8: str r5, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(6)?, 12)?)?;
    // 0x1d9dc: mov r1, #904
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x388)?)?;
    // 0x1d9e0: add r2, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::SP, 0x18)?)?;
    // 0x1d9e4: add r3, sp, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::SP, 0x14)?)?;
    // 0x1d9e8: str r4, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::new(6)?, 16)?)?;
    // 0x1d9ec: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1d9f0: ldr r0, [r10, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1d9f4: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1d9f8: add r1, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x10)?)?;
    // 0x1d9fc: bl 0x7f990 <entry+0x620a8>
    c.bl(0x7f990)?;
    // 0x1da00: ldr r0, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 24)?)?;
    // 0x1da04: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1da08: beq 0x1dba4 <entry+0x2bc>  
    c.b(C::Eq, 0x1dba4)?;
    // 0x1da0c: ldr r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 16)?)?;
    // 0x1da10: mov r2, #524
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x20c)?)?;
    // 0x1da14: str r4, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 12)?)?;
    // 0x1da18: mov r3, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x21c)?)?;
    // 0x1da1c: mov r4, #3616
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0xe20)?)?;
    // 0x1da20: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x1da24: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1c00)?)?;
    // 0x1da28: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(4)?, R::new(4)?, 0x1000)?)?;
    // 0x1da2c: str r1, [r6, r2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R1, R::new(6)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1da30: mov r1, #520
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x208)?)?;
    // 0x1da34: mov r2, #532
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x214)?)?;
    // 0x1da38: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x1da3c: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x1da40: str r0, [r6, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(6)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x1da44: mov r1, #3600
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe10)?)?;
    // 0x1da48: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x1da4c: str r0, [r6, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(6)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x1da50: add r1, r0, #16384
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x4000)?)?;
    // 0x1da54: str r1, [r6, r2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R1, R::new(6)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1da58: add r2, r0, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0x8000)?)?;
    // 0x1da5c: str r2, [r6, r3]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R2, R::new(6)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x1da60: add r3, r0, #49152
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xc000)?)?;
    // 0x1da64: str r3, [r6, r4]
    c.emit(arm32::transfer_word_register(C::Al, false, R::new(3)?, R::new(6)?, R::new(4)?, Shift::Lsl, 0)?)?;
    // 0x1da68: mov r4, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x228)?)?;
    // 0x1da6c: orr r4, r4, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(4)?, R::new(4)?, 0x1c00)?)?;
    // 0x1da70: str r0, [r6, r4]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(6)?, R::new(4)?, Shift::Lsl, 0)?)?;
    // 0x1da74: mov r0, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x22c)?)?;
    // 0x1da78: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1da7c: str r1, [r6, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R1, R::new(6)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1da80: mov r0, #564
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x234)?)?;
    // 0x1da84: mov r1, #191
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xbf)?)?;
    // 0x1da88: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1da8c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 0x1da90: str r2, [r6, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R2, R::new(6)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1da94: mov r0, #568
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x238)?)?;
    // 0x1da98: ldr r2, [pc, #0xac4]       
    c.literal(R::R2, 0x1e564)?;
    // 0x1da9c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1daa0: str r3, [r6, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::new(3)?, R::new(6)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1daa4: ldr r8, [r10, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(8)?, R::new(10)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1daa8: mov r0, r8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0x1daac: bl 0x17b0 <entry+0xfffffffffffe3ec8>
    c.bl(0x17b0)?;
    // 0x1dab0: ldr r0, [r10, r5, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::new(5)?, Shift::Lsl, 2)?)?;
    // 0x1dab4: mov r2, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xb90)?)?;
    // 0x1dab8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x1dabc: add r1, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x4)?)?;
    // 0x1dac0: add r1, r1, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xc00)?)?;
    // 0x1dac4: str r1, [r6, #0xa58]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 2648)?)?;
    // 0x1dac8: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 12)?)?;
    // 0x1dacc: ldr r1, [r10, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(10)?, R::R1, Shift::Lsl, 2)?)?;
    // 0x1dad0: ldr r1, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R1, R::R2, Shift::Lsl, 0)?)?;
    // 0x1dad4: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 0x1dad8: ldreq r1, [pc, #0xa94]       
    c.emit(arm32::transfer_word(C::Eq, true, R::R1, R::PC, 0xa94)?)?;
    // 0x1dadc: ldreq r1, [r1]
    c.emit(arm32::transfer_word(C::Eq, true, R::R1, R::R1, 0)?)?;
    // 0x1dae0: streq r1, [r0, #0xc40]
    c.emit(arm32::transfer_word(C::Eq, false, R::R1, R::R0, 3136)?)?;
    // 0x1dae4: ldr r0, [pc, #0xa8c]       
    c.literal(R::R0, 0x1e578)?;
    // 0x1dae8: ldr r5, [pc, #0xa8c]       
    c.literal(R::new(5)?, 0x1e57c)?;
    // 0x1daec: add r0, r7, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1daf0: cmp r0, #19
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x13)?)?;
    // 0x1daf4: bhi 0x1dc3c <entry+0x354>  
    c.b(C::Hi, 0x1dc3c)?;
    // 0x1daf8: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1dafc: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x0)?)?;
    // 0x1db00: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;

    c.check(0x1db04)?;
    for address in FORMAT_HANDLERS {c.emit(address)?;}
    c.check(0x1db54)
}
/// Pack codec channel format, assign controller packet descriptors and
/// synchronize known-good codec tables with the resource's register block.
fn configure_stream_formats(c:&mut Code)->Result<(),String>{
    c.check(0x1db54)?;
    // 0x1db54: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1db58: ldr r1, [pc, #0xa48]       
    c.literal(R::R1, 0x1e5a8)?;
    // 0x1db5c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1db60: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1db64: ldr r1, [pc, #0x9f4]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1db68: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1db6c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1db70: ldr r0, [pc, #0xa34]       
    c.literal(R::R0, 0x1e5ac)?;
    // 0x1db74: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 8)?)?;
    // 0x1db78: cmp r7, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1db7c: beq 0x1e378 <entry+0xa90>  
    c.b(C::Eq, 0x1e378)?;
    // 0x1db80: ldr r0, [pc, #0xa28]       
    c.literal(R::R0, 0x1e5b0)?;
    // 0x1db84: cmp r7, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1db88: beq 0x1e384 <entry+0xa9c>  
    c.b(C::Eq, 0x1e384)?;
    // 0x1db8c: ldr r0, [pc, #0xa20]       
    c.literal(R::R0, 0x1e5b4)?;
    // 0x1db90: cmp r7, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1db94: bne 0x1e390 <entry+0xaa8>  
    c.b(C::Ne, 0x1e390)?;
    // 0x1db98: add r0, r5, #2
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x2)?)?;
    // 0x1db9c: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1dba0: b 0x1e3a4 <entry+0xabc>  
    c.b(C::Al, 0x1e3a4)?;
    // 0x1dba4: ldr r1, [pc, #0xa18]       
    c.literal(R::R1, 0x1e5c4)?;
    // 0x1dba8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1dbac: mov r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x1)?)?;
    // 0x1dbb0: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dbb4: ldr r1, [pc, #0x9a4]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1dbb8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1dbbc: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dbc0: b 0x1e544 <entry+0xc5c>  
    c.b(C::Al, 0x1e544)?;
    // 0x1dbc4: ldr r8, [pc, #0x9a0]       
    c.literal(R::new(8)?, 0x1e56c)?;
    // 0x1dbc8: ldr r1, [pc, #0x998]       
    c.literal(R::R1, 0x1e568)?;
    // 0x1dbcc: mov r3, #130
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x82)?)?;
    // 0x1dbd0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1dbd4: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x400)?)?;
    // 0x1dbd8: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1dbdc: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dbe0: ldr r5, [pc, #0x978]       
    c.literal(R::new(5)?, 0x1e560)?;
    // 0x1dbe4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1dbe8: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1dbec: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dbf0: ldr r1, [pc, #0x978]       
    c.literal(R::R1, 0x1e570)?;
    // 0x1dbf4: mov r3, #131
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x83)?)?;
    // 0x1dbf8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1dbfc: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0x1dc00: str r6, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 0)?)?;
    // 0x1dc04: orr r3, r3, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x400)?)?;
    // 0x1dc08: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dc0c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1dc10: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x1dc14: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dc18: ldr r0, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 128)?)?;
    // 0x1dc1c: mov r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4)?)?;
    // 0x1dc20: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1dc24: mov r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2)?)?;
    // 0x1dc28: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 0)?)?;
    // 0x1dc2c: str r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 4)?)?;
    // 0x1dc30: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1dc34: bl 0x23ca8 <entry+0x63c0> 
    c.bl(0x23ca8)?;
    // 0x1dc38: b 0x1e540 <entry+0xc58>  
    c.b(C::Al, 0x1e540)?;
    // 0x1dc3c: ldr r0, [pc, #0x93c]       
    c.literal(R::R0, 0x1e580)?;
    // 0x1dc40: add r0, r7, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1dc44: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x3)?)?;
    // 0x1dc48: blo 0x1dc5c <entry+0x374>  
    c.b(C::Cc, 0x1dc5c)?;
    // 0x1dc4c: ldr r0, [pc, #0x930]       
    c.literal(R::R0, 0x1e584)?;
    // 0x1dc50: add r0, r7, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1dc54: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1dc58: bhi 0x1dc64 <entry+0x37c>  
    c.b(C::Hi, 0x1dc64)?;
    // 0x1dc5c: mov r1, #223
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xdf)?)?;
    // 0x1dc60: b 0x1dc68 <entry+0x380>  
    c.b(C::Al, 0x1dc68)?;
    // 0x1dc64: mov r1, #616
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x268)?)?;
    // 0x1dc68: ldr r2, [pc, #0x8f4]       
    c.literal(R::R2, 0x1e564)?;
    // 0x1dc6c: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 0x1dc70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1dc74: bl 0x17b0 <entry+0xfffffffffffe3ec8>
    c.bl(0x17b0)?;
    // 0x1dc78: b 0x1e524 <entry+0xc3c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1dc7c: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1dc80: ldr r1, [pc, #0x914]       
    c.literal(R::R1, 0x1e59c)?;
    // 0x1dc84: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1dc88: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 0x1dc8c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dc90: ldr r1, [pc, #0x8c8]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1dc94: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1dc98: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dc9c: add r0, r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x1)?)?;
    // 0x1dca0: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1dca4: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1dca8: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1dcac: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1dcb0: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1dcb4: mov r0, #44
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2c)?)?;
    // 0x1dcb8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dcbc: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dcc0: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1dcc4: mov r0, #48
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x30)?)?;
    // 0x1dcc8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dccc: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dcd0: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1dcd4: ldr r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 20)?)?;
    // 0x1dcd8: str r0, [r6, #0xa8c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2700)?)?;
    // 0x1dcdc: mov r0, #104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x68)?)?;
    // 0x1dce0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dce4: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dce8: str r0, [r6, #0xa94]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2708)?)?;
    // 0x1dcec: mov r0, #84
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x54)?)?;
    // 0x1dcf0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dcf4: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dcf8: str r0, [r6, #0xa9c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2716)?)?;
    // 0x1dcfc: mov r0, #88
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x58)?)?;
    // 0x1dd00: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dd04: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dd08: str r0, [r6, #0xa98]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2712)?)?;
    // 0x1dd0c: mov r0, #52
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x34)?)?;
    // 0x1dd10: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dd14: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dd18: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1dd1c: beq 0x1dd54 <entry+0x46c>  
    c.b(C::Eq, 0x1dd54)?;
    // 0x1dd20: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 16)?)?;
    // 0x1dd24: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1dd28: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x1dd2c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dd30: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dd34: str r0, [r6, #0xa80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2688)?)?;
    // 0x1dd38: mov r0, #60
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3c)?)?;
    // 0x1dd3c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1dd40: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dd44: str r0, [r6, #0xa84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2692)?)?;
    // 0x1dd48: mov r0, #4160
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1040)?)?;
    // 0x1dd4c: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1dd50: str r0, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2696)?)?;
    // 0x1dd54: mov r0, #76
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4c)?)?;
    // 0x1dd58: add r1, r6, #2720
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0xaa0)?)?;
    // 0x1dd5c: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1dd60: add r0, r9, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x20)?)?;
    // 0x1dd64: ldm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1dd68: stm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1dd6c: b 0x1e18c <entry+0x8a4>  
    c.b(C::Al, 0x1e18c)?;
    // 0x1dd70: ldr r1, [pc, #0x814]       
    c.literal(R::R1, 0x1e58c)?;
    // 0x1dd74: ldr r2, [pc, #0x7f0]       
    c.literal(R::R2, 0x1e56c)?;
    // 0x1dd78: mov r3, #21
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x15)?)?;
    // 0x1dd7c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10)?)?;
    // 0x1dd80: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x600)?)?;
    // 0x1dd84: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dd88: ldr r4, [pc, #0x7d0]       
    c.literal(R::new(4)?, 0x1e560)?;
    // 0x1dd8c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10)?)?;
    // 0x1dd90: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1dd94: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1dd98: ldr r2, [pc, #0x7c4]       
    c.literal(R::R2, 0x1e564)?;
    // 0x1dd9c: mov r1, #22
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x16)?)?;
    // 0x1dda0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1dda4: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    // 0x1dda8: bl 0x17b0 <entry+0xfffffffffffe3ec8>
    c.bl(0x17b0)?;
    // 0x1ddac: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1ddb0: ldr r1, [pc, #0x7d8]       
    c.literal(R::R1, 0x1e590)?;
    // 0x1ddb4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ddb8: mov r8, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x2)?)?;
    // 0x1ddbc: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1ddc0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ddc4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1ddc8: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1ddcc: add r0, r5, #3
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x3)?)?;
    // 0x1ddd0: add r1, r9, #76
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x4c)?)?;
    // 0x1ddd4: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1ddd8: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1dddc: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1dde0: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 24)?)?;
    // 0x1dde4: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1dde8: ldr r0, [r9, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 28)?)?;
    // 0x1ddec: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1ddf0: ldr r0, [r9, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 68)?)?;
    // 0x1ddf4: str r0, [r6, #0xa80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2688)?)?;
    // 0x1ddf8: ldr r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 20)?)?;
    // 0x1ddfc: str r0, [r6, #0xa84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2692)?)?;
    // 0x1de00: ldr r0, [r9, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 32)?)?;
    // 0x1de04: str r0, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2696)?)?;
    // 0x1de08: ldr r0, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 36)?)?;
    // 0x1de0c: str r0, [r6, #0xa8c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2700)?)?;
    // 0x1de10: ldr r0, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 40)?)?;
    // 0x1de14: str r0, [r6, #0xa94]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2708)?)?;
    // 0x1de18: ldr r0, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 72)?)?;
    // 0x1de1c: str r0, [r6, #0xa98]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2712)?)?;
    // 0x1de20: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1de24: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1de28: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 16)?)?;
    // 0x1de2c: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1de30: ldr r0, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 44)?)?;
    // 0x1de34: str r0, [r6, #0xa90]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2704)?)?;
    // 0x1de38: mov r0, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x48)?)?;
    // 0x1de3c: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1de40: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1de44: str r0, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 52)?)?;
    // 0x1de48: add r0, r6, #668
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x29c)?)?;
    // 0x1de4c: mov r6, r7
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x1de50: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1de54: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1de58: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1de5c: mov r7, r6
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(6)?))?;
    // 0x1de60: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1de64: str r8, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::R0, 20)?)?;
    // 0x1de68: b 0x1e524 <entry+0xc3c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1de6c: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1de70: ldr r1, [pc, #0x72c]       
    c.literal(R::R1, 0x1e5a4)?;
    // 0x1de74: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1de78: mov r4, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x2)?)?;
    // 0x1de7c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1de80: ldr r1, [pc, #0x6d8]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1de84: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1de88: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1de8c: add r0, r5, #7
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x7)?)?;
    // 0x1de90: add r1, r6, #2736
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0xab0)?)?;
    // 0x1de94: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1de98: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1de9c: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1dea0: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1dea4: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1dea8: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 16)?)?;
    // 0x1deac: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1deb0: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 52)?)?;
    // 0x1deb4: str r0, [r6, #0xa8c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2700)?)?;
    // 0x1deb8: ldr r0, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 76)?)?;
    // 0x1debc: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1dec0: ldr r0, [r9, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 80)?)?;
    // 0x1dec4: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1dec8: ldr r0, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 44)?)?;
    // 0x1decc: str r0, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2696)?)?;
    // 0x1ded0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1ded4: str r0, [r6, #0xa90]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2704)?)?;
    // 0x1ded8: str r0, [r6, #0xa94]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2708)?)?;
    // 0x1dedc: str r0, [r6, #0xa98]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2712)?)?;
    // 0x1dee0: ldr r0, [r9, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 84)?)?;
    // 0x1dee4: str r0, [r6, #0xa9c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2716)?)?;
    // 0x1dee8: mov r0, #92
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x5c)?)?;
    // 0x1deec: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1def0: add r0, r9, #88
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x58)?)?;
    // 0x1def4: mov r6, r7
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x1def8: ldm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1defc: stm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1df00: mov r7, r6
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(6)?))?;
    // 0x1df04: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1df08: str r4, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::R0, 20)?)?;
    // 0x1df0c: b 0x1e524 <entry+0xc3c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1df10: str r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 8)?)?;
    // 0x1df14: ldr r1, [pc, #0x66c]       
    c.literal(R::R1, 0x1e588)?;
    // 0x1df18: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1df1c: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1df20: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1df24: ldr r1, [pc, #0x634]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1df28: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1df2c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1df30: add r0, r5, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0xc)?)?;
    // 0x1df34: add r1, r8, #312
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(8)?, 0x138)?)?;
    // 0x1df38: mov r2, #120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x78)?)?;
    // 0x1df3c: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1df40: add r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1400)?)?;
    // 0x1df44: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1df48: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1df4c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1df50: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1df54: mov r0, #1328
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x530)?)?;
    // 0x1df58: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1df5c: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1df60: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1df64: mov r0, #308
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x134)?)?;
    // 0x1df68: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 0x1df6c: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1df70: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1df74: add r0, r6, #636
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x27c)?)?;
    // 0x1df78: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1df7c: bl 0x471c <entry+0xfffffffffffe6e34>
    c.bl(0x471c)?;
    // 0x1df80: add r0, r6, #756
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x2f4)?)?;
    // 0x1df84: add r1, r8, #1456
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(8)?, 0x5b0)?)?;
    // 0x1df88: mov r2, #120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x78)?)?;
    // 0x1df8c: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1df90: add r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1000)?)?;
    // 0x1df94: bl 0x471c <entry+0xfffffffffffe6e34>
    c.bl(0x471c)?;
    // 0x1df98: add r0, r6, #996
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x3e4)?)?;
    // 0x1df9c: add r1, r9, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x10)?)?;
    // 0x1dfa0: mov r2, #120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x78)?)?;
    // 0x1dfa4: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1dfa8: bl 0x471c <entry+0xfffffffffffe6e34>
    c.bl(0x471c)?;
    // 0x1dfac: add r1, r9, #136
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x88)?)?;
    // 0x1dfb0: add r0, r6, #92
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x5c)?)?;
    // 0x1dfb4: ldm r1, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1dfb8: add r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xc00)?)?;
    // 0x1dfbc: add r1, r6, #3184
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0xc70)?)?;
    // 0x1dfc0: stm r0, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1dfc4: mov r0, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x21c)?)?;
    // 0x1dfc8: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1dfcc: add r0, r9, #180
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0xb4)?)?;
    // 0x1dfd0: ldm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1dfd4: stm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1dfd8: mov r1, #852
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x354)?)?;
    // 0x1dfdc: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 12)?)?;
    // 0x1dfe0: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1dfe4: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x1dfe8: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1dfec: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1dff0: beq 0x1e500 <entry+0xc18>  
    c.b(C::Eq, 0x1e500)?;
    // 0x1dff4: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1dff8: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1dffc: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 0x1e000: bl 0x23840 <entry+0x5f58> 
    c.bl(0x23840)?;
    // 0x1e004: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e008: mov r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2)?)?;
    // 0x1e00c: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1e010: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1e014: b 0x1e520 <entry+0xc38>  
    c.b(C::Al, 0x1e520)?;
    // 0x1e018: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1e01c: ldr r1, [pc, #0x594]       
    c.literal(R::R1, 0x1e5b8)?;
    // 0x1e020: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e024: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e028: ldr r1, [pc, #0x530]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1e02c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e030: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e034: str r5, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(6)?, 2660)?)?;
    // 0x1e038: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1e03c: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1e040: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1e044: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1e048: ldr r0, [r9, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 56)?)?;
    // 0x1e04c: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1e050: ldr r0, [r9, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 48)?)?;
    // 0x1e054: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1e058: ldr r0, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 72)?)?;
    // 0x1e05c: str r0, [r6, #0xa8c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2700)?)?;
    // 0x1e060: ldr r0, [r9, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 64)?)?;
    // 0x1e064: str r0, [r6, #0xa84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2692)?)?;
    // 0x1e068: ldr r0, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 44)?)?;
    // 0x1e06c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1e070: beq 0x1e09c <entry+0x7b4>  
    c.b(C::Eq, 0x1e09c)?;
    // 0x1e074: ldr r0, [r9, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 60)?)?;
    // 0x1e078: str r0, [r6, #0xa80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2688)?)?;
    // 0x1e07c: ldr r0, [r9, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 52)?)?;
    // 0x1e080: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1e084: ldr r0, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 76)?)?;
    // 0x1e088: str r0, [r6, #0xa90]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2704)?)?;
    // 0x1e08c: ldr r0, [r9, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 68)?)?;
    // 0x1e090: str r0, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2696)?)?;
    // 0x1e094: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e098: str r0, [r6, #0x92c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2348)?)?;
    // 0x1e09c: ldr r0, [r9, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 116)?)?;
    // 0x1e0a0: str r0, [r6, #0xaa0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2720)?)?;
    // 0x1e0a4: ldr r0, [r9, #0x70]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 112)?)?;
    // 0x1e0a8: str r0, [r6, #0xa9c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2716)?)?;
    // 0x1e0ac: mov r0, #80
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x50)?)?;
    // 0x1e0b0: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1e0b4: ldr r1, [r9, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 64)?)?;
    // 0x1e0b8: ldr r0, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 36)?)?;
    // 0x1e0bc: lsl r1, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 3)?)?;
    // 0x1e0c0: cmp r1, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1e0c4: blo 0x1e0dc <entry+0x7f4>  
    c.b(C::Cc, 0x1e0dc)?;
    // 0x1e0c8: ldr r1, [r9, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 72)?)?;
    // 0x1e0cc: ldr r0, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 40)?)?;
    // 0x1e0d0: lsl r1, r1, #3
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsl, 3)?)?;
    // 0x1e0d4: cmp r1, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x1e0d8: bhs 0x1e550 <entry+0xc68>  
    c.b(C::Cs, 0x1e550)?;
    // 0x1e0dc: ldr r1, [pc, #0x4d8]       
    c.literal(R::R1, 0x1e5bc)?;
    // 0x1e0e0: ldr r2, [pc, #0x484]       
    c.literal(R::R2, 0x1e56c)?;
    // 0x1e0e4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1e0e8: mov r3, #1280
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x500)?)?;
    // 0x1e0ec: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e0f0: ldr r1, [pc, #0x468]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1e0f4: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1e0f8: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e0fc: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 0x1e100: mov r1, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5)?)?;
    // 0x1e104: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x2000)?)?;
    // 0x1e108: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e10c: b 0x1e4d0 <entry+0xbe8>  
    c.b(C::Al, 0x1e4d0)?;
    // 0x1e110: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1e114: ldr r1, [pc, #0x478]       
    c.literal(R::R1, 0x1e594)?;
    // 0x1e118: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e11c: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 0x1e120: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e124: ldr r1, [pc, #0x434]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1e128: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e12c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e130: add r0, r5, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x4)?)?;
    // 0x1e134: add r1, r9, #44
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x2c)?)?;
    // 0x1e138: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1e13c: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1e140: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1e144: ldr r0, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 36)?)?;
    // 0x1e148: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1e14c: ldr r0, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 40)?)?;
    // 0x1e150: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1e154: mov r0, #296
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x128)?)?;
    // 0x1e158: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e15c: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e160: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1e164: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 24)?)?;
    // 0x1e168: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1e16c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1e170: str r0, [r6, #0xa80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2688)?)?;
    // 0x1e174: mov r0, #48
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x30)?)?;
    // 0x1e178: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1e17c: add r0, r6, #644
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x284)?)?;
    // 0x1e180: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e184: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1e188: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e18c: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 12)?)?;
    // 0x1e190: mov r1, #852
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x354)?)?;
    // 0x1e194: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1e198: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x1e19c: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1e1a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1e1a4: beq 0x1e350 <entry+0xa68>  
    c.b(C::Eq, 0x1e350)?;
    // 0x1e1a8: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1e1ac: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1e1b0: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 0x1e1b4: bl 0x23840 <entry+0x5f58> 
    c.bl(0x23840)?;
    // 0x1e1b8: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e1bc: b 0x1e33c <entry+0xa54>  
    c.b(C::Al, 0x1e33c)?;
    // 0x1e1c0: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1e1c4: ldr r1, [pc, #0x3cc]       
    c.literal(R::R1, 0x1e598)?;
    // 0x1e1c8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e1cc: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 0x1e1d0: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e1d4: ldr r1, [pc, #0x384]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1e1d8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e1dc: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e1e0: add r0, r5, #5
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x5)?)?;
    // 0x1e1e4: add r1, r9, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x1c)?)?;
    // 0x1e1e8: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1e1ec: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1e1f0: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1e1f4: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1e1f8: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1e1fc: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 16)?)?;
    // 0x1e200: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1e204: ldr r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 20)?)?;
    // 0x1e208: str r0, [r6, #0xa84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2692)?)?;
    // 0x1e20c: ldr r0, [r9, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 24)?)?;
    // 0x1e210: str r0, [r6, #0xa88]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2696)?)?;
    // 0x1e214: mov r0, #168
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xa8)?)?;
    // 0x1e218: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e21c: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e220: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1e224: mov r0, #172
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xac)?)?;
    // 0x1e228: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e22c: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e230: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1e234: mov r0, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x38)?)?;
    // 0x1e238: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1e23c: add r0, r6, #652
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x28c)?)?;
    // 0x1e240: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e244: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1e248: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e24c: b 0x1e320 <entry+0xa38>  
    c.b(C::Al, 0x1e320)?;
    // 0x1e250: ldr r2, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 8)?)?;
    // 0x1e254: ldr r1, [pc, #0x344]       
    c.literal(R::R1, 0x1e5a0)?;
    // 0x1e258: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e25c: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 0x1e260: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e264: ldr r1, [pc, #0x2f4]       
    c.literal(R::R1, 0x1e560)?;
    // 0x1e268: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1e26c: bl 0x2ec94 <entry+0x113ac>
    c.bl(0x2ec94)?;
    // 0x1e270: add r0, r5, #6
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x6)?)?;
    // 0x1e274: add r1, r9, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x18)?)?;
    // 0x1e278: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1e27c: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1e280: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1e284: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 16)?)?;
    // 0x1e288: str r0, [r6, #0xa80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2688)?)?;
    // 0x1e28c: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1e290: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1e294: ldr r0, [r9, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 36)?)?;
    // 0x1e298: str r0, [r6, #0xa94]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2708)?)?;
    // 0x1e29c: ldr r0, [r9, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 40)?)?;
    // 0x1e2a0: str r0, [r6, #0xa98]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2712)?)?;
    // 0x1e2a4: ldr r0, [r9, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 20)?)?;
    // 0x1e2a8: str r0, [r6, #0xa84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2692)?)?;
    // 0x1e2ac: add r0, r6, #648
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x288)?)?;
    // 0x1e2b0: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e2b4: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1e2b8: add r1, r9, #52
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x34)?)?;
    // 0x1e2bc: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e2c0: ldr r0, [r9, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 44)?)?;
    // 0x1e2c4: str r0, [r6, #0xa9c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2716)?)?;
    // 0x1e2c8: ldr r0, [r9, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 48)?)?;
    // 0x1e2cc: str r0, [r6, #0xaa0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2720)?)?;
    // 0x1e2d0: add r0, r6, #676
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x2a4)?)?;
    // 0x1e2d4: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e2d8: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1e2dc: add r1, r6, #2736
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0xab0)?)?;
    // 0x1e2e0: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e2e4: add r0, r9, #64
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x40)?)?;
    // 0x1e2e8: ldm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e2ec: stm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e2f0: add r1, r6, #2752
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0xac0)?)?;
    // 0x1e2f4: ldr r0, [r9, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 76)?)?;
    // 0x1e2f8: str r0, [r6, #0xabc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2748)?)?;
    // 0x1e2fc: mov r0, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x21c)?)?;
    // 0x1e300: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e304: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e308: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1e30c: mov r0, #108
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x6c)?)?;
    // 0x1e310: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1e314: add r0, r9, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x50)?)?;
    // 0x1e318: ldm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e31c: stm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e320: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 12)?)?;
    // 0x1e324: mov r1, #852
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x354)?)?;
    // 0x1e328: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1e32c: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x1e330: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1e334: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1e338: beq 0x1e350 <entry+0x68>   
    c.b(C::Eq, 0x1e350)?;
    // 0x1e33c: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1e340: mov r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2)?)?;
    // 0x1e344: mov r7, r4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(4)?))?;
    // 0x1e348: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1e34c: b 0x1e524 <entry+0x23c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1e350: mov r0, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x5)?)?;
    // 0x1e354: mov r1, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5)?)?;
    // 0x1e358: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x2000)?)?;
    // 0x1e35c: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e360: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1e364: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1e368: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1e36c: bl 0x80af0 <entry+0x62808>
    c.bl(0x80af0)?;
    // 0x1e370: mov r7, r4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(4)?))?;
    // 0x1e374: b 0x1e524 <entry+0x23c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1e378: add r0, r5, #9
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x9)?)?;
    // 0x1e37c: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1e380: b 0x1e3a4 <entry+0xbc>   
    c.b(C::Al, 0x1e3a4)?;
    // 0x1e384: add r0, r5, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x8)?)?;
    // 0x1e388: str r0, [r6, #0xa64]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2660)?)?;
    // 0x1e38c: b 0x1e3a4 <entry+0xbc>   
    c.b(C::Al, 0x1e3a4)?;
    // 0x1e390: ldr r2, [pc, #0x1cc]       
    c.literal(R::R2, 0x1e564)?;
    // 0x1e394: mov r1, #30
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1e)?)?;
    // 0x1e398: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1e39c: orr r1, r1, #1280
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x500)?)?;
    // 0x1e3a0: bl 0x17b0 <entry+0xfffffffffffe34c8>
    c.bl(0x17b0)?;
    // 0x1e3a4: ldr r0, [r9, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 8)?)?;
    // 0x1e3a8: add r1, r9, #36
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x24)?)?;
    // 0x1e3ac: str r0, [r6, #0xa68]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2664)?)?;
    // 0x1e3b0: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 0x1e3b4: str r0, [r6, #0xa70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2672)?)?;
    // 0x1e3b8: mov r0, #668
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x29c)?)?;
    // 0x1e3bc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e3c0: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e3c4: str r0, [r6, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 56)?)?;
    // 0x1e3c8: mov r0, #672
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2a0)?)?;
    // 0x1e3cc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e3d0: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e3d4: str r0, [r6, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 60)?)?;
    // 0x1e3d8: add r0, r6, #648
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x288)?)?;
    // 0x1e3dc: ldm r1!, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e3e0: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1e3e4: stm r0!, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e3e8: ldm r1, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e3ec: add r1, r6, #2736
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0xab0)?)?;
    // 0x1e3f0: stm r0, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e3f4: add r0, r8, #676
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0x2a4)?)?;
    // 0x1e3f8: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 0x1e3fc: ldm r0!, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e400: stm r1!, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e404: ldm r0, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e408: add r0, r8, #716
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0x2cc)?)?;
    // 0x1e40c: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 0x1e410: stm r1, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e414: add r1, r6, #728
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(6)?, 0x2d8)?)?;
    // 0x1e418: ldm r0!, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e41c: add r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x800)?)?;
    // 0x1e420: stm r1!, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e424: ldm r0, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e428: mov r0, #660
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x294)?)?;
    // 0x1e42c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e430: stm r1, {r2, r3, r4, r5, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(4)?.bit() | R::new(5)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e434: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1e438: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e43c: str r0, [r6, #0xa7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2684)?)?;
    // 0x1e440: mov r0, #656
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x290)?)?;
    // 0x1e444: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e448: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e44c: str r0, [r6, #0xa74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2676)?)?;
    // 0x1e450: mov r0, #664
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x298)?)?;
    // 0x1e454: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1e458: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1e45c: str r0, [r6, #0xa78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2680)?)?;
    // 0x1e460: mov r0, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xd4)?)?;
    // 0x1e464: str r0, [r6, #0xa60]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2656)?)?;
    // 0x1e468: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 8)?)?;
    // 0x1e46c: ldr r1, [pc, #0x138]       
    c.literal(R::R1, 0x1e5ac)?;
    // 0x1e470: orr r0, r7, #2
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::new(7)?, 0x2)?)?;
    // 0x1e474: cmp r0, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1e478: mov r1, #852
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x354)?)?;
    // 0x1e47c: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1e480: orr r1, r1, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1800)?)?;
    // 0x1e484: str r2, [r6, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(6)?, 52)?)?;
    // 0x1e488: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 12)?)?;
    // 0x1e48c: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0x1e490: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0x1e494: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x1e498: beq 0x1e4dc <entry+0x1f4>  
    c.b(C::Eq, 0x1e4dc)?;
    // 0x1e49c: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0x1e4a0: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0x1e4a4: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 0x1e4a8: bl 0x23840 <entry+0x5558> 
    c.bl(0x23840)?;
    // 0x1e4ac: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e4b0: add r0, r6, #808
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x328)?)?;
    // 0x1e4b4: add r1, r9, #108
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x6c)?)?;
    // 0x1e4b8: add r0, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x800)?)?;
    // 0x1e4bc: mov r6, r7
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(7)?))?;
    // 0x1e4c0: ldm r1, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, true, R::R1, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e4c4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2)?)?;
    // 0x1e4c8: stm r0, {r2, r3, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::R0, R::R2.bit() | R::new(3)?.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0x1e4cc: mov r7, r6
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(6)?))?;
    // 0x1e4d0: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1e4d4: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1e4d8: b 0x1e524 <entry+0x23c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1e4dc: mov r0, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x5)?)?;
    // 0x1e4e0: mov r1, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5)?)?;
    // 0x1e4e4: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x2000)?)?;
    // 0x1e4e8: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e4ec: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1e4f0: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1e4f4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1e4f8: bl 0x80af0 <entry+0x62808>
    c.bl(0x80af0)?;
    // 0x1e4fc: b 0x1e524 <entry+0x23c>  
    c.b(C::Al, 0x1e524)?;
    // 0x1e500: mov r0, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x5)?)?;
    // 0x1e504: mov r1, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5)?)?;
    // 0x1e508: orr r0, r0, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x2000)?)?;
    // 0x1e50c: str r0, [r6, #0xa6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2668)?)?;
    // 0x1e510: ldr r0, [r10, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 144)?)?;
    // 0x1e514: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0x1e518: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1e51c: bl 0x80af0 <entry+0x62808>
    c.bl(0x80af0)?;
    // 0x1e520: ldr r7, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 8)?)?;
    // 0x1e524: ldr r1, [pc, #0x94]        
    c.literal(R::R1, 0x1e5c0)?;
    // 0x1e528: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e52c: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 0x1e530: bl 0x2ec94 <entry+0x109ac>
    c.bl(0x2ec94)?;
    // 0x1e534: ldr r1, [pc, #0x24]        
    c.literal(R::R1, 0x1e560)?;
    // 0x1e538: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1e53c: bl 0x2ec94 <entry+0x109ac>
    c.bl(0x2ec94)?;
    // 0x1e540: mov r4, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x0)?)?;
    // 0x1e544: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1e548: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1e54c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1e550: add r0, r6, #676
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x2a4)?)?;
    // 0x1e554: add r1, r9, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x80)?)?;
    // 0x1e558: b 0x1e4b8 <entry+0x1d0>  
    c.b(C::Al, 0x1e4b8)?;

    c.check(0x1e55c)?;
    for word in FORMAT_DATA {c.emit(word)?;}
    c.check(END)
}
pub(super) fn extend(c:&mut Code)->Result<(),String>{
    start_stream_format(c)?;
    configure_stream_formats(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn format_dispatch_contains_separate_literal_pools(){
        let mut c=Code::new(START);extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        for (i,word) in FORMAT_HANDLERS.iter().enumerate(){
            let at=0x1db04-START+i*4;
            assert_eq!(&c.bytes[at..at+4],&word.to_le_bytes());
        }
        for (i,word) in FORMAT_DATA.iter().enumerate(){
            let at=0x1e55c-START+i*4;
            assert_eq!(&c.bytes[at..at+4],&word.to_le_bytes());
        }
    }
}
