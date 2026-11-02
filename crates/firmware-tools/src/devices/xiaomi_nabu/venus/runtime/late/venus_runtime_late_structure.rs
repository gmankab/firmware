// SPDX-License-Identifier: GPL-3.0-only
//! Four small late Venus ARM32 routines: codec layout/queue initialization,
//! buffer command publishing, header preparation and command completion.
//! The second routine has two distinguished diagnostic-address data words.
//! No copied OEM instruction words or authentication material are used.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x19bf4;
const END:usize=0x1a1ec;
const TRACE_ON_VALIDATION:u32=0x000e_43a6;
const TRACE_FOOTER:u32=0x000e_2788;

/// Iterate codec slots, normalize per-type flags and publish descriptors.
fn emit_slot_configuration(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x19bf4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x19bf8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x19bfc: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0xc)?)?;
    // 0x19c00: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x19c04: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19c08: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x19c0c: ldr r0, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 132)?)?;
    // 0x19c10: str r0, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 4)?)?;
    // 0x19c14: str r2, [r1, #0x44]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, false, R::R2, R::R1, 68)?)?;
    // 0x19c18: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x19c1c: ldr r0, [r1, #-0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, -12)?)?;
    // 0x19c20: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 8)?)?;
    // 0x19c24: str r2, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 8)?)?;
    // 0x19c28: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x19c2c: beq 0x19d98 <entry+0x6ec>
    c.b(C::Eq, 0x19d98)?;
    // 0x19c30: add r1, r4, #516
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x204)?)?;
    // 0x19c34: mov r10, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x1)?)?;
    // 0x19c38: mov r9, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x8c00)?)?;
    // 0x19c3c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19c40: add r6, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R1, 0x1c00)?)?;
    // 0x19c44: add r1, r4, #556
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x22c)?)?;
    // 0x19c48: add r7, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R1, 0x1c00)?)?;
    // 0x19c4c: add r1, r4, #552
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x228)?)?;
    // 0x19c50: add r8, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::R1, 0x1c00)?)?;
    // 0x19c54: ldr r1, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 60)?)?;
    // 0x19c58: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x19c5c: beq 0x19d7c <entry+0x6d0>
    c.b(C::Eq, 0x19d7c)?;
    // 0x19c60: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19c64: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19c68: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x19c6c: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19c70: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19c74: bl 0xe3f8 <entry+0xffffffffffff4d4c>
    c.bl(0xe3f8)?;
    // 0x19c78: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 16)?)?;
    // 0x19c7c: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0xa)?)?;
    // 0x19c80: bne 0x19ca4 <entry+0x5f8>
    c.b(C::Ne, 0x19ca4)?;
    // 0x19c84: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 8)?)?;
    // 0x19c88: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x19c8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x19c90: bne 0x19ca4 <entry+0x5f8>
    c.b(C::Ne, 0x19ca4)?;
    // 0x19c94: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19c98: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19c9c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19ca0: bl 0x19788 <entry+0xdc>
    c.bl(0x19788)?;
    // 0x19ca4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ca8: bl 0x15bd8 <entry+0xffffffffffffc52c>
    c.bl(0x15bd8)?;
    // 0x19cac: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19cb0: bl 0x12cc4 <entry+0xffffffffffff9618>
    c.bl(0x12cc4)?;
    // 0x19cb4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19cb8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19cbc: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x19cc0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19cc4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19cc8: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x19ccc: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 0)?)?;
    // 0x19cd0: mov r0, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x108)?)?;
    // 0x19cd4: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0x1)?)?;
    // 0x19cd8: orr r0, r0, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8c00)?)?;
    // 0x19cdc: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    // 0x19ce0: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 0)?)?;
    // 0x19ce4: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x19ce8: mov r2, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xd4)?)?;
    // 0x19cec: orr r2, r2, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9400)?)?;
    // 0x19cf0: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x19cf4: add r2, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0xc)?)?;
    // 0x19cf8: str r10, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 4)?)?;
    // 0x19cfc: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(8)?, 0)?)?;
    // 0x19d00: add r2, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0xc)?)?;
    // 0x19d04: str r9, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(9)?, R::R1, 8)?)?;
    // 0x19d08: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(7)?, 0)?)?;
    // 0x19d0c: str r10, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 8)?)?;
    // 0x19d10: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 0)?)?;
    // 0x19d14: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x19d18: beq 0x19d4c <entry+0x6a0>
    c.b(C::Eq, 0x19d4c)?;
    // 0x19d1c: mov r2, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x8d00)?)?;
    // 0x19d20: str r2, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 12)?)?;
    // 0x19d24: add r2, r1, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x14)?)?;
    // 0x19d28: str r10, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 12)?)?;
    // 0x19d2c: str r2, [r8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(8)?, 0)?)?;
    // 0x19d30: mov r2, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x104)?)?;
    // 0x19d34: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x19d38: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 16)?)?;
    // 0x19d3c: add r1, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x14)?)?;
    // 0x19d40: str r1, [r7]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(7)?, 0)?)?;
    // 0x19d44: mov r1, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3f)?)?;
    // 0x19d48: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x19d4c: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 68)?)?;
    // 0x19d50: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1)?)?;
    // 0x19d54: str r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 68)?)?;
    // 0x19d58: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 0x19d5c: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1)?)?;
    // 0x19d60: str r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 76)?)?;
    // 0x19d64: ldr r1, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 60)?)?;
    // 0x19d68: cmp r0, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x19d6c: blo 0x19c60 <entry+0x5b4>
    c.b(C::Cc, 0x19c60)?;
    // 0x19d70: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 56)?)?;
    // 0x19d74: ldr r2, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 72)?)?;
    // 0x19d78: b 0x19d80 <entry+0x6d4>
    c.b(C::Al, 0x19d80)?;
    // 0x19d7c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x19d80: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x1)?)?;
    // 0x19d84: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x19d88: cmp r2, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x19d8c: str r2, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(4)?, 72)?)?;
    // 0x19d90: str r3, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(4)?, 76)?)?;
    // 0x19d94: blo 0x19c58 <entry+0x5ac>
    c.b(C::Cc, 0x19c58)?;
    // 0x19d98: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19d9c: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19da0: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x19da4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19da8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19dac: bl 0xe3f8 <entry+0xffffffffffff4d4c>
    c.bl(0xe3f8)?;
    // 0x19db0: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 16)?)?;
    // 0x19db4: cmp r0, #10
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0xa)?)?;
    // 0x19db8: bne 0x19ddc <entry+0x730>
    c.b(C::Ne, 0x19ddc)?;
    // 0x19dbc: ldr r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 8)?)?;
    // 0x19dc0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0x19dc4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x19dc8: bne 0x19ddc <entry+0x730>
    c.b(C::Ne, 0x19ddc)?;
    // 0x19dcc: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19dd0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19dd4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19dd8: bl 0x19788 <entry+0xdc>
    c.bl(0x19788)?;
    // 0x19ddc: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19de0: bl 0x15bd8 <entry+0xffffffffffffc52c>
    c.bl(0x15bd8)?;
    // 0x19de4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19de8: bl 0x12cc4 <entry+0xffffffffffff9618>
    c.bl(0x12cc4)?;
    // 0x19dec: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19df0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19df4: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x19df8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19dfc: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19e00: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x19e04: mov r0, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    // 0x19e08: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19e0c: mov r2, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x108)?)?;
    // 0x19e10: mvn r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0x1)?)?;
    // 0x19e14: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x19e18: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x19e1c: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 0x19e20: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x19e24: mov r2, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x22c)?)?;
    // 0x19e28: add r7, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc)?)?;
    // 0x19e2c: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x19e30: ldr r2, [r4, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(4)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x19e34: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0x19e38: mov r3, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xd4)?)?;
    // 0x19e3c: orr r3, r3, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9400)?)?;
    // 0x19e40: str r3, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 4)?)?;
    // 0x19e44: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x19e48: str r3, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 4)?)?;
    // 0x19e4c: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 0)?)?;
    // 0x19e50: mov r7, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x8c00)?)?;
    // 0x19e54: str r7, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 8)?)?;
    // 0x19e58: add r7, r2, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0xc)?)?;
    // 0x19e5c: str r7, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(4)?, 0)?)?;
    // 0x19e60: str r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 8)?)?;
    // 0x19e64: ldr r7, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 4)?)?;
    // 0x19e68: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x0)?)?;
    // 0x19e6c: beq 0x19ea0 <entry+0x7f4>
    c.b(C::Eq, 0x19ea0)?;
    // 0x19e70: mov r7, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x8d00)?)?;
    // 0x19e74: str r7, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 12)?)?;
    // 0x19e78: str r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 12)?)?;
    // 0x19e7c: add r3, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x14)?)?;
    // 0x19e80: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 0)?)?;
    // 0x19e84: mov r1, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x104)?)?;
    // 0x19e88: orr r1, r1, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    // 0x19e8c: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x19e90: add r0, r2, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x14)?)?;
    // 0x19e94: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x19e98: mov r0, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3f)?)?;
    // 0x19e9c: str r0, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R2, 16)?)?;
    // 0x19ea0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x19ea4: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x19ea8: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x19eac)
}

