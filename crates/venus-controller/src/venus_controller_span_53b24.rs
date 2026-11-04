// SPDX-License-Identifier: GPL-3.0-only
//! Codec buffer configuration and checked fixed-point scale arithmetic.
//!
//! Original Venus runtime `[0x53b24,0x54028)` consists of five executable
//! routines plus four explicit PC-referenced literal words. No OEM program
//! byte arrays are embedded; public ARM A32 instructions are emitted from
//! typed register names, conditions, operands and fixed branch targets.
use arm32::{Condition as C, DataOp as Op, Register as R, Shift};
pub const START: usize=0x53b24;
pub const END: usize=0x54028;
pub const FILE_OFFSET:usize=0x4000+START;
const CONTROLLER_TABLE_BASE:u32=0xffff_f807;
const CONTROLLER_FIELD_MASK:u32=0xffff_e01f;
const CODEC_DIAGNOSTIC:u32=0x000e_e028;
const BUFFER_DIAGNOSTIC:u32=0x000e_e028;

/// Public A32 UMULL: low and high halves of 64-bit unsigned product.
fn unsigned_long_multiply(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
    if lo==hi||[lo,hi,rm,rs].contains(&R::PC){return Err("invalid UMULL register operands".into());}
    Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|
       (rs.number()<<8)|rm.number())
}
/// A32 ADC Rd,Rn,#0: explicitly include the CPU carry flag.
fn add_with_carry_zero(cond:C,rd:R,rn:R)->Result<u32,String>{
    if rd==R::PC || rn==R::PC {return Err("invalid ADC-zero register".into());}
    Ok(((cond as u32)<<28)|0x02a0_0000|(rn.number()<<16)|(rd.number()<<12))
}

struct Code {at:usize, bytes:Vec<u8>}
impl Code {
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,at:usize)->Result<(),String>{
    if self.at!=at{return Err(format!("A32 codec helper at {:#x} != {at:#x}",self.at))}Ok(())
 }
 #[inline(never)]fn emit(&mut self,word:u32)->Result<(),String>{
    if !(0x40000..0x68000).contains(&self.at)||self.at%4!=0{return Err("Agent04 emitter out of bounds".into());}
    self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,condition:C,rd:R,target:usize)->Result<(),String>{
    let displacement=i32::try_from(target as i64 - self.at as i64 - 8)
       .map_err(|_|"A32 literal displacement out of range")?;
    self.emit(arm32::transfer_word(condition,true,rd,R::PC,displacement)?)
 }
}

