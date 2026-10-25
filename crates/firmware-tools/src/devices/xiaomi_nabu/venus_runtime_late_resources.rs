// SPDX-License-Identifier: GPL-3.0-only
//! Controller diagnostic routing and codec-session release callbacks.
//! The register selectors determine three controller resource classes;
//! private diagnostic string pointers remain explicit data words.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift,BlockMode};
use super::Code;
const START:usize=0x1b164;
const END:usize=0x1b304;
const RESOURCE_TRACES:[u32;6]=[0x000e_3869,0x000e_4dc0,0x000e_2788,
   0x000e_4dd7,0x000e_4e07,0x000e_4e2e];

/// Lookup one of three codec buffer controller slots, report the resource
/// result, then dispatch codec success/error callbacks.
fn emit_resource_slot_diagnostics(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x1b164: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1b168: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x1b16c: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::SP, 0x8)?)?;
    // 0x1b170: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1b174: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x2)?)?;
    // 0x1b178: str r1, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::SP, 4)?)?;
    // 0x1b17c: beq 0x1b19c <entry+0xfb0>
    c.b(C::Eq, 0x1b19c)?;
    // 0x1b180: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x1)?)?;
    // 0x1b184: beq 0x1b1a8 <entry+0xfbc>
    c.b(C::Eq, 0x1b1a8)?;
    // 0x1b188: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1b18c: bne 0x1b1b4 <entry+0xfc8>
    c.b(C::Ne, 0x1b1b4)?;
    // 0x1b190: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b194: ldr r5, [r0, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R0, 12)?)?;
    // 0x1b198: b 0x1b1ec <entry+0x1000>
    c.b(C::Al, 0x1b1ec)?;
    // 0x1b19c: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b1a0: ldr r5, [r0, #0x10]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R0, 16)?)?;
    // 0x1b1a4: b 0x1b1ec <entry+0x1000>
    c.b(C::Al, 0x1b1ec)?;
    // 0x1b1a8: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b1ac: ldr r5, [r0, #0x8]
    c.emit(arm32::transfer_word(C::Al, true, R::new(5)?, R::R0, 8)?)?;
    // 0x1b1b0: b 0x1b1ec <entry+0x1000>
    c.b(C::Al, 0x1b1ec)?;
    // 0x1b1b4: str r2, [sp]
    c.emit(arm32::transfer_word(C::Al, false, R::R2, R::SP, 0)?)?;
    // 0x1b1b8: ldr r1, [pc, #0xac]
    c.literal(R::R1, 0x1b26c)?;
    // 0x1b1bc: ldr r2, [pc, #0xac]
    c.literal(R::R2, 0x1b270)?;
    // 0x1b1c0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b1c4: mov r3, #36
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x24)?)?;
    // 0x1b1c8: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b1cc: ldr r1, [pc, #0xa0]
    c.literal(R::R1, 0x1b274)?;
    // 0x1b1d0: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b1d4: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b1d8: ldr r2, [pc, #0x98]
    c.literal(R::R2, 0x1b278)?;
    // 0x1b1dc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b1e0: mov r1, #37
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x25)?)?;
    // 0x1b1e4: mov r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(5)?, R::R0, 0x0)?)?;
    // 0x1b1e8: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b1ec: add r1, sp, #4
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R1, R::SP, 0x4)?)?;
    // 0x1b1f0: mov r0, r5
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(5)?))?;
    // 0x1b1f4: bl 0x80928 <entry+0x6673c>
    c.bl(0x80928)?;
    // 0x1b1f8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R0, 0x1)?)?;
    // 0x1b1fc: bne 0x1b230 <entry+0x1044>
    c.b(C::Ne, 0x1b230)?;
    // 0x1b200: ldr r2, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al, true, R::R2, R::SP, 4)?)?;
    // 0x1b204: ldr r1, [pc, #0x74]
    c.literal(R::R1, 0x1b280)?;
    // 0x1b208: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b20c: ldr r3, [r2, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::new(3)?, R::R2, 12)?)?;
    // 0x1b210: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b214: ldr r1, [pc, #0x58]
    c.literal(R::R1, 0x1b274)?;
    // 0x1b218: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b21c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b220: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b224: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R0, 20)?)?;
    // 0x1b228: bl 0x292c <entry+0xfffffffffffe8740>
    c.bl(0x292c)?;
    // 0x1b22c: b 0x1b260 <entry+0x1074>
    c.b(C::Al, 0x1b260)?;
    // 0x1b230: ldr r1, [pc, #0x44]
    c.literal(R::R1, 0x1b27c)?;
    // 0x1b234: ldr r2, [pc, #0x34]
    c.literal(R::R2, 0x1b270)?;
    // 0x1b238: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b23c: mov r3, #42
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::new(3)?, R::R0, 0x2a)?)?;
    // 0x1b240: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b244: ldr r1, [pc, #0x28]
    c.literal(R::R1, 0x1b274)?;
    // 0x1b248: mov r0, #8
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x8)?)?;
    // 0x1b24c: bl 0x2ec94 <entry+0x14aa8>
    c.bl(0x2ec94)?;
    // 0x1b250: ldr r2, [pc, #0x20]
    c.literal(R::R2, 0x1b278)?;
    // 0x1b254: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b258: mov r1, #43
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x2b)?)?;
    // 0x1b25c: bl 0x17b0 <entry+0xfffffffffffe75c4>
    c.bl(0x17b0)?;
    // 0x1b260: mov r0, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x0)?)?;
    // 0x1b264: sub sp, r11, #8
    c.emit(arm32::data_immediate(C::Al, Op::Sub, R::SP, R::R11, 0x8)?)?;
    // 0x1b268: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    for word in RESOURCE_TRACES {c.emit(word)?;}
    c.check(0x1b284)
}

