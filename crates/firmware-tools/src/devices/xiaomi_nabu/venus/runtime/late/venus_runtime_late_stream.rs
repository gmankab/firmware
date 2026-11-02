// SPDX-License-Identifier: GPL-3.0-only
//! Reconstructed A32 codec command table setup and serialized hardware words.
//!
//! Real function entry at 0x18628 and return at 0x1968c, followed by a
//! seven-word literal pool. This module is only a disjoint late-runtime
//! executable region and is never a complete or authenticated firmware.
//! Branch destinations are absolute runtime addresses; the emitter checks
//! each byte position and refuses gaps, overlaps or unsupported encodings.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;

/// A32 SMULBB: sign-extend the lower 16 bits of two operands and multiply.
/// This architecture formula is independent of the target firmware word.
fn signed_multiply_low_halfwords(cond: C, rd: R, rm: R, rs: R) -> Result<u32, String> {
    if [rd,rm,rs].iter().any(|r|r.number()>=13) {
        return Err("SMULBB forbids SP/PC operands".into());
    }
    Ok(((cond as u32)<<28) | 0x0160_0080 | (rd.number()<<16) |
       (rs.number()<<8) | rm.number())
}

/// Allocate codec packet slots and compute packed plane dimensions.
fn emit_packet_setup(c: &mut Code) -> Result<(), String> {
    c.check(0x18628)?;
    // 0x18628: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1862c: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x18630: sub sp, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x1c)?)?;
    // 0x18634: mov r9, r1
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R1))?;
    // 0x18638: mov r3, #112
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x70)?)?;
    // 0x1863c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x18640: ldr r1, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 12)?)?;
    // 0x18644: mov r10, r9
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(9)?))?;
    // 0x18648: ldr r6, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 16)?)?;
    // 0x1864c: orr r3, r3, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8000)?)?;
    // 0x18650: mov lr, r9
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(9)?))?;
    // 0x18654: ldr r5, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(5)?, R::R0, R::R1, Shift::Lsl, 2)?)?;
    // 0x18658: mov r1, #564
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x234)?)?;
    // 0x1865c: cmp r6, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x7)?)?;
    // 0x18660: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x18664: ldr r1, [r10, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x18668: str r0, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 8)?)?;
    // 0x1866c: ldr r0, [r0, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 132)?)?;
    // 0x18670: str r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 12)?)?;
    // 0x18674: mov r0, #568
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x238)?)?;
    // 0x18678: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 0)?)?;
    // 0x1867c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x18680: ldr r2, [lr, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::LR, R::R0, Shift::Lsl, 0)?)?;
    // 0x18684: mov r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xff)?)?;
    // 0x18688: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0xff00)?)?;
    // 0x1868c: str r0, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R2, 0)?)?;
    // 0x18690: str r3, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 4)?)?;
    // 0x18694: str r7, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R2, 4)?)?;
    // 0x18698: add r0, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0xc)?)?;
    // 0x1869c: moveq r7, #8
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(7)?, R::R0, 0x8)?)?;
    // 0x186a0: cmp r6, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x6)?)?;
    // 0x186a4: mov r3, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x9400)?)?;
    // 0x186a8: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 0)?)?;
    // 0x186ac: str r3, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 8)?)?;
    // 0x186b0: add r3, r2, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R2, 0xc)?)?;
    // 0x186b4: moveq r7, #4
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(7)?, R::R0, 0x4)?)?;
    // 0x186b8: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::LR, 0)?)?;
    // 0x186bc: str r6, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 24)?)?;
    // 0x186c0: str r7, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R2, 8)?)?;
    // 0x186c4: str lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::SP, 20)?)?;
    // 0x186c8: str r5, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 16)?)?;
    // 0x186cc: bne 0x186dc <later+0x2fc>
    c.b(C::Ne, 0x186dc)?;
    // 0x186d0: ldr r5, [r9, #0xc98]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(9)?, 3224)?)?;
    // 0x186d4: ldr r6, [r9, #0xc9c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 3228)?)?;
    // 0x186d8: b 0x18740 <later+0x360>
    c.b(C::Al, 0x18740)?;
    // 0x186dc: mov r1, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xec)?)?;
    // 0x186e0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x186e4: ldr r1, [r5, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(5)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x186e8: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    // 0x186ec: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 0x186f0: bhi 0x18700 <later+0x320>
    c.b(C::Hi, 0x18700)?;
    // 0x186f4: ldr r5, [r9, #0xce0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(9)?, 3296)?)?;
    // 0x186f8: ldr r6, [r9, #0xce4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 3300)?)?;
    // 0x186fc: b 0x18740 <later+0x360>
    c.b(C::Al, 0x18740)?;
    // 0x18700: ldr r6, [r9, #0xc9c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 3228)?)?;
    // 0x18704: ldr r5, [r9, #0xc98]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(9)?, 3224)?)?;
    // 0x18708: ldr r2, [pc, #0xf80]
    c.literal(R::R2, 0x19690)?;
    // 0x1870c: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x1)?)?;
    // 0x18710: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x18714: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x18718: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1871c: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x1)?)?;
    // 0x18720: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x18724: and r0, r1, r0
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x18728: mov r1, #194
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xc2)?)?;
    // 0x1872c: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x18730: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x18734: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x18738: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 0x1873c: ldr r3, [lr]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::LR, 0)?)?;
    // 0x18740: ldr r2, [r9, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2648)?)?;
    // 0x18744: lsl r5, r5, #12
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 12)?)?;
    // 0x18748: mov r4, #1040
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x410)?)?;
    // 0x1874c: add r1, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x4)?)?;
    // 0x18750: orr r6, r5, r6, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(6)?, R::new(5)?, R::new(6)?, Shift::Lsl, 4, false)?)?;
    // 0x18754: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x18758: orr r4, r4, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(4)?, R::new(4)?, 0x9000)?)?;
    // 0x1875c: ldr r7, [r2, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R2, 60)?)?;
    // 0x18760: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 0)?)?;
    // 0x18764: str r4, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::R0, 0)?)?;
    // 0x18768: add r0, r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(3)?, 0x4)?)?;
    // 0x1876c: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::LR, 0)?)?;
    // 0x18770: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x0)?)?;
    // 0x18774: orrne r6, r6, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Orr, R::new(6)?, R::new(6)?, 0x1)?)?;
    // 0x18778: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x2)?)?;
    // 0x1877c: moveq r5, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x18780: orr r7, r6, r5, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::new(6)?, R::new(5)?, Shift::Lsl, 2, false)?)?;
    // 0x18784: str r7, [r3]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(3)?, 0)?)?;
    // 0x18788: ldr r3, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 24)?)?;
    // 0x1878c: cmp r3, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x6)?)?;
    // 0x18790: bne 0x187a0 <later+0x3c0>
    c.b(C::Ne, 0x187a0)?;
    // 0x18794: ldr r5, [r9, #0xce0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(9)?, 3296)?)?;
    // 0x18798: ldr r6, [r9, #0xce4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 3300)?)?;
    // 0x1879c: b 0x1880c <later+0x42c>
    c.b(C::Al, 0x1880c)?;
    // 0x187a0: ldr r3, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 16)?)?;
    // 0x187a4: cmp r3, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x7)?)?;
    // 0x187a8: bne 0x187c8 <later+0x3e8>
    c.b(C::Ne, 0x187c8)?;
    // 0x187ac: ldr r7, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 16)?)?;
    // 0x187b0: mov r3, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xec)?)?;
    // 0x187b4: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x187b8: ldr r3, [r7, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::new(7)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x187bc: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::new(3)?, R::new(3)?, 0x1)?)?;
    // 0x187c0: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x1)?)?;
    // 0x187c4: bls 0x18794 <later+0x3b4>
    c.b(C::Ls, 0x18794)?;
    // 0x187c8: ldr r6, [r9, #0xc9c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 3228)?)?;
    // 0x187cc: ldr r5, [r9, #0xc98]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(9)?, 3224)?)?;
    // 0x187d0: ldr r2, [pc, #0xeb8]
    c.literal(R::R2, 0x19690)?;
    // 0x187d4: cmp r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x1)?)?;
    // 0x187d8: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 0x187dc: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 0x187e0: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x187e4: cmp r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x1)?)?;
    // 0x187e8: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x187ec: and r0, r1, r0
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 0x187f0: mov r1, #227
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe3)?)?;
    // 0x187f4: orr r1, r1, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1400)?)?;
    // 0x187f8: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x187fc: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x18800: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 0)?)?;
    // 0x18804: ldr r2, [r9, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2648)?)?;
    // 0x18808: ldr r0, [lr]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::LR, 0)?)?;
    // 0x1880c: ldr r7, [r2, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R2, 60)?)?;
    // 0x18810: mov r3, #20
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x14)?)?;
    // 0x18814: mov r12, r9
    c.emit(arm32::mov_register(C::Al, R::R12, R::new(9)?))?;
    // 0x18818: orr r3, r3, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9400)?)?;
    // 0x1881c: str r3, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 0)?)?;
    // 0x18820: lsl r3, r5, #12
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(5)?, Shift::Lsl, 12)?)?;
    // 0x18824: add r5, r1, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R1, 0x8)?)?;
    // 0x18828: orr r6, r3, r6, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(6)?, R::new(3)?, R::new(6)?, Shift::Lsl, 4, false)?)?;
    // 0x1882c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x18830: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x0)?)?;
    // 0x18834: orrne r6, r6, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Orr, R::new(6)?, R::new(6)?, 0x1)?)?;
    // 0x18838: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x2)?)?;
    // 0x1883c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x18840: moveq r7, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x18844: orr r7, r6, r7, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::new(6)?, R::new(7)?, Shift::Lsl, 2, false)?)?;
    // 0x18848: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 0)?)?;
    // 0x1884c: ldr r7, [r9, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2480)?)?;
    // 0x18850: ldr r6, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 2484)?)?;
    // 0x18854: str r5, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(10)?, 0)?)?;
    // 0x18858: mov r5, #1072
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x430)?)?;
    // 0x1885c: orr r5, r5, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x9000)?)?;
    // 0x18860: str r5, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 4)?)?;
    // 0x18864: add r5, r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x8)?)?;
    // 0x18868: str r5, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::LR, 0)?)?;
    // 0x1886c: mov r5, #52
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x34)?)?;
    // 0x18870: orr r7, r7, r6, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::new(7)?, R::new(6)?, Shift::Lsl, 16, false)?)?;
    // 0x18874: orr r5, r5, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x9400)?)?;
    // 0x18878: str r7, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 4)?)?;
    // 0x1887c: ldr r6, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 24)?)?;
    // 0x18880: ldr r7, [r9, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2480)?)?;
    // 0x18884: cmp r6, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x6)?)?;
    // 0x18888: ldr r6, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 2484)?)?;
    // 0x1888c: ldreq r3, [r9, #0x9f0]
    c.emit(arm32::transfer_word(C::Eq, true, R::new(3)?, R::new(9)?, 2544)?)?;
    // 0x18890: str r5, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 8)?)?;
    // 0x18894: lsl r6, r6, #16
    c.emit(arm32::mov_shifted_register(C::Al, R::new(6)?, R::new(6)?, Shift::Lsl, 16)?)?;
    // 0x18898: orr r3, r6, r7, lsl r3
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Orr, R::new(3)?, R::new(6)?, R::new(7)?, Shift::Lsl, R::new(3)?, false)?)?;
    // 0x1889c: add r7, r1, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R1, 0x10)?)?;
    // 0x188a0: str r3, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 8)?)?;
    // 0x188a4: ldr r3, [r12, #0xcbc]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, true, R::new(3)?, R::R12, 3260)?)?;
    // 0x188a8: str r7, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(10)?, 0)?)?;
    // 0x188ac: mov r7, #1120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x460)?)?;
    // 0x188b0: orr r7, r7, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x9000)?)?;
    // 0x188b4: str r7, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 12)?)?;
    // 0x188b8: add r7, r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0x10)?)?;
    // 0x188bc: str r7, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::LR, 0)?)?;
    // 0x188c0: str r3, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 12)?)?;
    // 0x188c4: ldr r3, [r12, #-0xcac]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R12, -3244)?)?;
    // 0x188c8: cmp r3, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x7)?)?;
    // 0x188cc: bne 0x188f8 <later+0x518>
    c.b(C::Ne, 0x188f8)?;
    // 0x188d0: ldr r6, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 16)?)?;
    // 0x188d4: mov r3, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xec)?)?;
    // 0x188d8: add r7, r9, #260
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(9)?, 0x104)?)?;
    // 0x188dc: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x188e0: ldr r3, [r6, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(3)?, R::new(6)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x188e4: sub r3, r3, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::new(3)?, R::new(3)?, 0x1)?)?;
    // 0x188e8: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x2)?)?;
    // 0x188ec: mov r3, r12
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R12))?;
    // 0x188f0: addlo r3, r7, #3072
    c.emit(arm32::data_immediate(C::Cc, Op::Add, R::new(3)?, R::new(7)?, 0xc00)?)?;
    // 0x188f4: b 0x18900 <later+0x520>
    c.b(C::Al, 0x18900)?;
    // 0x188f8: add r3, r9, #260
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(9)?, 0x104)?)?;
    // 0x188fc: add r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0xc00)?)?;
    // 0x18900: ldr r3, [r3]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(3)?, 0)?)?;
    // 0x18904: mov r7, #100
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x64)?)?;
    // 0x18908: orr r7, r7, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x9400)?)?;
    // 0x1890c: str r7, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 16)?)?;
    // 0x18910: str r3, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 16)?)?;
    // 0x18914: mov r3, #1248
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4e0)?)?;
    // 0x18918: orr r3, r3, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9000)?)?;
    // 0x1891c: str r3, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 20)?)?;
    // 0x18920: mov r3, #6
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x6)?)?;
    // 0x18924: orr r3, r3, #5242880
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x500000)?)?;
    // 0x18928: str r3, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 20)?)?;
    // 0x1892c: mov r3, #5504
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x1580)?)?;
    // 0x18930: orr r3, r3, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8000)?)?;
    // 0x18934: str r3, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 24)?)?;
    // 0x18938: mov r3, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x2)?)?;
    // 0x1893c: str r3, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 24)?)?;
    // 0x18940: add r3, r1, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R1, 0x20)?)?;
    // 0x18944: ldr lr, [r2, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::R2, 68)?)?;
    // 0x18948: ldr r7, [r2, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R2, 72)?)?;
    // 0x1894c: ldr r6, [r2, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::R2, 76)?)?;
    // 0x18950: ldr r5, [r2, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R2, 80)?)?;
    // 0x18954: ldr r4, [r2, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::R2, 84)?)?;
    // 0x18958: ldr r2, [r2, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R2, 88)?)?;
    // 0x1895c: str r3, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(10)?, 0)?)?;
    // 0x18960: mov r3, #388
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x184)?)?;
    // 0x18964: orr r3, r3, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9400)?)?;
    // 0x18968: str r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 28)?)?;
    // 0x1896c: add r3, r0, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x20)?)?;
    // 0x18970: ldr r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::SP, 20)?)?;
    // 0x18974: str r3, [r8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(8)?, 0)?)?;
    // 0x18978: orr r3, r6, r5, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(6)?, R::new(5)?, Shift::Lsl, 1, false)?)?;
    // 0x1897c: orr r3, r3, r4, lsl #7
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::new(4)?, Shift::Lsl, 7, false)?)?;
    // 0x18980: orr r2, r3, r2, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(3)?, R::R2, Shift::Lsl, 2, false)?)?;
    // 0x18984: orr r2, r2, lr, lsl #10
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::LR, Shift::Lsl, 10, false)?)?;
    // 0x18988: orr r2, r2, r7, lsl #11
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(7)?, Shift::Lsl, 11, false)?)?;
    // 0x1898c: orr r2, r2, #512
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x200)?)?;
    // 0x18990: str r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 28)?)?;
    // 0x18994: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x18998: cmp r2, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x6)?)?;
    // 0x1899c: bne 0x18b34 <later+0x754>
    c.b(C::Ne, 0x18b34)?;
    // 0x189a0: ldr r4, [r9, #0xca8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(9)?, 3240)?)?;
    // 0x189a4: ldr r2, [r9, #0x99c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2460)?)?;
    // 0x189a8: ldr r7, [r9, #0xca0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 3232)?)?;
    // 0x189ac: mov r12, #516
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x204)?)?;
    // 0x189b0: mov r5, #50
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x32)?)?;
    // 0x189b4: mov lr, #38400
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::LR, R::R0, 0x9600)?)?;
    // 0x189b8: mov r3, #66
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x42)?)?;
    // 0x189bc: mov r8, r10
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(10)?))?;
    // 0x189c0: orr r12, r12, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R12, R::R12, 0x9400)?)?;
    // 0x189c4: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1000)?)?;
    // 0x189c8: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x189cc: str r12, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 32)?)?;
    // 0x189d0: str r5, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 32)?)?;
    // 0x189d4: str lr, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R1, 36)?)?;
    // 0x189d8: add r6, r2, r4
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::R2, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 0x189dc: add r7, r7, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(7)?, 0xff)?)?;
    // 0x189e0: str r6, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 36)?)?;
    // 0x189e4: mov r6, #644
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x284)?)?;
    // 0x189e8: bic r7, r7, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::new(7)?, R::new(7)?, 0xff)?)?;
    // 0x189ec: orr r6, r6, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x9400)?)?;
    // 0x189f0: orr r7, r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1)?)?;
    // 0x189f4: str r6, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 40)?)?;
    // 0x189f8: str r5, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 40)?)?;
    // 0x189fc: mov r5, #5760
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x1680)?)?;
    // 0x18a00: orr r5, r5, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x8000)?)?;
    // 0x18a04: str r5, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 44)?)?;
    // 0x18a08: str r7, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 44)?)?;
    // 0x18a0c: mov r7, #51
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x33)?)?;
    // 0x18a10: str r12, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 48)?)?;
    // 0x18a14: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18a18: str r7, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 48)?)?;
    // 0x18a1c: str lr, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R1, 52)?)?;
    // 0x18a20: str r2, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 52)?)?;
    // 0x18a24: add r2, r4, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(4)?, 0xff)?)?;
    // 0x18a28: str r6, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 56)?)?;
    // 0x18a2c: str r7, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 56)?)?;
    // 0x18a30: str r5, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 60)?)?;
    // 0x18a34: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18a38: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18a3c: str r2, [r0, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 60)?)?;
    // 0x18a40: ldr r2, [r9, #0xce8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 3304)?)?;
    // 0x18a44: ldr r7, [r9, #0xcf0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 3312)?)?;
    // 0x18a48: ldr r4, [r9, #0x9a8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(9)?, 2472)?)?;
    // 0x18a4c: str r12, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 64)?)?;
    // 0x18a50: str r3, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 64)?)?;
    // 0x18a54: str lr, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R1, 68)?)?;
    // 0x18a58: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18a5c: add r10, r4, r7
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(10)?, R::new(4)?, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0x18a60: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18a64: str r10, [r0, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 68)?)?;
    // 0x18a68: str r6, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 72)?)?;
    // 0x18a6c: str r3, [r0, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 72)?)?;
    // 0x18a70: str r5, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 76)?)?;
    // 0x18a74: mov r10, r8
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(8)?))?;
    // 0x18a78: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18a7c: str r2, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 76)?)?;
    // 0x18a80: mov r2, #67
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x43)?)?;
    // 0x18a84: str r12, [r1, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 80)?)?;
    // 0x18a88: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x18a8c: str r2, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 80)?)?;
    // 0x18a90: str lr, [r1, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R1, 84)?)?;
    // 0x18a94: str r4, [r0, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::R0, 84)?)?;
    // 0x18a98: str r6, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 88)?)?;
    // 0x18a9c: str r2, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 88)?)?;
    // 0x18aa0: add r2, r7, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(7)?, 0xff)?)?;
    // 0x18aa4: str r5, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 92)?)?;
    // 0x18aa8: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18aac: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18ab0: str r2, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 92)?)?;
    // 0x18ab4: mov r2, #97
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x61)?)?;
    // 0x18ab8: str r12, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 96)?)?;
    // 0x18abc: str r2, [r0, #0x60]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 96)?)?;
    // 0x18ac0: str lr, [r1, #0x64]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R1, 100)?)?;
    // 0x18ac4: ldr r3, [r9, #0x9dc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2524)?)?;
    // 0x18ac8: str r3, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 100)?)?;
    // 0x18acc: str r6, [r1, #0x68]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 104)?)?;
    // 0x18ad0: str r2, [r0, #0x68]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 104)?)?;
    // 0x18ad4: str r5, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 108)?)?;
    // 0x18ad8: ldr r2, [r9, #0x9e0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2528)?)?;
    // 0x18adc: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18ae0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18ae4: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18ae8: str r2, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 108)?)?;
    // 0x18aec: mov r2, #113
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x71)?)?;
    // 0x18af0: str r12, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 112)?)?;
    // 0x18af4: str r2, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 112)?)?;
    // 0x18af8: str lr, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::LR, R::R1, 116)?)?;
    // 0x18afc: ldr r3, [r9, #0x9ec]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2540)?)?;
    c.check(0x18b00)
}

