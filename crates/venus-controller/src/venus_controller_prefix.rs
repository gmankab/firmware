// SPDX-License-Identifier: GPL-3.0-only
//! Venus controller function continuation at the start of Agent 20-04's
//! reserved runtime interval. This 2,604-byte region begins *inside* the
//! caller function; its actual prologue occurs before runtime 0x40000 and
//! belongs to the preceding agent. It ends at a real POP-to-PC return and
//! has six diagnostic/format literal words after that return. One additional
//! diagnostic word is branch-skipped at 0x405bc.
//!
//! Every executable A32 instruction is re-encoded from named public ARM
//! architecture operands. The seven non-executable words are explicitly
//! identified metadata, not opaque copy-from-reference bytes. Byte identity
//! is distinct from independently proven C-level semantics or OEM signing.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};

pub const START:usize=0x40000;
pub const END:usize=0x40a2c;
pub const FILE_OFFSET:usize=START+0x4000;
pub const INSTRUCTION_COUNT:usize=644;
pub const DATA_WORD_COUNT:usize=7;
// These constants are identifiable from PC-relative reads or branch skips.
const DIAGNOSTIC_WMFW:u32=0x000e_e028;
const FORMAT_DIAGNOSTIC:u32=0x000e_e028;
const CODEC_CHECK_CONTEXT:u32=0x000e_e45a;
const DIMENSION_BIT_MASK:u32=0x0003_ffff;
const CODEC_PARAMETER_CONTEXT:u32=0x000e_e41c;
const CODEC_BUFFER_ERROR:u32=0x000e_e3f0;
const CODEC_CHANNEL_NOTICE:u32=0x000e_e408;

struct Code{at:usize,bytes:Vec<u8>}
impl Code {
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,want:usize)->Result<(),String>{
  if self.at!=want{return Err(format!("A32 prefix expected 0x{want:x}, reached 0x{:x}",self.at));}
  Ok(())
 }
 fn emit(&mut self,w:u32)->Result<(),String>{
  if self.at<START||self.at>=END||self.at&3!=0 {return Err(format!("Agent04 prefix address out of range 0x{:x}",self.at));}
  self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
}
/// Named register-offset halfword transfer, not a vendor opcode word.
fn typed_halfword_register(cond:C,load:bool,rd:R,base:R,index:R)->Result<u32,String>{
 if [rd,base,index].contains(&R::PC){return Err("unsafe A32 halfword register index".into());}
 Ok(((cond as u32)<<28)|0x0180_00b0|((load as u32)<<20)
    |(base.number()<<16)|(rd.number()<<12)|index.number())
}
fn typed_signed_halfword_register(cond:C,rd:R,base:R,index:R)->Result<u32,String>{
 if [rd,base,index].contains(&R::PC){return Err("unsafe A32 signed halfword register index".into());}
 Ok(((cond as u32)<<28)|0x0190_00f0|(base.number()<<16)|(rd.number()<<12)|index.number())
}
fn typed_halfword_postindex(cond:C,load:bool,rd:R,base:R,offset:i32)->Result<u32,String>{
 if [rd,base].contains(&R::PC)||rd==base||offset.unsigned_abs()>255{return Err("unsafe A32 halfword postindexed".into());}
 Ok(arm32::store_halfword_postindexed(cond,rd,base,offset)?|((load as u32)<<20))
}
fn typed_signed_halfword_postindex(cond:C,rd:R,base:R,offset:i32)->Result<u32,String>{
 if [rd,base].contains(&R::PC)||rd==base||offset.unsigned_abs()>255{return Err("unsafe A32 signed halfword postindexed".into());}
 Ok(arm32::load_signed_halfword(cond,rd,base,offset)?&!(1<<24))
}