/// Publish codec service initialization commands, with optional extensions.
fn emit_service_commands(c:&mut Code)->Result<(),String>{
    c.check(0x19eac)?;
    // 0x19eac: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x19eb0: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x19eb4: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x19eb8: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x19ebc: ldr r6, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(5)?, 132)?)?;
    // 0x19ec0: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x19ec4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ec8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ecc: bl 0xe3f8 <entry+0xffffffffffff4d4c>
    c.bl(0xe3f8)?;
    // 0x19ed0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ed4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ed8: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19edc: bl 0x19788 <entry+0xdc>
    c.bl(0x19788)?;
    // 0x19ee0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ee4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ee8: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x19eec: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ef0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ef4: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x19ef8: mov r0, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    // 0x19efc: mov r5, r4
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(4)?))?;
    // 0x19f00: mov r1, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x108)?)?;
    // 0x19f04: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0x1)?)?;
    // 0x19f08: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x19f0c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x19f10: orr r1, r1, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    // 0x19f14: ldr r0, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x19f18: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x19f1c: mov r1, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x22c)?)?;
    // 0x19f20: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x19f24: ldr r1, [r4, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(4)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x19f28: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 0)?)?;
    // 0x19f2c: mov r2, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xd4)?)?;
    // 0x19f30: orr r2, r2, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9400)?)?;
    // 0x19f34: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 4)?)?;
    // 0x19f38: add r2, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0xc)?)?;
    // 0x19f3c: str r7, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 4)?)?;
    // 0x19f40: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(5)?, 0)?)?;
    // 0x19f44: mov r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x8c00)?)?;
    // 0x19f48: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 8)?)?;
    // 0x19f4c: add r0, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0xc)?)?;
    // 0x19f50: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x19f54: str r7, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 8)?)?;
    // 0x19f58: ldr r1, [pc, #0x58]
    c.literal(R::R1, 0x19fb8)?;
    // 0x19f5c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x19f60: bl 0x2ec94 <entry+0x155e8>
    c.bl(0x2ec94)?;
    // 0x19f64: ldr r1, [pc, #0x50]
    c.literal(R::R1, 0x19fbc)?;
    // 0x19f68: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x19f6c: bl 0x2ec94 <entry+0x155e8>
    c.bl(0x2ec94)?;
    // 0x19f70: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x19f74: beq 0x19fb0 <entry+0x904>
    c.b(C::Eq, 0x19fb0)?;
    // 0x19f78: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 0x19f7c: mov r1, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8d00)?)?;
    // 0x19f80: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x19f84: add r2, r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0x8)?)?;
    // 0x19f88: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 0)?)?;
    // 0x19f8c: str r7, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 0)?)?;
    // 0x19f90: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(5)?, 0)?)?;
    // 0x19f94: mov r2, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x104)?)?;
    // 0x19f98: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x19f9c: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 4)?)?;
    // 0x19fa0: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x8)?)?;
    // 0x19fa4: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x19fa8: mov r0, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3f)?)?;
    // 0x19fac: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 4)?)?;
    // 0x19fb0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x19fb4: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x19fb8)?;
    c.emit(TRACE_ON_VALIDATION)?;
    c.emit(TRACE_FOOTER)?;
    c.check(0x19fc0)
}

