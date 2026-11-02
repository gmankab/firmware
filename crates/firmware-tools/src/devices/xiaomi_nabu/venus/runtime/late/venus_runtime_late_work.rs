// SPDX-License-Identifier: GPL-3.0-only
//! Codec resource attach and controller state dispatch.
//! The four-entry branch table is kept separate from typed A32 code;
//! diagnostics and platform global addresses are not executable opcodes.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift, BlockMode};
use super::Code;
const START:usize=0x1ba40;
const END:usize=0x1bc34;
const ATTACH_LOGS:[u32;3]=[0x000e52aa, 0x000e2788, 0x000e5292];
const CONTROL_STATE_TARGETS:[u32;4]=[0x0001bb84, 0x0001bb94, 0x0001bba4, 0x0001bbcc];
const CONTROLLER_DIAGNOSTICS:[u32;5]=[0x0010dfc0, 0x000e2565, 0x000e52d4, 0x000e2788, 0x000e4dd7];

/// Request a codec resource from platform, attach it to session and mark
/// its header/status flags on success; otherwise publish error diagnostics.
fn attach_and_start_codec(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1ba40: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1ba44: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x10)?)?;
    // 0x1ba48: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x1ba4c: add r2, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R2, R::SP, 0x4)?)?;
    // 0x1ba50: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1ba54: mov r5, r0
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::R0))?;
    // 0x1ba58: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1ba5c: bl 0x253fc <entry+0xa0fc>
    c.bl(0x253fc)?;
    // 0x1ba60: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1ba64: cmp r4, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(4)?, 0x1)?)?;
    // 0x1ba68: bne 0x1ba88 <entry+0x788>
    c.b(C::Ne, 0x1ba88)?;
    // 0x1ba6c: ldr r1, [pc, #0x7c]
    c.literal(R::R1, 0x1baf0)?;
    // 0x1ba70: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ba74: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1ba78: ldr r1, [pc, #0x6c]
    c.literal(R::R1, 0x1baec)?;
    // 0x1ba7c: mov r0, #2
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x2)?)?;
    // 0x1ba80: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1ba84: b 0x1badc <entry+0x7dc>
    c.b(C::Al, 0x1badc)?;
    // 0x1ba88: ldr r0, [r5, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(5)?, 152)?)?;
    // 0x1ba8c: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 4)?)?;
    // 0x1ba90: ldr r1, [pc, #0x50]
    c.literal(R::R1, 0x1bae8)?;
    // 0x1ba94: mov r7, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(7)?, R::R0, 0x1)?)?;
    // 0x1ba98: str r2, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::R0, 28)?)?;
    // 0x1ba9c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1baa0: ldr r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 12)?)?;
    // 0x1baa4: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1baa8: ldr r1, [pc, #0x3c]
    c.literal(R::R1, 0x1baec)?;
    // 0x1baac: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1bab0: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1bab4: bl 0x26754 <entry+0xb454>
    c.bl(0x26754)?;
    // 0x1bab8: ldr r1, [r5, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(5)?, 152)?)?;
    // 0x1babc: mov r3, #3680
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0xe60)?)?;
    // 0x1bac0: orr r3, r3, #4096
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(3)?, R::new(3)?, 0x1000)?)?;
    // 0x1bac4: ldr r2, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::R1, 28)?)?;
    // 0x1bac8: str r0, [r2, r3]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R2, R::new(3)?, Shift::Lsl, 0)?)?;
    // 0x1bacc: mov r0, #516
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x204)?)?;
    // 0x1bad0: str r7, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(7)?, R::R1, 24)?)?;
    // 0x1bad4: orr r0, r0, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R0, R::R0, 0x1c00)?)?;
    // 0x1bad8: str r6, [r2, r0]
    c.emit(arm32::transfer_word_register(C::Al, false, R::new(6)?, R::R2, R::R0, Shift::Lsl, 0)?)?;
    // 0x1badc: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1bae0: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x10)?)?;
    // 0x1bae4: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for d in ATTACH_LOGS{c.emit(d)?;}
    c.check(0x1baf4)
}