/// Encode standard codec packets, clamp component values, and dispatch
/// associated codec format and DMA callbacks.
#[inline(never)]fn build_frame_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x53b24: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x53b28: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x53b2c: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x53b30: mov r8, r1
    c.emit(arm32::mov_register(C::Al,R::new(8)?,R::R1))?;
    // 0x53b34: mov r2, #36
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,36)?)?;
    // 0x53b38: mov r6, #72
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,72)?)?;
    // 0x53b3c: ldr r1, [r4, #0xd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,212)?)?;
    // 0x53b40: ldr r0, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,216)?)?;
    // 0x53b44: orr r2, r2, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,7168)?)?;
    // 0x53b48: orr r6, r6, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,7168)?)?;
    // 0x53b4c: mov r7, r1
    c.emit(arm32::mov_register(C::Al,R::new(7)?,R::R1))?;
    // 0x53b50: str r2, [r7, r0, lsl #3]!
    c.emit(arm32::transfer_word_register_preindexed_writeback(C::Al,false,R::R2,R::new(7)?,R::R0,Shift::Lsl,3)?)?;
    // 0x53b54: ldrb r0, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1)?)?;
    // 0x53b58: str r0, [r7, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,4)?)?;
    // 0x53b5c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x53b60: ldr r2, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,216)?)?;
    // 0x53b64: bl 0x53e2c <entry+0x4e8>   @ imm = #0x2c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e2c,true)?)?;
    // 0x53b68: mov r1, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,16711680)?)?;
    // 0x53b6c: orr r1, r1, #-268435456
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4026531840)?)?;
    // 0x53b70: and r2, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R2,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53b74: ldr r0, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,4)?)?;
    // 0x53b78: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x53b7c: bne 0x53b9c <entry+0x258>   @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53b9c,false)?)?;
    // 0x53b80: ldr r3, [r8, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2104)?)?;
    // 0x53b84: ldrb r0, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1)?)?;
    // 0x53b88: ldrb r1, [r3, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(3)?,221)?)?;
    // 0x53b8c: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x53b90: cmp r1, #127
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,127)?)?;
    // 0x53b94: movhs r1, #127
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,127)?)?;
    // 0x53b98: b 0x53bcc <entry+0x288>   @ imm = #0x2c
    c.emit(arm32::branch(C::Al,c.at as u32,0x53bcc,false)?)?;
    // 0x53b9c: ldr r3, [r8, #0x838]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(8)?,2104)?)?;
    // 0x53ba0: ldrb r5, [r8, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(5)?,R::new(8)?,348)?)?;
    // 0x53ba4: ldrb r0, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1)?)?;
    // 0x53ba8: ldrb r1, [r3, #0xdd]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(3)?,221)?)?;
    // 0x53bac: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x53bb0: add r1, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x53bb4: beq 0x53bc4 <entry+0x280>   @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53bc4,false)?)?;
    // 0x53bb8: cmp r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,63)?)?;
    // 0x53bbc: movhs r1, #63
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,63)?)?;
    // 0x53bc0: b 0x53bcc <entry+0x288>   @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x53bcc,false)?)?;
    // 0x53bc4: cmp r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,51)?)?;
    // 0x53bc8: movhs r1, #51
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R1,R::R0,51)?)?;
    // 0x53bcc: ldrb r3, [r3, #0xde]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(3)?,222)?)?;
    // 0x53bd0: orr r5, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(5)?,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53bd4: str r6, [r7, #0x8]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,8)?)?;
    // 0x53bd8: sub r2, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x53bdc: mov r3, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,65280)?)?;
    // 0x53be0: bic r2, r2, r2, asr #31
    c.emit(arm32::data_register(C::Al,Op::Bic,R::R2,R::R2,R::R2,Shift::Asr,31,false)?)?;
    // 0x53be4: and r3, r3, r2, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::R2,Shift::Lsl,8,false)?)?;
    // 0x53be8: orr r3, r5, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x53bec: orr r3, r3, #16777216
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,16777216)?)?;
    // 0x53bf0: str r3, [r7, #0xc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(7)?,12)?)?;
    // 0x53bf4: ldrb r3, [r8, #0x215]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,533)?)?;
    // 0x53bf8: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x53bfc: beq 0x53c80 <entry+0x33c>   @ imm = #0x7c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53c80,false)?)?;
    // 0x53c00: sub r3, r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R0,4)?)?;
    // 0x53c04: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x53c08: cmp r3, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x53c0c: blt 0x53c1c <entry+0x2d8>   @ imm = #0x8
    c.emit(arm32::branch(C::Lt,c.at as u32,0x53c1c,false)?)?;
    // 0x53c10: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53c14: mov r6, r1
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R1))?;
    // 0x53c18: movle r6, r3
    c.emit(arm32::mov_register(C::Le,R::new(6)?,R::new(3)?))?;
    // 0x53c1c: cmp r2, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R0,Shift::Lsl,0,false)?)?;
    // 0x53c20: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x53c24: bge 0x53c38 <entry+0x2f4>   @ imm = #0xc
    c.emit(arm32::branch(C::Ge,c.at as u32,0x53c38,false)?)?;
    // 0x53c28: sub r5, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R0,1)?)?;
    // 0x53c2c: mov r3, r1
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R1))?;
    // 0x53c30: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53c34: movle r3, r5
    c.emit(arm32::mov_register(C::Le,R::new(3)?,R::new(5)?))?;
    // 0x53c38: mov r5, #65280
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,65280)?)?;
    // 0x53c3c: and r3, r5, r3, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(5)?,R::new(3)?,Shift::Lsl,8,false)?)?;
    // 0x53c40: mov r5, r2
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R2))?;
    // 0x53c44: orr r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x53c48: add r6, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,2)?)?;
    // 0x53c4c: cmp r6, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x53c50: blt 0x53c60 <entry+0x31c>   @ imm = #0x8
    c.emit(arm32::branch(C::Lt,c.at as u32,0x53c60,false)?)?;
    // 0x53c54: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53c58: mov r5, r1
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R1))?;
    // 0x53c5c: movls r5, r6
    c.emit(arm32::mov_register(C::Ls,R::new(5)?,R::new(6)?))?;
    // 0x53c60: mov r6, #16711680
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,16711680)?)?;
    // 0x53c64: add r0, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5)?)?;
    // 0x53c68: and r6, r6, r5, lsl #16
    c.emit(arm32::data_register(C::Al,Op::And,R::new(6)?,R::new(6)?,R::new(5)?,Shift::Lsl,16,false)?)?;
    // 0x53c6c: cmp r0, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x53c70: orr r3, r3, r6
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::new(3)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x53c74: bge 0x53c8c <entry+0x348>   @ imm = #0x10
    c.emit(arm32::branch(C::Ge,c.at as u32,0x53c8c,false)?)?;
    // 0x53c78: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x53c7c: b 0x53c94 <entry+0x350>   @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x53c94,false)?)?;
    // 0x53c80: orr r1, r0, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R0,Shift::Lsl,8,false)?)?;
    // 0x53c84: orr r3, r1, r0, lsl #16
    c.emit(arm32::data_register(C::Al,Op::Orr,R::new(3)?,R::R1,R::R0,Shift::Lsl,16,false)?)?;
    // 0x53c88: b 0x53c94 <entry+0x350>   @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x53c94,false)?)?;
    // 0x53c8c: cmp r0, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53c90: movhi r0, r1
    c.emit(arm32::mov_register(C::Hi,R::R0,R::R1))?;
    // 0x53c94: mov r1, #92
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,92)?)?;
    // 0x53c98: orr r0, r3, r0, lsl #24
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(3)?,R::R0,Shift::Lsl,24,false)?)?;
    // 0x53c9c: mov r6, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1040)?)?;
    // 0x53ca0: orr r1, r1, #7168
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,7168)?)?;
    // 0x53ca4: orr r6, r6, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,4096)?)?;
    // 0x53ca8: str r1, [r7, #0x10]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(7)?,16)?)?;
    // 0x53cac: str r0, [r7, #0x14]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,20)?)?;
    // 0x53cb0: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x53cb4: ldr r1, [r4, #0xd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,212)?)?;
    // 0x53cb8: ldr r2, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,216)?)?;
    // 0x53cbc: bl 0x53e2c <entry+0x4e8>   @ imm = #0x168
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e2c,true)?)?;
    // 0x53cc0: ldr r1, [pc, #0x158]        @ 0x4dc <entry+0x4dc>
    c.literal(C::Al,R::R1,0x53e20)?;
    // 0x53cc4: and r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53cc8: ldr r1, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,4)?)?;
    // 0x53ccc: cmp r1, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x53cd0: bne 0x53cf8 <entry+0x3b4>   @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53cf8,false)?)?;
    // 0x53cd4: ldrb r1, [r8, #0x15c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,348)?)?;
    // 0x53cd8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x53cdc: beq 0x53cf8 <entry+0x3b4>   @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53cf8,false)?)?;
    // 0x53ce0: ldrb r1, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1)?)?;
    // 0x53ce4: cmp r1, #52
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,52)?)?;
    // 0x53ce8: blo 0x53cf8 <entry+0x3b4>   @ imm = #0x8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x53cf8,false)?)?;
    // 0x53cec: ldr r1, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,396)?)?;
    // 0x53cf0: add r1, r1, #51
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,51)?)?;
    // 0x53cf4: b 0x53d04 <entry+0x3c0>   @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x53d04,false)?)?;
    // 0x53cf8: ldrb r1, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1)?)?;
    // 0x53cfc: ldr r2, [r4, #0x18c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,396)?)?;
    // 0x53d00: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53d04: ldrb r1, [r1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,0)?)?;
    // 0x53d08: str r6, [r7, #0x18]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,24)?)?;
    // 0x53d0c: mov r6, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,20)?)?;
    // 0x53d10: orr r6, r6, #5120
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,5120)?)?;
    // 0x53d14: orr r0, r0, r1, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,3,false)?)?;
    // 0x53d18: str r0, [r7, #0x1c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,28)?)?;
    // 0x53d1c: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x53d20: ldr r1, [r4, #0xd4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,212)?)?;
    // 0x53d24: ldr r2, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,216)?)?;
    // 0x53d28: bl 0x53e2c <entry+0x4e8>   @ imm = #0xfc
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e2c,true)?)?;
    // 0x53d2c: ldrb r1, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1)?)?;
    // 0x53d30: str r6, [r7, #0x20]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::new(7)?,32)?)?;
    // 0x53d34: ldr r2, [pc, #0xe8]         @ 0x4e0 <entry+0x4e0>
    c.literal(C::Al,R::R2,0x53e24)?;
    // 0x53d38: and r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x53d3c: orr r0, r0, r1, lsl #5
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,5,false)?)?;
    // 0x53d40: str r0, [r7, #0x24]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(7)?,36)?)?;
    // 0x53d44: ldr r0, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,216)?)?;
    // 0x53d48: add r0, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,5)?)?;
    // 0x53d4c: str r0, [r4, #0xd8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,216)?)?;
    // 0x53d50: ldr r0, [r8, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,4)?)?;
    // 0x53d54: ldr r1, [r4, #0xe4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,228)?)?;
    // 0x53d58: ldr r2, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,232)?)?;
    // 0x53d5c: add r6, r1, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(6)?,R::R1,R::R2,Shift::Lsl,3,false)?)?;
    // 0x53d60: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x53d64: beq 0x53d84 <entry+0x440>   @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53d84,false)?)?;
    // 0x53d68: cmp r0, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,4096)?)?;
    // 0x53d6c: beq 0x53db0 <entry+0x46c>   @ imm = #0x3c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x53db0,false)?)?;
    // 0x53d70: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x53d74: bne 0x53dfc <entry+0x4b8>   @ imm = #0x80
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53dfc,false)?)?;
    // 0x53d78: mov r5, #3712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,3712)?)?;
    // 0x53d7c: mov r0, #3712
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3712)?)?;
    // 0x53d80: b 0x53d90 <entry+0x44c>   @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x53d90,false)?)?;
    // 0x53d84: mov r5, #628
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,628)?)?;
    // 0x53d88: orr r5, r5, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,3072)?)?;
    // 0x53d8c: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x53d90: bl 0x53e2c <entry+0x4e8>   @ imm = #0x94
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e2c,true)?)?;
    // 0x53d94: ldrb r1, [r4, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1)?)?;
    // 0x53d98: bic r0, r0, #63
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,63)?)?;
    // 0x53d9c: str r5, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(6)?,0)?)?;
    // 0x53da0: and r1, r1, #63
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,63)?)?;
    // 0x53da4: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53da8: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x53dac: b 0x53e10 <entry+0x4cc>   @ imm = #0x5c
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e10,false)?)?;
    // 0x53db0: ldr r0, [r4, #0x8c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,140)?)?;
    // 0x53db4: ldrb r2, [r4, #0x88]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,136)?)?;
    // 0x53db8: ldrd r8, r9, [r4, #128]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::new(8)?,R::new(4)?,128)?)?;
    // 0x53dbc: mov r3, #632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,632)?)?;
    // 0x53dc0: ldr r1, [r4, #0x90]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,144)?)?;
    // 0x53dc4: mov r7, #1792
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1792)?)?;
    // 0x53dc8: orr r3, r3, #3072
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,3072)?)?;
    // 0x53dcc: str r3, [r6]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::new(6)?,0)?)?;
    // 0x53dd0: mov r3, #4096
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4096)?)?;
    // 0x53dd4: and r0, r7, r0, lsl #8
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::new(7)?,R::R0,Shift::Lsl,8,false)?)?;
    // 0x53dd8: and r3, r3, r8, lsl #12
    c.emit(arm32::data_register(C::Al,Op::And,R::new(3)?,R::new(3)?,R::new(8)?,Shift::Lsl,12,false)?)?;
    // 0x53ddc: orr r0, r0, r2
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,0,false)?)?;
    // 0x53de0: mov r2, #253952
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,253952)?)?;
    // 0x53de4: orr r0, r0, r3
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x53de8: and r1, r2, r1, lsl #13
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::R1,Shift::Lsl,13,false)?)?;
    // 0x53dec: orr r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x53df0: orr r0, r0, r9, lsl #18
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(9)?,Shift::Lsl,18,false)?)?;
    // 0x53df4: str r0, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(6)?,4)?)?;
    // 0x53df8: b 0x53e10 <entry+0x4cc>   @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e10,false)?)?;
    // 0x53dfc: ldr r2, [pc, #0x24]         @ 0x4e4 <entry+0x4e4>
    c.literal(C::Al,R::R2,0x53e28)?;
    // 0x53e00: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x53e04: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x53e08: orr r1, r1, #15872
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15872)?)?;
    // 0x53e0c: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x52664
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x53e10: ldr r0, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,232)?)?;
    // 0x53e14: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x53e18: str r0, [r4, #0xe8]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,232)?)?;
    // 0x53e1c: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(0x53e20)?;
 for value in [CONTROLLER_TABLE_BASE, CONTROLLER_FIELD_MASK, CODEC_DIAGNOSTIC] {c.emit(value)?;}
 c.check(0x53e2c)
}

