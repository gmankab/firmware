// SPDX-License-Identifier: GPL-3.0-only
//! Reconstructed Venus codec state-machine operation: 20-way controller
//! dispatch, codec configuration, buffer planes, records and diagnostics.
//! Typed architecture operands are encoded into A32 instructions. Identified
//! jump tables and literal-address pools are non-executable data. This is a
//! partial disjoint firmware fragment; behavior on hardware is untested.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x1e704;
const END:usize=0x213fc;
/// Identified non-instruction dispatch targets at runtime 0x1e774–0x1e7c4.
const STATE_POOL_00: [u32; 20] = [
    0x0001eda4, // 0x01e774
    0x0001f0d4, // 0x01e778
    0x0001f38c, // 0x01e77c
    0x0001e7c4, // 0x01e780
    0x0001f38c, // 0x01e784
    0x0001f3a8, // 0x01e788
    0x0001f63c, // 0x01e78c
    0x0001f38c, // 0x01e790
    0x0001f9f8, // 0x01e794
    0x0001f38c, // 0x01e798
    0x0001fd58, // 0x01e79c
    0x0001f38c, // 0x01e7a0
    0x00020224, // 0x01e7a4
    0x0001e7c4, // 0x01e7a8
    0x0001f38c, // 0x01e7ac
    0x0001e7c4, // 0x01e7b0
    0x0001f38c, // 0x01e7b4
    0x0001f38c, // 0x01e7b8
    0x0001f38c, // 0x01e7bc
    0x000204dc, // 0x01e7c0
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x1f630–0x1f63c.
const STATE_POOL_01: [u32; 3] = [
    0xfbfeeffd, // 0x01f630
    0x0401100d, // 0x01f634
    0x000e53c3, // 0x01f638
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x1f9f4–0x1f9f8.
const STATE_POOL_02: [u32; 1] = [
    0x04011012, // 0x01f9f4
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x1fd4c–0x1fd58.
const STATE_POOL_03: [u32; 3] = [
    0x000e619c, // 0x01fd4c
    0x000e2788, // 0x01fd50
    0x000e61dc, // 0x01fd54
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x20214–0x20224.
const STATE_POOL_04: [u32; 4] = [
    0xffffefcc, // 0x020214
    0x000e6210, // 0x020218
    0x000e64e7, // 0x02021c
    0x000e6515, // 0x020220
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x204d8–0x204dc.
const STATE_POOL_05: [u32; 1] = [
    0x000e6531, // 0x0204d8
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x2071c–0x20720.
const STATE_POOL_06: [u32; 1] = [
    0x000e6435, // 0x02071c
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x20878–0x2087c.
const STATE_POOL_07: [u32; 1] = [
    0x000e64b0, // 0x020878
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x20ba8–0x20bac.
const STATE_POOL_08: [u32; 1] = [
    0x000e63fe, // 0x020ba8
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x20d18–0x20d1c.
const STATE_POOL_09: [u32; 1] = [
    0x000e6247, // 0x020d18
];
/// Identified non-instruction literal address/diagnostic data at runtime 0x213c4–0x213fc.
const STATE_POOL_10: [u32; 14] = [
    0x000e53c3, // 0x0213c4
    0x000e6568, // 0x0213c8
    0x000e2788, // 0x0213cc
    0x000e6593, // 0x0213d0
    0x000e65b5, // 0x0213d4
    0x000e65d0, // 0x0213d8
    0x000e65f2, // 0x0213dc
    0x000e627e, // 0x0213e0
    0x000e63d0, // 0x0213e4
    0x000e630b, // 0x0213e8
    0x000e62e9, // 0x0213ec
    0x000e629d, // 0x0213f0
    0x000e6130, // 0x0213f4
    0x000e6165, // 0x0213f8
];
fn emit_codec_state_machine(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 01e704: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 01e708: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 01e70c: sub sp, sp, #228
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0xe4)?)?;
    // 01e710: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 01e714: sub r0, r11, #104
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::R11, 0x68)?)?;
    // 01e718: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 01e71c: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 01e720: ldr r4, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(5)?, 144)?)?;
    // 01e724: ldr r10, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::new(4)?, 24)?)?;
    // 01e728: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 01e72c: ldr r1, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 12)?)?;
    // 01e730: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 8)?)?;
    // 01e734: ldr r1, [r5, r1, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(5)?, R::R1, Shift::Lsl, 2)?)?;
    // 01e738: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 64)?)?;
    // 01e73c: ldr r1, [pc, #0xeec]
    c.literal(R::R1, 0x1f630)?;
    // 01e740: str r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 56)?)?;
    // 01e744: add r0, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 01e748: ldr r1, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 128)?)?;
    // 01e74c: cmp r0, #19
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x13)?)?;
    // 01e750: ldr r1, [r1, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 128)?)?;
    // 01e754: str r1, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 24)?)?;
    // 01e758: ldr r1, [pc, #0xed4]
    c.literal(R::R1, 0x1f634)?;
    // 01e75c: str r1, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 36)?)?;
    // 01e760: bhi 0x1f38c <state+0xc88>
    c.b(C::Hi, 0x1f38c)?;
    // 01e764: add r1, pc, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x8)?)?;
    // 01e768: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 01e76c: str r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::SP, 76)?)?;
    // 01e770: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;
    c.check(0x1e774)?;
    // Literal address / jump-table words at 0x1e774–0x1e7c4 (not ARM instructions).
    for value in STATE_POOL_00 { c.emit(value)?; }
    c.check(0x1e7c4)?;
    // 01e7c4: ldr r4, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(4)?, 32)?)?;
    // 01e7c8: add r0, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x50)?)?;
    // 01e7cc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 01e7d0: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 01e7d4: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 01e7d8: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 01e7dc: bl 0x26754 <state+0x8050>
    c.bl(0x26754)?;
    // 01e7e0: mov r1, #3744
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xea0)?)?;
    // 01e7e4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01e7e8: str r0, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 01e7ec: bl 0x26754 <state+0x8050>
    c.bl(0x26754)?;
    // 01e7f0: mov r1, #676
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2a4)?)?;
    // 01e7f4: ldr r2, [pc, #0xe3c]
    c.literal(R::R2, 0x1f638)?;
    // 01e7f8: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 01e7fc: str r0, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 01e800: str r4, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 32)?)?;
    // 01e804: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 12)?)?;
    // 01e808: str r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 80)?)?;
    // 01e80c: mov r0, #624
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x270)?)?;
    // 01e810: ldr r8, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::SP, 64)?)?;
    // 01e814: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01e818: mov r5, r8
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(8)?))?;
    // 01e81c: mov r4, r8
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(8)?))?;
    // 01e820: mov r7, r8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(8)?))?;
    // 01e824: ldr r0, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01e828: str r0, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 84)?)?;
    // 01e82c: mov r0, #644
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x284)?)?;
    // 01e830: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01e834: ldr r0, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 01e838: str r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 88)?)?;
    // 01e83c: mov r0, #648
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x288)?)?;
    // 01e840: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01e844: ldr r0, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0)?)?;
    // 01e848: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 92)?)?;
    // 01e84c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01e850: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 01e854: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 01e858: mov r1, #51
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x33)?)?;
    // 01e85c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01e860: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01e864: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01e868: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01e86c: ldr r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 80)?)?;
    // 01e870: add r9, r8, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(8)?, 0x4)?)?;
    // 01e874: ldr r2, [pc, #0xdbc]
    c.literal(R::R2, 0x1f638)?;
    // 01e878: mov r1, #53
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x35)?)?;
    // 01e87c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01e880: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01e884: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01e888: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01e88c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01e890: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 108)?)?;
    // 01e894: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01e898: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01e89c: ldr r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 80)?)?;
    // 01e8a0: add r3, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::SP, 0x50)?)?;
    // 01e8a4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01e8a8: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01e8ac: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01e8b0: str r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 116)?)?;
    // 01e8b4: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 0)?)?;
    // 01e8b8: ldr r1, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(7)?, 0)?)?;
    // 01e8bc: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 01e8c0: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01e8c4: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 01e8c8: add r0, r8, #628
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0x274)?)?;
    // 01e8cc: add r1, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x1000)?)?;
    // 01e8d0: add r0, r10, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x94)?)?;
    // 01e8d4: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01e8d8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 01e8dc: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01e8e0: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01e8e4: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01e8e8: add r1, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x50)?)?;
    // 01e8ec: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01e8f0: ldr r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 116)?)?;
    // 01e8f4: ldr r2, [pc, #0xd3c]
    c.literal(R::R2, 0x1f638)?;
    // 01e8f8: mov r1, #59
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3b)?)?;
    // 01e8fc: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01e900: cmp r0, r4
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 01e904: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01e908: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01e90c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01e910: ldr r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 88)?)?;
    // 01e914: mov r7, r8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(8)?))?;
    // 01e918: mov r3, r9
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(9)?))?;
    // 01e91c: mov r8, r10
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::new(10)?))?;
    // 01e920: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 01e924: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 01e928: str r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 20)?)?;
    // 01e92c: ldr r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 92)?)?;
    // 01e930: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 01e934: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 01e938: str r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 24)?)?;
    // 01e93c: mov r0, #668
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x29c)?)?;
    // 01e940: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01e944: ldr r0, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0)?)?;
    // 01e948: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 01e94c: ldr r1, [r4, #0x4]!
    c.emit(arm32::transfer_word_preindexed_writeback(C::Al, true, R::R1, R::new(4)?, 4)?)?;
    // 01e950: str r3, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::SP, 48)?)?;
    // 01e954: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al, R::R2, R::R1, R::R0)?)?;
    // 01e958: cmn r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmn, R::R0, R::R2, 0x1)?)?;
    // 01e95c: beq 0x1e9d4 <state+0x2d0>
    c.b(C::Eq, 0x1e9d4)?;
    // 01e960: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 32)?)?;
    // 01e964: ldr r9, [pc, #0xccc]
    c.literal(R::new(9)?, 0x1f638)?;
    // 01e968: add r10, r0, #36
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::R0, 0x24)?)?;
    // 01e96c: ldr r0, [r10, r6, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::new(6)?, Shift::Lsl, 2)?)?;
    // 01e970: add r5, r8, r6, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::new(8)?, R::new(6)?, Shift::Lsl, 2, false)?)?;
    // 01e974: mov r1, #74
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4a)?)?;
    // 01e978: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 01e97c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01e980: str r0, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 128)?)?;
    // 01e984: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01e988: add r0, r3, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(3)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01e98c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01e990: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01e994: str r0, [r5, #0xf8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 248)?)?;
    // 01e998: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01e99c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01e9a0: ldr r0, [r5, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 128)?)?;
    // 01e9a4: ldr r3, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 48)?)?;
    // 01e9a8: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::new(6)?, 0x1)?)?;
    // 01e9ac: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01e9b0: add r0, r3, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(3)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01e9b4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01e9b8: str r0, [r5, #0x260]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 608)?)?;
    // 01e9bc: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 0)?)?;
    // 01e9c0: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 0)?)?;
    // 01e9c4: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al, R::R2, R::R1, R::R0)?)?;
    // 01e9c8: add r0, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x1)?)?;
    // 01e9cc: cmp r6, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(6)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 01e9d0: blo 0x1e96c <state+0x268>
    c.b(C::Cc, 0x1e96c)?;
    // 01e9d4: ldr r1, [r8, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 56)?)?;
    // 01e9d8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01e9dc: mov r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x0)?)?;
    // 01e9e0: mov r10, r8
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(8)?))?;
    // 01e9e4: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 72)?)?;
    // 01e9e8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 01e9ec: beq 0x1eb4c <state+0x448>
    c.b(C::Eq, 0x1eb4c)?;
    // 01e9f0: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 64)?)?;
    // 01e9f4: add r0, r10, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x48)?)?;
    // 01e9f8: mov r12, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x0)?)?;
    // 01e9fc: add r6, r0, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0x400)?)?;
    // 01ea00: add r0, r2, #136
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x88)?)?;
    // 01ea04: add r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1400)?)?;
    // 01ea08: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 68)?)?;
    // 01ea0c: add r0, r2, #956
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x3bc)?)?;
    // 01ea10: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 01ea14: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 01ea18: add r0, r2, #916
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x394)?)?;
    // 01ea1c: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 01ea20: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 40)?)?;
    // 01ea24: add r0, r2, #132
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x84)?)?;
    // 01ea28: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 01ea2c: add r5, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x1400)?)?;
    // 01ea30: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 32)?)?;
    // 01ea34: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x10)?)?;
    // 01ea38: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 60)?)?;
    // 01ea3c: ldr r0, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 60)?)?;
    // 01ea40: str r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 72)?)?;
    // 01ea44: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 01ea48: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 01ea4c: beq 0x1eb1c <state+0x418>
    c.b(C::Eq, 0x1eb1c)?;
    // 01ea50: ldr r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 44)?)?;
    // 01ea54: mov r4, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x0)?)?;
    // 01ea58: str r12, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R12, R::SP, 52)?)?;
    // 01ea5c: add r8, r1, r12, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(8)?, R::R1, R::R12, Shift::Lsl, 2, false)?)?;
    // 01ea60: ldr r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 40)?)?;
    // 01ea64: add r9, r1, r12, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(9)?, R::R1, R::R12, Shift::Lsl, 2, false)?)?;
    // 01ea68: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x0)?)?;
    // 01ea6c: bne 0x1eafc <state+0x3f8>
    c.b(C::Ne, 0x1eafc)?;
    // 01ea70: ldr r0, [r9, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(9)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 01ea74: ldr r1, [r8, r4, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(8)?, R::new(4)?, Shift::Lsl, 2)?)?;
    // 01ea78: mvn r2, #-2147483641
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R2, R::R0, 0x80000007)?)?;
    // 01ea7c: add r7, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::SP, 0x50)?)?;
    // 01ea80: mov r3, r7
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 01ea84: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 01ea88: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xf)?)?;
    // 01ea8c: and r0, r2, r0, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R2, R::R0, Shift::Lsr, 1, false)?)?;
    // 01ea90: and r1, r2, r1, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R1, R::R2, R::R1, Shift::Lsr, 1, false)?)?;
    // 01ea94: ldr r2, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 0)?)?;
    // 01ea98: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01ea9c: mov r10, r0
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R0))?;
    // 01eaa0: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 01eaa4: ldr r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 68)?)?;
    // 01eaa8: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 01eaac: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01eab0: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 01eab4: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 01eab8: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01eabc: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 01eac0: ldr r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 60)?)?;
    // 01eac4: ldr r2, [pc, #0xb6c]
    c.literal(R::R2, 0x1f638)?;
    // 01eac8: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 01eacc: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 64)?)?;
    // 01ead0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ead4: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ead8: mov r1, #97
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x61)?)?;
    // 01eadc: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 01eae0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01eae4: cmp r0, r10
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(10)?, Shift::Lsl, 0, false)?)?;
    // 01eae8: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 01eaec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01eaf0: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01eaf4: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01eaf8: ldr r0, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 60)?)?;
    // 01eafc: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x1)?)?;
    // 01eb00: cmp r4, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 01eb04: blo 0x1ea68 <state+0x364>
    c.b(C::Cc, 0x1ea68)?;
    // 01eb08: ldr r12, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R12, R::SP, 52)?)?;
    // 01eb0c: ldr r1, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 56)?)?;
    // 01eb10: mov r2, r0
    c.emit(arm32::mov_register(C::Al, R::R2, R::R0))?;
    // 01eb14: add r12, r12, r4
    c.emit(arm32::data_register(C::Al, Op::Add, R::R12, R::R12, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 01eb18: b 0x1eb20 <state+0x41c>
    c.b(C::Al, 0x1eb20)?;
    // 01eb1c: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 01eb20: ldr r3, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 68)?)?;
    // 01eb24: add r6, r6, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::new(6)?, 0x48)?)?;
    // 01eb28: add r3, r3, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x10)?)?;
    // 01eb2c: str r3, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::SP, 68)?)?;
    // 01eb30: ldr r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 72)?)?;
    // 01eb34: mov r4, r3
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(3)?))?;
    // 01eb38: add r4, r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x1)?)?;
    // 01eb3c: mov r3, r4
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(4)?))?;
    // 01eb40: cmp r4, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(4)?, R::R1, Shift::Lsl, 0, false)?)?;
    // 01eb44: str r3, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::SP, 72)?)?;
    // 01eb48: blo 0x1ea48 <state+0x344>
    c.b(C::Cc, 0x1ea48)?;
    // 01eb4c: ldr r5, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 64)?)?;
    // 01eb50: mov r0, #132
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x84)?)?;
    // 01eb54: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 01eb58: mov r7, r5
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(5)?))?;
    // 01eb5c: ldr r2, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(7)?, R::R0, Shift::Lsl, 0)?)?;
    // 01eb60: add r0, r5, #612
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(5)?, 0x264)?)?;
    // 01eb64: add r4, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R0, 0x1000)?)?;
    // 01eb68: add r0, r4, r12, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::R12, Shift::Lsl, 2, false)?)?;
    // 01eb6c: ldr r3, [r0, #0x158]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R0, 344)?)?;
    // 01eb70: ldr r1, [r0, #0x130]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 304)?)?;
    // 01eb74: add r0, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0xf)?)?;
    // 01eb78: mvn r1, #-2147483641
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0x80000007)?)?;
    // 01eb7c: add r3, r3, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0xf)?)?;
    // 01eb80: and r0, r1, r0, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R0, R::R1, R::R0, Shift::Lsr, 1, false)?)?;
    // 01eb84: and r1, r1, r3, lsr #1
    c.emit(arm32::data_register(C::Al, Op::And, R::R1, R::R1, R::new(3)?, Shift::Lsr, 1, false)?)?;
    // 01eb88: add r3, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::SP, 0x50)?)?;
    // 01eb8c: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01eb90: ldr r8, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::SP, 72)?)?;
    // 01eb94: mov r9, r0
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R0))?;
    // 01eb98: add r0, r8, r8, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::new(8)?, Shift::Lsl, 3, false)?)?;
    // 01eb9c: add r1, r4, r8, lsl #4
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(4)?, R::new(8)?, Shift::Lsl, 4, false)?)?;
    // 01eba0: add r0, r10, r0, lsl #3
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 3, false)?)?;
    // 01eba4: add r1, r1, #548
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x224)?)?;
    // 01eba8: add r0, r0, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x48)?)?;
    // 01ebac: add r6, r0, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0x400)?)?;
    // 01ebb0: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 0)?)?;
    // 01ebb4: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 01ebb8: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01ebbc: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 01ebc0: add r1, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x50)?)?;
    // 01ebc4: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01ebc8: ldr r6, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 32)?)?;
    // 01ebcc: ldr r2, [pc, #0xa64]
    c.literal(R::R2, 0x1f638)?;
    // 01ebd0: mov r1, #876
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x36c)?)?;
    // 01ebd4: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 01ebd8: add r0, r6, r8, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::new(8)?, Shift::Lsl, 2, false)?)?;
    // 01ebdc: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 01ebe0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ebe4: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ebe8: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 01ebec: cmp r0, r9
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(9)?, Shift::Lsl, 0, false)?)?;
    // 01ebf0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01ebf4: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ebf8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ebfc: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 8)?)?;
    // 01ec00: ldr r1, [pc, #0xdec]
    c.literal(R::R1, 0x1f9f4)?;
    // 01ec04: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 01ec08: orr r0, r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x2)?)?;
    // 01ec0c: cmp r0, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 01ec10: bne 0x1eca4 <state+0x5a0>
    c.b(C::Ne, 0x1eca4)?;
    // 01ec14: ldr r0, [r6, #0x60]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 96)?)?;
    // 01ec18: ldr r2, [pc, #0xa18]
    c.literal(R::R2, 0x1f638)?;
    // 01ec1c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ec20: add r1, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ec24: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 8)?)?;
    // 01ec28: str r0, [r10, #0x2dc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 732)?)?;
    // 01ec2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ec30: ldr r1, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 12)?)?;
    // 01ec34: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ec38: str r1, [r10, #0x2e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 736)?)?;
    // 01ec3c: mov r1, #118
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x76)?)?;
    // 01ec40: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01ec44: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ec48: ldr r0, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 36)?)?;
    // 01ec4c: ldr r1, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 8)?)?;
    // 01ec50: add r0, r0, #5
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x5)?)?;
    // 01ec54: cmp r1, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 01ec58: bne 0x1ec88 <state+0x584>
    c.b(C::Ne, 0x1ec88)?;
    // 01ec5c: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 100)?)?;
    // 01ec60: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 48)?)?;
    // 01ec64: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ec68: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ec6c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01ec70: str r0, [r10, #0x2e4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 740)?)?;
    // 01ec74: ldr r0, [r6, #0x64]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 100)?)?;
    // 01ec78: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ec7c: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ec80: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01ec84: str r0, [r10, #0x2f8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 760)?)?;
    // 01ec88: ldr r0, [r6, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 80)?)?;
    // 01ec8c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ec90: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ec94: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 01ec98: str r1, [r10, #0x3e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 992)?)?;
    // 01ec9c: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 01eca0: str r0, [r10, #0x3d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 984)?)?;
    // 01eca4: mov r0, #660
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x294)?)?;
    // 01eca8: ldr r2, [pc, #0x988]
    c.literal(R::R2, 0x1f638)?;
    // 01ecac: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01ecb0: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ecb4: str r0, [r10, #0x5d0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1488)?)?;
    // 01ecb8: ldr r0, [r6, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 84)?)?;
    // 01ecbc: str r0, [r10, #0x5d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1492)?)?;
    // 01ecc0: ldr r0, [r6, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 88)?)?;
    // 01ecc4: str r0, [r10, #0x5d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1496)?)?;
    // 01ecc8: ldr r0, [r6, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 76)?)?;
    // 01eccc: str r0, [r10, #0x5dc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1500)?)?;
    // 01ecd0: mov r0, #216
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xd8)?)?;
    // 01ecd4: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 01ecd8: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ecdc: str r0, [r10, #0x5e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1504)?)?;
    // 01ece0: mov r0, #220
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xdc)?)?;
    // 01ece4: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 01ece8: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ecec: str r0, [r10, #0x5e4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1508)?)?;
    // 01ecf0: mov r0, #652
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x28c)?)?;
    // 01ecf4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01ecf8: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ecfc: str r0, [r10, #0x5e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1512)?)?;
    // 01ed00: mov r0, #656
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x290)?)?;
    // 01ed04: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01ed08: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ed0c: str r0, [r10, #0x5ec]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1516)?)?;
    // 01ed10: mov r0, #664
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x298)?)?;
    // 01ed14: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01ed18: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ed1c: str r0, [r10, #0x5f0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1520)?)?;
    // 01ed20: mov r0, #1248
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4e0)?)?;
    // 01ed24: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01ed28: ldr r0, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ed2c: str r0, [r10, #0x5f4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1524)?)?;
    // 01ed30: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01ed34: ldr r1, [r10, #0xcb0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 3248)?)?;
    // 01ed38: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 01ed3c: mov r1, #920
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x398)?)?;
    // 01ed40: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ed44: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 01ed48: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ed4c: ldr r2, [r6, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 8)?)?;
    // 01ed50: ldr r1, [pc, #0xff4]
    c.literal(R::R1, 0x1fd4c)?;
    // 01ed54: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01ed58: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01ed5c: ldr r1, [pc, #0xfec]
    c.literal(R::R1, 0x1fd50)?;
    // 01ed60: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01ed64: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01ed68: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01ed6c: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01ed70: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01ed74: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01ed78: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01ed7c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ed80: bne 0x1ed94 <state+0x690>
    c.b(C::Ne, 0x1ed94)?;
    // 01ed84: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 01ed88: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 01ed8c: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01ed90: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 01ed94: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01ed98: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01ed9c: bl 0x19bf4 <state+0xffffffffffffb4f0>
    c.bl(0x19bf4)?;
    // 01eda0: b 0x20a64 <state+0x2360>
    c.b(C::Al, 0x20a64)?;
    // 01eda4: ldr r4, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(4)?, 32)?)?;
    // 01eda8: add r0, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x50)?)?;
    // 01edac: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 01edb0: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 01edb4: mov r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x0)?)?;
    // 01edb8: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 01edbc: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01edc0: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 01edc4: ldr r2, [pc, #0x86c]
    c.literal(R::R2, 0x1f638)?;
    // 01edc8: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 01edcc: mov r1, #171
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xab)?)?;
    // 01edd0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01edd4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01edd8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01eddc: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ede0: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 12)?)?;
    // 01ede4: str r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 80)?)?;
    // 01ede8: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 36)?)?;
    // 01edec: str r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 88)?)?;
    // 01edf0: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 40)?)?;
    // 01edf4: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 92)?)?;
    // 01edf8: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 48)?)?;
    // 01edfc: str r0, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 128)?)?;
    // 01ee00: ldr r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 64)?)?;
    // 01ee04: str r0, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 368)?)?;
    // 01ee08: ldr r0, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 72)?)?;
    // 01ee0c: str r0, [r10, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 488)?)?;
    // 01ee10: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 44)?)?;
    // 01ee14: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ee18: str r0, [r10, #0x7c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 124)?)?;
    // 01ee1c: beq 0x1ee38 <state+0x734>
    c.b(C::Eq, 0x1ee38)?;
    // 01ee20: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 52)?)?;
    // 01ee24: str r0, [r10, #0x84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 132)?)?;
    // 01ee28: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 68)?)?;
    // 01ee2c: str r0, [r10, #0x174]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 372)?)?;
    // 01ee30: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 01ee34: str r0, [r10, #0x1ec]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 492)?)?;
    // 01ee38: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01ee3c: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 01ee40: ldr r10, [pc, #0x7f0]
    c.literal(R::new(10)?, 0x1f638)?;
    // 01ee44: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 01ee48: mov r1, #187
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xbb)?)?;
    // 01ee4c: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 01ee50: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01ee54: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ee58: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ee5c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ee60: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 64)?)?;
    // 01ee64: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 01ee68: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 01ee6c: add r6, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0x4)?)?;
    // 01ee70: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01ee74: ldr r0, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 80)?)?;
    // 01ee78: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ee7c: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ee80: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01ee84: str r0, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 108)?)?;
    // 01ee88: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ee8c: mov r1, #189
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xbd)?)?;
    // 01ee90: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ee94: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01ee98: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ee9c: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01eea0: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 01eea4: add r9, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::SP, 0x50)?)?;
    // 01eea8: mov r3, r9
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(9)?))?;
    // 01eeac: ldr r0, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 80)?)?;
    // 01eeb0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01eeb4: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01eeb8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01eebc: str r0, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 116)?)?;
    // 01eec0: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 16)?)?;
    // 01eec4: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 36)?)?;
    // 01eec8: ldr r1, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 40)?)?;
    // 01eecc: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01eed0: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 01eed4: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01eed8: add r1, r4, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x14)?)?;
    // 01eedc: add r0, r0, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x94)?)?;
    // 01eee0: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01eee4: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 16)?)?;
    // 01eee8: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01eeec: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01eef0: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01eef4: mov r1, r9
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 01eef8: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01eefc: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01ef00: mov r1, #964
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3c4)?)?;
    // 01ef04: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 01ef08: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x400)?)?;
    // 01ef0c: ldr r0, [r0, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 116)?)?;
    // 01ef10: cmp r0, r5
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 01ef14: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01ef18: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ef1c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ef20: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01ef24: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 01ef28: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 01ef2c: ldr r0, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 128)?)?;
    // 01ef30: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ef34: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ef38: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01ef3c: str r0, [r1, #0xf8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 248)?)?;
    // 01ef40: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ef44: mov r1, #202
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xca)?)?;
    // 01ef48: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ef4c: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01ef50: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ef54: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01ef58: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 01ef5c: mov r3, r9
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(9)?))?;
    // 01ef60: ldr r0, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 128)?)?;
    // 01ef64: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ef68: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ef6c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01ef70: str r0, [r1, #0x260]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 608)?)?;
    // 01ef74: ldr r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 56)?)?;
    // 01ef78: ldr r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 64)?)?;
    // 01ef7c: ldr r1, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 72)?)?;
    // 01ef80: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01ef84: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 01ef88: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01ef8c: add r1, r4, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x50)?)?;
    // 01ef90: add r0, r0, #292
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x124)?)?;
    // 01ef94: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01ef98: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 56)?)?;
    // 01ef9c: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01efa0: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01efa4: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01efa8: mov r1, r9
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 01efac: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01efb0: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01efb4: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 01efb8: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 01efbc: mov r1, #211
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xd3)?)?;
    // 01efc0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01efc4: ldr r0, [r0, #0x260]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 608)?)?;
    // 01efc8: cmp r0, r5
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 01efcc: movhs r8, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::new(8)?, R::R0, 0x1)?)?;
    // 01efd0: mov r0, r8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 01efd4: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01efd8: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 44)?)?;
    // 01efdc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01efe0: beq 0x1f088 <state+0x984>
    c.b(C::Eq, 0x1f088)?;
    // 01efe4: ldr r0, [r10, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 132)?)?;
    // 01efe8: ldr r8, [pc, #0x648]
    c.literal(R::new(8)?, 0x1f638)?;
    // 01efec: mov r1, #218
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xda)?)?;
    // 01eff0: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01eff4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01eff8: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 01effc: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f000: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f004: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f008: str r0, [r10, #0xfc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 252)?)?;
    // 01f00c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f010: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f014: ldr r0, [r10, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 132)?)?;
    // 01f018: add r5, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x50)?)?;
    // 01f01c: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 01f020: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f024: add r0, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f028: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f02c: str r0, [r10, #0x264]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 612)?)?;
    // 01f030: ldr r2, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 60)?)?;
    // 01f034: ldr r0, [r4, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 68)?)?;
    // 01f038: ldr r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 76)?)?;
    // 01f03c: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f040: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 01f044: add r0, r10, #364
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x16c)?)?;
    // 01f048: add r1, r4, #96
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x60)?)?;
    // 01f04c: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01f050: ldr r0, [r4, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 60)?)?;
    // 01f054: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01f058: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01f05c: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01f060: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 01f064: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01f068: ldr r1, [r10, #0x264]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 612)?)?;
    // 01f06c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f070: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 01f074: cmp r1, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 01f078: mov r1, #227
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe3)?)?;
    // 01f07c: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f080: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 01f084: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f088: ldr r2, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 8)?)?;
    // 01f08c: ldr r1, [pc, #0xcc0]
    c.literal(R::R1, 0x1fd54)?;
    // 01f090: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f094: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f098: ldr r1, [pc, #0xcb0]
    c.literal(R::R1, 0x1fd50)?;
    // 01f09c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f0a0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f0a4: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f0a8: ldr r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 56)?)?;
    // 01f0ac: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01f0b0: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f0b4: ldr r0, [r5, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f0b8: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01f0bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f0c0: beq 0x209b0 <state+0x22ac>
    c.b(C::Eq, 0x209b0)?;
    // 01f0c4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 01f0c8: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01f0cc: bl 0x1e5c8 <state+0xfffffffffffffec4>
    c.bl(0x1e5c8)?;
    // 01f0d0: b 0x209f4 <state+0x22f0>
    c.b(C::Al, 0x209f4)?;
    // 01f0d4: ldr r8, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(4)?, 32)?)?;
    // 01f0d8: add r0, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x50)?)?;
    // 01f0dc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 01f0e0: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 01f0e4: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 01f0e8: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f0ec: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 01f0f0: ldr r5, [pc, #0x540]
    c.literal(R::new(5)?, 0x1f638)?;
    // 01f0f4: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 01f0f8: mov r1, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8)?)?;
    // 01f0fc: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01f100: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f104: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f108: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f10c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f110: ldr r0, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 12)?)?;
    // 01f114: mov r1, #20
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x14)?)?;
    // 01f118: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01f11c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f120: str r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 80)?)?;
    // 01f124: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f128: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 64)?)?;
    // 01f12c: mov r4, r9
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(9)?))?;
    // 01f130: mov r6, r9
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(9)?))?;
    // 01f134: add r7, r9, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(9)?, 0x4)?)?;
    // 01f138: ldr r1, [r4, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(4)?, R::R1, Shift::Lsl, 0)?)?;
    // 01f13c: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f140: str r1, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 88)?)?;
    // 01f144: mov r1, #24
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x18)?)?;
    // 01f148: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f14c: ldr r1, [r6, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(6)?, R::R1, Shift::Lsl, 0)?)?;
    // 01f150: str r1, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 92)?)?;
    // 01f154: mov r1, #44
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2c)?)?;
    // 01f158: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f15c: ldr r1, [r9, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(9)?, R::R1, Shift::Lsl, 0)?)?;
    // 01f160: str r1, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 368)?)?;
    // 01f164: mov r1, #48
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x30)?)?;
    // 01f168: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f16c: ldr r1, [r9, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(9)?, R::R1, Shift::Lsl, 0)?)?;
    // 01f170: str r1, [r10, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 488)?)?;
    // 01f174: mov r1, #104
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x68)?)?;
    // 01f178: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f17c: ldr r1, [r9, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(9)?, R::R1, Shift::Lsl, 0)?)?;
    // 01f180: str r1, [r10, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 728)?)?;
    // 01f184: mov r1, #21
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x15)?)?;
    // 01f188: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f18c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f190: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 108)?)?;
    // 01f194: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f198: ldr r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 80)?)?;
    // 01f19c: add r5, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x50)?)?;
    // 01f1a0: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 01f1a4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f1a8: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f1ac: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f1b0: str r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 116)?)?;
    // 01f1b4: mov r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x10)?)?;
    // 01f1b8: mov r10, r9
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(9)?))?;
    // 01f1bc: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01f1c0: ldr r2, [r10, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(10)?, R::R0, Shift::Lsl, 0)?)?;
    // 01f1c4: str r6, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 68)?)?;
    // 01f1c8: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 0)?)?;
    // 01f1cc: str r4, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 72)?)?;
    // 01f1d0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 01f1d4: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f1d8: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 01f1dc: add r0, r9, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x1c)?)?;
    // 01f1e0: add r1, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x1000)?)?;
    // 01f1e4: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01f1e8: add r0, r0, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x94)?)?;
    // 01f1ec: add r4, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R0, 0xc00)?)?;
    // 01f1f0: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 01f1f4: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 01f1f8: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f1fc: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01f200: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01f204: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 01f208: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01f20c: ldr r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 116)?)?;
    // 01f210: ldr r2, [pc, #0x420]
    c.literal(R::R2, 0x1f638)?;
    // 01f214: mov r1, #27
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1b)?)?;
    // 01f218: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f21c: cmp r0, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 01f220: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f224: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f228: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f22c: ldr r0, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 20)?)?;
    // 01f230: str r0, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 128)?)?;
    // 01f234: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f238: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f23c: mov r7, r9
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(9)?))?;
    // 01f240: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 4)?)?;
    // 01f244: str r1, [r10, #0xf8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 248)?)?;
    // 01f248: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f24c: str r0, [r10, #0x260]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 608)?)?;
    // 01f250: mov r0, #52
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x34)?)?;
    // 01f254: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01f258: ldr r0, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0)?)?;
    // 01f25c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f260: beq 0x1f320 <state+0xc1c>
    c.b(C::Eq, 0x1f320)?;
    // 01f264: ldr r0, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 16)?)?;
    // 01f268: mov r1, #56
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x38)?)?;
    // 01f26c: mov r4, r9
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(9)?))?;
    // 01f270: ldr r9, [pc, #0x3c0]
    c.literal(R::new(9)?, 0x1f638)?;
    // 01f274: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f278: str r0, [r10, #0x3f8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1016)?)?;
    // 01f27c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f280: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 01f284: ldr r1, [r4, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(4)?, R::R1, Shift::Lsl, 0)?)?;
    // 01f288: str r1, [r10, #0x3f4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 1012)?)?;
    // 01f28c: ldr r1, [pc, #0xf80]
    c.literal(R::R1, 0x20214)?;
    // 01f290: add r5, r4, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::new(4)?, R::R1, Shift::Lsl, 0, false)?)?;
    // 01f294: mov r1, #40
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x28)?)?;
    // 01f298: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f29c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f2a0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f2a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f2a8: str r0, [r10, #0x3e4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 996)?)?;
    // 01f2ac: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f2b0: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f2b4: ldr r0, [r10, #0x3f8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 1016)?)?;
    // 01f2b8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f2bc: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f2c0: add r5, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x50)?)?;
    // 01f2c4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f2c8: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 01f2cc: str r0, [r10, #0x3fc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1020)?)?;
    // 01f2d0: ldr r2, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 0)?)?;
    // 01f2d4: ldmib r4, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, true, R::new(4)?, R::R0.bit() | R::R1.bit(), BlockMode::IncrementBefore, false)?)?;
    // 01f2d8: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f2dc: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 01f2e0: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 01f2e4: add r1, r4, #12
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0xc)?)?;
    // 01f2e8: add r4, r10, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(10)?, 0x400)?)?;
    // 01f2ec: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f2f0: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01f2f4: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01f2f8: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 01f2fc: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01f300: ldr r0, [r10, #0x3fc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 1020)?)?;
    // 01f304: mov r1, #45
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2d)?)?;
    // 01f308: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 01f30c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f310: cmp r0, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 01f314: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f318: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f31c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f320: ldr r0, [r10, #0xcb0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 3248)?)?;
    // 01f324: ldr r2, [pc, #0x30c]
    c.literal(R::R2, 0x1f638)?;
    // 01f328: mov r1, #50
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x32)?)?;
    // 01f32c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f330: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f334: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f338: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f33c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f340: ldr r2, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(8)?, 8)?)?;
    // 01f344: ldr r1, [pc, #0xecc]
    c.literal(R::R1, 0x20218)?;
    // 01f348: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f34c: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f350: ldr r1, [pc, #0x9f8]
    c.literal(R::R1, 0x1fd50)?;
    // 01f354: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f358: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f35c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f360: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 01f364: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01f368: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f36c: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f370: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01f374: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f378: beq 0x20a08 <state+0x2304>
    c.b(C::Eq, 0x20a08)?;
    // 01f37c: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01f380: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01f384: bl 0x1e5c8 <state+0xfffffffffffffec4>
    c.bl(0x1e5c8)?;
    // 01f388: b 0x20a58 <state+0x2354>
    c.b(C::Al, 0x20a58)?;
    // 01f38c: ldr r2, [pc, #0x2a4]
    c.literal(R::R2, 0x1f638)?;
    // 01f390: mov r1, #91
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5b)?)?;
    // 01f394: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f398: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 01f39c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f3a0: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 01f3a4: b 0x20a70 <state+0x236c>
    c.b(C::Al, 0x20a70)?;
    // 01f3a8: ldr r1, [pc, #0xe6c]
    c.literal(R::R1, 0x2021c)?;
    // 01f3ac: ldr r2, [pc, #0xe6c]
    c.literal(R::R2, 0x20220)?;
    // 01f3b0: ldr r5, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(4)?, 32)?)?;
    // 01f3b4: mov r3, #174
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xae)?)?;
    // 01f3b8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 01f3bc: orr r3, r3, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0xa00)?)?;
    // 01f3c0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f3c4: ldr r8, [pc, #0x984]
    c.literal(R::new(8)?, 0x1fd50)?;
    // 01f3c8: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 01f3cc: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 01f3d0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f3d4: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f3d8: ldr r7, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 56)?)?;
    // 01f3dc: ldr r4, [pc, #0x254]
    c.literal(R::new(4)?, 0x1f638)?;
    // 01f3e0: mov r1, #175
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xaf)?)?;
    // 01f3e4: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f3e8: ldr r0, [r7, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(7)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f3ec: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f3f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f3f4: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f3f8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f3fc: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 24)?)?;
    // 01f400: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f404: str r0, [r10, #0x5b0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1456)?)?;
    // 01f408: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f40c: ldr r1, [r5, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 68)?)?;
    // 01f410: str r1, [r10, #0x5b4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 1460)?)?;
    // 01f414: ldr r1, [r5, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 72)?)?;
    // 01f418: str r1, [r10, #0x5bc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 1468)?)?;
    // 01f41c: ldr r1, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 28)?)?;
    // 01f420: str r1, [r10, #0x5b8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 1464)?)?;
    // 01f424: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 64)?)?;
    // 01f428: add r6, r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R1, 0x4)?)?;
    // 01f42c: add r1, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f430: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01f434: str r0, [r10, #0x5c0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1472)?)?;
    // 01f438: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f43c: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01f440: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f444: str r1, [r10, #0x5c4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 1476)?)?;
    // 01f448: mov r1, #696
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2b8)?)?;
    // 01f44c: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f450: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f454: ldr r0, [r5, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 68)?)?;
    // 01f458: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f45c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f460: add r1, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f464: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01f468: str r0, [r10, #0x2dc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 732)?)?;
    // 01f46c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f470: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01f474: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f478: str r1, [r10, #0x2e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 736)?)?;
    // 01f47c: mov r1, #700
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2bc)?)?;
    // 01f480: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f484: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f488: ldr r0, [r5, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 72)?)?;
    // 01f48c: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f490: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f494: add r1, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f498: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01f49c: str r0, [r10, #0x5c8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1480)?)?;
    // 01f4a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f4a4: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01f4a8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f4ac: str r1, [r10, #0x5cc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 1484)?)?;
    // 01f4b0: mov r1, #2752
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xac0)?)?;
    // 01f4b4: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f4b8: ldr r0, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 28)?)?;
    // 01f4bc: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f4c0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f4c4: add r1, r6, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(6)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f4c8: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01f4cc: str r0, [r10, #0x3e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 992)?)?;
    // 01f4d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f4d4: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01f4d8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f4dc: str r1, [r10, #0x3d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 984)?)?;
    // 01f4e0: mov r1, #708
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2c4)?)?;
    // 01f4e4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f4e8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f4ec: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 01f4f0: mov r6, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x2)?)?;
    // 01f4f4: mov r2, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x5)?)?;
    // 01f4f8: mov r4, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x5)?)?;
    // 01f4fc: orr r6, r6, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(6)?, R::new(6)?, 0x8000)?)?;
    // 01f500: str r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 88)?)?;
    // 01f504: ldr r1, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 16)?)?;
    // 01f508: str r1, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 92)?)?;
    // 01f50c: str r0, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 368)?)?;
    // 01f510: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 01f514: str r0, [r10, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 488)?)?;
    // 01f518: ldr r0, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 20)?)?;
    // 01f51c: str r0, [r10, #0x5d0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1488)?)?;
    // 01f520: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 32)?)?;
    // 01f524: str r0, [r10, #0x5d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1492)?)?;
    // 01f528: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 36)?)?;
    // 01f52c: str r0, [r10, #0x5d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1496)?)?;
    // 01f530: ldr r0, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 40)?)?;
    // 01f534: str r0, [r10, #0x5dc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1500)?)?;
    // 01f538: ldr r0, [r5, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 48)?)?;
    // 01f53c: str r0, [r10, #0x5e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1504)?)?;
    // 01f540: ldr r0, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 52)?)?;
    // 01f544: str r0, [r10, #0x5e4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1508)?)?;
    // 01f548: ldr r0, [r5, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 56)?)?;
    // 01f54c: str r0, [r10, #0x5e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1512)?)?;
    // 01f550: ldr r0, [r5, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 60)?)?;
    // 01f554: str r0, [r10, #0x5ec]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1516)?)?;
    // 01f558: ldr r0, [r5, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 64)?)?;
    // 01f55c: str r0, [r10, #0x5f0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1520)?)?;
    // 01f560: ldr r0, [r5, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 44)?)?;
    // 01f564: str r0, [r10, #0x5f4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1524)?)?;
    // 01f568: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 01f56c: ldr r1, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 16)?)?;
    // 01f570: ldr r0, [r0, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 52)?)?;
    // 01f574: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 01f578: add r0, r10, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x94)?)?;
    // 01f57c: add r3, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xc00)?)?;
    // 01f580: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 01f584: moveq r2, r6
    c.emit(arm32::mov_register(C::Eq, R::R2, R::new(6)?))?;
    // 01f588: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f58c: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 01f590: ldr r1, [r10, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 488)?)?;
    // 01f594: ldr r0, [r0, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 56)?)?;
    // 01f598: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 01f59c: add r0, r10, #292
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x124)?)?;
    // 01f5a0: add r3, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::R0, 0xc00)?)?;
    // 01f5a4: ldr r0, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 368)?)?;
    // 01f5a8: moveq r4, r6
    c.emit(arm32::mov_register(C::Eq, R::new(4)?, R::new(6)?))?;
    // 01f5ac: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f5b0: mov r4, r7
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(7)?))?;
    // 01f5b4: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f5b8: ldr r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 88)?)?;
    // 01f5bc: ldr r1, [pc, #0xf14]
    c.literal(R::R1, 0x204d8)?;
    // 01f5c0: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 01f5c4: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 01f5c8: str r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 20)?)?;
    // 01f5cc: ldr r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 92)?)?;
    // 01f5d0: add r0, r0, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xf)?)?;
    // 01f5d4: lsr r0, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsr, 4)?)?;
    // 01f5d8: str r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 24)?)?;
    // 01f5dc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f5e0: ldr r2, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 8)?)?;
    // 01f5e4: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f5e8: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f5ec: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 01f5f0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f5f4: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f5f8: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01f5fc: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f600: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f604: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01f608: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f60c: bne 0x1f620 <state+0xf1c>
    c.b(C::Ne, 0x1f620)?;
    // 01f610: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 01f614: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 01f618: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01f61c: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 01f620: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01f624: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01f628: bl 0x19fc0 <state+0xffffffffffffb8bc>
    c.bl(0x19fc0)?;
    // 01f62c: b 0x20a64 <state+0x2360>
    c.b(C::Al, 0x20a64)?;
    c.check(0x1f630)?;
    // Literal address / jump-table words at 0x1f630–0x1f63c (not ARM instructions).
    for value in STATE_POOL_01 { c.emit(value)?; }
    c.check(0x1f63c)?;
    // 01f63c: ldr r6, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::new(4)?, 32)?)?;
    // 01f640: add r0, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x50)?)?;
    // 01f644: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 01f648: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 01f64c: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 01f650: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f654: ldr r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 56)?)?;
    // 01f658: ldr r2, [pc, #-0x28]
    c.literal(R::R2, 0x1f638)?;
    // 01f65c: mov r1, #73
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x49)?)?;
    // 01f660: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f664: ldr r0, [r5, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f668: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f66c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f670: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f674: ldr r8, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::SP, 64)?)?;
    // 01f678: mov r0, #272
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x110)?)?;
    // 01f67c: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01f680: ldr r2, [pc, #-0x50]
    c.literal(R::R2, 0x1f638)?;
    // 01f684: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01f688: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f68c: mov r7, r8
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(8)?))?;
    // 01f690: ldr r0, [r7, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(7)?, R::R0, Shift::Lsl, 0)?)?;
    // 01f694: str r0, [r10, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2480)?)?;
    // 01f698: mov r0, #276
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x114)?)?;
    // 01f69c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01f6a0: ldr r0, [r8, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 01f6a4: str r0, [r10, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2484)?)?;
    // 01f6a8: ldr r0, [r8, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 24)?)?;
    // 01f6ac: str r0, [r10, #0x9f0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2544)?)?;
    // 01f6b0: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f6b4: ldr r0, [r5, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f6b8: ldr r1, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01f6bc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f6c0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 01f6c4: mov r1, #90
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5a)?)?;
    // 01f6c8: ldreq r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Eq, true, R::R0, R::new(10)?, 2648)?)?;
    // 01f6cc: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f6d0: ldreq r0, [r0, #0x304]
    c.emit(arm32::transfer_word(C::Eq, true, R::R0, R::R0, 772)?)?;
    // 01f6d4: str r0, [r10, #0x9f8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2552)?)?;
    // 01f6d8: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 16)?)?;
    // 01f6dc: str r0, [r10, #0x644]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1604)?)?;
    // 01f6e0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f6e4: ldr r4, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 64)?)?;
    // 01f6e8: add r9, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(4)?, 0x4)?)?;
    // 01f6ec: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f6f0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f6f4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f6f8: str r0, [r10, #0xa00]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2560)?)?;
    // 01f6fc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f700: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f704: ldr r0, [r10, #0x644]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 1604)?)?;
    // 01f708: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f70c: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f710: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f714: str r0, [r10, #0x64c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1612)?)?;
    // 01f718: mov r0, #236
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xec)?)?;
    // 01f71c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01f720: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 0)?)?;
    // 01f724: ldr r2, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 01f728: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 0)?)?;
    // 01f72c: str r6, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 76)?)?;
    // 01f730: add r6, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::SP, 0x50)?)?;
    // 01f734: mov r3, r6
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(6)?))?;
    // 01f738: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f73c: str r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 68)?)?;
    // 01f740: mov r7, r0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    // 01f744: add r0, r10, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x94)?)?;
    // 01f748: add r1, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x4)?)?;
    // 01f74c: add r5, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0xc00)?)?;
    // 01f750: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 01f754: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01f758: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01f75c: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 01f760: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 01f764: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01f768: ldr r0, [r10, #0x64c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 1612)?)?;
    // 01f76c: ldr r2, [pc, #-0x13c]
    c.literal(R::R2, 0x1f638)?;
    // 01f770: mov r1, #97
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x61)?)?;
    // 01f774: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f778: cmp r0, r7
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 01f77c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f780: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f784: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f788: str r4, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 72)?)?;
    // 01f78c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 01f790: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x1)?)?;
    // 01f794: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 01f798: bhi 0x1f854 <state+0x1150>
    c.b(C::Hi, 0x1f854)?;
    // 01f79c: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 01f7a0: ldr r1, [pc, #0xf74]
    c.literal(R::R1, 0x2071c)?;
    // 01f7a4: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 01f7a8: str r0, [r10, #0x648]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1608)?)?;
    // 01f7ac: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 01f7b0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f7b4: ldr r1, [pc, #0x594]
    c.literal(R::R1, 0x1fd50)?;
    // 01f7b8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 01f7bc: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f7c0: ldr r0, [r10, #0x648]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 1608)?)?;
    // 01f7c4: mov r4, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x2)?)?;
    // 01f7c8: add r5, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x50)?)?;
    // 01f7cc: orr r4, r4, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(4)?, R::new(4)?, 0x8000)?)?;
    // 01f7d0: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 01f7d4: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f7d8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f7dc: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f7e0: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f7e4: str r0, [r10, #0x650]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1616)?)?;
    // 01f7e8: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 68)?)?;
    // 01f7ec: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 0)?)?;
    // 01f7f0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 01f7f4: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01f7f8: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 01f7fc: add r0, r10, #220
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0xdc)?)?;
    // 01f800: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01f804: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 64)?)?;
    // 01f808: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01f80c: add r1, r0, #4352
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x1100)?)?;
    // 01f810: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01f814: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01f818: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01f81c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 01f820: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01f824: ldr r1, [r10, #0x650]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 1616)?)?;
    // 01f828: ldr r2, [pc, #-0x1f8]
    c.literal(R::R2, 0x1f638)?;
    // 01f82c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f830: cmp r1, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 01f834: mov r1, #2672
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xa70)?)?;
    // 01f838: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f83c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f840: ldr r0, [r10, #0x648]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 1608)?)?;
    // 01f844: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f848: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f84c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f850: str r0, [r10, #0xa04]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2564)?)?;
    // 01f854: ldr r5, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 76)?)?;
    // 01f858: ldr r4, [pc, #-0x228]
    c.literal(R::new(4)?, 0x1f638)?;
    // 01f85c: mov r1, #632
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x278)?)?;
    // 01f860: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01f864: ldr r0, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 28)?)?;
    // 01f868: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f86c: str r0, [r10, #0xa48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2632)?)?;
    // 01f870: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f874: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f878: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f87c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f880: str r0, [r10, #0xa0c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2572)?)?;
    // 01f884: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f888: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f88c: ldr r0, [r10, #0xa48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2632)?)?;
    // 01f890: mov r1, #125
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7d)?)?;
    // 01f894: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f898: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f89c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f8a0: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f8a4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f8a8: str r0, [r10, #0xa08]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2568)?)?;
    // 01f8ac: ldr r0, [r5, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 32)?)?;
    // 01f8b0: str r0, [r10, #0xa4c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2636)?)?;
    // 01f8b4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f8b8: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f8bc: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f8c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f8c4: str r0, [r10, #0xa14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2580)?)?;
    // 01f8c8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f8cc: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f8d0: ldr r0, [r10, #0xa4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2636)?)?;
    // 01f8d4: mov r1, #131
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x83)?)?;
    // 01f8d8: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f8dc: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f8e0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f8e4: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f8e8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f8ec: str r0, [r10, #0xa10]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2576)?)?;
    // 01f8f0: ldr r0, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 36)?)?;
    // 01f8f4: str r0, [r10, #0xa50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2640)?)?;
    // 01f8f8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f8fc: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f900: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01f904: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f908: str r0, [r10, #0xa1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2588)?)?;
    // 01f90c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f910: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f914: ldr r0, [r10, #0xa50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2640)?)?;
    // 01f918: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 01f91c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f920: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f924: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f928: str r0, [r10, #0xa18]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2584)?)?;
    // 01f92c: ldr r0, [r5, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 40)?)?;
    // 01f930: str r0, [r10, #0xa54]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2644)?)?;
    // 01f934: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01f938: add r0, r9, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(9)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01f93c: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 4)?)?;
    // 01f940: str r1, [r10, #0xa24]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 2596)?)?;
    // 01f944: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01f948: str r0, [r10, #0xa20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2592)?)?;
    // 01f94c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f950: ldr r1, [r10, #0xcb0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 3248)?)?;
    // 01f954: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 01f958: mov r1, #142
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x8e)?)?;
    // 01f95c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f960: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01f964: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01f968: ldr r2, [r5, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 8)?)?;
    // 01f96c: ldr r1, [pc, #0xf04]
    c.literal(R::R1, 0x20878)?;
    // 01f970: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f974: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f978: ldr r1, [pc, #0x3d0]
    c.literal(R::R1, 0x1fd50)?;
    // 01f97c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01f980: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01f984: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01f988: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 01f98c: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01f990: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01f994: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01f998: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01f99c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01f9a0: bne 0x1f9b4 <state+0x12b0>
    c.b(C::Ne, 0x1f9b4)?;
    // 01f9a4: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 01f9a8: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 01f9ac: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01f9b0: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 01f9b4: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 01f9b8: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 01f9bc: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::R0, 0x1)?)?;
    // 01f9c0: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 01f9c4: bhi 0x20994 <state+0x2290>
    c.b(C::Hi, 0x20994)?;
    // 01f9c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01f9cc: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01f9d0: str r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 44)?)?;
    // 01f9d4: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01f9d8: bl 0x196ac <state+0xffffffffffffafa8>
    c.bl(0x196ac)?;
    // 01f9dc: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 01f9e0: mov r1, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3)?)?;
    // 01f9e4: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 01f9e8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01f9ec: str r0, [r10, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 44)?)?;
    // 01f9f0: b 0x209a0 <state+0x229c>
    c.b(C::Al, 0x209a0)?;
    c.check(0x1f9f4)?;
    // Literal address / jump-table words at 0x1f9f4–0x1f9f8 (not ARM instructions).
    for value in STATE_POOL_02 { c.emit(value)?; }
    c.check(0x1f9f8)?;
    // 01f9f8: ldr r5, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(4)?, 32)?)?;
    // 01f9fc: add r0, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x50)?)?;
    // 01fa00: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 01fa04: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 01fa08: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 01fa0c: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01fa10: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 01fa14: ldr r2, [pc, #-0x3e4]
    c.literal(R::R2, 0x1f638)?;
    // 01fa18: mov r1, #246
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf6)?)?;
    // 01fa1c: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x900)?)?;
    // 01fa20: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01fa24: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fa28: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fa2c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fa30: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 01fa34: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01fa38: str r0, [r10, #0x998]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2456)?)?;
    // 01fa3c: ldr r0, [r5, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 16)?)?;
    // 01fa40: str r0, [r10, #0x9a4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2468)?)?;
    // 01fa44: ldr r0, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 20)?)?;
    // 01fa48: str r0, [r10, #0x9d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2520)?)?;
    // 01fa4c: str r5, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 72)?)?;
    // 01fa50: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 24)?)?;
    // 01fa54: ldr r5, [pc, #-0x424]
    c.literal(R::new(5)?, 0x1f638)?;
    // 01fa58: str r0, [r10, #0x9e4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2532)?)?;
    // 01fa5c: mov r0, #168
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xa8)?)?;
    // 01fa60: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 64)?)?;
    // 01fa64: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fa68: mov r8, r2
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::R2))?;
    // 01fa6c: ldr r0, [r8, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 01fa70: str r0, [r10, #0x9b0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2480)?)?;
    // 01fa74: mov r0, #172
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xac)?)?;
    // 01fa78: mov r10, r2
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::R2))?;
    // 01fa7c: mov r2, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0xb90)?)?;
    // 01fa80: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fa84: orr r2, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1000)?)?;
    // 01fa88: ldr r0, [r10, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(10)?, R::R0, Shift::Lsl, 0)?)?;
    // 01fa8c: str r0, [r1, #0x9b4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2484)?)?;
    // 01fa90: ldr r0, [r10, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 4)?)?;
    // 01fa94: str r0, [r1, #0x9b8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2488)?)?;
    // 01fa98: ldr r0, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 8)?)?;
    // 01fa9c: str r0, [r1, #0x9bc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2492)?)?;
    // 01faa0: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01faa4: str r0, [r1, #0x9c0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2496)?)?;
    // 01faa8: ldr r0, [r10, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 16)?)?;
    // 01faac: str r0, [r1, #0x9c4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2500)?)?;
    // 01fab0: ldr r0, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 20)?)?;
    // 01fab4: str r0, [r1, #0x9c8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2504)?)?;
    // 01fab8: ldr r0, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 24)?)?;
    // 01fabc: str r0, [r1, #0x9cc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2508)?)?;
    // 01fac0: ldr r0, [r10, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 28)?)?;
    // 01fac4: str r0, [r1, #0x9d0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2512)?)?;
    // 01fac8: ldr r0, [r10, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 32)?)?;
    // 01facc: str r0, [r1, #0x9d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2516)?)?;
    // 01fad0: ldr r0, [r10, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 36)?)?;
    // 01fad4: str r0, [r1, #0x9f4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 2548)?)?;
    // 01fad8: ldr r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 12)?)?;
    // 01fadc: ldr r1, [r1, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 2648)?)?;
    // 01fae0: ldr r6, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 76)?)?;
    // 01fae4: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01fae8: ldr r2, [r0, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R2, R::R0, R::R2, Shift::Lsl, 0)?)?;
    // 01faec: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x1)?)?;
    // 01faf0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 01faf4: ldreq r2, [r1, #0x304]
    c.emit(arm32::transfer_word(C::Eq, true, R::R2, R::R1, 772)?)?;
    // 01faf8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fafc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fb00: str r2, [r6, #0x9f8]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(6)?, 2552)?)?;
    // 01fb04: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01fb08: ldr r1, [r1, #0x300]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 768)?)?;
    // 01fb0c: str r1, [r6, #0x9f0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 2544)?)?;
    // 01fb10: mov r1, #22
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x16)?)?;
    // 01fb14: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01fb18: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fb1c: ldr r0, [r6, #0x998]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 2456)?)?;
    // 01fb20: ldr r9, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 64)?)?;
    // 01fb24: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01fb28: add r4, r9, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(9)?, 0x4)?)?;
    // 01fb2c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01fb30: add r1, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(4)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01fb34: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01fb38: str r0, [r6, #0x99c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2460)?)?;
    // 01fb3c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fb40: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01fb44: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fb48: str r1, [r6, #0x9a0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 2464)?)?;
    // 01fb4c: mov r1, #25
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x19)?)?;
    // 01fb50: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01fb54: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fb58: ldr r0, [r6, #0x9a4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 2468)?)?;
    // 01fb5c: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01fb60: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01fb64: add r1, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(4)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01fb68: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01fb6c: str r0, [r6, #0x9a8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2472)?)?;
    // 01fb70: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fb74: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01fb78: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fb7c: str r1, [r6, #0x9ac]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 2476)?)?;
    // 01fb80: mov r1, #29
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1d)?)?;
    // 01fb84: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01fb88: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fb8c: ldr r0, [r6, #0x9d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 2520)?)?;
    // 01fb90: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01fb94: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01fb98: add r1, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(4)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01fb9c: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01fba0: str r0, [r6, #0x9dc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2524)?)?;
    // 01fba4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fba8: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01fbac: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fbb0: str r1, [r6, #0x9e0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 2528)?)?;
    // 01fbb4: mov r1, #33
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x21)?)?;
    // 01fbb8: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01fbbc: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fbc0: ldr r0, [r6, #0x9e4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 2532)?)?;
    // 01fbc4: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01fbc8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01fbcc: add r1, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::new(4)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01fbd0: ldr r0, [r1, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 4)?)?;
    // 01fbd4: str r0, [r6, #0x9ec]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 2540)?)?;
    // 01fbd8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fbdc: ldr r1, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 8)?)?;
    // 01fbe0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fbe4: str r1, [r6, #0x9e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(6)?, 2536)?)?;
    // 01fbe8: mov r1, #548
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x224)?)?;
    // 01fbec: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01fbf0: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fbf4: ldr r0, [r6, #0xcb0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 3248)?)?;
    // 01fbf8: mov r1, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x228)?)?;
    // 01fbfc: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 01fc00: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01fc04: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fc08: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01fc0c: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fc10: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fc14: mov r0, #132
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x84)?)?;
    // 01fc18: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 0)?)?;
    // 01fc1c: add r4, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::SP, 0x50)?)?;
    // 01fc20: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fc24: mov r3, r4
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(4)?))?;
    // 01fc28: ldr r2, [r9, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(9)?, R::R0, Shift::Lsl, 0)?)?;
    // 01fc2c: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 0)?)?;
    // 01fc30: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01fc34: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 01fc38: add r0, r6, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0x94)?)?;
    // 01fc3c: add r1, r9, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x4)?)?;
    // 01fc40: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01fc44: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 0)?)?;
    // 01fc48: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01fc4c: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01fc50: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01fc54: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 01fc58: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01fc5c: ldr r0, [r6, #0x9a0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 2464)?)?;
    // 01fc60: ldr r2, [pc, #-0x630]
    c.literal(R::R2, 0x1f638)?;
    // 01fc64: mov r1, #45
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2d)?)?;
    // 01fc68: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01fc6c: cmp r0, r5
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 01fc70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01fc74: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fc78: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fc7c: ldr r0, [r6, #0x9f0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 2544)?)?;
    // 01fc80: ldr r1, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 0)?)?;
    // 01fc84: ldr r2, [r9]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(9)?, 0)?)?;
    // 01fc88: mov r3, r4
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(4)?))?;
    // 01fc8c: lsl r0, r1, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al, Op::Mov, R::R0, R::R0, R::R1, Shift::Lsl, R::R0, false)?)?;
    // 01fc90: ldr r1, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 0)?)?;
    // 01fc94: mov r10, r6
    c.emit(arm32::mov_register(C::Al, R::new(10)?, R::new(6)?))?;
    // 01fc98: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01fc9c: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 01fca0: add r0, r10, #220
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0xdc)?)?;
    // 01fca4: add r1, r9, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(9)?, 0x14)?)?;
    // 01fca8: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 01fcac: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 0)?)?;
    // 01fcb0: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01fcb4: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01fcb8: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01fcbc: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 01fcc0: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01fcc4: ldr r0, [r10, #0x9ac]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2476)?)?;
    // 01fcc8: ldr r2, [pc, #-0x698]
    c.literal(R::R2, 0x1f638)?;
    // 01fccc: mov r1, #50
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x32)?)?;
    // 01fcd0: orr r1, r1, #2560
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xa00)?)?;
    // 01fcd4: cmp r0, r5
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 01fcd8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01fcdc: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fce0: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fce4: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 01fce8: ldr r1, [pc, #0xeb8]
    c.literal(R::R1, 0x20ba8)?;
    // 01fcec: ldr r2, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R0, 8)?)?;
    // 01fcf0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01fcf4: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01fcf8: ldr r1, [pc, #0x50]
    c.literal(R::R1, 0x1fd50)?;
    // 01fcfc: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01fd00: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01fd04: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01fd08: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 01fd0c: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 01fd10: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01fd14: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 01fd18: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 01fd1c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fd20: bne 0x1fd34 <state+0x1630>
    c.b(C::Ne, 0x1fd34)?;
    // 01fd24: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 01fd28: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 01fd2c: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01fd30: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 01fd34: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01fd38: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 01fd3c: bl 0x18628 <state+0xffffffffffff9f24>
    c.bl(0x18628)?;
    // 01fd40: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 01fd44: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 01fd48: b 0x20a6c <state+0x2368>
    c.b(C::Al, 0x20a6c)?;
    c.check(0x1fd4c)?;
    // Literal address / jump-table words at 0x1fd4c–0x1fd58 (not ARM instructions).
    for value in STATE_POOL_03 { c.emit(value)?; }
    c.check(0x1fd58)?;
    // 01fd58: ldr r8, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(4)?, 32)?)?;
    // 01fd5c: ldr r4, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 64)?)?;
    // 01fd60: ldr r9, [pc, #-0x730]
    c.literal(R::new(9)?, 0x1f638)?;
    // 01fd64: mov r1, #82
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x52)?)?;
    // 01fd68: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01fd6c: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x0)?)?;
    // 01fd70: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 01fd74: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 01fd78: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fd7c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fd80: ldr r2, [r8, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(8)?, 8)?)?;
    // 01fd84: ldr r1, [pc, #0xf8c]
    c.literal(R::R1, 0x20d18)?;
    // 01fd88: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01fd8c: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01fd90: ldr r1, [pc, #-0x48]
    c.literal(R::R1, 0x1fd50)?;
    // 01fd94: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 01fd98: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 01fd9c: ldr r0, [r8, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 36)?)?;
    // 01fda0: mov r5, r4
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(4)?))?;
    // 01fda4: mov r1, #504
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1f8)?)?;
    // 01fda8: mov r6, r4
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(4)?))?;
    // 01fdac: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01fdb0: str r0, [r10, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 80)?)?;
    // 01fdb4: ldr r0, [r8, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 40)?)?;
    // 01fdb8: str r0, [r10, #0x678]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1656)?)?;
    // 01fdbc: mov r0, #532
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x214)?)?;
    // 01fdc0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fdc4: ldr r0, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01fdc8: str r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 88)?)?;
    // 01fdcc: ldr r1, [r4, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(4)?, R::R1, Shift::Lsl, 0)?)?;
    // 01fdd0: str r1, [r10, #0x60]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 96)?)?;
    // 01fdd4: mov r1, #536
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x218)?)?;
    // 01fdd8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 01fddc: ldr r1, [r6, r1]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R1, R::new(6)?, R::R1, Shift::Lsl, 0)?)?;
    // 01fde0: str r1, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 92)?)?;
    // 01fde4: str r0, [r10, #0x170]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 368)?)?;
    // 01fde8: mov r0, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x21c)?)?;
    // 01fdec: str r1, [r10, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 488)?)?;
    // 01fdf0: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 01fdf4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fdf8: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 01fdfc: str r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 48)?)?;
    // 01fe00: str r0, [r10, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 728)?)?;
    // 01fe04: mov r0, #596
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x254)?)?;
    // 01fe08: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fe0c: ldr r0, [r4, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 01fe10: str r0, [r10, #0x8fc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2300)?)?;
    // 01fe14: ldr r1, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 16)?)?;
    // 01fe18: ldr r0, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 12)?)?;
    // 01fe1c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 01fe20: mov r2, r1
    c.emit(arm32::mov_register(C::Al, R::R2, R::R1))?;
    // 01fe24: mov r3, r0
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R0))?;
    // 01fe28: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 01fe2c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fe30: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::new(3)?, R::R0, 0x1)?)?;
    // 01fe34: orrs r0, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Orr, R::R0, R::R1, R::R0, Shift::Lsl, 0, true)?)?;
    // 01fe38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01fe3c: and r2, r3, r2
    c.emit(arm32::data_register(C::Al, Op::And, R::R2, R::new(3)?, R::R2, Shift::Lsl, 0, false)?)?;
    // 01fe40: moveq r0, #1
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fe44: str r2, [r10, #0x900]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(10)?, 2304)?)?;
    // 01fe48: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 01fe4c: str r0, [r10, #0x904]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2308)?)?;
    // 01fe50: ldr r0, [r8, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 20)?)?;
    // 01fe54: str r0, [r10, #0x644]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1604)?)?;
    // 01fe58: ldr r0, [r8, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 44)?)?;
    // 01fe5c: str r0, [r10, #0x684]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1668)?)?;
    // 01fe60: ldr r0, [r8, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 48)?)?;
    // 01fe64: str r0, [r10, #0x694]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 1684)?)?;
    // 01fe68: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fe6c: str r0, [r10, #0x994]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2452)?)?;
    // 01fe70: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 01fe74: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 01fe78: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 01fe7c: mov r1, #111
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x6f)?)?;
    // 01fe80: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01fe84: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01fe88: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fe8c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fe90: mov r7, r10
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(10)?))?;
    // 01fe94: add r10, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::new(4)?, 0x4)?)?;
    // 01fe98: mov r1, #113
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x71)?)?;
    // 01fe9c: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 01fea0: ldr r0, [r7, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 80)?)?;
    // 01fea4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01fea8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01feac: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01feb0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01feb4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01feb8: str r0, [r7, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(7)?, 108)?)?;
    // 01febc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01fec0: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01fec4: ldr r0, [r7, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 80)?)?;
    // 01fec8: sub r9, r11, #104
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::new(9)?, R::R11, 0x68)?)?;
    // 01fecc: mov r3, r9
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(9)?))?;
    // 01fed0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01fed4: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01fed8: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01fedc: str r0, [r7, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(7)?, 116)?)?;
    // 01fee0: mov r0, #496
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1f0)?)?;
    // 01fee4: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01fee8: ldr r2, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 01feec: str r6, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 68)?)?;
    // 01fef0: ldr r1, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 0)?)?;
    // 01fef4: str r5, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 72)?)?;
    // 01fef8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 01fefc: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01ff00: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 01ff04: add r0, r7, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(7)?, 0x94)?)?;
    // 01ff08: add r1, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x4)?)?;
    // 01ff0c: add r6, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0xc00)?)?;
    // 01ff10: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 01ff14: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 01ff18: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01ff1c: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 01ff20: mov r1, r9
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 01ff24: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01ff28: ldr r0, [r7, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 116)?)?;
    // 01ff2c: ldr r2, [pc, #-0x8fc]
    c.literal(R::R2, 0x1f638)?;
    // 01ff30: mov r1, #120
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x78)?)?;
    // 01ff34: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01ff38: cmp r0, r5
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 01ff3c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 01ff40: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ff44: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ff48: str r8, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::SP, 60)?)?;
    // 01ff4c: ldr r0, [r8, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 16)?)?;
    // 01ff50: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ff54: bne 0x20010 <state+0x190c>
    c.b(C::Ne, 0x20010)?;
    // 01ff58: ldr r9, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 76)?)?;
    // 01ff5c: ldr r2, [pc, #-0x92c]
    c.literal(R::R2, 0x1f638)?;
    // 01ff60: mov r1, #125
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x7d)?)?;
    // 01ff64: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 01ff68: ldr r0, [r9, #0x644]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 1604)?)?;
    // 01ff6c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ff70: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ff74: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 01ff78: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 01ff7c: str r0, [r9, #0x640]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(9)?, 1600)?)?;
    // 01ff80: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 01ff84: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 01ff88: ldr r0, [r9, #0x644]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 1604)?)?;
    // 01ff8c: sub r8, r11, #104
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::new(8)?, R::R11, 0x68)?)?;
    // 01ff90: mov r3, r8
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(8)?))?;
    // 01ff94: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 01ff98: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 01ff9c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 01ffa0: str r0, [r9, #0x64c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(9)?, 1612)?)?;
    // 01ffa4: mov r0, #476
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1dc)?)?;
    // 01ffa8: ldr r5, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 64)?)?;
    // 01ffac: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 01ffb0: ldr r2, [r5, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R2, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 01ffb4: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 68)?)?;
    // 01ffb8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 0)?)?;
    // 01ffbc: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 01ffc0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 01ffc4: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 01ffc8: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 01ffcc: add r0, r9, #132
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x84)?)?;
    // 01ffd0: add r1, r5, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(5)?, 0x4)?)?;
    // 01ffd4: add r7, r0, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0x800)?)?;
    // 01ffd8: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 01ffdc: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 01ffe0: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 01ffe4: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 01ffe8: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 01ffec: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 01fff0: ldr r0, [r9, #0x64c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 1612)?)?;
    // 01fff4: ldr r2, [pc, #-0x9c4]
    c.literal(R::R2, 0x1f638)?;
    // 01fff8: mov r1, #131
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x83)?)?;
    // 01fffc: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020000: cmp r0, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 020004: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020008: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 02000c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020010: str r4, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 52)?)?;
    // 020014: ldr r4, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(4)?, 0)?)?;
    // 020018: sub r0, r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R0, R::new(4)?, 0x1)?)?;
    // 02001c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 020020: bhi 0x200f0 <state+0x19ec>
    c.b(C::Hi, 0x200f0)?;
    // 020024: ldr r6, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 76)?)?;
    // 020028: ldr r8, [pc, #-0x9f8]
    c.literal(R::new(8)?, 0x1f638)?;
    // 02002c: mov r1, #2192
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x890)?)?;
    // 020030: ldr r0, [r6, #0x678]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 1656)?)?;
    // 020034: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 020038: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02003c: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 020040: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 020044: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020048: str r0, [r6, #0x70]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 112)?)?;
    // 02004c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020050: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020054: ldr r0, [r6, #0x678]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 1656)?)?;
    // 020058: mov r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 02005c: cmp r4, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x2)?)?;
    // 020060: sub r9, r11, #104
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::new(9)?, R::R11, 0x68)?)?;
    // 020064: orr r5, r5, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(5)?, R::new(5)?, 0x8000)?)?;
    // 020068: mov r3, r9
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(9)?))?;
    // 02006c: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020070: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 020074: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 020078: str r0, [r6, #0x78]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 120)?)?;
    // 02007c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 020080: orr r0, r0, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x8000)?)?;
    // 020084: moveq r5, r0
    c.emit(arm32::mov_register(C::Eq, R::new(5)?, R::R0))?;
    // 020088: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 68)?)?;
    // 02008c: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 020090: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 0)?)?;
    // 020094: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 020098: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 02009c: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 0200a0: mov r7, r0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    // 0200a4: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 64)?)?;
    // 0200a8: add r0, r0, #516
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x204)?)?;
    // 0200ac: add r1, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x1000)?)?;
    // 0200b0: add r0, r6, #220
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(6)?, 0xdc)?)?;
    // 0200b4: add r4, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R0, 0xc00)?)?;
    // 0200b8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0200bc: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 0200c0: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 0200c4: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0200c8: mov r1, r9
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 0200cc: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 0200d0: ldr r1, [r6, #0x78]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(6)?, 120)?)?;
    // 0200d4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0200d8: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 0200dc: cmp r1, r7
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0200e0: mov r1, #149
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x95)?)?;
    // 0200e4: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0200e8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 0200ec: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0200f0: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 0200f4: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 0200f8: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 0200fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020100: bne 0x20bdc <state+0x24d8>
    c.b(C::Ne, 0x20bdc)?;
    // 020104: ldr r5, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 76)?)?;
    // 020108: mvn r1, #-268435455
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0xf0000001)?)?;
    // 02010c: ldr r0, [r5, #0x58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 88)?)?;
    // 020110: add r2, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R0, 0x1f)?)?;
    // 020114: and r3, r1, r2, lsr #4
    c.emit(arm32::data_register(C::Al, Op::And, R::new(3)?, R::R1, R::R2, Shift::Lsr, 4, false)?)?;
    // 020118: bic r2, r2, #31
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0x1f)?)?;
    // 02011c: str r3, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::new(5)?, 20)?)?;
    // 020120: ldr r3, [r5, #0x5c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(5)?, 92)?)?;
    // 020124: add r3, r3, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x1f)?)?;
    // 020128: and r1, r1, r3, lsr #4
    c.emit(arm32::data_register(C::Al, Op::And, R::R1, R::R1, R::new(3)?, Shift::Lsr, 4, false)?)?;
    // 02012c: mul r3, r1, r2
    c.emit(arm32::multiply_register(C::Al, R::new(3)?, R::R1, R::R2)?)?;
    // 020130: str r1, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(5)?, 24)?)?;
    // 020134: add r2, r3, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::new(3)?, 0xff)?)?;
    // 020138: bic r2, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R2, R::R2, 0xff)?)?;
    // 02013c: str r2, [r5, #0x95c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::new(5)?, 2396)?)?;
    // 020140: ldr r2, [r5, #0x8fc]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(5)?, 2300)?)?;
    // 020144: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x2)?)?;
    // 020148: blo 0x20168 <state+0x1a64>
    c.b(C::Cc, 0x20168)?;
    // 02014c: mov r2, #62
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x3e)?)?;
    // 020150: add r0, r2, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R2, R::R0, Shift::Lsl, 1, false)?)?;
    // 020154: bic r0, r0, #63
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x3f)?)?;
    // 020158: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al, R::R2, R::R1, R::R0)?)?;
    // 02015c: add r0, r2, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0xff)?)?;
    // 020160: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0xff)?)?;
    // 020164: str r0, [r5, #0x960]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 2400)?)?;
    // 020168: ldr r0, [r5, #0x684]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 1668)?)?;
    // 02016c: ldr r2, [pc, #-0xb3c]
    c.literal(R::R2, 0x1f638)?;
    // 020170: mov r1, #174
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xae)?)?;
    // 020174: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020178: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02017c: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 020180: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 020184: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020188: str r0, [r5, #0x67c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 1660)?)?;
    // 02018c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020190: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020194: ldr r0, [r5, #0x684]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 1668)?)?;
    // 020198: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02019c: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0201a0: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0201a4: str r0, [r5, #0x680]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 1664)?)?;
    // 0201a8: ldr r1, [r5, #0x8fc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 2300)?)?;
    // 0201ac: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 0201b0: bls 0x20bac <state+0x24a8>
    c.b(C::Ls, 0x20bac)?;
    // 0201b4: ldr r0, [r5, #0x694]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 1684)?)?;
    // 0201b8: ldr r2, [pc, #-0xb88]
    c.literal(R::R2, 0x1f638)?;
    // 0201bc: mov r1, #180
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb4)?)?;
    // 0201c0: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 0201c4: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0201c8: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0201cc: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 0201d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0201d4: str r0, [r5, #0x68c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 1676)?)?;
    // 0201d8: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0201dc: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0201e0: ldr r0, [r5, #0x694]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 1684)?)?;
    // 0201e4: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 0201e8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0201ec: add r0, r10, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(10)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0201f0: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0201f4: str r0, [r5, #0x690]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 1680)?)?;
    // 0201f8: ldr r3, [r5, #0x8fc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(5)?, 2300)?)?;
    // 0201fc: ldr r1, [r5, #0x680]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 1664)?)?;
    // 020200: cmp r3, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(3)?, 0x2)?)?;
    // 020204: add r1, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 020208: ldr r0, [r5, #0x95c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 2396)?)?;
    // 02020c: ldrhs r2, [r5, #0x960]
    c.emit(arm32::transfer_word(C::Cs, true, R::R2, R::new(5)?, 2400)?)?;
    // 020210: b 0x20bbc <state+0x24b8>
    c.b(C::Al, 0x20bbc)?;
    c.check(0x20214)?;
    // Literal address / jump-table words at 0x20214–0x20224 (not ARM instructions).
    for value in STATE_POOL_04 { c.emit(value)?; }
    c.check(0x20224)?;
    // 020224: ldr r4, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(4)?, 32)?)?;
    // 020228: add r10, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::SP, 0x50)?)?;
    // 02022c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 020230: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 020234: mov r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x0)?)?;
    // 020238: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 02023c: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 020240: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 12)?)?;
    // 020244: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020248: ldr r9, [pc, #-0xc18]
    c.literal(R::new(9)?, 0x1f638)?;
    // 02024c: str r0, [r1, #0x50]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 80)?)?;
    // 020250: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 020254: ldr r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 84)?)?;
    // 020258: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 02025c: str r0, [r1, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 728)?)?;
    // 020260: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 16)?)?;
    // 020264: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020268: str r0, [r1, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 84)?)?;
    // 02026c: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 36)?)?;
    // 020270: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020274: str r0, [r1, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 88)?)?;
    // 020278: ldr r0, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 40)?)?;
    // 02027c: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020280: str r0, [r1, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 92)?)?;
    // 020284: ldr r0, [r4, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 52)?)?;
    // 020288: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 02028c: str r0, [r1, #0x80]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 128)?)?;
    // 020290: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 020294: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020298: str r0, [r1, #0x170]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 368)?)?;
    // 02029c: ldr r0, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 80)?)?;
    // 0202a0: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 0202a4: str r0, [r1, #0x1e8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 488)?)?;
    // 0202a8: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 44)?)?;
    // 0202ac: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 0202b0: str r0, [r1, #0x84]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 132)?)?;
    // 0202b4: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 0202b8: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 0202bc: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 0202c0: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 0202c4: mov r1, #221
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xdd)?)?;
    // 0202c8: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    // 0202cc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0202d0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0202d4: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0202d8: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 64)?)?;
    // 0202dc: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 0202e0: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 0202e4: add r5, r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x4)?)?;
    // 0202e8: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 0202ec: ldr r0, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 80)?)?;
    // 0202f0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0202f4: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0202f8: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 0202fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020300: str r0, [r1, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 108)?)?;
    // 020304: mov r1, #1760
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x6e0)?)?;
    // 020308: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 02030c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020310: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 020314: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020318: mov r3, r10
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(10)?))?;
    // 02031c: ldr r0, [r0, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 80)?)?;
    // 020320: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020324: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 020328: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 02032c: str r0, [r1, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 116)?)?;
    // 020330: ldr r2, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 16)?)?;
    // 020334: ldr r0, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 36)?)?;
    // 020338: ldr r1, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 40)?)?;
    // 02033c: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 020340: mov r7, r0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    // 020344: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 020348: add r1, r4, #20
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x14)?)?;
    // 02034c: add r0, r0, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x94)?)?;
    // 020350: add r6, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0xc00)?)?;
    // 020354: ldr r0, [r4, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 16)?)?;
    // 020358: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 02035c: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020360: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 020364: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 020368: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 02036c: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 020370: ldr r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 116)?)?;
    // 020374: mov r1, #229
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe5)?)?;
    // 020378: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 02037c: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    // 020380: cmp r0, r7
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 020384: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020388: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 02038c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020390: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 48)?)?;
    // 020394: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020398: beq 0x20454 <state+0x1d50>
    c.b(C::Eq, 0x20454)?;
    // 02039c: ldr r0, [r10, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 128)?)?;
    // 0203a0: mov r1, #234
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xea)?)?;
    // 0203a4: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    // 0203a8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0203ac: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0203b0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 0203b4: str r0, [r10, #0xf8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 248)?)?;
    // 0203b8: ldr r10, [pc, #-0xd88]
    c.literal(R::new(10)?, 0x1f638)?;
    // 0203bc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0203c0: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0203c4: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 0203c8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0203cc: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 0203d0: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 0203d4: ldr r0, [r0, #0x80]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 128)?)?;
    // 0203d8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0203dc: add r0, r5, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0203e0: add r5, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x50)?)?;
    // 0203e4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0203e8: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 0203ec: str r0, [r1, #0x260]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 608)?)?;
    // 0203f0: ldr r2, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 56)?)?;
    // 0203f4: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 0203f8: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 80)?)?;
    // 0203fc: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 020400: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 020404: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 020408: add r1, r4, #60
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(4)?, 0x3c)?)?;
    // 02040c: add r0, r0, #292
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x124)?)?;
    // 020410: add r7, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0xc00)?)?;
    // 020414: ldr r0, [r4, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 56)?)?;
    // 020418: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 02041c: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020420: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 020424: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 020428: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 02042c: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 020430: mov r2, r10
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(10)?))?;
    // 020434: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 020438: mov r1, #239
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xef)?)?;
    // 02043c: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x600)?)?;
    // 020440: ldr r0, [r0, #0x260]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 608)?)?;
    // 020444: cmp r0, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 020448: movhs r8, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::new(8)?, R::R0, 0x1)?)?;
    // 02044c: mov r0, r8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 020450: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020454: ldr r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 84)?)?;
    // 020458: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 02045c: bne 0x20484 <state+0x1d80>
    c.b(C::Ne, 0x20484)?;
    // 020460: ldr r0, [r10, #0x84]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 132)?)?;
    // 020464: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 64)?)?;
    // 020468: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02046c: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 020470: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 020474: str r1, [r10, #0xfc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 252)?)?;
    // 020478: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 02047c: str r0, [r10, #0x264]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 612)?)?;
    // 020480: ldr r0, [r4, #0x54]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 84)?)?;
    // 020484: ldr r1, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 76)?)?;
    // 020488: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xf)?)?;
    // 02048c: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    // 020490: str r1, [r10, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 20)?)?;
    // 020494: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 80)?)?;
    // 020498: add r1, r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0xf)?)?;
    // 02049c: lsr r1, r1, #4
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R1, Shift::Lsr, 4)?)?;
    // 0204a0: str r1, [r10, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 24)?)?;
    // 0204a4: str r0, [r10, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 728)?)?;
    // 0204a8: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 0204ac: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 0204b0: ldr r5, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 56)?)?;
    // 0204b4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0204b8: ldr r0, [r5, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(5)?, R::R0, Shift::Lsl, 2)?)?;
    // 0204bc: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 0204c0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0204c4: beq 0x20b08 <state+0x2404>
    c.b(C::Eq, 0x20b08)?;
    // 0204c8: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0204cc: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0204d0: bl 0x1e5c8 <state+0xfffffffffffffec4>
    c.bl(0x1e5c8)?;
    // 0204d4: b 0x20b70 <state+0x246c>
    c.b(C::Al, 0x20b70)?;
    c.check(0x204d8)?;
    // Literal address / jump-table words at 0x204d8–0x204dc (not ARM instructions).
    for value in STATE_POOL_05 { c.emit(value)?; }
    c.check(0x204dc)?;
    // 0204dc: ldr r5, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::new(4)?, 32)?)?;
    // 0204e0: add r0, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::SP, 0x50)?)?;
    // 0204e4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0204e8: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 0204ec: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0204f0: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 0204f4: ldr r8, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::SP, 64)?)?;
    // 0204f8: mov r0, #2800
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xaf0)?)?;
    // 0204fc: ldr r6, [pc, #0xec0]
    c.literal(R::new(6)?, 0x213c4)?;
    // 020500: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020504: ldr r0, [r8, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 020508: mov r4, r8
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(8)?))?;
    // 02050c: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 020510: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 020514: str r0, [r10, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 728)?)?;
    // 020518: mov r0, #276
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x114)?)?;
    // 02051c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 020520: ldr r0, [r4, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 0)?)?;
    // 020524: str r0, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 84)?)?;
    // 020528: mov r0, #296
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x128)?)?;
    // 02052c: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 020530: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 020534: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 72)?)?;
    // 020538: str r0, [r10, #0x58]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 88)?)?;
    // 02053c: mov r0, #300
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x12c)?)?;
    // 020540: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 020544: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 020548: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 02054c: str r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 68)?)?;
    // 020550: str r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 92)?)?;
    // 020554: mov r0, #1328
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x530)?)?;
    // 020558: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 02055c: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020560: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 020564: str r1, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 16)?)?;
    // 020568: str r0, [r10, #0x38]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 56)?)?;
    // 02056c: mov r0, #308
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x134)?)?;
    // 020570: mov r1, r8
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(8)?))?;
    // 020574: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 020578: ldr r0, [r1, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 0)?)?;
    // 02057c: str r1, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 40)?)?;
    // 020580: str r0, [r10, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 60)?)?;
    // 020584: ldr r0, [r10, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 12)?)?;
    // 020588: ldr r1, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 56)?)?;
    // 02058c: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 020590: mov r1, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3)?)?;
    // 020594: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 020598: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 02059c: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0205a0: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0205a4: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 0205a8: add r7, r8, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(8)?, 0x4)?)?;
    // 0205ac: mov r1, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x5)?)?;
    // 0205b0: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 0205b4: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 0205b8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0205bc: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0205c0: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 0205c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0205c8: str r0, [r10, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 108)?)?;
    // 0205cc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0205d0: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0205d4: ldr r1, [pc, #0xdec]
    c.literal(R::R1, 0x213c8)?;
    // 0205d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0205dc: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 0205e0: ldr r6, [pc, #0xde4]
    c.literal(R::new(6)?, 0x213cc)?;
    // 0205e4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0205e8: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0205ec: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 0205f0: mov r0, #728
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2d8)?)?;
    // 0205f4: ldr r2, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 0)?)?;
    // 0205f8: ldr r1, [pc, #0xdd0]
    c.literal(R::R1, 0x213d0)?;
    // 0205fc: orr r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1800)?)?;
    // 020600: ldr r3, [r8, r0]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al, true, R::new(3)?, R::new(8)?, R::R0, Shift::Lsl, 0)?)?;
    // 020604: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020608: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 02060c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020610: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 020614: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020618: str r5, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, false, R::new(5)?, R::SP, 12)?)?;
    // 02061c: ldr r1, [pc, #0xdb0]
    c.literal(R::R1, 0x213d4)?;
    // 020620: ldr r0, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 12)?)?;
    // 020624: str r7, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 44)?)?;
    // 020628: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02062c: add r0, r7, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(7)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 020630: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 020634: str r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 116)?)?;
    // 020638: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 02063c: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020640: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020644: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 020648: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 02064c: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 68)?)?;
    // 020650: ldr r2, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 0)?)?;
    // 020654: add r7, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::SP, 0x50)?)?;
    // 020658: mov r3, r7
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 02065c: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 0)?)?;
    // 020660: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 020664: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 020668: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 02066c: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 020670: add r0, r10, #148
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x94)?)?;
    // 020674: str r4, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::SP, 20)?)?;
    // 020678: sub r1, r8, #1472
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R1, R::new(8)?, 0x5c0)?)?;
    // 02067c: add r6, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0xc00)?)?;
    // 020680: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 0)?)?;
    // 020684: str r8, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::SP, 36)?)?;
    // 020688: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 02068c: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020690: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 020694: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 020698: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 02069c: ldr r0, [r10, #0x74]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 116)?)?;
    // 0206a0: ldr r2, [pc, #0xd1c]
    c.literal(R::R2, 0x213c4)?;
    // 0206a4: mov r1, #15
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf)?)?;
    // 0206a8: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 0206ac: cmp r0, r5
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(5)?, Shift::Lsl, 0, false)?)?;
    // 0206b0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0206b4: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0206b8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0206bc: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    // 0206c0: ldr r10, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::R0, 0)?)?;
    // 0206c4: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 40)?)?;
    // 0206c8: ldr r1, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 0)?)?;
    // 0206cc: mul r0, r1, r10
    c.emit(arm32::multiply_register(C::Al, R::R0, R::R1, R::new(10)?)?)?;
    // 0206d0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0206d4: str r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 32)?)?;
    // 0206d8: beq 0x208d8 <state+0x21d4>
    c.b(C::Eq, 0x208d8)?;
    // 0206dc: str r1, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 60)?)?;
    // 0206e0: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 64)?)?;
    // 0206e4: add r0, r1, #248
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0xf8)?)?;
    // 0206e8: add r7, r1, #6272
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R1, 0x1880)?)?;
    // 0206ec: add r0, r0, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1800)?)?;
    // 0206f0: str r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 52)?)?;
    // 0206f4: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 12)?)?;
    // 0206f8: add r0, r0, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x10)?)?;
    // 0206fc: str r0, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 28)?)?;
    // 020700: mov r0, #404
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x194)?)?;
    // 020704: orr r0, r0, #5120
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1400)?)?;
    // 020708: str r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 48)?)?;
    // 02070c: mov r0, #292
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x124)?)?;
    // 020710: orr r0, r0, #3072
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0xc00)?)?;
    // 020714: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 72)?)?;
    // 020718: b 0x20750 <state+0x204c>
    c.b(C::Al, 0x20750)?;
    c.check(0x2071c)?;
    // Literal address / jump-table words at 0x2071c–0x20720 (not ARM instructions).
    for value in STATE_POOL_06 { c.emit(value)?; }
    c.check(0x20720)?;
    // 020720: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 020724: add r6, r6, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::new(6)?, 0x10)?)?;
    // 020728: add r7, r7, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(7)?, 0x48)?)?;
    // 02072c: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x4)?)?;
    // 020730: str r7, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 48)?)?;
    // 020734: str r6, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 52)?)?;
    // 020738: mov r7, r4
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::new(4)?))?;
    // 02073c: add r0, r0, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x48)?)?;
    // 020740: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 72)?)?;
    // 020744: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 40)?)?;
    // 020748: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 02074c: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 60)?)?;
    // 020750: ldr r6, [sp, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 28)?)?;
    // 020754: ldr r4, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 44)?)?;
    // 020758: ldr r5, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::SP, 76)?)?;
    // 02075c: ldr r2, [pc, #0xc60]
    c.literal(R::R2, 0x213c4)?;
    // 020760: mov r1, #796
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x31c)?)?;
    // 020764: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020768: ldr r0, [r6, r9, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(6)?, R::new(9)?, Shift::Lsl, 2)?)?;
    // 02076c: add r8, r5, r9, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(8)?, R::new(5)?, R::new(9)?, Shift::Lsl, 2, false)?)?;
    // 020770: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020774: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 020778: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 02077c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020780: str r0, [r8, #0xf8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(8)?, 248)?)?;
    // 020784: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020788: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 02078c: ldr r0, [r6, r9, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(6)?, R::new(9)?, Shift::Lsl, 2)?)?;
    // 020790: ldr r1, [pc, #0xc40]
    c.literal(R::R1, 0x213d8)?;
    // 020794: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 020798: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02079c: add r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 0207a0: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 0207a4: str r0, [r8, #0x260]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(8)?, 608)?)?;
    // 0207a8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0207ac: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 0207b0: ldr r6, [pc, #0xc14]
    c.literal(R::new(6)?, 0x213cc)?;
    // 0207b4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0207b8: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0207bc: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 0207c0: ldr r10, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 36)?)?;
    // 0207c4: ldr r0, [r7, #-0x348]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, -840)?)?;
    // 0207c8: ldr r2, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(10)?, 0)?)?;
    // 0207cc: str r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::SP, 68)?)?;
    // 0207d0: ldr r1, [r7, #-0x2d0]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(7)?, -720)?)?;
    // 0207d4: add r7, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::SP, 0x50)?)?;
    // 0207d8: mov r3, r7
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 0207dc: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 0207e0: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0207e4: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 0207e8: ldr r6, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 52)?)?;
    // 0207ec: add r5, r5, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::new(5)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0207f0: ldr r0, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 0)?)?;
    // 0207f4: sub r1, r6, #720
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R1, R::new(6)?, 0x2d0)?)?;
    // 0207f8: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 0207fc: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020800: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 020804: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 020808: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 02080c: ldr r0, [r8, #0x260]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 608)?)?;
    // 020810: ldr r2, [pc, #0xbac]
    c.literal(R::R2, 0x213c4)?;
    // 020814: mov r1, #39
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x27)?)?;
    // 020818: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 02081c: cmp r0, r4
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 020820: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020824: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020828: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 02082c: ldr r1, [pc, #0xba8]
    c.literal(R::R1, 0x213dc)?;
    // 020830: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020834: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 020838: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 02083c: ldr r1, [pc, #0xb88]
    c.literal(R::R1, 0x213cc)?;
    // 020840: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020844: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020848: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 02084c: ldr r8, [r10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::new(10)?, 0)?)?;
    // 020850: cmp r0, r9
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(9)?, Shift::Lsl, 0, false)?)?;
    // 020854: bls 0x2087c <state+0x2178>
    c.b(C::Ls, 0x2087c)?;
    // 020858: ldr r0, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 20)?)?;
    // 02085c: ldr r7, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 48)?)?;
    // 020860: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 68)?)?;
    // 020864: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 020868: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R1, R::R0, 0x1)?)?;
    // 02086c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 020870: movls r8, r0
    c.emit(arm32::mov_register(C::Ls, R::new(8)?, R::R0))?;
    // 020874: b 0x20884 <state+0x2180>
    c.b(C::Al, 0x20884)?;
    c.check(0x20878)?;
    // Literal address / jump-table words at 0x20878–0x2087c (not ARM instructions).
    for value in STATE_POOL_07 { c.emit(value)?; }
    c.check(0x2087c)?;
    // 02087c: ldr r7, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 48)?)?;
    // 020880: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 68)?)?;
    // 020884: ldr r0, [r4, #-0x78]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, -120)?)?;
    // 020888: ldr r1, [r4]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 0)?)?;
    // 02088c: add r10, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::SP, 0x50)?)?;
    // 020890: mov r2, r8
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(8)?))?;
    // 020894: mov r3, r10
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(10)?))?;
    // 020898: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 02089c: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 0208a0: mov r1, r6
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(6)?))?;
    // 0208a4: add r5, r0, r7
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::R0, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 0208a8: mov r0, r8
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(8)?))?;
    // 0208ac: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 0208b0: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 0208b4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0208b8: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0208bc: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 0208c0: ldr r0, [sp, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 32)?)?;
    // 0208c4: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(9)?, 0x1)?)?;
    // 0208c8: cmp r9, r0
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R0, Shift::Lsl, 0, false)?)?;
    // 0208cc: bne 0x20720 <state+0x201c>
    c.b(C::Ne, 0x20720)?;
    // 0208d0: ldr r0, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 16)?)?;
    // 0208d4: ldr r10, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::R0, 0)?)?;
    // 0208d8: cmp r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 0x0)?)?;
    // 0208dc: beq 0x2096c <state+0x2268>
    c.b(C::Eq, 0x2096c)?;
    // 0208e0: ldr r0, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 64)?)?;
    // 0208e4: ldr r9, [pc, #0xad8]
    c.literal(R::new(9)?, 0x213c4)?;
    // 0208e8: ldr r8, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(8)?, R::SP, 44)?)?;
    // 0208ec: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0208f0: add r0, r0, #732
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x2dc)?)?;
    // 0208f4: add r7, r0, #6144
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::R0, 0x1800)?)?;
    // 0208f8: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 0208fc: add r4, r0, #740
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R0, 0x2e4)?)?;
    // 020900: ldr r0, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 12)?)?;
    // 020904: add r5, r0, #136
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x88)?)?;
    // 020908: ldr r0, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 0)?)?;
    // 02090c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020910: beq 0x20954 <state+0x2250>
    c.b(C::Eq, 0x20954)?;
    // 020914: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 020918: mov r1, #66
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x42)?)?;
    // 02091c: mov r2, r9
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(9)?))?;
    // 020920: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 020924: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020928: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 02092c: ldr r0, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 4)?)?;
    // 020930: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020934: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 020938: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 02093c: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020940: ldr r0, [r5]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 0)?)?;
    // 020944: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020948: add r0, r8, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(8)?, R::R0, Shift::Lsl, 2, false)?)?;
    // 02094c: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 020950: str r0, [r4, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 20)?)?;
    // 020954: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::new(6)?, 0x1)?)?;
    // 020958: add r4, r4, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x4)?)?;
    // 02095c: add r5, r5, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::new(5)?, 0x4)?)?;
    // 020960: add r7, r7, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(7)?, 0x4)?)?;
    // 020964: cmp r6, r10
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(6)?, R::new(10)?, Shift::Lsl, 0, false)?)?;
    // 020968: bne 0x20908 <state+0x2204>
    c.b(C::Ne, 0x20908)?;
    // 02096c: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 020970: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 020974: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 020978: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 02097c: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 020980: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 020984: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 020988: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 02098c: bl 0x199dc <state+0xffffffffffffb2d8>
    c.bl(0x199dc)?;
    // 020990: b 0x20a64 <state+0x2360>
    c.b(C::Al, 0x20a64)?;
    // 020994: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 020998: mov r1, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x4)?)?;
    // 02099c: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 0209a0: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0209a4: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0209a8: bl 0x18628 <state+0xffffffffffff9f24>
    c.bl(0x18628)?;
    // 0209ac: b 0x20a70 <state+0x236c>
    c.b(C::Al, 0x20a70)?;
    // 0209b0: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 0209b4: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 0209b8: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0209bc: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 0209c0: ldr r0, [r4, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 64)?)?;
    // 0209c4: ldr r7, [r4, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(4)?, 72)?)?;
    // 0209c8: ldr r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 36)?)?;
    // 0209cc: ldr r3, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 40)?)?;
    // 0209d0: ldr r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 44)?)?;
    // 0209d4: stm sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 0209d8: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 0209dc: bl 0x1bec8 <state+0xffffffffffffd7c4>
    c.bl(0x1bec8)?;
    // 0209e0: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 0209e4: ldr r1, [r4, #0x78]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 120)?)?;
    // 0209e8: str r1, [r0, #0x3d4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 980)?)?;
    // 0209ec: ldr r1, [r4, #0x7c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 124)?)?;
    // 0209f0: str r1, [r0, #0x3d8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 984)?)?;
    // 0209f4: mov r4, r5
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(5)?))?;
    // 0209f8: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 0209fc: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 020a00: bl 0x196ac <state+0xffffffffffffafa8>
    c.bl(0x196ac)?;
    // 020a04: b 0x20a64 <state+0x2360>
    c.b(C::Al, 0x20a64)?;
    // 020a08: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 020a0c: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 020a10: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 020a14: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 020a18: ldr r1, [r7]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(7)?, 0)?)?;
    // 020a1c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 020a20: beq 0x20a58 <state+0x2354>
    c.b(C::Eq, 0x20a58)?;
    // 020a24: ldr r3, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::SP, 64)?)?;
    // 020a28: mov r2, #4160
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1040)?)?;
    // 020a2c: mov r0, #60
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3c)?)?;
    // 020a30: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020a34: ldr r7, [r3, r2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::new(7)?, R::new(3)?, R::R2, Shift::Lsl, 0)?)?;
    // 020a38: ldr r2, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 68)?)?;
    // 020a3c: ldr r0, [r3, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(3)?, R::R0, Shift::Lsl, 0)?)?;
    // 020a40: ldr r3, [r2]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 0)?)?;
    // 020a44: ldr r2, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 72)?)?;
    // 020a48: ldr r2, [r2]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R2, 0)?)?;
    // 020a4c: stm sp, {r0, r7}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::new(7)?.bit(), BlockMode::IncrementAfter, false)?)?;
    // 020a50: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 020a54: bl 0x1bec8 <state+0xffffffffffffd7c4>
    c.bl(0x1bec8)?;
    // 020a58: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 020a5c: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 020a60: bl 0x19eac <state+0xffffffffffffb7a8>
    c.bl(0x19eac)?;
    // 020a64: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 020a68: mov r1, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3)?)?;
    // 020a6c: str r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 20)?)?;
    // 020a70: mov r0, #540
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x21c)?)?;
    // 020a74: mov r1, #564
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x234)?)?;
    // 020a78: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 020a7c: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 020a80: ldr r0, [r10, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::R0, Shift::Lsl, 0)?)?;
    // 020a84: ldr r1, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 020a88: sub r1, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Sub, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 020a8c: bl 0x205c <state+0xfffffffffffe3958>
    c.bl(0x205c)?;
    // 020a90: mov r0, #3600
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xe10)?)?;
    // 020a94: mov r1, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x228)?)?;
    // 020a98: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020a9c: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 020aa0: ldr r0, [r10, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::R0, Shift::Lsl, 0)?)?;
    // 020aa4: ldr r1, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 020aa8: sub r1, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Sub, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 020aac: bl 0x205c <state+0xfffffffffffe3958>
    c.bl(0x205c)?;
    // 020ab0: mov r0, #3616
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0xe20)?)?;
    // 020ab4: mov r1, #568
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x238)?)?;
    // 020ab8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020abc: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 020ac0: ldr r0, [r10, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::R0, Shift::Lsl, 0)?)?;
    // 020ac4: ldr r1, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 020ac8: sub r1, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Sub, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 020acc: bl 0x205c <state+0xfffffffffffe3958>
    c.bl(0x205c)?;
    // 020ad0: mov r0, #532
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x214)?)?;
    // 020ad4: mov r1, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x22c)?)?;
    // 020ad8: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 020adc: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 020ae0: ldr r0, [r10, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(10)?, R::R0, Shift::Lsl, 0)?)?;
    // 020ae4: ldr r1, [r10, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::new(10)?, R::R1, Shift::Lsl, 0)?)?;
    // 020ae8: sub r1, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Sub, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 020aec: bl 0x205c <state+0xfffffffffffe3958>
    c.bl(0x205c)?;
    // 020af0: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 020af4: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 020af8: bl 0x80af0 <state+0x623ec>
    c.bl(0x80af0)?;
    // 020afc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020b00: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x1c)?)?;
    // 020b04: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 020b08: ldr r2, [pc, #0x8b4]
    c.literal(R::R2, 0x213c4)?;
    // 020b0c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 020b10: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020b14: orr r1, r1, #1792
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x700)?)?;
    // 020b18: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020b1c: ldr r1, [pc, #0x8d0]
    c.literal(R::R1, 0x213f4)?;
    // 020b20: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020b24: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020b28: ldr r1, [pc, #0x89c]
    c.literal(R::R1, 0x213cc)?;
    // 020b2c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020b30: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020b34: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 020b38: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 020b3c: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 020b40: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 020b44: ldr r0, [r4, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 48)?)?;
    // 020b48: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 020b4c: bne 0x20b70 <state+0x246c>
    c.b(C::Ne, 0x20b70)?;
    // 020b50: ldr r0, [r4, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 76)?)?;
    // 020b54: ldr r1, [r4, #0x50]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 80)?)?;
    // 020b58: ldr r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 36)?)?;
    // 020b5c: ldr r3, [r4, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(4)?, 40)?)?;
    // 020b60: stm sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al, false, R::SP, R::R0.bit() | R::R1.bit(), BlockMode::IncrementAfter, false)?)?;
    // 020b64: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 020b68: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 020b6c: bl 0x1bec8 <state+0xffffffffffffd7c4>
    c.bl(0x1bec8)?;
    // 020b70: ldr r2, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 8)?)?;
    // 020b74: ldr r1, [pc, #0x87c]
    c.literal(R::R1, 0x213f8)?;
    // 020b78: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 020b7c: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020b80: ldr r1, [pc, #0x844]
    c.literal(R::R1, 0x213cc)?;
    // 020b84: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 020b88: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020b8c: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 020b90: mov r1, r10
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(10)?))?;
    // 020b94: bl 0x1a0a0 <state+0xffffffffffffb99c>
    c.bl(0x1a0a0)?;
    // 020b98: ldr r0, [r5, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 144)?)?;
    // 020b9c: mov r1, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3)?)?;
    // 020ba0: mov r4, r5
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(5)?))?;
    // 020ba4: b 0x20a6c <state+0x2368>
    c.b(C::Al, 0x20a6c)?;
    c.check(0x20ba8)?;
    // Literal address / jump-table words at 0x20ba8–0x20bac (not ARM instructions).
    for value in STATE_POOL_08 { c.emit(value)?; }
    c.check(0x20bac)?;
    // 020bac: ldr r1, [r5, #0x690]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 1680)?)?;
    // 020bb0: mov r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x0)?)?;
    // 020bb4: add r1, r1, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R1, R::R0, Shift::Lsl, 0, false)?)?;
    // 020bb8: ldr r0, [r5, #0x95c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 2396)?)?;
    // 020bbc: add r2, r2, r0
    c.emit(arm32::data_register(C::Al, Op::Add, R::R2, R::R2, R::R0, Shift::Lsl, 0, false)?)?;
    // 020bc0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020bc4: cmp r1, r2
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    // 020bc8: ldr r2, [pc, #0x7f4]
    c.literal(R::R2, 0x213c4)?;
    // 020bcc: mov r1, #185
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb9)?)?;
    // 020bd0: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020bd4: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020bd8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020bdc: ldr r9, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 76)?)?;
    // 020be0: ldr r0, [r9, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 728)?)?;
    // 020be4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020be8: beq 0x20eac <state+0x27a8>
    c.b(C::Eq, 0x20eac)?;
    // 020bec: ldr lr, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::LR, R::SP, 64)?)?;
    // 020bf0: mov r12, #340
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R12, R::R0, 0x154)?)?;
    // 020bf4: mov r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x0)?)?;
    // 020bf8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x0)?)?;
    // 020bfc: orr r12, r12, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R12, R::R12, 0x1000)?)?;
    // 020c00: add r2, lr, #356
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::LR, 0x164)?)?;
    // 020c04: add r1, lr, r12
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::LR, R::R12, Shift::Lsl, 0, false)?)?;
    // 020c08: add r8, r2, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::R2, 0x1000)?)?;
    // 020c0c: ldr r2, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 60)?)?;
    // 020c10: add r5, r2, r3
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(5)?, R::R2, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 020c14: add r6, r9, r3
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(6)?, R::new(9)?, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 020c18: ldr r4, [r5, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::new(5)?, 52)?)?;
    // 020c1c: str r4, [r6, #0x698]
    c.emit(arm32::transfer_word(C::Al, false, R::new(4)?, R::new(6)?, 1688)?)?;
    // 020c20: add r4, r4, r4, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::new(4)?, R::new(4)?, Shift::Lsl, 1, false)?)?;
    // 020c24: add r4, lr, r4, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(4)?, R::LR, R::new(4)?, Shift::Lsl, 2, false)?)?;
    // 020c28: ldr r0, [r4, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 8)?)?;
    // 020c2c: str r0, [r6, #0x6a4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1700)?)?;
    // 020c30: ldr r0, [r4, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 12)?)?;
    // 020c34: str r0, [r6, #0x66c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1644)?)?;
    // 020c38: add r0, r1, r3
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 020c3c: ldr r0, [r0, #-0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, -12)?)?;
    // 020c40: str r0, [r6, #0x6b0]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1712)?)?;
    // 020c44: ldr r0, [r1, r3]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R1, R::new(3)?, Shift::Lsl, 0)?)?;
    // 020c48: str r0, [r6, #0x6bc]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1724)?)?;
    // 020c4c: ldr r0, [r8, r3, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(8)?, R::new(3)?, Shift::Lsl, 2)?)?;
    // 020c50: str r0, [r6, #0x6c8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1736)?)?;
    // 020c54: ldr r0, [r2, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R2, 16)?)?;
    // 020c58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020c5c: bne 0x20c78 <state+0x2574>
    c.b(C::Ne, 0x20c78)?;
    // 020c60: ldr r0, [r5, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 24)?)?;
    // 020c64: str r0, [r6, #0x654]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1620)?)?;
    // 020c68: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020c6c: add r0, lr, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::LR, R::R0, Shift::Lsl, 2, false)?)?;
    // 020c70: ldr r0, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 8)?)?;
    // 020c74: str r0, [r6, #0x660]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(6)?, 1632)?)?;
    // 020c78: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::new(7)?, 0x1)?)?;
    // 020c7c: add r3, r3, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::new(3)?, 0x4)?)?;
    // 020c80: cmp r7, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(7)?, 0x3)?)?;
    // 020c84: bne 0x20c10 <state+0x250c>
    c.b(C::Ne, 0x20c10)?;
    // 020c88: ldr r0, [r9, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 728)?)?;
    // 020c8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020c90: beq 0x20d1c <state+0x2618>
    c.b(C::Eq, 0x20d1c)?;
    // 020c94: mov r0, #348
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x15c)?)?;
    // 020c98: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020c9c: ldr r0, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::LR, R::R0, Shift::Lsl, 0)?)?;
    // 020ca0: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1f)?)?;
    // 020ca4: bic r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x1f)?)?;
    // 020ca8: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 72)?)?;
    // 020cac: mov r0, #336
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x150)?)?;
    // 020cb0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020cb4: ldr r0, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::LR, R::R0, Shift::Lsl, 0)?)?;
    // 020cb8: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xff)?)?;
    // 020cbc: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0xff)?)?;
    // 020cc0: str r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 68)?)?;
    // 020cc4: mov r0, #344
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x158)?)?;
    // 020cc8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020ccc: ldr r0, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::LR, R::R0, Shift::Lsl, 0)?)?;
    // 020cd0: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1f)?)?;
    // 020cd4: bic r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0x1f)?)?;
    // 020cd8: str r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 44)?)?;
    // 020cdc: mov r0, #332
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x14c)?)?;
    // 020ce0: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020ce4: ldr r0, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::LR, R::R0, Shift::Lsl, 0)?)?;
    // 020ce8: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xff)?)?;
    // 020cec: bic r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R0, R::R0, 0xff)?)?;
    // 020cf0: str r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 40)?)?;
    // 020cf4: ldr r0, [lr, r12]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::LR, R::R12, Shift::Lsl, 0)?)?;
    // 020cf8: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1f)?)?;
    // 020cfc: bic r4, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::new(4)?, R::R0, 0x1f)?)?;
    // 020d00: mov r0, #328
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x148)?)?;
    // 020d04: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020d08: ldr r0, [lr, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::LR, R::R0, Shift::Lsl, 0)?)?;
    // 020d0c: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xff)?)?;
    // 020d10: bic r6, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::new(6)?, R::R0, 0xff)?)?;
    // 020d14: b 0x20d50 <state+0x264c>
    c.b(C::Al, 0x20d50)?;
    c.check(0x20d18)?;
    // Literal address / jump-table words at 0x20d18–0x20d1c (not ARM instructions).
    for value in STATE_POOL_09 { c.emit(value)?; }
    c.check(0x20d1c)?;
    // 020d1c: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 68)?)?;
    // 020d20: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 020d24: add r0, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1f)?)?;
    // 020d28: bic r1, r0, #31
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::R1, R::R0, 0x1f)?)?;
    // 020d2c: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 020d30: mov r4, r1
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R1))?;
    // 020d34: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 020d38: str r1, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 44)?)?;
    // 020d3c: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 72)?)?;
    // 020d40: add r0, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0xff)?)?;
    // 020d44: bic r6, r0, #255
    c.emit(arm32::data_immediate(C::Al, Op::Bic, R::new(6)?, R::R0, 0xff)?)?;
    // 020d48: str r6, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 40)?)?;
    // 020d4c: str r6, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(6)?, R::SP, 68)?)?;
    // 020d50: add r5, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::SP, 0x50)?)?;
    // 020d54: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 020d58: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 020d5c: mov r8, lr
    c.emit(arm32::mov_register(C::Al, R::new(8)?, R::LR))?;
    // 020d60: mov r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x0)?)?;
    // 020d64: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 020d68: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 020d6c: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 020d70: add r0, r4, r4, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::new(4)?, R::new(4)?, Shift::Lsl, 1, false)?)?;
    // 020d74: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 020d78: orr r7, r7, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(7)?, R::new(7)?, 0x8000)?)?;
    // 020d7c: lsl r1, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 020d80: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 020d84: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 020d88: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 020d8c: ldr r9, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(9)?, R::SP, 76)?)?;
    // 020d90: mov r6, r0
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::R0))?;
    // 020d94: add r0, r8, #352
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0x160)?)?;
    // 020d98: add r1, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x1000)?)?;
    // 020d9c: add r0, r9, #724
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x2d4)?)?;
    // 020da0: add r4, r0, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R0, 0x400)?)?;
    // 020da4: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 020da8: mov r2, r4
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(4)?))?;
    // 020dac: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020db0: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 020db4: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 020db8: bl 0x21f8c <state+0x3888>
    c.bl(0x21f8c)?;
    // 020dbc: ldr r0, [r9, #0x66c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 1644)?)?;
    // 020dc0: ldr r2, [pc, #0x5fc]
    c.literal(R::R2, 0x213c4)?;
    // 020dc4: mov r1, #245
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xf5)?)?;
    // 020dc8: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020dcc: cmp r0, r6
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(6)?, Shift::Lsl, 0, false)?)?;
    // 020dd0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020dd4: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020dd8: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020ddc: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 44)?)?;
    // 020de0: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 020de4: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 020de8: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020dec: lsl r1, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 020df0: ldr r0, [sp, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 40)?)?;
    // 020df4: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 020df8: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 020dfc: add r0, r8, #368
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(8)?, 0x170)?)?;
    // 020e00: add r1, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R0, 0x1000)?)?;
    // 020e04: add r0, r9, #796
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x31c)?)?;
    // 020e08: add r6, r0, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0x400)?)?;
    // 020e0c: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 020e10: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 020e14: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020e18: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 020e1c: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 020e20: bl 0x21f8c <state+0x3888>
    c.bl(0x21f8c)?;
    // 020e24: ldr r0, [r9, #0x670]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 1648)?)?;
    // 020e28: ldr r2, [pc, #0x594]
    c.literal(R::R2, 0x213c4)?;
    // 020e2c: mov r1, #250
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xfa)?)?;
    // 020e30: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020e34: cmp r0, r4
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 020e38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 020e3c: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 020e40: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020e44: ldr r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 72)?)?;
    // 020e48: mov r2, r7
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(7)?))?;
    // 020e4c: mov r3, r5
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(5)?))?;
    // 020e50: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 020e54: lsl r1, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R1, R::R0, Shift::Lsl, 2)?)?;
    // 020e58: ldr r0, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 68)?)?;
    // 020e5c: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 020e60: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 020e64: add r0, r9, #868
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(9)?, 0x364)?)?;
    // 020e68: add r1, r8, #4480
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(8)?, 0x1180)?)?;
    // 020e6c: add r6, r0, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R0, 0x400)?)?;
    // 020e70: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 020e74: mov r2, r6
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(6)?))?;
    // 020e78: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 020e7c: mov r0, r6
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(6)?))?;
    // 020e80: mov r1, r5
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(5)?))?;
    // 020e84: bl 0x21f8c <state+0x3888>
    c.bl(0x21f8c)?;
    // 020e88: ldr r0, [r9, #0x674]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 1652)?)?;
    // 020e8c: ldr r2, [pc, #0x530]
    c.literal(R::R2, 0x213c4)?;
    // 020e90: mov r1, #255
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xff)?)?;
    // 020e94: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 020e98: cmp r0, r4
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(4)?, Shift::Lsl, 0, false)?)?;
    // 020e9c: movhs r10, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::new(10)?, R::R0, 0x1)?)?;
    // 020ea0: mov r0, r10
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(10)?))?;
    // 020ea4: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 020ea8: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 020eac: ldr r0, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 12)?)?;
    // 020eb0: mov r1, #2960
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xb90)?)?;
    // 020eb4: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020eb8: ldr r0, [r4, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::new(4)?, R::R0, Shift::Lsl, 2)?)?;
    // 020ebc: ldr r0, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R0, R::R0, R::R1, Shift::Lsl, 0)?)?;
    // 020ec0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 020ec4: bne 0x20fc4 <state+0x28c0>
    c.b(C::Ne, 0x20fc4)?;
    // 020ec8: ldr r0, [r9, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 2648)?)?;
    // 020ecc: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 24)?)?;
    // 020ed0: mov r1, r9
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(9)?))?;
    // 020ed4: bl 0x1c0d0 <state+0xffffffffffffd9cc>
    c.bl(0x1c0d0)?;
    // 020ed8: ldr r1, [pc, #0x500]
    c.literal(R::R1, 0x213e0)?;
    // 020edc: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 020ee0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020ee4: ldr r1, [pc, #0x4e0]
    c.literal(R::R1, 0x213cc)?;
    // 020ee8: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 020eec: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 020ef0: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 64)?)?;
    // 020ef4: mov r0, #544
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x220)?)?;
    // 020ef8: orr r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1000)?)?;
    // 020efc: ldr r1, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R0, Shift::Lsl, 0)?)?;
    // 020f00: ldr r0, [r9, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 2648)?)?;
    // 020f04: str r1, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 108)?)?;
    // 020f08: mov r1, #548
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x224)?)?;
    // 020f0c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f10: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f14: str r1, [r0, #0x2ac]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 684)?)?;
    // 020f18: mov r1, #552
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x228)?)?;
    // 020f1c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f20: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f24: str r1, [r0, #0x2b0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 688)?)?;
    // 020f28: mov r1, #556
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x22c)?)?;
    // 020f2c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f30: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f34: str r1, [r0, #0x2b4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 692)?)?;
    // 020f38: mov r1, #560
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x230)?)?;
    // 020f3c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f40: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f44: str r1, [r0, #0x2b8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 696)?)?;
    // 020f48: mov r1, #564
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x234)?)?;
    // 020f4c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f50: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f54: str r1, [r0, #0x2bc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 700)?)?;
    // 020f58: mov r1, #568
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x238)?)?;
    // 020f5c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f60: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f64: str r1, [r0, #0x2c0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 704)?)?;
    // 020f68: mov r1, #572
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x23c)?)?;
    // 020f6c: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f70: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f74: str r1, [r0, #0x2c4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 708)?)?;
    // 020f78: mov r1, #4672
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1240)?)?;
    // 020f7c: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f80: str r1, [r0, #0x2c8]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 712)?)?;
    // 020f84: mov r1, #580
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x244)?)?;
    // 020f88: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f8c: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020f90: str r1, [r0, #0x2cc]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 716)?)?;
    // 020f94: mov r1, #584
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x248)?)?;
    // 020f98: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020f9c: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020fa0: str r1, [r0, #0x2d0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 720)?)?;
    // 020fa4: mov r1, #588
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x24c)?)?;
    // 020fa8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020fac: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020fb0: str r1, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 92)?)?;
    // 020fb4: mov r1, #592
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x250)?)?;
    // 020fb8: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 020fbc: ldr r1, [r2, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::R1, R::R2, R::R1, Shift::Lsl, 0)?)?;
    // 020fc0: str r1, [r0, #0x60]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 96)?)?;
    // 020fc4: ldr r0, [r9, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(9)?, 728)?)?;
    // 020fc8: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x2)?)?;
    // 020fcc: blo 0x210ac <state+0x29a8>
    c.b(C::Cc, 0x210ac)?;
    // 020fd0: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 64)?)?;
    // 020fd4: ldr r1, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 76)?)?;
    // 020fd8: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 020fdc: add r0, r2, #428
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x1ac)?)?;
    // 020fe0: add r8, r1, #1712
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::R1, 0x6b0)?)?;
    // 020fe4: add r4, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::R0, 0x1000)?)?;
    // 020fe8: add r0, r1, #940
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R1, 0x3ac)?)?;
    // 020fec: add r5, r0, #1024
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R0, 0x400)?)?;
    // 020ff0: add r0, r2, #424
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R2, 0x1a8)?)?;
    // 020ff4: add r0, r0, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1000)?)?;
    // 020ff8: str r0, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::SP, 72)?)?;
    // 020ffc: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 021000: ldr r6, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(6)?, R::SP, 72)?)?;
    // 021004: add r10, r0, #64
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::R0, 0x40)?)?;
    // 021008: add r7, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(7)?, R::SP, 0x50)?)?;
    // 02100c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 021010: mov r2, #72
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x48)?)?;
    // 021014: mov r0, r7
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(7)?))?;
    // 021018: bl 0x31ec <state+0xfffffffffffe4ae8>
    c.bl(0x31ec)?;
    // 02101c: ldr r0, [r10], #4
    c.emit(arm32::transfer_word_postindexed(C::Al, true, R::R0, R::new(10)?, 4)?)?;
    // 021020: mov r3, r7
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::new(7)?))?;
    // 021024: str r0, [r8, #0x234]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(8)?, 564)?)?;
    // 021028: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 02102c: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 64)?)?;
    // 021030: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 021034: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 021038: str r1, [r8, #0x21c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(8)?, 540)?)?;
    // 02103c: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 021040: str r0, [r8, #0x228]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(8)?, 552)?)?;
    // 021044: ldr r2, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(6)?, 0)?)?;
    // 021048: ldr r0, [r8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 0)?)?;
    // 02104c: ldr r1, [r8, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(8)?, 12)?)?;
    // 021050: bl 0x22164 <state+0x3a60>
    c.bl(0x22164)?;
    // 021054: mov r7, r0
    c.emit(arm32::mov_register(C::Al, R::new(7)?, R::R0))?;
    // 021058: ldr r0, [r6]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(6)?, 0)?)?;
    // 02105c: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 021060: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 021064: bl 0x22064 <state+0x3960>
    c.bl(0x22064)?;
    // 021068: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 02106c: add r1, sp, #80
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x50)?)?;
    // 021070: bl 0x22040 <state+0x393c>
    c.bl(0x22040)?;
    // 021074: ldr r0, [r8, #0x228]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(8)?, 552)?)?;
    // 021078: ldr r2, [pc, #0x344]
    c.literal(R::R2, 0x213c4)?;
    // 02107c: mov r1, #49
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x31)?)?;
    // 021080: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x900)?)?;
    // 021084: cmp r0, r7
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(7)?, Shift::Lsl, 0, false)?)?;
    // 021088: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 02108c: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 021090: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 021094: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(9)?, 0x1)?)?;
    // 021098: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(8)?, R::new(8)?, 0x4)?)?;
    // 02109c: add r4, r4, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(4)?, R::new(4)?, 0x10)?)?;
    // 0210a0: add r5, r5, #72
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::new(5)?, 0x48)?)?;
    // 0210a4: cmp r9, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0x3)?)?;
    // 0210a8: bne 0x21008 <state+0x2904>
    c.b(C::Ne, 0x21008)?;
    // 0210ac: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 0210b0: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 0210b4: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 0210b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0210bc: bne 0x21118 <state+0x2a14>
    c.b(C::Ne, 0x21118)?;
    // 0210c0: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 0210c4: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0210c8: ldr r0, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 76)?)?;
    // 0210cc: str r0, [r10, #0x8f8]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2296)?)?;
    // 0210d0: add r0, r0, r0, lsl #1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R0, Shift::Lsl, 1, false)?)?;
    // 0210d4: ldr r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 64)?)?;
    // 0210d8: add r0, r1, r0, lsl #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R1, R::R0, Shift::Lsl, 2, false)?)?;
    // 0210dc: ldr r1, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 8)?)?;
    // 0210e0: str r1, [r10, #0x8f0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 2288)?)?;
    // 0210e4: ldr r0, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 12)?)?;
    // 0210e8: str r0, [r10, #0x8f4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2292)?)?;
    // 0210ec: str r1, [r10, #0x96c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 2412)?)?;
    // 0210f0: add r0, r10, #2416
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::new(10)?, 0x970)?)?;
    // 0210f4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0210f8: ldr r7, [r10, #0x8f4]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(10)?, 2292)?)?;
    // 0210fc: ldr r3, [r10, #0x8f0]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::new(10)?, 2288)?)?;
    // 021100: add r2, r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x1)?)?;
    // 021104: cmp r2, #4
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x4)?)?;
    // 021108: add r1, r1, r7, lsr #2
    c.emit(arm32::data_register(C::Al, Op::Add, R::R1, R::R1, R::new(7)?, Shift::Lsr, 2, false)?)?;
    // 02110c: add r3, r3, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::new(3)?, R::new(3)?, R::R1, Shift::Lsl, 0, false)?)?;
    // 021110: str r3, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al, false, R::new(3)?, R::R0, 4)?)?;
    // 021114: bne 0x210f8 <state+0x29f4>
    c.b(C::Ne, 0x210f8)?;
    // 021118: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 02111c: ldr r1, [r0, #0x32c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 812)?)?;
    // 021120: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 021124: beq 0x212ec <state+0x2be8>
    c.b(C::Eq, 0x212ec)?;
    // 021128: add r1, r10, #664
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(10)?, 0x298)?)?;
    // 02112c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 021130: add r5, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(5)?, R::R1, 0x1c00)?)?;
    // 021134: add r1, r10, #3728
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(10)?, 0xe90)?)?;
    // 021138: add r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1000)?)?;
    // 02113c: str r1, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 72)?)?;
    // 021140: add r1, r10, #668
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(10)?, 0x29c)?)?;
    // 021144: add r6, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(6)?, R::R1, 0x1c00)?)?;
    // 021148: add r1, r10, #660
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(10)?, 0x294)?)?;
    // 02114c: add r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1c00)?)?;
    // 021150: str r1, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 68)?)?;
    // 021154: add r1, r10, #584
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::new(10)?, 0x248)?)?;
    // 021158: add r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x1c00)?)?;
    // 02115c: str r1, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 64)?)?;
    // 021160: ldr r1, [r10, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 8)?)?;
    // 021164: ldr r2, [sp, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 36)?)?;
    // 021168: cmp r1, r2
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R1, R::R2, Shift::Lsl, 0, false)?)?;
    // 02116c: bne 0x21198 <state+0x2a94>
    c.b(C::Ne, 0x21198)?;
    // 021170: ldr r0, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 40)?)?;
    // 021174: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 021178: mov r8, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x7)?)?;
    // 02117c: str r0, [r10, #0x984]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(10)?, 2436)?)?;
    // 021180: ldr r0, [r10, #0x2d8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 728)?)?;
    // 021184: str r1, [r10, #0x988]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(10)?, 2440)?)?;
    // 021188: mov r10, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x1)?)?;
    // 02118c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 021190: moveq r10, #7
    c.emit(arm32::data_immediate(C::Eq, Op::Mov, R::new(10)?, R::R0, 0x7)?)?;
    // 021194: b 0x211b4 <state+0x2ab0>
    c.b(C::Al, 0x211b4)?;
    // 021198: ldr r2, [pc, #0x224]
    c.literal(R::R2, 0x213c4)?;
    // 02119c: mov r1, #106
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x6a)?)?;
    // 0211a0: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0211a4: mov r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x0)?)?;
    // 0211a8: orr r1, r1, #2304
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x900)?)?;
    // 0211ac: bl 0x17b0 <state+0xfffffffffffe30ac>
    c.bl(0x17b0)?;
    // 0211b0: mov r10, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x0)?)?;
    // 0211b4: ldr r2, [sp, #0x40]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 64)?)?;
    // 0211b8: ldr r1, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::SP, 52)?)?;
    // 0211bc: mvn r3, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::new(3)?, R::R0, 0x0)?)?;
    // 0211c0: ldr r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 56)?)?;
    // 0211c4: str r3, [r2]
    c.emit(arm32::transfer_word(C::Al, false, R::new(3)?, R::R2, 0)?)?;
    // 0211c8: ldr r1, [r1]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 0)?)?;
    // 0211cc: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R1, R::R1, 0x1)?)?;
    // 0211d0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 0211d4: bhi 0x211f4 <state+0x2af0>
    c.b(C::Hi, 0x211f4)?;
    // 0211d8: ldr r4, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 76)?)?;
    // 0211dc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x1)?)?;
    // 0211e0: str r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::new(4)?, 44)?)?;
    // 0211e4: mov r1, r4
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(4)?))?;
    // 0211e8: bl 0x196ac <state+0xffffffffffffafa8>
    c.bl(0x196ac)?;
    // 0211ec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0211f0: str r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 44)?)?;
    // 0211f4: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 0211f8: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 0211fc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 021200: beq 0x21254 <state+0x2b50>
    c.b(C::Eq, 0x21254)?;
    // 021204: ldr r0, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 48)?)?;
    // 021208: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 02120c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 021210: beq 0x212d4 <state+0x2bd0>
    c.b(C::Eq, 0x212d4)?;
    // 021214: ldr r7, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 76)?)?;
    // 021218: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 02121c: str r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(7)?, 68)?)?;
    // 021220: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 021224: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 021228: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 02122c: bl 0x1a1ec <state+0xffffffffffffbae8>
    c.bl(0x1a1ec)?;
    // 021230: ldr r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 68)?)?;
    // 021234: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R0, R::R0, 0x1)?)?;
    // 021238: cmp r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x4)?)?;
    // 02123c: str r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(7)?, 68)?)?;
    // 021240: blo 0x21224 <state+0x2b20>
    c.b(C::Cc, 0x21224)?;
    // 021244: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 021248: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 02124c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 021250: bne 0x212d4 <state+0x2bd0>
    c.b(C::Ne, 0x212d4)?;
    // 021254: ldr r0, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 76)?)?;
    // 021258: str r8, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::R0, 32)?)?;
    // 02125c: str r10, [r0, #0x24]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 36)?)?;
    // 021260: str r8, [r0, #0x28]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::R0, 40)?)?;
    // 021264: str r10, [r0, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::R0, 68)?)?;
    // 021268: b 0x212cc <state+0x2bc8>
    c.b(C::Al, 0x212cc)?;
    // 02126c: cmp r10, #4
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 0x4)?)?;
    // 021270: mov r4, r5
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(5)?))?;
    // 021274: beq 0x21284 <state+0x2b80>
    c.b(C::Eq, 0x21284)?;
    // 021278: cmp r10, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(10)?, 0x1)?)?;
    // 02127c: bne 0x2128c <state+0x2b88>
    c.b(C::Ne, 0x2128c)?;
    // 021280: ldr r4, [sp, #0x48]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 72)?)?;
    // 021284: bl 0x26754 <state+0x8050>
    c.bl(0x26754)?;
    // 021288: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 02128c: ldr r7, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::SP, 76)?)?;
    // 021290: ldr r0, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 56)?)?;
    // 021294: mov r1, r7
    c.emit(arm32::mov_register(C::Al, R::R1, R::new(7)?))?;
    // 021298: bl 0x1a1ec <state+0xffffffffffffbae8>
    c.bl(0x1a1ec)?;
    // 02129c: ldr r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 68)?)?;
    // 0212a0: mov r4, r6
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::new(6)?))?;
    // 0212a4: cmp r0, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x7)?)?;
    // 0212a8: beq 0x212b8 <state+0x2bb4>
    c.b(C::Eq, 0x212b8)?;
    // 0212ac: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x3)?)?;
    // 0212b0: bne 0x212c4 <state+0x2bc0>
    c.b(C::Ne, 0x212c4)?;
    // 0212b4: ldr r4, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 68)?)?;
    // 0212b8: bl 0x26754 <state+0x8050>
    c.bl(0x26754)?;
    // 0212bc: str r0, [r4]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(4)?, 0)?)?;
    // 0212c0: ldr r0, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 68)?)?;
    // 0212c4: add r10, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(10)?, R::R0, 0x1)?)?;
    // 0212c8: str r10, [r7, #0x44]
    c.emit(arm32::transfer_word(C::Al, false, R::new(10)?, R::new(7)?, 68)?)?;
    // 0212cc: cmp r10, r8
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(10)?, R::new(8)?, Shift::Lsl, 0, false)?)?;
    // 0212d0: bls 0x2126c <state+0x2b68>
    c.b(C::Ls, 0x2126c)?;
    // 0212d4: ldr r10, [sp, #0x4c]
    c.emit(arm32::transfer_word(C::Al, true, R::new(10)?, R::SP, 76)?)?;
    // 0212d8: add r9, r9, #1
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(9)?, R::new(9)?, 0x1)?)?;
    // 0212dc: ldr r0, [r10, #0xa58]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(10)?, 2648)?)?;
    // 0212e0: ldr r1, [r0, #0x32c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 812)?)?;
    // 0212e4: cmp r9, r1
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::new(9)?, R::R1, Shift::Lsl, 0, false)?)?;
    // 0212e8: blo 0x21160 <state+0x2a5c>
    c.b(C::Cc, 0x21160)?;
    // 0212ec: ldr r0, [sp, #0x34]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 52)?)?;
    // 0212f0: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 0)?)?;
    // 0212f4: sub r1, r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::R1, R::R0, 0x1)?)?;
    // 0212f8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x1)?)?;
    // 0212fc: bhi 0x21324 <state+0x2c20>
    c.b(C::Hi, 0x21324)?;
    // 021300: ldr r1, [pc, #0xe8]
    c.literal(R::R1, 0x213f0)?;
    // 021304: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 021308: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 02130c: ldr r1, [pc, #0xb8]
    c.literal(R::R1, 0x213cc)?;
    // 021310: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 021314: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 021318: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 02131c: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 021320: b 0x213b8 <state+0x2cb4>
    c.b(C::Al, 0x213b8)?;
    // 021324: ldr r1, [r10, #0x24]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(10)?, 36)?)?;
    // 021328: ldr r4, [sp, #0x38]
    c.emit(arm32::transfer_word(C::Al, true, R::new(4)?, R::SP, 56)?)?;
    // 02132c: cmp r1, #7
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x7)?)?;
    // 021330: bne 0x21354 <state+0x2c50>
    c.b(C::Ne, 0x21354)?;
    // 021334: ldr r1, [pc, #0xb0]
    c.literal(R::R1, 0x213ec)?;
    // 021338: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 02133c: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 021340: ldr r1, [pc, #0x84]
    c.literal(R::R1, 0x213cc)?;
    // 021344: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 021348: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 02134c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x4)?)?;
    // 021350: b 0x213b8 <state+0x2cb4>
    c.b(C::Al, 0x213b8)?;
    // 021354: mvn r1, #32768
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0x8000)?)?;
    // 021358: add r0, r0, r1
    c.emit(arm32::data_register(C::Al, Op::Add, R::R0, R::R0, R::R1, Shift::Lsl, 0, false)?)?;
    // 02135c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 021360: bhi 0x2137c <state+0x2c78>
    c.b(C::Hi, 0x2137c)?;
    // 021364: ldr r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::SP, 60)?)?;
    // 021368: ldr r0, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 16)?)?;
    // 02136c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 021370: ldreq r0, [r10, #0x2d8]
    c.emit(arm32::transfer_word(C::Eq, true, R::R0, R::new(10)?, 728)?)?;
    // 021374: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 021378: beq 0x2139c <state+0x2c98>
    c.b(C::Eq, 0x2139c)?;
    // 02137c: ldr r1, [pc, #0x60]
    c.literal(R::R1, 0x213e4)?;
    // 021380: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 021384: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 021388: ldr r1, [pc, #0x3c]
    c.literal(R::R1, 0x213cc)?;
    // 02138c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 021390: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 021394: mov r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x3)?)?;
    // 021398: b 0x213b8 <state+0x2cb4>
    c.b(C::Al, 0x213b8)?;
    // 02139c: ldr r1, [pc, #0x44]
    c.literal(R::R1, 0x213e8)?;
    // 0213a0: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0213a4: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 0213a8: ldr r1, [pc, #0x1c]
    c.literal(R::R1, 0x213cc)?;
    // 0213ac: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0213b0: bl 0x2ec94 <state+0x10590>
    c.bl(0x2ec94)?;
    // 0213b4: mov r0, #5
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x5)?)?;
    // 0213b8: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0213bc: str r0, [r1, #0x14]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::R1, 20)?)?;
    // 0213c0: b 0x20a70 <state+0x236c>
    c.b(C::Al, 0x20a70)?;
    c.check(0x213c4)?;
    // Literal address / jump-table words at 0x213c4–0x213fc (not ARM instructions).
    for value in STATE_POOL_10 { c.emit(value)?; }
    c.check(0x213fc)?;
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_codec_state_machine(c)?;
    c.check(END)
}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn state_machine_is_address_contiguous(){let mut c=Code::new(START); extend(&mut c).unwrap(); assert_eq!(c.bytes.len(),END-START);}
 #[test]fn jump_table_and_data_pools_are_not_instructions() {
     let mut c=Code::new(START); extend(&mut c).unwrap();
     for (start,pool) in [(0x1e774,&STATE_POOL_00[..]), (0x1f630,&STATE_POOL_01[..]),
                          (0x213c4,&STATE_POOL_10[..])] {
         let bytes=&c.bytes[start-START..start-START+pool.len()*4];
         for (actual,value) in bytes.chunks_exact(4).zip(pool.iter()) {
             assert_eq!(actual, value.to_le_bytes());
         }
     }
 }

}
