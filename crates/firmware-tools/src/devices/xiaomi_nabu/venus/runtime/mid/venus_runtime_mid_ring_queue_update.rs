// SPDX-License-Identifier: GPL-3.0-only
//! Queue entry lookup/update and conditional payload-delivery helpers.
//! The error-format pointers are distinct literal words, not executable
//! instructions. Each A32 operation has explicit operands and conditions.
use super::Code;
use arm32::{BlockMode,Condition as C,DataOp as Op,Register as R,Shift};
const START:usize=0x35ee4;
const END:usize=0x36048;
const ENTRY_DIAGNOSTICS:[u32;3]=[0x000e_c0ec,0x000e_c12d,0x000e_2788];
pub(super) fn emit(c:&mut Code)->Result<(),String>{
    c.check(START)?;
    // 0x35ee4: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35ee8: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x35eec: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x35ef0: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x35ef4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x35ef8: bl 0x35c3c <entry+0x7b0>   @ imm = #-0x2c4
    c.bl(0x35c3c)?;
    // 0x35efc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35f00: beq 0x35f18 <entry+0xa8c>   @ imm = #0x10
    c.b(C::Eq,0x35f18)?;
    // 0x35f04: ldr r1, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,4)?)?;
    // 0x35f08: cmp r1, r4
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x35f0c: bne 0x35f18 <entry+0xa8c>   @ imm = #0x4
    c.b(C::Ne,0x35f18)?;
    // 0x35f10: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x35f14: b 0x35fa8 <entry+0xb1c>   @ imm = #0x8c
    c.b(C::Al,0x35fa8)?;
    // 0x35f18: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // 0x35f1c: ldr r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,24)?)?;
    // 0x35f20: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x35f24: bhs 0x35f40 <entry+0xab4>   @ imm = #0x14
    c.b(C::Cs,0x35f40)?;
    // 0x35f28: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x35f2c: ldr r2, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,20)?)?;
    // 0x35f30: ldr r3, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,32)?)?;
    // 0x35f34: mla r0, r2, r3, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R2,R::new(3)?,R::R1)?)?;
    // 0x35f38: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35f3c: bne 0x35f94 <entry+0xb08>   @ imm = #0x50
    c.b(C::Ne,0x35f94)?;
    // 0x35f40: ldr r1, [pc, #0x64]         @ 0xb20 <entry+0xb20>
    c.literal_cond(C::Al,R::R1,0x35fac)?;
    // 0x35f44: ldr r2, [pc, #0x64]         @ 0xb24 <entry+0xb24>
    c.literal_cond(C::Al,R::R2,0x35fb0)?;
    // 0x35f48: mov r3, #138
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,138)?)?;
    // 0x35f4c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x35f50: orr r3, r3, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,768)?)?;
    // 0x35f54: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x72c8
    c.bl(0x2ec94)?;
    // 0x35f58: ldr r1, [pc, #0x54]         @ 0xb28 <entry+0xb28>
    c.literal_cond(C::Al,R::R1,0x35fb4)?;
    // 0x35f5c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x35f60: bl 0x2ec94 <entry+0xffffffffffff9808> @ imm = #-0x72d4
    c.bl(0x2ec94)?;
    // 0x35f64: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x35f68: bl 0x35d68 <entry+0x8dc>   @ imm = #-0x208
    c.bl(0x35d68)?;
    // 0x35f6c: ldr r0, [r6, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,16)?)?;
    // 0x35f70: ldr r1, [r6, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,24)?)?;
    // 0x35f74: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x35f78: bhs 0x35fa8 <entry+0xb1c>   @ imm = #0x28
    c.b(C::Cs,0x35fa8)?;
    // 0x35f7c: ldr r1, [r6, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,12)?)?;
    // 0x35f80: ldr r2, [r6, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(6)?,20)?)?;
    // 0x35f84: ldr r3, [r6, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(6)?,32)?)?;
    // 0x35f88: mla r0, r2, r3, r1
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::R2,R::new(3)?,R::R1)?)?;
    // 0x35f8c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35f90: popeq {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Eq,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35f94: str r5, [r0]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R0,0)?)?;
    // 0x35f98: str r4, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R0,4)?)?;
    // 0x35f9c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x35fa0: pop {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::LR.bit(),BlockMode::IncrementAfter,true)?)?;
    // 0x35fa4: b 0x35ca0 <entry+0x814>   @ imm = #-0x30c
    c.b(C::Al,0x35ca0)?;
    // 0x35fa8: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(0x35fac)?;
    for word in ENTRY_DIAGNOSTICS {c.emit(word)?;}
    c.check(0x35fb8)?;
    // 0x35fb8: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::LR.bit())?)?;
    // 0x35fbc: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x35fc0: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x35fc4: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x35fc8: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x35fcc: bl 0x35ea4 <entry+0xa18>   @ imm = #-0x130
    c.bl(0x35ea4)?;
    // 0x35fd0: mvn r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(8)?,R::R0,0)?)?;
    // 0x35fd4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x35fd8: beq 0x36040 <entry+0xbb4>   @ imm = #0x60
    c.b(C::Eq,0x36040)?;
    // 0x35fdc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x35fe0: ldr r2, [r5, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,12)?)?;
    // 0x35fe4: ldr r3, [r5, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(5)?,20)?)?;
    // 0x35fe8: ldr r4, [r5, #0x1c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(5)?,28)?)?;
    // 0x35fec: mla r0, r3, r4, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::R0,R::new(3)?,R::new(4)?,R::R2)?)?;
    // 0x35ff0: ldr r2, [r0, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,4)?)?;
    // 0x35ff4: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x35ff8: bhi 0x36024 <entry+0xb98>   @ imm = #0x24
    c.b(C::Hi,0x36024)?;
    // 0x35ffc: ldr r0, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,0)?)?;
    // 0x36000: str r0, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,0)?)?;
    // 0x36004: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x36008: bl 0x35d68 <entry+0x8dc>   @ imm = #-0x2a8
    c.bl(0x35d68)?;
    // 0x3600c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x36010: bl 0x35ea4 <entry+0xa18>   @ imm = #-0x174
    c.bl(0x35ea4)?;
    // 0x36014: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x36018: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x3601c: bne 0x35fe0 <entry+0xb54>   @ imm = #-0x44
    c.b(C::Ne,0x35fe0)?;
    // 0x36020: b 0x3602c <entry+0xba0>   @ imm = #0x4
    c.b(C::Al,0x3602c)?;
    // 0x36024: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x36028: bne 0x36040 <entry+0xbb4>   @ imm = #0x10
    c.b(C::Ne,0x36040)?;
    // 0x3602c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x36030: bl 0x35ea4 <entry+0xa18>   @ imm = #-0x194
    c.bl(0x35ea4)?;
    // 0x36034: mov r8, r0
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R0))?;
    // 0x36038: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x3603c: movne r8, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(8)?,R::R0,1)?)?;
    // 0x36040: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x36044: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::R11.bit() | R::PC.bit(),BlockMode::IncrementAfter,true)?)?;
    c.check(END)
}
#[cfg(test)]mod tests {
 use super::*;
 #[test]fn ring_queue_updates_are_contiguous(){
   let mut c=Code::new(START);emit(&mut c).unwrap();assert_eq!(c.at(),END);
   for (i,x) in ENTRY_DIAGNOSTICS.iter().enumerate(){
     let pos=0x35fac+i*4-START;
     assert_eq!(u32::from_le_bytes(c.bytes[pos..pos+4].try_into().unwrap()),*x);
   }
 }
}
