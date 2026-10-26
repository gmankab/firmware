// SPDX-License-Identifier: GPL-3.0-only
//! Reconstruct two independent Venus controller functions following the
//! indexed codec-plane code: signed packed halfword comparison and the
//! codec buffer dispatch into format-specific receiver callbacks.
//!
//! All opcodes are produced by checked, public A32 constructors with
//! explicit source registers/conditions. No OEM opcode words are embedded.
//! Full compatibility with the original firmware on hardware is unproven.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};

pub const START:usize=0x53944;
pub const END:usize=0x53b24;
pub const FILE_OFFSET:usize=0x4000+START;
struct Code { at:usize, bytes:Vec<u8> }
impl Code {
    fn new()->Self { Self{at:START,bytes:Vec::with_capacity(END-START)} }
    fn check(&self,a:usize)->Result<(),String> {
       if self.at!=a {return Err(format!("A32 middle 0x{:x} expected 0x{a:x}",self.at));}
       Ok(())
    }
    #[inline(never)] fn emit(&mut self, word:u32)->Result<(),String>{
       if self.at<START || self.at>=END || self.at&3 !=0 {
           return Err(format!("A32 middle out of range: 0x{:x}",self.at));
       }
       self.bytes.extend(word.to_le_bytes());self.at+=4;Ok(())
    }
}

/// Eight-lane plane normalization and optional scalar quality comparator.
/// Uses register-indexed shifts to avoid declaring input stride assumptions.
#[inline(never)]
fn normalize_packet_planes(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x53944: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x53948: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5394c: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x53950: ldr r8, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,8)?)?;
    // 0x53954: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x53958: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x5395c: mov r9, r2
    c.emit(arm32::mov_register(C::Al,R::new(9)?,R::R2))?;
    // 0x53960: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x53964: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x53968: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x5396c: ldr r2, [r0]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,0)?)?;
    // 0x53970: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x53974: lsl r6, r2, r3
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::R2,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x53978: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5397c: str r6, [r0], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(6)?,R::R0,4)?)?;
    // 0x53980: lslhi r7, r2, r3
    c.emit(arm32::data_register_shifted_by_register(C::Hi,Op::Mov,R::new(7)?,R::R0,R::R2,Shift::Lsl,R::new(3)?,false)?)?;
    // 0x53984: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x53988: bne 0x5396c <entry+0x28>    @ imm = #-0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5396c,false)?)?;
    // 0x5398c: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x53990: beq 0x539bc <entry+0x78>    @ imm = #0x24
    c.emit(arm32::branch(C::Eq,c.at as u32,0x539bc,false)?)?;
    // 0x53994: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x53998: mov r4, r5
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::new(5)?))?;
    // 0x5399c: ldr r0, [r4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
    // 0x539a0: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x539a4: lsl r0, r0, #8
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,8)?)?;
    // 0x539a8: blx 0x83c4c <entry+0x30308> @ imm = #0x3029c
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x539ac: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x539b0: str r0, [r4], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x539b4: cmp r6, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,8)?)?;
    // 0x539b8: bne 0x5399c <entry+0x58>    @ imm = #-0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5399c,false)?)?;
    // 0x539bc: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x539c0: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x539c4: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x539c8: beq 0x53a0c <entry+0xc8>    @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53a0c,false)?)?;
    // 0x539cc: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x539d0: mov r2, r10
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(10)?))?;
    // 0x539d4: mov r3, r5
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
    // 0x539d8: mov r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
    // 0x539dc: ldr r6, [r3], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(6)?,R::new(3)?,4)?)?;
    // 0x539e0: ldr r4, [r2], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(4)?,R::R2,4)?)?;
    // 0x539e4: add r7, r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
    // 0x539e8: subs r6, r4, r6
    c.emit(arm32::data_register(C::Al,Op::Sub,R::new(6)?,R::new(4)?,R::new(6)?,Shift::Lsl,0,true)?)?;
    // 0x539ec: rsbmi r6, r6, #0
    c.emit(arm32::data_immediate(C::Mi,Op::Rsb,R::new(6)?,R::new(6)?,0)?)?;
    // 0x539f0: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x539f4: add r0, r6, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x539f8: bne 0x539dc <entry+0x98>    @ imm = #-0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x539dc,false)?)?;
    // 0x539fc: lsr r2, r0, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R0,Shift::Lsr,4)?)?;
    // 0x53a00: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x53a04: cmp r2, r8
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(8)?,Shift::Lsl,0,false)?)?;
    // 0x53a08: movhs r0, #1
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x53a0c: ldr r2, [r5], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,true,R::R2,R::new(5)?,4)?)?;
    // 0x53a10: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x53a14: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x53a18: str r2, [r10], #4
    c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R2,R::new(10)?,4)?)?;
    // 0x53a1c: bne 0x53a0c <entry+0xc8>    @ imm = #-0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53a0c,false)?)?;
    // 0x53a20: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x53a24: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(0x53a28)
}