/// Continue controller setup, serialize primary + secondary codec records,
/// enumerate 8 controller channels and post the active stream to firmware.
#[inline(never)]
fn construct_controller_prefix(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // 0x40000: ldr r0, [sp, #0x50]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,80)?)?;
 // 0x40004: lsr r0, r10, r0
 c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::new(10)?,Shift::Lsr,R::R0,false)?)?;
 // 0x40008: bl 0x40a2c <entry+0xa2c>   @ imm = #0xa1c
 c.emit(arm32::branch(C::Al,c.at as u32,0x40a2c,true)?)?;
 // 0x4000c: mov r5, r0
 c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
 // 0x40010: ldr r0, [sp, #0x34]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,52)?)?;
 // 0x40014: ldr r1, [r8, #0x2d0]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,720)?)?;
 // 0x40018: ldr r2, [sp, #0x54]
 c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,84)?)?;
 // 0x4001c: ldr r3, [sp, #0x58]
 c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,88)?)?;
 // 0x40020: str r0, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,0)?)?;
 // 0x40024: ldr r0, [sp, #0x40]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,64)?)?;
 // 0x40028: str r0, [sp, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
 // 0x4002c: ldr r0, [sp, #0x2c]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,44)?)?;
 // 0x40030: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x40034: ldr r0, [sp, #0x60]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
 // 0x40038: lsr r0, r0, r2
 c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::R2,false)?)?;
 // 0x4003c: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x40040: bl 0x40a2c <entry+0xa2c>   @ imm = #0x9e4
 c.emit(arm32::branch(C::Al,c.at as u32,0x40a2c,true)?)?;
 // 0x40044: ldr r1, [r8, #0x30c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,780)?)?;
 // 0x40048: mov r4, #86
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,86)?)?;
 // 0x4004c: orr r4, r4, #256
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(4)?,R::new(4)?,256)?)?;
 // 0x40050: and r1, r1, #15
 c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::R1,15)?)?;
 // 0x40054: sub r1, r1, #1
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
 // 0x40058: cmp r1, #4
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,4)?)?;
 // 0x4005c: mov r1, #480
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,480)?)?;
 // 0x40060: andhi r0, r0, #2
 c.emit(arm32::data_immediate(C::Hi,Op::And,R::R0,R::R0,2)?)?;
 // 0x40064: andhi r5, r5, #2
 c.emit(arm32::data_immediate(C::Hi,Op::And,R::new(5)?,R::new(5)?,2)?)?;
 // 0x40068: orr r0, r5, r0, lsl #2
 c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::new(5)?,R::R0,Shift::Lsl,2,false)?)?;
 // 0x4006c: and r0, r1, r0, lsl #5
 c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R1,R::R0,Shift::Lsl,5,false)?)?;
 // 0x40070: ldr r1, [sp, #0x18]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,24)?)?;
 // 0x40074: bic r1, r1, #480
 c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R1,R::R1,480)?)?;
 // 0x40078: orr r1, r0, r1
 c.emit(arm32::data_register(C::Al,Op::Orr,R::R1,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x4007c: ldr r0, [r9]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
 // 0x40080: tst r0, #31
 c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
 // 0x40084: ldreq r2, [r0]
 c.emit(arm32::transfer_word(C::Eq,true,R::R2,R::R0,0)?)?;
 // 0x40088: andeq r2, r2, #1
 c.emit(arm32::data_immediate(C::Eq,Op::And,R::R2,R::R2,1)?)?;
 // 0x4008c: streq r2, [r0]
 c.emit(arm32::transfer_word(C::Eq,false,R::R2,R::R0,0)?)?;
 // 0x40090: mov r2, #20
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
 // 0x40094: orr r2, r2, #10240
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,10240)?)?;
 // 0x40098: str r2, [r0]
 c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,0)?)?;
 // 0x4009c: str r1, [r0, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,4)?)?;
 // 0x400a0: add r0, r0, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
 // 0x400a4: ldr r1, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
 // 0x400a8: add r1, r1, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
 // 0x400ac: str r1, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
 // 0x400b0: str r0, [r9]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
 // 0x400b4: ldr r0, [r8, #0xab8]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
 // 0x400b8: ldrsh r0, [r0, r4]
 c.emit(typed_signed_halfword_register(C::Al,R::R0,R::R0,R::new(4)?)?)?;
 // 0x400bc: cmp r0, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
 // 0x400c0: blt 0x400e0 <entry+0xe0>    @ imm = #0x18
 c.emit(arm32::branch(C::Lt,c.at as u32,0x400e0,false)?)?;
 // 0x400c4: ldr r2, [pc, #0x948]        @ 0xa14 <entry+0xa14>
 c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2376)?)?;
 // 0x400c8: mov r1, #804
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,804)?)?;
 // 0x400cc: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x400d0: orr r1, r1, #5120
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
 // 0x400d4: bl 0x17b0 <entry+0xfffffffffffc17b0> @ imm = #-0x3e92c
 c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
 // 0x400d8: ldr r0, [r8, #0xab8]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
 // 0x400dc: ldrh r0, [r0, r4]
 c.emit(typed_halfword_register(C::Al,true,R::R0,R::R0,R::new(4)?)?)?;
 // 0x400e0: ldr r1, [pc, #0x938]        @ 0xa20 <entry+0xa20>
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,2360)?)?;
 // 0x400e4: lsl r0, r0, #16
 c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
 // 0x400e8: mov r9, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
 // 0x400ec: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x400f0: asr r0, r0, #16
 c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
 // 0x400f4: ldr r5, [r1, r0, lsl #2]
 c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R1,R::R0,Shift::Lsl,2)?)?;
 // 0x400f8: ldr r0, [r5, #0xf0]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,240)?)?;
 // 0x400fc: ldr r1, [r5, #0x10c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,268)?)?;
 // 0x40100: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x40104: bne 0x40144 <entry+0x144>   @ imm = #0x38
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40144,false)?)?;
 // 0x40108: ldr r0, [r5, #0xf4]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,244)?)?;
 // 0x4010c: ldr r1, [r5, #0x108]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,264)?)?;
 // 0x40110: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40114: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x40118: bne 0x40144 <entry+0x144>   @ imm = #0x24
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40144,false)?)?;
 // 0x4011c: ldr r0, [r5, #0xf8]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,248)?)?;
 // 0x40120: ldr r1, [r5, #0x104]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,260)?)?;
 // 0x40124: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40128: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x4012c: bne 0x40144 <entry+0x144>   @ imm = #0x10
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40144,false)?)?;
 // 0x40130: ldr r0, [r5, #0xfc]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,252)?)?;
 // 0x40134: ldr r1, [r5, #0x100]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,256)?)?;
 // 0x40138: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x4013c: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x40140: moveq r3, #1
 c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
 // 0x40144: ldr r0, [sp, #0x3c]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,60)?)?;
 // 0x40148: ldr r1, [sp, #0x38]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,56)?)?;
 // 0x4014c: cmp r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
 // 0x40150: mov r10, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,255)?)?;
 // 0x40154: sub r6, r11, #92
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,92)?)?;
 // 0x40158: mov r7, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
 // 0x4015c: str r3, [sp, #0x54]
 c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,84)?)?;
 // 0x40160: strh r9, [r11, #-56]
 c.emit(arm32::store_halfword(C::Al,R::new(9)?,R::R11,-56)?)?;
 // 0x40164: str r5, [sp, #0x5c]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
 // 0x40168: orr r10, r10, #256
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,256)?)?;
 // 0x4016c: orr r0, r1, r0, lsr #1
 c.emit(arm32::data_register(C::Al,Op::Orr,R::R0,R::R1,R::R0,Shift::Lsr,1,false)?)?;
 // 0x40170: str r0, [sp, #0x60]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,96)?)?;
 // 0x40174: mov r0, r3
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
 // 0x40178: movne r0, #8
 c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,8)?)?;
 // 0x4017c: str r0, [r11, #-0x5c]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-92)?)?;
 // 0x40180: mov r0, #4
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
 // 0x40184: strh r0, [r11, #-104]
 c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-104)?)?;
 // 0x40188: mov r0, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
 // 0x4018c: sub r4, r11, #56
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,56)?)?;
 // 0x40190: sub r8, r11, #104
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,104)?)?;
 // 0x40194: add r1, r5, #12
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,12)?)?;
 // 0x40198: stm sp, {r0, r7, r10}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x4019c: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x401a0: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x401a4: mov r3, r8
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
 // 0x401a8: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11760
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x401ac: mov r0, #10
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
 // 0x401b0: add r1, r5, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,8)?)?;
 // 0x401b4: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x401b8: mov r3, r8
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
 // 0x401bc: stm sp, {r0, r7}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x401c0: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x401c4: orr r0, r0, #768
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
 // 0x401c8: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x401cc: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x401d0: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11788
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x401d4: mov r0, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
 // 0x401d8: mov r1, r5
 c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
 // 0x401dc: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x401e0: mov r3, r8
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
 // 0x401e4: stm sp, {r0, r7, r10}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(7)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x401e8: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x401ec: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x117a4
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x401f0: add r9, r9, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(9)?,R::new(9)?,1)?)?;
 // 0x401f4: add r5, r5, #32
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,32)?)?;
 // 0x401f8: cmp r9, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,8)?)?;
 // 0x401fc: bne 0x40188 <entry+0x188>   @ imm = #-0x7c
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40188,false)?)?;
 // 0x40200: ldr r1, [sp, #0x54]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
 // 0x40204: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x40208: sub r5, r11, #56
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,56)?)?;
 // 0x4020c: sub r7, r11, #104
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,104)?)?;
 // 0x40210: mov r9, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,9)?)?;
 // 0x40214: mov r8, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
 // 0x40218: orr r0, r0, #256
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
 // 0x4021c: mov r2, r5
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
 // 0x40220: mov r3, r7
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
 // 0x40224: str r9, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
 // 0x40228: str r8, [sp, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,4)?)?;
 // 0x4022c: mov r10, r0
 c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
 // 0x40230: mov r0, #256
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
 // 0x40234: str r10, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,8)?)?;
 // 0x40238: cmp r1, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
 // 0x4023c: ldr r1, [sp, #0x5c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
 // 0x40240: movne r0, #240
 c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,240)?)?;
 // 0x40244: add r4, r1, r0
 c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
 // 0x40248: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x4024c: add r1, r4, #12
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,12)?)?;
 // 0x40250: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11808
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x40254: mov r0, #10
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
 // 0x40258: add r1, r4, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
 // 0x4025c: mov r2, r5
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
 // 0x40260: mov r3, r7
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
 // 0x40264: stm sp, {r0, r8}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x40268: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x4026c: orr r0, r0, #768
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
 // 0x40270: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x40274: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x40278: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11830
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x4027c: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x40280: mov r1, r4
 c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
 // 0x40284: mov r2, r5
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
 // 0x40288: mov r3, r7
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
 // 0x4028c: str r9, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
 // 0x40290: stmib sp, {r8, r10}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(8)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x40294: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x1184c
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 c.check(0x40298)
}