/// Scan a table from its last record, returning a matched pointer or log it.
#[inline(never)]fn find_codec_record(c:&mut Code)->Result<(),String>{
 c.check(0x53e2c)?;
    // 0x53e2c: push {r4, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x53e30: add r11, sp, #8
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,8)?)?;
    // 0x53e34: add r1, r1, r2, lsl #3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,3,false)?)?;
    // 0x53e38: add r3, r1, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R1,4)?)?;
    // 0x53e3c: sub r1, r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R2,1)?)?;
    // 0x53e40: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x53e44: blt 0x53e68 <entry+0x524>   @ imm = #0x1c
    c.emit(arm32::branch(C::Lt,c.at as u32,0x53e68,false)?)?;
    // 0x53e48: sub r2, r3, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(3)?,8)?)?;
    // 0x53e4c: ldr r3, [r3, #-0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(3)?,-12)?)?;
    // 0x53e50: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x53e54: cmp r3, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x53e58: mov r3, r2
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R2))?;
    // 0x53e5c: bne 0x53e40 <entry+0x4fc>   @ imm = #-0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x53e40,false)?)?;
    // 0x53e60: ldr r4, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::R2,0)?)?;
    // 0x53e64: b 0x53e80 <entry+0x53c>   @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x53e80,false)?)?;
    // 0x53e68: ldr r2, [pc, #0x18]         @ 0x544 <entry+0x544>
    c.literal(C::Al,R::R2,0x53e88)?;
    // 0x53e6c: mov r1, #59
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,59)?)?;
    // 0x53e70: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x53e74: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x53e78: orr r1, r1, #15616
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,15616)?)?;
    // 0x53e7c: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x526d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x53e80: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x53e84: pop {r4, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(0x53e88)?;
 c.emit(BUFFER_DIAGNOSTIC)?;
 c.check(0x53e8c)
}