/// Clear and initialize a paired descriptor, then invoke callback services.
fn emit_command_headers(c:&mut Code)->Result<(),String>{
    c.check(0x19fc0)?;
    // 0x19fc0: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x19fc4: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x19fc8: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x19fcc: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x19fd0: ldr r6, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(5)?, 132)?)?;
    // 0x19fd4: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x19fd8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19fdc: bl 0x15bd8 <entry+0xffffffffffffc52c>
    c.bl(0x15bd8)?;
    // 0x19fe0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19fe4: bl 0x12cc4 <entry+0xffffffffffff9618>
    c.bl(0x12cc4)?;
    // 0x19fe8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19fec: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ff0: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x19ff4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x19ff8: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x19ffc: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x1a000: mov r0, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    // 0x1a004: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a008: mov r2, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x108)?)?;
    // 0x1a00c: mvn r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1a010: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x1a014: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a018: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x1a01c: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a020: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x1a024: mov r2, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x22c)?)?;
    // 0x1a028: add r5, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0xc)?)?;
    // 0x1a02c: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x1a030: ldr r2, [r4, r2]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(4)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x1a034: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0x1a038: mov r3, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xd4)?)?;
    // 0x1a03c: orr r3, r3, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9400)?)?;
    // 0x1a040: str r3, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 4)?)?;
    // 0x1a044: mov r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x1a048: str r3, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 4)?)?;
    // 0x1a04c: str r5, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 0)?)?;
    // 0x1a050: mov r5, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x8c00)?)?;
    // 0x1a054: str r5, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 8)?)?;
    // 0x1a058: add r5, r2, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R2, 0xc)?)?;
    // 0x1a05c: str r5, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(4)?, 0)?)?;
    // 0x1a060: str r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 8)?)?;
    // 0x1a064: beq 0x1a098 <entry+0x9ec>
    c.b(C::Eq, 0x1a098)?;
    // 0x1a068: mov r6, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x8d00)?)?;
    // 0x1a06c: str r6, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 12)?)?;
    // 0x1a070: str r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 12)?)?;
    // 0x1a074: add r3, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x14)?)?;
    // 0x1a078: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 0)?)?;
    // 0x1a07c: mov r1, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x104)?)?;
    // 0x1a080: orr r1, r1, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8c00)?)?;
    // 0x1a084: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x1a088: add r0, r2, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x14)?)?;
    // 0x1a08c: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1a090: mov r0, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3f)?)?;
    // 0x1a094: str r0, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R2, 16)?)?;
    // 0x1a098: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1a09c: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(0x1a0a0)
}

