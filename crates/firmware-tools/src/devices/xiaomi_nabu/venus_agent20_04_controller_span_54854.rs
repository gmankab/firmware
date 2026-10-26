// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller runtime resource initialization; fills output slots,
//! opens firmware controller handles, configures codec properties and reports
//! errors via the platform diagnostic callback. PC-relative message pointer
//! is deliberately modeled as one non-instruction word after the epilogue.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x54854;
pub const END:usize=0x54c1c;
pub const FILE_OFFSET:usize=0x4000+START;
const CONTROLLER_INIT_DIAGNOSTIC:u32=0x000e_e470;
struct Code{at:usize,bytes:Vec<u8>}
impl Code {
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,x:usize)->Result<(),String>{
  if self.at!=x{return Err(format!("Venus controller initialization at {:#x} expected {x:#x}",self.at))}Ok(())
 }
 #[inline(never)]fn emit(&mut self,word:u32)->Result<(),String>{
  if !(0x40000..0x68000).contains(&self.at)||self.at&3!=0{return Err("Agent04 reservation violation".into())}
  self.bytes.extend_from_slice(&word.to_le_bytes());self.at+=4;Ok(())
 }
 fn literal(&mut self,cond:C,dst:R,target:usize)->Result<(),String>{
  let offset=i32::try_from(target as i64-self.at as i64-8).map_err(|_|"PC literal offset out of range")?;
  self.emit(arm32::transfer_word(cond,true,dst,R::PC,offset)?)
 }
}