/// Clear the selected controller-owned codec handle, optionally closing
/// the hardware resource first. Two one-instruction return helpers follow.
fn emit_close_selected_resource(c:&mut Code)->Result<(),String>{
    c.check(0x1b284)?;
    // 0x1b284: push {r4, r5, r11, lr}
    c.emit(arm32::push_registers(C::Al, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x1b288: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al, Op::Add, R::R11, R::SP, 0x8)?)?;
    // 0x1b28c: mov r4, r0
    c.emit(arm32::mov_register(C::Al, R::new(4)?, R::R0))?;
    // 0x1b290: mov r5, r3
    c.emit(arm32::mov_register(C::Al, R::new(5)?, R::new(3)?))?;
    // 0x1b294: mov r3, r1
    c.emit(arm32::mov_register(C::Al, R::new(3)?, R::R1))?;
    // 0x1b298: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b29c: ldr r1, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R0, 28)?)?;
    // 0x1b2a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R1, 0x0)?)?;
    // 0x1b2a4: beq 0x1b2f0 <entry+0x1104>
    c.b(C::Eq, 0x1b2f0)?;
    // 0x1b2a8: ldr r0, [r1, #0xc]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::R1, 12)?)?;
    // 0x1b2ac: cmp r0, r3
    c.emit(arm32::data_register(C::Al, Op::Cmp, R::R0, R::R0, R::new(3)?, Shift::Lsl, 0, false)?)?;
    // 0x1b2b0: popne {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Ne, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1b2b4: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::R2, 0x0)?)?;
    // 0x1b2b8: beq 0x1b2cc <entry+0x10e0>
    c.b(C::Eq, 0x1b2cc)?;
    // 0x1b2bc: mov r2, #7
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R2, R::R0, 0x7)?)?;
    // 0x1b2c0: mov r0, r4
    c.emit(arm32::mov_register(C::Al, R::R0, R::new(4)?))?;
    // 0x1b2c4: orr r2, r2, #8192
    c.emit(arm32::data_immediate(C::Al, Op::Orr, R::R2, R::R2, 0x2000)?)?;
    // 0x1b2c8: bl 0x24f90 <entry+0xada4>
    c.bl(0x24f90)?;
    // 0x1b2cc: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al, Op::Cmp, R::R0, R::new(5)?, 0x0)?)?;
    // 0x1b2d0: beq 0x1b2e4 <entry+0x10f8>
    c.b(C::Eq, 0x1b2e4)?;
    // 0x1b2d4: ldr r1, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::new(4)?, 152)?)?;
    // 0x1b2d8: ldr r0, [r4, #0xb4]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 180)?)?;
    // 0x1b2dc: ldr r1, [r1, #0x1c]
    c.emit(arm32::transfer_word(C::Al, true, R::R1, R::R1, 28)?)?;
    // 0x1b2e0: bl 0x7fd1c <entry+0x65b30>
    c.bl(0x7fd1c)?;
    // 0x1b2e4: ldr r0, [r4, #0x98]
    c.emit(arm32::transfer_word(C::Al, true, R::R0, R::new(4)?, 152)?)?;
    // 0x1b2e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R1, R::R0, 0x0)?)?;
    // 0x1b2ec: str r1, [r0, #0x1c]
    c.emit(arm32::transfer_word(C::Al, false, R::R1, R::R0, 28)?)?;
    // 0x1b2f0: pop {r4, r5, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al, true, R::SP, R::new(4)?.bit() | R::new(5)?.bit() | R::R11.bit() | R::PC.bit(), BlockMode::IncrementAfter, true)?)?;
    // 0x1b2f4: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    // 0x1b2f8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    // 0x1b2fc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al, Op::Mov, R::R0, R::R0, 0x1)?)?;
    c.check(0x1b300)?;
    // 0x1b300: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;
    c.check(END)
}

pub(super) fn extend(c:&mut Code)->Result<(),String>{
    emit_resource_slot_diagnostics(c)?;
    emit_close_selected_resource(c)?;
    c.check(END)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn selected_resource_diagnostics_are_data_not_code(){
  let mut c=Code::new(START);extend(&mut c).unwrap();
  assert_eq!(c.bytes.len(),END-START);
  for (i,v) in RESOURCE_TRACES.iter().enumerate(){
    let offset=0x1b26c-START+i*4;
    assert_eq!(&c.bytes[offset..offset+4],&v.to_le_bytes());
  }
 }
}
