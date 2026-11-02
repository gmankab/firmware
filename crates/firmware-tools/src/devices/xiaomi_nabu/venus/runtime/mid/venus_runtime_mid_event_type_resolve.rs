// SPDX-License-Identifier: GPL-3.0-only
//! Dispatch codec-stream event types to validated channel callbacks and
//! controller error paths. Two indexed jump tables use named absolute
//! runtime addresses; six trailing controller error constants are data.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x33e10;
const END:usize=0x34088;
const LARGE_EVENT_TABLE:[u32;6]=[
  0x33ed4,0x33ed4,0x33e6c,0x33fb4,0x3400c,0x33ed4,
];
const SMALL_EVENT_TABLE:[u32;7]=[
  0x33ed4,0x33ed4,0x33ed4,0x33ed4,0x33fe4,0x33f04,0x33ed4,
];
const CONTROLLER_EVENT_LITERALS:[u32;6]=[
  0xfffefffd,0xffeeefff,0x000eb784,0x000eb988,0x000eb9b0,0x000e2788,
];
/// Shared event-type and descriptor-state resolver. The A32 handler branches
/// are re-encoded by checked instruction constructors rather than copied.
pub(super) fn emit(code:&mut Code)->Result<(),String>{
  let c=code;
  c.check(START)?;
    // 0x33e10: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x33e14: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x33e18: sub sp, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,8)?)?;
    // 0x33e1c: ldr lr, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R2,4)?)?;
    // 0x33e20: cmp lr, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,1114112)?)?;
    // 0x33e24: ble 0x33e9c <entry+0x211c>  @ imm = #0x70
    c.b(C::Le,0x33e9c)?;
    // 0x33e28: ldr r3, [pc, #0x244]        @ 0x22f4 <entry+0x22f4>
    c.literal(R::new(3)?,0x34074)?;
    // 0x33e2c: add r3, lr, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x33e30: cmp r3, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,7)?)?;
    // 0x33e34: blo 0x33f04 <entry+0x2184>  @ imm = #0xc8
    c.b(C::Cc,0x33f04)?;
    // 0x33e38: mvn r3, #1114112
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,1114112)?)?;
    // 0x33e3c: add r3, lr, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x33e40: cmp r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,5)?)?;
    // 0x33e44: bhi 0x3400c <entry+0x228c>  @ imm = #0x1c0
    c.b(C::Hi,0x3400c)?;
    // 0x33e48: lsl r12, r3, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x33e4c: add r3, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // 0x33e50: ldr pc, [r12, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R12,R::new(3)?,Shift::Lsl,0)?)?;
  c.check(0x33e54)?;
  for v in LARGE_EVENT_TABLE {c.emit(v)?;}
  c.check(0x33e6c)?;
    // 0x33e6c: ldrb r3, [r2, #0x21]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,33)?)?;
    // 0x33e70: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x33e74: bne 0x3401c <entry+0x229c>  @ imm = #0x1a0
    c.b(C::Ne,0x3401c)?;
    // 0x33e78: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x33e7c: beq 0x3404c <entry+0x22cc>  @ imm = #0x1c8
    c.b(C::Eq,0x3404c)?;
    // 0x33e80: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x33e84: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x33e88: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x2a0
    c.bl(0x33bf0)?;
    // 0x33e8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33e90: bne 0x33f2c <entry+0x21ac>  @ imm = #0x94
    c.b(C::Ne,0x33f2c)?;
    // 0x33e94: mov r1, #187
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,187)?)?;
    // 0x33e98: b 0x33f9c <entry+0x221c>  @ imm = #0xfc
    c.b(C::Al,0x33f9c)?;
    // 0x33e9c: ldr r3, [pc, #0x1cc]        @ 0x22f0 <entry+0x22f0>
    c.literal(R::new(3)?,0x34070)?;
    // 0x33ea0: add r12, lr, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x33ea4: cmp r12, #6
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R12,6)?)?;
    // 0x33ea8: bhi 0x33ef4 <entry+0x2174>  @ imm = #0x44
    c.b(C::Hi,0x33ef4)?;
    // 0x33eac: lsl r12, r12, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::R12,R::R12,Shift::Lsl,2)?)?;
    // 0x33eb0: add r3, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::PC,0)?)?;
    // 0x33eb4: ldr pc, [r12, r3]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::R12,R::new(3)?,Shift::Lsl,0)?)?;
  c.check(0x33eb8)?;
  for v in SMALL_EVENT_TABLE {c.emit(v)?;}
  c.check(0x33ed4)?;
    // 0x33ed4: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x33ed8: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x2f0
    c.bl(0x33bf0)?;
    // 0x33edc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33ee0: bne 0x33f2c <entry+0x21ac>  @ imm = #0x44
    c.b(C::Ne,0x33f2c)?;
    // 0x33ee4: ldr r2, [pc, #0x18c]        @ 0x22f8 <entry+0x22f8>
    c.literal(R::R2,0x34078)?;
    // 0x33ee8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33eec: mov r1, #1648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1648)?)?;
    // 0x33ef0: b 0x33fa8 <entry+0x2228>  @ imm = #0xb0
    c.b(C::Al,0x33fa8)?;
    // 0x33ef4: mvn r3, #69632
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(3)?,R::R0,69632)?)?;
    // 0x33ef8: add r3, lr, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::LR,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x33efc: cmp r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x33f00: bhs 0x3400c <entry+0x228c>  @ imm = #0x104
    c.b(C::Cs,0x3400c)?;
    // 0x33f04: ldr r12, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R2,8)?)?;
    // 0x33f08: add lr, r12, r12, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::LR,R::R12,R::R12,Shift::Lsl,2,false)?)?;
    // 0x33f0c: add r3, r0, lr, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::R0,R::LR,Shift::Lsl,2,false)?)?;
    // 0x33f10: ldr r3, [r3, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,360)?)?;
    // 0x33f14: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x33f18: beq 0x33f34 <entry+0x21b4>  @ imm = #0x14
    c.b(C::Eq,0x33f34)?;
    // 0x33f1c: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x33f20: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x338
    c.bl(0x33bf0)?;
    // 0x33f24: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33f28: beq 0x33f68 <entry+0x21e8>  @ imm = #0x38
    c.b(C::Eq,0x33f68)?;
    // 0x33f2c: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x33f30: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33f34: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x33f38: beq 0x33f70 <entry+0x21f0>  @ imm = #0x30
    c.b(C::Eq,0x33f70)?;
    // 0x33f3c: add r0, r1, lr, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::LR,Shift::Lsl,2,false)?)?;
    // 0x33f40: ldr r0, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,360)?)?;
    // 0x33f44: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33f48: beq 0x33f70 <entry+0x21f0>  @ imm = #0x20
    c.b(C::Eq,0x33f70)?;
    // 0x33f4c: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x33f50: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x33f54: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x36c
    c.bl(0x33bf0)?;
    // 0x33f58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33f5c: bne 0x33f2c <entry+0x21ac>  @ imm = #-0x38
    c.b(C::Ne,0x33f2c)?;
    // 0x33f60: mov r1, #225
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,225)?)?;
    // 0x33f64: b 0x33f9c <entry+0x221c>  @ imm = #0x30
    c.b(C::Al,0x33f9c)?;
    // 0x33f68: mov r1, #215
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,215)?)?;
    // 0x33f6c: b 0x33f9c <entry+0x221c>  @ imm = #0x28
    c.b(C::Al,0x33f9c)?;
    // 0x33f70: ldr r1, [pc, #0x104]        @ 0x22fc <entry+0x22fc>
    c.literal(R::R1,0x3407c)?;
    // 0x33f74: ldr r2, [pc, #0x104]        @ 0x2300 <entry+0x2300>
    c.literal(R::R2,0x34080)?;
    // 0x33f78: mov r3, #230
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,230)?)?;
    // 0x33f7c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x33f80: str r12, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::R12,R::SP,0)?)?;
    // 0x33f84: orr r3, r3, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,1536)?)?;
    // 0x33f88: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x52fc
    c.bl(0x2ec94)?;
    // 0x33f8c: ldr r1, [pc, #0xf0]         @ 0x2304 <entry+0x2304>
    c.literal(R::R1,0x34084)?;
    // 0x33f90: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x33f94: bl 0x2ec94 <entry+0xffffffffffffcf14> @ imm = #-0x5308
    c.bl(0x2ec94)?;
    // 0x33f98: mov r1, #231
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,231)?)?;
    // 0x33f9c: orr r1, r1, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1536)?)?;
    // 0x33fa0: ldr r2, [pc, #0xd0]         @ 0x22f8 <entry+0x22f8>
    c.literal(R::R2,0x34078)?;
    // 0x33fa4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x33fa8: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x33fac: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x33fb0: b 0x17b0 <entry+0xfffffffffffcfa30> @ imm = #-0x32808
    c.b(C::Al,0x17b0)?;
    // 0x33fb4: ldrb r3, [r2, #0x11]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,17)?)?;
    // 0x33fb8: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x33fbc: bne 0x34034 <entry+0x22b4>  @ imm = #0x70
    c.b(C::Ne,0x34034)?;
    // 0x33fc0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x33fc4: beq 0x3405c <entry+0x22dc>  @ imm = #0x90
    c.b(C::Eq,0x3405c)?;
    // 0x33fc8: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x33fcc: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x33fd0: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x3e8
    c.bl(0x33bf0)?;
    // 0x33fd4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x33fd8: bne 0x33f2c <entry+0x21ac>  @ imm = #-0xb4
    c.b(C::Ne,0x33f2c)?;
    // 0x33fdc: mov r1, #159
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,159)?)?;
    // 0x33fe0: b 0x33f9c <entry+0x221c>  @ imm = #-0x4c
    c.b(C::Al,0x33f9c)?;
    // 0x33fe4: ldrb r3, [r2, #0xd]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::R2,13)?)?;
    // 0x33fe8: tst r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,1)?)?;
    // 0x33fec: bne 0x33ffc <entry+0x227c>  @ imm = #0x8
    c.b(C::Ne,0x33ffc)?;
    // 0x33ff0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x33ff4: beq 0x34064 <entry+0x22e4>  @ imm = #0x68
    c.b(C::Eq,0x34064)?;
    // 0x33ff8: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x33ffc: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x34000: mov sp, r11
    c.emit(arm32::mov_register(C::Al,R::SP,R::R11))?;
    // 0x34004: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x34008: b 0x34088 <entry+0x2308>  @ imm = #0x78
    c.b(C::Al,0x34088)?;
    // 0x3400c: ldr r2, [pc, #0x64]         @ 0x22f8 <entry+0x22f8>
    c.literal(R::R2,0x34078)?;
    // 0x34010: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34014: mov r1, #1776
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1776)?)?;
    // 0x34018: b 0x33fa8 <entry+0x2228>  @ imm = #-0x78
    c.b(C::Al,0x33fa8)?;
    // 0x3401c: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x34020: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x438
    c.bl(0x33bf0)?;
    // 0x34024: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34028: bne 0x33f2c <entry+0x21ac>  @ imm = #-0x104
    c.b(C::Ne,0x33f2c)?;
    // 0x3402c: mov r1, #178
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,178)?)?;
    // 0x34030: b 0x33f9c <entry+0x221c>  @ imm = #-0x9c
    c.b(C::Al,0x33f9c)?;
    // 0x34034: mov r1, r2
    c.emit(arm32::mov_register(C::Al,R::R1,R::R2))?;
    // 0x34038: bl 0x33bf0 <entry+0x1e70>  @ imm = #-0x450
    c.bl(0x33bf0)?;
    // 0x3403c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x34040: bne 0x33f2c <entry+0x21ac>  @ imm = #-0x11c
    c.b(C::Ne,0x33f2c)?;
    // 0x34044: mov r1, #150
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,150)?)?;
    // 0x34048: b 0x33f9c <entry+0x221c>  @ imm = #-0xb4
    c.b(C::Al,0x33f9c)?;
    // 0x3404c: ldr r2, [pc, #0x24]         @ 0x22f8 <entry+0x22f8>
    c.literal(R::R2,0x34078)?;
    // 0x34050: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x34054: mov r1, #1728
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1728)?)?;
    // 0x34058: b 0x33fa8 <entry+0x2228>  @ imm = #-0xb8
    c.b(C::Al,0x33fa8)?;
    // 0x3405c: mov r1, #676
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,676)?)?;
    // 0x34060: b 0x34068 <entry+0x22e8>  @ imm = #0x0
    c.b(C::Al,0x34068)?;
    // 0x34064: mov r1, #648
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,648)?)?;
    // 0x34068: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x3406c: b 0x33fa0 <entry+0x2220>  @ imm = #-0xd4
    c.b(C::Al,0x33fa0)?;
  c.check(0x34070)?;
  for v in CONTROLLER_EVENT_LITERALS {c.emit(v)?;}
  c.check(END)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_tables_preserve_all_repeated_targets_and_literal_boundaries() {
        assert_eq!(LARGE_EVENT_TABLE.len(), 6);
        assert_eq!(SMALL_EVENT_TABLE.len(), 7);
        assert_eq!(CONTROLLER_EVENT_LITERALS.len(), 6);
        assert_eq!(LARGE_EVENT_TABLE[0], LARGE_EVENT_TABLE[1]);
        assert_eq!(SMALL_EVENT_TABLE[1], SMALL_EVENT_TABLE[3]);
        assert_eq!(CONTROLLER_EVENT_LITERALS[5], 0x000e2788);
        assert_eq!(END - START, 139 * 4 + 19 * 4);
        let mut code = Code::new(START);
        emit(&mut code).unwrap();
        assert_eq!(code.at(), END);
    }
}