/// Encode low-level codec descriptor fields and fixed-length packet headers.
fn emit_descriptor_fields(c: &mut Code) -> Result<(), String> {
    c.check(0x18b00)?;
    // 0x18b00: str r3, [r0, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 116)?)?;
    // 0x18b04: str r6, [r1, #0x78]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 120)?)?;
    // 0x18b08: str r2, [r0, #0x78]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 120)?)?;
    // 0x18b0c: add r2, r1, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x80)?)?;
    // 0x18b10: add r3, r0, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x80)?)?;
    // 0x18b14: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 0)?)?;
    // 0x18b18: str r5, [r1, #0x7c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 124)?)?;
    // 0x18b1c: ldr r2, [r9, #0x9e8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2536)?)?;
    // 0x18b20: ldr r7, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 20)?)?;
    // 0x18b24: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18b28: str r3, [r7]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(7)?, 0)?)?;
    // 0x18b2c: mov lr, r7
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(7)?))?;
    // 0x18b30: b 0x18cf0 <later+0x910>
    c.b(C::Al, 0x18cf0)?;
    // 0x18b34: ldr r3, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 16)?)?;
    // 0x18b38: mov r2, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xec)?)?;
    // 0x18b3c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x18b40: ldr r2, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::new(3)?, R::R2, Shift::Lsl, 0)?)?;
    // 0x18b44: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x2)?)?;
    // 0x18b48: cmpne r2, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Cmp, R::R0, R::R2, 0x1)?)?;
    // 0x18b4c: bne 0x18b64 <later+0x784>
    c.b(C::Ne, 0x18b64)?;
    // 0x18b50: add r2, r9, #220
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(9)?, 0xdc)?)?;
    // 0x18b54: add r3, r9, #516
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(9)?, 0x204)?)?;
    // 0x18b58: add r2, r2, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xc00)?)?;
    // 0x18b5c: add r3, r3, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x800)?)?;
    // 0x18b60: b 0x18b6c <later+0x78c>
    c.b(C::Al, 0x18b6c)?;
    // 0x18b64: sub r2, r12, #40
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R2, R::R12, 0x28)?)?;
    // 0x18b68: add r3, r9, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(9)?, 0xa00)?)?;
    // 0x18b6c: ldr r7, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::R2, 12)?)?;
    // 0x18b70: ldr r4, [r2, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::R2, 20)?)?;
    // 0x18b74: ldr r2, [r3]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(3)?, 0)?)?;
    // 0x18b78: mov r12, #516
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x204)?)?;
    // 0x18b7c: mov r5, #66
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x42)?)?;
    // 0x18b80: mov r3, #38400
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x9600)?)?;
    // 0x18b84: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x18b88: orr r12, r12, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R12, R::R12, 0x9400)?)?;
    // 0x18b8c: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1000)?)?;
    // 0x18b90: str r12, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 32)?)?;
    // 0x18b94: str r5, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 32)?)?;
    // 0x18b98: str r3, [r1, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 36)?)?;
    // 0x18b9c: add r6, r2, r4
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::R2, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 0x18ba0: add r7, r7, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(7)?, 0xff)?)?;
    // 0x18ba4: str r6, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 36)?)?;
    // 0x18ba8: mov r6, #644
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x284)?)?;
    // 0x18bac: bic r7, r7, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::new(7)?, R::new(7)?, 0xff)?)?;
    // 0x18bb0: orr r6, r6, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x9400)?)?;
    // 0x18bb4: orr r7, r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1)?)?;
    // 0x18bb8: str r6, [r1, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 40)?)?;
    // 0x18bbc: str r5, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R0, 40)?)?;
    // 0x18bc0: mov r5, #5760
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x1680)?)?;
    // 0x18bc4: orr r5, r5, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x8000)?)?;
    // 0x18bc8: str r5, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 44)?)?;
    // 0x18bcc: str r7, [r0, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 44)?)?;
    // 0x18bd0: mov r7, #67
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x43)?)?;
    // 0x18bd4: str r12, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 48)?)?;
    // 0x18bd8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18bdc: str r7, [r0, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 48)?)?;
    // 0x18be0: str r3, [r1, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 52)?)?;
    // 0x18be4: str r2, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 52)?)?;
    // 0x18be8: add r2, r4, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(4)?, 0xff)?)?;
    // 0x18bec: str r6, [r1, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 56)?)?;
    // 0x18bf0: str r7, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 56)?)?;
    // 0x18bf4: str r5, [r1, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 60)?)?;
    // 0x18bf8: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18bfc: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18c00: str r2, [r0, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 60)?)?;
    // 0x18c04: mov r2, #97
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x61)?)?;
    // 0x18c08: str r12, [r1, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 64)?)?;
    // 0x18c0c: str r2, [r0, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 64)?)?;
    // 0x18c10: str r3, [r1, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 68)?)?;
    // 0x18c14: ldr r7, [r9, #0xa1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2588)?)?;
    // 0x18c18: str r7, [r0, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 68)?)?;
    // 0x18c1c: str r6, [r1, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 72)?)?;
    // 0x18c20: str r2, [r0, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 72)?)?;
    // 0x18c24: str r5, [r1, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 76)?)?;
    // 0x18c28: ldr r2, [r9, #0xa18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2584)?)?;
    // 0x18c2c: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18c30: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18c34: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18c38: str r2, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 76)?)?;
    // 0x18c3c: mov r2, #113
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x71)?)?;
    // 0x18c40: str r12, [r1, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 80)?)?;
    // 0x18c44: str r2, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 80)?)?;
    // 0x18c48: str r3, [r1, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 84)?)?;
    // 0x18c4c: ldr r7, [r9, #0xa24]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2596)?)?;
    // 0x18c50: str r7, [r0, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 84)?)?;
    // 0x18c54: str r6, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 88)?)?;
    // 0x18c58: str r2, [r0, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 88)?)?;
    // 0x18c5c: str r5, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 92)?)?;
    // 0x18c60: ldr r2, [r9, #0xa20]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2592)?)?;
    // 0x18c64: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18c68: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18c6c: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18c70: str r2, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 92)?)?;
    // 0x18c74: mov r2, #65
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x41)?)?;
    // 0x18c78: str r12, [r1, #0x60]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 96)?)?;
    // 0x18c7c: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x18c80: str r2, [r0, #0x60]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 96)?)?;
    // 0x18c84: str r3, [r1, #0x64]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 100)?)?;
    // 0x18c88: ldr r7, [r9, #0xa0c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2572)?)?;
    // 0x18c8c: str r7, [r0, #0x64]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 100)?)?;
    // 0x18c90: str r6, [r1, #0x68]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 104)?)?;
    // 0x18c94: str r2, [r0, #0x68]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 104)?)?;
    // 0x18c98: str r5, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 108)?)?;
    // 0x18c9c: ldr r2, [r9, #0xa08]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2568)?)?;
    // 0x18ca0: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18ca4: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18ca8: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18cac: str r2, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 108)?)?;
    // 0x18cb0: mov r2, #81
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x51)?)?;
    // 0x18cb4: str r12, [r1, #0x70]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::R1, 112)?)?;
    // 0x18cb8: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 0x18cbc: str r2, [r0, #0x70]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 112)?)?;
    // 0x18cc0: str r3, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 116)?)?;
    // 0x18cc4: ldr r3, [r9, #0xa14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2580)?)?;
    // 0x18cc8: str r3, [r0, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 116)?)?;
    // 0x18ccc: str r6, [r1, #0x78]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R1, 120)?)?;
    // 0x18cd0: str r2, [r0, #0x78]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 120)?)?;
    // 0x18cd4: add r2, r1, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0x80)?)?;
    // 0x18cd8: add r3, r0, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x80)?)?;
    // 0x18cdc: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 0)?)?;
    // 0x18ce0: str r5, [r1, #0x7c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 124)?)?;
    // 0x18ce4: ldr r2, [r9, #0xa10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2576)?)?;
    // 0x18ce8: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::LR, 0)?)?;
    // 0x18cec: add r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0xff)?)?;
    // 0x18cf0: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 0x18cf4: orr r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1)?)?;
    // 0x18cf8: str r2, [r0, #0x7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 124)?)?;
    // 0x18cfc: mov r3, #580
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x244)?)?;
    // 0x18d00: mov r6, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x8)?)?;
    // 0x18d04: mov r5, #548
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x224)?)?;
    // 0x18d08: mov r7, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x48)?)?;
    // 0x18d0c: mov r2, #104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x68)?)?;
    // 0x18d10: orr r3, r3, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1c00)?)?;
    // 0x18d14: orr r5, r5, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x9400)?)?;
    // 0x18d18: orr r7, r7, #2097152
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x200000)?)?;
    // 0x18d1c: orr r2, r2, #262144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x40000)?)?;
    // 0x18d20: str r6, [r9, r3]
    c.emit(arm32::transfer_word_register(C::Al, false, R::new(6)?, R::new(9)?, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x18d24: mov r3, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x2)?)?;
    // 0x18d28: mov r6, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x8)?)?;
    // 0x18d2c: str r5, [r1, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 128)?)?;
    // 0x18d30: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x18d34: orr r6, r6, #4194304
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x400000)?)?;
    // 0x18d38: str r3, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 128)?)?;
    // 0x18d3c: mov r3, #1568
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x620)?)?;
    // 0x18d40: orr r3, r3, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x9000)?)?;
    // 0x18d44: str r3, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 132)?)?;
    // 0x18d48: str r6, [r0, #0x84]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 132)?)?;
    // 0x18d4c: add r6, r1, #140
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R1, 0x8c)?)?;
    // 0x18d50: str r6, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::new(10)?, 0)?)?;
    // 0x18d54: add r6, r0, #140
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0x8c)?)?;
    // 0x18d58: str r5, [r1, #0x88]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 136)?)?;
    // 0x18d5c: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::LR, 0)?)?;
    // 0x18d60: ldr r6, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 24)?)?;
    // 0x18d64: cmp r6, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x6)?)?;
    // 0x18d68: bne 0x18efc <later+0xb1c>
    c.b(C::Ne, 0x18efc)?;
    // 0x18d6c: mov r6, #34
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x22)?)?;
    // 0x18d70: add r7, r7, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(7)?, 0x80)?)?;
    // 0x18d74: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x18d78: str r6, [r0, #0x88]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 136)?)?;
    // 0x18d7c: mov r6, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x48)?)?;
    // 0x18d80: str r3, [r1, #0x8c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 140)?)?;
    // 0x18d84: orr r6, r6, #8388608
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x800000)?)?;
    // 0x18d88: str r6, [r0, #0x8c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 140)?)?;
    // 0x18d8c: mov r6, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x3)?)?;
    // 0x18d90: str r5, [r1, #0x90]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 144)?)?;
    // 0x18d94: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x18d98: str r6, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 144)?)?;
    // 0x18d9c: str r3, [r1, #0x94]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 148)?)?;
    // 0x18da0: str r7, [r0, #0x94]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 148)?)?;
    // 0x18da4: mov r7, #65
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x41)?)?;
    // 0x18da8: str r5, [r1, #0x98]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 152)?)?;
    // 0x18dac: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18db0: str r7, [r0, #0x98]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 152)?)?;
    // 0x18db4: add r7, r2, #128
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x80)?)?;
    // 0x18db8: str r3, [r1, #0x9c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 156)?)?;
    // 0x18dbc: str r7, [r0, #0x9c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 156)?)?;
    // 0x18dc0: mov r7, #81
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x51)?)?;
    // 0x18dc4: str r5, [r1, #0xa0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 160)?)?;
    // 0x18dc8: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18dcc: str r7, [r0, #0xa0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 160)?)?;
    // 0x18dd0: add r7, r2, #132
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x84)?)?;
    // 0x18dd4: str r3, [r1, #0xa4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 164)?)?;
    // 0x18dd8: str r7, [r0, #0xa4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 164)?)?;
    // 0x18ddc: mov r7, #65
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x41)?)?;
    // 0x18de0: str r5, [r1, #0xa8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 168)?)?;
    // 0x18de4: str r7, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 168)?)?;
    // 0x18de8: add r7, r2, #136
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x88)?)?;
    // 0x18dec: str r3, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 172)?)?;
    // 0x18df0: str r7, [r0, #0xac]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 172)?)?;
    // 0x18df4: mov r7, #81
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x51)?)?;
    // 0x18df8: str r5, [r1, #0xb0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 176)?)?;
    // 0x18dfc: str r7, [r0, #0xb0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 176)?)?;
    // 0x18e00: add r7, r2, #140
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x8c)?)?;
    // 0x18e04: str r3, [r1, #0xb4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 180)?)?;
    // 0x18e08: str r7, [r0, #0xb4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 180)?)?;
    // 0x18e0c: mov r7, #97
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x61)?)?;
    // 0x18e10: str r5, [r1, #0xb8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 184)?)?;
    // 0x18e14: str r7, [r0, #0xb8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 184)?)?;
    // 0x18e18: add r7, r2, #144
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x90)?)?;
    // 0x18e1c: str r3, [r1, #0xbc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 188)?)?;
    // 0x18e20: str r7, [r0, #0xbc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 188)?)?;
    // 0x18e24: mov r7, #113
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x71)?)?;
    // 0x18e28: str r5, [r1, #0xc0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 192)?)?;
    // 0x18e2c: str r7, [r0, #0xc0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 192)?)?;
    // 0x18e30: add r7, r2, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x94)?)?;
    // 0x18e34: str r3, [r1, #0xc4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 196)?)?;
    // 0x18e38: str r7, [r0, #0xc4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 196)?)?;
    // 0x18e3c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x18e40: str r5, [r1, #0xc8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 200)?)?;
    // 0x18e44: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18e48: str r7, [r0, #0xc8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 200)?)?;
    // 0x18e4c: add r7, r2, #152
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x98)?)?;
    // 0x18e50: str r3, [r1, #0xcc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 204)?)?;
    // 0x18e54: str r7, [r0, #0xcc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 204)?)?;
    // 0x18e58: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x18e5c: str r5, [r1, #0xd0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 208)?)?;
    // 0x18e60: str r7, [r0, #0xd0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 208)?)?;
    // 0x18e64: add r7, r2, #156
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x9c)?)?;
    // 0x18e68: str r3, [r1, #0xd4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 212)?)?;
    // 0x18e6c: str r7, [r0, #0xd4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 212)?)?;
    // 0x18e70: mov r7, #33
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x21)?)?;
    // 0x18e74: str r5, [r1, #0xd8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 216)?)?;
    // 0x18e78: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18e7c: str r7, [r0, #0xd8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 216)?)?;
    // 0x18e80: add r7, r2, #160
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0xa0)?)?;
    // 0x18e84: str r3, [r1, #0xdc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 220)?)?;
    // 0x18e88: str r7, [r0, #0xdc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 220)?)?;
    // 0x18e8c: mov r7, #33
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x21)?)?;
    // 0x18e90: str r5, [r1, #0xe0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 224)?)?;
    // 0x18e94: str r7, [r0, #0xe0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 224)?)?;
    // 0x18e98: add r7, r2, #164
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0xa4)?)?;
    // 0x18e9c: str r3, [r1, #0xe4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 228)?)?;
    // 0x18ea0: str r7, [r0, #0xe4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 228)?)?;
    // 0x18ea4: mov r7, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x31)?)?;
    // 0x18ea8: str r5, [r1, #0xe8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 232)?)?;
    // 0x18eac: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18eb0: str r7, [r0, #0xe8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 232)?)?;
    // 0x18eb4: add r7, r2, #168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0xa8)?)?;
    // 0x18eb8: str r3, [r1, #0xec]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 236)?)?;
    // 0x18ebc: str r7, [r0, #0xec]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 236)?)?;
    // 0x18ec0: str r5, [r1, #0xf0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 240)?)?;
    // 0x18ec4: mov r7, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x31)?)?;
    // 0x18ec8: add r5, r1, #248
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R1, 0xf8)?)?;
    // 0x18ecc: str r7, [r0, #0xf0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 240)?)?;
    // 0x18ed0: str r5, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(10)?, 0)?)?;
    // 0x18ed4: str r3, [r1, #0xf4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 244)?)?;
    // 0x18ed8: add r3, r0, #248
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xf8)?)?;
    // 0x18edc: add r1, r2, #172
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0xac)?)?;
    // 0x18ee0: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::LR, 0)?)?;
    // 0x18ee4: str r1, [r0, #0xf4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 244)?)?;
    // 0x18ee8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x18eec: ldr r1, [r9, #0xc9c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 3228)?)?;
    // 0x18ef0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x18ef4: movne r0, #2
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x18ef8: b 0x19088 <later+0xca8>
    c.b(C::Al, 0x19088)?;
    // 0x18efc: mov r6, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x3)?)?;
    // 0x18f00: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x18f04: str r6, [r0, #0x88]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 136)?)?;
    // 0x18f08: str r3, [r1, #0x8c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 140)?)?;
    // 0x18f0c: str r7, [r0, #0x8c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 140)?)?;
    // 0x18f10: mov r7, #65
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x41)?)?;
    // 0x18f14: str r5, [r1, #0x90]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 144)?)?;
    // 0x18f18: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18f1c: str r7, [r0, #0x90]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 144)?)?;
    // 0x18f20: mov r7, #81
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x51)?)?;
    // 0x18f24: str r3, [r1, #0x94]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 148)?)?;
    // 0x18f28: str r2, [r0, #0x94]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 148)?)?;
    // 0x18f2c: str r5, [r1, #0x98]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 152)?)?;
    // 0x18f30: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18f34: str r7, [r0, #0x98]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 152)?)?;
    // 0x18f38: add r7, r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x4)?)?;
    // 0x18f3c: str r3, [r1, #0x9c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 156)?)?;
    // 0x18f40: str r7, [r0, #0x9c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 156)?)?;
    // 0x18f44: mov r7, #65
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x41)?)?;
    // 0x18f48: str r5, [r1, #0xa0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 160)?)?;
    // 0x18f4c: str r7, [r0, #0xa0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 160)?)?;
    // 0x18f50: add r7, r2, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x8)?)?;
    // 0x18f54: str r3, [r1, #0xa4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 164)?)?;
    // 0x18f58: str r7, [r0, #0xa4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 164)?)?;
    // 0x18f5c: mov r7, #81
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x51)?)?;
    // 0x18f60: str r5, [r1, #0xa8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 168)?)?;
    // 0x18f64: str r7, [r0, #0xa8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 168)?)?;
    // 0x18f68: add r7, r2, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0xc)?)?;
    // 0x18f6c: str r3, [r1, #0xac]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 172)?)?;
    // 0x18f70: str r7, [r0, #0xac]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 172)?)?;
    // 0x18f74: mov r7, #97
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x61)?)?;
    // 0x18f78: str r5, [r1, #0xb0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 176)?)?;
    // 0x18f7c: str r7, [r0, #0xb0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 176)?)?;
    // 0x18f80: add r7, r2, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x10)?)?;
    // 0x18f84: str r3, [r1, #0xb4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 180)?)?;
    // 0x18f88: str r7, [r0, #0xb4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 180)?)?;
    // 0x18f8c: mov r7, #113
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x71)?)?;
    // 0x18f90: str r5, [r1, #0xb8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 184)?)?;
    // 0x18f94: str r7, [r0, #0xb8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 184)?)?;
    // 0x18f98: add r7, r2, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x14)?)?;
    // 0x18f9c: str r3, [r1, #0xbc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 188)?)?;
    // 0x18fa0: str r7, [r0, #0xbc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 188)?)?;
    // 0x18fa4: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x18fa8: str r5, [r1, #0xc0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 192)?)?;
    // 0x18fac: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18fb0: str r7, [r0, #0xc0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 192)?)?;
    // 0x18fb4: add r7, r2, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x18)?)?;
    // 0x18fb8: str r3, [r1, #0xc4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 196)?)?;
    // 0x18fbc: str r7, [r0, #0xc4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 196)?)?;
    // 0x18fc0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x18fc4: str r5, [r1, #0xc8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 200)?)?;
    // 0x18fc8: str r7, [r0, #0xc8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 200)?)?;
    // 0x18fcc: add r7, r2, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x1c)?)?;
    // 0x18fd0: str r3, [r1, #0xcc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 204)?)?;
    // 0x18fd4: str r7, [r0, #0xcc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 204)?)?;
    // 0x18fd8: mov r7, #33
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x21)?)?;
    // 0x18fdc: str r5, [r1, #0xd0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 208)?)?;
    // 0x18fe0: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x18fe4: str r7, [r0, #0xd0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 208)?)?;
    // 0x18fe8: add r7, r2, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x20)?)?;
    // 0x18fec: str r3, [r1, #0xd4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 212)?)?;
    // 0x18ff0: str r7, [r0, #0xd4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 212)?)?;
    // 0x18ff4: mov r7, #33
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x21)?)?;
    // 0x18ff8: str r5, [r1, #0xd8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 216)?)?;
    // 0x18ffc: str r7, [r0, #0xd8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 216)?)?;
    c.check(0x19000)
}

