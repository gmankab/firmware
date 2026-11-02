// SPDX-License-Identifier: GPL-3.0-only
//! Venus middle controller A32 source-derived instruction reconstruction.
//! Address `[0x65000,0x66000)` is an exact fragment; embedded constant
//! and handler-pointer tables are distinguished from executable A32 opcodes.
//! Binary match alone is not proof of original high-level source semantics.
use arm32::{Condition as C,DataOp as Op,Register as R,Shift};
pub const START:usize=0x65000;
pub const END:usize=0x66000;
pub const FILE_OFFSET:usize=0x4000+START;
struct Code{at:usize,bytes:Vec<u8>}
impl Code {
 fn new()->Self{Self{at:START,bytes:Vec::with_capacity(END-START)}}
 fn check(&self,expect:usize)->Result<(),String>{if self.at!=expect{return Err(format!("A32 expected 0x{expect:x}, reached 0x{:x}",self.at))}Ok(())}
 fn emit(&mut self,w:u32)->Result<(),String>{
  if self.at<START||self.at>=END||self.at>=0x68000||self.at&3!=0{return Err(format!("Invalid Agent04 location 0x{:x}",self.at))}
  self.bytes.extend_from_slice(&w.to_le_bytes());self.at+=4;Ok(())
 }
}
fn emit_byte_register_writeback(cond:C,load:bool,rd:R,rn:R,rm:R,shift:Shift,amount:u8)->Result<u32,String>{
 if rn==R::PC||rm==R::PC||(load&&rd==rn){return Err("invalid A32 byte register writeback".into())}
 Ok(arm32::transfer_byte_register(cond,load,rd,rn,rm,shift,amount)?|(1<<21))
}
fn emit_halfword_register(cond:C,load:bool,rd:R,base:R,index:R,writeback:bool)->Result<u32,String>{
 if [rd,base,index].contains(&R::PC)||(load&&rd==base&&writeback){return Err("unsafe A32 indexed halfword transfer".into())}
 Ok(((cond as u32)<<28)|0x0180_00b0|(if load{1<<20}else{0})|(if writeback{1<<21}else{0})|(base.number()<<16)|(rd.number()<<12)|index.number())
}
fn emit_signed_halfword_register(cond:C,rd:R,base:R,index:R,writeback:bool)->Result<u32,String>{
 if [rd,base,index].contains(&R::PC)||(writeback&&rd==base){return Err("invalid A32 indexed signed halfword operands".into())}
 Ok(((cond as u32)<<28)|0x0190_00f0|(if writeback{1<<21}else{0})|(base.number()<<16)|(rd.number()<<12)|index.number())
}
fn emit_signed_byte_postindexed(cond:C,rd:R,base:R,offset:i32)->Result<u32,String>{
 if [rd,base].contains(&R::PC)||rd==base||offset.unsigned_abs()>255 {return Err("invalid A32 signed byte postindex".into())}
 Ok(arm32::load_signed_byte(cond,rd,base,offset)? & !(1<<24))
}
fn emit_halfword_postindexed(cond:C,rd:R,rn:R,offset:i32)->Result<u32,String>{
 if rd==rn || [rd,rn].contains(&R::PC) {return Err("invalid A32 postindexed halfword load operands".into())}
 Ok(arm32::store_halfword_postindexed(cond,rd,rn,offset)? | (1<<20))
}
fn emit_umull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 UMULL operands".into())}
 Ok(((cond as u32)<<28)|0x0080_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
fn emit_adc_shifted(cond:C,rd:R,rn:R,rm:R,shift:Shift,amount:u8,flags:bool)->Result<u32,String>{
 Ok(arm32::data_register(cond,Op::Add,rd,rn,rm,shift,amount,flags)? | (1<<21))
}
fn emit_smull(cond:C,lo:R,hi:R,rm:R,rs:R)->Result<u32,String>{
 if lo==hi || [lo,hi,rm,rs].contains(&R::PC){return Err("unsafe A32 SMULL operands".into())}
 Ok(((cond as u32)<<28)|0x00c0_0090|(hi.number()<<16)|(lo.number()<<12)|(rs.number()<<8)|rm.number())
}
#[inline(never)]fn emit_controller_packet(c:&mut Code)->Result<(),String>{
 c.check(START)?;
 // Executable A32 block 0x65000..0x654f0.
 c.check(0x65000)?;
    // 0x65000: ldr r1, [r7, #0x38c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,908)?)?;
    // 0x65004: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65008: beq 0x65028 <entry+0xf828>  @ imm = #0x18
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65028,false)?)?;
    // 0x6500c: ldr r1, [r7, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2732)?)?;
    // 0x65010: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65014: ldrne r1, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(7)?,644)?)?;
    // 0x65018: cmpne r1, #8192
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,8192)?)?;
    // 0x6501c: bne 0x65028 <entry+0xf828>  @ imm = #0x4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65028,false)?)?;
    // 0x65020: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65024: strb r1, [r4, #0x3e8]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1000)?)?;
    // 0x65028: ldr r1, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,364)?)?;
    // 0x6502c: ldr r3, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(4)?,988)?)?;
    // 0x65030: sub r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65034: add r3, r8, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::new(8)?,1)?)?;
    // 0x65038: ldr r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,20)?)?;
    // 0x6503c: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x65040: cmpeq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x65044: bne 0x6507c <entry+0xf87c>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6507c,false)?)?;
    // 0x65048: ldr r1, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,84)?)?;
    // 0x6504c: cmp r1, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,17)?)?;
    // 0x65050: bhi 0x65064 <entry+0xf864>  @ imm = #0xc
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65064,false)?)?;
    // 0x65054: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x65058: mov r5, #137216
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,137216)?)?;
    // 0x6505c: tst r5, r3, lsl r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,R::R1,false)?)?;
    // 0x65060: bne 0x6507c <entry+0xf87c>  @ imm = #0x14
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6507c,false)?)?;
    // 0x65064: ldrb r1, [r8, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1814)?)?;
    // 0x65068: add r1, r1, r2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x6506c: strb r1, [r4, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,997)?)?;
    // 0x65070: ldrb r3, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1744)?)?;
    // 0x65074: add r1, r1, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x65078: strb r1, [r4, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1596)?)?;
    // 0x6507c: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x65080: bne 0x650bc <entry+0xf8bc>  @ imm = #0x34
    c.emit(arm32::branch(C::Ne,c.at as u32,0x650bc,false)?)?;
    // 0x65084: ldr r1, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,84)?)?;
    // 0x65088: cmp r1, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,8)?)?;
    // 0x6508c: cmpne r1, #15
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,15)?)?;
    // 0x65090: beq 0x650a8 <entry+0xf8a8>  @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x650a8,false)?)?;
    // 0x65094: cmp r1, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,16)?)?;
    // 0x65098: bne 0x650bc <entry+0xf8bc>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x650bc,false)?)?;
    // 0x6509c: ldrb r1, [r4, #0x171]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,369)?)?;
    // 0x650a0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x650a4: beq 0x652f8 <entry+0xfaf8>  @ imm = #0x24c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x652f8,false)?)?;
    // 0x650a8: ldrb r1, [r8, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1814)?)?;
    // 0x650ac: strb r1, [r4, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,997)?)?;
    // 0x650b0: ldrb r3, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(10)?,1744)?)?;
    // 0x650b4: add r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x650b8: strb r1, [r4, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1596)?)?;
    // 0x650bc: ldrb r1, [r10, #0xa]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,10)?)?;
    // 0x650c0: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x650c4: blo 0x65144 <entry+0xf944>  @ imm = #0x78
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65144,false)?)?;
    // 0x650c8: ldr r3, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,644)?)?;
    // 0x650cc: cmp r3, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,8192)?)?;
    // 0x650d0: bne 0x65144 <entry+0xf944>  @ imm = #0x6c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65144,false)?)?;
    // 0x650d4: ldrb r3, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(4)?,378)?)?;
    // 0x650d8: sub r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R1,R::R1,1)?)?;
    // 0x650dc: cmp r1, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x650e0: bne 0x65134 <entry+0xf934>  @ imm = #0x4c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65134,false)?)?;
    // 0x650e4: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x650e8: b 0x65140 <entry+0xf940>  @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x65140,false)?)?;
    // 0x650ec: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x650f0: strb r0, [r4, #0x175]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,373)?)?;
    // 0x650f4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x650f8: b 0x65104 <entry+0xf904>  @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x65104,false)?)?;
    // 0x650fc: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x65100: strb r0, [r4, #0x175]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,373)?)?;
    // 0x65104: strb r0, [r4, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,4)?)?;
    // 0x65108: ldr r8, [sp, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::SP,20)?)?;
    // 0x6510c: b 0x654a8 <entry+0xfca8>  @ imm = #0x394
    c.emit(arm32::branch(C::Al,c.at as u32,0x654a8,false)?)?;
    // 0x65110: ldr r0, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,364)?)?;
    // 0x65114: ldr r1, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,988)?)?;
    // 0x65118: sub r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x6511c: ldr r1, [sp, #0xc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::SP,12)?)?;
    // 0x65120: bl 0x81850 <entry+0x2c050> @ imm = #0x1c728
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x65124: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65128: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x6512c: beq 0x64dec <entry+0xf5ec>  @ imm = #-0x348
    c.emit(arm32::branch(C::Eq,c.at as u32,0x64dec,false)?)?;
    // 0x65130: b 0x64e08 <entry+0xf608>  @ imm = #-0x330
    c.emit(arm32::branch(C::Al,c.at as u32,0x64e08,false)?)?;
    // 0x65134: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x65138: beq 0x65144 <entry+0xf944>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65144,false)?)?;
    // 0x6513c: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x65140: str r1, [r4, #0x3f0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1008)?)?;
    // 0x65144: ldr r1, [r7, #0x550]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1360)?)?;
    // 0x65148: mov r3, #632
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,632)?)?;
    // 0x6514c: strh r1, [r4, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(4)?,R::new(3)?,false)?)?;
    // 0x65150: mov r3, #122
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,122)?)?;
    // 0x65154: ldr r1, [r7, #0x554]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1364)?)?;
    // 0x65158: orr r3, r3, #512
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,512)?)?;
    // 0x6515c: strh r1, [r4, r3]
    c.emit(emit_halfword_register(C::Al,false,R::R1,R::new(4)?,R::new(3)?,false)?)?;
    // 0x65160: ldr r1, [r7, #0x2e8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,744)?)?;
    // 0x65164: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65168: beq 0x65178 <entry+0xf978>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65178,false)?)?;
    // 0x6516c: ldrb r1, [r8, #0x159]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,345)?)?;
    // 0x65170: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65174: beq 0x65200 <entry+0xfa00>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65200,false)?)?;
    // 0x65178: ldr r1, [r7, #0x118]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,280)?)?;
    // 0x6517c: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x65180: bne 0x651ac <entry+0xf9ac>  @ imm = #0x24
    c.emit(arm32::branch(C::Ne,c.at as u32,0x651ac,false)?)?;
    // 0x65184: ldr r1, [r7, #0x4d4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1236)?)?;
    // 0x65188: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6518c: ldrbne r1, [r4, #0x17a]
    c.emit(arm32::transfer_byte(C::Ne,true,R::R1,R::new(4)?,378)?)?;
    // 0x65190: moveq r1, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65194: add r0, r0, r1, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::R1,Shift::Lsl,2,false)?)?;
    // 0x65198: lsl r1, r9, #3
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::new(9)?,Shift::Lsl,3)?)?;
    // 0x6519c: ldr r0, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,108)?)?;
    // 0x651a0: lsr r0, r0, r1
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Mov,R::R0,R::R0,R::R0,Shift::Lsr,R::R1,false)?)?;
    // 0x651a4: strb r0, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,380)?)?;
    // 0x651a8: b 0x653c0 <entry+0xfbc0>  @ imm = #0x210
    c.emit(arm32::branch(C::Al,c.at as u32,0x653c0,false)?)?;
    // 0x651ac: ldr r1, [r7, #0x458]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,1112)?)?;
    // 0x651b0: add r5, r7, #1008
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(7)?,1008)?)?;
    // 0x651b4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x651b8: ldrne r1, [r7, #0x460]
    c.emit(arm32::transfer_word(C::Ne,true,R::R1,R::new(7)?,1120)?)?;
    // 0x651bc: cmpne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x651c0: beq 0x65234 <entry+0xfa34>  @ imm = #0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65234,false)?)?;
    // 0x651c4: ldr r6, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::SP,8)?)?;
    // 0x651c8: mov r1, r8
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(8)?))?;
    // 0x651cc: mov r2, r6
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(6)?))?;
    // 0x651d0: bl 0x761e0 <entry+0x209e0> @ imm = #0x11008
    c.emit(arm32::branch(C::Al,c.at as u32,0x761e0,true)?)?;
    // 0x651d4: ldr r1, [r4, #0x3b4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,948)?)?;
    // 0x651d8: ldr r2, [r4, #0x3b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,952)?)?;
    // 0x651dc: ldr r0, [r4, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1796)?)?;
    // 0x651e0: bl 0x71828 <entry+0x1c028> @ imm = #0xc640
    c.emit(arm32::branch(C::Al,c.at as u32,0x71828,true)?)?;
    // 0x651e4: ldr r0, [r4, #0x704]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1796)?)?;
    // 0x651e8: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x651ec: mov r2, r8
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(8)?))?;
    // 0x651f0: bl 0x71978 <entry+0x1c178> @ imm = #0xc780
    c.emit(arm32::branch(C::Al,c.at as u32,0x71978,true)?)?;
    // 0x651f4: strb r0, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,380)?)?;
    // 0x651f8: ldr r9, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::new(4)?,400)?)?;
    // 0x651fc: b 0x65310 <entry+0xfb10>  @ imm = #0x10c
    c.emit(arm32::branch(C::Al,c.at as u32,0x65310,false)?)?;
    // 0x65200: ldr r5, [sp, #0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::SP,8)?)?;
    // 0x65204: mov r0, r10
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(10)?))?;
    // 0x65208: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x6520c: bl 0x63704 <entry+0xdf04>  @ imm = #-0x1b10
    c.emit(arm32::branch(C::Al,c.at as u32,0x63704,true)?)?;
    // 0x65210: ldr r0, [r4, #0x20]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,32)?)?;
    // 0x65214: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65218: beq 0x6528c <entry+0xfa8c>  @ imm = #0x6c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6528c,false)?)?;
    // 0x6521c: ldrb r3, [r7, #0x418]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(7)?,1048)?)?;
    // 0x65220: mov r1, r5
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(5)?))?;
    // 0x65224: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x65228: str r7, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,0)?)?;
    // 0x6522c: bl 0x6b990 <entry+0x16190> @ imm = #0x675c
    c.emit(arm32::branch(C::Al,c.at as u32,0x6b990,true)?)?;
    // 0x65230: b 0x652a0 <entry+0xfaa0>  @ imm = #0x68
    c.emit(arm32::branch(C::Al,c.at as u32,0x652a0,false)?)?;
    // 0x65234: ldr r1, [r7, #0xaac]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2732)?)?;
    // 0x65238: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x6523c: beq 0x652e8 <entry+0xfae8>  @ imm = #0xa4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x652e8,false)?)?;
    // 0x65240: add r0, r10, r9, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(10)?,R::new(9)?,Shift::Lsl,2,false)?)?;
    // 0x65244: cmp r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,0)?)?;
    // 0x65248: ldr r0, [r0, #0x150]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,336)?)?;
    // 0x6524c: strb r0, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,380)?)?;
    // 0x65250: beq 0x6530c <entry+0xfb0c>  @ imm = #0xb4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6530c,false)?)?;
    // 0x65254: ldr r1, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,2744)?)?;
    // 0x65258: ldrb r1, [r1, #0xe0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R1,224)?)?;
    // 0x6525c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65260: beq 0x65310 <entry+0xfb10>  @ imm = #0xa8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65310,false)?)?;
    // 0x65264: ldrb r1, [r8, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1814)?)?;
    // 0x65268: mov r0, r2
    c.emit(arm32::mov_register(C::Al,R::R0,R::R2))?;
    // 0x6526c: bl 0x81850 <entry+0x2c050> @ imm = #0x1c5dc
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x65270: add r0, r8, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(8)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65274: ldrb r1, [r10, #0x150]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,336)?)?;
    // 0x65278: ldrb r0, [r0, #0x72a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1834)?)?;
    // 0x6527c: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65280: cmp r0, #51
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,51)?)?;
    // 0x65284: movhs r0, #51
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::R0,R::R0,51)?)?;
    // 0x65288: b 0x652f0 <entry+0xfaf0>  @ imm = #0x60
    c.emit(arm32::branch(C::Al,c.at as u32,0x652f0,false)?)?;
    // 0x6528c: ldr r2, [pc, #0x848]        @ 0x102dc <entry+0x102dc>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,2120)?)?;
    // 0x65290: mov r1, #85
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,85)?)?;
    // 0x65294: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65298: orr r1, r1, #4608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,4608)?)?;
    // 0x6529c: bl 0x17b0 <entry+0xfffffffffffabfb0> @ imm = #-0x63af4
    c.emit(arm32::branch(C::Al,c.at as u32,0x17b0,true)?)?;
    // 0x652a0: ldrb r0, [r8, #0x215]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,533)?)?;
    // 0x652a4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x652a8: beq 0x65484 <entry+0xfc84>  @ imm = #0x1d4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65484,false)?)?;
    // 0x652ac: ldrb r0, [r4, #0x25e]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,606)?)?;
    // 0x652b0: mov r1, #126
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,126)?)?;
    // 0x652b4: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x652b8: strh r0, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1,false)?)?;
    // 0x652bc: mov r1, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,384)?)?;
    // 0x652c0: ldrb r0, [r4, #0x25f]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,607)?)?;
    // 0x652c4: strh r0, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1,false)?)?;
    // 0x652c8: mov r1, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,130)?)?;
    // 0x652cc: ldrb r0, [r4, #0x260]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,608)?)?;
    // 0x652d0: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x652d4: strh r0, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1,false)?)?;
    // 0x652d8: mov r1, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,388)?)?;
    // 0x652dc: ldrb r0, [r4, #0x261]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,609)?)?;
    // 0x652e0: strh r0, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::R0,R::new(4)?,R::R1,false)?)?;
    // 0x652e4: b 0x65484 <entry+0xfc84>  @ imm = #0x198
    c.emit(arm32::branch(C::Al,c.at as u32,0x65484,false)?)?;
    // 0x652e8: add r0, r0, r9, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(9)?,Shift::Lsl,2,false)?)?;
    // 0x652ec: ldr r0, [r0, #0x6c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,108)?)?;
    // 0x652f0: strb r0, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,380)?)?;
    // 0x652f4: b 0x65310 <entry+0xfb10>  @ imm = #0x14
    c.emit(arm32::branch(C::Al,c.at as u32,0x65310,false)?)?;
    // 0x652f8: mov r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,3)?)?;
    // 0x652fc: strb r1, [r4, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,997)?)?;
    // 0x65300: ldrb r1, [r10, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,1744)?)?;
    // 0x65304: add r1, r1, #3
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,3)?)?;
    // 0x65308: b 0x650b8 <entry+0xf8b8>  @ imm = #-0x258
    c.emit(arm32::branch(C::Al,c.at as u32,0x650b8,false)?)?;
    // 0x6530c: mov r9, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,0)?)?;
    // 0x65310: add r2, r5, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(5)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x65314: ldrb r3, [r8, #0x215]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,533)?)?;
    // 0x65318: ldrb r1, [r2, #0x13]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R2,19)?)?;
    // 0x6531c: ldrb r2, [r2, #0x16]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,22)?)?;
    // 0x65320: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x65324: beq 0x653b0 <entry+0xfbb0>  @ imm = #0x84
    c.emit(arm32::branch(C::Eq,c.at as u32,0x653b0,false)?)?;
    // 0x65328: and r3, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R0,255)?)?;
    // 0x6532c: sub r6, r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(3)?,4)?)?;
    // 0x65330: cmp r6, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x65334: mov r5, r6
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::new(6)?))?;
    // 0x65338: movgt r5, r2
    c.emit(arm32::mov_register(C::Gt,R::new(5)?,R::R2))?;
    // 0x6533c: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65340: mov r6, #126
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,126)?)?;
    // 0x65344: movlt r5, r1
    c.emit(arm32::mov_register(C::Lt,R::new(5)?,R::R1))?;
    // 0x65348: orr r6, r6, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(6)?,R::new(6)?,256)?)?;
    // 0x6534c: strh r5, [r4, r6]
    c.emit(emit_halfword_register(C::Al,false,R::new(5)?,R::new(4)?,R::new(6)?,false)?)?;
    // 0x65350: sub r6, r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(6)?,R::new(3)?,1)?)?;
    // 0x65354: mov r5, #384
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,384)?)?;
    // 0x65358: cmp r6, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x6535c: movgt r6, r2
    c.emit(arm32::mov_register(C::Gt,R::new(6)?,R::R2))?;
    // 0x65360: cmp r3, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65364: movls r6, r1
    c.emit(arm32::mov_register(C::Ls,R::new(6)?,R::R1))?;
    // 0x65368: strh r6, [r4, r5]
    c.emit(emit_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::new(5)?,false)?)?;
    // 0x6536c: add r5, r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(5)?,R::new(3)?,2)?)?;
    // 0x65370: cmp r5, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x65374: mov r6, r5
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::new(5)?))?;
    // 0x65378: movhi r6, r2
    c.emit(arm32::mov_register(C::Hi,R::new(6)?,R::R2))?;
    // 0x6537c: cmp r5, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65380: mov r5, #130
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,130)?)?;
    // 0x65384: movlo r6, r1
    c.emit(arm32::mov_register(C::Cc,R::new(6)?,R::R1))?;
    // 0x65388: orr r5, r5, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,256)?)?;
    // 0x6538c: strh r6, [r4, r5]
    c.emit(emit_halfword_register(C::Al,false,R::new(6)?,R::new(4)?,R::new(5)?,false)?)?;
    // 0x65390: add r6, r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(3)?,5)?)?;
    // 0x65394: cmp r6, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x65398: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x6539c: movhi r3, r2
    c.emit(arm32::mov_register(C::Hi,R::new(3)?,R::R2))?;
    // 0x653a0: cmp r6, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R1,Shift::Lsl,0,false)?)?;
    // 0x653a4: mov r6, #388
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,388)?)?;
    // 0x653a8: movlo r3, r1
    c.emit(arm32::mov_register(C::Cc,R::new(3)?,R::R1))?;
    // 0x653ac: strh r3, [r4, r6]
    c.emit(emit_halfword_register(C::Al,false,R::new(3)?,R::new(4)?,R::new(6)?,false)?)?;
    // 0x653b0: ldrb r3, [r8, #0xca]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(8)?,202)?)?;
    // 0x653b4: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x653b8: strbne r2, [r4, #0x24b]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R2,R::new(4)?,587)?)?;
    // 0x653bc: strbne r1, [r4, #0x24a]
    c.emit(arm32::transfer_byte(C::Ne,false,R::R1,R::new(4)?,586)?)?;
    // 0x653c0: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x653c4: strb r1, [r4, #0x175]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,373)?)?;
    // 0x653c8: ldr r1, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,644)?)?;
    // 0x653cc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x653d0: bne 0x65484 <entry+0xfc84>  @ imm = #0xac
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65484,false)?)?;
    // 0x653d4: ldrb r1, [r7, #0x328]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(7)?,808)?)?;
    // 0x653d8: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x653dc: cmpeq r9, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::new(9)?,1)?)?;
    // 0x653e0: bne 0x65484 <entry+0xfc84>  @ imm = #0x9c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65484,false)?)?;
    // 0x653e4: mov r1, #78
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,78)?)?;
    // 0x653e8: ldr r2, [r7, #0xa5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,2652)?)?;
    // 0x653ec: ldr r3, [r7, #0xa60]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::new(7)?,2656)?)?;
    // 0x653f0: orr r1, r1, #256
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,256)?)?;
    // 0x653f4: ldrh r1, [r7, r1]
    c.emit(emit_halfword_register(C::Al,true,R::R1,R::new(7)?,R::R1,false)?)?;
    // 0x653f8: asr r2, r2, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::R2,Shift::Asr,4)?)?;
    // 0x653fc: asr r3, r3, #4
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,4)?)?;
    // 0x65400: mul r6, r3, r2
    c.emit(arm32::multiply_register(C::Al,R::new(6)?,R::new(3)?,R::R2)?)?;
    // 0x65404: mul r2, r6, r1
    c.emit(arm32::multiply_register(C::Al,R::R2,R::new(6)?,R::R1)?)?;
    // 0x65408: mov r1, #15424
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,15424)?)?;
    // 0x6540c: orr r1, r1, #229376
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,229376)?)?;
    // 0x65410: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65414: bhi 0x65484 <entry+0xfc84>  @ imm = #0x68
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65484,false)?)?;
    // 0x65418: ldr r1, [r7, #0x1b8]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,440)?)?;
    // 0x6541c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x65420: bne 0x65484 <entry+0xfc84>  @ imm = #0x5c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65484,false)?)?;
    // 0x65424: and r2, r0, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R2,R::R0,255)?)?;
    // 0x65428: cmp r2, #35
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,35)?)?;
    // 0x6542c: blo 0x65468 <entry+0xfc68>  @ imm = #0x34
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65468,false)?)?;
    // 0x65430: ldr r0, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2744)?)?;
    // 0x65434: mov r6, #110
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,110)?)?;
    // 0x65438: mov r1, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,18)?)?;
    // 0x6543c: mov r3, #35
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,35)?)?;
    // 0x65440: str r6, [r0, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::new(6)?,R::R0,848)?)?;
    // 0x65444: str r3, [r0, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,852)?)?;
    // 0x65448: str r1, [r0, #0x358]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,856)?)?;
    // 0x6544c: ldr r1, [pc, #0x690]        @ 0x102e4 <entry+0x102e4>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1680)?)?;
    // 0x65450: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x65454: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x367c8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65458: ldr r1, [pc, #0x678]        @ 0x102d8 <entry+0x102d8>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,1656)?)?;
    // 0x6545c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x65460: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x367d4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65464: b 0x65484 <entry+0xfc84>  @ imm = #0x18
    c.emit(arm32::branch(C::Al,c.at as u32,0x65484,false)?)?;
    // 0x65468: ldr r0, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2744)?)?;
    // 0x6546c: ldr r1, [r7, #0xd08]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3336)?)?;
    // 0x65470: str r1, [r0, #0x350]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,848)?)?;
    // 0x65474: ldr r1, [r7, #0xd0c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3340)?)?;
    // 0x65478: str r1, [r0, #0x354]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,852)?)?;
    // 0x6547c: ldr r1, [r7, #0xd10]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,3344)?)?;
    // 0x65480: str r1, [r0, #0x358]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::R0,856)?)?;
    // 0x65484: ldr r0, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,644)?)?;
    // 0x65488: ldr r9, [sp, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::new(9)?,R::SP,16)?)?;
    // 0x6548c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x65490: ldreq r0, [r7, #0xaac]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(7)?,2732)?)?;
    // 0x65494: cmpeq r0, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65498: bne 0x654a8 <entry+0xfca8>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x654a8,false)?)?;
    // 0x6549c: ldrb r1, [r4, #0x17c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,380)?)?;
    // 0x654a0: ldr r0, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2744)?)?;
    // 0x654a4: bl 0x38ec0 <entry+0xfffffffffffe36c0> @ imm = #-0x2c5ec
    c.emit(arm32::branch(C::Al,c.at as u32,0x38ec0,true)?)?;
    // 0x654a8: ldrb r0, [r4, #0x175]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,373)?)?;
    // 0x654ac: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x654b0: bne 0x654d4 <entry+0xfcd4>  @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x654d4,false)?)?;
    // 0x654b4: ldrb r0, [r4, #0x172]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,370)?)?;
    // 0x654b8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x654bc: bne 0x65698 <entry+0xfe98>  @ imm = #0x1d4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65698,false)?)?;
    // 0x654c0: ldr r1, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,400)?)?;
    // 0x654c4: mov r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,2)?)?;
    // 0x654c8: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
    // 0x654cc: beq 0x654e0 <entry+0xfce0>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x654e0,false)?)?;
    // 0x654d0: b 0x65698 <entry+0xfe98>  @ imm = #0x1c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x65698,false)?)?;
    // 0x654d4: ldr r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,400)?)?;
    // 0x654d8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x654dc: beq 0x654f8 <entry+0xfcf8>  @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x654f8,false)?)?;
    // 0x654e0: ldr r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,332)?)?;
    // 0x654e4: add r1, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R1,R::R1,1)?)?;
    // 0x654e8: str r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,332)?)?;
    // 0x654ec: b 0x65580 <entry+0xfd80>  @ imm = #0x8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x65580,false)?)?;
 c.check(0x654f0)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_654F0:[u32;2]=[0x000f1788,0x000e2788];
 for value in INLINE_DATA_654F0{c.emit(value)?;}
 // Executable A32 block 0x654f8..0x65794.
 c.check(0x654f8)?;
    // 0x654f8: ldrb r0, [r10, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,3)?)?;
    // 0x654fc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x65500: bne 0x65530 <entry+0xfd30>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65530,false)?)?;
    // 0x65504: ldr r0, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1760)?)?;
    // 0x65508: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6550c: beq 0x6551c <entry+0xfd1c>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6551c,false)?)?;
    // 0x65510: subs r0, r0, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R0,R::R0,1,true)?)?;
    // 0x65514: str r0, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1760)?)?;
    // 0x65518: bne 0x65528 <entry+0xfd28>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65528,false)?)?;
    // 0x6551c: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x65520: bic r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,16)?)?;
    // 0x65524: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x65528: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6552c: strb r0, [r10, #0x3]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,3)?)?;
    // 0x65530: ldrb r0, [r10, #0x4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(10)?,4)?)?;
    // 0x65534: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x65538: bne 0x65568 <entry+0xfd68>  @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65568,false)?)?;
    // 0x6553c: ldr r0, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,1760)?)?;
    // 0x65540: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65544: beq 0x65554 <entry+0xfd54>  @ imm = #0x8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65554,false)?)?;
    // 0x65548: subs r0, r0, #1
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::Sub,R::R0,R::R0,1,true)?)?;
    // 0x6554c: str r0, [r4, #0x6e0]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1760)?)?;
    // 0x65550: bne 0x65560 <entry+0xfd60>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65560,false)?)?;
    // 0x65554: ldr r0, [r9]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x65558: bic r0, r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Bic,R::R0,R::R0,16)?)?;
    // 0x6555c: str r0, [r9]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(9)?,0)?)?;
    // 0x65560: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65564: strb r0, [r10, #0x4]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(10)?,4)?)?;
    // 0x65568: ldr r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // 0x6556c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65570: str r0, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,364)?)?;
    // 0x65574: str r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,356)?)?;
    // 0x65578: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6557c: str r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,332)?)?;
    // 0x65580: ldr r2, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,92)?)?;
    // 0x65584: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x65588: bne 0x655c0 <entry+0xfdc0>  @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x655c0,false)?)?;
    // 0x6558c: ldrb r0, [r8, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,1814)?)?;
    // 0x65590: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65594: beq 0x655a8 <entry+0xfda8>  @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x655a8,false)?)?;
    // 0x65598: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x6559c: bl 0x5886c <entry+0x306c>  @ imm = #-0xcd38
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x655a0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x655a4: beq 0x655b0 <entry+0xfdb0>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x655b0,false)?)?;
    // 0x655a8: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x655ac: str r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,332)?)?;
    // 0x655b0: ldr r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,356)?)?;
    // 0x655b4: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x655b8: str r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,356)?)?;
    // 0x655bc: b 0x65694 <entry+0xfe94>  @ imm = #0xd0
    c.emit(arm32::branch(C::Al,c.at as u32,0x65694,false)?)?;
    // 0x655c0: cmp r1, r2
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R2,Shift::Lsl,0,false)?)?;
    // 0x655c4: ldrbeq r2, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Eq,true,R::R2,R::new(10)?,7)?)?;
    // 0x655c8: cmpeq r2, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x655cc: beq 0x65604 <entry+0xfe04>  @ imm = #0x30
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65604,false)?)?;
    // 0x655d0: ldrb r1, [r10, #0x9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,9)?)?;
    // 0x655d4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x655d8: bne 0x65618 <entry+0xfe18>  @ imm = #0x38
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65618,false)?)?;
    // 0x655dc: ldrb r1, [r10, #0x7]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,7)?)?;
    // 0x655e0: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x655e4: bne 0x65618 <entry+0xfe18>  @ imm = #0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65618,false)?)?;
    // 0x655e8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x655ec: str r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,332)?)?;
    // 0x655f0: ldrb r1, [r10, #0x8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(10)?,8)?)?;
    // 0x655f4: ldr r2, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,356)?)?;
    // 0x655f8: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x655fc: str r1, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,356)?)?;
    // 0x65600: b 0x65618 <entry+0xfe18>  @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x65618,false)?)?;
    // 0x65604: ldr r2, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(10)?,356)?)?;
    // 0x65608: add r1, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x6560c: str r1, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,356)?)?;
    // 0x65610: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65614: str r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(10)?,332)?)?;
    // 0x65618: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6561c: movne r0, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65620: strne r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Ne,false,R::R0,R::new(4)?,476)?)?;
    // 0x65624: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x65628: bl 0x5886c <entry+0x306c>  @ imm = #-0xcdc4
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x6562c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65630: bne 0x65640 <entry+0xfe40>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65640,false)?)?;
    // 0x65634: ldrb r0, [r8, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,281)?)?;
    // 0x65638: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6563c: beq 0x65838 <entry+0x10038> @ imm = #0x1f4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65838,false)?)?;
    // 0x65640: ldr r0, [r10, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,92)?)?;
    // 0x65644: ldr r1, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,332)?)?;
    // 0x65648: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x6564c: cmp r1, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65650: bne 0x6565c <entry+0xfe5c>  @ imm = #0x4
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6565c,false)?)?;
    // 0x65654: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x65658: str r0, [r4, #0x1dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,476)?)?;
    // 0x6565c: ldr r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,400)?)?;
    // 0x65660: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65664: ldreq r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(10)?,356)?)?;
    // 0x65668: addeq r0, r0, #1
    c.emit(arm32::data_immediate(C::Eq,Op::Add,R::R0,R::R0,1)?)?;
    // 0x6566c: streq r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(10)?,356)?)?;
    // 0x65670: mov r0, r8
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(8)?))?;
    // 0x65674: bl 0x5886c <entry+0x306c>  @ imm = #-0xce10
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x65678: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6567c: bne 0x6568c <entry+0xfe8c>  @ imm = #0x8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6568c,false)?)?;
    // 0x65680: ldrb r0, [r8, #0x119]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(8)?,281)?)?;
    // 0x65684: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65688: beq 0x65848 <entry+0x10048> @ imm = #0x1b8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65848,false)?)?;
    // 0x6568c: ldr r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // 0x65690: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x65694: str r0, [r10, #0x168]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(10)?,360)?)?;
    // 0x65698: ldr r0, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,644)?)?;
    // 0x6569c: cmp r0, #8192
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,8192)?)?;
    // 0x656a0: bne 0x6592c <entry+0x1012c> @ imm = #0x284
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6592c,false)?)?;
    // 0x656a4: ldrb r0, [r4, #0x63d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1597)?)?;
    // 0x656a8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x656ac: beq 0x656b8 <entry+0xfeb8>  @ imm = #0x4
    c.emit(arm32::branch(C::Eq,c.at as u32,0x656b8,false)?)?;
    // 0x656b0: ldrb r1, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,996)?)?;
    // 0x656b4: b 0x656c4 <entry+0xfec4>  @ imm = #0x8
    c.emit(arm32::branch(C::Al,c.at as u32,0x656c4,false)?)?;
    // 0x656b8: ldr r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // 0x656bc: ldrb r1, [r8, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(8)?,1814)?)?;
    // 0x656c0: bl 0x81850 <entry+0x2c050> @ imm = #0x1c188
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x656c4: ldr r3, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,24)?)?;
    // 0x656c8: ldr r2, [r4, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,36)?)?;
    // 0x656cc: ldr r0, [r4, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,24)?)?;
    // 0x656d0: ldr r7, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,400)?)?;
    // 0x656d4: ldrb r3, [r3, #0x3dc]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(3)?,R::new(3)?,988)?)?;
    // 0x656d8: ldrb r2, [r2, #0x54]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::R2,84)?)?;
    // 0x656dc: cmp r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,1)?)?;
    // 0x656e0: bne 0x65714 <entry+0xff14>  @ imm = #0x2c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65714,false)?)?;
    // 0x656e4: sub r3, r2, #9
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R2,9)?)?;
    // 0x656e8: tst r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,255)?)?;
    // 0x656ec: beq 0x65758 <entry+0xff58>  @ imm = #0x64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65758,false)?)?;
    // 0x656f0: cmp r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x656f4: bhs 0x657d4 <entry+0xffd4>  @ imm = #0xd8
    c.emit(arm32::branch(C::Cs,c.at as u32,0x657d4,false)?)?;
    // 0x656f8: mov r5, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,4)?)?;
    // 0x656fc: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x65700: moveq r5, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,3)?)?;
    // 0x65704: bl 0x5886c <entry+0x306c>  @ imm = #-0xcea0
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x65708: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x6570c: moveq r5, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x65710: b 0x657f8 <entry+0xfff8>  @ imm = #0xe0
    c.emit(arm32::branch(C::Al,c.at as u32,0x657f8,false)?)?;
    // 0x65714: tst r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::new(7)?,255)?)?;
    // 0x65718: beq 0x65778 <entry+0xff78>  @ imm = #0x58
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65778,false)?)?;
    // 0x6571c: mov r3, #5
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x65720: cmp r2, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,17)?)?;
    // 0x65724: bhi 0x6588c <entry+0x1008c> @ imm = #0x160
    c.emit(arm32::branch(C::Hi,c.at as u32,0x6588c,false)?)?;
    // 0x65728: mov r5, #33024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,33024)?)?;
    // 0x6572c: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x65730: orr r5, r5, #65536
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,65536)?)?;
    // 0x65734: tst r5, r6, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,R::R2,false)?)?;
    // 0x65738: bne 0x6581c <entry+0x1001c> @ imm = #0xdc
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6581c,false)?)?;
    // 0x6573c: mov r5, #1536
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1536)?)?;
    // 0x65740: tst r5, r6, lsl r2
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,R::R2,false)?)?;
    // 0x65744: beq 0x65850 <entry+0x10050> @ imm = #0x104
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65850,false)?)?;
    // 0x65748: mov r3, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,4)?)?;
    // 0x6574c: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x65750: moveq r3, #3
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,3)?)?;
    // 0x65754: b 0x6588c <entry+0x1008c> @ imm = #0x130
    c.emit(arm32::branch(C::Al,c.at as u32,0x6588c,false)?)?;
    // 0x65758: cmp r3, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,3)?)?;
    // 0x6575c: blo 0x65800 <entry+0x10000> @ imm = #0x9c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65800,false)?)?;
    // 0x65760: cmp r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,15)?)?;
    // 0x65764: beq 0x65800 <entry+0x10000> @ imm = #0x94
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65800,false)?)?;
    // 0x65768: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x6576c: cmp r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,8)?)?;
    // 0x65770: bne 0x65894 <entry+0x10094> @ imm = #0x11c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65894,false)?)?;
    // 0x65774: b 0x65aa8 <entry+0x102a8> @ imm = #0x32c
    c.emit(arm32::branch(C::Al,c.at as u32,0x65aa8,false)?)?;
    // 0x65778: sub r3, r2, #8
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::new(3)?,R::R2,8)?)?;
    // 0x6577c: mov r6, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x65780: cmp r3, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,9)?)?;
    // 0x65784: bhi 0x65894 <entry+0x10094> @ imm = #0x108
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65894,false)?)?;
    // 0x65788: lsl r3, r3, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Lsl,2)?)?;
    // 0x6578c: add r7, pc, #0
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::PC,0)?)?;
    // 0x65790: ldr pc, [r3, r7]
    c.emit(arm32::transfer_word_register(C::Al,true,R::PC,R::new(3)?,R::new(7)?,Shift::Lsl,0)?)?;
 c.check(0x65794)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_65794:[u32;10]=[0x00065aa8,0x000657bc,0x000657bc,0x00065800,0x00065894,0x00065894,0x00065894,0x000657bc,0x000657bc,0x00065ab4];
 for value in INLINE_DATA_65794{c.emit(value)?;}
 // Executable A32 block 0x657bc..0x65ad8.
 c.check(0x657bc)?;
    // 0x657bc: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x657c0: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x657c4: moveq r6, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x657c8: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x657cc: moveq r6, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x657d0: b 0x65894 <entry+0x10094> @ imm = #0xbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x65894,false)?)?;
    // 0x657d4: cmp r2, #15
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,15)?)?;
    // 0x657d8: movne r5, #5
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(5)?,R::R0,5)?)?;
    // 0x657dc: cmpne r2, #8
    c.emit(arm32::data_immediate(C::Ne,Op::Cmp,R::R0,R::R2,8)?)?;
    // 0x657e0: bne 0x657f8 <entry+0xfff8>  @ imm = #0x10
    c.emit(arm32::branch(C::Ne,c.at as u32,0x657f8,false)?)?;
    // 0x657e4: lsl r0, r1, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsl,24)?)?;
    // 0x657e8: and r5, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(5)?,R::R1,1)?)?;
    // 0x657ec: asr r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,24)?)?;
    // 0x657f0: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x657f4: addgt r5, r5, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Add,R::new(5)?,R::new(5)?,1)?)?;
    // 0x657f8: and r0, r5, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(5)?,255)?)?;
    // 0x657fc: b 0x65890 <entry+0x10090> @ imm = #0x8c
    c.emit(arm32::branch(C::Al,c.at as u32,0x65890,false)?)?;
    // 0x65800: mov r6, #3
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,3)?)?;
    // 0x65804: tst r1, #255
    c.emit(arm32::data_immediate(C::Al,Op::Tst,R::R0,R::R1,255)?)?;
    // 0x65808: moveq r6, #2
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x6580c: bl 0x5886c <entry+0x306c>  @ imm = #-0xcfa8
    c.emit(arm32::branch(C::Al,c.at as u32,0x5886c,true)?)?;
    // 0x65810: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65814: moveq r6, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x65818: b 0x65894 <entry+0x10094> @ imm = #0x74
    c.emit(arm32::branch(C::Al,c.at as u32,0x65894,false)?)?;
    // 0x6581c: cmp r2, #16
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,16)?)?;
    // 0x65820: bne 0x65878 <entry+0x10078> @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65878,false)?)?;
    // 0x65824: and r0, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(7)?,255)?)?;
    // 0x65828: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x6582c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x65830: moveq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x65834: b 0x6588c <entry+0x1008c> @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x6588c,false)?)?;
    // 0x65838: ldr r0, [r10, #0x14c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,332)?)?;
    // 0x6583c: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65840: bne 0x6565c <entry+0xfe5c>  @ imm = #-0x1ec
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6565c,false)?)?;
    // 0x65844: b 0x65654 <entry+0xfe54>  @ imm = #-0x1f8
    c.emit(arm32::branch(C::Al,c.at as u32,0x65654,false)?)?;
    // 0x65848: ldr r0, [r10, #0x164]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,356)?)?;
    // 0x6584c: b 0x65694 <entry+0xfe94>  @ imm = #-0x1c0
    c.emit(arm32::branch(C::Al,c.at as u32,0x65694,false)?)?;
    // 0x65850: cmp r2, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,17)?)?;
    // 0x65854: bne 0x6588c <entry+0x1008c> @ imm = #0x30
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6588c,false)?)?;
    // 0x65858: ldrb r0, [r0, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1814)?)?;
    // 0x6585c: and r1, r7, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R1,R::new(7)?,255)?)?;
    // 0x65860: mov r3, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,2)?)?;
    // 0x65864: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x65868: moveq r3, #0
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x6586c: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x65870: moveq r3, #5
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(3)?,R::R0,5)?)?;
    // 0x65874: b 0x6588c <entry+0x1008c> @ imm = #0x10
    c.emit(arm32::branch(C::Al,c.at as u32,0x6588c,false)?)?;
    // 0x65878: lsl r0, r1, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R1,Shift::Lsl,24)?)?;
    // 0x6587c: and r3, r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::And,R::new(3)?,R::R1,1)?)?;
    // 0x65880: asr r0, r0, #24
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R0,Shift::Asr,24)?)?;
    // 0x65884: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65888: addgt r3, r3, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Add,R::new(3)?,R::new(3)?,1)?)?;
    // 0x6588c: and r0, r3, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(3)?,255)?)?;
    // 0x65890: add r6, r0, #5
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::R0,5)?)?;
    // 0x65894: add r0, r4, r6, lsl #2
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(4)?,R::new(6)?,Shift::Lsl,2,false)?)?;
    // 0x65898: ldr r0, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,76)?)?;
    // 0x6589c: str r0, [r4, #0x518]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1304)?)?;
    // 0x658a0: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // 0x658a4: ldr r0, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2744)?)?;
    // 0x658a8: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x658ac: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x658b0: beq 0x65904 <entry+0x10104> @ imm = #0x4c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65904,false)?)?;
    // 0x658b4: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x658b8: bne 0x65910 <entry+0x10110> @ imm = #0x50
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65910,false)?)?;
    // 0x658bc: ldr r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,988)?)?;
    // 0x658c0: mvn r7, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(7)?,R::R0,0)?)?;
    // 0x658c4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x658c8: bne 0x658ec <entry+0x100ec> @ imm = #0x1c
    c.emit(arm32::branch(C::Ne,c.at as u32,0x658ec,false)?)?;
    // 0x658cc: mov r1, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,20)?)?;
    // 0x658d0: orr r1, r1, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R1,R::R1,1024)?)?;
    // 0x658d4: strh r7, [r4, r1]
    c.emit(emit_halfword_register(C::Al,false,R::new(7)?,R::new(4)?,R::R1,false)?)?;
    // 0x658d8: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x658dc: str r1, [r4, #0x400]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1024)?)?;
    // 0x658e0: str r1, [r4, #0x404]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1028)?)?;
    // 0x658e4: str r1, [r4, #0x408]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1032)?)?;
    // 0x658e8: str r1, [r4, #0x40c]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1036)?)?;
    // 0x658ec: ldrb r1, [r4, #0x63d]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,1597)?)?;
    // 0x658f0: ldrb r6, [r8, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(6)?,R::new(8)?,1814)?)?;
    // 0x658f4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x658f8: beq 0x65950 <entry+0x10150> @ imm = #0x50
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65950,false)?)?;
    // 0x658fc: ldrb r1, [r4, #0x3e4]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,996)?)?;
    // 0x65900: b 0x65958 <entry+0x10158> @ imm = #0x50
    c.emit(arm32::branch(C::Al,c.at as u32,0x65958,false)?)?;
    // 0x65904: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65908: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x6590c: b 0x65920 <entry+0x10120> @ imm = #0xc
    c.emit(arm32::branch(C::Al,c.at as u32,0x65920,false)?)?;
    // 0x65910: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65914: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65918: movgt r1, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x6591c: mov r0, r1
    c.emit(arm32::mov_register(C::Al,R::R0,R::R1))?;
    // 0x65920: strb r1, [r4, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1001)?)?;
    // 0x65924: strb r0, [r4, #0x3ea]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::new(4)?,1002)?)?;
    // 0x65928: ldr r0, [r7, #0x284]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,644)?)?;
    // 0x6592c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x65930: mov r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x65934: ldreq r0, [r4, #0x190]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::new(4)?,400)?)?;
    // 0x65938: addeq r0, r4, r0, lsl #2
    c.emit(arm32::data_register(C::Eq,Op::Add,R::R0,R::new(4)?,R::R0,Shift::Lsl,2,false)?)?;
    // 0x6593c: ldreq r0, [r0, #0x4c]
    c.emit(arm32::transfer_word(C::Eq,true,R::R0,R::R0,76)?)?;
    // 0x65940: streq r0, [r4, #0x518]
    c.emit(arm32::transfer_word(C::Eq,false,R::R0,R::new(4)?,1304)?)?;
    // 0x65944: mov r0, r6
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(6)?))?;
    // 0x65948: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x6594c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x65950: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x65954: bl 0x81850 <entry+0x2c050> @ imm = #0x1bef4
    c.emit(arm32::branch(C::Al,c.at as u32,0x81850,true)?)?;
    // 0x65958: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x6595c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65960: cmpne r0, r6, lsr #1
    c.emit(arm32::data_register(C::Ne,Op::Cmp,R::R0,R::R0,R::new(6)?,Shift::Lsr,1,false)?)?;
    // 0x65964: beq 0x659ac <entry+0x101ac> @ imm = #0x40
    c.emit(arm32::branch(C::Eq,c.at as u32,0x659ac,false)?)?;
    // 0x65968: lsr r5, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Lsr,1)?)?;
    // 0x6596c: mov r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,1)?)?;
    // 0x65970: mov r3, r6
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::new(6)?))?;
    // 0x65974: cmp r5, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x65978: mov r2, r5
    c.emit(arm32::mov_register(C::Al,R::R2,R::new(5)?))?;
    // 0x6597c: bhs 0x65994 <entry+0x10194> @ imm = #0x10
    c.emit(arm32::branch(C::Cs,c.at as u32,0x65994,false)?)?;
    // 0x65980: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65984: beq 0x659ac <entry+0x101ac> @ imm = #0x20
    c.emit(arm32::branch(C::Eq,c.at as u32,0x659ac,false)?)?;
    // 0x65988: add r2, r2, r3
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::R2,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x6598c: cmp r2, r6
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(6)?,Shift::Lsl,0,false)?)?;
    // 0x65990: blo 0x65980 <entry+0x10180> @ imm = #-0x18
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65980,false)?)?;
    // 0x65994: asr r5, r5, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(5)?,Shift::Asr,1)?)?;
    // 0x65998: add r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R0,R::R0,1)?)?;
    // 0x6599c: asr r3, r3, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(3)?,R::new(3)?,Shift::Asr,1)?)?;
    // 0x659a0: cmp r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(5)?,0)?)?;
    // 0x659a4: bgt 0x65974 <entry+0x10174> @ imm = #-0x38
    c.emit(arm32::branch(C::Gt,c.at as u32,0x65974,false)?)?;
    // 0x659a8: sub r0, r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::R0,R::R0,1)?)?;
    // 0x659ac: mov r2, #20
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,20)?)?;
    // 0x659b0: mov r1, r4
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(4)?))?;
    // 0x659b4: cmp r0, #3
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,3)?)?;
    // 0x659b8: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // 0x659bc: movge r0, #3
    c.emit(arm32::data_immediate(C::Ge,Op::Mov,R::R0,R::R0,3)?)?;
    // 0x659c0: ldrsh r2, [r1, r2]!
    c.emit(emit_signed_halfword_register(C::Al,R::R2,R::R1,R::R2,true)?)?;
    // 0x659c4: cmp r2, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x659c8: ble 0x659f8 <entry+0x101f8> @ imm = #0x28
    c.emit(arm32::branch(C::Le,c.at as u32,0x659f8,false)?)?;
    // 0x659cc: mov r3, #1040
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1040)?)?;
    // 0x659d0: add r7, r4, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,1024)?)?;
    // 0x659d4: ldrh r3, [r4, r3]
    c.emit(emit_halfword_register(C::Al,true,R::new(3)?,R::new(4)?,R::new(3)?,false)?)?;
    // 0x659d8: add r2, r7, r2, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R2,R::new(7)?,R::R2,Shift::Lsl,1,false)?)?;
    // 0x659dc: strh r3, [r2]
    c.emit(arm32::store_halfword(C::Al,R::new(3)?,R::R2,0)?)?;
    // 0x659e0: mov r2, #18
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,18)?)?;
    // 0x659e4: orr r2, r2, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,1024)?)?;
    // 0x659e8: ldrsh r3, [r1]
    c.emit(arm32::load_signed_halfword(C::Al,R::new(3)?,R::R1,0)?)?;
    // 0x659ec: ldrh r2, [r4, r2]
    c.emit(emit_halfword_register(C::Al,true,R::R2,R::new(4)?,R::R2,false)?)?;
    // 0x659f0: add r3, r7, r3, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(3)?,R::new(7)?,R::new(3)?,Shift::Lsl,1,false)?)?;
    // 0x659f4: strh r2, [r3, #8]
    c.emit(arm32::store_halfword(C::Al,R::R2,R::new(3)?,8)?)?;
    // 0x659f8: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x659fc: beq 0x65a48 <entry+0x10248> @ imm = #0x44
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65a48,false)?)?;
    // 0x65a00: mov r2, #254
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,254)?)?;
    // 0x65a04: add r7, r4, r0, lsl #1
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(4)?,R::R0,Shift::Lsl,1,false)?)?;
    // 0x65a08: orr r2, r2, #768
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,768)?)?;
    // 0x65a0c: ldrh r3, [r7, r2]
    c.emit(emit_halfword_register(C::Al,true,R::new(3)?,R::new(7)?,R::R2,false)?)?;
    // 0x65a10: ldr r2, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::SP,24)?)?;
    // 0x65a14: ldr r6, [r2, #0xa6c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::R2,2668)?)?;
    // 0x65a18: mov r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,0)?)?;
    // 0x65a1c: asr r5, r6, #2
    c.emit(arm32::mov_shifted_register(C::Al,R::new(5)?,R::new(6)?,Shift::Asr,2)?)?;
    // 0x65a20: asr r6, r6, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::new(6)?,R::new(6)?,Shift::Asr,1)?)?;
    // 0x65a24: cmp r5, r3
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(5)?,R::new(3)?,Shift::Lsl,0,false)?)?;
    // 0x65a28: mov r5, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,6)?)?;
    // 0x65a2c: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x65a30: orr r5, r5, #1024
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(5)?,R::new(5)?,1024)?)?;
    // 0x65a34: movle r3, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x65a38: ldrh r7, [r7, r5]
    c.emit(emit_halfword_register(C::Al,true,R::new(7)?,R::new(7)?,R::new(5)?,false)?)?;
    // 0x65a3c: cmp r6, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x65a40: movle r2, #1
    c.emit(arm32::data_immediate(C::Le,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x65a44: b 0x65a50 <entry+0x10250> @ imm = #0x4
    c.emit(arm32::branch(C::Al,c.at as u32,0x65a50,false)?)?;
    // 0x65a48: mov r3, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x65a4c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x65a50: strh r0, [r1]
    c.emit(arm32::store_halfword(C::Al,R::R0,R::R1,0)?)?;
    // 0x65a54: ldr r7, [sp, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::SP,24)?)?;
    // 0x65a58: ldr r0, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2744)?)?;
    // 0x65a5c: ldr r1, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R0,20)?)?;
    // 0x65a60: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65a64: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65a68: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65a6c: movgt r1, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65a70: cmp r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(3)?,0)?)?;
    // 0x65a74: movne r3, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x65a78: ands r1, r3, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::new(3)?,R::R1,Shift::Lsl,0,true)?)?;
    // 0x65a7c: strb r1, [r4, #0x3e9]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(4)?,1001)?)?;
    // 0x65a80: beq 0x65924 <entry+0x10124> @ imm = #-0x164
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65924,false)?)?;
    // 0x65a84: ldr r0, [r7, #0xab8]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(7)?,2744)?)?;
    // 0x65a88: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65a8c: ldr r0, [r0, #0x14]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R0,20)?)?;
    // 0x65a90: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65a94: movgt r1, #1
    c.emit(arm32::data_immediate(C::Gt,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65a98: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x65a9c: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x65aa0: and r0, r2, r1
    c.emit(arm32::data_register(C::Al,Op::And,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65aa4: b 0x65924 <entry+0x10124> @ imm = #-0x188
    c.emit(arm32::branch(C::Al,c.at as u32,0x65924,false)?)?;
    // 0x65aa8: ands r6, r1, #255
    c.emit(arm32::data_immediate_with_flags(C::Al,Op::And,R::new(6)?,R::R1,255,true)?)?;
    // 0x65aac: movne r6, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::new(6)?,R::R0,1)?)?;
    // 0x65ab0: b 0x65894 <entry+0x10094> @ imm = #-0x224
    c.emit(arm32::branch(C::Al,c.at as u32,0x65894,false)?)?;
    // 0x65ab4: ldrb r0, [r0, #0x716]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R0,1814)?)?;
    // 0x65ab8: mov r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,2)?)?;
    // 0x65abc: cmp r0, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,1)?)?;
    // 0x65ac0: moveq r6, #4
    c.emit(arm32::data_immediate(C::Eq,Op::Mov,R::new(6)?,R::R0,4)?)?;
    // 0x65ac4: b 0x65894 <entry+0x10094> @ imm = #-0x238
    c.emit(arm32::branch(C::Al,c.at as u32,0x65894,false)?)?;
    // 0x65ac8: ldr r3, [sp, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::SP,4)?)?;
    // 0x65acc: ldrb r0, [r3, r2]
    c.emit(arm32::transfer_byte_register(C::Al,true,R::R0,R::new(3)?,R::R2,Shift::Lsl,0)?)?;
    // 0x65ad0: str r0, [r4, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,988)?)?;
    // 0x65ad4: b 0x64b20 <entry+0xf320>  @ imm = #-0xfbc
    c.emit(arm32::branch(C::Al,c.at as u32,0x64b20,false)?)?;
 c.check(0x65ad8)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_65AD8:[u32;4]=[0x000e2788,0x000f1748,0x000f17a8,0x000f17be];
 for value in INLINE_DATA_65AD8{c.emit(value)?;}
 // Executable A32 block 0x65ae8..0x65d60.
 c.check(0x65ae8)?;
    // 0x65ae8: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x65aec: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x65af0: sub sp, sp, #12
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,12)?)?;
    // 0x65af4: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x65af8: ldr r0, [r1, #0x10]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R1,16)?)?;
    // 0x65afc: mov r6, r2
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R2))?;
    // 0x65b00: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65b04: beq 0x65bdc <entry+0x103dc> @ imm = #0xd0
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65bdc,false)?)?;
    // 0x65b08: ldr r2, [r1]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R1,0)?)?;
    // 0x65b0c: ldr r5, [r2]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::R2,0)?)?;
    // 0x65b10: ldr r2, [r5, #0xa8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,168)?)?;
    // 0x65b14: cmp r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,1)?)?;
    // 0x65b18: bhi 0x65b28 <entry+0x10328> @ imm = #0x8
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65b28,false)?)?;
    // 0x65b1c: ldrb r2, [r4, #0x6d8]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(4)?,1752)?)?;
    // 0x65b20: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x65b24: beq 0x65bf8 <entry+0x103f8> @ imm = #0xcc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65bf8,false)?)?;
    // 0x65b28: ldr r1, [r1, #0x18]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,24)?)?;
    // 0x65b2c: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x65b30: blo 0x65be4 <entry+0x103e4> @ imm = #0xac
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65be4,false)?)?;
    // 0x65b34: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65b38: bne 0x65bec <entry+0x103ec> @ imm = #0xac
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65bec,false)?)?;
    // 0x65b3c: ldr r0, [r6, #0x7dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(6)?,2012)?)?;
    // 0x65b40: ldr r1, [r6, #0x7e0]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(6)?,2016)?)?;
    // 0x65b44: ldr r7, [r4, #0x1c0]
    c.emit(arm32::transfer_word(C::Al,true,R::new(7)?,R::new(4)?,448)?)?;
    // 0x65b48: ldr r8, [r4, #0x23c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(8)?,R::new(4)?,572)?)?;
    // 0x65b4c: mov r9, #100
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(9)?,R::R0,100)?)?;
    // 0x65b50: mul r2, r1, r0
    c.emit(arm32::multiply_register(C::Al,R::R2,R::R1,R::R0)?)?;
    // 0x65b54: asr r0, r2, #31
    c.emit(arm32::mov_shifted_register(C::Al,R::R0,R::R2,Shift::Asr,31)?)?;
    // 0x65b58: add r1, r2, r0, lsr #26
    c.emit(arm32::data_register(C::Al,Op::Add,R::R1,R::R2,R::R0,Shift::Lsr,26,false)?)?;
    // 0x65b5c: rsb r2, r7, r1, asr #6
    c.emit(arm32::data_register(C::Al,Op::Rsb,R::R2,R::new(7)?,R::R1,Shift::Asr,6,false)?)?;
    // 0x65b60: asr r1, r1, #6
    c.emit(arm32::mov_shifted_register(C::Al,R::R1,R::R1,Shift::Asr,6)?)?;
    // 0x65b64: mul r0, r2, r9
    c.emit(arm32::multiply_register(C::Al,R::R0,R::R2,R::new(9)?)?)?;
    // 0x65b68: blx 0x83c4c <entry+0x2e44c> @ imm = #0x1e0dc
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x65b6c: mov r10, r0
    c.emit(arm32::mov_register(C::Al,R::new(10)?,R::R0))?;
    // 0x65b70: mul r0, r8, r9
    c.emit(arm32::multiply_register(C::Al,R::R0,R::new(8)?,R::new(9)?)?)?;
    // 0x65b74: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x65b78: blx 0x83c4c <entry+0x2e44c> @ imm = #0x1e0cc
    c.emit(arm32::branch_exchange_immediate(c.at as u32,0x83c4c)?)?;
    // 0x65b7c: ldrd r8, r9, [r5, #200]
    c.emit(arm32::transfer_doubleword(C::Al,true,R::new(8)?,R::new(5)?,200)?)?;
    // 0x65b80: ldr r6, [r6, #0x4]
    c.emit(arm32::transfer_word(C::Al,true,R::new(6)?,R::new(6)?,4)?)?;
    // 0x65b84: mov r3, r0
    c.emit(arm32::mov_register(C::Al,R::new(3)?,R::R0))?;
    // 0x65b88: ldrb r0, [r4, #0x6d9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(4)?,1753)?)?;
    // 0x65b8c: ldr r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(4)?,1732)?)?;
    // 0x65b90: add r7, r8, r9
    c.emit(arm32::data_register(C::Al,Op::Add,R::new(7)?,R::new(8)?,R::new(9)?,Shift::Lsl,0,false)?)?;
    // 0x65b94: cmp r6, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,2)?)?;
    // 0x65b98: lsr r2, r7, #1
    c.emit(arm32::mov_shifted_register(C::Al,R::R2,R::new(7)?,Shift::Lsr,1)?)?;
    // 0x65b9c: bne 0x65c0c <entry+0x1040c> @ imm = #0x68
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65c0c,false)?)?;
    // 0x65ba0: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65ba4: beq 0x65c54 <entry+0x10454> @ imm = #0xa8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65c54,false)?)?;
    // 0x65ba8: cmp r7, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,33)?)?;
    // 0x65bac: bhi 0x65c98 <entry+0x10498> @ imm = #0xe4
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65c98,false)?)?;
    // 0x65bb0: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65bb4: cmp r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,12)?)?;
    // 0x65bb8: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65bbc: blo 0x65ca0 <entry+0x104a0> @ imm = #0xdc
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65ca0,false)?)?;
    // 0x65bc0: mov r7, #10
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x65bc4: cmp r0, #29
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,29)?)?;
    // 0x65bc8: bhi 0x65c90 <entry+0x10490> @ imm = #0xc0
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65c90,false)?)?;
    // 0x65bcc: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x65bd0: cmp r0, #25
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,25)?)?;
    // 0x65bd4: movhs r7, #6
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // 0x65bd8: b 0x65c90 <entry+0x10490> @ imm = #0xb0
    c.emit(arm32::branch(C::Al,c.at as u32,0x65c90,false)?)?;
    // 0x65bdc: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x65be0: b 0x65d54 <entry+0x10554> @ imm = #0x16c
    c.emit(arm32::branch(C::Al,c.at as u32,0x65d54,false)?)?;
    // 0x65be4: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65be8: beq 0x65c04 <entry+0x10404> @ imm = #0x14
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65c04,false)?)?;
    // 0x65bec: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65bf0: str r0, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::new(4)?,1732)?)?;
    // 0x65bf4: b 0x65d34 <entry+0x10534> @ imm = #0x138
    c.emit(arm32::branch(C::Al,c.at as u32,0x65d34,false)?)?;
    // 0x65bf8: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x65bfc: str r5, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::new(4)?,1732)?)?;
    // 0x65c00: b 0x65d38 <entry+0x10538> @ imm = #0x130
    c.emit(arm32::branch(C::Al,c.at as u32,0x65d38,false)?)?;
    // 0x65c04: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x65c08: b 0x65d38 <entry+0x10538> @ imm = #0x128
    c.emit(arm32::branch(C::Al,c.at as u32,0x65d38,false)?)?;
    // 0x65c0c: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65c10: beq 0x65cb0 <entry+0x104b0> @ imm = #0x98
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65cb0,false)?)?;
    // 0x65c14: cmp r7, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,19)?)?;
    // 0x65c18: cmpls r10, #47
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::new(10)?,47)?)?;
    // 0x65c1c: bhi 0x65cf4 <entry+0x104f4> @ imm = #0xd0
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65cf4,false)?)?;
    // 0x65c20: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65c24: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x65c28: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65c2c: blo 0x65cfc <entry+0x104fc> @ imm = #0xc8
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65cfc,false)?)?;
    // 0x65c30: mov r7, #14
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,14)?)?;
    // 0x65c34: cmp r0, #29
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,29)?)?;
    // 0x65c38: movls r7, #8
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,8)?)?;
    // 0x65c3c: cmpls r0, #24
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,24)?)?;
    // 0x65c40: bhi 0x65cec <entry+0x104ec> @ imm = #0xa4
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65cec,false)?)?;
    // 0x65c44: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x65c48: cmp r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x65c4c: movhs r7, #5
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(7)?,R::R0,5)?)?;
    // 0x65c50: b 0x65cec <entry+0x104ec> @ imm = #0x94
    c.emit(arm32::branch(C::Al,c.at as u32,0x65cec,false)?)?;
    // 0x65c54: cmp r7, #33
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,33)?)?;
    // 0x65c58: bhi 0x65c98 <entry+0x10498> @ imm = #0x38
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65c98,false)?)?;
    // 0x65c5c: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65c60: cmp r7, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,12)?)?;
    // 0x65c64: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65c68: blo 0x65ca0 <entry+0x104a0> @ imm = #0x30
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65ca0,false)?)?;
    // 0x65c6c: mov r7, #22
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,22)?)?;
    // 0x65c70: cmp r0, #29
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,29)?)?;
    // 0x65c74: movls r7, #12
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,12)?)?;
    // 0x65c78: cmpls r0, #24
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,24)?)?;
    // 0x65c7c: bhi 0x65c90 <entry+0x10490> @ imm = #0xc
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65c90,false)?)?;
    // 0x65c80: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x65c84: cmp r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x65c88: blo 0x65c90 <entry+0x10490> @ imm = #0x0
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65c90,false)?)?;
    // 0x65c8c: mov r7, #6
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,6)?)?;
    // 0x65c90: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x65c94: bhs 0x65ca0 <entry+0x104a0> @ imm = #0x4
    c.emit(arm32::branch(C::Cs,c.at as u32,0x65ca0,false)?)?;
    // 0x65c98: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65c9c: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65ca0: cmp r9, #12
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,12)?)?;
    // 0x65ca4: cmpls r8, #29
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::new(8)?,29)?)?;
    // 0x65ca8: blo 0x65d14 <entry+0x10514> @ imm = #0x64
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65d14,false)?)?;
    // 0x65cac: b 0x65d0c <entry+0x1050c> @ imm = #0x58
    c.emit(arm32::branch(C::Al,c.at as u32,0x65d0c,false)?)?;
    // 0x65cb0: cmp r7, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,19)?)?;
    // 0x65cb4: cmpls r10, #34
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::new(10)?,34)?)?;
    // 0x65cb8: bhi 0x65cf4 <entry+0x104f4> @ imm = #0x34
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65cf4,false)?)?;
    // 0x65cbc: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65cc0: cmp r7, #8
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(7)?,8)?)?;
    // 0x65cc4: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65cc8: blo 0x65cfc <entry+0x104fc> @ imm = #0x2c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65cfc,false)?)?;
    // 0x65ccc: mov r7, #28
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,28)?)?;
    // 0x65cd0: cmp r0, #29
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,29)?)?;
    // 0x65cd4: movls r7, #18
    c.emit(arm32::data_immediate(C::Ls,Op::Mov,R::new(7)?,R::R0,18)?)?;
    // 0x65cd8: cmpls r0, #24
    c.emit(arm32::data_immediate(C::Ls,Op::Cmp,R::R0,R::R0,24)?)?;
    // 0x65cdc: bhi 0x65cec <entry+0x104ec> @ imm = #0x8
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65cec,false)?)?;
    // 0x65ce0: mov r7, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(7)?,R::R0,1)?)?;
    // 0x65ce4: cmp r0, #20
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,20)?)?;
    // 0x65ce8: movhs r7, #10
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(7)?,R::R0,10)?)?;
    // 0x65cec: cmp r3, r7
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(3)?,R::new(7)?,Shift::Lsl,0,false)?)?;
    // 0x65cf0: bhs 0x65cfc <entry+0x104fc> @ imm = #0x4
    c.emit(arm32::branch(C::Cs,c.at as u32,0x65cfc,false)?)?;
    // 0x65cf4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65cf8: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65cfc: cmp r9, #9
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(9)?,9)?)?;
    // 0x65d00: bhi 0x65d0c <entry+0x1050c> @ imm = #0x4
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65d0c,false)?)?;
    // 0x65d04: cmp r8, #19
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,19)?)?;
    // 0x65d08: blo 0x65d14 <entry+0x10514> @ imm = #0x4
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65d14,false)?)?;
    // 0x65d0c: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65d10: str r1, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(4)?,1732)?)?;
    // 0x65d14: str r10, [sp]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::SP,0)?)?;
    // 0x65d18: stmib sp, {r0, r1}
    c.emit(arm32::transfer_multiple(C::Al,false,R::SP,R::R0.bit()|R::R1.bit(),arm32::BlockMode::IncrementBefore,false)?)?;
    // 0x65d1c: ldr r1, [pc, #0x3c]         @ 0x10560 <entry+0x10560>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,60)?)?;
    // 0x65d20: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x65d24: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x37098
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65d28: ldr r1, [pc, #0x34]         @ 0x10564 <entry+0x10564>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,52)?)?;
    // 0x65d2c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x65d30: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x370a4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65d34: mov r5, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,1)?)?;
    // 0x65d38: ldr r2, [r4, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(4)?,1732)?)?;
    // 0x65d3c: ldr r1, [pc, #0x24]         @ 0x10568 <entry+0x10568>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,36)?)?;
    // 0x65d40: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x65d44: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x370b8
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65d48: ldr r1, [pc, #0x14]         @ 0x10564 <entry+0x10564>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,20)?)?;
    // 0x65d4c: mov r0, #4
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,4)?)?;
    // 0x65d50: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x370c4
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65d54: mov r0, r5
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(5)?))?;
    // 0x65d58: sub sp, r11, #28
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::R11,28)?)?;
    // 0x65d5c: pop {r4, r5, r6, r7, r8, r9, r10, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
 c.check(0x65d60)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_65D60:[u32;3]=[0x000f1950,0x000e2788,0x000f182b];
 for value in INLINE_DATA_65D60{c.emit(value)?;}
 // Executable A32 block 0x65d6c..0x65e18.
 c.check(0x65d6c)?;
    // 0x65d6c: push {r11, lr}
    c.emit(arm32::push_registers(C::Al,R::R11.bit()|R::LR.bit())?)?;
    // 0x65d70: mov r11, sp
    c.emit(arm32::mov_register(C::Al,R::R11,R::SP))?;
    // 0x65d74: ldr r2, [r0, #0x168]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,360)?)?;
    // 0x65d78: ldr r3, [r0, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,364)?)?;
    // 0x65d7c: sub r2, r3, r2
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::new(3)?,R::R2,Shift::Lsl,0,false)?)?;
    // 0x65d80: adds r12, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Add,R::R12,R::R2,R::R1,Shift::Lsl,0,true)?)?;
    // 0x65d84: beq 0x65df0 <entry+0x105f0> @ imm = #0x64
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65df0,false)?)?;
    // 0x65d88: ldr r2, [r0, #0x6c4]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R0,1732)?)?;
    // 0x65d8c: ldr r3, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(3)?,R::R0,92)?)?;
    // 0x65d90: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65d94: mov lr, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::LR,R::R0,1)?)?;
    // 0x65d98: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x65d9c: str r3, [r0, #0x6d4]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1748)?)?;
    // 0x65da0: mov r3, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,0)?)?;
    // 0x65da4: movne r1, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x65da8: cmp r12, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R12,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65dac: movhs r3, #1
    c.emit(arm32::data_immediate(C::Cs,Op::Mov,R::new(3)?,R::R0,1)?)?;
    // 0x65db0: cmp r2, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,0)?)?;
    // 0x65db4: movne r2, #1
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x65db8: ands r1, r2, r3
    c.emit(arm32::data_register(C::Al,Op::And,R::R1,R::R2,R::new(3)?,Shift::Lsl,0,true)?)?;
    // 0x65dbc: add r2, r0, #12
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R0,12)?)?;
    // 0x65dc0: add r3, r0, #44
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(3)?,R::R0,44)?)?;
    // 0x65dc4: movne lr, #2
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::LR,R::R0,2)?)?;
    // 0x65dc8: addne r2, r0, #13
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::R2,R::R0,13)?)?;
    // 0x65dcc: cmp r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,0)?)?;
    // 0x65dd0: mov r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,2)?)?;
    // 0x65dd4: addne r3, r0, #45
    c.emit(arm32::data_immediate(C::Ne,Op::Add,R::new(3)?,R::R0,45)?)?;
    // 0x65dd8: movne r1, #0
    c.emit(arm32::data_immediate(C::Ne,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65ddc: str lr, [r0, #0x5c]
    c.emit(arm32::transfer_word(C::Al,false,R::LR,R::R0,92)?)?;
    // 0x65de0: str r2, [r0, #0x6c8]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R0,1736)?)?;
    // 0x65de4: str r3, [r0, #0x6cc]
    c.emit(arm32::transfer_word(C::Al,false,R::new(3)?,R::R0,1740)?)?;
    // 0x65de8: strb r1, [r0, #0x6d0]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::R0,1744)?)?;
    // 0x65dec: pop {r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x65df0: ldr r1, [pc, #0x20]         @ 0x10618 <entry+0x10618>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,32)?)?;
    // 0x65df4: ldr r2, [pc, #0x20]         @ 0x1061c <entry+0x1061c>
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::PC,32)?)?;
    // 0x65df8: mov r3, #86
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(3)?,R::R0,86)?)?;
    // 0x65dfc: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x65e00: orr r3, r3, #5376
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::new(3)?,R::new(3)?,5376)?)?;
    // 0x65e04: bl 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x37178
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,true)?)?;
    // 0x65e08: ldr r1, [pc, #0x10]         @ 0x10620 <entry+0x10620>
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::PC,16)?)?;
    // 0x65e0c: mov r0, #16
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,16)?)?;
    // 0x65e10: pop {r11, lr}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::R11.bit()|R::LR.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x65e14: b 0x2ec94 <entry+0xfffffffffffd9494> @ imm = #-0x37188
    c.emit(arm32::branch(C::Al,c.at as u32,0x2ec94,false)?)?;
 c.check(0x65e18)?;
 // Handler pointers/parameters are DATA, not executable instructions.
 const INLINE_DATA_65E18:[u32;3]=[0x000f17e6,0x000f1811,0x000e2788];
 for value in INLINE_DATA_65E18{c.emit(value)?;}
 // Executable A32 block 0x65e24..0x66000.
 c.check(0x65e24)?;
    // 0x65e24: push {r4, r5, r6, r7, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x65e28: add r11, sp, #16
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,16)?)?;
    // 0x65e2c: mov r4, r1
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R1))?;
    // 0x65e30: mov r5, r0
    c.emit(arm32::mov_register(C::Al,R::new(5)?,R::R0))?;
    // 0x65e34: ldr r0, [r4, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,588)?)?;
    // 0x65e38: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65e3c: beq 0x65e90 <entry+0x10690> @ imm = #0x4c
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65e90,false)?)?;
    // 0x65e40: add r7, r4, #664
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(4)?,664)?)?;
    // 0x65e44: mov r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(6)?,R::R0,0)?)?;
    // 0x65e48: ldr r1, [r7]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(7)?,0)?)?;
    // 0x65e4c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x65e50: bne 0x65e78 <entry+0x10678> @ imm = #0x20
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65e78,false)?)?;
    // 0x65e54: ldr r1, [r5, #0x26c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(5)?,620)?)?;
    // 0x65e58: ldr r2, [r7, #-0x8]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(7)?,-8)?)?;
    // 0x65e5c: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65e60: beq 0x65e78 <entry+0x10678> @ imm = #0x10
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65e78,false)?)?;
    // 0x65e64: mov r0, r4
    c.emit(arm32::mov_register(C::Al,R::R0,R::new(4)?))?;
    // 0x65e68: mov r1, r6
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(6)?))?;
    // 0x65e6c: mov r2, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,1)?)?;
    // 0x65e70: bl 0x61960 <entry+0xc160>  @ imm = #-0x4518
    c.emit(arm32::branch(C::Al,c.at as u32,0x61960,true)?)?;
    // 0x65e74: ldr r0, [r4, #0x24c]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(4)?,588)?)?;
    // 0x65e78: cmp r6, #7
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,7)?)?;
    // 0x65e7c: popgt {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Gt,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x65e80: add r6, r6, #1
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(6)?,R::new(6)?,1)?)?;
    // 0x65e84: add r7, r7, #72
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(7)?,72)?)?;
    // 0x65e88: cmp r6, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(6)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65e8c: blo 0x65e48 <entry+0x10648> @ imm = #-0x4c
    c.emit(arm32::branch(C::Cc,c.at as u32,0x65e48,false)?)?;
    // 0x65e90: pop {r4, r5, r6, r7, r11, pc}
    c.emit(arm32::transfer_multiple(C::Al,true,R::SP,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::R11.bit()|R::PC.bit(),arm32::BlockMode::IncrementAfter,true)?)?;
    // 0x65e94: push {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    c.emit(arm32::push_registers(C::Al,R::new(4)?.bit()|R::new(5)?.bit()|R::new(6)?.bit()|R::new(7)?.bit()|R::new(8)?.bit()|R::new(9)?.bit()|R::new(10)?.bit()|R::R11.bit()|R::LR.bit())?)?;
    // 0x65e98: add r11, sp, #28
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R11,R::SP,28)?)?;
    // 0x65e9c: sub sp, sp, #108
    c.emit(arm32::data_immediate(C::Al,Op::Sub,R::SP,R::SP,108)?)?;
    // 0x65ea0: mov r4, r0
    c.emit(arm32::mov_register(C::Al,R::new(4)?,R::R0))?;
    // 0x65ea4: mov r1, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,0)?)?;
    // 0x65ea8: ldr r5, [r4, #0x2c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(5)?,R::new(4)?,44)?)?;
    // 0x65eac: add r2, r4, #460
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::new(4)?,460)?)?;
    // 0x65eb0: add r2, r2, #2048
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::R2,R::R2,2048)?)?;
    // 0x65eb4: ldrb r0, [r5, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,1596)?)?;
    // 0x65eb8: ldr r10, [r5, #0x24]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::new(5)?,36)?)?;
    // 0x65ebc: str r1, [r5, #0x1b0]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,432)?)?;
    // 0x65ec0: str r1, [r5, #0x1b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,436)?)?;
    // 0x65ec4: str r2, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R2,R::R11,-60)?)?;
    // 0x65ec8: ldr r9, [r2, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::new(9)?,R::R2,R::R0,Shift::Lsl,2)?)?;
    // 0x65ecc: str r1, [r5, #0x2b4]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,692)?)?;
    // 0x65ed0: str r1, [r5, #0x2bc]
    c.emit(arm32::transfer_word(C::Al,false,R::R1,R::new(5)?,700)?)?;
    // 0x65ed4: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65ed8: ldrb r1, [r9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(9)?,0)?)?;
    // 0x65edc: strb r1, [r5, #0x34c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R1,R::new(5)?,844)?)?;
    // 0x65ee0: bne 0x65f10 <entry+0x10710> @ imm = #0x28
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65f10,false)?)?;
    // 0x65ee4: ldr r1, [r10, #0x16c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::new(10)?,364)?)?;
    // 0x65ee8: ldr r2, [r5, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::new(5)?,988)?)?;
    // 0x65eec: sub r2, r2, r1
    c.emit(arm32::data_register(C::Al,Op::Sub,R::R2,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65ef0: ldrb r1, [r4, #0x996]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::new(4)?,2454)?)?;
    // 0x65ef4: cmp r2, r1
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::R2,R::R1,Shift::Lsl,0,false)?)?;
    // 0x65ef8: bhi 0x65f10 <entry+0x10710> @ imm = #0x10
    c.emit(arm32::branch(C::Hi,c.at as u32,0x65f10,false)?)?;
    // 0x65efc: ldrb r2, [r9, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R2,R::new(9)?,3)?)?;
    // 0x65f00: cmp r2, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R2,2)?)?;
    // 0x65f04: addhs r0, r1, r0
    c.emit(arm32::data_register(C::Cs,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65f08: ldrhs r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Cs,true,R::R1,R::R11,-60)?)?;
    // 0x65f0c: ldrhs r9, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Cs,true,R::new(9)?,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x65f10: ldrb r0, [r5, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(5)?,997)?)?;
    // 0x65f14: str r0, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::SP,60)?)?;
    // 0x65f18: ldr r0, [r5, #0x190]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(5)?,400)?)?;
    // 0x65f1c: str r5, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,false,R::new(5)?,R::R11,-48)?)?;
    // 0x65f20: str r10, [r11, #-0x34]
    c.emit(arm32::transfer_word(C::Al,false,R::new(10)?,R::R11,-52)?)?;
    // 0x65f24: str r4, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,false,R::new(4)?,R::R11,-64)?)?;
    // 0x65f28: cmp r0, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,2)?)?;
    // 0x65f2c: bne 0x6612c <entry+0x1092c> @ imm = #0x1f8
    c.emit(arm32::branch(C::Ne,c.at as u32,0x6612c,false)?)?;
    // 0x65f30: ldr r0, [r10, #0x54]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::new(10)?,84)?)?;
    // 0x65f34: cmp r0, #17
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,17)?)?;
    // 0x65f38: bhi 0x6612c <entry+0x1092c> @ imm = #0x1ec
    c.emit(arm32::branch(C::Hi,c.at as u32,0x6612c,false)?)?;
    // 0x65f3c: mov r2, #33024
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R2,R::R0,33024)?)?;
    // 0x65f40: mov r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R1,R::R0,1)?)?;
    // 0x65f44: orr r2, r2, #196608
    c.emit(arm32::data_immediate(C::Al,Op::Orr,R::R2,R::R2,196608)?)?;
    // 0x65f48: tst r2, r1, lsl r0
    c.emit(arm32::data_register_shifted_by_register(C::Al,Op::Tst,R::R0,R::R2,R::R1,Shift::Lsl,R::R0,false)?)?;
    // 0x65f4c: beq 0x6612c <entry+0x1092c> @ imm = #0x1d8
    c.emit(arm32::branch(C::Eq,c.at as u32,0x6612c,false)?)?;
    // 0x65f50: ldrb r8, [r9, #0x1]
    c.emit(arm32::transfer_byte(C::Al,true,R::new(8)?,R::new(9)?,1)?)?;
    // 0x65f54: cmp r8, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(8)?,0)?)?;
    // 0x65f58: beq 0x66084 <entry+0x10884> @ imm = #0x124
    c.emit(arm32::branch(C::Eq,c.at as u32,0x66084,false)?)?;
    // 0x65f5c: mov r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::R0,R::R0,0)?)?;
    // 0x65f60: add r7, r10, #584
    c.emit(arm32::data_immediate(C::Al,Op::Add,R::new(7)?,R::new(10)?,584)?)?;
    // 0x65f64: mvn r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mvn,R::new(6)?,R::R0,0)?)?;
    // 0x65f68: mov r5, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(5)?,R::R0,0)?)?;
    // 0x65f6c: mov r4, #0
    c.emit(arm32::data_immediate(C::Al,Op::Mov,R::new(4)?,R::R0,0)?)?;
    // 0x65f70: str r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,false,R::R0,R::R11,-56)?)?;
    // 0x65f74: ldr r10, [sp, #0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::new(10)?,R::SP,60)?)?;
    // 0x65f78: str r7, [sp, #0x44]
    c.emit(arm32::transfer_word(C::Al,false,R::new(7)?,R::SP,68)?)?;
    // 0x65f7c: ldrb r0, [r9]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,0)?)?;
    // 0x65f80: add r0, r9, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::new(9)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65f84: add r0, r0, r5
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R0,R::new(5)?,Shift::Lsl,0,false)?)?;
    // 0x65f88: ldrb r1, [r0, #0xb]
    c.emit(arm32::transfer_byte(C::Al,true,R::R1,R::R0,11)?)?;
    // 0x65f8c: cmp r1, #1
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,1)?)?;
    // 0x65f90: bne 0x65fb0 <entry+0x107b0> @ imm = #0x18
    c.emit(arm32::branch(C::Ne,c.at as u32,0x65fb0,false)?)?;
    // 0x65f94: ldr r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x65f98: ldrsb r0, [r0, #4]
    c.emit(arm32::load_signed_byte(C::Al,R::R0,R::R0,4)?)?;
    // 0x65f9c: ldr r1, [r1, #0x3dc]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R1,988)?)?;
    // 0x65fa0: add r0, r1, r0
    c.emit(arm32::data_register(C::Al,Op::Add,R::R0,R::R1,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65fa4: mov r1, r7
    c.emit(arm32::mov_register(C::Al,R::R1,R::new(7)?))?;
    // 0x65fa8: bl 0x670a4 <entry+0x118a4> @ imm = #0x10f4
    c.emit(arm32::branch(C::Al,c.at as u32,0x670a4,true)?)?;
    // 0x65fac: mov r6, r0
    c.emit(arm32::mov_register(C::Al,R::new(6)?,R::R0))?;
    // 0x65fb0: cmp r6, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::new(6)?,0)?)?;
    // 0x65fb4: blt 0x65fc8 <entry+0x107c8> @ imm = #0xc
    c.emit(arm32::branch(C::Lt,c.at as u32,0x65fc8,false)?)?;
    // 0x65fb8: ldrb r0, [r9, #0x3]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::new(9)?,3)?)?;
    // 0x65fbc: cmp r4, r0
    c.emit(arm32::data_register(C::Al,Op::Cmp,R::R0,R::new(4)?,R::R0,Shift::Lsl,0,false)?)?;
    // 0x65fc0: addlt r4, r4, #1
    c.emit(arm32::data_immediate(C::Lt,Op::Add,R::new(4)?,R::new(4)?,1)?)?;
    // 0x65fc4: b 0x66068 <entry+0x10868> @ imm = #0x9c
    c.emit(arm32::branch(C::Al,c.at as u32,0x66068,false)?)?;
    // 0x65fc8: ldr r0, [r11, #-0x38]
    c.emit(arm32::transfer_word(C::Al,true,R::R0,R::R11,-56)?)?;
    // 0x65fcc: cmp r0, #0
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R0,0)?)?;
    // 0x65fd0: beq 0x65fe4 <entry+0x107e4> @ imm = #0xc
    c.emit(arm32::branch(C::Eq,c.at as u32,0x65fe4,false)?)?;
    // 0x65fd4: ldr r1, [r11, #-0x3c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-60)?)?;
    // 0x65fd8: and r0, r10, #255
    c.emit(arm32::data_immediate(C::Al,Op::And,R::R0,R::new(10)?,255)?)?;
    // 0x65fdc: ldr r0, [r1, r0, lsl #2]
    c.emit(arm32::transfer_word_register(C::Al,true,R::R0,R::R1,R::R0,Shift::Lsl,2)?)?;
    // 0x65fe0: b 0x6605c <entry+0x1085c> @ imm = #0x74
    c.emit(arm32::branch(C::Al,c.at as u32,0x6605c,false)?)?;
    // 0x65fe4: ldr r2, [r11, #-0x40]
    c.emit(arm32::transfer_word(C::Al,true,R::R2,R::R11,-64)?)?;
    // 0x65fe8: ldr r1, [r11, #-0x30]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R11,-48)?)?;
    // 0x65fec: ldrb r0, [r2, #0x99a]
    c.emit(arm32::transfer_byte(C::Al,true,R::R0,R::R2,2458)?)?;
    // 0x65ff0: strb r0, [r1, #0x3e5]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,997)?)?;
    // 0x65ff4: strb r0, [r1, #0x63c]
    c.emit(arm32::transfer_byte(C::Al,false,R::R0,R::R1,1596)?)?;
    // 0x65ff8: ldr r1, [r2, #0x39c]
    c.emit(arm32::transfer_word(C::Al,true,R::R1,R::R2,924)?)?;
    // 0x65ffc: cmp r1, #2
    c.emit(arm32::data_immediate(C::Al,Op::Cmp,R::R0,R::R1,2)?)?;
 c.check(END)
}
pub fn fragment()->Result<Vec<u8>,String>{let mut c=Code::new();emit_controller_packet(&mut c)?;Ok(c.bytes)}
#[cfg(test)]mod tests{use super::*;
 #[test]fn chunk_len_and_bounds(){assert_eq!(fragment().unwrap().len(),END-START);assert!(END<=0x68000);}
 #[test]fn original_firmware_byte_comparison_when_present(){
  if let Ok(path)=std::env::var("AGENT04_PRIVATE_VENUS"){
   let reference=std::fs::read(path).unwrap();let generated=fragment().unwrap();
   assert_eq!(&generated[..],&reference[FILE_OFFSET..FILE_OFFSET+generated.len()]);
  }
 }
}