/// Controller processing loop prologue dispatching four session modes.
fn codec_work_loop_prefix(c:&mut Code)->Result<(),String>{
    c.check(0x1baf4)?;
    // 0x1baf4: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1baf8: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x1c)?)?;
    // 0x1bafc: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x4)?)?;
    // 0x1bb00: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1bb04: ldr r0, [pc, #0x114]
    c.literal(R::R0, 0x1bc20)?;
    // 0x1bb08: ldr r1, [pc, #0x114]
    c.literal(R::R1, 0x1bc24)?;
    // 0x1bb0c: ldr r7, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::new(7)?, R::new(4)?, 152)?)?;
    // 0x1bb10: str r1, [r0]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 0)?)?;
    // 0x1bb14: bl 0x307c <entry+0xfffffffffffe7d7c>
    c.bl(0x307c)?;
    // 0x1bb18: ldr r5, [pc, #0x110]
    c.literal(R::new(5)?, 0x1bc30)?;
    // 0x1bb1c: mov r10, #616
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(10)?, R::R0, 0x268)?)?;
    // 0x1bb20: mov r8, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(8)?, R::R0, 0x0)?)?;
    // 0x1bb24: mov r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(9)?, R::R0, 0x0)?)?;
    // 0x1bb28: orr r10, r10, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::new(10)?, R::new(10)?, 0x1c00)?)?;
    // 0x1bb2c: str r8, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::new(7)?, 24)?)?;
    // 0x1bb30: ldr r1, [pc, #0xf0]
    c.literal(R::R1, 0x1bc28)?;
    // 0x1bb34: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1bb38: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1bb3c: ldr r1, [pc, #0xe8]
    c.literal(R::R1, 0x1bc2c)?;
    // 0x1bb40: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1bb44: bl 0x2ec94 <entry+0x13994>
    c.bl(0x2ec94)?;
    // 0x1bb48: ldr r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 20)?)?;
    // 0x1bb4c: bl 0x294c <entry+0xfffffffffffe764c>
    c.bl(0x294c)?;
    // 0x1bb50: ldr r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 20)?)?;
    // 0x1bb54: bl 0x293c <entry+0xfffffffffffe763c>
    c.bl(0x293c)?;
    // 0x1bb58: mov r6, r9
    c.emit(arm32::mov_register(C::Al, R::new(6)?, R::new(9)?))?;
    // 0x1bb5c: ldr r0, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(7)?, 24)?)?;
    // 0x1bb60: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x3)?)?;
    // 0x1bb64: bhi 0x1bbf8 <entry+0x8f8>
    c.b(C::Hi, 0x1bbf8)?;
    // 0x1bb68: lsl r0, r0, #2
    c.emit(arm32::mov_shifted_register(C::Al, R::R0, R::R0, Shift::Lsl, 2)?)?;
    // 0x1bb6c: add r1, pc, #0
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::PC, 0x0)?)?;
    // 0x1bb70: ldr pc, [r0, r1]
    c.emit(arm32::transfer_word_register(C::Al, true, R::PC, R::R0, R::R1, Shift::Lsl, 0)?)?;
    for target in CONTROL_STATE_TARGETS{c.emit(target)?;}
    c.check(0x1bb84)
}