/// Populate extended controller records and per-plane lookup tables.
fn emit_record_tables(c: &mut Code) -> Result<(), String> {
    c.check(0x19000)?;
    // 0x19000: add r7, r2, #36
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x24)?)?;
    // 0x19004: str r3, [r1, #0xdc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 220)?)?;
    // 0x19008: str r7, [r0, #0xdc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 220)?)?;
    // 0x1900c: mov r7, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x31)?)?;
    // 0x19010: str r5, [r1, #0xe0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 224)?)?;
    // 0x19014: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x19018: str r7, [r0, #0xe0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 224)?)?;
    // 0x1901c: add r7, r2, #40
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R2, 0x28)?)?;
    // 0x19020: str r3, [r1, #0xe4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 228)?)?;
    // 0x19024: str r7, [r0, #0xe4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 228)?)?;
    // 0x19028: str r5, [r1, #0xe8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::R1, 232)?)?;
    // 0x1902c: mov r7, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x31)?)?;
    // 0x19030: add r5, r1, #240
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R1, 0xf0)?)?;
    // 0x19034: str r7, [r0, #0xe8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 232)?)?;
    // 0x19038: str r5, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(10)?, 0)?)?;
    // 0x1903c: str r3, [r1, #0xec]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 236)?)?;
    // 0x19040: add r3, r0, #240
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xf0)?)?;
    // 0x19044: add r1, r2, #44
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0x2c)?)?;
    // 0x19048: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::LR, 0)?)?;
    // 0x1904c: str r1, [r0, #0xec]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 236)?)?;
    // 0x19050: ldr r0, [r9, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 16)?)?;
    // 0x19054: ldr r1, [r9, #0xc9c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 3228)?)?;
    // 0x19058: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x7)?)?;
    // 0x1905c: bne 0x1907c <later+0xc9c>
    c.b(C::Ne, 0x1907c)?;
    // 0x19060: ldr r2, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 16)?)?;
    // 0x19064: mov r0, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xec)?)?;
    // 0x19068: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 0x1906c: ldr r0, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R2, R::R0, Shift::Lsl, 0)?)?;
    // 0x19070: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x1)?)?;
    // 0x19074: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x19078: ldrls r1, [r9, #0xce4]
    c.emit(arm32::transfer_word(C::Ls, true, R::R1, R::new(9)?, 3300)?)?;
    // 0x1907c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 0x19080: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x19084: moveq r0, #4
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 0x19088: ldr r1, [r9, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 2648)?)?;
    // 0x1908c: add r2, r5, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(5)?, 0x4)?)?;
    // 0x19090: ldr r1, [r1, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 48)?)?;
    // 0x19094: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 0)?)?;
    // 0x19098: mov r2, #232
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xe8)?)?;
    // 0x1909c: orr r2, r2, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9400)?)?;
    // 0x190a0: str r2, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(5)?, 0)?)?;
    // 0x190a4: add r2, r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(3)?, 0x4)?)?;
    // 0x190a8: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::LR, 0)?)?;
    // 0x190ac: lsl r2, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R0, Shift::Lsl, 8)?)?;
    // 0x190b0: orr r1, r2, r1, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R1, R::R2, R::R1, Shift::Lsl, 16, false)?)?;
    // 0x190b4: orr r1, r1, #2
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x2)?)?;
    // 0x190b8: str r1, [r3]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(3)?, 0)?)?;
    // 0x190bc: ldr r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 24)?)?;
    // 0x190c0: cmp r1, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x6)?)?;
    // 0x190c4: bne 0x19134 <later+0xd54>
    c.b(C::Ne, 0x19134)?;
    // 0x190c8: ldr r1, [r9, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 2480)?)?;
    // 0x190cc: ldr r7, [r9, #0x9c8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2504)?)?;
    // 0x190d0: ldr r4, [r9, #0x9f0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(9)?, 2544)?)?;
    // 0x190d4: mov r2, #127
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x7f)?)?;
    // 0x190d8: mov r6, #128
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x80)?)?;
    // 0x190dc: add r2, r2, r1, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R2, R::R1, Shift::Lsl, 4, false)?)?;
    // 0x190e0: lsr r12, r2, #7
    c.emit(arm32::mov_shifted_register(C::Al, R::R12, R::R2, Shift::Lsr, 7)?)?;
    // 0x190e4: lsl r2, r7, r4
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Mov, R::R2, R::R0, R::new(7)?, Shift::Lsl, R::new(4)?, false)?)?;
    // 0x190e8: orr r2, r6, r2, lsl #10
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(6)?, R::R2, Shift::Lsl, 10, false)?)?;
    // 0x190ec: mov r6, #512
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x200)?)?;
    // 0x190f0: sub r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R2, R::R2, 0x1)?)?;
    // 0x190f4: lsr lr, r2, #7
    c.emit(arm32::mov_shifted_register(C::Al, R::LR, R::R2, Shift::Lsr, 7)?)?;
    // 0x190f8: ldr r2, [r9, #0xc9c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 3228)?)?;
    // 0x190fc: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x19100: movne r6, #256
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(6)?, R::R0, 0x100)?)?;
    // 0x19104: smulbb r0, r0, r6
    c.emit(signed_multiply_low_halfwords(C::Al, R::R0, R::R0, R::new(6)?)?)?;
    // 0x19108: lsl r0, r0, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 1)?)?;
    // 0x1910c: lsr r2, r0, #7
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R0, Shift::Lsr, 7)?)?;
    // 0x19110: mov r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x7)?)?;
    // 0x19114: add r0, r0, r7, lsl r4
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Add, R::R0, R::R0, R::new(7)?, Shift::Lsl, R::new(4)?, false)?)?;
    // 0x19118: bic r0, r0, #-268435449
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0xf0000007)?)?;
    // 0x1911c: mul r4, r1, r0
    c.emit(arm32::multiply_register(C::Al, R::new(4)?, R::R1, R::R0)?)?;
    // 0x19120: mvn r0, #-536870897
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R0, R::R0, 0xe000000f)?)?;
    // 0x19124: and r0, r0, r4, lsl #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R0, R::new(4)?, Shift::Lsl, 1, false)?)?;
    // 0x19128: add r0, r0, #127
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x7f)?)?;
    // 0x1912c: lsr r7, r0, #7
    c.emit(arm32::mov_shifted_register(C::Al, R::new(7)?, R::R0, Shift::Lsr, 7)?)?;
    // 0x19130: b 0x19144 <later+0xd64>
    c.b(C::Al, 0x19144)?;
    // 0x19134: mov r2, #60
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x3c)?)?;
    // 0x19138: mov lr, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::LR, R::R0, 0x0)?)?;
    // 0x1913c: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x19140: mov r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x0)?)?;
    // 0x19144: mov r0, #5440
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1540)?)?;
    // 0x19148: mov r1, #324
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x144)?)?;
    // 0x1914c: orr r4, r2, r2, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(4)?, R::R2, R::R2, Shift::Lsl, 16, false)?)?;
    // 0x19150: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8000)?)?;
    // 0x19154: orr r1, r1, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x9400)?)?;
    // 0x19158: str r0, [r5, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 4)?)?;
    // 0x1915c: lsl r0, r2, #16
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R2, Shift::Lsl, 16)?)?;
    // 0x19160: lsl r2, r2, #1
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 1)?)?;
    // 0x19164: str r0, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 4)?)?;
    // 0x19168: add r0, r5, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0xc)?)?;
    // 0x1916c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 0)?)?;
    // 0x19170: str r1, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(5)?, 8)?)?;
    // 0x19174: add r1, r3, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(3)?, 0xc)?)?;
    // 0x19178: ldr r6, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 20)?)?;
    // 0x1917c: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 0)?)?;
    // 0x19180: str r4, [r3, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::new(3)?, 8)?)?;
    // 0x19184: ldr r4, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 24)?)?;
    // 0x19188: cmp r4, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x6)?)?;
    // 0x1918c: bne 0x191e8 <later+0xe08>
    c.b(C::Ne, 0x191e8)?;
    // 0x19190: mov r0, #1360
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x550)?)?;
    // 0x19194: mov r1, #344
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x158)?)?;
    // 0x19198: orr r0, r0, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x9000)?)?;
    // 0x1919c: orr r1, r1, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x9400)?)?;
    // 0x191a0: str r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 12)?)?;
    // 0x191a4: orr r0, r2, lr, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R2, R::LR, Shift::Lsl, 16, false)?)?;
    // 0x191a8: add r2, r2, lr
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R2, R::LR, Shift::Lsl, 0, false)?)?;
    // 0x191ac: str r0, [r3, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 12)?)?;
    // 0x191b0: mov r0, #340
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x154)?)?;
    // 0x191b4: orr r0, r0, #37888
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x9400)?)?;
    // 0x191b8: str r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 16)?)?;
    // 0x191bc: orr r0, r2, r7, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R2, R::new(7)?, Shift::Lsl, 16, false)?)?;
    // 0x191c0: add r2, r2, r7
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R2, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0x191c4: str r0, [r3, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 16)?)?;
    // 0x191c8: add r0, r5, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x18)?)?;
    // 0x191cc: orr r7, r2, r12, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::R2, R::R12, Shift::Lsl, 16, false)?)?;
    // 0x191d0: add r2, r2, r12
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R2, R::R12, Shift::Lsl, 0, false)?)?;
    // 0x191d4: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 0)?)?;
    // 0x191d8: str r1, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(5)?, 20)?)?;
    // 0x191dc: add r1, r3, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(3)?, 0x18)?)?;
    // 0x191e0: str r1, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 0)?)?;
    // 0x191e4: str r7, [r3, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(3)?, 20)?)?;
    // 0x191e8: mov lr, r6
    c.emit(arm32::mov_register(C::Al, R::LR, R::new(6)?))?;
    // 0x191ec: cmp r2, #1376
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x560)?)?;
    // 0x191f0: bls 0x19240 <later+0xe60>
    c.b(C::Ls, 0x19240)?;
    // 0x191f4: sub r0, r2, #1376
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::R2, 0x560)?)?;
    // 0x191f8: ldr r1, [pc, #0x494]
    c.literal(R::R1, 0x19694)?;
    // 0x191fc: ldr r2, [pc, #0x494]
    c.literal(R::R2, 0x19698)?;
    // 0x19200: mov r3, #102
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x66)?)?;
    // 0x19204: str r0, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 0)?)?;
    // 0x19208: orr r3, r3, #5632
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1600)?)?;
    // 0x1920c: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x19210: bl 0x2ec94 <later+0x168b4>
    c.bl(0x2ec94)?;
    // 0x19214: ldr r1, [pc, #0x480]
    c.literal(R::R1, 0x1969c)?;
    // 0x19218: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1921c: bl 0x2ec94 <later+0x168b4>
    c.bl(0x2ec94)?;
    // 0x19220: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x19224: mov r0, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2bc)?)?;
    // 0x19228: mov r1, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8)?)?;
    // 0x1922c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x19230: orr r1, r1, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x2000)?)?;
    // 0x19234: str r1, [r9, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R1, R::new(9)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x19238: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 0x1923c: ldr r1, [lr]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::LR, 0)?)?;
    // 0x19240: mov r2, #1232
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x4d0)?)?;
    // 0x19244: mov r6, #34816
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x8800)?)?;
    // 0x19248: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1924c: orr r2, r2, #36864
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x9000)?)?;
    // 0x19250: str r2, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 0)?)?;
    // 0x19254: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19258: ldr r5, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 24)?)?;
    // 0x1925c: cmp r5, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x6)?)?;
    // 0x19260: moveq r2, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x19264: cmp r5, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x6)?)?;
    // 0x19268: lsl r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R2, R::R2, Shift::Lsl, 4)?)?;
    // 0x1926c: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 0)?)?;
    // 0x19270: ldr r2, [r9, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2480)?)?;
    // 0x19274: ldr r7, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2484)?)?;
    // 0x19278: str r6, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 4)?)?;
    // 0x1927c: orr r2, r7, r2, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(7)?, R::R2, Shift::Lsl, 16, false)?)?;
    // 0x19280: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x19284: add r2, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0xc)?)?;
    // 0x19288: ldr r7, [r9, #0x9f0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2544)?)?;
    // 0x1928c: ldr r6, [r9, #0x9f8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(9)?, 2552)?)?;
    // 0x19290: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 0)?)?;
    // 0x19294: mov r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x4)?)?;
    // 0x19298: orr r2, r2, #34816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x8800)?)?;
    // 0x1929c: str r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 8)?)?;
    // 0x192a0: add r2, r1, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R1, 0xc)?)?;
    // 0x192a4: str r2, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::LR, 0)?)?;
    // 0x192a8: mov r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2)?)?;
    // 0x192ac: moveq r2, #3
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R2, R::R0, 0x3)?)?;
    // 0x192b0: orr r6, r2, r6, lsl #10
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(6)?, R::R2, R::new(6)?, Shift::Lsl, 10, false)?)?;
    // 0x192b4: orr r7, r6, r7, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::new(6)?, R::new(7)?, Shift::Lsl, 3, false)?)?;
    // 0x192b8: str r7, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 8)?)?;
    // 0x192bc: bne 0x192dc <later+0xefc>
    c.b(C::Ne, 0x192dc)?;
    // 0x192c0: ldr r3, [r9, #0x9c8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2504)?)?;
    // 0x192c4: ldr r7, [r9, #0x9cc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2508)?)?;
    // 0x192c8: lsl r3, r3, #14
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 14)?)?;
    // 0x192cc: orr r3, r3, r7, lsl #22
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::new(7)?, Shift::Lsl, 22, false)?)?;
    // 0x192d0: mov r7, #2320
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x910)?)?;
    // 0x192d4: orr r7, r7, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8000)?)?;
    // 0x192d8: b 0x1937c <later+0xf9c>
    c.b(C::Al, 0x1937c)?;
    // 0x192dc: ldr r7, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 12)?)?;
    // 0x192e0: ldr r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 8)?)?;
    // 0x192e4: mov r6, #288
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x120)?)?;
    // 0x192e8: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x192ec: ldr r7, [r5, r7, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(7)?, R::new(5)?, R::new(7)?, Shift::Lsl, 2)?)?;
    // 0x192f0: ldr r5, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(5)?, 144)?)?;
    // 0x192f4: ldr r5, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(5)?, 32)?)?;
    // 0x192f8: ldr r6, [r7, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::new(7)?, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x192fc: ldr r4, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(5)?, 20)?)?;
    // 0x19300: ldr r5, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(5)?, 24)?)?;
    // 0x19304: cmp r6, #8
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x8)?)?;
    // 0x19308: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1930c: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(6)?, R::R0, 0x1)?)?;
    // 0x19310: lsl r5, r5, #17
    c.emit(arm32::mov_shifted_register(C::Al, R::new(5)?, R::new(5)?, Shift::Lsl, 17)?)?;
    // 0x19314: orr r5, r5, r4, lsl #30
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(5)?, R::new(5)?, R::new(4)?, Shift::Lsl, 30, false)?)?;
    // 0x19318: orr r6, r5, r6, lsl #15
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(6)?, R::new(5)?, R::new(6)?, Shift::Lsl, 15, false)?)?;
    // 0x1931c: mov r5, #292
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x124)?)?;
    // 0x19320: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1000)?)?;
    // 0x19324: ldr r5, [r7, r5]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(5)?, R::new(7)?, R::new(5)?, Shift::Lsl, 0)?)?;
    // 0x19328: cmp r5, #8
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x8)?)?;
    // 0x1932c: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x19330: orr r3, r6, r3, lsl #13
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(6)?, R::new(3)?, Shift::Lsl, 13, false)?)?;
    // 0x19334: mov r6, #280
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x118)?)?;
    // 0x19338: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x1933c: ldr r6, [r7, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::new(7)?, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x19340: ldr r5, [pc, #0x358]
    c.literal(R::new(5)?, 0x196a0)?;
    // 0x19344: add r6, r5, r6, lsl #7
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::new(5)?, R::new(6)?, Shift::Lsl, 7, false)?)?;
    // 0x19348: orr r3, r3, r6
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 0x1934c: mov r6, #296
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x128)?)?;
    // 0x19350: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x19354: ldr r6, [r7, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::new(7)?, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x19358: orr r3, r3, r6
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 0x1935c: mov r6, #284
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x11c)?)?;
    // 0x19360: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x19364: ldr r7, [r7, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(7)?, R::new(7)?, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x19368: mvn r6, #17
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(6)?, R::R0, 0x11)?)?;
    // 0x1936c: add r7, r6, r7, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(7)?, R::new(6)?, R::new(7)?, Shift::Lsl, 1, false)?)?;
    // 0x19370: orr r3, r3, r7
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0x19374: mov r7, #276
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x114)?)?;
    // 0x19378: orr r7, r7, #34816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8800)?)?;
    // 0x1937c: str r7, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 12)?)?;
    // 0x19380: str r3, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 12)?)?;
    // 0x19384: mov r7, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x4)?)?;
    // 0x19388: ldr r3, [r9, #0x9f0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2544)?)?;
    // 0x1938c: orr r7, r7, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8400)?)?;
    // 0x19390: str r7, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 16)?)?;
    // 0x19394: add r7, r0, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0x18)?)?;
    // 0x19398: orr r2, r2, r3, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(3)?, Shift::Lsl, 3, false)?)?;
    // 0x1939c: str r2, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 16)?)?;
    // 0x193a0: ldr r2, [r9, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2480)?)?;
    // 0x193a4: ldr r3, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2484)?)?;
    // 0x193a8: str r7, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(10)?, 0)?)?;
    // 0x193ac: mov r7, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x8)?)?;
    // 0x193b0: orr r7, r7, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8400)?)?;
    // 0x193b4: str r7, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 20)?)?;
    // 0x193b8: add r7, r1, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R1, 0x18)?)?;
    // 0x193bc: str r7, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::LR, 0)?)?;
    // 0x193c0: orr r2, r3, r2, lsl #16
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(3)?, R::R2, Shift::Lsl, 16, false)?)?;
    // 0x193c4: str r2, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 20)?)?;
    // 0x193c8: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0x193cc: cmp r2, #6
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x6)?)?;
    // 0x193d0: bne 0x194b4 <later+0x10d4>
    c.b(C::Ne, 0x194b4)?;
    // 0x193d4: ldr r3, [r9, #0x9c8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2504)?)?;
    // 0x193d8: ldr r7, [r9, #0x9cc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(9)?, 2508)?)?;
    // 0x193dc: ldr r12, [r9, #0x9f4]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::new(9)?, 2548)?)?;
    // 0x193e0: ldr r2, [r9, #0x9d4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2516)?)?;
    // 0x193e4: ldr r4, [r9, #0x9d0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(9)?, 2512)?)?;
    // 0x193e8: mov r6, #68
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x44)?)?;
    // 0x193ec: add r5, r0, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x1c)?)?;
    c.check(0x193f0)
}