/// Continue the original controller routine at 0x40298; this is not a separate callable entry.
#[inline(never)]
fn controller_prefix_part_1(c:&mut Code)->Result<(),String>{
 c.check(0x40298)?;
 // 0x40298: ldr r4, [sp, #0x64]
 c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::SP,100)?)?;
 // 0x4029c: ldr r8, [sp, #0x68]
 c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
 // 0x402a0: mov r1, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
 // 0x402a4: mov r2, #10304
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,10304)?)?;
 // 0x402a8: ldr r0, [r4]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
 // 0x402ac: mov r3, r0
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
 // 0x402b0: tst r3, #31
 c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
 // 0x402b4: ldr r7, [r6], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(7)?,R::new(6)?,4)?)?;
 // 0x402b8: add r1, r1, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
 // 0x402bc: ldreq r5, [r3]
 c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::new(3)?,0)?)?;
 // 0x402c0: andeq r5, r5, #1
 c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
 // 0x402c4: streq r5, [r3]
 c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::new(3)?,0)?)?;
 // 0x402c8: stm r3, {r2, r7}
 c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R2.bit() | R::new(7)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x402cc: add r2, r2, #4
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
 // 0x402d0: add r3, r3, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,8)?)?;
 // 0x402d4: cmp r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
 // 0x402d8: ldr r7, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::R11,-32)?)?;
 // 0x402dc: add r7, r7, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
 // 0x402e0: str r7, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::R11,-32)?)?;
 // 0x402e4: bne 0x402b0 <entry+0x2b0>   @ imm = #-0x3c
 c.emit(arm32::branch(C::Ne,c.at as u32,0x402b0,false)?)?;
 // 0x402e8: add r0, r0, #64
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
 // 0x402ec: str r0, [r4]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,0)?)?;
 // 0x402f0: mov r4, #344
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,344)?)?;
 // 0x402f4: ldr r0, [r8, #0xab8]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
 // 0x402f8: ldrsh r0, [r0, r4]
 c.emit(typed_signed_halfword_register(C::Al,R::R0,R::R0,R::new(4)?)?)?;
 // 0x402fc: cmp r0, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
 // 0x40300: blt 0x40320 <entry+0x320>   @ imm = #0x18
 c.emit(arm32::branch(C::Lt,c.at as u32,0x40320,false)?)?;
 // 0x40304: ldr r2, [pc, #0x708]        @ 0xa14 <entry+0xa14>
 c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1800)?)?;
 // 0x40308: mov r1, #844
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,844)?)?;
 // 0x4030c: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x40310: orr r1, r1, #5120
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5120)?)?;
 // 0x40314: bl 0x17b0 <entry+0xfffffffffffc17b0> @ imm = #-0x3eb6c
 c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
 // 0x40318: ldr r0, [r8, #0xab8]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2744)?)?;
 // 0x4031c: ldrh r0, [r0, r4]
 c.emit(typed_halfword_register(C::Al,true,R::R0,R::R0,R::new(4)?)?)?;
 // 0x40320: ldr r1, [pc, #0x6f8]        @ 0xa20 <entry+0xa20>
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1784)?)?;
 // 0x40324: lsl r0, r0, #16
 c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Lsl,16)?)?;
 // 0x40328: mov r7, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
 // 0x4032c: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40330: asr r0, r0, #16
 c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,16)?)?;
 // 0x40334: ldr r5, [r1, r0, lsl #2]
 c.emit(arm32::transfer_word_register(C::Al,true,R::new(5)?,R::R1,R::R0,Shift::Lsl,2)?)?;
 // 0x40338: ldr r0, [r5, #0xf0]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,240)?)?;
 // 0x4033c: ldr r1, [r5, #0x10c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,268)?)?;
 // 0x40340: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x40344: bne 0x40384 <entry+0x384>   @ imm = #0x38
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40384,false)?)?;
 // 0x40348: ldr r0, [r5, #0xf4]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,244)?)?;
 // 0x4034c: ldr r1, [r5, #0x108]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,264)?)?;
 // 0x40350: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40354: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x40358: bne 0x40384 <entry+0x384>   @ imm = #0x24
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40384,false)?)?;
 // 0x4035c: ldr r0, [r5, #0xf8]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,248)?)?;
 // 0x40360: ldr r1, [r5, #0x104]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,260)?)?;
 // 0x40364: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40368: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x4036c: bne 0x40384 <entry+0x384>   @ imm = #0x10
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40384,false)?)?;
 // 0x40370: ldr r0, [r5, #0xfc]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,252)?)?;
 // 0x40374: ldr r1, [r5, #0x100]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,256)?)?;
 // 0x40378: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x4037c: cmp r0, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R0,R::R1,Shift::Lsl,0,false)?)?;
 // 0x40380: moveq r3, #1
 c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,1)?)?;
 // 0x40384: cmp r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
 // 0x40388: mov r0, r3
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(3)?))?;
 // 0x4038c: mov r10, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(10)?,R::R0,255)?)?;
 // 0x40390: sub r6, r11, #92
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,92)?)?;
 // 0x40394: mov r9, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,1)?)?;
 // 0x40398: str r3, [sp, #0x54]
 c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::SP,84)?)?;
 // 0x4039c: strh r7, [r11, #-56]
 c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::R11,-56)?)?;
 // 0x403a0: str r5, [sp, #0x5c]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,92)?)?;
 // 0x403a4: movne r0, #8
 c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,8)?)?;
 // 0x403a8: orr r10, r10, #256
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(10)?,R::new(10)?,256)?)?;
 // 0x403ac: str r0, [r11, #-0x5c]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-92)?)?;
 // 0x403b0: mov r0, #4
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
 // 0x403b4: strh r0, [r11, #-104]
 c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-104)?)?;
 // 0x403b8: mov r0, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
 // 0x403bc: sub r4, r11, #56
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,56)?)?;
 // 0x403c0: sub r8, r11, #104
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(8)?,R::R11,104)?)?;
 // 0x403c4: add r1, r5, #12
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,12)?)?;
 // 0x403c8: stm sp, {r0, r9, r10}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(9)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x403cc: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x403d0: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x403d4: mov r3, r8
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
 // 0x403d8: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11990
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x403dc: mov r0, #10
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
 // 0x403e0: add r1, r5, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(5)?,8)?)?;
 // 0x403e4: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x403e8: mov r3, r8
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
 // 0x403ec: stm sp, {r0, r9}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(9)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x403f0: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x403f4: orr r0, r0, #768
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
 // 0x403f8: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x403fc: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x40400: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x119b8
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x40404: mov r0, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,9)?)?;
 // 0x40408: mov r1, r5
 c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
 // 0x4040c: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x40410: mov r3, r8
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(8)?))?;
 // 0x40414: stm sp, {r0, r9, r10}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(9)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x40418: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x4041c: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x119d4
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x40420: add r7, r7, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
 // 0x40424: add r5, r5, #32
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,32)?)?;
 // 0x40428: cmp r7, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
 // 0x4042c: bne 0x403b8 <entry+0x3b8>   @ imm = #-0x7c
 c.emit(arm32::branch(C::Ne,c.at as u32,0x403b8,false)?)?;
 // 0x40430: ldr r1, [sp, #0x54]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,84)?)?;
 // 0x40434: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x40438: sub r5, r11, #56
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(5)?,R::R11,56)?)?;
 // 0x4043c: sub r7, r11, #104
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(7)?,R::R11,104)?)?;
 // 0x40440: mov r9, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,9)?)?;
 // 0x40444: mov r8, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,1)?)?;
 // 0x40448: orr r0, r0, #256
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
 // 0x4044c: mov r2, r5
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
 // 0x40450: mov r3, r7
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
 // 0x40454: str r9, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
 // 0x40458: str r8, [sp, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,4)?)?;
 // 0x4045c: mov r10, r0
 c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
 // 0x40460: mov r0, #256
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,256)?)?;
 // 0x40464: str r10, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,8)?)?;
 // 0x40468: cmp r1, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
 // 0x4046c: ldr r1, [sp, #0x5c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,92)?)?;
 // 0x40470: movne r0, #240
 c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,240)?)?;
 // 0x40474: add r4, r1, r0
 c.emit(arm32::data_register(C::Al,Op::Add,R::new(4)?,R::R1,R::R0,Shift::Lsl,0,false)?)?;
 // 0x40478: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x4047c: add r1, r4, #12
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,12)?)?;
 // 0x40480: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11a38
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x40484: mov r0, #10
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,10)?)?;
 // 0x40488: add r1, r4, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,8)?)?;
 // 0x4048c: mov r2, r5
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
 // 0x40490: mov r3, r7
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
 // 0x40494: stm sp, {r0, r8}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x40498: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x4049c: orr r0, r0, #768
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,768)?)?;
 // 0x404a0: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x404a4: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x404a8: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11a60
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x404ac: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x404b0: mov r1, r4
 c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
 // 0x404b4: mov r2, r5
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
 // 0x404b8: mov r3, r7
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(7)?))?;
 // 0x404bc: str r9, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,0)?)?;
 // 0x404c0: stmib sp, {r8, r10}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::new(8)?.bit() | R::new(10)?.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x404c4: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11a7c
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x404c8: ldr r10, [sp, #0x64]
 c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,100)?)?;
 // 0x404cc: ldr r8, [sp, #0x68]
 c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
 // 0x404d0: mov r2, #2144
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2144)?)?;
 // 0x404d4: mov r1, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
 // 0x404d8: orr r2, r2, #8192
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,8192)?)?;
 // 0x404dc: ldr r0, [r10]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
 // 0x404e0: mov r3, r0
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
 // 0x404e4: tst r3, #31
 c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(3)?,31)?)?;
 // 0x404e8: ldr r5, [r6], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(5)?,R::new(6)?,4)?)?;
 // 0x404ec: add r1, r1, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
 // 0x404f0: ldreq r4, [r3]
 c.emit(arm32::transfer_word(C::Eq,true,R::new(4)?,R::new(3)?,0)?)?;
 // 0x404f4: andeq r4, r4, #1
 c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(4)?,R::new(4)?,1)?)?;
 // 0x404f8: streq r4, [r3]
 c.emit(arm32::transfer_word(C::Eq,false,R::new(4)?,R::new(3)?,0)?)?;
 // 0x404fc: stm r3, {r2, r5}
 c.emit(arm32::transfer_multiple(C::Al,false,R::new(3)?,R::R2.bit() | R::new(5)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x40500: add r2, r2, #4
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,4)?)?;
 // 0x40504: add r3, r3, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,8)?)?;
 // 0x40508: cmp r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
 // 0x4050c: ldr r5, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R11,-32)?)?;
 // 0x40510: add r5, r5, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
 // 0x40514: str r5, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-32)?)?;
 // 0x40518: bne 0x404e4 <entry+0x4e4>   @ imm = #-0x3c
 c.emit(arm32::branch(C::Ne,c.at as u32,0x404e4,false)?)?;
 // 0x4051c: add r0, r0, #64
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,64)?)?;
 // 0x40520: str r0, [r10]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
 c.check(0x40524)
}