/// Set minimum 16-bit plane sizes and return signed step count.
#[inline(never)]fn normalize_halfword_lengths(c:&mut Code)->Result<(),String>{
 c.check(0x53e8c)?;
    // 0x53e8c: mov r12, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,0)?)?;
    // 0x53e90: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x53e94: blt 0x53eb4 <entry+0x570>   @ imm = #0x18
    c.emit(arm32::branch(C::Lt,c.at as u32,0x53eb4,false)?)?;
    // 0x53e98: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x53e9c: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x53ea0: mov lr, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,32768)?)?;
    // 0x53ea4: tst lr, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Tst,R::R0,R::LR,R::R0,Shift::Lsl,1,false)?)?;
    // 0x53ea8: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x53eac: moveq r12, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R12,R::R0,1)?)?;
    // 0x53eb0: orreq r12, r12, r0, lsl #1
    c.emit(arm32::data_register(C::Eq,Op::Orr,R::R12,R::R12,R::R0,Shift::Lsl,1,false)?)?;
    // 0x53eb4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x53eb8: strh r12, [r2]
    c.emit(arm32::store_halfword(C::Al,R::R12,R::R2,0)?)?;
    // 0x53ebc: add r0, r0, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // 0x53ec0: strh r0, [r3]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(3)?,0)?)?;
    // 0x53ec4: mov r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x53ec8: add r0, r0, r1, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,1,false)?)?;
    // 0x53ecc: sub r0, r0, r12
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::R12,Shift::Lsl,0,false)?)?;
    // 0x53ed0: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x53ed4: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x53ed8: bx lr
    c.emit(arm32::bx(C::Al,R::LR))?;

 c.check(0x53edc)
}