/// Verify and publish codec controller state, then return to caller.
fn emit_controller_finish(c: &mut Code) -> Result<(), String> {
    c.check(0x193f0)?;
    // 0x193f0: orr r6, r6, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x8400)?)?;
    // 0x193f4: str r5, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(10)?, 0)?)?;
    // 0x193f8: str r6, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::R0, 24)?)?;
    // 0x193fc: add r6, r1, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R1, 0x1c)?)?;
    // 0x19400: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::LR, 0)?)?;
    // 0x19404: lsl r3, r3, #14
    c.emit(arm32::mov_shifted_register(C::Al, R::new(3)?, R::new(3)?, Shift::Lsl, 14)?)?;
    // 0x19408: orr r3, r3, r7, lsl #22
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::new(7)?, Shift::Lsl, 22, false)?)?;
    // 0x1940c: orr r3, r3, r12, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(3)?, R::R12, Shift::Lsl, 2, false)?)?;
    // 0x19410: orr r2, r3, r2, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(3)?, R::R2, Shift::Lsl, 1, false)?)?;
    // 0x19414: orr r2, r2, r4
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 0x19418: str r2, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 24)?)?;
    // 0x1941c: ldr r2, [r9, #0x9d0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2512)?)?;
    // 0x19420: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x19424: beq 0x19464 <later+0x1084>
    c.b(C::Eq, 0x19464)?;
    // 0x19428: ldr r2, [r9, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2648)?)?;
    // 0x1942c: mov r7, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x48)?)?;
    // 0x19430: add r5, r0, #36
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x24)?)?;
    // 0x19434: add r6, r1, #36
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R1, 0x24)?)?;
    // 0x19438: orr r7, r7, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8400)?)?;
    // 0x1943c: ldr r3, [r2, #0x2e8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 744)?)?;
    // 0x19440: str r7, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 28)?)?;
    // 0x19444: str r3, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R1, 28)?)?;
    // 0x19448: mov r3, #76
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x4c)?)?;
    // 0x1944c: ldr r2, [r2, #0x2ec]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R2, 748)?)?;
    // 0x19450: orr r3, r3, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8400)?)?;
    // 0x19454: str r5, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::new(10)?, 0)?)?;
    // 0x19458: str r3, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 32)?)?;
    // 0x1945c: str r6, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::LR, 0)?)?;
    // 0x19460: str r2, [r1, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 32)?)?;
    // 0x19464: ldr r1, [r9, #0x9bc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 2492)?)?;
    // 0x19468: ldr r0, [pc, #0x234]
    c.literal(R::R0, 0x196a4)?;
    // 0x1946c: ldr r7, [r0, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(7)?, R::R0, R::R1, Shift::Lsl, 2)?)?;
    // 0x19470: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x19474: beq 0x195d0 <later+0x11f0>
    c.b(C::Eq, 0x195d0)?;
    // 0x19478: ldr r2, [r9, #0x9c0]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 2496)?)?;
    // 0x1947c: ldr r3, [r9, #0x9c4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(9)?, 2500)?)?;
    // 0x19480: ldr r1, [r9, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(9)?, 2484)?)?;
    // 0x19484: ldr r0, [r0, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R2, Shift::Lsl, 2)?)?;
    // 0x19488: orr r2, r7, r0, lsl #11
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(7)?, R::R0, Shift::Lsl, 11, false)?)?;
    // 0x1948c: sub r0, r0, r7
    c.emit(arm32::data_register(C::Al, Op::Sub, R::R0, R::R0, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0x19490: lsl r0, r0, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Mov, R::R0, R::R0, R::R0, Shift::Lsl, R::new(3)?, false)?)?;
    // 0x19494: orr r4, r2, r3, lsl #22
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(4)?, R::R2, R::new(3)?, Shift::Lsl, 22, false)?)?;
    // 0x19498: blx 0x82be4 <later+0x6a804>
    c.emit(arm32::branch_exchange_immediate(c.at() as u32, 0x82be4)?)?;
    // 0x1949c: lsl r1, r0, #25
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 25)?)?;
    // 0x194a0: orr r7, r4, r0, lsl #25
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(7)?, R::new(4)?, R::R0, Shift::Lsl, 25, false)?)?;
    // 0x194a4: cmp r0, r1, asr #25
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Asr, 25, false)?)?;
    // 0x194a8: bne 0x1958c <later+0x11ac>
    c.b(C::Ne, 0x1958c)?;
    // 0x194ac: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x194b0: b 0x195d0 <later+0x11f0>
    c.b(C::Al, 0x195d0)?;
    // 0x194b4: ldr r2, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 12)?)?;
    // 0x194b8: ldr r9, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 8)?)?;
    // 0x194bc: mov r3, #284
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x11c)?)?;
    // 0x194c0: mov r7, #296
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x128)?)?;
    // 0x194c4: mov r6, #280
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x118)?)?;
    // 0x194c8: mov r5, #292
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x124)?)?;
    // 0x194cc: mov r4, #288
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x120)?)?;
    // 0x194d0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x194d4: orr r7, r7, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x1000)?)?;
    // 0x194d8: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x1000)?)?;
    // 0x194dc: orr r5, r5, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x1000)?)?;
    // 0x194e0: orr r4, r4, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(4)?, R::new(4)?, 0x1000)?)?;
    // 0x194e4: ldr r2, [r9, r2, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::new(9)?, R::R2, Shift::Lsl, 2)?)?;
    // 0x194e8: ldr lr, [r2, r7]
    c.emit(arm32::transfer_word_register(C::Al, true, R::LR, R::R2, R::new(7)?, Shift::Lsl, 0)?)?;
    // 0x194ec: ldr r12, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R12, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x194f0: ldr r6, [r2, r6]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(6)?, R::R2, R::new(6)?, Shift::Lsl, 0)?)?;
    // 0x194f4: ldr r5, [r2, r5]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(5)?, R::R2, R::new(5)?, Shift::Lsl, 0)?)?;
    // 0x194f8: ldr r2, [r2, r4]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R2, R::new(4)?, Shift::Lsl, 0)?)?;
    // 0x194fc: ldr r4, [r9, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(9)?, 144)?)?;
    // 0x19500: mov r7, #84
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x54)?)?;
    // 0x19504: orr r7, r7, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8400)?)?;
    // 0x19508: ldr r4, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(4)?, 32)?)?;
    // 0x1950c: cmp r2, #8
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x8)?)?;
    // 0x19510: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0x19514: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x19518: cmp r5, #8
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x8)?)?;
    // 0x1951c: ldr r3, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 20)?)?;
    // 0x19520: ldr r4, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(4)?, 24)?)?;
    // 0x19524: str r7, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 24)?)?;
    // 0x19528: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 0x1952c: lsl r4, r4, #17
    c.emit(arm32::mov_shifted_register(C::Al, R::new(4)?, R::new(4)?, Shift::Lsl, 17)?)?;
    // 0x19530: orr r3, r4, r3, lsl #30
    c.emit(arm32::data_register(C::Al, Op::Orr, R::new(3)?, R::new(4)?, R::new(3)?, Shift::Lsl, 30, false)?)?;
    // 0x19534: orr r2, r3, r2, lsl #15
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::new(3)?, R::R2, Shift::Lsl, 15, false)?)?;
    // 0x19538: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 0x1953c: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 0x19540: orr r2, r2, r3, lsl #13
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(3)?, Shift::Lsl, 13, false)?)?;
    // 0x19544: ldr r3, [pc, #0x154]
    c.literal(R::new(3)?, 0x196a0)?;
    // 0x19548: add r3, r3, r6, lsl #7
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::new(3)?, R::new(6)?, Shift::Lsl, 7, false)?)?;
    // 0x1954c: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x19550: mvn r3, #17
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0x11)?)?;
    // 0x19554: orr r2, r2, lr
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::LR, Shift::Lsl, 0, false)?)?;
    // 0x19558: add r3, r3, r12, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::new(3)?, R::R12, Shift::Lsl, 1, false)?)?;
    // 0x1955c: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x19560: orr r2, r2, r3
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R2, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x19564: mov r3, #88
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x58)?)?;
    // 0x19568: str r2, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 24)?)?;
    // 0x1956c: add r2, r0, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0x20)?)?;
    // 0x19570: orr r3, r3, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8400)?)?;
    // 0x19574: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 0)?)?;
    // 0x19578: str r3, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R0, 28)?)?;
    // 0x1957c: add r0, r1, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x20)?)?;
    // 0x19580: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::LR, 0)?)?;
    // 0x19584: str r7, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 28)?)?;
    // 0x19588: b 0x195f0 <later+0x1210>
    c.b(C::Al, 0x195f0)?;
    // 0x1958c: ldr r1, [pc, #0x114]
    c.literal(R::R1, 0x196a8)?;
    // 0x19590: ldr r2, [pc, #0x100]
    c.literal(R::R2, 0x19698)?;
    // 0x19594: mov r3, #197
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xc5)?)?;
    // 0x19598: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1959c: orr r3, r3, #5632
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1600)?)?;
    // 0x195a0: bl 0x2ec94 <later+0x168b4>
    c.bl(0x2ec94)?;
    // 0x195a4: ldr r1, [pc, #0xf0]
    c.literal(R::R1, 0x1969c)?;
    // 0x195a8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x195ac: bl 0x2ec94 <later+0x168b4>
    c.bl(0x2ec94)?;
    // 0x195b0: ldr r2, [pc, #0xd8]
    c.literal(R::R2, 0x19690)?;
    // 0x195b4: mov r1, #198
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xc6)?)?;
    // 0x195b8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x195bc: orr r1, r1, #5632
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1600)?)?;
    // 0x195c0: bl 0x17b0 <later+0xfffffffffffe93d0>
    c.bl(0x17b0)?;
    // 0x195c4: ldr lr, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 20)?)?;
    // 0x195c8: ldr r5, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(10)?, 0)?)?;
    // 0x195cc: ldr r6, [lr]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::LR, 0)?)?;
    // 0x195d0: mov r0, #1104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x450)?)?;
    // 0x195d4: add r2, r5, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(5)?, 0x4)?)?;
    // 0x195d8: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8000)?)?;
    // 0x195dc: str r2, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 0)?)?;
    // 0x195e0: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 0)?)?;
    // 0x195e4: add r0, r6, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x4)?)?;
    // 0x195e8: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::LR, 0)?)?;
    // 0x195ec: str r7, [r6]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::new(6)?, 0)?)?;
    // 0x195f0: add r1, r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0x4)?)?;
    // 0x195f4: mov r3, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x8400)?)?;
    // 0x195f8: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x195fc: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 0)?)?;
    // 0x19600: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0x19604: add r3, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0x4)?)?;
    // 0x19608: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::LR, 0)?)?;
    // 0x1960c: str r7, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 0)?)?;
    // 0x19610: ldr r6, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 12)?)?;
    // 0x19614: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(6)?, 0x0)?)?;
    // 0x19618: beq 0x1964c <later+0x126c>
    c.b(C::Eq, 0x1964c)?;
    // 0x1961c: mov r1, #1120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x460)?)?;
    // 0x19620: mov r3, #100
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x64)?)?;
    // 0x19624: orr r1, r1, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x8000)?)?;
    // 0x19628: orr r3, r3, #33792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x8400)?)?;
    // 0x1962c: str r1, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R2, 4)?)?;
    // 0x19630: add r1, r2, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0xc)?)?;
    // 0x19634: str r7, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 4)?)?;
    // 0x19638: str r1, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 0)?)?;
    // 0x1963c: str r3, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 8)?)?;
    // 0x19640: add r3, r0, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xc)?)?;
    // 0x19644: str r3, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::LR, 0)?)?;
    // 0x19648: str r7, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R0, 8)?)?;
    // 0x1964c: mov r0, #112
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x70)?)?;
    // 0x19650: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8000)?)?;
    // 0x19654: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 0x19658: mov r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xff)?)?;
    // 0x1965c: orr r0, r0, #65280
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0xff00)?)?;
    // 0x19660: str r2, [r1]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 0)?)?;
    // 0x19664: str r0, [r3]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 0)?)?;
    // 0x19668: add r0, r1, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x8)?)?;
    // 0x1966c: str r0, [r10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 0)?)?;
    // 0x19670: add r0, r3, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(3)?, 0x8)?)?;
    // 0x19674: str r2, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R1, 4)?)?;
    // 0x19678: str r0, [lr]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::LR, 0)?)?;
    // 0x1967c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x19680: str r0, [r3, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(3)?, 4)?)?;
    // 0x19684: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x19688: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 0x1968c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x19690)?;
    for literal in [
        0x000e_3d16_u32, // runtime diagnostic record
        0x000e_42c5, // codec buffer validation diagnostic
        0x000e_42f6, // command buffer validation diagnostic
        0x000e_2788, // diagnostic footer
        0xffff_fb80, // signed -1152 field displacement
        0x000e_44c4, // per-frame codec diagnostic
        0x000e_430a, // codec layout diagnostic
    ] { c.emit(literal)?; }
    c.check(0x196ac)
}

/// One full command-controller routine, whose output is concatenated to
/// the two earlier independent functions in the agent-03 late fragment.
pub(super) fn extend(c: &mut Code) -> Result<(),String> {
    emit_packet_setup(c)?;
    emit_descriptor_fields(c)?;
    emit_record_tables(c)?;
    emit_controller_finish(c)?;
    Ok(())
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn smulbb_matches_arm_architecture_and_rejects_invalid_operands() {
        assert_eq!(signed_multiply_low_halfwords(C::Al,R::R0,R::R0,R::new(6).unwrap()).unwrap(),0xe160_0680);
        assert_eq!(signed_multiply_low_halfwords(C::Eq,R::R1,R::new(2).unwrap(),R::new(3).unwrap()).unwrap(),0x0161_0382);
        assert!(signed_multiply_low_halfwords(C::Al,R::SP,R::R0,R::R0).is_err());
        assert!(signed_multiply_low_halfwords(C::Al,R::R0,R::PC,R::R0).is_err());
    }
}