/// Continue the original controller routine at 0x40524; this is not a separate callable entry.
#[inline(never)]
fn controller_prefix_part_2(c:&mut Code)->Result<(),String>{
 c.check(0x40524)?;
 // 0x40524: ldr r4, [r8, #0xab8]
 c.emit(arm32::transfer_word(C::Al,true,R::new(4)?,R::new(8)?,2744)?)?;
 // 0x40528: ldrb r0, [r4, #0x143]
 c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,323)?)?;
 // 0x4052c: cmp r0, #1
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
 // 0x40530: bne 0x406d8 <entry+0x6d8>   @ imm = #0x1a0
 c.emit(arm32::branch(C::Ne,c.at as u32,0x406d8,false)?)?;
 // 0x40534: sub r6, r11, #56
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::R11,56)?)?;
 // 0x40538: mov r1, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
 // 0x4053c: mov r2, #24
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,24)?)?;
 // 0x40540: mov r5, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
 // 0x40544: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x40548: bl 0x31ec <entry+0xfffffffffffc31ec> @ imm = #-0x3d364
 c.emit(arm32::branch(C::Al,c.at as u32,0x31ec,true)?)?;
 // 0x4054c: ldrb r0, [r4, #0x160]
 c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,352)?)?;
 // 0x40550: strh r5, [r11, #-106]
 c.emit(arm32::store_halfword(C::Al,R::new(5)?,R::R11,-106)?)?;
 // 0x40554: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40558: beq 0x405c0 <entry+0x5c0>   @ imm = #0x60
 c.emit(arm32::branch(C::Eq,c.at as u32,0x405c0,false)?)?;
 // 0x4055c: add r0, r4, #98
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(4)?,98)?)?;
 // 0x40560: mvn r1, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R1,R::R0,0)?)?;
 // 0x40564: sub r2, r11, #92
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,92)?)?;
 // 0x40568: add r0, r0, #256
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,256)?)?;
 // 0x4056c: ldrsh r3, [r0], #2
 c.emit(typed_signed_halfword_postindex(C::Al,R::new(3)?,R::R0,2)?)?;
 // 0x40570: add r1, r1, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
 // 0x40574: cmp r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
 // 0x40578: str r3, [r2], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R2,4)?)?;
 // 0x4057c: blt 0x4056c <entry+0x56c>   @ imm = #-0x18
 c.emit(arm32::branch(C::Lt,c.at as u32,0x4056c,false)?)?;
 // 0x40580: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x40584: add r1, r4, #372
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::new(4)?,372)?)?;
 // 0x40588: strh r0, [r11, #-106]
 c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-106)?)?;
 // 0x4058c: ldrsh r2, [r1], #2
 c.emit(typed_signed_halfword_postindex(C::Al,R::R2,R::R1,2)?)?;
 // 0x40590: add r0, r0, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
 // 0x40594: cmp r0, #3
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
 // 0x40598: str r2, [r7], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,false,R::R2,R::new(7)?,4)?)?;
 // 0x4059c: bne 0x4058c <entry+0x58c>   @ imm = #-0x18
 c.emit(arm32::branch(C::Ne,c.at as u32,0x4058c,false)?)?;
 // 0x405a0: b 0x40604 <entry+0x604>   @ imm = #0x5c
 c.emit(arm32::branch(C::Al,c.at as u32,0x40604,false)?)?;
 // 0x405a4: ldr r3, [sp, #0x68]
 c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,104)?)?;
 // 0x405a8: ldrb r6, [r3, #0x3dc]
 c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(3)?,988)?)?;
 // 0x405ac: mov r3, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
 // 0x405b0: cmp r6, #1
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,1)?)?;
 // 0x405b4: andne r3, r1, #1
 c.emit(arm32::data_immediate(C::Ne,Op::And,R::new(3)?,R::R1,1)?)?;
 // 0x405b8: b 0x3fa48 <entry+0xfffffffffffffa48> @ imm = #-0xb78
 c.emit(arm32::branch(C::Al,c.at as u32,0x3fa48,false)?)?;
 // 0x405bc: data: DIAGNOSTIC_WMFW
 c.emit(DIAGNOSTIC_WMFW)?;
 // 0x405c0: ldr r2, [pc, #0x45c]        @ 0xa24 <entry+0xa24>
 c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,1116)?)?;
 // 0x405c4: mvn r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::R0,R::R0,0)?)?;
 // 0x405c8: sub r1, r11, #92
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,92)?)?;
 // 0x405cc: ldrh r3, [r2], #2
 c.emit(typed_halfword_postindex(C::Al,true,R::new(3)?,R::R2,2)?)?;
 // 0x405d0: add r0, r0, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
 // 0x405d4: cmp r0, #8
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8)?)?;
 // 0x405d8: str r3, [r1], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R1,4)?)?;
 // 0x405dc: blt 0x405cc <entry+0x5cc>   @ imm = #-0x18
 c.emit(arm32::branch(C::Lt,c.at as u32,0x405cc,false)?)?;
 // 0x405e0: ldr r1, [pc, #0x440]        @ 0xa28 <entry+0xa28>
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1088)?)?;
 // 0x405e4: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x405e8: sub r2, r11, #104
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::R11,104)?)?;
 // 0x405ec: strh r0, [r11, #-106]
 c.emit(arm32::store_halfword(C::Al,R::R0,R::R11,-106)?)?;
 // 0x405f0: ldrh r3, [r1], #2
 c.emit(typed_halfword_postindex(C::Al,true,R::new(3)?,R::R1,2)?)?;
 // 0x405f4: add r0, r0, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
 // 0x405f8: cmp r0, #3
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
 // 0x405fc: str r3, [r2], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,false,R::new(3)?,R::R2,4)?)?;
 // 0x40600: bne 0x405f0 <entry+0x5f0>   @ imm = #-0x18
 c.emit(arm32::branch(C::Ne,c.at as u32,0x405f0,false)?)?;
 // 0x40604: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x40608: mov r7, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,0)?)?;
 // 0x4060c: sub r4, r11, #106
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,106)?)?;
 // 0x40610: add r5, sp, #108
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::SP,108)?)?;
 // 0x40614: mov r8, #9
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(8)?,R::R0,9)?)?;
 // 0x40618: mov r1, #11
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,11)?)?;
 // 0x4061c: orr r0, r0, #1792
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,1792)?)?;
 // 0x40620: strh r7, [r11, #-106]
 c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::R11,-106)?)?;
 // 0x40624: strh r7, [sp, #108]
 c.emit(arm32::store_halfword(C::Al,R::new(7)?,R::SP,108)?)?;
 // 0x40628: stm sp, {r1, r8}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R1.bit() | R::new(8)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x4062c: sub r1, r11, #92
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,92)?)?;
 // 0x40630: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x40634: mov r3, r5
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
 // 0x40638: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x4063c: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x40640: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11bf8
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x40644: mov r0, #3
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,3)?)?;
 // 0x40648: str r8, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(8)?,R::SP,0)?)?;
 // 0x4064c: sub r1, r11, #104
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R11,104)?)?;
 // 0x40650: mov r2, r4
 c.emit(arm32::mov_register(C::Al,R::R2,R::new(4)?))?;
 // 0x40654: mov r3, r5
 c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(5)?))?;
 // 0x40658: str r0, [sp, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,4)?)?;
 // 0x4065c: mov r0, #255
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,255)?)?;
 // 0x40660: orr r0, r0, #256
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R0,R::R0,256)?)?;
 // 0x40664: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x40668: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x4066c: bl 0x2ea50 <entry+0xfffffffffffeea50> @ imm = #-0x11c24
 c.emit(arm32::branch(C::Al,c.at as u32,0x2ea50,true)?)?;
 // 0x40670: ldrsh r0, [r11, #-106]
 c.emit(arm32::load_signed_halfword(C::Al,R::R0,R::R11,-106)?)?;
 // 0x40674: ldrsh r2, [sp, #108]
 c.emit(arm32::load_signed_halfword(C::Al,R::R2,R::SP,108)?)?;
 // 0x40678: ldr r8, [sp, #0x68]
 c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,104)?)?;
 // 0x4067c: ldr r1, [r6, r0, lsl #2]
 c.emit(arm32::transfer_word_register(C::Al,true,R::R1,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
 // 0x40680: rsb r2, r2, #32
 c.emit(arm32::data_immediate(C::Al,Op::Rsb,R::R2,R::R2,32)?)?;
 // 0x40684: lsl r1, r1, r2
 c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R1,R::R0,R::R1,Shift::Lsl,R::R2,false)?)?;
 // 0x40688: str r1, [r6, r0, lsl #2]
 c.emit(arm32::transfer_word_register(C::Al,false,R::R1,R::new(6)?,R::R0,Shift::Lsl,2)?)?;
 // 0x4068c: mov r1, #10368
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,10368)?)?;
 // 0x40690: ldr r0, [r10]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
 // 0x40694: mov r2, r0
 c.emit(arm32::mov_register(C::Al,R::R2,R::R0))?;
 // 0x40698: tst r2, #31
 c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R2,31)?)?;
 // 0x4069c: ldr r3, [r6], #4
 c.emit(arm32::transfer_word_postindexed(C::Al,true,R::new(3)?,R::new(6)?,4)?)?;
 // 0x406a0: add r7, r7, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,1)?)?;
 // 0x406a4: ldreq r5, [r2]
 c.emit(arm32::transfer_word(C::Eq,true,R::new(5)?,R::R2,0)?)?;
 // 0x406a8: andeq r5, r5, #1
 c.emit(arm32::data_immediate(C::Eq,Op::And,R::new(5)?,R::new(5)?,1)?)?;
 // 0x406ac: streq r5, [r2]
 c.emit(arm32::transfer_word(C::Eq,false,R::new(5)?,R::R2,0)?)?;
 // 0x406b0: stm r2, {r1, r3}
 c.emit(arm32::transfer_multiple(C::Al,false,R::R2,R::R1.bit() | R::new(3)?.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x406b4: add r1, r1, #4
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,4)?)?;
 // 0x406b8: add r2, r2, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,8)?)?;
 // 0x406bc: cmp r7, #4
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,4)?)?;
 // 0x406c0: ldr r3, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R11,-32)?)?;
 // 0x406c4: add r3, r3, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
 // 0x406c8: str r3, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R11,-32)?)?;
 // 0x406cc: bne 0x40698 <entry+0x698>   @ imm = #-0x3c
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40698,false)?)?;
 // 0x406d0: add r0, r0, #32
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,32)?)?;
 // 0x406d4: str r0, [r10]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
 // 0x406d8: ldr r0, [sp, #0x60]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,96)?)?;
 // 0x406dc: cmp r0, #1
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
 // 0x406e0: bne 0x40700 <entry+0x700>   @ imm = #0x18
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40700,false)?)?;
 // 0x406e4: ldr r0, [r8, #0x858]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2136)?)?;
 // 0x406e8: ldr r2, [pc, #0x324]        @ 0xa14 <entry+0xa14>
 c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,804)?)?;
 // 0x406ec: mov r1, #1952
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1952)?)?;
 // 0x406f0: orr r1, r1, #4096
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4096)?)?;
 // 0x406f4: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x406f8: movne r0, #1
 c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
 // 0x406fc: bl 0x17b0 <entry+0xfffffffffffc17b0> @ imm = #-0x3ef54
 c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
 // 0x40700: ldr r0, [sp, #0x58]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::SP,88)?)?;
 // 0x40704: cmp r0, #1
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
 // 0x40708: bne 0x40728 <entry+0x728>   @ imm = #0x18
 c.emit(arm32::branch(C::Ne,c.at as u32,0x40728,false)?)?;
 // 0x4070c: ldr r0, [r8, #0x868]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2152)?)?;
 // 0x40710: ldr r2, [pc, #0x2fc]        @ 0xa14 <entry+0xa14>
 c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,764)?)?;
 // 0x40714: mov r1, #162
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,162)?)?;
 // 0x40718: orr r1, r1, #5888
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,5888)?)?;
 // 0x4071c: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40720: movne r0, #1
 c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,1)?)?;
 // 0x40724: bl 0x17b0 <entry+0xfffffffffffc17b0> @ imm = #-0x3ef7c
 c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
 c.check(0x40728)
}