/// Process selected codec work item, update resource flags and retry until
/// a later command completes. This routine does not return normally.
fn codec_work_loop_cases(c:&mut Code)->Result<(),String>{
    c.check(0x1bb84)?;
    // 0x1bb84: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1bb88: bl 0x1ba40 <entry+0x740>
    c.bl(0x1ba40)?;
    // 0x1bb8c: mov r9, r0
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R0))?;
    // 0x1bb90: b 0x1bc10 <entry+0x910>
    c.b(C::Al, 0x1bc10)?;
    // 0x1bb94: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1bb98: bl 0x1b434 <entry+0x134>
    c.bl(0x1b434)?;
    // 0x1bb9c: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1bba0: b 0x1bb5c <entry+0x85c>
    c.b(C::Al, 0x1bb5c)?;
    // 0x1bba4: bl 0x26754 <entry+0xb454>
    c.bl(0x26754)?;
    // 0x1bba8: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bbac: mov r2, #604
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x25c)?)?;
    // 0x1bbb0: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x1c00)?)?;
    // 0x1bbb4: ldr r1, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 28)?)?;
    // 0x1bbb8: str r0, [r1, r2]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::R2, Shift::Lsl, 0)?)?;
    // 0x1bbbc: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1bbc0: mov r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x1)?)?;
    // 0x1bbc4: bl 0xd790 <entry+0xffffffffffff2490>
    c.bl(0xd790)?;
    // 0x1bbc8: b 0x1bbe4 <entry+0x8e4>
    c.b(C::Al, 0x1bbe4)?;
    // 0x1bbcc: bl 0x26754 <entry+0xb454>
    c.bl(0x26754)?;
    // 0x1bbd0: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1bbd4: ldr r1, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 28)?)?;
    // 0x1bbd8: str r0, [r1, r10]
    c.emit(arm32::transfer_word_register(C::Al, false, R::R0, R::R1, R::new(10)?, Shift::Lsl, 0)?)?;
    // 0x1bbdc: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1bbe0: bl 0x1a7fc <entry+0xfffffffffffff4fc>
    c.bl(0x1a7fc)?;
    // 0x1bbe4: mov r9, r0
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::R0))?;
    // 0x1bbe8: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1bbec: str r8, [r0, #0x18]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::R0, 24)?)?;
    // 0x1bbf0: str r8, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::new(8)?, R::R0, 28)?)?;
    // 0x1bbf4: b 0x1bc10 <entry+0x910>
    c.b(C::Al, 0x1bc10)?;
    // 0x1bbf8: mov r1, #238
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0xee)?)?;
    // 0x1bbfc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1bc00: mov r2, r5
    c.emit(arm32::mov_register(C::Al, R::R2, R::new(5)?))?;
    // 0x1bc04: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R1, R::R1, 0x100)?)?;
    // 0x1bc08: bl 0x17b0 <entry+0xfffffffffffe64b0>
    c.bl(0x17b0)?;
    // 0x1bc0c: mov r9, r6
    c.emit(arm32::mov_register(C::Al, R::new(9)?, R::new(6)?))?;
    // 0x1bc10: mov r6, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(6)?, R::R0, 0x0)?)?;
    // 0x1bc14: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(9)?, 0x0)?)?;
    // 0x1bc18: beq 0x1bb5c <entry+0x85c>
    c.b(C::Eq, 0x1bb5c)?;
    // 0x1bc1c: b 0x1bb30 <entry+0x830>
    c.b(C::Al, 0x1bb30)?;
    for d in CONTROLLER_DIAGNOSTICS{c.emit(d)?;}
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    attach_and_start_codec(c)?;
    codec_work_loop_prefix(c)?;
    codec_work_loop_cases(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn work_loop_targets_and_diagnostics_are_data(){
    let mut c=Code::new(START);extend(&mut c).unwrap();
    assert_eq!(c.bytes.len(),END-START);
    for (a,nums) in [(0x1bae8,&ATTACH_LOGS[..]),(0x1bb74,&CONTROL_STATE_TARGETS[..]),
      (0x1bc20,&CONTROLLER_DIAGNOSTICS[..])]{
      for (i,v) in nums.iter().enumerate(){
        let offset=a-START+4*i;assert_eq!(&c.bytes[offset..offset+4],&v.to_le_bytes());
      }
    }
 }
}