/// Fixed-point macroblock dimensions using unsigned long multiply and MLA.
#[inline(never)]fn transform_fixed_point_dimensions(c:&mut Code)->Result<(),String>{
 c.check(0x53edc)?;
    // 0x53edc: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x53ee0: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x53ee4: ldr lr, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,8)?)?;
    // 0x53ee8: mov r12, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R12,R::R0,4)?)?;
    // 0x53eec: cmp lr, #32
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::LR,32)?)?;
    // 0x53ef0: ldr lr, [r11, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::LR,R::R11,12)?)?;
    // 0x53ef4: moveq r12, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R12,R::R0,5)?)?;
    // 0x53ef8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x53efc: blt 0x53f4c <entry+0x608>   @ imm = #0x48
    c.emit(arm32::branch(C::Lt,c.at as u32,0x53f4c,false)?)?;
    // 0x53f00: umull r5, r6, lr, r0
    c.emit(unsigned_long_multiply(C::Al,R::new(5)?,R::new(6)?,R::LR,R::R0)?)?;
    // 0x53f04: asr r0, r0, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,31)?)?;
    // 0x53f08: ldr r8, [r11, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::R11,16)?)?;
    // 0x53f0c: mla r7, lr, r0, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::new(7)?,R::LR,R::R0,R::new(6)?)?)?;
    // 0x53f10: and r0, r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::R12,255)?)?;
    // 0x53f14: sub r4, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R0,32)?)?;
    // 0x53f18: asr r6, r7, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(6)?,R::R0,R::new(7)?,Shift::Asr,R::R0,false)?)?;
    // 0x53f1c: lsr r5, r5, r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::new(5)?,R::R0,R::new(5)?,Shift::Lsr,R::R0,false)?)?;
    // 0x53f20: rsb r0, r0, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R0,R::R0,32)?)?;
    // 0x53f24: cmp r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(4)?,0)?)?;
    // 0x53f28: orr r0, r5, r7, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,R::R0,false)?)?;
    // 0x53f2c: asrge r0, r7, r4
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R0,R::R0,R::new(7)?,Shift::Asr,R::new(4)?,false)?)?;
    // 0x53f30: asrge r6, r7, #31
    c.emit(arm32::mov_shifted_register(C::Ge,R::new(6)?,R::new(7)?,Shift::Asr,31)?)?;
    // 0x53f34: subs r0, r0, r8
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(8)?,Shift::Lsl,0,true)?)?;
    // 0x53f38: sbc r4, r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Sbc,R::new(4)?,R::new(6)?,0)?)?;
    // 0x53f3c: lsr r0, r0, #18
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,18)?)?;
    // 0x53f40: orr r0, r0, r4, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,14,false)?)?;
    // 0x53f44: add r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3)?)?;
    // 0x53f48: b 0x53f50 <entry+0x60c>   @ imm = #0x0
    c.emit(arm32::branch(C::Al,c.at as u32,0x53f50,false)?)?;
    // 0x53f4c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x53f50: strh r0, [r2]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R2,0)?)?;
    // 0x53f54: umull r0, r2, lr, r1
    c.emit(unsigned_long_multiply(C::Al,R::R0,R::R2,R::LR,R::R1)?)?;
    // 0x53f58: asr r1, r1, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,31)?)?;
    // 0x53f5c: mla r4, lr, r1, r2
    c.emit(arm32::multiply_accumulate(C::Al,R::new(4)?,R::LR,R::R1,R::R2)?)?;
    // 0x53f60: and r1, r12, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R12,255)?)?;
    // 0x53f64: sub r7, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R1,32)?)?;
    // 0x53f68: asr r2, r4, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R2,R::R0,R::new(4)?,Shift::Asr,R::R1,false)?)?;
    // 0x53f6c: lsr r0, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::R1,false)?)?;
    // 0x53f70: rsb r1, r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R1,R::R1,32)?)?;
    // 0x53f74: cmp r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,0)?)?;
    // 0x53f78: orr r0, r0, r4, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Orr,R::R0,R::R0,R::new(4)?,Shift::Lsl,R::R1,false)?)?;
    // 0x53f7c: asrge r0, r4, r7
    c.emit(arm32::data_register_shifted_by_register(C::Ge,Op::Mov,R::R0,R::R0,R::new(4)?,Shift::Asr,R::new(7)?,false)?)?;
    // 0x53f80: asrge r2, r4, #31
    c.emit(arm32::mov_shifted_register(C::Ge,R::R2,R::new(4)?,Shift::Asr,31)?)?;
    // 0x53f84: lsr r0, r0, #18
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsr,18)?)?;
    // 0x53f88: orr r0, r0, r2, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::R2,Shift::Lsl,14,false)?)?;
    // 0x53f8c: add r0, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,2)?)?;
    // 0x53f90: strh r0, [r3]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::new(3)?,0)?)?;
    // 0x53f94: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(0x53f98)
}