/// Continue the original controller routine at 0x40728; this is not a separate callable entry.
#[inline(never)]
fn controller_prefix_part_3(c:&mut Code)->Result<(),String>{
 c.check(0x40728)?;
 // 0x40728: ldr r0, [r8, #0x858]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2136)?)?;
 // 0x4072c: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40730: beq 0x40794 <entry+0x794>   @ imm = #0x5c
 c.emit(arm32::branch(C::Eq,c.at as u32,0x40794,false)?)?;
 // 0x40734: ldr r1, [r8, #0x6f4]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1780)?)?;
 // 0x40738: sub r4, r11, #32
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
 // 0x4073c: mov r5, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
 // 0x40740: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40744: mov r2, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
 // 0x40748: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x4074c: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x40750: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x40754: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x40758: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x4075c: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x40760: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x273c
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40764: ldr r0, [r8, #0x6f4]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1780)?)?;
 // 0x40768: ldr r1, [r8, #0x85c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2140)?)?;
 // 0x4076c: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40770: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x40774: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x40778: mov r2, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
 // 0x4077c: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40780: str r1, [sp, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
 // 0x40784: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x40788: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x4078c: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x40790: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x276c
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40794: ldr r0, [r8, #0x868]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2152)?)?;
 // 0x40798: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x4079c: beq 0x40800 <entry+0x800>   @ imm = #0x5c
 c.emit(arm32::branch(C::Eq,c.at as u32,0x40800,false)?)?;
 // 0x407a0: ldr r1, [r8, #0x6fc]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1788)?)?;
 // 0x407a4: sub r4, r11, #32
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
 // 0x407a8: mov r5, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
 // 0x407ac: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x407b0: mov r2, #4
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,4)?)?;
 // 0x407b4: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x407b8: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x407bc: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x407c0: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x407c4: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x407c8: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x407cc: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x27a8
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x407d0: ldr r0, [r8, #0x6fc]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,1788)?)?;
 // 0x407d4: ldr r1, [r8, #0x86c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,2156)?)?;
 // 0x407d8: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x407dc: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x407e0: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x407e4: mov r2, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,5)?)?;
 // 0x407e8: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x407ec: str r1, [sp, #0x4]
 c.emit(arm32::transfer_word(C::Al,false,R::R1,R::SP,4)?)?;
 // 0x407f0: str r0, [sp, #0x8]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,8)?)?;
 // 0x407f4: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x407f8: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x407fc: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x27d8
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40800: ldr r0, [r8, #0x298]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,664)?)?;
 // 0x40804: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40808: beq 0x408fc <entry+0x8fc>   @ imm = #0xec
 c.emit(arm32::branch(C::Eq,c.at as u32,0x408fc,false)?)?;
 // 0x4080c: add r4, r8, #2144
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(8)?,2144)?)?;
 // 0x40810: mov r5, #67
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,67)?)?;
 // 0x40814: sub r9, r11, #32
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(9)?,R::R11,32)?)?;
 // 0x40818: mov r7, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,2)?)?;
 // 0x4081c: ldr r0, [r4]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,0)?)?;
 // 0x40820: sub r6, r5, #67
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(5)?,67)?)?;
 // 0x40824: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40828: beq 0x40880 <entry+0x880>   @ imm = #0x50
 c.emit(arm32::branch(C::Eq,c.at as u32,0x40880,false)?)?;
 // 0x4082c: ldr r1, [r8, #0x6f8]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1784)?)?;
 // 0x40830: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40834: str r9, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
 // 0x40838: str r7, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
 // 0x4083c: mov r2, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
 // 0x40840: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40844: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x40848: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x4084c: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x40850: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x282c
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40854: ldr r0, [r4, #0x4]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,4)?)?;
 // 0x40858: ldr r1, [r8, #0x6f8]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1784)?)?;
 // 0x4085c: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40860: str r9, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
 // 0x40864: str r7, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
 // 0x40868: mov r2, #3
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
 // 0x4086c: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40870: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x40874: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x40878: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x4087c: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x2858
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40880: ldr r0, [r4, #0x10]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,16)?)?;
 // 0x40884: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40888: beq 0x408e0 <entry+0x8e0>   @ imm = #0x50
 c.emit(arm32::branch(C::Eq,c.at as u32,0x408e0,false)?)?;
 // 0x4088c: ldr r1, [r8, #0x700]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1792)?)?;
 // 0x40890: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40894: str r9, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
 // 0x40898: str r7, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
 // 0x4089c: mov r2, #6
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,6)?)?;
 // 0x408a0: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x408a4: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x408a8: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x408ac: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x408b0: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x288c
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x408b4: ldr r0, [r4, #0x14]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,20)?)?;
 // 0x408b8: ldr r1, [r8, #0x700]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1792)?)?;
 // 0x408bc: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x408c0: str r9, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(9)?,R::SP,20)?)?;
 // 0x408c4: str r7, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
 // 0x408c8: mov r2, #7
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,7)?)?;
 // 0x408cc: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x408d0: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x408d4: mov r0, r6
 c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
 // 0x408d8: mov r1, #5
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,5)?)?;
 // 0x408dc: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x28b8
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x408e0: ldr r1, [r8, #0x298]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,664)?)?;
 // 0x408e4: sub r2, r5, #66
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R2,R::new(5)?,66)?)?;
 // 0x408e8: add r0, r5, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::new(5)?,1)?)?;
 // 0x408ec: add r4, r4, #32
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(4)?,R::new(4)?,32)?)?;
 // 0x408f0: mov r5, r0
 c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
 // 0x408f4: cmp r2, r1
 c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
 // 0x408f8: blo 0x4081c <entry+0x81c>   @ imm = #-0xe4
 c.emit(arm32::branch(C::Cc,c.at as u32,0x4081c,false)?)?;
 // 0x408fc: ldr r0, [r10]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,0)?)?;
 // 0x40900: mov r2, #182
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,182)?)?;
 // 0x40904: orr r2, r2, #3328
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,3328)?)?;
 // 0x40908: tst r0, #31
 c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R0,31)?)?;
 // 0x4090c: ldreq r1, [r0]
 c.emit(arm32::transfer_word(C::Eq,true,R::R1,R::R0,0)?)?;
 // 0x40910: andeq r1, r1, #1
 c.emit(arm32::data_immediate(C::Eq,Op::And,R::R1,R::R1,1)?)?;
 // 0x40914: streq r1, [r0]
 c.emit(arm32::transfer_word(C::Eq,false,R::R1,R::R0,0)?)?;
 // 0x40918: mov r1, #436
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,436)?)?;
 // 0x4091c: orr r1, r1, #14336
 c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,14336)?)?;
 // 0x40920: stm r0, {r1, r2}
 c.emit(arm32::transfer_multiple(C::Al,false,R::R0,R::R1.bit() | R::R2.bit(),arm32::BlockMode::IncrementAfter,false)?)?;
 // 0x40924: add r0, r0, #8
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,8)?)?;
 // 0x40928: ldr r1, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-32)?)?;
 // 0x4092c: add r1, r1, #1
 c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
 // 0x40930: str r1, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R11,-32)?)?;
 // 0x40934: str r0, [r10]
 c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,0)?)?;
 c.check(0x40938)
}

