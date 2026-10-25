// SPDX-License-Identifier: GPL-3.0-only
//! Late Venus controller session start and worker-entry checks. Two complete
//! functions with literal-address pools separately identified from ARM A32.
use super::Code;
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
const START:usize=0x213fc;
const END:usize=0x21604;
// Non-executable diagnostic-pointer pool [0x214ec,0x214f8).
const CONTROL_POOL_0: [u32;3] = [
    0x000e6638, // runtime 0x214ec
    0x000e2788, // runtime 0x214f0
    0x000e6614, // runtime 0x214f4
];
// Non-executable diagnostic-pointer pool [0x215ec,0x21604).
const CONTROL_POOL_1: [u32;6] = [
    0x000e665a, // runtime 0x215ec
    0x000e2788, // runtime 0x215f0
    0x000e66b6, // runtime 0x215f4
    0x000e53c3, // runtime 0x215f8
    0x000e667c, // runtime 0x215fc
    0x000e66a2, // runtime 0x21600
];
fn initialize_and_check(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x213fc: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x21400: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x21404: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x21408: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x2140c: add r3, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::new(3)?, R::SP, 0x4)?)?;
    // 0x21410: ldr r2, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::new(4)?, 144)?)?;
    // 0x21414: ldr r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 180)?)?;
    // 0x21418: add r1, r2, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R2, 0x1c)?)?;
    // 0x2141c: add r2, r2, #24
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::R2, 0x18)?)?;
    // 0x21420: bl 0x7f990 <entry+0x5e594>
    c.bl(0x7f990)?;
    // 0x21424: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x21428: ldr r5, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R0, 24)?)?;
    // 0x2142c: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x0)?)?;
    // 0x21430: beq 0x214c4 <entry+0xc8>
    c.b(C::Eq, 0x214c4)?;
    // 0x21434: ldr r1, [pc, #0xb0]
    c.literal(R::R1, 0x214ec)?;
    // 0x21438: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x2143c: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 0x21440: mov r6, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x1)?)?;
    // 0x21444: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21448: ldr r1, [pc, #0xa0]
    c.literal(R::R1, 0x214f0)?;
    // 0x2144c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x21450: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21454: mov r2, #716
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x2cc)?)?;
    // 0x21458: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x2145c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x21460: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x21464: bl 0x31ec <entry+0xfffffffffffe1df0>
    c.bl(0x31ec)?;
    // 0x21468: bl 0x26754 <entry+0x5358>
    c.bl(0x26754)?;
    // 0x2146c: mov r1, #3664
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xe50)?)?;
    // 0x21470: orr r1, r1, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1000)?)?;
    // 0x21474: str r0, [r5, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(5)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x21478: mov r1, #588
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x24c)?)?;
    // 0x2147c: ldr r0, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 216)?)?;
    // 0x21480: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x1c00)?)?;
    // 0x21484: str r0, [r5, r1]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::new(5)?, R::R1, Shift::Lsl, 0)?)?;
    // 0x21488: mov r0, #584
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x248)?)?;
    // 0x2148c: mvn r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mvn, R::R1, R::R0, 0x0)?)?;
    // 0x21490: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x21494: str r1, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R1, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x21498: mov r0, #516
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x204)?)?;
    // 0x2149c: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x214a0: str r6, [r5, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::new(6)?, R::new(5)?, R::R0, Shift::Lsl, 0)?)?;
    // 0x214a4: ldr r0, [r4, #0x88]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 136)?)?;
    // 0x214a8: str r0, [r5]
    c.emit(arm32::transfer_word(C::Al, false, R::R0, R::new(5)?, 0)?)?;
    // 0x214ac: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x214b0: ldr r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 32)?)?;
    // 0x214b4: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x214b8: bl 0x1d910 <entry+0xffffffffffffc514>
    c.bl(0x1d910)?;
    // 0x214bc: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x214c0: b 0x214e0 <entry+0xe4>
    c.b(C::Al, 0x214e0)?;
    // 0x214c4: ldr r1, [pc, #0x28]
    c.literal(R::R1, 0x214f4)?;
    // 0x214c8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x214cc: mov r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(4)?, R::R0, 0x1)?)?;
    // 0x214d0: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x214d4: ldr r1, [pc, #0x14]
    c.literal(R::R1, 0x214f0)?;
    // 0x214d8: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x214dc: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x214e0: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x214e4: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x10)?)?;
    // 0x214e8: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x214ec)?;
    for value in CONTROL_POOL_0 {c.emit(value)?;}
    c.check(0x214f8)?;
    // 0x214f8: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x214fc: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x21500: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x21504: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x21508: ldr r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 12)?)?;
    // 0x2150c: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x20)?)?;
    // 0x21510: bl 0x80a94 <entry+0x5f698>
    c.bl(0x80a94)?;
    // 0x21514: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x21518: bne 0x2155c <entry+0x160>
    c.b(C::Ne, 0x2155c)?;
    // 0x2151c: ldr r1, [pc, #0xd8]
    c.literal(R::R1, 0x215fc)?;
    // 0x21520: ldr r2, [pc, #0xd8]
    c.literal(R::R2, 0x21600)?;
    // 0x21524: mov r3, #183
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xb7)?)?;
    // 0x21528: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x2152c: orr r3, r3, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0xb00)?)?;
    // 0x21530: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21534: ldr r1, [pc, #0xb4]
    c.literal(R::R1, 0x215f0)?;
    // 0x21538: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x2153c: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21540: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x21544: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 32)?)?;
    // 0x21548: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x2154c: bne 0x215b4 <entry+0x1b8>
    c.b(C::Ne, 0x215b4)?;
    // 0x21550: mov r1, #952
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x3b8)?)?;
    // 0x21554: orr r1, r1, #2048
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x800)?)?;
    // 0x21558: b 0x215a8 <entry+0x1ac>
    c.b(C::Al, 0x215a8)?;
    // 0x2155c: ldr r1, [pc, #0x88]
    c.literal(R::R1, 0x215ec)?;
    // 0x21560: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x21564: mov r5, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x1)?)?;
    // 0x21568: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x2156c: ldr r1, [pc, #0x7c]
    c.literal(R::R1, 0x215f0)?;
    // 0x21570: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x21574: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x21578: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 144)?)?;
    // 0x2157c: ldr r0, [r1, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 8)?)?;
    // 0x21580: add r1, r1, #32
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::R1, 0x20)?)?;
    // 0x21584: bl 0x80a94 <entry+0x5f698>
    c.bl(0x80a94)?;
    // 0x21588: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x2158c: bne 0x215cc <entry+0x1d0>
    c.b(C::Ne, 0x215cc)?;
    // 0x21590: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x21594: ldr r0, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 32)?)?;
    // 0x21598: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x0)?)?;
    // 0x2159c: bne 0x215b4 <entry+0x1b8>
    c.b(C::Ne, 0x215b4)?;
    // 0x215a0: mov r1, #197
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xc5)?)?;
    // 0x215a4: orr r1, r1, #2816
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0xb00)?)?;
    // 0x215a8: ldr r2, [pc, #0x48]
    c.literal(R::R2, 0x215f8)?;
    // 0x215ac: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x215b0: bl 0x17b0 <entry+0xfffffffffffe03b4>
    c.bl(0x17b0)?;
    // 0x215b4: ldr r0, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 144)?)?;
    // 0x215b8: ldr r1, [r0, #0x20]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 32)?)?;
    // 0x215bc: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x215c0: bl 0x1c9c8 <entry+0xffffffffffffb5cc>
    c.bl(0x1c9c8)?;
    // 0x215c4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x215c8: b 0x215e4 <entry+0x1e8>
    c.b(C::Al, 0x215e4)?;
    // 0x215cc: ldr r1, [pc, #0x20]
    c.literal(R::R1, 0x215f4)?;
    // 0x215d0: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x215d4: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x215d8: ldr r1, [pc, #0x10]
    c.literal(R::R1, 0x215f0)?;
    // 0x215dc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x215e0: bl 0x2ec94 <entry+0xd898>
    c.bl(0x2ec94)?;
    // 0x215e4: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x215e8: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    c.check(0x215ec)?;
    for value in CONTROL_POOL_1 {c.emit(value)?;}
    c.check(0x21604)?;
    c.check(END)
}
pub(super) fn extend(c:&mut Code)->Result<(),String>{initialize_and_check(c)?;c.check(END)}
#[cfg(test)]mod tests{use super::*;#[test]fn controller_entry_exact(){let mut c=Code::new(START);extend(&mut c).unwrap();assert_eq!(c.bytes.len(),END-START);}}