/// Carry-aware signed geometry checks and 16-bit output production.
#[inline(never)]fn finish_fixed_point_dimensions(c:&mut Code)->Result<(),String>{
 c.check(0x53f98)?;
    // 0x53f98: push {r4, r5, r6, r7, r8, r9, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x53f9c: add r11, sp, #24
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,24)?)?;
    // 0x53fa0: ldr r12, [r11, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R12,R::R11,8)?)?;
    // 0x53fa4: mov lr, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,0)?)?;
    // 0x53fa8: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x53fac: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x53fb0: blt 0x53fe8 <entry+0x6a4>   @ imm = #0x30
    c.emit(arm32::branch(C::Lt,c.at as u32,0x53fe8,false)?)?;
    // 0x53fb4: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x53fb8: adds r4, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::new(4)?,R::R2,1,true)?)?;
    // 0x53fbc: umull r8, r6, r4, r0
    c.emit(unsigned_long_multiply(C::Al,R::new(8)?,R::new(6)?,R::new(4)?,R::R0)?)?;
    // 0x53fc0: asr r7, r0, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(7)?,R::R0,Shift::Asr,31)?)?;
    // 0x53fc4: mla r5, r4, r7, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::new(5)?,R::new(4)?,R::new(7)?,R::new(6)?)?)?;
    // 0x53fc8: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x53fcc: adc r4, r4, #0
    c.emit(add_with_carry_zero(C::Al,R::new(4)?,R::new(4)?)?)?;
    // 0x53fd0: mla r6, r4, r0, r5
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::new(4)?,R::R0,R::new(5)?)?)?;
    // 0x53fd4: lsr r0, r8, #18
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::new(8)?,Shift::Lsr,18)?)?;
    // 0x53fd8: orr r0, r0, r6, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(6)?,Shift::Lsl,14,false)?)?;
    // 0x53fdc: add r4, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::R0,3)?)?;
    // 0x53fe0: tst r4, #32768
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(4)?,32768)?)?;
    // 0x53fe4: movne r4, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x53fe8: adds r0, r2, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Add,R::R0,R::R2,1,true)?)?;
    // 0x53fec: strh r4, [r3]
    c.emit(arm32::store_halfword(C::Al,R::new(4)?,R::new(3)?,0)?)?;
    // 0x53ff0: asr r5, r1, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::R1,Shift::Asr,31)?)?;
    // 0x53ff4: umull r2, r3, r0, r1
    c.emit(unsigned_long_multiply(C::Al,R::R2,R::new(3)?,R::R0,R::R1)?)?;
    // 0x53ff8: mla r6, r0, r5, r3
    c.emit(arm32::multiply_accumulate(C::Al,R::new(6)?,R::R0,R::new(5)?,R::new(3)?)?)?;
    // 0x53ffc: adc r0, lr, #0
    c.emit(add_with_carry_zero(C::Al,R::R0,R::LR)?)?;
    // 0x54000: mla r3, r0, r1, r6
    c.emit(arm32::multiply_accumulate(C::Al,R::new(3)?,R::R0,R::R1,R::new(6)?)?)?;
    // 0x54004: lsr r0, r2, #18
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Lsr,18)?)?;
    // 0x54008: orr r0, r0, r3, lsl #14
    c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R0,R::new(3)?,Shift::Lsl,14,false)?)?;
    // 0x5400c: add r1, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R0,2)?)?;
    // 0x54010: add r0, r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,3)?)?;
    // 0x54014: sub r0, r0, r4
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R0,R::new(4)?,Shift::Lsl,0,false)?)?;
    // 0x54018: strh r1, [r12]
    c.emit(arm32::store_halfword(C::Al,R::R1,R::R12,0)?)?;
    // 0x5401c: lsl r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
    // 0x54020: asr r0, r0, #16
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
    // 0x54024: pop {r4, r5, r6, r7, r8, r9, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();build_frame_packet(&mut c)?;find_codec_record(&mut c)?;
 normalize_halfword_lengths(&mut c)?;transform_fixed_point_dimensions(&mut c)?;
 finish_fixed_point_dimensions(&mut c)?;Ok(c.bytes)
}
#[cfg(test)] mod tests {
 use super::*;
 #[test]fn five_functions_and_two_literal_pools_are_adjacent(){
   let b=fragment().unwrap();assert_eq!(b.len(),END-START);
   for (address,value) in [(0x53e20,CONTROLLER_TABLE_BASE),(0x53e24,CONTROLLER_FIELD_MASK),
        (0x53e28,CODEC_DIAGNOSTIC),(0x53e88,BUFFER_DIAGNOSTIC)] {
      assert_eq!(u32::from_le_bytes(b[address-START..address-START+4].try_into().unwrap()),value);
   }
 }
 #[test]fn compares_private_reference_when_available(){
   let b=fragment().unwrap();
   if let Ok(reference)=std::env::var("NABU_VENUS_REFERENCE").or_else(|_| std::env::var("AGENT04_PRIVATE_VENUS")) {
     let original=std::fs::read(reference).unwrap();
     let ref_bytes=&original[FILE_OFFSET..FILE_OFFSET+b.len()];
     if let Some(i)=b.iter().zip(ref_bytes).position(|(a,c)|a!=c) {
       panic!("A32 mismatch at 0x{:x}, got {:02x}, reference {:02x}",START+i,b[i],ref_bytes[i]);
     }
   }
   if let Ok(output)=std::env::var("AGENT04_EMIT_FRAGMENT") {
     std::fs::write(output,b).unwrap();
   }
 }
 #[test]fn signed_carry_and_umull_are_checked(){
   assert_eq!(add_with_carry_zero(C::Al,R::R0,R::LR).unwrap(),0xe2ae_0000);
   assert!(unsigned_long_multiply(C::Al,R::R0,R::R0,R::R1,R::R2).is_err());
 }
}