/// Continue the original controller routine at 0x40938; this is not a separate callable entry.
#[inline(never)]
fn controller_prefix_part_4(c:&mut Code)->Result<(),String>{
 c.check(0x40938)?;
 // 0x40938: ldr r0, [r8, #0x980]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2432)?)?;
 // 0x4093c: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x40940: beq 0x409a0 <entry+0x9a0>   @ imm = #0x58
 c.emit(arm32::branch(C::Eq,c.at as u32,0x409a0,false)?)?;
 // 0x40944: ldr r1, [r8, #0x704]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1796)?)?;
 // 0x40948: sub r4, r11, #32
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
 // 0x4094c: mov r5, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
 // 0x40950: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40954: mov r2, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
 // 0x40958: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x4095c: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x40960: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x40964: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x40968: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x4096c: mov r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
 // 0x40970: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x294c
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40974: ldr r0, [r8, #0x984]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2436)?)?;
 // 0x40978: ldr r1, [r8, #0x708]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1800)?)?;
 // 0x4097c: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x40980: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x40984: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x40988: mov r2, #1
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
 // 0x4098c: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x40990: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x40994: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x40998: mov r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
 // 0x4099c: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x2978
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x409a0: ldr r0, [r8, #0x988]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2440)?)?;
 // 0x409a4: cmp r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
 // 0x409a8: beq 0x40a08 <entry+0xa08>   @ imm = #0x58
 c.emit(arm32::branch(C::Eq,c.at as u32,0x40a08,false)?)?;
 // 0x409ac: ldr r1, [r8, #0x70c]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1804)?)?;
 // 0x409b0: sub r4, r11, #32
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(4)?,R::R11,32)?)?;
 // 0x409b4: mov r5, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,2)?)?;
 // 0x409b8: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x409bc: mov r2, #2
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,2)?)?;
 // 0x409c0: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x409c4: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x409c8: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x409cc: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x409d0: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x409d4: mov r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
 // 0x409d8: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x29b4
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x409dc: ldr r0, [r8, #0x98c]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(8)?,2444)?)?;
 // 0x409e0: ldr r1, [r8, #0x710]
 c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(8)?,1808)?)?;
 // 0x409e4: str r10, [sp, #0x10]
 c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,16)?)?;
 // 0x409e8: str r4, [sp, #0x14]
 c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::SP,20)?)?;
 // 0x409ec: str r5, [sp]
 c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::SP,0)?)?;
 // 0x409f0: mov r2, #3
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,3)?)?;
 // 0x409f4: mov r3, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
 // 0x409f8: stmib sp, {r0, r1}
 c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit() | R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
 // 0x409fc: mov r0, #0
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
 // 0x40a00: mov r1, #8
 c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,8)?)?;
 // 0x40a04: bl 0x3e02c <entry+0xffffffffffffe02c> @ imm = #-0x29e0
 c.emit(arm32::branch(C::Al,c.at as u32,0x3e02c,true)?)?;
 // 0x40a08: ldr r0, [r11, #-0x20]
 c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-32)?)?;
 // 0x40a0c: sub sp, r11, #28
 c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
 // 0x40a10: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
 c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit() | R::new(5)?.bit() | R::new(6)?.bit() | R::new(7)?.bit() | R::new(8)?.bit() | R::new(9)?.bit() | R::new(10)?.bit() | R::R11.bit() | R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x40a14)
}

