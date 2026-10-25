// SPDX-License-Identifier: GPL-3.0-only
//! Codec weighting/coefficients aggregation and controller capture diagnostics.
//! Numeric codec classification and feature flags are reconstructed as typed
//! ARM32 instructions; OEM string addresses are separate literal data words.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x1a670;
const END:usize=0x1a8a8;
const COEFFICIENT_PRIMARY:u32=0x000e_4654;
const COEFFICIENT_SECONDARY:u32=0x000e_469c;
const CONTROLLER_LOGS:[u32;5]=[0x000e_46e4,0x000e_4707,0x000e_2788,0x000e_471e,0x000e_474a];

/// Accumulate eighteen codec-usage bucket values and pick median class,
/// preserving mixed ARM/Thumb scaled division calls as BLX.
fn emit_weighted_coefficients(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1a670: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1a674: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1a678: sub sp, sp, #36
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x24)?)?;
    // 0x1a67c: str r3, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::SP, 0)?)?;
    // 0x1a680: mov r3, r0
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R0))?;
    // 0x1a684: ldr r0, [pc, #0x168]
    c.literal(R::R0, 0x1a7f4)?;
    // 0x1a688: ldr r8, [pc, #0x168]
    c.literal(R::new(8)?, 0x1a7f8)?;
    // 0x1a68c: str r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 4)?)?;
    // 0x1a690: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 0x1a694: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x1a698: mvn r9, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(9)?, R::R0, 0x8)?)?;
    // 0x1a69c: mov r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x0)?)?;
    // 0x1a6a0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1a6a4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1a6a8: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1a6ac: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 32)?)?;
    // 0x1a6b0: cmp r9, #9
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0x9)?)?;
    // 0x1a6b4: ldr r7, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al, true, R::new(7)?, R::R0, 4)?)?;
    // 0x1a6b8: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0x1a6bc: rsbhs r7, r7, #0
    c.emit(arm32::data_immediate(C::Cs, Op::Rsb, R::new(7)?, R::new(7)?, 0x0)?)?;
    // 0x1a6c0: ldr r12, [r3]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::new(3)?, 0)?)?;
    // 0x1a6c4: cmp r7, #4
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x4)?)?;
    // 0x1a6c8: mul r0, r12, r7
    c.emit(arm32::multiply_register(C::Al, R::R0, R::R12, R::new(7)?)?)?;
    // 0x1a6cc: bne 0x1a6e8 <entry+0x4fc>
    c.b(C::Ne, 0x1a6e8)?;
    // 0x1a6d0: ldr r7, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(8)?, 0)?)?;
    // 0x1a6d4: ldr r6, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(4)?, 0)?)?;
    // 0x1a6d8: cmp r9, #9
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0x9)?)?;
    // 0x1a6dc: rsbhs r7, r7, #0
    c.emit(arm32::data_immediate(C::Cs, Op::Rsb, R::new(7)?, R::new(7)?, 0x0)?)?;
    // 0x1a6e0: mul lr, r7, r6
    c.emit(arm32::multiply_register(C::Al, R::LR, R::new(7)?, R::new(6)?)?)?;
    // 0x1a6e4: b 0x1a764 <entry+0x578>
    c.b(C::Al, 0x1a764)?;
    // 0x1a6e8: ldr r6, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(4)?, 0)?)?;
    // 0x1a6ec: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 12)?)?;
    // 0x1a6f0: ldr r0, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R11, 8)?)?;
    // 0x1a6f4: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 16)?)?;
    // 0x1a6f8: str r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 28)?)?;
    // 0x1a6fc: str r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 20)?)?;
    // 0x1a700: ldr r7, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(8)?, 0)?)?;
    // 0x1a704: cmp r9, #9
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0x9)?)?;
    // 0x1a708: mov r5, r3
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(3)?))?;
    // 0x1a70c: ldr r1, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 100)?)?;
    // 0x1a710: rsbhs r7, r7, #0
    c.emit(arm32::data_immediate(C::Cs, Op::Rsb, R::new(7)?, R::new(7)?, 0x0)?)?;
    // 0x1a714: mul lr, r7, r6
    c.emit(arm32::multiply_register(C::Al, R::LR, R::new(7)?, R::new(6)?)?)?;
    // 0x1a718: str r1, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 8)?)?;
    // 0x1a71c: ldr r2, [r0, #0x68]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R0, 104)?)?;
    // 0x1a720: ldr r1, [r0, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 488)?)?;
    // 0x1a724: ldr r7, [r0, #0x170]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R0, 368)?)?;
    // 0x1a728: str r12, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::SP, 24)?)?;
    // 0x1a72c: mul r0, lr, r2
    c.emit(arm32::multiply_register(C::Al, R::R0, R::LR, R::R2)?)?;
    // 0x1a730: blx 0x83c4c <entry+0x69a60>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32, 0x83c4c)?)?;
    // 0x1a734: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 12)?)?;
    // 0x1a738: ldr r2, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 8)?)?;
    // 0x1a73c: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 0x1a740: mul r0, r2, r1
    c.emit(arm32::multiply_register(C::Al, R::R0, R::R2, R::R1)?)?;
    // 0x1a744: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1a748: blx 0x83c4c <entry+0x69a60>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32, 0x83c4c)?)?;
    // 0x1a74c: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 0x1a750: ldr r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 16)?)?;
    // 0x1a754: ldr r1, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 20)?)?;
    // 0x1a758: ldr r12, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::SP, 24)?)?;
    // 0x1a75c: ldr r2, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 28)?)?;
    // 0x1a760: mov lr, r6
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(6)?))?;
    // 0x1a764: add r10, r10, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::new(10)?, 0x1)?)?;
    // 0x1a768: add r1, r12, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R12, R::R1, Shift::Lsl, 0, false)?)?;
    // 0x1a76c: add r5, lr, r5
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::LR, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 0x1a770: add r2, r0, r2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R0, R::R2, Shift::Lsl, 0, false)?)?;
    // 0x1a774: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(9)?, 0x1)?)?;
    // 0x1a778: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::new(8)?, 0x4)?)?;
    // 0x1a77c: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x4)?)?;
    // 0x1a780: add r3, r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x4)?)?;
    // 0x1a784: cmp r10, #18
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 0x12)?)?;
    // 0x1a788: bne 0x1a6ac <entry+0x4c0>
    c.b(C::Ne, 0x1a6ac)?;
    // 0x1a78c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x1a790: beq 0x1a7bc <entry+0x5d0>
    c.b(C::Eq, 0x1a7bc)?;
    // 0x1a794: mov r0, r2
    c.emit(arm32::mov_register(C::Al, R::R0, R::R2))?;
    // 0x1a798: mov r7, r1
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R1))?;
    // 0x1a79c: mov r6, r5
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(5)?))?;
    // 0x1a7a0: blx 0x83c4c <entry+0x69a60>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32, 0x83c4c)?)?;
    // 0x1a7a4: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 4)?)?;
    // 0x1a7a8: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 0x1a7ac: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1a7b0: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x1a7b4: blx 0x83c4c <entry+0x69a60>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32, 0x83c4c)?)?;
    // 0x1a7b8: b 0x1a7c8 <entry+0x5dc>
    c.b(C::Al, 0x1a7c8)?;
    // 0x1a7bc: ldr r4, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 4)?)?;
    // 0x1a7c0: mov r0, #64
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x40)?)?;
    // 0x1a7c4: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1a7c8: ldr r2, [sp]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 0)?)?;
    // 0x1a7cc: ldr r1, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R11, 8)?)?;
    // 0x1a7d0: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R2, 0)?)?;
    // 0x1a7d4: ldrb r1, [r1, #0x948]
    c.emit(arm32::transfer_byte(C::Al, true, R::R1, R::R1, 2376)?)?;
    // 0x1a7d8: tst r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Tst, R::R0, R::R1, 0x1)?)?;
    // 0x1a7dc: ldrne r1, [r4]
    c.emit(arm32::transfer_word(C::Ne, true, R::R1, R::new(4)?, 0)?)?;
    // 0x1a7e0: strne r1, [r2]
    c.emit(arm32::transfer_word(C::Ne, false, R::R1, R::R2, 0)?)?;
    // 0x1a7e4: strne r0, [r4]
    c.emit(arm32::transfer_word(C::Ne, false, R::R0, R::new(4)?, 0)?)?;
    // 0x1a7e8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1a7ec: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1a7f0: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.emit(COEFFICIENT_PRIMARY)?;
    c.emit(COEFFICIENT_SECONDARY)?;
    c.check(0x1a7fc)
}