/// Allocate handles and populate several groups of runtime table entries.
/// The source contains real conditional branches and a tail-called logger.
#[inline(never)]fn install_controller_resource_state(c:&mut Code)->Result<(),String>{
 c.check(START)?;
    // 0x54854: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x54858: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x5485c: sub sp, sp, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,4)?)?;
    // 0x54860: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x54864: mov r10, r1
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R1))?;
    // 0x54868: ldr r9, [pc, #0x3a8]        @ 0x12d4 <entry+0x12d4>
    c.literal(C::Al,R::new(9)?,0x54c18)?;
    // 0x5486c: mov r2, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
    // 0x54870: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x54874: mov r8, #48
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,48)?)?;
    // 0x54878: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x5487c: ldr r6, [r4, #0x650]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(4)?,1616)?)?;
    // 0x54880: ldr r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,44)?)?;
    // 0x54884: add r7, r0, #6
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,6)?)?;
    // 0x54888: str r2, [r1, #0x2c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R1,44)?)?;
    // 0x5488c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54890: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x54894: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54898: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f2c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5489c: ldr r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,44)?)?;
    // 0x548a0: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x548a4: str r0, [r1, r8]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x548a8: mov r1, #41
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,41)?)?;
    // 0x548ac: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x548b0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x548b4: ldr r0, [r0, r8]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(8)?,Shift::Lsl,0)?)?;
    // 0x548b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x548bc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x548c0: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x53118
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x548c4: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x548c8: add r8, r8, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,4)?)?;
    // 0x548cc: cmp r5, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x548d0: bne 0x5488c <entry+0xf48>   @ imm = #-0x4c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x5488c,false)?)?;
    // 0x548d4: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x548d8: ldr r8, [pc, #0x338]        @ 0x12d4 <entry+0x12d4>
    c.literal(C::Al,R::new(8)?,0x54c18)?;
    // 0x548dc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x548e0: mov r6, #140
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,140)?)?;
    // 0x548e4: str r7, [r0, #0x78]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,120)?)?;
    // 0x548e8: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x548ec: ldrb r1, [r4, #0x3e0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,992)?)?;
    // 0x548f0: ldr r9, [r4, #0x654]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,1620)?)?;
    // 0x548f4: add r0, r0, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
    // 0x548f8: add r7, r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::R0,2)?)?;
    // 0x548fc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54900: mov r1, #32
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,32)?)?;
    // 0x54904: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x54908: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f334
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x5490c: ldr r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,44)?)?;
    // 0x54910: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x54914: str r0, [r1, r6]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x54918: mov r1, #824
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,824)?)?;
    // 0x5491c: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x54920: ldr r0, [r0, r6]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(6)?,Shift::Lsl,0)?)?;
    // 0x54924: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54928: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x5492c: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x53184
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54930: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x54934: add r6, r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,4)?)?;
    // 0x54938: cmp r5, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x5493c: bne 0x548fc <entry+0xfb8>   @ imm = #-0x48
    c.emit(arm32::branch(C::Ne,c.at as u32,0x548fc,false)?)?;
    // 0x54940: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x54944: ldr r8, [pc, #0x2cc]        @ 0x12d4 <entry+0x12d4>
    c.literal(C::Al,R::new(8)?,0x54c18)?;
    // 0x54948: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x5494c: mov r5, #164
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,164)?)?;
    // 0x54950: str r7, [r0, #0xd4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R0,212)?)?;
    // 0x54954: ldrb r0, [r4, #0x3df]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,991)?)?;
    // 0x54958: ldrb r1, [r4, #0x3e0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,992)?)?;
    // 0x5495c: ldr r9, [r4, #0x658]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,1624)?)?;
    // 0x54960: add r7, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x54964: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54968: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x5496c: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x54970: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f39c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54974: ldr r1, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,44)?)?;
    // 0x54978: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x5497c: str r0, [r1, r5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::R1,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x54980: mov r1, #836
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,836)?)?;
    // 0x54984: ldr r0, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,44)?)?;
    // 0x54988: ldr r0, [r0, r5]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R0,R::new(5)?,Shift::Lsl,0)?)?;
    // 0x5498c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54990: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54994: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x531ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54998: add r0, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(6)?,1)?)?;
    // 0x5499c: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x549a0: add r5, r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,4)?)?;
    // 0x549a4: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x549a8: bne 0x54964 <entry+0x1020>  @ imm = #-0x4c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x54964,false)?)?;
    // 0x549ac: add r0, r4, #812
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,812)?)?;
    // 0x549b0: add r5, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,1024)?)?;
    // 0x549b4: add r0, r4, #756
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,756)?)?;
    // 0x549b8: add r9, r0, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::R0,1024)?)?;
    // 0x549bc: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x549c0: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x549c4: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x549c8: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x549cc: moveq r2, r5
    c.emit(arm32::mov_register(C::Eq,R::R2,R::new(5)?))?;
    // 0x549d0: ldr r6, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,0)?)?;
    // 0x549d4: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x549d8: beq 0x54a0c <entry+0x10c8>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54a0c,false)?)?;
    // 0x549dc: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x549e0: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x549e4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x549e8: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f414
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x549ec: str r0, [r4, #0x858]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2136)?)?;
    // 0x549f0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x549f4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x549f8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x549fc: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f428
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54a00: str r0, [r4, #0x85c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2140)?)?;
    // 0x54a04: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x54a08: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x54a0c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x54a10: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x54a14: moveq r2, r5
    c.emit(arm32::mov_register(C::Eq,R::R2,R::new(5)?))?;
    // 0x54a18: ldr r6, [r2, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,4)?)?;
    // 0x54a1c: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x54a20: ldrne r2, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Ne,true,R::R2,R::new(4)?,664)?)?;
    // 0x54a24: cmpne r2, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x54a28: beq 0x54a84 <entry+0x1140>  @ imm = #0x54
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54a84,false)?)?;
    // 0x54a2c: add r0, r4, #100
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,100)?)?;
    // 0x54a30: str r5, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
    // 0x54a34: add r7, r4, #2144
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,2144)?)?;
    // 0x54a38: mov r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,0)?)?;
    // 0x54a3c: add r5, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::R0,2048)?)?;
    // 0x54a40: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54a44: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54a48: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54a4c: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f478
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54a50: str r0, [r7, r8, lsl #5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::new(8)?,Shift::Lsl,5)?)?;
    // 0x54a54: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54a58: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54a5c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54a60: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f48c
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54a64: str r0, [r5, r8, lsl #5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(5)?,R::new(8)?,Shift::Lsl,5)?)?;
    // 0x54a68: add r8, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::new(8)?,1)?)?;
    // 0x54a6c: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x54a70: cmp r8, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(8)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x54a74: blo 0x54a40 <entry+0x10fc>  @ imm = #-0x3c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x54a40,false)?)?;
    // 0x54a78: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x54a7c: ldr r5, [sp]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,0)?)?;
    // 0x54a80: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x54a84: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x54a88: mov r2, r9
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(9)?))?;
    // 0x54a8c: moveq r2, r5
    c.emit(arm32::mov_register(C::Eq,R::R2,R::new(5)?))?;
    // 0x54a90: ldr r6, [r2, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,8)?)?;
    // 0x54a94: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x54a98: beq 0x54acc <entry+0x1188>  @ imm = #0x2c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54acc,false)?)?;
    // 0x54a9c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54aa0: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54aa4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54aa8: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f4d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54aac: str r0, [r4, #0x868]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2152)?)?;
    // 0x54ab0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54ab4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54ab8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54abc: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f4e8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54ac0: str r0, [r4, #0x86c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2156)?)?;
    // 0x54ac4: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x54ac8: ldrb r1, [r0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,0)?)?;
    // 0x54acc: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x54ad0: moveq r9, r5
    c.emit(arm32::mov_register(C::Eq,R::new(9)?,R::new(5)?))?;
    // 0x54ad4: ldr r6, [r9, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(9)?,12)?)?;
    // 0x54ad8: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x54adc: ldrne r1, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(4)?,664)?)?;
    // 0x54ae0: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x54ae4: beq 0x54b34 <entry+0x11f0>  @ imm = #0x48
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54b34,false)?)?;
    // 0x54ae8: add r0, r4, #116
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,116)?)?;
    // 0x54aec: add r7, r4, #2160
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,2160)?)?;
    // 0x54af0: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x54af4: add r8, r0, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(8)?,R::R0,2048)?)?;
    // 0x54af8: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54afc: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54b00: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54b04: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f530
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54b08: str r0, [r7, r5, lsl #5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(7)?,R::new(5)?,Shift::Lsl,5)?)?;
    // 0x54b0c: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54b10: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54b14: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54b18: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f544
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54b1c: str r0, [r8, r5, lsl #5]
    c.emit(arm32::transfer_word_register(C::Al,false,R::R0,R::new(8)?,R::new(5)?,Shift::Lsl,5)?)?;
    // 0x54b20: add r5, r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x54b24: ldr r0, [r4, #0x298]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,664)?)?;
    // 0x54b28: cmp r5, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x54b2c: blo 0x54af8 <entry+0x11b4>  @ imm = #-0x3c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x54af8,false)?)?;
    // 0x54b30: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x54b34: ldrb r0, [r0, #0x153]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,339)?)?;
    // 0x54b38: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54b3c: beq 0x54c10 <entry+0x12cc>  @ imm = #0xcc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x54c10,false)?)?;
    // 0x54b40: ldr r2, [r4, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1796)?)?;
    // 0x54b44: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54b48: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54b4c: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f578
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54b50: ldr r6, [pc, #0xc0]         @ 0x12d4 <entry+0x12d4>
    c.literal(C::Al,R::new(6)?,0x54c18)?;
    // 0x54b54: mov r1, #118
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,118)?)?;
    // 0x54b58: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54b5c: str r0, [r4, #0x980]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2432)?)?;
    // 0x54b60: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54b64: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x54b68: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54b6c: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x533c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54b70: ldr r2, [r4, #0x708]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1800)?)?;
    // 0x54b74: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54b78: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54b7c: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f5a8
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54b80: mov r1, #123
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,123)?)?;
    // 0x54b84: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54b88: str r0, [r4, #0x984]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2436)?)?;
    // 0x54b8c: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54b90: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54b94: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x54b98: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x533f0
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54b9c: ldr r0, [r4, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,2744)?)?;
    // 0x54ba0: ldrb r0, [r0, #0x154]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,340)?)?;
    // 0x54ba4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x54ba8: blo 0x54c10 <entry+0x12cc>  @ imm = #0x60
    c.emit(arm32::branch(C::Cc,c.at as u32,0x54c10,false)?)?;
    // 0x54bac: ldr r2, [r4, #0x70c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1804)?)?;
    // 0x54bb0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54bb4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54bb8: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f5e4
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54bbc: ldr r6, [pc, #0x54]         @ 0x12d4 <entry+0x12d4>
    c.literal(C::Al,R::new(6)?,0x54c18)?;
    // 0x54bc0: mov r1, #129
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,129)?)?;
    // 0x54bc4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54bc8: str r0, [r4, #0x988]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2440)?)?;
    // 0x54bcc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54bd0: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x54bd4: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54bd8: bl 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x53430
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x54bdc: ldr r2, [r4, #0x710]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1808)?)?;
    // 0x54be0: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x54be4: mov r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,256)?)?;
    // 0x54be8: bl 0x355dc <entry+0xfffffffffffe1c98> @ imm = #-0x1f614
    c.emit(arm32::branch(C::Al,c.at as u32,0x355dc,true)?)?;
    // 0x54bec: mov r1, #134
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,134)?)?;
    // 0x54bf0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x54bf4: str r0, [r4, #0x98c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,2444)?)?;
    // 0x54bf8: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x54bfc: movne r0, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x54c00: orr r1, r1, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,768)?)?;
    // 0x54c04: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x54c08: pop {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x54c0c: b 0x17b0 <entry+0xfffffffffffade6c> @ imm = #-0x53464
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,false)?)?;
    // 0x54c10: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x54c14: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;

 c.check(0x54c18)?;
 c.emit(CONTROLLER_INIT_DIAGNOSTIC)?;
 c.check(END)
}

pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();install_controller_resource_state(&mut c)?;Ok(c.bytes)
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn initializer_and_tail_literal_are_adjacent(){
  let b=fragment().unwrap();assert_eq!(b.len(),END-START);
  assert_eq!(u32::from_le_bytes(b[b.len()-4..].try_into().unwrap()),CONTROLLER_INIT_DIAGNOSTIC);
 }
 #[test]fn compare_reference_if_private_path_provided(){
  let b=fragment().unwrap();
  if let Ok(path)=std::env::var("AGENT04_PRIVATE_VENUS"){
   let orig=std::fs::read(path).unwrap();let reference=&orig[FILE_OFFSET..FILE_OFFSET+b.len()];
   if let Some(i)=b.iter().zip(reference).position(|(a,c)|a!=c){panic!("mismatch at runtime {:#x}: {:02x} != {:02x}",START+i,b[i],reference[i]);}
  }
  if let Ok(path)=std::env::var("AGENT04_EMIT_FRAGMENT"){std::fs::write(path,b).unwrap()}
 }
}