/// Continue the original controller routine at 0x40a14; this is not a separate callable entry.
#[inline(never)]
fn controller_prefix_part_5(c:&mut Code)->Result<(),String>{
 c.check(0x40a14)?;
 // 0x40a14: data: FORMAT_DIAGNOSTIC
 c.emit(FORMAT_DIAGNOSTIC)?;
 // 0x40a18: data: CODEC_CHECK_CONTEXT
 c.emit(CODEC_CHECK_CONTEXT)?;
 // 0x40a1c: data: DIMENSION_BIT_MASK
 c.emit(DIMENSION_BIT_MASK)?;
 // 0x40a20: data: CODEC_PARAMETER_CONTEXT
 c.emit(CODEC_PARAMETER_CONTEXT)?;
 // 0x40a24: data: CODEC_BUFFER_ERROR
 c.emit(CODEC_BUFFER_ERROR)?;
 // 0x40a28: data: CODEC_CHANNEL_NOTICE
 c.emit(CODEC_CHANNEL_NOTICE)?;
 c.check(0x40a2c)
}

/// Standalone generated fragment. Never reads a reference firmware file.
pub fn fragment()->Result<Vec<u8>,String>{
 let mut c=Code::new();
 construct_controller_prefix(&mut c)?;
 controller_prefix_part_1(&mut c)?;
 controller_prefix_part_2(&mut c)?;
 controller_prefix_part_3(&mut c)?;
 controller_prefix_part_4(&mut c)?;
 controller_prefix_part_5(&mut c)?;
 Ok(c.bytes)
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn prefix_exact_length_and_bounds(){
  let bytes=fragment().unwrap();
  assert_eq!(bytes.len(),END-START);
  assert_eq!(START,0x40000);
  assert_eq!(END,super::super::venus_controller_middle::FIRST_START);
  assert_eq!(INSTRUCTION_COUNT*4+DATA_WORD_COUNT*4,bytes.len());
 }
 #[test]fn branch_skipped_data_is_explicit(){
  let bytes=fragment().unwrap();
  let word=|addr:usize| ->u32 {u32::from_le_bytes(bytes[addr-START..addr-START+4].try_into().unwrap())};
  assert_eq!(word(0x405bc),DIAGNOSTIC_WMFW);
  assert_eq!(word(0x40a14),FORMAT_DIAGNOSTIC);
  assert_eq!(word(0x40a1c),DIMENSION_BIT_MASK);
  assert_eq!(word(0x40a28),CODEC_CHANNEL_NOTICE);
  // The final real return at 0x40a10 loads PC from the stack.
  let ret_mask=R::new(4).unwrap().bit()|R::new(5).unwrap().bit()|
    R::new(6).unwrap().bit()|R::new(7).unwrap().bit()|
    R::new(8).unwrap().bit()|R::new(9).unwrap().bit()|
    R::new(10).unwrap().bit()|R::R11.bit()|R::PC.bit();
  assert_eq!(word(0x40a10),arm32::transfer_multiple(C::Al,true,R::SP,
    ret_mask,arm32::BlockMode::IncrementAfter,true).unwrap());
 }
}