/// Check the platform codec controller resource result and report its
/// category-specific diagnostic before returning to the codec dispatcher.
fn emit_controller_resource_capture(c:&mut Code)->Result<(),String>{
    c.check(0x1a7fc)?;
    // 0x1a7fc: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1a800: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x1a804: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x1a808: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x1a80c: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1a810: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x4)?)?;
    // 0x1a814: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1a818: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0x1a81c: bl 0x80928 <entry+0x6673c>
    c.bl(0x80928)?;
    // 0x1a820: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1a824: bne 0x1a858 <entry+0x66c>
    c.b(C::Ne, 0x1a858)?;
    // 0x1a828: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 4)?)?;
    // 0x1a82c: ldr r1, [pc, #0x70]
    c.literal(R::R1, 0x1a8a4)?;
    // 0x1a830: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1a834: ldr r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 12)?)?;
    // 0x1a838: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1a83c: ldr r1, [pc, #0x58]
    c.literal(R::R1, 0x1a89c)?;
    // 0x1a840: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1a844: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1a848: ldr r0, [r4, #0x94]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 148)?)?;
    // 0x1a84c: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 0x1a850: bl 0x292c <entry+0xfffffffffffe8740>
    c.bl(0x292c)?;
    // 0x1a854: b 0x1a888 <entry+0x69c>
    c.b(C::Al, 0x1a888)?;
    // 0x1a858: ldr r1, [pc, #0x34]
    c.literal(R::R1, 0x1a894)?;
    // 0x1a85c: ldr r2, [pc, #0x34]
    c.literal(R::R2, 0x1a898)?;
    // 0x1a860: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1a864: mov r3, #20
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x14)?)?;
    // 0x1a868: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1a86c: ldr r1, [pc, #0x28]
    c.literal(R::R1, 0x1a89c)?;
    // 0x1a870: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1a874: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1a878: ldr r2, [pc, #0x20]
    c.literal(R::R2, 0x1a8a0)?;
    // 0x1a87c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1a880: mov r1, #21
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x15)?)?;
    // 0x1a884: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1a888: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1a88c: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x8)?)?;
    // 0x1a890: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for w in CONTROLLER_LOGS{c.emit(w)?;}
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_weighted_coefficients(c)?;
    emit_controller_resource_capture(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
  use super::*;
  #[test]fn distinct_data_literal_pools_are_not_instructions(){
      let mut c=Code::new(START);
      extend(&mut c).unwrap();
      assert_eq!(c.bytes.len(),END-START);
      assert_eq!(&c.bytes[0x1a7f4-START..0x1a7f8-START],&COEFFICIENT_PRIMARY.to_le_bytes());
      assert_eq!(&c.bytes[0x1a7f8-START..0x1a7fc-START],&COEFFICIENT_SECONDARY.to_le_bytes());
      let offset=0x1a894-START;
      for (index,word) in CONTROLLER_LOGS.iter().enumerate(){
         assert_eq!(&c.bytes[offset+4*index..offset+4*(index+1)],&word.to_le_bytes());
      }
  }
}