/// Allocate/dispatch the selected codec buffer to the runtime callback
/// selected by the source descriptor's format ID.
#[inline(never)]
fn apply_codec_frame_update(c:&mut Code)->Result<(),String>{
 c.check(0x53a28)?;
    // 0x53a28: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x53a2c: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x53a30: sub sp, sp, #64
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,64)?)?;
    // 0x53a34: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x53a38: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x53a3c: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x53a40: mov r4, r2
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R2))?;
    // 0x53a44: str r0, [r11, #-0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-20)?)?;
    // 0x53a48: ldr r7, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(6)?,4)?)?;
    // 0x53a4c: cmp r7, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4096)?)?;
    // 0x53a50: bne 0x53aa0 <entry+0x15c>   @ imm = #0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53aa0,false)?)?;
    // 0x53a54: ldr r7, [r5, #0x60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(5)?,96)?)?;
    // 0x53a58: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x53a5c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x53a60: bl 0x7e050 <entry+0x2a70c> @ imm = #0x2a5e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x7e050,true)?)?;
    // 0x53a64: ldr r0, [r5, #0x148]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,328)?)?;
    // 0x53a68: sub r6, r11, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,20)?)?;
    // 0x53a6c: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x53a70: str r0, [r11, #-0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-20)?)?;
    // 0x53a74: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x53a78: ldr r5, [r5, #0x74]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(5)?,116)?)?;
    // 0x53a7c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x53a80: bl 0x3e2c8 <entry+0xfffffffffffea984> @ imm = #-0x157c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e2c8,true)?)?;
    // 0x53a84: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x53a88: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x53a8c: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x53a90: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x53a94: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x53a98: bl 0x3ebe8 <entry+0xfffffffffffeb2a4> @ imm = #-0x14eb8
    c.emit(arm32::branch(C::Al,c.at as u32,0x3ebe8,true)?)?;
    // 0x53a9c: b 0x53b18 <entry+0x1d4>   @ imm = #0x74
    c.emit(arm32::branch(C::Al,c.at as u32,0x53b18,false)?)?;
    // 0x53aa0: add r1, r5, #332
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,332)?)?;
    // 0x53aa4: add r0, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::SP,8)?)?;
    // 0x53aa8: mov r2, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,48)?)?;
    // 0x53aac: bl 0x471c <entry+0xfffffffffffb0dd8> @ imm = #-0x4f398
    c.emit(arm32::branch(C::Al,c.at as u32,0x471c,true)?)?;
    // 0x53ab0: cmp r7, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,2)?)?;
    // 0x53ab4: beq 0x53ad4 <entry+0x190>   @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53ad4,false)?)?;
    // 0x53ab8: cmp r7, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8192)?)?;
    // 0x53abc: bne 0x53ae4 <entry+0x1a0>   @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53ae4,false)?)?;
    // 0x53ac0: add r2, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,8)?)?;
    // 0x53ac4: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x53ac8: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x53acc: bl 0x7d6a4 <entry+0x29d60> @ imm = #0x29bd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x7d6a4,true)?)?;
    // 0x53ad0: b 0x53ae4 <entry+0x1a0>   @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x53ae4,false)?)?;
    // 0x53ad4: add r2, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,8)?)?;
    // 0x53ad8: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x53adc: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x53ae0: bl 0x7b7ac <entry+0x27e68> @ imm = #0x27cc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x7b7ac,true)?)?;
    // 0x53ae4: ldr r0, [r5, #0x148]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,328)?)?;
    // 0x53ae8: ldr r1, [sp, #0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,48)?)?;
    // 0x53aec: sub r5, r11, #20
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,20)?)?;
    // 0x53af0: str r0, [r11, #-0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-20)?)?;
    // 0x53af4: ldr r0, [sp, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
    // 0x53af8: sub r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53afc: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x53b00: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x53b04: bl 0x3e2c8 <entry+0xfffffffffffea984> @ imm = #-0x15844
    c.emit(arm32::branch(C::Al,c.at as u32,0x3e2c8,true)?)?;
    // 0x53b08: add r2, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::SP,8)?)?;
    // 0x53b0c: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x53b10: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x53b14: bl 0x3ec90 <entry+0xfffffffffffeb34c> @ imm = #-0x14e8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x3ec90,true)?)?;
    // 0x53b18: ldr r0, [r11, #-0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-20)?)?;
    // 0x53b1c: sub sp, r11, #16
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,16)?)?;
    // 0x53b20: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
    let mut c=Code::new(); normalize_packet_planes(&mut c)?;
    apply_codec_frame_update(&mut c)?;Ok(c.bytes)
}

#[cfg(test)] mod tests {
use super::*;
#[test] fn a32_codec_framing_is_adjacent(){
 let b=fragment().unwrap();assert_eq!(b.len(),END-START);
 assert_eq!(FILE_OFFSET,0x57944);
 assert_eq!(START,0x53944);
 assert_eq!(END,0x53b24);
}
}