/// Assemble codec command words from runtime geometry and publish the
/// resulting short command sequence to the controller ring.
fn emit_geometry_commands(c:&mut Code)->Result<(),String>{
    c.check(0x1a0a0)?;
    // 0x1a0a0: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1a0a4: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x18)?)?;
    // 0x1a0a8: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1a0ac: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x1a0b0: ldr r8, [r5, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(5)?, 132)?)?;
    // 0x1a0b4: bl 0x15ea8 <entry+0xffffffffffffc7fc>
    c.bl(0x15ea8)?;
    // 0x1a0b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a0bc: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a0c0: bl 0xe3f8 <entry+0xffffffffffff4d4c>
    c.bl(0xe3f8)?;
    // 0x1a0c4: mov r0, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x228)?)?;
    // 0x1a0c8: ldr r2, [r4, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 2648)?)?;
    // 0x1a0cc: mov r7, r4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(4)?))?;
    // 0x1a0d0: mov r6, #3120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0xc30)?)?;
    // 0x1a0d4: mov r12, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x22c)?)?;
    // 0x1a0d8: ldr lr, [r4, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::new(4)?, 728)?)?;
    // 0x1a0dc: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1a0e0: orr r6, r6, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x8000)?)?;
    // 0x1a0e4: orr r12, r12, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R12, R::R12, 0x1c00)?)?;
    // 0x1a0e8: ldr r0, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x1a0ec: ldr r3, [r2, #0x3cc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 972)?)?;
    // 0x1a0f0: ldr r2, [r2, #0x3d0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R2, 976)?)?;
    // 0x1a0f4: str r6, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 0)?)?;
    // 0x1a0f8: mov r6, r4
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(4)?))?;
    // 0x1a0fc: ldr r1, [r6, r12]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(6)?, R::R12, Shift::Lsl, 0)?)?;
    // 0x1a100: orr r2, lr, r2, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::LR, R::R2, Shift::Lsl, 3, false)?)?;
    // 0x1a104: orr r2, r2, r3, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(3)?, Shift::Lsl, 2, false)?)?;
    // 0x1a108: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 0)?)?;
    // 0x1a10c: add r2, r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0x8)?)?;
    // 0x1a110: ldr r12, [r4, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::new(4)?, 88)?)?;
    // 0x1a114: ldr r3, [r4, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 92)?)?;
    // 0x1a118: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(7)?, 0)?)?;
    // 0x1a11c: mov r2, #168
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xa8)?)?;
    // 0x1a120: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x1a124: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 4)?)?;
    // 0x1a128: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x8)?)?;
    // 0x1a12c: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 0)?)?;
    // 0x1a130: orr r0, r12, r3, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R12, R::new(3)?, Shift::Lsl, 16, false)?)?;
    // 0x1a134: str r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 4)?)?;
    // 0x1a138: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a13c: bl 0x12cc4 <entry+0xffffffffffff9618>
    c.bl(0x12cc4)?;
    // 0x1a140: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a144: bl 0x15b4c <entry+0xffffffffffffc4a0>
    c.bl(0x15b4c)?;
    // 0x1a148: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a14c: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a150: bl 0x13b14 <entry+0xffffffffffffa468>
    c.bl(0x13b14)?;
    // 0x1a154: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1a158: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0x1a15c: bl 0x12e5c <entry+0xffffffffffff97b0>
    c.bl(0x12e5c)?;
    // 0x1a160: ldr r1, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(7)?, 0)?)?;
    // 0x1a164: mov r0, #264
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x108)?)?;
    // 0x1a168: mvn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0x1)?)?;
    // 0x1a16c: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(8)?, 0x0)?)?;
    // 0x1a170: orr r0, r0, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8c00)?)?;
    // 0x1a174: str r0, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 0)?)?;
    // 0x1a178: add r3, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R1, 0xc)?)?;
    // 0x1a17c: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 0)?)?;
    // 0x1a180: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x1a184: mov r2, #212
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xd4)?)?;
    // 0x1a188: orr r2, r2, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9400)?)?;
    // 0x1a18c: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x1a190: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1a194: str r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 4)?)?;
    // 0x1a198: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(7)?, 0)?)?;
    // 0x1a19c: mov r3, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8c00)?)?;
    // 0x1a1a0: str r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 8)?)?;
    // 0x1a1a4: add r3, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xc)?)?;
    // 0x1a1a8: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(6)?, 0)?)?;
    // 0x1a1ac: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 8)?)?;
    // 0x1a1b0: beq 0x1a1e4 <entry+0xb38>
    c.b(C::Eq, 0x1a1e4)?;
    // 0x1a1b4: mov r3, #36096
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8d00)?)?;
    // 0x1a1b8: str r3, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 12)?)?;
    // 0x1a1bc: str r2, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 12)?)?;
    // 0x1a1c0: add r2, r1, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x14)?)?;
    // 0x1a1c4: str r2, [r7]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(7)?, 0)?)?;
    // 0x1a1c8: mov r2, #260
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x104)?)?;
    // 0x1a1cc: orr r2, r2, #35840
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8c00)?)?;
    // 0x1a1d0: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 16)?)?;
    // 0x1a1d4: add r1, r0, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x14)?)?;
    // 0x1a1d8: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 0)?)?;
    // 0x1a1dc: mov r1, #63
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3f)?)?;
    // 0x1a1e0: str r1, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 16)?)?;
    // 0x1a1e4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1a1e8: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;

    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_slot_configuration(c)?;
    emit_service_commands(c)?;
    emit_command_headers(c)?;
    emit_geometry_commands(c)?;
    Ok(())
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn command_helpers_have_exact_layout(){
        let mut c=Code::new(START);extend(&mut c).unwrap();
        assert_eq!(c.bytes.len(),END-START);
        assert_eq!(&c.bytes[0x19fb8-START..0x19fbc-START],&TRACE_ON_VALIDATION.to_le_bytes());
        assert_eq!(&c.bytes[0x19fbc-START..0x19fc0-START],&TRACE_FOOTER.to_le_bytes());
    }
}
